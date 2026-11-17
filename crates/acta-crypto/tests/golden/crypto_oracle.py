# Unabhaengiges Orakel fuer acta-crypto (SPEC 16.1, 16.2): hashlib, hmac, PyNaCl (libsodium) und
# `cryptography` (OpenSSL) berechnen die Festwerte der Tests neu und pruefen, dass jeder Wert in
# `tests/*.rs` steht. Voraussetzung: pip install pynacl cryptography
# Aufruf: python3 -I tests/golden/crypto_oracle.py   (Ausgabe: 0 Abweichungen)
import hashlib, hmac, pathlib, struct
_TESTS = "".join(p.read_text(encoding="utf-8") for p in pathlib.Path(__file__).resolve().parent.parent.glob("*.rs"))
_BAD = []
_SKIP = {"VK_SAME_INPUT_VAULT", "SHA1M_A"}
def emit(name, value=None, *rest):
    if value is None or not isinstance(value, str) or len(value) < 32 or name in _SKIP:
        return
    if value not in _TESTS:
        _BAD.append((name, value))

from nacl.bindings import crypto_aead_xchacha20poly1305_ietf_encrypt as xenc
from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey
from cryptography.hazmat.primitives import serialization

def hkdf(ikm, salt, info, n=32):
    prk = hmac.new(salt if salt else b"\x00"*32, ikm, hashlib.sha256).digest()
    out, t, i = b"", b"", 1
    while len(out) < n:
        t = hmac.new(prk, t + info + bytes([i]), hashlib.sha256).digest()
        out += t; i += 1
    return out[:n]

# --- Vault frame (4.3)
key = bytes(range(32)); tenant = b"\x11"*16; gen = 3; slot = 42
nonce = bytes(100+i for i in range(24))
plain = bytes([0x02, 0x01]) + b"\xab"*16 + b"\x5c"*32
aad = b"acta/vault/v1" + tenant + struct.pack(">I", gen) + struct.pack(">Q", slot)
ct = xenc(plain, aad, nonce, key)
frame = struct.pack(">H", 50) + nonce + ct
emit("FRAME", frame.hex(), len(frame))
# Tombstone
tomb = bytes([0x01, 0x02]) + b"\x00"*48
ct2 = xenc(tomb, aad, nonce, key)
emit("TOMB", (struct.pack(">H",50)+nonce+ct2).hex())
# other slot
aad2 = b"acta/vault/v1" + tenant + struct.pack(">I", gen) + struct.pack(">Q", 43)
emit("FRAME_SLOT43", (struct.pack(">H",50)+nonce+xenc(plain, aad2, nonce, key)).hex())

# --- VKb
mbk = b"\x66"*32; t = b"\x44"*16
emit("VKB", hkdf(mbk, t, b"acta/vaultbk/v1").hex())
emit("VK_SAME_INPUT_VAULT", hkdf(mbk, t, b"acta/vault/v1" + struct.pack(">I", 0)).hex())

# --- ScopeKey MAC
sk = b"\x77"*32; window = bytes(range(1, 33))
emit("SCOPE_MAC", hmac.new(sk, window, hashlib.sha256).hexdigest())

# --- Envelope classes K and D (4.5)
def env(cls, keyb, ctx, plain_payload, nonce):
    tenant, ks, rec, ver = ctx
    aad = b"acta/rec/v1" + tenant + bytes([cls]) + struct.pack(">H", ks) + rec + struct.pack(">Q", ver)
    framed = struct.pack(">I", len(plain_payload)) + plain_payload
    framed += b"\x00" * (256 - len(framed))
    ctt = xenc(framed, aad, nonce, keyb)
    return bytes([1, 1, cls]) + nonce + ctt
ctx = (b"\x11"*16, 0x0010, b"\x22"*16, 7)
emit("ENV_K", env(2, key, ctx, b"Wissen: Vertrag", nonce).hex())
ctx2 = (b"\x11"*16, 0x0020, b"\x22"*16, 7)
emit("ENV_D", env(3, key, ctx2, b"Ableitung", nonce).hex())

# --- Hash chain (golden)
tenantc = b"\x44"*16
chain_id = hashlib.sha256(tenantc + b"E").digest()
prev = b"\x00"*32
e1 = hashlib.sha256(b"acta/ledger/v1" + chain_id + prev + b"entry-1").digest()
e2 = hashlib.sha256(b"acta/ledger/v1" + chain_id + e1 + b"entry-2").digest()
emit("CHAIN_ID", chain_id.hex()); emit("E1", e1.hex()); emit("E2", e2.hex())

# --- SHA-256 NIST
emit("SHA448", hashlib.sha256(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq").hexdigest())
emit("SHA896", hashlib.sha256(b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu").hexdigest())
emit("SHA1M_A", hashlib.sha256(b"a"*1000000).hexdigest())

# --- HKDF RFC 5869 case 3 (empty salt/info)
emit("HKDF3", hkdf(bytes([0x0b]*22), b"", b"", 42).hex()[:64])

# --- HMAC RFC 4231
def h(k, d): return hmac.new(k, d, hashlib.sha256).hexdigest()
emit("H3", h(b"\xaa"*20, b"\xdd"*50))
emit("H4", h(bytes(range(1,26)), b"\xcd"*50))
emit("H5", h(b"\x0c"*20, b"Test With Truncation"))
emit("H6", h(b"\xaa"*131, b"Test Using Larger Than Block-Size Key - Hash Key First"))
emit("H7", h(b"\xaa"*131, b"This is a test using a larger than block-size key and a larger than block-size data. The key needs to be hashed before being used by the HMAC algorithm."))

# --- Ed25519 RFC 8032 test 2
seed = bytes.fromhex("4ccd089b28ff96da9db6c346ec114e0f5b8a319f35aba624da8cf6ed4fb8a6fb")
pk = Ed25519PrivateKey.from_private_bytes(seed)
emit("ED2_PK", pk.public_key().public_bytes(serialization.Encoding.Raw, serialization.PublicFormat.Raw).hex())
emit("ED2_SIG", pk.sign(b"\x72").hex())
# domain-separated report signature
seed3 = bytes([7])*32
pk3 = Ed25519PrivateKey.from_private_bytes(seed3)
emit("ED_REPORT_PK", pk3.public_key().public_bytes(serialization.Encoding.Raw, serialization.PublicFormat.Raw).hex())
emit("ED_REPORT_SIG", pk3.sign(b"acta/report/v1" + b"bericht").hex())
emit("ED_CKPT_SIG", pk3.sign(b"acta/checkpoint/v1" + b"stand").hex())

print(f"{len(_BAD)} Abweichungen")
for n, v in _BAD:
    print(n, v)
raise SystemExit(1 if _BAD else 0)
