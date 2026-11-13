//! # Modul: wiring.rs
//!
//! **Zweck:** Verdrahtung und Initialisierung aller Teilsysteme und Ports.
//!
//! **Architektur:** Gehört zur Crate `acta-engine` (Ring 2).
//! Eingehaltene Invarianten: Fail-Closed bei Fehlern, ACID/SSI-Transaktionsgarantien.

use crate::*;
use acta_ports::DriftStatusProvider;

impl Acta {
    /// Configures or replaces the active [`MetricsSink`].
    pub fn set_metrics_sink(&self, sink: Arc<dyn acta_ports::MetricsSink>) {
        *self.metrics_sink.write() = sink;
    }

    /// Returns the active [`MetricsSink`].
    pub fn metrics_sink(&self) -> Arc<dyn acta_ports::MetricsSink> {
        self.metrics_sink.read().clone()
    }

    // ANCHOR[acta-engine:WIR-001]
    /// Registriert die Router-Komponente.
    pub fn set_router(&self, router: std::sync::Weak<dyn DriftStatusProvider>) {
        *self.router.write() = Some(router);
    }

    // ANCHOR[acta-engine:WIR-002]
    /// Registriert die Score-Kalibrator-Komponente.
    pub fn set_calibrator(
        &self,
        calibrator: std::sync::Weak<parking_lot::Mutex<acta_rank::IsotonicCalibrator>>,
    ) {
        *self.calibrator.write() = Some(calibrator);
    }
}
