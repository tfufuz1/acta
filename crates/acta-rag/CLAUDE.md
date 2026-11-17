# acta-rag

acta-rag: Abrufpipeline: Hybridsuche, RRF, Reranking, Kontextaufbau, Belegpruefung, Eval.

> Teil des Acta-Workspace. Verbindliche Vorgaben: Wurzel-`CLAUDE.md` und `docs/SPEC.md`.

## Einordnung
- **Level:** 4 (`rings.toml`)
- **Größenbudget:** 7000 LOC Produktionscode (G-HUB); Überschreitung nur mit ADR
- **Aufgabe laut SPEC 3.3:** Abrufpipeline: Hybridsuche, Fusion, Reranking, Kontextaufbau, Belegprüfung, Bewertungswerkzeug; implementiert `ErasureLayer` für Antwort- und Sitzungs-Caches
- **Erlaubte interne Abhängigkeiten (SPEC 3.3):** base, crypto, ledger, text, vec, history, pstore, gate, access, infer
- **Ist-Abhängigkeiten (Cargo.toml):** acta-access, acta-base, acta-crypto, acta-gate, acta-history, acta-infer, acta-ledger, acta-pstore, acta-text, acta-vec
- **Externe Abhängigkeiten:** thiserror
- **Dev-Abhängigkeiten (zählen nicht für G1):** acta-testkit, proptest
- **Referenz im Quelltext:** Level L4 · Budget 7000 LOC · Spezifikation docs/SPEC.md Kap. 9.4-9.7

## Status
- **Stand:** Teil. Umgesetzt sind die Rechenkerne des Rank-Moduls, neu nach SPEC gebaut (nicht Archivcode kopiert):
  - `fusion`: RRF in Ganzzahl (`floor(2^32 / (rrf_k + rang))`, Summe über Listen), Gleichstand Score absteigend, Schlüssel aufsteigend; Auswahl nach Reranker mit `max_chunks_per_doc` und `rerank_keep` (SPEC 9.4).
  - `validity`: Gültigkeitszeit `[valid_from, valid_to)` mit `Option`-Präsenz statt `0` (SPEC 6.3, Filter `valid_at` in 9.4).
  - `metrics`: Recall@k, MRR, nDCG über `fx_ln` in Q32.32, Latenzperzentile nach Nächster-Rang-Verfahren (SPEC 9.7).
  - `quality`: Regressionsgrenzen nach Anhang A (Recall@10 −0,02; Zitiergenauigkeit −0,02; Enthaltungsgenauigkeit −0,03; Latenz p95 +20 %) und Status `QualityDrift`, der die Aktivierung verhindert (SPEC 9.7, 14).
- **Nicht umgesetzt (kein Spezifikationseintrag, SPEC 0.4 Regel 3):** Platt-, Isotonic- und Conformal-Kalibrierung, Drift über Score-Verteilungen, DiBud, Erklärungsbericht, Resonanz-Bonus, die alte `ScoredItem`-API mit `f32`-Scores.
- **Offen (abhängig von anderen Crates oder Neubau):** Hybridsuche (BM25 aus `acta-text`, Vektor aus `acta-vec`, PPR aus `acta-graph`), Kontextaufbau, Belegprüfung (Z17), Reranker-Anbindung über `acta-infer`, `ErasureLayer`.
- **Produktionscode (ohne Tests/Leerzeilen/Kommentare):** 365 LOC (Budget 7000).
- **Tests:** 32 Unit-Tests, 6 Property-Tests (proptest, je 256 Fälle), Positivkontrolle durch gezielte Mutationen geprüft (Tie-Break und Latenzgrenze werden erkannt).
- **Archiv-Quelle:** `archive/acta-rag/rank/` bleibt bis zur vollständigen Überführung erhalten (siehe `archive/acta-rag/MANIFEST.md`). Der Archivordner wird erst nach vollständiger Überführung entfernt.

## Regeln für diesen Crate
- Abhängigkeiten nur nach unten: `level(A) > level(B)`; keine Querkanten (G1).
- Keine Alias-Re-Exports, kein `include!`, kein `#[deprecated]` (HR-6).
- Ein Fehlertyp je Crate, `thiserror`, `#[non_exhaustive]`, kein `Other(String)` (HR-4).
- Workspace-Lints gelten (`[lints] workspace = true`): kein `unwrap`, `expect`, `panic`, Indexierung mit `[]` auf Produktionspfaden.
- `unsafe` ist hier verboten (G-UNSAFE, nur `acta-sys` darf es).
- Keine Zeit- oder Zufallsquellen außerhalb der injizierten Ports `Clock` und `CryptoRng` (G3).
- Verbotene Formulierungen aus SPEC 1.6 (Liste A) nicht verwenden (G-WORD).
- Vor Abschluss: `cargo xtask gates`, `cargo test -p acta-rag`, `cargo clippy -p acta-rag --all-targets -- -D warnings`, `cargo fmt --all -- --check`.

## Vorgehen bei Überführung aus dem Archiv
1. Archivdateien lesen, nicht blind kopieren. Namen, Importe und Verweise auf frühere Spezifikationsstände sind auf Acta umzuschreiben (siehe `archive/INDEX.md`).
2. Jede Funktion gegen den Lebenszyklus in SPEC Kap. 19 und die Zusagen in 1.5 und 16.3 prüfen. Funktionen ohne Eintrag dort werden nicht implementiert (SPEC 0.4 Regel 3).
3. Tests mit Positivkontrolle mitführen (SPEC 16.4).
4. Nach vollständiger, grün geprüfter Überführung wird der Archiv-Ordner dieses Crates entfernt.
