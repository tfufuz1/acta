//! acta-sys: einzige Stelle mit `unsafe` (G-UNSAFE; jeder Block mit `// SAFETY:`).
//!
//! Level L1 · Budget 1600 LOC · Spezifikation docs/SPEC.md Kap. 4.6, 13.1, 13.2.
//!
//! Umgesetzt: `SecureBuf` (mlock, MADV_DONTDUMP, feste Kapazität, Nullung), Prozesshärtung
//! (`RLIMIT_CORE`, `PR_SET_DUMPABLE`), Ports `SystemClock` und `OsCryptoRng`.
//! Hostprüfungen als reine Auswertung (`host`). Offen: Dateilesen der Prüfwerte im Knoten, Sandbox (Landlock, seccomp; SPEC 13.2 gehört in den Worker),
//! SIMD-Kerne (Archiv `simd/`, Abschnitt Überführung).
//!
//! Extraktionsarchiv: `archive/acta-sys/MANIFEST.md`.
#![deny(unsafe_op_in_unsafe_fn)]

pub mod clock;
pub mod error;
pub mod harden;
pub mod host;
pub mod rng;
pub mod secure_buf;

pub use clock::SystemClock;
pub use error::SysError;
pub use harden::{harden_process, memlock_soft_limit};
pub use host::{check_core_pattern, check_memlock, check_ptrace_scope, check_swap};
pub use rng::OsCryptoRng;
pub use secure_buf::SecureBuf;
