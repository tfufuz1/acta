//! # Zentrales Datenbankhandle für Python (`PyActa`)
//!
//! Dieses Modul definiert `PyActa` (in Python als `Db` exponiert), das primäre
//! Objekt für Datenbankinteraktionen, Collection-Verwaltung und globale Statistiken.
//!
//! ## Ring-Zugehörigkeit
//! Ring 4 (FFI-Boundary / Bindings).
//!
//! ## Architektur
//! `PyActa` fungiert gleichzeitig als:
//! 1. **Collection-Manager:** Öffnen, Auflisten und Löschen von Namespaces/Collections.
//! 2. **Default-Collection-Fassade:** Durch Einbindung von `acta_crud_methods!` und
//!    `acta_batch_methods!` können CRUD-Methoden (`insert`, `search`, etc.) direkt
//!    auf der `Db`-Instanz aufgerufen werden (sie operieren dann auf der internen Standard-Collection).
//! 3. **Python Context Manager:** Unterstützt `with acta.open(...) as db:`, wobei beim
//!    Verlassen des Blocks in `__exit__` automatisch ein `flush()` auf die Festplatte erfolgt.

use acta_db::Acta;
use numpy::PyReadonlyArray1;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::runtime::Runtime;

use crate::bindings::collection::PyCollection;
use crate::bindings::common::*;
use crate::bindings::db_stats::PyDbStats;
use crate::bindings::document::PyDocument;
use crate::bindings::search_result::PySearchResult;
use crate::bindings::storage_stats::PyStorageStats;
use crate::bindings::vector_index_stats::PyVectorIndexStats;

// ─── PyActa (Database Facade) ────────────────────────────────────────────

// ANCHOR[acta-py:DB-001]
/// Zentrales Python-Handle für eine geöffnete Acta-Datenbankinstanz (`Db`).
///
/// # Felder
/// - `inner`: Thread-sichere Referenz auf den Rust-Datenbankkern (`Acta`).
/// - `runtime`: Asynchrone Tokio-Multi-Thread-Laufzeitumgebung.
/// - `worker_threads`: Konfigurierte Thread-Anzahl im Tokio-Pool.
/// - `poisoned`: Vergiftungsindikator zur Verhinderung von Folgeaufrufen nach Panics.
#[pyclass(name = "Db")]
pub struct PyActa {
    pub(crate) inner: Arc<Acta>,
    pub(crate) runtime: Arc<Runtime>,
    pub(crate) worker_threads: usize,
    pub(crate) poisoned: Arc<AtomicBool>,
}

impl PyActa {
    /// Konstruiert ein neues `PyActa`-Handle.
    pub fn new(
        inner: Arc<Acta>,
        runtime: Arc<Runtime>,
        worker_threads: usize,
        poisoned: Arc<AtomicBool>,
    ) -> Self {
        Self {
            inner,
            runtime,
            worker_threads,
            poisoned,
        }
    }
}

#[pymethods]
impl PyActa {
    // ANCHOR[acta-py:DB-002]
    /// Gibt die konfigurierte Anzahl an Tokio-Worker-Threads für diese Instanz zurück.
    #[getter]
    pub fn worker_threads(&self) -> usize {
        self.worker_threads
    }

    // ANCHOR[acta-py:DB-003]
    /// Gibt `True` zurück, wenn die Datenbankinstanz nach einem vorangegangenen Rust-Panic vergiftet ist.
    #[getter]
    pub fn is_poisoned(&self) -> bool {
        self.poisoned.load(Ordering::SeqCst)
    }

    // ANCHOR[acta-py:DB-004]
    /// Interner Testhook zur Verifikation der FFI-Panik-Isolation.
    ///
    /// # Panics
    /// Test-Panic wird ausgelöst und durch `run_blocking_ffi` kontrolliert in `PyErr` gewandelt.
    #[pyo3(signature = (message=None))]
    pub fn _trigger_panic_for_test(&self, py: Python<'_>, message: Option<String>) -> PyResult<()> {
        crate::bindings::functions::trigger_panic_helper(py, &self.poisoned, message)
    }

    // ── Context Manager Protocol ──

    // ANCHOR[acta-py:DB-005]
    /// Tritt in den Python Context-Manager ein (`with db:`). Liefert `self`.
    fn __enter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    // ANCHOR[acta-py:DB-006]
    /// Verlässt den Context-Manager und führt einen synchronen `flush()` aller ausstehenden Schreiboperationen aus.
    ///
    /// # Fehlerbehandlung
    /// Unterdrückt keine aufgetretenen Exceptions (liefert `Ok(false)`).
    #[pyo3(signature = (_exc_type=None, _exc_val=None, _exc_tb=None))]
    fn __exit__(
        &self,
        py: Python<'_>,
        _exc_type: Option<&Bound<'_, PyAny>>,
        _exc_val: Option<&Bound<'_, PyAny>>,
        _exc_tb: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<bool> {
        let rt = &self.runtime;
        run_blocking_ffi(py, &self.poisoned, || {
            rt.block_on(self.inner.flush()).map_err(acta_err)
        })?;
        Ok(false) // Do not suppress exceptions
    }

    // ── Collection Management ──

    // ANCHOR[acta-py:DB-007]
    /// Öffnet oder erstellt eine spezifische Collection (Namespace) innerhalb der Datenbank.
    ///
    /// # Parameter
    /// - `name`: Name der Collection (maximal 64 Zeichen).
    ///
    /// # Rückgabe
    /// Eine neue `PyCollection`-Instanz, die auf diesen Namespace zeigt.
    pub fn collection(&self, name: &str, py: Python<'_>) -> PyResult<PyCollection> {
        validate_collection_name(name)?;
        let rt = &self.runtime;
        let name_owned = name.to_string();
        let col = run_blocking_ffi(py, &self.poisoned, || {
            rt.block_on(self.inner.collection(&name_owned))
                .map_err(acta_err)
        })?;
        Ok(PyCollection::new(
            col,
            self.runtime.clone(),
            self.poisoned.clone(),
        ))
    }

    // ANCHOR[acta-py:DB-008]
    /// Liefert eine Liste der Namen aller vorhandenen Collections in der Datenbank.
    pub fn list_collections(&self, py: Python<'_>) -> PyResult<Vec<String>> {
        let rt = &self.runtime;
        run_blocking_ffi(py, &self.poisoned, || {
            rt.block_on(self.inner.list_collections())
                .map_err(acta_err)
        })
    }

    // ANCHOR[acta-py:DB-009]
    /// Löscht eine Collection und entfernt alle darin enthaltenen Daten unwiderruflich aus dem Storage.
    ///
    /// # Parameter
    /// - `name`: Name der zu entfernenden Collection.
    pub fn drop_collection(&self, name: &str, py: Python<'_>) -> PyResult<()> {
        validate_collection_name(name)?;
        let rt = &self.runtime;
        let name_owned = name.to_string();
        let tenant_id = acta_types::TenantId::SYSTEM;
        run_blocking_ffi(py, &self.poisoned, || {
            rt.block_on(
                self.inner
                    .drop_collection(&name_owned, tenant_id, &[0u8; 32]),
            )
            .map(|_| ())
            .map_err(acta_err)
        })
    }

    // ANCHOR[acta-py:DB-010]
    /// Schreibt alle im Speicher gepufferten Daten (MemTable/WAL) persistent auf die Festplatte (`fsync`).
    pub fn flush(&self, py: Python<'_>) -> PyResult<()> {
        let rt = &self.runtime;
        run_blocking_ffi(py, &self.poisoned, || {
            rt.block_on(self.inner.flush()).map_err(acta_err)
        })
    }

    // ANCHOR[acta-py:DB-011]
    /// Ruft kombinierte Statistiken für Vektorindex, LSM-Tree, Kalibrierung und Drift ab.
    pub fn stats(&self, py: Python<'_>) -> PyResult<PyDbStats> {
        let rt = &self.runtime;
        let stats = run_blocking_ffi(py, &self.poisoned, || {
            rt.block_on(self.inner.stats()).map_err(acta_err)
        })?;

        Ok(PyDbStats {
            drift_status: stats.drift_status,
            calibration_ece: stats.calibration_ece,
            last_calibration_at: stats.last_calibration_at,
            active_memory_count: stats.active_memory_count,
            pid_pool_size: stats.pid_pool_size,
            index_stats: PyVectorIndexStats {
                num_vectors: stats.index_stats.num_vectors,
                memory_usage_bytes: stats.index_stats.memory_usage_bytes,
                num_layers: stats.index_stats.num_layers,
            },
            storage_stats: PyStorageStats {
                num_segments: stats.storage_stats.num_segments,
                total_size_bytes: stats.storage_stats.total_size_bytes,
                memtable_size_bytes: stats.storage_stats.memtable_size_bytes,
            },
        })
    }

    // ANCHOR[acta-py:DB-012]
    /// Gibt die Anzahl der Dokumente in der Standard-Collection zurück.
    pub fn len(&self, py: Python<'_>) -> PyResult<usize> {
        let rt = &self.runtime;
        run_blocking_ffi(py, &self.poisoned, || {
            rt.block_on(self.inner.len()).map_err(acta_err)
        })
    }

    // ANCHOR[acta-py:DB-013]
    /// Prüft, ob die Standard-Collection leer ist.
    pub fn is_empty(&self, py: Python<'_>) -> PyResult<bool> {
        let rt = &self.runtime;
        run_blocking_ffi(py, &self.poisoned, || {
            rt.block_on(self.inner.is_empty()).map_err(acta_err)
        })
    }
}

// ── Generated CRUD + Batch Methods ──
acta_crud_methods!(PyActa);
acta_batch_methods!(PyActa);
