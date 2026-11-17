//! acta-rag: Abrufpipeline (Teil dieser Stufe).
//!
//! Level L4 · Budget 7000 LOC · Spezifikation docs/SPEC.md Kap. 6.3, 9.2, 9.4 bis 9.7, Anhang A.
//!
//! Umgesetzt sind die reinen Rechenkerne aus dem Archiv `archive/acta-rag/rank`, neu nach SPEC
//! gebaut:
//! - `fusion`: Reciprocal Rank Fusion in Ganzzahl-Arithmetik und Auswahl nach dem Reranker,
//! - `validity`: Gültigkeitszeit `[valid_from, valid_to)` mit Filter `valid_at`,
//! - `metrics`: Recall@k, MRR, nDCG und Latenzperzentile in Festkomma Q32.32,
//! - `quality`: Regressionsgrenzen nach Anhang A und der Status `QualityDrift`.
//!
//! Nicht umgesetzt, weil die Spezifikation dafür keinen Eintrag (SPEC 0.4 Regel 3) oder noch keine
//! Vorgänger-Crates hat: Platt-, Isotonic- und Conformal-Kalibrierung, Drift über
//! Score-Verteilungen, DiBud, Erklärungsbericht, Hybridsuche, Kontextaufbau, Belegprüfung und
//! `ErasureLayer`.
//!
//! Kein Gleitkomma in Ranking-Pfaden (Z7). Kein `unsafe`, kein Zugriff auf Zeit oder Zufall.

#![forbid(unsafe_code)]

pub mod error;
pub mod fusion;
pub mod metrics;
pub mod quality;
pub mod validity;
