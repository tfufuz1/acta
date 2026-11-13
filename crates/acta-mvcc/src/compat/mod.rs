// ANCHOR[id=acta-mvcc-compat-root]
//! # Kompatibilitätsmodul für Acta-Workspace
//!
//! Re-exportiert Adapter und Konvertierungen für bestehende Workspace-Verbraucher.
//! Dieses Modul wird ausschließlich hinter dem Feature `workspace` kompiliert.

/// Brücke zur Konvertierung lokaler `MvccError`-Fehler in `acta_types::ActaError`.
pub mod error;

pub use acta_types::error as workspace_error;
pub use acta_types::error::ActaError;
pub use acta_types::error::Result;
