//! Fehlerkonvertierung StoreError -> ActaError (SPEC §5.3, §6.2)

use crate::error::StoreError;
use acta_types::ActaError;

#[cfg(test)]
pub mod tests;

impl From<StoreError> for ActaError {
    fn from(err: StoreError) -> Self {
        match err {
            StoreError::Storage(s) => ActaError::Storage(s),
            StoreError::Io(e) => ActaError::Io(e),
            StoreError::WalCorruption { offset, reason } => {
                ActaError::wal_corruption(offset, reason)
            }
            StoreError::ChecksumMismatch { path, block_id } => {
                ActaError::checksum_mismatch(path, block_id)
            }
            StoreError::WalTruncationDetected {
                expected_hmac,
                actual_hmac,
            } => ActaError::wal_truncation_detected(expected_hmac, actual_hmac),
            StoreError::CrossDeviceLink {
                source_path,
                target_path,
            } => ActaError::cross_device_link(source_path, target_path),
            StoreError::Transaction(s) => ActaError::Transaction(s),
            StoreError::Conflict(s) => ActaError::Conflict(s),
            StoreError::CommitTimeout { tx_id } => ActaError::CommitTimeout { tx_id },
            StoreError::Serialization(s) => ActaError::Serialization(s),
            StoreError::ParseError(s) => ActaError::ParseError(s),
            StoreError::InvalidInput(s) => ActaError::InvalidInput(s),
            StoreError::NotFound(s) => ActaError::NotFound(s),
            StoreError::Internal(s) => ActaError::Internal(s),
            StoreError::MemoryBudgetExceeded { used_mb, limit_mb } => {
                ActaError::MemoryBudgetExceeded { used_mb, limit_mb }
            }
            StoreError::LimitExceeded { limit, context } => {
                ActaError::LimitExceeded { limit, context }
            }
            StoreError::DurabilityConfig(s) => ActaError::durability_config(s),
            StoreError::Sandbox(s) => ActaError::Sandbox(s),
        }
    }
}
