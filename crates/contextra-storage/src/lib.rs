//! # Contextra Storage Engine
//!
//! Eingebetteter, synchroner, crash-sicherer Schlüssel-Wert-Kern nach SPEC.md.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod api;
pub mod blob;
pub mod catalog;
pub mod codec;
pub mod engine;
pub mod env;
pub mod erase;
pub mod keys;
pub mod redo;
pub mod run;
pub mod sync;

#[cfg(any(test, feature = "testing"))]
pub mod testing;
