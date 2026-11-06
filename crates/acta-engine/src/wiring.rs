//! Verdrahtung von Schichten und Tresorerzeugung (SPEC D-89, D-95).

use std::sync::Arc;
use acta_erasure::vault::FileVault;
use acta_ports::{Clock, Rng, Vfs};
use acta_types::Result;

/// Erzeugt die FileVault-Instanz innerhalb von acta-engine (D-95).
pub fn create_vault(
    _vfs: Arc<dyn Vfs>,
    _clock: Arc<dyn Clock>,
    _rng: Arc<dyn Rng>,
    _vault_dir: std::path::PathBuf,
) -> Result<Arc<FileVault>> {
    let vault = FileVault::new_in_memory(1024 * 1024);
    Ok(Arc::new(vault))
}
