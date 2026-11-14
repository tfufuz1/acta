//! P0-2 / Befund F2 — Regressionstest.
//!
//! Fehlerfall: Der Flush hat die SST über `manifest.append_batch` committet und
//! schlägt danach am abschließenden `sync_dir(data_dir)` fehl (nach dem Entfernen
//! der versiegelten WAL-Dateien). Früher löschte der Fehlerpfad die SST, obwohl das
//! Manifest sie referenzierte. Die WAL war zu diesem Zeitpunkt bereits weg, also
//! ging der Datensatz verloren.
//!
//! Erwartung: Nach dem Fehler bleibt die SST bestehen, und nach einem Neustart ist
//! der Schlüssel lesbar.

#![forbid(unsafe_code)]

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;

use acta_store::io_seam::{RealFs, StoreIo, Vfs, VfsFile, VfsOpenOptions};
use acta_store::lsm::{LsmConfig, LsmStorage};
use acta_store::{StoreError, StoreResult};
use acta_ids::TxId;
use bytes::Bytes;
use std::time::Duration;
use tempfile::TempDir;

/// Vfs-Wrapper um `RealFs`, der den `fail_at`-ten `sync_dir`-Aufruf (1-basiert)
/// nach dem Scharfschalten fehlschlagen lässt.
struct SyncDirFault {
    inner: RealFs,
    armed: AtomicBool,
    calls: AtomicUsize,
    fail_at: AtomicUsize,
}

impl SyncDirFault {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            inner: RealFs,
            armed: AtomicBool::new(false),
            calls: AtomicUsize::new(0),
            fail_at: AtomicUsize::new(0),
        })
    }
}

impl Vfs for SyncDirFault {
    fn open(&self, path: &Path, opts: &VfsOpenOptions) -> StoreResult<Box<dyn VfsFile>> {
        self.inner.open(path, opts)
    }
    fn write_atomic(&self, path: &Path, data: &[u8]) -> StoreResult<()> {
        self.inner.write_atomic(path, data)
    }
    fn create_dir_all(&self, path: &Path) -> StoreResult<()> {
        self.inner.create_dir_all(path)
    }
    fn remove_file(&self, path: &Path) -> StoreResult<()> {
        self.inner.remove_file(path)
    }
    fn rename(&self, from: &Path, to: &Path) -> StoreResult<()> {
        self.inner.rename(from, to)
    }
    fn read_dir(&self, path: &Path) -> StoreResult<Vec<PathBuf>> {
        self.inner.read_dir(path)
    }
    fn exists(&self, path: &Path) -> StoreResult<bool> {
        self.inner.exists(path)
    }
    fn len(&self, path: &Path) -> StoreResult<u64> {
        self.inner.len(path)
    }
    fn list_dir(&self, path: &Path) -> StoreResult<Vec<String>> {
        self.inner.list_dir(path)
    }
    fn sync_dir(&self, path: &Path) -> StoreResult<()> {
        if self.armed.load(Ordering::SeqCst) {
            let n = self.calls.fetch_add(1, Ordering::SeqCst) + 1;
            if n == self.fail_at.load(Ordering::SeqCst) {
                return Err(StoreError::Storage(format!(
                    "injected sync_dir failure at call {n}"
                )));
            }
        }
        self.inner.sync_dir(path)
    }
}

fn config_at(path: &Path, vfs: Option<StoreIo>) -> LsmConfig {
    LsmConfig {
        path: path.to_path_buf(),
        memtable_size_limit: 1024 * 1024,
        max_ram_mb: 64,
        tx_timeout: Duration::from_secs(60),
        vfs,
        ..Default::default()
    }
}

/// Zählt, wie oft ein erfolgreicher Flush `sync_dir` aufruft (Referenzlauf).
async fn count_sync_dir_calls_of_successful_flush() -> usize {
    let tmp = TempDir::new().expect("temp dir");
    let fault = SyncDirFault::new();
    let cfg = config_at(tmp.path(), Some(StoreIo::new(fault.clone())));
    let storage = LsmStorage::new(cfg).await.expect("open");

    fault.armed.store(true, Ordering::SeqCst);
    let tx = TxId::new(1);
    storage.put(tx, b"key", b"val").await.expect("put");
    storage.commit(tx).await.expect("commit");
    storage.flush().await.expect("flush must succeed in reference run");
    let calls = fault.calls.load(Ordering::SeqCst);

    storage.wait_shutdown().await;
    calls
}

#[tokio::test]
async fn p0_2_flush_failure_after_manifest_commit_keeps_sst_and_data() {
    // Der letzte sync_dir-Aufruf eines Flushs liegt nach dem Manifest-Commit
    // (WAL-Entfernung + sync_dir). Genau diesen Aufruf lassen wir scheitern.
    let total = count_sync_dir_calls_of_successful_flush().await;
    assert!(total >= 2, "expected flush to call sync_dir at least twice, got {total}");

    let tmp = TempDir::new().expect("temp dir");
    let fault = SyncDirFault::new();
    let cfg = config_at(tmp.path(), Some(StoreIo::new(fault.clone())));
    {
        let storage = LsmStorage::new(cfg.clone()).await.expect("open");
        fault.fail_at.store(total, Ordering::SeqCst);
        fault.armed.store(true, Ordering::SeqCst);

        let tx = TxId::new(1);
        storage.put(tx, b"key", b"val").await.expect("put");
        storage.commit(tx).await.expect("commit");

        let res = storage.flush().await;
        assert!(res.is_err(), "flush must surface the injected sync_dir failure");
        storage.wait_shutdown().await;
    }

    // Die SST muss nach dem Fehler noch vorhanden sein.
    let ssts: Vec<_> = std::fs::read_dir(tmp.path())
        .expect("read data dir")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "sst"))
        .collect();
    assert!(
        !ssts.is_empty(),
        "manifested SST was deleted after flush error (F2 regression)"
    );

    // Neustart ohne Fault-Injection: Daten müssen lesbar sein.
    let recovered = LsmStorage::new(config_at(tmp.path(), None))
        .await
        .expect("recover");
    assert_eq!(
        recovered.get(b"key").await.expect("get"),
        Some(Bytes::from_static(b"val")),
        "data lost after flush error + restart (F2 regression)"
    );
    recovered.wait_shutdown().await;
}
