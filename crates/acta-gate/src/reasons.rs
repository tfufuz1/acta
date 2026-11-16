//! Gründe für Sperren und Löschvorgänge (SPEC 8.1, 8.2).
//!
//! Die Codes stehen im Gate-Log (SPEC 5.3) und sind stabil. Sie enthalten
//! keine Kennungen von Personen (Z15).

/// Grund einer Einschränkung `Restricted(grund)` (SPEC 8.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum RestrictReason {
    /// Aufbewahrungspflicht läuft noch.
    Retention,
    /// Rechtliche Aufbewahrung (`legal_hold`).
    LegalHold,
    /// Widerspruch der betroffenen Person (Art. 21), bis zur Prüfung.
    Objection,
    /// Einschränkung auf Antrag (Art. 18) oder ungeklärte Identität.
    Contested,
    /// Mehrere Betroffene; Review entscheidet (Z3).
    MultiSubjectReview,
    /// Vorsorglich, bis ein Auslöser geprüft ist.
    Precautionary,
}

impl RestrictReason {
    /// Stabiler Code für das Gate-Log.
    #[must_use]
    pub const fn code(self) -> u8 {
        match self {
            Self::Retention => 1,
            Self::LegalHold => 2,
            Self::Objection => 3,
            Self::Contested => 4,
            Self::MultiSubjectReview => 5,
            Self::Precautionary => 6,
        }
    }

    /// Rückabbildung eines Codes; unbekannte Codes sind `None`.
    #[must_use]
    pub const fn from_code(code: u8) -> Option<Self> {
        match code {
            1 => Some(Self::Retention),
            2 => Some(Self::LegalHold),
            3 => Some(Self::Objection),
            4 => Some(Self::Contested),
            5 => Some(Self::MultiSubjectReview),
            6 => Some(Self::Precautionary),
            _ => None,
        }
    }
}

/// Auslöser eines Löschvorgangs (SPEC 8.2, Aufzählung „Auslöser“).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum EraseReason {
    /// Antrag der betroffenen Person (Art. 17 Abs. 1).
    Request,
    /// Widerruf einer Einwilligung.
    Withdrawal,
    /// Widerspruch (Art. 21), nach Prüfung.
    Objection,
    /// Fristablauf der Aufbewahrungsklasse.
    RetentionExpiry,
    /// Zweckfortfall.
    PurposeEnded,
    /// Takedown in Ebene K (6.7).
    Takedown,
    /// Off-Boarding eines Mandanten.
    Offboarding,
    /// Ausscheiden von Mitarbeitenden (`Actor`).
    Departure,
    /// Ablauf von `quarantine_purge_after`.
    QuarantineExpiry,
    /// Ablauf von `payload_retention`.
    PayloadRetentionExpiry,
    /// Ablauf von `history_horizon`.
    HistoryHorizonExpiry,
    /// Gerichtliche Anordnung.
    CourtOrder,
}

impl EraseReason {
    /// Stabiler Code für das Gate-Log.
    #[must_use]
    pub const fn code(self) -> u8 {
        match self {
            Self::Request => 1,
            Self::Withdrawal => 2,
            Self::Objection => 3,
            Self::RetentionExpiry => 4,
            Self::PurposeEnded => 5,
            Self::Takedown => 6,
            Self::Offboarding => 7,
            Self::Departure => 8,
            Self::QuarantineExpiry => 9,
            Self::PayloadRetentionExpiry => 10,
            Self::HistoryHorizonExpiry => 11,
            Self::CourtOrder => 12,
        }
    }

    /// Rückabbildung eines Codes; unbekannte Codes sind `None`.
    #[must_use]
    pub const fn from_code(code: u8) -> Option<Self> {
        match code {
            1 => Some(Self::Request),
            2 => Some(Self::Withdrawal),
            3 => Some(Self::Objection),
            4 => Some(Self::RetentionExpiry),
            5 => Some(Self::PurposeEnded),
            6 => Some(Self::Takedown),
            7 => Some(Self::Offboarding),
            8 => Some(Self::Departure),
            9 => Some(Self::QuarantineExpiry),
            10 => Some(Self::PayloadRetentionExpiry),
            11 => Some(Self::HistoryHorizonExpiry),
            12 => Some(Self::CourtOrder),
            _ => None,
        }
    }
}
