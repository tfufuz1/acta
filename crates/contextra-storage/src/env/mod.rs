//! # Env Modul
//!
//! Produktive Umgebungskomponenten: Dateisystem, Uhrzeit, Entropie und atomare Schreiboperationen.

pub mod atomic;
pub mod clock;
pub mod entropy;
pub mod real_fs;

pub use atomic::write_atomic;
pub use clock::RealClock;
pub use entropy::RealEntropy;
pub use real_fs::RealFs;
