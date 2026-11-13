# FINDINGS: Befunde außerhalb des primären Auftrags D

## 1. Cargo.toml Defekt in `crates/acta-store`
- **Datei**: `crates/acta-store/Cargo.toml:17`
- **Beobachtung**:
  ```toml
  workspace = ["dep:acta-types", "dep:acta-ports"]
  ```
  steht in Konflikt mit den nicht-optionalen Abhängigkeiten:
  ```toml
  acta-types = { workspace = true }
  acta-ports = { workspace = true }
  ```
- **Auswirkung**: Cargo verweigert das Parsen des Manifests mit:
  `feature "workspace" includes "dep:acta-types", but "acta-types" is not an optional dependency`
- **Reproduktion**: `cargo check -p acta-store`
- **Lösungsvorschlag**: In `Cargo.toml` die Dependencies als `optional = true` deklarieren, wie in commit `15ef71077`.

## 2. Fehlende ActaError-Varianten in Tests fremder Module
- **Dateien**: `src/lsm/tests/commit_tests.rs`, `src/sstable/tests.rs`
- **Beobachtung**: Tests versuchen `ActaError::InvalidInput` und `ActaError::ChecksumMismatch` abzugleichen, während `StoreError` verwendet wird.
- **Zuständigkeit**: Agent B (SSTable) und Agent E (LSM-Schreibpfad).
