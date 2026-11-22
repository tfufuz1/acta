# IMPLEMENTATION_SPEC.md — Refaktorisierungsplan `contextra-store` → `contextra-storage`

**Revision 2 · ersetzt Revision 1 · Messstand 22.11.2026**

| | |
|---|---|
| **Status** | Operativ und normativ für Reihenfolge, Abnahme, Entscheidungen und Messbasis. Bei Widerspruch gilt `SPEC.md` (R2). Änderungen an `SPEC.md` sind in §13 als Anträge formuliert, nicht vorgenommen. |
| **Quellstand** | Repository `tfufuz1/acta`, HEAD `8a24bf4`, 62 Commits. Crate-Quelltext unverändert seit `47c9865` (`git diff 47c9865 HEAD -- crates/contextra-store` ist leer). Historische Baseline: `014319d`. |
| **Verifikation** | Statisch. Messskript in Anhang A, reproduzierbar mit `python3 measure_baseline.py crates/contextra-store`. **Nicht kompiliert, nicht laufzeitgemessen:** keine Rust-Toolchain in der Erstellungsumgebung. Alle Laufzeitwerte (H-S\*) bleiben Hypothesen bis G-P. |
| **Gegenüber Rev. 1** | 7 falsche und 6 teilweise falsche Angaben korrigiert (§3). Neu: Blocker (§4), Paradigmen (§1.4), Konsumentenanalyse und Port-Migration (§9), Testmigration mit Markern (§8), Anträge an `SPEC.md` (§13), Messskript (Anhang A). |

---

## 0. Kernbefunde

1. **Baseline ist aus HEAD nicht baubar.** Kein Workspace-Manifest, keine der sieben internen Abhängigkeiten (`contextra-ids, -env, -sys, -mvcc, -crypto, -types, -ports`) im Repository. Ein vollständiger Workspace (39 Mitglieder) existiert nur in `014319d`. Keine Rust-Toolchain vorhanden.
2. **Der Umfang liegt im Konsumentenvertrag, nicht im Kern.** Der Port `StorageEngine` (asynchron, `TxId`-basiert, SSI-Tracking, 26 Methoden) hat 122 Dateien mit Textbezug in fünf Crates. SPEC R2 ersetzt seine Semantik vollständig. Rev. 1 plante dafür keine Phase.
3. **Namenskollision `contextra-ports`.** Das historische Crate (7 184 Zeilen, 46 Dateien in `813305c` entfernt) wird von acht Crates genutzt: `Purger` (24 externe Dateien), `LayerScanner` (7), `FeatureRing` (5). Das neue Port-Crate nach SPEC §3.1 kann diesen Namen bei einer Reintegration nicht tragen (BL-2).
4. **Produktionscode: 20 369 Zeilen.** Testcode: 11 646 Zeilen in `src/` und 25 563 in `tests/`, zusammen 62 % der 59 788 `.rs`-Zeilen. Rev. 1 bezeichnete die 32 015 Zeilen von `src/` als Umfang; das ist nicht der Produktionscode.
5. **Schichtregel verletzt.** 42 Modulkanten mit 142 Vorkommen, 3 Zyklen (`compat↔lsm`, `compat↔tenant_codec`, `lsm↔util`). `lsm` ist ein God-Modul mit Kanten zu zehn Modulen (inkl. `error`). Gegen den Zielschnitt aus SPEC §3.3 verletzen 11 Vorkommen die Regel klar; 4 betreffen eine Typzuordnung (`stats`).
6. **Beweisbarkeit (Z-14) ist durch den Altstand nicht belegt.** Die vier loom-Dateien (524 Zeilen) importieren keinen Produktionscode; sie testen eigene Nachbildungen. Die zehn Lock-Ränge werden nur per Konstanten-Test auf Monotonie geprüft, zur Laufzeit nicht erzwungen.
7. **Regelbrüche gegen P-1…P-14 in der Produktion:** 133 Integer-`as`-Casts (P-14), 86 direkte FS-Aufrufe außerhalb `io_seam` (P-1), 12 statische Items (P-5), 43 `cfg(feature)` in Logik (P-5), 11 Dateien über 400 Zeilen (P-6, Maximum 966). Keine `unwrap`, `expect` oder `panic` (P-13 erfüllt).
8. **Sicherheitsbefunde.** Der Legacy-HMAC-Schlüssel ist per konstantem Byte `0x5A` XOR-maskiert (Feature `legacy-wal-key`), also kein Geheimnis. Sieben Testschalter sind prozessweite Statics, davon drei ohne Feature-Gate. Diese drei werden im WAL-Append-Pfad gelesen.
9. **Fehlerpfad doppelt.** Der WAL-Replay fällt bei jedem Fehler des mmap-Pfads still auf einen zweiten Leseweg zurück (`wal/replay.rs:115–135`, nur `tracing::warn!`). Zwei Replay-Pfade für ein Format; ob sie identisch fehlerklassifizieren, ist ungeprüft.
10. **Rev. 1 hatte 13 von 22 Prüfpunkten falsch oder unvollständig** (§3). Die Testklassifikation summierte auf 158 statt 160 Dateien, die Hypothesenliste nannte H-S19 statt H-S18, und die Workspace-Zahl lag bei 36 statt 39.

---

## 1. Auftrag, Methodik, Paradigmen

### 1.1 Auftrag und Abgrenzung

Überführung des Altstands in `contextra-storage` nach SPEC R2. Dieses Dokument legt fest: Reihenfolge, Abnahme je Arbeitspaket (WP), Entscheidungen vor Phasenbeginn und Messbasis. **Nicht Teil:** Änderungen an `SPEC.md` (nur Anträge, §13) und aufsetzende Schichten außer der Migrationsbrücke (§9).

### 1.2 Entwurfsprinzipien

| ID | Prinzip | Begründung aus dem Befund | Prüfmittel |
|---|---|---|---|
| M-1 | Messen vor Planen. Jede Zahl hat Definition (Anhang A) und Reproduktion. | 7 falsche Zahlen in Rev. 1 | Messskript, CI-Vergleich der Zahlen |
| M-2 | Verhaltensoracle statt Byte-Oracle | Formate ändern sich (SD-04); Verhalten (Sichtbarkeit, Versionsreihenfolge, Fehlerklassen) bleibt | Differentialläufe gegen Baseline `014319d` (§8.1) |
| M-3 | Vertikale Scheiben nach Invarianten statt nach Modulen | 3 Modulzyklen; kleinste sinnvolle Einheit ist eine Invariantenfamilie aus Format, Schreibpfad, Recovery und Test | Rückverfolgbarkeit INV-ID → Test (M-09) |
| M-4 | Fehlerinjektionstest vor Implementierung | 15 Fehlerklassen; F-10, F-14, F-15 ohne belegtes Altnetz (Abgleich in WP-0.6) | Absturzmatrix rot → grün je Persistenzschritt |
| M-5 | Dauerhaftigkeit als Typzustand statt Konvention | P-3 (Reihenfolge) ist im Altcode Konvention | `Synced<T>` als Pflichteingabe des Catalog-Edits |
| M-6 | Sans-I/O-Zustandsmaschinen für Recovery, Redo, Erase, Compaction | Absturzzustände sind nur über Effekte vollständig aufzählbar | Absturzenumeration über Effektfolge (WP-9.2) |
| M-7 | Parse, don't validate | P-4, INV-FMT-2; im Altcode Längenprüfung nicht durchgängig belegt | Dekoder erzeugen `Bounded<T, MAX>`, `CanonicalVarint`; Fuzz, miri |
| M-8 | Fail-closed, kein stiller Zweitpfad | mmap-Fallback (§0.9); F-6 verlangt „nie als leer“ | Kein Fallback im Kern; Gegenprobe-Test je Lesepfad |
| M-9 | Strangler für Konsumenten | 122 Dateien hängen am Alt-Port | Adapter, Umstellung je Crate, Abbau nach Kriterien §9.4 |
| M-10 | Jede Altfunktion trägt einen Status | Offene Punkte verschwinden sonst still bis zum Phasenende | Migrationstabelle: `Szenario`, `ersetzt`, `verworfen`, `Adapter` |

### 1.3 Entscheidung: Neubau mit Verhaltensoracle

| Treiber (gemessen) | A: Umbau in place | B: Neubau nach SPEC | Ergebnis |
|---|---|---|---|
| Formatgenerationen | WAL-, SST- und Manifest-Formate müssten lesbar bleiben (`SSTABLE_FORMAT_VERSION = 4`, `MANIFEST_VERSION = 2`; Anzahl der Altversionen in WP-0.6 zu zählen) | Verwerfen (SD-04) | **B** |
| Kopplung | 42 Modulkanten, 3 Zyklen, `lsm` mit 10 Zielmodulen (inkl. `error`) | Zielschnitt erzwingt die Entflechtung | **B**; A bräuchte die Entflechtung als Vorarbeit, also praktisch den Neubau |
| Sync-Umstellung | 201 `async fn`, 203 `tokio`-Referenzen in Produktion | Durchgehend synchron (SD-02) | **B**; A ist ein Vollumbau aller Module |
| Testkopplung | 128 von 160 Integrationstests nutzen interne Modulpfade | Öffentliche API plus interne Tests | **B**; A müsste interne Pfade stabil halten und damit den Schnitt blockieren |
| Beweisbarkeit | loom-Modelle ohne Produktbezug | `sync.rs`-Fassade, loom auf echtem Code | **B** |

**Entscheidung:** B, mit Elementen einer Strangler-Migration für Konsumenten (C). Der Altstand dient als Verhaltensoracle (M-2) und als Szenariensteinbruch. Kein Altcode wird übernommen ohne Szenario-Test.
**Kosten:** Parallelbetrieb bis Phase M, doppelte Testpflege, höheres Anfangsrisiko. **Gegenmaßnahme:** Oracle-Tests ab G-0 (§8).

### 1.4 Paradigmen und Konzepte

| Paradigma | Anwendung im Projekt | Gewinn | Prüfmittel |
|---|---|---|---|
| Funktionaler Kern, imperative Hülle | Planer `plan()`, `retain()`, Katalog-Reducer `apply_edit(state, edit)` sind rein; E/A nur in der Hülle | Deterministisch testbar ohne Dateisystem | proptest auf dem Kern, Modelltest |
| Sans-I/O-Zustandsmaschinen | Redo-Schreiber, Recovery (11 Schritte), Erase E0–E7, Compaction-Job als `step(state, event) -> (state, Vec<Effect>)` | Absturzzustände sind Effektfolgen, kein Mock-Dateisystem nötig | Absturzenumeration (WP-9.2) |
| Typzustand für Dauerhaftigkeit | `Synced<T>` entsteht nur nach `sync_all` und `sync_dir`; `Catalog::commit` verlangt `Synced<FileSet>` | P-3 wird zur Compile-Zeit geprüft | Compile-fail-Tests (`trybuild`, nur dev) |
| Capability-Typen | `Space<Rd\|Ap\|Wr\|Adm>` (SD-34); `Pending` als eigener Typ | Falsche Rechte und Zustände sind nicht formulierbar | Compile-fail-Tests |
| Parse, don't validate | `Bounded<T, MAX>`, `CanonicalVarint`, `Offset`, `Len` (Newtypes mit `checked_*`) | Längenprüfung ist strukturell, nicht vergessbar; P-14 | Fuzz, miri (`codec`, `keys`) |
| Event-sourced Metadaten | Catalog = Snapshot plus Edit-Log; Zustand ist Reduktion (SD-05) | Ein Edit ist ein Zustandsübergang (P-2); Rollover ist trivial | Absturzmatrix Catalog |
| Strangler Fig, Branch by Abstraction | Adapter `contextra-storage-tx` (§9); Alt-Port bleibt bis zur Ablösung | Keine Umschaltung auf einen Schlag | Konformitätssuite (§9.4) |
| Charakterisierungstests (Feathers) | Verhalten des Altcodes als Referenz, Abweichungen als Befund | Verhaltenserhalt ohne Formatbindung | Differentialläufe (§8.1) |
| Deterministische Simulation | `Runtime::manual()`, Seeds, FaultFs | Fehler sind reproduzierbar, Seed steht im Bericht | Seed-Ausgabe bei jedem Fehlschlag |
| Model Checking | loom auf `sync.rs` (echter Code), Modelle 1–5 (§16.7) | Vollständige Erkundung der Verschränkungen | loom-Läufe in CI-Nachtlauf |
| Datenorientierung | `Bytes`-Ausschnitte ohne Kopie, Puffer je Job, Allokationsbudget (P-11) | Heißpfad ohne vermeidbare Kopie | Zählender Allokator |
| Gruppen-Commit (Flat Combining) | Ein `fsync` je Gruppe (SD-29, §10.2) | Durchsatz bei parallelen Schreibern (H-S2) | Messung H-S2 |

---

## 2. Ist-Zustand (gemessen)

### 2.1 Repository und Baseline

| Merkmal | Messwert | Beleg |
|---|---|---|
| HEAD | `8a24bf4` | `git log -1` |
| Crate-Stand | `47c9865` ist der letzte Crate-Commit, `b001610` der Stand laut SPEC | `git diff --stat b001610 HEAD -- crates/contextra-store` leer |
| Root-Manifest im HEAD | nicht vorhanden; `crates/` enthält nur `contextra-store` | `git ls-tree HEAD` |
| Historische Bereinigung | `813305c`: 21 Crate-Verzeichnisse, 2 931 Dateien; `47c9865`: 93 Verzeichnisse, 1 762 Dateien | `git show --name-status` |
| Historischer Workspace | 39 Mitglieder (38 unter `crates/`, 1 unter `benchmarks/`), 1 ausgeschlossen (`contextra-vault`) | `014319d:Cargo.toml` |
| Crate-Abhängigkeiten (Laufzeit) | 22 Einträge: 7 intern (`ids, env, types, ports, sys, mvcc, crypto`), 15 extern (u. a. `tokio`, `parking_lot`, `memmap2`, `blake3`, `crc32fast`, `lru`, `quick_cache`, `ahash`, `tokio-util`, `tracing`, `thiserror`, `uuid`, `rand`, `serde`, `bytes`) | `Cargo.toml` |
| Dev-Abhängigkeiten | 11: `contextra-testkit`, `contextra-env`, `libc`, `serde_json`, `tokio`, `tempfile`, `criterion`, `proptest`, `loom`, `futures`, `redb` | `Cargo.toml` |
| Plattform-Governance | `capabilities.fragment.toml` und `[package.metadata.contextra] ring = "1"`; `allowed_internal` nennt 5 Plattform-Crates, die SPEC R2 nicht mehr nutzt | `capabilities.fragment.toml` |
| Werkzeuge im Crate | `tools/runner.sh` (Test-, Mess- und Audit-Runner) — im SPEC-Layout nicht vorgesehen | `tools/` |
| Toolchain | nicht vorhanden | `cargo: not found` |

### 2.2 Umfang nach Modul

Produktionszeilen = vor dem ersten `#[cfg(test)] mod`; Testcode = Testdateien und Testmodule.

| Modul (Alt) | Produktion | Testcode in `src/` | Produktionsdateien | Ziel (SPEC §3.2) | Behandlung |
|---|---:|---:|---:|---|---|
| `wal/` | 4 664 | 3 707 | 15 | `redo/` | Neu; Szenarien übernehmen |
| `lsm/` | 4 848 | 2 790 | 33 | `api/`, `engine/` | Neu; God-Modul zerlegen |
| `sstable/` | 3 370 | 918 | 20 | `run/` | Neu; Blocksuche als Algorithmus |
| `compaction/` | 1 610 | 2 354 | 11 | `engine/compaction/` | Neu; Planer rein |
| `compat/` | 1 977 | 673 | 15 | entfällt | Port-Brücke, Lizenz, Scanner verwerfen (§4) |
| `manifest/` | 1 161 | 444 | 8 | `catalog/` | Neu |
| `memtable/` | 526 | 478 | 4 | `engine/memtable.rs` | Neu, ohne Sharding |
| `io_seam/` | 365 | 0 | 3 | `env/`, Port `Fs` | Neu |
| `system_pressure/` | 279 | 0 | 3 | `env/governor.rs` | Neu |
| `tenant_codec/` | 3 | 43 | 1 | entfällt (NZ-18) | Verwerfen |
| `scanner/` | 3 | 0 | 1 | entfällt | Verwerfen |
| Dateien unmittelbar unter `src/` | 1 563 | 239 | 15 | verteilt | Einzeln zuordnen (WP-0.6) |
| **Summe** | **20 369** | **11 646** | **129** | | |

Zusätzlich: `tests/` 25 563 Zeilen in 160 `.rs`-Dateien (176 Dateien gesamt inkl. Fixtures). `benches/`, `examples/`, `fuzz/`: 2 210 Zeilen.

### 2.3 Kopplung und Schichtregel

Gemessen: Kanten `use crate::<modul>` über Modulgrenzen in der Produktion. Zuordnung der Module zum Zielschnitt nach §3.3 SPEC (`lsm` = api und engine, `wal` = redo, `sstable` = run, `manifest` = catalog, `util` = keys, `io_seam` und `fsync` = env).

| Kante | Vorkommen | Zielschnitt | Status |
|---|---:|---|---|
| `lsm → error`, `sstable → error`, `wal → error`, `manifest → error`, `compaction → error` | 19, 12, 10, 6, 5 | `error` als gemeinsamer Typ | zulässig |
| `lsm → wal` | 11 | engine → redo | zulässig |
| `sstable → io_seam` | 6 | run → env | zulässig |
| `sstable → wal` | 4 | run → redo | **Verletzung** |
| `compaction → io_seam` | 2 | engine → env (nicht im Diagramm) | **Verletzung** |
| `compaction → system_pressure` | 1 | engine → env | **Verletzung** |
| `util → lsm` | 1 | keys → engine | **Verletzung**, Teil des Zyklus `lsm↔util` |
| `lsm → budget` | 1 | engine → env | **Verletzung** |
| `lsm → io_seam` | 1 | engine → env | **Verletzung** |
| `lsm → util` | 1 | api/engine → keys | **Verletzung**, Teil des Zyklus `lsm↔util` |
| `lsm → stats`, `compaction → stats` | 1, 3 | engine → Typ aus `api/` | Zuordnungsfall, siehe Ergebnis |
| `compat → lsm` (8), `lsm → compat` (2) | 10 | entfällt | Zyklus, verschwindet mit `compat` |
| `compat → tenant_codec` (5), `tenant_codec → compat` (2) | 7 | entfällt | Zyklus, verschwindet mit `compat` |

**Ergebnis:** 3 Zyklen. 11 klare Verletzungsvorkommen (`sstable→wal` 4, `compaction→io_seam` 2, `compaction→system_pressure` 1, `lsm→budget` 1, `lsm→io_seam` 1, `lsm→util` 1, `util→lsm` 1). 4 Zuordnungsfälle (`lsm→stats` 1, `compaction→stats` 3): `stats` liegt im Zielschnitt in `api/`, wird aber von der Engine referenziert. `lsm` hat ausgehende Kanten zu zehn Modulen (`error, wal, compaction, sstable, memtable, compat, budget, io_seam, stats, util`).

### 2.4 Regelkonformität gegen P-1…P-14

| Regel | Messung (Produktion) | Soll | Status |
|---|---|---|---|
| P-1 nur über `Fs` | 86 direkte FS-Aufrufe außerhalb `io_seam` | 0 | ✗ |
| P-2 ein Catalog-Edit je Zustandsänderung | Manifest-Einträge tragen `PathBuf` (`ManifestEntry::Add { path, max_tx }`, `manifest/entry.rs:23`) | numerische IDs, ein Edit je Änderung | ✗ (ersetzt) |
| P-4 totale Dekodierung | keine Längenstruktur belegbar; 13 Fuzz-Ziele vorhanden | total | ungeprüft |
| P-5 keine globalen Zustände | 12 statische Items: 7 Testschalter, 4 Dateinamen-Zähler, 1 `Once` | 0 (Injektion über `Env::hooks()`) | ✗ |
| P-5 keine `cfg` in Logik | 43 Stellen: `fault-injection` 17, `workspace` 11, `legacy-wal-key` 7, `block-cache-v2` 6, `memory-only-storage` 1, `docid-128` 1 | nur `testing` | ✗ |
| P-6 Dateien ≤ 400 Zeilen | 11 Dateien, Maximum 966 (`compaction/engine.rs`) | 0 | ✗ |
| P-9 kein Lock über E/A außer Committer | nicht statisch messbar | Lock-Analyse in F5 | offen |
| P-12 keine Geheimnisse in Logs | nicht gemessen | Lecks-Orakel | offen |
| P-13 kein `unwrap`, `expect`, `panic` | 0 Vorkommen; 2 `allow`-Attribute | 0 | ✓ |
| P-14 keine Breiten-Casts | 133 Integer-`as`-Casts (Regex, Obergrenze, Anhang A.1) | 0 | ✗ |

### 2.5 Nebenläufigkeit und Beweisbarkeit

- **Sperren:** `parking_lot` in Produktion. 10 Rang-Konstanten in `lsm/lock_order.rs` (`RANK_*` 100…950). Außerhalb der Datei genau eine Referenz; keine Laufzeitprüfung, kein Rangstack.
- **loom:** 4 Dateien, 524 Zeilen (`tests/loom_*.rs`). Sie importieren `loom::sync` direkt und enthalten keinen Import aus `contextra_store`. Die Modelle sind handgeschriebene Nachbildungen.
- **cfg(loom):** 24 Stellen; Produktionsmodule sind in `lib.rs` unter `cfg(not(loom))` ausgeschlossen. Der Produktionscode ist unter loom nicht erreichbar.
- **Async:** 201 `async fn`, 203 `tokio`-Referenzen in Produktion, obwohl blockierende E/A stattfindet (`fsync.rs`: `pub async fn sync_file`).
- **Retry:** kein Wiederholungsmuster für `fsync` in Produktion gefunden. Konform zu INV-LOG-5, aber nicht getestet.

### 2.6 Fehler- und Sicherheitsbefunde

| ID | Befund | Ort | Konsequenz |
|---|---|---|---|
| B-a | WAL-Replay über mmap; bei jedem Fehler still zweiter Lesepfad, nur `warn!` | `wal/replay.rs:115–135`; mmap über `contextra_sys::mmap_readonly` (`:163`) | Zwei Replay-Pfade. Der mmap-Pfad ist über `contextra-sys` (Unsafe-Insel) angebunden, nicht über `memmap2` direkt. |
| B-b | Legacy-HMAC-Schlüssel XOR-maskiert mit `0x5A` | `wal/hmac.rs:12–36`, Feature `legacy-wal-key` (6 Testziele) | Kein Geheimnis; mit Binärzugriff rekonstruierbar. Verwerfen (SD-04). |
| B-c | Drei Testschalter ohne Gate, im Append-Pfad mit `SeqCst` geladen | `wal/core.rs:27–29`; gelesen in `wal/io/append.rs:38, 50, 52` | Testlogik im Release-Pfad. Prozessweiter Zustand wirkt auf alle Instanzen. |
| B-d | Vier Testschalter unter `fault-injection` | `wal/core.rs:32–40`, `wal/flusher.rs:14–15` | Injektion über Feature statt über `Hooks`. |
| B-e | Prozessweite Zähler für Dateinamen | `wal/flusher.rs:12`, `compaction/engine.rs:23`, `sstable/reader/open.rs:440`, `lsm/recovery/salt.rs:12` | Namensraum nach Neustart reproduzierbar. Eindeutigkeit ist nicht über eine Hochwassermarke gesichert (F-14). |
| B-f | Tote Features | `inspect`, `sieve-cache`: 0 `cfg`-Treffer; `test-hooks`: 1 Treffer | Konfigurationsrauschen |
| B-g | Doku-Drift | `fuzz/README.md` nennt 1 von 13 Zielen | Doku nicht maßgeblich |
| B-h | Invariantenbenennung | 30 eigene `INV-*`-Kennungen (z. B. `INV-FSYNC-1`: 32 Stellen, `INV-LOCK-ORDER-1`: 12, `INV-STORE-1`: 10), kein Bezug zu den 43 Kennungen von SPEC §18 | Mapping-Tabelle ist Pflichtartefakt von WP-0.6 |

### 2.7 Tests

| Messgröße | Wert |
|---|---|
| Testdateien | 160 in `tests/` (25 563 Zeilen) und 32 Testmodul-Dateien in `src/` |
| Testfunktionen | 126 `#[test]`, 525 `#[tokio::test]`, 12 `#[ignore]` |
| proptest-Dateien | 14 |
| `[[test]]`-Ziele | 22: 11 nur `test-hooks`, 4 `fault-injection` plus `test-hooks`, 6 `legacy-wal-key`, 1 ohne Feature (`differential_redb`) |
| Integrationstests mit internem Modulpfad | 128 von 160 (`contextra_store::wal::…`, `::lsm::…` usw.) |
| Dateien mit Legacy-Bezug im Namen | 13 |
| Byte-Identitätstests `mz_*` | 5 Dateien, 3 Support-Verzeichnisse; Format-Regression gegen Altformat |
| Golden-Dateien | `tests/golden/` (Altformate, `MANIFEST.sha256`), `tests/fixtures/` (Altformate) |

### 2.8 Konsumenten (Stand `014319d`)

| Crate | `.rs`-Dateien | Bezug zum Store | Ablösepfad |
|---|---:|---|---|
| `contextra-db` | 150 | 37 Dateien importieren `contextra_store` (Manifest: `features = ["workspace"]`); 50 Dateien mit `StorageEngine`; 2 mit `FeatureRing` | Adapter (M.1), Lizenzentkopplung (M.4) |
| `contextra-text` | 74 | 34 mit `StorageEngine`; 8 mit `Purger`; 2 mit `LayerScanner` | Adapter (M.1), Erasure-Brücke (M.2) |
| `contextra-graph` | 155 | 18 mit `StorageEngine`; 7 mit `Purger` | Adapter (M.1), M.2 |
| `contextra-engine` | 54 | 8 mit `StorageEngine`; 8 Tests importieren `contextra_store` **ohne Abhängigkeit im Manifest** (Befund); `MergeOperator` in `tests/wasm_merge_operator.rs:13` | M.1; Tests vor Migration klären |
| `contextra-testkit` | 26 | 12 mit `StorageEngine` | M.1 |
| `contextra-vector` | nicht gezählt | 6 mit `Purger`; 2 mit `LayerScanner` | M.2 |
| `contextra-erasure` | nicht gezählt | 3 mit `Purger`; 3 mit `LayerScanner` | M.2 |
| `contextra-crypto`, `-ids`, `-testkit` | nicht gezählt | `FeatureRing` in 1, 1 und 1 Datei | M.4 |

„Dateien mit Textbezug“ sind Textsuchtreffer und enthalten `use`-Zeilen, Kommentare und Tests.

---

## 3. Korrekturen gegenüber Rev. 1

Status: ✓ bestätigt · ✗ falsch · ◐ teilweise oder unvollständig · ? nicht verifiziert.

| # | Rev. 1 | Messung | Status |
|---|---|---|---|
| 1 | `src/` 32 015 Zeilen | 32 015, davon 11 646 Testcode, 20 369 Produktion | ✓ (Interpretation korrigiert) |
| 2 | `tests/` 25 563 Zeilen, Header „159 Dateien“ | 160 `.rs`-Dateien, 176 Dateien gesamt | ✗ (Header) |
| 3 | 59 788 Zeilen insgesamt | 59 788 `.rs`-Zeilen (src, tests, benches, examples, fuzz) | ✓ |
| 4 | „36 Contextra-Crates im Workspace“ | 39 Mitglieder (38 `crates/`, 1 `benchmarks/`) | ✗ |
| 5 | `io_seam/` und `fsync.rs` ≈ 500 Zeilen | 890 (`io_seam.rs` 397, `io_seam/` 365, `fsync.rs` 128) | ✗ |
| 6 | `system_pressure/` 279 Zeilen | 279 im Verzeichnis, plus Datei `system_pressure.rs` mit 175 | ◐ |
| 7 | `wal/` 8 371, `lsm/` 7 638, `sstable/` 4 288, `compaction/` 3 964, `manifest/` 1 605, `memtable/` 1 004, `compat/` 2 650 | exakt (Summe inkl. Testcode) | ✓ |
| 8 | 13 Fuzz-Ziele, 6 Benches | 13 und 6; davon 3 Benches mit Feature `workspace` | ✓ |
| 9 | „achtstufige Sperrhierarchie“ (B-4) | 10 Ränge in `lsm/lock_order.rs` | ✗ |
| 10 | 16 Memtable-Shards | `SHARD_COUNT = 16` (`memtable/shard.rs:19`) | ✓ |
| 11 | 142 Tombstone-Maskierungsstellen | 142 Treffer (`TOMBSTONE_BIT` bzw. `1 << 63`) | ✓ |
| 12 | Bloom-Filter fest für 100 000 Elemente | `BloomFilter::new(100_000, 0.01)` (`sstable/builder/writer.rs:102`) | ✓ |
| 13 | Drei Block-Caches | `lru`, `quick`, `sieve` plus `mod.rs` und `traits.rs` | ✓ |
| 14 | „Sechs statische Testschalter (wal/core.rs), im Release“ | 7 Schalter: 3 ungegatet in `core.rs`, 3 gegatet in `core.rs`, 1 gegatet in `flusher.rs`. Die drei ungegateten laufen im Append-Pfad mit. | ◐ |
| 15 | 91 direkte FS-Aufrufe am Port vorbei (B-3) | 86 (Definition Anhang A) | ✗ |
| 16 | `memmap2` entfernen | mmap läuft über `contextra_sys::mmap_readonly`; `memmap2` wird nicht direkt aufgerufen | ◐ |
| 17 | Abhängigkeitsliste (§2) | Fehlend: `ahash`, `tokio-util`, `contextra-sys`, `contextra-mvcc`, `contextra-crypto`. Dev: `redb`, `criterion`, `libc`, `tempfile`, `serde_json`, `futures`, `contextra-testkit` | ◐ |
| 18 | loom-Modelle 1–4 vorhanden (F5) | Dateien vorhanden, testen aber keinen Produktionscode | ◐ |
| 19 | Hypothesen H-S1…H-S19 (§5, G-P) | SPEC §20: H-S1…H-S18 | ✗ |
| 20 | Testklassifikation: 158 Dateien | 160 Dateien; die Klassenzuordnung über Dateinamen ist nicht reproduzierbar | ✗ (ersetzt durch Marker, §8.2) |
| 21 | Versionen WAL V1–V3, SST v0–v4, Manifest v0–v2 (B-5) | `SSTABLE_FORMAT_VERSION = 4`, `MANIFEST_VERSION = 2`; WAL-Konstante nicht gemessen | ◐ |
| 22 | B-1, B-2, B-9, B-11, B-12, B-13, B-14 | nicht gegengeprüft; B-9 („Fehler verschluckt“) ist nicht geprüft | ? (WP-9.5 prüft) |

**Bestätigt:** 8 · **falsch:** 7 · **teilweise:** 6 · **nicht verifiziert:** 1 (Bündel).

---

## 4. Blocker und Vorbedingungen

Jeder Blocker muss vor dem genannten Gate entschieden sein. Empfehlungen sind Vorschläge; die Entscheidung trägt der Projektverantwortliche.

| ID | Befund | Optionen | Empfehlung | Frist |
|---|---|---|---|---|
| **BL-1** | Baseline nicht baubar (§2.1) | (a) Workspace-Subset aus `014319d` mit den 8 Abhängigkeitscrates (7 Laufzeit, 1 Dev), Lockfile fixiert; (b) vollständige Historie | (a). Bei Fehlschlag: Befund, kein Umbau. | G-0 |
| **BL-2** | `contextra-ports` existiert historisch mit 7 184 Zeilen und Nutzern | (a) neues Crate `contextra-storage-ports`; (b) `contextra-ports` behalten und Historie umbenennen; (c) neues Crate in eigenem Workspace | (a). Kostet nichts, da das Crate noch nicht existiert. (b) berührt 46 Historiendateien; (c) verhindert die Reintegration. Antrag CR-1. | G-0 |
| **BL-3** | `StorageEngine` mit 122 Dateien Textbezug, Semantik nicht im Kern | siehe O-10 | Adapter `contextra-storage-tx` mit Austrittskriterien (§9.4) | G-A |
| **BL-4** | Konsumenten fehlen im HEAD; `014319d` enthält sie | siehe O-9 | Konsumenten bleiben außerhalb dieses Repositories bis Phase M | G-0 |
| **BL-5** | `Purger` (24 Dateien) und `LayerScanner` (7) sind Port-Traits, deren Implementierung im Store liegt (`compat/scanner/purge.rs`, `memtable.rs`, `sstable.rs`, `wal.rs`) | (a) Brücke: `Purger` über `Erase` und `erase_status`, `LayerScanner` über `Segment` und `View`; (b) Konsumenten auf neue API umstellen | (a) als WP-M.2 | vor M.2 |
| **BL-6** | `compat/license.rs`: `validate_against_features` koppelt `DurabilityMode` an `FeatureRing::Sovereign` (Persistenzpflicht). `FeatureRing` in 5 Konsumenten-Dateien. | Lizenzprüfung entfällt aus dem Kern; der Kern erzwingt nur `integrity`-Pflicht und `domain_sealed`-Konsistenz (`Config`-Validierung) | Entfernen; Prüfung in den Konsumenten (M.4) | vor M.4 |
| **BL-7** | Referenzhardware (A-09) und Toolchain (O-1) nicht festgelegt. SPEC §20 fordert Schwellen relativ zu einer „Referenz“, nennt aber keinen Wert. | WP-0.7: Altstand auf derselben Hardware messen; daraus die Basis für H-S-Grenzen | Altstand-Basis vor P1 | G-0 |

---

## 5. Zielabbildung

| Ziel | Arbeitspakete | Abnahme (messbar) |
|---|---|---|
| Z-1 Atomare Changes mit Guards | 3.5, 4.1, 4.2 | Absturzmatrix: jede Präfixlänge des Redo ergibt einen Modellzustand (INV-LOG-2, INV-VIS-2); Modelltest 0 Abweichungen |
| Z-2 Views, Holds, Consumer | 4.1, 4.7, 5.1 | INV-VIS-3 und INV-RET-1 mit loom auf echtem `sync.rs`; `view_at` unter Untergrenze liefert `HistoryPruned` |
| Z-3 Physische Löschung | 3.3, 3.4, 6.1, 6.2 | ErasureOracle: 0 Treffer in allen Dateien (Runs, Blob-Generationen, Redo, Segmente, `.building`); Absturzmatrix E1–E7 |
| Z-4 Autark | 0.3, 1.1, 1.2 | `cargo tree -e normal -p contextra-storage` zeigt nur `contextra-storage-ports`; `cargo deny` grün |
| Z-5 Einfach | Gesamt | Dateien gemäß §6 (≈ 92); 0 Zyklen; 0 Schichtverletzungen |
| Z-6 Beweisbar robust | 9.1–9.5 | Netze aus SPEC §19 grün; 43 von 43 INV-IDs zitiert (`tests/invariants.rs`) |
| Z-7 Agenten- und RAG-Daten | 4.2, 5.1 | `Busy{retry_after}` in Stalltests; `Pending` ohne Executor (SPEC §7.8.6) |
| Z-8 Betreibbar | 8.1 | `verify` findet injizierte Befunde (Bitflip, Lost Write, Misdirected Write); Metriken ohne Schlüsselbytes (INV-API-1) |
| Z-9 Budgetierte Ergebnisse | 3.2, 5.1 | `scan_page` mit Fortschrittsgarantie (`oversized`); `estimate` ≤ 2× obere Schranke (H-S8) |
| Z-10 Erweiterbar ohne Formatbruch | 3.2, 5.2, 5.3 | Pflicht- und optional-Bit-Tests; Golden des Segment-Rahmens (P10) |
| Z-11 Robust unter dem Fehlermodell | 1.3, 9.2, 9.3 | je ein FaultFs-Test für F-1…F-15 mit definiertem Zustand danach |
| Z-12 Ressourcen | 1.4, 4.6 | Allokationszähler (P-11); RSS-Grenze; `ulimit -n 256` mit 1 000 Runs |
| Z-13 Sicherheit | 2.1, 7.1–7.3 | Downgrade-Test (F-15); Nonce-Property (INV-KEY-1); Lecks-Orakel 0 Treffer |
| Z-14 Nebenläufigkeit | 0.4, 9.4 | Rangstack-Test (INV-LOCK-1) im Debug-Build; loom-Modelle 1–5 auf echtem Code; Mutationsprobe findet Reihenfolgefehler |

---

## 6. Arbeitspakete

**Umfang** (Schätzung, Ø 180 Produktionszeilen je Datei): S ≤ 800 · M 800–2 000 · L 2 000–4 000 · XL > 4 000.
**Komplexität:** 1 gering · 2 mittel · 3 hoch (Persistenz, Nebenläufigkeit, Kreuzabhängigkeit).

### WP-0 Fundament

| WP | Gegenstand | Alt-Quelle | Umfang | Kompl. | Abnahme |
|---|---|---|---|---|---|
| 0.1 | Baseline-Rekonstruktion aus `014319d`, Lockfile, Warnungsinventar | Historie (B.2) | S | 2 | `cargo check --lib` grün; Warnungszahl und Test-Kompilierstatus protokolliert |
| 0.2 | Workspace, Toolchain-Pin, Lints (P-13), `deny.toml`, `rustfmt.toml`, CI-Gerüst, `tests/layering.rs`, `tests/invariants.rs`; `capabilities.fragment.toml` ersetzt | `Cargo.toml`, `capabilities.fragment.toml` | S | 1 | Schichtverletzung-Probe bricht CI; `cargo tree`-Gate aktiv |
| 0.3 | `contextra-storage-ports` nach SPEC §4 | alte Port-Traits nur als Referenz | S | 1 | `cargo doc` ohne Warnung; keine Abhängigkeit; `FsCaps`, `KeyProvider` konform |
| 0.4 | `sync.rs`-Fassade, `Ranked<T, RANK>` mit Debug-Rangstack | `lsm/lock_order.rs` (107 Zeilen) | S | 2 | INV-LOCK-1 testet Rangverletzung als Panic; loom-Smoke grün |
| 0.5 | Referenzcodierer (Python) für Goldens A.1–A.7; Dev-Crate `contextra-crypto-ref` (O-6) | `contextra-crypto` (`014319d`) als Kandidat prüfen | M | 2 | Goldens in zwei unabhängigen Implementierungen identisch |
| 0.6 | Testinventar und Mapping: jede Testdatei mit Disposition; Mapping der 30 Alt-`INV-*`-Kennungen auf neue IDs; Zuordnung der 15 Dateien unmittelbar unter `src/` | `tests/`, `src/**/tests*`, Befund B-h | S | 1 | 192 Dateien je mit Status; Mapping vollständig |
| 0.7 | Altstand-Referenzmessung auf Referenzhardware: H-S1, H-S2, H-S7, H-S12 | `benches/` (6) | M | 2 | Bericht mit Streuung; Basis für H-S-Grenzen (BL-7) |

### WP-1 Primitive und Umgebung

| WP | Gegenstand | Alt-Quelle | Umfang | Kompl. | Abnahme |
|---|---|---|---|---|---|
| 1.1 | `codec`: kanonischer Varint, CRC32C portabel, XXH64, Tag, KeyRange, Bytes, Limits | `wal/encode.rs`, `util.rs` (Ideen) | M | 2 | Goldens A.1–A.3; `decode∘encode = id`; Fuzz 60 s; miri |
| 1.2 | `env`: `RealFs` (nur `std`), `Clock`, `Entropy` (O-13), Suite `fs_suite` | `io_seam.rs`, `io_seam/`, `fsync.rs` (890 Zeilen) | M | 2 | `fs_suite` grün; INV-FS-1: `Strict` verweigert schwache Eigenschaften |
| 1.3 | `testing`: `MemFs`, `FaultFs` (Absturz, Sektor-Zerriss, Lost und Misdirected Write, EIO, ENOSPC, Umordnung, Hooks) | Testschalter als Muster, kein Code | L | 3 | Selbsttests §19.2; Läufe deterministisch aus Seed |
| 1.4 | `Runtime` (teilbar, `manual()`), Governor-Gerüst, Metrics-Port, `PressureSource` | `system_pressure/` (279), `system_pressure.rs` (175), `budget.rs` (91) | M | 2 | Deterministische Einplanung; Quoten-Tests |

### WP-2 Schlüssel und Kopf

| WP | Gegenstand | Alt-Quelle | Umfang | Kompl. | Abnahme |
|---|---|---|---|---|---|
| 2.1 | `keys`: Hierarchie, zentrale Nonce-Tabelle, Epochen, `Secret32` | `key_source.rs` (67), `util.rs` Sealer (155) | M | 2 | INV-KEY-1 als Property; Nonce-Goldens A.6 |
| 2.2 | `FORMAT`: Pflicht- und optionale Bits, Modus, Downgrade-Schutz | Altes WAL-Header (nicht übernommen) | S | 1 | F-15-Test; Golden A.4 |

### WP-3 Persistenz

| WP | Gegenstand | Alt-Quelle | Umfang | Kompl. | Abnahme |
|---|---|---|---|---|---|
| 3.1 | `run`, Schreibseite: Builder, Block, Footer, Index, Filter (Bloom, XXH64), PrefixFilter, RangeDel | `sstable/builder/*`, `sstable/bloom.rs`, `sstable/format.rs` | M | 2 | Golden A.6 (Run); H-S9; Bitflip-Netz |
| 3.2 | `run`, Leseseite, `TableCache`, CLOCK-`BlockCache` (eine Implementierung), Extension-Schnittstelle | `sstable/reader/*` (Algorithmus `block_search.rs`), `sstable/block_cache/*` (3 Varianten) | M | 2 | H-S1, H-S12; ein Cache-Pfad; verzögertes Laden |
| 3.3 | `blob`: Record, Writer, Reader, Sweep; Hole-Records unverschlüsselt | neu | S | 3 | INV-BLOB-1…3; Orakel |
| 3.4 | `catalog`: Edit, Log, State, CURRENT, Reserve, Rollover | `manifest/` (1 161) | M | 3 | INV-CAT-1…4; Absturzmatrix Catalog; Torn-Tail-Test; H-S4, H-S5 |
| 3.5 | `redo`: Frame, Segment, Seal, Writer, Reader, Recover, Prealloc | `wal/` (4 664 Produktion; Dateigröße inkl. Testmodul: `replay.rs` 729, `flusher.rs` 771) | L | 3 | INV-LOG-1…8; Absturzmatrix Redo; Golden A.5; Fuzz `wal_roundtrip` |

### WP-4 Engine

| WP | Gegenstand | Alt-Quelle | Umfang | Kompl. | Abnahme |
|---|---|---|---|---|---|
| 4.1 | `memtable` (`RwLock<BTreeMap>`, 64-Op-Unterbatches), `version`, `views`, `retention` (rein), `registry` | `memtable/` (526), Anteile aus `lsm/` | M | 2 | H-S7 (±20 % zum Altstand); Modelltest für INV-RET-1 |
| 4.2 | `committer` (Flat Combining, Gegendruck vor dem Ticket, Guard-Vorwärmung), Listener-Dispatch | Anteile aus `lsm/` (`commit`, `group_commit`, `ops/write`) | L | 3 | H-S2; loom-Modelle 1–2 auf echtem Code; INV-VIS-1…3 |
| 4.3 | `flush`, `recovery` (11 Schritte, §13.1), Verwaisten-Suche | `lsm/flush.rs`, `lsm/recovery/*` (`startup.rs` 584) | L | 3 | Absturzmatrix §13.2 vollständig; H-S4; INV-FLUSH-1 |
| 4.4 | `compaction`: `plan` (rein), `retain` (rein), `exec`, `install` | `compaction/` (1 610 Produktion; `engine.rs` 966) | M | 2 | Planer-Gesetze (keine Überlappung auf Level ≥ 1); INV-CLAIM-1 (loom) |
| 4.5 | Blob-GC und Konsolidierung (SD-43) | neu | S | 2 | H-S17; INV-BLOB-2 |
| 4.6 | Maintenance (Lanes, Claims), Graveyard, Cleanup | `compaction/engine/throttle.rs` (Konzept) | S | 2 | INV-CAT-4; Claim-Konflikttests |
| 4.7 | Dauerhafte Holds, Consumer-Cursor | `pin_checkpoint`-Logik in `lsm/` | S | 2 | INV-HOLD-1 (P9) |

### WP-5 API

| WP | Gegenstand | Alt-Quelle | Umfang | Kompl. | Abnahme |
|---|---|---|---|---|---|
| 5.1 | `api`: Storage, Space, Change und Guard, View, Cursor, Page und ResumeToken, `estimate`, `history`, Config, Error mit `hint()`, `submit`/`Pending`, `inspect` | `lsm/engine/facade.rs`, `error.rs` (180), `traits.rs` (139) | L | 2 | Konformitätstests (`testing::conformance`); Lecks-Orakel; Fehlermatrix §7.7 |
| 5.2 | Segment-Rahmen: Writer, Reader, Siegel | neu | M | 2 | INV-SEG-1 (P10) |
| 5.3 | Extension-Schnittstelle (instabil bis F10, A-10) | neu | S | 1 | Dokumentierte Instabilität; Lesetest für Extension-Blöcke |

### WP-6 Löschung

| WP | Gegenstand | Alt-Quelle | Umfang | Kompl. | Abnahme |
|---|---|---|---|---|---|
| 6.1 | `erase/plan`, `erase/exec` (E0–E7), `erase_status`, `ErasureReceipt`, `Erase::domain` | `compat/scanner/purge.rs` (Referenz), Purge-Logik in `lsm/` | L | 3 | INV-ERA-1…7; Absturzmatrix E1–E7; Roll-forward-Test |
| 6.2 | `erase/registry`, `verify`, `physical_scan` | neu | M | 2 | INV-SCAN-1; ErasureOracle |

### WP-7 Modi und Schlüssel

| WP | Gegenstand | Alt-Quelle | Umfang | Kompl. | Abnahme |
|---|---|---|---|---|---|
| 7.1 | `Authenticated`: Mac16 mit AAD-Bindung in allen Writern | `wal/hmac.rs` (Konzept) | M | 2 | Manipulationstests (A-3); Goldens im Modus `Authenticated` |
| 7.2 | `Encrypted` (AEAD), `domain_sealed`, KEK-Rotation in O(1), Epochen-Klon | neu | L | 3 | INV-KEYP-1, INV-EPOCH-1; H-S18 |
| 7.3 | `RollbackAnchor` (optional), Secret-Handling | neu | S | 2 | F-10-Test mit Anker; Lecks-Orakel |

### WP-8 Betrieb

| WP | Gegenstand | Alt-Quelle | Umfang | Kompl. | Abnahme |
|---|---|---|---|---|---|
| 8.1 | `verify` (Online, Offline), `checkpoint` (Copy, Hardlink), `open_readonly`, `hibernate`, `stats` | neu | L | 2 | Injizierte Befunde erkannt; H-S16 |
| 8.2 | `ColdTier`-Port (F10) | neu | S | 2 | Löschung über `ColdTier::delete` in E6 |

### WP-9 Verifikation (ab P1 parallel)

| WP | Gegenstand | Alt-Quelle | Umfang | Kompl. | Abnahme |
|---|---|---|---|---|---|
| 9.1 | Modelltest: Referenzmodell mit Holds, Consumern, `erased_ranges` | `tests/model_based_mvcc_flush.rs`, `tests/campaign_prop_lsm_ops.rs` (Szenarien) | L | 3 | Seed-reproduzierbar; alle Operationen aus §19.1 |
| 9.2 | Absturzenumerator: enumeriert I/O-Indizes und Hook-Marken, vergleicht mit Modell-Präfix | `tests/crash_prefix_enumeration.rs`, `fault_injection_*` (Szenarien) | M | 3 | Jede Zeile aus §13.2 ist ein Testfall |
| 9.3 | Bitflip-Netz, Fuzz (13 Ziele neu gerahmt), Stateful-Fuzz | `fuzz/fuzz_targets/*`, `tests/chaos_bitflip_sstable.rs` | M | 2 | Smoke 60 s je Ziel; Bitflip liefert nie falsche Daten |
| 9.4 | loom-Modelle 1–5 auf `sync.rs` (ersetzt die Nachbildungen in `tests/loom_*`) | `tests/loom_*` (4 Dateien, Nachbildungen) | M | 3 | Modelle importieren `contextra_storage`; Mutationsprobe (Reihenfolgefehler) wird gefunden |
| 9.5 | Orakel: Löschung, Lecks, Allokation, Schicht, Ressourcen, Befundlauf für B-c, B-e, B-h | neu | M | 2 | Jedes Orakel hat einen absichtlich verletzenden Gegenbeispieltest |
| 9.6 | Benchmarks H-S gegen Altstand-Basis | `benches/` (6; 3 mit `workspace`) | M | 1 | G-P-Bericht |

### WP-M Migration (ab G-F)

| WP | Gegenstand | Alt-Quelle | Umfang | Kompl. | Abnahme |
|---|---|---|---|---|---|
| M.1 | Adapter `contextra-storage-tx`: TxId, Puffer, Savepoints, SSI-Lesemengen, Future-Fassade | 45 Dateien mit Tx-, Commit-, Guard-, SSI- oder Rollback-Bezug im Namen, als Szenarien | XL | 3 | Konformitätssuite ≥ 95 % mit Oracle-Abgleich (§9.4) |
| M.2 | Erasure-Brücke: `Purger`- und `LayerScanner`-Nachfolge | `compat/scanner/*` (Referenz) | L | 3 | Tests von `contextra-vector`, `-text`, `-graph`, `-erasure` grün |
| M.3 | Konsumenten-Umstellung: `db`, `text`, `graph`, `testkit`, `engine`, `vector`, `erasure` | Konsumenten (§2.8) | XL | 3 | Je Crate eigene Abnahme; Altcode-Tests als Szenarien übernommen |
| M.4 | Lizenzentkopplung (`FeatureRing`) | `compat/license.rs` | S | 1 | Kern ohne `FeatureRing`-Bezug |
| M.5 | Abbau: `legacy/`, Adapter, `compat` | – | S | 1 | Kriterien §9.4 erfüllt |

**Summe Zielumfang (Schätzung):** 92 Dateien laut SPEC §3.2 und §4, bei Ø 180 Zeilen ≈ 16,5 k Produktionszeilen gegen 20,4 k im Altstand. Testcode: Planungsannahme 1:1 zum Produktionscode, nicht gemessen.

---

## 7. Phasen, Gates, Reihenfolge

| Phase | Arbeitspakete | Eintritt | Austritt (messbar) |
|---|---|---|---|
| **P0** | 0.1–0.7 | BL-1, BL-2, BL-4, BL-7 entschieden | **G-0:** Altcode kompiliert; Port-Crate konform; Goldens A.1–A.3 in zwei Implementierungen; Altstand-Basis gemessen |
| **P1** | 1.1–1.4 | G-0 | **G-1:** FaultFs-Selbsttests; `fs_suite` für `RealFs` und `MemFs`; Codec-Goldens; Fuzz-Smoke |
| **P2** | 2.1, 2.2 | G-1 | **G-2:** Nonce-Property; Downgrade-Test; Goldens A.4, A.6 |
| **P3** | 3.1–3.3 | G-2 | **G-3:** H-S1, H-S9, H-S12 gemessen (Abweichung dokumentiert, §20-Regel); Bitflip Run grün |
| **P4** | 3.4, Recovery aus 4.3 | G-3 | **G-4:** Absturzmatrix Catalog und Redo; H-S4, H-S5 gemessen |
| **P5** | 3.5, 4.1, 4.2, 5.1 (Kern) | G-4 | **G-5:** H-S2, H-S7 (bei > 2× Abweichung greift §20); H-S8, H-S13; loom 1–2 auf echtem Code; Modelltest Stufe 1 |
| **P6** | Flush, Blob-Flush, 4.4–4.6 | G-5 | **G-6:** H-S3, H-S10, H-S17; Modelltest vollständig; INV-CLAIM-1 mit loom |
| **P7** | 6.1, 6.2 | G-6 | **G-7:** ErasureOracle 0 Treffer; Absturzmatrix E1–E7 |
| **P8** | 7.1–7.3 | G-7 | **G-8:** H-S14, H-S18; INV-KEYP-1; INV-EPOCH-1; Goldens in `Authenticated` und `Encrypted` |
| **P9** | 4.7, 8.1, Rest von 5.1 | G-8 | **G-9:** H-S16; INV-HOLD-1; `verify`-Befunde |
| **P10** | 5.2, 5.3, 8.2 | G-9 | **G-10:** INV-SEG-1; Segment-Goldens |
| **G-A** | Adapter-Anforderungen AR-1…AR-13 (§9.3) entschieden; O-10, O-11 | parallel ab G-5 | CR-2, CR-3 (§13) verabschiedet |
| **G-F** | Format-Freeze nach §9.8 (Punkte 1–14) | P3–P8 abgeschlossen, G-A | Goldens aller Dateiarten in drei Modi; zwei Reviewer; ab hier ist jede Formatänderung ein Migrationswerkzeug |
| **G-P** | Leistungsbericht H-S1…H-S18 | P9 | Bericht; Abbruchkriterien §20 ausgewertet |
| **M1–M5** | §6, WP-M | G-F, G-P | Ablösekriterien (§9.4) erfüllt |

**Kritischer Pfad:** P0 → P1 → P2 → P3 → P4 → P5 → P6 → G-F → M.1 → M.3. P5 trägt das höchste Risiko (Zusammenspiel von Redo, Committer und Recovery). P1 (Umgebung und Codec) und P2 (Schlüssel) laufen parallel. WP-9 läuft ab P1 mit.

**Abhängigkeitskanten (verbindlich):**
- 3.1, 3.2 benötigen 1.1, 1.2, 2.1.
- 3.5 benötigt 1.1, 1.2, 2.1, 3.4 (Segmentzustand über Catalog-Edits).
- 4.1 benötigt 3.1, 3.2.
- 4.2 benötigt 3.5, 4.1.
- 4.3 benötigt 3.4, 3.5, 4.1, 4.2.
- 6.x benötigt 3.3, 4.3, 4.4.
- 7.x ist Querschnitt: Formatfelder für Modi werden in P2/P3 als Reserve angelegt, damit G-F nicht an Modusfeldern scheitert.

**Zeitschätzung:** nicht angegeben. Es liegt keine Messung der Umsetzungsgeschwindigkeit vor. Steuerung erfolgt über Gates.

---

## 8. Verhaltensoracle und Testmigration

### 8.1 Oracle-Ebenen

| Ebene | Oracle | Gültig für | Ungültig für |
|---|---|---|---|
| O1 API-Verhalten | Altcode aus `014319d` auf Baseline; Referenzmodell | Versionsreihenfolge, Tombstones, Scan-Ordnung, `put_if_absent`, Idempotenz | Alle Pfade, die SPEC R2 ändert (Tx, SSI, Stamp-Semantik) |
| O2 Fehlerklassen | Altcode für F-1, F-2, F-5 | Klassenzuordnung (Abbruch, Korruption, Speicher voll) | Fehlertypen selbst (SPEC §7.7 ändert die Enum-Struktur) |
| O3 Bytes | Python-Referenzcodierer (WP-0.5) | Goldens des Freeze (Anhang A der SPEC) | Altformate, die SPEC verwirft |
| O4 Crash-Zustände | Modell: Präfix der quittierten Changes (SPEC §19.1) | Absturzmatrix | – |

### 8.2 Testinventar nach Markern

Marker sind objektiv (Dateiinhalt, `Cargo.toml`). Eine Datei kann mehrere Marker tragen. Die Disposition ist verbindlich.

| Marker | Anzahl | Disposition |
|---|---:|---|
| Interner Modulpfad (`contextra_store::wal::…`) | 128 | Umschreiben auf öffentliche API, oder Unit-Test im Modul neu |
| `[[test]]` mit `test-hooks` | 15 (11 + 4 mit `fault-injection`) | Ersetzen durch `Hooks` bzw. FaultFs; Feature entfällt |
| `[[test]]` mit `legacy-wal-key` | 6 | **Verwerfen** (SD-04) |
| Legacy-Bezug im Dateinamen | 13 | Prüfen; in der Regel verwerfen |
| Byte-Identität `mz_*` | 5 | **Verwerfen** (Altformat) |
| `proptest!` | 14 Dateien | Gesetze übernehmen, Logik neu |
| `loom` | 4 Dateien | **Verwerfen** als Nachbildung; Ersatz in WP-9.4 |
| `#[tokio::test]` | 525 Funktionen | Szenarien, synchron neu formuliert |
| `#[test]` | 126 Funktionen | Übernahme nach Prüfung |
| `#[ignore]` | 12 | Grund dokumentieren; Ziel: 0 offene |
| `differential_redb` | 1 | Behalten als Oracle-Harness (SPEC §19.1, begrenzt) |

### 8.3 Szenario-Extraktion

1. Jede Testdatei erhält Marker und Disposition (WP-0.6).
2. Jedes Szenario wird einer INV-ID oder einem Alt-Kennung zugeordnet (Mapping aus WP-0.6).
3. Jedes Szenario läuft gegen den Altcode auf Baseline. Ergebnis: `konsistent`, `erwartet abweichend` (SPEC-Änderung, Verweis auf CR), oder `Befund` (Altfehler).
4. Die Szenarien werden gegen die neue API neu geschrieben, mit INV-Zitat im Testnamen.
5. Abweichungsregister: jede Abweichung ist ein Eintrag mit Entscheidung.

---

## 9. Konsumenten- und Port-Migration (Phase M)

### 9.1 Inventar

Siehe §2.8 für die Crate-Liste. Gesamt: 122 Dateien mit `StorageEngine`-Textbezug, 24 mit `Purger`, 7 mit `LayerScanner`, 5 mit `FeatureRing`.

### 9.2 Methodennutzung `StorageEngine`

Zählung: Textsuche auf `.name(` über die Konsumenten `db`, `engine`, `text`, `graph`, `testkit`. Die Werte sind **Obergrenzen**; generische Namen (`get`, `put`, `delete`, `commit`) enthalten Fremdaufrufe.

| Port-Methode | Aufrufe | Neu (SPEC) | Adapter nötig |
|---|---:|---|---|
| `get` | 795 | `View::get` | nein (Namenskollision) |
| `commit` | 491 | `Storage::apply(Change)` | ja: TxId zu Change |
| `put` / `delete` | 140 / 97 | `Change::put` / `Change::delete` | ja: Puffer |
| `scan_prefix` / `_bounded` | 85 / 15 | `View::scan` / `scan_page` | ja: Cursor, Token |
| `scan_prefix_at` / `get_at_seq` | 28 / 32 | `View` über `view_at(stamp)` | ja: seq zu Stamp (AR-1) |
| `last_seq_no` | 32 | `visible_stamp` | ja (AR-1) |
| `rollback` | 69 | Puffer verwerfen | ja |
| `rollback_to_tx` | 20 | keine (SD-09) | ja: Savepoints (AR-5) |
| `last_tx_id` | 29 | keine | ja: Zähler (AR-6) |
| `get_tracked` / `get_at_seq_tracked` / `scan_prefix_tracked` | 19 / 10 / 2 | `view_at` plus `unchanged_since` | ja: SSI (AR-2) |
| `supports_ssi_tracking` | 6 | Konstante `true` im Adapter | ja |
| `put_if_absent` | 10 | `Guard::absent` | ja (AR-8) |
| `pin_checkpoint` / `unpin_checkpoint` | 9 / 8 | `hold` / `hold_drop` | ja (AR-4) |
| `flush` | 18 | ungeklärt | Lücke (AR-3) |
| `stats` | 47 | `stats()` | nein; Felder prüfen (AR-11) |
| `metrics_sink` / `set_metrics_sink` | 14 / 3 | `Metrics` | nein; Namen prüfen (AR-13) |
| `delete_prefix` / `delete_many` / `put_batch` | 3 / 1 / 1 | `Change::delete_range`, `Change` | ja |
| `scan` / `scan_bounded` | 23 / 1 | `View::scan` | ja |

**Lücke:** Die Konsumenten nutzen 26 Methoden des Ports. 15 lassen sich direkt auf Kernprimitive abbilden. 10 brauchen Adapterzustand (Tx-Puffer, TxId, Savepoints, SSI-Lesemengen, Commit-Semantik). 1 ist ungeklärt (`flush`, AR-3).

### 9.3 Anforderungen an den Adapter (AR)

| AR | Anforderung | Abbildung | Entscheidung oder Lücke |
|---|---|---|---|
| AR-1 | `seq_no` ist Stamp | `last_seq_no = visible_stamp`; `get_at_seq(s) = view_at(s)` | Stamps sind über angenommene Changes lückenlos (INV-STAMP-1); der Log selbst kann Lücken haben (§9.4). Keine Änderung nötig, Klarstellung in CR-2. |
| AR-2 | SSI-Lesemenge ohne Entry-Stamp | Lesemenge = (Schlüssel, `view.stamp()`); Validierung über `unchanged_since(space, key, view.stamp())` | `Entry` exponiert keinen Stamp (SPEC §7.2). Kein Blocker: Der Snapshot-Stamp genügt für Optimistic Concurrency Control. CR-2. |
| AR-3 | `flush()` | nicht definiert; SPEC nennt `flush_async` (§7.8.6, SD-59) ohne Signatur | **Lücke, O-11** |
| AR-4 | `pin_checkpoint(seq)` | `hold(stamp, owner, ttl)`; Adapter wählt `hold_durable`, wenn Persistenz gefordert ist | Eigentümer-Namensraum des Adapters |
| AR-5 | `rollback_to_tx` | Adapterpuffer mit Marken | Kein Eingriff in den Kern |
| AR-6 | `last_tx_id` | Zähler in eigenem Space, per `Change` fortgeschrieben | Muss Neustart überleben; Kollision mit Stamp-Zähler vermeiden |
| AR-7 | `BoxFuture` | `submit` plus `Pending::on_ready`; Executor im Adapter | Executor-Wahl liegt beim Konsumenten (NZ-20) |
| AR-8 | `put_if_absent` | `Guard::absent` im selben `Change` | Gleiche Semantik; Konformitätstest |
| AR-9 | `scan_prefix` als Iterator | Cursor, intern `scan_page` mit Stamp-Bindung | `Expired` muss der Adapter kapseln |
| AR-10 | `delete_prefix` | `Change::delete_range` | Löschsemantik ≠ Erase (keine physische Löschung) |
| AR-11 | `StorageStats` | `stats()` | Feldliste der Konsumenten ungemessen |
| AR-12 | Fehlerabbildung | `Conflict` wird `TxConflict` (nicht wiederholbar) | Port-Fehlertyp festlegen |
| AR-13 | `metrics_sink` | `Metrics::counter(&'static str, …)` | Altcode nimmt ggf. dynamische Namen; prüfen |

### 9.4 Ablösekriterien

Der Adapter wird abgebaut, wenn alle Bedingungen erfüllt sind:
1. Kein Konsument importiert mehr `StorageEngine`.
2. `legacy/` ist nicht mehr im Build-Graphen.
3. Die Konformitätssuite (aus den Tx-, SSI- und Rollback-Szenarien) besteht zu mindestens 95 % mit Oracle-Abgleich.
4. Die Konsumenten-Benchmarks liegen innerhalb ±20 % des Altstands auf derselben Hardware.

---

## 10. Format-Freeze (G-F)

- Freeze-Liste SPEC §9.8 (Punkte 1–14) wird unverändert übernommen. Neue Felder aus Rev. 2 (`0x0012…0x0014`, Segment-Rahmen, Epoche) sind Punkt 14.
- Goldens in allen drei Modi (`Plain`, `Authenticated`, `Encrypted`) für jede Dateiart. Referenz: Python-Codierer (WP-0.5) und zweite Rust-Implementierung der Goldens.
- Altformate (`tests/golden/`, `tests/fixtures/`) gehen nicht in den Kern. Sie werden Eingabe eines Importers, falls A-01 (keine produktiven Altdaten) sich als falsch erweist.
- `tenant_codec_vectors.json` (Altbestand) ist Referenz für die Mandanten-Codierung in der oberen Schicht (NZ-18).

---

## 11. Risiken

| ID | Risiko | Auslöser (messbar) | Gegenmaßnahme |
|---|---|---|---|
| R-1 | Monorepo bricht durch Umbau | `cargo check --workspace` rot nach P0 | Strangler: Altcode bleibt bis M.5 unter `legacy/`; Konsumenten zeigen auf die letzte lauffähige Version |
| R-2 | Adapter-Komplexität (SSI, Savepoints) unterschätzt | Konformitätssuite < 95 % nach M.1 | Fallback: Konsumenten-Umstellung je Crate ohne Adapter |
| R-3 | `BTreeMap`-Memtable zu langsam | H-S7 verfehlt um > 2× | §20: Skip-Liste als isolierte, gesondert geprüfte Einheit |
| R-4 | `make_durable` blockiert den Schreibpfad | p99 `apply` über Schwelle in H-S2 | Aufruf vor dem Ticket (SPEC §10.2 Schritt 1b); Domänenschlüssel-Cache |
| R-5 | Redo-Recovery zu komplex für eine Einheit | WP-3.5 überschreitet Phase um > 1 Gate | Aufteilung in Schreib-, Lese- und Recovery-Teil; Absturzenumerator zuerst |
| R-6 | Formatfehler vor Freeze | Golden-Abweichung zwischen Implementierungen | G-F erst nach zwei Implementierungen |
| R-7 | Committer-Lock über Cache-Fehlgriffe | Guard-Vorwärmung ohne Wirkung in H-S2 | Vorwärmung erweitern; Messung vor Umbau |
| R-8 | Plattformverhalten (`F_FULLFSYNC`, Verzeichnis-Sync) | `FsCaps`-Test auf macOS schlägt fehl | A-05 in P1 testen; `Strict` verweigert |
| R-9 | Produktive Altdaten existieren | Befund außerhalb des Repositories | Importer wird Pflicht (F-I) |
| R-10 | Konsumententests nicht baubar (Manifest-Befund `contextra-engine`) | `cargo test -p contextra-engine` bricht in Compile | Vor M.3 klären; kein Gating auf dieser Basis |
| R-11 | Entropiequelle nur Unix | Build auf Nicht-Unix mit `Entropy` | Fail-closed (O-13) |
| R-12 | Verlust der Scanner-Funktionen | Tests von `contextra-vector`, `-text`, `-graph` rot | Erasure-Brücke vor Abbau von `compat/` (M.2) |
| R-13 | Dokumentdrift zwischen SPEC und IMPL | Zahl in SPEC oder IMPL weicht vom Messskript ab | Zahlen aus Anhang A in CI vergleichen; Anträge nach §1.5 |

---

## 12. Offene Entscheidungen

| ID | Frage | Empfehlung | Entscheidung bis |
|---|---|---|---|
| O-1 | Toolchain und MSRV | Stabile Version mit `File::lock`/`try_lock`, gepinnt in WP-0.2 | G-0 |
| O-2 | Abgeleitete Daten nach `Erase` (Embeddings, Postings) | Segmente je Löschdomäne; `rewrite_without` Pflicht für löschfähige Formate | F10 |
| O-3 | `BodyCache` | Nur wenn H-S15 es belegt | F5 |
| O-4 | Windows-`Fs` | Eigenes Crate nach V1 (NZ-6) | nach P9 |
| O-5 | Schwelle `blob_min_bytes` | 16 MiB, in P6 messen | P6 |
| O-6 | Referenz-`Cipher`/`Mac` | Dev-Crate `contextra-crypto-ref`; Kandidat `contextra-crypto` aus `014319d`, Lizenz und Abhängigkeiten prüfen | G-F |
| O-7 | Blockkompression (SD-13) | Frühestens V2 nach Messung | nach P9 |
| **O-8** | Name des Port-Crates (BL-2) | `contextra-storage-ports` (CR-1) | G-0 |
| **O-9** | Konsumenten im Repository? (BL-4) | Nein, bis Phase M; M läuft in eigenem Zweig oder Repository | G-0 |
| **O-10** | Strategie für `StorageEngine` (BL-3) | Adapter `contextra-storage-tx` mit Austrittskriterien (§9.4); Alternative native Umstellung aller 122 Dateien verworfen | G-A |
| **O-11** | Semantik von `flush()` (AR-3) | `flush()` = Barriere plus Run-Bau (INV-FLUSH-1); `sync()` = Durabilität; `flush_async` streichen oder definieren | G-A |
| **O-12** | Stamp-Exposition (AR-2) | Kein Entry-Stamp; Lesemenge über `view.stamp()` | G-A |
| **O-13** | Entropiequelle | `/dev/urandom` (Unix); fail-closed auf anderen Plattformen (konsistent zu NZ-6) | vor P2 |
| **O-14** | Lizenzentkopplung (BL-6) | Kern ohne Lizenzbezug; Prüfung bei Konsumenten | vor M.4 |
| **O-15** | Lock-Rangprüfung | Nur im Debug-Build (`debug_assertions`, SPEC §16.1); Bestätigung | G-0 |

---

## 13. Änderungsanträge an `SPEC.md`

Nach SPEC §1.5 sind Änderungen an den Abschnitten 6–9, 13, 14 und 18 mit zwei Reviewern und Referenzvektor zu versehen. Die übrigen Anträge sind Klarstellungen.

| CR | Abschnitt | Änderung | Grund |
|---|---|---|---|
| CR-1 | §3.1, §4 | Port-Crate heißt `contextra-storage-ports` (O-8) | BL-2 |
| CR-2 | §7.2, §7.8.1 | Lesemenge für SSI ist (Schlüssel, `view.stamp()`); Muster festschreiben. Stamp-Klarstellung nach AR-1. | AR-1, AR-2 |
| CR-3 | §7.1, §7.8.6, SD-59 | `flush()` definieren oder streichen; `flush_async` und `sync` vereinheitlichen (O-11) | AR-3 |
| CR-4 | §16.7, §19.1 | loom-Modelle müssen den echten `sync.rs`-Pfad verwenden; Nachbildungen sind unzulässig | §2.5 |
| CR-5 | §20 | Referenzwerte sind Altstand-Messungen auf Referenzhardware (BL-7) | BL-7 |
| CR-6 | Anhang B.1 (B-3, B-4, B-10) | B-3: 86 statt 91 (Definition Anhang A). B-4: 10 Ränge statt acht. B-10: 7 Schalter, davon 3 ungegatet im Append-Pfad. | §3, Zeilen 4, 9, 15 |
| CR-7 | §3.2, §4 | `Entropy`-Standard nur `std` (`/dev/urandom`, fail-closed sonst) (O-13) | R-11 |
| CR-8 | §2.2, Anhang B | Migrationsschicht (Adapter, Erasure-Brücke) als Schicht außerhalb des Kerns benennen | BL-3, BL-5 |
| CR-9 | §20 | Zählung H-S1…H-S18 ist korrekt; Rev. 1 nannte H-S19 | §3, Zeile 19 |

---

## 14. Definition of Done

### 14.1 Globale Metriken

Alle Metriken sind mit dem Messskript (Anhang A) auszugeben und in CI zu vergleichen.

| ID | Metrik | Altstand (Ist) | Soll | Werkzeug |
|---|---|---:|---:|---|
| M-01 | Produktionsdateien über 400 Zeilen | 11 | 0 | Skript |
| M-02 | Integer-`as`-Casts in Produktion | 133 | 0 | Skript, `clippy` |
| M-03 | direkte FS-Aufrufe außerhalb `io_seam` (Ist) bzw. `env/real_fs.rs` (Soll) | 86 | 0 | Skript |
| M-04 | statische Items in Produktion | 12 | 0 | Skript |
| M-05 | `cfg(feature)` in Logik außer `testing` | 43 | 0 | Skript |
| M-06 | Modulzyklen | 3 | 0 | `tests/layering.rs` |
| M-07 | Schichtverletzungen gegen §3.3 (klar) | 11 (+4 Zuordnung) | 0 | `tests/layering.rs` |
| M-08 | `unwrap`, `expect`, `panic` in Produktion | 0 | 0 | Skript, `clippy` |
| M-09 | INV-IDs mit Testzitat (neue Kennungen) | 0 von 43 | 43 von 43 | `tests/invariants.rs` |
| M-10 | Laufzeitabhängigkeiten des Kerns | 22 | 1 (`contextra-storage-ports`) | `cargo tree -e normal` |
| M-11 | H-S-Messwerte berichtet | 0 von 18 | 18 von 18 | G-P-Bericht |
| M-12 | Integrationstests mit internem Modulpfad | 128 von 160 | 0 | Skript |

### 14.2 Je Arbeitspaket

- Jede Datei ≤ 400 Zeilen; Funktionen typisch ≤ 60 Zeilen (P-6).
- Jeder Persistenzschritt hat eine Zeile in der Absturzmatrix (§13.2 oder §14.3).
- Jede Invariante, die das WP berührt, wird im Test per ID zitiert.
- Formatänderungen: Goldens aktualisiert, zwei Reviewer, Freeze-Liste geprüft (§1.5).
- `clippy -D warnings`, `cargo doc` ohne Warnung, `cargo deny` grün.
- `miri` für `codec` und `keys`.
- Allokationsbudget (P-11) für `apply` und `get` mit zählendem Allokator geprüft, sofern das WP den Heißpfad berührt.

---

## Anhang A: Messdefinitionen und Messskript

### A.1 Definitionen

| Messgröße | Definition | Ausschluss |
|---|---|---|
| Produktionszeilen | Zeilen von `src/**/*.rs` vor dem ersten `#[cfg(test)] mod … {` | Testdateien (Pfadmuster `tests?` oder `*_tests?.rs`) |
| Testcode in `src/` | Zeilen von Testdateien und Testmodulen | – |
| `unwrap`, `expect`, `panic` | `.unwrap()`, `.expect(`, `panic!(`, `todo!`, `unimplemented!`, `unreachable!` | Testcode |
| Integer-`as`-Casts | `\sas\s+(u\|i)(8\|16\|32\|64\|128\|size)` | Float-Casts |
| direkte FS-Aufrufe | `std::fs::`, `tokio::fs::`, `File::open(`, `File::create(`, `OpenOptions::new(` | `io_seam` |
| statische Items | `static`-Deklarationen auf Modulebene | `const` |
| `cfg(feature)` | `cfg(feature = "…")` in Produktionscode | – |
| Modulkanten | `use crate::<modul>` über Modulgrenzen; Modul = erstes Pfadsegment unter `src/` | Selbstkanten |
| Zyklen | Modulpaare mit Kanten in beiden Richtungen | – |
| Testfunktionen | `#[test]`, `#[tokio::test`, `#[ignore` in `src/` und `tests/` | Benches, Examples, Fuzz |
| Interne Pfadkopplung | Integrationstests mit `contextra_store::(wal\|lsm\|…)::` | – |

**Grenzen der Messung:** Alle Zählungen sind regex-basiert und daher Obergrenzen. Kommentare und Zeichenketten können Treffer erzeugen. WP-0.6 prüft Stichproben. Die Schichtverletzungen in §2.3 sind eine Handauswertung nach der Modulzuordnung, keine Skriptausgabe.

### A.2 Ausgabe bei HEAD `8a24bf4`

```
LOC  src-Produktion=20369  src-Testcode=11646  tests/=25563
Produktionsdateien=129  >400 Zeilen=11  max=(966, 'src/compaction/engine.rs')
unwrap/expect/panic/todo/unreachable: 0
int-as-Casts (P-14): 133
direkte FS-Aufrufe ausserhalb io_seam (P-1): 86
statische Items (P-5): 12
async fn (SD-02): 201
tokio-Referenzen (SD-01/02): 203
cfg(feature) in Produktion: {'workspace': 11, 'legacy-wal-key': 7, 'fault-injection': 17, 'memory-only-storage': 1, 'block-cache-v2': 6, 'docid-128': 1} Summe 43
Modulkanten=42 Vorkommen=142 Zyklen=3 [('compat', 'lsm'), ('compat', 'tenant_codec'), ('lsm', 'util')]
Top-Kanten: [(('lsm', 'error'), 19), (('sstable', 'error'), 12), (('lsm', 'wal'), 11), (('wal', 'error'), 10), (('compat', 'lsm'), 8), (('manifest', 'error'), 6), (('sstable', 'io_seam'), 6), (('compaction', 'error'), 5)]
Tests: #[test]=126 #[tokio::test]=525 #[ignore]=12 proptest!-Dateien=14
Testdateien tests/=160 +src/=32 interne Modulpfade (tests/)=128  [[test]]-Ziele=22 {'test-hooks': 11, 'fault-injection,test-hooks': 4, '(keine)': 1, 'legacy-wal-key': 6}
```

Ausgabe und Quellstand sind im Lauf vom 22.11.2026 gegen `crates/contextra-store` geprüft.

### A.3 Messskript

Voraussetzung: Python ≥ 3.11 (`tomllib`). Aufruf: `python3 measure_baseline.py crates/contextra-store`.

```python
#!/usr/bin/env python3
"""Baseline-Messung für crates/contextra-store (statisch, ohne Compiler).
Aufruf: python3 measure_baseline.py <pfad/zu/crates/contextra-store>
Messdefinitionen: Anhang A des IMPLEMENTATION_SPEC.md (Revision 2)."""
import os, re, sys, collections, tomllib

ROOT = sys.argv[1] if len(sys.argv) > 1 else "."
SRC, TESTS = os.path.join(ROOT, "src"), os.path.join(ROOT, "tests")
TEST_PATH = re.compile(r"(^|/)tests?(\.rs|/)|_tests?\.rs$")
INLINE_TEST = re.compile(r"#\[cfg\(test\)\]\s*\n\s*mod\s+\w+\s*\{")
FS_CALL = re.compile(r"std::fs::|tokio::fs::|\bFile::(open|create)\(|OpenOptions::new\(")
INT_CAST = re.compile(r"\sas\s+(u8|u16|u32|u64|u128|usize|i8|i16|i32|i64|i128|isize)\b")
STATIC = re.compile(r"^\s*(pub(\([a-z]+\))? )?static\s+(mut\s+)?[A-Z_0-9]+\s*:", re.M)

def rel(p): return os.path.relpath(p, ROOT).replace(os.sep, "/")

def prod_text(path):
    s = open(path, encoding="utf-8").read()
    if TEST_PATH.search(rel(path)): return ""
    m = INLINE_TEST.search(s)
    return s[:m.start()] if m else s

files = [os.path.join(d, n) for d, _, ns in os.walk(SRC) for n in ns if n.endswith(".rs")]
prod = {f: prod_text(f) for f in files}
prod_files = [f for f in files if not TEST_PATH.search(rel(f))]

# 1 Umfang
loc_prod = sum(len(prod[f].splitlines()) for f in files)
loc_test_src = sum(len(open(f, encoding="utf-8").read().splitlines()) for f in files) - loc_prod
loc_tests = sum(len(open(os.path.join(d, n), encoding="utf-8").read().splitlines())
                for d, _, ns in os.walk(TESTS) for n in ns if n.endswith(".rs"))
big = sorted(((len(prod[f].splitlines()), rel(f)) for f in prod_files), reverse=True)
big = [x for x in big if x[0] > 400]

# 2 Regeln P-13/P-14/P-1/P-5/P-5b
allp = "\n".join(prod.values())
viol = {
    "unwrap/expect/panic/todo/unreachable": len(re.findall(r"\.unwrap\(\)|\.expect\(|\bpanic!\(|\b(todo|unimplemented|unreachable)!\(", allp)),
    "int-as-Casts (P-14)": len(INT_CAST.findall(allp)),
    "direkte FS-Aufrufe ausserhalb io_seam (P-1)": sum(len(FS_CALL.findall(prod[f])) for f in prod_files if "io_seam" not in f),
    "statische Items (P-5)": len(STATIC.findall(allp)),
    "async fn (SD-02)": len(re.findall(r"\basync fn\b", allp)),
    "tokio-Referenzen (SD-01/02)": len(re.findall(r"\btokio::", allp)),
}
cfgs = collections.Counter(f for f in re.findall(r'cfg\(feature\s*=\s*"([a-z0-9-]+)"\)', allp))

# 3 Kopplung (Modul = erstes Pfadsegment unter src/)
def module(f):
    parts = rel(f).split("/")
    return parts[1].removesuffix(".rs")
edges = collections.Counter()
for f in prod_files:
    src_mod = module(f)
    for m in re.finditer(r"\buse\s+crate::([a-z_]+)", prod[f]):
        if m.group(1) != src_mod: edges[(src_mod, m.group(1))] += 1
cycles = sorted({tuple(sorted(e)) for e in edges if (e[1], e[0]) in edges})

# 4 Tests
all_rs = [os.path.join(d, n) for base in (TESTS, SRC) for d, _, ns in os.walk(base) for n in ns if n.endswith(".rs")]
tt = "\n".join(open(f, encoding="utf-8").read() for f in all_rs)
tf = [f for f in all_rs if TEST_PATH.search(rel(f))]          # Testdateien (tests/ + Testmodul-Dateien in src/)
tests_only = [f for f in tf if rel(f).startswith("tests/")]
internal = sum(1 for f in tests_only if re.search(r"contextra_store::(wal|lsm|sstable|compaction|manifest|memtable|engine|scanner|compat|tenant_codec)::", open(f, encoding="utf-8").read()))
cargo = tomllib.load(open(os.path.join(ROOT, "Cargo.toml"), "rb"))
req = collections.Counter(",".join(sorted(t.get("required-features", []))) or "(keine)" for t in cargo.get("test", []))

print(f"LOC  src-Produktion={loc_prod}  src-Testcode={loc_test_src}  tests/={loc_tests}")
print(f"Produktionsdateien={len(prod_files)}  >400 Zeilen={len(big)}  max={big[0] if big else '-'}")
for k, v in viol.items(): print(f"{k}: {v}")
print("cfg(feature) in Produktion:", dict(cfgs), "Summe", sum(cfgs.values()))
print(f"Modulkanten={len(edges)} Vorkommen={sum(edges.values())} Zyklen={len(cycles)} {cycles}")
print("Top-Kanten:", edges.most_common(8))
print(f"Tests: #[test]={tt.count('#[test]')} #[tokio::test]={tt.count('#[tokio::test')} #[ignore]={tt.count('#[ignore')} proptest!-Dateien={sum(1 for f in all_rs if 'proptest!' in open(f, encoding='utf-8').read())}")
print(f"Testdateien tests/={len(tests_only)} +src/={len(tf)-len(tests_only)} interne Modulpfade (tests/)={internal}  [[test]]-Ziele={len(cargo.get('test', []))} {dict(req)}")
```

---

*Ende Revision 2.*
