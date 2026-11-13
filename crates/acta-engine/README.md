# acta-engine

Transaktions-, Index- und Ausführungsorchestrator (Ebene E2 Integration, Klasse C, Ring 3).

## Rolle & Zweck
`acta-engine` verbindet unabhängige Speicher- und Index-Bausteine (LSM-Store, HNSW-Vektorindex, BM25-Volltext, CSR-Graph) zu einer konsistenten Transaktions- und RAG-Ausführungsumgebung.

```
                           +------------------------+
                           |    acta-engine    |
                           +-----------+------------+
                                       |
                   +-------------------+-------------------+
                   |                   |                   |
                   v                   v                   v
           +---------------+   +---------------+   +---------------+
           | StorageEngine |   |  VectorIndex  |   |   TextIndex   |
           | (acta-   |   | (acta-   |   | (acta-   |
           |    store)     |   |    vector)    |   |    text)      |
           +---------------+   +---------------+   +---------------+
                                       |
                                       v
                               +---------------+
                               |  GraphIndex   |
                               | (acta-   |
                               |    graph)     |
                               +---------------+
```

## Baukasten-Einordnung & Status
- **Ebene:** Ebene E2 (Integration)
- **Klasse:** Klasse C (Integrations- und Kompositionsschicht)
- **Ring:** Ring 3 (Ausführungsorchestrator)
- **Sicherheits-Invariante:** `#![forbid(unsafe_code)]`
- **Zero-Panic-Doktrin:** Vollständig konform im Produktivcode

## Tatsächliche Abhängigkeiten (laut `cargo tree`)
- **Kern-Closure (`--no-default-features`):**
  - `acta-types`, `acta-ports`, `acta-mvcc`, `acta-checkpoint`
  - `acta-store`, `acta-vector`, `acta-text`, `acta-graph`, `acta-rank`
  - Transitive Mikrokerne: `acta-ids`, `acta-env`, `acta-sys`, `acta-crypto`
- **Optionale Feature-Abhängigkeiten:**
  - `acta-adapt` (über `adaptive-candidate-pool-sizing`)
  - `acta-infer-onnx` (über `onnx`)
  - `acta-sandbox` (über `sandbox`)
- **Entwicklungs-Abhängigkeiten:**
  - `acta-ingest`, `acta-testkit`, `tokio`, `tempfile`, `proptest`, `loom`, `wat`, `sha2`

## Feature-Tabelle

| Feature | Zweck | Abhängigkeiten / Weiterleitungen |
|---|---|---|
| `onnx` | In-Process ONNX Text-Einbettung | `acta-infer-onnx/onnx` |
| `reranking` | Aktiviert Re-Ranking Support | — |
| `sandbox` | WASM Merge Operator Isolierung | `acta-sandbox` |
| `experimental-diskann` | DiskANN Vektor-Index Modus | `acta-vector/experimental-diskann` |
| `background-maintenance` | Automatische Hintergrundbereinigung | — |
| `graph-connectivity-health` | Graph-Perkolations- und Zustandsanalyse | `acta-graph/graph-connectivity-health` |
| `edge-reinforcement-learning` | Weiterleitung für Graph RL | Weiterleitung von `cognition` |
| `adaptive-candidate-pool-sizing` | Dynamische Suchpoolanpassung | `acta-adapt` |
| `exp-extraction` | Experimentelle Wissensextraktion | — |
| `entity-extraction` | Regelbasierte & OpenIE Extraktion | `exp-extraction` |
| `auto-extraction-opt-out` | Deaktiviert automatische Extraktion | — |

## Architektur-Eigenschaften
- **Domänenreinheit (M7):** Keine Saga-, Witness- oder DeletionProof-Logik im Kern. Die physische Bereinigung delegiert rein an neutrale `Purger`- und `LayerScanner`-Schnittstellen.
- **Lizenz-Naht:** Sämtliche Lizenz-Gate Zugriffe sind im internen Modul `src/license_seam.rs` gekapselt für nahtlose spätere Umstellung auf `acta-license`.
- **Transaktions-Isolation:** 4-Index 2-Phase-Commit mit Intent-Logging und idempotentem `CommitLedger`-Rollback.
- **Konsumenten-Schutz:** Re-Exporte und Konsumentenpfade für `acta-db` und `acta-cognition` sind über den Compile-Time Test `tests/s64_public_surface.rs` abgesichert.

## Minimalbeispiel
```rust,no_run
use acta_engine::{Acta, ActaConfig};
use tempfile::tempdir;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let config = ActaConfig::default();
    let engine = Acta::open_with_config(dir.path(), config).await?;
    let col = engine.collection("knowledge").await?;
    col.insert("doc1", &[0.1; 768], None).await?;
    Ok(())
}
```
