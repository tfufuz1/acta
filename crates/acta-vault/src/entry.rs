//! Tresor-Eintrag (SPEC 4.3): Klartextlayout, Rahmen und Positions-AAD.
//!
//! Ein Eintrag hält einen Schlüssel. Deshalb trägt er bewusst keine `Debug`- und `Clone`-Ableitung
//! (CLAUDE.md, Regel für Schlüsseltypen). Die Versiegelung unter dem Tresorschlüssel ist hier
//! **nicht** enthalten, siehe `CLAUDE.md`, Abschnitt Status (offene Entscheidung zum Umschlag).

use crate::error::VaultError;

/// Länge des Klartexts: `[kind u8][status u8][id 16][key 32]`.
pub const PLAIN_LEN: usize = 50;
/// Länge des Rahmens: `[len u16][nonce 24][ciphertext 50][tag 16]`.
pub const FRAME_LEN: usize = 2 + NONCE_LEN + PLAIN_LEN + TAG_LEN;
/// Nonce-Länge (XChaCha20-Poly1305).
pub const NONCE_LEN: usize = 24;
/// Tag-Länge (Poly1305).
pub const TAG_LEN: usize = 16;
/// Domänenpräfix der Positions-AAD (SPEC 4.3, Domänenpräfixe).
pub const AAD_DOMAIN: &[u8] = b"acta/vault/v1";

/// Art des Schlüssels im Tresor (SPEC 4.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Kind {
    /// `SubjectKey`.
    Subject,
    /// `DocumentKey`; Kennung ist die `VerRef`.
    Document,
    /// `SegmentKey`; Kennung ist die `file_id`.
    Segment,
    /// `ScopeKey`; Kennung ist die `ErasureId`.
    Scope,
    /// `CommitKey`; Kennung ist die Epoche.
    Commit,
    /// `SubjectAlias`; Kennung ist der Alias-Tag.
    Alias,
    /// `LogKey`; Kennung ist der Monatsindex.
    Log,
}

impl Kind {
    /// Code im Klartext.
    #[must_use]
    pub const fn code(self) -> u8 {
        match self {
            Self::Subject => 0x01,
            Self::Document => 0x02,
            Self::Segment => 0x03,
            Self::Scope => 0x04,
            Self::Commit => 0x05,
            Self::Alias => 0x06,
            Self::Log => 0x07,
        }
    }

    /// Rückabbildung eines Codes. Unbekannte Codes sind `None`.
    #[must_use]
    pub const fn from_code(code: u8) -> Option<Self> {
        match code {
            0x01 => Some(Self::Subject),
            0x02 => Some(Self::Document),
            0x03 => Some(Self::Segment),
            0x04 => Some(Self::Scope),
            0x05 => Some(Self::Commit),
            0x06 => Some(Self::Alias),
            0x07 => Some(Self::Log),
            _ => None,
        }
    }
}

/// Status eines Eintrags (SPEC 4.3, Anhang B.5).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Status {
    /// `aktiv (0x01)`.
    Active,
    /// `Grabstein (0x02)`: der Schlüssel ist vernichtet.
    Tombstone,
}

impl Status {
    /// Code im Klartext.
    #[must_use]
    pub const fn code(self) -> u8 {
        match self {
            Self::Active => 0x01,
            Self::Tombstone => 0x02,
        }
    }

    /// Rückabbildung eines Codes. Unbekannte Codes sind `None`.
    #[must_use]
    pub const fn from_code(code: u8) -> Option<Self> {
        match code {
            0x01 => Some(Self::Active),
            0x02 => Some(Self::Tombstone),
            _ => None,
        }
    }
}

/// Ein Tresor-Eintrag im Klartext.
pub struct Entry {
    kind: Kind,
    status: Status,
    id: [u8; 16],
    key: [u8; 32],
}

impl Entry {
    /// Neuer aktiver Eintrag.
    #[must_use]
    pub const fn new(kind: Kind, id: [u8; 16], key: [u8; 32]) -> Self {
        Self {
            kind,
            status: Status::Active,
            id,
            key,
        }
    }

    /// Art des Schlüssels.
    #[must_use]
    pub const fn kind(&self) -> Kind {
        self.kind
    }

    /// Status des Eintrags.
    #[must_use]
    pub const fn status(&self) -> Status {
        self.status
    }

    /// Kennung im Klartext (nur im Chiffrat dauerhaft, SPEC 4.3).
    #[must_use]
    pub const fn id(&self) -> &[u8; 16] {
        &self.id
    }

    /// Schlüsselmaterial. Nur für die Versiegelung im Tresor-Crate gedacht.
    // Wird erst mit der Versiegelung im Tresor genutzt (Arbeitspaket 3).
    #[allow(dead_code)]
    #[must_use]
    pub(crate) const fn key(&self) -> &[u8; 32] {
        &self.key
    }

    /// Vernichtet den Schlüssel: Status wird `Grabstein`, das Material wird genullt.
    pub fn tombstone(&mut self) {
        self.status = Status::Tombstone;
        self.key = [0; 32];
    }

    /// Klartext `[kind][status][id][key]`.
    #[must_use]
    pub fn encode_plain(&self) -> [u8; PLAIN_LEN] {
        let mut out = [0_u8; PLAIN_LEN];
        out[0] = self.kind.code();
        out[1] = self.status.code();
        out[2..18].copy_from_slice(&self.id);
        out[18..50].copy_from_slice(&self.key);
        out
    }

    /// Liest einen Klartext. Länge, Art und Status werden strikt geprüft.
    ///
    /// # Errors
    /// [`VaultError::Malformed`] bei falscher Länge, unbekannter Art oder unbekanntem Status.
    pub fn decode_plain(bytes: &[u8]) -> Result<Self, VaultError> {
        let arr = <[u8; PLAIN_LEN]>::try_from(bytes).map_err(|_| VaultError::Malformed)?;
        let [kind_code, status_code, rest @ ..] = arr;
        let kind = Kind::from_code(kind_code).ok_or(VaultError::Malformed)?;
        let status = Status::from_code(status_code).ok_or(VaultError::Malformed)?;
        let mut id = [0_u8; 16];
        let mut key = [0_u8; 32];
        let (id_part, key_part) = rest.split_at(16);
        id.copy_from_slice(id_part);
        key.copy_from_slice(key_part);
        Ok(Self {
            kind,
            status,
            id,
            key,
        })
    }
}

/// Positions-AAD: `acta/vault/v1 ‖ TenantId ‖ generation u32 ‖ slot u64` (SPEC 4.3).
///
/// Die AAD bindet den Eintrag an seine Position. Ein an eine andere Stelle kopierter Eintrag
/// lässt sich nicht öffnen.
#[must_use]
pub fn position_aad(tenant: &[u8; 16], generation: u32, slot: u64) -> Vec<u8> {
    let mut out = Vec::with_capacity(AAD_DOMAIN.len() + 16 + 4 + 8);
    out.extend_from_slice(AAD_DOMAIN);
    out.extend_from_slice(tenant);
    out.extend_from_slice(&generation.to_be_bytes());
    out.extend_from_slice(&slot.to_be_bytes());
    out
}

/// Rahmen `[len u16][nonce][ct][tag]`. `len` ist die Länge des Chiffrats und fest 50.
#[must_use]
pub fn build_frame(
    nonce: &[u8; NONCE_LEN],
    ciphertext: &[u8; PLAIN_LEN],
    tag: &[u8; TAG_LEN],
) -> [u8; FRAME_LEN] {
    let mut out = [0_u8; FRAME_LEN];
    let len = u16::try_from(PLAIN_LEN).unwrap_or(u16::MAX);
    out[0..2].copy_from_slice(&len.to_be_bytes());
    out[2..26].copy_from_slice(nonce);
    out[26..76].copy_from_slice(ciphertext);
    out[76..92].copy_from_slice(tag);
    out
}

/// Bestandteile eines Rahmens: Nonce, Chiffrat, Tag.
pub type FrameParts = ([u8; NONCE_LEN], [u8; PLAIN_LEN], [u8; TAG_LEN]);

/// Zerlegt einen Rahmen. Falsche Gesamtlänge oder falsches `len`-Feld sind Fehler.
///
/// # Errors
/// [`VaultError::Malformed`] bei abweichender Länge oder `len`.
pub fn split_frame(bytes: &[u8]) -> Result<FrameParts, VaultError> {
    let arr = <[u8; FRAME_LEN]>::try_from(bytes).map_err(|_| VaultError::Malformed)?;
    let len = u16::from_be_bytes([arr[0], arr[1]]);
    if usize::from(len) != PLAIN_LEN {
        return Err(VaultError::Malformed);
    }
    let mut nonce = [0_u8; NONCE_LEN];
    let mut ct = [0_u8; PLAIN_LEN];
    let mut tag = [0_u8; TAG_LEN];
    nonce.copy_from_slice(&arr[2..26]);
    ct.copy_from_slice(&arr[26..76]);
    tag.copy_from_slice(&arr[76..92]);
    Ok((nonce, ct, tag))
}

#[cfg(test)]
mod tests {
    use super::*;

    const ID: [u8; 16] = [0xAB; 16];
    const KEY: [u8; 32] = [0x5C; 32];

    #[test]
    fn spec_lengths_are_fixed() {
        assert_eq!(PLAIN_LEN, 50);
        assert_eq!(FRAME_LEN, 92);
    }

    #[test]
    fn plain_roundtrip_keeps_kind_status_id_and_key() {
        let entry = Entry::new(Kind::Document, ID, KEY);
        let plain = entry.encode_plain();
        assert_eq!(plain[0], 0x02);
        assert_eq!(plain[1], 0x01);
        let back = Entry::decode_plain(&plain).expect("roundtrip");
        assert_eq!(back.kind(), Kind::Document);
        assert_eq!(back.status(), Status::Active);
        assert_eq!(back.id(), &ID);
        assert_eq!(back.key(), &KEY);
    }

    #[test]
    fn tombstone_zeroes_key_and_sets_status() {
        let mut entry = Entry::new(Kind::Subject, ID, KEY);
        entry.tombstone();
        assert_eq!(entry.status(), Status::Tombstone);
        assert_eq!(entry.key(), &[0_u8; 32]);
        let plain = entry.encode_plain();
        assert_eq!(plain[1], 0x02);
        assert!(plain[18..].iter().all(|b| *b == 0));
    }

    #[test]
    fn every_kind_code_roundtrips_and_unknown_is_rejected() {
        let all = [
            Kind::Subject,
            Kind::Document,
            Kind::Segment,
            Kind::Scope,
            Kind::Commit,
            Kind::Alias,
            Kind::Log,
        ];
        for kind in all {
            assert_eq!(Kind::from_code(kind.code()), Some(kind));
        }
        assert_eq!(Kind::from_code(0x00), None);
        assert_eq!(Kind::from_code(0x08), None);
    }

    #[test]
    fn decode_rejects_wrong_length_unknown_kind_and_status() {
        let mut plain = Entry::new(Kind::Log, ID, KEY).encode_plain();
        assert!(matches!(
            Entry::decode_plain(&plain[..49]),
            Err(VaultError::Malformed)
        ));
        plain[0] = 0xFF;
        assert!(matches!(
            Entry::decode_plain(&plain),
            Err(VaultError::Malformed)
        ));
        plain[0] = 0x01;
        plain[1] = 0x09;
        assert!(matches!(
            Entry::decode_plain(&plain),
            Err(VaultError::Malformed)
        ));
    }

    #[test]
    fn position_aad_binds_tenant_generation_and_slot() {
        let tenant = [1_u8; 16];
        let base = position_aad(&tenant, 3, 42);
        assert_eq!(base.len(), 13 + 16 + 4 + 8);
        assert!(base.starts_with(AAD_DOMAIN));
        assert_ne!(base, position_aad(&[2_u8; 16], 3, 42));
        assert_ne!(base, position_aad(&tenant, 4, 42));
        assert_ne!(base, position_aad(&tenant, 3, 43));
    }

    #[test]
    fn frame_roundtrip_and_fixed_len_field() {
        let nonce = [7_u8; NONCE_LEN];
        let ct = [8_u8; PLAIN_LEN];
        let tag = [9_u8; TAG_LEN];
        let frame = build_frame(&nonce, &ct, &tag);
        assert_eq!(&frame[0..2], &50_u16.to_be_bytes());
        let (n, c, t) = split_frame(&frame).expect("roundtrip");
        assert_eq!((n, c, t), (nonce, ct, tag));
    }

    #[test]
    fn split_frame_rejects_wrong_length_field() {
        let mut frame = build_frame(&[0; NONCE_LEN], &[0; PLAIN_LEN], &[0; TAG_LEN]);
        frame[1] = 51;
        assert!(matches!(split_frame(&frame), Err(VaultError::Malformed)));
        assert!(matches!(
            split_frame(&frame[..91]),
            Err(VaultError::Malformed)
        ));
    }
}
