# Copilot-Anweisungen für Acta

Nicht normativ. Normativ ist ausschließlich `docs/SPEC.md` ; bei Widerspruch gilt die SPEC. Gemeinsame Regeln und Fachprofile: `AGENTS.md` im Wurzelverzeichnis und `crates/<crate>/AGENTS.md`.

Acta ist eine eingebettete Rust-Bibliothek (Single-Node, kein Server) für lokales Wissen und Gedächtnis von Agenten- und RAG-Systemen: Dokumente, BM25-Suche, HNSW-Vektorsuche, RRF-Hybridsuche, MVCC-Zeitreisen und ein gemessener, unabhängig prüfbarer Löschprozess. Der Build-Status ändert sich; ermittle ihn mit `cargo check --workspace --all-targets --locked`, nimm ihn nicht aus Dokumenten an.

## Regeln

- `unsafe` nur in `acta-sys` und `acta-simd`, jeweils mit `// SAFETY:`. Alle anderen Crates: `#![forbid(unsafe_code)]`. Kein `mmap`.
- Kein `unwrap`, `expect`, `panic!`, `todo!`, `unimplemented!`, `unreachable!`, kein `let _ =` auf `Result`, kein ungeprüftes Indexieren im Produktionscode.
- Zeit, Zufall und IDs nur über `Clock`, `Rng`, `IdGen` aus `acta-ports`.
- I/O nur über den Port `Vfs`. Ein `fsync`-Fehler wird propagiert und nie auf demselben Handle wiederholt.
- Der Mandant kommt immer aus dem `TenantContext`, nie als Literal oder Default.
- Verschlüsselung und WAL-Integrität sind nicht optional. Kein `cfg(feature)`-Schalter, kein Laufzeitparameter dafür.
- Dokumente werden mit `DocRef` adressiert; `DocId(u64)` gibt es nicht.
- Jeder Test trägt `#[ctx_test(id = "T-…")]` mit einer SPEC-ID.
- Keine Doppeldefinitionen öffentlicher Typen. Kanonische Definition vor dem Hinzufügen suchen (`docs/TYPE_REGISTRY.md`).
- Aussagen über Löschung nur im Wortlaut aus SPEC 6.4, auch in Kommentaren und Fehlertexten.

## Ringe

| Ring | Crates |
|---|---|
| 0 | `acta-types`, `acta-ports` |
| 1 | `acta-sys`, `acta-simd`, `acta-durable-fs`, `acta-crypto` |
| 2 | `acta-mvcc`, `acta-store`, `acta-text`, `acta-vector`, `acta-rank` |
| 3 | `acta-erasure`, `acta-engine` |
| 4 | `acta` |
| dev | `acta-testkit`, `acta-testmacros`, `acta-lab`, `ctx-verify`, `tests/arch` |

Abhängigkeiten zeigen nur auf gleiche oder niedrigere Ringe; innerhalb von Ring 2 und für `erasure`, `engine`, `acta` gilt die Matrix SPEC 5.3. `acta-erasure` kennt keine Ring-2-Crates. `acta-vector` hängt weder an `sys` noch an `mvcc`. `lab` und `ctx-verify` nutzen keine Produktions-Decoder.

## Nicht vorschlagen

Diese Crates sind entfernt und werden nicht neu angelegt oder referenziert: `acta-graph`, `acta-cognition`, `acta-adapt`, `acta-kvcache`, `acta-router`, `acta-agent`, `acta-infer-*`, `acta-mcp`, `acta-py`, `acta-privacy`, `acta-ingest`, `acta-sandbox`, `acta-wire`, `acta-checkpoint`, `acta-license`, `acta-audit-export`, `acta-avv-generator`, `acta-core`, `acta-db`. Ebenso entfallen: Lizenz-Gate, Embedder, Reranker, Query-Rewriter, automatische Extraktion, Windows-Code, `WasmMergeOperator` (SPEC D-24, D-84).
