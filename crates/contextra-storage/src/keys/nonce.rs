//! # Zentrale Nonce-Tabelle
//!
//! Normative Regeln (SPEC §15.2, Golden A.6, SD-50):
//! Nonces haben genau 12 Bytes.
//! Jedes Präfix ist disjunkt (`0x01` bis `0x05`).

/// Erstellt die 12-Byte-Nonce für einen Run-Block.
///
/// Format: `0x01 || block_offset u64 || 0u24`
pub fn run_block_nonce(block_offset: u64) -> [u8; 12] {
    let mut nonce = [0u8; 12];
    nonce[0] = 0x01;
    nonce[1..9].copy_from_slice(&block_offset.to_le_bytes());
    // bytes 9..12 bleiben 0
    nonce
}

/// Erstellt die 12-Byte-Nonce für einen Redo-Frame.
///
/// Format: `0x02 || frame_index u32 || 0u56`
pub fn redo_frame_nonce(frame_index: u32) -> [u8; 12] {
    let mut nonce = [0u8; 12];
    nonce[0] = 0x02;
    nonce[1..5].copy_from_slice(&frame_index.to_le_bytes());
    // bytes 5..12 bleiben 0
    nonce
}

/// Erstellt die 12-Byte-Nonce für einen domänenversiegelten Redo-Op-Körper.
///
/// Format: `0x03 || segment_id u32 || frame_index u32 || op_index u24`
pub fn redo_op_body_nonce(segment_id: u32, frame_index: u32, op_index: u32) -> [u8; 12] {
    let mut nonce = [0u8; 12];
    nonce[0] = 0x03;
    nonce[1..5].copy_from_slice(&segment_id.to_le_bytes());
    nonce[5..9].copy_from_slice(&frame_index.to_le_bytes());
    let op_bytes = op_index.to_le_bytes();
    nonce[9..12].copy_from_slice(&op_bytes[..3]);
    nonce
}

/// Erstellt die 12-Byte-Nonce für einen Blob-Record.
///
/// Format: `0x04 || offset u64 || 0u24`
pub fn blob_record_nonce(offset: u64) -> [u8; 12] {
    let mut nonce = [0u8; 12];
    nonce[0] = 0x04;
    nonce[1..9].copy_from_slice(&offset.to_le_bytes());
    // bytes 9..12 bleiben 0
    nonce
}

/// Erstellt die 12-Byte-Nonce für einen domänenversiegelten Blob-Körper.
///
/// Format: `0x05 || blob_no u56 || offset u32`
pub fn blob_body_nonce(blob_no: u64, offset: u32) -> [u8; 12] {
    let mut nonce = [0u8; 12];
    nonce[0] = 0x05;
    let b_bytes = blob_no.to_le_bytes();
    nonce[1..8].copy_from_slice(&b_bytes[..7]); // u56
    nonce[8..12].copy_from_slice(&offset.to_le_bytes()); // u32
    nonce
}

/// Erstellt die 12-Byte-Nonce für einen Catalog-Record.
///
/// Format: `record_index u64 || 0u32`
pub fn catalog_record_nonce(record_index: u64) -> [u8; 12] {
    let mut nonce = [0u8; 12];
    nonce[0..8].copy_from_slice(&record_index.to_le_bytes());
    // bytes 8..12 bleiben 0
    nonce
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_golden_a6_nonces() {
        // Run-Block: block_offset = 4096
        assert_eq!(
            run_block_nonce(4096),
            [0x01, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
        );

        // Redo-Frame: frame_index = 3
        assert_eq!(
            redo_frame_nonce(3),
            [0x02, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
        );

        // Redo-Op-Körper: segment_id = 0x12, frame = 3, op = 1
        assert_eq!(
            redo_op_body_nonce(0x12, 3, 1),
            [0x03, 0x12, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00]
        );

        // Blob-Record: offset = 4096
        assert_eq!(
            blob_record_nonce(4096),
            [0x04, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
        );

        // Blob-Körper (Domäne): blob_no = 0x71, offset = 4096
        assert_eq!(
            blob_body_nonce(0x71, 4096),
            [0x05, 0x71, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00]
        );

        // Catalog-Record: record_index = 2
        assert_eq!(
            catalog_record_nonce(2),
            [0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
        );
    }
}
