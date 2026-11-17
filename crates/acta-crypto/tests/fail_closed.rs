//! Z10 (Spec 16.3): `T-RNG-*` und `T-NOFALLBACK-*`. Kein Fehler im Schlüssel-, Zufalls-, Ableitungs-
//! oder Siegelpfad führt zu Ersatzwert, Fallback-Schlüssel oder Klartext. Fehlermeldungen und
//! Codes tragen keine Werte (Z15).

use acta_base::{ClassifiedError, CryptoRng, EntropyError, ErrorClass};
use acta_crypto::aead::{open, seal, Derived, Knowledge, Personal, SealCtx, Sealed};
use acta_crypto::error::CryptoError;
use acta_crypto::kdf::derive_dek;
use acta_crypto::key::{
    CommitKey, DataKey, DocumentKey, IndexSecret, LogKey, MasterKey, ScopeKey, SegmentKey,
    SubjectKey,
};
use acta_crypto::sign::NotarySigner;
use std::sync::atomic::{AtomicUsize, Ordering};

struct Failing;
impl CryptoRng for Failing {
    fn try_fill(&self, _dst: &mut [u8]) -> Result<(), EntropyError> {
        Err(EntropyError)
    }
}

/// Liefert die ersten `n` Ziehungen, danach Fehler.
struct FailsAfter(usize, AtomicUsize);
impl CryptoRng for FailsAfter {
    fn try_fill(&self, dst: &mut [u8]) -> Result<(), EntropyError> {
        if self.1.fetch_add(1, Ordering::SeqCst) >= self.0 {
            return Err(EntropyError);
        }
        dst.fill(0x33);
        Ok(())
    }
}

fn ctx() -> SealCtx {
    SealCtx {
        tenant: [1; 16],
        keyspace: 1,
        record_ref: [2; 16],
        version: 1,
    }
}

#[test]
fn t_rng_every_generator_fails_closed_with_positive_control() {
    let ok = FailsAfter(usize::MAX, AtomicUsize::new(0));
    macro_rules! gen {
        ($($t:ty),+) => {$(
            assert!(<$t>::generate(&ok).is_ok(), concat!(stringify!($t), " Positivkontrolle"));
            assert!(matches!(<$t>::generate(&Failing), Err(CryptoError::Entropy)), stringify!($t));
        )+};
    }
    gen!(
        DataKey,
        SubjectKey,
        DocumentKey,
        SegmentKey,
        ScopeKey,
        MasterKey,
        IndexSecret,
        LogKey,
        CommitKey
    );
    assert!(NotarySigner::generate(&ok).is_ok());
    assert!(matches!(
        NotarySigner::generate(&Failing),
        Err(CryptoError::Entropy)
    ));
}

#[test]
fn t_rng_seal_fails_closed_on_every_plane() {
    let key = DataKey::from_bytes([5; 32]);
    assert!(matches!(
        seal::<Personal, _>(&key, &ctx(), b"x", &Failing),
        Err(CryptoError::Entropy)
    ));
    assert!(matches!(
        seal::<Knowledge, _>(&key, &ctx(), b"x", &Failing),
        Err(CryptoError::Entropy)
    ));
    assert!(matches!(
        seal::<Derived, _>(&key, &ctx(), b"x", &Failing),
        Err(CryptoError::Entropy)
    ));
    // Ein Fehler nach erfolgreichen Ziehungen erzeugt ebenfalls nichts.
    let rng = FailsAfter(1, AtomicUsize::new(0));
    assert!(seal::<Personal, _>(&key, &ctx(), b"x", &rng).is_ok());
    assert!(seal::<Personal, _>(&key, &ctx(), b"x", &rng).is_err());
}

#[test]
fn t_nofallback_wrong_or_missing_key_never_yields_plaintext() -> Result<(), CryptoError> {
    let rng = FailsAfter(usize::MAX, AtomicUsize::new(0));
    let key = DataKey::from_bytes([5; 32]);
    let sealed: Sealed<Personal> = seal(&key, &ctx(), b"geheim", &rng)?;
    assert_eq!(open(&key, &ctx(), &sealed)?.as_slice(), b"geheim"); // Positivkontrolle

    // Ein vernichteter oder fremder Schlüssel (hier: ein anderer, auch der Nullschlüssel) öffnet nichts.
    for wrong in [DataKey::from_bytes([0; 32]), DataKey::from_bytes([6; 32])] {
        assert!(matches!(
            open(&wrong, &ctx(), &sealed),
            Err(CryptoError::DecryptionFailed)
        ));
    }
    Ok(())
}

#[test]
fn t_nofallback_derivation_errors_are_propagated() {
    let doc = DocumentKey::from_bytes([1; 32]);
    let s = SubjectKey::from_bytes([2; 32]);
    let dup = derive_dek(&doc, &[0; 16], &[([1; 16], &s), ([1; 16], &s)]);
    assert!(matches!(dup, Err(CryptoError::DuplicateSubject)));
    let many: Vec<([u8; 16], &SubjectKey)> = (0..=255_u8)
        .map(|i| {
            let mut t = [0_u8; 16];
            t[0] = i;
            (t, &s)
        })
        .collect();
    assert!(matches!(
        derive_dek(&doc, &[0; 16], &many),
        Err(CryptoError::TooManySubjects)
    ));
}

#[test]
fn errors_carry_no_values_and_have_stable_codes() {
    let all = [
        (CryptoError::Entropy, "crypto.entropy", ErrorClass::Crypto),
        (
            CryptoError::DecryptionFailed,
            "crypto.decryption_failed",
            ErrorClass::Crypto,
        ),
        (
            CryptoError::Malformed,
            "crypto.malformed",
            ErrorClass::Integrity,
        ),
        (
            CryptoError::Unsupported,
            "crypto.unsupported",
            ErrorClass::Invalid,
        ),
        (
            CryptoError::TooLarge,
            "crypto.too_large",
            ErrorClass::Invalid,
        ),
        (
            CryptoError::TooManySubjects,
            "crypto.too_many_subjects",
            ErrorClass::Invalid,
        ),
        (
            CryptoError::DuplicateSubject,
            "crypto.duplicate_subject",
            ErrorClass::Invalid,
        ),
        (CryptoError::Internal, "crypto.internal", ErrorClass::Crypto),
        (
            CryptoError::SignatureInvalid,
            "crypto.signature_invalid",
            ErrorClass::Integrity,
        ),
    ];
    for (err, code, class) in all {
        assert_eq!(err.code(), code);
        assert_eq!(err.class(), class);
        // Keine Hex-Folge, keine Ziffernfolge in Meldung und Debug-Ausgabe.
        for text in [err.to_string(), format!("{err:?}")] {
            assert!(!text.chars().any(|c| c.is_ascii_digit()), "{text}");
        }
    }
}

#[test]
fn crypto_never_reports_erased() {
    // `Erased` entscheidet der Aufrufer (Gate oder Register), nie der Krypto-Kern (4.5).
    let key = DataKey::from_bytes([5; 32]);
    let rng = FailsAfter(usize::MAX, AtomicUsize::new(0));
    let sealed: Result<Sealed<Personal>, _> = seal(&key, &ctx(), b"x", &rng);
    let wrong = DataKey::from_bytes([6; 32]);
    let err = sealed.and_then(|s| open(&wrong, &ctx(), &s));
    assert!(matches!(err, Err(CryptoError::DecryptionFailed)));
}
