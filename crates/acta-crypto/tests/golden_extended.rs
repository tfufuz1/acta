//! Erweiterte Festwerte (Spec 16.1, 16.2): VKb, ScopeKey-MAC, Umschläge K und D, Hash-Kette,
//! RFC 4231 (Fälle 3 bis 7), RFC 5869 (Fall 3), RFC 8032 (Test 2), NIST SHA-256.
//! Orakel: `tests/golden/crypto_oracle.py` (hashlib, hmac, PyNaCl, `cryptography`).

use acta_base::{CryptoRng, EntropyError};
use acta_crypto::aead::{seal, Derived, Knowledge, SealCtx, Sealed};
use acta_crypto::domain::{CHECKPOINT, REPORT};
use acta_crypto::error::CryptoError;
use acta_crypto::hash::{sha256, HashChain, GENESIS};
use acta_crypto::kdf::{derive_vault_backup_key, derive_vault_key, hkdf_sha256};
use acta_crypto::key::{DataKey, MasterKey, ScopeKey};
use acta_crypto::mac::hmac_sha256;
use acta_crypto::sign::NotarySigner;

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

struct Fixed(Vec<u8>);
impl CryptoRng for Fixed {
    fn try_fill(&self, dst: &mut [u8]) -> Result<(), EntropyError> {
        for (d, s) in dst.iter_mut().zip(self.0.iter()) {
            *d = *s;
        }
        Ok(())
    }
}

fn kat_rng() -> Fixed {
    Fixed((0..24_u8).map(|i| 100 + i).collect())
}

#[test]
fn vault_backup_key_matches_reference_and_is_separated() -> Result<(), CryptoError> {
    let m = MasterKey::from_bytes([0x66; 32]);
    let t = [0x44; 16];
    // Eingabe wie in der Referenz: M_bk(e) = 0x66.., Mandant 0x44.., info = "acta/vaultbk/v1".
    // Der DataKey gibt kein Material frei; Prüfung über eine Versiegelung mit bekanntem Ergebnis
    // steht in `vault_frame.rs`. Hier: Eigenschaften.
    let a = derive_vault_backup_key(&m, &t)?;
    let b = derive_vault_backup_key(&m, &t)?;
    let other_tenant = derive_vault_backup_key(&m, &[0x45; 16])?;
    let vk = derive_vault_key(&m, &t, 0)?;
    let ctx = SealCtx {
        tenant: t,
        keyspace: 1,
        record_ref: [1; 16],
        version: 1,
    };
    let sealed: Sealed<Derived> = seal(&a, &ctx, b"x", &kat_rng())?;
    assert!(acta_crypto::aead::open(&b, &ctx, &sealed).is_ok()); // Positivkontrolle
    assert!(acta_crypto::aead::open(&other_tenant, &ctx, &sealed).is_err());
    assert!(acta_crypto::aead::open(&vk, &ctx, &sealed).is_err());
    Ok(())
}

#[test]
fn vault_backup_hkdf_reference() -> Result<(), CryptoError> {
    let okm = hkdf_sha256(&[&[0x66; 32]], &[0x44; 16], b"acta/vaultbk/v1")?;
    assert_eq!(
        hex(okm.as_slice()),
        "f7578fd551e89347741588f2bab53a8eae99d44c52413451b8a2826ee136ec29"
    );
    Ok(())
}

#[test]
fn scope_key_mac_matches_reference() -> Result<(), CryptoError> {
    let key = ScopeKey::from_bytes([0x77; 32]);
    let window: Vec<u8> = (1..=32).collect();
    let tag = key.mac(&[&window])?;
    assert_eq!(
        hex(&tag),
        "f07dfb381f3188c917e6a67769f4ea3e1dac7c3efbf8c5600e4175fcca17213b"
    );
    assert!(key.verify_mac(&[&window], &tag)?);
    let mut bad = tag;
    if let Some(b) = bad.first_mut() {
        *b ^= 1;
    }
    assert!(!key.verify_mac(&[&window], &bad)?);
    assert!(!ScopeKey::from_bytes([0x78; 32]).verify_mac(&[&window], &tag)?);
    Ok(())
}

fn env_ctx(keyspace: u16) -> SealCtx {
    SealCtx {
        tenant: [0x11; 16],
        keyspace,
        record_ref: [0x22; 16],
        version: 7,
    }
}

fn env_key() -> DataKey {
    DataKey::from_bytes(std::array::from_fn(|i| i as u8))
}

#[test]
fn knowledge_and_derived_envelopes_match_reference() -> Result<(), CryptoError> {
    let k: Sealed<Knowledge> = seal(&env_key(), &env_ctx(0x0010), b"Wissen: Vertrag", &kat_rng())?;
    assert_eq!(
        hex(k.as_bytes()),
        "0101026465666768696a6b6c6d6e6f707172737475767778797a7b7c73f9bfa847faf1e25ea71ff00acfbd29da3571e42ccab08c5564a72b5e7810ab06437e2cb6f7c7ea06651c545608516ee5d01f9fb234a6cf0f0b48fb88be6c56d3c4e5d06ea42dee0dbc94cdfa8e4bf3ed81258ee3c3fb915da991e690fcb54c7d9d77f121d813008c023f91355992b8602b7291a0aa5cae3f13e4550120ca8971bdc369c8d86c4ce0606be42bcf183b4889c75b38944b161be8eaddbf39fa4f1ee95d8c91ffdebf4bc8b220ea2822812c7689c750a7bf774db9a5e127c4adaa4e8a50c34f080b8023bd2b29ca6708da4e949ea1dc06048068d276de2fd40923978bb6da2324f05606412ac622f869fd6bca00d530090e494e81d8aecc5f9267e7ee526c8d9fd7d5ba33e5a4bd985c"
    );
    let d: Sealed<Derived> = seal(&env_key(), &env_ctx(0x0020), b"Ableitung", &kat_rng())?;
    assert_eq!(
        hex(d.as_bytes()),
        "0101036465666768696a6b6c6d6e6f707172737475767778797a7b7c73f9b9be4ce5e7ee44e851c16fbdc95bbb5271e42ccab08c5564a72b5e7810ab06437e2cb6f7c7ea06651c545608516ee5d01f9fb234a6cf0f0b48fb88be6c56d3c4e5d06ea42dee0dbc94cdfa8e4bf3ed81258ee3c3fb915da991e690fcb54c7d9d77f121d813008c023f91355992b8602b7291a0aa5cae3f13e4550120ca8971bdc369c8d86c4ce0606be42bcf183b4889c75b38944b161be8eaddbf39fa4f1ee95d8c91ffdebf4bc8b220ea2822812c7689c750a7bf774db9a5e127c4adaa4e8a50c34f080b8023bd2b29ca6708da4e949ea1dc06048068d276de2fd40923978bb6da2324f05606412ac622f869fd6bca00d530090e494e81d8aecc5f92fdce6ba80e68c36f0bcc98f762ee7f14"
    );
    Ok(())
}

#[test]
fn hash_chain_matches_reference() {
    // chain_id = SHA-256(TenantId ‖ "E") (Spec 6.2), Tenant 0x44..
    let chain_id = sha256(&[&[0x44; 16], b"E"]);
    assert_eq!(
        hex(&chain_id),
        "dbb40b4242f532e7204ba27974b4a8e2eb9d4c7c7de61dcadcd51e7b8ed32e86"
    );
    let mut chain = HashChain::new(chain_id);
    assert_eq!(chain.head(), GENESIS);
    let e1 = chain.append(b"entry-1");
    let e2 = chain.append(b"entry-2");
    assert_eq!(
        hex(&e1),
        "7e995d79b574b75cd60636145381242a29434b54249c7004a864856c626df125"
    );
    assert_eq!(
        hex(&e2),
        "799b5b9bc721a7801a6a8cb6b9ac9d3261648a9c08523c1f2f62fce5d1fee1e2"
    );
}

#[test]
fn sha256_nist_vectors() {
    let m448 = b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq";
    assert_eq!(
        hex(&sha256(&[m448])),
        "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1"
    );
    let m896 = b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu";
    assert_eq!(
        hex(&sha256(&[m896])),
        "cf5b16a778af8380036ce59e7b0492370b249b11e8f07a51afac45037afee9d1"
    );
    let million = vec![b'a'; 1_000_000];
    assert_eq!(
        hex(&sha256(&[&million])),
        "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0"
    );
}

#[test]
fn rfc4231_cases_3_to_7() -> Result<(), CryptoError> {
    let cases: [(Vec<u8>, Vec<u8>, &str); 5] = [
        (
            vec![0xaa; 20],
            vec![0xdd; 50],
            "773ea91e36800e46854db8ebd09181a72959098b3ef8c122d9635514ced565fe",
        ),
        (
            (1..=25).collect(),
            vec![0xcd; 50],
            "82558a389a443c0ea4cc819899f2083a85f0faa3e578f8077a2e3ff46729665b",
        ),
        (
            vec![0x0c; 20],
            b"Test With Truncation".to_vec(),
            "a3b6167473100ee06e0c796c2955552bfa6f7c0a6a8aef8b93f860aab0cd20c5",
        ),
        (
            vec![0xaa; 131],
            b"Test Using Larger Than Block-Size Key - Hash Key First".to_vec(),
            "60e431591ee0b67f0d8a26aacbf5b77f8e0bc6213728c5140546040f0ee37f54",
        ),
        (
            vec![0xaa; 131],
            b"This is a test using a larger than block-size key and a larger than block-size data. The key needs to be hashed before being used by the HMAC algorithm.".to_vec(),
            "9b09ffa71b942fcb27635fbcd5b0e944bfdc63644f0713938a7f51535c3a35e2",
        ),
    ];
    for (key, data, want) in cases {
        assert_eq!(hex(&hmac_sha256(&key, &[&data])?), want);
    }
    Ok(())
}

#[test]
fn rfc5869_case3_empty_salt_and_info() -> Result<(), CryptoError> {
    let okm = hkdf_sha256(&[&[0x0b; 22]], &[], &[])?;
    // RFC-Ausgabe hat 42 Byte; HKDF ist präfixstabil.
    assert_eq!(
        hex(okm.as_slice()),
        "8da4e775a563c18f715f802a063c5a31b8a11f5c5ee1879ec3454e5f3c738d2d"
    );
    Ok(())
}

#[test]
fn ed25519_rfc8032_test2_and_domain_signatures() -> Result<(), CryptoError> {
    let seed: [u8; 32] = std::array::from_fn(|i| {
        let s = "4ccd089b28ff96da9db6c346ec114e0f5b8a319f35aba624da8cf6ed4fb8a6fb";
        s.get(2 * i..2 * i + 2)
            .and_then(|h| u8::from_str_radix(h, 16).ok())
            .unwrap_or(0)
    });
    let signer = NotarySigner::from_seed(seed);
    assert_eq!(
        hex(&signer.public_key()),
        "3d4017c3e843895a92b70aa74d1b7ebc9c982ccf2ec4968cc0cd55f12af4660c"
    );
    assert_eq!(
        hex(&signer.sign(&[], &[0x72])),
        "92a009a9f0d4cab8720e820b5f642540a2b27b5416503f8fb3762223ebdb69da085ac1e43e15996e458f3613d0f11d8c387b2eaeb4302aeeb00d291612bb0c00"
    );

    let s = NotarySigner::from_seed([7; 32]);
    assert_eq!(
        hex(&s.public_key()),
        "ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c"
    );
    assert_eq!(
        hex(&s.sign(REPORT, b"bericht")),
        "07870ea1d59b0db589ad1f17c025e8ca21cff2decdf963f16d4aa6dc23aedad0f6d0cc3f58e7a976ddc3f3f09438e039b0d5b58b5a3bbf03ab7abc3ad54b9a0e"
    );
    assert_eq!(
        hex(&s.sign(CHECKPOINT, b"stand")),
        "cec9e68dc9a6c95779b802b7e08d808751c1dba0c5fcef01a787df61da421df93b3ae45377bdcab6ed9cadd81171c5bef3442b221564b5e1c0458aed3b23500e"
    );
    Ok(())
}
