//! Fehlertyp von acta-rag (HR-4: ein Fehlertyp je Crate, kein `Other(String)`).
//!
//! Alle Varianten sind feste Befundcodes ohne Nutzdaten (Z15): Weder Kennungen noch Werte
//! erscheinen in Meldungen oder Codes.

use acta_base::{ClassifiedError, ErrorClass};

/// Fehler von acta-rag.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum RagError {
    /// Eine Trefferliste nennt denselben Schlüssel mehrfach.
    #[error("candidate listed more than once")]
    DuplicateCandidate,
    /// Ein Parameter oder ein Wert liegt außerhalb des zulässigen Bereichs.
    #[error("value out of range")]
    OutOfRange,
    /// Die Reranker-Werte einer Auswahl sind nur teilweise vorhanden.
    #[error("rerank scores incomplete")]
    IncompleteRerankScores,
    /// Eine Gültigkeitsspanne ist leer oder umgekehrt.
    #[error("validity window is empty")]
    EmptyValidityWindow,
    /// Eine Kennzahl ist mangels Referenzdaten nicht definiert. Das ist ein Messfehler und wird
    /// nie als Erfolg gemeldet.
    #[error("measurement not defined")]
    NotMeasurable,
    /// Ganzzahlüberlauf bei einer Summe, Summand- oder Rangbildung.
    #[error("arithmetic overflow")]
    ArithmeticOverflow,
}

impl ClassifiedError for RagError {
    fn class(&self) -> ErrorClass {
        ErrorClass::Invalid
    }

    fn code(&self) -> &'static str {
        match self {
            Self::DuplicateCandidate => "rag.duplicate_candidate",
            Self::OutOfRange => "rag.out_of_range",
            Self::IncompleteRerankScores => "rag.rerank_incomplete",
            Self::EmptyValidityWindow => "rag.validity_empty",
            Self::NotMeasurable => "rag.not_measurable",
            Self::ArithmeticOverflow => "rag.overflow",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // SPEC: 15.2, Z15 (Befundcodes sind stabil, eindeutig und ohne Nutzdaten)
    #[test]
    fn codes_are_distinct_invalid_and_free_of_values() {
        let all = [
            RagError::DuplicateCandidate,
            RagError::OutOfRange,
            RagError::IncompleteRerankScores,
            RagError::EmptyValidityWindow,
            RagError::NotMeasurable,
            RagError::ArithmeticOverflow,
        ];
        for (i, a) in all.iter().enumerate() {
            assert_eq!(a.class(), ErrorClass::Invalid);
            assert!(!a.to_string().chars().any(|c| c.is_ascii_digit()));
            for b in all.iter().skip(i + 1) {
                assert_ne!(a.code(), b.code());
            }
        }
    }
}
