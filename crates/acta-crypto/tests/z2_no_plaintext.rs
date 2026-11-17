//! Z2 (Spec 16.3): `T-CRY-1` (Bytes-Grep auf bekannte Klartexte und Kennungen), `T-META-*`
//! (keine exakten Längen, keine dauerhaften deterministischen Pseudonyme). Jede Negativaussage
//! hat eine Positivkontrolle (16.4).

use acta_base::{CryptoRng, EntropyError};
use acta_crypto::aead::{open, seal, Personal, SealCtx, Sealed};
use acta_crypto::error::CryptoError;
use acta_crypto::key::DataKey;
use acta_crypto::pad::target_len;
use std::sync::atomic::{AtomicU64, Ordering};

/// Zähler-Quelle: jede Ziehung liefert neue, verschiedene Bytes.
struct Counter(AtomicU64);
impl CryptoRng for Counter {
    fn try_fill(&self, dst: &mut [u8]) -> Result<(), EntropyError> {
        for chunk in dst.chunks_mut(8) {
            let v = self
                .0
                .fetch_add(1, Ordering::SeqCst)
                .wrapping_mul(0x9E37_79B9_7F4A_7C15);
            for (d, s) in chunk.iter_mut().zip(v.to_be_bytes()) {
                *d = s;
            }
        }
        Ok(())
    }
}

struct Fixed;
impl CryptoRng for Fixed {
    fn try_fill(&self, dst: &mut [u8]) -> Result<(), EntropyError> {
        dst.fill(0x11);
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

fn key() -> DataKey {
    DataKey::from_bytes([0x42; 32])
}

fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    !needle.is_empty() && haystack.windows(needle.len()).any(|w| w == needle)
}

const CANARIES: [&[u8]; 3] = [
    b"Erika Mustermann",
    b"az:4711/2026",
    b"IBAN DE02120300000000202051",
];

#[test]
fn t_cry_1_known_plaintexts_and_ids_do_not_appear_in_envelopes() -> Result<(), CryptoError> {
    let rng = Counter(AtomicU64::new(1));
    for canary in CANARIES {
        let mut payload = b"Akte: ".to_vec();
        payload.extend_from_slice(canary);
        payload.extend_from_slice(&[b'x'; 300]);
        let sealed: Sealed<Personal> = seal(&key(), &ctx(), &payload, &rng)?;

        // Positivkontrolle: dieselbe Suche findet den Canary im Klartext.
        assert!(contains(&payload, canary), "Positivkontrolle");
        assert!(!contains(sealed.as_bytes(), canary), "Klartext im Umschlag");
        assert_eq!(
            open(&key(), &ctx(), &sealed)?.as_slice(),
            payload.as_slice()
        );
    }
    Ok(())
}

#[test]
fn t_cry_1_grep_positive_control_with_a_broken_sealer() {
    // Ein „Umschlag“ ohne Verschlüsselung muss vom Grep gefunden werden.
    let mut fake = vec![0x01, 0x01, 0x01];
    fake.extend_from_slice(CANARIES[0]);
    assert!(contains(&fake, CANARIES[0]));
}

#[test]
fn t_meta_exact_lengths_are_hidden_within_a_size_class() -> Result<(), CryptoError> {
    let rng = Counter(AtomicU64::new(1));
    let lens: Vec<usize> = [1_usize, 17, 100, 252]
        .iter()
        .map(|n| {
            let payload = vec![7_u8; *n];
            seal::<Personal, _>(&key(), &ctx(), &payload, &rng).map(|s| s.as_bytes().len())
        })
        .collect::<Result<_, _>>()?;
    assert!(lens.windows(2).all(|w| w[0] == w[1]), "{lens:?}");

    // Positivkontrolle: über die Klassengrenze hinweg ändert sich die Länge.
    let small = seal::<Personal, _>(&key(), &ctx(), &[0; 252], &rng)?
        .as_bytes()
        .len();
    let large = seal::<Personal, _>(&key(), &ctx(), &[0; 253], &rng)?
        .as_bytes()
        .len();
    assert_ne!(small, large);
    // Umschlaglänge = Kopf 27 + Zielgröße + Tag 16.
    assert_eq!(small, 27 + target_len(4 + 252)? + 16);
    Ok(())
}

#[test]
fn t_meta_no_deterministic_pseudonym_from_equal_inputs() -> Result<(), CryptoError> {
    let rng = Counter(AtomicU64::new(1));
    let a: Sealed<Personal> = seal(&key(), &ctx(), b"gleich", &rng)?;
    let b: Sealed<Personal> = seal(&key(), &ctx(), b"gleich", &rng)?;
    assert_ne!(a.as_bytes(), b.as_bytes());

    // Positivkontrolle: mit fester Nonce wären die Umschläge gleich, der Test könnte es erkennen.
    let c: Sealed<Personal> = seal(&key(), &ctx(), b"gleich", &Fixed)?;
    let d: Sealed<Personal> = seal(&key(), &ctx(), b"gleich", &Fixed)?;
    assert_eq!(c.as_bytes(), d.as_bytes());
    Ok(())
}
