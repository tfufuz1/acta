//! # Modul: mod.rs
//!
//! **Zweck:** Interne Implementierungsmodule der `Acta`-Hauptklasse.
//!
//! **Architektur:** Gehört zur Crate `acta-engine` (Ring 2).
//! Eingehaltene Invarianten: Fail-Closed bei Fehlern, ACID/SSI-Transaktionsgarantien.

mod collections;
mod crud;
mod diagnostics;
pub mod erasure;
mod lifecycle;
mod wiring;
