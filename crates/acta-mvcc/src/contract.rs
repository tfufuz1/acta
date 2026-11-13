// ANCHOR[acta-mvcc:CONTRACT-ROOT]
//! # Minimalverträge und Umgebungsabstraktionen für `acta-mvcc`
//!
//! LINK crates/acta-mvcc/src/lib.rs
//! LINK crates/acta-env/src/lib.rs
//!
//! Dieses Modul definiert die minimalen Umgebungs-Traits (`Clock`, `MetricsSink`),
//! von denen der autarke Kern abhängt. Ports gehören dem Konsumenten;
//! der Kern importiert keine höheren Schichten.

pub use acta_env::{Clock, MetricsSink, SystemClock};
