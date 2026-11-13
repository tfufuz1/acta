# FlatBuffers Code Generation & Governance

## Overview
This document describes the auto-generated FlatBuffers IPC code stored in `crates/acta-wire/src/acta_generated.rs` and managed by the `generated` module (`crates/acta-wire/src/generated.rs`).

## Generator Tooling & Version
- **Compiler:** `flatc` (FlatBuffers Compiler)
- **Version:** `24.12.23` (FlatBuffers 24.3 Rust runtime)
- **Schema Source:** `schemas/acta.fbs`
- **Schema SHA-256 Pin:** `schemas/acta.fbs.sha256`

## Regeneration Command
To regenerate `src/acta_generated.rs` from `schemas/acta.fbs`, execute the following command from the crate directory (`crates/acta-wire`):
```bash
flatc --rust -o src schemas/acta.fbs
```
After regenerating, update the SHA-256 hash in `schemas/acta.fbs.sha256`:
```bash
sha256sum schemas/acta.fbs | awk '{print $1}' > schemas/acta.fbs.sha256
```
Then verify that all tests pass:
```bash
cargo test -p acta-wire
```

## Drift Protection Tests (`tests/schema_drift.rs`)
Drift protection is enforced via crate-level integration tests in `tests/schema_drift.rs`:
1. **`test_schema_hash_matches_pin`:** Calculates the SHA-256 digest of `schemas/acta.fbs` (using a standard FIPS 180-4 implementation without third-party crate dependencies) and compares it against `schemas/acta.fbs.sha256`. If a discrepancy is detected, the test fails, printing the new hash and regeneration instructions.
2. **`test_generated_code_matches_flatc_output_if_available`:** If `flatc` is present in `PATH` and reports version `24.12.23`, the test regenerates code into a temporary directory, applies the post-processing transformations, and asserts byte-for-byte identity with `src/acta_generated.rs`. If `flatc` is not installed or reports an unexpected version, an unmissable warning notice is printed to stderr, and only this partial test passes while the hash pin test remains strictly enforced.

## Finding on `build.rs`
The existing `build.rs` checks `Path::new("../../schemas/acta.fbs").exists()`. Because Cargo executes build scripts with the crate root directory (`crates/acta-wire`) as current working directory, this path resolves to `<Repo-Root>/schemas/acta.fbs`. Since `<Repo-Root>/schemas/` does not exist (the schema is located at `crates/acta-wire/schemas/acta.fbs`), the condition in `build.rs` is never met and the build script is a no-op in normal crate builds. Per governance instructions, `build.rs` is preserved unchanged.

## Safety Governance & Unsafe Baseline
- **Absence of `// SAFETY:` Comments:** Code produced by `flatc` contains `unsafe` blocks for low-level byte table access without generating inline `// SAFETY:` comments.
- **Strict Drift Protection:** To preserve schema consistency and ensure schema drift tests pass, hand editing of `src/acta_generated.rs` is strictly prohibited.
- **Module Isolation:** All generated code is encapsulated within `crates/acta-wire/src/generated.rs`, which applies module-level lints and suppression (`@generated, siehe tests/schema_drift.rs`).
- **Baseline Exception (GV-1/XT-1):** The unsafe block count in `acta_generated.rs` (43 unsafe blocks) is covered under the baseline exception requested via GV-1/XT-1 in `.agents/governance/unsafe-safety-baseline.toml`.
