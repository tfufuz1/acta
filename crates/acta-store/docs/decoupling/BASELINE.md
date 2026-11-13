# BASELINE — acta-store (Auftrag S-37)

## 1. Ausgangszustand vor Beginn
- Commit-Basis: `ca103a3c743df21a70c3b0b0e2b3972a693a5ca4` (origin/main)
- `cargo check -p acta-store`: Nach minimaler Bereinigung von ungenutztem `_runtime` Feld in `scanner/memtable.rs` grün (0 Compilerfehler).
- Workspace-Status: Vorbestehende Compilerfehler in `acta-cognition` (`CognitionError`, `ActaError` in `channel.rs` und `context_compaction/session.rs`). Crate-lokaler Check für acta-store ist isoliert grün.
- Bridge-Referenzen (`tools/count_bridge_refs.py -c acta-store`): **238** Referenzen auf `acta_types` / `acta_ports` / `acta_testkit` im Kern außerhalb von `compat/`.
- Verteilung der Bridge-Referenzen laut Analyse:
  - `compaction/`: 82
  - `lsm/`: 65
  - `scanner/`: 38
  - `wal/`: 17
  - `sstable/`: 15
  - `manifest/`: 3
  - `engine/`: 2
  - `tenant_codec/`: 1

## 2. Feature-Befund
- `Cargo.toml` deklariert:
  - `default = []`
  - `test-hooks = []`
  - `inspect = []`
  - `memory-only-storage = []`
  - `fault-injection = []`
  - `block-cache-v2 = ["dep:quick_cache"]`
  - `sieve-cache = []`
  - `docid-128 = ["acta-ids/docid-128"]` (auf `acta-ids` umgelenkt)
  - `legacy-wal-key = []`
