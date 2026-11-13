//! # WAL Modultests (Ring 1)
//!
//! Sammelmodul fuer interne WAL-Tests.

pub use super::*;
#[cfg(feature = "encryption-at-rest")]
use acta_crypto::crypto::KeyManager;
use acta_types::{ActaError, Result};

#[cfg(test)]
mod encode_tests;
#[cfg(test)]
mod flusher_tests;
#[cfg(feature = "encryption-at-rest")]
#[cfg(test)]
mod hmac_tests;
#[cfg(test)]
mod io_tests;
#[cfg(feature = "encryption-at-rest")]
#[cfg(test)]
mod replay_tests;
