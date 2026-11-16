//! Fehlertyp von acta-gate (HR-4: ein Fehlertyp je Crate, kein `Other(String)`).
//!
//! `LayerError` aus SPEC 3.7 ist bewusst derselbe Typ: ein zweiter Fehlertyp
//! wäre nach HR-4 nicht zulässig.

use acta_base::{ClassifiedError, ErrorClass};

/// Fehler von acta-gate. Meldungen enthalten weder Schlüssel noch Kennungen (Z15).
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum GateError {
    /// Der Mandant ist vergiftet (SPEC 15.2); bis zum Neustart keine Schreibvorgänge.
    #[error("gate of tenant is poisoned")]
    Poisoned,
    /// Ein leerer Stapel wurde übergeben.
    #[error("empty operation batch")]
    EmptyBatch,
    /// Eine Herkunft ohne Ordinale ist nicht konstruierbar (Z13).
    #[error("lineage is empty")]
    EmptyLineage,
    /// Schreiben in das Gate-Log ist fehlgeschlagen; der Aufruf meldet keinen Erfolg.
    #[error("gate log write failed")]
    LogWrite,
    /// `fsync` des Gate-Logs ist fehlgeschlagen (Z4, SPEC 3.7 Fsync-Regel).
    #[error("gate log fsync failed")]
    LogSync,
    /// Die HMAC-Kette ist an dieser Stelle gebrochen oder die Folge hat eine Lücke.
    #[error("gate log chain broken at sequence {seq}")]
    ChainBroken {
        /// Sequenznummer des ersten fehlerhaften Rahmens.
        seq: u64,
    },
    /// Ein Rahmen ist strukturell ungültig.
    #[error("gate log frame malformed")]
    LogFormat,
    /// Terminaler Zustand `Erased`: keine Rücknahme und keine Einschränkung mehr.
    #[error("erase state is terminal")]
    EraseTerminal,
    /// `Release` auf eine Version, die nicht eingeschränkt ist.
    #[error("version is not restricted")]
    NotRestricted,
    /// Ein Zähler für Ordinale würde zurückgesetzt.
    #[error("high-water mark regression")]
    HighWaterRegression,
    /// Eine Erasure-Schicht konnte nicht planen, leeren oder messen.
    #[error("erasure layer {layer} failed")]
    Layer {
        /// Stabiler Name der Schicht (`ErasureLayer::name`).
        layer: &'static str,
    },
    /// Der HMAC-Schlüssel konnte nicht verwendet werden.
    #[error("gate mac unavailable")]
    Mac,
}

impl ClassifiedError for GateError {
    fn class(&self) -> ErrorClass {
        match self {
            Self::Poisoned
            | Self::LogWrite
            | Self::LogSync
            | Self::ChainBroken { .. }
            | Self::LogFormat => ErrorClass::Integrity,
            Self::EmptyBatch | Self::EmptyLineage => ErrorClass::Invalid,
            Self::EraseTerminal | Self::NotRestricted | Self::HighWaterRegression => {
                ErrorClass::Policy
            }
            Self::Layer { .. } => ErrorClass::Transient,
            Self::Mac => ErrorClass::Crypto,
        }
    }

    fn code(&self) -> &'static str {
        match self {
            Self::Poisoned => "gate.poisoned",
            Self::EmptyBatch => "gate.batch.empty",
            Self::EmptyLineage => "gate.lineage.empty",
            Self::LogWrite => "gate.log.write",
            Self::LogSync => "gate.log.sync",
            Self::ChainBroken { .. } => "gate.log.chain",
            Self::LogFormat => "gate.log.format",
            Self::EraseTerminal => "gate.erase.terminal",
            Self::NotRestricted => "gate.release.not_restricted",
            Self::HighWaterRegression => "gate.highwater.regression",
            Self::Layer { .. } => "gate.layer",
            Self::Mac => "gate.mac",
        }
    }
}
