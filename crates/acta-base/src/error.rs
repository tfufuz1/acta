//! Fehlerarchitektur (Spec 3.5) und Fehlertyp von acta-base (HR-4).

/// Fehlerklasse. `Integrity` und `Crypto` loesen Vergiftung aus (15.2).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ErrorClass {
    /// Wiederholung mit Backoff.
    Transient,
    /// Integritaetsfehler (Vergiftung).
    Integrity,
    /// Kryptofehler (Vergiftung).
    Crypto,
    /// Nur aus Gate oder Registerbeleg, nie aus Abwesenheit eines Schluessels (Z10).
    Erased,
    /// Ablehnung (Rolle, Zugriff, Freigabe).
    Policy,
    /// Ablehnung der Eingabe; nichts wird geschrieben.
    Invalid,
}

/// Bereich einer Vergiftung (SPEC 15.2), aufsteigend nach Reichweite.
///
/// Dieser Crate speichert keinen Vergiftungszustand; den fuehrt `acta-node`. Vergiftung
/// schliesst den kleinsten nachweislich isolierenden Bereich.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum PoisonScope {
    /// Ein Umschlag mit Authentisierungsfehler bei vorhandenem Schluessel.
    Document,
    /// Kopf-MAC oder Pruefsumme eines Segments.
    Segment,
    /// Tresor, Manifest, Gate-Log, Ledger-Kette, Senkenwiderspruch, `FsyncFailed` ohne Ersatz.
    Tenant,
    /// Anker, Uhr, Zufall, Ordinalueberlauf.
    Node,
}

/// Klassifizierbarer Fehler. `Display` und `code()` enthalten keine Personendaten (Z15).
pub trait ClassifiedError: std::error::Error {
    /// Fehlerklasse.
    fn class(&self) -> ErrorClass;
    /// Stabiler Code ohne Nutzdaten und Kennungen.
    fn code(&self) -> &'static str;
}

/// Fehler von acta-base (ein Fehlertyp je Crate, kein `Other(String)`).
///
/// Alle Varianten sind Eingabefehler der kanonischen Kodierung. Ob ein solcher Fehler beim
/// Aufrufer einen Integritaetsfehler bedeutet (etwa beim Lesen einer Platte), entscheidet der
/// Aufrufer; die Klasse hier ist `Invalid`. Die Texte enthalten weder Werte noch Laengen (Z15).
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum BaseError {
    /// Weniger Bytes vorhanden als das Feld verlangt.
    #[error("canonical input truncated")]
    Truncated,
    /// Laengenfeld groesser als die zulaessige Obergrenze oder als der Rest der Eingabe.
    #[error("canonical length out of bounds")]
    LengthOutOfBounds,
    /// Nach dem letzten erwarteten Feld sind Bytes uebrig.
    #[error("canonical input has trailing bytes")]
    TrailingBytes,
    /// Wert liegt ausserhalb des erlaubten Bereichs.
    #[error("value out of range")]
    OutOfRange,
}

impl ClassifiedError for BaseError {
    fn class(&self) -> ErrorClass {
        ErrorClass::Invalid
    }

    fn code(&self) -> &'static str {
        match self {
            Self::Truncated => "base.canon.truncated",
            Self::LengthOutOfBounds => "base.canon.length",
            Self::TrailingBytes => "base.canon.trailing",
            Self::OutOfRange => "base.range",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // SPEC: 3.5 (Fehlerarchitektur), Z15
    #[test]
    fn codes_are_stable_and_class_is_invalid() {
        let all = [
            (BaseError::Truncated, "base.canon.truncated"),
            (BaseError::LengthOutOfBounds, "base.canon.length"),
            (BaseError::TrailingBytes, "base.canon.trailing"),
            (BaseError::OutOfRange, "base.range"),
        ];
        for (e, code) in all {
            assert_eq!(e.code(), code);
            assert_eq!(e.class(), ErrorClass::Invalid);
        }
    }

    // SPEC: 15.2, Z15
    #[test]
    fn display_texts_carry_no_values_and_codes_are_distinct() {
        let all = [
            BaseError::Truncated,
            BaseError::LengthOutOfBounds,
            BaseError::TrailingBytes,
            BaseError::OutOfRange,
        ];
        for (i, a) in all.iter().enumerate() {
            assert!(!a.to_string().chars().any(|c| c.is_ascii_digit()));
            for b in all.iter().skip(i + 1) {
                assert_ne!(a.code(), b.code());
                assert_ne!(a.to_string(), b.to_string());
            }
        }
    }

    // SPEC: 15.2 (Reichweite waechst von Document bis Node)
    #[test]
    fn poison_scope_is_ordered_by_reach() {
        assert!(PoisonScope::Document < PoisonScope::Segment);
        assert!(PoisonScope::Segment < PoisonScope::Tenant);
        assert!(PoisonScope::Tenant < PoisonScope::Node);
        assert_ne!(PoisonScope::Document, PoisonScope::Node);
    }
}
