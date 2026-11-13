//! Fehlerbrücke zwischen `TextError` und `ActaError` (SPEC §5.3).
// ANCHOR[acta-text:COMPAT-ERR-001]

use crate::error::TextError;
use acta_types::ActaError;

impl From<TextError> for ActaError {
    fn from(err: TextError) -> Self {
        match err {
            TextError::InvalidInput(msg) => ActaError::InvalidInput(msg),
            TextError::Text(msg) => ActaError::Text(msg),
            TextError::CapacityExceeded { requested, maximum } => {
                ActaError::InvalidInput(format!(
                    "Kapazitätsgrenze überschritten: gefordert {requested}, maximal {maximum}"
                ))
            }
            TextError::Storage(msg) => ActaError::Storage(msg),
            TextError::Serialization(msg) => ActaError::Storage(msg),
            TextError::Io(e) => ActaError::Io(e),
            TextError::Internal(msg) => ActaError::Internal(msg),
        }
    }
}

impl From<ActaError> for TextError {
    fn from(err: ActaError) -> Self {
        match err {
            ActaError::InvalidInput(msg) => TextError::InvalidInput(msg),
            ActaError::Text(msg) => TextError::Text(msg),
            ActaError::Storage(msg) => TextError::Storage(msg),
            ActaError::Io(e) => TextError::Storage(e.to_string()),
            other => TextError::Internal(other.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_mapping_table_driven_bitgleich() {
        let cases: Vec<(TextError, fn(&ActaError) -> bool)> = vec![
            (
                TextError::InvalidInput("bad query".into()),
                |ce| matches!(ce, ActaError::InvalidInput(msg) if msg == "bad query"),
            ),
            (
                TextError::Text("regex fail".into()),
                |ce| matches!(ce, ActaError::Text(msg) if msg == "regex fail"),
            ),
            (
                TextError::CapacityExceeded {
                    requested: 10,
                    maximum: 5,
                },
                |ce| matches!(ce, ActaError::InvalidInput(msg) if msg == "Kapazitätsgrenze überschritten: gefordert 10, maximal 5"),
            ),
            (
                TextError::Storage("corrupt wal".into()),
                |ce| matches!(ce, ActaError::Storage(msg) if msg == "corrupt wal"),
            ),
            (
                TextError::Serialization("bincode err".into()),
                |ce| matches!(ce, ActaError::Storage(msg) if msg == "bincode err"),
            ),
            (
                TextError::Internal("state lock".into()),
                |ce| matches!(ce, ActaError::Internal(msg) if msg == "state lock"),
            ),
        ];

        for (te, predicate) in cases {
            let te_display = te.to_string();
            let ce: ActaError = te.into();
            assert!(predicate(&ce));
            // Display-Verhalten prüfen
            match &ce {
                ActaError::InvalidInput(m) | ActaError::Text(m) | ActaError::Storage(m) | ActaError::Internal(m) => {
                    assert!(te_display.contains(m));
                }
                _ => {}
            }
        }

        // Reverse conversion
        let c_inv = ActaError::InvalidInput("inv".into());
        let t_inv: TextError = c_inv.into();
        assert!(matches!(t_inv, TextError::InvalidInput(s) if s == "inv"));
    }
}
