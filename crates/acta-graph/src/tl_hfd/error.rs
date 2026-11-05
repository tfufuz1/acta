//! Error types for Thresholded Local Hyper-Flow Diffusion (TL-HFD).

use thiserror::Error;

/// Error type for TL-HFD operations and parameter validations.
#[derive(Error, Debug, Clone, PartialEq)]
pub enum TlHfdError {
    /// Invalid parameter value provided to TL-HFD.
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    /// Non-finite value encountered during diffusion calculation.
    #[error("Non-finite value encountered during diffusion calculation")]
    NonFinite,
}

impl From<TlHfdError> for acta_types::ActaError {
    fn from(err: TlHfdError) -> Self {
        acta_types::ActaError::InvalidInput(err.to_string())
    }
}
