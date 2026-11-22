//! # Katalog-Modul
//!
//! SPEC §9.7: Der Katalog als einzige Wahrheit über lebendige Dateien,
//! atomare Edits (P-2), crash-sicherer Rollover und Typzustand für Dauerhaftigkeit (M-5).

pub mod codec;
pub mod current;
pub mod durable;
pub mod edit;
pub mod field_decode;
pub mod log;
pub mod reserve;
pub mod rollover;
pub mod state;
pub mod types;

pub use current::{read_current, write_current, CURRENT_FILE_NAME};
pub use durable::{sync_existing, write_atomic_synced, Synced};
pub use edit::{CatalogEdit, FIELD_OPTIONAL_FLAG};
pub use log::{catalog_file_name, CatalogHeader, CatalogLog, CATALOG_HEADER_LEN, CATALOG_MAGIC};
pub use reserve::{IdError, IdReserve, ID_RESERVE_BLOCK};
pub use rollover::rollover;
pub use state::{apply_edit, CatalogState};
pub use types::{
    CatalogError, CatalogFileKind, CatalogKeyRange, CatalogSpaceOptions, ErasePendingMeta,
    ErasedRangeMeta, FileMeta, HoldMeta,
};
