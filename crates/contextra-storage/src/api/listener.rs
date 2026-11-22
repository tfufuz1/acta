//! # Listener-Benachrichtigungen
//!
//! Normative Referenzen: SPEC §7.4, §16.1 (INV-LOCK-1), SD-62.
//! Keine Integer-Casts (M-02 = 0).

use std::collections::VecDeque;
use std::fmt;
use crate::codec::tag::TagKind;
use crate::sync::{Arc, RankedMutex};

/// Zusammenfassung einer einzelnen Operation für Listener.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpSummary {
    /// Space-Kennung.
    pub space: u16,
    /// Art der Operation (Put, Delete, RangeDelete, EraseRange).
    pub kind: TagKind,
    /// Nutzerschlüssel.
    pub key: Vec<u8>,
    /// Header-Bytes.
    pub header: Vec<u8>,
}

/// Ereignis über einen erfolgreich festgeschriebenen Change.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeEvent {
    /// Eindeutiger Stamp des Changes.
    pub stamp: u64,
    /// Liste der Operationen in diesem Change.
    pub ops: Vec<OpSummary>,
}

/// Fehler beim Empfang oder der Zustellung an Listener.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListenerError {
    /// Listener hat Ereignisse verpasst (SD-62).
    Lagged {
        /// Erster verpasster Stamp.
        missed_from: u64,
    },
    /// Zustellung fehlgeschlagen oder Listener meldet Fehler.
    Failed(String),
}

impl fmt::Display for ListenerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lagged { missed_from } => {
                write!(
                    f,
                    "ListenerLagged: Ereignisse ab Stamp {} verpasst",
                    missed_from
                )
            }
            Self::Failed(msg) => write!(f, "ListenerFailed: {}", msg),
        }
    }
}

impl std::error::Error for ListenerError {}

/// Empfänger von Change-Ereignissen.
pub trait Listener: Send + Sync {
    /// Wird bei jedem sichtbaren Change in Stamp-Reihenfolge aufgerufen.
    fn on_change(&self, event: &ChangeEvent) -> Result<(), ListenerError>;
}

/// Konfiguration für einen registrierten Listener.
#[derive(Debug, Clone)]
pub struct ListenerConfig {
    /// Maximale Warteschlangengröße vor Überlauf (Standard: 1024).
    pub queue_limit: usize,
    /// Maximale Frist für die Zustellung in Millisekunden (Standard: 5000).
    pub deadline_ms: u64,
}

impl Default for ListenerConfig {
    fn default() -> Self {
        Self {
            queue_limit: 1024,
            deadline_ms: 5000,
        }
    }
}

/// Zustand eines einzelnen Abonnenten.
struct Subscriber {
    id: u64,
    listener: Arc<dyn Listener>,
    config: ListenerConfig,
    queue: VecDeque<ChangeEvent>,
    lagged: Option<u64>,
    active: bool,
}

/// Zustand der Listener-Warteschlange.
struct ListenerQueueState {
    next_id: u64,
    subscribers: Vec<Subscriber>,
}

/// Warteschlange und Verwaltung aller Listener unter Rang 7 Sperre (INV-LOCK-1).
pub struct ListenerQueue {
    inner: RankedMutex<ListenerQueueState, 7>,
}

impl Default for ListenerQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl ListenerQueue {
    /// Erstellt eine neue Listener-Warteschlange.
    pub fn new() -> Self {
        Self {
            inner: RankedMutex::new(ListenerQueueState {
                next_id: 1,
                subscribers: Vec::new(),
            }),
        }
    }

    /// Registriert einen neuen Listener.
    pub fn subscribe(&self, listener: Arc<dyn Listener>, config: ListenerConfig) -> u64 {
        let mut guard = self.inner.lock();
        let id = guard.next_id;
        guard.next_id = guard.next_id.saturating_add(1);

        guard.subscribers.push(Subscriber {
            id,
            listener,
            config,
            queue: VecDeque::new(),
            lagged: None,
            active: true,
        });

        id
    }

    /// Meldet einen Listener ab.
    pub fn unsubscribe(&self, id: u64) {
        let mut guard = self.inner.lock();
        guard.subscribers.retain(|s| s.id != id);
    }

    /// Reiht ein neues Ereignis in alle aktiven Listener-Warteschlangen ein (SPEC §10.2 Schritt 11).
    ///
    /// SD-62: Läuft die Warteschlange eines Listeners über, wird er abgemeldet und
    /// als `lagged` markiert. Der Commit-Pfad wird NIEMALS blockiert.
    pub fn push_event(&self, event: ChangeEvent) {
        let mut guard = self.inner.lock();

        for sub in &mut guard.subscribers {
            if !sub.active {
                continue;
            }

            if sub.queue.len() >= sub.config.queue_limit {
                // Warteschlange voll: Abmelden (SD-62)
                sub.active = false;
                if sub.lagged.is_none() {
                    sub.lagged = Some(event.stamp);
                }
                sub.queue.clear();
            } else {
                sub.queue.push_back(event.clone());
            }
        }
    }

    /// Prüft den Status eines Listeners und liefert ggf. `ListenerLagged` zurück (SD-62).
    pub fn poll_status(&self, id: u64) -> Result<(), ListenerError> {
        let guard = self.inner.lock();
        if let Some(sub) = guard.subscribers.iter().find(|s| s.id == id) {
            if let Some(missed) = sub.lagged {
                return Err(ListenerError::Lagged {
                    missed_from: missed,
                });
            }
        }
        Ok(())
    }

    /// Führt die Zustellung ausstehender Ereignisse ohne gehaltene Sperre aus (P-10).
    pub fn dispatch_pending(&self) {
        // 1. Hole anstehende Ereignisse unter kurzem Lock (Rang 7)
        let pending = {
            let mut guard = self.inner.lock();
            let mut items = Vec::new();
            for sub in &mut guard.subscribers {
                if sub.active && !sub.queue.is_empty() {
                    let events: Vec<ChangeEvent> = sub.queue.drain(..).collect();
                    items.push((sub.id, Arc::clone(&sub.listener), events));
                }
            }
            items
        };

        // 2. Zustellung OHNE gehaltene Sperre
        for (id, listener, events) in pending {
            for event in events {
                let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    listener.on_change(&event)
                }));

                match res {
                    Ok(Ok(())) => {}
                    _ => {
                        // Fehler oder Panik: Abmelden (SD-62, F-12)
                        let mut guard = self.inner.lock();
                        if let Some(sub) = guard.subscribers.iter_mut().find(|s| s.id == id) {
                            sub.active = false;
                            if sub.lagged.is_none() {
                                sub.lagged = Some(event.stamp);
                            }
                        }
                        break;
                    }
                }
            }
        }
    }
}
