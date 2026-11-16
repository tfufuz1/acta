//! acta-gate: `ErasedView`, `GateWriter`, Roaring-Mengen, Epoche, `ErasureLayer`, `LayerAck`.
//!
//! Level L1 · Budget 1200 LOC · Spezifikation docs/SPEC.md Kap. 3.7, 5.3
//!
//! Der Crate führt keine E/A aus: Das Log wird über den Port `GateSink` geschrieben, der
//! Durchstich zur Platte liegt in `acta-fs`/`acta-node`. Zeit und Zufall werden nicht benötigt.
//! Herkunft der Verträge: `archive/acta-gate/MANIFEST.md` (Umbau auf Roaring und HMAC-Kette,
//! `parking_lot` und `serde` entfallen).
#![forbid(unsafe_code)]

pub mod error;
pub mod layer;
pub mod log;
pub mod reasons;
mod snapshot;
pub mod state;
pub mod view;
pub mod writer;

pub use error::GateError;
pub use layer::{ErasureLayer, LayerAck, LayerPlan, Residue};
pub use log::{GateKey, GateOp, HighWaterKind};
pub use reasons::{EraseReason, RestrictReason};
pub use state::{GateSnapshot, GateState, Lineage, LineageHit, TargetSet};
pub use view::{ErasedView, GateView};
pub use writer::{GateSink, GateWriter};
