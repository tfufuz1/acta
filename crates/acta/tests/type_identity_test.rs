//! Compile-time and runtime type identity verification for acta facade re-exports.
//! Ensures no breaking type-identity changes were introduced.

#![forbid(unsafe_code)]

fn _assert_same_type<T>(_: T, _: T) {}

#[test]
fn test_facade_type_identities() {
    // 1. ActaError
    let facade_err: acta::ActaError =
        acta::ActaError::InvalidInput("test".into());
    let types_err: acta_types::ActaError =
        acta_types::ActaError::InvalidInput("test".into());
    _assert_same_type(facade_err, types_err);

    // 2. DocId
    let facade_doc_id: acta::DocId = acta::DocId::from_u64(1);
    let types_doc_id: acta_types::DocId = acta_types::DocId::from_u64(1);
    _assert_same_type(facade_doc_id, types_doc_id);

    // 3. ScoredDocument
    let facade_doc = acta::ScoredDocument {
        id: acta::DocId::from_u64(1),
        score: 1.0,
        metadata: None,
        vector: None,
    };
    let types_doc = acta_types::ScoredDocument {
        id: acta_types::DocId::from_u64(1),
        score: 1.0,
        metadata: None,
        vector: None,
    };
    _assert_same_type(facade_doc, types_doc);

    // 4. DistanceMetric
    let facade_metric: acta::DistanceMetric = acta::DistanceMetric::Cosine;
    let types_metric: acta_types::DistanceMetric = acta_types::DistanceMetric::Cosine;
    _assert_same_type(facade_metric, types_metric);

    // 5. AutoExtractionMode
    let facade_auto: acta::collection_profile::AutoExtractionMode =
        acta::collection_profile::AutoExtractionMode::Enabled;
    let types_auto: acta_types::types::domain::AutoExtractionMode =
        acta_types::types::domain::AutoExtractionMode::Enabled;
    _assert_same_type(facade_auto, types_auto);
}

// Compile-time check via const fn pointers
const _: fn() = || {
    fn check_err(_: acta::ActaError) -> acta_types::ActaError {
        unreachable!()
    }
    fn check_metric(_: acta::DistanceMetric) -> acta_types::DistanceMetric {
        unreachable!()
    }
    fn check_doc_id(_: acta::DocId) -> acta_types::DocId {
        unreachable!()
    }
};
