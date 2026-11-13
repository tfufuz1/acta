# AGENTS.md — `acta-mvcc`

> Werkzeug/Crate-Kontext für Agenten. Der obere Teil ist generiert; manuelle Notizen unterhalb ergänzen.

<!-- BEGIN GENERATED: crate_inventory.py -->

## Inventar (generiert)

- **Pfad:** `crates/acta-mvcc` · **Ring:** 0 · **Version:** 0.1.0
- **Beschreibung:** Multi-Version Concurrency Control (MVCC), sequence log, and transaction buffer for Acta
- **src:** 10 Dateien, 3305 LOC · **unsafe:** 0 Vorkommen, Policy `forbid`
- **Interne Abhängigkeiten (normal, 4):** `acta-env`, `acta-ids`, `acta-ports`, `acta-types`
- **Interne Dev/Build-Abhängigkeiten (3):** `acta-env`, `acta-ports`, `acta-types`
- **Externe Abhängigkeiten (normal, 7):** `ahash`, `bytes`, `loom`, `parking_lot`, `serde`, `thiserror`, `tracing`
- **Reverse-Abhängigkeiten (3):** `acta-db`, `acta-store`, `acta-vector`
- **Kopplung an Hub-Crates:** ausgehend `acta-ports`, `acta-types`; eingehend `acta-db`
- **Features (5):** `default`, `docid-128`, `ports-compat`, `types-compat`, `workspace`

### Öffentliche API (grob, aus src/)
- struct (11): `PruningBlockerInfo`, `ReadSet`, `SeqBitTombstone`, `SeqLogEntry`, `SequenceLog`, `SequenceLogSsiValidator`, `SnapshotGuard`, `SnapshotRegistry`, `SnapshotRegistryConfig`, `TxBuffer`, `TxBufferConfig`
- enum (3): `IndexOp`, `MvccError`, `SeqLogChange`
- trait (3): `SsiValidator`, `StagedOpSize`, `TombstoneSemanticsCheck`
- fn (111): `begin`, `begin_at`, `changes_since`, `clear`, `clock`, `compact`, `config`, `deletion_seq`, `diagnose_pruning_blocker`, `diagnose_pruning_blocker_at`, `discard`, `discard_kv`, `doc_id`, `drain`, `drain_kv` … (+96)
- type (1): `Result`
- const (8): `DEFAULT_MAX_OPS_PER_TX`, `DEFAULT_MAX_PIN_DURATION`, `DEFAULT_MAX_READ_SET_KEYS`, `DEFAULT_MAX_TOTAL_STAGED_BYTES`, `DEFAULT_MAX_TRACKED_COMMIT_KEYS`, `DEFAULT_MAX_TX_STAGED_BYTES`, `DEFAULT_SHARD_COUNT`, `STAGING_ENTRY_OVERHEAD_BYTES`
- mod (9): `compat`, `error`, `mvcc_error`, `seq_log`, `snapshot`, `ssi`, `tombstone`, `tx_buffer`, `types`
- pub use: `acta_env::{Clock, MetricsSink, SystemClock}`, `acta_ids::{DocId, TenantId, TxId, TOMBSTONE_BIT}`, `acta_types::error as workspace_error`, `acta_types::error::ActaError`, `acta_types::error::Result`, `crate::compat::{workspace_error as error, ActaError, Result}`, `crate::mvcc_error::Result`, `crate::mvcc_error::{MvccError, Result as MvccResult}`, `seq_log::{SeqLogChange, SeqLogEntry, SequenceLog}`, `snapshot::{SnapshotGuard, SnapshotRegistry}` … (+3)

Regeneriert mit `python3 tools/crate_inventory.py agents`. Manuelle Notizen stehen außerhalb dieses Blocks.

<!-- END GENERATED: crate_inventory.py -->
