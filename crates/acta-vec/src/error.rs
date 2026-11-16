//! Fehlertyp von acta-vec (HR-4: ein Fehlertyp je Crate, kein `Other(String)`).

use acta_base::{ClassifiedError, ErrorClass};

/// Fehler von acta-vec (SPEC 3.3, 9.3).
///
/// Gemäß Z15 enthalten Meldungen und Fehlercodes weder Werte noch Längen oder Kennungen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum VecError {
    /// Vektordimensionen stimmen nicht überein.
    #[error("vector dimension mismatch")]
    DimensionMismatch,
    /// Nicht-endliche Gleitkommazahl (NaN oder Inf) im Eingabevektor.
    #[error("non-finite float in vector")]
    NonFiniteFloat,
    /// Leere Vektordaten übergeben.
    #[error("empty vector data")]
    EmptyVector,
    /// Leere Herkunft (Z13: Ableitungen ohne Herkunft unzulässig).
    #[error("empty lineage")]
    EmptyLineage,
    /// Segmentkapazität überschritten.
    #[error("segment capacity exceeded")]
    CapacityExceeded,
    /// Zugriff durch Gate gesperrt.
    #[error("gate violation")]
    GateViolation,
    /// Segmentdaten beschädigt.
    #[error("corrupted segment data")]
    CorruptedSegment,
    /// Positivkontrolle der Canary-Messung fehlgeschlagen (Z11).
    #[error("canary measurement failed")]
    CanaryFailed,
    /// Funktion ist noch nicht umgesetzt.
    #[error("not implemented")]
    Unimplemented,
}

impl ClassifiedError for VecError {
    fn class(&self) -> ErrorClass {
        match self {
            Self::DimensionMismatch
            | Self::NonFiniteFloat
            | Self::EmptyVector
            | Self::EmptyLineage
            | Self::CapacityExceeded
            | Self::CanaryFailed
            | Self::Unimplemented => ErrorClass::Invalid,
            Self::GateViolation => ErrorClass::Erased,
            Self::CorruptedSegment => ErrorClass::Integrity,
        }
    }

    fn code(&self) -> &'static str {
        match self {
            Self::DimensionMismatch => "vec.dimension_mismatch",
            Self::NonFiniteFloat => "vec.non_finite_float",
            Self::EmptyVector => "vec.empty_vector",
            Self::EmptyLineage => "vec.empty_lineage",
            Self::CapacityExceeded => "vec.capacity_exceeded",
            Self::GateViolation => "vec.gate_violation",
            Self::CorruptedSegment => "vec.corrupted_segment",
            Self::CanaryFailed => "vec.canary_failed",
            Self::Unimplemented => "vec.unimplemented",
        }
    }
}
