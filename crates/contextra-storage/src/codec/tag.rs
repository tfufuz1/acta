//! # Tags und interne Schlüssel
//!
//! Definitionen für `tag = (stamp << 8) | kind` und Sortierregeln (SPEC §6.2, Anhang A.3).
//! Sortierung: Nutzerschlüssel aufsteigend, Tag absteigend (neueste Version zuerst).

use std::cmp::Ordering;

/// Art der Operation / des Eintrags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum TagKind {
    /// Normales Schreiben (Header + Körper).
    Put = 1,
    /// Punktuelles Löschen.
    Delete = 2,
    /// Bereichslöschung (Tombstone).
    RangeDelete = 3,
    /// Physische Bereichslöschung.
    EraseRange = 4,
    /// Referenz auf ausgelagerten Blob-Körper (nur in Runs).
    PutRef = 5,
    /// Reserviert für spätere Merge-Operatoren (SD-12).
    Merge = 6,
}

impl TagKind {
    /// Versucht ein Byte in ein `TagKind` zu dekodieren.
    pub fn from_u8(b: u8) -> Option<Self> {
        match b {
            1 => Some(Self::Put),
            2 => Some(Self::Delete),
            3 => Some(Self::RangeDelete),
            4 => Some(Self::EraseRange),
            5 => Some(Self::PutRef),
            6 => Some(Self::Merge),
            _ => None,
        }
    }

    /// Gibt das Byte zurück.
    pub const fn as_u8(self) -> u8 {
        self as u8
    }
}

/// Ein 64-Bit-Tag: `(stamp << 8) | kind`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tag(pub u64);

impl Tag {
    /// Erstellt ein neues Tag aus Stamp und Kind.
    pub fn new(stamp: u64, kind: TagKind) -> Self {
        // Stamp < 2^56 (INV-STAMP-2)
        debug_assert!(stamp < (1 << 56), "INV-STAMP-2: Stamp Überlauf");
        Self((stamp << 8) | (kind.as_u8() as u64))
    }

    /// Liefert den Stamp.
    pub const fn stamp(self) -> u64 {
        self.0 >> 8
    }

    /// Liefert das Kind.
    pub fn kind(self) -> Option<TagKind> {
        TagKind::from_u8((self.0 & 0xFF) as u8)
    }

    /// Liefert die Rohbytes in Little-Endian.
    pub const fn to_le_bytes(self) -> [u8; 8] {
        self.0.to_le_bytes()
    }

    /// Liest ein Tag aus Little-Endian-Bytes.
    pub const fn from_le_bytes(bytes: [u8; 8]) -> Self {
        Self(u64::from_le_bytes(bytes))
    }
}

/// Interner Schlüssel: Nutzerschlüssel aufsteigend, Tag absteigend.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InternalKey {
    /// Nutzerschlüssel.
    pub user_key: Vec<u8>,
    /// Tag (Stamp + Kind).
    pub tag: Tag,
}

impl InternalKey {
    /// Erstellt einen neuen internen Schlüssel.
    pub fn new(user_key: Vec<u8>, tag: Tag) -> Self {
        Self { user_key, tag }
    }
}

impl Ord for InternalKey {
    fn cmp(&self, other: &Self) -> Ordering {
        // Nutzerschlüssel aufsteigend
        match self.user_key.cmp(&other.user_key) {
            Ordering::Equal => {
                // Tag absteigend (neueste Version zuerst)
                other.tag.0.cmp(&self.tag.0)
            }
            ord => ord,
        }
    }
}

impl PartialOrd for InternalKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_golden_a3_tag() {
        // stamp = 1, Put (1)
        let t1 = Tag::new(1, TagKind::Put);
        assert_eq!(t1.to_le_bytes(), [0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        // stamp = 7, Delete (2)
        let t2 = Tag::new(7, TagKind::Delete);
        assert_eq!(t2.to_le_bytes(), [0x02, 0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        // stamp = 2^56 - 1, PutRef (5)
        let max_stamp = (1u64 << 56) - 1;
        let t3 = Tag::new(max_stamp, TagKind::PutRef);
        assert_eq!(t3.to_le_bytes(), [0x05, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);

        // Sortierung: ("k", tag(7, Delete)) steht VOR ("k", tag(1, Put))
        let k_del = InternalKey::new(b"k".to_vec(), t2);
        let k_put = InternalKey::new(b"k".to_vec(), t1);
        assert!(k_del < k_put);
    }
}
