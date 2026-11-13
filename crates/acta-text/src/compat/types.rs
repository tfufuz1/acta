//! Typkonvertierungen zwischen lokalen Typen und `acta-types`.
// ANCHOR[acta-text:COMPAT-TYPES-001]

use crate::types::{DocId, TextHit};

impl From<acta_types::DocId> for DocId {
    #[inline]
    fn from(id: acta_types::DocId) -> Self {
        #[cfg(not(feature = "docid-128"))]
        {
            Self(id.inner())
        }
        #[cfg(feature = "docid-128")]
        {
            Self(id.inner() as u128)
        }
    }
}

impl From<DocId> for acta_types::DocId {
    #[inline]
    fn from(id: DocId) -> Self {
        #[cfg(not(feature = "docid-128"))]
        {
            acta_types::DocId::new(id.inner() as _)
        }
        #[cfg(feature = "docid-128")]
        {
            acta_types::DocId::new(id.inner() as _)
        }
    }
}

impl From<TextHit> for acta_types::ScoredDocument {
    fn from(hit: TextHit) -> Self {
        Self {
            doc_id: hit.doc_id.into(),
            score: hit.score,
        }
    }
}

impl From<acta_types::ScoredDocument> for TextHit {
    fn from(doc: acta_types::ScoredDocument) -> Self {
        Self {
            doc_id: doc.doc_id.into(),
            score: doc.score,
        }
    }
}
