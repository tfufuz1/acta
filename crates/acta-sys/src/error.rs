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
    /// Aktiver Swap ohne deklarierte Verschlüsselung (SPEC 13.1).
    #[error("active swap without declared encryption")]
    HostSwapActive,
    /// Core-Dumps werden an ein Programm weitergeleitet (`core_pattern` beginnt mit `|`).
    #[error("core dumps are piped to a program")]
    HostCorePipe,
    /// `kernel.yama.ptrace_scope` ist 0 (SPEC 13.1 verlangt mindestens 1).
    #[error("ptrace scope is below the required minimum")]
    HostPtraceScope,
    /// Das weiche Limit `RLIMIT_MEMLOCK` reicht nicht für den Schlüsselspeicher.
    #[error("memlock soft limit is below the required amount")]
    HostMemlockLimit,
    /// Eingabe eines Hostprüfungswertes ist nicht lesbar oder nicht erwartbar; fail-closed.
    #[error("host probe value is unreadable or malformed")]
    HostProbeInvalid,
}

impl ClassifiedError for SysError {
    fn class(&self) -> ErrorClass {
        match self {
            Self::Map
            | Self::Mlock
            | Self::DontDump
            | Self::CoreLimit
            | Self::Dumpable
            | Self::HostSwapActive
            | Self::HostCorePipe
            | Self::HostPtraceScope
            | Self::HostMemlockLimit => ErrorClass::Policy,
            Self::Entropy(_) => ErrorClass::Crypto,
            Self::HostProbeInvalid => ErrorClass::Invalid,
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
            Self::HostSwapActive => "sys.host.swap",
            Self::HostCorePipe => "sys.host.core_pipe",
            Self::HostPtraceScope => "sys.host.ptrace_scope",
            Self::HostMemlockLimit => "sys.host.memlock",
            Self::HostProbeInvalid => "sys.host.invalid",
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
            (
                SysError::HostSwapActive,
                "sys.host.swap",
                ErrorClass::Policy,
            ),
            (
                SysError::HostCorePipe,
                "sys.host.core_pipe",
                ErrorClass::Policy,
            ),
            (
                SysError::HostPtraceScope,
                "sys.host.ptrace_scope",
                ErrorClass::Policy,
            ),
            (
                SysError::HostMemlockLimit,
                "sys.host.memlock",
                ErrorClass::Policy,
            ),
            (
                SysError::HostProbeInvalid,
                "sys.host.invalid",
                ErrorClass::Invalid,
            ),
        ];
        for (e, code, class) in cases {
            assert_eq!(e.code(), code);
            assert_eq!(e.class(), class);
        }
    }
}
