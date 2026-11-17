//! Reciprocal Rank Fusion in Ganzzahl-Arithmetik und Auswahl nach dem Reranker (SPEC 9.4, Z7).
//!
//! Jede Liste ist nach Rang geordnet; Position 0 entspricht Rang 1. Der Beitrag eines Treffers je
//! Liste ist `floor(2^32 / (rrf_k + rang))`; die Summe über alle Listen ist der Fusionswert. Es
//! gibt kein Gleitkomma: gleiche Eingaben liefern byte-identische Ergebnisse.

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

use acta_base::fx::Fx;

use crate::error::RagError;

/// Zähler der Ganzzahl-RRF: `floor(RRF_NUMERATOR / (rrf_k + rang))` (SPEC 9.4).
pub const RRF_NUMERATOR: u64 = 1 << 32;

/// Ergebnis der Fusion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fused<K> {
    /// Schlüssel des Treffers, bei Chunks `(KOrd, Chunk)`.
    pub key: K,
    /// Summe der ganzzahligen Beiträge über alle Listen, in denen der Schlüssel vorkommt.
    pub score: u64,
    /// Rang nach der Fusion, beginnend bei 1.
    pub rank: usize,
}

/// Fusioniert Trefferlisten nach RRF und behält die besten `keep` Treffer.
///
/// Gleichstand: Fusionswert absteigend, danach Schlüssel aufsteigend (SPEC 9.2, 9.4).
///
/// # Errors
/// `DuplicateCandidate`, wenn eine Liste einen Schlüssel mehrfach nennt. `ArithmeticOverflow`,
/// wenn `rrf_k + rang` oder eine Summe überläuft.
pub fn rrf_fuse<K: Ord + Clone>(
    lists: &[&[K]],
    rrf_k: u64,
    keep: usize,
) -> Result<Vec<Fused<K>>, RagError> {
    let mut scores: BTreeMap<K, u64> = BTreeMap::new();
    for list in lists {
        let mut seen: BTreeSet<&K> = BTreeSet::new();
        for (position, key) in list.iter().enumerate() {
            if !seen.insert(key) {
                return Err(RagError::DuplicateCandidate);
            }
            let rank = u64::try_from(position)
                .ok()
                .and_then(|p| p.checked_add(1))
                .ok_or(RagError::ArithmeticOverflow)?;
            let denominator = rrf_k
                .checked_add(rank)
                .ok_or(RagError::ArithmeticOverflow)?;
            let contribution = RRF_NUMERATOR
                .checked_div(denominator)
                .ok_or(RagError::OutOfRange)?;
            let entry = scores.entry(key.clone()).or_insert(0);
            *entry = entry
                .checked_add(contribution)
                .ok_or(RagError::ArithmeticOverflow)?;
        }
    }
    let mut ranked: Vec<(K, u64)> = scores.into_iter().collect();
    ranked.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    ranked.truncate(keep);
    Ok(ranked
        .into_iter()
        .enumerate()
        .map(|(i, (key, score))| Fused {
            key,
            score,
            rank: i.saturating_add(1),
        })
        .collect())
}

/// Ein Kandidat für die Auswahl nach Fusion und Reranker (SPEC 9.4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candidate<K, D> {
    /// Schlüssel für den Gleichstand (`KOrd`, `VerOrd`, Chunk).
    pub key: K,
    /// Dokumentkennung für `max_chunks_per_doc`.
    pub doc: D,
    /// RRF-Rang, beginnend bei 1.
    pub rrf_rank: usize,
    /// Reranker-Wert auf Q32.32 gerundet. `None`, wenn kein Reranker lief.
    pub rerank: Option<Fx>,
}

/// Sortiert die Kandidaten und begrenzt sie.
///
/// Reihenfolge: Reranker-Wert absteigend (nur wenn alle Kandidaten einen Wert haben), dann
/// RRF-Rang aufsteigend, dann Schlüssel aufsteigend. Je Dokument bleiben höchstens
/// `max_per_doc` Treffer, insgesamt höchstens `keep`.
///
/// # Errors
/// `IncompleteRerankScores`, wenn nur ein Teil der Kandidaten einen Reranker-Wert hat.
/// `DuplicateCandidate`, wenn ein Schlüssel doppelt vorkommt.
pub fn select_top<K: Ord + Clone, D: Ord + Clone>(
    mut candidates: Vec<Candidate<K, D>>,
    max_per_doc: usize,
    keep: usize,
) -> Result<Vec<Candidate<K, D>>, RagError> {
    let scored = candidates.iter().filter(|c| c.rerank.is_some()).count();
    if scored != 0 && scored != candidates.len() {
        return Err(RagError::IncompleteRerankScores);
    }
    ensure_unique_keys(&candidates)?;
    candidates.sort_by(compare_for_selection);

    let mut per_doc: BTreeMap<D, usize> = BTreeMap::new();
    let mut out = Vec::new();
    for candidate in candidates {
        if out.len() >= keep {
            break;
        }
        let count = per_doc.entry(candidate.doc.clone()).or_insert(0);
        if *count < max_per_doc {
            *count = count.saturating_add(1);
            out.push(candidate);
        }
    }
    Ok(out)
}

fn ensure_unique_keys<K: Ord, D>(candidates: &[Candidate<K, D>]) -> Result<(), RagError> {
    let mut seen: BTreeSet<&K> = BTreeSet::new();
    for candidate in candidates {
        if !seen.insert(&candidate.key) {
            return Err(RagError::DuplicateCandidate);
        }
    }
    Ok(())
}

fn compare_for_selection<K: Ord, D>(a: &Candidate<K, D>, b: &Candidate<K, D>) -> Ordering {
    b.rerank
        .cmp(&a.rerank)
        .then_with(|| a.rrf_rank.cmp(&b.rrf_rank))
        .then_with(|| a.key.cmp(&b.key))
}

#[cfg(test)]
mod tests {
    use super::*;

    // SPEC: 9.4 (RRF, Ganzzahl). Handgerechnet: 2^32/61 = 70409299, 2^32/62 = 69273666.
    #[test]
    fn two_lists_with_swapped_order_tie_break_on_key() {
        let text = ["a", "b"];
        let vector = ["b", "a"];
        let fused = rrf_fuse(&[&text[..], &vector[..]], 60, 10).unwrap();
        assert_eq!(fused.len(), 2);
        assert_eq!(fused[0].key, "a");
        assert_eq!(fused[1].key, "b");
        assert_eq!(fused[0].score, 70_409_299 + 69_273_666);
        assert_eq!(fused[0].score, fused[1].score);
        assert_eq!(fused[0].rank, 1);
        assert_eq!(fused[1].rank, 2);
    }

    // SPEC: 9.4. Golden-Werte: 2^32/61 + 2^32/62 = 139682965, 2^32/63 + 2^32/61 = 138583383.
    #[test]
    fn golden_scores_for_three_keys() {
        let l1 = [1_u32, 2, 3];
        let l2 = [3_u32, 1];
        let fused = rrf_fuse(&[&l1[..], &l2[..]], 60, 10).unwrap();
        let by_key: Vec<(u32, u64)> = fused.iter().map(|f| (f.key, f.score)).collect();
        assert_eq!(
            by_key,
            vec![(1, 139_682_965), (3, 138_583_383), (2, 69_273_666)]
        );
    }

    #[test]
    fn rrf_k_zero_gives_full_numerator_at_rank_one() {
        let only = [7_u8];
        let fused = rrf_fuse(&[&only[..]], 0, 1).unwrap();
        assert_eq!(fused[0].score, RRF_NUMERATOR);
    }

    #[test]
    fn keep_truncates_after_sorting() {
        let l = [1_u8, 2, 3, 4];
        let fused = rrf_fuse(&[&l[..]], 60, 2).unwrap();
        let keys: Vec<u8> = fused.iter().map(|f| f.key).collect();
        assert_eq!(keys, vec![1, 2]);
    }

    #[test]
    fn duplicate_within_list_is_rejected() {
        let l = [1_u8, 2, 1];
        assert_eq!(
            rrf_fuse(&[&l[..]], 60, 10),
            Err(RagError::DuplicateCandidate)
        );
    }

    #[test]
    fn overflow_of_rank_sum_is_reported() {
        let l = [1_u8];
        assert_eq!(
            rrf_fuse(&[&l[..]], u64::MAX, 10),
            Err(RagError::ArithmeticOverflow)
        );
    }

    fn cand(key: u32, doc: u8, rrf_rank: usize, rerank: Option<i32>) -> Candidate<u32, u8> {
        Candidate {
            key,
            doc,
            rrf_rank,
            rerank: rerank.map(Fx::from_int),
        }
    }

    // SPEC: 9.4 (Gleichstand: Reranker absteigend, RRF-Rang aufsteigend, Schlüssel aufsteigend)
    #[test]
    fn reranker_order_then_rank_then_key() {
        let cands = vec![
            cand(10, 1, 3, Some(5)),
            cand(11, 2, 1, Some(5)),
            cand(9, 3, 2, Some(7)),
            cand(8, 4, 2, Some(5)),
        ];
        let out = select_top(cands, 3, 10).unwrap();
        let keys: Vec<u32> = out.iter().map(|c| c.key).collect();
        // 9 hat den höchsten Reranker-Wert (7). Bei Wert 5 entscheidet der RRF-Rang:
        // 11 (Rang 1) vor 8 (Rang 2) vor 10 (Rang 3).
        assert_eq!(keys, vec![9, 11, 8, 10]);
    }

    #[test]
    fn without_reranker_order_follows_rrf_rank() {
        let cands = vec![
            cand(1, 1, 2, None),
            cand(2, 2, 1, None),
            cand(3, 3, 3, None),
        ];
        let out = select_top(cands, 3, 2).unwrap();
        let keys: Vec<u32> = out.iter().map(|c| c.key).collect();
        assert_eq!(keys, vec![2, 1]);
    }

    #[test]
    fn max_chunks_per_doc_skips_surplus_chunks() {
        let cands = vec![
            cand(1, 7, 1, None),
            cand(2, 7, 2, None),
            cand(3, 7, 3, None),
            cand(4, 8, 4, None),
        ];
        let out = select_top(cands, 2, 10).unwrap();
        let keys: Vec<u32> = out.iter().map(|c| c.key).collect();
        assert_eq!(keys, vec![1, 2, 4]);
    }

    #[test]
    fn partial_rerank_scores_are_rejected() {
        let cands = vec![cand(1, 1, 1, Some(3)), cand(2, 2, 2, None)];
        assert_eq!(
            select_top(cands, 3, 10),
            Err(RagError::IncompleteRerankScores)
        );
    }

    #[test]
    fn duplicate_key_in_selection_is_rejected() {
        let cands = vec![cand(1, 1, 1, None), cand(1, 2, 2, None)];
        assert_eq!(select_top(cands, 3, 10), Err(RagError::DuplicateCandidate));
    }
}
