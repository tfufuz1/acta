# Baseline-Protokoll: `acta-checkpoint` (Stand vor Umbau WP-1)

## Messungen am HEAD 882aada72

- Datum / Commit: HEAD `882aada72`
- Rust-Version: `rustc 1.89.0 (29483883e 2025-08-04)` / `cargo 1.89.0`
- Ziel-Crate: `crates/acta-checkpoint`

### 1. Prüfbefehle vor dem Eingriff

- `cargo check -p acta-checkpoint --all-targets --locked`: **GRÜN** (Dauer ~1.1s)
- `cargo test -p acta-checkpoint --locked`:
  - Lib-Tests: 59 passed; 0 failed
  - Vorab bekannte Testfehlschläge:
    - `error_display_test.rs`: `test_checkpoint_error_display_parity_with_acta_error` schlägt fehl wegen `"IO error:"` vs `"I/O error:"`.
    - `golden_corpus_test.rs`: Fehlschlag wegen `id_width_bits: 64`.
- `cargo check --workspace --lib --bins --locked --keep-going`: Siehe `/tmp/ws_lib_baseline.txt`
- `cargo check --workspace --all-targets --locked --keep-going`: Siehe `/tmp/ws_all_baseline.txt` (35 fremde Targets rot)

### 2. Entkopplungsmetriken vor Umbau

- Brücken-Referenzen (`tools/count_bridge_refs.py -c acta-checkpoint`): 4 Referenzen
  - `src/store.rs`: `use acta_ports::BoxFuture;`, `use acta_types::TxId;`
  - `src/source.rs`: `use acta_types::{Result, TxId};`, `impl<T: acta_ports::StorageEngine + ?Sized> SnapshotSource for T`
- Zero-Panic Analyzer: 0 Verstöße in Produktionscode
- Gate Layering (`python3 tools/gate_layering.py --crate acta-checkpoint`): GRÜN (mit transitional `workspace`)
- Keine-Default-Features Check (`cargo check -p acta-checkpoint --no-default-features`): Vorab **ROT** wegen unaufgelöster Imports von `acta_ports` und `acta_types` in `src/store.rs`.
