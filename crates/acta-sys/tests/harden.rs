//! Prüft, dass `harden_process` `RLIMIT_CORE` hart auf 0 setzt (SPEC 13.1).
//! Eigenes Testbinary: die Härtung ist prozessweit.

use acta_sys::harden_process;
use rustix::process::{getrlimit, Resource};

#[test]
fn harden_sets_core_limit_to_zero() {
    harden_process().expect("hardening");
    let lim = getrlimit(Resource::Core);
    assert_eq!(lim.current, Some(0));
    assert_eq!(lim.maximum, Some(0));
}
