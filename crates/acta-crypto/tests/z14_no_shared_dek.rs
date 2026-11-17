//! Z14 (Spec 4.2, 16.2): Zwei Versionen teilen nie denselben DEK (`no_shared_dek`).
//! Der DEK gibt kein Material frei. Als Beobachtung dient ein Umschlag mit fester Nonce:
//! gleiche Chiffrate bei gleichem Klartext und Kontext bedeuten gleichen DEK.

use acta_base::{CryptoRng, EntropyError};
use acta_crypto::aead::{seal, Personal, SealCtx, Sealed};
use acta_crypto::kdf::{derive_dek, hkdf_sha256, SubjectTag};
use acta_crypto::key::{DataKey, DocumentKey, SubjectKey};
use proptest::prelude::*;

struct Fixed;
impl CryptoRng for Fixed {
    fn try_fill(&self, dst: &mut [u8]) -> Result<(), EntropyError> {
        dst.fill(0x5a);
        Ok(())
    }
}

fn fingerprint(key: &DataKey) -> Vec<u8> {
    let ctx = SealCtx {
        tenant: [0; 16],
        keyspace: 1,
        record_ref: [0; 16],
        version: 0,
    };
    let sealed: Result<Sealed<Personal>, _> = seal(key, &ctx, b"fingerprint", &Fixed);
    sealed.map(|s| s.as_bytes().to_vec()).unwrap_or_default()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Input {
    doc: [u8; 32],
    ver: [u8; 16],
    subjects: Vec<(SubjectTag, [u8; 32])>,
}

fn dek_of(i: &Input) -> Result<Vec<u8>, TestCaseError> {
    let doc = DocumentKey::from_bytes(i.doc);
    let keys: Vec<SubjectKey> = i
        .subjects
        .iter()
        .map(|(_, k)| SubjectKey::from_bytes(*k))
        .collect();
    let refs: Vec<(SubjectTag, &SubjectKey)> = i
        .subjects
        .iter()
        .zip(keys.iter())
        .map(|((t, _), k)| (*t, k))
        .collect();
    let dek = derive_dek(&doc, &i.ver, &refs).map_err(|e| TestCaseError::fail(e.to_string()))?;
    Ok(fingerprint(&dek))
}

fn arb_input() -> impl Strategy<Value = Input> {
    (
        any::<[u8; 32]>(),
        any::<[u8; 16]>(),
        proptest::collection::btree_map(any::<[u8; 16]>(), any::<[u8; 32]>(), 0..4),
    )
        .prop_map(|(doc, ver, m)| Input {
            doc,
            ver,
            subjects: m.into_iter().collect(),
        })
}

proptest! {
    /// Verschiedene Eingaben (VerRef, DocumentKey oder Betroffenenmenge) ergeben verschiedene DEKs;
    /// gleiche Eingaben denselben.
    #[test]
    fn no_shared_dek(a in arb_input(), b in arb_input()) {
        let (da, db) = (dek_of(&a)?, dek_of(&b)?);
        prop_assert!(!da.is_empty());
        if a == b {
            prop_assert_eq!(da, db);
        } else {
            prop_assert_ne!(da, db);
        }
    }

    /// Nur die VerRef ändert sich: neuer DEK (Berichtigung erzeugt eine neue Version).
    #[test]
    fn new_version_ref_gives_new_dek(a in arb_input(), ver in any::<[u8; 16]>()) {
        prop_assume!(ver != a.ver);
        let mut b = a.clone();
        b.ver = ver;
        prop_assert_ne!(dek_of(&a)?, dek_of(&b)?);
    }
}

/// Positivkontrolle (16.4): Eine fehlerhafte Ableitung ohne VerRef-Salz würde Versionen
/// zusammenlegen, und genau diese Beobachtung schlägt an.
#[test]
fn positive_control_detects_a_derivation_without_ver_ref() -> Result<(), String> {
    let doc = [7_u8; 32];
    let broken = |_ver: &[u8; 16]| hkdf_sha256(&[&doc], &[], b"acta/dek/v1\x00");
    let a = broken(&[1; 16]).map_err(|e| e.to_string())?;
    let b = broken(&[2; 16]).map_err(|e| e.to_string())?;
    assert_eq!(
        a.as_slice(),
        b.as_slice(),
        "die kaputte Ableitung teilt DEKs"
    );

    let (x, y) = (
        Input {
            doc,
            ver: [1; 16],
            subjects: vec![],
        },
        Input {
            doc,
            ver: [2; 16],
            subjects: vec![],
        },
    );
    let (dx, dy) = (
        dek_of(&x).map_err(|e| e.to_string())?,
        dek_of(&y).map_err(|e| e.to_string())?,
    );
    assert_ne!(dx, dy, "die echte Ableitung trennt Versionen");
    Ok(())
}

/// Grenze der Betroffenenzahl (255 gültig, 256 abgelehnt). Tötet die Mutante `>` → `>=` in `derive_dek`.
#[test]
fn subject_count_boundary_is_255() {
    use acta_crypto::error::CryptoError;
    let doc = DocumentKey::from_bytes([1; 32]);
    let s = SubjectKey::from_bytes([2; 32]);
    let make = |n: u16| -> Vec<(SubjectTag, &SubjectKey)> {
        (0..n)
            .map(|i| {
                let mut t = [0_u8; 16];
                t[..2].copy_from_slice(&i.to_be_bytes());
                (t, &s)
            })
            .collect()
    };
    assert!(derive_dek(&doc, &[0; 16], &make(255)).is_ok());
    assert!(matches!(
        derive_dek(&doc, &[0; 16], &make(256)),
        Err(CryptoError::TooManySubjects)
    ));
}
