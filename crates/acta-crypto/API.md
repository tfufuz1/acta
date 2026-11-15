# Öffentliche API von acta-crypto (Fassung 3.0 Final)

## Module
- `aead`: Authentifizierte Datensatz-Verschlüsselung mit XChaCha20-Poly1305 (Umschlag v3, `seal_record`, `open_record`, `Envelope`, `SealCtx`).
- `anti_tamper`: Konstantzeit-Vergleiche (`constant_time_compare`), `VolatileEncryptionKey`, `Blake3Chain`.
- `dek`: Data Encryption Key Handling (`Dek`, `derive_dek`, `wrap_dek`, `unwrap_dek`).
- `error`: Fehlertypen (`CryptoError`, `Result<T>`).
- `kdf`: Schlüsselableitung via HKDF-SHA256 (`hkdf_derive`) und Argon2id (`derive_key_argon2id`, `KdfHeader`).
- `secret`: Schutzcontainer mit `ZeroizeOnDrop` (`SecretKey`, `Secret`).
- `wal_crypto`: HMAC-SHA256 Kettenintegrität für WAL-Frames.
