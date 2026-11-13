//! # Modul: diagnostics.rs
//!
//! **Zweck:** Systemdiagnose, Integritätsprüfungen und Metrikerfassung.
//!
//! **Architektur:** Gehört zur Crate `acta-engine` (Ring 2).
//! Eingehaltene Invarianten: Fail-Closed bei Fehlern, ACID/SSI-Transaktionsgarantien.

use crate::*;
use acta_types::Result;
use std::sync::Arc;

impl Acta {
    // ANCHOR[acta-engine:STA-014]
    /// Dokumentation für `stats` im Rahmen der Transaktions- und Facade-Orchestrierung.
    #[tracing::instrument(level = "trace", skip(self))]
    pub async fn stats(&self) -> Result<ActaStats> {
        let default_col = self.default_col().await?;
        let active_memory_count = default_col.len().await;
        let index_stats = default_col.stats().await?;
        let storage_stats = self.storage.stats().await?;

        let drift_status = self
            .router
            .read()
            .as_ref()
            .and_then(|w| w.upgrade())
            .map(|r| r.overall_drift_status())
            .unwrap_or_else(|| "nicht verfügbar".to_string());

        let (calibration_ece, last_calibration_at) = self
            .calibrator
            .read()
            .as_ref()
            .and_then(|w| w.upgrade())
            .map(|c| {
                let guard = c.lock();
                (guard.cached_ece(), guard.last_calibration_at())
            })
            .unwrap_or((None, None));

        let pid_pool_size = None;

        Ok(ActaStats {
            drift_status,
            calibration_ece,
            last_calibration_at,
            active_memory_count,
            pid_pool_size,
            index_stats,
            storage_stats,
        })
    }

    // ANCHOR[acta-engine:FLU-053]
    /// Dokumentation für `flush` im Rahmen der Transaktions- und Facade-Orchestrierung.
    #[tracing::instrument(level = "trace", skip(self))]
    pub async fn flush(&self) -> Result<()> {
        self.storage.flush().await?;
        Ok(())
    }

    // ANCHOR[acta-engine:DIA-003]
    /// Liefert Zugriff auf die Registry verwaister Knoten.
    pub fn orphan_registry(&self) -> &Arc<acta_checkpoint::InstanceOrphanRegistry> {
        &self.orphan_registry
    }
}
