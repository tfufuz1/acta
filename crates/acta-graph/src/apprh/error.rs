//! Error types for Averaging-based Personalized PageRank for Hypergraphs (APPRH).

use thiserror::Error;

/// Error type for APPRH operations and parameter validations.
#[derive(Error, Debug, Clone, PartialEq)]
// ANCHOR[acta-graph:SRC-APPRH-ERROR-001]
pub enum ApprhError {
    /// Invalid parameter value provided to APPRH.
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    /// Empty seed list provided to APPRH.
    #[error("Seed list cannot be empty")]
    EmptySeeds,

    /// Non-finite value encountered during APPRH calculation.
    #[error("Non-finite value encountered during APPRH calculation")]
    NonFinite,
}

impl From<ApprhError> for acta_types::ActaError {
    fn from(err: ApprhError) -> Self {
        acta_types::ActaError::InvalidInput(err.to_string())
    }
}
