//! acta-base: Newtypes, Fehlerklassen, Ports, Festkomma.
//!
//! Level L0 · Budget 2500 LOC · Spezifikation docs/SPEC.md Kap. 2.2, 2.4, 3.5, 3.7, 4.6, 6.2, 15.2.
//!
//! Module: `canon`, `class`, `error`, `fx`, `ids`, `limits`, `ports`. Wiederausgefuehrt sind nur
//! `DataClass`, `BaseError`, `ClassifiedError`, `ErrorClass`, `Clock`, `CryptoRng` und
//! `EntropyError`; alle weiteren Typen (z. B. `ids::ChainId`, `error::PoisonScope`) liegen in
//! ihrem Modul. Der Crate enthaelt keine Schluessel, keine Platten-E/A, keine Sperre.
#![forbid(unsafe_code)]

pub mod canon;
pub mod class;
pub mod error;
pub mod fx;
pub mod ids;
pub mod limits;
pub mod ports;

pub use class::DataClass;
pub use error::{BaseError, ClassifiedError, ErrorClass};
pub use ports::{Clock, CryptoRng, EntropyError};
