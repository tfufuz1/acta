# acta-base

acta-base: Newtypes, Fehlerklassen, Ports, Festkomma.

> Teil des Acta-Workspace. Verbindliche Vorgaben: Wurzel-`CLAUDE.md` und `docs/SPEC.md`.

## Einordnung
- **Level:** 0 (`rings.toml`)
- **Größenbudget:** 2500 LOC Produktionscode (G-HUB); Überschreitung nur mit ADR
- **Aufgabe laut SPEC 3.3:** Newtypes (Ids, Ordinale), `DataClass`, `ErrorClass`, `ClassifiedError`, Ports `Clock`, `CryptoRng` (fallibel), kanonische Kodierung (geprüfte Big-Endian-Felder), Festkomma-Arithmetik `fx` (Q32.32, `fx_ln`, `isqrt`), Grenzwerte
- **Erlaubte interne Abhängigkeiten (SPEC 3.3):** –
- **Ist-Abhängigkeiten (Cargo.toml):** keine
- **Externe Abhängigkeiten:** thiserror
- **Dev-Abhängigkeiten (zählen nicht für G1):** proptest
- **Referenz im Quelltext:** Level L0 · Budget 2500 LOC · Spezifikation docs/SPEC.md Kap. 2.2, 2.4, 3.5, 3.7, 4.6, 5.1.

## Status
- **Stand:** implementiert (Teil)
- **Produktionscode (ohne Tests/Leerzeilen/Kommentare):** 603 LOC von 2500 Budget
- **Umgesetzt:** Ids und Ordinale inkl. `random` und UUIDv7 für `ErasureId`; SPEC 2.4-Newtypes `SubjectTag`, `ClusterId`, `IndexGen`, `ObjectId` (32 Byte); Klassen (`DataClass` mit Ebenenzuordnung); `BaseError` und `ClassifiedError`; Ports `Clock` und `CryptoRng` (fallibel); kanonische Kodierung (`canon::Reader`/`Writer`, geprüfte Längen vor Allokation, atomarer Schreiber); Festkomma `fx` (Q32.32, `fx_ln`, `isqrt`); Grenzwerte `limits` (Anhang A.1 bis A.4, Teilmenge, siehe unten).
- **Goldvektoren T-TT-FX (Z7):** 18 Werte in `tests/fx_golden.rs` sind mit dem unabhängigen Orakel `tests/golden/fx_ln_oracle.py` (100-stellige Dezimalarithmetik, half-even) reproduziert: 0 Abweichungen. Der BM25-Wert ist ebenfalls bestätigt. Die Positivkontrolle `golden_check_detects_a_wrong_value` ist aktiv.
- **Proptests (`tests/proptest_base.rs`, 12 Eigenschaften):** kanonische Hin- und Rückumwandlung aller Feldtypen, Blob-Grenze, keine Panik bei beliebiger Eingabe, Schreiber-Obergrenze und Atomarität, UUIDv7-Zeitordnung, streng monotone Ordinale, Monotonie und Determinismus von `fx_ln`, Sättigung von Addition und Subtraktion gegen eine i128-Referenz, Algebra der Multiplikation und Division, `isqrt`.
- **Offen und nicht geprüft:**
  - **aarch64:** Die Abnahme verlangt Goldvektoren auf x86_64 und aarch64 (Z7). Hier liegen weder ein aarch64-Target noch qemu vor, deshalb ist der aarch64-Lauf **nicht geprüft**. Die Arithmetik ist ganzzahlig und ohne Gleitkomma, eine Abweichung ist aber nicht ausgeschlossen, bis der Lauf stattgefunden hat.
  - **Anhang A unvollständig:** Nicht aufgenommen sind alle A.5–A.9 sowie A.2 (`erase_registry_sink`, `access_log_retention`), A.3 (`bridge_threads`, abhängig von der Kernzahl) und die Monatsparameter (`split_retest_interval`, `actor_handle_epoch`, deren Einheit „Monate“ ist). Ergänzung nach Bedarf der Crates, die sie brauchen.
  - **SPEC-Parameter ohne Startwert** (`T_BACKUP`, `anchor_destroy_max`, `support_min`, `inference_budget`) bleiben absichtlich ohne Konstante (0.4 Regel 1).
- **Archiv-Quelle:** `archive/acta-base/` (siehe `archive/acta-base/MANIFEST.md`). Der Archivordner wird **nicht** entfernt: die Id-Tests aus `archive/acta-base/ids/tests/` sind noch nicht als Orakel überführt.

## Regeln für diesen Crate
- Abhängigkeiten nur nach unten: `level(A) > level(B)`; keine Querkanten (G1).
- Keine Alias-Re-Exports, kein `include!`, kein `#[deprecated]` (HR-6).
- Ein Fehlertyp je Crate, `thiserror`, `#[non_exhaustive]`, kein `Other(String)` (HR-4).
- Workspace-Lints gelten (`[lints] workspace = true`): kein `unwrap`, `expect`, `panic`, Indexierung mit `[]` auf Produktionspfaden.
- `unsafe` ist hier verboten (G-UNSAFE, nur `acta-sys` darf es).
- Keine Zeit- oder Zufallsquellen außerhalb der injizierten Ports `Clock` und `CryptoRng` (G3).
- Verbotene Formulierungen aus SPEC 1.6 (Liste A) nicht verwenden (G-WORD).
- Vor Abschluss: `cargo xtask gates`, `cargo test -p acta-base`, `cargo clippy -p acta-base --all-targets -- -D warnings`, `cargo fmt --all -- --check`.

## Vorgehen bei Überführung aus dem Archiv
1. Archivdateien lesen, nicht blind kopieren. Namen, Importe und Verweise auf frühere Spezifikationsstände sind auf Acta umzuschreiben (siehe `archive/INDEX.md`).
2. Jede Funktion gegen den Lebenszyklus in SPEC Kap. 19 und die Zusagen in 1.5 und 16.3 prüfen. Funktionen ohne Eintrag dort werden nicht implementiert (SPEC 0.4 Regel 3).
3. Tests mit Positivkontrolle mitführen (SPEC 16.4).
4. Nach vollständiger, grün geprüfter Überführung wird der Archiv-Ordner dieses Crates entfernt.

## Entscheidung zum Archiv `archive/acta-base/`
Nichts wurde 1:1 uebernommen, weil das Archiv ein anderes Modell hat (u64-`TenantId`, `DocId`, `serde`) bzw. Bestandteile ausserhalb der SPEC-Zuordnung:
- `ids`: alte Id-Typen, ersetzt durch SPEC 2.4 (16-Byte-Handles, Ordinale).
- `env/clock.rs` (`SystemClock`, `SystemTime`): gehoert nach `acta-sys` (3.3), nicht nach L0.
- `env/rng.rs` (`SeededRng`, `f64`): Testdouble, liegt schon in `acta-testkit`; `f64` ist im Kern verboten.
- `env/id_gen.rs` (`IdGen`), `metrics`: ohne Eintrag in SPEC Kap. 19 (Default-deny, 0.4 Regel 3).
Das Archiv bleibt bis zur Ueberfuehrung von `acta-sys` und `acta-testkit` bestehen und wird erst danach entfernt.
