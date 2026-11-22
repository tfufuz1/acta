//! # Storage Engine
//!
//! SPEC §12, §13, §16:
//! - `flush`: 5-Schritte-Schreibübergabe aus eingefrorenen Memtables
//! - `recovery`: 11-Schritte-Wiederanlauf nach Absturz als Zustandsmaschine
//! - `compaction`: Leveled-Kompaktion mit reiner Planung und Versionenaufbewahrung

pub mod compaction;
pub mod flush;
pub mod recovery;

pub use compaction::{
    execute_compaction, install_compaction, plan, retain_key_versions, Claims,
    CompactionEntry, CompactionExecutionResult, CompactionInstallError, CompactionParams,
    CompactionPlan, RetainResult,
};
pub use flush::{
    flush_memtable, CommitterSync, FlushError, FrozenMemtable, MemtableEntry,
};
pub use recovery::{
    recover_instance, RecoveryError, RecoveryResult, RecoveryStep,
};
