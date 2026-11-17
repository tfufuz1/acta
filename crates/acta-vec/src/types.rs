//! Schlüssel, Einträge, Treffer und Kandidatenfilter für den Vektorindex.

use acta_base::ids::{KOrd, VerOrd};
use roaring::{RoaringBitmap, RoaringTreemap};

use crate::quantize::QuantizedVector;

/// Schlüssel eines Vektors. Die Ordnung entspricht dem Gleichstand aus SPEC 9.2/9.3:
/// zuerst K vor P, innerhalb davon nach Ordinal und Chunk aufsteigend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum VectorId {
    /// Vektor aus der Wissensebene K.
    Knowledge {
        /// K-Objekt.
        kord: KOrd,
        /// Chunk innerhalb des Objekts.
        chunk: u32,
    },
    /// Vektor aus der Personalebene P.
    Personal {
        /// Version des P-Dokuments.
        ver: VerOrd,
        /// Chunk innerhalb der Version.
        chunk: u32,
    },
}

impl VectorId {
    /// Schlüssel für einen K-Vektor.
    #[must_use]
    pub const fn knowledge(kord: KOrd, chunk: u32) -> Self {
        Self::Knowledge { kord, chunk }
    }

    /// Schlüssel für einen P-Vektor.
    #[must_use]
    pub const fn personal(ver: VerOrd, chunk: u32) -> Self {
        Self::Personal { ver, chunk }
    }
}

/// Eintrag des Index: Schlüssel und quantisierter Vektor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VectorEntry {
    key: VectorId,
    vector: QuantizedVector,
}

impl VectorEntry {
    /// Erzeugt einen Eintrag.
    #[must_use]
    pub const fn new(key: VectorId, vector: QuantizedVector) -> Self {
        Self { key, vector }
    }

    /// Schlüssel des Eintrags.
    #[must_use]
    pub const fn key(&self) -> VectorId {
        self.key
    }

    /// Quantisierter Vektor.
    #[must_use]
    pub const fn vector(&self) -> &QuantizedVector {
        &self.vector
    }

    /// Zerlegt den Eintrag in Schlüssel und Vektor.
    #[must_use]
    pub fn into_parts(self) -> (VectorId, QuantizedVector) {
        (self.key, self.vector)
    }
}

/// Treffer einer Vektorsuche mit exakter Ähnlichkeit als Q32.32.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VectorHit {
    /// Schlüssel des Treffers.
    pub key: VectorId,
    /// Ähnlichkeit in Q32.32.
    pub score: acta_base::fx::Fx,
}

/// Zugriffsmenge für die Kandidatenauswahl (SPEC 9.3, 9.4).
///
/// K-Vektoren sind zulässig, wenn ihr `KOrd` in `kords` liegt, P-Vektoren, wenn ihre
/// `VerOrd` in `vers` liegt. Die Menge stammt aus der Sichtklasse des Nutzers.
#[derive(Debug, Clone, PartialEq)]
pub struct CandidateFilter {
    vers: RoaringBitmap,
    kords: RoaringTreemap,
}

impl CandidateFilter {
    /// Erzeugt einen Filter aus erlaubten Versionen und K-Objekten.
    #[must_use]
    pub const fn new(vers: RoaringBitmap, kords: RoaringTreemap) -> Self {
        Self { vers, kords }
    }

    /// Ob ein Schlüssel zugelassen ist.
    #[must_use]
    pub fn allows(&self, key: VectorId) -> bool {
        match key {
            VectorId::Knowledge { kord, .. } => self.kords.contains(kord.0),
            VectorId::Personal { ver, .. } => self.vers.contains(ver.0),
        }
    }
}
