# Implementierungsplan: `acta-gate`

> **Status:** verbindlich für die Crate-Überführung · **Paket:** 2 (L1)
> **Vorgaben:** [docs/spec/03_systemarchitektur.md](../../docs/spec/03_systemarchitektur.md) (3.4 HR-2/HR-3, 3.6, 3.7) · [docs/spec/05_speicher.md](../../docs/spec/05_speicher.md) (5.2, 5.3) · [docs/spec/08_loeschkern.md](../../docs/spec/08_loeschkern.md) (8.1, 8.5, 8.7) · [docs/spec/16_test_freigabe.md](../../docs/spec/16_test_freigabe.md) · [docs/spec/anhang_c_durabilitaet.md](../../docs/spec/anhang_c_durabilitaet.md) · [CLAUDE.md](CLAUDE.md) · [archive/acta-gate/MANIFEST.md](../../archive/acta-gate/MANIFEST.md)

## 0. Verifikationsstand
- Ist-Stand laut `CLAUDE.md` des Crates (Stand: Tests dort als grün angegeben). **In dieser Planung nicht neu ausgeführt**: Toolchain 1.89.0 nicht verfügbar.
- Archivquellen nur über Manifest und Dateiliste bewertet, nicht Zeile für Zeile.

## 1. Einordnung
- **Level:** 1. **Abhängigkeiten:** `acta-base`. Keine Kante zu `acta-crypto` (gleiches Level, G1). Die HMAC-Kette läuft direkt über `hmac`/`sha2`.
- **Budget:** 1 200 LOC (G-HUB). Ist laut CLAUDE.md 1 030 LOC, also 170 LOC Reserve.
- **Externe Abhängigkeiten:** `roaring`, `hmac`, `sha2`, `zeroize`, `thiserror`. Dev: `acta-testkit`, `proptest`.
- **Rolle:** Die einzige Sperrstruktur, die jeder Lesepfad vor jeder Entschlüsselung befragt (Z1). Der Löschkern erreicht Segmente, Indizes und Caches **ausschließlich** über `ErasureLayer` (3.3).

## 2. Ist-Stand (laut CLAUDE.md)
| Teil | Stand |
|---|---|
| `state` | `GateState`, `LineageHit` (Rangfolge Erased > Redacted > Restricted > Superseded), `Lineage` (Z13), `TargetSet` |
| `GateSnapshot` | Roaring-Mengen und Epoche; Zeigertausch |
| `view` | `ErasedView`: fail-closed bei nicht lesbarer Sperre (`Erased`, Epoche `u64::MAX`) |
| `log` | Gate-Log `[seq][op][len][payload][tag]`, HMAC-Kette, Operationen `0x01` bis `0x07` |
| `writer` | `GateWriter`: Z4-Reihenfolge (RAM zuerst, dann Anhängen und `fsync`); Fehler vergiftet |
| `compact` | Verdichtung (5.3, C23): Snapshot `AGSN` mit `acta/gate/snap/v1`, Operation `0x08` bindet das neue Log an den Snapshot. `recover_with_snapshot`. |
| `layer` | `ErasureLayer`-Vertrag, `LayerAck`, `LayerPlan`, `Residue` |
| `GateSink` | Port (`append`, `sync`, `install_snapshot`); Standard lehnt Snapshot ab |
| Tests | `gate_state` (11), `gate_writer` (10), `gate_model` (2, Proptest gegen Referenzmodell), `gate_compact` (9) |

## 3. Spec-Bezug
- **Z1 (Sofortige Lesesperre):** `check_ver` und `is_redacted_k` sind O(1) ohne E/A. Schreibsperre und Lesesperre wirken nach dem `fsync` des Gate-Logs.
- **Z4 (Crash-Invarianz):** C07 bis C09 und C23 gehören zum Gate. Physische Tatsachen haben Vorrang vor dem Journal (WP-4).
- **Z6:** Das Gate ändert keine Objekt-IDs und keine K-Ledger-Einträge.
- **Z11:** Messen gehört nicht hierher. `Residue` ist nur der Datentyp. Die Positivkontrolle liegt bei dem Crate, der misst (`acta-erasure`).
- **Z13 (Herkunftszwang):** `Lineage` ist nicht leer konstruierbar.
- **5.2 und 5.3:** `ErasedVers` hat Vorrang vor `RestrictedVers` und `SupersededVers`. `ErasedVers`, `ErasedSubjects` und `RedactedK` werden bei der Verdichtung nie verkleinert.
- **Zustandsautomat 8.1:** Das Gate kennt die Zustände `Gated`, `Restricted`, `Superseded`, `Erased`, `Expired`. `Pending` und `Quarantine` sind keine Gate-Zustände. Sie liegen in `acta-erasure` bzw. `acta-pstore`.

## 4. Archiv-Entscheidungen

| Archivquelle | Entscheidung | Begründung |
|---|---|---|
| `erasure/src/gate/read_gate.rs`, `erased_set.rs`, `log.rs` | Bereits umgesetzt (Abgleich) | Ergebnisse in CLAUDE.md beschrieben |
| `erasure/src/gate/batch.rs` (262 Zeilen, `async`) | Konzept prüfen, Stapelbildung synchron neu schreiben | `async` verboten (G-NOTOKIO). Stapel für `destroy_many` gehört in `acta-vault`. |
| `erasure/src/gate/fence.rs` (441 Zeilen, `async`) | Konzept prüfen, Zaun-Logik in `GateWriter` integrieren | Epochen-Prüfung vor jeder Ausgabe (9.5) ist Bestandteil des Lesepfads. Der Zaun ist kein eigener Baustein. Prüfen, ob Z1 ohne ihn gilt. |
| `erasure/src/gate/log.rs` (`serde`) | Bereits umgesetzt (Abgleich) | Handkodiert |
| `ports/src/gate.rs` (`serde`) | Bereits umgesetzt (`ErasedView`, `GateSink`) | Port-Form nach 3.7 |
| `ports/src/erasure.rs` (`serde`) | Bereits umgesetzt (`ErasureLayer`) | Vertrag nach 3.7 |
| `types/src/tombstone.rs` (`panic`) | **Nicht hier.** Grabstein-Typ gehört zu `acta-vault` (4.3) | Tresor-Einträge sind dort |
| `erasure/tests/t_gate.rs` | Übernehmen (Abgleich mit Z1) | |
| `erasure/tests/loom_gate.rs`, `gate_linearization_test.rs` | Übernehmen als `loom`-Test, `async` entfernen | Z1-Linearisierung (`T-CONC-GATE`) |

**Verworfen:** `parking_lot` (nicht auf der Whitelist). Sperren über `std::sync`, Lock-Order aus 3.6 einhalten.

## 5. Offene Punkte und Spec-Lücken

1. **Datei-Sink fehlt.** `GateSink` wird nur als Trait geführt. Die Datei-Implementierung auf `DurableVfs` (`acta-fs`) gehört nach `acta-seg` (L2), weil sie Speicher ist. Zusammenführung mit `acta-node` (L7) nur über Konstruktion, nicht über Logik.
2. **`ErasureLayer` ohne Implementierung.** Der Vertrag steht, aber kein Implementierer. Die Erfüllung von Z11 (Messung mit Positivkontrolle) liegt bei `acta-erasure`. Dort ist der Plan bindend.
3. **Stapelbildung und Zaun.** Die Archiv-Konzepte `batch.rs` und `fence.rs` sind nicht überführt. Vor Übernahme entscheiden, ob ein eigener Baustein nötig ist. Default-deny: nur was 5.3 und 3.6 fordern.
4. **Restrisiko bei abgeschnittenem letztem Rahmen.** Eine beschädigte Längenangabe im letzten Rahmen ist von einem Abbruch nicht zu unterscheiden. Ausgewiesen im Crate, im Bericht aufzunehmen (20.3).
5. **Epochenwert bei nicht lesbarer Sperre.** `Epoche = u64::MAX` ist fail-closed. Dokumentieren, dass ein Leser diesen Wert nie als gültigen Stand deutet.
6. **Wöchentliche Verdichtung** (Anhang A) wird vom Knoten geplant (`acta-node`). Der Crate stellt nur `compact` bereit.

## 6. Arbeitspakete

**AP 1 – Restarbeiten gemäß CLAUDE.md schließen.** Loom-Linearisierungstest `T-CONC-GATE` übernehmen (ohne `async`). Crash-Matrix für C07 bis C09 und C23 gegen `GateSink`-Fehlerinjektion.
- *Abnahme:* Jeder Crash-Punkt hat einen Test mit Positivkontrolle (16.4). I1 (durable Sperre geht nie verloren) und I2 (kein Klartext eines gesperrten Ziels) sind geprüft.

**AP 2 – Archiv-Tests übernehmen.** `t_gate`, `loom_gate`, `gate_linearization_test` nach `tests/`. Panik-Stellen in Tests sind erlaubt, im Produktionscode nicht.
- *Abnahme:* Alle Tests grün, `loom`-Lauf mit dokumentiertem Modellumfang.

**AP 3 – Entscheidung zu `batch.rs` und `fence.rs`.** Konzept gegen 5.3, 3.6 und 9.5 prüfen. Übernehmen, was gefordert ist, den Rest verwerfen. Entscheidung im Crate-Status festhalten.
- *Abnahme:* Keine async-Abhängigkeit. Jede übernommene Funktion hat einen Verweis auf eine SPEC-Stelle.

**AP 4 – Datei-Sink in `acta-seg` verlagern (nach Paket 3).** Der Crate bleibt bei `GateSink` als Trait.
- *Abnahme:* Kein `std::fs` im Crate (G-FS). Der Sink kommt aus `acta-seg`.

**AP 5 – Gates.** G-NOTOKIO, G-LINT (keine Panik-Stellen im Produktionscode), G-HUB (≤ 1 200 LOC), G-NOSERDE-DISK, G-UNSAFE.
- *Abnahme:* `cargo xtask gates` grün, Clippy `-D warnings`.

**AP 6 – Archivordner.** `archive/acta-gate/` entfernen, sobald AP 1 bis 5 grün sind. Das Gate ist dann vollständig überführt.

## 7. Tests und Gates

| Test | Zusage / Gate |
|---|---|
| `gate_state` (11), `gate_writer` (10), `gate_compact` (9) | Z1, Z4, C23 |
| `gate_model` (Proptest gegen Referenzmodell) | Z1, 5.3 |
| `t_gate`, `loom_gate` (neu nach AP 2) | Z1, `T-CONC-GATE` |
| Crash-Matrix C07 bis C09 (AP 1) | Z4, `T-CRASH-*` |
| G-READ (öffentliche Lesepfade nur über `ErasedView`), G-ERA | 16.6 |
| G-NOTOKIO, G-LINT, G-HUB, G-NOSERDE-DISK | 16.6 |

## 8. Befehle
```
cargo test -p acta-gate --locked
cargo clippy -p acta-gate --all-targets --locked -- -D warnings
cargo fmt --all -- --check
cargo xtask gates
```
