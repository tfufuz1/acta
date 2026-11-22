# ARCHITECTURE

Kurzkarte. Maßgeblich ist `SPEC.md`. Status-Aussagen stammen aus `IMPLEMENTATION_SPEC.md` Rev. 3 und aus `lib.rs`; Zahlen daraus sind nicht von uns reproduziert. Stand 2026-11-22.

## 1. Crates und Schichten

```
contextra-storage-ports   Traits und Werttypen, keine Logik, keine Abhängigkeiten
        ▲
contextra-storage         Kern, nur std (+ ports), forbid(unsafe)
        ▲ (nur dev-dependency)
contextra-crypto-ref      Referenz für Cipher/Mac/Kdf
```

Der Kern hängt an `ports`, nie umgekehrt (SPEC §3.1). Schichtregeln: SPEC §3.3 und IMPL §5.1, geprüft von `make layering`.

## 2. Module von `contextra-storage` und Status

Quelle: `crates/contextra-storage/src/lib.rs`.

| Modul | Zweck | Stand |
|---|---|---|
| `codec` | Varint, Tags, Nonces, Byte-Formate | übersetzt, getestet (Golden, Property) |
| `keys` | Schlüssel und Kopf | übersetzt |
| `env` | Fs (MemFs/RealFs), Uhr, Entropie, Governor, Metriken | übersetzt; `std::fs` nur in `real_fs.rs` |
| `redo` | Redo-Log: Frame, Header, Writer/Replay-Zustandsmaschinen | übersetzt; `redo`-Tests 20/24 grün |
| `run` | Run-Dateien: Block, Index, Filter, Footer, Directory, Reader/Builder | übersetzt |
| `blob` | Blob-Dateien, Referenzen, Sweep | übersetzt |
| `catalog` | Katalog: Edits, Log, Rollover, Current | übersetzt |
| `engine` | Committer, Memtable, Version, Views, Registry, Retention | übersetzt, **ohne** `recovery`, `flush`, `compaction` im Modulbaum; Committer-Redo ist Stub (S-1) |
| `sync` | Sperren mit Rang (`Ranked<T, RANK>`) | übersetzt; Lint-Verstöße (S-8) |
| `api` | öffentliche Fläche | nur Listener; **keine Facade** (S-3) |
| `erase` | kontrollierte Löschung | nur Doku (S-4) |
| `testing` | Testhilfen, Feature `testing` (Default an, S-9) | übersetzt |

Dateien für `recovery.rs`, `flush.rs` und `compaction/` liegen vor, sind aber nicht per `pub mod` eingebunden (`make metrics` M-17).

## 3. Dateiarten

Redo (WAL) · Run (sortierte Tabelle: Blöcke, Index, Filter, Footer, Directory) · Blob (Wertekörper) · Catalog (Metadaten-Log) · Segment · KEYS und FORMAT. Formate sind per Freeze-Liste (SPEC §9.8) und Referenzvektoren (Anhang A, `tools/reference/golden/`) eingefroren.

## 4. Pfade (SPEC-Verweise)

Schreibpfad (§10): `apply` über Flat-Combining und Gruppen-Fsync im `committer`, Sichtbarkeit und Untergrenze (§10.3), Gegendruck (§10.4, Erratum E-4: nicht unter dem Committer-Lock warten), Redo-Segment-Lebenszyklus, `ENOSPC` und Poison (§10.5). Lesepfad: §11. Wartung (Flush, Compaction, Blob-GC): §12. Recovery und Absturzmatrix: §13. Löschung: §14. Verschlüsselung: §15.

## 5. Sperrränge (SPEC §16.1, `INV-LOCK-1`)

Nur aufsteigend erwerben, jede höchstens einmal je Thread. Unter `debug_assertions` prüft ein Rangstack.

| Rang | Sperre | Hält über E/A |
|---|---|---|
| 1 | `committer` | ja (einzige Ausnahme, P-9) |
| 2 | `intake` | nein |
| 3 | `state` (`RwLock<Arc<Version>>`) | nein |
| 4 | Memtable-`RwLock` | nein |
| 5 | `registry` | nein |
| 6 | `maint` (je Lane) | nein |
| 7 | `notify` | nein |

Ränge 5–7 sind Blätter. Cache- und Governor-Sperren sind Blätter und nie über E/A gehalten. Nachweise per `loom` (`make loom`).

## 6. Kennungen

| Kennung | Bedeutung | Ort |
|---|---|---|
| `Z-n`, `NZ-n` | Ziel, Nicht-Ziel | SPEC §2 |
| `P-n` | Codierregel | SPEC §3.5 |
| `F-n` | Fehlerklasse | SPEC §5 |
| `A-n` / `A-nn` | Angreifer / Annahme (zweistellig) | SPEC §5, §22 |
| `SD-nn` | Entscheidung | SPEC §21 |
| `INV-*` | Invariante (43 laut M-09) | SPEC §18 |
| `H-S*` | Leistungshypothese | SPEC §20 |
| `O-n` | offene Entscheidung | SPEC §22 |
| `E-n` | Erratum (Kollision mit Erase-Schritten E0–E7) | SPEC §22.3 |
| `G-*` | Gate | IMPL §6 |
| `S-n` | Befund | IMPL §4 |
| `BL-n` | Blocker | IMPL §3 |
| `WP-n` | Arbeitspaket (WP-M Migration) | IMPL §5 |
| `CR-n` | Änderungsantrag an die SPEC | IMPL §10 |
| `M-nn` | Metrik (Kollision mit Migration M1–M5) | IMPL §11, `make metrics` |

## 7. Lesepfade mit Zeilenbereichen (Stand `main`, ändert sich)

SPEC: Zweck §1.1 (13–22) · Ziele §2 (41–59) · Schichten §3 (105–181) · Ports §4 (182–272) · Fehlermodell §5 (273–307) · Vokabular §6.1 (310–343) · API §7 (418–549) · Formate §9 (611–810) · Schreibpfad §10 (811–911) · Lesepfad §11 (912) · Wartung §12 (928) · Recovery §13 (1041) · Löschung §14 (1106) · Krypto §15 (1191) · Nebenläufigkeit §16 (1285) · Invarianten §18 (1403) · Tests §19 (1455) · Leistung §20 (1502) · Entscheidungen §21 (1531) · Errata §22 (1604) · Goldens Anhang A (1666).

IMPL: Kernbefunde §0 (14) · Ist-Zustand §1 (29) · Blocker §3 (108) · Befunde §4 (126) · Arbeitspakete §5 (166) · Gates §6 (283) · Sofortmaßnahmen §7 (305) · Änderungsanträge §10 (365) · Definition of Done §11 (386) · Messbefehle Anhang A (413).

Zeilennummern driften. Bei Abweichung mit `grep -n '^#' SPEC.md` neu bestimmen.

## 8. Tests

`crates/contextra-storage/tests/`: `blob`, `catalog`, `engine`, `env`, `format`, `keys`, `loom`, `redo`, `run`, `testing` (Verzeichnisse) sowie `codec_property`, `env_tests`, `fs_mem_test`, `fs_real_test`, `golden_anhang_a`, `invariants`, `layering`. Aufruf: `make test-target T=<name>`. `tests/engine` kompiliert derzeit nicht.
