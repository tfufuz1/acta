// FILE-CONTEXT
// ZWECK: Shared integration test helper functions for the `acta` primary facade test suite.
// INVARIANTEN: Greift ausschließlich über die öffentliche Facade (acta::*) zu; no internal crate imports.

//! # Modul: Gemeinsame Integrations-Test-Hilfsfunktionen (`common`)
//!
//! Stellt wiederverwendbare Hilfsfunktionen für die Integrationstests der Fassade bereit.
//!
//! ## Invarianten
//! - Verwendet ausschließlich die öffentliche Schnittstelle von `acta`.
//! - Keine direkten internen Crate-Imports aus tieferen Ringen.

use acta::performance_profile::PerformanceProfile;
use acta::{builder, Acta, ActaError};
use serde_json::{json, Value};
use tempfile::TempDir;

// ANCHOR[acta:TEST-COM-001]
/// Erstellt eine schnelle, temporäre `Acta`-Testdatenbank mit `PerformanceProfile::BareMetal`.
///
/// Vermeidet Festplatten-Syncs im WAL für maximale Testausführungsgeschwindigkeit.
pub async fn create_test_db(
    tmp_dir: &TempDir,
    dimension: usize,
) -> Result<Acta, ActaError> {
    builder(dimension)
        .with_storage_path(tmp_dir.path())
        .with_performance_profile(PerformanceProfile::BareMetal)
        .build()
        .await
}

// ANCHOR[acta:TEST-COM-002]
/// Generiert einen synthetischen Testvektor der gegebenen Dimension mit einem konstanten Wert.
pub fn dummy_vector(dimension: usize, value: f32) -> Vec<f32> {
    vec![value; dimension]
}

// ANCHOR[acta:TEST-COM-003]
/// Erzeugt standardisierte JSON-Metadaten für Testdokumente mit Tag und Sequenznummer.
pub fn dummy_metadata(tag: &str, sequence: usize) -> Value {
    json!({
        "tag": tag,
        "sequence": sequence,
        "test_env": "facade_integration"
    })
}
