//! Gemeinsame Hilfen für die Gate-Tests: ein im Speicher geführtes Log mit Fehlerinjektion.
//!
//! Der Zustand `pending` ist angehängt, aber nicht haltbar. Ein Absturz verwirft ihn; nur
//! `durable` überlebt. So lässt sich Z4 prüfen, ohne Dateisystem.
#![allow(dead_code, clippy::expect_used, clippy::unwrap_used)]

use std::io;
use std::sync::{Arc, Mutex};

use acta_gate::{GateKey, GateSink, GateView, GateWriter};

/// Speicherlog mit getrenntem haltbaren und angehängten Bereich.
#[derive(Default)]
pub struct Disk {
    durable: Vec<u8>,
    pending: Vec<u8>,
    pub syncs: u32,
}

pub type SharedDisk = Arc<Mutex<Disk>>;

pub fn disk() -> SharedDisk {
    Arc::new(Mutex::new(Disk::default()))
}

/// Haltbarer Stand, so wie er nach einem Absturz vorliegt.
pub fn durable(disk: &SharedDisk) -> Vec<u8> {
    disk.lock().expect("disk lock").durable.clone()
}

/// Sink mit optionaler Fehlerinjektion für Anhängen und `fsync`.
pub struct FaultSink {
    disk: SharedDisk,
    fail_append: bool,
    fail_sync: bool,
}

impl FaultSink {
    pub fn ok(disk: &SharedDisk) -> Self {
        Self {
            disk: Arc::clone(disk),
            fail_append: false,
            fail_sync: false,
        }
    }

    pub fn failing_append(disk: &SharedDisk) -> Self {
        Self {
            fail_append: true,
            ..Self::ok(disk)
        }
    }

    pub fn failing_sync(disk: &SharedDisk) -> Self {
        Self {
            fail_sync: true,
            ..Self::ok(disk)
        }
    }
}

impl GateSink for FaultSink {
    fn append(&mut self, bytes: &[u8]) -> io::Result<()> {
        if self.fail_append {
            return Err(io::Error::other("injected append failure"));
        }
        self.disk
            .lock()
            .expect("disk lock")
            .pending
            .extend_from_slice(bytes);
        Ok(())
    }

    fn sync(&mut self) -> io::Result<()> {
        let mut d = self.disk.lock().expect("disk lock");
        d.syncs += 1;
        if self.fail_sync {
            // Nach FsyncFailed ist der Inhalt nicht als stabil anzunehmen (SPEC 3.7).
            d.pending.clear();
            return Err(io::Error::other("injected fsync failure"));
        }
        let pending = std::mem::take(&mut d.pending);
        d.durable.extend_from_slice(&pending);
        Ok(())
    }
}

/// Fester Testschlüssel; jede Anwendung erhält einen eigenen.
pub fn key() -> GateKey {
    GateKey::from_bytes([7u8; 32])
}

/// Öffnet das Gate aus einem durablen Stand.
pub fn open(durable_log: &[u8], sink: FaultSink) -> (GateWriter, GateView) {
    GateWriter::recover(key(), durable_log, Box::new(sink)).expect("recover")
}

/// Frisches Gate über einem neuen Speicherlog.
pub fn fresh(disk: &SharedDisk) -> (GateWriter, GateView) {
    open(&[], FaultSink::ok(disk))
}
