//! Top-k-Auswahl mit fester Rangfolge (SPEC 9.2, 9.3).
//!
//! Rangfolge: Score absteigend, bei Gleichstand Schlüssel aufsteigend. Die Ordnung ist total,
//! daher hängt das Ergebnis nicht von der Reihenfolge der Segmente ab.

use std::cmp::Ordering;
use std::collections::BinaryHeap;

use crate::types::VectorHit;

/// Treffer in Rangordnung. Größer bedeutet schlechter, die Heap-Wurzel ist der schwächste Treffer.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Ranked(VectorHit);

impl Ord for Ranked {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .0
            .score
            .cmp(&self.0.score)
            .then_with(|| self.0.key.cmp(&other.0.key))
    }
}

impl PartialOrd for Ranked {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Begrenzte Auswahl der `k` besten Treffer.
pub(crate) struct TopK {
    k: usize,
    heap: BinaryHeap<Ranked>,
}

impl TopK {
    /// Leere Auswahl für höchstens `k` Treffer.
    pub(crate) fn new(k: usize) -> Self {
        Self {
            k,
            heap: BinaryHeap::with_capacity(k.min(4096)),
        }
    }

    /// Nimmt einen Treffer auf, falls er zu den `k` besten gehört.
    pub(crate) fn push(&mut self, hit: VectorHit) {
        if self.k == 0 {
            return;
        }
        let candidate = Ranked(hit);
        if self.heap.len() < self.k {
            self.heap.push(candidate);
            return;
        }
        let better = self.heap.peek().is_some_and(|worst| candidate < *worst);
        if better {
            self.heap.pop();
            self.heap.push(candidate);
        }
    }

    /// Ergebnis in Rangfolge, bester Treffer zuerst.
    pub(crate) fn into_sorted(self) -> Vec<VectorHit> {
        self.heap
            .into_sorted_vec()
            .into_iter()
            .map(|ranked| ranked.0)
            .collect()
    }
}
