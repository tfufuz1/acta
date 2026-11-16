//! `OsCryptoRng`: Implementierung des Ports `CryptoRng` über das Betriebssystem (SPEC 4.6).
//!
//! Fehlbar: ein Fehler der Quelle wird als `EntropyError` gemeldet, nie durch einen Ersatzwert
//! ersetzt (Z10). Die Quelle ist `getrandom` (Whitelist, SPEC 3.8).

use acta_base::{CryptoRng, EntropyError};

/// Zufall des Betriebssystems.
#[derive(Debug, Default, Clone, Copy)]
pub struct OsCryptoRng;

impl CryptoRng for OsCryptoRng {
    fn try_fill(&self, dst: &mut [u8]) -> Result<(), EntropyError> {
        getrandom::fill(dst).map_err(|_| EntropyError)
    }
}
