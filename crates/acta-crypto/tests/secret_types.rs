//! G-CRY und Z10: Geheimnistypen ohne `Clone`/`Debug`/`Default`, mit Zeroisierung beim Verwerfen.
//! Orakel: Merkmalsprüfung zur Übersetzungszeit (Inherent-Const-Vorrang), mit Positivkontrolle.

use acta_crypto::key::{
    CommitKey, DataKey, DedupKey, DocumentKey, IndexSecret, LogKey, MasterKey, SegmentKey,
    SubjectKey,
};
use acta_crypto::sign::NotarySigner;
use std::fmt::Debug;
use std::marker::PhantomData;
use zeroize::ZeroizeOnDrop;

struct Probe<T>(PhantomData<T>);
trait Fallback {
    const CLONE: bool = false;
    const DEBUG: bool = false;
    const DEFAULT: bool = false;
}
impl<T> Fallback for Probe<T> {}
impl<T: Clone> Probe<T> {
    const CLONE: bool = true;
}
impl<T: Debug> Probe<T> {
    const DEBUG: bool = true;
}
impl<T: Default> Probe<T> {
    const DEFAULT: bool = true;
}

macro_rules! forbidden {
    ($($t:ty),+) => {$(
        check(<Probe<$t>>::CLONE, false, concat!(stringify!($t), ": Clone"));
        check(<Probe<$t>>::DEBUG, false, concat!(stringify!($t), ": Debug"));
        check(<Probe<$t>>::DEFAULT, false, concat!(stringify!($t), ": Default"));
    )+};
}

fn check(flag: bool, expected: bool, what: &str) {
    assert_eq!(flag, expected, "{what}");
}

fn wiped_on_drop<T: ZeroizeOnDrop>() {}

#[test]
fn secrets_have_no_clone_debug_default() {
    // Positivkontrolle: die Sonde erkennt die Merkmale an einem Typ, der sie trägt.
    check(<Probe<String>>::CLONE, true, "Positivkontrolle Clone");
    check(<Probe<String>>::DEBUG, true, "Positivkontrolle Debug");
    check(<Probe<String>>::DEFAULT, true, "Positivkontrolle Default");
    forbidden!(
        DataKey,
        SubjectKey,
        DocumentKey,
        SegmentKey,
        MasterKey,
        IndexSecret,
        NotarySigner
    );
}

#[test]
fn secrets_are_zeroized_on_drop() {
    wiped_on_drop::<DataKey>();
    wiped_on_drop::<SubjectKey>();
    wiped_on_drop::<DocumentKey>();
    wiped_on_drop::<SegmentKey>();
    wiped_on_drop::<MasterKey>();
    wiped_on_drop::<IndexSecret>();
    wiped_on_drop::<LogKey>();
    wiped_on_drop::<CommitKey>();
    wiped_on_drop::<DedupKey>();
    wiped_on_drop::<NotarySigner>();
}

// Schlüsseltrennung: MAC-Schlüssel können keinen Umschlag versiegeln (SealKey fehlt),
// Positivkontrolle an LogKey. Orakel: Merkmalssonde zur Übersetzungszeit.
trait NotSeal {
    const SEAL: bool = false;
}
impl<T> NotSeal for Probe<T> {}
impl<T: acta_crypto::aead::SealKey> Probe<T> {
    const SEAL: bool = true;
}

#[test]
fn mac_only_keys_cannot_seal() {
    assert!(
        std::hint::black_box(<Probe<LogKey>>::SEAL),
        "Positivkontrolle"
    );
    assert!(!std::hint::black_box(<Probe<CommitKey>>::SEAL));
    assert!(!std::hint::black_box(<Probe<DedupKey>>::SEAL));
}
