# AGENTS.md — acta-engine

## Scope & Purpose
`acta-engine` is a Ring 3 crate in Acta Cognitive OS. It provides the core storage, vector/text/graph indexing, transaction orchestration (`DbTransaction`), and collection management (`Collection`, `Acta`).

## Strict Invariants
- `#![forbid(unsafe_code)]` enforced at crate root (`src/lib.rs`).
- Thread-safe concurrent collection access.
- Monotonic `TxId` allocation.
- Zero-Panic-Doctrine: return `ActaError` via `?`.
