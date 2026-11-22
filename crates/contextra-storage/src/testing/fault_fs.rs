//! # Fehlereinspritzendes Dateisystem (FaultFs)
//!
//! Deterministische Fehlereinspritzung nach SPEC §4, §18 und IMPL §6 WP-1.3.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]

use contextra_storage_ports::{
    DirEntry, File, Fs, FsCaps, LockGuard, LockMode, OpenMode, Perm,
};
use std::io;
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

/// Konfiguration für Fehlereinspritzungen.
#[derive(Default)]
pub struct FaultConfig {
    /// Vorgabe, nach wie vielen Operationen ein Fehler ausgelöst wird (0 = deaktiviert).
    pub fail_after_ops: AtomicU64,
    /// Löst Fehler bei Sync-Operationen aus.
    pub fail_sync: AtomicBool,
    /// Simuliert vollen Speicherplatz (ENOSPC).
    pub fail_enospc: AtomicBool,
    /// Simuliert Dateisystemfehler (EIO).
    pub fail_io: AtomicBool,
}

/// Dateisystem-Wrapper mit gezielter Fehlereinspritzung.
pub struct FaultFs<F> {
    inner: F,
    config: Arc<FaultConfig>,
}

impl<F: Fs> FaultFs<F> {
    /// Erstellt ein neues `FaultFs`.
    pub fn new(inner: F, config: Arc<FaultConfig>) -> Self {
        Self { inner, config }
    }

    fn check_op(&self) -> io::Result<()> {
        if self.config.fail_enospc.load(Ordering::Relaxed) {
            return Err(io::Error::new(
                io::ErrorKind::StorageFull,
                "Injected ENOSPC",
            ));
        }
        if self.config.fail_io.load(Ordering::Relaxed) {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Injected EIO",
            ));
        }
        let ops = self.config.fail_after_ops.load(Ordering::Relaxed);
        if ops > 0 {
            if self.config.fail_after_ops.fetch_sub(1, Ordering::Relaxed) == 1 {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Injected crash after N operations",
                ));
            }
        }
        Ok(())
    }
}

struct FaultFile {
    inner: Box<dyn File>,
    config: Arc<FaultConfig>,
}

impl File for FaultFile {
    fn read_at(&self, buf: &mut [u8], off: u64) -> io::Result<usize> {
        self.inner.read_at(buf, off)
    }

    fn write_all_at(&self, buf: &[u8], off: u64) -> io::Result<()> {
        if self.config.fail_enospc.load(Ordering::Relaxed) {
            return Err(io::Error::new(
                io::ErrorKind::StorageFull,
                "Injected ENOSPC",
            ));
        }
        self.inner.write_all_at(buf, off)
    }

    fn len(&self) -> io::Result<u64> {
        self.inner.len()
    }

    fn set_len(&self, len: u64) -> io::Result<()> {
        self.inner.set_len(len)
    }

    fn sync_data(&self) -> io::Result<()> {
        if self.config.fail_sync.load(Ordering::Relaxed) {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Injected sync failure",
            ));
        }
        self.inner.sync_data()
    }

    fn sync_all(&self) -> io::Result<()> {
        if self.config.fail_sync.load(Ordering::Relaxed) {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Injected sync failure",
            ));
        }
        self.inner.sync_all()
    }
}

impl<F: Fs> Fs for FaultFs<F> {
    fn create_dir_all(&self, p: &Path, perm: Perm) -> io::Result<()> {
        self.check_op()?;
        self.inner.create_dir_all(p, perm)
    }

    fn open(&self, p: &Path, mode: OpenMode) -> io::Result<Box<dyn File>> {
        self.check_op()?;
        let file = self.inner.open(p, mode)?;
        Ok(Box::new(FaultFile {
            inner: file,
            config: Arc::clone(&self.config),
        }))
    }

    fn rename(&self, from: &Path, to: &Path) -> io::Result<()> {
        self.check_op()?;
        self.inner.rename(from, to)
    }

    fn remove_file(&self, p: &Path) -> io::Result<()> {
        self.check_op()?;
        self.inner.remove_file(p)
    }

    fn remove_dir(&self, p: &Path) -> io::Result<()> {
        self.check_op()?;
        self.inner.remove_dir(p)
    }

    fn list_dir(&self, p: &Path) -> io::Result<Vec<DirEntry>> {
        self.check_op()?;
        self.inner.list_dir(p)
    }

    fn sync_dir(&self, p: &Path) -> io::Result<()> {
        if self.config.fail_sync.load(Ordering::Relaxed) {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Injected sync failure",
            ));
        }
        self.inner.sync_dir(p)
    }

    fn lock(&self, p: &Path, mode: LockMode) -> io::Result<Box<dyn LockGuard>> {
        self.check_op()?;
        self.inner.lock(p, mode)
    }

    fn caps(&self) -> FsCaps {
        self.inner.caps()
    }

    fn hard_link(&self, from: &Path, to: &Path) -> io::Result<()> {
        self.check_op()?;
        self.inner.hard_link(from, to)
    }
}
