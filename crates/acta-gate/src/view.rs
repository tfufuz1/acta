//! Lesende Sicht auf das Gate (SPEC 3.7 `ErasedView`, 5.3).
//!
//! Alle Lesepfade nutzen ausschließlich diese Schnittstelle. Der Stand liegt hinter einer
//! Zeigerschicht: Ein Schreiber ersetzt den Zeiger, Leser halten einen `Arc` und blockieren
//! einander nicht länger als den Zeigertausch. Ist die Sperrung nicht lesbar (vergiftete
//! Sperre), meldet jede Abfrage den sicheren Wert: Zugriff verweigert.

use std::sync::{Arc, RwLock};

use acta_base::ids::{KOrd, SubjOrd, VerOrd};

use crate::state::{GateSnapshot, GateState, Lineage, LineageHit};

/// Geteilter Zeiger auf den aktuellen Stand.
pub(crate) type Shared = Arc<RwLock<Arc<GateSnapshot>>>;

/// Nur lesender Zugriff auf den Sperrzustand (SPEC 3.7).
pub trait ErasedView: Send + Sync {
    /// Zustand einer Version. O(1), ohne E/A.
    fn check_ver(&self, ver: VerOrd) -> GateState;
    /// Takedown in K? O(1), ohne E/A.
    fn is_redacted_k(&self, k: KOrd) -> bool;
    /// Betroffener mit laufender oder abgeschlossener Löschung (verhindert neue Zuordnung).
    fn is_subject_erased(&self, subject: SubjOrd) -> bool;
    /// Epoche des gelesenen Standes (Sequenznummer des letzten Rahmens).
    fn epoch(&self) -> u64;
    /// Prüft eine Herkunft gegen den Stand.
    fn lineage_hit(&self, lineage: &Lineage) -> LineageHit;
}

/// Lesegriff; billig kopierbar, teilt den Zeiger mit dem Schreiber.
#[derive(Debug, Clone)]
pub struct GateView {
    shared: Shared,
}

impl GateView {
    pub(crate) const fn new(shared: Shared) -> Self {
        Self { shared }
    }

    /// Momentaufnahme des Standes; `None`, wenn die Sperrung nicht lesbar ist.
    fn load(&self) -> Option<Arc<GateSnapshot>> {
        let guard = self.shared.read().ok()?;
        Some(Arc::clone(&*guard))
    }
}

impl ErasedView for GateView {
    fn check_ver(&self, ver: VerOrd) -> GateState {
        self.load().map_or(GateState::Erased, |s| s.state(ver))
    }

    fn is_redacted_k(&self, k: KOrd) -> bool {
        self.load().is_none_or(|s| s.is_redacted_k(k))
    }

    fn is_subject_erased(&self, subject: SubjOrd) -> bool {
        self.load().is_none_or(|s| s.is_subject_erased(subject))
    }

    fn epoch(&self) -> u64 {
        self.load().map_or(u64::MAX, |s| s.epoch())
    }

    fn lineage_hit(&self, lineage: &Lineage) -> LineageHit {
        self.load()
            .map_or(LineageHit::Erased, |s| s.lineage_hit(lineage))
    }
}
