//! # Test-Hooks
//!
//! Deterministische Test-Hooks nach SPEC §4, §18 und IMPL §6 WP-1.3.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use contextra_storage_ports::{HookPoint, Hooks};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Aufzeichnender Test-Hook für deterministische Synchronisation und Fehlerprüfungen.
#[derive(Default, Clone)]
pub struct TestHooks {
    counts: Arc<Mutex<HashMap<HookPoint, usize>>>,
}

impl TestHooks {
    /// Erstellt eine neue Instanz von `TestHooks`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Liefert die Anzahl der Aufrufe für einen bestimmten Hook-Punkt.
    pub fn count(&self, point: HookPoint) -> usize {
        let counts = self.counts.lock().unwrap();
        counts.get(&point).copied().unwrap_or(0)
    }

    /// Setzt alle Aufzeichnungszähler zurück.
    pub fn reset(&self) {
        let mut counts = self.counts.lock().unwrap();
        counts.clear();
    }
}

impl Hooks for TestHooks {
    fn point(&self, p: HookPoint) {
        let mut counts = self.counts.lock().unwrap();
        *counts.entry(p).or_insert(0) += 1;
    }
}
