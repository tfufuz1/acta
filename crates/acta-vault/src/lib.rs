//! acta-vault: Mandantentresore, KeyAnchor, FileAnchor, Wurzelrotation, Sicherungsexport.
//!
//! Level L2 · Budget 5500 LOC · Spezifikation docs/SPEC.md Kap. 4.3, 4.4
//!
//! Stand Arbeitspaket 2: Eintragslayout, Rahmen und Positions-AAD (Kap. 4.3). Die Versiegelung
//! unter dem Tresorschlüssel, Persistenz, Rotation und Export fehlen noch (siehe `CLAUDE.md`).
#![forbid(unsafe_code)]

pub mod entry;
pub mod error;
