//! Funktionstests für `SecureBuf` und die Ports (SPEC 4.6, Z10).
//! Orakel: Nullung nach Erzeugung, Länge fest, Entropiefehler liefert Err statt Ersatzwert.

use acta_base::{Clock, CryptoRng};
use acta_sys::{OsCryptoRng, SecureBuf, SysError};
use acta_testkit::{FailingRng, SeededRng};

#[test]
fn zeroed_buffer_is_all_zero_and_fixed_length() {
    let b = SecureBuf::zeroed(64).expect("mlock");
    assert_eq!(b.len(), 64);
    assert!(!b.is_empty());
    assert!(b.as_slice().iter().all(|&x| x == 0));
}

#[test]
fn from_slice_copies_content() {
    let src = [7_u8, 8, 9, 10];
    let b = SecureBuf::from_slice(&src).expect("mlock");
    assert_eq!(b.as_slice(), &src);
}

#[test]
fn mutation_is_visible_through_as_slice() {
    let mut b = SecureBuf::zeroed(4).expect("mlock");
    b.as_mut_slice().copy_from_slice(&[1, 2, 3, 4]);
    assert_eq!(b.as_slice(), &[1, 2, 3, 4]);
}

#[test]
fn zero_length_buffer_needs_no_lock() {
    let b = SecureBuf::zeroed(0).expect("no lock needed");
    assert!(b.is_empty());
    assert_eq!(b.len(), 0);
}

#[test]
fn random_fills_from_rng() {
    let a = SecureBuf::random(32, &SeededRng::new(42)).expect("mlock");
    let b = SecureBuf::random(32, &SeededRng::new(42)).expect("mlock");
    assert_eq!(a.as_slice(), b.as_slice());
    assert!(a.as_slice().iter().any(|&x| x != 0));
}

#[test]
fn entropy_failure_yields_error_not_buffer() {
    let r = SecureBuf::random(32, &FailingRng);
    assert!(matches!(r, Err(SysError::Entropy(_))));
}

#[test]
fn os_rng_fills_nonzero_and_is_fallible_api() {
    let mut out = [0_u8; 32];
    OsCryptoRng.try_fill(&mut out).expect("os entropy");
    assert!(out.iter().any(|&x| x != 0));
}

#[test]
fn system_clock_is_after_2020() {
    use acta_sys::SystemClock;
    // 2020-01-01 in ms; Orakel ist die Kalenderkonstante, nicht der Code.
    assert!(SystemClock.now_ms() > 1_577_836_800_000);
}
