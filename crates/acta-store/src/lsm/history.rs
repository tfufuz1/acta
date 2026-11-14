//! # History-Floor und geprüfte Zeitreise-Reads (Spec §5)
//!
//! `raise_history_floor` hält die Commit-Reihenfolge **Floor → Verwerfen** ein: der Floor
//! wird zuerst im Manifest durabel und erst danach im Speicher sichtbar; erst ab dann darf
//! eine Retention-Policy darunterliegende Versionen verwerfen.

use super::engine::LsmStorage;
use crate::error::{StoreError, StoreResult as Result};
use crate::history::{PartitionId, ReadOutcome, RetentionPolicy};
use crate::manifest::ManifestEntry;
use bytes::Bytes;
use std::sync::atomic::Ordering;
use std::sync::Arc;

impl LsmStorage {
    /// Aktueller persistierter History-Floor der Partition (0 = unbegrenzt zeitreisefähig).
    pub fn history_floor(&self, partition: PartitionId) -> u64 {
        self.history_floors
            .read()
            .get(&partition)
            .copied()
            .unwrap_or(0)
    }

    /// Hebt den History-Floor an (monoton; ein niedrigerer Wert ist ein No-op).
    ///
    /// Liefert den danach gültigen Floor. Der Floor darf nicht über den zuletzt
    /// vergebenen Seq hinausgehen (sonst wären künftige Writes „unter" dem Floor).
    pub async fn raise_history_floor(&self, partition: PartitionId, floor: u64) -> Result<u64> {
        let last = self.next_seq_no.load(Ordering::SeqCst).saturating_sub(1);
        if floor > last {
            return Err(StoreError::InvalidInput(format!(
                "history floor {floor} exceeds last assigned seq {last}"
            )));
        }
        let current = self.history_floor(partition);
        if floor <= current {
            return Ok(current);
        }
        // 1. Durabel (fsync im Manifest) ...
        self.manifest
            .append(&ManifestEntry::HistoryFloor {
                partition,
                seq: floor,
            })
            .await?;
        // 2. ... dann sichtbar. Monotoner Merge unter Schreib-Lock.
        let mut guard = self.history_floors.write();
        let slot = guard.entry(partition).or_insert(0);
        if floor > *slot {
            *slot = floor;
        }
        Ok(*slot)
    }

    /// Ersetzt die Retention-Policy (z. B. `KeepAll` für die Chronik).
    pub fn set_retention_policy(&self, policy: Arc<dyn RetentionPolicy>) {
        self.compaction_engine.set_retention(policy);
    }

    /// Zeitreise-Read mit Horizont-Prüfung: `seq < floor` ⇒ `HorizonExceeded`
    /// statt einer stillen, womöglich falschen `NotFound`-Antwort.
    pub async fn get_at_seq_checked(
        &self,
        partition: PartitionId,
        key: &[u8],
        seq: u64,
    ) -> Result<ReadOutcome<Bytes>> {
        let floor = self.history_floor(partition);
        if seq < floor {
            return Ok(ReadOutcome::HorizonExceeded { floor });
        }
        Ok(match self.get_at_seq(key, seq).await? {
            Some(v) => ReadOutcome::Found(v),
            None => ReadOutcome::NotFound,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::history::{PartitionRetention, Retention};
    use crate::lsm::config::LsmConfig;
    use acta_ids::TxId;

    async fn open(path: &std::path::Path) -> LsmStorage {
        let mut cfg = LsmConfig::default();
        cfg.path = path.to_path_buf();
        LsmStorage::new(cfg).await.expect("open")
    }

    #[tokio::test]
    async fn floor_is_durable_monotone_and_enforced() {
        let dir = tempfile::tempdir().expect("tmp");
        let s = open(dir.path()).await;
        for i in 0..5u64 {
            let tx = TxId::new(i + 1);
            s.put(tx, format!("k{i}").as_bytes(), b"v").await.expect("put");
            s.commit(tx).await.expect("commit");
        }
        let last = s.last_seq_no().await.expect("seq");
        assert!(s.raise_history_floor(0, last + 100).await.is_err());
        assert_eq!(s.raise_history_floor(0, 3).await.expect("raise"), 3);
        assert_eq!(s.raise_history_floor(0, 2).await.expect("noop"), 3);
        assert_eq!(
            s.get_at_seq_checked(0, b"k0", 2).await.expect("rd"),
            ReadOutcome::HorizonExceeded { floor: 3 }
        );
        assert!(s.get_at_seq_checked(0, b"k0", last).await.expect("rd").is_found());
        s.wait_shutdown().await;
        drop(s);

        let s2 = open(dir.path()).await;
        assert_eq!(s2.history_floor(0), 3, "floor must survive restart");
        // §4.3: Seq darf nach Neustart nie zurückfallen.
        assert!(s2.last_seq_no().await.expect("seq") >= last);
    }

    #[tokio::test]
    async fn keep_all_retains_old_versions_through_flush() {
        let dir = tempfile::tempdir().expect("tmp");
        let s = open(dir.path()).await;
        s.set_retention_policy(std::sync::Arc::new(
            PartitionRetention::new().with_retention(0, Retention::KeepAll),
        ));
        let mut first_seq = 0;
        for i in 0..4u64 {
            let tx = TxId::new(i + 1);
            s.put(tx, b"k", format!("v{i}").as_bytes()).await.expect("put");
            s.commit(tx).await.expect("commit");
            if i == 0 {
                first_seq = s.last_seq_no().await.expect("seq");
            }
        }
        s.flush().await.expect("flush");
        let v = s.get_at_seq(b"k", first_seq).await.expect("get");
        assert_eq!(v.as_deref(), Some(&b"v0"[..]));
    }

    #[tokio::test]
    async fn missing_manifest_with_sstables_fails_closed_and_tmp_is_cleaned() {
        let dir = tempfile::tempdir().expect("tmp");
        let s = open(dir.path()).await;
        let tx = TxId::new(1);
        s.put(tx, b"k", b"v").await.expect("put");
        s.commit(tx).await.expect("commit");
        s.flush().await.expect("flush");
        s.wait_shutdown().await;
        drop(s);

        // F5: verwaiste Builder-Tmp-Datei wird beim Start entfernt.
        let tmp = dir.path().join("sst-00000000000000000009-000000.sst.tmp.4242");
        std::fs::write(&tmp, b"partial").expect("w");
        let s = open(dir.path()).await;
        assert!(!tmp.exists(), "leftover .sst.tmp.<pid> must be removed");
        s.wait_shutdown().await;
        drop(s);

        // F6: MANIFEST weg, SST vorhanden => kein stiller Neuaufbau.
        std::fs::remove_file(dir.path().join("MANIFEST")).expect("rm");
        let mut cfg = LsmConfig::default();
        cfg.path = dir.path().to_path_buf();
        let res = LsmStorage::new(cfg).await;
        assert!(res.is_err(), "missing MANIFEST with SSTables must fail closed");
    }
}
