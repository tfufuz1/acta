#![allow(clippy::expect_used, clippy::unwrap_used)]
//! Positivkontrolle für `mlock` (SPEC 16.4, Z11): Die Sperre muss im Betriebssystem
//! sichtbar sein (VmLck wächst), sonst wäre jeder Löschnachweis über Speicher ein Scheinbeweis.
//! Eigenes Testbinary, damit parallele Tests `VmLck` nicht verfälschen.

use acta_sys::SecureBuf;

fn vm_lck_kib() -> u64 {
    let status = std::fs::read_to_string("/proc/self/status").expect("procfs");
    status
        .lines()
        .find(|l| l.starts_with("VmLck:"))
        .and_then(|l| l.split_whitespace().nth(1))
        .and_then(|v| v.parse().ok())
        .expect("VmLck line")
}

#[test]
fn locked_buffer_is_visible_in_vmlck_and_released_on_drop() {
    let before = vm_lck_kib();
    let buf = SecureBuf::zeroed(64 * 1024).expect("mlock");
    let during = vm_lck_kib();
    assert!(
        during >= before + 60,
        "Positivkontrolle: VmLck muss um ~64 KiB wachsen (vorher {before}, danach {during})"
    );
    drop(buf);
    let after = vm_lck_kib();
    assert!(
        after + 60 <= during,
        "Entsperren nach Drop fehlt (während {during}, danach {after})"
    );
}
