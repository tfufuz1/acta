# Inventur der toten Test-Gates (WP-A.1)

Untersuchung aller 11 Stellen mit `#[cfg(feature = "encryption-at-rest")]` in `crates/acta-store/src/wal/`:

| Nr | Datei | Zeile(n) | Konstrukt / Kontext | Analyse & Entscheidung | Ergebnis |
|---|---|---|---|---|---|
| 1 | `src/wal/tests.rs` | 6 | `use acta_crypto::crypto::KeyManager;` | `KeyManager` ist bedingungslos in `acta_crypto` verfügbar. Gate entfernen. | Reaktiviert |
| 2 | `src/wal/tests.rs` | 14 | `mod hmac_tests;` | `hmac_tests` nutzt `WalHmac` und `Wal`. Bedingungslos verfügbar. Gate entfernen. | Reaktiviert (418 LOC) |
| 3 | `src/wal/tests.rs` | 19 | `mod replay_tests;` | `replay_tests` nutzt Standard-Wal-APIs. Bedingungslos verfügbar. Gate entfernen. | Reaktiviert (1621 LOC) |
| 4 | `src/wal/tests/flusher_tests.rs` | 349 | `test_wal_rewrite_multi_frame_chunking_encrypted` | Nutzt `KeyManager`. Gate entfernen. | Reaktiviert |
| 5 | `src/wal/tests/encode_tests.rs` | 6 | `use crate::wal::KeyManager;` | Bedingungslos verfügbar. Gate entfernen. | Reaktiviert |
| 6 | `src/wal/tests/encode_tests.rs` | 9 | `use std::sync::Arc;` | Bedingungslos verfügbar. Gate entfernen. | Reaktiviert |
| 7 | `src/wal/tests/encode_tests.rs` | 12 | `use tokio::fs;` | Bedingungslos verfügbar. Gate entfernen. | Reaktiviert |
| 8 | `src/wal/tests/encode_tests.rs` | 123 | `test_batch_encryption_single_nonce_layout` | Nutzt `KeyManager`. Gate entfernen. | Reaktiviert |
| 9 | `src/wal/tests/encode_tests.rs` | 184 | `#[cfg(not(feature = "encryption-at-rest"))] test_batch_plaintext_layout` | Sollte immer laufen, da Feature nicht existiert. Invertiertes Gate entfernen, damit Plaintext-Layout dauerhaft geprüft wird. | Bereinigt |
| 10 | `src/wal/tests/hmac_tests.rs` | 8 | `use acta_crypto::wal_crypto::WalHmac;` | `WalHmac` ist bedingungslos in `acta-crypto` vorhanden. Gate entfernen. | Reaktiviert |
| 11 | `src/wal/tests/hmac_tests.rs` | 11 | `use std::sync::Arc;` | Bedingungslos verfügbar. Gate entfernen. | Reaktiviert |

## Zusammenfassung
Alle 11 Stellen bezogen sich auf ein historisches Feature `encryption-at-rest`, dessen Funktionalität (`KeyManager`, `WalHmac`, verschlüsselte WAL-Frames) längst bedingungslos über `acta-crypto` in `acta-store` integriert ist. Durch die Bereinigung der toten Gates werden über 2000 Zeilen wertvoller Durabilitäts- und Replay-Tests wieder aktiv kompiliert. `tests/no_encryption_feature.rs` bleibt uneingeschränkt grün, da in `Cargo.toml` weiterhin kein solches Feature deklariert ist.
