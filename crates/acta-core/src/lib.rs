//! `Acta` Core — Types, traits, and error handling (Strangler Facade).
//!
//! # Architecture Role (Legacy Triebwerk — Layer 0 -> Ring 0-4 Migration)
//!
//! As part of the Ring-Modell Phase 1b migration, the functionality of `acta-core`
//! has been decomposed into modular Ring-0 crates:
//! - [`acta_types`]: Canonical domain models, IDs, budgets, filters, and error types.
//! - [`acta_ports`]: Abstract `dyn`-compatible subsystem interfaces and traits.
//! - [`acta_mvcc`]: Multi-version concurrency control, sequence log, and tx staging.
//! - [`acta_wire`]: Zero-copy FlatBuffers IPC and wire message adapters.
//!
//! This crate acts as a backward-compatible Strangler Facade, re-exporting all types
//! with deprecation markers to avoid breaking existing downstream callers.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

// Re-export Ring-0 modular crates
#[deprecated(
    since = "0.1.0",
    note = "Moved to acta_types as part of Ring-Modell Phase 1b"
)]
pub use acta_types::error;
#[deprecated(
    since = "0.1.0",
    note = "Moved to acta_types as part of Ring-Modell Phase 1b"
)]
pub use acta_types::error_dto;
#[deprecated(
    since = "0.1.0",
    note = "Moved to acta_types as part of Ring-Modell Phase 1b"
)]
pub use acta_types::model_fingerprint;
#[deprecated(
    since = "0.1.0",
    note = "Moved to acta_types as part of Ring-Modell Phase 1b"
)]
pub use acta_types::schema;
#[deprecated(
    since = "0.1.0",
    note = "Moved to acta_types as part of Ring-Modell Phase 1b"
)]
pub use acta_types::tombstone;
#[deprecated(
    since = "0.1.0",
    note = "Moved to acta_types as part of Ring-Modell Phase 1b"
)]
pub use acta_types::types;

/// Traits submodule re-exporting `acta_ports` for backward compatibility.
#[deprecated(
    since = "0.1.0",
    note = "Moved to acta_ports as part of Ring-Modell Phase 1b"
)]
pub mod traits {
    pub use acta_ports::*;
}

#[deprecated(
    since = "0.1.0",
    note = "Moved to acta_mvcc as part of Ring-Modell Phase 1b"
)]
pub use acta_mvcc::seq_log;
#[deprecated(
    since = "0.1.0",
    note = "Moved to acta_mvcc as part of Ring-Modell Phase 1b"
)]
pub use acta_mvcc::snapshot;
#[deprecated(
    since = "0.1.0",
    note = "Moved to acta_mvcc as part of Ring-Modell Phase 1b"
)]
pub use acta_mvcc::tx_buffer;

/// IPC module re-exporting `acta_wire` for backward compatibility.
#[deprecated(
    since = "0.1.0",
    note = "Moved to acta_wire as part of Ring-Modell Phase 1b"
)]
pub mod ipc {
    pub use acta_wire::*;
}

#[deprecated(
    since = "0.1.0",
    note = "Moved to acta_mvcc::seq_log as part of Ring-Modell Phase 1b"
)]
pub use acta_mvcc::seq_log::{SeqLogChange, SeqLogEntry, SequenceLog};

#[deprecated(
    since = "0.1.0",
    note = "Moved to acta_mvcc::snapshot as part of Ring-Modell Phase 1b"
)]
pub use acta_mvcc::snapshot::{SnapshotGuard, SnapshotRegistry};

#[deprecated(
    since = "0.1.0",
    note = "Moved to acta_mvcc::tx_buffer as part of Ring-Modell Phase 1b"
)]
pub use acta_mvcc::tx_buffer::{IndexOp, TxBuffer};

#[deprecated(
    since = "0.1.0",
    note = "Moved to acta_ports as part of Ring-Modell Phase 1b"
)]
pub use acta_ports::*;

#[deprecated(
    since = "0.1.0",
    note = "Moved to acta_types::error as part of Ring-Modell Phase 1b"
)]
pub use acta_types::error::{ActaError, Result};

#[deprecated(
    since = "0.1.0",
    note = "Moved to acta_types::error_dto as part of Ring-Modell Phase 1b"
)]
pub use acta_types::error_dto::ActaErrorDto;

#[deprecated(
    since = "0.1.0",
    note = "Moved to acta_types::model_fingerprint as part of Ring-Modell Phase 1b"
)]
pub use acta_types::model_fingerprint::ModelFingerprint;

#[deprecated(
    since = "0.1.0",
    note = "Moved to acta_types::schema as part of Ring-Modell Phase 1b"
)]
pub use acta_types::schema::{DocIdWidth, ManifestSchemaVersion};

#[deprecated(
    since = "0.1.0",
    note = "Moved to acta_types::tombstone as part of Ring-Modell Phase 1b"
)]
pub use acta_types::tombstone::{SeqBitTombstone, TombstoneSemanticsCheck};
