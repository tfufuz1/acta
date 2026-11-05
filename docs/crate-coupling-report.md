# Crate-Kopplungsbericht

- Erzeugt: 2026-11-06 00:15
- Workspace-Root: `/home/freddy/Projekte/acta`
- Analysierte Crates: 21 · Komponenten-Tiefe: 1 · Zeilen (ohne Leerzeilen): 191.304
- Legende: **prod** = src/bin/build ohne `cfg(test)`; **dev** = tests/benches/examples/fuzz und `cfg(test)`-Code.
- Hinweis: Textuelle Analyse ohne Compiler. Glob-Importe (`*`) sind nicht auflösbar. Siehe Kopf des Skripts für Grenzen.

## 1. Überblick

| Crate | Ring | Zeilen | Fan-out prod | Fan-in prod | Cargo-Deps intern (n/d/b) | Pub-Items | API-Lecks | Zyklus |
|---|---|---:|---:|---:|---|---:|---:|---|
| [acta-core](#acta-core) | 0 | 322 | 3 | 4 | 3/0/0 | 0 | 13 | – |
| [acta-crypto](#acta-crypto) | 0 | 15950 | 2 | 5 | 2/1/0 | 73 | 4 | – |
| [acta-crypto-fuzz](#acta-crypto-fuzz) | – | 356 | 1 | 0 | 2/0/0 | 0 | 0 | – |
| [acta-db](#acta-db) | 3 | 12443 | 5 | 1 | 10/1/0 | 13 | 9 | – |
| [acta-db-fuzz](#acta-db-fuzz) | – | 51 | 1 | 0 | 2/0/0 | 0 | 0 | – |
| [acta-engine](#acta-engine) | 3 | 24027 | 9 | 1 | 10/3/0 | 81 | 28 | – |
| [acta-graph](#acta-graph) | 0 | 25811 | 2 | 2 | 2/1/0 | 125 | 23 | – |
| [acta-mcp](#acta-mcp) | 4 | 9821 | 4 | 1 | 4/0/0 | 36 | 8 | – |
| [acta-mcp-fuzz](#acta-mcp-fuzz) | – | 91 | 1 | 0 | 1/0/0 | 0 | 0 | – |
| [acta-mvcc](#acta-mvcc) | 0 | 4572 | 2 | 3 | 2/0/0 | 22 | 1 | – |
| [acta-ports](#acta-ports) | 0 | 3602 | 1 | 10 | 1/0/0 | 65 | 2 | – |
| [acta-rank](#acta-rank) | 0 | 6069 | 1 | 2 | 2/0/0 | 21 | 2 | – |
| [acta-simd](#acta-simd) | 0 | 1472 | 1 | 1 | 1/0/0 | 53 | 13 | – |
| [acta-store](#acta-store) | 1 | 46092 | 5 | 2 | 6/2/0 | 105 | 6 | – |
| [acta-store-fuzz](#acta-store-fuzz) | – | 660 | 2 | 0 | 2/0/0 | 0 | 0 | – |
| [acta-sys](#acta-sys) | 0 | 632 | 0 | 3 | 0/0/0 | 14 | 0 | – |
| [acta-text](#acta-text) | 0 | 10725 | 1 | 2 | 2/0/0 | 42 | 3 | – |
| [acta-text-fuzz](#acta-text-fuzz) | – | 10 | 1 | 0 | 1/0/0 | 0 | 0 | – |
| [acta-types](#acta-types) | 0 | 6761 | 0 | 10 | 0/0/0 | 65 | 0 | – |
| [acta-vector](#acta-vector) | 0 | 21641 | 6 | 2 | 6/0/0 | 29 | 0 | – |
| [acta-vector-fuzz](#acta-vector-fuzz) | – | 196 | 1 | 0 | 2/0/0 | 0 | 0 | – |

## 2. Workspace-weite Auffälligkeiten

### 2.1 Zyklen

- Kein Zyklus im prod-Code-Graphen gefunden.
- Kein Zyklus in den normalen Cargo-Abhängigkeiten.

### 2.2 Abweichungen Cargo.toml ↔ Code

| Crate | Ziel | Deklariert als | Befund |
|---|---|---|---|
| `acta-crypto` | `acta-crypto` | dev | kein Code-Zugriff gefunden (prüfen: Makro, Feature, cfg, oder entfernen) |
| `acta-crypto-fuzz` | `acta-types` | normal | kein Code-Zugriff gefunden (prüfen: Makro, Feature, cfg, oder entfernen) |
| `acta-db` | `acta-crypto` | normal | nur in Tests/Benches genutzt, aber als normale Abhängigkeit deklariert → dev-dependency? |
| `acta-db` | `acta-rank` | normal | nur in Tests/Benches genutzt, aber als normale Abhängigkeit deklariert → dev-dependency? |
| `acta-db` | `acta-store` | normal | nur in Tests/Benches genutzt, aber als normale Abhängigkeit deklariert → dev-dependency? |
| `acta-db` | `acta-text` | normal | nur in Tests/Benches genutzt, aber als normale Abhängigkeit deklariert → dev-dependency? |
| `acta-db` | `acta-vector` | dev/normal | nur in Tests/Benches genutzt, aber als normale Abhängigkeit deklariert → dev-dependency? |
| `acta-db-fuzz` | `acta-core` | normal | kein Code-Zugriff gefunden (prüfen: Makro, Feature, cfg, oder entfernen) |
| `acta-engine` | `acta-core` | dev | kein Code-Zugriff gefunden (prüfen: Makro, Feature, cfg, oder entfernen) |
| `acta-engine` | `acta-sys` | normal | kein Code-Zugriff gefunden (prüfen: Makro, Feature, cfg, oder entfernen) |
| `acta-rank` | `acta-ports` | normal | kein Code-Zugriff gefunden (prüfen: Makro, Feature, cfg, oder entfernen) |
| `acta-store` | `acta-graph` | dev | kein Code-Zugriff gefunden (prüfen: Makro, Feature, cfg, oder entfernen) |
| `acta-store` | `acta-types` | normal | nur in Tests/Benches genutzt, aber als normale Abhängigkeit deklariert → dev-dependency? |
| `acta-store` | `acta-vector` | dev | kein Code-Zugriff gefunden (prüfen: Makro, Feature, cfg, oder entfernen) |
| `acta-text` | `acta-types` | normal | nur in Tests/Benches genutzt, aber als normale Abhängigkeit deklariert → dev-dependency? |
| `acta-vector-fuzz` | `acta-core` | normal | kein Code-Zugriff gefunden (prüfen: Makro, Feature, cfg, oder entfernen) |

### 2.3 Schwache Kanten (billig zu trennen)

Kanten im prod-Code mit höchstens 3 verschiedenen Symbolen. Reihenfolge: wenigste Symbole, dann wenigste Fundstellen.

| Von | Nach | Symbole | Fundstellen | Dateien | Symbole (prod) |
|---|---|---:|---:|---:|---|
| `acta-db` | `acta-graph` | 1 | 1 | 1 | `percolation::PercolationConfig` |
| `acta-db` | `acta-ports` | 1 | 1 | 1 | `QueryRewriter` |
| `acta-db` | `acta-sys` | 1 | 1 | 1 | `LockedRegions` |
| `acta-engine` | `acta-mvcc` | 1 | 1 | 1 | `tx_buffer::TxBuffer` |
| `acta-mvcc` | `acta-types` | 1 | 1 | 1 | `*` |
| `acta-text-fuzz` | `acta-text` | 1 | 1 | 1 | `tokenizer::tokenize` |
| `acta-vector` | `acta-types` | 1 | 1 | 1 | `SaturatingU16::from` |
| `acta-core` | `acta-ports` | 1 | 2 | 1 | `*`×2 |
| `acta-crypto` | `acta-ports` | 1 | 2 | 2 | `Clock`×2 |
| `acta-vector` | `acta-sys` | 1 | 2 | 2 | `mmap_readonly`×2 |
| `acta-vector-fuzz` | `acta-vector` | 1 | 2 | 2 | `hnsw`×2 |
| `acta-mvcc` | `acta-ports` | 1 | 5 | 1 | `MetricsSink`×5 |
| `acta-text` | `acta-ports` | 1 | 5 | 3 | `StorageEngine`×5 |
| `acta-vector` | `acta-crypto` | 1 | 6 | 2 | `wal_crypto::WalHmac::new`×6 |
| `acta-db` | `acta-types` | 2 | 2 | 2 | `DistanceMetric`, `MAX_SEARCH_K` |
| `acta-db-fuzz` | `acta-db` | 2 | 2 | 1 | `fusion::weighted_reciprocal_rank_fusion`, `SearchResult` |
| `acta-mcp-fuzz` | `acta-mcp` | 2 | 2 | 2 | `protocol`, `prompt_injection` |
| `acta-store-fuzz` | `acta-core` | 2 | 2 | 2 | `StorageEngine`, `TxId` |
| `acta-crypto-fuzz` | `acta-crypto` | 2 | 3 | 3 | `deletion_proof`×2, `wal_crypto` |
| `acta-engine` | `acta-text` | 2 | 3 | 2 | `Language`×2, `inverted::InvertedIndex` |
| `acta-store` | `acta-sys` | 2 | 3 | 2 | `mmap_readonly`×2, `set_restrictive_file_acl` |
| `acta-vector` | `acta-simd` | 2 | 3 | 3 | `validate_vector`×2, `kernels::scalar` |
| `acta-simd` | `acta-core` | 2 | 5 | 2 | `Result`×4, `ActaError` |
| `acta-engine` | `acta-crypto` | 2 | 6 | 2 | `TenantIsolatedKvStore`×5, `deletion_proof` |
| `acta-mcp` | `acta-rank` | 3 | 3 | 2 | `fusion::ProvenanceRecord`, `IsotonicCalibrator`, `IsotonicCalibrator::with_defaults` |
| `acta-engine` | `acta-vector` | 3 | 5 | 5 | `hnsw::HnswIndex`×2, `HnswIndex`×2, `acorn::FilteredIndex` |
| `acta-core` | `acta-mvcc` | 3 | 6 | 1 | `seq_log`×2, `snapshot`×2, `tx_buffer`×2 |

**Nur in Tests/Benches/Beispielen** (für das Produkt irrelevant, aber sie koppeln den Test-Build):

| Von | Nach | Fundstellen | Symbole |
|---|---|---:|---|
| `acta-db` | `acta-crypto` | 4 | `CryptoError`, `deletion_proof`, `deletion_proof::GraphRepairAttestation`, `deletion_proof::DeletionProof` |
| `acta-db` | `acta-rank` | 1 | `IsotonicCalibrator::new` |
| `acta-db` | `acta-store` | 12 | `LsmConfig`×5, `LsmStorage::new`×4, `LsmStorage`×2, `lsm` |
| `acta-db` | `acta-text` | 11 | `Language::English`×7, `Language`×2, `bm25::score_term`, `inverted::InvertedIndex` |
| `acta-db` | `acta-vector` | 6 | `HnswConfig`×3, `HnswIndex`×2, `HnswIndex::try_new` |
| `acta-graph` | `acta-store` | 2 | `LsmConfig`, `LsmStorage::new` |
| `acta-store` | `acta-types` | 2 | `TxId`, `TenantId` |
| `acta-text` | `acta-types` | 132 | `Result`×67, `TOMBSTONE_BIT`×52, `TxId`×7, `TxId::new`×2 … (+3) |

### 2.4 Hub-Symbole (Kandidaten für types/ports)

Symbole, die aus einem Nicht-Basis-Crate von mindestens 3 verschiedenen Crates im prod-Code importiert werden. Wer sie in ein schmales Basis-Crate verschiebt, kann viele Kanten kappen.

| Quelle | Symbol | Nutzer | Crates |
|---|---|---:|---|
| `acta-core` | `Result` | 3 | `acta-simd`, `acta-store`, `acta-vector` |

### 2.5 API-Lecks

Öffentliche Items, deren Signatur oder öffentliche Felder Typen anderer Workspace-Crates enthalten, sowie `pub use` fremder Crates. Diese Stellen machen eine Entkopplung am schwersten, weil Nutzer des Crates die fremde Abhängigkeit mitsehen.

| Crate | Komponente | Item | Fremde Crates | Fundstelle |
|---|---|---|---|---|
| `acta-core` | `(crate-root)` | `pub use acta_types::error` | `acta-types` | `src/lib.rs:15` |
| `acta-core` | `(crate-root)` | `pub use acta_types::error_dto` | `acta-types` | `src/lib.rs:24` |
| `acta-core` | `(crate-root)` | `pub use acta_types::model_fingerprint` | `acta-types` | `src/lib.rs:29` |
| `acta-core` | `(crate-root)` | `pub use acta_types::schema` | `acta-types` | `src/lib.rs:34` |
| `acta-core` | `(crate-root)` | `pub use acta_types::tombstone` | `acta-types` | `src/lib.rs:39` |
| `acta-core` | `(crate-root)` | `pub use acta_types::types` | `acta-types` | `src/lib.rs:44` |
| `acta-core` | `(crate-root)` | `pub use acta_ports::*` | `acta-ports` | `src/lib.rs:56` |
| `acta-core` | `(crate-root)` | `pub use acta_mvcc::seq_log` | `acta-mvcc` | `src/lib.rs:59` |
| `acta-core` | `(crate-root)` | `pub use acta_mvcc::snapshot` | `acta-mvcc` | `src/lib.rs:64` |
| `acta-core` | `(crate-root)` | `pub use acta_mvcc::tx_buffer` | `acta-mvcc` | `src/lib.rs:69` |
| `acta-core` | `(crate-root)` | `pub use acta_ports::*` | `acta-ports` | `src/lib.rs:102` |
| `acta-core` | `(crate-root)` | `pub use acta_types::error_dto::ActaErrorDto` | `acta-types` | `src/lib.rs:114` |
| `acta-core` | `(crate-root)` | `pub use acta_types::model_fingerprint::ModelFingerprint` | `acta-types` | `src/lib.rs:120` |
| `acta-crypto` | `kv_cipher` | `struct EncryptedKvLayer` | `acta-types` | `src/kv_cipher.rs:40` |
| `acta-crypto` | `kv_cipher` | `pub use acta_types::ModelFingerprint` | `acta-types` | `src/kv_cipher.rs:30` |
| `acta-crypto` | `kv_segment::segment` | `struct KvSegment` | `acta-types` | `src/kv_segment/segment.rs:34` |
| `acta-crypto` | `kv_segment::store` | `type SpillHandler` | `acta-types` | `src/kv_segment/store.rs:17` |
| `acta-db` | `(crate-root)` | `pub use acta_engine` | `acta-engine` | `src/lib.rs:8` |
| `acta-db` | `(crate-root)` | `pub use acta_engine::MetadataFilter` | `acta-engine` | `src/lib.rs:11` |
| `acta-db` | `(crate-root)` | `pub use acta_engine::collection::maintenance::PercolationResult` | `acta-engine` | `src/lib.rs:23` |
| `acta-db` | `(crate-root)` | `pub use acta_engine::SandboxBridge` | `acta-engine` | `src/lib.rs:25` |
| `acta-db` | `(crate-root)` | `pub use acta_engine::background_workers::*` | `acta-engine` | `src/lib.rs:69` |
| `acta-db` | `(crate-root)` | `pub use acta_engine::fusion` | `acta-engine` | `src/lib.rs:79` |
| `acta-db` | `(crate-root)` | `pub use acta_graph::percolation::PercolationConfig` | `acta-graph` | `src/lib.rs:106` |
| `acta-db` | `(crate-root)` | `pub use acta_types::DistanceMetric` | `acta-types` | `src/lib.rs:110` |
| `acta-db` | `multistep` | `pub use acta_ports::QueryRewriter` | `acta-ports` | `src/multistep.rs:9` |
| `acta-engine` | `background_workers::expiry_workers` | `fn start_decay_cleanup_worker` | `acta-ports` | `src/background_workers/expiry_workers.rs:67` |
| `acta-engine` | `collection::crud::auto_extraction` | `pub use acta_types::AutoExtractionMode` | `acta-types` | `src/collection/crud/auto_extraction.rs:11` |
| `acta-engine` | `collection` | `fn compute_default_importance` | `acta-types` | `src/collection/mod.rs:115` |
| `acta-engine` | `collection` | `struct Collection` | `acta-store`, `acta-vector` | `src/collection/mod.rs:288` |
| `acta-engine` | `collection::query_builder::scope` | `struct ScopeConstraint` | `acta-types` | `src/collection/query_builder/scope.rs:12` |
| `acta-engine` | `collection::query_builder::strategy` | `enum SearchStrategy` | `acta-types` | `src/collection/query_builder/strategy.rs:4` |
| `acta-engine` | `extraction::open_ie` | `fn extract_triples` | `acta-ports` | `src/extraction/open_ie.rs:73` |
| `acta-engine` | `fusion` | `fn fuse_search_results_with_strategy` | `acta-types` | `src/fusion.rs:16` |
| `acta-engine` | `fusion` | `fn fuse_search_results_with_signal_strategies` | `acta-types` | `src/fusion.rs:35` |
| `acta-engine` | `(crate-root)` | `type ConsolidationLauncher` | `acta-store` | `src/lib.rs:408` |
| `acta-engine` | `(crate-root)` | `fn register_consolidation_launcher` | `acta-store` | `src/lib.rs:422` |
| `acta-engine` | `(crate-root)` | `struct ActaStats` | `acta-ports` | `src/lib.rs:453` |
| `acta-engine` | `(crate-root)` | `struct ActaConfig` | `acta-ports`, `acta-store`, `acta-types` | `src/lib.rs:511` |
| `acta-engine` | `(crate-root)` | `type TenantCollectionMap` | `acta-store` | `src/lib.rs:642` |
| `acta-engine` | `(crate-root)` | `pub use acta_ports::TextEmbeddingEngine` | `acta-ports` | `src/lib.rs:328` |
| `acta-engine` | `(crate-root)` | `pub use acta_ports::VectorDeleteMode` | `acta-ports` | `src/lib.rs:329` |
| `acta-engine` | `(crate-root)` | `pub use acta_store::lsm::DurabilityMode` | `acta-store` | `src/lib.rs:330` |
| `acta-engine` | `(crate-root)` | `pub use acta_graph::percolation::PercolationConfig` | `acta-graph` | `src/lib.rs:401` |
| `acta-engine` | `(crate-root)` | `pub use acta_text::Language` | `acta-text` | `src/lib.rs:403` |
| `acta-engine` | `(crate-root)` | `pub use acta_ports::DriftStatusProvider` | `acta-ports` | `src/lib.rs:467` |
| `acta-engine` | `(crate-root)` | `pub use acta_types::DistanceMetric` | `acta-types` | `src/lib.rs:678` |
| `acta-engine` | `metrics_names` | `fn record_histogram_safe` | `acta-ports` | `src/metrics_names.rs:27` |
| `acta-engine` | `metrics_names` | `fn record_gauge_safe` | `acta-ports` | `src/metrics_names.rs:39` |
| `acta-engine` | `metrics_names` | `fn record_counter_safe` | `acta-ports` | `src/metrics_names.rs:51` |
| `acta-engine` | `temporal_filter` | `struct ValidityWindow` | `acta-types` | `src/temporal_filter.rs:18` |
| `acta-engine` | `temporal_filter` | `fn apply_temporal_validity_filter` | `acta-types` | `src/temporal_filter.rs:159` |
| `acta-engine` | `transaction::cleanup` | `fn cleanup_orphaned_consolidation_intents` | `acta-ports` | `src/transaction/cleanup.rs:7` |
| `acta-engine` | `transaction::db_transaction` | `struct DbTransaction` | `acta-vector` | `src/transaction/db_transaction.rs:11` |
| `acta-graph` | `apprh::diffusion` | `fn run_apprh_push` | `acta-types` | `src/apprh/diffusion.rs:22` |
| `acta-graph` | `apprh` | `fn apprh_local` | `acta-types` | `src/apprh/mod.rs:34` |
| `acta-graph` | `apprh` | `fn forward_push_apprh` | `acta-types` | `src/apprh/mod.rs:49` |
| `acta-graph` | `apprh::shadow` | `struct ApprhShadowComparison` | `acta-types` | `src/apprh/shadow.rs:28` |
| `acta-graph` | `apprh::shadow` | `fn shadow_compare_forward_push_vs_apprh` | `acta-types` | `src/apprh/shadow.rs:50` |
| `acta-graph` | `edge_reinforcement_buffer` | `struct CooccurrenceSignal` | `acta-types` | `src/edge_reinforcement_buffer.rs:26` |
| `acta-graph` | `edge_reinforcement_buffer` | `struct TraversalSignal` | `acta-types` | `src/edge_reinforcement_buffer.rs:35` |
| `acta-graph` | `hyperedge_suggest` | `fn validate_candidates_with_llm` | `acta-ports` | `src/hyperedge_suggest.rs:113` |
| `acta-graph` | `(crate-root)` | `trait GraphIndexExt` | `acta-ports` | `src/lib.rs:121` |
| `acta-graph` | `path_rag::k_path` | `struct KPathResult` | `acta-types` | `src/path_rag/k_path.rs:34` |
| `acta-graph` | `path_rag` | `struct GraphPath` | `acta-types` | `src/path_rag/mod.rs:31` |
| `acta-graph` | `path_rag` | `fn forward_push_ppr` | `acta-types` | `src/path_rag/mod.rs:92` |
| `acta-graph` | `path_rag` | `pub use acta_types::EntityId` | `acta-types` | `src/path_rag/mod.rs:25` |
| `acta-graph` | `percolation` | `fn find_rebonding_candidates` | `acta-ports`, `acta-types` | `src/percolation.rs:71` |
| `acta-graph` | `ppr::shadow_hook` | `fn log_tl_hfd_vs_baseline_discrepancy` | `acta-types` | `src/ppr/shadow_hook.rs:9` |
| `acta-graph` | `ppr::shadow_hook` | `fn log_tl_hfd_discrepancy_tracing` | `acta-types` | `src/ppr/shadow_hook.rs:49` |
| `acta-graph` | `ppr_stream` | `struct PprCandidateStream` | `acta-ports` | `src/ppr_stream.rs:21` |
| `acta-graph` | `tl_hfd::diffusion` | `fn run_diffusion` | `acta-types` | `src/tl_hfd/diffusion.rs:27` |
| `acta-graph` | `tl_hfd::lovasz` | `fn truncate_participants` | `acta-types` | `src/tl_hfd/lovasz.rs:13` |
| `acta-graph` | `tl_hfd::lovasz` | `fn compute_lovasz_extension` | `acta-types` | `src/tl_hfd/lovasz.rs:54` |
| `acta-graph` | `tl_hfd` | `fn tl_hfd_local` | `acta-types` | `src/tl_hfd/mod.rs:30` |
| `acta-graph` | `tl_hfd::shadow` | `struct ShadowComparison` | `acta-types` | `src/tl_hfd/shadow.rs:50` |
| `acta-graph` | `tl_hfd::shadow` | `fn shadow_compare_forward_push_vs_tl_hfd` | `acta-types` | `src/tl_hfd/shadow.rs:72` |
| `acta-mcp` | `config` | `fn create_embedding_provider` | `acta-types` | `src/config.rs:128` |
| `acta-mcp` | `config` | `fn create_llm_text_generator` | `acta-types` | `src/config.rs:275` |
| `acta-mcp` | `explain` | `struct ExplainResponse` | `acta-rank` | `src/explain.rs:23` |
| `acta-mcp` | `plugin_status` | `struct PluginStatusResponse` | `acta-ports` | `src/plugin_status.rs:9` |
| `acta-mcp` | `plugin_status` | `struct PluginStatusEntry` | `acta-ports` | `src/plugin_status.rs:16` |
| `acta-mcp` | `plugin_status` | `fn handle_plugin_status` | `acta-ports` | `src/plugin_status.rs:36` |
| `acta-mcp` | `routing` | `struct RoutingHandle` | `acta-ports`, `acta-rank` | `src/routing.rs:106` |
| `acta-mcp` | `server` | `struct McpServer` | `acta-ports` | `src/server.rs:19` |
| `acta-mvcc` | `(crate-root)` | `pub use acta_types::*` | `acta-types` | `src/lib.rs:1` |
| `acta-ports` | `kv` | `struct PrefixKey` | `acta-types` | `src/kv.rs:43` |
| `acta-ports` | `(crate-root)` | `pub use acta_types::*` | `acta-types` | `src/lib.rs:21` |
| `acta-rank` | `fusion::rrf` | `pub use acta_types::FusionStrategy` | `acta-types` | `src/fusion/rrf.rs:10` |
| `acta-rank` | `fusion::types` | `pub use acta_types::OnSignalFailure` | `acta-types` | `src/fusion/types.rs:155` |
| `acta-simd` | `dispatch` | `fn cosine_distance` | `acta-core` | `src/dispatch.rs:8` |
| `acta-simd` | `dispatch` | `fn euclidean_distance` | `acta-core` | `src/dispatch.rs:37` |
| `acta-simd` | `dispatch` | `fn dot_product_distance` | `acta-core` | `src/dispatch.rs:66` |
| `acta-simd` | `dispatch` | `fn cosine_distance_f32_bytes` | `acta-core` | `src/dispatch.rs:95` |
| `acta-simd` | `dispatch` | `fn euclidean_distance_f32_bytes` | `acta-core` | `src/dispatch.rs:117` |
| `acta-simd` | `dispatch` | `fn dot_product_distance_f32_bytes` | `acta-core` | `src/dispatch.rs:139` |
| `acta-simd` | `dispatch` | `fn dot_product_u8` | `acta-core` | `src/dispatch.rs:161` |
| `acta-simd` | `dispatch` | `fn euclidean_distance_sq_u8` | `acta-core` | `src/dispatch.rs:186` |
| `acta-simd` | `dispatch` | `fn cosine_similarity_parts_u8` | `acta-core` | `src/dispatch.rs:208` |
| `acta-simd` | `(crate-root)` | `fn validate_vector` | `acta-core` | `src/lib.rs:32` |
| `acta-simd` | `(crate-root)` | `fn compute_distance` | `acta-core` | `src/lib.rs:42` |
| `acta-simd` | `(crate-root)` | `fn compute_distance_trusted` | `acta-core` | `src/lib.rs:53` |
| `acta-simd` | `(crate-root)` | `fn compute_distance_f32_bytes_trusted` | `acta-core` | `src/lib.rs:67` |
| `acta-store` | `lsm::engine` | `struct LsmStorage` | `acta-mvcc` | `src/lsm/engine.rs:25` |
| `acta-store` | `lsm::observer` | `struct CommitContext` | `acta-core` | `src/lsm/observer.rs:62` |
| `acta-store` | `lsm::tests` | `pub use acta_core::TOMBSTONE_BIT` | `acta-core` | `src/lsm/tests.rs:3` |
| `acta-store` | `wal` | `struct KeyManager` | `acta-crypto` | `src/wal/mod.rs:268` |
| `acta-store` | `wal` | `struct WalConfig` | `acta-crypto` | `src/wal/mod.rs:316` |
| `acta-store` | `wal` | `pub use acta_crypto::crypto::KeyManager` | `acta-crypto` | `src/wal/mod.rs:263` |
| `acta-text` | `(crate-root)` | `struct Bm25Scorer` | `acta-ports` | `src/lib.rs:42` |
| `acta-text` | `stream` | `struct Bm25CandidateStream` | `acta-ports` | `src/stream.rs:27` |
| `acta-text` | `wand` | `fn block_max_wand_search` | `acta-ports` | `src/wand.rs:219` |

## 3. Kantenliste

| Von | Nach | prod Fundstellen | prod Symbole | dev Fundstellen | Cargo-Art |
|---|---|---:|---:|---:|---|
| `acta-engine` | `acta-types` | 252 | 52 | 172 | normal |
| `acta-vector` | `acta-core` | 60 | 11 | 65 | normal |
| `acta-graph` | `acta-types` | 39 | 12 | 53 | normal |
| `acta-store` | `acta-core` | 35 | 14 | 115 | normal |
| `acta-engine` | `acta-ports` | 33 | 16 | 21 | normal |
| `acta-engine` | `acta-store` | 30 | 11 | 106 | normal |
| `acta-crypto` | `acta-types` | 25 | 9 | 14 | normal |
| `acta-engine` | `acta-graph` | 20 | 16 | 76 | normal |
| `acta-graph` | `acta-ports` | 20 | 15 | 40 | normal |
| `acta-mcp` | `acta-ports` | 19 | 10 | 51 | normal |
| `acta-store` | `acta-ports` | 14 | 6 | 10 | normal |
| `acta-vector` | `acta-ports` | 13 | 4 | 10 | normal |
| `acta-core` | `acta-types` | 11 | 8 | 0 | normal |
| `acta-rank` | `acta-types` | 10 | 8 | 9 | normal |
| `acta-mcp` | `acta-types` | 9 | 6 | 4 | normal |
| `acta-ports` | `acta-types` | 9 | 7 | 0 | normal |
| `acta-mcp` | `acta-crypto` | 8 | 4 | 7 | normal |
| `acta-store-fuzz` | `acta-store` | 8 | 6 | 0 | normal |
| `acta-core` | `acta-mvcc` | 6 | 3 | 0 | normal |
| `acta-db` | `acta-engine` | 6 | 6 | 1 | normal |
| `acta-engine` | `acta-crypto` | 6 | 2 | 2 | dev/normal? |
| `acta-engine` | `acta-rank` | 6 | 5 | 0 | normal |
| `acta-vector` | `acta-crypto` | 6 | 1 | 2 | normal |
| `acta-engine` | `acta-vector` | 5 | 3 | 119 | dev/normal |
| `acta-mvcc` | `acta-ports` | 5 | 1 | 2 | normal |
| `acta-simd` | `acta-core` | 5 | 2 | 0 | normal |
| `acta-store` | `acta-crypto` | 5 | 4 | 2 | normal |
| `acta-store` | `acta-mvcc` | 5 | 4 | 1 | normal |
| `acta-text` | `acta-ports` | 5 | 1 | 81 | normal |
| `acta-crypto-fuzz` | `acta-crypto` | 3 | 2 | 0 | normal |
| `acta-engine` | `acta-text` | 3 | 2 | 69 | normal |
| `acta-mcp` | `acta-rank` | 3 | 3 | 0 | normal |
| `acta-store` | `acta-sys` | 3 | 2 | 3 | normal |
| `acta-vector` | `acta-simd` | 3 | 2 | 7 | normal |
| `acta-core` | `acta-ports` | 2 | 1 | 0 | normal |
| `acta-crypto` | `acta-ports` | 2 | 1 | 9 | normal |
| `acta-db` | `acta-types` | 2 | 2 | 47 | normal |
| `acta-db-fuzz` | `acta-db` | 2 | 2 | 0 | normal |
| `acta-mcp-fuzz` | `acta-mcp` | 2 | 2 | 0 | normal |
| `acta-store-fuzz` | `acta-core` | 2 | 2 | 0 | normal |
| `acta-vector` | `acta-sys` | 2 | 1 | 0 | normal |
| `acta-vector-fuzz` | `acta-vector` | 2 | 1 | 0 | normal |
| `acta-db` | `acta-graph` | 1 | 1 | 15 | normal |
| `acta-db` | `acta-ports` | 1 | 1 | 13 | normal |
| `acta-db` | `acta-sys` | 1 | 1 | 0 | normal |
| `acta-engine` | `acta-mvcc` | 1 | 1 | 3 | normal |
| `acta-mvcc` | `acta-types` | 1 | 1 | 3 | normal |
| `acta-text-fuzz` | `acta-text` | 1 | 1 | 0 | normal |
| `acta-vector` | `acta-types` | 1 | 1 | 0 | normal |
| `acta-db` | `acta-crypto` | 0 | 0 | 4 | normal |
| `acta-db` | `acta-rank` | 0 | 0 | 1 | normal |
| `acta-db` | `acta-store` | 0 | 0 | 12 | normal |
| `acta-db` | `acta-text` | 0 | 0 | 11 | normal |
| `acta-db` | `acta-vector` | 0 | 0 | 6 | dev/normal |
| `acta-graph` | `acta-store` | 0 | 0 | 2 | dev |
| `acta-store` | `acta-types` | 0 | 0 | 2 | normal |
| `acta-text` | `acta-types` | 0 | 0 | 132 | normal |

## 4. Crates im Detail

### acta-core

- Pfad: `crates/acta-core` · Zeilen: 322 · Rust-Dateien: 3 · Ring: 0
- Interne Cargo-Abhängigkeiten: `acta-mvcc` (normal), `acta-ports` (normal), `acta-types` (normal)

**Ausgehend (Importe aus anderen Workspace-Crates)**

| Ziel | prod | dev | Dateien (prod) | Symbole (prod) | Symbole (nur dev) |
|---|---:|---:|---:|---|---|
| `acta-types` | 11 | 0 | 1 | `error`×2, `schema`×2, `tombstone`×2, `error_dto`, `model_fingerprint`, `types`, `error_dto::ActaErrorDto`, `model_fingerprint::ModelFingerprint` | – |
| `acta-mvcc` | 6 | 0 | 1 | `seq_log`×2, `snapshot`×2, `tx_buffer`×2 | – |
| `acta-ports` | 2 | 0 | 1 | `*`×2 | – |

**Pro Komponente**

- `(crate-root)` (122 Zeilen)
  - → `acta-mvcc`: prod: `seq_log`×2, `snapshot`×2, `tx_buffer`×2
  - → `acta-ports`: prod: `*`×2
  - → `acta-types`: prod: `error`×2, `schema`×2, `tombstone`×2, `error_dto`, `model_fingerprint`, `types`, `error_dto::ActaErrorDto`, `model_fingerprint::ModelFingerprint`

**Öffentliche Schnittstelle** (erreichbar über `pub mod`-Kette oder `pub use`)

Keine pub-Items erkannt (Bibliothek ohne lib.rs, nur Binary, oder alles intern).
- ⚠ Re-Exporte **fremder Crates**: `acta_types::error`, `acta_types::error_dto`, `acta_types::model_fingerprint`, `acta_types::schema`, `acta_types::tombstone`, `acta_types::types`, `acta_ports::*`, `acta_mvcc::seq_log`, `acta_mvcc::snapshot`, `acta_mvcc::tx_buffer`, `acta_ports::*`, `acta_types::error_dto::ActaErrorDto`, `acta_types::model_fingerprint::ModelFingerprint`

**Eingehend (wer nutzt dieses Crate)**

| Nutzer | Symbole prod | Symbole dev |
|---|---|---|
| `acta-vector` | `ActaError::invalid_input`×19, `MAX_SEARCH_K`×15, `Result`×8, `ActaError::HnswConnectivityDegraded`×4, `ActaError::Index`×4, `DocId`×3, `DistanceMetric`×2, `SequenceLog::new`×2, `SeqLogChange::Insert`, `SeqLogChange::Delete` … (+1) | `types`×9, `traits::VectorIndex`×8, `ActaError::InvalidInput`×6, `ActaError::EmbeddingDimensionMismatch`×4 … (+6) |
| `acta-store` | `MAX_SCAN_MERGE_ACCUMULATOR`×6, `ActaError::Storage`×5, `TOMBSTONE_BIT`×4, `TxId`×4, `ResourceTracker`×2, `ActaError::MemoryBudgetExceeded`×2, `ActaError::Internal`×2, `Result`×2, `ActaError`×2, `TxId::INTERNAL_BASE`×2 … (+4) | `ResourceBudget`×28, `ResourceTracker::new`×27, `StorageEngine`×2, `SnapshotRegistry::new`×2 … (+5) |
| `acta-simd` | `Result`×4, `ActaError` | – |
| `acta-store-fuzz` | `StorageEngine`, `TxId` | – |

### acta-crypto

- Pfad: `crates/acta-crypto` · Zeilen: 15950 · Rust-Dateien: 67 · Ring: 0
- Interne Cargo-Abhängigkeiten: `acta-crypto` (dev), `acta-ports` (normal), `acta-types` (normal)

**Ausgehend (Importe aus anderen Workspace-Crates)**

| Ziel | prod | dev | Dateien (prod) | Symbole (prod) | Symbole (nur dev) |
|---|---:|---:|---:|---|---|
| `acta-types` | 25 | 14 | 5 | `TenantId`×8, `Result`×6, `ActaError::Crypto`×3, `TenantScoped::new`×2, `TenantScoped`×2, `TxId`, `ActaError`, `ModelFingerprint`, `ActaError::ParseError` | – |
| `acta-ports` | 2 | 9 | 2 | `Clock`×2 | `SystemClock`×9 |

**Pro Komponente**

- `crypto` (835 Zeilen)
  - → `acta-types`: prod: `TenantId`×5, `TenantScoped::new`×2, `TenantScoped`×2, `TxId` · dev: `TenantId`×2
- `error` (108 Zeilen)
  - → `acta-types`: prod: `ActaError` · dev: `ActaError`
- `kv_cipher` (386 Zeilen)
  - → `acta-ports`: dev: `SystemClock`
  - → `acta-types`: prod: `Result`×6, `ActaError::Crypto`×3, `TenantId`, `ModelFingerprint`, `ActaError::ParseError`
- `kv_segment` (1224 Zeilen)
  - → `acta-types`: prod: `TenantId`×2 · dev: `TenantId`
- `kv_shredding` (629 Zeilen)
  - → `acta-ports`: prod: `Clock`
- `revocation_log` (418 Zeilen)
  - → `acta-ports`: prod: `Clock` · dev: `SystemClock`
- `benches/crypto_benchmarks.rs` (124 Zeilen)
  - → `acta-types`: dev: `TenantId`
- `tests/campaign_cov_revocation_log.rs` (33 Zeilen)
  - → `acta-ports`: dev: `SystemClock`
- `tests/campaign_j25.rs` (422 Zeilen)
  - → `acta-types`: dev: `TenantId`
- `tests/campaign_j26_hypotheses.rs` (604 Zeilen)
  - → `acta-ports`: dev: `SystemClock`
- `tests/eviction_worker_tokio_runtime_nonblocking_regression.rs` (61 Zeilen)
  - → `acta-types`: dev: `TenantId`
- `tests/j24closure-acta-crypto_closure_tests.rs` (116 Zeilen)
  - → `acta-types`: dev: `TxId`×2
- `tests/j25-acta-crypto-crypto-shredding-audit-_wiring_test.rs` (160 Zeilen)
  - → `acta-ports`: dev: `SystemClock`
- `tests/j25closure-acta-crypto_symbols_test.rs` (142 Zeilen)
  - → `acta-ports`: dev: `SystemClock`
- `tests/kv_revocation_wiring.rs` (67 Zeilen)
  - → `acta-ports`: dev: `SystemClock`
- `tests/kv_segment_concurrency.rs` (267 Zeilen)
  - → `acta-types`: dev: `TenantId`
- `tests/kv_segment_integration.rs` (115 Zeilen)
  - → `acta-types`: dev: `TenantId`
- `tests/kv_segment_proptests.rs` (78 Zeilen)
  - → `acta-types`: dev: `TenantId`
- `tests/loom_kv_deferred_zeroize.rs` (157 Zeilen)
  - → `acta-types`: dev: `TenantId`
- `tests/proptests.rs` (125 Zeilen)
  - → `acta-types`: dev: `TenantId`
- `tests/revocation_and_wal_error_paths.rs` (128 Zeilen)
  - → `acta-ports`: dev: `SystemClock`
- `tests/revocation_log_atomic_persist.rs` (125 Zeilen)
  - → `acta-ports`: dev: `SystemClock`

**Öffentliche Schnittstelle** (erreichbar über `pub mod`-Kette oder `pub use`)

- `anti_tamper`
  - 1 struct
    - `struct VolatileEncryptionKey` · 3 Methoden
- `audit_chain`
  - 1 enum, 1 fn, 5 struct
    - `enum DataClass`
    - `struct AuditChainEntry` · 1 Methoden
    - `struct AuditChainAnchor`
    - `struct AuditChainHeadSignature`
    - `struct EncryptedCommitmentSalt` · 2 Methoden
    - `fn compute_record_commitment`
    - `struct AuditChain` · 13 Methoden
- `crypto`
  - 1 struct
    - `struct KeyManager` · 18 Methoden
- `deletion_proof`
  - 4 enum, 3 fn, 4 struct
    - `struct GraphRepairAttestation`
    - `fn hash_deleted_keys_length_prefixed`
    - `struct DeletionProofKeyPair` · 3 Methoden
    - `enum VerificationKey`
    - `struct LayerCleanupProof` · 3 Methoden
    - `enum DeletionLayer`
    - `enum ExcludedScope`
    - `enum DeletionScope`
    - `struct DeletionProof` · 12 Methoden
    - `fn compute_wal_delete_receipt`
    - `fn verify_wal_delete_receipt`
- `deletion_proof_typestate`
  - 3 struct
    - `struct Missing`
    - `struct Cleaned`
    - `struct DeletionProofBuilder` · 10 Methoden
- `ed25519_proof`
  - 2 enum, 2 fn, 1 struct
    - `enum DeletionProofError`
    - `struct DeletionProofKeyPair` · 4 Methoden
    - `fn sign_deletion_proof_v3`
    - `fn verify_deletion_proof_v3`
    - `enum SignatureVersion` · 1 Methoden
- `error`
  - 1 enum, 1 type
    - `type Result`
    - `enum CryptoError` · 1 Methoden
- `kdf`
  - 10 const, 1 fn, 3 struct
    - `const KDF_HEADER_MAGIC`
    - `const KDF_HEADER_VERSION_1`
    - `const KDF_ID_ARGON2ID`
    - `const MIN_SALT_LEN`
    - `const MIN_M_COST_KIB`
    - `const MIN_T_COST`
    - `const MIN_P_COST`
    - `const DEFAULT_M_COST_KIB`
    - `const DEFAULT_T_COST`
    - `const DEFAULT_P_COST`
    - `struct KdfParams` · 1 Methoden
    - `struct KdfHeader` · 4 Methoden
    - `struct DerivedKey`
    - `fn derive_key_argon2id`
- `kv_cipher`
  - 1 const, 2 struct, 1 trait
    - `const CURRENT_KV_FORMAT_VERSION`
    - `struct EncryptedKvLayer` ⚠ fremd: `acta-types`
    - `trait KvCipher` · Methoden: seal, open
    - `struct KvSegmentCipher` · 7 Methoden
- `kv_segment`
  - 1 const, 1 fn, 4 struct, 1 type
    - `struct EvictionWorker` · 3 Methoden
    - `fn emergency_wipe`
    - `const CURRENT_KV_KEY_DERIVATION_VERSION`
    - `struct EncryptedSegmentPayload`
    - `struct KvSegment` · 8 Methoden ⚠ fremd: `acta-types`
    - `type SpillHandler` ⚠ fremd: `acta-types`
    - `struct TenantIsolatedKvStore` · 15 Methoden
- `kv_shredding`
  - 1 const, 1 fn, 5 struct, 1 trait
    - `const DEFAULT_SHRED_KEY_GROUP_SIZE`
    - `struct SubKey`
    - `struct GroupKek`
    - `struct RecordDek`
    - `struct EncryptedRecordPayload`
    - `fn derive_subkey`
    - `trait GroupRevocationListener` · Methoden: on_group_revoked
    - `struct KeyRegistry` · 19 Methoden
- `revocation_log`
  - 1 enum, 1 fn, 2 struct
    - `enum RevocationTarget` · 1 Methoden
    - `struct RevocationEntry`
    - `fn compute_entry_hash`
    - `struct RevocationLog` · 7 Methoden
- `wal_completeness`
  - 1 fn
    - `fn verify_wal_chain_completeness`
- `wal_crypto`
  - 4 struct, 1 trait
    - `trait KmsProvider` · Methoden: get_key
    - `struct EncryptedWal` · 3 Methoden
    - `struct WalHmac` · 3 Methoden
    - `struct WalEntrySnapshot`
    - `struct IntegrityVerifier` · 10 Methoden
- Zusätzlich 9 `pub`-Items in privaten Modulen ohne Re-Export (nicht Teil der echten API).
- Interne Re-Exporte (`pub use`): `hash_deleted_keys_length_prefixed`
- ⚠ Re-Exporte **fremder Crates**: `acta_types::ModelFingerprint`

**Eingehend (wer nutzt dieses Crate)**

| Nutzer | Symbole prod | Symbole dev |
|---|---|---|
| `acta-mcp` | `CryptoKey`×3, `CryptoKey::try_new`×2, `TenantIsolatedKvStore::new`×2, `KvSegmentCipher::new` | `deletion_proof::DeletionProof`, `AuditChain` |
| `acta-engine` | `TenantIsolatedKvStore`×5, `deletion_proof` | `kv_segment::KvSegment` |
| `acta-vector` | `wal_crypto::WalHmac::new`×6 | – |
| `acta-store` | `wal_crypto`×2, `verify_wal_chain_completeness`, `wal_crypto::WalHmac`, `crypto::KeyManager` | – |
| `acta-crypto-fuzz` | `deletion_proof`×2, `wal_crypto` | – |
| `acta-db` | – | `CryptoError`, `deletion_proof`, `deletion_proof::GraphRepairAttestation`, `deletion_proof::DeletionProof` |

**Nach außen ungenutzte öffentliche Items (Heuristik, 64 von 73)**: `VolatileEncryptionKey`, `DataClass`, `AuditChainEntry`, `AuditChainAnchor`, `AuditChainHeadSignature`, `EncryptedCommitmentSalt`, `compute_record_commitment`, `hash_deleted_keys_length_prefixed`, `DeletionProofKeyPair`, `VerificationKey`, `LayerCleanupProof`, `DeletionLayer`, `ExcludedScope`, `DeletionScope`, `compute_wal_delete_receipt`, `verify_wal_delete_receipt`, `Missing`, `Cleaned`, `DeletionProofBuilder`, `DeletionProofError`, `DeletionProofKeyPair`, `sign_deletion_proof_v3`, `verify_deletion_proof_v3`, `SignatureVersion`, `Result`, `KDF_HEADER_MAGIC`, `KDF_HEADER_VERSION_1`, `KDF_ID_ARGON2ID`, `MIN_SALT_LEN`, `MIN_M_COST_KIB` …

**Auffälligkeiten**

- acta-crypto: kein Code-Zugriff gefunden (prüfen: Makro, Feature, cfg, oder entfernen)

### acta-crypto-fuzz

- Pfad: `crates/acta-crypto/fuzz` · Zeilen: 356 · Rust-Dateien: 3
- Interne Cargo-Abhängigkeiten: `acta-crypto` (normal), `acta-types` (normal)

**Ausgehend (Importe aus anderen Workspace-Crates)**

| Ziel | prod | dev | Dateien (prod) | Symbole (prod) | Symbole (nur dev) |
|---|---:|---:|---:|---|---|
| `acta-crypto` | 3 | 0 | 3 | `deletion_proof`×2, `wal_crypto` | – |

**Pro Komponente**

- `fuzz_targets/deletion_proof_tamper.rs` (103 Zeilen)
  - → `acta-crypto`: prod: `deletion_proof`
- `fuzz_targets/deletion_proof_verify_external_fuzz.rs` (115 Zeilen)
  - → `acta-crypto`: prod: `deletion_proof`
- `fuzz_targets/wal_hmac_chain_verify_fuzz.rs` (138 Zeilen)
  - → `acta-crypto`: prod: `wal_crypto`

**Öffentliche Schnittstelle** (erreichbar über `pub mod`-Kette oder `pub use`)

Keine pub-Items erkannt (Bibliothek ohne lib.rs, nur Binary, oder alles intern).
- Zusätzlich 5 `pub`-Items in privaten Modulen ohne Re-Export (nicht Teil der echten API).

**Eingehend (wer nutzt dieses Crate)**

Wird von keinem anderen analysierten Workspace-Crate importiert.

**Auffälligkeiten**

- acta-types: kein Code-Zugriff gefunden (prüfen: Makro, Feature, cfg, oder entfernen)

### acta-db

- Pfad: `crates/acta-db` · Zeilen: 12443 · Rust-Dateien: 63 · Ring: 3
- Interne Cargo-Abhängigkeiten: `acta-vector` (dev), `acta-crypto` (normal), `acta-engine` (normal), `acta-graph` (normal), `acta-ports` (normal), `acta-rank` (normal), `acta-store` (normal), `acta-sys` (normal), `acta-text` (normal), `acta-types` (normal), `acta-vector` (normal)

**Ausgehend (Importe aus anderen Workspace-Crates)**

| Ziel | prod | dev | Dateien (prod) | Symbole (prod) | Symbole (nur dev) |
|---|---:|---:|---:|---|---|
| `acta-engine` | 6 | 1 | 1 | `(crate)`, `MetadataFilter`, `collection::maintenance::PercolationResult`, `SandboxBridge`, `background_workers::*`, `fusion` | `DeletionLayer` |
| `acta-types` | 2 | 47 | 2 | `DistanceMetric`, `MAX_SEARCH_K` | `Result`×7, `TxId`×6, `DocId`×4, `FusionWeights`×3, `TxId::new`×3 … (+17) |
| `acta-graph` | 1 | 15 | 1 | `percolation::PercolationConfig` | `CsrGraph`×10, `path_rag::PathRAGEngine`×2, `csr::CsrGraph`×2, `PathRAGEngine` |
| `acta-ports` | 1 | 13 | 1 | `QueryRewriter` | `GraphIndex`×4, `StorageEngine`×3, `BoxFuture`×2, `LlmTextGenerator`, `EmbeddingError` … (+2) |
| `acta-sys` | 1 | 0 | 1 | `LockedRegions` | – |
| `acta-crypto` | 0 | 4 | 0 | – | `CryptoError`, `deletion_proof`, `deletion_proof::GraphRepairAttestation`, `deletion_proof::DeletionProof` |
| `acta-rank` | 0 | 1 | 0 | – | `IsotonicCalibrator::new` |
| `acta-store` | 0 | 12 | 0 | – | `LsmConfig`×5, `LsmStorage::new`×4, `LsmStorage`×2, `lsm` |
| `acta-text` | 0 | 11 | 0 | – | `Language::English`×7, `Language`×2, `bm25::score_term`, `inverted::InvertedIndex` |
| `acta-vector` | 0 | 6 | 0 | – | `HnswConfig`×3, `HnswIndex`×2, `HnswIndex::try_new` |

**Pro Komponente**

- `(crate-root)` (775 Zeilen)
  - → `acta-engine`: prod: `(crate)`, `MetadataFilter`, `collection::maintenance::PercolationResult`, `SandboxBridge`, `background_workers::*`, `fusion`
  - → `acta-graph`: prod: `percolation::PercolationConfig`
  - → `acta-rank`: dev: `IsotonicCalibrator::new`
  - → `acta-types`: prod: `DistanceMetric` · dev: `TenantId`, `Result`, `ActaError::InvalidInput`
- `multistep` (620 Zeilen)
  - → `acta-graph`: dev: `CsrGraph`
  - → `acta-ports`: prod: `QueryRewriter` · dev: `BoxFuture`
  - → `acta-text`: dev: `Language::English`
  - → `acta-types`: prod: `MAX_SEARCH_K` · dev: `ActaError::Internal`
- `volatile_vault` (450 Zeilen)
  - → `acta-sys`: prod: `LockedRegions`
- `examples/collections.rs` (107 Zeilen)
  - → `acta-types`: dev: `Result`, `TenantId::new`
- `examples/hybrid_search.rs` (85 Zeilen)
  - → `acta-types`: dev: `Result`
- `examples/quickstart.rs` (56 Zeilen)
  - → `acta-types`: dev: `Result`
- `examples/verify_deletion_proof_external.rs` (228 Zeilen)
  - → `acta-crypto`: dev: `CryptoError`, `deletion_proof`, `deletion_proof::GraphRepairAttestation`
  - → `acta-types`: dev: `Result`×2, `DocId::new`
- `tests/atomic_commit.rs` (92 Zeilen)
  - → `acta-types`: dev: `EntityId`
- `tests/auto_community_detection_test.rs` (117 Zeilen)
  - → `acta-types`: dev: `EntityId`
- `tests/cascade_invalidation_integration.rs` (133 Zeilen)
  - → `acta-graph`: dev: `path_rag::PathRAGEngine`
  - → `acta-ports`: dev: `GraphIndex`
- `tests/cascade_invalidation_test.rs` (110 Zeilen)
  - → `acta-graph`: dev: `path_rag::PathRAGEngine`
  - → `acta-ports`: dev: `GraphIndex`
  - → `acta-types`: dev: `LinkRelation`
- `tests/checkpoint_layer_bounds.rs` (133 Zeilen)
  - → `acta-store`: dev: `LsmConfig`×2, `LsmStorage::new`×2
  - → `acta-types`: dev: `TxId`
- `tests/checkpoint_stress.rs` (74 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/compound_split_recall_impact.rs` (400 Zeilen)
  - → `acta-types`: dev: `FusionWeights`
- `tests/compound_split_recall_impact_test.rs` (444 Zeilen)
  - → `acta-types`: dev: `FusionWeights`
- `tests/consolidation_double_trigger_test.rs` (133 Zeilen)
  - → `acta-graph`: dev: `CsrGraph`
  - → `acta-store`: dev: `LsmStorage`, `LsmConfig`
  - → `acta-text`: dev: `Language::English`
  - → `acta-vector`: dev: `HnswIndex`, `HnswConfig`
- `tests/consolidation_integration_test.rs` (226 Zeilen)
  - → `acta-ports`: dev: `BoxFuture`, `LlmTextGenerator`
  - → `acta-types`: dev: `DocId`, `Result`
- `tests/context_compaction_retry_test.rs` (187 Zeilen)
  - → `acta-store`: dev: `LsmConfig`, `LsmStorage::new`
  - → `acta-types`: dev: `TxId::new`×2
- `tests/cross_domain_chaos_matrix_test.rs` (259 Zeilen)
  - → `acta-graph`: dev: `csr::CsrGraph`
  - → `acta-ports`: dev: `EmbeddingError`
  - → `acta-store`: dev: `lsm`
  - → `acta-text`: dev: `Language`
  - → `acta-types`: dev: `DocId`
- `tests/deletion_proof_integration.rs` (270 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
- `tests/diskann_collection_fallback.rs` (91 Zeilen)
  - → `acta-graph`: dev: `csr::CsrGraph`
  - → `acta-ports`: dev: `VectorIndex`
  - → `acta-store`: dev: `LsmStorage`, `LsmConfig`
  - → `acta-text`: dev: `Language`
  - → `acta-types`: dev: `TxId::new`
- `tests/docid_128_consolidation.rs` (34 Zeilen)
  - → `acta-types`: dev: `DocId`, `TxId`
- `tests/drop_collection_proof_vs_files.rs` (81 Zeilen)
  - → `acta-engine`: dev: `DeletionLayer`
- `tests/e2e_compliance_scenario.rs` (459 Zeilen)
  - → `acta-crypto`: dev: `deletion_proof::DeletionProof`
- `tests/embedding_dimension_switch.rs` (85 Zeilen)
  - → `acta-types`: dev: `ActaError`
- `tests/fault_injection_2pc.rs` (717 Zeilen)
  - → `acta-graph`: dev: `CsrGraph`
  - → `acta-text`: dev: `Language::English`
- `tests/graph_entity_delete_test.rs` (149 Zeilen)
  - → `acta-graph`: dev: `CsrGraph`
  - → `acta-ports`: dev: `GraphIndex`
  - → `acta-text`: dev: `Language::English`
  - → `acta-vector`: dev: `HnswIndex`, `HnswConfig`
- `tests/graph_hybrid_search.rs` (287 Zeilen)
  - → `acta-ports`: dev: `GraphIndex`
  - → `acta-types`: dev: `ActaError::SnapshotUnsupportedForSignal`×2, `GraphTraversalStrategy`, `EntityId::from_key`, `PprAlgorithm::Auto` … (+1)
- `tests/hnsw_delete_and_backfill.rs` (195 Zeilen)
  - → `acta-graph`: dev: `CsrGraph`
  - → `acta-text`: dev: `Language::English`
- `tests/instance_orphan_registry_test.rs` (123 Zeilen)
  - → `acta-types`: dev: `TxId`
- `tests/integration_temporal_filter.rs` (192 Zeilen)
  - → `acta-graph`: dev: `CsrGraph`
  - → `acta-types`: dev: `TxId`
- `tests/multistep_query_rewriter_modalities.rs` (186 Zeilen)
  - → `acta-graph`: dev: `CsrGraph`
  - → `acta-text`: dev: `Language::English`
  - → `acta-types`: dev: `ActaError::Internal`
- `tests/pid_pool_production_caller.rs` (106 Zeilen)
  - → `acta-graph`: dev: `CsrGraph`
  - → `acta-ports`: dev: `Clock`
  - → `acta-text`: dev: `Language::English`
- `tests/proptest_search_invariants.rs` (207 Zeilen)
  - → `acta-text`: dev: `bm25::score_term`
- `tests/search_result_bound.rs` (205 Zeilen)
  - → `acta-graph`: dev: `CsrGraph`
  - → `acta-types`: dev: `DistanceMetric`
- `tests/snapshot_api.rs` (163 Zeilen)
  - → `acta-types`: dev: `ActaError::CapabilityUnsupported`×2, `EntityId::new`
  - → `acta-vector`: dev: `HnswIndex::try_new`, `HnswConfig`
- `tests/split_brain_vector_orphan_test.rs` (501 Zeilen)
  - → `acta-types`: dev: `DocId`
- `tests/sprint2_acid_tests.rs` (345 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-store`: dev: `LsmStorage::new`
  - → `acta-types`: dev: `TenantId`, `TxId`
- `tests/stabilization_sprint_tests.rs` (135 Zeilen)
  - → `acta-types`: dev: `FusionWeights`
- `tests/supersedes_cascading_tombstone_test.rs` (161 Zeilen)
  - → `acta-graph`: dev: `PathRAGEngine`
- `tests/text_integration.rs` (96 Zeilen)
  - → `acta-text`: dev: `inverted::InvertedIndex`
- `tests/truncation_filter_recall_test.rs` (270 Zeilen)
  - → `acta-graph`: dev: `CsrGraph`
  - → `acta-types`: dev: `MemoryType`
- `tests/zettelkasten_links_test.rs` (262 Zeilen)
  - → `acta-types`: dev: `HybridQuery`, `FusionWeights::new`

**Öffentliche Schnittstelle** (erreichbar über `pub mod`-Kette oder `pub use`)

- `multistep`
  - 2 const, 3 struct
    - `const DEFAULT_MAX_SUB_QUERIES`
    - `const DEFAULT_RERANK_POOL_MULTIPLIER`
    - `struct MultiStepConfig`
    - `struct MultiStepResult`
    - `struct MultiStepEngine` · 4 Methoden
- `volatile_vault`
  - 2 enum, 6 struct
    - `enum SignalModality`
    - `struct VaultChunk` · 3 Methoden
    - `struct VaultConfig`
    - `struct PurgeReceipt`
    - `struct CommitReceipt`
    - `enum VaultError`
    - `struct VolatileContextVault` · 9 Methoden
    - `struct VaultChunkMetadata`
- Zusätzlich 8 `pub`-Items in privaten Modulen ohne Re-Export (nicht Teil der echten API).
- ⚠ Re-Exporte **fremder Crates**: `acta_engine`, `acta_engine::MetadataFilter`, `acta_engine::collection::maintenance::PercolationResult`, `acta_engine::SandboxBridge`, `acta_engine::background_workers::*`, `acta_engine::fusion`, `acta_graph::percolation::PercolationConfig`, `acta_types::DistanceMetric`, `acta_ports::QueryRewriter`

**Eingehend (wer nutzt dieses Crate)**

| Nutzer | Symbole prod | Symbole dev |
|---|---|---|
| `acta-db-fuzz` | `fusion::weighted_reciprocal_rank_fusion`, `SearchResult` | – |

**Nach außen ungenutzte öffentliche Items (Heuristik, 13 von 13)**: `DEFAULT_MAX_SUB_QUERIES`, `DEFAULT_RERANK_POOL_MULTIPLIER`, `MultiStepConfig`, `MultiStepResult`, `MultiStepEngine`, `SignalModality`, `VaultChunk`, `VaultConfig`, `PurgeReceipt`, `CommitReceipt`, `VaultError`, `VolatileContextVault`, `VaultChunkMetadata`

**Auffälligkeiten**

- acta-crypto: nur in Tests/Benches genutzt, aber als normale Abhängigkeit deklariert → dev-dependency?
- acta-rank: nur in Tests/Benches genutzt, aber als normale Abhängigkeit deklariert → dev-dependency?
- acta-store: nur in Tests/Benches genutzt, aber als normale Abhängigkeit deklariert → dev-dependency?
- acta-text: nur in Tests/Benches genutzt, aber als normale Abhängigkeit deklariert → dev-dependency?
- acta-vector: nur in Tests/Benches genutzt, aber als normale Abhängigkeit deklariert → dev-dependency?

### acta-db-fuzz

- Pfad: `crates/acta-db/fuzz` · Zeilen: 51 · Rust-Dateien: 1
- Interne Cargo-Abhängigkeiten: `acta-core` (normal), `acta-db` (normal)

**Ausgehend (Importe aus anderen Workspace-Crates)**

| Ziel | prod | dev | Dateien (prod) | Symbole (prod) | Symbole (nur dev) |
|---|---:|---:|---:|---|---|
| `acta-db` | 2 | 0 | 1 | `fusion::weighted_reciprocal_rank_fusion`, `SearchResult` | – |

**Pro Komponente**

- `fuzz_targets/rrf_fusion.rs` (51 Zeilen)
  - → `acta-db`: prod: `fusion::weighted_reciprocal_rank_fusion`, `SearchResult`

**Öffentliche Schnittstelle** (erreichbar über `pub mod`-Kette oder `pub use`)

Keine pub-Items erkannt (Bibliothek ohne lib.rs, nur Binary, oder alles intern).
- Zusätzlich 3 `pub`-Items in privaten Modulen ohne Re-Export (nicht Teil der echten API).

**Eingehend (wer nutzt dieses Crate)**

Wird von keinem anderen analysierten Workspace-Crate importiert.

**Auffälligkeiten**

- acta-core: kein Code-Zugriff gefunden (prüfen: Makro, Feature, cfg, oder entfernen)

### acta-engine

- Pfad: `crates/acta-engine` · Zeilen: 24027 · Rust-Dateien: 117 · Ring: 3
- Interne Cargo-Abhängigkeiten: `acta-core` (dev), `acta-crypto` (dev), `acta-vector` (dev), `acta-crypto` (normal/optional via encryption-at-rest), `acta-graph` (normal), `acta-mvcc` (normal), `acta-ports` (normal), `acta-rank` (normal), `acta-store` (normal), `acta-sys` (normal), `acta-text` (normal), `acta-types` (normal), `acta-vector` (normal)

**Ausgehend (Importe aus anderen Workspace-Crates)**

| Ziel | prod | dev | Dateien (prod) | Symbole (prod) | Symbole (nur dev) |
|---|---:|---:|---:|---|---|
| `acta-types` | 252 | 172 | 33 | `ActaError::invalid_input`×50, `Result`×23, `ActaError::Internal`×19, `EntityId`×14, `MAX_SEARCH_K`×12, `ActaError::Serialization`×8, `ImportanceScore::new`×8, `domain::MemoryLink`×7, `HybridQuery`×7, `EntityId::from_key`×7 … (+42) | `ActaError::from`×10, `domain::LinkRelation::Elaborates`×2, `ActaError::HnswConnectivityDegraded`, `FusionWeights::new`, `PprConfig::default` … (+7) |
| `acta-ports` | 33 | 21 | 11 | `MetricsSink`×9, `StorageEngine`×5, `KvLifecycleHooks`×4, `VectorIndexStats`×2, `DriftStatusProvider`×2, `VectorIndex`, `Reranker`, `license`, `SystemClock::new`, `NoopMetricsSink` … (+6) | `Clock` |
| `acta-store` | 30 | 106 | 7 | `LsmStorage`×6, `tenant_codec::TenantScopedStorage`×5, `kv::KvSegmentManager`×4, `tenant_codec::TenantScopedStorage::new`×4, `SystemPressure`×3, `kv::KvSegmentPayload`×2, `tenant_codec::TenantKeyCodec`×2, `PressureLevel::Critical`, `LsmConfig`, `lsm::DurabilityMode` … (+1) | `lsm`×10, `compaction`×2, `sstable`×2 |
| `acta-graph` | 20 | 76 | 8 | `hyperedge::HyperEdgeId`×2, `percolation::PercolationConfig`×2, `CsrGraph::load_from_storage`×2, `hyperedge::HyperEdge`×2, `cascade_invalidate_edges_for_superseded_doc`, `percolation::compute_percolation_health`, `percolation::should_trigger_rebonding`, `percolation::find_rebonding_candidates`, `CsrGraph`, `hyperedge::RoleId::new` … (+6) | `csr::CsrGraph`×15, `csr`, `CsrGraph::new` |
| `acta-crypto` | 6 | 2 | 2 | `TenantIsolatedKvStore`×5, `deletion_proof` | `kv_segment::KvSegment` |
| `acta-rank` | 6 | 0 | 3 | `IsotonicCalibrator`×2, `fusion`, `fusion::fuse_search_results_with_strategy`, `fusion::fuse_search_results_with_signal_strategies`, `fusion::fuse_signals` | – |
| `acta-vector` | 5 | 119 | 5 | `hnsw::HnswIndex`×2, `HnswIndex`×2, `acorn::FilteredIndex` | `HnswConfig`×58, `hnsw::HnswConfig::default`, `hnsw::HnswIndex::try_new`, `acorn`, `HnswIndex::try_new` |
| `acta-text` | 3 | 69 | 2 | `Language`×2, `inverted::InvertedIndex` | `Language::English`×64, `Language::German`×4 |
| `acta-mvcc` | 1 | 3 | 1 | `tx_buffer::TxBuffer` | `tx_buffer::IndexOp` |

**Pro Komponente**

- `(crate-root)` (718 Zeilen)
  - → `acta-crypto`: prod: `deletion_proof`
  - → `acta-graph`: prod: `percolation::PercolationConfig`
  - → `acta-ports`: prod: `MetricsSink`×3, `KvLifecycleHooks`×3, `license::LicenseGate`, `BoxFuture`, `StorageEngine`, `TextEmbeddingEngine`, `VectorDeleteMode`, `DriftStatusProvider`, `VectorIndexStats`, `StorageStats`
  - → `acta-rank`: prod: `IsotonicCalibrator`
  - → `acta-store`: prod: `lsm::DurabilityMode`, `LsmStorage`, `tenant_codec::TenantScopedStorage`
  - → `acta-text`: prod: `Language`
  - → `acta-types`: prod: `Result`×13, `ActaError::Internal`×10, `DistanceMetric`×2, `DocId`×2, `ActaError::InvalidInput`×2, `ActaError::Serialization`×2, `DistanceMetric::Cosine` · dev: `DocId::new`
- `background_workers` (1075 Zeilen)
  - → `acta-graph`: prod: `hyperedge::HyperEdgeId` · dev: `CsrGraph`×8, `hyperedge::HyperEdgeId`
  - → `acta-mvcc`: prod: `tx_buffer::TxBuffer` · dev: `tx_buffer::TxBuffer`×2, `tx_buffer::IndexOp`
  - → `acta-ports`: prod: `StorageEngine`×2, `VectorIndex`
  - → `acta-store`: dev: `LsmStorage`×8, `LsmConfig`×8
  - → `acta-text`: dev: `Language::English`×8
  - → `acta-types`: prod: `Result`×4 · dev: `Result`×2, `ActaError::HnswConnectivityDegraded`
  - → `acta-vector`: prod: `hnsw::HnswIndex`×2 · dev: `HnswIndex`×8, `HnswConfig`×8, `hnsw::HnswConfig::default`, `hnsw::HnswIndex::try_new`
- `collection` (10912 Zeilen)
  - → `acta-crypto`: prod: `TenantIsolatedKvStore`×5
  - → `acta-graph`: prod: `cascade_invalidate_edges_for_superseded_doc`, `percolation::PercolationConfig`, `percolation::compute_percolation_health`, `percolation::should_trigger_rebonding`, `percolation::find_rebonding_candidates`, `CsrGraph`, `hyperedge::HyperEdgeId`, `hyperedge::RoleId::new`, `hyperedge::HyperEdgeId::new`, `hyperedge::RoleBinding` … (+4) · dev: `CsrGraph`×40, `csr::CsrGraph`×15, `csr`, `percolation::PercolationConfig`
  - → `acta-ports`: prod: `VectorIndexStats`, `StorageEngine` · dev: `StorageStats`×5, `StorageEngine`×3, `TextEmbeddingEngine`, `BoxFuture`
  - → `acta-store`: prod: `kv::KvSegmentManager`×4, `SystemPressure`×3, `LsmStorage`×2, `tenant_codec::TenantScopedStorage`×2, `kv::KvSegmentPayload`×2, `tenant_codec::TenantKeyCodec`×2, `PressureLevel::Critical` · dev: `LsmStorage`×36, `LsmConfig`×35, `lsm`×10
  - → `acta-text`: prod: `inverted::InvertedIndex`, `Language` · dev: `Language::English`×50, `Language::German`×4, `Language`
  - → `acta-types`: prod: `ActaError::invalid_input`×37, `MAX_SEARCH_K`×12, `EntityId`×10, `ImportanceScore::new`×8, `ActaError::Internal`×7, `domain::MemoryLink`×7, `EntityId::from_key`×7, `Entity::new`×6, `DecayFunction::None`×6, `HybridQuery`×6 … (+36) · dev: `Result`×25, `ActaError::InvalidInput`×15, `ActaError::from`×10, `MAX_SEARCH_K`×4 … (+27)
  - → `acta-vector`: prod: `HnswIndex`, `acorn::FilteredIndex` · dev: `HnswIndex`×44, `HnswConfig`×44
- `acta_impl` (1330 Zeilen)
  - → `acta-graph`: prod: `CsrGraph::load_from_storage`×2
  - → `acta-ports`: prod: `MetricsSink`×3, `Reranker`, `license`, `SystemClock::new`, `NoopMetricsSink`, `KvLifecycleHooks`, `DriftStatusProvider`
  - → `acta-rank`: prod: `IsotonicCalibrator`
  - → `acta-store`: prod: `tenant_codec::TenantScopedStorage::new`×4, `LsmStorage`×3, `tenant_codec::TenantScopedStorage`×2, `LsmConfig`
  - → `acta-types`: prod: `ActaError::invalid_input`×9, `EntityId`×4, `ActaError::Internal`×2, `FusionWeights`×2, `ActaError::PolicyViolation`×2, `ActaError::Storage`×2, `ActaError::Transaction`, `MemoryType`, `GraphTraversalStrategy`, `HybridQuery` … (+1) · dev: `Result`×3
- `export` (337 Zeilen)
  - → `acta-types`: prod: `TxId::new` · dev: `DocId::from_key`×4
- `extraction` (517 Zeilen)
  - → `acta-ports`: prod: `LlmTextGenerator`
- `filter` (290 Zeilen)
  - → `acta-types`: prod: `ActaError::invalid_input`×2, `ActaError` · dev: `ActaError::InvalidInput`×2
- `fusion` (54 Zeilen)
  - → `acta-rank`: prod: `fusion`, `fusion::fuse_search_results_with_strategy`, `fusion::fuse_search_results_with_signal_strategies`, `fusion::fuse_signals`
  - → `acta-types`: prod: `FusionStrategy`, `SignalFusionStrategies`
- `import` (114 Zeilen)
  - → `acta-types`: prod: `ActaError::Serialization`×2, `ActaError::invalid_input`×2, `Result`
- `merge_operator_adapter` (175 Zeilen)
  - → `acta-store`: prod: `MergeOperator`
- `metrics_names` (50 Zeilen)
  - → `acta-ports`: prod: `MetricsSink`×3
- `temporal_filter` (311 Zeilen)
  - → `acta-types`: prod: `TxId`
- `transaction` (1350 Zeilen)
  - → `acta-crypto`: dev: `kv_segment::KvSegment`, `TenantIsolatedKvStore`
  - → `acta-graph`: prod: `hyperedge::HyperEdge`×2 · dev: `CsrGraph`
  - → `acta-ports`: prod: `StorageEngine`
  - → `acta-store`: dev: `LsmStorage`, `LsmConfig`
  - → `acta-text`: dev: `Language::English`
  - → `acta-types`: prod: `TenantId::try_new` · dev: `DocId::new`×4, `DocId`×2, `EntityId`, `TenantId::try_new`
  - → `acta-vector`: prod: `HnswIndex` · dev: `HnswIndex`, `HnswConfig`
- `tests/auto_extraction_runtime_mode.rs` (225 Zeilen)
  - → `acta-types`: dev: `Result`
- `tests/auto_extraction_wiring.rs` (277 Zeilen)
  - → `acta-types`: dev: `EntityId::from_key`×5
- `tests/auto_triple_extraction.rs` (111 Zeilen)
  - → `acta-types`: dev: `EntityId::from_key`×2
- `tests/chunker_robustness.rs` (58 Zeilen)
  - → `acta-types`: dev: `DocId`
- `tests/commit_crash_consistency.rs` (287 Zeilen)
  - → `acta-graph`: dev: `CsrGraph`
  - → `acta-text`: dev: `Language::English`
  - → `acta-vector`: dev: `HnswIndex`, `HnswConfig`
- `tests/crud_error_robustness.rs` (103 Zeilen)
  - → `acta-types`: dev: `ActaError`, `Result`
- `tests/decay_and_expiry.rs` (147 Zeilen)
  - → `acta-ports`: dev: `Clock`
  - → `acta-types`: dev: `Result`×2, `MemoryType`
- `tests/engine_commit_crash_points.rs` (214 Zeilen)
  - → `acta-graph`: dev: `CsrGraph`
  - → `acta-text`: dev: `Language::English`
  - → `acta-vector`: dev: `HnswIndex`, `HnswConfig`
- `tests/engine_recovery_crash_windows.rs` (362 Zeilen)
  - → `acta-graph`: dev: `CsrGraph`
  - → `acta-text`: dev: `Language::English`
  - → `acta-vector`: dev: `HnswIndex`, `HnswConfig`
- `tests/export_import_roundtrip.rs` (85 Zeilen)
  - → `acta-types`: dev: `ActaError`, `Result`
- `tests/filter_metadata_eval.rs` (99 Zeilen)
  - → `acta-types`: dev: `FilterExpr`, `Result`
- `tests/hybrid_adaptive_overfetch.rs` (123 Zeilen)
  - → `acta-graph`: dev: `CsrGraph`
- `tests/hybrid_pathrag_snapshot.rs` (103 Zeilen)
  - → `acta-graph`: dev: `CsrGraph`
- `tests/hybrid_query_fusion.rs` (88 Zeilen)
  - → `acta-types`: dev: `Result`
- `tests/hybrid_signal_failure_semantics.rs` (99 Zeilen)
  - → `acta-graph`: dev: `CsrGraph`
- `tests/invariant_determinism.rs` (74 Zeilen)
  - → `acta-types`: dev: `Result`×2
- `tests/invariant_durability.rs` (100 Zeilen)
  - → `acta-types`: dev: `Result`
- `tests/invariant_isolation.rs` (122 Zeilen)
  - → `acta-types`: dev: `Result`, `ActaError::Internal`
- `tests/invariant_tenant_isolation.rs` (128 Zeilen)
  - → `acta-types`: dev: `Result`×2
- `tests/j03_acta_engine_collection_crud_mainten_wiring_tests.rs` (116 Zeilen)
  - → `acta-ports`: dev: `LlmTextGenerator`, `BoxFuture`
  - → `acta-types`: dev: `DocId`, `Result`
- `tests/j03closure-acta-engine_wiring_test.rs` (119 Zeilen)
  - → `acta-ports`: dev: `LlmTextGenerator`, `BoxFuture`
  - → `acta-types`: dev: `DocId`, `Result`
- `tests/kv_cache_integration_test.rs` (110 Zeilen)
  - → `acta-types`: dev: `TenantId`
- `tests/kv_hooks_production_wiring.rs` (88 Zeilen)
  - → `acta-ports`: dev: `KvLifecycleHooks`
  - → `acta-types`: dev: `Result`×3
- `tests/kv_lifecycle_hooks_rollback.rs` (77 Zeilen)
  - → `acta-graph`: dev: `CsrGraph`
  - → `acta-ports`: dev: `KvLifecycleHooks`
  - → `acta-store`: dev: `LsmStorage`, `LsmConfig`
  - → `acta-text`: dev: `Language::English`
  - → `acta-vector`: dev: `HnswIndex`, `HnswConfig`
- `tests/metrics_instrumentation.rs` (122 Zeilen)
  - → `acta-types`: dev: `Result`×4
- `tests/multi_index_transaction.rs` (99 Zeilen)
  - → `acta-types`: dev: `Result`
- `tests/no_crypto_proof_stub.rs` (136 Zeilen)
  - → `acta-types`: dev: `ActaError::Internal`
- `tests/orphan_worker_started_on_open.rs` (82 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
- `tests/query_builder_scope_acorn.rs` (238 Zeilen)
  - → `acta-graph`: dev: `CsrGraph`
  - → `acta-vector`: dev: `acorn`
- `tests/query_builder_time_travel.rs` (75 Zeilen)
  - → `acta-types`: dev: `Result`
- `tests/rule_based_extraction_determinism.rs` (165 Zeilen)
  - → `acta-types`: dev: `Result`
- `tests/ssi_read_path_coverage.rs` (158 Zeilen)
  - → `acta-graph`: dev: `CsrGraph::new`
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-text`: dev: `Language::English`
  - → `acta-types`: dev: `domain::LinkRelation::Elaborates`, `DistanceMetric::Cosine`
  - → `acta-vector`: dev: `HnswIndex::try_new`, `HnswConfig`
- `tests/tenant_collection_handle_sharing.rs` (61 Zeilen)
  - → `acta-types`: dev: `Result`×2, `TenantId`
- `tests/tenant_index_persistence_reload.rs` (85 Zeilen)
  - → `acta-types`: dev: `Result`×2, `TenantId`
- `tests/tenant_policy_required.rs` (50 Zeilen)
  - → `acta-types`: dev: `Result`×2
- `tests/tenant_wrapper_coverage.rs` (210 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `Result`
- `tests/wasm_merge_operator.rs` (339 Zeilen)
  - → `acta-store`: dev: `compaction`×2, `sstable`×2, `MergeOperator`
  - → `acta-types`: dev: `ActaError`
- `tests/write_skew_via_engine_api.rs` (198 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `Result`×2, `ActaError`

**Öffentliche Schnittstelle** (erreichbar über `pub mod`-Kette oder `pub use`)

- `(crate-root)`
  - 2 enum, 1 fn, 5 struct, 1 trait, 3 type
    - `type ConsolidationLauncher` ⚠ fremd: `acta-store`
    - `fn register_consolidation_launcher` ⚠ fremd: `acta-store`
    - `struct Document`
    - `struct ActaStats` ⚠ fremd: `acta-ports`
    - `type DbStats`
    - `struct CommunityDetectionConfig`
    - `enum EmbeddingBackend`
    - `enum TenantPolicy`
    - `struct ActaConfig` · 3 Methoden ⚠ fremd: `acta-ports`, `acta-store`, `acta-types`
    - `type TenantCollectionMap` ⚠ fremd: `acta-store`
    - `struct Acta` · 6 Methoden
    - `trait SandboxBridge` · Methoden: db_search, db_insert, db_get
- `background_workers`
  - 1 fn
    - `fn start_decay_cleanup_worker` *(re-exportiert)* ⚠ fremd: `acta-ports`
- `chunker`
  - 3 fn, 2 struct
    - `fn estimate_tokens`
    - `struct ChunkerConfig`
    - `struct MarkdownChunker` · 3 Methoden
    - `fn chunk_text_with_overlap`
    - `fn chunk_text`
- `collection`
  - 4 const, 1 enum, 5 fn, 7 struct
    - `const MAX_SCAN_RESULTS` *(re-exportiert)*
    - `fn sorted_unique_shards`
    - `struct PercolationResult`
    - `struct CollectionConfig`
    - `struct StoredDocument`
    - `struct StoredDocumentMeta`
    - `fn parse_importance_score`
    - `fn compute_default_importance` ⚠ fremd: `acta-types`
    - `fn ensure_importance_metadata`
    - `fn extract_effective_importance`
    - `struct Collection` · 48 Methoden ⚠ fremd: `acta-store`, `acta-vector`
    - `const METRIC_SEARCH_LATENCY_VECTOR`
    - `const METRIC_SEARCH_LATENCY_TEXT`
    - `const METRIC_SEARCH_LATENCY_GRAPH`
    - `struct ScopeConstraint` *(re-exportiert)* · 2 Methoden ⚠ fremd: `acta-types`
    - `enum SearchStrategy` *(re-exportiert)* · 3 Methoden ⚠ fremd: `acta-types`
    - `struct SignalWeights` *(re-exportiert)* · 1 Methoden
- `export`
  - 1 const, 4 struct
    - `const SCHEMA_VERSION_V1`
    - `struct ExportDocumentV1`
    - `struct ExportCollectionV1`
    - `struct ExportMemoryV1`
    - `struct ExportRelationV1`
- `extraction`
  - 2 const, 4 fn, 4 struct, 1 trait
    - `trait EntityExtractor` · Methoden: extract_triples
    - `struct OpenIeExtractor`
    - `fn create_extractor`
    - `fn extract_triples_for_mode`
    - `fn extract_triples_for_config`
    - `fn extract_triples` ⚠ fremd: `acta-ports`
    - `const MAX_EXTRACTION_TEXT_LENGTH`
    - `const MAX_EXTRACTED_TRIPLES`
    - `struct RuleBasedExtractor` · 2 Methoden
    - `struct ExtractedTriple`
    - `struct EntityExtractionConfig`
- `filter`
  - 2 enum, 1 fn
    - `fn extract_memory_type`
    - `enum FilterOp`
    - `enum MetadataFilter` · 1 Methoden
- `fusion`
  - 3 fn
    - `fn fuse_search_results_with_strategy` ⚠ fremd: `acta-types`
    - `fn fuse_search_results_with_signal_strategies` ⚠ fremd: `acta-types`
    - `fn fuse_signals`
- `import`
  - 1 struct
    - `struct ImportSummary`
- `kv_cache_integration`
  - 3 fn, 1 struct
    - `struct AccessCounterAttentionExporter` · 3 Methoden
    - `fn build_eviction_worker`
    - `fn build_eviction_worker_with_exporter`
    - `fn start_kv_cache_eviction`
- `merge_operator_adapter`
  - 1 struct
    - `struct WasmMergeOperator` · 2 Methoden
- `metrics_names`
  - 7 const, 3 fn
    - `const COMMIT_LATENCY`
    - `const REBUILD_DURATION`
    - `const BACKPRESSURE_LEVEL`
    - `const CHECKPOINT_DURATION`
    - `const SEARCH_LATENCY_VECTOR`
    - `const SEARCH_LATENCY_TEXT`
    - `const SEARCH_LATENCY_GRAPH`
    - `fn record_histogram_safe` ⚠ fremd: `acta-ports`
    - `fn record_gauge_safe` ⚠ fremd: `acta-ports`
    - `fn record_counter_safe` ⚠ fremd: `acta-ports`
- `temporal_filter`
  - 3 fn, 1 struct, 1 type
    - `type FusionResult`
    - `struct ValidityWindow` · 2 Methoden ⚠ fremd: `acta-types`
    - `fn extract_validity_window`
    - `fn apply_temporal_validity_filter` ⚠ fremd: `acta-types`
    - `fn apply_temporal_validity_filter_at`
- `transaction`
  - 1 enum, 1 fn, 1 struct
    - `fn cleanup_orphaned_consolidation_intents` *(re-exportiert)* ⚠ fremd: `acta-ports`
    - `struct DbTransaction` *(re-exportiert)* · 10 Methoden ⚠ fremd: `acta-vector`
  - … 1 weitere Items nicht aufgelistet (`--max-items`)
- Zusätzlich 40 `pub`-Items in privaten Modulen ohne Re-Export (nicht Teil der echten API).
- ⚠ Re-Exporte **fremder Crates**: `acta_types::AutoExtractionMode`, `acta_ports::TextEmbeddingEngine`, `acta_ports::VectorDeleteMode`, `acta_store::lsm::DurabilityMode`, `acta_graph::percolation::PercolationConfig`, `acta_text::Language`, `acta_ports::DriftStatusProvider`, `acta_types::DistanceMetric`

**Eingehend (wer nutzt dieses Crate)**

| Nutzer | Symbole prod | Symbole dev |
|---|---|---|
| `acta-db` | `(crate)`, `MetadataFilter`, `collection::maintenance::PercolationResult`, `SandboxBridge`, `background_workers::*`, `fusion` | `DeletionLayer` |

*Hinweis: Mindestens ein Crate importiert per Glob (`*`); ungenutzte Items sind deshalb nicht bestimmbar.*

**Auffälligkeiten**

- acta-core: kein Code-Zugriff gefunden (prüfen: Makro, Feature, cfg, oder entfernen)
- acta-sys: kein Code-Zugriff gefunden (prüfen: Makro, Feature, cfg, oder entfernen)

### acta-graph

- Pfad: `crates/acta-graph` · Zeilen: 25811 · Rust-Dateien: 109 · Ring: 0
- Interne Cargo-Abhängigkeiten: `acta-store` (dev), `acta-ports` (normal), `acta-types` (normal)

**Ausgehend (Importe aus anderen Workspace-Crates)**

| Ziel | prod | dev | Dateien (prod) | Symbole (prod) | Symbole (nur dev) |
|---|---:|---:|---:|---|---|
| `acta-types` | 39 | 53 | 25 | `EntityId`×14, `ActaError::InvalidInput`×5, `ActaError`×4, `PprConfig`×3, `Result`×3, `Edge::new`×2, `TxId`×2, `ActaError::Internal`×2, `Edge`, `MemoryLink` … (+2) | `Entity::new`×10, `TlHfdParams::default`×4, `PprAlgorithm::Auto`×4, `TlHfdParams` |
| `acta-ports` | 20 | 40 | 8 | `GraphIndex`×3, `StorageEngine`×2, `BoxFuture`×2, `graph::GraphMutationError::Internal`×2, `graph::RoleBinding`, `graph::HyperEdgeId`, `graph::GraphMutationError`, `graph::GraphMutationError::LockAcquisitionTimeout`, `graph::GraphMutationError::RoleBindingInvalid`, `graph::GraphMutationError::EpochReclamationPending` … (+5) | `Result`×2, `graph`, `graph_index::GraphIndex` |
| `acta-store` | 0 | 2 | 0 | – | `LsmConfig`, `LsmStorage::new` |

**Pro Komponente**

- `(crate-root)` (122 Zeilen)
  - → `acta-ports`: prod: `GraphIndex`, `BoxFuture`
  - → `acta-types`: prod: `TxId`, `Result`
- `apprh` (911 Zeilen)
  - → `acta-types`: prod: `EntityId`×4, `ActaError`×2, `ActaError::InvalidInput`×2
- `cascade` (907 Zeilen)
  - → `acta-ports`: dev: `GraphIndex`
  - → `acta-types`: prod: `ActaError::InvalidInput`×2 · dev: `Entity::new`×10
- `community` (1269 Zeilen)
  - → `acta-ports`: dev: `GraphIndex`
- `csr` (5984 Zeilen)
  - → `acta-ports`: prod: `graph::GraphMutationError::Internal`×2, `StorageEngine`, `BoxFuture`, `graph::RoleBinding`, `graph::HyperEdgeId`, `graph::GraphMutationError`, `graph::GraphMutationError::LockAcquisitionTimeout`, `graph::GraphMutationError::RoleBindingInvalid`, `graph::GraphMutationError::EpochReclamationPending`, `graph::GraphMutationError::InsufficientParticipants` … (+3) · dev: `GraphIndex`×2
  - → `acta-types`: prod: `PprConfig`×3, `Edge::new`×2, `Edge`, `MemoryLink`, `LinkRelation::Supersedes`, `EntityId`, `TxId` · dev: `Result`
- `edge_reinforcement` (159 Zeilen)
  - → `acta-types`: dev: `EntityId`
- `edge_reinforcement_buffer` (183 Zeilen)
  - → `acta-ports`: dev: `GraphIndex`
  - → `acta-types`: prod: `EntityId` · dev: `EntityId`
- `entity_extraction` (271 Zeilen)
  - → `acta-types`: prod: `ActaError::Internal`
- `error` (90 Zeilen)
  - → `acta-types`: prod: `ActaError`
- `hyperedge_suggest` (155 Zeilen)
  - → `acta-ports`: prod: `LlmTextGenerator`
- `path_rag` (1371 Zeilen)
  - → `acta-types`: prod: `EntityId`×2, `Result`×2, `DocId`
- `percolation` (267 Zeilen)
  - → `acta-ports`: prod: `GraphIndex` · dev: `GraphIndex`
  - → `acta-types`: prod: `EntityId`
- `ppr` (3173 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`×3, `GraphIndex`×2
  - → `acta-store`: dev: `LsmConfig`, `LsmStorage::new`
  - → `acta-types`: prod: `EntityId` · dev: `TlHfdParams::default`×4, `TlHfdParams`
- `ppr_stream` (137 Zeilen)
  - → `acta-ports`: prod: `GraphIndex`
  - → `acta-types`: prod: `ActaError::Internal`
- `provenance` (229 Zeilen)
  - → `acta-types`: dev: `EntityId`
- `session_dag` (854 Zeilen)
  - → `acta-ports`: prod: `StorageEngine`
- `tl_hfd` (698 Zeilen)
  - → `acta-types`: prod: `EntityId`×4, `ActaError`, `ActaError::InvalidInput`
- `benches/csr_traversal_bench.rs` (193 Zeilen)
  - → `acta-ports`: dev: `GraphIndex`
  - → `acta-types`: dev: `Result`×4, `Edge::new`×2
- `tests/apprh_determinism_permutation.rs` (128 Zeilen)
  - → `acta-types`: dev: `EntityId`
- `tests/apprh_gate_monitor.rs` (284 Zeilen)
  - → `acta-types`: dev: `EntityId`
- `tests/apprh_selector_shadow_to_production.rs` (215 Zeilen)
  - → `acta-types`: dev: `EntityId`
- `tests/apprh_tests.rs` (210 Zeilen)
  - → `acta-types`: dev: `EntityId`
- `tests/budgeted_k_path_hub_entity.rs` (144 Zeilen)
  - → `acta-types`: dev: `EntityId`
- `tests/community_hyperedges_included_flag.rs` (122 Zeilen)
  - → `acta-ports`: dev: `GraphIndex`
- `tests/csr_benchmark.rs` (157 Zeilen)
  - → `acta-ports`: dev: `GraphIndex`
- `tests/csr_complexity_bench.rs` (146 Zeilen)
  - → `acta-ports`: dev: `GraphIndex`
- `tests/dangling_nodes_audit_test.rs` (238 Zeilen)
  - → `acta-ports`: dev: `GraphIndex`
  - → `acta-types`: dev: `PprAlgorithm::Auto`×3
- `tests/diffusion_shadow_gate_thresholds.rs` (85 Zeilen)
  - → `acta-types`: dev: `EntityId`
- `tests/entity_extraction_tests.rs` (74 Zeilen)
  - → `acta-types`: dev: `DocId`
- `tests/graph_collection_mutation_trait_test.rs` (63 Zeilen)
  - → `acta-ports`: dev: `graph`
- `tests/hub_node_benchmark.rs` (95 Zeilen)
  - → `acta-ports`: dev: `GraphIndex`
  - → `acta-types`: dev: `Edge::new`×2
- `tests/hyperedge_cascade_crash_recovery.rs` (162 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
- `tests/hyperedge_cascade_queue_persistence.rs` (73 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
- `tests/hyperedge_memory_budget.rs` (90 Zeilen)
  - → `acta-types`: dev: `EntityId`
- `tests/hyperedge_payload_sharing.rs` (32 Zeilen)
  - → `acta-types`: dev: `EntityId`
- `tests/hyperedge_persistence_survives_restart.rs` (98 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
- `tests/hyperedge_suggest.rs` (152 Zeilen)
  - → `acta-ports`: dev: `Result`
- `tests/hyperedge_suggest_llm_test.rs` (110 Zeilen)
  - → `acta-ports`: dev: `Result`
- `tests/integration_graph.rs` (109 Zeilen)
  - → `acta-ports`: dev: `GraphIndex`
- `tests/j15-acta-graph-graph-nebenmodule-appr-p_arc_slice_tests.rs` (26 Zeilen)
  - → `acta-types`: dev: `EntityId`
- `tests/leiden_reference_validation.rs` (149 Zeilen)
  - → `acta-ports`: dev: `GraphIndex`
- `tests/path_rag_at_snapshot.rs` (299 Zeilen)
  - → `acta-ports`: dev: `GraphIndex`
- `tests/ppr_alloc_test.rs` (111 Zeilen)
  - → `acta-types`: dev: `PprAlgorithm::Auto`
- `tests/ppr_candidate_stream.rs` (308 Zeilen)
  - → `acta-ports`: dev: `BoxFuture`×9
  - → `acta-types`: dev: `Result`×8
- `tests/ppr_snapshot_isolation.rs` (346 Zeilen)
  - → `acta-ports`: dev: `GraphIndex`
- `tests/ppr_strategy_equivalence.rs` (96 Zeilen)
  - → `acta-ports`: dev: `graph_index::GraphIndex`
- `tests/proptest_csr_invariants.rs` (325 Zeilen)
  - → `acta-ports`: dev: `GraphIndex`
- `tests/proptest_tl_hfd_diffusion.rs` (273 Zeilen)
  - → `acta-types`: dev: `EntityId`
- `tests/proptest_tl_hfd_lovasz.rs` (261 Zeilen)
  - → `acta-types`: dev: `EntityId`
- `tests/provenance_delete_cascade.rs` (128 Zeilen)
  - → `acta-ports`: dev: `GraphIndex`
- `tests/source_doc_ids_populated.rs` (147 Zeilen)
  - → `acta-ports`: dev: `GraphIndex`
- `tests/star_expansion_equals_clique.rs` (42 Zeilen)
  - → `acta-types`: dev: `EntityId`
- `tests/tlhfd_shadow_mode_parity.rs` (281 Zeilen)
  - → `acta-types`: dev: `EntityId`

**Öffentliche Schnittstelle** (erreichbar über `pub mod`-Kette oder `pub use`)

- `(crate-root)`
  - 1 trait
    - `trait GraphIndexExt` · Methoden: remove_entity ⚠ fremd: `acta-ports`
- `apprh`
  - 2 enum, 4 fn, 9 struct, 1 trait
    - `fn run_apprh_push` ⚠ fremd: `acta-types`
    - `enum ApprhError`
    - `struct ApprhGateMonitorConfig` · 1 Methoden
    - `struct ApprhObservationRecord`
    - `struct ApprhGateMonitorSnapshot`
    - `struct ApprhGateMonitor` · 7 Methoden
    - `fn apprh_local` ⚠ fremd: `acta-types`
    - `fn forward_push_apprh` ⚠ fremd: `acta-types`
    - `struct ApprhParams` · 1 Methoden
    - `enum ApprhMode`
    - `struct ApprhSelectorConfig`
    - `struct ApprhSelector` · 8 Methoden
    - `trait ApprhFlipGate` · Methoden: should_flip
    - `struct DefaultApprhFlipGate`
    - `struct ApprhShadowComparison` ⚠ fremd: `acta-types`
    - `fn shadow_compare_forward_push_vs_apprh` ⚠ fremd: `acta-types`
- `arc_slice`
  - 1 struct
    - `struct ArcSlice` · 9 Methoden
- `cascade`
  - 3 const, 1 enum, 4 fn, 4 struct, 1 trait, 1 type
    - `const MAX_HYPEREDGE_CASCADE_FANOUT`
    - `const DEFAULT_HYPEREDGE_CASCADE_FANOUT_LIMIT`
    - `struct CascadeInvalidationReport`
    - `fn cascade_invalidate_edges_for_superseded_doc`
    - `const CASCADE_QUEUE_PREFIX`
    - `enum CascadeStatus`
    - `struct DeletionProof`
    - `struct CascadeTicket`
    - `trait GraphGarbageCollection` · Methoden: sweep_orphans
    - `struct HyperedgeCascadeReport`
    - `type CascadeReport`
    - `fn topological_sort_hyperedge_closure`
    - `fn cascade_invalidate_hyperedges_for_superseded_doc`
    - `fn verify_doc_hyperedge_deletion_completeness`
- `community`
  - 1 fn, 4 struct
    - `struct CommunityDetectionConfig`
    - `struct VirtualHyperedgeNode` · 1 Methoden
    - `struct StarExpansionIterator` · 2 Methoden
    - `struct CommunityAssignment`
    - `fn detect_communities`
- `consistency_enforcement`
  - 4 struct, 1 trait, 1 type
    - `type EdgeId`
    - `struct ConflictPattern`
    - `struct EdgeAssertion` · 1 Methoden
    - `trait ContradictionDetector` · Methoden: conflicts
    - `struct ExactPredicateConflictDetector`
    - `struct ConsistencyEnforcer` · 9 Methoden
- `csr`
  - 1 struct
    - `struct CsrGraph` *(re-exportiert)* · 53 Methoden
- `edge_reinforcement`
  - 4 fn, 1 struct
    - `struct EdgeReinforcementConfig`
    - `fn compute_edge_weight`
    - `fn apply_cooccurrence_reinforcement`
    - `fn apply_traversal_reinforcement`
    - `fn apply_weight_normalization`
- `edge_reinforcement_buffer`
  - 3 struct
    - `struct CooccurrenceSignal` ⚠ fremd: `acta-types`
    - `struct TraversalSignal` ⚠ fremd: `acta-types`
    - `struct EdgeReinforcementBuffer` · 6 Methoden
- `entity_extraction`
  - 1 fn, 2 struct, 1 trait
    - `struct ExtractedRelation` · 1 Methoden
    - `trait EntityExtractor` · Methoden: extract
    - `struct RuleBasedEntityExtractor` · 1 Methoden
    - `fn ingest_extracted_relations`
- `error`
  - 1 enum
    - `enum GraphMutationError`
- `hyperedge`
  - 2 const, 2 fn, 7 struct
    - `const HYPEREDGE_PREFIX`
    - `const HYPEREDGE_BY_ENTITY_PREFIX`
    - `fn star_weight`
    - `struct HyperEdgeId` · 2 Methoden
    - `struct RoleId` · 2 Methoden
    - `struct RoleBinding` · 1 Methoden
    - `struct HyperEdge` · 9 Methoden
    - `struct HyperEdgeView` · 3 Methoden
    - `struct RoleInterner` · 8 Methoden
    - `fn sort_dedup_entities`
    - `struct ConsolidationNodesGuard` · 4 Methoden
- `hyperedge_suggest`
  - 1 const, 1 enum, 2 fn, 2 struct
    - `const MAX_RELATE_PARTICIPANTS`
    - `struct HyperEdgeCandidate`
    - `struct ValidatedHyperEdgeCandidate`
    - `enum HyperedgeSuggestError`
    - `fn compute_co_occurrence_candidates`
    - `fn validate_candidates_with_llm` ⚠ fremd: `acta-ports`
- `path_rag`
  - 1 const, 1 fn, 7 struct, 2 trait
    - `struct KPathConfig`
    - `struct KPathResult` ⚠ fremd: `acta-types`
    - `trait KPathDiffusion` · Methoden: find_k_paths
    - `struct GraphPath` ⚠ fremd: `acta-types`
    - `trait PathGraph` · Methoden: neighbors_with_weights, predecessors_with_weights, hyperedges_for_entity, get_hyperedge, hyperedge_participants
    - `struct PprParams`
- `percolation`
  - 3 fn, 1 struct
- `ppr`
  - 4 fn, 6 struct
- `ppr_stream`
  - 1 const, 1 struct
- `provenance`
  - 2 struct
- `session_dag`
  - 2 const, 6 struct, 1 type
- `tl_hfd`
  - 2 const, 1 enum, 5 fn, 4 struct, 1 trait
  - … 45 weitere Items nicht aufgelistet (`--max-items`)
- Zusätzlich 17 `pub`-Items in privaten Modulen ohne Re-Export (nicht Teil der echten API).
- ⚠ Re-Exporte **fremder Crates**: `acta_types::EntityId`

**Eingehend (wer nutzt dieses Crate)**

| Nutzer | Symbole prod | Symbole dev |
|---|---|---|
| `acta-engine` | `hyperedge::HyperEdgeId`×2, `percolation::PercolationConfig`×2, `CsrGraph::load_from_storage`×2, `hyperedge::HyperEdge`×2, `cascade_invalidate_edges_for_superseded_doc`, `percolation::compute_percolation_health`, `percolation::should_trigger_rebonding`, `percolation::find_rebonding_candidates`, `CsrGraph`, `hyperedge::RoleId::new` … (+6) | `csr::CsrGraph`×15, `csr`, `CsrGraph::new` |
| `acta-db` | `percolation::PercolationConfig` | `CsrGraph`×10, `path_rag::PathRAGEngine`×2, `csr::CsrGraph`×2, `PathRAGEngine` |

**Nach außen ungenutzte öffentliche Items (Heuristik, 114 von 125)**: `run_apprh_push`, `ApprhError`, `ApprhGateMonitorConfig`, `ApprhObservationRecord`, `ApprhGateMonitorSnapshot`, `ApprhGateMonitor`, `apprh_local`, `forward_push_apprh`, `ApprhParams`, `ApprhMode`, `ApprhSelectorConfig`, `ApprhSelector`, `ApprhFlipGate`, `DefaultApprhFlipGate`, `ApprhShadowComparison`, `shadow_compare_forward_push_vs_apprh`, `ArcSlice`, `MAX_HYPEREDGE_CASCADE_FANOUT`, `DEFAULT_HYPEREDGE_CASCADE_FANOUT_LIMIT`, `CascadeInvalidationReport`, `CASCADE_QUEUE_PREFIX`, `CascadeStatus`, `DeletionProof`, `CascadeTicket`, `GraphGarbageCollection`, `HyperedgeCascadeReport`, `CascadeReport`, `topological_sort_hyperedge_closure`, `cascade_invalidate_hyperedges_for_superseded_doc`, `verify_doc_hyperedge_deletion_completeness` …

### acta-mcp

- Pfad: `crates/acta-mcp` · Zeilen: 9821 · Rust-Dateien: 41 · Ring: 4
- Interne Cargo-Abhängigkeiten: `acta-crypto` (normal), `acta-ports` (normal), `acta-rank` (normal), `acta-types` (normal)

**Ausgehend (Importe aus anderen Workspace-Crates)**

| Ziel | prod | dev | Dateien (prod) | Symbole (prod) | Symbole (nur dev) |
|---|---:|---:|---:|---|---|
| `acta-ports` | 19 | 51 | 5 | `plugin::PluginRegistry`×4, `BoxFuture`×3, `DriftStatusProvider`×3, `license::FeatureRing`×2, `plugin::PluginCapability`×2, `MockEmbedder::new`, `EmbeddingProvider`, `StorageEngine`, `ContextSegment::new`, `plugin::PluginRegistry::new` | `EmbeddingError`×20, `MockEmbedder`×2, `Clock` |
| `acta-types` | 9 | 4 | 5 | `ActaError`×4, `ActaError::InvalidInput`, `ActaError::NotFound`, `TenantId::try_new`, `TenantId::SYSTEM`, `TenantId` | `DocId`, `AutoExtractionMode`, `EntityId::from_key`, `TokenBudget::new` |
| `acta-crypto` | 8 | 7 | 2 | `CryptoKey`×3, `CryptoKey::try_new`×2, `TenantIsolatedKvStore::new`×2, `KvSegmentCipher::new` | `deletion_proof::DeletionProof`, `AuditChain` |
| `acta-rank` | 3 | 0 | 2 | `fusion::ProvenanceRecord`, `IsotonicCalibrator`, `IsotonicCalibrator::with_defaults` | – |

**Pro Komponente**

- `config` (554 Zeilen)
  - → `acta-ports`: prod: `MockEmbedder::new`, `BoxFuture`
  - → `acta-types`: prod: `ActaError`
- `explain` (239 Zeilen)
  - → `acta-ports`: dev: `EmbeddingError`×2, `BoxFuture`, `EmbeddingProvider`
  - → `acta-rank`: prod: `fusion::ProvenanceRecord`
- `plugin_status` (39 Zeilen)
  - → `acta-ports`: prod: `license::FeatureRing`×2, `plugin::PluginCapability`×2, `plugin::PluginRegistry`
- `prompt_injection` (1204 Zeilen)
  - → `acta-ports`: dev: `Clock`
- `protocol` (73 Zeilen)
  - → `acta-types`: prod: `ActaError`×2, `ActaError::InvalidInput`, `ActaError::NotFound`
- `routing` (285 Zeilen)
  - → `acta-crypto`: prod: `TenantIsolatedKvStore::new`×2, `CryptoKey::try_new`, `KvSegmentCipher::new`
  - → `acta-ports`: prod: `DriftStatusProvider`×3, `BoxFuture`×2
  - → `acta-rank`: prod: `IsotonicCalibrator`, `IsotonicCalibrator::with_defaults`
  - → `acta-types`: dev: `DocId`
- `sandbox` (823 Zeilen)
  - → `acta-crypto`: prod: `CryptoKey`×3, `CryptoKey::try_new` · dev: `CryptoKey::try_new`×2
- `server` (197 Zeilen)
  - → `acta-ports`: prod: `plugin::PluginRegistry`×3, `EmbeddingProvider`
  - → `acta-types`: prod: `ActaError`
- `server_tools` (870 Zeilen)
  - → `acta-ports`: prod: `StorageEngine`, `ContextSegment::new`, `plugin::PluginRegistry::new`
  - → `acta-types`: prod: `TenantId::try_new`, `TenantId::SYSTEM`
- `tests` (1207 Zeilen)
  - → `acta-crypto`: dev: `deletion_proof::DeletionProof`, `TenantIsolatedKvStore::new`, `CryptoKey::try_new`, `KvSegmentCipher::new`
  - → `acta-ports`: dev: `EmbeddingError`×2, `BoxFuture`, `EmbeddingProvider`
- `tools_crud` (175 Zeilen)
  - → `acta-types`: prod: `TenantId`
- `tests/campaign_j42_mcp_audit.rs` (76 Zeilen)
  - → `acta-crypto`: dev: `AuditChain`
  - → `acta-types`: dev: `AutoExtractionMode`
- `tests/cloud_query_scoped_guard.rs` (160 Zeilen)
  - → `acta-ports`: dev: `EmbeddingError`×2, `BoxFuture`, `EmbeddingProvider`
- `tests/confirm_required_semantics.rs` (186 Zeilen)
  - → `acta-ports`: dev: `BoxFuture`×2, `EmbeddingError`×2, `EmbeddingProvider`
- `tests/acta_delete_no_unverified_proof.rs` (175 Zeilen)
  - → `acta-ports`: dev: `BoxFuture`×2, `EmbeddingError`×2, `EmbeddingProvider`
- `tests/j18-acta-mcp_wiring_test.rs` (129 Zeilen)
  - → `acta-ports`: dev: `MockEmbedder`, `BoxFuture`, `EmbeddingProvider`, `plugin::PluginRegistry`
- `tests/j18closure-acta-mcp_wiring_test.rs` (66 Zeilen)
  - → `acta-ports`: dev: `MockEmbedder`, `EmbeddingProvider`
- `tests/license_gate_entrypoint.rs` (108 Zeilen)
  - → `acta-ports`: dev: `EmbeddingError`×2, `license::FeatureRing`, `BoxFuture`, `EmbeddingProvider`
- `tests/mcp_relate_tools.rs` (353 Zeilen)
  - → `acta-ports`: dev: `EmbeddingError`×2, `EmbeddingProvider`
  - → `acta-types`: dev: `EntityId::from_key`
- `tests/mcp_test.rs` (1129 Zeilen)
  - → `acta-ports`: dev: `EmbeddingError`×2, `BoxFuture`, `EmbeddingProvider`
  - → `acta-types`: dev: `TokenBudget::new`
- `tests/plugin_status_tool.rs` (144 Zeilen)
  - → `acta-ports`: dev: `EmbeddingError`×2, `BoxFuture`, `license::FeatureRing`, `EmbeddingProvider`
- `tests/tool_registry_classification.rs` (167 Zeilen)
  - → `acta-ports`: dev: `BoxFuture`×2, `EmbeddingError`×2, `EmbeddingProvider`

**Öffentliche Schnittstelle** (erreichbar über `pub mod`-Kette oder `pub use`)

- `config`
  - 2 fn, 4 struct
    - `struct EmbeddingConfig` · 2 Methoden
    - `fn create_embedding_provider` ⚠ fremd: `acta-types`
    - `struct LlmConfig` · 2 Methoden
    - `fn create_llm_text_generator` ⚠ fremd: `acta-types`
    - `struct RouterConfig` · 1 Methoden
    - `struct MockLlmGenerator`
- `explain`
  - 2 struct
    - `struct ExplainRequest`
    - `struct ExplainResponse` ⚠ fremd: `acta-rank`
- `io`
  - 2 const, 1 fn
    - `const MAX_RPC_BYTES`
    - `const MAX_SEARCH_QUERY_BYTES`
    - `fn read_line_bounded`
- `plugin_status`
  - 1 fn, 2 struct
    - `struct PluginStatusResponse` ⚠ fremd: `acta-ports`
    - `struct PluginStatusEntry` ⚠ fremd: `acta-ports`
    - `fn handle_plugin_status` ⚠ fremd: `acta-ports`
- `proof_key`
  - 1 fn
    - `fn deletion_proof_key_from_env`
- `proof_key_env`
  - 1 enum, 2 fn, 1 struct
    - `enum ProofKeySource`
    - `struct ProofKeyResolution`
    - `fn resolve_proof_key`
    - `fn resolve_proof_key_from_env`
- `protocol`
  - 1 enum, 1 fn
    - `enum McpError` · 6 Methoden
    - `fn response_from_error`
- `routing`
  - 2 fn, 1 struct
    - `struct RoutingHandle` ⚠ fremd: `acta-ports`, `acta-rank`
    - `fn setup_routing`
    - `fn setup_kv_bridge`
- `sandbox`
  - 4 const, 1 enum, 4 struct
    - `const MAX_VOLATILE_RESULTS`
    - `const MAX_VOLATILE_KEY_BYTES`
    - `const MAX_VOLATILE_OUTPUT_BYTES`
    - `struct ToolDefinition`
    - `const TOOL_REGISTRY`
    - `enum ToolCategory`
    - `struct SandboxPolicy`
    - `struct VolatileToolResult` · 2 Methoden
    - `struct McpSandbox` · 8 Methoden
- `server`
  - 1 struct
    - `struct McpServer` · 12 Methoden ⚠ fremd: `acta-ports`
- `validation`
  - 2 fn
    - `fn is_write_allowed_by_env`
    - `fn validate_collection_name`
- Zusätzlich 9 `pub`-Items in privaten Modulen ohne Re-Export (nicht Teil der echten API).

**Eingehend (wer nutzt dieses Crate)**

| Nutzer | Symbole prod | Symbole dev |
|---|---|---|
| `acta-mcp-fuzz` | `protocol`, `prompt_injection` | – |

**Nach außen ungenutzte öffentliche Items (Heuristik, 36 von 36)**: `EmbeddingConfig`, `create_embedding_provider`, `LlmConfig`, `create_llm_text_generator`, `RouterConfig`, `MockLlmGenerator`, `ExplainRequest`, `ExplainResponse`, `MAX_RPC_BYTES`, `MAX_SEARCH_QUERY_BYTES`, `read_line_bounded`, `PluginStatusResponse`, `PluginStatusEntry`, `handle_plugin_status`, `deletion_proof_key_from_env`, `ProofKeySource`, `ProofKeyResolution`, `resolve_proof_key`, `resolve_proof_key_from_env`, `McpError`, `response_from_error`, `RoutingHandle`, `setup_routing`, `setup_kv_bridge`, `MAX_VOLATILE_RESULTS`, `MAX_VOLATILE_KEY_BYTES`, `MAX_VOLATILE_OUTPUT_BYTES`, `ToolDefinition`, `TOOL_REGISTRY`, `ToolCategory` …

### acta-mcp-fuzz

- Pfad: `crates/acta-mcp/fuzz` · Zeilen: 91 · Rust-Dateien: 2
- Interne Cargo-Abhängigkeiten: `acta-mcp` (normal)

**Ausgehend (Importe aus anderen Workspace-Crates)**

| Ziel | prod | dev | Dateien (prod) | Symbole (prod) | Symbole (nur dev) |
|---|---:|---:|---:|---|---|
| `acta-mcp` | 2 | 0 | 2 | `protocol`, `prompt_injection` | – |

**Pro Komponente**

- `fuzz_targets/fuzz_jsonrpc_parsing.rs` (32 Zeilen)
  - → `acta-mcp`: prod: `protocol`
- `fuzz_targets/fuzz_prompt_injection_guard.rs` (59 Zeilen)
  - → `acta-mcp`: prod: `prompt_injection`

**Öffentliche Schnittstelle** (erreichbar über `pub mod`-Kette oder `pub use`)

Keine pub-Items erkannt (Bibliothek ohne lib.rs, nur Binary, oder alles intern).

**Eingehend (wer nutzt dieses Crate)**

Wird von keinem anderen analysierten Workspace-Crate importiert.

### acta-mvcc

- Pfad: `crates/acta-mvcc` · Zeilen: 4572 · Rust-Dateien: 22 · Ring: 0
- Interne Cargo-Abhängigkeiten: `acta-ports` (normal), `acta-types` (normal)

**Ausgehend (Importe aus anderen Workspace-Crates)**

| Ziel | prod | dev | Dateien (prod) | Symbole (prod) | Symbole (nur dev) |
|---|---:|---:|---:|---|---|
| `acta-ports` | 5 | 2 | 1 | `MetricsSink`×5 | `metrics` |
| `acta-types` | 1 | 3 | 1 | `*` | `TxId`×2, `DocId` |

**Pro Komponente**

- `(crate-root)` (21 Zeilen)
  - → `acta-types`: prod: `*`
- `ssi` (1008 Zeilen)
  - → `acta-ports`: prod: `MetricsSink`×5
- `benches/mvcc_bench.rs` (63 Zeilen)
  - → `acta-types`: dev: `DocId`
- `tests/campaign_cov_ssi_coarsening.rs` (46 Zeilen)
  - → `acta-types`: dev: `TxId`
- `tests/campaign_cov_tx_buffer_reap.rs` (48 Zeilen)
  - → `acta-types`: dev: `TxId`
- `tests/ssi_metrics_observability.rs` (119 Zeilen)
  - → `acta-ports`: dev: `MetricsSink`, `metrics`

**Öffentliche Schnittstelle** (erreichbar über `pub mod`-Kette oder `pub use`)

- `seq_log`
  - 1 const, 1 enum, 2 struct
    - `const DEFAULT_MAX_PIN_DURATION`
    - `struct SeqLogEntry` · 1 Methoden
    - `enum SeqLogChange` · 2 Methoden
    - `struct SequenceLog` · 19 Methoden
- `snapshot`
  - 2 struct
    - `struct SnapshotRegistry` · 9 Methoden
    - `struct SnapshotGuard` · 1 Methoden
- `ssi`
  - 1 const, 4 struct, 1 trait
    - `const DEFAULT_MAX_TRACKED_COMMIT_KEYS`
    - `struct ReadSet` · 13 Methoden
    - `trait SsiValidator` · Methoden: validate
    - `struct PruningBlockerInfo`
    - `struct SequenceLogSsiValidator` · 19 Methoden
    - `struct SequenceLogSsiValidatorBuilder` · 7 Methoden
- `tx_buffer`
  - 6 const, 1 enum, 2 struct, 1 trait
    - `const STAGING_ENTRY_OVERHEAD_BYTES`
    - `const DEFAULT_SHARD_COUNT`
    - `const DEFAULT_MAX_OPS_PER_TX`
    - `const DEFAULT_MAX_READ_SET_KEYS`
    - `const DEFAULT_MAX_TX_STAGED_BYTES`
    - `const DEFAULT_MAX_TOTAL_STAGED_BYTES`
    - `struct TxBufferConfig`
    - `enum IndexOp` · 2 Methoden
    - `trait StagedOpSize` · Methoden: staged_bytes
    - `struct TxBuffer` · 36 Methoden
- ⚠ Re-Exporte **fremder Crates**: `acta_types::*`

**Eingehend (wer nutzt dieses Crate)**

| Nutzer | Symbole prod | Symbole dev |
|---|---|---|
| `acta-core` | `seq_log`×2, `snapshot`×2, `tx_buffer`×2 | – |
| `acta-store` | `tx_buffer::STAGING_ENTRY_OVERHEAD_BYTES`×2, `SequenceLogSsiValidator`, `SsiValidator`, `SequenceLogSsiValidator::new_with_bounds` | – |
| `acta-engine` | `tx_buffer::TxBuffer` | `tx_buffer::IndexOp` |

**Nach außen ungenutzte öffentliche Items (Heuristik, 17 von 22)**: `DEFAULT_MAX_PIN_DURATION`, `SeqLogEntry`, `SeqLogChange`, `SequenceLog`, `SnapshotRegistry`, `SnapshotGuard`, `DEFAULT_MAX_TRACKED_COMMIT_KEYS`, `ReadSet`, `PruningBlockerInfo`, `SequenceLogSsiValidatorBuilder`, `DEFAULT_SHARD_COUNT`, `DEFAULT_MAX_OPS_PER_TX`, `DEFAULT_MAX_READ_SET_KEYS`, `DEFAULT_MAX_TX_STAGED_BYTES`, `DEFAULT_MAX_TOTAL_STAGED_BYTES`, `TxBufferConfig`, `StagedOpSize`

### acta-ports

- Pfad: `crates/acta-ports` · Zeilen: 3602 · Rust-Dateien: 25 · Ring: 0
- Interne Cargo-Abhängigkeiten: `acta-types` (normal)

**Ausgehend (Importe aus anderen Workspace-Crates)**

| Ziel | prod | dev | Dateien (prod) | Symbole (prod) | Symbole (nur dev) |
|---|---:|---:|---:|---|---|
| `acta-types` | 9 | 0 | 4 | `ActaError`×3, `types::domain`, `model_fingerprint::ModelFingerprint`, `types::domain::TenantId`, `DocId`, `*`, `ActaError::Plugin` | – |

**Pro Komponente**

- `(crate-root)` (130 Zeilen)
  - → `acta-types`: prod: `*`
- `graph` (185 Zeilen)
  - → `acta-types`: prod: `ActaError`, `types::domain`
- `kv` (231 Zeilen)
  - → `acta-types`: prod: `model_fingerprint::ModelFingerprint`, `types::domain::TenantId`, `ActaError`, `DocId`
- `plugin` (387 Zeilen)
  - → `acta-types`: prod: `ActaError`, `ActaError::Plugin`

**Öffentliche Schnittstelle** (erreichbar über `pub mod`-Kette oder `pub use`)

- `(crate-root)`
  - 2 type
    - `type BoxFuture`
    - `type BoxStream`
- `attention`
  - 1 struct, 1 trait
    - `struct RequestId`
    - `trait AttentionExporter` · Methoden: export_attention_weights
- `checkpoint`
  - 2 trait
    - `trait Checkpoint` · Methoden: take_snapshot, restore
    - `trait CheckpointCoordinator` · Methoden: create_named_checkpoint, restore_named_checkpoint, drop_named_checkpoint, list_named_checkpoints
- `clock`
  - 1 struct, 1 trait
    - `trait Clock` · Methoden: now_unix_nanos, monotonic_nanos
    - `struct SystemClock` · 1 Methoden
- `embedding`
  - 1 enum, 2 struct, 4 trait
    - `struct ContextSegment` · 4 Methoden
    - `trait TextEmbeddingEngine` · Methoden: embed, embed_batch
    - `enum EmbeddingError`
    - `trait EmbeddingProvider` · Methoden: provider_name, embed, embedding_dim, embed_batch
    - `trait LlmTextGenerator` · Methoden: generate, generate_with_context
    - `trait LlmTextGeneratorStreaming` · Methoden: generate_stream
    - `struct MockEmbedder` · 3 Methoden
- `graph`
  - 1 enum, 3 struct, 1 trait
    - `struct RoleId` · 2 Methoden
    - `struct RoleBinding` · 1 Methoden
    - `struct HyperEdgeId` · 2 Methoden
    - `enum GraphMutationError`
    - `trait GraphCollectionMutation` · Methoden: relate_n_ary
- `graph_index`
  - 1 struct, 2 trait
    - `trait GraphIndex` · Methoden: traverse, neighbors, remove_edge, add_bidirectional, multi_traverse, multi_traverse_at…
    - `struct GraphIndexStats`
    - `trait CommunityResolver` · Methoden: get_community
- `id_gen`
  - 1 struct, 1 trait
    - `trait IdGen` · Methoden: next_id
    - `struct SequentialIdGen` · 1 Methoden
- `kv`
  - 5 struct, 2 trait
    - `struct KvLayout`
    - `struct RopeConfig`
    - `struct PrefixKey` ⚠ fremd: `acta-types`
    - `struct KvBlock`
    - `struct KvPrefixHit`
    - `trait KvPrefixStore` · Methoden: lookup, insert, evict
    - `trait KvLifecycleHooks` · Methoden: on_rollback, remove_doc_segments, purge_tenant
- `kv_bridge_port`
  - 1 trait
    - `trait KvBridgeStorage` · Methoden: get, put, commit
- `license`
  - 3 enum, 2 struct, 1 trait
    - `enum VectorDeleteMode`
    - `enum FeatureRing`
    - `struct AuthorizedRing` · 1 Methoden
    - `enum LicenseError`
    - `trait LicenseGate` · Methoden: check_ring, authorize
    - `struct OpenFastGate`
- `lifecycle`
  - 1 struct, 3 trait
    - `struct GroundingAssessment`
    - `trait GroundingValidator` · Methoden: validate_grounding
    - `trait ResponseGroundingValidator` · Methoden: score_grounding
    - `trait ContextPreparer` · Methoden: prepare_context
- `metrics`
  - 1 enum, 2 struct, 1 trait
    - `trait MetricsSink` · Methoden: record_counter, record_gauge, record_histogram
    - `struct NoopMetricsSink`
    - `enum MetricEvent`
    - `struct TestMetricsSink` · 3 Methoden
- `observability`
  - 1 trait
    - `trait DriftStatusProvider` · Methoden: overall_drift_status
- `plugin`
  - 1 enum, 2 struct, 1 trait
    - `struct PluginCapability` · 1 Methoden
    - `trait PluginManifest` · Methoden: name, requires, conflicts, feature_ring_required, capability, activate
    - `enum PluginError`
    - `struct PluginRegistry` · 6 Methoden
- `query_rewriter`
  - 1 trait
    - `trait QueryRewriter` · Methoden: rewrite, rewrite_structured
- `reranker`
  - 1 trait
    - `trait Reranker` · Methoden: rerank, record_implicit_feedback, rerank_deadline_ms
- `rng`
  - 1 struct, 1 trait
    - `trait Rng` · Methoden: next_u64, fill_bytes, next_unit_f64
    - `struct SeededRng` · 2 Methoden
- `storage`
  - 1 const, 1 struct, 1 trait
    - `struct StorageStats`
    - `const MAX_SCAN_MERGE_ACCUMULATOR`
    - `trait StorageEngine` · Methoden: get, get_at_seq, get_tracked, get_at_seq_tracked, scan_prefix_tracked, supports_ssi_tracking…
- `text_index`
  - 1 struct, 2 trait
    - `trait SegmentSynthesizer` · Methoden: synthesize_segment, model_id
    - `struct TextIndexStats`
    - `trait TextIndex` · Methoden: search, search_at, insert, delete, commit, rollback…
- `vector_index`
  - 1 struct, 2 trait
    - `struct VectorIndexStats`
    - `trait VectorIndex` · Methoden: insert, all_doc_ids, insert_batch, search, search_at, search_filtered…
    - `trait HybridSearchProvider` · Methoden: search_hybrid
- Interne Re-Exporte (`pub use`): `TextEmbeddingEngine`
- ⚠ Re-Exporte **fremder Crates**: `acta_types::*`

**Eingehend (wer nutzt dieses Crate)**

| Nutzer | Symbole prod | Symbole dev |
|---|---|---|
| `acta-engine` | `MetricsSink`×9, `StorageEngine`×5, `KvLifecycleHooks`×4, `VectorIndexStats`×2, `DriftStatusProvider`×2, `VectorIndex`, `Reranker`, `license`, `SystemClock::new`, `NoopMetricsSink` … (+6) | `Clock` |
| `acta-graph` | `GraphIndex`×3, `StorageEngine`×2, `BoxFuture`×2, `graph::GraphMutationError::Internal`×2, `graph::RoleBinding`, `graph::HyperEdgeId`, `graph::GraphMutationError`, `graph::GraphMutationError::LockAcquisitionTimeout`, `graph::GraphMutationError::RoleBindingInvalid`, `graph::GraphMutationError::EpochReclamationPending` … (+5) | `Result`×2, `graph`, `graph_index::GraphIndex` |
| `acta-mcp` | `plugin::PluginRegistry`×4, `BoxFuture`×3, `DriftStatusProvider`×3, `license::FeatureRing`×2, `plugin::PluginCapability`×2, `MockEmbedder::new`, `EmbeddingProvider`, `StorageEngine`, `ContextSegment::new`, `plugin::PluginRegistry::new` | `EmbeddingError`×20, `MockEmbedder`×2, `Clock` |
| `acta-store` | `MetricsSink`×6, `Clock`×4, `SystemClock::new`, `license::FeatureRing`, `license::FeatureRing::Sovereign`, `NoopMetricsSink` | `StorageEngine`×4, `StorageStats`, `storage::StorageEngine`, `SystemClock` |
| `acta-vector` | `MetricsSink`×6, `Rng`×3, `NoopMetricsSink`×2, `SeededRng::new`×2 | `SeededRng`×5 |
| `acta-mvcc` | `MetricsSink`×5 | `metrics` |
| `acta-text` | `StorageEngine`×5 | `BoxFuture`×50, `StorageStats`×31 |
| `acta-core` | `*`×2 | – |
| `acta-crypto` | `Clock`×2 | `SystemClock`×9 |
| `acta-db` | `QueryRewriter` | `GraphIndex`×4, `StorageEngine`×3, `BoxFuture`×2, `LlmTextGenerator` … (+3) |

*Hinweis: Mindestens ein Crate importiert per Glob (`*`); ungenutzte Items sind deshalb nicht bestimmbar.*

### acta-rank

- Pfad: `crates/acta-rank` · Zeilen: 6069 · Rust-Dateien: 42 · Ring: 0
- Interne Cargo-Abhängigkeiten: `acta-ports` (normal), `acta-types` (normal)

**Ausgehend (Importe aus anderen Workspace-Crates)**

| Ziel | prod | dev | Dateien (prod) | Symbole (prod) | Symbole (nur dev) |
|---|---:|---:|---:|---|---|
| `acta-types` | 10 | 9 | 8 | `ConfigFingerprint`×3, `ActaError`, `FusionWeights`, `FusionStrategy`, `SignalFusionStrategies`, `MAX_SEARCH_K`, `OnSignalFailure`, `DocId` | – |

**Pro Komponente**

- `calibration` (635 Zeilen)
  - → `acta-types`: prod: `ConfigFingerprint`×3
- `fusion` (2051 Zeilen)
  - → `acta-types`: prod: `ActaError`, `FusionWeights`, `FusionStrategy`, `SignalFusionStrategies`, `MAX_SEARCH_K`, `OnSignalFailure`, `DocId`
- `tests/calibration_deep_tests.rs` (315 Zeilen)
  - → `acta-types`: dev: `ConfigFingerprint`
- `tests/conformal_tests.rs` (176 Zeilen)
  - → `acta-types`: dev: `ConfigFingerprint`
- `tests/dibud_equivalence.rs` (210 Zeilen)
  - → `acta-types`: dev: `DocId`
- `tests/global_fusion_community_aware.rs` (111 Zeilen)
  - → `acta-types`: dev: `ActaError`
- `tests/j12closure-acta-rank_dibud_provenance.rs` (24 Zeilen)
  - → `acta-types`: dev: `DocId`
- `tests/rank_tests.rs` (290 Zeilen)
  - → `acta-types`: dev: `ConfigFingerprint`
- `tests/rrf_reference.rs` (99 Zeilen)
  - → `acta-types`: dev: `DocId`
- `tests/rrf_two_phase_scaling.rs` (173 Zeilen)
  - → `acta-types`: dev: `FusionStrategy`
- `tests/score_normalized_fallback.rs` (158 Zeilen)
  - → `acta-types`: dev: `FusionStrategy`

**Öffentliche Schnittstelle** (erreichbar über `pub mod`-Kette oder `pub use`)

- `calibration`
  - 1 const, 1 enum, 3 struct, 1 trait
    - `enum ConformalError`
    - `trait ConformalCalibrator` · Methoden: update, threshold
    - `struct AdaptiveConformalCalibrator` · 8 Methoden
    - `const DEFAULT_ECE_REBUILD_THRESHOLD`
    - `struct IsotonicCalibrator` · 13 Methoden
    - `struct PlattScaler` · 11 Methoden
- `dibud`
  - 2 enum, 2 fn, 3 struct
    - `fn fuse_exact_prefix`
    - `fn fuse_exact_prefix_async`
    - `enum DiBudStep`
    - `struct DiBudFusionState` · 8 Methoden
    - `enum BudgetedChannel` · 3 Methoden
    - `struct FusionBudget` · 1 Methoden
    - `struct DiBudOutcome`
- `drift`
  - 1 enum, 1 struct
    - `enum DriftStatus` · 1 Methoden
    - `struct DriftDetector` · 6 Methoden
- `explain`
  - 1 enum, 1 fn, 2 struct
    - `enum SignalKind` · 1 Methoden
    - `struct ExplanationEntry`
    - `struct RetrievalExplanation` · 1 Methoden
    - `fn explain`
- `fusion`
  - 2 struct
    - `struct ProvenanceBuilder` *(re-exportiert)* · 10 Methoden
    - `struct BoundedTopK` *(re-exportiert)* · 6 Methoden
- Zusätzlich 25 `pub`-Items in privaten Modulen ohne Re-Export (nicht Teil der echten API).
- ⚠ Re-Exporte **fremder Crates**: `acta_types::FusionStrategy`, `acta_types::OnSignalFailure`

**Eingehend (wer nutzt dieses Crate)**

| Nutzer | Symbole prod | Symbole dev |
|---|---|---|
| `acta-engine` | `IsotonicCalibrator`×2, `fusion`, `fusion::fuse_search_results_with_strategy`, `fusion::fuse_search_results_with_signal_strategies`, `fusion::fuse_signals` | – |
| `acta-mcp` | `fusion::ProvenanceRecord`, `IsotonicCalibrator`, `IsotonicCalibrator::with_defaults` | – |
| `acta-db` | – | `IsotonicCalibrator::new` |

**Nach außen ungenutzte öffentliche Items (Heuristik, 20 von 21)**: `ConformalError`, `ConformalCalibrator`, `AdaptiveConformalCalibrator`, `DEFAULT_ECE_REBUILD_THRESHOLD`, `PlattScaler`, `fuse_exact_prefix`, `fuse_exact_prefix_async`, `DiBudStep`, `DiBudFusionState`, `BudgetedChannel`, `FusionBudget`, `DiBudOutcome`, `DriftStatus`, `DriftDetector`, `SignalKind`, `ExplanationEntry`, `RetrievalExplanation`, `explain`, `ProvenanceBuilder`, `BoundedTopK`

**Auffälligkeiten**

- acta-ports: kein Code-Zugriff gefunden (prüfen: Makro, Feature, cfg, oder entfernen)

### acta-simd

- Pfad: `crates/acta-simd` · Zeilen: 1472 · Rust-Dateien: 9 · Ring: 0
- Interne Cargo-Abhängigkeiten: `acta-core` (normal)

**Ausgehend (Importe aus anderen Workspace-Crates)**

| Ziel | prod | dev | Dateien (prod) | Symbole (prod) | Symbole (nur dev) |
|---|---:|---:|---:|---|---|
| `acta-core` | 5 | 0 | 2 | `Result`×4, `ActaError` | – |

**Pro Komponente**

- `(crate-root)` (70 Zeilen)
  - → `acta-core`: prod: `Result`×4
- `dispatch` (203 Zeilen)
  - → `acta-core`: prod: `ActaError`

**Öffentliche Schnittstelle** (erreichbar über `pub mod`-Kette oder `pub use`)

- `(crate-root)`
  - 4 fn
    - `fn validate_vector` ⚠ fremd: `acta-core`
    - `fn compute_distance` ⚠ fremd: `acta-core`
    - `fn compute_distance_trusted` ⚠ fremd: `acta-core`
    - `fn compute_distance_f32_bytes_trusted` ⚠ fremd: `acta-core`
- `dispatch`
  - 9 fn
    - `fn cosine_distance` ⚠ fremd: `acta-core`
    - `fn euclidean_distance` ⚠ fremd: `acta-core`
    - `fn dot_product_distance` ⚠ fremd: `acta-core`
    - `fn cosine_distance_f32_bytes` ⚠ fremd: `acta-core`
    - `fn euclidean_distance_f32_bytes` ⚠ fremd: `acta-core`
    - `fn dot_product_distance_f32_bytes` ⚠ fremd: `acta-core`
    - `fn dot_product_u8` ⚠ fremd: `acta-core`
    - `fn euclidean_distance_sq_u8` ⚠ fremd: `acta-core`
    - `fn cosine_similarity_parts_u8` ⚠ fremd: `acta-core`
- `kernels`
  - 38 fn, 2 struct
    - `fn cosine_distance_avx2`
    - `fn euclidean_distance_avx2`
    - `fn dot_product_avx2`
    - `fn cosine_distance_f32_bytes_avx2`
    - `fn euclidean_distance_f32_bytes_avx2`
    - `fn dot_product_f32_bytes_avx2`
    - `fn hsum256_ps_avx`
    - `fn dot_product_u8_avx2`
    - `fn euclidean_distance_sq_u8_avx2`
    - `fn cosine_similarity_parts_u8_avx2`
    - `fn hsum256_epi32_avx2`
    - `fn cosine_distance_avx512`
    - `fn euclidean_distance_avx512`
    - `fn dot_product_avx512`
    - `fn cosine_distance_f32_bytes_avx512`
    - `fn euclidean_distance_f32_bytes_avx512`
    - `fn dot_product_f32_bytes_avx512`
    - `fn hsum512_ps_avx`
    - `fn dot_product_u8_avx512vnni`
    - `fn euclidean_distance_sq_u8_avx512`
    - `fn cosine_similarity_parts_u8_avx512`
    - `fn hsum512_epi32_avx512`
    - `fn cosine_distance_neon`
    - `fn euclidean_distance_neon`
    - `fn dot_product_neon`
    - `struct CosineSimilarityPartsU8`
    - `struct CosineSimilarityPartsF32U8`
    - `fn cosine_distance_scalar`
    - `fn euclidean_distance_scalar`
    - `fn dot_product_scalar`
    - `fn cosine_distance_f32_bytes_scalar`
    - `fn euclidean_distance_f32_bytes_scalar`
    - `fn dot_product_f32_bytes_scalar`
    - `fn dot_product_u8_scalar`
    - `fn euclidean_distance_sq_u8_scalar`
    - `fn cosine_similarity_parts_u8_scalar`
    - `fn dot_product_f32_u8`
    - `fn euclidean_distance_sq_f32_u8`
    - `fn cosine_similarity_parts_f32_u8`
    - `fn normalize_inplace`

**Eingehend (wer nutzt dieses Crate)**

| Nutzer | Symbole prod | Symbole dev |
|---|---|---|
| `acta-vector` | `validate_vector`×2, `kernels::scalar` | `kernels::scalar::dot_product_scalar`, `kernels::scalar::euclidean_distance_scalar`, `kernels::scalar::cosine_distance_scalar`, `kernels::scalar::dot_product_u8_scalar` … (+2) |

**Nach außen ungenutzte öffentliche Items (Heuristik, 46 von 53)**: `cosine_distance`, `euclidean_distance`, `dot_product_distance`, `cosine_distance_f32_bytes`, `euclidean_distance_f32_bytes`, `dot_product_distance_f32_bytes`, `dot_product_u8`, `euclidean_distance_sq_u8`, `cosine_similarity_parts_u8`, `cosine_distance_avx2`, `euclidean_distance_avx2`, `dot_product_avx2`, `cosine_distance_f32_bytes_avx2`, `euclidean_distance_f32_bytes_avx2`, `dot_product_f32_bytes_avx2`, `hsum256_ps_avx`, `dot_product_u8_avx2`, `euclidean_distance_sq_u8_avx2`, `cosine_similarity_parts_u8_avx2`, `hsum256_epi32_avx2`, `cosine_distance_avx512`, `euclidean_distance_avx512`, `dot_product_avx512`, `cosine_distance_f32_bytes_avx512`, `euclidean_distance_f32_bytes_avx512`, `dot_product_f32_bytes_avx512`, `hsum512_ps_avx`, `dot_product_u8_avx512vnni`, `euclidean_distance_sq_u8_avx512`, `cosine_similarity_parts_u8_avx512` …

### acta-store

- Pfad: `crates/acta-store` · Zeilen: 46092 · Rust-Dateien: 231 · Ring: 1
- Interne Cargo-Abhängigkeiten: `acta-graph` (dev), `acta-vector` (dev), `acta-core` (normal), `acta-crypto` (normal), `acta-mvcc` (normal), `acta-ports` (normal), `acta-sys` (normal), `acta-types` (normal)

**Ausgehend (Importe aus anderen Workspace-Crates)**

| Ziel | prod | dev | Dateien (prod) | Symbole (prod) | Symbole (nur dev) |
|---|---:|---:|---:|---|---|
| `acta-core` | 35 | 115 | 14 | `MAX_SCAN_MERGE_ACCUMULATOR`×6, `ActaError::Storage`×5, `TOMBSTONE_BIT`×4, `TxId`×4, `ResourceTracker`×2, `ActaError::MemoryBudgetExceeded`×2, `ActaError::Internal`×2, `Result`×2, `ActaError`×2, `TxId::INTERNAL_BASE`×2 … (+4) | `ResourceBudget`×28, `ResourceTracker::new`×27, `StorageEngine`×2, `SnapshotRegistry::new`×2, `ActaError::InvalidInput`×2 … (+4) |
| `acta-ports` | 14 | 10 | 5 | `MetricsSink`×6, `Clock`×4, `SystemClock::new`, `license::FeatureRing`, `license::FeatureRing::Sovereign`, `NoopMetricsSink` | `StorageEngine`×4, `StorageStats`, `storage::StorageEngine`, `SystemClock` |
| `acta-crypto` | 5 | 2 | 5 | `wal_crypto`×2, `verify_wal_chain_completeness`, `wal_crypto::WalHmac`, `crypto::KeyManager` | – |
| `acta-mvcc` | 5 | 1 | 5 | `tx_buffer::STAGING_ENTRY_OVERHEAD_BYTES`×2, `SequenceLogSsiValidator`, `SsiValidator`, `SequenceLogSsiValidator::new_with_bounds` | – |
| `acta-sys` | 3 | 3 | 2 | `mmap_readonly`×2, `set_restrictive_file_acl` | `reopen_and_dup2`×2, `verify_file_acl_owner_only` |
| `acta-types` | 0 | 2 | 0 | – | `TxId`, `TenantId` |

**Pro Komponente**

- `compaction` (3215 Zeilen)
  - → `acta-core`: prod: `ResourceTracker`×2, `ActaError::MemoryBudgetExceeded`×2, `ActaError::Internal`×2, `ActaError::Sandbox`, `Result`, `TOMBSTONE_BIT` · dev: `ResourceTracker::new`×26, `ResourceBudget`×26, `TxId::new`×3, `ActaError::MemoryBudgetExceeded`×2 … (+3)
  - → `acta-ports`: prod: `Clock`×2, `SystemClock::new`
- `kv` (211 Zeilen)
  - → `acta-core`: prod: `ActaError`
- `lsm` (6144 Zeilen)
  - → `acta-core`: prod: `MAX_SCAN_MERGE_ACCUMULATOR`×6, `ActaError::Storage`×5, `TxId`×4, `ActaError`, `ActaError::DurabilityConfig`, `Result`, `TxId::new`, `TxId::INTERNAL_BASE`, `StorageStats` · dev: `TOMBSTONE_BIT`
  - → `acta-crypto`: prod: `verify_wal_chain_completeness`
  - → `acta-mvcc`: prod: `tx_buffer::STAGING_ENTRY_OVERHEAD_BYTES`, `SequenceLogSsiValidator`, `SsiValidator`, `SequenceLogSsiValidator::new_with_bounds`
  - → `acta-ports`: prod: `MetricsSink`×5, `Clock`×2, `license::FeatureRing`, `license::FeatureRing::Sovereign`, `NoopMetricsSink`
- `memtable` (918 Zeilen)
  - → `acta-core`: dev: `TOMBSTONE_BIT`
  - → `acta-mvcc`: prod: `tx_buffer::STAGING_ENTRY_OVERHEAD_BYTES`
- `sstable` (3316 Zeilen)
  - → `acta-core`: prod: `TOMBSTONE_BIT`×3, `TxId::INTERNAL_BASE` · dev: `ActaError`
- `system_pressure` (321 Zeilen)
  - → `acta-ports`: prod: `MetricsSink`
- `wal` (7189 Zeilen)
  - → `acta-core`: dev: `TxId`×6
  - → `acta-crypto`: prod: `wal_crypto`×2, `wal_crypto::WalHmac`, `crypto::KeyManager` · dev: `wal_crypto::WalHmac`, `crypto::KeyManager`
  - → `acta-sys`: prod: `mmap_readonly`×2, `set_restrictive_file_acl` · dev: `verify_file_acl_owner_only`
- `benches/wal_bench.rs` (140 Zeilen)
  - → `acta-core`: dev: `TxId`
- `fuzz/fuzz_targets/fuzz_recovery_arbitrary_state.rs` (52 Zeilen)
  - → `acta-core`: dev: `StorageEngine`
- `fuzz/fuzz_targets/wal_mutation_chaos.rs` (272 Zeilen)
  - → `acta-core`: dev: `TxId`
- `tests/adaptive_compaction_determinism.rs` (38 Zeilen)
  - → `acta-ports`: dev: `StorageStats`
- `tests/blocking_util_pool_measurement.rs` (90 Zeilen)
  - → `acta-ports`: dev: `MetricsSink`
- `tests/budget_drift.rs` (54 Zeilen)
  - → `acta-mvcc`: dev: `tx_buffer::STAGING_ENTRY_OVERHEAD_BYTES`
- `tests/campaign_j11_memory_deletion.rs` (48 Zeilen)
  - → `acta-ports`: dev: `storage::StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/campaign_j41_store.rs` (36 Zeilen)
  - → `acta-types`: dev: `TenantId`
- `tests/chaos_dropped_write.rs` (145 Zeilen)
  - → `acta-sys`: dev: `reopen_and_dup2`×2
- `tests/compaction_contiguous_selection.rs` (153 Zeilen)
  - → `acta-core`: dev: `StorageStats`
- `tests/compaction_key_range_selection.rs` (101 Zeilen)
  - → `acta-core`: dev: `ResourceTracker`, `SnapshotRegistry::new`, `ResourceBudget`
- `tests/deletion_proof_unregistered.rs` (51 Zeilen)
  - → `acta-core`: dev: `ActaError`
- `tests/dirlock_fail_closed.rs` (167 Zeilen)
  - → `acta-core`: dev: `ActaError`
- `tests/durability_mode_feature_gate.rs` (74 Zeilen)
  - → `acta-ports`: dev: `license::FeatureRing`
- `tests/group_commit_test.rs` (222 Zeilen)
  - → `acta-core`: dev: `ActaError`
- `tests/j07-acta-store-lsm-engine-kv-segment-te_integration_test.rs` (241 Zeilen)
  - → `acta-core`: dev: `StorageStats`
  - → `acta-ports`: dev: `SystemClock`, `Clock`
- `tests/j07closure-acta-store_wiring_test.rs` (115 Zeilen)
  - → `acta-core`: dev: `Result`×2, `TxId`, `SnapshotRegistry::new`, `ResourceTracker::new` … (+1)
- `tests/kv_delete_mode_wiring_test.rs` (107 Zeilen)
  - → `acta-core`: dev: `ActaError`
- `tests/kv_deletion_proof_registry.rs` (68 Zeilen)
  - → `acta-core`: dev: `ActaError`
- `tests/lsm_storage_merge_operator.rs` (111 Zeilen)
  - → `acta-core`: dev: `ActaError::InvalidInput`×2
- `tests/manifest_replace_rank_compat.rs` (68 Zeilen)
  - → `acta-core`: dev: `Result`
- `tests/observer_deadline_no_runtime_block.rs` (67 Zeilen)
  - → `acta-core`: dev: `TxId`
- `tests/proptest_model_based.rs` (236 Zeilen)
  - → `acta-core`: dev: `Result`
- `tests/scan_limits_and_ranges.rs` (59 Zeilen)
  - → `acta-core`: dev: `TxId`
- `tests/ssi_tracked_read_seq_clamp.rs` (122 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
- `tests/ssi_tracked_reads_via_trait.rs` (89 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
- `tests/sstable_format_v3_to_v4_compat.rs` (158 Zeilen)
  - → `acta-core`: dev: `TOMBSTONE_BIT`
- `tests/support/mod.rs` (79 Zeilen)
  - → `acta-core`: dev: `StorageEngine`
- `tests/tenant_codec_isolation_props.rs` (226 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
- `tests/tenant_scoped_storage.rs` (131 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
- `tests/test_wp02_flush_mvcc_bug.rs` (38 Zeilen)
  - → `acta-core`: dev: `Result`
- `tests/toctou_put_if_absent.rs` (209 Zeilen)
  - → `acta-core`: dev: `error::ActaError`, `types::*`, `Result`, `traits`
- `tests/wal_backpressure.rs` (294 Zeilen)
  - → `acta-core`: dev: `TxId`
- `tests/wal_flusher_rollback.rs` (92 Zeilen)
  - → `acta-core`: dev: `Result`×2, `TxId`
- `tests/wal_legacy_key_forced_migration.rs` (228 Zeilen)
  - → `acta-core`: dev: `TxId`
- `tests/wal_legacy_key_migration_store.rs` (93 Zeilen)
  - → `acta-core`: dev: `TxId`
- `tests/wal_legacy_key_retirement_marker.rs` (109 Zeilen)
  - → `acta-core`: dev: `TxId`
- `tests/wal_legacy_migration_crash.rs` (44 Zeilen)
  - → `acta-core`: dev: `TxId`
- `tests/wal_no_hmac_replay.rs` (52 Zeilen)
  - → `acta-core`: dev: `TxId`
- `tests/wal_recovery_runtime_stall.rs` (116 Zeilen)
  - → `acta-core`: dev: `TxId`
- `tests/wal_systematic_crash.rs` (198 Zeilen)
  - → `acta-core`: dev: `ActaError::Storage`×2
- `tests/wal_torn_write_fault_injection.rs` (113 Zeilen)
  - → `acta-core`: dev: `TxId`
- `tests/wal_truncate_ordering.rs` (257 Zeilen)
  - → `acta-core`: dev: `ActaError`

**Öffentliche Schnittstelle** (erreichbar über `pub mod`-Kette oder `pub use`)

- `compaction`
  - 1 enum, 1 fn, 7 struct, 2 trait
    - `struct WorkloadMetrics` · 4 Methoden
    - `struct WorkloadMetricsSnapshot`
    - `enum CompactionStrategy`
    - `struct AdaptiveCompactionPlan`
    - `trait AdaptiveCompactionPlanner` · Methoden: plan_compaction
    - `struct CostBasedAdaptivePlanner` · 2 Methoden
    - `struct CompactionConfig` *(re-exportiert)*
    - `struct CompactionEngine` *(re-exportiert)* · 14 Methoden
    - `trait MergeOperator` · Methoden: merge
    - `fn retain_key_versions`
    - `struct TtlMetadata`
- `engine`
  - 1 struct
    - `struct InMemoryStorageEngine` · 1 Methoden
- `kv`
  - 1 enum, 3 struct
    - `enum KvDeleteMode`
    - `struct KvSegmentConfig`
    - `struct KvSegmentPayload`
    - `struct KvSegmentManager` · 6 Methoden
- `lsm`
  - 7 const, 4 enum, 7 struct, 1 trait
    - `enum DurabilityMode` · 1 Methoden
    - `enum DurabilityConfigError`
    - `struct LsmConfig`
    - `enum StorageHealth` *(re-exportiert)*
    - `struct LsmStorage` *(re-exportiert)* · 37 Methoden ⚠ fremd: `acta-mvcc`
    - `const MAX_KEY_SIZE`
    - `const MAX_VALUE_SIZE`
    - `const MAX_BATCH_SIZE`
    - `const MAX_INTERNAL_MERGE_ENTRIES_FACTOR`
    - `const MAX_GROUP_COMMIT_BATCH_SIZE`
    - `const MIN_ENTRIES_FOR_SSTABLE_REBUILD`
    - `const DEFAULT_MAX_OBSERVER_LATENCY`
    - `enum WriteOrigin`
    - `struct WalEntryRef` · 6 Methoden
    - `struct CommitContext` ⚠ fremd: `acta-core`
    - `struct CommittedBatch`
    - `trait WalObserver` · Methoden: requires_durability, on_commit
    - `struct ObserverRegistry` · 14 Methoden
    - `struct AsyncObserverAdapter` · 1 Methoden
- `manifest`
  - 2 const, 1 enum, 1 struct
    - `struct Manifest` *(re-exportiert)* · 8 Methoden
    - `enum ManifestEntry` *(re-exportiert)* · 2 Methoden
    - `const MAX_MANIFEST_ENTRY_SIZE`
    - `const DEFAULT_ROLLOVER_THRESHOLD_BYTES`
- `memtable`
  - 1 const, 2 fn, 1 struct
    - `fn is_tombstone`
    - `fn is_valid_range_bounds`
    - `const RESERVED_PREFIXES`
    - `struct MemTable` · 16 Methoden
- `sstable`
  - 3 const, 8 fn, 10 struct, 1 trait, 2 type
    - `const BLOCK_CACHE_SHARDS` *(re-exportiert)*
    - `struct SieveCacheBackend` *(re-exportiert)*
    - `trait BlockCacheBackend` *(re-exportiert)* · Methoden: new, get, insert, len, is_empty, contains
    - `struct LruBlockCacheBackend` *(re-exportiert)*
    - `struct QuickCacheBlockCacheBackend` *(re-exportiert)*
    - `type BlockCacheShard` *(re-exportiert)*
    - `type BlockCacheShard` *(re-exportiert)*
    - `struct BlockCache` *(re-exportiert)* · 8 Methoden
    - `const SSTABLE_MAGIC_MFSX` *(re-exportiert)*
    - `const SSTABLE_MAGIC_LEGACY` *(re-exportiert)*
    - `fn create_block_cache` *(re-exportiert)*
    - `fn create_block_cache_with_shards` *(re-exportiert)*
    - `fn get_entry_at_index` *(re-exportiert)*
    - `fn get_entry_key_at_index` *(re-exportiert)*
    - `fn binary_search_first_index_in_block` *(re-exportiert)*
    - `fn binary_search_index_in_block` *(re-exportiert)*
    - `fn binary_search_entry_in_block` *(re-exportiert)*
    - `fn block_binary_search` *(re-exportiert)*
    - `struct BloomFilter` *(re-exportiert)* · 6 Methoden
    - `struct BlockBuilder` *(re-exportiert)* · 6 Methoden
    - `struct SstableMetadata` *(re-exportiert)*
    - `struct SstableBuilder` *(re-exportiert)* · 7 Methoden
    - `struct SstableReader` *(re-exportiert)* · 13 Methoden
    - `struct SstableStream` *(re-exportiert)* · 2 Methoden
- `system_pressure`
  - 6 const, 1 enum, 2 struct
    - `const WAL_QUEUE_CRITICAL_THRESHOLD`
    - `const WAL_QUEUE_ELEVATED_THRESHOLD`
    - `const SCHEDULER_QUEUE_CRITICAL`
    - `const SCHEDULER_QUEUE_ELEVATED`
    - `const BLOCKING_UTIL_CRITICAL`
    - `const BLOCKING_UTIL_ELEVATED`
    - `enum PressureLevel`
    - `struct SystemPressure`
    - `struct SystemPressureMonitor` · 4 Methoden
- `tenant_codec`
  - 2 struct, 1 trait
    - `struct TenantKeyCodec` · 6 Methoden
    - `trait StorageEngineHandle` · Methoden: get_handle, get_at_seq_handle, get_tracked_handle, get_at_seq_tracked_handle, scan_prefix_tracked_handle, supports_ssi_tracking_handle…
    - `struct TenantScopedStorage` · 2 Methoden
- `wal`
  - 5 const, 4 enum, 2 fn, 6 static, 7 struct, 1 trait, 1 type
    - `enum WalParseError`
  - … 25 weitere Items nicht aufgelistet (`--max-items`)
- Zusätzlich 27 `pub`-Items in privaten Modulen ohne Re-Export (nicht Teil der echten API).
- ⚠ Re-Exporte **fremder Crates**: `acta_crypto::crypto::KeyManager`

**Eingehend (wer nutzt dieses Crate)**

| Nutzer | Symbole prod | Symbole dev |
|---|---|---|
| `acta-engine` | `LsmStorage`×6, `tenant_codec::TenantScopedStorage`×5, `kv::KvSegmentManager`×4, `tenant_codec::TenantScopedStorage::new`×4, `SystemPressure`×3, `kv::KvSegmentPayload`×2, `tenant_codec::TenantKeyCodec`×2, `PressureLevel::Critical`, `LsmConfig`, `lsm::DurabilityMode` … (+1) | `lsm`×10, `compaction`×2, `sstable`×2 |
| `acta-store-fuzz` | `wal`×3, `manifest::Manifest`, `memtable::MemTable`, `sstable::binary_search_index_in_block`, `sstable::binary_search_entry_in_block`, `sstable::block_binary_search` | – |
| `acta-db` | – | `LsmConfig`×5, `LsmStorage::new`×4, `LsmStorage`×2, `lsm` |
| `acta-graph` | – | `LsmConfig`, `LsmStorage::new` |

**Nach außen ungenutzte öffentliche Items (Heuristik, 91 von 105)**: `WorkloadMetrics`, `WorkloadMetricsSnapshot`, `CompactionStrategy`, `AdaptiveCompactionPlan`, `AdaptiveCompactionPlanner`, `CostBasedAdaptivePlanner`, `CompactionConfig`, `CompactionEngine`, `retain_key_versions`, `TtlMetadata`, `InMemoryStorageEngine`, `KvDeleteMode`, `KvSegmentConfig`, `DurabilityConfigError`, `StorageHealth`, `MAX_KEY_SIZE`, `MAX_VALUE_SIZE`, `MAX_BATCH_SIZE`, `MAX_INTERNAL_MERGE_ENTRIES_FACTOR`, `MAX_GROUP_COMMIT_BATCH_SIZE`, `MIN_ENTRIES_FOR_SSTABLE_REBUILD`, `DEFAULT_MAX_OBSERVER_LATENCY`, `WriteOrigin`, `WalEntryRef`, `CommitContext`, `CommittedBatch`, `WalObserver`, `ObserverRegistry`, `AsyncObserverAdapter`, `ManifestEntry` …

**Auffälligkeiten**

- acta-graph: kein Code-Zugriff gefunden (prüfen: Makro, Feature, cfg, oder entfernen)
- acta-types: nur in Tests/Benches genutzt, aber als normale Abhängigkeit deklariert → dev-dependency?
- acta-vector: kein Code-Zugriff gefunden (prüfen: Makro, Feature, cfg, oder entfernen)

### acta-store-fuzz

- Pfad: `crates/acta-store/fuzz` · Zeilen: 660 · Rust-Dateien: 8
- Interne Cargo-Abhängigkeiten: `acta-core` (normal), `acta-store` (normal)

**Ausgehend (Importe aus anderen Workspace-Crates)**

| Ziel | prod | dev | Dateien (prod) | Symbole (prod) | Symbole (nur dev) |
|---|---:|---:|---:|---|---|
| `acta-store` | 8 | 0 | 6 | `wal`×3, `manifest::Manifest`, `memtable::MemTable`, `sstable::binary_search_index_in_block`, `sstable::binary_search_entry_in_block`, `sstable::block_binary_search` | – |
| `acta-core` | 2 | 0 | 2 | `StorageEngine`, `TxId` | – |

**Pro Komponente**

- `fuzz_targets/fuzz_manifest_load.rs` (24 Zeilen)
  - → `acta-store`: prod: `manifest::Manifest`
- `fuzz_targets/fuzz_memtable_concurrent.rs` (103 Zeilen)
  - → `acta-store`: prod: `memtable::MemTable`
- `fuzz_targets/fuzz_recovery_arbitrary_state.rs` (52 Zeilen)
  - → `acta-core`: prod: `StorageEngine`
- `fuzz_targets/fuzz_sstable_binary_search.rs` (30 Zeilen)
  - → `acta-store`: prod: `sstable::binary_search_index_in_block`, `sstable::binary_search_entry_in_block`, `sstable::block_binary_search`
- `fuzz_targets/fuzz_wal_replay.rs` (36 Zeilen)
  - → `acta-store`: prod: `wal`
- `fuzz_targets/wal_mutation_chaos.rs` (272 Zeilen)
  - → `acta-core`: prod: `TxId`
  - → `acta-store`: prod: `wal`
- `fuzz_targets/wal_roundtrip.rs` (34 Zeilen)
  - → `acta-store`: prod: `wal`

**Öffentliche Schnittstelle** (erreichbar über `pub mod`-Kette oder `pub use`)

Keine pub-Items erkannt (Bibliothek ohne lib.rs, nur Binary, oder alles intern).

**Eingehend (wer nutzt dieses Crate)**

Wird von keinem anderen analysierten Workspace-Crate importiert.

### acta-sys

- Pfad: `crates/acta-sys` · Zeilen: 632 · Rust-Dateien: 6 · Ring: 0
- Interne Cargo-Abhängigkeiten: keine

**Ausgehend (Importe aus anderen Workspace-Crates)**

Keine.

**Pro Komponente**

Keine Importe aus anderen Workspace-Crates.

**Öffentliche Schnittstelle** (erreichbar über `pub mod`-Kette oder `pub use`)

- `acl_win32`
  - 4 fn
    - `fn set_restrictive_file_acl`
    - `fn verify_file_acl_owner_only`
    - `fn verify_file_acl_owner_only`
    - `fn set_restrictive_file_acl`
- `mlock`
  - 6 fn
    - `fn mem_lock`
    - `fn mem_unlock`
    - `fn mem_lock`
    - `fn mem_unlock`
    - `fn mem_lock`
    - `fn mem_unlock`
- `mmap`
  - 1 fn
    - `fn mmap_readonly`
- `posix`
  - 2 fn
    - `fn reopen_and_dup2`
    - `fn reopen_and_dup2`
- `vault`
  - 1 struct
    - `struct LockedRegions` · 5 Methoden

**Eingehend (wer nutzt dieses Crate)**

| Nutzer | Symbole prod | Symbole dev |
|---|---|---|
| `acta-store` | `mmap_readonly`×2, `set_restrictive_file_acl` | `reopen_and_dup2`×2, `verify_file_acl_owner_only` |
| `acta-vector` | `mmap_readonly`×2 | – |
| `acta-db` | `LockedRegions` | – |

**Nach außen ungenutzte öffentliche Items (Heuristik, 6 von 14)**: `mem_lock`, `mem_unlock`, `mem_lock`, `mem_unlock`, `mem_lock`, `mem_unlock`

### acta-text

- Pfad: `crates/acta-text` · Zeilen: 10725 · Rust-Dateien: 45 · Ring: 0
- Interne Cargo-Abhängigkeiten: `acta-ports` (normal), `acta-types` (normal)

**Ausgehend (Importe aus anderen Workspace-Crates)**

| Ziel | prod | dev | Dateien (prod) | Symbole (prod) | Symbole (nur dev) |
|---|---:|---:|---:|---|---|
| `acta-ports` | 5 | 81 | 3 | `StorageEngine`×5 | `BoxFuture`×50, `StorageStats`×31 |
| `acta-types` | 0 | 132 | 0 | – | `Result`×67, `TOMBSTONE_BIT`×52, `TxId`×7, `TxId::new`×2, `ActaError::CapabilityUnsupported`×2 … (+2) |

**Pro Komponente**

- `(crate-root)` (225 Zeilen)
  - → `acta-ports`: prod: `StorageEngine`×3 · dev: `BoxFuture`
- `inverted` (1963 Zeilen)
  - → `acta-ports`: dev: `StorageStats`×3
  - → `acta-types`: dev: `TOMBSTONE_BIT`×6, `ContextChunk`
- `stream` (125 Zeilen)
  - → `acta-ports`: prod: `StorageEngine`
- `wand` (753 Zeilen)
  - → `acta-ports`: prod: `StorageEngine` · dev: `StorageStats`×2, `BoxFuture`
  - → `acta-types`: dev: `TxId`×7, `TOMBSTONE_BIT`×6, `TxId::new`×2
- `tests/alloc_profiler.rs` (448 Zeilen)
  - → `acta-ports`: dev: `StorageStats`×2
  - → `acta-types`: dev: `Result`×16
- `tests/audit_inspection.rs` (563 Zeilen)
  - → `acta-ports`: dev: `BoxFuture`×32, `StorageStats`×4
  - → `acta-types`: dev: `Result`×34, `TOMBSTONE_BIT`×10
- `tests/bm25_candidate_stream.rs` (306 Zeilen)
  - → `acta-ports`: dev: `StorageStats`×2
- `tests/concurrent_metadata.rs` (195 Zeilen)
  - → `acta-ports`: dev: `StorageStats`×2
- `tests/docid_128_tombstone_resolution.rs` (242 Zeilen)
  - → `acta-ports`: dev: `BoxFuture`×16, `StorageStats`×2
  - → `acta-types`: dev: `Result`×17, `TOMBSTONE_BIT`×6
- `tests/j31closure-acta-text_closure_verification_test.rs` (64 Zeilen)
  - → `acta-types`: dev: `DocId`
- `tests/pipeline_identity_regression.rs` (270 Zeilen)
  - → `acta-ports`: dev: `StorageStats`×2
  - → `acta-types`: dev: `TOMBSTONE_BIT`×6
- `tests/proptest_bm25_invariants.rs` (334 Zeilen)
  - → `acta-ports`: dev: `StorageStats`×2
  - → `acta-types`: dev: `TOMBSTONE_BIT`×6
- `tests/text_search_at_wrappers.rs` (322 Zeilen)
  - → `acta-ports`: dev: `StorageStats`×2
  - → `acta-types`: dev: `TOMBSTONE_BIT`×6, `ActaError::CapabilityUnsupported`×2
- `tests/tombstone_read_your_writes.rs` (266 Zeilen)
  - → `acta-ports`: dev: `StorageStats`×2
  - → `acta-types`: dev: `TOMBSTONE_BIT`×6
- `tests/tombstone_update.rs` (220 Zeilen)
  - → `acta-ports`: dev: `StorageStats`×2
- `tests/wand_tie_break_threshold.rs` (221 Zeilen)
  - → `acta-ports`: dev: `StorageStats`×2
- `tests/write_amplification.rs` (218 Zeilen)
  - → `acta-ports`: dev: `StorageStats`×2

**Öffentliche Schnittstelle** (erreichbar über `pub mod`-Kette oder `pub use`)

- `(crate-root)`
  - 1 struct
    - `struct Bm25Scorer` · 1 Methoden ⚠ fremd: `acta-ports`
- `bm25`
  - 2 const, 3 fn, 3 struct, 1 type
    - `const BM25_K1`
    - `const BM25_B`
    - `struct BM25` · 2 Methoden
    - `fn score_term`
    - `fn score_term_with_params`
    - `type FieldId`
    - `struct FieldWeight` · 1 Methoden
    - `struct BM25F` · 2 Methoden
    - `fn score_term_bm25f`
- `domain`
  - 4 fn, 4 static, 2 struct, 1 trait
    - `struct LegalDomainVocabulary`
    - `fn get_legal_compound_stems`
    - `fn get_legal_protected_terms`
    - `static LEGAL_COMPOUND_STEMS`
    - `static LEGAL_PROTECTED_TERMS`
    - `struct MedicalDomainVocabulary`
    - `fn get_medical_compound_stems`
    - `fn get_medical_protected_terms`
    - `static MEDICAL_COMPOUND_STEMS`
    - `static MEDICAL_PROTECTED_TERMS`
    - `trait DomainVocabulary` · Methoden: compound_stems, protected_terms, domain_id
- `inverted`
  - 2 struct
    - `struct InvertedIndex` *(re-exportiert)* · 9 Methoden
    - `struct BM25MorphIndex` *(re-exportiert)* · 2 Methoden
- `morphology`
  - 2 struct
    - `struct TokenReductionMetrics` *(re-exportiert)* · 1 Methoden
    - `struct PassthroughTokenizer` *(re-exportiert)* · 1 Methoden
- `posting_list`
  - 2 const, 4 struct, 2 type
    - `const POSTING_LIST_V2_MAGIC`
    - `const BLOCK_SIZE`
    - `type DocIdRaw`
    - `type DocIdRaw`
    - `struct PostingBlockInfo`
    - `struct Posting` · 2 Methoden
    - `struct PostingList` · 10 Methoden
    - `struct ResidentPostingIndex` · 7 Methoden
- `stream`
  - 1 const, 1 struct
    - `const DEFAULT_STREAM_BATCH_SIZE`
    - `struct Bm25CandidateStream` · 7 Methoden ⚠ fremd: `acta-ports`
- `tokenizer`
  - 1 fn, 2 struct, 1 trait
    - `trait Tokenizer` · Methoden: tokenize
    - `struct DefaultTokenizer`
    - `struct GermanMorphTokenizer` · 1 Methoden
    - `fn tokenize`
- `wand`
  - 1 fn, 2 struct
    - `struct BoundedTopK` · 4 Methoden
    - `struct WandSearchResult`
    - `fn block_max_wand_search` ⚠ fremd: `acta-ports`
- Zusätzlich 9 `pub`-Items in privaten Modulen ohne Re-Export (nicht Teil der echten API).

**Eingehend (wer nutzt dieses Crate)**

| Nutzer | Symbole prod | Symbole dev |
|---|---|---|
| `acta-engine` | `Language`×2, `inverted::InvertedIndex` | `Language::English`×64, `Language::German`×4 |
| `acta-text-fuzz` | `tokenizer::tokenize` | – |
| `acta-db` | – | `Language::English`×7, `Language`×2, `bm25::score_term`, `inverted::InvertedIndex` |

**Nach außen ungenutzte öffentliche Items (Heuristik, 39 von 42)**: `BM25_K1`, `BM25_B`, `BM25`, `score_term_with_params`, `FieldId`, `FieldWeight`, `BM25F`, `score_term_bm25f`, `LegalDomainVocabulary`, `get_legal_compound_stems`, `get_legal_protected_terms`, `LEGAL_COMPOUND_STEMS`, `LEGAL_PROTECTED_TERMS`, `MedicalDomainVocabulary`, `get_medical_compound_stems`, `get_medical_protected_terms`, `MEDICAL_COMPOUND_STEMS`, `MEDICAL_PROTECTED_TERMS`, `DomainVocabulary`, `BM25MorphIndex`, `Bm25Scorer`, `TokenReductionMetrics`, `PassthroughTokenizer`, `POSTING_LIST_V2_MAGIC`, `BLOCK_SIZE`, `DocIdRaw`, `DocIdRaw`, `PostingBlockInfo`, `Posting`, `PostingList` …

**Auffälligkeiten**

- acta-types: nur in Tests/Benches genutzt, aber als normale Abhängigkeit deklariert → dev-dependency?

### acta-text-fuzz

- Pfad: `crates/acta-text/fuzz` · Zeilen: 10 · Rust-Dateien: 1
- Interne Cargo-Abhängigkeiten: `acta-text` (normal)

**Ausgehend (Importe aus anderen Workspace-Crates)**

| Ziel | prod | dev | Dateien (prod) | Symbole (prod) | Symbole (nur dev) |
|---|---:|---:|---:|---|---|
| `acta-text` | 1 | 0 | 1 | `tokenizer::tokenize` | – |

**Pro Komponente**

- `fuzz_targets/fuzz_bm25_tokenize.rs` (10 Zeilen)
  - → `acta-text`: prod: `tokenizer::tokenize`

**Öffentliche Schnittstelle** (erreichbar über `pub mod`-Kette oder `pub use`)

Keine pub-Items erkannt (Bibliothek ohne lib.rs, nur Binary, oder alles intern).

**Eingehend (wer nutzt dieses Crate)**

Wird von keinem anderen analysierten Workspace-Crate importiert.

### acta-types

- Pfad: `crates/acta-types` · Zeilen: 6761 · Rust-Dateien: 29 · Ring: 0
- Interne Cargo-Abhängigkeiten: keine

**Ausgehend (Importe aus anderen Workspace-Crates)**

Keine.

**Pro Komponente**

Keine Importe aus anderen Workspace-Crates.

**Öffentliche Schnittstelle** (erreichbar über `pub mod`-Kette oder `pub use`)

- `error`
  - 3 enum, 1 struct, 1 type
    - `enum HnswDeletionError`
    - `struct StepId` · 2 Methoden
    - `enum CacheDirective`
    - `type Result`
    - `enum ActaError` · 17 Methoden
- `error_dto`
  - 1 struct
    - `struct ActaErrorDto` · 2 Methoden
- `model_fingerprint`
  - 1 struct
    - `struct ModelFingerprint` · 1 Methoden
- `retrieval_strategy`
  - 1 enum
    - `enum RetrievalStrategy`
- `schema`
  - 2 enum
    - `enum DocIdWidth` · 3 Methoden
    - `enum ManifestSchemaVersion` · 7 Methoden
- `tenant_scope`
  - 1 enum, 1 struct
    - `struct TenantScoped` · 4 Methoden
    - `enum TenantScopeViolation`
- `tombstone`
  - 1 struct, 1 trait
    - `trait TombstoneSemanticsCheck` · Methoden: is_tombstone, make_tombstone
    - `struct SeqBitTombstone`
- `types`
  - 3 const, 12 enum, 2 fn, 32 struct, 2 type
    - `enum BudgetStrategy`
    - `struct TokenBudget` · 11 Methoden
    - `struct Reservation` · 1 Methoden
    - `struct ResourceBudget`
    - `struct ResourceTracker` · 6 Methoden
    - `enum DistanceMetric` *(re-exportiert)* · 2 Methoden
    - `struct Embedding` *(re-exportiert)* · 5 Methoden
    - `struct ScoredDocument` *(re-exportiert)* · 1 Methoden
    - `struct RerankResult` *(re-exportiert)*
    - `struct TenantId` *(re-exportiert)* · 5 Methoden
    - `struct CollectionId` *(re-exportiert)* · 4 Methoden
    - `struct DocId` *(re-exportiert)*
    - `struct DocId` *(re-exportiert)* · 8 Methoden
    - `struct EntityId` *(re-exportiert)* · 6 Methoden
    - `struct TxId` *(re-exportiert)* · 7 Methoden
    - `fn serialize` *(re-exportiert)*
    - `fn deserialize` *(re-exportiert)*
    - `struct WorkflowState` *(re-exportiert)*
    - `const TOMBSTONE_BIT` *(re-exportiert)*
    - `const EXPIRY_METADATA_KEY` *(re-exportiert)*
    - `const MAX_SEARCH_K` *(re-exportiert)*
    - `enum LinkRelation` *(re-exportiert)*
    - `struct MemoryLink` *(re-exportiert)*
    - `struct Entity` *(re-exportiert)* · 2 Methoden
    - `struct Edge` *(re-exportiert)* · 7 Methoden
    - `enum MemoryType` *(re-exportiert)* · 4 Methoden
    - `struct TlHfdParams` *(re-exportiert)*
    - `enum PprAlgorithm` *(re-exportiert)*
    - `struct PprConfig` *(re-exportiert)*
    - `struct ConfigFingerprint` *(re-exportiert)* · 4 Methoden
    - `enum AutoExtractionMode` *(re-exportiert)* · 1 Methoden
    - `enum FilterExpr` · 1 Methoden
    - `struct ImportanceScore` · 2 Methoden
    - `enum DecayFunction` · 2 Methoden
    - `struct MemoryImportance` · 2 Methoden
    - `struct SaturatingU8` · 2 Methoden
    - `enum FusionStrategy`
    - `struct SignalFusionStrategies` · 2 Methoden
    - `enum GraphTraversalStrategy`
    - `struct FusionWeights` · 5 Methoden
    - `struct ContextChunk` · 3 Methoden
    - `struct ContextWindow`
    - `enum Signal`
    - `enum OnSignalFailure`
    - `struct QueryRewriteOutput` · 4 Methoden
    - `struct ScoredEntry`
    - `struct HybridQuery` · 11 Methoden
    - `type QueryOptions`
    - `struct HybridQueryBuilder` · 17 Methoden
    - `type QueryOptionsBuilder`
    - `struct SaturatingU16` · 2 Methoden
- Interne Re-Exporte (`pub use`): `RerankResult`

**Eingehend (wer nutzt dieses Crate)**

| Nutzer | Symbole prod | Symbole dev |
|---|---|---|
| `acta-engine` | `ActaError::invalid_input`×50, `Result`×23, `ActaError::Internal`×19, `EntityId`×14, `MAX_SEARCH_K`×12, `ActaError::Serialization`×8, `ImportanceScore::new`×8, `domain::MemoryLink`×7, `HybridQuery`×7, `EntityId::from_key`×7 … (+42) | `ActaError::from`×10, `domain::LinkRelation::Elaborates`×2, `ActaError::HnswConnectivityDegraded`, `FusionWeights::new` … (+8) |
| `acta-graph` | `EntityId`×14, `ActaError::InvalidInput`×5, `ActaError`×4, `PprConfig`×3, `Result`×3, `Edge::new`×2, `TxId`×2, `ActaError::Internal`×2, `Edge`, `MemoryLink` … (+2) | `Entity::new`×10, `TlHfdParams::default`×4, `PprAlgorithm::Auto`×4, `TlHfdParams` |
| `acta-crypto` | `TenantId`×8, `Result`×6, `ActaError::Crypto`×3, `TenantScoped::new`×2, `TenantScoped`×2, `TxId`, `ActaError`, `ModelFingerprint`, `ActaError::ParseError` | – |
| `acta-core` | `error`×2, `schema`×2, `tombstone`×2, `error_dto`, `model_fingerprint`, `types`, `error_dto::ActaErrorDto`, `model_fingerprint::ModelFingerprint` | – |
| `acta-rank` | `ConfigFingerprint`×3, `ActaError`, `FusionWeights`, `FusionStrategy`, `SignalFusionStrategies`, `MAX_SEARCH_K`, `OnSignalFailure`, `DocId` | – |
| `acta-mcp` | `ActaError`×4, `ActaError::InvalidInput`, `ActaError::NotFound`, `TenantId::try_new`, `TenantId::SYSTEM`, `TenantId` | `DocId`, `AutoExtractionMode`, `EntityId::from_key`, `TokenBudget::new` |
| `acta-ports` | `ActaError`×3, `types::domain`, `model_fingerprint::ModelFingerprint`, `types::domain::TenantId`, `DocId`, `*`, `ActaError::Plugin` | – |
| `acta-db` | `DistanceMetric`, `MAX_SEARCH_K` | `Result`×7, `TxId`×6, `DocId`×4, `FusionWeights`×3 … (+18) |
| `acta-mvcc` | `*` | `TxId`×2, `DocId` |
| `acta-vector` | `SaturatingU16::from` | – |
| `acta-store` | – | `TxId`, `TenantId` |
| `acta-text` | – | `Result`×67, `TOMBSTONE_BIT`×52, `TxId`×7, `TxId::new`×2 … (+3) |

*Hinweis: Mindestens ein Crate importiert per Glob (`*`); ungenutzte Items sind deshalb nicht bestimmbar.*

### acta-vector

- Pfad: `crates/acta-vector` · Zeilen: 21641 · Rust-Dateien: 109 · Ring: 0
- Interne Cargo-Abhängigkeiten: `acta-core` (normal), `acta-crypto` (normal), `acta-ports` (normal), `acta-simd` (normal), `acta-sys` (normal), `acta-types` (normal)

**Ausgehend (Importe aus anderen Workspace-Crates)**

| Ziel | prod | dev | Dateien (prod) | Symbole (prod) | Symbole (nur dev) |
|---|---:|---:|---:|---|---|
| `acta-core` | 60 | 65 | 13 | `ActaError::invalid_input`×19, `MAX_SEARCH_K`×15, `Result`×8, `ActaError::HnswConnectivityDegraded`×4, `ActaError::Index`×4, `DocId`×3, `DistanceMetric`×2, `SequenceLog::new`×2, `SeqLogChange::Insert`, `SeqLogChange::Delete` … (+1) | `types`×9, `traits::VectorIndex`×8, `ActaError::InvalidInput`×6, `ActaError::EmbeddingDimensionMismatch`×4, `DistanceMetric::Euclidean`×3 … (+5) |
| `acta-ports` | 13 | 10 | 1 | `MetricsSink`×6, `Rng`×3, `NoopMetricsSink`×2, `SeededRng::new`×2 | `SeededRng`×5 |
| `acta-crypto` | 6 | 2 | 2 | `wal_crypto::WalHmac::new`×6 | – |
| `acta-simd` | 3 | 7 | 3 | `validate_vector`×2, `kernels::scalar` | `kernels::scalar::dot_product_scalar`, `kernels::scalar::euclidean_distance_scalar`, `kernels::scalar::cosine_distance_scalar`, `kernels::scalar::dot_product_u8_scalar`, `kernels::scalar::euclidean_distance_sq_u8_scalar` … (+1) |
| `acta-sys` | 2 | 0 | 2 | `mmap_readonly`×2 | – |
| `acta-types` | 1 | 0 | 1 | `SaturatingU16::from` | – |

**Pro Komponente**

- `acorn` (183 Zeilen)
  - → `acta-core`: prod: `DocId`
- `diskann` (3590 Zeilen)
  - → `acta-core`: prod: `MAX_SEARCH_K`×9
  - → `acta-crypto`: prod: `wal_crypto::WalHmac::new`×6 · dev: `wal_crypto::WalHmac::new`×2
  - → `acta-simd`: prod: `validate_vector`
  - → `acta-sys`: prod: `mmap_readonly`
- `distance` (208 Zeilen)
  - → `acta-core`: dev: `ActaError::EmbeddingDimensionMismatch`×4, `DistanceMetric`
  - → `acta-simd`: prod: `kernels::scalar` · dev: `kernels::scalar::dot_product_scalar`, `kernels::scalar::euclidean_distance_scalar`, `kernels::scalar::cosine_distance_scalar`, `kernels::scalar::dot_product_u8_scalar` … (+2)
- `hnsw` (4932 Zeilen)
  - → `acta-core`: prod: `MAX_SEARCH_K`×6, `ActaError::HnswConnectivityDegraded`×4, `ActaError::Index`×4, `DocId`×2, `Result`×2, `SequenceLog::new`×2, `SeqLogChange::Insert`, `SeqLogChange::Delete`, `DistanceMetric`, `SequenceLog` · dev: `DistanceMetric`
  - → `acta-ports`: prod: `MetricsSink`×6, `Rng`×3, `NoopMetricsSink`×2, `SeededRng::new`×2 · dev: `SeededRng::new`×3, `Rng`, `MetricsSink`
  - → `acta-simd`: prod: `validate_vector`
  - → `acta-types`: prod: `SaturatingU16::from`
- `persistence` (1095 Zeilen)
  - → `acta-core`: dev: `Result`×2
  - → `acta-sys`: prod: `mmap_readonly`
- `quantize` (972 Zeilen)
  - → `acta-core`: prod: `ActaError::invalid_input`×19, `Result`×6, `DistanceMetric` · dev: `ActaError::InvalidInput`×6, `Result`×2
- `benches/acorn_vs_post_filter.rs` (53 Zeilen)
  - → `acta-core`: dev: `DocId`
- `benches/audit_benchmarks.rs` (430 Zeilen)
  - → `acta-core`: dev: `traits::VectorIndex`, `types`
- `benches/distance_bench.rs` (34 Zeilen)
  - → `acta-simd`: dev: `kernels::scalar`
- `benches/distance_nan_impact_bench.rs` (24 Zeilen)
  - → `acta-core`: dev: `DistanceMetric`
- `benches/flush_threshold_amplification.rs` (177 Zeilen)
  - → `acta-core`: dev: `traits::VectorIndex`, `types`
- `benches/hnsw_bench.rs` (90 Zeilen)
  - → `acta-core`: dev: `traits::VectorIndex`, `types`
- `benches/hnsw_delete_search_bench.rs` (86 Zeilen)
  - → `acta-core`: dev: `traits::VectorIndex`, `types`
- `benches/overfetch_selectivity.rs` (78 Zeilen)
  - → `acta-ports`: dev: `SeededRng`
- `benches/sq8_bench.rs` (40 Zeilen)
  - → `acta-core`: dev: `DistanceMetric`
- `tests/acorn_naive_recall.rs` (70 Zeilen)
  - → `acta-core`: dev: `DocId`
- `tests/campaign_j23_vector_index.rs` (477 Zeilen)
  - → `acta-core`: dev: `ActaError::Index`×6
- `tests/campaign_j24_test.rs` (609 Zeilen)
  - → `acta-core`: dev: `VectorIndex`, `types`
- `tests/diskann_corruption_fallback_test.rs` (102 Zeilen)
  - → `acta-core`: dev: `TxId::new`
- `tests/diskann_docid128_tombstones.rs` (44 Zeilen)
  - → `acta-core`: dev: `ActaError::Io`, `TxId`
- `tests/diskann_recall_test.rs` (218 Zeilen)
  - → `acta-core`: dev: `TxId`
- `tests/diskann_search_filtered.rs` (170 Zeilen)
  - → `acta-core`: dev: `MAX_SEARCH_K`
- `tests/distance_determinism.rs` (64 Zeilen)
  - → `acta-core`: dev: `DistanceMetric`
- `tests/docid_128_hnsw.rs` (69 Zeilen)
  - → `acta-core`: dev: `Result`×2
- `tests/docid_high_bits_no_collision.rs` (197 Zeilen)
  - → `acta-core`: dev: `Result`×2, `ActaError::Io`
- `tests/filtered_selectivity_recall.rs` (154 Zeilen)
  - → `acta-ports`: dev: `SeededRng`
- `tests/hnsw_rebuild_search_consistency.rs` (133 Zeilen)
  - → `acta-core`: dev: `DistanceMetric::Euclidean`
- `tests/hnsw_seed_determinism.rs` (294 Zeilen)
  - → `acta-ports`: dev: `SeededRng`
- `tests/j32closure-acta-vector_acorn_test.rs` (12 Zeilen)
  - → `acta-core`: dev: `DocId`
- `tests/k02_snapshot_rebuild_determinism.rs` (152 Zeilen)
  - → `acta-core`: dev: `DistanceMetric::Euclidean`
- `tests/loom_quantizer_race_test.rs` (114 Zeilen)
  - → `acta-core`: dev: `ActaError`
- `tests/partial_rebuild_veto_f02_guard.rs` (97 Zeilen)
  - → `acta-ports`: dev: `SeededRng`
- `tests/poisoning.rs` (109 Zeilen)
  - → `acta-core`: dev: `traits::VectorIndex`, `types`
- `tests/proptest_distance_quantize.rs` (89 Zeilen)
  - → `acta-core`: dev: `DistanceMetric`
- `tests/quantize_persistence_audit.rs` (302 Zeilen)
  - → `acta-core`: dev: `traits::VectorIndex`, `ActaError`, `types`
- `tests/rabitq_recall_calibration.rs` (117 Zeilen)
  - → `acta-core`: dev: `ActaError`
- `tests/recalibration.rs` (55 Zeilen)
  - → `acta-core`: dev: `DistanceMetric::Euclidean`
- `tests/recall_audit.rs` (405 Zeilen)
  - → `acta-core`: dev: `traits::VectorIndex`, `types`
- `tests/rollback.rs` (55 Zeilen)
  - → `acta-core`: dev: `traits::VectorIndex`, `types`
- `tests/simd_numerical_audit.rs` (171 Zeilen)
  - → `acta-core`: dev: `DistanceMetric`
- `tests/sq8_bias_calibration.rs` (20 Zeilen)
  - → `acta-core`: dev: `DistanceMetric`
- `tests/sq8_rerank_recall.rs` (163 Zeilen)
  - → `acta-ports`: dev: `SeededRng`

**Öffentliche Schnittstelle** (erreichbar über `pub mod`-Kette oder `pub use`)

- `acorn`
  - 1 enum, 1 fn, 1 struct, 1 trait
    - `fn compute_gamma_edge_budget`
    - `trait FilteredIndex` · Methoden: search_knn_acorn
    - `enum AcornError`
    - `struct NaiveReferenceIndex` · 3 Methoden
- `candidate_stream`
  - 1 const, 1 struct
    - `const DEFAULT_VECTOR_STREAM_BATCH_SIZE`
    - `struct VectorCandidateStream` · 8 Methoden
- `compute_pool`
  - 2 struct
    - `struct TaskHandle` · 1 Methoden
    - `struct ComputePool` · 5 Methoden
- `diskann`
  - 1 struct
    - `struct DiskAnnIndex` *(re-exportiert)* · 2 Methoden
- `hnsw`
  - 2 const, 7 struct, 1 trait, 1 type
    - `struct AdaptiveEfPolicy` · 2 Methoden
    - `struct AdaptiveEfStats`
    - `struct AdaptiveEfStateMachine` · 6 Methoden
    - `const ARENA_ALIGNMENT_BYTES`
    - `const ARENA_ALIGNMENT_U32`
    - `type BacklinkKey`
    - `struct BacklinkTable` · 4 Methoden
    - `struct HnswArena` · 8 Methoden
    - `struct DeletionStats`
    - `trait GhostFreeVectorIndex` · Methoden: remove_with_graph_repair
    - `struct Sq8Bias` · 2 Methoden
- `partial_rebuild`
  - 1 fn, 2 struct
    - `struct PartialRebuildConfig`
    - `struct TraversalTracker` · 4 Methoden
    - `fn should_trigger_partial_rebuild`
- `persistence`
  - 2 struct
    - `struct MmapIndex` *(re-exportiert)* · 5 Methoden
    - `struct NodeRecord` *(re-exportiert)* · 2 Methoden
- `quantize`
  - 2 const, 1 struct
    - `const DEFAULT_P_LOW`
    - `const DEFAULT_P_HIGH`
    - `struct ScalarQuantizer` · 17 Methoden
- `quantize_rabitq`
  - 1 struct
    - `struct RaBitQQuantizer` · 5 Methoden
- Zusätzlich 26 `pub`-Items in privaten Modulen ohne Re-Export (nicht Teil der echten API).

**Eingehend (wer nutzt dieses Crate)**

| Nutzer | Symbole prod | Symbole dev |
|---|---|---|
| `acta-engine` | `hnsw::HnswIndex`×2, `HnswIndex`×2, `acorn::FilteredIndex` | `HnswConfig`×58, `hnsw::HnswConfig::default`, `hnsw::HnswIndex::try_new`, `acorn` … (+1) |
| `acta-vector-fuzz` | `hnsw`×2 | – |
| `acta-db` | – | `HnswConfig`×3, `HnswIndex`×2, `HnswIndex::try_new` |

**Nach außen ungenutzte öffentliche Items (Heuristik, 28 von 29)**: `compute_gamma_edge_budget`, `AcornError`, `NaiveReferenceIndex`, `DEFAULT_VECTOR_STREAM_BATCH_SIZE`, `VectorCandidateStream`, `TaskHandle`, `ComputePool`, `DiskAnnIndex`, `AdaptiveEfPolicy`, `AdaptiveEfStats`, `AdaptiveEfStateMachine`, `ARENA_ALIGNMENT_BYTES`, `ARENA_ALIGNMENT_U32`, `BacklinkKey`, `BacklinkTable`, `HnswArena`, `DeletionStats`, `GhostFreeVectorIndex`, `Sq8Bias`, `PartialRebuildConfig`, `TraversalTracker`, `should_trigger_partial_rebuild`, `MmapIndex`, `NodeRecord`, `DEFAULT_P_LOW`, `DEFAULT_P_HIGH`, `ScalarQuantizer`, `RaBitQQuantizer`

### acta-vector-fuzz

- Pfad: `crates/acta-vector/fuzz` · Zeilen: 196 · Rust-Dateien: 2
- Interne Cargo-Abhängigkeiten: `acta-core` (normal), `acta-vector` (normal)

**Ausgehend (Importe aus anderen Workspace-Crates)**

| Ziel | prod | dev | Dateien (prod) | Symbole (prod) | Symbole (nur dev) |
|---|---:|---:|---:|---|---|
| `acta-vector` | 2 | 0 | 2 | `hnsw`×2 | – |

**Pro Komponente**

- `fuzz_targets/fuzz_hnsw_persistence.rs` (118 Zeilen)
  - → `acta-vector`: prod: `hnsw`
- `fuzz_targets/hnsw_insert_search.rs` (78 Zeilen)
  - → `acta-vector`: prod: `hnsw`

**Öffentliche Schnittstelle** (erreichbar über `pub mod`-Kette oder `pub use`)

Keine pub-Items erkannt (Bibliothek ohne lib.rs, nur Binary, oder alles intern).
- Zusätzlich 2 `pub`-Items in privaten Modulen ohne Re-Export (nicht Teil der echten API).

**Eingehend (wer nutzt dieses Crate)**

Wird von keinem anderen analysierten Workspace-Crate importiert.

**Auffälligkeiten**

- acta-core: kein Code-Zugriff gefunden (prüfen: Makro, Feature, cfg, oder entfernen)

