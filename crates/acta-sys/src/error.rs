//! Fehlertyp von acta-sys (HR-4: ein Fehlertyp je Crate, kein `Other(String)`).
//!
//! `Display` und `code()` enthalten keine Werte, Längen oder Kennungen (Z15).

use acta_base::{ClassifiedError, EntropyError, ErrorClass};

/// Fehler der Systemschicht. Jede Variante ist fail-closed: der Aufrufer erhält keinen
/// Ersatzwert, keinen ungeschützten Puffer und keinen Fallback (Z10).
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[non_exhaustive]
pub enum SysError {
    /// Die anonyme Speicherabbildung für einen Puffer ist fehlgeschlagen.
    #[error("memory mapping refused by the operating system")]
    Map,
    /// `mlock` wurde vom Betriebssystem verweigert (z. B. `RLIMIT_MEMLOCK`).
    #[error("memory lock refused by the operating system")]
    Mlock,
    /// `madvise(MADV_DONTDUMP)` ist fehlgeschlagen.
    #[error("core dump exclusion refused by the operating system")]
    DontDump,
    /// `RLIMIT_CORE` konnte nicht auf 0 gesetzt werden.
    #[error("core dump limit could not be set")]
    CoreLimit,
    /// `PR_SET_DUMPABLE` konnte nicht auf 0 gesetzt werden.
    #[error("dumpable flag could not be cleared")]
    Dumpable,
    /// Die Entropiequelle hat versagt; es entsteht kein Puffer mit Zufall.
    #[error("entropy source failed")]
    Entropy(#[source] EntropyError),
}

impl ClassifiedError for SysError {
    fn class(&self) -> ErrorClass {
        match self {
            Self::Map | Self::Mlock | Self::DontDump | Self::CoreLimit | Self::Dumpable => {
                ErrorClass::Policy
            }
            Self::Entropy(_) => ErrorClass::Crypto,
        }
    }

    fn code(&self) -> &'static str {
        match self {
            Self::Map => "sys.map",
            Self::Mlock => "sys.mlock",
            Self::DontDump => "sys.dontdump",
            Self::CoreLimit => "sys.core_limit",
            Self::Dumpable => "sys.dumpable",
            Self::Entropy(_) => "sys.entropy",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codes_are_stable_and_classes_match_spec() {
        let cases = [
            (SysError::Map, "sys.map", ErrorClass::Policy),
            (SysError::Mlock, "sys.mlock", ErrorClass::Policy),
            (SysError::DontDump, "sys.dontdump", ErrorClass::Policy),
            (SysError::CoreLimit, "sys.core_limit", ErrorClass::Policy),
            (SysError::Dumpable, "sys.dumpable", ErrorClass::Policy),
            (
                SysError::Entropy(EntropyError),
                "sys.entropy",
                ErrorClass::Crypto,
            ),
        ];
        for (e, code, class) in cases {
            assert_eq!(e.code(), code);
            assert_eq!(e.class(), class);
        }
    }
}
