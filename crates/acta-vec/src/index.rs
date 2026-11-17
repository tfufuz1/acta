//! Segmentierter exakter Vektorindex mit Lineage (SPEC 9.3, 8.5, 9.4; Z1, Z7, Z11, Z13, Z16).
//!
//! Jeder Eintrag trägt seine `Lineage`. Eine Löschung entfernt Einträge, deren Schlüssel oder
//! Herkunft die Zielmenge trifft. Betroffene Segmente werden durch ein Ersatzsegment ausgetauscht,
//! die alten Vektorbytes werden genullt. Es gibt kein Tombstone-Bit (SPEC 8.5).
//!
//! Die Suche bewertet jede zugelassene Kandidatin exakt. Der Nachrang ist total: Score absteigend,
//! dann Schlüssel aufsteigend. Damit hängt das Ergebnis nicht von der Segmentierung ab.

use std::collections::BTreeMap;
use std::sync::{Mutex, MutexGuard, RwLock};

use acta_base::fx::Fx;
use acta_base::ids::TenantOrd;
use acta_gate::error::GateError;
use acta_gate::layer::{ErasureLayer, LayerAck, LayerPlan, Residue};
use acta_gate::state::{GateState, Lineage, LineageHit, TargetSet};
use acta_gate::view::ErasedView;
use zeroize::Zeroize;

use crate::distance::sim_parts;
use crate::error::VecError;
use crate::lineage::Derived;
use crate::quantize::QuantizedVector;
use crate::topk::TopK;
use crate::types::{CandidateFilter, VectorEntry, VectorHit, VectorId};

/// Name der Schicht im Erasure-Protokoll.
const LAYER: &str = "vec";
/// Obergrenze für Plan-Einträge; abgeschlossene Einträge werden vorher verworfen.
const MAX_PLANS: usize = 1024;

/// Ein Segment: spaltenweise Ablage mit Schlüsseln, Herkunft, Norm und Vektorbytes.
#[derive(Debug)]
struct Segment {
    dim: usize,
    keys: Vec<VectorId>,
    lineages: Vec<Lineage>,
    norms: Vec<u32>,
    data: Vec<i8>,
}

impl Segment {
    fn new(dim: usize) -> Self {
        Self {
            dim,
            keys: Vec::new(),
            lineages: Vec::new(),
            norms: Vec::new(),
            data: Vec::new(),
        }
    }

    fn len(&self) -> usize {
        self.keys.len()
    }

    fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    fn push(&mut self, key: VectorId, lineage: Lineage, vector: &QuantizedVector) {
        self.keys.push(key);
        self.lineages.push(lineage);
        self.norms.push(vector.norm2());
        self.data.extend_from_slice(vector.as_bytes());
    }

    /// Alle Einträge in Einfügereihenfolge.
    fn rows(&self) -> impl Iterator<Item = (VectorId, &Lineage, &[i8], u32)> {
        self.keys
            .iter()
            .zip(&self.lineages)
            .zip(self.data.chunks_exact(self.dim))
            .zip(&self.norms)
            .map(|(((key, lineage), row), norm)| (*key, lineage, row, *norm))
    }

    /// Vektorbytes und Norm der Zeile `idx`.
    fn row(&self, idx: usize) -> Option<(&[i8], u32)> {
        let start = idx.checked_mul(self.dim)?;
        let end = start.checked_add(self.dim)?;
        let bytes = self.data.get(start..end)?;
        let norm = self.norms.get(idx).copied()?;
        Some((bytes, norm))
    }

    /// Ersetzt das Segment durch eine Kopie ohne die getroffenen Einträge.
    ///
    /// Das alte Segment wird genullt. Liefert die Anzahl entfernter Einträge.
    fn erase_matching(&mut self, target: &TargetSet) -> usize {
        let removed = self
            .rows()
            .filter(|(key, lineage, _, _)| matches_target(*key, lineage, target))
            .count();
        if removed == 0 {
            return 0;
        }
        let mut kept = Segment::new(self.dim);
        for (key, lineage, row, norm) in self.rows() {
            if !matches_target(key, lineage, target) {
                kept.keys.push(key);
                kept.lineages.push(lineage.clone());
                kept.norms.push(norm);
                kept.data.extend_from_slice(row);
            }
        }
        let mut old = std::mem::replace(self, kept);
        old.data.zeroize();
        removed
    }
}

/// Trifft ein Eintrag die Zielmenge? Direkt über den Schlüssel oder über die Herkunft (Z13).
fn matches_target(key: VectorId, lineage: &Lineage, target: &TargetSet) -> bool {
    let direct = match key {
        VectorId::Knowledge { kord, .. } => target.kords.contains(kord.0),
        VectorId::Personal { ver, .. } => target.vers.contains(ver.0),
    };
    direct
        || !target.vers.is_disjoint(lineage.vers())
        || lineage
            .kords()
            .iter()
            .any(|kord| target.kords.contains(kord))
}

/// Sichtbarkeit eines Eintrags aus Gate-Sicht (Z1): kein Takedown, Version live, Herkunft frei.
pub(crate) fn is_visible(key: VectorId, lineage: &Lineage, view: &dyn ErasedView) -> bool {
    let key_visible = match key {
        VectorId::Knowledge { kord, .. } => !view.is_redacted_k(kord),
        VectorId::Personal { ver, .. } => view.check_ver(ver) == GateState::Live,
    };
    key_visible && view.lineage_hit(lineage) == LineageHit::Live
}

/// Position eines Schlüssels: Segment und Zeile.
#[derive(Debug, Clone, Copy)]
struct Location {
    segment: usize,
    row: usize,
}

#[derive(Debug, Default)]
struct State {
    segments: Vec<Segment>,
    locations: BTreeMap<VectorId, Location>,
}

/// Offener oder abgeschlossener Löschplan. Die Zielmenge ist Teil des Registers, weil der
/// Plan selbst nur Zähler trägt (`LayerPlan`).
#[derive(Debug)]
struct PendingPlan {
    plan: LayerPlan,
    target: TargetSet,
    done: bool,
}

#[derive(Debug, Default)]
struct Registry {
    tenant: Option<TenantOrd>,
    plans: Vec<PendingPlan>,
}

/// Segmentierter exakter Vektorindex für eine feste Dimension.
///
/// Ein Index gehört zu genau einem Mandanten. Die Bindung wird beim ersten Löschvorgang festgelegt
/// und danach bei jedem Aufruf geprüft.
#[derive(Debug)]
pub struct SegmentedVectorIndex {
    dim: usize,
    segment_capacity: usize,
    state: RwLock<State>,
    registry: Mutex<Registry>,
}

impl SegmentedVectorIndex {
    /// Legt einen leeren Index an. `segment_capacity` begrenzt die Einträge je Segment.
    ///
    /// # Errors
    /// `InvalidConfig`, wenn `dim` oder `segment_capacity` null ist.
    pub fn new(dim: usize, segment_capacity: usize) -> Result<Self, VecError> {
        if dim == 0 || segment_capacity == 0 {
            return Err(VecError::InvalidConfig);
        }
        Ok(Self {
            dim,
            segment_capacity,
            state: RwLock::new(State::default()),
            registry: Mutex::new(Registry::default()),
        })
    }

    /// Dimension der gespeicherten Vektoren.
    #[must_use]
    pub const fn dim(&self) -> usize {
        self.dim
    }

    /// Fügt einen abgeleiteten Vektor ein. Die Herkunft gehört zum Eintrag (Z13).
    ///
    /// # Errors
    /// `DimensionMismatch`, `DuplicateKey`, `CorruptedSegment` bei gesperrtem Zustand.
    pub fn put_derived(&mut self, entry: Derived<VectorEntry>) -> Result<(), VecError> {
        let (entry, lineage) = entry.into_parts();
        let (key, vector) = entry.into_parts();
        if vector.len() != self.dim {
            return Err(VecError::DimensionMismatch);
        }
        let (dim, capacity) = (self.dim, self.segment_capacity);
        let state = self
            .state
            .get_mut()
            .map_err(|_| VecError::CorruptedSegment)?;
        if state.locations.contains_key(&key) {
            return Err(VecError::DuplicateKey);
        }
        if state.segments.last().is_none_or(|s| s.len() >= capacity) {
            state.segments.push(Segment::new(dim));
        }
        let segment = state.segments.len().saturating_sub(1);
        let Some(last) = state.segments.last_mut() else {
            return Err(VecError::CorruptedSegment);
        };
        let row = last.len();
        last.push(key, lineage, &vector);
        state.locations.insert(key, Location { segment, row });
        Ok(())
    }

    /// Anzahl aktiver Einträge.
    #[must_use]
    pub fn total_active_entries(&self) -> usize {
        self.state.read().map_or(0, |s| s.locations.len())
    }

    /// Anzahl logisch entfernter, physisch noch vorhandener Einträge.
    ///
    /// Der Index führt kein Tombstone-Bit (SPEC 8.5). Löschung verdichtet sofort, daher ist der
    /// Wert strukturell 0.
    #[must_use]
    pub const fn tombstone_count(&self) -> usize {
        0
    }

    /// Exakte Flat-Suche über alle Segmente.
    ///
    /// `filter = None` hebt jede Zugriffsbeschränkung auf. Produktionspfade übergeben die
    /// Sichtklasse des Nutzers oder nutzen [`Self::search_visible`].
    ///
    /// # Errors
    /// `DimensionMismatch` bei falscher Abfragedimension, `CorruptedSegment` bei gesperrtem Zustand.
    pub fn search(
        &self,
        query: &QuantizedVector,
        k: usize,
        filter: Option<&CandidateFilter>,
    ) -> Result<Vec<VectorHit>, VecError> {
        self.scan(query, k, |key, _| filter.is_none_or(|f| f.allows(key)))
    }

    /// Suche mit Zugriffsfilter und Gate-Sicht (Z1, Z16).
    ///
    /// Gelöschte, eingeschränkte oder über die Herkunft betroffene Einträge werden nicht gelesen.
    /// Sie beeinflussen weder Ergebnis noch Rangfolge.
    ///
    /// # Errors
    /// Wie [`Self::search`].
    pub fn search_visible(
        &self,
        query: &QuantizedVector,
        k: usize,
        filter: &CandidateFilter,
        view: &dyn ErasedView,
    ) -> Result<Vec<VectorHit>, VecError> {
        self.scan(query, k, |key, lineage| {
            filter.allows(key) && is_visible(key, lineage, view)
        })
    }

    /// Alle Einträge als Rohdaten, sortiert nach Schlüssel, für den Neuaufbau.
    ///
    /// # Errors
    /// `CorruptedSegment`, wenn eine gespeicherte Norm nicht zu den Bytes passt (Integritätsprüfung).
    pub fn rebuild(&self) -> Result<Self, VecError> {
        let state = self.state.read().map_err(|_| VecError::CorruptedSegment)?;
        let mut entries: Vec<(VectorId, Lineage, QuantizedVector)> =
            Vec::with_capacity(state.locations.len());
        for segment in &state.segments {
            for (key, lineage, row, norm) in segment.rows() {
                let vector = QuantizedVector::from_bytes(row.to_vec())?;
                if vector.norm2() != norm {
                    return Err(VecError::CorruptedSegment);
                }
                entries.push((key, lineage.clone(), vector));
            }
        }
        drop(state);
        entries.sort_by(|a, b| a.0.cmp(&b.0));

        let mut fresh = Self::new(self.dim, self.segment_capacity)?;
        for (key, lineage, vector) in entries {
            fresh.put_derived(Derived::new(VectorEntry::new(key, vector), lineage))?;
        }
        Ok(fresh)
    }

    /// Bewertet einen Schlüssel exakt. `None`, wenn er fehlt oder nicht zugelassen ist.
    pub(crate) fn score_key<F>(
        &self,
        query: &QuantizedVector,
        key: VectorId,
        accept: F,
    ) -> Result<Option<Fx>, VecError>
    where
        F: Fn(VectorId, &Lineage) -> bool,
    {
        if query.len() != self.dim {
            return Err(VecError::DimensionMismatch);
        }
        let state = self.state.read().map_err(|_| VecError::CorruptedSegment)?;
        let Some(location) = state.locations.get(&key).copied() else {
            return Ok(None);
        };
        let Some(segment) = state.segments.get(location.segment) else {
            return Err(VecError::CorruptedSegment);
        };
        let (Some(lineage), Some((row, norm))) = (
            segment.lineages.get(location.row),
            segment.row(location.row),
        ) else {
            return Err(VecError::CorruptedSegment);
        };
        if !accept(key, lineage) {
            return Ok(None);
        }
        sim_parts(query.as_bytes(), query.norm2(), row, norm).map(Some)
    }

    /// Dimension und Abfrage prüfen, dann alle zugelassenen Einträge exakt bewerten.
    ///
    /// Segmente werden auf Arbeiter verteilt. Jeder Arbeiter liefert seine Top-k. Die globale
    /// Top-k ist Teilmenge dieser Vereinigung, und die totale Rangfolge macht die
    /// Zusammenführung unabhängig von der Reihenfolge der Arbeiter.
    fn scan<F>(
        &self,
        query: &QuantizedVector,
        k: usize,
        accept: F,
    ) -> Result<Vec<VectorHit>, VecError>
    where
        F: Fn(VectorId, &Lineage) -> bool + Sync,
    {
        if query.len() != self.dim {
            return Err(VecError::DimensionMismatch);
        }
        let state = self.state.read().map_err(|_| VecError::CorruptedSegment)?;
        let workers = std::thread::available_parallelism()
            .map_or(1, std::num::NonZeroUsize::get)
            .min(state.segments.len())
            .max(1);
        let group_size = state.segments.len().div_ceil(workers).max(1);
        let accept = &accept;
        let parts: Vec<Result<Vec<VectorHit>, VecError>> = std::thread::scope(|scope| {
            let handles: Vec<_> = state
                .segments
                .chunks(group_size)
                .map(|group| scope.spawn(move || scan_group(group, query, k, accept)))
                .collect();
            handles
                .into_iter()
                .map(|handle| handle.join().unwrap_or(Err(VecError::CorruptedSegment)))
                .collect()
        });
        let mut top = TopK::new(k);
        for part in parts {
            for hit in part? {
                top.push(hit);
            }
        }
        Ok(top.into_sorted())
    }

    /// Anzahl der Einträge, die die Zielmenge trifft.
    fn count_matches(&self, target: &TargetSet) -> Result<usize, GateError> {
        let state = self.state.read().map_err(|_| layer_error())?;
        Ok(state
            .segments
            .iter()
            .map(|segment| {
                segment
                    .rows()
                    .filter(|(key, lineage, _, _)| matches_target(*key, lineage, target))
                    .count()
            })
            .sum())
    }

    /// Entfernt alle Treffer der Zielmenge und verdichtet die Segmente.
    fn erase_matching(&self, target: &TargetSet) -> Result<usize, GateError> {
        let mut state = self.state.write().map_err(|_| layer_error())?;
        let mut removed = 0;
        for segment in &mut state.segments {
            removed += segment.erase_matching(target);
        }
        if removed > 0 {
            state.segments.retain(|segment| !segment.is_empty());
            reindex(&mut state);
        }
        Ok(removed)
    }
}

/// Bewertet die Segmente einer Gruppe und liefert deren Top-k.
fn scan_group<F>(
    group: &[Segment],
    query: &QuantizedVector,
    k: usize,
    accept: &F,
) -> Result<Vec<VectorHit>, VecError>
where
    F: Fn(VectorId, &Lineage) -> bool,
{
    let mut top = TopK::new(k);
    for segment in group {
        for (key, lineage, row, norm) in segment.rows() {
            if !accept(key, lineage) {
                continue;
            }
            let score = sim_parts(query.as_bytes(), query.norm2(), row, norm)?;
            top.push(VectorHit { key, score });
        }
    }
    Ok(top.into_sorted())
}

/// Baut die Positionstabelle nach einer Verdichtung neu auf.
fn reindex(state: &mut State) {
    let State {
        segments,
        locations,
    } = state;
    locations.clear();
    for (segment, seg) in segments.iter().enumerate() {
        for (row, key) in seg.keys.iter().enumerate() {
            locations.insert(*key, Location { segment, row });
        }
    }
}

/// Einheitlicher Fehler der Erasure-Schicht (Z15: ohne Kennungen).
const fn layer_error() -> GateError {
    GateError::Layer { layer: LAYER }
}

fn to_u64(value: usize) -> Result<u64, GateError> {
    u64::try_from(value).map_err(|_| layer_error())
}

fn lock_registry(registry: &Mutex<Registry>) -> Result<MutexGuard<'_, Registry>, GateError> {
    registry.lock().map_err(|_| layer_error())
}

fn check_tenant(registry: &Registry, tenant: TenantOrd) -> Result<(), GateError> {
    match registry.tenant {
        Some(bound) if bound != tenant => Err(layer_error()),
        _ => Ok(()),
    }
}

fn bind_tenant(registry: &mut Registry, tenant: TenantOrd) -> Result<(), GateError> {
    check_tenant(registry, tenant)?;
    if registry.tenant.is_none() {
        registry.tenant = Some(tenant);
    }
    Ok(())
}

impl ErasureLayer for SegmentedVectorIndex {
    fn name(&self) -> &'static str {
        LAYER
    }

    /// Plant die Löschung. Der Plan trägt nur Zähler; die Zielmenge wird im Register gehalten.
    ///
    /// Zwei offene Pläne mit gleichem Zähler, aber verschiedenen Zielen, sind nicht
    /// unterscheidbar. Ein solcher Aufruf wird abgewiesen, bis der erste Plan abgeschlossen ist.
    fn plan(&self, tenant: TenantOrd, target: &TargetSet) -> Result<LayerPlan, GateError> {
        let entries = self.count_matches(target)?;
        let plan = LayerPlan::new(LAYER, to_u64(entries)?);
        let mut registry = lock_registry(&self.registry)?;
        bind_tenant(&mut registry, tenant)?;

        let conflict = registry
            .plans
            .iter()
            .any(|p| !p.done && p.plan == plan && p.target != *target);
        if conflict {
            return Err(layer_error());
        }
        let known = registry
            .plans
            .iter()
            .any(|p| !p.done && p.plan == plan && p.target == *target);
        if known {
            return Ok(plan);
        }
        if registry.plans.len() >= MAX_PLANS {
            registry.plans.retain(|p| !p.done);
        }
        if registry.plans.len() >= MAX_PLANS {
            return Err(layer_error());
        }
        registry.plans.push(PendingPlan {
            plan: plan.clone(),
            target: target.clone(),
            done: false,
        });
        Ok(plan)
    }

    /// Führt einen geplanten Löschvorgang aus. Wiederholung mit demselben Plan liefert `purged == 0`.
    ///
    /// Ein unbekannter Plan wird abgewiesen und löscht nichts.
    fn purge(&self, tenant: TenantOrd, plan: &LayerPlan) -> Result<LayerAck, GateError> {
        let mut registry = lock_registry(&self.registry)?;
        bind_tenant(&mut registry, tenant)?;
        if let Some(pending) = registry
            .plans
            .iter_mut()
            .find(|p| !p.done && p.plan == *plan)
        {
            let removed = self.erase_matching(&pending.target)?;
            pending.done = true;
            return Ok(LayerAck::new(LAYER, tenant, to_u64(removed)?));
        }
        if registry.plans.iter().any(|p| p.done && p.plan == *plan) {
            return Ok(LayerAck::new(LAYER, tenant, 0));
        }
        Err(layer_error())
    }

    /// Misst die Rückstände der Zielmenge. Die Messung zählt Treffer und liefert keine Kennungen.
    fn residue(&self, tenant: TenantOrd, target: &TargetSet) -> Result<Vec<Residue>, GateError> {
        {
            let registry = lock_registry(&self.registry)?;
            check_tenant(&registry, tenant)?;
        }
        let count = self.count_matches(target)?;
        Ok(vec![Residue::new(LAYER, to_u64(count)?)])
    }
}
