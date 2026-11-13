//! # Modul: lib.rs
//!
//! **Zweck:** Zentrale Facade und Orchestrierung der Acta-Engine (Ring 2).
//!
//! **Architektur:** Gehört zur Crate `acta-engine` (Ring 2).
//! Eingehaltene Invarianten: Fail-Closed bei Fehlern, kein ungeschützter globaler Zustand.

// FILE-CONTEXT
// ZWECK: Acta Core Engine Orchestrator & Facade (Layer 3 - Engine).
// INVARIANTEN: Monoton steigende TxId-Allokation; Reparaturgarantie beim Öffnen (repair_on_open); Strikte Isolation von Namespaces.
// NICHT-OFFENSICHTLICH: Lock-Hierarchie: collections (RwLock) -> kv_locks -> embedder (RwLock).

#![forbid(unsafe_code)]

pub use acta_crypto::deletion_proof::{
    DeletionLayer, DeletionProof, DeletionScope, ExcludedScope,
};
use acta_ports::license::LicenseGate;
#[cfg(feature = "sandbox")]
use acta_ports::BoxFuture;
use acta_ports::StorageEngine;
pub use acta_ports::TextEmbeddingEngine;
pub use acta_ports::VectorDeleteMode;
#[cfg(not(loom))]
pub use acta_store::lsm::DurabilityMode;
#[cfg(not(loom))]
use acta_store::LsmStorage;
use acta_types::{DocId, TenantId};
use acta_vector::{HnswConfig, HnswIndex};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

#[cfg(not(loom))]
pub mod background_workers;
#[cfg(not(loom))]
pub mod chunker;
pub mod collection;
#[cfg(not(loom))]
pub mod decay_controller;
#[cfg(not(loom))]
pub mod export;
#[cfg(feature = "entity-extraction")]
#[cfg(not(loom))]
pub mod extraction;
#[cfg(not(loom))]
pub mod filter;
#[cfg(not(loom))]
pub mod fusion;
#[cfg(not(loom))]
pub mod import;
#[cfg(not(loom))]
pub mod metrics_names;
#[cfg(not(loom))]
pub mod temporal_filter;
#[cfg(not(loom))]
pub mod transaction;

#[cfg(feature = "sandbox")]
pub mod merge_operator_adapter;
#[cfg(feature = "sandbox")]
pub use merge_operator_adapter::WasmMergeOperator;

#[cfg(not(loom))]
pub use decay_controller::{AdaptiveDecayController, DecayControllerConfig, DecaySignalInputs};

#[cfg(not(loom))]
pub use export::{
    ExportCollectionV1, ExportDocumentV1, ExportMemoryV1, ExportRelationV1, SCHEMA_VERSION_V1,
};
#[cfg(not(loom))]
pub use import::ImportSummary;

#[cfg(not(loom))]
#[cfg(feature = "background-maintenance")]
pub use background_workers::start_decay_cleanup_worker;
#[cfg(not(loom))]
pub use background_workers::{start_expiry_cleanup_worker, start_orphan_cleanup_worker};

#[cfg(not(loom))]
pub use collection::crud::MAX_SCAN_RESULTS;
#[cfg(feature = "graph-connectivity-health")]
pub use collection::maintenance::PercolationConfig;
#[cfg(not(loom))]
#[cfg(feature = "graph-connectivity-health")]
pub use collection::maintenance::PercolationResult;
#[cfg(not(loom))]
pub use collection::query_builder::{HybridQueryBuilder, SearchStrategy, SignalWeights};
#[cfg(not(loom))]
pub use collection::{Collection, CollectionConfig};
pub use acta_checkpoint;
pub use acta_text::Language;
#[cfg(not(loom))]
#[allow(deprecated)]
pub use filter::MetadataFilter;

#[cfg(not(loom))]
#[allow(clippy::type_complexity)]
// ANCHOR[acta-engine:LCH-001]
/// Funktions-Typdefinition zum Starten von Wissenskonsolidierungs-Workern.
pub type ConsolidationLauncher = Arc<
    dyn Fn(
            Arc<Collection<LsmStorage>>,
            std::time::Duration,
            usize,
            tokio_util::sync::CancellationToken,
        ) -> tokio::task::JoinHandle<()>
        + Send
        + Sync,
>;

// ANCHOR[acta-engine:LCH-002]
/// Registriert eine Konsolidierungs-Launcher-Funktion (deprecated: via `ActaConfig` setzen).
#[cfg(not(loom))]
#[deprecated(
    note = "Instanzgebundenen Launcher via ActaConfig::with_consolidation_launcher setzen (P29)"
)]
pub fn register_consolidation_launcher<F>(_f: F)
where
    F: Fn(
            Arc<Collection<LsmStorage>>,
            std::time::Duration,
            usize,
            tokio_util::sync::CancellationToken,
        ) -> tokio::task::JoinHandle<()>
        + Send
        + Sync
        + 'static,
{
    tracing::warn!(
        "register_consolidation_launcher is deprecated and has no effect. Set consolidation_launcher on ActaConfig instead."
    );
}

#[cfg(not(loom))]
pub use fusion::{ProvenanceRecord, SearchResult, SignalContribution};

#[cfg(not(loom))]
#[derive(Debug, Clone, Serialize, Deserialize)]
// ANCHOR[acta-engine:DOC-001]
/// Repräsentiert ein Dokument mit eindeutiger ID, Text, Vektoreinbettung und Metadaten.
pub struct Document {
    pub id: String,
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// ANCHOR[acta-engine:STA-001]
/// Aggregierte Betriebs- und Belegungsstatistiken der Engine.
pub struct ActaStats {
    pub drift_status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calibration_ece: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_calibration_at: Option<u64>,
    pub active_memory_count: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pid_pool_size: Option<usize>,
    pub index_stats: acta_ports::VectorIndexStats,
    pub storage_stats: acta_ports::StorageStats,
}

pub use acta_ports::DriftStatusProvider;
// ANCHOR[acta-engine:STA-002]
/// Alias für `ActaStats` zur Abwärtskompatibilität.
pub type DbStats = ActaStats;

#[derive(Debug, Clone, Serialize, Deserialize)]
// ANCHOR[acta-engine:CMD-001]
/// Konfiguration für Community-Erkennungs- und Graph-Clustering-Algorithmen.
pub struct CommunityDetectionConfig {
    pub auto_trigger_threshold: u64,
}

impl Default for CommunityDetectionConfig {
    fn default() -> Self {
        Self {
            auto_trigger_threshold: 100,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
// ANCHOR[acta-engine:EMB-001]
/// Wählbares Backend zur Berechnung von Text-Embeddings.
pub enum EmbeddingBackend {
    Onnx {
        model_name: String,
        cache_dir: Option<std::path::PathBuf>,
    },
    None,
}

impl Default for EmbeddingBackend {
    fn default() -> Self {
        Self::Onnx {
            model_name: "nomic-embed-text".to_string(),
            cache_dir: None,
        }
    }
}

/// Tenant isolation enforcement policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TenantPolicy {
    /// Non-tenant collection access (`collection(...)`) is allowed (Default / Admin / Single-tenant path).
    #[default]
    Optional,
    /// Non-tenant collection access is forbidden; callers MUST use tenant-scoped handles (`collection_for_tenant(...)`).
    Required,
}

#[cfg(not(loom))]
#[derive(Clone)]
// ANCHOR[acta-engine:CFG-001]
/// Globale Konfiguration für die Acta-Engine-Initialisierung.
pub struct ActaConfig {
    pub dimension: usize,
    pub max_elements: usize,
    pub distance_metric: acta_types::DistanceMetric,
    pub encryption_passphrase: Option<String>,
    /// Maximum RAM the LSM storage engine may use (in MB). Passed directly to LsmConfig::max_ram_mb.
    /// Default: 2048 (matches LsmConfig default).
    pub max_ram_mb: u64,

    /// WAL group-commit window in microseconds. 0 = disabled (immediate single commit).
    /// Passed directly to LsmConfig::group_commit_window_micros.
    /// Default: 500 (matches LsmConfig default).
    pub group_commit_window_micros: u64,

    /// Durability mode for WAL persistence. Passed directly to LsmConfig::durability_mode.
    /// Default: DurabilityMode::Full.
    pub durability_mode: DurabilityMode,

    /// Controls whether deletion proof generation and HMAC integrity guarantees are active.
    /// Default: false.
    pub deletion_proof_active: bool,

    /// Controls vector index deletion repair behavior.
    /// Default: VectorDeleteMode::BackgroundRepair.
    pub vector_delete_mode: VectorDeleteMode,

    /// MemTable flush threshold in bytes. Passed directly to LsmConfig::memtable_size_limit.
    /// Default: 67_108_864 (64 MiB, matches LsmConfig default).
    pub memtable_size_limit: usize,
    pub expiry_reaper_interval: std::time::Duration,
    pub orphan_registry_path: Option<std::path::PathBuf>,
    pub community_detection: CommunityDetectionConfig,
    pub embedding_backend: EmbeddingBackend,
    pub consolidation_enabled: bool,
    pub consolidation_interval: std::time::Duration,
    pub max_llm_calls_per_cycle: usize,
    pub consolidation_launcher: Option<ConsolidationLauncher>,
    /// Policy controlling whether multi-tenant isolation is required.
    pub tenant_policy: TenantPolicy,
    /// Configured metrics sink for recording engine operational metrics.
    pub metrics_sink: Option<Arc<dyn acta_ports::MetricsSink>>,
}

#[cfg(not(loom))]
impl ActaConfig {
    // ANCHOR[acta-engine:BLD-003]
    /// Konfiguriert den optionalen Konsolidierungs-Launcher auf dem Builder.
    pub fn with_consolidation_launcher(mut self, launcher: ConsolidationLauncher) -> Self {
        self.consolidation_launcher = Some(launcher);
        self
    }

    // ANCHOR[acta-engine:BLD-004]
    /// Konfiguriert die Mandantenisolations- und Zugriffspolicy.
    pub fn with_tenant_policy(mut self, policy: TenantPolicy) -> Self {
        self.tenant_policy = policy;
        self
    }

    // ANCHOR[acta-engine:BLD-005]
    /// Konfiguriert die Metrik-Senke zur Erfassung von Systemmetriken.
    pub fn with_metrics_sink(mut self, sink: Arc<dyn acta_ports::MetricsSink>) -> Self {
        self.metrics_sink = Some(sink);
        self
    }
}

#[cfg(not(loom))]
impl std::fmt::Debug for ActaConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ActaConfig")
            .field("dimension", &self.dimension)
            .field("max_elements", &self.max_elements)
            .field("distance_metric", &self.distance_metric)
            .field(
                "encryption_passphrase",
                &self.encryption_passphrase.as_ref().map(|_| "***"),
            )
            .field("max_ram_mb", &self.max_ram_mb)
            .field(
                "group_commit_window_micros",
                &self.group_commit_window_micros,
            )
            .field("durability_mode", &self.durability_mode)
            .field("deletion_proof_active", &self.deletion_proof_active)
            .field("vector_delete_mode", &self.vector_delete_mode)
            .field("memtable_size_limit", &self.memtable_size_limit)
            .field("expiry_reaper_interval", &self.expiry_reaper_interval)
            .field("orphan_registry_path", &self.orphan_registry_path)
            .field("community_detection", &self.community_detection)
            .field("embedding_backend", &self.embedding_backend)
            .field("consolidation_enabled", &self.consolidation_enabled)
            .field("consolidation_interval", &self.consolidation_interval)
            .field("max_llm_calls_per_cycle", &self.max_llm_calls_per_cycle)
            .field(
                "consolidation_launcher",
                &self.consolidation_launcher.as_ref().map(|_| "Fn(...)"),
            )
            .field("tenant_policy", &self.tenant_policy)
            .field(
                "metrics_sink",
                &self.metrics_sink.as_ref().map(|_| "dyn MetricsSink"),
            )
            .finish()
    }
}

#[cfg(not(loom))]
impl Default for ActaConfig {
    fn default() -> Self {
        Self {
            dimension: 768,
            max_elements: 1_000_000,
            distance_metric: acta_types::DistanceMetric::Cosine,
            encryption_passphrase: None,
            max_ram_mb: 2048,
            group_commit_window_micros: 500,
            durability_mode: DurabilityMode::default(),
            deletion_proof_active: false,
            vector_delete_mode: VectorDeleteMode::BackgroundRepair,
            memtable_size_limit: 64 * 1024 * 1024,
            expiry_reaper_interval: std::time::Duration::from_secs(60),
            orphan_registry_path: None,
            community_detection: CommunityDetectionConfig::default(),
            embedding_backend: EmbeddingBackend::default(),
            consolidation_enabled: true,
            consolidation_interval: std::time::Duration::from_secs(6 * 3600),
            max_llm_calls_per_cycle: 10,
            consolidation_launcher: None,
            tenant_policy: TenantPolicy::Optional,
            metrics_sink: None,
        }
    }
}

#[cfg(not(loom))]
// ANCHOR[acta-engine:TCM-001]
/// Typdefinition für die mandantenisolierte Collection-Verwaltung.
pub type TenantCollectionMap = ahash::AHashMap<
    (TenantId, String),
    Arc<Collection<acta_store::tenant_codec::TenantScopedStorage<Arc<LsmStorage>>>>,
>;

#[cfg(not(loom))]
// ANCHOR[acta-engine:CTX-001]
/// Zentraler Orchestrator und Einstiegspunkt der Acta-Engine.
pub struct Acta {
    storage: Arc<LsmStorage>,
    next_tx: Arc<AtomicU64>,
    dimension: usize,
    config: ActaConfig,
    license_gate: Arc<dyn LicenseGate>,
    expiry_reaper_interval: std::time::Duration,
    community_detection_threshold: u64,
    collections: tokio::sync::RwLock<ahash::AHashMap<String, Arc<Collection<LsmStorage>>>>,
    tenant_collections: tokio::sync::RwLock<TenantCollectionMap>,
    cancel_token: tokio_util::sync::CancellationToken,
    task_tracker: tokio_util::task::TaskTracker,
    embedder: parking_lot::RwLock<Option<Arc<dyn TextEmbeddingEngine>>>,
    orphan_registry: Arc<acta_checkpoint::InstanceOrphanRegistry>,
    router: parking_lot::RwLock<Option<std::sync::Weak<dyn DriftStatusProvider>>>,
    calibrator: parking_lot::RwLock<
        Option<std::sync::Weak<parking_lot::Mutex<acta_rank::IsotonicCalibrator>>>,
    >,
    metrics_sink: parking_lot::RwLock<Arc<dyn acta_ports::MetricsSink>>,
    proof_key_provider:
        parking_lot::RwLock<Option<Arc<dyn acta_impl::erasure::ProofKeyProvider>>>,
}

pub use acta_impl::erasure::{
    summary_from_witness, ErasureFailure, ErasureReport, LayerSummary, ProofKeyProvider,
    UNMEASURED_ERASURE_LAYERS,
};

#[cfg(not(loom))]
mod acta_impl;

pub use acta_types::DistanceMetric;
pub use serde_json::json;

#[cfg(not(loom))]
impl Acta {
    // ANCHOR[acta-engine:STR-001]
    /// Gibt den zugrundeliegenden LSM-Speicher zurück (intern/verborgen).
    #[doc(hidden)]
    pub fn inner_storage(&self) -> Arc<LsmStorage> {
        self.storage.clone()
    }

    /// Returns a reference to the active [`ActaConfig`].
    pub fn config(&self) -> &ActaConfig {
        &self.config
    }

    /// Returns a reference to the attached [`LicenseGate`].
    pub fn license_gate(&self) -> Arc<dyn LicenseGate> {
        Arc::clone(&self.license_gate)
    }
}

// ANCHOR[acta-engine:SND-001]
/// Brücke zur sicheren Ausführung von sandboxed Operatoren (z.B. Wasm).
#[cfg(feature = "sandbox")]
pub trait SandboxBridge: Send + Sync {
    fn db_search<'a>(
        &'a self,
        query: &'a [u8],
        k: usize,
    ) -> BoxFuture<'a, acta_types::Result<Vec<u8>>>;
    fn db_insert<'a>(
        &'a self,
        key: &'a [u8],
        value: &'a [u8],
    ) -> BoxFuture<'a, acta_types::Result<()>>;
    fn db_get<'a>(
        &'a self,
        key: &'a [u8],
    ) -> BoxFuture<'a, acta_types::Result<Option<Vec<u8>>>>;
}

#[cfg(feature = "sandbox")]
impl SandboxBridge for Acta {
    fn db_search<'a>(
        &'a self,
        query: &'a [u8],
        k: usize,
    ) -> BoxFuture<'a, acta_types::Result<Vec<u8>>> {
        Box::pin(async move {
            let f32_count = query.len() / 4;
            let mut vector = Vec::with_capacity(f32_count);
            for i in 0..f32_count {
                let start = i * 4;
                let bits = u32::from_le_bytes(
                    query
                        .get(start..start + 4)
                        .ok_or_else(|| {
                            acta_types::ActaError::Serialization("Query too short".into())
                        })?
                        .try_into()
                        .map_err(|_| {
                            acta_types::ActaError::Serialization("Invalid slice".into())
                        })?,
                );
                vector.push(f32::from_bits(bits));
            }

            let results: Vec<SearchResult> = self.search(&vector, k).await?;
            serde_json::to_vec(&results)
                .map_err(|e| acta_types::ActaError::Internal(e.to_string()))
        })
    }

    fn db_insert<'a>(
        &'a self,
        key: &'a [u8],
        value: &'a [u8],
    ) -> BoxFuture<'a, acta_types::Result<()>> {
        Box::pin(async move {
            let id = String::from_utf8_lossy(key).to_string();
            let val_json: Value = serde_json::from_slice(value)
                .unwrap_or(serde_json::json!({ "raw_data": String::from_utf8_lossy(value) }));

            self.insert(&id, &[], Some(val_json)).await
        })
    }

    fn db_get<'a>(
        &'a self,
        key: &'a [u8],
    ) -> BoxFuture<'a, acta_types::Result<Option<Vec<u8>>>> {
        Box::pin(async move {
            let id = String::from_utf8_lossy(key).to_string();
            let doc = self.get(&id).await?;
            match doc {
                Some(d) => Ok(Some(serde_json::to_vec(&d).map_err(|e| {
                    acta_types::ActaError::Internal(e.to_string())
                })?)),
                None => Ok(None),
            }
        })
    }
}
