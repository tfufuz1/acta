//! Ed25519-Signaturen mit Domänentrennung (Spec 4.1).

use acta_base::{CryptoRng, EntropyError};
use acta_crypto::domain::{CHECKPOINT, REPORT};
use acta_crypto::error::CryptoError;
use acta_crypto::sign::{verify, NotarySigner};

struct FailingRng;
impl CryptoRng for FailingRng {
    fn try_fill(&self, _dst: &mut [u8]) -> Result<(), EntropyError> {
        Err(EntropyError)
    }
}

#[test]
fn signature_verifies_and_binds_domain_and_message() -> Result<(), CryptoError> {
    let signer = NotarySigner::from_seed([7; 32]);
    let pk = signer.public_key();
    let sig = signer.sign(REPORT, b"bericht");
    verify(&pk, REPORT, b"bericht", &sig)?;

    // Eine Signatur gilt nicht in einer anderen Domäne, nicht für eine andere Nachricht.
    assert!(matches!(
        verify(&pk, CHECKPOINT, b"bericht", &sig),
        Err(CryptoError::SignatureInvalid)
    ));
    assert!(matches!(
        verify(&pk, REPORT, b"anders", &sig),
        Err(CryptoError::SignatureInvalid)
    ));

    // Fremder Schlüssel.
    let other = NotarySigner::from_seed([8; 32]).public_key();
    assert!(matches!(
        verify(&other, REPORT, b"bericht", &sig),
        Err(CryptoError::SignatureInvalid)
    ));
    Ok(())
}

#[test]
fn generation_fails_closed_without_entropy() {
    assert!(matches!(
        NotarySigner::generate(&FailingRng),
        Err(CryptoError::Entropy)
    ));
}

// Orakel: bekannte Schwachschlüssel-Konstruktion (Kleinordnungspunkt). Der öffentliche
// Schlüssel ist der Identitätspunkt (kompr. 01 00..00), R ist ebenfalls der Identitätspunkt,
// S = 0. Dann gilt die Gleichung [S]B = R + [k]A für jede Nachricht. Die nicht strikte
// Prüfung akzeptiert das, `verify_strict` lehnt es ab (RFC 8032 lässt beides zu, Acta nur strikt).
// SPEC 4.1 (Ed25519), Z-Bezug: Berichte/Checkpoints (4.7).
#[test]
fn small_order_key_is_rejected_strictly_with_positive_control() -> Result<(), CryptoError> {
    let mut identity = [0_u8; 32];
    identity[0] = 1;
    let mut sig = [0_u8; 64];
    sig[0] = 1; // R = Identität, S = 0

    // Positivkontrolle: die nicht strikte Prüfung der Bibliothek akzeptiert die Fälschung.
    let weak =
        ed25519_dalek::VerifyingKey::from_bytes(&identity).map_err(|_| CryptoError::Malformed)?;
    let message = [REPORT, b"beliebig".as_slice()].concat();
    let lenient = ed25519_dalek::Verifier::verify(
        &weak,
        &message,
        &ed25519_dalek::Signature::from_bytes(&sig),
    );
    assert!(
        lenient.is_ok(),
        "Positivkontrolle: nicht strikte Prüfung akzeptiert"
    );

    // Negativaussage: Acta lehnt sie ab.
    assert!(matches!(
        verify(&identity, REPORT, b"beliebig", &sig),
        Err(CryptoError::SignatureInvalid)
    ));
    Ok(())
}

// Bit-Flip über jede Position der Signatur und der Nachricht (Negativtest, 16.4).
#[test]
fn every_bit_flip_in_signature_is_rejected() -> Result<(), CryptoError> {
    let signer = NotarySigner::from_seed([3; 32]);
    let pk = signer.public_key();
    let sig = signer.sign(CHECKPOINT, b"stand");
    verify(&pk, CHECKPOINT, b"stand", &sig)?; // Positivkontrolle
    for byte in 0..sig.len() {
        for bit in 0..8 {
            let mut bad = sig;
            if let Some(b) = bad.get_mut(byte) {
                *b ^= 1 << bit;
            }
            assert!(
                verify(&pk, CHECKPOINT, b"stand", &bad).is_err(),
                "byte {byte} bit {bit}"
            );
        }
    }
    Ok(())
}
