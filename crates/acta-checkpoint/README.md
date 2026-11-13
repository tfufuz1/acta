# acta-checkpoint

Persistent Checkpoint Engine, Hardlink Cloner und Blake3 Manifest-Verifikation für Acta Storage.

## Zweck & Abgrenzung
`acta-checkpoint` verwaltet Time-Travel Checkpoints, konsistente Snapshot-Exporte, RAII-gesicherte Transaktions-Rollbacks und Orphan-Recovery für heterogene Storage-Engines über schmale, selbst definierte Verträge (`contract::SnapshotSource`, `contract::CheckpointVfs`, `contract::FsSeam`).

## Reifegrad & Qualität
- **Status:** Stufe R3 (Release-fähig / Beta, Ebene 1 Retrieval)
- **Zero-Panic-Doktrin:** Vollständig typisierte Fehler ([`CheckpointError`]), kein `unwrap`/`expect`/`panic!` im Produktionscode.
- **Autarkie:** Der Kern (`--no-default-features`) hängt ausschließlich von `acta-ids` und `acta-env` ab.
- **Port-Isolation:** Alle Typen- und Port-Brücken zu `acta-types` und `acta-ports` sind in `src/compat/` hinter dem Feature `workspace` gekapselt.

## Interne Abhängigkeiten (Kern, `--no-default-features`)
```text
acta-checkpoint v0.1.0
├── acta-env v0.1.0
└── acta-ids v0.1.0
```
*(Optionale Integrationsbrücken `acta-types` und `acta-ports` liegen hinter dem Feature `workspace`)*

## Features

| Feature | Beschreibung | Default |
|---|---|---|
| `workspace` | Aktiviert Integrationsbrücken für `acta-types` und `acta-ports` in `src/compat/` | Ja (bis Z) |
| `types-compat` | Alias für `workspace` zur Rückwärtskompatibilität | Nein |
| `ports-compat` | Alias für `workspace` zur Rückwärtskompatibilität | Nein |
| `docid-128` | Aktiviert 128-Bit Identifier-Breite über `acta-ids` | Nein |

## Minimalbeispiel (Kompilierbar und Standalone)
```rust
use std::sync::Arc;
use acta_checkpoint::contract::InMemorySnapshotSource;
use acta_checkpoint::PersistentCheckpointStore;
use acta_ids::TxId;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = Arc::new(InMemorySnapshotSource::new());
    let store = PersistentCheckpointStore::open(source, "demo_ns").await?;

    let cp = store
        .create_checkpoint("cp1", "demo_coll", 1, TxId::new(100), json!({}))
        .await?;
    println!("Checkpoint erstellt: {}", cp.name);

    let restored = store.restore_checkpoint("cp1").await?;
    println!("Checkpoint wiederhergestellt: {}", restored.name);
    Ok(())
}
```

## Bekannte Grenzen
- Checkpoint-Namen und Collection-IDs sind auf maximal 256 Unicode-Zeichen begrenzt und dürfen nicht leer oder reine Whitespaces sein.
- Hardlink-Cloning setzt ein Dateisystem mit Hardlink-Unterstützung voraus; bei Cross-Device-Dateisystemgrenzen greift ein deterministischer Fallback auf File-Copy.
