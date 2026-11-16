//! Fehlertyp von acta-fs (HR-4: ein Fehlertyp je Crate, `#[non_exhaustive]`).
//!
//! Andere Fehler melden die Ports als `std::io::Error` (Spec 3.7). Nur das
//! Fsync-Versagen braucht eine eigene Klasse, weil es die Datei endgültig entwertet.

use acta_base::{ClassifiedError, ErrorClass};

/// `fsync` der Datei ist fehlgeschlagen.
///
/// Nach diesem Fehler wird dieselbe Datei nie wieder als stabil angenommen (Spec 3.7,
/// WP-7). Der Aufrufer schreibt den Inhalt aus Journal oder Speicher in eine neue
/// Datei. Scheitert auch das, wird der betroffene Bereich vergiftet (Spec 15.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("fsync failed; the file is no longer treated as stable")]
#[non_exhaustive]
pub struct FsyncFailed;

impl ClassifiedError for FsyncFailed {
    fn class(&self) -> ErrorClass {
        ErrorClass::Integrity
    }

    fn code(&self) -> &'static str {
        "fs.fsync_failed"
    }
}

/// Fehler der zusammengesetzten Dateioperationen dieses Crates (`atomic`).
///
/// Die Ports `Vfs` und `VFile` melden `io::Error` und [`FsyncFailed`], so schreibt es
/// Spec 3.7 vor; dieser Typ gehört den Operationen darüber.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum FsError {
    /// E/A-Fehler ohne Fsync-Versagen. Der Zielpfad hat entweder den alten oder den vollen
    /// neuen Inhalt; ein erneuter Aufruf ist zulässig.
    #[error("file operation failed")]
    Io(#[source] std::io::Error),
    /// `FsyncFailed` auch auf der Ersatzdatei: kein Ersatz mehr, der Bereich ist zu
    /// vergiften (Spec 15.2, Tenant). Der Zielpfad behält den alten Inhalt.
    #[error("fsync failed without replacement; area must be poisoned")]
    Poisoned,
}

impl ClassifiedError for FsError {
    fn class(&self) -> ErrorClass {
        match self {
            Self::Io(_) => ErrorClass::Transient,
            Self::Poisoned => ErrorClass::Integrity,
        }
    }

    fn code(&self) -> &'static str {
        match self {
            Self::Io(_) => "fs.io",
            Self::Poisoned => "fs.poisoned",
        }
    }
}
