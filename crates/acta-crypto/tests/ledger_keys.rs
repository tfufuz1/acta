//! K_dedup (Blob-Salz, SPEC 2.4/4.2), `CommitKey` (Commitments, SPEC 2.7/4.2) und `LogKey`
//! (Zugriffsprotokoll, SPEC 4.2). Orakel: unabhängige Python-Referenz (RFC 5869 und HMAC aus
//! der Standardbibliothek), Eigenschaften und Negativtests mit Positivkontrolle.

use acta_base::{CryptoRng, EntropyError};
use acta_crypto::aead::{open, seal, Derived, SealCtx, Sealed};
use acta_crypto::error::CryptoError;
use acta_crypto::kdf::{blob_salt, commitment, derive_dedup_key};
use acta_crypto::key::{CommitKey, IndexSecret, LogKey};

struct Fixed(u8);
impl CryptoRng for Fixed {
    fn try_fill(&self, dst: &mut [u8]) -> Result<(), EntropyError> {
        dst.fill(self.0);
        Ok(())
    }
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

fn tenant(n: u8) -> [u8; 16] {
    let mut t = [0_u8; 16];
    for (i, b) in t.iter_mut().enumerate() {
        *b = u8::try_from(i).unwrap_or(0).wrapping_add(n);
    }
    t
}

#[test]
fn blob_salt_matches_python_reference() -> Result<(), CryptoError> {
    let kidx = IndexSecret::from_bytes([0x11; 32]);
    let dedup = derive_dedup_key(&kidx, &tenant(0))?;
    assert_eq!(
        hex(&blob_salt(&dedup, b"inhalt")?),
        "2795650a46b6921f74ee4a1461f8bc8e"
    );
    Ok(())
}

#[test]
fn blob_salt_is_idempotent_and_separated() -> Result<(), CryptoError> {
    let kidx = IndexSecret::from_bytes([0x11; 32]);
    let a = derive_dedup_key(&kidx, &tenant(0))?;
    let a2 = derive_dedup_key(&kidx, &tenant(0))?;
    let b = derive_dedup_key(&kidx, &tenant(1))?;
    let s = blob_salt(&a, b"x")?;
    assert_eq!(
        s,
        blob_salt(&a2, b"x")?,
        "gleicher Inhalt, gleicher Mandant: gleiches Salz"
    );
    assert_ne!(
        s,
        blob_salt(&b, b"x")?,
        "keine mandantenübergreifende Gleichheit"
    );
    assert_ne!(s, blob_salt(&a, b"y")?, "anderer Inhalt, anderes Salz");
    let other = derive_dedup_key(&IndexSecret::from_bytes([0x12; 32]), &tenant(0))?;
    assert_ne!(s, blob_salt(&other, b"x")?, "ohne K_dedup nicht prüfbar");
    Ok(())
}

#[test]
fn commitment_matches_python_reference() -> Result<(), CryptoError> {
    let ck = CommitKey::from_bytes([0x22; 32]);
    assert_eq!(
        hex(&commitment(&ck, &[0x33; 16])?),
        "f7fb8acfe7dd5797e9fc54d3b8e5f55b"
    );
    Ok(())
}

#[test]
fn commitments_separate_epochs_and_key_ids() -> Result<(), CryptoError> {
    let e1 = CommitKey::from_bytes([0x22; 32]);
    let e2 = CommitKey::from_bytes([0x23; 32]);
    let c = commitment(&e1, &[1; 16])?;
    assert_eq!(c, commitment(&e1, &[1; 16])?);
    assert_ne!(c, commitment(&e2, &[1; 16])?, "andere Epoche");
    assert_ne!(c, commitment(&e1, &[2; 16])?, "andere KeyId");
    Ok(())
}

fn ctx(version: u64) -> SealCtx {
    SealCtx {
        tenant: tenant(0),
        keyspace: 9,
        record_ref: [5; 16],
        version,
    }
}

// A-Ledger-Eintrag unter LogKey(m): Roundtrip, Bit-Flip, falscher Monat, Replay auf anderer Position.
#[test]
fn log_key_seals_entries_and_rejects_foreign_use() -> Result<(), CryptoError> {
    let month = LogKey::generate(&Fixed(0x44))?;
    let other_month = LogKey::generate(&Fixed(0x45))?;
    let sealed: Sealed<Derived> = seal(&month, &ctx(1), b"eintrag", &Fixed(7))?;
    assert_eq!(open(&month, &ctx(1), &sealed)?.as_slice(), b"eintrag"); // Positivkontrolle
    assert!(
        open(&other_month, &ctx(1), &sealed).is_err(),
        "Monatsschlüssel vernichtet/fremd"
    );
    assert!(
        open(&month, &ctx(2), &sealed).is_err(),
        "Replay an anderer Position"
    );
    let mut bytes = sealed.as_bytes().to_vec();
    if let Some(b) = bytes.last_mut() {
        *b ^= 1;
    }
    let flipped = Sealed::<Derived>::from_bytes(bytes)?;
    assert!(open(&month, &ctx(1), &flipped).is_err(), "Bit-Flip");
    Ok(())
}
