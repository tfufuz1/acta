//! Konvertierungen für Typen zwischen acta-vector und acta-types (SPEC-v2 §4.3).

use crate::types::{DistanceMetric, ScoredDocument};

impl From<DistanceMetric> for acta_types::DistanceMetric {
    fn from(m: DistanceMetric) -> Self {
        match m {
            DistanceMetric::Cosine => acta_types::DistanceMetric::Cosine,
            DistanceMetric::Euclidean => acta_types::DistanceMetric::Euclidean,
            DistanceMetric::DotProduct => acta_types::DistanceMetric::DotProduct,
        }
    }
}

impl From<acta_types::DistanceMetric> for DistanceMetric {
    fn from(m: acta_types::DistanceMetric) -> Self {
        match m {
            acta_types::DistanceMetric::Cosine => DistanceMetric::Cosine,
            acta_types::DistanceMetric::Euclidean => DistanceMetric::Euclidean,
            acta_types::DistanceMetric::DotProduct => DistanceMetric::DotProduct,
            _ => DistanceMetric::Cosine,
        }
    }
}

impl From<ScoredDocument> for acta_types::ScoredDocument {
    fn from(sd: ScoredDocument) -> Self {
        acta_types::ScoredDocument {
            doc_id: sd.doc_id,
            score: sd.score,
        }
    }
}

impl From<acta_types::ScoredDocument> for ScoredDocument {
    fn from(sd: acta_types::ScoredDocument) -> Self {
        ScoredDocument {
            doc_id: sd.doc_id,
            score: sd.score,
            distance: 1.0 - sd.score,
        }
    }
}
