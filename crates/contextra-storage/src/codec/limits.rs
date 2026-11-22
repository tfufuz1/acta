//! # System-Limits und Schwellenwerte
//!
//! Normative Obergrenzen aus SPEC §6.7.

/// Maximale Länge eines Nutzerschlüssels (4 KiB).
pub const MAX_KEY_BYTES: usize = 4096;

/// Maximale Headerlänge je Eintrag (255 B).
pub const MAX_HEADER_LEN: usize = 255;

/// Maximaler Wertkörper standardmäßig (1 MiB).
pub const MAX_VALUE_BYTES: usize = 1024 * 1024;

/// Maximale Größe eines Changes (32 MiB).
pub const MAX_CHANGE_BYTES: usize = 32 * 1024 * 1024;

/// Maximale Anzahl von Operationen in einem Change.
pub const MAX_OPS_PER_CHANGE: usize = 100_000;

/// Maximale Größe eines Redo-Frames (64 MiB).
pub const MAX_FRAME_BYTES: usize = 64 * 1024 * 1024;

/// Maximale Anzahl an Gewichten je Eintrag.
pub const MAX_WEIGHTS: usize = 4;

/// Maximale Anzahl an Spaces (64).
pub const MAX_SPACES: usize = 64;

/// Maximale Anzahl registrierter Consumer (64).
pub const MAX_CONSUMERS: usize = 64;

/// Maximale Anzahl aktiver Holds je Instanz (10 000).
pub const MAX_HOLDS: usize = 10_000;

/// Maximale Anzahl dauerhafter Holds (1 000).
pub const MAX_DURABLE_HOLDS: usize = 1_000;

/// Maximale Länge eines Spacenamens in Bytes.
pub const MAX_SPACE_NAME_LEN: usize = 64;

/// Maximale Stamp-Grenze (< 2^56, INV-STAMP-2).
pub const MAX_STAMP: u64 = (1 << 56) - 1;
