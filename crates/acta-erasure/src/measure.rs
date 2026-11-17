//! Messungen mit Positivkontrolle (SPEC 8.5, Z11).
//!
//! Ein Nachweis entsteht nur, wenn das Messverfahren zuvor an einem synthetischen Treffer
//! (Canary) bewiesen hat, dass es funktioniert. Trifft es den Canary nicht, ist der Lauf ungültig
//! und zählt nie als bestanden.

/// Ergebnis einer Prüfung oder einer Positivkontrolle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Check {
    /// Prüfung bestanden bzw. Prüfung wie erwartet.
    Passed,
    /// Prüfung nicht bestanden.
    Failed,
    /// Nicht gemessen oder Lauf ungültig.
    NotMeasured,
}

/// Messung aus Positivkontrolle und eigentlichem Ergebnis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Measurement {
    control: Check,
    result: Check,
}

impl Measurement {
    /// Erzeugt eine Messung.
    #[must_use]
    pub const fn new(positive_control: Check, result: Check) -> Self {
        Self {
            control: positive_control,
            result,
        }
    }

    /// Gültiges Ergebnis nach Z11: Nur eine bestandene Positivkontrolle macht es gültig. In allen
    /// anderen Fällen ist der Lauf ungültig und liefert `NotMeasured`, nie `Passed`.
    #[must_use]
    pub const fn verdict(self) -> Check {
        match self.control {
            Check::Passed => self.result,
            Check::Failed | Check::NotMeasured => Check::NotMeasured,
        }
    }

    /// `true` nur bei gültiger und bestandener Messung.
    #[must_use]
    pub const fn is_passed(self) -> bool {
        matches!(self.verdict(), Check::Passed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn z11_passed_control_passes_passed_result() {
        assert_eq!(
            Measurement::new(Check::Passed, Check::Passed).verdict(),
            Check::Passed
        );
        assert!(Measurement::new(Check::Passed, Check::Passed).is_passed());
    }

    #[test]
    fn z11_failed_control_never_yields_passed() {
        // Positivkontrolle: der Canary wird nicht erkannt. Auch ein "bestandenes" Ergebnis
        // ist dann wertlos.
        let m = Measurement::new(Check::Failed, Check::Passed);
        assert_eq!(m.verdict(), Check::NotMeasured);
        assert!(!m.is_passed());
    }

    #[test]
    fn z11_missing_control_never_yields_passed() {
        let m = Measurement::new(Check::NotMeasured, Check::Passed);
        assert_eq!(m.verdict(), Check::NotMeasured);
        assert!(!m.is_passed());
    }

    #[test]
    fn failed_result_with_valid_control_stays_failed() {
        let m = Measurement::new(Check::Passed, Check::Failed);
        assert_eq!(m.verdict(), Check::Failed);
        assert!(!m.is_passed());
    }
}
