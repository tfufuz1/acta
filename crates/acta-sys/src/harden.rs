//! Prozesshärtung (SPEC 13.1): keine Core-Dumps, nicht dumpbar, Sperrlimit lesbar.
//!
//! Fail-closed: schlägt eine Härtung fehl, verweigert der Aufrufer den Start (SPEC 14.1).

use rustix::process::{set_dumpable_behavior, setrlimit, DumpableBehavior, Resource, Rlimit};

use crate::error::SysError;

/// Setzt `RLIMIT_CORE` auf 0 (hart) und `PR_SET_DUMPABLE` auf 0.
///
/// # Errors
/// `SysError::CoreLimit` oder `SysError::Dumpable`, wenn das Betriebssystem die Änderung verweigert.
pub fn harden_process() -> Result<(), SysError> {
    setrlimit(
        Resource::Core,
        Rlimit {
            current: Some(0),
            maximum: Some(0),
        },
    )
    .map_err(|_| SysError::CoreLimit)?;
    set_dumpable_behavior(DumpableBehavior::NotDumpable).map_err(|_| SysError::Dumpable)?;
    Ok(())
}

/// Weiches Limit `RLIMIT_MEMLOCK` in Bytes, falls vom Betriebssystem gesetzt (Selbsttest, SPEC 14.1).
#[must_use]
pub fn memlock_soft_limit() -> Option<u64> {
    rustix::process::getrlimit(Resource::Memlock).current
}
