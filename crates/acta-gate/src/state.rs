//! Gate-Zustand (SPEC 5.3): Roaring-Mengen und Epoche, vollständig im Arbeitsspeicher.
//!
//! Die Übergänge sind rein (keine E/A). Persistenz und Reihenfolge (Z4) liegen in `writer`.

use std::collections::BTreeMap;

use acta_base::ids::{KOrd, SubjOrd, VerOrd};
use roaring::{RoaringBitmap, RoaringTreemap};

use crate::error::GateError;
use crate::log::GateOp;
use crate::reasons::RestrictReason;

/// Zustand einer Version aus Sicht eines Lesepfads (SPEC 5.3, 8.1).
///
/// `Erased` fasst die Zustände `Gated`, `Expired` und `Erased` zusammen: für alle
/// drei ist kein Lesen möglich (Z1). `ErasedVers` hat Vorrang vor `RestrictedVers`
/// und `SupersededVers`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum GateState {
    /// Kein Eintrag im Gate.
    Live,
    /// Eingeschränkt; nur `restricted_read` (8.1).
    Restricted(RestrictReason),
    /// Ältere Version; nur `restricted_read`.
    Superseded,
    /// Erledigt oder laufend gelöscht; kein Lesen, kein Schreiben.
    Erased,
}

/// Ergebnis einer Herkunftsprüfung (SPEC 3.7, 5.2 Ziffer 3).
///
/// Bei mehreren Treffern gilt die restriktivste Wirkung in dieser Reihenfolge:
/// `Erased` > `Redacted` > `Restricted` > `Superseded` > `Live`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum LineageHit {
    /// Kein Element der Herkunft ist betroffen.
    Live,
    /// Eine ältere Version ist Teil der Herkunft.
    Superseded,
    /// Eine eingeschränkte Version ist Teil der Herkunft.
    Restricted(RestrictReason),
    /// Ein K-Objekt mit Takedown ist Teil der Herkunft (`RedactedK`).
    Redacted,
    /// Eine erledigte oder laufend gelöschte Version ist Teil der Herkunft.
    Erased,
}

impl LineageHit {
    const fn severity(self) -> u8 {
        match self {
            Self::Live => 0,
            Self::Superseded => 1,
            Self::Restricted(_) => 2,
            Self::Redacted => 3,
            Self::Erased => 4,
        }
    }

    /// Die restriktivere der beiden Wirkungen; bei Gleichstand bleibt `self`.
    #[must_use]
    pub const fn worst(self, other: Self) -> Self {
        if other.severity() > self.severity() {
            other
        } else {
            self
        }
    }
}

impl From<GateState> for LineageHit {
    fn from(state: GateState) -> Self {
        match state {
            GateState::Live => Self::Live,
            GateState::Restricted(reason) => Self::Restricted(reason),
            GateState::Superseded => Self::Superseded,
            GateState::Erased => Self::Erased,
        }
    }
}

/// Herkunft einer abgeleiteten Struktur (`source_set`, SPEC 5.2).
///
/// Nicht leer (Z13): eine leere Herkunft ist nicht konstruierbar.
#[derive(Debug, Clone, PartialEq)]
pub struct Lineage {
    vers: RoaringBitmap,
    kords: RoaringTreemap,
}

impl Lineage {
    /// Erzeugt eine Herkunft aus Versionen und K-Objekten.
    ///
    /// # Errors
    /// `GateError::EmptyLineage`, wenn beide Mengen leer sind.
    pub fn new(vers: RoaringBitmap, kords: RoaringTreemap) -> Result<Self, GateError> {
        if vers.is_empty() && kords.is_empty() {
            return Err(GateError::EmptyLineage);
        }
        Ok(Self { vers, kords })
    }

    /// Versionen (`VerOrd`) der Herkunft.
    #[must_use]
    pub const fn vers(&self) -> &RoaringBitmap {
        &self.vers
    }

    /// K-Objekte (`KOrd`) der Herkunft.
    #[must_use]
    pub const fn kords(&self) -> &RoaringTreemap {
        &self.kords
    }
}

/// Zielmenge eines Löschvorgangs (SPEC 3.7). Die Ordinale stammen aus dem Gate.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TargetSet {
    /// Betroffene (`SubjOrd`).
    pub subjects: RoaringBitmap,
    /// Versionen (`VerOrd`).
    pub vers: RoaringBitmap,
    /// K-Objekte (`KOrd`).
    pub kords: RoaringTreemap,
}

/// Ein Stand des Gates. Jede Änderung erzeugt einen neuen Stand (Zeigertausch, 5.3).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GateSnapshot {
    pub(crate) epoch: u64,
    pub(crate) erased_subjects: RoaringBitmap,
    pub(crate) erased_vers: RoaringBitmap,
    pub(crate) restricted_vers: RoaringBitmap,
    pub(crate) restrict_reasons: BTreeMap<u32, RestrictReason>,
    pub(crate) superseded_vers: RoaringBitmap,
    pub(crate) redacted_k: RoaringTreemap,
    pub(crate) high_water: [u64; 3],
}

impl GateSnapshot {
    /// Epoche: Sequenznummer des letzten angewendeten Rahmens (SPEC 5.3).
    #[must_use]
    pub const fn epoch(&self) -> u64 {
        self.epoch
    }

    /// Zustand einer Version. O(1) ohne E/A (SPEC 5.3).
    ///
    /// Ein Eintrag in `restricted_vers` ohne Grund ist ein interner Widerspruch;
    /// er wird fail-closed als `Erased` gemeldet.
    #[must_use]
    pub fn state(&self, ver: VerOrd) -> GateState {
        let v = ver.0;
        if self.erased_vers.contains(v) {
            return GateState::Erased;
        }
        if self.restricted_vers.contains(v) {
            return match self.restrict_reasons.get(&v) {
                Some(reason) => GateState::Restricted(*reason),
                None => GateState::Erased,
            };
        }
        if self.superseded_vers.contains(v) {
            return GateState::Superseded;
        }
        GateState::Live
    }

    /// Takedown in K? O(1) ohne E/A.
    #[must_use]
    pub fn is_redacted_k(&self, k: KOrd) -> bool {
        self.redacted_k.contains(k.0)
    }

    /// Betroffener mit laufender oder abgeschlossener Löschung (verhindert neue Zuordnung).
    #[must_use]
    pub fn is_subject_erased(&self, subject: SubjOrd) -> bool {
        self.erased_subjects.contains(subject.0)
    }

    /// Prüft eine Herkunft gegen den Stand; die Prüfung bricht beim ersten `Erased` ab.
    #[must_use]
    pub fn lineage_hit(&self, lineage: &Lineage) -> LineageHit {
        let mut worst = LineageHit::Live;
        for v in lineage.vers.iter() {
            worst = worst.worst(LineageHit::from(self.state(VerOrd(v))));
            if worst == LineageHit::Erased {
                return worst;
            }
        }
        for k in lineage.kords.iter() {
            if self.redacted_k.contains(k) {
                worst = worst.worst(LineageHit::Redacted);
            }
        }
        worst
    }

    /// Wendet einen Rahmen an. Schlägt die Anwendung fehl, bleibt der Stand unverändert
    /// (der Aufrufer arbeitet auf einer Kopie).
    pub(crate) fn apply(&mut self, op: &GateOp, seq: u64) -> Result<(), GateError> {
        match *op {
            GateOp::EraseSubject(subject) => {
                self.erased_subjects.insert(subject.0);
            }
            GateOp::EraseVer { ver, .. } => self.erase_ver(ver.0),
            GateOp::Restrict { ver, reason } => {
                if self.erased_vers.contains(ver.0) {
                    return Err(GateError::EraseTerminal);
                }
                self.restricted_vers.insert(ver.0);
                self.restrict_reasons.insert(ver.0, reason);
            }
            GateOp::Release(ver) => {
                if self.erased_vers.contains(ver.0) {
                    return Err(GateError::EraseTerminal);
                }
                if !self.restricted_vers.remove(ver.0) {
                    return Err(GateError::NotRestricted);
                }
                self.restrict_reasons.remove(&ver.0);
            }
            GateOp::Supersede(ver) => {
                if !self.erased_vers.contains(ver.0) {
                    self.superseded_vers.insert(ver.0);
                }
            }
            GateOp::RedactK(k) => {
                self.redacted_k.insert(k.0);
            }
            GateOp::HighWater { kind, value } => {
                let slot = self
                    .high_water
                    .get_mut(kind.index())
                    .ok_or(GateError::HighWaterRegression)?;
                if value < *slot {
                    return Err(GateError::HighWaterRegression);
                }
                *slot = value;
            }
        }
        self.epoch = seq;
        Ok(())
    }

    /// `Erased` ist terminal: die Version verliert Einschränkung und Überholung.
    fn erase_ver(&mut self, ver: u32) {
        self.erased_vers.insert(ver);
        self.restricted_vers.remove(ver);
        self.restrict_reasons.remove(&ver);
    }
}
