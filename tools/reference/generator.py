#!/usr/bin/env python3
"""
Unabhängiger Python-Referenzcodierer für SPEC.md (Revision R2, Anhang A).
Berechnet alle Goldens deterministisch und gibt .hex-Dateien aus.
"""

import os
import struct

# --- A.1 CRC32C (Castagnoli, Polynom 0x82F63B78 reflektiert) ---

POLY_CRC32C = 0x82F63B78

def make_crc32c_table():
    table = []
    for i in range(256):
        crc = i
        for _ in range(8):
            if crc & 1:
                crc = (crc >> 1) ^ POLY_CRC32C
            else:
                crc >>= 1
        table.append(crc)
    return table

CRC32C_TABLE = make_crc32c_table()

def crc32c(seed: int, data: bytes) -> int:
    crc = (~seed) & 0xFFFFFFFF
    for b in data:
        crc = (crc >> 8) ^ CRC32C_TABLE[(crc ^ b) & 0xFF]
    return (~crc) & 0xFFFFFFFF

# --- A.1 XXH64 ---

PRIME64_1 = 0x9E3779B185EBCA87
PRIME64_2 = 0xC2B2AE3D27D4EB4F
PRIME64_3 = 0x165667B19E3779F9
PRIME64_4 = 0x85EBCA77C2B2AE63
PRIME64_5 = 0x27D4EB2F165667C5
MASK64 = 0xFFFFFFFFFFFFFFFF

def rol64(val: int, bits: int) -> int:
    return ((val << bits) | (val >> (64 - bits))) & MASK64

def xxh64_round(acc: int, input_val: int) -> int:
    acc = (acc + input_val * PRIME64_2) & MASK64
    acc = rol64(acc, 31)
    acc = (acc * PRIME64_1) & MASK64
    return acc

def xxh64_merge_round(acc: int, val: int) -> int:
    val = xxh64_round(0, val)
    acc = (acc ^ val) & MASK64
    acc = (acc * PRIME64_1 + PRIME64_4) & MASK64
    return acc

def xxh64(data: bytes, seed: int = 0) -> int:
    length = len(data)
    if length >= 32:
        v1 = (seed + PRIME64_1 + PRIME64_2) & MASK64
        v2 = (seed + PRIME64_2) & MASK64
        v3 = seed & MASK64
        v4 = (seed - PRIME64_1) & MASK64

        offset = 0
        while offset + 32 <= length:
            v1 = xxh64_round(v1, struct.unpack_from("<Q", data, offset)[0])
            v2 = xxh64_round(v2, struct.unpack_from("<Q", data, offset + 8)[0])
            v3 = xxh64_round(v3, struct.unpack_from("<Q", data, offset + 16)[0])
            v4 = xxh64_round(v4, struct.unpack_from("<Q", data, offset + 24)[0])
            offset += 32

        h64 = (rol64(v1, 1) + rol64(v2, 7) + rol64(v3, 12) + rol64(v4, 18)) & MASK64
        h64 = xxh64_merge_round(h64, v1)
        h64 = xxh64_merge_round(h64, v2)
        h64 = xxh64_merge_round(h64, v3)
        h64 = xxh64_merge_round(h64, v4)
    else:
        h64 = (seed + PRIME64_5) & MASK64

    h64 = (h64 + length) & MASK64

    rem = length % 32
    offset = length - rem

    while rem >= 8:
        k1 = xxh64_round(0, struct.unpack_from("<Q", data, offset)[0])
        h64 = (rol64(h64 ^ k1, 27) * PRIME64_1 + PRIME64_4) & MASK64
        offset += 8
        rem -= 8

    if rem >= 4:
        k1 = (struct.unpack_from("<I", data, offset)[0] * PRIME64_1) & MASK64
        h64 = (rol64(h64 ^ k1, 23) * PRIME64_2 + PRIME64_3) & MASK64
        offset += 4
        rem -= 4

    while rem > 0:
        k1 = (data[offset] * PRIME64_5) & MASK64
        h64 = (rol64(h64 ^ k1, 11) * PRIME64_1) & MASK64
        offset += 1
        rem -= 1

    h64 ^= h64 >> 33
    h64 = (h64 * PRIME64_2) & MASK64
    h64 ^= h64 >> 29
    h64 = (h64 * PRIME64_3) & MASK64
    h64 ^= h64 >> 32

    return h64

# --- A.2 Kanonischer Varint (LEB128 für u64) ---

def encode_varint(val: int) -> bytes:
    res = bytearray()
    while val >= 0x80:
        res.append((val & 0x7F) | 0x80)
        val >>= 7
    res.append(val & 0x7F)
    return bytes(res)

# --- A.3 Tag ---

def make_tag(stamp: int, kind: int) -> bytes:
    tag_val = (stamp << 8) | (kind & 0xFF)
    return struct.pack("<Q", tag_val)

# --- A.4 FORMAT (Plain, 51 Bytes) ---

def make_format_plain() -> bytes:
    version = 1
    required = 0
    optional = 0
    integrity_mode = 0  # Plain
    instance_id = bytes(range(16))
    created_ms = 1_700_000_000_000

    header = bytearray()
    header.extend(b"CTXS")
    header.extend(struct.pack("<H", version))
    header.extend(struct.pack("<Q", required))
    header.extend(struct.pack("<Q", optional))
    header.append(integrity_mode)
    header.extend(instance_id)
    header.extend(struct.pack("<Q", created_ms))

    chk = crc32c(0, header)
    header.extend(struct.pack("<I", chk))
    return bytes(header)

# --- A.5 Redo-Frame und Seal-Frame (Plain) ---

def make_redo_frame_plain() -> bytes:
    chain_seed = 0xA5A5A5A5
    # Op: kind(1) space(1) opflags(0) klen(1) key("k") vlen(1) value("v")
    op = bytearray([0x01, 0x01, 0x00, 0x00, 0x01, ord('k'), 0x01, ord('v')])
    len_field = 13 + len(op)  # 21 = 0x15
    stamp = 1
    op_count = 1
    flags = 0

    frame = bytearray()
    frame.extend(struct.pack("<I", len_field))
    frame.extend(struct.pack("<Q", stamp))
    frame.extend(struct.pack("<I", op_count))
    frame.append(flags)
    frame.extend(op)

    chk = crc32c(chain_seed, frame)
    frame.extend(struct.pack("<I", chk))
    return bytes(frame)

def make_redo_seal_plain() -> bytes:
    # Seal-Frame nach Frame 1
    # op_count = 0xFFFFFFFF, flags.SEAL = 0x80, payload = frame_count(1u64) || final_chain(0x6F638DC4)
    # len = 13 + 12 = 25
    prev_chk = 0x6F638DC4
    len_field = 25
    stamp = 1
    op_count = 0xFFFFFFFF
    flags = 0x80
    frame_count = 1
    final_chain = 0x6F638DC4

    seal = bytearray()
    seal.extend(struct.pack("<I", len_field))
    seal.extend(struct.pack("<Q", stamp))
    seal.extend(struct.pack("<I", op_count))
    seal.append(flags)
    seal.extend(struct.pack("<Q", frame_count))
    seal.extend(struct.pack("<I", final_chain))

    chk = crc32c(prev_chk, seal)
    seal.extend(struct.pack("<I", chk))
    return bytes(seal)

# --- A.6 BlobRef und Nonces ---

def make_blob_ref() -> bytes:
    blob_no = 0x71
    offset = 4096
    length = 1000
    crc = 0xDEADBEEF
    domain = 42
    return struct.pack("<QQII I", blob_no, offset, length, crc, domain)

def make_nonces() -> dict:
    nonces = {}
    # Run-Block: 0x01 || block_offset u64 || 0u24
    b = bytearray([0x01])
    b.extend(struct.pack("<Q", 4096))
    b.extend(b"\x00\x00\x00")
    nonces["run_block"] = bytes(b)

    # Redo-Frame: 0x02 || frame_index u32 || 0u56
    b = bytearray([0x02])
    b.extend(struct.pack("<I", 3))
    b.extend(b"\x00" * 7)
    nonces["redo_frame"] = bytes(b)

    # Redo-Op-Körper: 0x03 || segment_id u32 || frame_index u32 || op_index u24
    b = bytearray([0x03])
    b.extend(struct.pack("<I", 0x12))
    b.extend(struct.pack("<I", 3))
    b.extend(struct.pack("<I", 1)[:3])
    nonces["redo_op_body"] = bytes(b)

    # Blob-Record: 0x04 || offset u64 || 0u24
    b = bytearray([0x04])
    b.extend(struct.pack("<Q", 4096))
    b.extend(b"\x00\x00\x00")
    nonces["blob_record"] = bytes(b)

    # Blob-Körper (Domäne): 0x05 || blob_no u56 || offset u32
    b = bytearray([0x05])
    b.extend(struct.pack("<Q", 0x71)[:7])
    b.extend(struct.pack("<I", 4096))
    nonces["blob_body_domain"] = bytes(b)

    # Catalog-Record: record_index u64 || 0u32
    b = bytearray()
    b.extend(struct.pack("<Q", 2))
    b.extend(b"\x00\x00\x00\x00")
    nonces["catalog_record"] = bytes(b)

    return nonces

# --- A.7 ResumeToken ---

def make_resume_token_plain() -> bytes:
    # "CTXT" | version u8 (1) | space u16 | dir u8 (0 Fwd) | stamp u64 | range_hash u64 | klen varint | last_key | chk
    magic = b"CTXT"
    version = 1
    space = 1
    direction = 0  # Fwd
    stamp = 42
    range_hash = xxh64(b"prefix/", 0)
    last_key = b"prefix/item-1"

    token = bytearray(magic)
    token.append(version)
    token.extend(struct.pack("<H", space))
    token.append(direction)
    token.extend(struct.pack("<Q", stamp))
    token.extend(struct.pack("<Q", range_hash))
    token.extend(encode_varint(len(last_key)))
    token.extend(last_key)

    chk = crc32c(0, token)
    token.extend(struct.pack("<I", chk))
    return bytes(token)

# --- Selbsttests & Generierung ---

def run_self_tests():
    # A.1 CRC32C
    assert crc32c(0, b"123456789") == 0xE3069283, "CRC32C 123456789 failed"
    assert crc32c(0, b"") == 0x00000000, "CRC32C empty failed"
    assert crc32c(0, b"a") == 0xC1D04330, "CRC32C 'a' failed"
    assert crc32c(crc32c(0, b"1234"), b"56789") == 0xE3069283, "CRC32C chaining failed"

    # A.1 XXH64
    assert xxh64(b"", 0) == 0xEF46DB3751D8E999, "XXH64 empty failed"
    assert xxh64(b"a", 0) == 0xD24EC4F1A98C6E5B, "XXH64 'a' failed"
    assert xxh64(b"abc", 0) == 0x44BC2CF5AD770999, "XXH64 'abc' failed"

    # A.4 FORMAT
    fmt = make_format_plain()
    assert fmt.hex() == "4354585301000000000000000000000000000000000000000102030405060708090a0b0c0d0e0f0068e5cf8b0100005663b886", "FORMAT mismatch"

    # A.5 Redo Frame
    frame = make_redo_frame_plain()
    assert frame.hex() == "150000000100000000000000010000000001010000016b0176c48d636f", "Redo frame mismatch"

    # A.5 Redo Seal
    seal = make_redo_seal_plain()
    assert seal.hex() == "190000000100000000000000ffffffff800100000000000000c48d636fdb7083de", "Redo seal mismatch"

    # A.6 Bloom
    h1 = xxh64(b"k", 0)
    h2 = xxh64(b"k", 0x9E3779B97F4A7C15) | 1
    pos = [(h1 + i * h2) % 64 for i in range(7)]
    assert pos == [35, 60, 21, 46, 7, 32, 57], f"Bloom pos mismatch: {pos}"

    # A.6 Nonces
    nonces = make_nonces()
    assert nonces["run_block"].hex() == "010010000000000000000000"
    assert nonces["redo_frame"].hex() == "020300000000000000000000"
    assert nonces["redo_op_body"].hex() == "031200000003000000010000"
    assert nonces["blob_record"].hex() == "040010000000000000000000"
    assert nonces["blob_body_domain"].hex() == "057100000000000000100000"
    assert nonces["catalog_record"].hex() == "020000000000000000000000"

    print("Alle Python-Selbsttests erfolgreich bestanden!")

def generate_golden_files(out_dir: str):
    os.makedirs(out_dir, exist_ok=True)

    files = {
        "a4_format_plain.hex": (make_format_plain(), "FORMAT-Datei Plain (51 Bytes, A.4)"),
        "a5_redo_frame_plain.hex": (make_redo_frame_plain(), "Redo-Frame Always Put space=1 key=k value=v (29 Bytes, A.5)"),
        "a5_redo_seal_plain.hex": (make_redo_seal_plain(), "Redo-Seal-Frame mit frame_count=1 final_chain (29 Bytes, A.5)"),
        "a6_blobref.hex": (make_blob_ref(), "BlobRef blob_no=0x71 offset=4096 len=1000 (28 Bytes, A.6)"),
        "a7_resumetoken_plain.hex": (make_resume_token_plain(), "ResumeToken Plain version=1 stamp=42 (A.7)"),
    }

    nonces = make_nonces()
    for name, b in nonces.items():
        files[f"a6_nonce_{name}.hex"] = (b, f"12-Byte Nonce für {name} (A.6, §15.2)")

    # Varints
    varint_vectors = [
        ("0", 0), ("1", 1), ("127", 127), ("128", 128), ("300", 300),
        ("16384", 16384), ("2p32", 1 << 32), ("2p64m1", (1 << 64) - 1)
    ]
    for label, val in varint_vectors:
        files[f"a2_varint_{label}.hex"] = (encode_varint(val), f"Varint LEB128 für {val} (A.2)")

    # Tags
    tags = [
        ("put_s1", make_tag(1, 1)),
        ("del_s7", make_tag(7, 2)),
        ("putref_max", make_tag((1 << 56) - 1, 5)),
    ]
    for label, t in tags:
        files[f"a3_tag_{label}.hex"] = (t, f"Tag für {label} (A.3)")

    for fname, (data, desc) in sorted(files.items()):
        path = os.path.join(out_dir, fname)
        with open(path, "w", encoding="utf-8") as f:
            f.write(f"# {desc}\n")
            f.write(data.hex() + "\n")
        print(f"Erzeugt: {fname} ({len(data)} Bytes)")

if __name__ == "__main__":
    run_self_tests()
    out_path = os.path.join(os.path.dirname(__file__), "golden")
    generate_golden_files(out_path)
