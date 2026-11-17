//! Tresor-Rahmen (Spec 4.3): fester 92-Byte-Rahmen ohne Padding.
//!
//! Layout: `[len u16 = 50][nonce 24][ciphertext 50][tag 16]`. Der Klartext
//! `[kind u8][status u8][id 16][key 32]` bildet der Tresor (`acta-vault`). Die AAD ist
//! `acta/vault/v1 ‖ TenantId ‖ generation u32 ‖ slot u64` und bindet den Eintrag an seine
//! Position. Die Nonce entsteht hier aus dem Port `CryptoRng`; scheitert er, entsteht kein
//! Rahmen (Z10). Jeder Authentisierungsfehler ist `DecryptionFailed`, ohne Ursachenunterscheidung.

use crate::aead::{SealKey, NONCE_LEN, TAG_LEN};
use crate::domain;
use crate::error::CryptoError;
use acta_base::CryptoRng;
use chacha20poly1305::aead::AeadInPlace;
use chacha20poly1305::{Tag, XNonce};
use zeroize::Zeroizing;

/// Länge des Klartexts eines Eintrags.
pub const PLAIN_LEN: usize = 50;
/// Länge des Rahmens: `2 + 24 + 50 + 16`.
pub const FRAME_LEN: usize = 2 + NONCE_LEN + PLAIN_LEN + TAG_LEN;
const LEN_FIELD: usize = 2;

/// Position eines Eintrags im Tresor (Teil der AAD).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VaultPos {
    /// Mandantenkennung (16 Byte).
    pub tenant: [u8; 16],
    /// Generation des Tresors.
    pub generation: u32,
    /// Slot im Tresor.
    pub slot: u64,
}

fn aad(pos: &VaultPos) -> Vec<u8> {
    let mut out = Vec::with_capacity(domain::VAULT.len() + 16 + 4 + 8);
    out.extend_from_slice(domain::VAULT);
    out.extend_from_slice(&pos.tenant);
    out.extend_from_slice(&pos.generation.to_be_bytes());
    out.extend_from_slice(&pos.slot.to_be_bytes());
    out
}

/// Versiegelt einen Eintragsklartext zu einem 92-Byte-Rahmen.
///
/// # Errors
/// [`CryptoError::Entropy`], wenn `rng` versagt (dann entsteht kein Rahmen),
/// [`CryptoError::Internal`] bei internem Fehler.
pub fn seal_entry<K: SealKey>(
    key: &K,
    pos: &VaultPos,
    plain: &[u8; PLAIN_LEN],
    rng: &dyn CryptoRng,
) -> Result<[u8; FRAME_LEN], CryptoError> {
    let mut nonce = [0_u8; NONCE_LEN];
    rng.try_fill(&mut nonce)?;
    let mut buf = Zeroizing::new(*plain);
    let tag = key
        .cipher()?
        .encrypt_in_place_detached(&XNonce::from(nonce), &aad(pos), buf.as_mut_slice())
        .map_err(|_| CryptoError::Internal)?;

    let len = u16::try_from(PLAIN_LEN).map_err(|_| CryptoError::Internal)?;
    let mut out = Vec::with_capacity(FRAME_LEN);
    out.extend_from_slice(&len.to_be_bytes());
    out.extend_from_slice(&nonce);
    out.extend_from_slice(buf.as_slice());
    out.extend_from_slice(tag.as_slice());
    <[u8; FRAME_LEN]>::try_from(out).map_err(|_| CryptoError::Internal)
}

/// Öffnet einen Rahmen an seiner Position.
///
/// # Errors
/// [`CryptoError::Malformed`] bei falscher Gesamtlänge oder falschem `len`-Feld (öffentliche,
/// feste Werte), [`CryptoError::DecryptionFailed`] bei falschem Schlüssel, falscher Position
/// oder Manipulation.
pub fn open_entry<K: SealKey>(
    key: &K,
    pos: &VaultPos,
    frame: &[u8],
) -> Result<Zeroizing<[u8; PLAIN_LEN]>, CryptoError> {
    let arr = <[u8; FRAME_LEN]>::try_from(frame).map_err(|_| CryptoError::Malformed)?;
    let len_bytes = arr.get(..LEN_FIELD).ok_or(CryptoError::Malformed)?;
    let len = u16::from_be_bytes(
        <[u8; LEN_FIELD]>::try_from(len_bytes).map_err(|_| CryptoError::Malformed)?,
    );
    if usize::from(len) != PLAIN_LEN {
        return Err(CryptoError::Malformed);
    }
    let nonce_end = LEN_FIELD + NONCE_LEN;
    let ct_end = nonce_end + PLAIN_LEN;
    let nonce = <[u8; NONCE_LEN]>::try_from(
        arr.get(LEN_FIELD..nonce_end)
            .ok_or(CryptoError::Malformed)?,
    )
    .map_err(|_| CryptoError::Malformed)?;
    let ct = <[u8; PLAIN_LEN]>::try_from(arr.get(nonce_end..ct_end).ok_or(CryptoError::Malformed)?)
        .map_err(|_| CryptoError::Malformed)?;
    let tag = <[u8; TAG_LEN]>::try_from(arr.get(ct_end..).ok_or(CryptoError::Malformed)?)
        .map_err(|_| CryptoError::Malformed)?;

    let mut buf = Zeroizing::new(ct);
    key.cipher()?
        .decrypt_in_place_detached(
            &XNonce::from(nonce),
            &aad(pos),
            buf.as_mut_slice(),
            &Tag::from(tag),
        )
        .map_err(|_| CryptoError::DecryptionFailed)?;
    Ok(buf)
}
