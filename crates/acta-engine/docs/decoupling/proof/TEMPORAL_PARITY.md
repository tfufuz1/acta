# Äquivalenz- und Differenzbeweis: Temporalfilter (S-33)

- **Untersuchte Module:**
  - `crates/acta-engine/src/temporal_filter.rs`
  - `crates/acta-rank/src/temporal.rs`
  - `crates/acta-engine/tests/legacy_reference/temporal_filter_legacy.rs`
- **Test-Suite:** `crates/acta-engine/tests/proof_temporal_filter_vs_rank.rs`
- **Ergebnis Testlauf:** 8 passed; 0 failed (inkl. 10.000 Proptest-Fälle)

## Logische und algorithmische Äquivalenz
Die Extraktions- und Bewertungslogik ist bit- und verhaltensidentisch:
1. `extract_validity_window(metadata)`:
   - Extrahiert `tx_valid_from`, `tx_valid_to`, `business_valid_from`, `business_valid_to` identisch.
   - Unterstützt identische Alias-Namen (`tx_valid_until`, `valid_from`, `valid_to`, `valid_until`).
   - Identische Behandlung von String- und Integer-Darstellungen sowie negativen Timestamps.
2. `is_valid_bitemporal(as_of_tx, as_of_business)`:
   - Identische Semantik des Intervalls `[valid_from, valid_until)` (geschlossener Anfang, offenes Ende).
   - Identisches Fail-Open-Verhalten bei fehlenden Metadaten (leeres Fenster ist immer gültig).
3. `sort_candidates_deterministic`:
   - Identische Sortierung: Score absteigend, Tie-Breaking nach Dokument-ID aufsteigend.

## Schnittstellen- und Typdivergenzen (Verhinderung der direkten Re-Export-Ablösung)
Gemäß Prompt-Abschnitt 3 (ENG-B-1.3 / ENG-B-1.4) und Regel 0.9 darf eine Ablösung durch `pub use acta_rank::temporal::{...}` NUR erfolgen, wenn Typidentität vorliegt und Aufrufer in `src/collection/**` ohne Änderung kompilieren.

Folgende Divergenzen verhindern aktuell die direkte Ablösung:
1. **`TxId`-Typidentität:**
   - `acta-engine::temporal_filter` nutzt `acta_types::TxId` (aus dem Ring-0-Kernel).
   - `acta-rank::temporal` nutzt den autarken Ring-0-Klasse-A-Lokaltyp `acta_rank::local_types::TxId`.
   - Aufrufer in `src/collection/query_builder/builder_exec.rs` und `src/collection/search/mod.rs` (Eigentum ENG-A) übergeben `acta_types::TxId`. Ein Re-Export von `acta-rank` führt zu Typmismatch `expected acta_rank::local_types::TxId, found acta_types::TxId`.
2. **`Clock`-Trait-Abstraktion:**
   - `acta-engine::temporal_filter::apply_temporal_validity_filter_with_clock` akzeptiert `&dyn acta_ports::Clock`.
   - `acta-rank::temporal::apply_temporal_validity_filter_with_clock` akzeptiert `&dyn acta_rank::calibration::Clock`.
3. **`ValidityWindow`-Felder:**
   - `ValidityWindow.tx_valid_from` und `tx_valid_to` sind vom Typ `Option<acta_rank::local_types::TxId>` statt `Option<acta_types::TxId>`.

## Entscheidung und Übergabe
- **Entscheidung:** Keine Ablösung von `crates/acta-engine/src/temporal_filter.rs` durch einen `pub use`-Reexport in diesem Auftrag. Die Engine-Kopie bleibt voll funktionsfähig und verhaltensgleich erhalten.
- **Übergabe an ENG-A / maintainer:**
  - Zur vollständigen Ablösung muss in `acta-rank` entweder eine typkompatible Fassung über `acta-rank::compat` angeboten werden, oder die Aufrufer in `src/collection/**` müssen auf Typkonvertierungen (`into()`) angepasst werden, was Änderungen an Manifesten und ENG-A-Dateien erfordert.
