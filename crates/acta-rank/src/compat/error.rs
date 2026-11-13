//! Fehlerkonvertierung für `acta-rank` nach `acta_types::ActaError`.

use crate::error::RankError;

impl From<RankError> for acta_types::ActaError {
    fn from(err: RankError) -> Self {
        match err {
            RankError::InvalidInput(msg) => acta_types::ActaError::invalid_input(msg),
            RankError::Calibration(msg) => acta_types::ActaError::invalid_input(msg),
            RankError::FusionFailed(msg) => acta_types::ActaError::invalid_input(msg),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rank_error_to_acta_error_mapping() {
        let cases = vec![
            (
                RankError::InvalidInput("invalid doc id".to_string()),
                "Invalid input: invalid doc id",
            ),
            (
                RankError::Calibration("pava error".to_string()),
                "Invalid input: pava error",
            ),
            (
                RankError::FusionFailed("no active sets".to_string()),
                "Invalid input: no active sets",
            ),
        ];

        for (rank_err, expected_display) in cases {
            let ctx_err: acta_types::ActaError = rank_err.into();
            assert_eq!(ctx_err.to_string(), expected_display);
            assert!(
                matches!(ctx_err, acta_types::ActaError::InvalidInput(_)),
                "Erwartete InvalidInput, erhalten: {ctx_err:?}"
            );
        }
    }
}
