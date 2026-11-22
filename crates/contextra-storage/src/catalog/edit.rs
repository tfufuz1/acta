//! # Katalog-Edit und TLV-Kodierung
//!
//! SPEC §9.7: Ein Edit ist ein einzelner Record aus TLV-Feldern (P-2).
//! Bit 0x8000 markiert optionale Felder; unbekannte Pflichtfelder führen zu Unsupported.

use crate::catalog::codec::*;
use crate::catalog::types::{
    CatalogError, CatalogFileKind, CatalogSpaceOptions, ErasePendingMeta, ErasedRangeMeta,
    FileMeta, HoldMeta,
};

// Feld-IDs exakt nach SPEC §9.7
/// Feld-ID 0x0001: Hinzugefügte Dateien (`[FileMeta]`).
pub const FIELD_ADD_FILES: u16 = 0x0001;
/// Feld-ID 0x0002: Entfernte Dateien (`[(kind, id)]`).
pub const FIELD_REMOVE_FILES: u16 = 0x0002;
/// Feld-ID 0x0003: Delta der Blob-Lebendbytes (`[(blob_no, delta)]`).
pub const FIELD_BLOB_LIVE_DELTA: u16 = 0x0003;
/// Feld-ID 0x0004: Generationstausch für Blobs nach Sweep (`[(blob_no, gen_alt, gen_neu)]`).
pub const FIELD_BLOB_REPLACE: u16 = 0x0004;
/// Feld-ID 0x0005: Geflushter Stamp je Space (`[(space, stamp)]`).
pub const FIELD_SPACE_FLUSHED: u16 = 0x0005;
/// Feld-ID 0x0006: Redo-Log-Grenzen (`floor | floor_stamp | hwm`).
pub const FIELD_REDO: u16 = 0x0006;
/// Feld-ID 0x0007: ID-Hochwassermarken (`run | blob | segment | tmp | keys_gen`).
pub const FIELD_RESERVE: u16 = 0x0007;
/// Feld-ID 0x0008: Neue Spaces (`[(id, name, options)]`).
pub const FIELD_SPACES_ADD: u16 = 0x0008;
/// Feld-ID 0x0009: Entfernte Spaces (`[id]`).
pub const FIELD_SPACES_DROP: u16 = 0x0009;
/// Feld-ID 0x000A: Consumer-Cursor aktualisieren (`[(name, stamp, max_lag)]`).
pub const FIELD_CONSUMER_SET: u16 = 0x000A;
/// Feld-ID 0x000B: Consumer-Cursor löschen (`[name]`).
pub const FIELD_CONSUMER_DROP: u16 = 0x000B;
/// Feld-ID 0x000C: Nachweis vernichteter Domänenschlüssel (`[(domain, stamp)]`).
pub const FIELD_DOMAIN_RETIRED: u16 = 0x000C;
/// Feld-ID 0x000D: Ausstehende physische Löschung hinzufügen (`[(id, space, range, level)]`).
pub const FIELD_ERASE_PENDING_ADD: u16 = 0x000D;
/// Feld-ID 0x000E: Löschung abgeschlossen (`[id]`).
pub const FIELD_ERASE_DONE: u16 = 0x000E;
/// Feld-ID 0x000F: Löschung durch Blob-Sweep erfasst (`[id]`).
pub const FIELD_ERASE_SWEPT: u16 = 0x000F;
/// Feld-ID 0x0010: Gelöschten Bereich registrieren (`[(space, range, stamp)]`).
pub const FIELD_ERASED_RANGES_ADD: u16 = 0x0010;
/// Feld-ID 0x0011: Gelöschten Bereich deregistrieren (`[(space, range, stamp)]`).
pub const FIELD_ERASED_RANGES_REMOVE: u16 = 0x0011;
/// Feld-ID 0x0012: Dauerhaften Hold setzen (`[(hold_id, stamp, owner, expires_ms)]`).
pub const FIELD_HOLD_SET: u16 = 0x0012;
/// Feld-ID 0x0013: Dauerhaften Hold aufheben (`[hold_id]`).
pub const FIELD_HOLD_DROP: u16 = 0x0013;
/// Feld-ID 0x0014: Schlüsselepoch je Space inkrementieren (`[(space, epoch)]`).
pub const FIELD_KEY_EPOCH: u16 = 0x0014;

/// Bit-Maske für optionale Felder (SPEC §9.7).
pub const FIELD_OPTIONAL_FLAG: u16 = 0x8000;

/// Ein atomarer Katalog-Edit nach SPEC §9.7 (P-2).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CatalogEdit {
    /// Hinzugefügte Dateien (0x0001).
    pub add_files: Vec<FileMeta>,
    /// Entfernte Dateien (0x0002).
    pub remove_files: Vec<(CatalogFileKind, u64)>,
    /// Delta der Lebendbytes von Blobs (0x0003).
    pub blob_live_delta: Vec<(u64, i64)>,
    /// Generationstausch für Blobs (0x0004).
    pub blob_replace: Vec<(u64, u32, u32)>,
    /// Flush-Fortschritt je Space (0x0005).
    pub space_flushed: Vec<(u16, u64)>,
    /// Redo-Log-Grenzen (0x0006).
    pub redo: Option<(u64, u64, u64)>,
    /// ID-Hochwassermarken (0x0007).
    pub reserve: Option<(u64, u64, u64, u64, u64)>,
    /// Hinzugefügte Spaces (0x0008).
    pub spaces_add: Vec<(u16, Vec<u8>, CatalogSpaceOptions)>,
    /// Entfernte Spaces (0x0009).
    pub spaces_drop: Vec<u16>,
    /// Gesetzte Consumer-Cursor (0x000A).
    pub consumer_set: Vec<(Vec<u8>, u64, u64)>,
    /// Entfernte Consumer-Cursor (0x000B).
    pub consumer_drop: Vec<Vec<u8>>,
    /// Vernichtete Domänenschlüssel (0x000C).
    pub domain_retired: Vec<(u32, u64)>,
    /// Neue anhängige physische Löschungen (0x000D).
    pub erase_pending_add: Vec<ErasePendingMeta>,
    /// Abgeschlossene physische Löschungen (0x000E).
    pub erase_done: Vec<u64>,
    /// Im Sweep abgearbeitete Löschungen (0x000F).
    pub erase_swept: Vec<u64>,
    /// Neue gelöschte Schlüsselbereiche (0x0010).
    pub erased_ranges_add: Vec<ErasedRangeMeta>,
    /// Bereinigte gelöschte Schlüsselbereiche (0x0011).
    pub erased_ranges_remove: Vec<ErasedRangeMeta>,
    /// Gesetzte dauerhafte Holds (0x0012).
    pub hold_set: Vec<HoldMeta>,
    /// Aufgehobene Holds (0x0013).
    pub hold_drop: Vec<u64>,
    /// Schlüsselepoch je Space (0x0014).
    pub key_epoch: Vec<(u16, u32)>,
}

impl CatalogEdit {
    /// Kodiert den Edit in die TLV-Bytes eines Records (SPEC §9.7).
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::new();

        let mut write_tlv = |field_id: u16, payload: &[u8]| {
            if payload.is_empty() { return; }
            out.extend_from_slice(&field_id.to_le_bytes());
            let len_u64 = u64::try_from(payload.len()).unwrap_or(0);
            put_varint(&mut out, len_u64);
            out.extend_from_slice(payload);
        };

        // 0x0001 add_files
        if !self.add_files.is_empty() {
            let mut p = Vec::new();
            put_varint(&mut p, u64::try_from(self.add_files.len()).unwrap_or(0));
            for f in &self.add_files {
                p.push(f.kind.to_u8());
                p.extend_from_slice(&f.id.to_le_bytes());
                p.extend_from_slice(&f.space.to_le_bytes());
                p.push(f.level);
                p.push(f.tier);
                p.extend_from_slice(&f.flags.to_le_bytes());
                p.extend_from_slice(&f.min_stamp.to_le_bytes());
                p.extend_from_slice(&f.max_stamp.to_le_bytes());
                p.extend_from_slice(&f.bytes.to_le_bytes());
                p.extend_from_slice(&f.entries.to_le_bytes());
                p.extend_from_slice(&f.generation.to_le_bytes());
                p.extend_from_slice(&f.segment_kind.to_le_bytes());
                p.extend_from_slice(&f.blob_no_min.to_le_bytes());
                p.extend_from_slice(&f.blob_no_max.to_le_bytes());
                put_bytes(&mut p, &f.min_key);
                put_bytes(&mut p, &f.max_key);
            }
            write_tlv(FIELD_ADD_FILES, &p);
        }

        // 0x0002 remove_files
        if !self.remove_files.is_empty() {
            let mut p = Vec::new();
            put_varint(&mut p, u64::try_from(self.remove_files.len()).unwrap_or(0));
            for (k, id) in &self.remove_files {
                p.push(k.to_u8());
                p.extend_from_slice(&id.to_le_bytes());
            }
            write_tlv(FIELD_REMOVE_FILES, &p);
        }

        // 0x0003 blob_live_delta
        if !self.blob_live_delta.is_empty() {
            let mut p = Vec::new();
            put_varint(&mut p, u64::try_from(self.blob_live_delta.len()).unwrap_or(0));
            for (b, d) in &self.blob_live_delta {
                p.extend_from_slice(&b.to_le_bytes());
                p.extend_from_slice(&d.to_le_bytes());
            }
            write_tlv(FIELD_BLOB_LIVE_DELTA, &p);
        }

        // 0x0004 blob_replace
        if !self.blob_replace.is_empty() {
            let mut p = Vec::new();
            put_varint(&mut p, u64::try_from(self.blob_replace.len()).unwrap_or(0));
            for (b, ga, gn) in &self.blob_replace {
                p.extend_from_slice(&b.to_le_bytes());
                p.extend_from_slice(&ga.to_le_bytes());
                p.extend_from_slice(&gn.to_le_bytes());
            }
            write_tlv(FIELD_BLOB_REPLACE, &p);
        }

        // 0x0005 space_flushed
        if !self.space_flushed.is_empty() {
            let mut p = Vec::new();
            put_varint(&mut p, u64::try_from(self.space_flushed.len()).unwrap_or(0));
            for (s, st) in &self.space_flushed {
                p.extend_from_slice(&s.to_le_bytes());
                p.extend_from_slice(&st.to_le_bytes());
            }
            write_tlv(FIELD_SPACE_FLUSHED, &p);
        }

        // 0x0006 redo
        if let Some((floor, floor_st, hwm)) = self.redo {
            let mut p = Vec::new();
            p.extend_from_slice(&floor.to_le_bytes());
            p.extend_from_slice(&floor_st.to_le_bytes());
            p.extend_from_slice(&hwm.to_le_bytes());
            write_tlv(FIELD_REDO, &p);
        }

        // 0x0007 reserve
        if let Some((r_hwm, b_hwm, s_hwm, t_hwm, k_gen)) = self.reserve {
            let mut p = Vec::new();
            p.extend_from_slice(&r_hwm.to_le_bytes());
            p.extend_from_slice(&b_hwm.to_le_bytes());
            p.extend_from_slice(&s_hwm.to_le_bytes());
            p.extend_from_slice(&t_hwm.to_le_bytes());
            p.extend_from_slice(&k_gen.to_le_bytes());
            write_tlv(FIELD_RESERVE, &p);
        }

        // 0x0008 spaces_add
        if !self.spaces_add.is_empty() {
            let mut p = Vec::new();
            put_varint(&mut p, u64::try_from(self.spaces_add.len()).unwrap_or(0));
            for (id, name, opt) in &self.spaces_add {
                p.extend_from_slice(&id.to_le_bytes());
                put_bytes(&mut p, name);
                p.push(opt.header_len);
                p.push(opt.weights_k);
                p.extend_from_slice(&opt.flags.to_le_bytes());
                p.extend_from_slice(&opt.separate_threshold.to_le_bytes());
                p.push(opt.prefix_filter);
                p.push(opt.locality_prefix);
            }
            write_tlv(FIELD_SPACES_ADD, &p);
        }

        // 0x0009 spaces_drop
        if !self.spaces_drop.is_empty() {
            let mut p = Vec::new();
            put_varint(&mut p, u64::try_from(self.spaces_drop.len()).unwrap_or(0));
            for s in &self.spaces_drop {
                p.extend_from_slice(&s.to_le_bytes());
            }
            write_tlv(FIELD_SPACES_DROP, &p);
        }

        // 0x000A consumer_set
        if !self.consumer_set.is_empty() {
            let mut p = Vec::new();
            put_varint(&mut p, u64::try_from(self.consumer_set.len()).unwrap_or(0));
            for (name, c_stamp, max_lag) in &self.consumer_set {
                put_bytes(&mut p, name);
                p.extend_from_slice(&c_stamp.to_le_bytes());
                p.extend_from_slice(&max_lag.to_le_bytes());
            }
            write_tlv(FIELD_CONSUMER_SET, &p);
        }

        // 0x000B consumer_drop
        if !self.consumer_drop.is_empty() {
            let mut p = Vec::new();
            put_varint(&mut p, u64::try_from(self.consumer_drop.len()).unwrap_or(0));
            for name in &self.consumer_drop {
                put_bytes(&mut p, name);
            }
            write_tlv(FIELD_CONSUMER_DROP, &p);
        }

        // 0x000C domain_retired
        if !self.domain_retired.is_empty() {
            let mut p = Vec::new();
            put_varint(&mut p, u64::try_from(self.domain_retired.len()).unwrap_or(0));
            for (d, st) in &self.domain_retired {
                p.extend_from_slice(&d.to_le_bytes());
                p.extend_from_slice(&st.to_le_bytes());
            }
            write_tlv(FIELD_DOMAIN_RETIRED, &p);
        }

        // 0x000D erase_pending_add
        if !self.erase_pending_add.is_empty() {
            let mut p = Vec::new();
            put_varint(&mut p, u64::try_from(self.erase_pending_add.len()).unwrap_or(0));
            for ep in &self.erase_pending_add {
                p.extend_from_slice(&ep.id.to_le_bytes());
                p.extend_from_slice(&ep.space.to_le_bytes());
                put_range(&mut p, &ep.range);
                p.push(ep.level);
            }
            write_tlv(FIELD_ERASE_PENDING_ADD, &p);
        }

        // 0x000E erase_done
        if !self.erase_done.is_empty() {
            let mut p = Vec::new();
            put_varint(&mut p, u64::try_from(self.erase_done.len()).unwrap_or(0));
            for id in &self.erase_done {
                p.extend_from_slice(&id.to_le_bytes());
            }
            write_tlv(FIELD_ERASE_DONE, &p);
        }

        // 0x000F erase_swept
        if !self.erase_swept.is_empty() {
            let mut p = Vec::new();
            put_varint(&mut p, u64::try_from(self.erase_swept.len()).unwrap_or(0));
            for id in &self.erase_swept {
                p.extend_from_slice(&id.to_le_bytes());
            }
            write_tlv(FIELD_ERASE_SWEPT, &p);
        }

        // 0x0010 erased_ranges_add
        if !self.erased_ranges_add.is_empty() {
            let mut p = Vec::new();
            put_varint(&mut p, u64::try_from(self.erased_ranges_add.len()).unwrap_or(0));
            for er in &self.erased_ranges_add {
                p.extend_from_slice(&er.space.to_le_bytes());
                put_range(&mut p, &er.range);
                p.extend_from_slice(&er.stamp_s.to_le_bytes());
            }
            write_tlv(FIELD_ERASED_RANGES_ADD, &p);
        }

        // 0x0011 erased_ranges_remove
        if !self.erased_ranges_remove.is_empty() {
            let mut p = Vec::new();
            put_varint(&mut p, u64::try_from(self.erased_ranges_remove.len()).unwrap_or(0));
            for er in &self.erased_ranges_remove {
                p.extend_from_slice(&er.space.to_le_bytes());
                put_range(&mut p, &er.range);
                p.extend_from_slice(&er.stamp_s.to_le_bytes());
            }
            write_tlv(FIELD_ERASED_RANGES_REMOVE, &p);
        }

        // 0x0012 hold_set
        if !self.hold_set.is_empty() {
            let mut p = Vec::new();
            put_varint(&mut p, u64::try_from(self.hold_set.len()).unwrap_or(0));
            for h in &self.hold_set {
                p.extend_from_slice(&h.hold_id.to_le_bytes());
                p.extend_from_slice(&h.stamp.to_le_bytes());
                put_bytes(&mut p, &h.owner);
                p.extend_from_slice(&h.expires_ms.to_le_bytes());
            }
            write_tlv(FIELD_HOLD_SET, &p);
        }

        // 0x0013 hold_drop
        if !self.hold_drop.is_empty() {
            let mut p = Vec::new();
            put_varint(&mut p, u64::try_from(self.hold_drop.len()).unwrap_or(0));
            for h in &self.hold_drop {
                p.extend_from_slice(&h.to_le_bytes());
            }
            write_tlv(FIELD_HOLD_DROP, &p);
        }

        // 0x0014 key_epoch
        if !self.key_epoch.is_empty() {
            let mut p = Vec::new();
            put_varint(&mut p, u64::try_from(self.key_epoch.len()).unwrap_or(0));
            for (s, ep) in &self.key_epoch {
                p.extend_from_slice(&s.to_le_bytes());
                p.extend_from_slice(&ep.to_le_bytes());
            }
            write_tlv(FIELD_KEY_EPOCH, &p);
        }

        out
    }

    /// Dekodiert einen Edit aus TLV-Bytes.
    ///
    /// Gesetztes Bit 0x8000 markiert ein optionales Feld (unbekannt => überspringen).
    /// Unbekannte Pflichtfelder führen zu CatalogError::UnsupportedField.
    pub fn decode(mut buf: &[u8]) -> Result<Self, CatalogError> {
        let mut edit = Self::default();

        while !buf.is_empty() {
            if buf.len() < 2 {
                return Err(CatalogError::UnexpectedEof);
            }
            let (fid_bytes, r) = buf.split_at(2);
            let mut fb = [0u8; 2];
            fb.copy_from_slice(fid_bytes);
            let field_id = u16::from_le_bytes(fb);

            let (len_u64, consumed) = get_varint(r)?;
            let (_, r2) = r.split_at(consumed);
            let len = usize::try_from(len_u64)
                .map_err(|_| CatalogError::InvalidFormat("Längenüberlauf"))?;

            if r2.len() < len {
                return Err(CatalogError::UnexpectedEof);
            }
            let (payload, remaining) = r2.split_at(len);
            buf = remaining;

            let is_optional = (field_id & FIELD_OPTIONAL_FLAG) != 0;
            let actual_id = field_id & !FIELD_OPTIONAL_FLAG;

            match actual_id {
                FIELD_ADD_FILES => decode_add_files(&mut edit, payload)?,
                FIELD_REMOVE_FILES => decode_remove_files(&mut edit, payload)?,
                FIELD_BLOB_LIVE_DELTA => decode_blob_live_delta(&mut edit, payload)?,
                FIELD_BLOB_REPLACE => decode_blob_replace(&mut edit, payload)?,
                FIELD_SPACE_FLUSHED => decode_space_flushed(&mut edit, payload)?,
                FIELD_REDO => decode_redo(&mut edit, payload)?,
                FIELD_RESERVE => decode_reserve(&mut edit, payload)?,
                FIELD_SPACES_ADD => decode_spaces_add(&mut edit, payload)?,
                FIELD_SPACES_DROP => decode_spaces_drop(&mut edit, payload)?,
                FIELD_CONSUMER_SET => decode_consumer_set(&mut edit, payload)?,
                FIELD_CONSUMER_DROP => decode_consumer_drop(&mut edit, payload)?,
                FIELD_DOMAIN_RETIRED => decode_domain_retired(&mut edit, payload)?,
                FIELD_ERASE_PENDING_ADD => decode_erase_pending_add(&mut edit, payload)?,
                FIELD_ERASE_DONE => decode_erase_done(&mut edit, payload)?,
                FIELD_ERASE_SWEPT => decode_erase_swept(&mut edit, payload)?,
                FIELD_ERASED_RANGES_ADD => decode_erased_ranges_add(&mut edit, payload)?,
                FIELD_ERASED_RANGES_REMOVE => decode_erased_ranges_remove(&mut edit, payload)?,
                FIELD_HOLD_SET => decode_hold_set(&mut edit, payload)?,
                FIELD_HOLD_DROP => decode_hold_drop(&mut edit, payload)?,
                FIELD_KEY_EPOCH => decode_key_epoch(&mut edit, payload)?,
                _ => {
                    if !is_optional {
                        return Err(CatalogError::UnsupportedField(field_id));
                    }
                }
            }
        }

        Ok(edit)
    }
}
