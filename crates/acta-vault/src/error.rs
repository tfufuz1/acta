//! Fehlertyp von acta-vault (HR-4: ein Fehlertyp je Crate, kein `Other(String)`).
//!
//! Meldungen enthalten weder Schlüsselmaterial noch Kennungen (Z15).

use acta_base::{ClassifiedError, ErrorClass};

/// Fehler von acta-vault.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum VaultError {
    /// Ein Tresor-Eintrag hat eine unerwartete Länge oder einen unbekannten Wert. Das ist ein
    /// Integritätsfehler (Vergiftung, SPEC 15.2), kein Ersatzwert (Z10).
    #[error("vault entry malformed")]
    Malformed,
}

impl ClassifiedError for VaultError {
    fn class(&self) -> ErrorClass {
        match self {
            Self::Malformed => ErrorClass::Integrity,
        }
    }

    fn code(&self) -> &'static str {
        match self {
            Self::Malformed => "vault.malformed",
        }
    }
}
