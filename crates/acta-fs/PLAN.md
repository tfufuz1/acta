# Implementierungsplan: `acta-fs`

> **Status:** verbindlich für die Crate-Überführung · **Paket:** 1 (L1)
> **Vorgaben:** [docs/spec/03_systemarchitektur.md](../../docs/spec/03_systemarchitektur.md) (3.7) · [docs/spec/05_speicher.md](../../docs/spec/05_speicher.md) (5.6 WP-5 bis WP-7) · [docs/spec/13_sicherheit_lieferkette.md](../../docs/spec/13_sicherheit_lieferkette.md) (13.1) · [docs/spec/anhang_c_durabilitaet.md](../../docs/spec/anhang_c_durabilitaet.md) · [docs/spec/16_test_freigabe.md](../../docs/spec/16_test_freigabe.md) (16.4) · [CLAUDE.md](CLAUDE.md) · [archive/acta-fs/MANIFEST.md](../../archive/acta-fs/MANIFEST.md)

## 0. Verifikationsstand dieser Planung
- Ist-Stand laut `CLAUDE.md` des Crates und `pub`-Oberfläche. **Build und Tests wurden in dieser Planung nicht ausgeführt** (Toolchain nicht verfügbar).
- Der Quelltext des Archivs (`durable-fs`, `store`, `ports`, `testkit`) wurde über das Manifest bewertet, nicht Zeile für Zeile gelesen. Vor AP 3 ist jede übernommene Datei zu lesen.

## 1. Einordnung
- **Level:** 1. **Abhängigkeiten:** `acta-base`. Keine Querkanten.
- **Budget:** 2 500 LOC (G-HUB). Ist laut CLAUDE.md ca. 510 LOC.
- **Externe Abhängigkeiten:** `thiserror`. Dev: `proptest`.
- **Einzige Stelle mit `std::fs`** (G-FS). Alle anderen Crates erreichen Dateien nur über den `Vfs`-Port.

## 2. Ist-Stand (laut CLAUDE.md)
| Modul | Stand |
|---|---|
| `vfs` | Ports `Vfs` und `VFile` mit den Signaturen aus 3.7 |
| `error` | `FsyncFailed` (Klasse `Integrity`, Code `fs.fsync_failed`), `FsError` (`Io`, `Poisoned`) |
| `durable` | `DurableVfs`: atomares Ersetzen (rename + Verzeichnis-fsync), Scrub, `O_DIRECT`-Rücklesen (4096-Byte-Blöcke, ausgerichteter Puffer), Fsync-Entwertung |
| `fault` | `FaultVfs<V>` mit `FaultPlan` (`FaultPoint`, `FaultAction`: Fehlerbild, zerrissener Schreibvorgang, Absturz), `arm`, `arm_also` |
| `atomic` | `write_atomic`: Ersatzdatei, `sync_data`, `rename_atomic`; nach `FsyncFailed` Ersatz in **neuer** Datei |
| Tests | `conformance` (dieselbe Suite gegen `DurableVfs` und `FaultVfs`), `crash_matrix` (6 Fälle inkl. Positivkontrolle), `durable_vfs` (8), `fault_vfs` (5) |

## 3. Spec-Bezug
- **WP-5:** Atomar ersetzen = neue Datei, `fsync`, `rename`, `fsync` des Verzeichnisses.
- **WP-6:** Scrub = Überschreiben mit Nullen, `fsync`, `unlink`. Ein Löschnachweis entsteht daraus **nicht** (A-FLASH, Restrisiko im Bericht).
- **WP-7 / Fsync-Regel (3.7):** Nach `FsyncFailed` wird dieselbe Datei nie erneut synchronisiert und als stabil angenommen. Der Inhalt wird in eine neue Datei geschrieben. Scheitert auch das, wird der Bereich vergiftet.
- **13.1:** Zulässig sind `ext4` und `xfs`. Copy-on-Write-Dateisysteme und Snapshots verweigern den Start, außer sie sind deklariert. **Dafür fehlt in `acta-fs` noch eine Dateisystem-Erkennung** (siehe AP 4).
- **Z4 und Anhang C:** Der Port liefert die Primitive. Die Crash-Matrix C01 bis C27 gehört zu den Verbrauchern (`acta-seg`, `acta-vault`, `acta-erasure`). `acta-fs` testet die Primitive selbst (I7).

## 4. Archiv-Entscheidungen

| Archivquelle | Entscheidung | Begründung |
|---|---|---|
| `durable-fs/src/vfs.rs`, `ports/src/vfs.rs` | Bereits umgesetzt; nur Abgleich | 3.7 ist maßgeblich |
| `durable-fs/src/lib.rs` (455 Zeilen, `std::fs`, `parking_lot`) | Teile in `durable` übertragen, Rest verwerfen | `parking_lot` ist nicht auf der Whitelist. Sperren sind `std::sync`. |
| `durable-fs/src/scrub.rs` (`float` in 2 Stellen) | Prüfen, Scrub-Logik übernehmen | Float darf nicht in Prüfpfade (Z7). Scrub nutzt keine Gleitkomma-Werte. |
| `durable-fs/src/fault_injector.rs` (300 Zeilen) | In `fault` aufgehen lassen | Doppelte Fehlerinjektion vermeiden. Ein Injektor. |
| `testkit/src/fault_vfs.rs` (457 Zeilen) | Gegen `fault` abgleichen | Bereits teilweise umgesetzt |
| `testkit/src/conformance/vfs.rs` | Übernommen (`conformance`) | Orakel-Suite |
| `durable-fs/src/journal.rs` (436 Zeilen) | **Nach `acta-seg`** | Journal ist Teil des Segmentspeichers (5.1), nicht des Dateisystem-Ports |
| `durable-fs/src/health.rs` (139 Zeilen) | **Nicht hier.** Zustandsmaschine nach `acta-node` (14.6) | Gesundheitszustand ist Betriebslogik, nicht Dateisystem |
| `durable-fs/src/platform.rs` | Nur Konstanten prüfen (`O_DIRECT`), Rest verwerfen | Plattformteil ist in `acta-fs` bereits über `DIRECT_ALIGN` abgedeckt |
| `store/src/io_seam.rs`, `store/src/io_seam/store_io.rs`, `store/src/io_seam/real_fs.rs` | **Nicht** übernehmen | `async` ist im Kern verboten (G-NOTOKIO). Der Port ist synchron. |
| `store/src/fsync.rs` (`async` in 8 Stellen) | Nur Regel WP-7 übernehmen | Synchrone Neufassung über `FsyncFailed` |
| `durable-fs/tests/crash_matrix_exhaustive.rs`, `inv_crash_matrix.rs` | Als Matrix-Orakel übernehmen (Vfs-Ebene) | Schnitt: Fälle, die `Vfs`-Primitive betreffen. Fälle des Journals gehen nach `acta-seg`. |
| `durable-fs/tests/scrub_refusal_matrix.rs`, `scrub_and_remove.rs` | Übernehmen | Scrub-Verweigerung ist Vorbedingung von 13.1 |
| `durable-fs/tests/journal_*`, `scrub_journal_tests.rs`, `journal_crash_resume.rs` | **Nach `acta-seg`** | Gehören zum Journal |
| `durable-fs/tests/p03_health_state_tests.rs` | **Nach `acta-node`** | Zustandsmaschine |
| `durability_tests.rs`, `basic.rs` | Prüfen, Doppelungen streichen | |
| `durable-fs/benches/fs_bench.rs` | Übernehmen (Messung Z-Werte, 15.3 `Segmentersatz`) | Kein Test |

**Verworfene Abhängigkeiten:** `crc32fast` (Prüfsummen laufen über `crc32c`, Whitelist), `parking_lot` (`std::sync`).

## 5. Offene Punkte und Spec-Lücken

1. **Dateisystem-Erkennung (13.1, Selbsttest 14.1).** Der Selbsttest muss den Dateisystemtyp und die `O_DIRECT`-Unterstützung prüfen. Das geht nur über `statfs`. `statfs` gehört in `acta-fs` (einzige Stelle für Dateizugriffe). **Fehlt im Ist-Stand.** Als Port `FsKind` ergänzen (AP 4).
2. **aarch64 `O_DIRECT`.** Der Wert `0o40000` gilt für x86_64 und aarch64. Auf aarch64 ist das **nicht auf Hardware geprüft** [A]. Im Bericht ausweisen.
3. **Absturz zwischen `rename` und Verzeichnis-`fsync`.** Abgedeckt nur durch den Plan `Crash` vor dem Rename, nicht innerhalb des Aufrufs [A]. Entweder Testlücke schließen (Fault-Punkt zwischen den beiden Schritten) oder als Annahme beibehalten.
4. **Scrub vs. Flash (A-FLASH).** Scrub ist kein Löschnachweis auf SSD und COW. Der Crate misst das nicht und behauptet es nicht. Das Restrisiko steht im Bericht (20.3).
5. **Test-Ausnahme zu G-FS.** Einheitstests in `acta-sys` lesen `/proc/self/status` mit `std::fs`. G-FS erlaubt das nur in `acta-fs`. Lösung: Lesen über einen Port aus `acta-fs` (AP 4) **oder** eine ADR-Ausnahme nur für `tests/`. Entscheidung vor Abnahme von `acta-sys`.

## 6. Arbeitspakete

**AP 1 – Konformitätstests gegen 3.7 festziehen.** Jede Signatur von `Vfs` und `VFile` gegen 3.7 prüfen, insbesondere `list`, `remove_scrub`, `rename_atomic`, `sync_data -> Result<(), FsyncFailed>`.
- *Abnahme:* Konformitäts-Suite läuft gegen `DurableVfs` und `FaultVfs`. Abweichungen sind dokumentiert.

**AP 2 – Crash-Matrix für Primitive vervollständigen.** Lücke aus Abschnitt 5 Punkt 3 schließen (Fault-Punkt zwischen Rename und Verzeichnis-`fsync`). Fälle aus `crash_matrix_exhaustive.rs` übernehmen, die `Vfs`-Primitive betreffen.
- *Abnahme:* Jeder Fault-Punkt hat einen Test mit Positivkontrolle (16.4). Negativaussage „Datei nicht stabil“ wird am Anwesenheitsnachweis gemessen.

**AP 3 – Scrub und Fsync-Regel übernehmen.** `scrub_refusal_matrix` und `scrub_and_remove` übernehmen. Float aus `scrub.rs` entfernen. WP-7 (keine Wiederholung nach `FsyncFailed`) als Test mit doppelter Fsync-Prüfung festhalten.
- *Abnahme:* `single_fsync_…` und `double_fsync_…` bestehen. Entfernen der Fsync-Entwertung lässt sie scheitern (bereits belegt, wiederholen).

**AP 4 – Dateisystem-Erkennung und `O_DIRECT`-Prüfung.** Port `FsKind` in `acta-fs` (über `statfs`, ohne neue Abhängigkeit, Whitelist prüfen: `rustix` hat das Feature `fs`, das **nicht** auf der Whitelist steht). Entscheidung nötig: `statfs` über `libc`-freie Alternative oder ADR. Ergebnis: `Ext4`, `Xfs`, `Cow { kind }`, `Other`.
- *Abnahme:* Test mit Doppel-Port liefert jeden Fall. Unbekannter Typ liefert `Other` und verweigert den Start (fail-closed, Z10).

**AP 5 – Journal, Zustand und Fehlerinjektor wegschneiden.** `journal.rs` und Tests nach `acta-seg` (Paket 2). `health.rs` und Test nach `acta-node` (Paket 8). `fault_injector.rs` in `fault` zusammenführen.
- *Abnahme:* Kein Doppel-Injektor. `grep` nach `FaultInjector` findet nur noch `fault`.

**AP 6 – Verbote und Gates.** `parking_lot`, `crc32fast`, `async` entfernen. Der Port bleibt synchron.
- *Abnahme:* G-NOTOKIO, G-FS, G-LINT bestehen (kein `unwrap`, `expect`, Indexierung in Produktionspfaden).

**AP 7 – Archivordner entfernen.** Erst nach AP 1 bis 6 und grünen Gates. `archive/acta-fs/` bleibt bis Paket 2 (`acta-seg`) bestehen, weil das Journal dort noch liegt. Entfernung erfolgt dann mit dem Journal-Abschluss.

## 7. Tests und Gates

| Test | Zusage / Gate |
|---|---|
| `conformance` (2 Suiten) | 3.7 |
| `crash_matrix` (6 Fälle) + Erweiterung AP 2 | Z4, 16.4 I7, WP-5/6/7 |
| `durable_vfs` (8), `fault_vfs` (5) | WP-5, WP-6 |
| Neu: `FsKind`-Tests (AP 4) | 13.1, Z10 |
| G-FS, G-NOTOKIO, G-UNSAFE, G-LINT, G-HUB (≤ 2 500 LOC) | 16.6 |

## 8. Befehle
```
cargo test -p acta-fs --locked
cargo clippy -p acta-fs --all-targets --locked -- -D warnings
cargo fmt --all -- --check
cargo xtask gates
```
