# IMPLEMENTATION_SPEC.md — Umsetzungsplan `contextra-storage`

**Revision 3 · ersetzt Revision 2 · Messstand 22.11.2026**

| | |
|---|---|
| **Status** | Operativ für Reihenfolge, Abnahme, Entscheidungen und Restumfang. Bei Widerspruch gilt `SPEC.md` (R2). Änderungen an `SPEC.md` sind in §10 als Anträge formuliert, nicht vorgenommen. |
| **Quellstand** | Repository `tfufuz1/acta`, HEAD `c09c5c0` („Shell-Commit“). Der Klon ist flach (`--depth 1`); die Commit-Historie ist nicht prüfbar. Rev. 2 bezog sich auf den Altstand `8a24bf4` mit 62 Commits. |
| **Verifikation** | Dynamisch und statisch. Der Workspace wurde mit `cargo check`, `cargo test`, `cargo clippy` und `cargo doc` geprüft. Toolchain: `rustc 1.97.0 (stable)`, weil der Pin `1.89.0` in `rust-toolchain.toml` in dieser Umgebung nicht ladbar ist (siehe BL-7). Die Leistungswerte aus `REPORT.md` wurden nicht reproduziert (siehe S-10). |
| **Gegenüber Rev. 2** | Der Altstand `contextra-store` ist als Plan-Gegenstand erledigt und entfernt (§2, Anhang B). Der Neubau existiert. Neu: Ist-Zustand des Neubaus (§1), Kritische Befunde S-1…S-20 (§4), aktualisierter Restumfang je Arbeitspaket (§5), neu bewertete Gates (§6). |

---

## 0. Kernbefunde

1. **Der Neubau kompiliert und die meisten Kern-Bausteine sind vorhanden.** `cargo check --workspace --all-targets` ist grün. Codec, Schlüsselhierarchie, Nonce-Protokoll, Catalog (Edit, Log, Reserve, Rollover), Run-Schreib- und Leseseite, Blob-Format, Ports, `MemFs`, `FaultFs` und `RealFs` existieren und sind größtenteils getestet.
2. **Die Bausteine sind nicht verdrahtet.** Es gibt keine öffentliche Storage-Facade (`api/` enthält nur den Listener). `engine/recovery.rs`, `engine/flush.rs`, `engine/compaction/*` und `engine/maintenance` sind nicht im Modulbaum deklariert und werden nicht übersetzt. `erase/` ist eine Datei mit 11 Zeilen Doku ohne Code.
3. **Die Durabilität ist nicht real.** Der Committer schreibt im Redo-Pfad nur einen 8-Byte-Stamp als Platzhalter-Frame (`committer.rs:946–951`). Quittierte Changes sind nach einem Absturz nicht wiederherstellbar. Das ist die kritischste Lücke, weil sie F-1 und Z-1 verletzt, obwohl der Redo-Codec selbst getestet ist.
4. **Die Testsuite ist überwiegend grün, aber nicht belastbar.** Vier Tests in `tests/redo` schlagen fehl. Drei davon sind Testfehler (falscher Offset, nicht zurückgesetzter Fehlerschalter). Einer deckt einen Implementierungsfehler auf: Ein Bitflip in einem gesiegelten Segment führt zu stillem Abschneiden statt zu `Corrupt` (INV-LOG-6). Die vier Loom-Tests sind unter `cfg(loom)` rot.
5. **Die Invariantenprüfung ist formal.** 14 der 43 INV-IDs aus SPEC §18 haben keinen Test. `tests/invariants.rs` prüft nur `≥ 35` und zählt auch Doc-Kommentare.
6. **Die Leistungsbelege in `REPORT.md` sind nicht reproduzierbar.** Es gibt keinen Benchmark-Code im Crate. SPEC §20 sagt ausdrücklich, dass nichts gemessen wurde. `REPORT.md` zurückzuziehen oder als Hypothese zu kennzeichnen ist Teil von S-10.
7. **Die Lint-Vorgaben sind verletzt.** Der Workspace verbietet `unwrap`, `expect` und `panic`, aber `cargo clippy -p contextra-storage --lib` meldet acht Deny-Fehler im Produktionscode. `sync.rs` hebt den Panic-Lint mit `#![allow(clippy::panic)]` auf.
8. **Der Altcode ist nicht mehr der Prüfgegenstand.** `legacy/contextra-store` (343 `.rs`-Dateien) liegt im Repo, ist aber ohne Workspace-Abhängigkeiten nicht baubar (BL-1). Der Verhaltensoracle-Plan aus Rev. 2 ist damit nicht ausführbar, bis BL-1 entschieden ist.

**Gesamturteil:** Die Formate und Algorithmen sind weitgehend vorhanden. Die Verdrahtung zu einem ausführbaren, durablen System fehlt. G-3 und alle späteren Gates sind nicht erreicht.

---

## 1. Ist-Zustand des Neubaus (gemessen)

### 1.1 Umfang

| Messgröße | Wert | Methode |
|---|---:|---|
| `contextra-storage/src`, `.rs`-Dateien | 93 | `find … -name '*.rs'` |
| `contextra-storage/src`, Zeilen gesamt | 17 420 | `wc -l` |
| Dateien über 400 Zeilen (P-6) | 1 (`engine/committer.rs`, 1 103) | `wc -l` |
| `contextra-storage/tests`, `.rs`-Dateien | 49 | `find` |
| `contextra-storage-ports`, `contextra-crypto-ref` | 1 Datei + 3 Dateien | `find` |
| `fuzz/fuzz_targets` | 3 (Soll ≥ 8, SPEC §19.1) | `ls` |
| `benches/` im Crate | 0 | `ls` |
| `.github/` (CI) | nicht vorhanden | `ls -A` |

### 1.2 Modulbaum (Deklaration in `mod.rs`)

| Modul | Deklariert und übersetzt | Inhalt |
|---|---|---|
| `api` | ja | nur `listener` (8 Zeilen); keine Facade |
| `codec` | ja | varint, crc32c, xxh64, bloom, tag, keyrange, bytes, limits |
| `keys` | ja | hierarchy, nonce, epoch, protocol, secret (8 Zeilen Re-Export) |
| `env` | ja | real_fs, clock, entropy, atomic, governor, metrics, runtime |
| `redo` | ja | frame, header, op, driver, replay, replay_sm, writer_sm, error |
| `run` | ja | builder, block, data, index, filter, footer, rangedel, meta, reader, table_cache, block_cache, extension |
| `blob` | ja | record, writer, reader, sweep, blob_ref |
| `catalog` | ja | edit, log, state, current, format, reserve, rollover, durable, codec, types |
| `engine` | teilweise | nur `committer`, `memtable`, `registry`, `retention`, `version`, `views` |
| `engine::recovery`, `engine::flush`, `engine::compaction::*` | **nein** (Dateien vorhanden, nicht deklariert) | werden nicht übersetzt |
| `erase` | ja, leer | `mod.rs` mit 11 Zeilen Doku |
| `testing` (Feature) | ja | mem_fs, fault_fs, hooks, conformance/fs_suite |
| `sync` | ja | Rangprüfung, Fassade über `std::sync` bzw. `loom` |

### 1.3 Tests und Lints (gemessen)

| Prüfung | Ergebnis |
|---|---|
| `cargo check --workspace --all-targets` | grün, 3 Warnungen in `tests/redo` (ungenutzte Imports und `mut`) |
| `cargo test --workspace --no-fail-fast` | alle Binaries grün außer `contextra-storage::redo` (20 von 24 bestanden) |
| `cargo test -p contextra-storage --test redo` | 4 Fehlschläge (siehe S-5, S-6) |
| `RUSTFLAGS="--cfg loom" cargo test --test loom` | 0 von 4 grün (S-11) |
| `cargo clippy -p contextra-storage --lib` | 8 Deny-Fehler (`unwrap`), 75 Warnungen |
| `cargo clippy --workspace --all-targets` | 24 Deny-Fehler an 21 Stellen (8 Produktion, 13 Test), 75 Warnungen |
| `cargo doc -p contextra-storage --no-deps` | 2 Warnungen (`run/index.rs:23`, `catalog/durable.rs:3`) |
| `unsafe` | 0 Treffer; `forbid(unsafe_code)` in beiden Crates |
| `cargo tree -e normal -p contextra-storage` | nur `contextra-storage-ports` (Z-4 erfüllt) |
| `cfg(feature)` in Logik | nur `testing` (P-5 erfüllt für Feature-Gates) |
| Statische Items | 2 (`env/atomic.rs:11` `static AtomicU64`; `sync.rs:26–27` `thread_local!`) |

### 1.4 Abdeckung der Invarianten

29 verschiedene INV-IDs werden in `tests/` oder `src/testing/` zitiert. SPEC §18 enthält 43. Ohne Testzitat sind: `INV-ERA-1` bis `INV-ERA-7`, `INV-SCAN-1`, `INV-STAMP-1`, `INV-STAMP-2`, `INV-POISON-1`, `INV-KEYP-1`, `INV-HOLD-1`, `INV-SEG-1`.

---

## 2. Abgeschlossen und aus dem Plan entfernt

Die folgenden Punkte sind im Neubau umgesetzt und nachgewiesen. Sie erscheinen in den Arbeitspaketen nur noch, wenn ein Restpunkt verbleibt.

| Eintrag | Nachweis |
|---|---|
| **Workspace-Build** (WP-0.1, neu definiert) | `cargo check --workspace --all-targets` grün; Mitglieder `contextra-storage-ports`, `contextra-storage`, `contextra-crypto-ref` wie SPEC §3.1 |
| **Port-Crate** (BL-2 / O-8 entschieden) | `crates/contextra-storage-ports` mit `Fs`, `File`, `FsCaps`, `Clock`, `Entropy`, `Checksum`, `Cipher`, `Mac`, `Kdf`, `KeyProvider`, `SecretAlloc`, `RollbackAnchor`, `ColdTier`, `Secret32`; Signaturen stimmen mit SPEC §4 überein (Abweichungen in WP-0.3) |
| **Referenzcodierer** (WP-0.5, Goldens A.1–A.6) | `tools/reference/generator.py` mit 23 Golden-Dateien; `crates/contextra-crypto-ref` mit 4 Golden-Tests; `tests/golden_anhang_a.rs` mit 7 grünen Tests |
| **Codec-Primitive** (WP-1.1) | `codec/varint`, `crc32c`, `xxh64`, `tag`, `keyrange`, `bloom`, `limits`; `tests/codec_property.rs` (6 Tests grün) |
| **Run-Schreib- und Leseseite** (WP-3.1, WP-3.2) | `run/builder`, `block`, `data`, `index`, `filter`, `footer`, `rangedel`, `meta`, `reader`, `table_cache`, `block_cache` (CLOCK); `tests/run/` (8 Tests grün) |
| **Blob-Format** (WP-3.3, ohne Sweep) | `blob/record`, `writer`, `reader`, `blob_ref`; Hole-Records unverschlüsselt |
| **Catalog** (WP-3.4) | `catalog/edit`, `log`, `state`, `current`, `format`, `reserve`, `rollover`, `durable`; `tests/catalog/` (21 Tests grün) mit Absturzmatrix |
| **FORMAT-Header, Downgrade-Erkennung auf Modulebene** (WP-2.2, Teil) | `catalog/format.rs`; `tests/format/f15_test.rs` (3 Tests grün) |
| **Schlüsselhierarchie und Nonce-Protokoll** (WP-2.1, Teil) | `keys/hierarchy`, `keys/nonce` (Präfixe `0x01`–`0x05` disjunkt), `keys/secret` (`Secret32`); `tests/keys/` (11 Tests grün) |
| **Dateisystem-Konformität** (WP-1.2, Teil) | `testing/conformance/fs_suite.rs`; `tests/fs_real_test.rs`, `tests/fs_mem_test.rs` grün |
| **Schichtregel-Test** (WP-0.2, Teil) | `tests/layering.rs` grün; `cargo tree`-Prüfung grün (Z-4) |
| **Keine Fremdnamen und kein FeatureRing im Neubau** (BL-6, M.4) | `grep FeatureRing crates/` ergibt 0 Treffer |
| **Konsistente Tag-Kinds, Stamp-Grenze** (SPEC §6.2) | `codec/tag.rs`; `MERGE` liefert `UnsupportedMerge`; `MAX_STAMP` in `codec/limits.rs` |
| **Lock-Rangprüfung zur Laufzeit im Debug-Build** (INV-LOCK-1, O-15 entschieden) | `sync.rs` `push_rank`/`pop_rank` mit `thread_local!` |
| **Entfernte Altlasten** | Rev. 1 §3 (Korrekturen gegenüber Rev. 1, alle erledigt); Rev. 2 §2 (Ist-Messung des Altstands); Rev. 2 §8.2 (Testmarker des Altbestands); Rev. 2 Anhang A.2–A.3 (Messskript für `crates/contextra-store`, Pfad existiert nicht mehr); BL-6 (FeatureRing); Befunde B-a bis B-e, B-f, B-h, soweit sie nur den Altcode betreffen |

---

## 3. Blocker und Vorbedingungen

Jeder Blocker muss vor dem genannten Gate entschieden sein. Empfehlungen sind Vorschläge; die Entscheidung trägt der Projektverantwortliche.

| ID | Befund | Optionen | Empfehlung | Frist |
|---|---|---|---|---|
| **BL-1** | Der Altstand `legacy/contextra-store` ist nicht eigenständig baubar. `ahash`, `contextra-ids`, `-env`, `-sys`, `-mvcc`, `-crypto` und weitere sind `workspace = true`, ohne `[workspace.dependencies]` im Wurzelmanifest. | (a) Mini-Workspace unter `legacy/` mit fixiertem Lockfile; (b) Oracle nur als Golden-Dateien ohne Lauf | (a). Bei Fehlschlag: Befund, kein Umbau. | G-0 |
| **BL-3** | `StorageEngine` mit 122 Dateien mit Textbezug außerhalb des Repos; Semantik nicht im Kern | Adapter `contextra-storage-tx` mit Austrittskriterien (§9.4 Rev. 2) | Adapter | G-A |
| **BL-4** | Konsumenten (`contextra-db`, `-text`, `-graph`, …) liegen nicht im Repo | bleiben außerhalb bis Phase M | unverändert | G-0 |
| **BL-5** | `Purger` (24 Dateien) und `LayerScanner` (7) als Port-Traits ohne Implementierung im Neubau | (a) Erasure-Brücke über `erase` und `View`; (b) Konsumenten umstellen | (a), nach WP-6 | vor M.2 |
| **BL-7** | Referenzhardware, Toolchain-Pin. `rust-toolchain.toml` pinnt `1.89.0`; in dieser Umgebung nicht ladbar. Verwendet wurde `stable 1.97.0`. Referenzhardware für H-S ist nicht festgelegt. | Pin auf 1.97 anheben, oder 1.89 in CI verankern; Referenzhardware dokumentieren | Pin entscheiden und dokumentieren; Referenzhardware in CI festlegen | G-0 |
| **BL-8** (neu) | Ohne verdrahteten Redo-Pfad (S-1) gilt jedes Durabilitätsversprechen als unbelegt. Tests über `MemFs` können diesen Fehler nicht finden. | Blocker für alle Arbeiten an WP-4.x und WP-5.1 | Vor jeder weiteren API-Arbeit beheben | sofort |
| **BL-9** (neu) | Kein CI im Repo (`.github/` fehlt). Gates sind damit nicht maschinell erzwungen (R-13). | Minimal-CI: `cargo check`, `cargo test`, `cargo clippy -D warnings`, `cargo doc`, `tests/layering.rs` | Einführen | G-0 |

BL-2 (Port-Name) und BL-6 (Lizenzkopplung) sind entschieden bzw. obsolet und entfallen.

---

## 4. Kritische Befunde (Rev. 3)

Schwere: **K** kritisch (Verstoß gegen Z-1, F-1, F-6, F-7, F-10 oder P-4 mit Panikpfad) · **H** hoch · **M** mittel. Alle Belege mit Datei:Zeile, geprüft durch Lesen des Codes oder durch Testlauf.

| ID | Schwere | Befund | Beleg | Behebt in |
|---|---|---|---|---|
| **S-1** | K | Committer schreibt pro Change nur den 8-Byte-Stamp als Redo-Frame („Frame-Dummy“). Operationen, Schlüssel und Werte werden nicht persistiert. Die Schnittstelle `RedoWriter` hat keine Produktionsimplementierung. | `engine/committer.rs:946–951`, `:517`, `:956` | WP-4.2, WP-3.5 |
| **S-2** | K | Recovery, Flush und Compaction sind nicht im Modulbaum deklariert und werden nicht übersetzt. `recover_instance` hat keinen Aufrufer. | `engine/mod.rs` (nur 6 Module); `engine/recovery.rs:150` | WP-4.3, WP-4.4 |
| **S-3** | K | Keine öffentliche Storage-Facade (`Storage::open`, `space`, `apply`, `view`). Typen `Space<Rd\|Ap\|Wr\|Adm>`, `Config`, `Error` mit `hint()`, `Page`, `ResumeToken`, `Estimate` fehlen. | `api/mod.rs` (nur `listener`); Suche nach den Typnamen ohne Treffer | WP-5.1 |
| **S-4** | K | `erase/` enthält keinen Code. `Erase` (E0–E7), `ErasureReceipt`, `verify_erasure`, `scan_physical`, `Erased`-Antworten fehlen. Z-3 ist nicht erfüllt. | `erase/mod.rs` (11 Zeilen) | WP-6.1, WP-6.2 |
| **S-5** | K | INV-LOG-6 verletzt: `replay_sm.rs` klassifiziert jeden Prüfsummenfehler als `TornTail`, auch wenn danach gültige Daten folgen. `replay.rs` schneidet das Segment bei `good_offset` ab und siegelt neu. Ein Bitflip im ersten Frame eines gesiegelten Segments verliert stillschweigend quittierte Daten. Der Test `test_inv_log_6_corrupt_in_sealed_segment` schlägt fehl. | `redo/replay_sm.rs:223–229`; `redo/replay.rs:142–189`; `tests/redo/invariants_tests.rs:188–202` | WP-3.5 |
| **S-6** | M | Drei rote Redo-Tests sind Testfehler: Der Offset `48+29` trifft die Prüfsumme von Frame 1 statt des Stamps; der Frame belegt Bytes 48–79. `fail_enospc` wird nicht zurückgesetzt, bevor die Recovery läuft. Korrektur: Offset `48+31`, und vor der Recovery `fail_enospc.store(false)`. | `tests/redo/crash_matrix_tests.rs:49, 89–97, 116–120` | sofort |
| **S-7** | K | P-4 verletzt: Längen aus Dateiinhalten werden vor der Prüfung allokiert. `Vec::with_capacity(count)` mit ungeprüftem Zählwert kann paniken oder abbrechen. `vec![0u8; rlen]` bis 4 GiB vor dem Lesen. | `run/index.rs:117`; `blob/reader.rs:143, 164, 189` | WP-1.1, WP-3.3 |
| **S-8** | H | Deny-Lint verletzt im Produktionscode: 8 `unwrap` (`engine/committer.rs:563, 569, 571, 580`; `sync.rs:101, 126, 155, 161`). `sync.rs` hebt den Lint mit `#![allow(clippy::panic)]` auf und enthält 7 `panic!`. | `cargo clippy`; `sync.rs:9, 37, 54, 86, 107, 132, 197, 214` | WP-0.2, WP-0.4 |
| **S-9** | H | Feature `testing` ist Standard (`default = ["testing"]`). Die Release-API enthält damit Testhaken. SPEC §3.4 verlangt das Gegenteil, und der Test `no_test_hooks_in_release_api` fehlt. | `crates/contextra-storage/Cargo.toml` | WP-0.2 |
| **S-10** | H | `REPORT.md` behauptet bestandene Messungen (H-S1 p99 18,9 µs; H-S9 100 %). Es gibt keinen Benchmark-Code im Crate; `criterion` steht nur im Altcode. H-S9 misst Filterabweisung statt Block-Lesevorgänge und nur eine Run; SPEC §20 verlangt ≥ 90 % der Runs. Der Befund wird als nicht reproduziert geführt. | `REPORT.md`; `Cargo.toml` (keine `[[bench]]`) | WP-9.6, CR-5 |
| **S-11** | H | Loom-Nachweis nicht belastbar: 0 von 4 Tests unter `cfg(loom)` grün. `loom_commit_tests.rs` testet eigene Typen (`TestTicket`, `IntakeQueue`), kein Produktionsmodul. Rang 6 (`maint`) ist nirgends verwendet. | `tests/loom/*`; `sync.rs:26–27` | WP-9.4, WP-0.4 |
| **S-12** | H | INV-FS-1 nicht verdrahtet: `check_fs_policy` wird nur in Tests aufgerufen. Eine Instanz auf einem schwachen Dateisystem wird nicht abgelehnt. `Config::fs_policy` fehlt, da kein `Config` existiert. | `env/mod.rs:26`; Aufrufe nur in `tests/env/main.rs` | WP-1.2, WP-5.1 |
| **S-13** | H | Invariantenabdeckung formal: 14 von 43 INV-IDs ohne Test. `tests/invariants.rs` prüft `≥ 35` und zählt Doc-Kommentare in `src/`. | `tests/invariants.rs`; Abschnitt 1.4 | WP-0.6, WP-9.5 |
| **S-14** | H | Guards prüfen nur die aktive Memtable, nicht frozen-Memtables und Runs (SPEC §6.5, §10.2 Schritt 5). Bei Rotation übersieht `Absent` Schlüssel (Lost Update), und `Equals` lehnt gültige Werte ab. Latent, da `Version::rotate` nicht aufgerufen wird. | `engine/committer.rs:824–884` (`sp.active`); `engine/version.rs:137` | WP-4.2 |
| **S-15** | H | Gegendruck und Schreibfehler weichen von SPEC ab: Slowdown ist `yield_now`; `Stopped` liefert sofort `Busy` statt bis `stall_timeout` zu warten; jeder Schreibfehler poisoned die Instanz, auch ENOSPC, das `Busy{StorageFull}` sein soll. Ticket-ID ist fest `1u64`. | `engine/committer.rs:742–748, 760, 956–967` | WP-4.2 |
| **S-16** | M | Views lesen nur die Memtable. `get` über Runs, `scan`, `get_many`, `bodies`, `history` fehlen. | `engine/views.rs:32–60` | WP-4.1 |
| **S-17** | M | Fehlerhafte Testfunktion in Produktionscode exportiert: `create_view_inverted_order_buggy`. | `engine/mod.rs:27`; `engine/views.rs:116` | sofort (entfernen oder `cfg(test)`) |
| **S-18** | M | Kompaktions-Install legt Eingaben direkt ins Graveyard, ohne `obsolete`-Markierung und ohne Arc-Drop-Bedingung. Leser mit alter Version könnten Dateien verlieren (SPEC §16.5). Der Code ist derzeit nicht übersetzt. | `engine/compaction/install.rs:132–147` | WP-4.4 |
| **S-19** | M | Testabdeckung der Fuzz-Ziele und Werkzeuge unvollständig: 3 von 13 geforderten Fuzz-Zielen; `arbitrary` als Dev-Abhängigkeit fehlt; `deny.toml` hat nur `[licenses]`; `miri` nicht konfiguriert; `docs/archive` fehlt. | `fuzz/fuzz_targets/`; `Cargo.toml`; `deny.toml` | WP-9.3, WP-0.2 |
| **S-20** | M | Namensdrift: SPEC nennt `contextra-ports`, Code `contextra-storage-ports`. SPEC nennt `legacy/contextra-store` als eingefrorene Referenz, das Layout enthält außerdem `legacy/archive/` (mehrere Crates, u. a. `contextra-db`, `-engine`, `-crypto`, `-checkpoint`), das SPEC nicht erwähnt. | `SPEC.md` §3.1; `legacy/` | CR-1, §10 |

**Weitere Abweichungen (M, je Arbeitspaket behandelt):**

- `Env` enthält `hooks`, `access_sink`, `checksum` nicht (SPEC §4 verlangt sie immer). Zusatz `File::is_empty()` ist nicht in der SPEC.
- `Extension` hat nur `id()`; `build` und `merge` fehlen (SPEC §4 nennt sie mit `..`).
- `Decision::DropBody` existiert in `engine/compaction/retain.rs:119`, ist aber in SPEC §12.2 nicht definiert. Entweder aufnehmen (CR) oder streichen.
- `KeyProvider`: NotFound-Semantik (`nie angelegt ⇒ Err(NotFound)`) ist nicht implementiert; der Vertrag steht nicht im Rustdoc.
- P-5: `env/atomic.rs:11` ist ein prozessweiter Zähler; die Injektion über `Env::hooks()` fehlt.
- P-14: 2 eindeutige `as`-Casts (`keys/protocol.rs:109, 310`) und 5 Grenzfälle (`catalog/format.rs:65`, `env/governor.rs:36, 46, 47, 50`). Der Lint `as_conversions` ist nicht aktiviert.
- P-12: `Secret32` ohne `Debug`/`Clone` ist korrekt; ein Test, der Ausgabeverbote prüft, fehlt.
- Die Bedingung „Fehler ohne Daten“ (§15.7) und das Leck-Orakel sind nicht geprüft.

---

## 5. Arbeitspakete: Restumfang

**Legende:** ✓ abgeschlossen (nur noch in §2) · ◐ Rest · ✗ offen.
**Umfang** (Schätzung): S ≤ 800 · M 800–2 000 · L 2 000–4 000 · XL > 4 000 Zeilen. **Kompl.:** 1 gering · 2 mittel · 3 hoch.

### WP-0 Fundament

| WP | Status | Verbleibende Arbeit | Abnahme | Umfang | Kompl. |
|---|---|---|---|---|---|
| 0.1 | ✓ | — (Workspace-Build, siehe §2). BL-1 bleibt für den Altstand offen. | — | — | — |
| 0.2 | ◐ | CI einführen (BL-9); `deny.toml` um `[advisories]`, `[bans]`, `[sources]` erweitern; `default = ["testing"]` entfernen (S-9); Layering-Regeln schärfen (Abschnitt 5.1 unten); Lint `as_conversions` aktivieren; `warn(missing_docs)` in CI zum Fehler machen | `cargo deny` grün; Schichtverletzungs-Probe bricht CI; Release-Build ohne Testhaken | S | 1 |
| 0.3 | ◐ | `Env` um `hooks`, `access_sink`, `checksum` ergänzen; `NotFound` in `KeyProvider` aufnehmen; `Extension` um `build`/`merge` ergänzen oder SPEC ändern; `File::is_empty` streichen oder SPEC ergänzen; `Decision::DropBody` entscheiden (CR) | `cargo doc` ohne Warnung; Port-Vertrag-Tests für `KeyProvider` | S | 1 |
| 0.4 | ◐ | `#![allow(clippy::panic)]` entfernen (S-8); `unwrap` in `sync.rs` und `committer.rs` ersetzen; Rang 6 einbinden oder streichen; Release-Verhalten des Rangstacks dokumentieren (O-15) | `cargo clippy -D warnings` grün; INV-LOCK-1-Test für Rangverletzung als Panic | S | 2 |
| 0.5 | ◐ | Goldens A.7 (`ResumeToken`) und A.8 (Authenticated, Encrypted, Run, Blob, Catalog) in zwei Implementierungen erzeugen | Goldens in Python und Rust identisch | M | 2 |
| 0.6 | ✗ | Mapping INV-ID → Test für die 14 fehlenden IDs; `tests/invariants.rs` auf 43/43 mit Testpflicht umstellen (nicht Doc-Kommentare) (S-13) | `tests/invariants.rs` schlägt bei fehlendem Test fehl | S | 1 |
| 0.7 | ✗ | Referenzmessung des Altstands (BL-1 abhängig) und Referenzmessung des Neubaus auf derselben Hardware | Bericht mit Streuung; Basis für H-S-Grenzen (BL-7) | M | 2 |

#### 5.1 Schichtregeln (Ergänzung zu SPEC §3.3)

`tests/layering.rs` ist grün, deckt aber nicht alle Kanten der Zielschichtung ab:

- `codec` und `env` sind Senken und dürfen keine internen Module importieren (fehlt im Test).
- `api` darf nur `engine`, `erase`, `catalog`, `keys` und `codec` als Typquellen nutzen; keine Kante `engine → keys` außer über Schnittstellen (SPEC §3.3 zeigt sie nicht).
- `erase → engine` ist die einzige erlaubte Kante von `erase` nach oben; `erase → blob`, `erase → catalog`, `erase → run`, `erase → redo` sind laut Test erlaubt, laut SPEC §3.3 nicht.
- Inline-Pfade wie `crate::run::data::…` ohne `use` werden nicht erfasst (Beispiel `engine/compaction/exec.rs:79`); der Test muss auch `crate::`-Pfade im Code prüfen.

### WP-1 Primitive und Umgebung

| WP | Status | Verbleibende Arbeit | Abnahme | Umfang | Kompl. |
|---|---|---|---|---|---|
| 1.1 | ◐ | P-4 in Decodern beheben (S-7: `run/index.rs:117`, `blob/reader.rs:143, 164, 189`); Längenprüfung gegen §6.7 vor jeder Allokation; Fuzz-Smoke 60 s je Ziel; `miri` für `codec`; typisierte Newtypes `Bounded<T, MAX>`, `CanonicalVarint` prüfen (M-7) | Fuzz und Dekoder-Tests mit übergroßen Längen bestehen ohne Panik; `miri` grün | M | 2 |
| 1.2 | ◐ | `check_fs_policy` in den Öffnungspfad (S-12); `Entropy`-Standard auf `std`/`/dev/urandom` beschränken, fail-closed sonst (CR-7, O-13) | INV-FS-1: `Strict` verweigert schwache Eigenschaften im Öffnen | S | 2 |
| 1.3 | ◐ | `FaultFs`: Testhygiene für Fehlerschalter (in S-6 gezeigt: Schalter vor der Recovery zurücksetzen, ggf. Helfer `FaultCfg::reset()`); Absturzenumerator (WP-9.2); Zähler `ATOMIC_WRITE_COUNTER` (P-5) durch `Env::hooks` ersetzen | Selbsttests §19.2 vollständig; Läufe deterministisch aus Seed | M | 3 |
| 1.4 | ◐ | `PressureSource`-Implementierung und Governor-Anbindung an den Committer (`env/governor.rs`, Rückgabe `Busy{retry_after}`) | Stalltests mit `Busy{retry_after}` | S | 2 |

### WP-2 Schlüssel und Kopf

| WP | Status | Verbleibende Arbeit | Abnahme | Umfang | Kompl. |
|---|---|---|---|---|---|
| 2.1 | ◐ | Epochen in Writern verwenden (E-13: `derive_run_key`, `derive_blob_key`, `derive_segment_key` werden nur in Tests aufgerufen); `SecretAlloc` in Pfade einbinden oder als optional dokumentieren (SD-56); INV-KEY-1-Test mit der Produktionsfunktion statt eigener Ableitung | INV-KEY-1 als Property über Produktionscode | M | 2 |
| 2.2 | ◐ | `Config.integrity` und Abgleich mit `FORMAT` bei Öffnung (F-15 End-to-End); Ausgabe mit `Unsupported{integrity_mode}` | F-15-Test auf Öffnungsebene | S | 1 |

### WP-3 Persistenz

| WP | Status | Verbleibende Arbeit | Abnahme | Umfang | Kompl. |
|---|---|---|---|---|---|
| 3.1 | ◐ | Block-AAD prüfen (§9.5 nicht verifiziert); H-S9 mit Messung nach §20 (Abweichung S-10 beheben) | Golden A.6 (Run) in zwei Implementierungen; H-S9 nach SPEC-Definition gemessen | S | 2 |
| 3.2 | ◐ | H-S1 und H-S12 mit Messcode reproduzierbar machen (S-10); Extension-Blöcke: `build`/`merge` (Abschnitt 4) | H-S1, H-S12 gemessen; Lesetest für Extension-Blöcke | S | 2 |
| 3.3 | ◐ | `sweep_blob` mit Katalog-Edit verbinden (sonst kein Löschen); P-4 im Reader (S-7); INV-BLOB-1…3 testen | Orakel: keine Blob-Datei ohne Katalog-Eintrag verwaist | M | 3 |
| 3.4 | ✓ | — (siehe §2). Die Absturzmatrix des Catalogs ist grün. | — | — | — |
| 3.5 | ◐ | **Kritisch.** (a) Committer-Verdrahtung über `RedoWriter` mit echten Frames (S-1); (b) Torn-Tail-Klassifikation nach S-5 (Folgedaten prüfen, nicht nur Lesepuffer; Nullbytes ausnehmen); (c) Shared-Modus `open_readonly` schreibt nicht (`redo/replay.rs:177–190`); (d) Absturzmatrix §13.2: Zeilen 2, 4, 5 rot; 6 und 7 teilweise; 12–16, 21, 22 ohne Test; (e) `driver.rs` anbinden, `reserve_segment_id` nutzen | INV-LOG-1…8 grün; Absturzmatrix §13.2 vollständig; Golden A.5 (Redo) mit echten Frames; Fuzz `wal_roundtrip` | L | 3 |

### WP-4 Engine

| WP | Status | Verbleibende Arbeit | Abnahme | Umfang | Kompl. |
|---|---|---|---|---|---|
| 4.1 | ◐ | Views über Runs erweitern (S-16): `get`, `get_many`, `scan`, `bodies`, `history`; `Version::rotate` aufrufen (S-14) | INV-VIS-1…3 mit Runs; Modelltest für INV-RET-1 | M | 2 |
| 4.2 | ✗ | **Kritisch.** (a) Ops in Frames kodieren (S-1); (b) Guards über aktive, frozen und Runs (S-14); (c) Gegendruck mit `stall_timeout`, ENOSPC als `Busy` (S-15); (d) dynamische Ticket-ID (S-15); (e) Datei auf ≤ 400 Zeilen (P-6, 1 103 Zeilen, Aufteilung z. B. in `intake`, `group`, `guards`, `publish`); (f) `make_durable` vor Ticket bleibt (INV-KEYP-1, prüfen gegen `NotFound`) | H-S2 gemessen; INV-VIS-1…3; loom-Modelle 1–2 auf echtem Code (WP-9.4); Guard-Tests mit frozen-Memtable | L | 3 |
| 4.3 | ✗ | **Kritisch.** Modul `recovery` deklarieren (S-2) und übersetzen; 11 Schritte aus §13.1 umsetzen; Domänenentsiegelung (Schritt 6) und Schritte 9–11 ergänzen; Shared-Modus (3.5 c) | Absturzmatrix §13.2 vollständig grün; H-S4 gemessen; INV-FLUSH-1 | L | 3 |
| 4.4 | ◐ | `plan` und `retain` sind rein, aber nicht übersetzt (S-2). `install` mit `obsolete` und Arc-Drop-Bedingung (S-18); Planer-Gesetze testen (keine Überlappung auf Level ≥ 1) | Planer-Gesetze; INV-CLAIM-1 mit loom auf echtem Code | M | 2 |
| 4.5 | ✗ | Blob-GC und Konsolidierung (SD-43): Planer, Katalog-Edit, Sweep (§12.3). Abhängig von 3.3 | H-S17; INV-BLOB-2 | S | 2 |
| 4.6 | ✗ | Modul `maintenance` (Lanes, Claims); `Graveyard` als eigenes Modul; `Claim`-Konflikttests mit `Registry` abgleichen | INV-CAT-4; Claim-Konflikttests | S | 2 |
| 4.7 | ◐ | Dauerhafte Holds (`hold_durable`), Konsumenten-Cursor persistent (`Consumer`, SPEC §7.8.2), `HoldId`, Maximalrückstand; derzeit flüchtig (`registry.rs`) | INV-HOLD-1 (P9) | S | 2 |

### WP-5 API

| WP | Status | Verbleibende Arbeit | Abnahme | Umfang | Kompl. |
|---|---|---|---|---|---|
| 5.1 | ✗ | **Kritisch.** Facade `Storage`, `Space<Rd\|Ap\|Wr\|Adm>` mit `AdminCap` (SD-34), `Config` (§6.6, inkl. `fs_policy`), `Change` mit `durability()`, `Guard`, `Receipt`, `View`/`view_at`, `Cursor`, `Page`/`PageLimit`/`ResumeToken`, `Estimate`, `history`, einheitlicher `Error` mit `code()`, `retryable()`, `hint()` (§7.7), `submit`/`Pending` (§7.8.6), `inspect` | Konformitätstests (`testing::conformance`); Fehlermatrix §7.7; Lecks-Orakel (INV-API-1) | XL | 2 |
| 5.2 | ✗ | Segment-Rahmen: Writer, Reader, Siegel (kein Modul `segment` vorhanden) | INV-SEG-1 (P10) | M | 2 |
| 5.3 | ◐ | `Extension` um `build`/`merge` ergänzen (Abschnitt 4); Dokumentation der Instabilität bis F10 (A-10) | Lesetest für Extension-Blöcke | S | 1 |

### WP-6 Löschung

| WP | Status | Verbleibende Arbeit | Abnahme | Umfang | Kompl. |
|---|---|---|---|---|---|
| 6.1 | ✗ | **Kritisch.** `erase/plan`, `erase/exec` (E0–E7), `erase_status`, `ErasureReceipt`, `Erase::domain`; Marker `EraseRange` im Lesepfad (`Erased`, §14.6); Absturzmatrix E1–E7 | INV-ERA-1…7 mit Test; Roll-forward-Test | L | 3 |
| 6.2 | ✗ | `erase/registry`, `verify`, `physical_scan` (`INV-SCAN-1`: ändert nichts, gibt keine Inhalte preis) | ErasureOracle: 0 Treffer in allen Dateiarten (Runs, Blob-Generationen, Redo, Segmente, `.building`) | M | 2 |

### WP-7 Modi und Schlüssel

| WP | Status | Verbleibende Arbeit | Abnahme | Umfang | Kompl. |
|---|---|---|---|---|---|
| 7.1 | ◐ | `Authenticated`: MAC mit AAD-Bindung in allen Writern prüfen; Goldens im Modus `Authenticated` fehlen (A.8) | Manipulationstests (A-3); Goldens | M | 2 |
| 7.2 | ✗ | `Encrypted` (AEAD), `domain_sealed` im Committer (§15.3, kein Siegel vorhanden), `derive_domain`, KEK-Rotation `rotate_kek` in O(1), Epochen-Klon | INV-KEYP-1, INV-EPOCH-1; H-S18 | L | 3 |
| 7.3 | ✗ | `RollbackAnchor` ist nur Trait; `record`/`latest` werden nicht aufgerufen; `RecoveryError::Rolledback` wird nie erzeugt. Anker in Recovery und Commit verdrahten | F-10-Test mit Anker; Lecks-Orakel | S | 2 |

### WP-8 Betrieb

| WP | Status | Verbleibende Arbeit | Abnahme | Umfang | Kompl. |
|---|---|---|---|---|---|
| 8.1 | ✗ | `verify` (online, offline), `checkpoint` (Copy, Hardlink), `open_readonly`, `hibernate`, `stats`, `health()` mit `durable_stamp` und `maybe_lost` (E-2) | injizierte Befunde (Bitflip, Lost Write, Misdirected Write) erkannt; H-S16 | L | 2 |
| 8.2 | ✗ | `ColdTier`-Port besteht; Löschung über `ColdTier::delete` in E6 ist nicht verdrahtet | Löschung über `ColdTier::delete` in E6 | S | 2 |

### WP-9 Verifikation

| WP | Status | Verbleibende Arbeit | Abnahme | Umfang | Kompl. |
|---|---|---|---|---|---|
| 9.1 | ✗ | Referenzmodell mit Holds, Consumern, `erased_ranges`; Seed-Ausgabe bei Fehlschlag | seed-reproduzierbar; alle Operationen aus §19.1 | L | 3 |
| 9.2 | ✗ | Absturzenumerator über I/O-Indizes und Hook-Marken, Vergleich mit Modell-Präfix. Derzeit nur `FaultFs::fail_after_ops` in einem Test (`tests/testing/fault_fs_tests.rs:28`); `tests/crash_prefix_enumeration.rs` existiert nicht | jede Zeile aus §13.2 ist ein Testfall | M | 3 |
| 9.3 | ◐ | Fuzz-Ziele auf ≥ 8 erweitern: Redo, Blob, Catalog, FORMAT, KEYS, KeyRange, ResumeToken; Stateful-Fuzz; `arbitrary` als Dev-Abhängigkeit (S-19); Bitflip-Netz für Run ist grün | Smoke 60 s je Ziel; Bitflip liefert nie falsche Daten | M | 2 |
| 9.4 | ✗ | loom-Modelle 1–5 auf echtem `sync.rs` und echtem Produktionscode (CR-4); eigene Nachbildungen in `tests/loom/` ersetzen (S-11); Mutationsprobe für Reihenfolgefehler | Modelle importieren `contextra_storage`; Mutationsprobe wird gefunden; `cfg(loom)` grün | M | 3 |
| 9.5 | ✗ | Orakel: Löschung, Lecks, Allokation, Schicht, Ressourcen, Befundlauf; Gegenbeispieltest je Orakel; INV-Abdeckung (S-13) | jedes Orakel hat einen absichtlich verletzenden Gegenbeispieltest | M | 2 |
| 9.6 | ✗ | Benchmarks H-S gegen Basis (BL-7) als `[[bench]]`-Ziele; `REPORT.md` nach Messung neu erstellen oder zurückziehen (S-10) | G-P-Bericht mit Rohdaten in `results/` | M | 1 |

### WP-M Migration (ab G-F)

| WP | Status | Verbleibende Arbeit | Abnahme | Umfang | Kompl. |
|---|---|---|---|---|---|
| M.1 | ✗ | Adapter `contextra-storage-tx`: TxId, Puffer, Savepoints, SSI-Lesemengen, Future-Fassade (Rev. 2 §9.2/§9.3, AR-1…AR-13) | Konformitätssuite ≥ 95 % mit Oracle-Abgleich | XL | 3 |
| M.2 | ✗ | Erasure-Brücke für `Purger` und `LayerScanner` (BL-5) | Tests von `contextra-vector`, `-text`, `-graph`, `-erasure` grün | L | 3 |
| M.3 | ✗ | Konsumenten-Umstellung (BL-4: außerhalb des Repos) | je Crate eigene Abnahme | XL | 3 |
| M.4 | ✓ | — (kein `FeatureRing` im Neubau, siehe §2) | — | — | — |
| M.5 | ✗ | Abbau von `legacy/` nach M.3 und Ablösekriterien (Rev. 2 §9.4) | Kriterien erfüllt | S | 1 |

---

## 6. Phasen und Gates (neu bewertet)

| Phase | Arbeitspakete | Gate-Kriterium (SPEC/Plan) | Status | Begründung |
|---|---|---|---|---|
| **P0** | 0.1–0.7 | G-0: Altcode kompiliert; Port-Crate konform; Goldens A.1–A.3 in zwei Implementierungen; Altstand-Basis gemessen | **teilweise** | Neubau kompiliert; Port konform mit Lücken (0.3); Goldens A.1–A.3 ✓; Altstand-Basis ✗ (BL-1) |
| **P1** | 1.1–1.4 | G-1: FaultFs-Selbsttests; `fs_suite` für `RealFs` und `MemFs`; Codec-Goldens; Fuzz-Smoke | **teilweise** | `fs_suite` ✓; Codec-Goldens ✓; FaultFs-Selbsttests grün, Fehlerschalter werden im Redo-Test vor der Recovery nicht zurückgesetzt (S-6); Fuzz-Smoke für 3 Ziele nicht gefahren; INV-FS-1 nicht verdrahtet |
| **P2** | 2.1, 2.2 | G-2: Nonce-Property; Downgrade-Test; Goldens A.4, A.6 | **teilweise** | Nonce-Präfixe ✓; A.4 ✓; A.6 ✓ für Redo, Run, Blob; Property auf eigener Ableitung; F-15 nur auf Modulebene |
| **P3** | 3.1–3.3 | G-3: H-S1, H-S9, H-S12 gemessen; Bitflip Run grün | **verfehlt** | Bitflip Run ✓; keine reproduzierbare Messung (S-10) |
| **P4** | 3.4, Recovery aus 4.3 | G-4: Absturzmatrix Catalog und Redo; H-S4, H-S5 gemessen | **verfehlt** | Catalog ✓; Redo-Absturzmatrix rot (S-5, S-6); Recovery nicht übersetzt (S-2) |
| **P5** | 3.5, 4.1, 4.2, 5.1 (Kern) | G-5: H-S2, H-S7; H-S8, H-S13; loom 1–2 auf echtem Code; Modelltest Stufe 1 | **nicht begonnen** | 4.2 und 5.1 offen; loom nicht auf echtem Code (S-11) |
| **P6–P10** | 4.4–4.6, 6.x, 7.x, 8.x, 5.2, 5.3 | G-6 … G-10 | **nicht begonnen** | abhängig von P5 |
| **G-A** | AR-1…AR-13 entschieden; O-10, O-11 | CR-2, CR-3 verabschiedet | **offen** | Entscheidungen fehlen (§8) |
| **G-F** | Format-Freeze nach SPEC §9.8 | Goldens aller Dateiarten in drei Modi; zwei Reviewer | **verfehlt** | Goldens für `Authenticated`, `Encrypted`, Run, Blob, Catalog fehlen (A.8) |
| **G-P** | Leistungsbericht H-S1…H-S18 | Bericht; Abbruchkriterien §20 | **verfehlt** | 0 von 18 gemessen (M-11) |
| **M1–M5** | §5 WP-M | Ablösekriterien erfüllt | **nicht begonnen** | — |

**Kritischer Pfad (neu):** 0.2/0.4 (Lints) → S-5/S-6 (Redo-Tests grün) → S-7 (P-4) → 3.5/4.2 (Redo-Verdrahtung, S-1) → 4.3 (Recovery, S-2) → 5.1 (Facade, S-3) → 6.1 (Erase, S-4) → 7.2/7.3 (Domänen, Anker) → 9.2/9.4 (Absturzenumerator, Loom) → G-F → M.1 → M.3.

**Begründung der Umordnung:** Der alte Pfad setzte P3 auf einen verdrahteten Committer voraus. Dieser existiert nicht. Die Umordnung zieht die Verdrahtung in P4/P5 vor und verschiebt Messungen (G-3, G-P) nach hinten, bis der Pfad durabel und testbar ist.

---

## 7. Sofortmaßnahmen (vor jeder Feature-Arbeit)

Diese Punkte sind klein, blockieren aber jede Aussage über Stabilität:

1. **Tests korrigieren:** Offset in `crash_matrix_tests.rs` auf `48+31`; `fail_enospc` vor der Recovery zurücksetzen (S-6). Danach muss `cargo test -p contextra-storage --test redo` grün sein.
2. **Deny-Lints beheben:** acht `unwrap` in Produktion (S-8); `#![allow(clippy::panic)]` in `sync.rs` entfernen und die sieben `panic!` durch `Err` ersetzen. `cargo clippy --workspace --all-targets -- -D warnings` muss grün sein.
3. **Standard-Features:** `default = ["testing"]` entfernen (S-9). Alle Tests, die `testing` brauchen, bekommen `required-features`.
4. **Längenprüfung:** `run/index.rs:117` und `blob/reader.rs:143, 164, 189` vor jeder Allokation gegen die Restlänge und §6.7 prüfen (S-7).
5. **INV-Test schärfen:** `tests/invariants.rs` darf nur Testdateien zählen und muss bei fehlendem Test fehlschlagen (S-13).
6. **REPORT.md:** H-S1 und H-S9 als „nicht reproduziert“ kennzeichnen, bis Messcode existiert (S-10).
7. **Unbenutzten Testcode entfernen:** `create_view_inverted_order_buggy` aus Produktion entfernen (S-17).
8. **CI-Gerüst** (BL-9): `cargo check`, `cargo test`, `cargo clippy -D warnings`, `cargo doc`, `cargo deny`, `tests/layering.rs`.

---

## 8. Risiken

| ID | Risiko | Status | Gegenmaßnahme |
|---|---|---|---|
| R-1 | Monorepo bricht durch Umbau | **eingetreten** teilweise: Workspace baut, aber `recovery`/`flush`/`compaction` sind nicht übersetzt | Gate-Kriterium „übersetzt“ je Modul; `cargo check` mit `--all-targets` in CI |
| R-2 | Adapter-Komplexität (SSI, Savepoints) | offen (M.1) | Fallback: Konsumenten-Umstellung je Crate ohne Adapter |
| R-3 | `BTreeMap`-Memtable zu langsam | offen (H-S7 nicht gemessen) | §20: Skip-Liste als isolierte Einheit |
| R-4 | `make_durable` blockiert den Schreibpfad | offen (H-S2 nicht gemessen) | Aufruf vor dem Ticket (SPEC §10.2 Schritt 1b); Domänenschlüssel-Cache |
| R-5 | Redo-Recovery zu komplex für eine Einheit | **eingetreten**: WP-3.5 und 4.3 überschreiten den Plan | Aufteilung: Redo-Verdrahtung, Replay-Korrektur, Recovery getrennt |
| R-6 | Formatfehler vor Freeze | offen (A.7, A.8 fehlen) | G-F erst nach zwei Implementierungen |
| R-7 | Committer-Lock über Cache-Fehlgriffe | offen | Messung vor Umbau |
| R-8 | Plattformverhalten (`F_FULLFSYNC`, Verzeichnis-Sync) | offen | `FsCaps`-Test auf macOS; `Strict` verweigert |
| R-9 | Produktive Altdaten existieren | offen (A-01) | Importer wird Pflicht (F-I) |
| R-10 | Konsumententests nicht baubar | offen (BL-4) | vor M.3 klären |
| R-11 | Entropiequelle nur Unix | offen (O-13, CR-7) | fail-closed |
| R-12 | Verlust der Scanner-Funktionen | offen (BL-5) | Erasure-Brücke vor Abbau von `legacy/` |
| R-13 | Dokumentdrift zwischen SPEC und Repo; keine CI | **eingetreten**: `contextra-ports` vs. `contextra-storage-ports`; keine `.github/` | BL-9; Anträge nach SPEC §1.5 |
| **R-14** (neu) | Grüne Tests täuschen Durabilität vor, weil `MemFs`-Tests keine echten Frames prüfen | **eingetreten** (S-1) | Persistenztests mit Wiederöffnung nach Frame-Inhalt: Ops müssen nach Neustart gelesen werden |
| **R-15** (neu) | Toolchain-Abweichung: Pin 1.89.0 nicht geprüft; Ergebnisse gelten für 1.97 | offen (BL-7) | Pin entscheiden; Ergebnisse in CI mit dem Pin wiederholen |
| **R-16** (neu) | Leistungsbelege ohne Code (REPORT.md) werden übernommen | **eingetreten** (S-10) | H-S-Status nur mit Messcode und Rohdaten |

---

## 9. Offene Entscheidungen

| ID | Frage | Empfehlung | Status | Entscheidung bis |
|---|---|---|---|---|
| O-1 | Toolchain und MSRV | Pin entscheiden: 1.89 (SPEC A-05) oder 1.97; in CI verankern | **offen** (BL-7) | G-0 |
| O-2 | Abgeleitete Daten nach `Erase` (Embeddings, Postings) | Segmente je Löschdomäne; `rewrite_without` Pflicht | offen | F10 |
| O-3 | `BodyCache` | nur wenn H-S15 es belegt | offen | F5 |
| O-4 | Windows-`Fs` | eigenes Crate nach V1 (NZ-6) | offen | nach P9 |
| O-5 | Schwelle `blob_min_bytes` | 16 MiB, in P6 messen | offen | P6 |
| O-6 | Referenz-`Cipher`/`Mac` | `contextra-crypto-ref` ist vorhanden; Kandidat `contextra-crypto` aus Altstand prüfen | **teilweise entschieden** (Dev-Crate ✓) | G-F |
| O-7 | Blockkompression (SD-13) | frühestens V2 | offen | nach P9 |
| O-8 | Name des Port-Crates | `contextra-storage-ports` | **entschieden** (umgesetzt) | — |
| O-9 | Konsumenten im Repository? | nein, bis Phase M | offen | G-0 |
| O-10 | Strategie für `StorageEngine` | Adapter `contextra-storage-tx` | offen | G-A |
| O-11 | Semantik von `flush()` | `flush()` = Barriere plus Run-Bau (INV-FLUSH-1); `sync()` = Durabilität; `flush_async` streichen oder definieren | offen | G-A |
| O-12 | Stamp-Exposition | Kein Entry-Stamp; Lesemenge über `view.stamp()` | offen | G-A |
| O-13 | Entropiequelle | `/dev/urandom` (Unix); fail-closed sonst | **teilweise** (`env/entropy.rs` vorhanden) | vor P2 |
| O-14 | Lizenzentkopplung (BL-6) | obsolet, kein Lizenzbezug im Neubau | **entschieden** | — |
| O-15 | Lock-Rangprüfung | nur im Debug-Build (`debug_assertions`) | **entschieden** (umgesetzt) | — |

---

## 10. Änderungsanträge an `SPEC.md`

Nach SPEC §1.5 sind Änderungen an den Abschnitten 6–9, 13, 14 und 18 mit zwei Reviewern und Referenzvektor zu versehen. Die übrigen Anträge sind Klarstellungen.

| CR | Abschnitt | Änderung | Grund |
|---|---|---|---|
| CR-1 | §3.1, §4 | Port-Crate heißt `contextra-storage-ports` (umgesetzt; SPEC nachziehen) | S-20 |
| CR-2 | §7.2, §7.8.1 | Lesemenge für SSI ist (Schlüssel, `view.stamp()`) | O-12 |
| CR-3 | §7.1, §7.8.6, SD-59 | `flush()` definieren oder streichen; `flush_async` und `sync` vereinheitlichen (O-11) | O-11 |
| CR-4 | §16.7, §19.1 | loom-Modelle müssen echten `sync.rs`-Pfad und echten Produktionscode verwenden | S-11 |
| CR-5 | §20 | Referenzwerte sind Messungen auf Referenzhardware; `REPORT.md` ist keine Messquelle, solange kein Messcode existiert | S-10 |
| CR-6 | §18, Anhang B.1 | Befunde-Zählungen des Altstands sind historisch; Anhang B nur als Herkunft führen | Rev. 2 §3 |
| CR-7 | §3.2, §4 | `Entropy`-Standard nur `std` (`/dev/urandom`, fail-closed sonst) | R-11 |
| CR-8 | §2.2, Anhang B | Migrationsschicht (Adapter, Erasure-Brücke) als Schicht außerhalb des Kerns benennen | BL-3, BL-5 |
| CR-9 | §20 | H-S1…H-S18 bleibt; Rev. 1 nannte H-S19 fälschlich | Rev. 2 §3 |
| **CR-10** (neu) | §4 | `Decision::DropBody` aufnehmen oder aus `compaction/retain.rs` streichen | Abschnitt 4 |
| **CR-11** (neu) | §4 | `File::is_empty()` aufnehmen oder streichen | Abschnitt 4 |
| **CR-12** (neu) | §3.4 | `default`-Feature ausdrücklich verbieten (Test `no_test_hooks_in_release_api`) | S-9 |

---

## 11. Definition of Done (Metriken, Stand Rev. 3)

Alle Metriken sind mit den Befehlen aus Anhang A zu erheben und in CI zu vergleichen.

| ID | Metrik | Stand Rev. 3 | Soll | Werkzeug |
|---|---|---:|---:|---|
| M-01 | Produktionsdateien über 400 Zeilen | 1 (`committer.rs`, 1 103) | 0 | `wc -l` |
| M-02 | Integer-`as`-Casts in Produktion | 2 eindeutig, 5 Grenzfälle | 0 | `clippy` mit `as_conversions` |
| M-03 | direkte `std::fs`-Aufrufe außerhalb `env/real_fs.rs` | 0 | 0 | `grep` |
| M-04 | statische Items in Produktion | 2 (`atomic.rs:11`, `sync.rs:26`) | 0 | `grep` |
| M-05 | `cfg(feature)` in Logik außer `testing` | 0 | 0 | `grep` |
| M-06 | Modulzyklen | 0 (Layering-Test grün; Test lückenhaft, §5.1) | 0 | `tests/layering.rs` |
| M-07 | Schichtverletzungen gegen §3.3 | nicht vollständig prüfbar (§5.1) | 0 | `tests/layering.rs` (erweitert) |
| M-08 | `unwrap`, `expect`, `panic` in Produktion | 8 `unwrap` + 7 `panic` | 0 | `clippy -D warnings` |
| M-09 | INV-IDs mit Testzitat | 29 von 43 | 43 von 43 | `tests/invariants.rs` (verschärft) |
| M-10 | Laufzeitabhängigkeiten des Kerns | 1 (`contextra-storage-ports`) | 1 | `cargo tree -e normal` |
| M-11 | H-S-Messwerte berichtet | 0 von 18 | 18 von 18 | G-P-Bericht |
| M-12 | Tests grün | 4 von 24 Redo-Tests rot | 0 rot | `cargo test --workspace` |
| M-13 | Clippy-Deny-Fehler | 8 (lib), 24 (alle Targets) | 0 | `cargo clippy -D warnings` |
| M-14 | Loom grün unter `cfg(loom)` | 0 von 4 | 4 von 4 (auf echtem Code) | `RUSTFLAGS="--cfg loom" cargo test --test loom` |
| M-15 | Fuzz-Ziele | 3 | ≥ 8 | `fuzz/fuzz_targets` |
| M-16 | Doc-Warnungen | 2 | 0 | `cargo doc -D warnings` |
| M-17 | Dateien mit Modul-Verdrahtungsfehler (nicht übersetzt) | 4 (`recovery`, `flush`, `compaction/*`, `maintenance`) | 0 | Modulbaum-Prüfung |
| M-18 | Public-API-Facade (`Storage::open` …) vorhanden | nein | ja | Doc-Test |

---

## Anhang A: Messbefehle und Messstand

### A.1 Befehle

Alle Befehle im Wurzelverzeichnis des Repos. Der Toolchain-Pin wird wegen BL-7 mit `RUSTUP_TOOLCHAIN=stable` überschrieben, bis die Entscheidung getroffen ist.

```bash
export RUSTUP_TOOLCHAIN=stable
cargo check --workspace --all-targets
cargo test --workspace --no-fail-fast
cargo test -p contextra-storage --test redo
RUSTFLAGS="--cfg loom" cargo test -p contextra-storage --test loom
cargo clippy -p contextra-storage --lib
cargo clippy --workspace --all-targets
cargo doc -p contextra-storage --no-deps
cargo tree -e normal -p contextra-storage
```

Regelzählungen (Produktionscode, Tests ausgenommen über den Pfad `tests/` und `#[cfg(test)]`):

```bash
# Dateien über 400 Zeilen
find crates/contextra-storage/src -name '*.rs' -exec wc -l {} + | awk '$1>400'
# unwrap/expect/panic (Produktion, grob)
grep -rn -E '\.unwrap\(\)|\.expect\(|\bpanic!\(' crates/contextra-storage/src
# statische Items
grep -rn -E '^\s*(pub(\([a-z]+\))? )?static ' crates/contextra-storage/src
# Integer-Casts (Grenzfälle manuell)
grep -rn -E '\sas\s+(u8|u16|u32|u64|u128|usize|i8|i16|i32|i64|i128|isize)\b' crates/contextra-storage/src
# Invariantenzitate
grep -rhoE 'INV-[A-Z]+-[0-9]+' crates/contextra-storage/tests crates/contextra-storage/src/testing | sort -u | wc -l
```

Das bisherige Messskript `measure_baseline.py` in Rev. 2 bezog sich auf `crates/contextra-store`. Es ist für den Neubau nicht mehr gültig und wird durch die obigen Befehle ersetzt.

### A.2 Messstand 22.11.2026

| Prüfung | Ergebnis |
|---|---|
| `cargo check --workspace --all-targets` | grün; 3 Warnungen |
| `cargo test --workspace --no-fail-fast` | `tests/engine` kompiliert nicht (21 Fehler, fehlende Module `engine::compaction`, `engine::recovery`, `engine::flush`, `KeyProvider`-Methoden); `redo`: 20 von 24 bestanden, 4 fehlgeschlagen. Der Lauf endet mit Compilerfehlern. |
| `contextra-crypto-ref` | 2 Unit- und 4 Golden-Tests grün |
| `contextra-storage` Unit-Tests | 25 grün |
| `tests/blob` | 11 grün |
| `tests/catalog` | 21 grün |
| `tests/codec_property` | 6 grün |
| `tests/engine` | nicht kompilierbar (Stand `af80842`; im älteren Stand `c09c5c0` liefen 7 Tests) |
| `tests/format` | 8 grün |
| `tests/golden_anhang_a` | 7 grün |
| `tests/invariants` | 1 grün (formal, S-13) |
| `tests/keys` | 11 grün |
| `tests/layering` | 1 grün (lückenhaft, §5.1) |
| `tests/run` | 8 grün |
| `tests/testing` | 20 grün |
| `tests/loom` ohne `cfg(loom)` | 4 grün (Testhülle ohne Modellprüfung) |
| `tests/loom` mit `cfg(loom)` | 0 von 4 grün (Prüfbericht, S-11) |
| `cargo clippy -p contextra-storage --lib` | 8 Deny-Fehler, 75 Warnungen |
| `cargo clippy --workspace --all-targets` | 24 Deny-Fehler an 21 Stellen, 75 Warnungen |
| `cargo doc -p contextra-storage --no-deps` | 2 Warnungen |
| `unsafe` | 0 |
| `cargo tree` | 1 interne Abhängigkeit |

---

## Anhang B: Herkunft und Auditspur

**Methodik Rev. 3.** Vier fokussierte Prüfungen, jeweils gegen einen Abschnitt von SPEC.md: (1) Ports, Codec, Keys, Env, Formate; (2) Schreibpfad, Recovery, Wartung; (3) öffentliche API, Löschung, Sicherheit, Nebenläufigkeit; (4) Invarianten, Tests, Hypothesen, Repo-Hygiene. Zentrale Befunde (S-1, S-2, S-3, S-4, S-5, S-7, S-9, S-13, Modulbaum, Default-Feature, Clippy-Zählung) wurden zusätzlich von Hand im Quelltext geprüft.

**Einschränkungen.**
- Flacher Klon: keine Commit-Historie. Die Aussage „Crate-Quelltext unverändert seit `47c9865`“ aus Rev. 2 ist damit nicht überprüfbar.
- Toolchain `stable 1.97.0` statt Pin `1.89.0` (BL-7).
- Die Loom-Diagnose (S-11) stammt aus einem Prüflauf; die Ursache „Rang-Verletzung“ ist nicht von der Testinfrastruktur getrennt belegt.
- Die Ursache von S-5 wurde im Code abgeleitet und in einer Kopie mit der vorgeschlagenen Korrektur bestätigt (Befund der Prüfung); eine Korrektur ist noch nicht im Repo umgesetzt.

**Befunde des Altstands.** Die Befunde B-a (zwei Replay-Pfade, `wal/replay.rs`), B-b (XOR-maskierter Legacy-Schlüssel), B-c (ungegatete Testschalter im Append-Pfad), B-d (Testschalter unter `fault-injection`), B-e (prozessweite Zähler), B-f (tote Features), B-g (Doku-Drift) und B-h (30 eigene INV-Kennungen) gelten für `legacy/contextra-store` und sind mit dem Neubau nicht mehr Teil des Umsetzungsplans. Sie bleiben im Altstand dokumentiert und werden mit BL-1 erneut bewertet, falls der Altstand als Oracle genutzt wird.

---

*Ende Revision 3.*
