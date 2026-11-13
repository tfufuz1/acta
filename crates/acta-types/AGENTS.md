# AGENTS.md — `acta-types`

> Werkzeug/Crate-Kontext für Agenten. Der obere Teil ist generiert; manuelle Notizen unterhalb ergänzen.

<!-- BEGIN GENERATED: crate_inventory.py -->

## Inventar (generiert)

- **Pfad:** `crates/acta-types` · **Ring:** 0 · **Version:** 0.1.0
- **Beschreibung:** Canonical domain types, IDs, budgets, filters, and error types for Acta
- **src:** 25 Dateien, 5281 LOC · **unsafe:** 0 Vorkommen, Policy `forbid`
- **Interne Abhängigkeiten (normal, 1):** `acta-ids`
- **Interne Dev/Build-Abhängigkeiten (0):** –
- **Externe Abhängigkeiten (normal, 8):** `ahash`, `bincode`, `blake3`, `bytes`, `serde`, `serde_json`, `sha2`, `thiserror`
- **Reverse-Abhängigkeiten (28):** `acta`, `acta-adapt`, `acta-agent`, `acta-audit-export`, `acta-bench`, `acta-checkpoint`, `acta-cognition`, `acta-crypto`, `acta-db`, `acta-graph`, `acta-infer-candle`, `acta-infer-ollama`, `acta-infer-onnx`, `acta-ingest`, `acta-kvcache`, `acta-license`, `acta-mcp`, `acta-mvcc`, `acta-ports`, `acta-py`, `acta-rank`, `acta-router`, `acta-simd`, `acta-store`, `acta-testkit`, `acta-text`, `acta-vault`, `acta-vector`
- **Kopplung an Hub-Crates:** ausgehend –; eingehend `acta`, `acta-db`, `acta-ports`
- **Features (3):** `auto-extraction-opt-out`, `default`, `docid-128`

### Öffentliche API (grob, aus src/)
- struct (42): `Armed`, `ConfigFingerprint`, `ContextChunk`, `ContextWindow`, `ActaErrorDto`, `ControlEvidence`, `Edge`, `Embedding`, `Entity`, `EraseReceipt`, `EraseRequest`, `ErasureId`, `FusionWeights`, `GateId`, `HybridQuery` … (+27)
- enum (32): `AutoExtractionMode`, `BudgetStrategy`, `CacheDirective`, `ActaError`, `DecayFunction`, `DistanceMetric`, `DocIdWidth`, `EraseMode`, `EraseTarget`, `ErasureStatus`, `FilterExpr`, `FusionStrategy`, `GateVariant`, `GraphTraversalStrategy`, `HnswDeletionError` … (+17)
- trait (1): `TombstoneSemanticsCheck`
- fn (108): `arm`, `as_bytes`, `as_metadata_key`, `as_slice`, `as_u8`, `available`, `budget`, `build`, `builder`, `bytes`, `bytes_len`, `capability_unsupported`, `checksum_mismatch`, `class`, `combined_text_owned` … (+93)
- type (3): `QueryOptions`, `QueryOptionsBuilder`, `Result`
- const (2): `EXPIRY_METADATA_KEY`, `MAX_SEARCH_K`
- mod (17): `budget`, `domain`, `erasure`, `error`, `error_dto`, `filter`, `ids`, `importance`, `model_fingerprint`, `refs`, `retrieval_strategy`, `saos`, `schema`, `tenant_scope`, `tombstone` … (+2)
- pub use: `budget::*`, `acta_ids::*`, `acta_ids::TOMBSTONE_BIT`, `acta_ids::{ CollectionRef, DocKey, DocRef, OperatorRef, SubjectRef, TenantRef, TenantScoped, MAX_REF_LEN, }`, `acta_ids::{CollectionId, DocId, EntityId, IdError, TenantId, TxId, ID_WIDTH_BITS}`, `acta_ids::{TenantScopeViolation, TenantScoped}`, `document::*`, `domain::*`, `erasure::*`, `error::{ActaError, HnswDeletionError, Result}` … (+15)

Regeneriert mit `python3 tools/crate_inventory.py agents`. Manuelle Notizen stehen außerhalb dieses Blocks.

<!-- END GENERATED: crate_inventory.py -->
