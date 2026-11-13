//! # Konsumenten-Anker (S-64A)
//!
//! Reiner Compile-Time-Pin-Test, der sicherstellt, dass alle öffentlichen Re-Exporte
//! von `acta-engine` sowie alle Schnittstellen, die von `acta-db` und `acta-cognition`
//! konsumiert werden, vorhanden, unverändert typisiert und quellkompatibel bleiben.

#![allow(dead_code, unused_imports)]

use std::sync::Arc;

// 1. Konsumenten-Pfade von acta-db
fn _pin_db_consumer_surface() {
    use acta_engine::collection::{self, Collection, CollectionConfig};
    use acta_engine::{
        Acta, ActaConfig, ActaStats, DbStats, Document, EmbeddingBackend,
        HybridQueryBuilder, Language, ProvenanceRecord, SearchResult, SearchStrategy,
        SignalContribution, SignalWeights,
    };
    use acta_engine::DurabilityMode;
    use acta_engine::fusion;
    use acta_engine::decay_controller::{
        AdaptiveDecayController, DecayControllerConfig, DecaySignalInputs,
    };
    use acta_engine::{
        background_workers, chunker, export, filter, import, temporal_filter, transaction,
        CommunityDetectionConfig, ExportCollectionV1, ExportDocumentV1, ExportMemoryV1,
        ExportRelationV1, ImportSummary, MAX_SCAN_RESULTS, SCHEMA_VERSION_V1,
    };

    #[cfg(feature = "sandbox")]
    use acta_engine::SandboxBridge;
}

// 2. Konsumenten-Pfade von acta-cognition
fn _pin_cognition_consumer_surface() {
    use acta_engine::collection::Collection;
    use acta_engine::Language;
    let _lang = Language::English;
    let _lang_de = Language::German;
}

// 3. Öffentliche Re-Exporte der Engine Facade (src/lib.rs)
fn _pin_engine_public_surface() {
    use acta_engine::{
        Backends, Collection, CollectionConfig, CommunityDetectionConfig, Acta,
        ActaConfig, ActaStats, DbStats, Document, DurabilityMode, EmbeddingBackend,
        HybridQueryBuilder, Language, ProvenanceRecord, SearchResult, SearchStrategy,
        SignalContribution, SignalWeights, TenantCollectionMap, TenantPolicy,
        TextEmbeddingEngine, VectorDeleteMode, MAX_SCAN_RESULTS,
    };

    #[cfg(feature = "graph-connectivity-health")]
    use acta_engine::{PercolationConfig, PercolationResult};

    #[cfg(feature = "background-maintenance")]
    use acta_engine::start_decay_cleanup_worker;

    use acta_engine::{start_expiry_cleanup_worker, start_orphan_cleanup_worker};
}

#[test]
fn test_public_surface_compile_time_anchor() {
    // Reiner Compile-Time-Check, benötigt keine Laufzeitlogik
}
