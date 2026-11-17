//! Gültigkeitszeit `[valid_from, valid_to)` (SPEC 6.3) und der Filter `valid_at` im Abruf (SPEC 9.4).
//!
//! Tage sind Kalendertage seit 1970-01-01 als `i32`. Ein fehlender Wert ist `None` (Präsenzbit
//! nicht gesetzt), nie `0`.

use crate::error::RagError;

/// Gültigkeitsspanne eines Eintrags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValidityWindow {
    valid_from: Option<i32>,
    valid_to: Option<i32>,
}

impl ValidityWindow {
    /// Unbegrenzte Gültigkeit.
    pub const UNBOUNDED: Self = Self {
        valid_from: None,
        valid_to: None,
    };

    /// Erstellt eine Spanne. Sind beide Grenzen gesetzt, muss `valid_from < valid_to` gelten.
    ///
    /// # Errors
    /// `EmptyValidityWindow`, wenn die Spanne leer oder umgekehrt ist.
    pub fn new(valid_from: Option<i32>, valid_to: Option<i32>) -> Result<Self, RagError> {
        if let (Some(from), Some(to)) = (valid_from, valid_to) {
            if from >= to {
                return Err(RagError::EmptyValidityWindow);
            }
        }
        Ok(Self {
            valid_from,
            valid_to,
        })
    }

    /// Beginn der Gültigkeit (einschließlich), falls gesetzt.
    #[must_use]
    pub const fn valid_from(&self) -> Option<i32> {
        self.valid_from
    }

    /// Ende der Gültigkeit (ausschließlich), falls gesetzt.
    #[must_use]
    pub const fn valid_to(&self) -> Option<i32> {
        self.valid_to
    }

    /// Ob der Tag `day` in der Spanne liegt.
    #[must_use]
    pub fn contains(&self, day: i32) -> bool {
        self.valid_from.is_none_or(|from| day >= from) && self.valid_to.is_none_or(|to| day < to)
    }
}

/// Entfernt alle Einträge, deren Gültigkeitsspanne den Tag `day` nicht enthält. Die Reihenfolge
/// bleibt erhalten.
pub fn retain_valid_at<T>(items: &mut Vec<T>, day: i32, window_of: impl Fn(&T) -> ValidityWindow) {
    items.retain(|item| window_of(item).contains(day));
}

#[cfg(test)]
mod tests {
    use super::*;

    // SPEC: 6.3 (Halboffenes Intervall)
    #[test]
    fn half_open_interval_boundaries() {
        let w = ValidityWindow::new(Some(10), Some(20)).unwrap();
        assert!(!w.contains(9));
        assert!(w.contains(10));
        assert!(w.contains(19));
        assert!(!w.contains(20));
    }

    #[test]
    fn missing_bounds_are_open() {
        let from_only = ValidityWindow::new(Some(5), None).unwrap();
        assert!(from_only.contains(i32::MAX));
        assert!(!from_only.contains(4));
        let to_only = ValidityWindow::new(None, Some(5)).unwrap();
        assert!(to_only.contains(i32::MIN));
        assert!(!to_only.contains(5));
        assert!(ValidityWindow::UNBOUNDED.contains(0));
    }

    // SPEC: 6.3 (`kein Wert` ist über das Präsenzbit definiert, nicht über 0)
    #[test]
    fn zero_is_a_real_day_not_absence() {
        let w = ValidityWindow::new(Some(0), None).unwrap();
        assert_eq!(w.valid_from(), Some(0));
        assert!(w.contains(0));
        assert!(!w.contains(-1));
    }

    #[test]
    fn empty_or_reversed_window_is_rejected() {
        assert_eq!(
            ValidityWindow::new(Some(5), Some(5)),
            Err(RagError::EmptyValidityWindow)
        );
        assert_eq!(
            ValidityWindow::new(Some(6), Some(5)),
            Err(RagError::EmptyValidityWindow)
        );
    }

    #[test]
    fn retain_keeps_order_and_drops_out_of_window_items() {
        let mut items = vec![
            ("alt", ValidityWindow::new(Some(0), Some(10)).unwrap()),
            ("neu", ValidityWindow::new(Some(10), None).unwrap()),
            ("ohne", ValidityWindow::UNBOUNDED),
        ];
        retain_valid_at(&mut items, 12, |(_, w)| *w);
        let names: Vec<&str> = items.iter().map(|(n, _)| *n).collect();
        assert_eq!(names, vec!["neu", "ohne"]);
    }
}
