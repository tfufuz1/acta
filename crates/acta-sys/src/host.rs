//! Hostprüfungen nach SPEC 13.1 und 14.1 als reine Auswertung.
//!
//! Die Funktionen nehmen den Inhalt der Prüfwerte entgegen (z. B. von `/proc/swaps`). Das Lesen
//! der Dateien ist nicht Aufgabe dieses Crates: nach G-FS erfolgt es über `acta-fs`, verdrahtet
//! im Knoten (L7). Jede Auswertung ist fail-closed: unlesbare oder unerwartete Eingaben
//! liefern `HostProbeInvalid`, nie einen Standardwert.

use crate::error::SysError;

/// Prüft `/proc/swaps`. Ohne Einträge ist Swap deaktiviert. Aktiver Swap wird nur akzeptiert,
/// wenn der Betreiber die Verschlüsselung ausdrücklich erklärt hat (`encrypted_declared`).
///
/// # Errors
/// `HostProbeInvalid` bei fehlender Kopfzeile oder leerer Eingabe; `HostSwapActive` bei aktivem
/// Swap ohne Deklaration.
pub fn check_swap(proc_swaps: &str, encrypted_declared: bool) -> Result<(), SysError> {
    let mut lines = proc_swaps.lines();
    let header = lines.next().ok_or(SysError::HostProbeInvalid)?;
    if !header.starts_with("Filename") {
        return Err(SysError::HostProbeInvalid);
    }
    let active = lines.any(|l| !l.trim().is_empty());
    if active && !encrypted_declared {
        return Err(SysError::HostSwapActive);
    }
    Ok(())
}

/// Prüft `kernel.core_pattern`. Ein Muster mit führendem `|` leitet Core-Dumps an ein Programm
/// weiter und wird verweigert.
///
/// # Errors
/// `HostCorePipe` bei Weiterleitung, `HostProbeInvalid` bei leerer Eingabe.
pub fn check_core_pattern(pattern: &str) -> Result<(), SysError> {
    let trimmed = pattern.trim();
    if trimmed.is_empty() {
        return Err(SysError::HostProbeInvalid);
    }
    if trimmed.starts_with('|') {
        return Err(SysError::HostCorePipe);
    }
    Ok(())
}

/// Prüft `kernel.yama.ptrace_scope` (SPEC 13.1: mindestens 1).
///
/// # Errors
/// `HostPtraceScope` bei Wert 0, `HostProbeInvalid` bei nicht lesbarem Wert.
pub fn check_ptrace_scope(value: &str) -> Result<(), SysError> {
    let scope: u8 = value
        .trim()
        .parse()
        .map_err(|_| SysError::HostProbeInvalid)?;
    if scope < 1 {
        return Err(SysError::HostPtraceScope);
    }
    Ok(())
}

/// Prüft, ob das weiche `RLIMIT_MEMLOCK` (`None` = unbegrenzt) mindestens `required_bytes` erlaubt.
///
/// # Errors
/// `HostMemlockLimit`, wenn das Limit kleiner als `required_bytes` ist.
pub fn check_memlock(soft_limit: Option<u64>, required_bytes: u64) -> Result<(), SysError> {
    match soft_limit {
        None => Ok(()),
        Some(v) if v >= required_bytes => Ok(()),
        Some(_) => Err(SysError::HostMemlockLimit),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SWAPS_EMPTY: &str = "Filename\t\t\t\tType\t\tSize\t\tUsed\t\tPriority\n";
    const SWAPS_ACTIVE: &str =
        "Filename\t\t\t\tType\t\tSize\t\tUsed\t\tPriority\n/dev/sda2 partition 2097148 0 -2\n";

    #[test]
    fn swap_disabled_is_accepted() {
        assert_eq!(check_swap(SWAPS_EMPTY, false), Ok(()));
    }

    #[test]
    fn active_swap_is_refused_unless_declared_encrypted() {
        assert_eq!(
            check_swap(SWAPS_ACTIVE, false),
            Err(SysError::HostSwapActive)
        );
        assert_eq!(check_swap(SWAPS_ACTIVE, true), Ok(()));
    }

    #[test]
    fn swap_probe_without_header_is_invalid() {
        assert_eq!(check_swap("", false), Err(SysError::HostProbeInvalid));
        assert_eq!(
            check_swap("garbage\n", false),
            Err(SysError::HostProbeInvalid)
        );
    }

    #[test]
    fn core_pipe_is_refused() {
        assert_eq!(check_core_pattern("core\n"), Ok(()));
        assert_eq!(
            check_core_pattern("|/usr/share/apport/apport %p"),
            Err(SysError::HostCorePipe)
        );
        assert_eq!(check_core_pattern("  "), Err(SysError::HostProbeInvalid));
    }

    #[test]
    fn ptrace_scope_needs_at_least_one() {
        assert_eq!(check_ptrace_scope("1\n"), Ok(()));
        assert_eq!(check_ptrace_scope("2"), Ok(()));
        assert_eq!(check_ptrace_scope("0\n"), Err(SysError::HostPtraceScope));
        assert_eq!(check_ptrace_scope("x"), Err(SysError::HostProbeInvalid));
        assert_eq!(check_ptrace_scope("300"), Err(SysError::HostProbeInvalid));
    }

    #[test]
    fn memlock_limit_must_cover_requirement() {
        assert_eq!(check_memlock(None, 1 << 30), Ok(()));
        assert_eq!(check_memlock(Some(1 << 20), 1 << 20), Ok(()));
        assert_eq!(
            check_memlock(Some(65_536), 1 << 20),
            Err(SysError::HostMemlockLimit)
        );
    }
}
