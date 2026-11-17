//! acta-erasure: Löschkern nach SPEC Kap. 8.
//!
//! Level L4 · Budget 9000 LOC · Spezifikation docs/spec/08_loeschkern.md
//!
//! Dieser Crate liegt in Arbeitspaket 1 vor: die reine Logik ohne E/A.
//!
//! - [`saga`]: Zustandsautomat der Löschsaga S0 bis S9 (8.3, Anhang B.1)
//! - [`read`]: Lesesperre und `restricted_read` (8.1, Z1, Z16)
//! - [`measure`]: Messung mit Positivkontrolle (8.5, Z11)
//! - [`classes`]: Erfüllungsklassen G, B, K, M (8.7)
//! - [`multi`]: Mehrbetroffene Datensätze (8.9, Z3)
//!
//! Noch nicht umgesetzt (Abschnitt Status in `CLAUDE.md`): Speicherung und Ausführung der
//! Schritte (Tresor, Segmente, Journal, Ledger), Restore-Schutz (8.8) und Ausgangskorb (8.6).
//! Die zugehörigen Crates `acta-vault`, `acta-seg`, `acta-ledger` und `acta-pstore` sind Gerüste.
#![forbid(unsafe_code)]

pub mod classes;
pub mod error;
pub mod measure;
pub mod multi;
pub mod read;
pub mod saga;
