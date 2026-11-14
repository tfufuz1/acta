//! # Zeitreise-Semantik: Retention, History-Floor, `ReadOutcome` (Spec §5)
//!
//! * [`Retention`] – pro Partition festgelegte Aufbewahrung (N: `KeepAll`, P:
//!   `LiveWhileKeyAlive`, D: `Window`).
//! * [`RetentionPolicy`] – ersetzt die fest verdrahtete Regel in Flush und Compaction.
//!   Der Snapshot-Floor (`min_active_seqno`) bleibt zusätzliche Untergrenze, ist aber
//!   nicht mehr alleinige Wahrheit.
//! * [`ReadOutcome`] – unterscheidet „existierte nicht", „gelöscht" und „außerhalb des
//!   Horizonts" (keine stillen falschen Antworten bei `as_of < floor`).

use std::collections::BTreeMap;
use std::sync::Arc;

/// Partitions-ID. Solange der Store nur eine physische Partition kennt, ist das `0`.
pub type PartitionId = u32;

/// Die Standard-Partition des Stores.
pub const DEFAULT_PARTITION: PartitionId = 0;

/// Aufbewahrungsklasse einer Partition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Retention {
    /// N (Chronik): nie eine Version verwerfen, auch Tombstones nicht.
    KeepAll,
    /// P (Tresor): alle Versionen oberhalb des Snapshot-Floors, darunter nur die jeweils
    /// neueste („Floor-Version"). Die Lebensdauer ergibt sich aus dem Subject-DEK
    /// (ReadGate), nicht aus dem Store.
    LiveWhileKeyAlive,
    /// D (Ableitungen): zusätzlich Untergrenze `min_seq_floor`; Versionen darunter
    /// dürfen (bei Schattierung durch eine neuere Floor-Version) verworfen werden.
    Window { min_seq_floor: u64 },
}

/// Entscheidet, ob eine Version physisch verworfen werden darf.
pub trait RetentionPolicy: Send + Sync + std::fmt::Debug {
    /// Persistierter Floor der Partition (Zeitreise darunter ⇒ `HorizonExceeded`).
    fn floor(&self, partition: PartitionId) -> u64;

    /// `newer_exists`: eine neuere Version mit `seq <= snapshot_floor` bleibt erhalten
    /// (die Version ist also für keinen Snapshot mehr die sichtbare).
    /// `snapshot_floor`: kleinster gepinnter Snapshot-Seq.
    fn may_drop_version(
        &self,
        partition: PartitionId,
        seq: u64,
        newer_exists: bool,
        snapshot_floor: u64,
    ) -> bool;
}

/// Standard-Policy: `Retention` pro Partition, Rückfall auf `LiveWhileKeyAlive`
/// (entspricht dem bisherigen Verhalten von `retain_key_versions`).
#[derive(Debug, Clone, Default)]
pub struct PartitionRetention {
    per_partition: BTreeMap<PartitionId, Retention>,
    floors: BTreeMap<PartitionId, u64>,
}

impl PartitionRetention {
    pub fn new() -> Self {
        Self::default()
    }

    /// Setzt die Retention einer Partition.
    pub fn with_retention(mut self, partition: PartitionId, r: Retention) -> Self {
        self.per_partition.insert(partition, r);
        self
    }

    /// Setzt den (persistierten) History-Floor einer Partition.
    pub fn with_floor(mut self, partition: PartitionId, floor: u64) -> Self {
        self.floors.insert(partition, floor);
        self
    }

    pub fn retention(&self, partition: PartitionId) -> Retention {
        self.per_partition
            .get(&partition)
            .copied()
            .unwrap_or(Retention::LiveWhileKeyAlive)
    }

    /// Standard-Policy als `Arc<dyn RetentionPolicy>`.
    pub fn default_arc() -> Arc<dyn RetentionPolicy> {
        Arc::new(Self::default())
    }
}

impl RetentionPolicy for PartitionRetention {
    fn floor(&self, partition: PartitionId) -> u64 {
        self.floors.get(&partition).copied().unwrap_or(0)
    }

    fn may_drop_version(
        &self,
        partition: PartitionId,
        seq: u64,
        newer_exists: bool,
        snapshot_floor: u64,
    ) -> bool {
        if !newer_exists || seq > snapshot_floor {
            return false;
        }
        match self.retention(partition) {
            Retention::KeepAll => false,
            Retention::LiveWhileKeyAlive => true,
            Retention::Window { min_seq_floor } => seq < min_seq_floor,
        }
    }
}

/// Filtert die Versionen EINES Schlüssels (absteigend nach Roh-Seq) gemäß Policy.
///
/// Regeln (Obermenge der alten `retain_key_versions`):
/// 1. Alles oberhalb von `snapshot_floor` bleibt.
/// 2. Die neueste Version `<= snapshot_floor` bleibt (Floor-Version).
/// 3. Ältere Versionen nur, wenn `policy.may_drop_version(..)` zustimmt.
pub fn retain_versions_with_policy<T, F>(
    entries: Vec<T>,
    get_seq: F,
    snapshot_floor: u64,
    partition: PartitionId,
    policy: &dyn RetentionPolicy,
) -> Vec<T>
where
    F: Fn(&T) -> u64,
{
    let mut retained = Vec::with_capacity(entries.len());
    let mut found_floor_version = false;
    for entry in entries {
        let raw_seq = get_seq(&entry) & !acta_ids::TOMBSTONE_BIT;
        if raw_seq > snapshot_floor {
            retained.push(entry);
        } else if !found_floor_version {
            retained.push(entry);
            found_floor_version = true;
        } else if !policy.may_drop_version(partition, raw_seq, true, snapshot_floor) {
            retained.push(entry);
        }
    }
    retained
}

/// Ergebnis eines Zeitreise-Reads (Spec §5.2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReadOutcome<T> {
    Found(T),
    /// Der Schlüssel existierte zu diesem Seq nachweislich nicht.
    NotFound,
    /// Subject geshreddet; wirkt unabhängig vom angefragten Seq (D-4).
    Erased { erase_seq: u64, erasure_id: [u8; 16] },
    /// `as_of` liegt unterhalb des persistierten History-Floors.
    HorizonExceeded { floor: u64 },
}

impl<T> ReadOutcome<T> {
    pub fn is_found(&self) -> bool {
        matches!(self, ReadOutcome::Found(_))
    }

    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> ReadOutcome<U> {
        match self {
            ReadOutcome::Found(v) => ReadOutcome::Found(f(v)),
            ReadOutcome::NotFound => ReadOutcome::NotFound,
            ReadOutcome::Erased { erase_seq, erasure_id } => {
                ReadOutcome::Erased { erase_seq, erasure_id }
            }
            ReadOutcome::HorizonExceeded { floor } => ReadOutcome::HorizonExceeded { floor },
        }
    }
}

/// Prüft den Horizont: `seq < floor` ⇒ `HorizonExceeded`.
pub fn check_horizon(seq: u64, floor: u64) -> Option<ReadOutcome<()>> {
    (seq < floor).then_some(ReadOutcome::HorizonExceeded { floor })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn versions() -> Vec<u64> {
        vec![30, 20, 10, 5]
    }

    #[test]
    fn live_while_key_alive_matches_legacy_rule() {
        let p = PartitionRetention::new();
        let got = retain_versions_with_policy(versions(), |s| *s, 10, 0, &p);
        let legacy = crate::compaction::retain_key_versions(versions(), |s| *s, 10);
        assert_eq!(got, legacy);
        assert_eq!(got, vec![30, 20, 10]);
    }

    #[test]
    fn keep_all_never_drops() {
        let p = PartitionRetention::new().with_retention(0, Retention::KeepAll);
        let got = retain_versions_with_policy(versions(), |s| *s, u64::MAX, 0, &p);
        assert_eq!(got, versions());
    }

    #[test]
    fn window_drops_only_below_min_seq_floor() {
        let p = PartitionRetention::new().with_retention(0, Retention::Window { min_seq_floor: 8 });
        // snapshot_floor 30: neueste (30) bleibt; 20,10 bleiben (>= 8); 5 fällt (< 8).
        let got = retain_versions_with_policy(versions(), |s| *s, 30, 0, &p);
        assert_eq!(got, vec![30, 20, 10]);
    }

    #[test]
    fn tombstone_bit_is_masked() {
        let p = PartitionRetention::new();
        let tb = acta_ids::TOMBSTONE_BIT;
        let v = vec![20 | tb, 10, 5];
        let got = retain_versions_with_policy(v, |s| *s, 20, 0, &p);
        assert_eq!(got, vec![20 | tb]);
    }

    #[test]
    fn horizon_check() {
        assert_eq!(
            check_horizon(3, 5),
            Some(ReadOutcome::HorizonExceeded { floor: 5 })
        );
        assert_eq!(check_horizon(5, 5), None);
    }
}
