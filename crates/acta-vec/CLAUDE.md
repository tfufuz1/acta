# acta-vec

acta-vec: Segmentierter exakter i8-Vektorindex mit Lineage.

> Teil des Acta-Workspace. Verbindliche Vorgaben: Wurzel-`CLAUDE.md` und `docs/SPEC.md`.

## Einordnung
- **Level:** 3 (`rings.toml`)
- **Größenbudget:** 4000 LOC Produktionscode (G-HUB); Überschreitung nur mit ADR
- **Aufgabe laut SPEC 3.3:** Segmentierter Vektorindex (exakt, `i8`), Lineage
- **Erlaubte interne Abhängigkeiten (SPEC 3.3):** base, crypto, gate, vault, seg
- **Ist-Abhängigkeiten (Cargo.toml):** acta-base, acta-crypto, acta-gate, acta-seg, acta-vault
- **Externe Abhängigkeiten:** roaring, thiserror
- **Dev-Abhängigkeiten (zählen nicht für G1):** acta-testkit, proptest
- **Referenz im Quelltext:** Level L3 · Budget 4000 LOC · Spezifikation docs/SPEC.md Kap. 9.3

## Status
- **Stand:** Überführung AP 1 bis 4 umgesetzt (Quantisierung, exakte Ähnlichkeit, segmentierter Index mit parallelem Scan, Sichtklasse und Gate-Filter, Löschung über `ErasureLayer`, ANN-Schnittstelle mit exaktem Nachrang, Qualitätsmessung).
- **Offen:**
  - `t_vec_recall_1_accuracy_and_quality_report` schlägt fehl: Recall@10 = 0,85 statt ≥ 0,99. Ursache: der synthetische Testkorpus (verschobene Sinuskurven) hat Kosinusabstände an der Top-10-Grenze von rund 3·10⁻⁵, der i8-Quantisierungsfehler liegt bei bis zu 1,7·10⁻³. Auf unkorreliertem 768-d-Zufallskorpus misst dieselbe Implementierung 0,99. Entscheidung über Korpus oder Abnahmekriterium ausstehend (siehe PLAN 3 und 7).
  - Messbericht AP 5 (p95 150 ms, 1 Mio. × 768, 8 Kerne) ist nicht erstellt. Gemessen auf 1 Kern, 100 000 × 768, Filter 50 %: Median 33 ms, p95 35 ms.
- **Produktionscode (ohne Tests/Leerzeilen/Kommentare):** 884 LOC (Budget 4000)
- **Archiv-Quelle:** `archive/acta-vec/` bleibt bis der SIMD-Teil nach `archive/acta-sys/` verlegt ist (PLAN AP 6).

## Regeln für diesen Crate
- Abhängigkeiten nur nach unten: `level(A) > level(B)`; keine Querkanten (G1).
- Keine Alias-Re-Exports, kein `include!`, kein `#[deprecated]` (HR-6).
- Ein Fehlertyp je Crate, `thiserror`, `#[non_exhaustive]`, kein `Other(String)` (HR-4).
- Workspace-Lints gelten (`[lints] workspace = true`): kein `unwrap`, `expect`, `panic`, Indexierung mit `[]` auf Produktionspfaden.
- `unsafe` ist hier verboten (G-UNSAFE, nur `acta-sys` darf es).
- Keine Zeit- oder Zufallsquellen außerhalb der injizierten Ports `Clock` und `CryptoRng` (G3).
- Verbotene Formulierungen aus SPEC 1.6 (Liste A) nicht verwenden (G-WORD).
- Vor Abschluss: `cargo xtask gates`, `cargo test -p acta-vec`, `cargo clippy -p acta-vec --all-targets -- -D warnings`, `cargo fmt --all -- --check`.

## Vorgehen bei Überführung aus dem Archiv
1. Archivdateien lesen, nicht blind kopieren. Namen, Importe und Verweise auf frühere Spezifikationsstände sind auf Acta umzuschreiben (siehe `archive/INDEX.md`).
2. Jede Funktion gegen den Lebenszyklus in SPEC Kap. 19 und die Zusagen in 1.5 und 16.3 prüfen. Funktionen ohne Eintrag dort werden nicht implementiert (SPEC 0.4 Regel 3).
3. Tests mit Positivkontrolle mitführen (SPEC 16.4).
4. Nach vollständiger, grün geprüfter Überführung wird der Archiv-Ordner dieses Crates entfernt.
