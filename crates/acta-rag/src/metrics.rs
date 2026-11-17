//! Bewertungskennzahlen des Bewertungswerkzeugs (SPEC 9.7) in Festkomma Q32.32.
//!
//! Recall@k, MRR und nDCG sind Verhältnisse in `[0, 1]`. Eine Kennzahl ohne Referenzdaten ist
//! `NotMeasurable` und wird nie als Erfolg (1,0 oder 0,0) gemeldet.

use std::collections::{BTreeMap, BTreeSet};

use acta_base::fx::{fx_ln, Fx};

use crate::error::RagError;

/// Verhältnis `numerator / denominator` in Q32.32.
///
/// # Errors
/// `NotMeasurable` bei `denominator == 0`; `ArithmeticOverflow` bei Werten über `i64::MAX`.
pub fn ratio(numerator: u64, denominator: u64) -> Result<Fx, RagError> {
    let n = i64::try_from(numerator).map_err(|_| RagError::ArithmeticOverflow)?;
    let d = i64::try_from(denominator).map_err(|_| RagError::ArithmeticOverflow)?;
    Fx::from_ratio(n, d).ok_or(RagError::NotMeasurable)
}

/// Recall@k: Anteil der relevanten Schlüssel unter den ersten `k` Treffern.
///
/// # Errors
/// `NotMeasurable`, wenn keine relevanten Schlüssel angegeben sind.
pub fn recall_at_k<K: Ord>(ranked: &[K], relevant: &[K], k: usize) -> Result<Fx, RagError> {
    let relevant_set: BTreeSet<&K> = relevant.iter().collect();
    if relevant_set.is_empty() {
        return Err(RagError::NotMeasurable);
    }
    let found: BTreeSet<&K> = ranked
        .iter()
        .take(k)
        .filter(|key| relevant_set.contains(key))
        .collect();
    ratio(len_u64(found.len())?, len_u64(relevant_set.len())?)
}

/// Reziproker Rang des ersten relevanten Treffers (`1 / rang`). `0`, wenn keiner vorkommt.
///
/// # Errors
/// `NotMeasurable`, wenn keine relevanten Schlüssel angegeben sind.
pub fn reciprocal_rank<K: Ord>(ranked: &[K], relevant: &[K]) -> Result<Fx, RagError> {
    let relevant_set: BTreeSet<&K> = relevant.iter().collect();
    if relevant_set.is_empty() {
        return Err(RagError::NotMeasurable);
    }
    match ranked.iter().position(|key| relevant_set.contains(key)) {
        None => Ok(Fx::ZERO),
        Some(index) => {
            let rank = len_u64(index)?
                .checked_add(1)
                .ok_or(RagError::ArithmeticOverflow)?;
            ratio(1, rank)
        }
    }
}

/// Arithmetisches Mittel der Einzelwerte, etwa der reziproken Ränge zur MRR.
///
/// # Errors
/// `NotMeasurable` bei leerer Eingabe.
pub fn mean(values: &[Fx]) -> Result<Fx, RagError> {
    if values.is_empty() {
        return Err(RagError::NotMeasurable);
    }
    let sum = values.iter().fold(Fx::ZERO, |acc, v| acc.add(*v));
    let count = i32::try_from(values.len()).map_err(|_| RagError::OutOfRange)?;
    sum.div(Fx::from_int(count)).ok_or(RagError::NotMeasurable)
}

/// nDCG@k mit ganzzahligen Gewinnen je relevantem Schlüssel.
///
/// Der Diskont ist `1 / log2(i + 1) = ln 2 / ln(i + 1)` für die 1-basierte Position `i`, gerechnet
/// über `fx_ln`. Doppelte Schlüssel in `ranked` zählen nur beim ersten Auftreten.
///
/// # Errors
/// `NotMeasurable`, wenn kein Schlüssel einen Gewinn größer null hat. `OutOfRange`, wenn ein
/// Gewinn nicht in `i32` passt.
pub fn ndcg_at_k<K: Ord>(ranked: &[K], gains: &BTreeMap<K, u32>, k: usize) -> Result<Fx, RagError> {
    let mut seen: BTreeSet<&K> = BTreeSet::new();
    let ranked_gains = ranked
        .iter()
        .filter(|key| seen.insert(*key))
        .map(|key| gains.get(key).copied().unwrap_or(0));
    let dcg = discounted_sum(ranked_gains, k)?;

    let mut ideal: Vec<u32> = gains.values().copied().filter(|g| *g > 0).collect();
    ideal.sort_unstable_by(|a, b| b.cmp(a));
    let idcg = discounted_sum(ideal.into_iter(), k)?;
    if idcg == Fx::ZERO {
        return Err(RagError::NotMeasurable);
    }
    dcg.div(idcg).ok_or(RagError::NotMeasurable)
}

fn discounted_sum<I: Iterator<Item = u32>>(gains: I, k: usize) -> Result<Fx, RagError> {
    let mut total = Fx::ZERO;
    for (position, gain) in gains.take(k).enumerate() {
        if gain == 0 {
            continue;
        }
        let rank = len_u64(position)?
            .checked_add(1)
            .ok_or(RagError::ArithmeticOverflow)?;
        let g = i32::try_from(gain).map_err(|_| RagError::OutOfRange)?;
        total = total.add(Fx::from_int(g).mul(discount(rank)?));
    }
    Ok(total)
}

/// `1 / log2(rank + 1)` für die 1-basierte Position `rank`.
fn discount(rank: u64) -> Result<Fx, RagError> {
    let arg = rank
        .checked_add(1)
        .and_then(|v| i64::try_from(v).ok())
        .ok_or(RagError::OutOfRange)?;
    let x = Fx::from_ratio(arg, 1).ok_or(RagError::OutOfRange)?;
    let ln = fx_ln(x).ok_or(RagError::OutOfRange)?;
    Fx::LN2.div(ln).ok_or(RagError::NotMeasurable)
}

/// Perzentil nach dem Nächster-Rang-Verfahren, ganzzahlig und ohne Interpolation.
///
/// `percent` liegt in `1..=100`. Die Werte werden dabei aufsteigend sortiert.
///
/// # Errors
/// `OutOfRange` bei `percent` außerhalb von `1..=100`, `NotMeasurable` bei leerer Eingabe.
pub fn percentile_nearest_rank(values: &mut [u64], percent: u8) -> Result<u64, RagError> {
    if !(1..=100).contains(&percent) {
        return Err(RagError::OutOfRange);
    }
    if values.is_empty() {
        return Err(RagError::NotMeasurable);
    }
    values.sort_unstable();
    let n = len_u64(values.len())?;
    let rank = u64::from(percent)
        .checked_mul(n)
        .and_then(|v| v.checked_add(99))
        .map(|v| v / 100)
        .ok_or(RagError::ArithmeticOverflow)?;
    let index = usize::try_from(rank.saturating_sub(1)).map_err(|_| RagError::OutOfRange)?;
    values.get(index).copied().ok_or(RagError::NotMeasurable)
}

fn len_u64(n: usize) -> Result<u64, RagError> {
    u64::try_from(n).map_err(|_| RagError::ArithmeticOverflow)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Q32.32 als f64, nur zur Prüfung gegen Referenzwerte in Tests.
    fn to_f64(x: Fx) -> f64 {
        x.to_bits() as f64 / 4_294_967_296.0
    }

    // SPEC: 9.7 (Recall@k)
    #[test]
    fn recall_counts_distinct_relevant_hits_in_top_k() {
        let ranked = [1_u32, 2, 3, 4];
        assert_eq!(
            recall_at_k(&ranked, &[2, 9], 3),
            Ok(Fx::from_ratio(1, 2).unwrap())
        );
        assert_eq!(recall_at_k(&ranked, &[2, 3], 10), Ok(Fx::ONE));
        assert_eq!(recall_at_k(&ranked, &[9], 10), Ok(Fx::ZERO));
    }

    #[test]
    fn recall_without_relevant_items_is_not_measurable() {
        assert_eq!(recall_at_k(&[1_u32], &[], 10), Err(RagError::NotMeasurable));
    }

    // SPEC: 9.7 (MRR)
    #[test]
    fn reciprocal_rank_and_mean() {
        let r1 = reciprocal_rank(&[5_u32, 6, 7], &[7]).unwrap();
        assert_eq!(r1, Fx::from_ratio(1, 3).unwrap());
        assert_eq!(reciprocal_rank(&[5_u32, 6], &[9]).unwrap(), Fx::ZERO);
        let r2 = reciprocal_rank(&[1_u32, 2], &[2]).unwrap();
        assert_eq!(r2, Fx::from_ratio(1, 2).unwrap());
        assert_eq!(mean(&[Fx::ONE, r2]), Ok(Fx::from_ratio(3, 4).unwrap()));
    }

    #[test]
    fn mean_of_nothing_is_not_measurable() {
        assert_eq!(mean(&[]), Err(RagError::NotMeasurable));
    }

    // SPEC: 9.7 (nDCG). Ideale Reihenfolge ergibt exakt 1.
    #[test]
    fn ndcg_is_exactly_one_for_ideal_order() {
        let mut gains = BTreeMap::new();
        gains.insert(1_u32, 3_u32);
        gains.insert(2, 1);
        gains.insert(3, 2);
        assert_eq!(ndcg_at_k(&[1_u32, 3, 2], &gains, 3), Ok(Fx::ONE));
    }

    // SPEC: 9.7 (nDCG). Gewinne a=3, b=1; Ranking [b, a].
    // DCG = 1/log2(2) + 3/log2(3); IDCG = 3/log2(2) + 1/log2(3) (Referenz in f64, nur Test).
    #[test]
    fn ndcg_matches_reference_value_for_swapped_pair() {
        let mut gains = BTreeMap::new();
        gains.insert("a", 3_u32);
        gains.insert("b", 1);
        let got = ndcg_at_k(&["b", "a"], &gains, 2).unwrap();
        let dcg = 1.0 + 3.0 / 3.0_f64.log2();
        let idcg = 3.0 + 1.0 / 3.0_f64.log2();
        let want = dcg / idcg;
        assert!((to_f64(got) - want).abs() < 1e-6);
    }

    #[test]
    fn ndcg_without_gains_is_not_measurable() {
        let gains: BTreeMap<u32, u32> = BTreeMap::new();
        assert_eq!(ndcg_at_k(&[1_u32], &gains, 5), Err(RagError::NotMeasurable));
    }

    // SPEC: 9.7 (Latenzperzentile)
    #[test]
    fn nearest_rank_percentiles() {
        let mut v: Vec<u64> = (1..=10).rev().collect();
        assert_eq!(percentile_nearest_rank(&mut v, 50), Ok(5));
        assert_eq!(percentile_nearest_rank(&mut v, 95), Ok(10));
        assert_eq!(percentile_nearest_rank(&mut v, 10), Ok(1));
        assert_eq!(percentile_nearest_rank(&mut v, 99), Ok(10));
    }

    #[test]
    fn percentile_rejects_bad_input() {
        let mut v = vec![1_u64];
        assert_eq!(
            percentile_nearest_rank(&mut v, 0),
            Err(RagError::OutOfRange)
        );
        assert_eq!(
            percentile_nearest_rank(&mut v, 101),
            Err(RagError::OutOfRange)
        );
        assert_eq!(
            percentile_nearest_rank(&mut [], 50),
            Err(RagError::NotMeasurable)
        );
    }
}
