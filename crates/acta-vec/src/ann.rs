//! ANN-Kandidatenerzeuger mit exaktem Nachrang (SPEC 9.3).
//!
//! Ein ANN-Backend liefert nur Kandidatenschlüssel. Die Bewertung ist exakt. Ein Erzeuger, der
//! sich nicht deterministisch verhält, schaltet Z7 für diesen Pfad aus. Das muss der Aufrufer
//! über [`AnnCandidateGenerator::is_deterministic`] ausweisen.

use std::collections::BTreeSet;

use acta_gate::state::Lineage;
use acta_gate::view::ErasedView;

use crate::error::VecError;
use crate::index::{is_visible, SegmentedVectorIndex};
use crate::quantize::QuantizedVector;
use crate::topk::TopK;
use crate::types::{CandidateFilter, VectorHit, VectorId};

/// Liefert Kandidatenschlüssel für eine Abfrage.
pub trait AnnCandidateGenerator: Send + Sync {
    /// Ob derselbe Eingabevektor immer dieselben Kandidaten liefert (Z7).
    fn is_deterministic(&self) -> bool;

    /// Höchstens `top_n` Kandidatenschlüssel. Die Reihenfolge ist ohne Bedeutung.
    fn generate_candidates(&self, query: &QuantizedVector, top_n: usize) -> Vec<VectorId>;
}

/// Suche mit ANN-Kandidaten und exakter Nachbewertung, ohne Zugriffsfilter.
///
/// Nur für Pfade, deren Zugriffsmenge der Aufrufer bereits anderweitig erzwungen hat.
/// Produktionspfade nutzen [`search_with_ann_visible`].
///
/// # Errors
/// `DimensionMismatch` bei falscher Abfragedimension, `CorruptedSegment` bei gesperrtem Zustand.
pub fn search_with_ann<A>(
    index: &SegmentedVectorIndex,
    query: &QuantizedVector,
    ann: &A,
    k: usize,
) -> Result<Vec<VectorHit>, VecError>
where
    A: AnnCandidateGenerator + ?Sized,
{
    rerank(index, query, ann, k, |_, _| true)
}

/// Suche mit ANN-Kandidaten, exakter Nachbewertung und Zugriffs- und Gate-Filter (Z1, Z16).
///
/// # Errors
/// Wie [`search_with_ann`].
pub fn search_with_ann_visible<A>(
    index: &SegmentedVectorIndex,
    query: &QuantizedVector,
    ann: &A,
    k: usize,
    filter: &CandidateFilter,
    view: &dyn ErasedView,
) -> Result<Vec<VectorHit>, VecError>
where
    A: AnnCandidateGenerator + ?Sized,
{
    rerank(index, query, ann, k, |key, lineage: &Lineage| {
        filter.allows(key) && is_visible(key, lineage, view)
    })
}

fn rerank<A, F>(
    index: &SegmentedVectorIndex,
    query: &QuantizedVector,
    ann: &A,
    k: usize,
    accept: F,
) -> Result<Vec<VectorHit>, VecError>
where
    A: AnnCandidateGenerator + ?Sized,
    F: Fn(VectorId, &Lineage) -> bool,
{
    if query.len() != index.dim() {
        return Err(VecError::DimensionMismatch);
    }
    let mut seen = BTreeSet::new();
    let mut top = TopK::new(k);
    for key in ann.generate_candidates(query, k) {
        if !seen.insert(key) {
            continue;
        }
        if let Some(score) = index.score_key(query, key, &accept)? {
            top.push(VectorHit { key, score });
        }
    }
    Ok(top.into_sorted())
}
