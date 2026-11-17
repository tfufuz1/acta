//! Fehlertyp von acta-erasure (HR-4: ein Fehlertyp je Crate, kein `Other(String)`).
//!
//! Meldungen enthalten weder Kennungen noch Schlüsselmaterial (Z15).

use acta_base::{ClassifiedError, ErrorClass};
use acta_gate::GateError;

/// Fehler von acta-erasure.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ErasureError {
    /// Der Übergang ist nach SPEC 8.3 nicht zulässig. Der Zustand bleibt unverändert.
    #[error("saga transition not allowed")]
    IllegalTransition,
    /// Die Saga ist im Endzustand `Reported` und nimmt keine Schritte mehr an.
    #[error("saga is terminal")]
    Terminal,
    /// Lesen ist nicht zulässig. Ohne Zugriffsrecht dasselbe Fehlerbild wie bei einem
    /// nicht vorhandenen Ziel (Z16).
    #[error("target not readable")]
    NotReadable,
    /// Das Ziel ist erledigt oder läuft gelöscht; nur für Nutzer mit Zugriffsrecht (SPEC 8.1).
    #[error("target is in erased state")]
    Erased,
    /// Fehler aus dem Gate (acta-gate).
    #[error(transparent)]
    Gate(#[from] GateError),
}

impl ClassifiedError for ErasureError {
    fn class(&self) -> ErrorClass {
        match self {
            Self::IllegalTransition | Self::Terminal => ErrorClass::Invalid,
            Self::NotReadable | Self::Erased => ErrorClass::Erased,
            Self::Gate(inner) => inner.class(),
        }
    }

    fn code(&self) -> &'static str {
        match self {
            Self::IllegalTransition => "erasure.transition",
            Self::Terminal => "erasure.terminal",
            Self::NotReadable => "erasure.not_readable",
            Self::Erased => "erasure.erased",
            Self::Gate(inner) => inner.code(),
        }
    }
}
