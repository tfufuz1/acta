# FINDINGS — acta-store Modul A (WAL)

Beobachtungen und Defekte außerhalb des zugewiesenen Eigentums von Agent A:

## Finding #1: Ungültiges `workspace`-Feature in `crates/acta-store/Cargo.toml`
- **Datei:** `crates/acta-store/Cargo.toml:17`
- **Beobachtung:**
  ```toml
  [features]
  workspace = ["dep:acta-types", "dep:acta-ports"]
  ```
  `acta-types` und `acta-ports` sind unter `[dependencies]` nicht mit `optional = true` deklariert.
  Cargo meldet Fehler:
  `feature 'workspace' includes 'dep:acta-types', but 'acta-types' is not an optional dependency`
- **Reproduktion:** `cargo check -p acta-store`
- **Vorschlag (Übergabe an Agent G):**
  In `crates/acta-store/Cargo.toml`:
  Entweder `optional = true` bei beiden Abhängigkeiten ergänzen:
  ```toml
  acta-types = { workspace = true, optional = true }
  acta-ports = { workspace = true, optional = true }
  ```
  oder das ungenutzte `workspace`-Feature bereinigen:
  ```toml
  workspace = []
  ```

## Finding #2: Fehlende Modul-Deklarationen in `crates/acta-store/src/lib.rs`
- **Datei:** `crates/acta-store/src/lib.rs`
- **Beobachtung:** `mod budget;` und `mod stats;` fehlen in `src/lib.rs`, obwohl `crate::budget::ResourceTracker` und `crate::stats::StorageStats` referenziert werden.
- **Reproduktion:** Nach Auflösung des Manifest-Fehlers scheitert der Build an fehlenden Modulen `budget` und `stats`.
- **Vorschlag:** Laut Baseline-Vorbedingung in `lib.rs` nach `pub use error::{StoreError, StoreResult};` einfügen:
  ```rust
  pub mod budget;
  pub mod stats;
  ```
