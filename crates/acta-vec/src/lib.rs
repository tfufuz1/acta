//! acta-vec: Segmentierter exakter i8-Vektorindex mit Lineage.
//!
//! Level L3 · Budget 4000 LOC · Spezifikation docs/SPEC.md Kap. 9.3
//!
//! Vektoren werden vor dem Speichern nach `i8` quantisiert, `norm2` als `u32` gespeichert.
//! Die Suche ist exakt und ganzzahlig: Ergebnisse hängen nur von den gespeicherten Bytes, dem
//! Abfragevektor und der Zugriffsmenge ab (Z7, Z16). Kein `unsafe` (G-UNSAFE).
#![forbid(unsafe_code)]

pub mod ann;
pub mod distance;
pub mod error;
pub mod index;
pub mod lineage;
pub mod quantize;
pub mod report;
pub mod types;

mod topk;
