//! # Integrationstest: auto extraction default enabled
//!
//! **Zweck:** Prüft spezifische Garantien, Transaktionskonsistenz und Fehlertoleranz der Engine.
//!
//! Gehört zur Testsuite von `acta-engine`.

#![cfg(not(loom))]
// FILE-CONTEXT
// ZWECK: Testet den kompilierte Default-Wert fuer AutoExtractionConfig (enabled: true vs. opt-out).

use acta_engine::collection::crud::AutoExtractionConfig;

// ANCHOR[acta-engine:TST01-001]
/// Testblock zur Überprüfung von `test_auto_extraction_config_default_value`.
#[test]
fn test_auto_extraction_config_default_value() {
    let cfg = AutoExtractionConfig::default();

    #[cfg(not(feature = "auto-extraction-opt-out"))]
    {
        assert!(
            cfg.enabled,
            "AutoExtractionConfig::default().enabled must be true by default"
        );
    }

    #[cfg(feature = "auto-extraction-opt-out")]
    {
        assert!(
            !cfg.enabled,
            "AutoExtractionConfig::default().enabled must be false when auto-extraction-opt-out feature is active"
        );
    }
}
