//! # Bloom-Filter
//!
//! Doppel-Hashing über XXH64 nach SPEC §9.5 und Anhang A.6.
//! Layout: `k u8 | nbits u64 | bits[nbits / 8]`

use super::xxh64::xxh64;

const SEED2: u64 = 0x9E3779B97F4A7C15;

/// Ein dynamischer Bloom-Filter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BloomFilter {
    k: u8,
    nbits: u64,
    bits: Vec<u8>,
}

impl BloomFilter {
    /// Erstellt einen neuen Bloom-Filter für `key_count` Schlüssel mit `bits_per_key`.
    pub fn new(key_count: usize, bits_per_key: usize) -> Self {
        let bits_per_key = if bits_per_key == 0 { 10 } else { bits_per_key };
        let count_u64 = key_count as u64;
        let bpk_u64 = bits_per_key as u64;
        let raw_bits = count_u64.saturating_mul(bpk_u64);
        let min_bits = std::cmp::max(64, raw_bits);
        // Auf Vielfaches von 64 aufrunden
        let nbits = min_bits.div_ceil(64) * 64;
        let byte_len = (nbits / 8) as usize;

        Self {
            k: 7,
            nbits,
            bits: vec![0u8; byte_len],
        }
    }

    /// Erstellt einen Filter mit expliziter Bitanzahl (z. B. für Tests oder Einlesen).
    pub fn with_explicit_bits(k: u8, nbits: u64, bits: Vec<u8>) -> Self {
        Self { k, nbits, bits }
    }

    /// Fügt einen Nutzerschlüssel in den Bloom-Filter ein.
    pub fn insert(&mut self, key: &[u8]) {
        let h1 = xxh64(key, 0);
        let h2 = xxh64(key, SEED2) | 1;

        for i in 0..self.k {
            let offset = (i as u64).wrapping_mul(h2);
            let pos = h1.wrapping_add(offset) % self.nbits;
            let byte_idx = (pos / 8) as usize;
            let bit_mask = 1u8 << (pos % 8);
            if let Some(b) = self.bits.get_mut(byte_idx) {
                *b |= bit_mask;
            }
        }
    }

    /// Prüft, ob ein Schlüssel möglicherweise im Filter enthalten ist.
    pub fn may_contain(&self, key: &[u8]) -> bool {
        if self.bits.is_empty() || self.nbits == 0 {
            return false;
        }

        let h1 = xxh64(key, 0);
        let h2 = xxh64(key, SEED2) | 1;

        for i in 0..self.k {
            let offset = (i as u64).wrapping_mul(h2);
            let pos = h1.wrapping_add(offset) % self.nbits;
            let byte_idx = (pos / 8) as usize;
            let bit_mask = 1u8 << (pos % 8);
            match self.bits.get(byte_idx) {
                Some(&b) if (b & bit_mask) != 0 => {}
                _ => return false,
            }
        }

        true
    }

    /// Liefert k (Anzahl Hashfunktionen).
    pub fn k(&self) -> u8 {
        self.k
    }

    /// Liefert nbits (Anzahl Bits im Filter).
    pub fn nbits(&self) -> u64 {
        self.nbits
    }

    /// Liefert die Rohbytes des Bit-Arrays.
    pub fn as_bytes(&self) -> &[u8] {
        &self.bits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_golden_a6_bloom() {
        let mut filter = BloomFilter::with_explicit_bits(7, 64, vec![0u8; 8]);
        filter.insert(b"k");

        // Hashprüfungen laut Golden A.6
        let h1 = xxh64(b"k", 0);
        assert_eq!(h1, 0xC3D31922C50B1B63);
        let h2 = xxh64(b"k", SEED2) | 1;
        assert_eq!(h2, 0x6CBC77349206F859 | 1);

        // Positionen berechnen und prüfen: 35, 60, 21, 46, 7, 32, 57
        let expected_positions = [35, 60, 21, 46, 7, 32, 57];
        for (i, &expected) in expected_positions.iter().enumerate() {
            let pos = h1.wrapping_add((i as u64).wrapping_mul(h2)) % 64;
            assert_eq!(pos, expected);
        }

        assert!(filter.may_contain(b"k"));
        assert!(!filter.may_contain(b"nonexistent_key"));
    }
}
