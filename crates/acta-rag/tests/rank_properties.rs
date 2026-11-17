//! Eigenschaftstests der Rangkerne (SPEC 9.2, 9.4, 9.7, Z7). Deterministisch über proptest.

use std::collections::{BTreeMap, BTreeSet};

use acta_base::fx::Fx;
use acta_rag::fusion::{rrf_fuse, select_top, Candidate, RRF_NUMERATOR};
use acta_rag::metrics::{ndcg_at_k, percentile_nearest_rank};
use acta_rag::validity::{retain_valid_at, ValidityWindow};
use proptest::prelude::*;

fn unique(values: Vec<u32>) -> Vec<u32> {
    let mut seen = BTreeSet::new();
    values.into_iter().filter(|v| seen.insert(*v)).collect()
}

proptest! {
    // Z7: Die RRF-Summe hängt nicht von der Reihenfolge der Listen ab.
    #[test]
    fn rrf_is_independent_of_list_order(
        a in prop::collection::vec(0_u32..40, 0..25),
        b in prop::collection::vec(0_u32..40, 0..25),
        k in 0_u64..200,
    ) {
        let a = unique(a);
        let b = unique(b);
        let forward = rrf_fuse(&[&a[..], &b[..]], k, 100).unwrap();
        let backward = rrf_fuse(&[&b[..], &a[..]], k, 100).unwrap();
        prop_assert_eq!(forward, backward);
    }

    // Die Fusion stimmt mit einer direkten Referenzsumme überein und ist korrekt sortiert.
    #[test]
    fn rrf_matches_reference_sum_and_order(
        a in prop::collection::vec(0_u32..40, 0..25),
        b in prop::collection::vec(0_u32..40, 0..25),
        k in 0_u64..200,
    ) {
        let a = unique(a);
        let b = unique(b);
        let mut reference: BTreeMap<u32, u64> = BTreeMap::new();
        for list in [&a, &b] {
            for (i, key) in list.iter().enumerate() {
                let contribution = RRF_NUMERATOR / (k + i as u64 + 1);
                *reference.entry(*key).or_insert(0) += contribution;
            }
        }
        let fused = rrf_fuse(&[&a[..], &b[..]], k, usize::MAX).unwrap();
        prop_assert_eq!(fused.len(), reference.len());
        for (position, f) in fused.iter().enumerate() {
            prop_assert_eq!(reference.get(&f.key).copied(), Some(f.score));
            prop_assert_eq!(f.rank, position + 1);
        }
        for pair in fused.windows(2) {
            let (x, y) = (&pair[0], &pair[1]);
            prop_assert!(x.score > y.score || (x.score == y.score && x.key < y.key));
        }
    }

    // SPEC 9.7: Die ideale Reihenfolge ergibt nDCG exakt 1.
    #[test]
    fn ideal_order_gives_ndcg_one(
        gains in prop::collection::btree_map(0_u32..60, 1_u32..1000, 1..20),
        k in 1_usize..25,
    ) {
        let mut ranked: Vec<(u32, u32)> = gains.iter().map(|(key, g)| (*key, *g)).collect();
        ranked.sort_by(|x, y| y.1.cmp(&x.1).then(x.0.cmp(&y.0)));
        let order: Vec<u32> = ranked.iter().map(|(key, _)| *key).collect();
        let value = ndcg_at_k(&order, &gains, k).unwrap();
        prop_assert_eq!(value, Fx::ONE);
    }

    // Perzentile sind monoton in p.
    #[test]
    fn percentile_is_monotone_in_percent(
        values in prop::collection::vec(any::<u64>(), 1..50),
        p in 1_u8..=99,
    ) {
        let mut a = values.clone();
        let mut b = values;
        let lo = percentile_nearest_rank(&mut a, p).unwrap();
        let hi = percentile_nearest_rank(&mut b, p + 1).unwrap();
        prop_assert!(lo <= hi);
    }

    // Auswahl: höchstens `keep` Treffer, höchstens `max_per_doc` je Dokument.
    #[test]
    fn selection_respects_caps(
        cands in prop::collection::vec((0_u32..500, 0_u8..6, 1_usize..30), 0..40),
        max_per_doc in 1_usize..4,
        keep in 0_usize..20,
    ) {
        let mut seen = BTreeSet::new();
        let list: Vec<Candidate<u32, u8>> = cands
            .into_iter()
            .filter(|(key, _, _)| seen.insert(*key))
            .map(|(key, doc, rank)| Candidate { key, doc, rrf_rank: rank, rerank: None })
            .collect();
        let out = select_top(list, max_per_doc, keep).unwrap();
        prop_assert!(out.len() <= keep);
        let mut per_doc: BTreeMap<u8, usize> = BTreeMap::new();
        for c in &out {
            *per_doc.entry(c.doc).or_insert(0) += 1;
        }
        for count in per_doc.values() {
            prop_assert!(*count <= max_per_doc);
        }
    }

    // Gültigkeit: Nach dem Filter bleibt genau das, was den Tag enthält, in Reihenfolge.
    #[test]
    fn validity_filter_keeps_exactly_containing_windows(
        spans in prop::collection::vec((any::<i32>(), any::<i32>()), 0..20),
        day in any::<i32>(),
    ) {
        let mut items: Vec<ValidityWindow> = spans
            .iter()
            .filter_map(|(a, b)| {
                let (lo, hi) = if a <= b { (*a, *b) } else { (*b, *a) };
                ValidityWindow::new(Some(lo), Some(hi)).ok()
            })
            .collect();
        let expected: Vec<ValidityWindow> =
            items.iter().copied().filter(|w| w.contains(day)).collect();
        retain_valid_at(&mut items, day, |w| *w);
        prop_assert_eq!(items, expected);
    }
}
