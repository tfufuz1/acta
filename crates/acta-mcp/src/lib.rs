#![deny(dead_code)]
#![forbid(unsafe_code)]
//! stdio JSON-RPC 2.0 Protokollschicht des Acta-MCP (ADR-010).
//!
//! Verbleibender Umfang: Protokoll-Typen, begrenzter stdio-I/O, Embedding-Provider-Auflösung
//! und Eingabevalidierung.
//!
//! Entfernt: Server-Dispatch, Tool-Implementierungen, Sandbox, Routing sowie Egress- und
//! Prompt-Guard-Schicht. Diese benötigten die Crates `acta`, `acta-privacy`, `acta-prompt-guard`,
//! `acta-router` und die Infer-Backends, die nicht im Repository existieren.

// =========================================================================
// SECTION: Modul-Deklaration
// =========================================================================
// FILE-CONTEXT
// ZWECK:       stdio JSON-RPC 2.0 Protokollschicht (kein HTTP, ADR-010)
// INVARIANTEN: Transport ausschließlich stdin/stdout; bounded RPC message size
// SIEHE AUCH:  ADR-010

pub mod config;
pub mod io;
pub mod protocol;
pub mod validation;

pub use config::*;
pub use io::{read_line_bounded, MAX_RPC_BYTES, MAX_SEARCH_QUERY_BYTES};
pub use validation::{is_write_allowed_by_env, validate_collection_name};

// !SECTION: Modul-Deklaration
