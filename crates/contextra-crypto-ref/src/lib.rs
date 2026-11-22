//! # Referenz-Kryptografie (`contextra-crypto-ref`)
//!
//! Referenzimplementierung der Ports `Cipher`, `Mac` und `Kdf` aus `contextra-storage-ports`.
//! Verwendet BLAKE3 (keyed hash / derive_key) und ChaCha20-Poly1305 (AEAD).
//! Dient als Dev-Crate für Goldens und Tests (O-6); gelangt nie in den Kern (Z-4).

use contextra_storage_ports::{Cipher, Kdf, Mac};
use std::io;
use subtle::ConstantTimeEq;

/// BLAKE3-basierte MAC-Implementierung (konstante Zeit).
#[derive(Debug, Clone, Copy, Default)]
pub struct Blake3Mac;

impl Mac for Blake3Mac {
    fn mac16(&self, key: &[u8; 32], parts: &[&[u8]]) -> [u8; 16] {
        let mut hasher = blake3::Hasher::new_keyed(key);
        for &part in parts {
            hasher.update(part);
        }
        let hash = hasher.finalize();
        let mut out = [0u8; 16];
        if let Some(src) = hash.as_bytes().get(..16) {
            out.copy_from_slice(src);
        }
        out
    }

    fn verify16(&self, key: &[u8; 32], parts: &[&[u8]], tag: &[u8; 16]) -> bool {
        let expected = self.mac16(key, parts);
        expected.ct_eq(tag).into()
    }
}

/// BLAKE3-basierte KDF-Implementierung.
#[derive(Debug, Clone, Copy, Default)]
pub struct Blake3Kdf;

impl Kdf for Blake3Kdf {
    fn derive(&self, key: &[u8; 32], context: &[&[u8]]) -> [u8; 32] {
        let full_context = if let Some(&first) = context.first() {
            String::from_utf8_lossy(first).to_string()
        } else {
            "ctxs default context".to_string()
        };

        let mut material = Vec::new();
        for &c in context.iter().skip(1) {
            material.extend_from_slice(c);
        }
        material.extend_from_slice(key);
        blake3::derive_key(&full_context, &material)
    }
}

/// ChaCha20-Poly1305 AEAD-Chiffre.
#[derive(Debug, Clone, Copy, Default)]
pub struct ChaChaCipher;

impl Cipher for ChaChaCipher {
    fn seal(&self, key: &[u8; 32], nonce: &[u8; 12], aad: &[u8], plain: &[u8]) -> Vec<u8> {
        use chacha20poly1305::aead::{Aead, Payload};
        use chacha20poly1305::{ChaCha20Poly1305, KeyInit, Nonce};

        let cipher = ChaCha20Poly1305::new(key.into());
        let nonce_ref = Nonce::from_slice(nonce);
        let payload = Payload { msg: plain, aad };
        cipher.encrypt(nonce_ref, payload).unwrap_or_default()
    }

    fn open(&self, key: &[u8; 32], nonce: &[u8; 12], aad: &[u8], sealed: &[u8]) -> io::Result<Vec<u8>> {
        use chacha20poly1305::aead::{Aead, Payload};
        use chacha20poly1305::{ChaCha20Poly1305, KeyInit, Nonce};

        let cipher = ChaCha20Poly1305::new(key.into());
        let nonce_ref = Nonce::from_slice(nonce);
        let payload = Payload { msg: sealed, aad };
        cipher
            .decrypt(nonce_ref, payload)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cipher_seal_open_roundtrip() {
        let cipher = ChaChaCipher;
        let key = [0x42u8; 32];
        let nonce = [0x11u8; 12];
        let aad = b"additional-auth-data";
        let plain = b"hello secret world!";

        let sealed = cipher.seal(&key, &nonce, aad, plain);
        assert_ne!(&sealed[..], plain);

        let opened = cipher.open(&key, &nonce, aad, &sealed).expect("open succeeds");
        assert_eq!(&opened[..], plain);

        // Manipulation des Ciphertexts muss scheitern
        let mut tampered = sealed.clone();
        if let Some(b) = tampered.get_mut(0) {
            *b ^= 1;
        }
        assert!(cipher.open(&key, &nonce, aad, &tampered).is_err());
    }

    #[test]
    fn test_mac_verify() {
        let mac = Blake3Mac;
        let key = [0x77u8; 32];
        let parts = [b"part1".as_slice(), b"part2".as_slice()];

        let tag = mac.mac16(&key, &parts);
        assert!(mac.verify16(&key, &parts, &tag));

        let mut invalid_tag = tag;
        invalid_tag[0] ^= 1;
        assert!(!mac.verify16(&key, &parts, &invalid_tag));
    }
}
