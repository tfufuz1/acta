//! # CRC32C (Castagnoli)
//!
//! Portabler, sicherer Prüfsummenalgorithmus ohne externe Abhängigkeiten (SD-64).
//! Polynom: 0x82F63B78 (reflektiert).
//!
//! Verifiziert gegen Referenzvektoren aus SPEC Anhang A.1:
//! - CRC32C("123456789", 0) = 0xE3069283
//! - CRC32C("", 0) = 0x00000000
//! - CRC32C("a", 0) = 0xC1D04330
//! - Fortsetzung: crc32c(crc32c(0, "1234"), "56789") = 0xE3069283

#![allow(clippy::indexing_slicing)]

const TABLE: [u32; 256] = {
    let mut table = [0u32; 256];
    let poly = 0x82F63B78u32;
    let mut i = 0usize;
    while i < 256 {
        let mut crc = i as u32;
        let mut j = 0;
        while j < 8 {
            if (crc & 1) != 0 {
                crc = (crc >> 1) ^ poly;
            } else {
                crc >>= 1;
            }
            j += 1;
        }
        table[i] = crc;
        i += 1;
    }
    table
};

/// Berechnet oder setzt eine CRC32C-Prüfsumme fort.
///
/// `seed`: Vorheriger CRC-Wert (bzw. 0 für Initialberechnung).
pub fn crc32c(seed: u32, data: &[u8]) -> u32 {
    let mut crc = !seed;
    for &byte in data {
        let index = ((crc as u8) ^ byte) as usize;
        let entry = match TABLE.get(index) {
            Some(&val) => val,
            None => 0,
        };
        crc = (crc >> 8) ^ entry;
    }
    !crc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_golden_a1_crc32c() {
        assert_eq!(crc32c(0, b"123456789"), 0xE3069283);
        assert_eq!(crc32c(0, b""), 0x00000000);
        assert_eq!(crc32c(0, b"a"), 0xC1D04330);
        let part1 = crc32c(0, b"1234");
        let part2 = crc32c(part1, b"56789");
        assert_eq!(part2, 0xE3069283);
    }
}
