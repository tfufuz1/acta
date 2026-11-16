# Implementierungsplan: `acta-ledger`

> **Status:** verbindlich für die Crate-Überführung · **Paket:** 2 (L2)
> **Vorgaben:** [docs/spec/06_wissensebene.md](../../docs/spec/06_wissensebene.md) (6.2 Protokolle, Commitments, Registersenke) · [docs/spec/08_loeschkern.md](../../docs/spec/08_loeschkern.md) (8.8 Restore, 8.9) · [docs/spec/04_kryptografie.md](../../docs/spec/04_kryptografie.md) (4.4 Anker-Köpfe, 4.7) · [docs/spec/anhang_a_parameter.md](../../docs/spec/anhang_a_parameter.md) (A.2) · [CLAUDE.md](CLAUDE.md) · [archive/acta-ledger/MANIFEST.md](../../archive/acta-ledger/MANIFEST.md)

## 0. Verifikationsstand
- Ist-Stand: **Gerüst mit Fehlertyp** (17 LOC) laut CLAUDE.md. Die Ledger-Logik existiert noch nicht in `crates/`.
- Nicht ausgeführt: Build und Tests (Toolchain nicht verfügbar).

## 1. Einordnung
- **Level:** 2. **Abhängigkeiten (SPEC 3.3):** `acta-base`, `acta-crypto`, `acta-fs`. Querkanten verboten.
- **Budget:** 2 800 LOC (G-HUB).
- **Externe Abhängigkeiten (Whitelist):** `hmac`, `sha2`, `thiserror`. **Nicht:** `bincode`, `blake3`, `futures-util`, `parking_lot`.
- **Rolle:** Drei getrennte, append-only Hash-Ketten je Mandant: **K** (Wissen), **E** (Nachweise), **A** (Zugriff). Dazu signierte Checkpoints, Commitments und der Trait `RegistrySink`.

## 2. Ist-Stand
- `LedgerError` als einziger Typ (HR-4). Keine Logik.
- Damit ist der gesamte Umfang des Plans Neubau gegen das Archiv (`archive/acta-ledger/`, 3 268 Zeilen).

## 3. Spec-Bezug
- **6.2:** `chain_id = SHA-256(TenantId ‖ "K"|"E"|"A")`. Hash je Eintrag `SHA-256(prev ‖ kanonischer Eintrag)`, Domäne `acta/ledger/v1`. Kanonischer Eintrag `[seq u64][kind u8][len u32][payload]`. Alle `checkpoint_every` = 1 024 Einträge ein Checkpoint, signiert über `notary_sign` (`acta/checkpoint/v1`), Kopf gemeldet über `record_head`.
- **Z6:** Löschungen in P ändern das **K-Ledger nie**. E und A werden getrennt geführt.
- **Z15:** Ledger enthalten keine Personendaten und keine Roh-Kennungen. Nur epochengeschlüsselte Commitments.
- **Commitments:** `commit(e, KeyId) = HMAC-SHA256(CommitKey(e), "acta/cmt/v1" ‖ KeyId)[..16]`. `KeyId` ist `SubjectTag`, `VerRef` oder Segment-`file_id`, **nie** eine externe Kennung.
- **Registersenke (6.2):** Gate-Log und E-Ledger werden über `erase_registry_sink` gespiegelt. Pflichtparameter, getrennter Ausfallbereich (R-SINK). Spiegeln ist asynchron und blockiert die Sperre nie.
- **A-Ledger:** Einträge versiegelt unter `LogKey(m)` (Monat). Ablauf von `access_log_retention` vernichtet den Monatsschlüssel, die Kette bleibt über die Chiffrate prüfbar. Abgelaufene Monate tragen `Pruned { block, last_hash }`.
- **8.8 (Restore):** Ein zurückgerolltes K-Ledger führt zu `Quarantine`, bis Vier-Augen-Bestätigung (`accept_rollback`). Gate-Log und E-Ledger werden aus der Senke vorwärts ergänzt.

## 4. Archiv-Entscheidungen

| Archivquelle | Entscheidung | Begründung |
|---|---|---|
| `erasure/src/ledger.rs`, `ledger/record.rs`, `ledger/entry.rs` | Konzept prüfen, Format neu handkodieren | `serde`/`bincode` für Plattenformate verboten (G-NOSERDE-DISK). Format nach 6.2 neu. |
| `ledger/persistent.rs`, `ledger/storage.rs`, `ledger/durable.rs` | **Neu** über `acta-fs` (`Vfs`) | `std::fs` nur in `acta-fs` (G-FS). `durable.rs` ist Doppelung zu `DurableVfs`. |
| `ledger/sink.rs` (228 Zeilen) | Übernehmen als `DirSink` | Spec 6.2 nennt `DirSink` im Kern. Prüfung `st_dev` (R-SINK) ergänzen. |
| `ledger/error.rs` | Bereits umgesetzt (`LedgerError`) | HR-4 |
| `ports/src/ledger.rs` | Trait-Form prüfen gegen 6.2 | `RegistrySink` gehört in dieses Crate (L2): es braucht den Trait, die Netzsenke (`acta-sink-net`, L8) implementiert ihn (HR-2). |
| `ports/src/report_signer.rs` | **Nicht hier.** Nach `acta-reports` (L5) | Berichte sind dort. Ledger braucht nur den Checkpoint-Signierer aus `NotarySigner`/Anker. |
| `tests/ledger_spec_tests.rs`, `p6_ledger_durability.rs` | Übernehmen als Oracle, Panik-Stellen in Tests erlaubt | Absturz- und Dauerhaftigkeitsprüfung |
| `tests/p5_wal_sink_tests.rs` | Übernehmen, Sink-Teil | WAL-Logik gehört in `acta-seg`, nicht hierher |
| `tests/p7_report_chain_tests.rs`, `t_report_ledger.rs` | **Nach `acta-reports`** | Berichtsketten sind dort |
| `tests/revocation_log_vfs.rs` | Prüfen, ob Inhalt in E-Ledger aufgeht | Spec kennt kein eigenes Widerrufsprotokoll. Default-deny: nur was 6.2 fordert. |

**Verworfen:** `blake3` (Hash-Kette über `sha2`), `futures-util` (async verboten), `parking_lot` (`std::sync`).

**Wichtig:** Der Hash-Kettentyp existiert bereits in `acta-crypto` (`HashChain`, mehrere Instanzen laut CLAUDE.md). Ledger **verwendet diesen Typ**, implementiert keine zweite Kette (3.1 Prinzip 4).

## 5. Offene Punkte

1. **Kanonische Eintragsform.** Die Spec legt `[seq][kind][len][payload]` fest. Die Zuordnung der `kind`-Werte (Commit, Ref, Tag, Redacted, Expired, Waiver, DecisionAppended, PolicyChanged, RootRotated, Restored, Re-Witness, restricted_read, ...) ist in 6.2 nur teilweise aufgezählt. **Entscheidung nötig:** Wertetabelle als ADR festhalten, bevor Golden-Vektoren erzeugt werden.
2. **Registersenke vs. `DirSink` bei Pflichtparameter.** `erase_registry_sink` hat keinen Startwert (0.4 Regel 1). Der Crate darf ohne konfigurierte Senke nicht starten (R-SINK). Test nötig.
3. **Anker-Kopf-Meldung (`record_head`).** Der Anker liegt in `acta-vault` (L2, gleiches Level). `acta-ledger` darf `acta-vault` nicht kennen (Querkante). Lösung: `KeyAnchor`-Teilmenge (`record_head`, `last_head`, `notary_sign`) als Trait im Ledger, den der Anker implementiert (HR-2). **Entscheidung als ADR festhalten.**
4. **Checkpoint-Zeitpunkt und Commit-Epoche.** `commit_epoch` (30 Tage) bestimmt, wann `CommitKey(e)` wechselt. Die Zeitquelle läuft über `Clock` (G3). Abhängigkeit zu `acta-base`, nicht zu `acta-sys`.
5. **A-Ledger-Verschlüsselung.** `LogKey(m)` ist ein Schlüssel aus dem Tresor (`acta-vault`). Ledger braucht ein **Schlüssel-Muster ohne Tresorkenntnis**: den Schlüssel als Parameter übergeben (wie `acta-seg`, HR-9).

## 6. Arbeitspakete

**AP 1 – Kanonische Form und Wertetabelle (ADR).** Format `[seq u64][kind u8][len u32][payload]` als Modul `entry`, handkodiert mit geprüften Feldern (`canon` aus `acta-base`). Wertetabelle der `kind`-Werte festlegen.
- *Abnahme:* Golden-Vektoren aus unabhängiger Referenz (Python-Skript analog `fx_ln_oracle.py`). Keine `serde`-Abhängigkeit.

**AP 2 – Hash-Kette und Checkpoints.** Drei Instanzen von `crypto::HashChain` mit `chain_id` gemäß 6.2. Checkpoint alle `checkpoint_every` Einträge, signiert über den Anker-Port (AP-Punkt 3).
- *Abnahme:* Proptest: Umordnung, Auslassung oder Änderung eines Eintrags wird erkannt. Positivkontrolle: absichtlich manipulierte Kette muss scheitern (16.4).

**AP 3 – Commitments.** `commit(e, KeyId)` mit `CommitKey(e)` aus `acta-crypto` (`commitment`-Funktion ist dort vorhanden). Anonymisierung nach `commit_key_retention`.
- *Abnahme:* Test `T-LOG-*`: Kein Canary-Klartext und keine externe Kennung in Ledger-Bytes (Z15). Positivkontrolle: Canary-Kennung wird im Kontrolllauf gefunden.

**AP 4 – Registersenke.** `RegistrySink`-Trait, `DirSink` mit `st_dev`-Prüfung gegen Datenmedium (R-SINK), asynchrones Spiegeln nach 6.2. `last_acked` und idempotentes Erneutsenden (C26).
- *Abnahme:* Start verweigert ohne Senke oder bei gleichem `st_dev`. Crash-Test C26 mit Positivkontrolle.

**AP 5 – A-Ledger mit Monatsschlüsseln.** Versiegelung unter einem übergebenen `LogKey(m)`. `Pruned { block, last_hash }` nach Ablauf.
- *Abnahme:* Nach Vernichtung des Monatsschlüssels bleibt die Kette über die Chiffrate prüfbar (Z12-naher Test). Nutzlast ist ohne Schlüssel nicht lesbar.

**AP 6 – Archiv-Tests übernehmen und Lücken schließen.** `ledger_spec_tests`, `p6_ledger_durability` als Oracle. Crash-Punkte C16, C18, C19, C20, C26 abdecken.
- *Abnahme:* Jeder Punkt hat `T-CRASH-*`-Test mit Positivkontrolle.

**AP 7 – Gates.** G-NOSERDE-DISK, G-FS (nur über `acta-fs`), G-NOTOKIO, G-LINT, G-HUB (≤ 2 800 LOC), G-CRY (`let _ =` auf Result verboten).
- *Abnahme:* `cargo xtask gates` grün.

**AP 8 – Archivordner.** `archive/acta-ledger/` erst entfernen, wenn `report`-Teile nach `acta-reports` verlagert sind (dort im Paket 5 zugesagt) und AP 1 bis 7 grün sind.

## 7. Tests und Gates

| Test | Zusage / Gate |
|---|---|
| Golden-Vektoren (AP 1) | 6.2, Format |
| Proptest Kettenintegrität (AP 2) | 16.2 Eigenschaft, I5 |
| `T-LOG-*` (AP 3) | Z15 |
| `T-VCS-ERASE-1` (K-Ledger unverändert) | Z6 |
| C16, C18, C19, C20, C26 | Z4 |
| `T-REST-*` (Kopf-Abgleich) | Z12 |
| G-NOSERDE-DISK, G-FS, G-NOTOKIO, G-LINT, G-HUB | 16.6 |

## 8. Befehle
```
cargo test -p acta-ledger --locked
cargo clippy -p acta-ledger --all-targets --locked -- -D warnings
cargo fmt --all -- --check
cargo xtask gates
```
