# acta-sys

acta-sys: einzige Stelle mit `unsafe` (G-UNSAFE; jeder Block mit `// SAFETY:`).

> Teil des Acta-Workspace. Verbindliche Vorgaben: Wurzel-`CLAUDE.md` und `docs/SPEC.md`.

## Einordnung
- **Level:** 1 (`rings.toml`)
- **Größenbudget:** 1600 LOC Produktionscode (G-HUB); Überschreitung nur mit ADR
- **Aufgabe laut SPEC 3.3:** Einziges eigenes `unsafe`: `mlock`, `MADV_DONTDUMP`, `RLIMIT_CORE`, `PR_SET_DUMPABLE`, `SecureBuf`; Sandbox (Landlock, seccomp); Hostprüfungen; `SystemClock`, `OsCryptoRng`
- **Erlaubte interne Abhängigkeiten (SPEC 3.3):** base
- **Ist-Abhängigkeiten (Cargo.toml):** acta-base
- **Externe Abhängigkeiten:** getrandom, landlock, rustix, seccompiler, thiserror, zeroize
- **Dev-Abhängigkeiten (zählen nicht für G1):** acta-testkit, proptest
- **Referenz im Quelltext:** Level L1 · Budget 1600 LOC · Spezifikation docs/SPEC.md Kap. 4.6, 13.1, 13.2.

## Status
- **Stand:** implementiert (Teil)
- **Produktionscode (ohne Tests/Leerzeilen/Kommentare):** rund 260 LOC von 1600 Budget
- **Umgesetzt:**
  - `SecureBuf`: jeder Puffer in eigener anonymer, seitenausgerichteter Abbildung (`mmap`), `mlock` und `MADV_DONTDUMP` fail-closed, Nullung vor `munlock`/`munmap` in `Drop`. Feste Kapazität. Kein `Default`, `Clone`, `Debug`, `Serialize`. Konstruktoren `zeroed`, `from_slice`, `random` (Entropiefehler → `Err`, nie ein Ersatzpuffer).
  - `harden_process`: `RLIMIT_CORE` hart auf 0, `PR_SET_DUMPABLE` auf 0 (SPEC 13.1). `memlock_soft_limit` für den Selbsttest.
  - Ports: `SystemClock` (`Clock`), `OsCryptoRng` (`CryptoRng`, Quelle `getrandom`).
  - `SysError` (HR-4): Klassen `Policy` für Host- und Speicherverweigerung, `Crypto` für Entropie, `Invalid` für unlesbare Prüfwerte, Codes ohne Werte.
  - `host`: Hostprüfungen nach 13.1/14.1 als reine Auswertung: Swap (`check_swap`, aktiver Swap nur mit Verschlüsselungserklärung), `core_pattern` ohne Pipe, `ptrace_scope` ≥ 1, `RLIMIT_MEMLOCK` gegen Bedarf. Fail-closed bei unlesbaren Werten.
- **Tests:** Einheitstests in `error.rs`; `tests/secure_buf.rs` (Funktion, Entropiefehler, Uhr, Zufall); `tests/mlock_control.rs` als Positivkontrolle über `VmLck` in `/proc/self/status` (eigenes Testbinary, da Sperrzustand prozessweit ist); `tests/harden.rs`.
- **Abweichungen von Archiv und Spec (bewusst):**
  - Archiv `mlock.rs` mit Richtlinie `Tolerant` (Warnung statt Fehler) **nicht übernommen**: verletzt Z10 (kein Fallback im Sicherheitspfad).
  - Kein `libc`, kein `tracing`: nicht auf der Whitelist (SPEC 3.8). Umgesetzt über `rustix` (Features `mm`, `process` aus der Whitelist) und `getrandom`.
  - Seitengröße 4096 als Annahme [A-PAGE]; `rustix::param` bräuchte das Feature `param`, das nicht auf der Whitelist steht.
  - `SystemClock`: Der Port liefert `u64` ohne Fehler. Eine Systemuhr vor 1970 wird auf 0 abgebildet (dokumentiert, Rücksprung-Erkennung liegt beim Knoten).
  - Jeder `SecureBuf` liegt auf eigenen Seiten. Dadurch hebt `munlock` nie die Sperre eines anderen Geheimnisses auf. Der Archiv-Ansatz (Vec auf dem Heap) hätte geteilte Seiten zugelassen.
- **Offen:**
  - Lesen der Prüfwerte (`/proc/swaps`, `core_pattern`, `ptrace_scope`) über `acta-fs` im Knoten (L7), verdrahtet in 14.1 Schritt 2.
  - Dateisystemtyp (statfs) und `O_DIRECT`-Unterstützung fehlen. `rustix::fs` ist nicht auf der Whitelist.
  - Sandbox (Landlock, seccomp) nach 13.2. Sie gehört zum Worker-Prozess (`acta-parse-worker` nutzt `acta-sys`), noch nicht begonnen.
  - SIMD-Kerne (Archiv `simd/`, ca. 3 000 Zeilen mit Gleitkomma): nicht überführt. Der Archivordner bleibt deshalb bestehen.
  - Gate G3 (keine ungeprüfte Zeit- oder Zufallsquelle) ist noch nicht in `xtask` umgesetzt; `SystemClock` nutzt `SystemTime::now` als einzige zugelassene Stelle.
  - Der Test `mlock_control.rs` liest `/proc/self/status` mit `std::fs`. G-FS erlaubt Dateizugriffe nur in `acta-fs`; die Ausnahme gilt nur für Tests. Die Gate-Regel ist noch nicht implementiert.
- **Archiv-Quelle:** `archive/acta-sys/` (siehe `archive/acta-sys/MANIFEST.md`). Der Archivordner bleibt bestehen, bis SIMD, Sandbox und Hostprüfungen überführt oder per ADR verworfen sind.

## Regeln für diesen Crate
- Abhängigkeiten nur nach unten: `level(A) > level(B)`; keine Querkanten (G1).
- Keine Alias-Re-Exports, kein `include!`, kein `#[deprecated]` (HR-6).
- Ein Fehlertyp je Crate, `thiserror`, `#[non_exhaustive]`, kein `Other(String)` (HR-4).
- Workspace-Lints gelten (`[lints] workspace = true`): kein `unwrap`, `expect`, `panic`, Indexierung mit `[]` auf Produktionspfaden.
- `unsafe` ist in diesem Crate erlaubt; jeder Block braucht `// SAFETY:` (G-UNSAFE, SPEC 3.1 Prinzip 6).
- Keine Zeit- oder Zufallsquellen außerhalb der injizierten Ports `Clock` und `CryptoRng` (G3).
- Verbotene Formulierungen aus SPEC 1.6 (Liste A) nicht verwenden (G-WORD).
- Vor Abschluss: `cargo xtask gates`, `cargo test -p acta-sys`, `cargo clippy -p acta-sys --all-targets -- -D warnings`, `cargo fmt --all -- --check`.

## Vorgehen bei Überführung aus dem Archiv
1. Archivdateien lesen, nicht blind kopieren. Namen, Importe und Verweise auf frühere Spezifikationsstände sind auf Acta umzuschreiben (siehe `archive/INDEX.md`).
2. Jede Funktion gegen den Lebenszyklus in SPEC Kap. 19 und die Zusagen in 1.5 und 16.3 prüfen. Funktionen ohne Eintrag dort werden nicht implementiert (SPEC 0.4 Regel 3).
3. Tests mit Positivkontrolle mitführen (SPEC 16.4).
4. Nach vollständiger, grün geprüfter Überführung wird der Archiv-Ordner dieses Crates entfernt.
