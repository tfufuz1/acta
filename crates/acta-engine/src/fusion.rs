//! Hybrid Search Signal Fusion implementations (Reciprocal Rank Fusion & Score Normalization).
//!
//! Re-exports and redirects all fusion types and operations to `acta-rank`.

pub use acta_rank::fusion::{
    apply_resonance_bonus, reciprocal_rank_fusion, score_normalized_fusion_with_options,
    weighted_reciprocal_rank_fusion, weighted_reciprocal_rank_fusion_with_options,
    weighted_reciprocal_rank_fusion_with_priority, weights_to_signal_factors, BoundedTopK,
    MetadataMergePriority, ProvenanceBuilder, ProvenanceRecord, ResonanceConfig, SearchReport,
    SearchResult, Signal, SignalContribution, SignalFailurePolicy, SignalKind,
};

pub use acta_types::{FusionStrategy, SignalFusionStrategies};

/// Fuses search result sets using the specified FusionStrategy.
pub fn fuse_search_results_with_strategy(
    result_sets: Vec<(String, Vec<SearchResult>, f32)>,
    max_results: usize,
    priority: MetadataMergePriority,
    include_provenance: bool,
    resonance_config: Option<&ResonanceConfig>,
    strategy: acta_types::FusionStrategy,
) -> Vec<SearchResult> {
    acta_rank::fusion::fuse_search_results_with_strategy(
        result_sets,
        max_results,
        priority,
        include_provenance,
        resonance_config,
        strategy,
    )
}

/// Fuses search result sets using per-signal fusion strategies (`SignalFusionStrategies`).
pub fn fuse_search_results_with_signal_strategies(
    result_sets: Vec<(String, Vec<SearchResult>, f32)>,
    max_results: usize,
    priority: MetadataMergePriority,
    include_provenance: bool,
    resonance_config: Option<&ResonanceConfig>,
    strategies: acta_types::SignalFusionStrategies,
) -> Vec<SearchResult> {
    acta_rank::fusion::fuse_search_results_with_signal_strategies(
        result_sets,
        max_results,
        priority,
        include_provenance,
        resonance_config,
        strategies,
    )
}

/// Convenience function for fusing multi-signal search results.
pub fn fuse_signals(
    result_sets: Vec<(String, Vec<SearchResult>, f32)>,
    max_results: usize,
) -> Vec<SearchResult> {
    acta_rank::fusion::fuse_signals(result_sets, max_results)
}
