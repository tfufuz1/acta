//! # Synchronization Fassade
//!
//! Kapselt Sperren über `std::sync` (unter `cfg(loom)`: `loom::sync`).
//! Dies ist die einzige Stelle im gesamten Crate mit `cfg(loom)`.
//!
//! Unter `cfg(debug_assertions)` erzwingt `Ranked<T, RANK>` die Einhaltung
//! der Sperrhierarchie (INV-LOCK-1) zur Laufzeit.

#![allow(clippy::indexing_slicing)]
#![allow(clippy::panic)]
#![allow(clippy::collapsible_if)]

#[cfg(not(loom))]
pub use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
#[cfg(not(loom))]
pub use std::sync::{Arc, Condvar, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};

#[cfg(loom)]
pub use loom::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
#[cfg(loom)]
pub use loom::sync::{Arc, Condvar, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};

#[cfg(debug_assertions)]
use std::cell::RefCell;

#[cfg(debug_assertions)]
thread_local! {
    static LOCK_RANK_STACK: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
}

/// Prüft, ob ein Sperrrang erworben werden darf (INV-LOCK-1).
#[cfg(debug_assertions)]
fn push_rank(rank: u8) {
    LOCK_RANK_STACK.with(|stack| {
        let mut s = stack.borrow_mut();
        if let Some(&last) = s.last() {
            if last >= rank {
                panic!(
                    // INV-LOCK-1: Sperren nur in aufsteigender Rangfolge
                    "INV-LOCK-1: Lock-Rang-Verletzung! Aktuell gehalten: {}, angefordert: {}",
                    last, rank
                );
            }
        }
        s.push(rank);
    });
}

#[cfg(debug_assertions)]
fn pop_rank(rank: u8) {
    LOCK_RANK_STACK.with(|stack| {
        let mut s = stack.borrow_mut();
        if let Some(top) = s.pop() {
            if top != rank {
                panic!(
                    "INV-LOCK-1: Lock-Rang-Stack inkonsistent! Erwartet: {}, gefunden: {}",
                    rank, top
                );
            }
        }
    });
}

/// Typisierte Sperre mit festem Rang (INV-LOCK-1).
pub struct RankedMutex<T, const RANK: u8> {
    inner: Mutex<T>,
}

impl<T, const RANK: u8> RankedMutex<T, RANK> {
    /// Erstellt einen neuen typisierten Mutex.
    pub fn new(val: T) -> Self {
        Self {
            inner: Mutex::new(val),
        }
    }

    /// Erwirbt die Sperre und prüft im Debug-Modus den Rang.
    pub fn lock(&self) -> RankedMutexGuard<'_, T, RANK> {
        #[cfg(debug_assertions)]
        push_rank(RANK);

        let guard = match self.inner.lock() {
            Ok(g) => g,
            Err(e) => {
                #[cfg(debug_assertions)]
                pop_rank(RANK);
                panic!("Lock poisoned: {:?}", e);
            }
        };

        RankedMutexGuard {
            _guard: Some(guard),
        }
    }

    /// Wartet atomar auf einer Condvar unter Beachtung des Rangstacks.
    pub fn wait<'a>(
        &self,
        cond: &Condvar,
        mut guard: RankedMutexGuard<'a, T, RANK>,
    ) -> RankedMutexGuard<'a, T, RANK> {
        let inner = guard._guard.take().unwrap();
        #[cfg(debug_assertions)]
        pop_rank(RANK);

        let next_inner = match cond.wait(inner) {
            Ok(g) => g,
            Err(e) => panic!("Condvar wait poisoned: {:?}", e),
        };

        #[cfg(debug_assertions)]
        push_rank(RANK);

        RankedMutexGuard {
            _guard: Some(next_inner),
        }
    }

    /// Wartet mit Zeitüberschreitung atomar auf einer Condvar.
    #[cfg(not(loom))]
    pub fn wait_timeout<'a>(
        &self,
        cond: &Condvar,
        mut guard: RankedMutexGuard<'a, T, RANK>,
        dur: std::time::Duration,
    ) -> (RankedMutexGuard<'a, T, RANK>, std::sync::WaitTimeoutResult) {
        let inner = guard._guard.take().unwrap();
        #[cfg(debug_assertions)]
        pop_rank(RANK);

        let (next_inner, res) = match cond.wait_timeout(inner, dur) {
            Ok(pair) => pair,
            Err(e) => panic!("Condvar wait_timeout poisoned: {:?}", e),
        };

        #[cfg(debug_assertions)]
        push_rank(RANK);

        (
            RankedMutexGuard {
                _guard: Some(next_inner),
            },
            res,
        )
    }
}

/// Guard für einen `RankedMutex`.
pub struct RankedMutexGuard<'a, T, const RANK: u8> {
    _guard: Option<MutexGuard<'a, T>>,
}

impl<T, const RANK: u8> std::ops::Deref for RankedMutexGuard<'_, T, RANK> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &**self._guard.as_ref().unwrap()
    }
}

impl<T, const RANK: u8> std::ops::DerefMut for RankedMutexGuard<'_, T, RANK> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut **self._guard.as_mut().unwrap()
    }
}

impl<T, const RANK: u8> Drop for RankedMutexGuard<'_, T, RANK> {
    fn drop(&mut self) {
        if self._guard.is_some() {
            #[cfg(debug_assertions)]
            pop_rank(RANK);
        }
    }
}

/// Typisierte RwLock mit festem Rang (INV-LOCK-1).
pub struct RankedRwLock<T, const RANK: u8> {
    inner: RwLock<T>,
}

impl<T, const RANK: u8> RankedRwLock<T, RANK> {
    /// Erstellt eine neue typisierte RwLock.
    pub fn new(val: T) -> Self {
        Self {
            inner: RwLock::new(val),
        }
    }

    /// Erwirbt die Lesesperre.
    pub fn read(&self) -> RankedRwLockReadGuard<'_, T, RANK> {
        #[cfg(debug_assertions)]
        push_rank(RANK);

        let guard = match self.inner.read() {
            Ok(g) => g,
            Err(e) => {
                #[cfg(debug_assertions)]
                pop_rank(RANK);
                panic!("RwLock read poisoned: {:?}", e);
            }
        };

        RankedRwLockReadGuard { _guard: guard }
    }

    /// Erwirbt die Schreibsperre.
    pub fn write(&self) -> RankedRwLockWriteGuard<'_, T, RANK> {
        #[cfg(debug_assertions)]
        push_rank(RANK);

        let guard = match self.inner.write() {
            Ok(g) => g,
            Err(e) => {
                #[cfg(debug_assertions)]
                pop_rank(RANK);
                panic!("RwLock write poisoned: {:?}", e);
            }
        };

        RankedRwLockWriteGuard { _guard: guard }
    }
}

/// Leseguard für eine `RankedRwLock`.
pub struct RankedRwLockReadGuard<'a, T, const RANK: u8> {
    _guard: RwLockReadGuard<'a, T>,
}

impl<T, const RANK: u8> std::ops::Deref for RankedRwLockReadGuard<'_, T, RANK> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self._guard
    }
}

impl<T, const RANK: u8> Drop for RankedRwLockReadGuard<'_, T, RANK> {
    fn drop(&mut self) {
        #[cfg(debug_assertions)]
        pop_rank(RANK);
    }
}

/// Schreibguard für eine `RankedRwLock`.
pub struct RankedRwLockWriteGuard<'a, T, const RANK: u8> {
    _guard: RwLockWriteGuard<'a, T>,
}

impl<T, const RANK: u8> std::ops::Deref for RankedRwLockWriteGuard<'_, T, RANK> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self._guard
    }
}

impl<T, const RANK: u8> std::ops::DerefMut for RankedRwLockWriteGuard<'_, T, RANK> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self._guard
    }
}

impl<T, const RANK: u8> Drop for RankedRwLockWriteGuard<'_, T, RANK> {
    fn drop(&mut self) {
        #[cfg(debug_assertions)]
        pop_rank(RANK);
    }
}
