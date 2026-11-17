//! Mehrbetroffene Datensätze (SPEC 8.9, Z3).
//!
//! Der Trockenlauf prüft die Mitbetroffenen einer Version auf Aufbewahrungspflicht, Legal Hold,
//! Einwand und offenen Vorgang. Ergebnis ist nur die Anzahl; Kennungen der Mitbetroffenen
//! verlassen den Trockenlauf nie (SPEC 8.9, Bericht).
//!
//! Automatische Löschung gibt es hier nicht: Konflikte werden sofort eingeschränkt und
//! entscheidet ein Review. Nach Ablauf der Frist gibt es nie ein automatisches `DeleteAll`.

use acta_gate::RestrictReason;

/// Befund eines Trockenlaufs für einen Mitbetroffenen (nur die Art, nie die Kennung).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Finding {
    /// Aufbewahrungspflicht mit `retain_until` in der Zukunft.
    Retention,
    /// `legal_hold` gesetzt.
    LegalHold,
    /// Registrierter Widerspruch oder offener Streit.
    Objection,
    /// Offener Vorgang mit Zuweisung im Fachverfahren.
    OpenProceeding,
}

impl Finding {
    /// Aufbewahrungspflicht und Hold verbieten die Vernichtung des gemeinsamen Dokuments.
    const fn blocks_deletion(self) -> bool {
        matches!(self, Self::Retention | Self::LegalHold)
    }
}

/// Modus einer Löschung eines Betroffenen aus einer Version (SPEC 8.9).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Mode {
    /// `SubjectKey` und `DocumentKey` werden vernichtet; die Version ist für alle unlesbar.
    DeleteAll,
    /// Neue Version ohne den Betroffenen, Inhalt geschwärzt; die alte Version wird bereinigt.
    RedactSplit,
    /// Neue Version ohne Bindung an den Betroffenen; die Daten bleiben für die Mitbetroffenen.
    RestrictShared,
}

/// Ergebnis des Trockenlaufs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Screening {
    /// Kein Befund und keine Sammelakte: `DeleteAll` ist der Standardmodus.
    NoConflict,
    /// Review nötig; die Version steht bis zur Entscheidung auf `Restricted(MultiSubjectReview)`.
    ReviewRequired {
        /// Anzahl der Mitbetroffenen (ohne Kennungen).
        co_subjects: u32,
        /// Anzahl der Befunde.
        findings: u32,
        /// Sammelakte: `RedactSplit` ist Pflicht (8.9).
        collection: bool,
        /// Aufbewahrung oder Hold eines Mitbetroffenen liegt vor.
        blocking: bool,
    },
}

/// Bewertet die Mitbetroffenen einer Version.
///
/// `findings` enthält je Befund einen Eintrag. Die Funktion nimmt keine Kennungen entgegen.
#[must_use]
pub fn screen(co_subjects: u32, collection: bool, findings: &[Finding]) -> Screening {
    if findings.is_empty() && !collection {
        return Screening::NoConflict;
    }
    Screening::ReviewRequired {
        co_subjects,
        findings: u32::try_from(findings.len()).unwrap_or(u32::MAX),
        collection,
        blocking: findings.iter().any(|f| f.blocks_deletion()),
    }
}

/// Eingeschränkter Zustand, den der Trockenlauf sofort setzt (Z3).
#[must_use]
pub const fn initial_restriction(screening: Screening) -> Option<RestrictReason> {
    match screening {
        Screening::NoConflict => None,
        Screening::ReviewRequired { .. } => Some(RestrictReason::MultiSubjectReview),
    }
}

/// Zulässige Modi nach Befund. Die Auswahl selbst trifft ein Review.
///
/// - Sammelakte: nur `RedactSplit` (Pflicht laut 8.9).
/// - Aufbewahrung oder Hold eines Mitbetroffenen: `DeleteAll` entfällt, weil es das gemeinsame
///   Dokument vernichten würde.
/// - Sonst alle Modi.
#[must_use]
pub const fn allowed_modes(screening: Screening) -> &'static [Mode] {
    match screening {
        Screening::NoConflict => &[Mode::DeleteAll],
        Screening::ReviewRequired {
            collection: true, ..
        } => &[Mode::RedactSplit],
        Screening::ReviewRequired { blocking: true, .. } => {
            &[Mode::RedactSplit, Mode::RestrictShared]
        }
        Screening::ReviewRequired { .. } => {
            &[Mode::DeleteAll, Mode::RedactSplit, Mode::RestrictShared]
        }
    }
}

/// Ergebnis nach Ablauf der Review-Frist (`multi_review_max`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DeadlineOutcome {
    /// Der Vorgang darf ohne Review fortfahren.
    Proceed(Mode),
    /// Eskalation an die Datenschutzfunktion (`ReviewOverdue`); das Dokument bleibt eingeschränkt.
    Escalate,
}

/// Nach Fristablauf gibt es kein automatisches `DeleteAll` (Z3, 8.9).
#[must_use]
pub const fn at_review_deadline(screening: Screening) -> DeadlineOutcome {
    match screening {
        Screening::NoConflict => DeadlineOutcome::Proceed(Mode::DeleteAll),
        Screening::ReviewRequired { .. } => DeadlineOutcome::Escalate,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_findings_and_no_collection_is_no_conflict() {
        let s = screen(2, false, &[]);
        assert_eq!(s, Screening::NoConflict);
        assert_eq!(allowed_modes(s), &[Mode::DeleteAll]);
        assert_eq!(initial_restriction(s), None);
    }

    #[test]
    fn z3_conflict_restricts_immediately_with_multi_subject_review() {
        let s = screen(1, false, &[Finding::Objection]);
        assert_eq!(
            initial_restriction(s),
            Some(RestrictReason::MultiSubjectReview)
        );
    }

    #[test]
    fn collection_without_findings_still_needs_review_and_redact_split() {
        let s = screen(5, true, &[]);
        assert!(matches!(
            s,
            Screening::ReviewRequired {
                collection: true,
                ..
            }
        ));
        assert_eq!(allowed_modes(s), &[Mode::RedactSplit]);
    }

    #[test]
    fn retention_or_hold_of_co_subject_forbids_delete_all() {
        for finding in [Finding::Retention, Finding::LegalHold] {
            let s = screen(1, false, &[finding]);
            let modes = allowed_modes(s);
            assert!(!modes.contains(&Mode::DeleteAll), "{finding:?}");
            assert!(modes.contains(&Mode::RestrictShared), "{finding:?}");
        }
    }

    #[test]
    fn objection_allows_all_modes_for_review() {
        let s = screen(1, false, &[Finding::Objection]);
        assert_eq!(
            allowed_modes(s),
            &[Mode::DeleteAll, Mode::RedactSplit, Mode::RestrictShared]
        );
    }

    #[test]
    fn report_carries_counts_only() {
        let s = screen(3, false, &[Finding::Objection, Finding::OpenProceeding]);
        assert_eq!(
            s,
            Screening::ReviewRequired {
                co_subjects: 3,
                findings: 2,
                collection: false,
                blocking: false,
            }
        );
    }

    #[test]
    fn z3_deadline_never_yields_delete_all_for_conflicts() {
        let conflict = screen(1, false, &[Finding::Objection]);
        assert_eq!(at_review_deadline(conflict), DeadlineOutcome::Escalate);
        assert_eq!(
            at_review_deadline(Screening::NoConflict),
            DeadlineOutcome::Proceed(Mode::DeleteAll)
        );
    }
}
