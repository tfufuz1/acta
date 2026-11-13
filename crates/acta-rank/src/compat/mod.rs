//! # Kompatibilitätsbrücke (`compat`)
//!
//! **Zweck:** Enthält alle Konvertierungen und Schnittstellenanbindungen an die
//! Workspace-Crates `acta_types` und `acta_ports`.
//!
//! Dieses Modul wird ausschließlich über das Feature `workspace` aktiviert.
//! Der Kern von `acta-rank` ist vollständig unabhängig von diesen Crates.

pub mod clock;
pub mod error;
pub mod types;

pub use clock::{IsotonicPortsClockExt, PortsClockAdapter};
pub use types::{
    apply_temporal_validity_filter_with_ports_clock,
    signal_contribution_synthesized_from_types,
    weights_to_signal_factors_types_compat,
};
