# FROZEN SURFACE: Außenfläche der Komponente D (Compaction & System-Pressure)

## 1. Angebotene Symbole (Exportiert)

### Re-exportiert über `crates/acta-store/src/lib.rs` (aus `src/compaction.rs`):
- `AdaptiveCompactionPlan` (aus `compaction::adaptive`)
- `AdaptiveCompactionPlanner` (aus `compaction::adaptive`)
- `CompactionStrategy` (aus `compaction::adaptive`)
- `CostBasedAdaptivePlanner` (aus `compaction::adaptive`)
- `WorkloadMetrics` (aus `compaction::adaptive`)
- `WorkloadMetricsSnapshot` (aus `compaction::adaptive`)
- `CompactionConfig` (aus `compaction::config`)
- `CompactionEngine` (aus `compaction::engine`)
- `MergeOperator` (aus `compaction::merge_operator`)
- `TtlMetadata` (aus `compaction`)

### Re-exportiert über `crates/acta-store/src/lib.rs` (aus `src/system_pressure.rs`):
- `PressureLevel`
- `SystemPressure`
- `SystemPressureMonitor`

### Intern genutzt im Crate (`crates/acta-store/src/`):
- `crate::compaction::retain_key_versions` in `src/lsm/ops/compaction.rs`
- `crate::compaction::{CompactionEngine, MergeOperator}` in `src/lsm/engine.rs`, `src/lsm/recovery.rs`
- `crate::compaction::CompactionConfig` in `src/lsm/config.rs`
- `crate::system_pressure::SystemPressure` in `src/lsm/engine.rs`
- `crate::system_pressure::SystemPressureMonitor` in `src/lsm/recovery.rs`
- `crate::system_pressure::PressureLevel` in `src/lsm/tests/commit_tests.rs`
- `crate::system_pressure::WAL_QUEUE_CRITICAL_THRESHOLD` in `src/lsm/tests/commit_tests.rs`

### Externe Konsumenten im Workspace:
- `acta-engine`:
  - `acta_store::PressureLevel` (`src/collection/crud/internal.rs`)
  - `acta_store::SystemPressure` (`src/collection/mod.rs`)
  - `acta_store::MergeOperator` (`src/merge_operator_adapter.rs`, `tests/wasm_merge_operator.rs`)
  - `acta_store::compaction::{CompactionConfig, CompactionEngine}` (`tests/wasm_merge_operator.rs`)

## 2. Konsumierte Symbole (Abhängigkeiten)
- `crate::budget::ResourceTracker`: Ressourcenbudgetierung für Compaction
- `crate::stats::StorageStats`: Statistikaggregation
- `crate::io_seam::Vfs`, `VfsFile`, `RealFs`: Virtuelles Dateisystem
- `crate::sstable::{BlockCache, SstableBuilder, SstableReader}`: SSTable Erstellung, Lesen und Caching
- `crate::manifest::Manifest`: SSTable-Registrierung und -Metadaten
- `crate::wal::KeyManager`: Verschlüsselungsschlüssel-Verwaltung
- `acta_mvcc::SnapshotRegistry`: Sichtbarkeit und MVCC-Snapshots
- `acta_env::Clock`, `MetricsSink`: Zeit und Metriken
- `acta_ids::TOMBSTONE_BIT`: Tombstone-Kennzeichnung
