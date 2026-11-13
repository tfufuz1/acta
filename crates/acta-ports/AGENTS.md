# AGENTS.md — `acta-ports`

> Werkzeug/Crate-Kontext für Agenten. Der obere Teil ist generiert; manuelle Notizen unterhalb ergänzen.

<!-- BEGIN GENERATED: crate_inventory.py -->

## Inventar (generiert)

- **Pfad:** `crates/acta-ports` · **Ring:** 0 · **Version:** 0.1.0
- **Beschreibung:** Canonical dyn-compatible port traits for Acta subsystems
- **src:** 42 Dateien, 4227 LOC · **unsafe:** 0 Vorkommen, Policy `forbid`
- **Interne Abhängigkeiten (normal, 2):** `acta-env`, `acta-types`
- **Interne Dev/Build-Abhängigkeiten (0):** –
- **Externe Abhängigkeiten (normal, 7):** `ahash`, `blake3`, `bytes`, `futures-util`, `serde`, `serde_json`, `thiserror`
- **Reverse-Abhängigkeiten (21):** `acta`, `acta-adapt`, `acta-agent`, `acta-checkpoint`, `acta-cognition`, `acta-crypto`, `acta-db`, `acta-graph`, `acta-infer-candle`, `acta-infer-ollama`, `acta-infer-onnx`, `acta-kvcache`, `acta-license`, `acta-mcp`, `acta-mvcc`, `acta-rank`, `acta-router`, `acta-store`, `acta-testkit`, `acta-text`, `acta-vector`
- **Kopplung an Hub-Crates:** ausgehend `acta-types`; eingehend `acta`, `acta-db`
- **Features (4):** `conformance`, `contract-tests`, `default`, `testkit`

### Öffentliche API (grob, aus src/)
- struct (47): `Armed`, `AuthorizedRing`, `BarrierExclusive`, `BarrierShared`, `CompactionPermit`, `ContextSegment`, `Envelope`, `ErasureEvent`, `FileId`, `FileKey`, `GraphIndexStats`, `GroundingAssessment`, `HyperEdgeId`, `KeyBuf`, `KeyRange` … (+32)
- enum (20): `EgressDecision`, `EmbeddingError`, `EraseTarget`, `FeatureRing`, `FileKind`, `GateDecision`, `GraphMutationError`, `KeyBound`, `KeyCoverage`, `LicenseError`, `ObservedPhase`, `ObserverError`, `PluginError`, `RangeGate`, `ScanTechnique` … (+5)
- trait (43): `AttentionExporter`, `Checkpoint`, `CheckpointCoordinator`, `CommitBarrier`, `CommunityResolver`, `ContextPreparer`, `DriftStatusProvider`, `EgressPolicy`, `EmbeddingProvider`, `ErasureLayer`, `ErasureObserver`, `ErasureTarget`, `FileKeyring`, `GraphCollectionMutation`, `GraphIndex` … (+28)
- fn (64): `activate_all`, `affected_units`, `as_bytes`, `as_str`, `as_u64`, `assert_attention_exporter_contract`, `assert_checkpoint_contract`, `assert_checkpoint_coordinator_contract`, `assert_egress_policy_contract`, `assert_embedding_contract`, `assert_erasure_layer_contract`, `assert_graph_index_contract`, `assert_keys_contract`, `assert_ledger_contract`, `assert_license_contract` … (+49)
- type (4): `BoxFuture`, `BoxStream`, `SignerId`, `Timestamp`
- const (1): `MAX_SCAN_MERGE_ACCUMULATOR`
- mod (43): `attention`, `barrier`, `check`, `checkpoint`, `clock`, `conformance`, `contract_tests`, `embedding`, `env`, `erasure`, `erasure_legacy`, `fs`, `gate`, `graph`, `graph_index` … (+28)
- pub use: `attention::*`, `barrier::{ BarrierExclusive, BarrierShared, CommitBarrier, CompactionPermit, PurgeBarrier, PurgePermit, }`, `checkpoint::*`, `clock::*`, `conformance::*`, `acta_env::*`, `acta_env::clock::*`, `acta_env::id_gen::*`, `acta_env::metrics::*`, `acta_env::rng::*` … (+59)

Regeneriert mit `python3 tools/crate_inventory.py agents`. Manuelle Notizen stehen außerhalb dieses Blocks.

<!-- END GENERATED: crate_inventory.py -->
