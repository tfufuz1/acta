#![allow(clippy::panic, clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]

use contextra_storage::codec::crc32c::crc32c;
use contextra_storage::codec::tag::{Tag, TagKind};
use contextra_storage::codec::varint::encode_u64;
use contextra_storage::keys::nonce::*;
use std::fs;
use std::path::Path;

fn read_hex_file(name: &str) -> Vec<u8> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let hex_path = manifest_dir.join("../../tools/reference/golden").join(name);
    let content = fs::read_to_string(&hex_path)
        .unwrap_or_else(|e| panic!("Konnte Golden-Datei {} nicht lesen: {}", hex_path.display(), e));

    let hex_line = content
        .lines()
        .find(|l| !l.starts_with('#') && !l.trim().is_empty())
        .expect("Hex line found");

    hex::decode(hex_line.trim()).expect("Valid hex string")
}

mod hex {
    pub fn decode(s: &str) -> Result<Vec<u8>, ()> {
        if s.len() % 2 != 0 {
            return Err(());
        }
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|_| ()))
            .collect()
    }
}

#[test]
fn test_python_goldens_format_plain() {
    let golden = read_hex_file("a4_format_plain.hex");

    let mut header = [0u8; 47];
    header[0..4].copy_from_slice(b"CTXS");
    header[4..6].copy_from_slice(&1u16.to_le_bytes());
    header[6..14].copy_from_slice(&0u64.to_le_bytes());
    header[14..22].copy_from_slice(&0u64.to_le_bytes());
    header[22] = 0; // Plain
    for i in 0..16 {
        header[23 + i] = i as u8;
    }
    header[39..47].copy_from_slice(&1_700_000_000_000u64.to_le_bytes());

    let chk = crc32c(0, &header);
    let mut rust_bytes = [0u8; 51];
    rust_bytes[..47].copy_from_slice(&header);
    rust_bytes[47..51].copy_from_slice(&chk.to_le_bytes());

    assert_eq!(rust_bytes.to_vec(), golden);
}

#[test]
fn test_python_goldens_redo_frame_and_seal() {
    let frame_golden = read_hex_file("a5_redo_frame_plain.hex");
    let seal_golden = read_hex_file("a5_redo_seal_plain.hex");

    // Frame
    let chain_seed: u32 = 0xA5A5A5A5;
    let op = [0x01, 0x01, 0x00, 0x00, 0x01, 0x6b, 0x01, 0x76];
    let mut frame = Vec::new();
    frame.extend_from_slice(&21u32.to_le_bytes());
    frame.extend_from_slice(&1u64.to_le_bytes());
    frame.extend_from_slice(&1u32.to_le_bytes());
    frame.push(0u8);
    frame.extend_from_slice(&op);
    let chk = crc32c(chain_seed, &frame);
    frame.extend_from_slice(&chk.to_le_bytes());

    assert_eq!(frame, frame_golden);

    // Seal
    let mut seal = Vec::new();
    seal.extend_from_slice(&25u32.to_le_bytes());
    seal.extend_from_slice(&1u64.to_le_bytes());
    seal.extend_from_slice(&0xFFFFFFFFu32.to_le_bytes());
    seal.push(0x80u8);
    seal.extend_from_slice(&1u64.to_le_bytes());
    seal.extend_from_slice(&0x6F638DC4u32.to_le_bytes());
    let seal_chk = crc32c(0x6F638DC4, &seal);
    seal.extend_from_slice(&seal_chk.to_le_bytes());

    assert_eq!(seal, seal_golden);
}

#[test]
fn test_python_goldens_blobref_and_nonces() {
    let blobref_golden = read_hex_file("a6_blobref.hex");
    let mut br = [0u8; 28];
    br[0..8].copy_from_slice(&0x71u64.to_le_bytes());
    br[8..16].copy_from_slice(&4096u64.to_le_bytes());
    br[16..20].copy_from_slice(&1000u32.to_le_bytes());
    br[20..24].copy_from_slice(&0xDEADBEEFu32.to_le_bytes());
    br[24..28].copy_from_slice(&42u32.to_le_bytes());
    assert_eq!(br.to_vec(), blobref_golden);

    // Nonces
    assert_eq!(run_block_nonce(4096).to_vec(), read_hex_file("a6_nonce_run_block.hex"));
    assert_eq!(redo_frame_nonce(3).to_vec(), read_hex_file("a6_nonce_redo_frame.hex"));
    assert_eq!(redo_op_body_nonce(0x12, 3, 1).to_vec(), read_hex_file("a6_nonce_redo_op_body.hex"));
    assert_eq!(blob_record_nonce(4096).to_vec(), read_hex_file("a6_nonce_blob_record.hex"));
    assert_eq!(blob_body_nonce(0x71, 4096).to_vec(), read_hex_file("a6_nonce_blob_body_domain.hex"));
    assert_eq!(catalog_record_nonce(2).to_vec(), read_hex_file("a6_nonce_catalog_record.hex"));
}

#[test]
fn test_python_goldens_varints_and_tags() {
    let varints = [
        ("a2_varint_0.hex", 0),
        ("a2_varint_1.hex", 1),
        ("a2_varint_127.hex", 127),
        ("a2_varint_128.hex", 128),
        ("a2_varint_300.hex", 300),
        ("a2_varint_16384.hex", 16384),
        ("a2_varint_2p32.hex", 1u64 << 32),
        ("a2_varint_2p64m1.hex", u64::MAX),
    ];
    for (f, val) in varints {
        let golden = read_hex_file(f);
        let mut buf = [0u8; 10];
        let len = encode_u64(val, &mut buf);
        assert_eq!(&buf[..len], &golden[..], "Varint mismatch for {}", f);
    }

    let tags = [
        ("a3_tag_put_s1.hex", Tag::new(1, TagKind::Put)),
        ("a3_tag_del_s7.hex", Tag::new(7, TagKind::Delete)),
        ("a3_tag_putref_max.hex", Tag::new((1u64 << 56) - 1, TagKind::PutRef)),
    ];
    for (f, tag) in tags {
        let golden = read_hex_file(f);
        assert_eq!(tag.to_le_bytes().to_vec(), golden, "Tag mismatch for {}", f);
    }
}
