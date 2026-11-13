# UNSAFE Transition tracking for `acta-vector`

| Datei:Zeile | Grund | Ziel-Crate für Migration | Tracking-ID | Status |
| --- | --- | --- | --- | --- |
| `src/persistence.rs:451` | `memmap2::Mmap::map(&file)` Zero-Copy Read-Only Memory-Mapping | `acta-sys` (`acta_sys::mmap_readonly`) | `TRANS-VEC-001` | MIGRATED |
| `src/diskann.rs:1589` | `memmap2::Mmap::map(&file)` Zero-Copy Read-Only Memory-Mapping | `acta-sys` (`acta_sys::mmap_readonly`) | `TRANS-VEC-002` | MIGRATED |

## Summary
All historical `unsafe` blocks in `crates/acta-vector` have been migrated to safe facades in `acta-sys` (specifically `acta_sys::mmap_readonly`).
`crates/acta-vector` now strictly enforces `#![forbid(unsafe_code)]` at the crate root (`src/lib.rs`) without any conflicting `#![allow(unsafe_code)]` directives.
