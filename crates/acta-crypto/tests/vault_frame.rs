//! Tresor-Rahmen (Spec 4.3): Festwerte aus PyNaCl (libsodium), Positionsbindung, Fail-Closed.
//! Orakel: `tests/golden/crypto_oracle.py`.

use acta_base::{CryptoRng, EntropyError};
use acta_crypto::error::CryptoError;
use acta_crypto::key::{DataKey, MasterKey};
use acta_crypto::vault::{open_entry, seal_entry, VaultPos, FRAME_LEN, PLAIN_LEN};

const KAT_FRAME: &str = "00326465666768696a6b6c6d6e6f707172737475767778797a7b7e72521b548522292c9b36940dc41662f0100e2db87096ecd00938fb7702244cf75a1f2270eaab9bb65a3940080a540d32b928834131a6b9a35379538e68eac62eae";
const KAT_TOMBSTONE: &str = "00326465666768696a6b6c6d6e6f707172737475767778797a7b7d71f9b0ff2e898287309d3fa66fbdc95bbb5271e42ccab08c5564a72b5e7810ab06437e2cb6f7c7ea06651c545608516ee5dd59843baa05a87afdfc20245dc6a6c4";
const KAT_SLOT_43: &str = "00326465666768696a6b6c6d6e6f707172737475767778797a7b7e72521b548522292c9b36940dc41662f0100e2db87096ecd00938fb7702244cf75a1f2270eaab9bb65a3940080a540d32b9f8c2a72eab44e05e5a6a22e26d0a7517";

struct Fixed(Vec<u8>);
impl CryptoRng for Fixed {
    fn try_fill(&self, dst: &mut [u8]) -> Result<(), EntropyError> {
        for (d, s) in dst.iter_mut().zip(self.0.iter()) {
            *d = *s;
        }
        Ok(())
    }
}

struct Failing;
impl CryptoRng for Failing {
    fn try_fill(&self, _dst: &mut [u8]) -> Result<(), EntropyError> {
        Err(EntropyError)
    }
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

fn kat_rng() -> Fixed {
    Fixed((0..24_u8).map(|i| 100 + i).collect())
}

fn key() -> DataKey {
    DataKey::from_bytes(std::array::from_fn(|i| i as u8))
}

fn pos(slot: u64) -> VaultPos {
    VaultPos {
        tenant: [0x11; 16],
        generation: 3,
        slot,
    }
}

fn entry_plain() -> [u8; PLAIN_LEN] {
    let mut p = [0x5c_u8; PLAIN_LEN];
    p[0] = 0x02;
    p[1] = 0x01;
    p[2..18].fill(0xab);
    p
}

fn tombstone_plain() -> [u8; PLAIN_LEN] {
    let mut p = [0_u8; PLAIN_LEN];
    p[0] = 0x01;
    p[1] = 0x02;
    p
}

#[test]
fn frame_matches_independent_reference() -> Result<(), CryptoError> {
    assert_eq!(FRAME_LEN, 92);
    let frame = seal_entry(&key(), &pos(42), &entry_plain(), &kat_rng())?;
    assert_eq!(hex(&frame), KAT_FRAME);
    let tomb = seal_entry(&key(), &pos(42), &tombstone_plain(), &kat_rng())?;
    assert_eq!(hex(&tomb), KAT_TOMBSTONE);
    let other = seal_entry(&key(), &pos(43), &entry_plain(), &kat_rng())?;
    assert_eq!(hex(&other), KAT_SLOT_43);
    Ok(())
}

#[test]
fn roundtrip_and_position_binding() -> Result<(), CryptoError> {
    let frame = seal_entry(&key(), &pos(42), &entry_plain(), &kat_rng())?;
    let plain = open_entry(&key(), &pos(42), &frame)?;
    assert_eq!(*plain, entry_plain()); // Positivkontrolle

    // Eintrag an anderer Position, Generation oder Mandant lässt sich nicht öffnen.
    let mut other_tenant = pos(42);
    other_tenant.tenant[0] ^= 1;
    let mut other_gen = pos(42);
    other_gen.generation += 1;
    for wrong in [pos(43), other_tenant, other_gen] {
        assert!(matches!(
            open_entry(&key(), &wrong, &frame),
            Err(CryptoError::DecryptionFailed)
        ));
    }
    let wrong_key = DataKey::from_bytes([9; 32]);
    assert!(matches!(
        open_entry(&wrong_key, &pos(42), &frame),
        Err(CryptoError::DecryptionFailed)
    ));
    Ok(())
}

#[test]
fn every_bit_flip_is_rejected() -> Result<(), CryptoError> {
    let frame = seal_entry(&key(), &pos(1), &entry_plain(), &kat_rng())?;
    open_entry(&key(), &pos(1), &frame)?; // Positivkontrolle
    for byte in 0..frame.len() {
        for bit in 0..8 {
            let mut bad = frame;
            if let Some(b) = bad.get_mut(byte) {
                *b ^= 1 << bit;
            }
            assert!(
                open_entry(&key(), &pos(1), &bad).is_err(),
                "byte {byte} bit {bit}"
            );
        }
    }
    Ok(())
}

#[test]
fn structural_errors_are_malformed() -> Result<(), CryptoError> {
    let frame = seal_entry(&key(), &pos(1), &entry_plain(), &kat_rng())?;
    assert!(matches!(
        open_entry(&key(), &pos(1), frame.get(..91).unwrap_or(&[])),
        Err(CryptoError::Malformed)
    ));
    let mut long = frame.to_vec();
    long.push(0);
    assert!(matches!(
        open_entry(&key(), &pos(1), &long),
        Err(CryptoError::Malformed)
    ));
    let mut bad_len = frame;
    if let Some(b) = bad_len.get_mut(1) {
        *b = 51;
    }
    assert!(matches!(
        open_entry(&key(), &pos(1), &bad_len),
        Err(CryptoError::Malformed)
    ));
    Ok(())
}

#[test]
fn entropy_failure_produces_no_frame() -> Result<(), CryptoError> {
    let res = seal_entry(&key(), &pos(1), &entry_plain(), &Failing);
    assert!(matches!(res, Err(CryptoError::Entropy)));
    // Positivkontrolle: mit funktionierender Quelle entsteht ein Rahmen.
    seal_entry(&key(), &pos(1), &entry_plain(), &kat_rng())?;
    Ok(())
}

#[test]
fn tombstone_reseal_uses_fresh_nonce_and_differs() -> Result<(), CryptoError> {
    // Gleicher Slot, anderer Klartext (Grabstein), andere Nonce: kein Nonce-Wiederverwenden.
    let a = seal_entry(&key(), &pos(7), &entry_plain(), &kat_rng())?;
    let other_nonce = Fixed((0..24_u8).map(|i| 200 + i).collect());
    let b = seal_entry(&key(), &pos(7), &tombstone_plain(), &other_nonce)?;
    assert_ne!(a.get(2..26), b.get(2..26));
    assert!(open_entry(&key(), &pos(7), &b)?.first() == Some(&0x01));
    Ok(())
}

#[test]
fn backup_key_differs_from_vault_key() -> Result<(), CryptoError> {
    use acta_crypto::kdf::{derive_vault_backup_key, derive_vault_key};
    let m = MasterKey::from_bytes([0x66; 32]);
    let t = [0x44; 16];
    let vk = derive_vault_key(&m, &t, 0)?;
    let vkb = derive_vault_backup_key(&m, &t)?;
    let frame = seal_entry(&vk, &pos(1), &entry_plain(), &kat_rng())?;
    assert!(open_entry(&vkb, &pos(1), &frame).is_err());
    Ok(())
}
