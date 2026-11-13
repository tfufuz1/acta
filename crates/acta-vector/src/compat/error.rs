//! Fehler-Abbildung für `acta-vector` (SPEC-v2 §5.3, compat/).
// Nur aktiv wenn das workspace-Feature (oder das ältere ports-compat-Alias) eingeschaltet ist.
#![cfg(any(feature = "ports-compat", feature = "workspace"))]

use crate::error::VectorError;

impl From<VectorError> for acta_types::ActaError {
    fn from(err: VectorError) -> Self {
        match err {
            VectorError::InvalidInput(msg) => acta_types::ActaError::InvalidInput(msg),
            VectorError::Index(msg) => acta_types::ActaError::Index(msg),
            VectorError::Storage(msg) => acta_types::ActaError::Storage(msg),
            VectorError::DimensionMismatch { expected, actual } => {
                acta_types::ActaError::EmbeddingDimensionMismatch {
                    expected,
                    got: actual,
                }
            }
            VectorError::ConnectivityDegraded { deleted_ratio } => {
                acta_types::ActaError::HnswConnectivityDegraded { deleted_ratio }
            }
            VectorError::GraphRepairFailed(_msg) => {
                acta_types::ActaError::GraphRepairFailed(
                    acta_types::HnswDeletionError::VerificationFailed {
                        remaining_pointers: 1,
                    },
                )
            }
            VectorError::NotFound(msg) => acta_types::ActaError::NotFound(msg),
            VectorError::Conflict(msg) => acta_types::ActaError::Conflict(msg),
            VectorError::Concurrency(msg) => acta_types::ActaError::Index(msg),
            VectorError::Io(msg) => acta_types::ActaError::Io(std::io::Error::other(msg)),
            VectorError::Unsupported(msg) => {
                acta_types::ActaError::capability_unsupported("vector", msg)
            }
            VectorError::Internal(msg) => acta_types::ActaError::Internal(msg),
        }
    }
}

impl From<acta_types::ActaError> for VectorError {
    fn from(err: acta_types::ActaError) -> Self {
        match err {
            acta_types::ActaError::InvalidInput(msg) => VectorError::InvalidInput(msg),
            acta_types::ActaError::Index(msg) => VectorError::Index(msg),
            acta_types::ActaError::Storage(msg) => VectorError::Storage(msg),
            acta_types::ActaError::EmbeddingDimensionMismatch { expected, got } => {
                VectorError::DimensionMismatch {
                    expected,
                    actual: got,
                }
            }
            acta_types::ActaError::HnswConnectivityDegraded { deleted_ratio } => {
                VectorError::ConnectivityDegraded { deleted_ratio }
            }
            acta_types::ActaError::GraphRepairFailed(e) => {
                VectorError::GraphRepairFailed(e.to_string())
            }
            acta_types::ActaError::NotFound(msg) => VectorError::NotFound(msg),
            acta_types::ActaError::Conflict(msg) => VectorError::Conflict(msg),
            acta_types::ActaError::Io(e) => VectorError::Io(e.to_string()),
            acta_types::ActaError::CapabilityUnsupported { capability, reason } => {
                VectorError::Unsupported(format!("{capability}: {reason}"))
            }
            acta_types::ActaError::Internal(msg) => VectorError::Internal(msg),
            other => VectorError::Index(other.to_string()),
        }
    }
}
