//! # acta-engine (Deprecated Façade)
//!
//! Dieses Crate wurde gemäß SPEC(11).md S-63/S-64 und ADR-0122 aufgelöst.
//! Alle Funktionalitäten sind nach `acta-db` migriert.
//
// INVARIANTE: INV-2: Name bleibt als #[deprecated]-Fassade erhalten.

#![deprecated(
    since = "0.2.0",
    note = "acta-engine has been merged into acta-db (ADR-0122)"
)]
#![forbid(unsafe_code)]

#[allow(deprecated)]
pub use acta_db::*;
