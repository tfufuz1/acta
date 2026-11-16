# Implementierungsplan: `acta-vault`

> **Status:** verbindlich für die Crate-Überführung · **Paket:** 2 (L2)
> **Vorgaben:** [docs/spec/04_kryptografie.md](../../docs/spec/04_kryptografie.md) (4.2 bis 4.4, 4.7) · [docs/spec/08_loeschkern.md](../../docs/spec/08_loeschkern.md) (8.3 S5/S6/S7, 8.5, 8.8) · [docs/spec/03_systemarchitektur.md](../../docs/spec/03_systemarchitektur.md) (3.4 HR-2, 3.6 Lock-Order) · [docs/spec/anhang_a_parameter.md](../../docs/spec/anhang_a_parameter.md) (A.1) · [docs/spec/anhang_c_durabilitaet.md](../../docs/spec/anhang_c_durabilitaet.md) (C12 bis C17, C27) · [CLAUDE.md](CLAUDE.md) · [archive/acta-vault/MANIFEST.md](../../archive/acta-vault/MANIFEST.md)

## 0. Verifikationsstand
- Ist-Stand: **Gerüst mit Fehlertyp** (17 LOC) laut CLAUDE.md.
- Nicht ausgeführt (Toolchain nicht verfügbar). Archivdateien nur über Manifest bewertet.

## 1. Einordnung
- **Level:** 2. **Abhängigkeiten (SPEC 3.3):** `acta-base`, `acta-crypto`, `acta-fs`.
- **Budget:** 5 500 LOC (G-HUB).
- **Externe Abhängigkeiten (Whitelist):** `hkdf`, `sha2`, `zeroize`, `chacha20poly1305` (über `acta-crypto`), **`argon2`** (nur für `FileAnchor`, SPEC 4.1). Laut CLAUDE.md fehlt `argon2` in den Abhängigkeiten. **Ergänzen.** **Nicht:** `aes-gcm-siv`, `ahash`, `bincode`, `blake3`, `futures-util`, `parking_lot`, `rand`.
- **Rolle:** Mandantentresor (Schlüsselverwahrung), `KeyAnchor`, Wurzelrotation, Sicherungsexport, Vernichtung.

## 2. Ist-Stand
- `VaultError` als einziger Typ (HR-4). Keine Logik.
- Gesamter Umfang ist Neubau gegen das Archiv (`archive/acta-vault/`, 6 797 Zeilen). Die Archiv-Datei `erasure/src/vault/*` ist die Vorlage für Slot, Tabelle, Rehash, Scrub, Sealer.

## 3. Spec-Bezug
- **Eintragsformat (4.3):** Klartext `[kind u8][status u8][id 16][key 32]` = 50 Byte. Versiegelt `[len u16][nonce 24][ciphertext 50][tag 16]` = 92 Byte.
- **AAD:** `"acta/vault/v1" ‖ TenantId ‖ generation u32 ‖ slot u64`. Die Position im Tresor bindet den Eintrag, die `id` steht nur im Chiffrat.
- **`kind`:** `0x01` SubjectKey, `0x02` DocumentKey, `0x03` SegmentKey, `0x04` ScopeKey, `0x05` CommitKey, `0x06` SubjectAlias, `0x07` LogKey. **`status`:** `0x01` aktiv, `0x02` vernichtet.
- **R-KEY-FIRST (WP-1):** Schlüssel durabel im Tresor, bevor ein Umschlag unter ihm geschrieben oder referenziert wird.
- **Grabstein:** `status = 0x02`, `id` und `key` genullt, gleiche Länge, gleiche Versiegelung. Dann `fsync`, dann Direct-Read-Verifikation. Kein Kennungsbezug.
- **Speicherindex** `KeyId → slot` existiert **nur im RAM** und wird beim Öffnen durch Scan aufgebaut (Ziel: 10 Mio. Einträge unter 30 s, Z-Qualitätsziel 15.3).
- **Wurzelrotation (4.3):** Schnappschuss `N0`, Neuschreiben unter `VK(t,g+1)` ohne Sperre, `vault_freeze` für den Schwanz, atomarer Austausch, `anchor.destroy(g)`, `RootRotated` im E-Ledger. Reihenfolge: Tatsache vor Journal (C16).
- **Sicherungsexport (4.3):** Neuversiegelung unter `VKb(t,e) = HKDF(M_bk(e), TenantId, "acta/vaultbk/v1")`, ohne Grabsteine. Epochen `backup_epoch` = 24 h, Vernichtung `end(e) + T_BACKUP`.
- **KeyAnchor (4.4):** Trait mit `current_generation`, `master`, `rotate`, `destroy`, `backup_master`, `destroy_backup_epoch`, `index_secret`, `notary_sign`, `notary_public`, `record_head`, `last_head`, `assurance`.
- **A-ANCHOR:** `anchor_destroy_max` ist Pflicht je Anker. `FileAnchor` ist **nicht** für `Besonders` zulässig (R-ANCHOR).
- **Z10 (Fail-Closed):** Ein fehlender Schlüssel ohne Gate-Eintrag ist `Integrity`, nie `Erased`. Keine Standardschlüssel.

## 4. Archiv-Entscheidungen

| Archivquelle | Entscheidung | Begründung |
|---|---|---|
| `erasure/src/vault/slot.rs`, `table.rs`, `open.rs` | Vorlage, neu nach 4.3 | Format 92 B und AAD sind festgelegt. Archiv-Format prüfen, nicht übernehmen. |
| `erasure/src/vault/rehash.rs`, `table.rs` (`float`) | Gleitkomma entfernen | Keine Gleitkomma-Werte in Prüfpfaden (Z7) |
| `erasure/src/vault/keyring.rs` (`async`) | Neu synchron | G-NOTOKIO |
| `erasure/src/vault/sealer.rs` (825 Zeilen, `async`, `serde`) | Aufteilen in Sealer (Umschlag) und Schlüsselverwaltung; `async` raus | Umschlag liegt in `acta-crypto`. Keine doppelte Kryptologik. |
| `erasure/src/vault/durable.rs` (651 Zeilen, `float`) | **Neu** über `acta-fs` (`Vfs`, `write_atomic`, `rename_atomic`) | Doppelung zu `DurableVfs`; `std::fs` verboten |
| `erasure/src/vault/scrub.rs` | Logik nach `acta-fs` (`remove_scrub`) | Scrub ist Dateisystem-Operation |
| `erasure/src/vault/key_store.rs`, `ports/src/keys.rs` (`serde`, `unsafe`) | Schlüssel-Typen nach `acta-crypto` (bereits dort) | `unsafe` verboten außerhalb `acta-sys`. `serde` verboten. |
| `ports/src/key_material.rs` | Prüfen gegen `acta-crypto::key` | Typen sind dort bereits vorhanden |
| `erasure/src/kv_shredding.rs` (980 Zeilen) | **Verwerfen** | Kein Gegenstück in SPEC 4.3. Default-deny (0.4 Regel 1). Falls ein Bedarf nachgewiesen wird: ADR. |
| `erasure/src/revocation_log.rs` (`serde`) | **Prüfen gegen E-Ledger** (`acta-ledger`) | Widerrufsprotokoll ist durch E-Ledger (6.2) abgedeckt. Default-deny: nur übernehmen, was dort fehlt. |
| `erasure/tests/vault_spec_conformance.rs` (567 Zeilen) | Übernehmen als Oracle | Spec-Abgleich |
| `erasure/tests/vault_slot_layout.rs` | Übernehmen, Layout gegen 4.3 neu prüfen | Format |
| `erasure/tests/p1_key_hierarchy_and_purge.rs` | Übernehmen | Schlüsselhierarchie und Vernichtung |
| `erasure/tests/p2_vault_rewrite_tests.rs` (`async`) | Übernehmen, `async` raus | Rotation (4.3) |
| `erasure/tests/shredding_envelope_test.rs`, `t_sealer_keyring.rs`, `t_vault_core.rs` | Übernehmen | Umschlag und Tresor-Kern |

**Verworfen:** `aes-gcm-siv` (nicht auf der Whitelist, Algorithmus laut 4.1 nur XChaCha20-Poly1305), `ahash`, `bincode`, `blake3`, `futures-util`, `parking_lot`, `rand`.

## 5. Offene Punkte und Spec-Lücken

1. **Tresor-AAD und `SealCtx`.** Die Tresor-AAD (`acta/vault/v1 ‖ TenantId ‖ generation ‖ slot`) hat ein anderes Format als der Umschlag v1 (4.5). `acta-crypto::seal` nimmt `SealCtx`. **Prüfen**, ob `SealCtx` diese AAD abbilden kann. Falls nicht: Erweiterung in `acta-crypto` (eigene Entscheidung, Crate-Grenze beachten) oder ADR für einen dedizierten Tresor-Umschlag.
2. **`VKb(t,e)`-Ableitung.** `VAULT_BK` ist als Präfix in `acta-crypto::domain` vorhanden. Eine Ableitungsfunktion für `M_bk(e)`-Schlüssel mit `TenantId`-Salz fehlt im Kern. Prüfen, ob `derive_vault_key` analog genügt.
3. **`KeyAnchor`-Teilmenge im Ledger (siehe `acta-ledger`, Punkt 3).** Der Anker liegt hier in `acta-vault` (L2). Ledger braucht `record_head`, `last_head`, `notary_sign`. Der Trait gehört laut HR-2 dem Crate, das ihn braucht (`acta-ledger`). Der Anker implementiert ihn.
4. **`FileAnchor` und Passphrase.** Argon2id mit `m ≥ 65 536 KiB, t ≥ 3, p ≥ 4`, Passphrase ≥ 80 Bit Entropie (Stärkeprüfung beim Anlegen), Speicherung ausgeschlossen. Die Entropie-Schätzung hat keinen Spec-Algorithmus. **Entscheidung nötig** (Verfahren zur Schätzung als ADR).
5. **Schlüsselbestimmung der Ableitung beim Start.** Der Speicherindex wird per Scan aufgebaut. Ein Scan über 10 Mio. Einträge unter 30 s ist Zielwert, nicht Zusage. Messbericht nötig (16.5).
6. **`Zeroizing`/`SecureBuf`.** Schlüsselmaterial lebt in `SecureBuf` (`acta-sys`, L1). `acta-vault` hängt von `acta-sys` nicht ab (SPEC 3.3). Lösung: `SecureBuf` über `acta-base`-Port? **Entscheidung nötig**, sonst liegt Material in gewöhnlichem Speicher (Z10-Lücke).

## 6. Arbeitspakete

**AP 1 – Eintragsformat und Tresorlayout (4.3).** Modul `slot`: 50-Byte-Klartext, 92-Byte-Chiffrat, Lesen und Schreiben mit geprüften Feldern. AAD gemäß Punkt 5.1 umsetzen.
- *Abnahme:* Golden-Vektor aus unabhängiger Referenz (Python-Skript). Negativtest: falsche Position im Tresor führt zu `Crypto`. Compile-Fail-Test: kein `Clone`/`Debug` auf Eintragstypen.

**AP 2 – Tresor öffnen, Scan und Speicherindex.** Modul `open`: RAM-Index `KeyId → slot`, beim Öffnen durch Scan. Kein persistenter Index.
- *Abnahme:* Nur RAM-Index, keine Klartext-IDs in Dateien (`T-CRY-1`-Grep, Z2). Ziel-Messung 10 Mio. als Messbericht (AP 7).

**AP 3 – Schlüssel zuerst und Grabstein (WP-1, 4.3).** `put` (fsync, Gruppen-Commit nach 3.6), `destroy` als Grabstein, `destroy_many` gebündelt. Direct-Read-Verifikation über `acta-fs`.
- *Abnahme:* Crash-Matrix C12 (`destroy_many` mitten im Batch) und C13 (Grabstein ohne Direct-Read) mit Positivkontrolle. Entfernen der Verifikation lässt C13 scheitern.

**AP 4 – Wurzelrotation (4.3, C14 bis C16).** Schnappschuss, Neuschreiben in `vault-<ord>.<g+1>.tmp`, `vault_freeze`, atomarer Austausch, `anchor.destroy`, `RootRotated`. Start setzt Schritt 4 fort, sobald der Anker weiter ist.
- *Abnahme:* Crash-Tests C14, C15, C16. Tresor `g` bleibt bis zum Austausch unverändert lesbar. Lock-Order aus 3.6 durch `loom`-Test oder Ordnungstest.

**AP 5 – Sicherungsexport (4.3, C17, C27).** Export unter `VKb(t,e)` ohne Grabsteine, Epochen, Vernichtung nach `T_BACKUP`. Halber Export wird verworfen (C17).
- *Abnahme:* Export nach Ablauf der Epoche ist nicht mehr lesbar (Test mit `ManualClock`). `RestoreBeyondHorizon` wird geliefert (Z12, verbunden mit `acta-erasure`).

**AP 6 – `FileAnchor` und `KeyAnchor`.** Trait nach 4.4 (Teilmenge für Ledger, siehe Punkt 5.3). `FileAnchor` mit Argon2id, Stärkeprüfung und Verbot für `Besonders`.
- *Abnahme:* Test: `FileAnchor` mit `Besonders`-Konfiguration startet nicht (R-ANCHOR). Argon2-Parameter an Mindestwerte gebunden.

**AP 7 – Messung und Positivkontrolle.** Tresor-Öffnen (10 Mio.) und Rotation (10 Mio.) als Benchmark mit Messbericht (16.5, 15.3). Aufnahme in Messbericht mit Commit-Stand.
- *Abnahme:* Messwerte im Bericht, Abweichungen als ADR. Ohne Messbericht bleiben die [Q]-Werte offen.

**AP 8 – Gates und Archivordner.** G-CRY, G-FS, G-NOTOKIO, G-LINT, G-HUB (≤ 5 500 LOC), G-UNSAFE. `archive/acta-vault/` erst entfernen, wenn Punkte 5.1 bis 5.6 entschieden sind.
- *Abnahme:* `cargo xtask gates` grün, Entscheidungen in CLAUDE.md dokumentiert.

## 7. Tests und Gates

| Test | Zusage / Gate |
|---|---|
| Golden-Vektor Slot (AP 1) | 4.3 Format |
| `T-CRY-1` (keine Klartext-IDs in Dateien) | Z2 |
| `T-NOFALLBACK-*` (fehlender Schlüssel ohne Beleg = `Integrity`) | Z10 |
| C12, C13, C14, C15, C16, C17, C27 | Z4, `T-CRASH-VAULT-*` |
| `T-ROT-*` (Rotation, Positivkontrolle) | 4.3, Messung 120 s |
| `T-BACKUP-*` (Export-Ablauf) | 4.3, Z12 |
| `T-REST-*` | Z12 |
| G-CRY, G-FS, G-NOTOKIO, G-LINT, G-HUB | 16.6 |

## 8. Befehle
```
cargo test -p acta-vault --locked
cargo clippy -p acta-vault --all-targets --locked -- -D warnings
cargo fmt --all -- --check
cargo xtask gates
```
