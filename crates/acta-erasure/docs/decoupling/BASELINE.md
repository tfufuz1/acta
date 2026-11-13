# BASELINE acta-erasure (AUFTRAG ERASURE-A)

## Messungen am HEAD 882aada72

- **Umfang:** 16288 LOC in 82 Rust-Dateien
- **Tests:** Alle 41 Integrationstest-Suiten und Lib-Unit-Tests bestanden (`cargo test -p acta-erasure --locked`)
- **Gates:**
  - `cargo check -p acta-erasure --no-default-features --locked`: GRÜN
  - `cargo test -p acta-erasure --locked`: GRÜN
  - `python3 tools/isolation_proof.py -c acta-erasure --modes no-default,default,each-feature`: GRÜN (alle Features isoliert baubar)
  - `python3 tools/export_crate.py acta-erasure --out /tmp/export-acta-erasure`: GRÜN (Gate G3 bestanden)
  - `python3 tools/modularity_score.py -c acta-erasure`: Score 100/100, Ebene E0, 0 Bridge Refs
  - `python3 tools/decouple_inspector.py -c acta-erasure`: Score 100/100, 0 interne Abhängigkeiten
  - `python3 tools/zero_panic_analyzer.py --crate acta-erasure --fail-on-prod`: 0 Prod-Verstöße
  - `python3 tools/gate_layering.py --crate acta-erasure`: GRÜN
