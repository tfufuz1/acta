//! Lesesperre und `restricted_read` (SPEC 8.1, Z1, Z16).
//!
//! Jeder Lesepfad prüft den Gate-Zustand **vor** jeder Entschlüsselung und Ausgabe. Die
//! Entscheidung hängt nur vom Zustand und vom Zugriffsrecht ab, nie von der Existenz des Ziels.
//! Unbekannte Zustände werden verweigert (Z10, fail-closed).

use acta_gate::GateState;

/// Zweck eines `restricted_read` (SPEC 8.1, Rolle `DataProtection`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Purpose {
    /// Auskunft, Art. 15.
    Auskunft,
    /// Rechtsanspruch.
    Rechtsanspruch,
    /// Berichtigung, Art. 16.
    Berichtigung,
    /// Review.
    Review,
    /// Aufbewahrung.
    Aufbewahrung,
    /// Aufsicht.
    Aufsicht,
}

/// Zugriffsrecht des Lesenden auf das Ziel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Access {
    /// Kein Zugriffsrecht. Dasselbe Fehlerbild wie bei einem nicht vorhandenen Ziel (Z16).
    None,
    /// Regulärer Zugriff auf ein `Live`-Ziel.
    Standard,
    /// Rolle `DataProtection` mit Zweck. Nur für `Restricted` und `Superseded`.
    DataProtection(Purpose),
}

/// Ergebnis der Lesesperre.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ReadDecision {
    /// Lesen zulässig.
    Allowed,
    /// Lesen nur über `restricted_read`. Der Zugriff ist zweckgebunden zu protokollieren (Z15).
    AllowedRestrictedRead,
    /// Ziel ist gesperrt oder erledigt. Nur für Nutzer mit Zugriffsrecht (8.1).
    Erased,
    /// Fehlerbild wie bei einem nicht vorhandenen Ziel (Z16).
    NotFound,
}

/// Lesesperre nach Gate-Zustand und Zugriffsrecht (SPEC 8.1).
///
/// `Erased` gilt für `Gated`, `Expired` und `Erased` gleichermaßen (vgl. `acta_gate::GateState`).
/// `restricted_read` öffnet niemals erledigte Ziele.
#[must_use]
pub const fn decide_read(state: GateState, access: Access) -> ReadDecision {
    match access {
        Access::None => ReadDecision::NotFound,
        Access::Standard => match state {
            GateState::Live => ReadDecision::Allowed,
            GateState::Erased => ReadDecision::Erased,
            _ => ReadDecision::NotFound,
        },
        Access::DataProtection(_) => match state {
            GateState::Live => ReadDecision::Allowed,
            GateState::Restricted(_) | GateState::Superseded => ReadDecision::AllowedRestrictedRead,
            GateState::Erased => ReadDecision::Erased,
            _ => ReadDecision::NotFound,
        },
    }
}

/// Schreiben ist nur für `Live` zulässig (SPEC 8.1, Z1).
#[must_use]
pub const fn write_allowed(state: GateState) -> bool {
    matches!(state, GateState::Live)
}

#[cfg(test)]
mod tests {
    use super::*;
    use acta_gate::RestrictReason;

    const DP: Access = Access::DataProtection(Purpose::Auskunft);

    fn all_states() -> [GateState; 5] {
        [
            GateState::Live,
            GateState::Restricted(RestrictReason::Retention),
            GateState::Restricted(RestrictReason::MultiSubjectReview),
            GateState::Superseded,
            GateState::Erased,
        ]
    }

    #[test]
    fn z1_gated_or_erased_is_never_readable_for_standard_access() {
        for state in all_states() {
            if matches!(state, GateState::Live) {
                continue;
            }
            let decision = decide_read(state, Access::Standard);
            assert_ne!(decision, ReadDecision::Allowed);
            assert_ne!(decision, ReadDecision::AllowedRestrictedRead);
        }
    }

    #[test]
    fn z1_erased_is_never_opened_even_for_data_protection() {
        assert_eq!(decide_read(GateState::Erased, DP), ReadDecision::Erased);
        assert_ne!(
            decide_read(GateState::Erased, DP),
            ReadDecision::AllowedRestrictedRead
        );
    }

    #[test]
    fn z16_without_access_every_non_live_target_looks_like_missing() {
        for state in all_states() {
            assert_eq!(decide_read(state, Access::None), ReadDecision::NotFound);
        }
    }

    #[test]
    fn restricted_state_is_hidden_from_standard_access() {
        assert_eq!(
            decide_read(
                GateState::Restricted(RestrictReason::Retention),
                Access::Standard
            ),
            ReadDecision::NotFound
        );
        assert_eq!(
            decide_read(GateState::Superseded, Access::Standard),
            ReadDecision::NotFound
        );
    }

    #[test]
    fn restricted_read_opens_restricted_and_superseded_only() {
        assert_eq!(
            decide_read(GateState::Restricted(RestrictReason::LegalHold), DP),
            ReadDecision::AllowedRestrictedRead
        );
        assert_eq!(
            decide_read(GateState::Superseded, DP),
            ReadDecision::AllowedRestrictedRead
        );
    }

    #[test]
    fn live_is_readable_and_writable_and_erased_is_not_writable() {
        assert_eq!(
            decide_read(GateState::Live, Access::Standard),
            ReadDecision::Allowed
        );
        assert!(write_allowed(GateState::Live));
        for state in all_states() {
            if !matches!(state, GateState::Live) {
                assert!(!write_allowed(state));
            }
        }
    }

    #[test]
    fn erased_with_standard_access_reports_erased() {
        assert_eq!(
            decide_read(GateState::Erased, Access::Standard),
            ReadDecision::Erased
        );
    }
}
