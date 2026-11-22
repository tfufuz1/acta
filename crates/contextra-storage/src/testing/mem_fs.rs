//! # In-Memory Dateisystem (MemFs)
//!
//! Deterministisches Dateisystem für Tests mit Trennung von Page Cache und dauerhaftem Zustand
//! nach SPEC §4, §18 und IMPL §6 WP-1.3.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]

use contextra_storage_ports::{
    DirEntry, File, Fs, FsCaps, LockGuard, LockMode, OpenMode, Perm,
};
use std::collections::{BTreeMap, BTreeSet};
use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
struct FileContent {
    data: Vec<u8>,
    durable: Vec<u8>,
    #[allow(dead_code)]
    perm: Perm,
}

struct MemFileHandle {
    content: Arc<RwLock<FileContent>>,
}

impl File for MemFileHandle {
    fn read_at(&self, buf: &mut [u8], off: u64) -> io::Result<usize> {
        let inner = self.content.read().unwrap();
        let off_usize = usize::try_from(off).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        if off_usize >= inner.data.len() {
            return Ok(0);
        }
        let available = &inner.data[off_usize..];
        let to_copy = std::cmp::min(buf.len(), available.len());
        buf[..to_copy].copy_from_slice(&available[..to_copy]);
        Ok(to_copy)
    }

    fn write_all_at(&self, buf: &[u8], off: u64) -> io::Result<()> {
        let mut inner = self.content.write().unwrap();
        let off_usize = usize::try_from(off).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        let end_usize = off_usize.checked_add(buf.len()).ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "Offset-Überlauf")
        })?;
        if inner.data.len() < end_usize {
            inner.data.resize(end_usize, 0);
        }
        inner.data[off_usize..end_usize].copy_from_slice(buf);
        Ok(())
    }

    fn len(&self) -> io::Result<u64> {
        let inner = self.content.read().unwrap();
        Ok(inner.data.len() as u64)
    }

    fn set_len(&self, len: u64) -> io::Result<()> {
        let mut inner = self.content.write().unwrap();
        let len_usize = usize::try_from(len).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        inner.data.resize(len_usize, 0);
        Ok(())
    }

    fn sync_data(&self) -> io::Result<()> {
        let mut inner = self.content.write().unwrap();
        inner.durable = inner.data.clone();
        Ok(())
    }

    fn sync_all(&self) -> io::Result<()> {
        self.sync_data()
    }
}

struct Inner {
    files: BTreeMap<PathBuf, Arc<RwLock<FileContent>>>,
    durable_files: BTreeSet<PathBuf>,
    dirs: BTreeSet<PathBuf>,
    durable_dirs: BTreeSet<PathBuf>,
    locks: BTreeSet<PathBuf>,
}

/// In-Memory Dateisystem für Tests.
#[derive(Clone)]
pub struct MemFs {
    inner: Arc<RwLock<Inner>>,
}

impl Default for MemFs {
    fn default() -> Self {
        Self::new()
    }
}

impl MemFs {
    /// Erstellt ein leeres `MemFs`.
    pub fn new() -> Self {
        let mut dirs = BTreeSet::new();
        dirs.insert(PathBuf::from("/"));
        dirs.insert(PathBuf::from("."));
        Self {
            inner: Arc::new(RwLock::new(Inner {
                files: BTreeMap::new(),
                durable_files: BTreeSet::new(),
                dirs: dirs.clone(),
                durable_dirs: dirs,
                locks: BTreeSet::new(),
            })),
        }
    }

    /// Simuliert einen Absturz: Verwirft alle nicht-persistierten Datei- und Verzeichnisänderungen.
    pub fn crash(&self) {
        let mut inner = self.inner.write().unwrap();
        // Locks freigeben
        inner.locks.clear();
        // Nicht persistierte Dateien entfernen
        let durable_files = inner.durable_files.clone();
        inner.files.retain(|p, _| durable_files.contains(p));
        // Persistierte Dateien auf ihren durable Stand zurücksetzen
        for file in inner.files.values() {
            let mut f = file.write().unwrap();
            f.data = f.durable.clone();
        }
        // Verzeichnisse auf durable Stand zurücksetzen
        inner.dirs = inner.durable_dirs.clone();
    }
}

struct MemLockGuard {
    path: PathBuf,
    inner: Arc<RwLock<Inner>>,
}

impl LockGuard for MemLockGuard {}

impl Drop for MemLockGuard {
    fn drop(&mut self) {
        let mut inner = self.inner.write().unwrap();
        inner.locks.remove(&self.path);
    }
}

impl Fs for MemFs {
    fn create_dir_all(&self, p: &Path, _perm: Perm) -> io::Result<()> {
        let mut inner = self.inner.write().unwrap();
        let mut curr = PathBuf::new();
        for comp in p.components() {
            curr.push(comp);
            inner.dirs.insert(curr.clone());
        }
        Ok(())
    }

    fn open(&self, p: &Path, mode: OpenMode) -> io::Result<Box<dyn File>> {
        let mut inner = self.inner.write().unwrap();
        match mode {
            OpenMode::Read => {
                let file = inner.files.get(p).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::NotFound, "Datei nicht gefunden")
                })?;
                Ok(Box::new(MemFileHandle {
                    content: Arc::clone(file),
                }))
            }
            OpenMode::ReadWrite => {
                let file = inner.files.get(p).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::NotFound, "Datei nicht gefunden")
                })?;
                Ok(Box::new(MemFileHandle {
                    content: Arc::clone(file),
                }))
            }
            OpenMode::CreateNew(perm) => {
                if inner.files.contains_key(p) {
                    return Err(io::Error::new(
                        io::ErrorKind::AlreadyExists,
                        "Datei existiert bereits",
                    ));
                }
                let handle = Arc::new(RwLock::new(FileContent {
                    data: Vec::new(),
                    durable: Vec::new(),
                    perm,
                }));
                inner.files.insert(p.to_path_buf(), Arc::clone(&handle));
                Ok(Box::new(MemFileHandle { content: handle }))
            }
        }
    }

    fn rename(&self, from: &Path, to: &Path) -> io::Result<()> {
        let mut inner = self.inner.write().unwrap();
        let handle = inner.files.remove(from).ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, "Quelldatei nicht gefunden")
        })?;
        inner.files.insert(to.to_path_buf(), handle);
        Ok(())
    }

    fn remove_file(&self, p: &Path) -> io::Result<()> {
        let mut inner = self.inner.write().unwrap();
        inner.files.remove(p).ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, "Datei nicht gefunden")
        })?;
        Ok(())
    }

    fn remove_dir(&self, p: &Path) -> io::Result<()> {
        let mut inner = self.inner.write().unwrap();
        if !inner.dirs.remove(p) {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Verzeichnis nicht gefunden",
            ));
        }
        Ok(())
    }

    fn list_dir(&self, p: &Path) -> io::Result<Vec<DirEntry>> {
        let inner = self.inner.read().unwrap();
        let mut entries = Vec::new();
        for (f_path, handle) in &inner.files {
            if let Some(parent) = f_path.parent() {
                if parent == p {
                    if let Some(name) = f_path.file_name().and_then(|n| n.to_str()) {
                        let len = handle.read().unwrap().data.len() as u64;
                        entries.push(DirEntry {
                            name: name.to_string(),
                            len,
                        });
                    }
                }
            }
        }
        entries.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(entries)
    }

    fn sync_dir(&self, p: &Path) -> io::Result<()> {
        let mut inner = self.inner.write().unwrap();
        inner.durable_dirs.insert(p.to_path_buf());
        // Nur Dateien und Verzeichnisse, die direkt in `p` liegen, dauerhaft machen (SPEC §19.2)
        let to_add_files: Vec<PathBuf> = inner
            .files
            .keys()
            .filter(|f_path| match f_path.parent() {
                Some(parent) => parent == p || (p == Path::new(".") && parent == Path::new("")),
                None => p == Path::new(".") || p == Path::new(""),
            })
            .cloned()
            .collect();
        for f in to_add_files {
            inner.durable_files.insert(f);
        }

        let existing_files: BTreeSet<PathBuf> = inner.files.keys().cloned().collect();
        inner.durable_files.retain(|f_path| {
            let in_dir = match f_path.parent() {
                Some(parent) => parent == p || (p == Path::new(".") && parent == Path::new("")),
                None => p == Path::new(".") || p == Path::new(""),
            };
            if in_dir {
                existing_files.contains(f_path)
            } else {
                true
            }
        });

        let to_add_dirs: Vec<PathBuf> = inner
            .dirs
            .iter()
            .filter(|d| match d.parent() {
                Some(parent) => parent == p || (p == Path::new(".") && parent == Path::new("")),
                None => p == Path::new(".") || p == Path::new(""),
            })
            .cloned()
            .collect();
        for d in to_add_dirs {
            inner.durable_dirs.insert(d);
        }

        let existing_dirs = inner.dirs.clone();
        inner.durable_dirs.retain(|d| {
            let in_dir = match d.parent() {
                Some(parent) => parent == p || (p == Path::new(".") && parent == Path::new("")),
                None => p == Path::new(".") || p == Path::new(""),
            };
            if in_dir {
                existing_dirs.contains(d)
            } else {
                true
            }
        });
        Ok(())
    }

    fn lock(&self, p: &Path, _mode: LockMode) -> io::Result<Box<dyn LockGuard>> {
        let mut inner = self.inner.write().unwrap();
        if inner.locks.contains(p) {
            return Err(io::Error::new(
                io::ErrorKind::WouldBlock,
                "Dateisperre bereits gehalten",
            ));
        }
        inner.locks.insert(p.to_path_buf());
        Ok(Box::new(MemLockGuard {
            path: p.to_path_buf(),
            inner: Arc::clone(&self.inner),
        }))
    }

    fn caps(&self) -> FsCaps {
        FsCaps {
            durable_file_sync: true,
            durable_dir_sync: true,
            atomic_rename: true,
            private_perm: true,
            no_follow_root: true,
        }
    }

    fn hard_link(&self, from: &Path, to: &Path) -> io::Result<()> {
        let mut inner = self.inner.write().unwrap();
        let handle = inner.files.get(from).ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, "Quelldatei nicht gefunden")
        })?;
        let clone_handle = Arc::clone(handle);
        inner.files.insert(to.to_path_buf(), clone_handle);
        Ok(())
    }
}
