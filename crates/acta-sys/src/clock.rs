//! `SystemClock`: Implementierung des Ports `Clock` mit der Systemuhr (SPEC 3.7, 4.6).
//!
//! Zeit ist nicht vertrauenswürdig (A-CLOCK). Ein Rücksprung wird nicht hier erkannt, sondern
//! vom Knoten über den Vergleich aufeinanderfolgender Werte.
//!
//! Abweichung: Der Port liefert `u64` ohne Fehler. Eine Systemuhr vor dem Unix-Epochenbeginn
//! wird daher auf 0 abgebildet und dies hier ausdrücklich dokumentiert.

use std::time::{SystemTime, UNIX_EPOCH};

use acta_base::Clock;

/// Systemuhr als injizierbarer Port.
#[derive(Debug, Default, Clone, Copy)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now_ms(&self) -> u64 {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(d) => u64::try_from(d.as_millis()).unwrap_or(u64::MAX),
            Err(_) => 0,
        }
    }
}
