# Baseline-Erhebung S-34 `acta-graph` (Entkopplung)

Datum: 2026-11-15
Branch: jules-13242595197714389149-b82bf81c
Toolchain: rustc 1.89.0 (29483883e 2025-08-04) / cargo 1.89.0 (c24e10642 2025-06-23)

## 1. Kompilierungs- und Workspace-Status (Baseline)
- `cargo check --workspace --lib --bins --locked`: GRÜN
- `cargo check --workspace --all-targets --locked`: Vorbestehend ROT (2 Fehler in `acta-store`: `MemtableScanner.runtime` unbenutztes Feld)
  - `acta-store` (lib): error: field `runtime` is never read (crates/acta-store/src/scanner/memtable.rs:32:5)
  - `acta-store` (lib test): error: field `runtime` is never read
- `cargo check -p acta-graph --all-targets`: GRÜN (2 Deprecation-Warnungen bzgl. `compat::purger::graph_purger` und `GraphPurger`)
- `cargo check -p acta-graph --no-default-features`: 51 Fehler (unresolved imports `acta_types`, `acta_ports`, `GraphProvider`)

## 2. Tests (Baseline mit unverändertem Code)
- `cargo test -p acta-graph --lib`: 218 passed, 0 failed, 0 ignored (Dauer: ~54s)
- `cargo test -p acta-graph --test golden_corpus`: 2 passed, 0 failed, 0 ignored

## 3. Referenzinventur
- Referenzen auf `types`, `ports`, `testkit` laut `python3 tools/count_bridge_refs.py -c acta-graph`: 135 Treffer
- Aufteilung:
  - `cascade.rs`: `ActaError::InvalidInput`
  - `csr/`: `csr/graph_persist.rs`, `csr/graph_read/`, `csr/inner/`, `csr/tests/`
  - `ppr/`: `ppr/cost.rs`, `ppr/shadow_hook.rs`, `ppr/mod.rs`
  - `apprh/`: `apprh/mod.rs`, `apprh/error.rs`, etc.
  - `tl_hfd/`: `tl_hfd/diffusion.rs`, `tl_hfd/shadow.rs`, `tl_hfd/error.rs`
  - `path_rag/`: `path_rag/mod.rs`
  - `community.rs`: `CommunityDetectionConfig`, `CommunityAssignment`
  - `hyperedge.rs`: `ActaError`, Re-Exports von Ports
  - `entity_extraction.rs`: `DocId`, `EntityId`, `ActaError`
  - `provenance.rs`: `DocId`, `TxId`
  - `session_dag.rs`: `ActaError`

## 4. Feature-Befund & Fremdnutzer
- Bestehende Features in `acta-graph/Cargo.toml`:
  - `default = ["workspace", "experimental-graph"]`
  - `workspace = ["ports-compat", "types-compat"]`
  - `types-compat = ["dep:acta-types", "dep:acta-ports"]`
  - `ports-compat = ["types-compat"]`
  - `experimental-graph = []`
  - `docid-128 = []`
  - `graph-connectivity-health = []`
  - `edge-reinforcement-learning = []`
  - `ppr-forward-push = []`
  - `k-path-diffusion = []`
  - `apprh-diffusion = []`
  - `physio-percolation = ["graph-connectivity-health"]`
- Feature-Weiterleitungen durch Fremdnutzer:
  - `acta-engine`:
    - `graph-connectivity-health = ["acta-graph/graph-connectivity-health"]`
  - `acta-cognition`:
    - `graph-connectivity-health = ["acta-graph/physio-percolation", "acta-engine/graph-connectivity-health"]`
    - `edge-reinforcement-learning = ["acta-graph/edge-reinforcement-learning", "acta-engine/edge-reinforcement-learning"]`
- Abhängigkeiten von `acta-graph`:
  - `acta-db` (`workspace = true`)
  - `acta-engine` (`workspace = true`)
  - `acta-agent` (`workspace = true`)
  - `acta-cognition` (`workspace = true`)
  - `acta-graph-fuzz` (`path = ".."`)

## 5. Zielvorgabe S-34
- Entkopplung des Kerns: Kern-Closure nur `acta-ids`, `acta-env`.
- `acta-types` und `acta-ports` nur hinter `workspace` in `src/compat/`.
- Fehlerzahl ohne Default-Features von 51 auf 0 reduzieren.
- Brücken-Referenzen im Kern auf 0 reduzieren.
