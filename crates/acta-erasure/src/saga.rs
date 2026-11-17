//! Löschsaga als Zustandsautomat je Löschvorgang (SPEC 8.3).
//!
//! Der Automat ist rein: er führt keine E/A aus und kennt keine Schlüssel. Er prüft nur, ob ein
//! Übergang zulässig ist. Durabilität (Z4) und die Ausführung der Schritte liegen beim Aufrufer.
//!
//! **Abweichung von Anhang B.1.** Anhang B nennt `Requested → Planned` und `Planned → Gated`.
//! Kapitel 8.3 legt dagegen `S1 → Gated` und `S2 → Planned` fest. Dieser Automat folgt 8.3,
//! weil Kapitel 8 das Verfahren beschreibt und Anhang B es nur zusammenfasst. Der Widerspruch
//! ist offen (siehe `CLAUDE.md`, Abschnitt Status).

use acta_base::ids::ErasureId;

use crate::error::ErasureError;

/// Schwelle des vorgezogenen Pfads: 70 % des Zeitbudgets `erase_sla_B` (SPEC 8.3), in Promille.
pub const ADVANCED_SLA_PERMILLE: u16 = 700;

/// Weg der Saga. Er wird vor S1 festgelegt (siehe [`choose_route`]) und danach nicht mehr geändert.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Route {
    /// Standardpfad: S1, S2, S3, S4, S5.
    Standard,
    /// Vorgezogener Pfad (Klasse `Besonders` oder SLA ≥ 70 %): S1, S2, S5a, S3, S4, S5b.
    Advanced,
}

/// Wahl des Wegs. Ganzzahlig, ohne Gleitkomma (SPEC 2.4).
#[must_use]
pub const fn choose_route(besonders: bool, sla_used_permille: u16) -> Route {
    if besonders || sla_used_permille >= ADVANCED_SLA_PERMILLE {
        Route::Advanced
    } else {
        Route::Standard
    }
}

/// Zustand der Saga nach Abschluss eines Schritts (SPEC 8.3, Tabelle der Schritte).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Stage {
    /// S0: Anfrage im E-Ledger, `ErasureId` vergeben.
    Requested,
    /// S1: Trockenlauf abgeschlossen, Gate-Sperre durabel.
    Gated,
    /// S2: Messmuster, Ausgangskorb, Kopienregister und Arbeitslisten erzeugt.
    Planned,
    /// S5a, nur vorgezogener Pfad: Schlüssel der Zielmenge vernichtet, bevor S3 läuft.
    EarlyKeyTombstoned,
    /// S3: Caches, Segmente, Indizes und Journal bereinigt.
    Purged,
    /// S4: Messung Klasse B.
    PlaintextMeasured,
    /// S5 (Standardpfad) bzw. S5b (vorgezogener Pfad): Schlüssel vernichtet, Grabstein gesetzt.
    KeyTombstoned,
    /// S6: Rotation und `DestroyProof`.
    Rotated,
    /// S7: Sicherungsfenster abgelaufen, Sicherungsepochen vernichtet.
    BackupsExpired,
    /// S8: Messung Klasse K und M.
    Measured,
    /// S9: Bericht im E-Ledger, Senke bestätigt. Endzustand.
    Reported,
}

impl Stage {
    /// Schrittkennung nach SPEC 8.3.
    #[must_use]
    pub const fn step(self) -> &'static str {
        match self {
            Self::Requested => "S0",
            Self::Gated => "S1",
            Self::Planned => "S2",
            Self::EarlyKeyTombstoned => "S5a",
            Self::Purged => "S3",
            Self::PlaintextMeasured => "S4",
            Self::KeyTombstoned => "S5",
            Self::Rotated => "S6",
            Self::BackupsExpired => "S7",
            Self::Measured => "S8",
            Self::Reported => "S9",
        }
    }
}

/// Grund, aus dem eine Saga auf `Pending` wartet (SPEC Anhang B.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PendingReason {
    /// Anker-Bestätigung fehlt.
    Anchor,
    /// Registersenke nicht erreichbar.
    Sink,
    /// Review-Entscheidung fehlt.
    Review,
    /// Entscheidung des Archivs fehlt.
    Archive,
    /// Sicherungsfenster läuft noch.
    Backup,
    /// Nicht behebbarer Fehler; Alarm `SagaStuck`.
    SagaStuck,
    /// Messung ungültig; Alarm `MeasureInvalid`.
    MeasureInvalid,
}

/// Nachfolger eines Zustands auf einem Weg. `None` am Endzustand und bei unzulässigen Kanten.
const fn successor(route: Route, stage: Stage) -> Option<Stage> {
    use Stage::{
        BackupsExpired, EarlyKeyTombstoned, Gated, KeyTombstoned, Measured, PlaintextMeasured,
        Planned, Purged, Reported, Requested, Rotated,
    };
    match (route, stage) {
        (_, Requested) => Some(Gated),
        (_, Gated) => Some(Planned),
        (Route::Standard, Planned) => Some(Purged),
        (Route::Advanced, Planned) => Some(EarlyKeyTombstoned),
        (Route::Advanced, EarlyKeyTombstoned) => Some(Purged),
        (_, Purged) => Some(PlaintextMeasured),
        (_, PlaintextMeasured) => Some(KeyTombstoned),
        (_, KeyTombstoned) => Some(Rotated),
        (_, Rotated) => Some(BackupsExpired),
        (_, BackupsExpired) => Some(Measured),
        (_, Measured) => Some(Reported),
        (Route::Standard, EarlyKeyTombstoned) | (_, Reported) => None,
    }
}

/// Eine Löschsaga (SPEC 8.3). Die Saga kennt keine Schlüssel; sie hält nur Weg, Stand und Wartegrund.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Saga {
    id: ErasureId,
    route: Route,
    stage: Stage,
    pending: Option<PendingReason>,
}

impl Saga {
    /// Neue Saga im Zustand `Requested` (S0).
    #[must_use]
    pub const fn new(id: ErasureId, route: Route) -> Self {
        Self {
            id,
            route,
            stage: Stage::Requested,
            pending: None,
        }
    }

    /// Kennung des Löschvorgangs.
    #[must_use]
    pub const fn id(&self) -> ErasureId {
        self.id
    }

    /// Weg der Saga.
    #[must_use]
    pub const fn route(&self) -> Route {
        self.route
    }

    /// Letzter abgeschlossener Schritt.
    #[must_use]
    pub const fn stage(&self) -> Stage {
        self.stage
    }

    /// Wartegrund, falls die Saga auf `Pending` steht.
    #[must_use]
    pub const fn pending(&self) -> Option<PendingReason> {
        self.pending
    }

    /// Führt den Übergang zum nächsten Schritt aus, sofern der Weg ihn vorsieht.
    ///
    /// # Errors
    /// `Terminal` nach `Reported`; `IllegalTransition` bei wartender Saga oder unzulässigem Ziel.
    /// Der Zustand bleibt in allen Fehlerfällen unverändert.
    pub fn advance(&mut self, to: Stage) -> Result<(), ErasureError> {
        if self.stage == Stage::Reported {
            return Err(ErasureError::Terminal);
        }
        if self.pending.is_some() {
            return Err(ErasureError::IllegalTransition);
        }
        match successor(self.route, self.stage) {
            Some(next) if next == to => {
                self.stage = to;
                Ok(())
            }
            _ => Err(ErasureError::IllegalTransition),
        }
    }

    /// Setzt die Saga auf `Pending` (Anhang B.1: jeder Zustand kann wartend werden).
    ///
    /// # Errors
    /// `Terminal` nach `Reported`.
    pub fn suspend(&mut self, reason: PendingReason) -> Result<(), ErasureError> {
        if self.stage == Stage::Reported {
            return Err(ErasureError::Terminal);
        }
        self.pending = Some(reason);
        Ok(())
    }

    /// Hebt das Warten auf; die Saga setzt beim gespeicherten Schritt fort.
    ///
    /// # Errors
    /// `Terminal` nach `Reported`; `IllegalTransition`, wenn die Saga nicht wartet.
    pub fn resume(&mut self) -> Result<(), ErasureError> {
        if self.stage == Stage::Reported {
            return Err(ErasureError::Terminal);
        }
        match self.pending.take() {
            Some(_) => Ok(()),
            None => Err(ErasureError::IllegalTransition),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id() -> ErasureId {
        ErasureId::from_v7(1_700_000_000_000, [7; 10]).expect("fixture: ms fits 48 bits")
    }

    fn run(saga: &mut Saga, steps: &[Stage]) -> Result<(), ErasureError> {
        for step in steps {
            saga.advance(*step)?;
        }
        Ok(())
    }

    const STANDARD: [Stage; 9] = [
        Stage::Gated,
        Stage::Planned,
        Stage::Purged,
        Stage::PlaintextMeasured,
        Stage::KeyTombstoned,
        Stage::Rotated,
        Stage::BackupsExpired,
        Stage::Measured,
        Stage::Reported,
    ];

    const ADVANCED: [Stage; 10] = [
        Stage::Gated,
        Stage::Planned,
        Stage::EarlyKeyTombstoned,
        Stage::Purged,
        Stage::PlaintextMeasured,
        Stage::KeyTombstoned,
        Stage::Rotated,
        Stage::BackupsExpired,
        Stage::Measured,
        Stage::Reported,
    ];

    #[test]
    fn standard_route_runs_to_reported() {
        let mut saga = Saga::new(id(), Route::Standard);
        assert!(run(&mut saga, &STANDARD).is_ok());
        assert_eq!(saga.stage(), Stage::Reported);
    }

    #[test]
    fn advanced_route_runs_to_reported_with_early_key_step() {
        let mut saga = Saga::new(id(), Route::Advanced);
        assert!(run(&mut saga, &ADVANCED).is_ok());
        assert_eq!(saga.stage(), Stage::Reported);
    }

    #[test]
    fn steps_cannot_be_skipped() {
        let mut saga = Saga::new(id(), Route::Standard);
        assert!(matches!(
            saga.advance(Stage::Planned),
            Err(ErasureError::IllegalTransition)
        ));
        assert_eq!(saga.stage(), Stage::Requested);
    }

    #[test]
    fn key_tombstone_requires_plaintext_measurement_on_standard_route() {
        let mut saga = Saga::new(id(), Route::Standard);
        assert!(run(&mut saga, &[Stage::Gated, Stage::Planned, Stage::Purged]).is_ok());
        assert!(matches!(
            saga.advance(Stage::KeyTombstoned),
            Err(ErasureError::IllegalTransition)
        ));
    }

    #[test]
    fn early_key_step_exists_only_on_advanced_route() {
        let mut standard = Saga::new(id(), Route::Standard);
        assert!(run(&mut standard, &[Stage::Gated, Stage::Planned]).is_ok());
        assert!(matches!(
            standard.advance(Stage::EarlyKeyTombstoned),
            Err(ErasureError::IllegalTransition)
        ));

        let mut advanced = Saga::new(id(), Route::Advanced);
        assert!(run(&mut advanced, &[Stage::Gated, Stage::Planned]).is_ok());
        // S5a muss vor S3 liegen.
        assert!(matches!(
            advanced.advance(Stage::Purged),
            Err(ErasureError::IllegalTransition)
        ));
    }

    #[test]
    fn reported_is_terminal() {
        let mut saga = Saga::new(id(), Route::Standard);
        assert!(run(&mut saga, &STANDARD).is_ok());
        assert!(matches!(
            saga.advance(Stage::Gated),
            Err(ErasureError::Terminal)
        ));
        assert!(matches!(
            saga.suspend(PendingReason::Sink),
            Err(ErasureError::Terminal)
        ));
    }

    #[test]
    fn pending_blocks_progress_until_resumed() {
        let mut saga = Saga::new(id(), Route::Standard);
        assert!(saga.advance(Stage::Gated).is_ok());
        assert!(saga.suspend(PendingReason::Anchor).is_ok());
        assert!(matches!(
            saga.advance(Stage::Planned),
            Err(ErasureError::IllegalTransition)
        ));
        assert_eq!(saga.pending(), Some(PendingReason::Anchor));
        assert!(saga.resume().is_ok());
        assert!(saga.advance(Stage::Planned).is_ok());
    }

    #[test]
    fn resume_without_pending_is_rejected() {
        let mut saga = Saga::new(id(), Route::Standard);
        assert!(matches!(
            saga.resume(),
            Err(ErasureError::IllegalTransition)
        ));
    }

    #[test]
    fn route_is_advanced_for_besonders_or_sla_at_70_percent() {
        assert_eq!(choose_route(true, 0), Route::Advanced);
        assert_eq!(choose_route(false, 700), Route::Advanced);
        assert_eq!(choose_route(false, 699), Route::Standard);
        assert_eq!(choose_route(false, 0), Route::Standard);
    }
}
