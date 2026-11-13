# Baseline `acta-simd` (WP-M1)

Stand vor Eingriffen / Baseline-Erhebung:

- Git Commit Baseline: `f0eda1cb9` / Arbeitsstand
- Rust Toolchain: `1.89.0` (rustc 1.89.0)

## 1. Teststatus unverändert
- `cargo test -p acta-simd`:
  - unittests src/lib.rs: 13 passed, 0 failed
  - tests/building_block_conformance.rs: 3 passed, 0 failed
  - tests/differential_tests.rs: 18 passed, 0 failed
  - tests/edge_cases_tests.rs: 24 passed, 0 failed
  - tests/kernel_reference_tests.rs: 6 passed, 0 failed
  - tests/quantized_and_mixed_precision_tests.rs: 11 passed, 0 failed
  - tests/simd_cross_validation.rs: 15 passed, 0 failed
  - doc-tests: 2 passed, 0 failed
  - Gesamt: 92 Tests grün (0 failed, 0 ignored, 0 flaky)

- `cargo test -p acta-simd --all-features`:
  - 92 Tests grün

- `cargo clippy -p acta-simd --all-targets -- -D warnings`: 0 warnings, grün
- `cargo clippy -p acta-simd --all-targets --all-features -- -D warnings`: 0 warnings, grün

## 2. Ist-Analyse der Referenzen auf `acta_types`
- `crates/acta-simd/src/metric.rs`: 8 Vorkommen
  - `From<DistanceMetric> for acta_types::DistanceMetric` (Zeilen 12-18)
  - `From<acta_types::DistanceMetric> for DistanceMetric` (Zeilen 23-31)
- `crates/acta-simd/src/error.rs`: 5 Vorkommen
  - `From<SimdError> for acta_types::ActaError` (Zeilen 24-38)
- Gesamt: Exakt 13 Vorkommen unter `src/`.
- In `tests/`: 0 Vorkommen.
- Alle Vorkommen im Kern sind hinter `#[cfg(feature = "types-compat")]`.
