//! Abgeleitete Werte mit Herkunft (SPEC 5.2, Z13).

use acta_gate::state::Lineage;

/// Ein Wert, der aus einer Herkunft abgeleitet ist.
///
/// Ohne `Lineage` ist keine Konstruktion möglich; die Herkunft ist nicht leer (Z13).
#[derive(Debug, Clone, PartialEq)]
pub struct Derived<T> {
    value: T,
    lineage: Lineage,
}

impl<T> Derived<T> {
    /// Verbindet einen Wert mit seiner Herkunft.
    #[must_use]
    pub const fn new(value: T, lineage: Lineage) -> Self {
        Self { value, lineage }
    }

    /// Abgeleiteter Wert.
    #[must_use]
    pub const fn value(&self) -> &T {
        &self.value
    }

    /// Herkunft des Wertes.
    #[must_use]
    pub const fn lineage(&self) -> &Lineage {
        &self.lineage
    }

    /// Zerlegt in Wert und Herkunft.
    #[must_use]
    pub fn into_parts(self) -> (T, Lineage) {
        (self.value, self.lineage)
    }
}
