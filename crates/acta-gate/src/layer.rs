//! Erasure-Schichten (SPEC 3.7 `ErasureLayer`, `LayerAck`).
//!
//! Implementiert von `seg`, `pstore`, `text`, `vec`, `rag` und dem Schlüssel-Cache. Dieser Crate
//! definiert nur die Verträge. Ein Löschnachweis entsteht erst aus einer Messung mit bestandener
//! Positivkontrolle (Z11); das gehört in den Crate, der die Messung ausführt.

use acta_base::ids::TenantOrd;

use crate::error::GateError;
use crate::state::TargetSet;

/// Plan einer Schicht für ein Ziel. Enthält nur Zähler, keine Kennungen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayerPlan {
    layer: &'static str,
    entries: u64,
}

impl LayerPlan {
    /// Erzeugt einen Plan.
    #[must_use]
    pub const fn new(layer: &'static str, entries: u64) -> Self {
        Self { layer, entries }
    }

    /// Name der Schicht.
    #[must_use]
    pub const fn layer(&self) -> &'static str {
        self.layer
    }

    /// Anzahl der geplanten Einträge.
    #[must_use]
    pub const fn entries(&self) -> u64 {
        self.entries
    }
}

/// Bestätigung einer Schicht, dass ihre Einträge für das Ziel entfernt sind.
///
/// `purge` ist idempotent: ein zweiter Aufruf mit demselben Plan liefert dieselbe Bestätigung
/// mit `purged == 0`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayerAck {
    layer: &'static str,
    tenant: TenantOrd,
    purged: u64,
}

impl LayerAck {
    /// Erzeugt eine Bestätigung.
    #[must_use]
    pub const fn new(layer: &'static str, tenant: TenantOrd, purged: u64) -> Self {
        Self {
            layer,
            tenant,
            purged,
        }
    }

    /// Name der Schicht.
    #[must_use]
    pub const fn layer(&self) -> &'static str {
        self.layer
    }

    /// Mandant der Bestätigung.
    #[must_use]
    pub const fn tenant(&self) -> TenantOrd {
        self.tenant
    }

    /// Anzahl der in diesem Aufruf entfernten Einträge.
    #[must_use]
    pub const fn purged(&self) -> u64 {
        self.purged
    }
}

/// Messung von Rückständen einer Schicht (Z11: Ergebnis nur mit Positivkontrolle).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Residue {
    layer: &'static str,
    count: u64,
}

impl Residue {
    /// Erzeugt einen Messwert.
    #[must_use]
    pub const fn new(layer: &'static str, count: u64) -> Self {
        Self { layer, count }
    }

    /// Name der Schicht.
    #[must_use]
    pub const fn layer(&self) -> &'static str {
        self.layer
    }

    /// Gefundene Rückstände.
    #[must_use]
    pub const fn count(&self) -> u64 {
        self.count
    }
}

/// Vertrag einer Erasure-Schicht (SPEC 3.7). `LayerError` ist `GateError` (HR-4).
pub trait ErasureLayer: Send + Sync {
    /// Stabiler Name der Schicht, etwa `"seg"` oder `"vec"`.
    fn name(&self) -> &'static str;

    /// Plant die Entfernung für ein Ziel; ändert nichts.
    ///
    /// # Errors
    /// `GateError::Layer` bei nicht erreichbarer Schicht.
    fn plan(&self, tenant: TenantOrd, target: &TargetSet) -> Result<LayerPlan, GateError>;

    /// Entfernt die geplanten Einträge; idempotent.
    ///
    /// # Errors
    /// `GateError::Layer` bei Fehlern; ein wiederholter Aufruf ist zulässig.
    fn purge(&self, tenant: TenantOrd, plan: &LayerPlan) -> Result<LayerAck, GateError>;

    /// Misst Rückstände des Ziels.
    ///
    /// # Errors
    /// `GateError::Layer` bei Fehlern der Messung.
    fn residue(&self, tenant: TenantOrd, target: &TargetSet) -> Result<Vec<Residue>, GateError>;
}
