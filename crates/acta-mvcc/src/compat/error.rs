// ANCHOR[id=compat-error-root]
//! # Workspace-Kompatibilitätsabbildung für Fehler
//!
//! LINK crates/acta-mvcc/src/mvcc_error.rs
//! LINK crates/acta-mvcc/src/compat/mod.rs
//!
//! Dieses Modul implementiert die tabellengesteuerte Konvertierung des lokalen `MvccError`
//! in den globalen `acta_types::ActaError`.

use crate::mvcc_error::MvccError;
use acta_types::ActaError;

// SECTION Error Conversion
// ANCHOR[id=from-mvcc-error-for-acta-error]
impl From<MvccError> for ActaError {
    fn from(err: MvccError) -> Self {
        // NOTE: Bit- und stringgleiche Abbildung gemäß SPEC-v2 §5.3 / SPEC-v3 §5.3
        match err {
            MvccError::Conflict(msg) => ActaError::Conflict(msg),
            MvccError::LimitExceeded { limit, context } => {
                ActaError::limit_exceeded(limit, context)
            }
            MvccError::Transaction(msg) => ActaError::Transaction(msg),
            MvccError::Storage(msg) => ActaError::Storage(msg),
            MvccError::InvalidInput(msg) => ActaError::InvalidInput(msg),
            MvccError::Internal(msg) => ActaError::Internal(msg),
        }
    }
}
// !SECTION Error Conversion

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_mapping_table_driven() {
        let cases = vec![
            (
                MvccError::Conflict("conflict msg".into()),
                ActaError::Conflict("conflict msg".into()),
            ),
            (
                MvccError::limit_exceeded(5, "snapshots"),
                ActaError::limit_exceeded(5, "snapshots"),
            ),
            (
                MvccError::Transaction("aborted".into()),
                ActaError::Transaction("aborted".into()),
            ),
            (
                MvccError::Storage("shard out of bounds".into()),
                ActaError::Storage("shard out of bounds".into()),
            ),
            (
                MvccError::InvalidInput("sentinel".into()),
                ActaError::InvalidInput("sentinel".into()),
            ),
            (
                MvccError::Internal("corrupt state".into()),
                ActaError::Internal("corrupt state".into()),
            ),
        ];

        for (mvcc_err, expected_ctx_err) in cases {
            let actual_ctx_err: ActaError = mvcc_err.into();
            assert_eq!(actual_ctx_err.to_string(), expected_ctx_err.to_string());
        }
    }
}
