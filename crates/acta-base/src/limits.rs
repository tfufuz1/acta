//! Grenzwerte und Startwerte (Spec Anhang A). Pflichtparameter ohne Startwert (z. B. `T_BACKUP`,
//! `anchor_destroy_max`, `support_min`) stehen hier bewusst **nicht**: es gibt keine stillen
//! Standardwerte (0.4 Regel 1). Zeiten in Millisekunden, Groessen in Byte.

/// Zulaessiger Bereich eines Parameters (beide Grenzen inklusive).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bounds {
    /// Untere Grenze.
    pub min: u64,
    /// Obere Grenze.
    pub max: u64,
}

impl Bounds {
    /// `true`, wenn `v` im Bereich liegt.
    #[must_use]
    pub const fn contains(&self, v: u64) -> bool {
        v >= self.min && v <= self.max
    }
}

/// Eine Sekunde in Millisekunden.
pub const SECOND_MS: u64 = 1_000;
/// Eine Minute in Millisekunden.
pub const MINUTE_MS: u64 = 60 * SECOND_MS;
/// Eine Stunde in Millisekunden.
pub const HOUR_MS: u64 = 60 * MINUTE_MS;
/// Ein Tag in Millisekunden.
pub const DAY_MS: u64 = 24 * HOUR_MS;
/// Ein KiB.
pub const KIB: u64 = 1_024;
/// Ein MiB.
pub const MIB: u64 = 1_024 * KIB;
/// Ein GiB.
pub const GIB: u64 = 1_024 * MIB;

/// A.1 `clock_skew_max` (4.6): Startwert 5 min.
pub const CLOCK_SKEW_MAX_MS: u64 = 5 * MINUTE_MS;
/// A.1 `clock_skew_max`: Bereich 1 bis 30 min.
pub const CLOCK_SKEW_MAX_BOUNDS: Bounds = Bounds {
    min: MINUTE_MS,
    max: 30 * MINUTE_MS,
};
/// A.1 `max_subjects_per_doc` (4.2): Startwert 64.
pub const MAX_SUBJECTS_PER_DOC: u64 = 64;
/// A.1 `max_subjects_per_doc`: Bereich 2 bis 256.
pub const MAX_SUBJECTS_PER_DOC_BOUNDS: Bounds = Bounds { min: 2, max: 256 };
/// A.1 `dek_cache_max` (3.6): Startwert 4 096 Eintraege.
pub const DEK_CACHE_MAX: u64 = 4_096;
/// A.1 `dek_cache_max`: Bereich 256 bis 65 536.
pub const DEK_CACHE_MAX_BOUNDS: Bounds = Bounds {
    min: 256,
    max: 65_536,
};
/// A.1 Argon2id (`FileAnchor`): Speicher in KiB, nur heraufsetzbar.
pub const ARGON2_MIN_MEMORY_KIB: u32 = 65_536;
/// A.1 Argon2id: Durchlaeufe, nur heraufsetzbar.
pub const ARGON2_MIN_PASSES: u32 = 3;
/// A.1 Argon2id: Parallelitaet, nur heraufsetzbar.
pub const ARGON2_MIN_LANES: u32 = 4;
/// A.1 Passphrase-Entropie in Bit, nur heraufsetzbar.
pub const PASSPHRASE_MIN_ENTROPY_BITS: u32 = 80;

/// A.2 `checkpoint_every` (6.2): Startwert 1 024 Eintraege.
pub const CHECKPOINT_EVERY: u64 = 1_024;
/// A.2 `checkpoint_every`: Bereich 64 bis 65 536.
pub const CHECKPOINT_EVERY_BOUNDS: Bounds = Bounds {
    min: 64,
    max: 65_536,
};
/// A.2 `gate_propagation_max` (9.5): fest 1 Rahmen.
pub const GATE_PROPAGATION_MAX_FRAMES: u64 = 1;

/// A.3 `wal_max_bytes` (5.1): Startwert 64 MiB.
pub const WAL_MAX_BYTES: u64 = 64 * MIB;
/// A.3 `wal_max_bytes`: Bereich 4 MiB bis 1 GiB.
pub const WAL_MAX_BYTES_BOUNDS: Bounds = Bounds {
    min: 4 * MIB,
    max: GIB,
};
/// A.3 `segment_max_bytes` (5.1): Startwert 256 MiB.
pub const SEGMENT_MAX_BYTES: u64 = 256 * MIB;
/// A.3 `segment_max_bytes`: Bereich 16 MiB bis 1 GiB.
pub const SEGMENT_MAX_BYTES_BOUNDS: Bounds = Bounds {
    min: 16 * MIB,
    max: GIB,
};
/// A.3 Segmentblock (Klartext): fest 64 KiB je `format_version`.
pub const SEGMENT_BLOCK_BYTES: u64 = 64 * KIB;
/// A.3 `group_commit_window` (3.6): Startwert 2 ms.
pub const GROUP_COMMIT_WINDOW_MS: u64 = 2;
/// A.3 `group_commit_window`: Bereich 0 bis 20 ms.
pub const GROUP_COMMIT_WINDOW_BOUNDS: Bounds = Bounds { min: 0, max: 20 };
/// A.3 `reader_drain_max` (3.6): Startwert 30 s.
pub const READER_DRAIN_MAX_MS: u64 = 30 * SECOND_MS;
/// A.3 `reader_drain_max`: Bereich 5 s bis 5 min.
pub const READER_DRAIN_MAX_BOUNDS: Bounds = Bounds {
    min: 5 * SECOND_MS,
    max: 5 * MINUTE_MS,
};
/// A.3 `bridge_queue_max` (3.6): Startwert 1 024.
pub const BRIDGE_QUEUE_MAX: u64 = 1_024;
/// A.3 `bridge_queue_max`: Bereich 64 bis 65 536.
pub const BRIDGE_QUEUE_MAX_BOUNDS: Bounds = Bounds {
    min: 64,
    max: 65_536,
};
/// A.3 `min_free_bytes` (8.3): Startwert 1 GiB.
pub const MIN_FREE_BYTES: u64 = GIB;
/// A.3 `min_free_bytes`: Bereich 64 MiB bis 100 GiB.
pub const MIN_FREE_BYTES_BOUNDS: Bounds = Bounds {
    min: 64 * MIB,
    max: 100 * GIB,
};

/// A.4 `erase_return_timeout` (8.3): Startwert 2 s.
pub const ERASE_RETURN_TIMEOUT_MS: u64 = 2 * SECOND_MS;
/// A.4 `erase_return_timeout`: Bereich 100 ms bis 10 s.
pub const ERASE_RETURN_TIMEOUT_BOUNDS: Bounds = Bounds {
    min: 100,
    max: 10 * SECOND_MS,
};
/// A.4 `saga_retry_max` (8.3): Startwert 8.
pub const SAGA_RETRY_MAX: u64 = 8;
/// A.4 `saga_retry_max`: Bereich 1 bis 100.
pub const SAGA_RETRY_MAX_BOUNDS: Bounds = Bounds { min: 1, max: 100 };
/// A.4 Backoff der Saga, untere Grenze: 1 s.
pub const SAGA_BACKOFF_MIN_MS: u64 = SECOND_MS;
/// A.4 Backoff der Saga, obere Grenze: 1 h.
pub const SAGA_BACKOFF_MAX_MS: u64 = HOUR_MS;
/// A.4 `quarantine_max` (2.2): Startwert 24 h.
pub const QUARANTINE_MAX_MS: u64 = 24 * HOUR_MS;
/// A.4 `quarantine_max`: Bereich 1 bis 72 h.
pub const QUARANTINE_MAX_BOUNDS: Bounds = Bounds {
    min: HOUR_MS,
    max: 72 * HOUR_MS,
};
/// A.4 `quarantine_purge_after` (2.2): Startwert 7 Tage.
pub const QUARANTINE_PURGE_AFTER_MS: u64 = 7 * DAY_MS;
/// A.4 `quarantine_purge_after`: Bereich 2 bis 30 Tage.
pub const QUARANTINE_PURGE_AFTER_BOUNDS: Bounds = Bounds {
    min: 2 * DAY_MS,
    max: 30 * DAY_MS,
};

/// A.2 `commit_epoch` (6.2): Startwert 30 Tage.
pub const COMMIT_EPOCH_MS: u64 = 30 * DAY_MS;
/// A.2 `commit_epoch`: Bereich 7 bis 90 Tage.
pub const COMMIT_EPOCH_BOUNDS: Bounds = Bounds {
    min: 7 * DAY_MS,
    max: 90 * DAY_MS,
};
/// A.2 `commit_key_retention` (6.2, 15.4): Startwert 90 Tage.
pub const COMMIT_KEY_RETENTION_START_MS: u64 = 90 * DAY_MS;
/// R-RETMAX (15.4): Obergrenze 3 Jahre, hier zu 1 095 Tagen gerechnet.
pub const COMMIT_KEY_RETENTION_MAX_MS: u64 = 1_095 * DAY_MS;
/// A.2 `ops_log_retention` (13.5): Startwert 90 Tage.
pub const OPS_LOG_RETENTION_MS: u64 = 90 * DAY_MS;
/// A.2 `ops_log_retention`: Bereich 7 bis 365 Tage.
pub const OPS_LOG_RETENTION_BOUNDS: Bounds = Bounds {
    min: 7 * DAY_MS,
    max: 365 * DAY_MS,
};
/// A.3 `wal_max_age` (5.1): Startwert 30 s.
pub const WAL_MAX_AGE_MS: u64 = 30 * SECOND_MS;
/// A.3 `wal_max_age`: Bereich 1 s bis 10 min.
pub const WAL_MAX_AGE_BOUNDS: Bounds = Bounds {
    min: SECOND_MS,
    max: 10 * MINUTE_MS,
};
/// A.3 `cluster_seg_min` (5.1): Startwert 4 MiB.
pub const CLUSTER_SEG_MIN_BYTES: u64 = 4 * MIB;
/// A.3 `cluster_seg_min`: Bereich 256 KiB bis 64 MiB.
pub const CLUSTER_SEG_MIN_BOUNDS: Bounds = Bounds {
    min: 256 * KIB,
    max: 64 * MIB,
};
/// A.3 `dp_cluster_min_bytes` (5.5): Startwert 1 MiB.
pub const DP_CLUSTER_MIN_BYTES: u64 = MIB;
/// A.3 `dp_cluster_min_bytes`: Bereich 64 KiB bis 16 MiB.
pub const DP_CLUSTER_MIN_BOUNDS: Bounds = Bounds {
    min: 64 * KIB,
    max: 16 * MIB,
};
/// A.3 `ingest_idem_ttl` (5.4): Startwert 7 Tage.
pub const INGEST_IDEM_TTL_MS: u64 = 7 * DAY_MS;
/// A.3 `ingest_idem_ttl`: Bereich 1 bis 30 Tage.
pub const INGEST_IDEM_TTL_BOUNDS: Bounds = Bounds {
    min: DAY_MS,
    max: 30 * DAY_MS,
};
/// A.4 `erase_sla_B` (8.7): Startwert 24 h.
pub const ERASE_SLA_B_MS: u64 = 24 * HOUR_MS;
/// A.4 `erase_sla_B`: Bereich 1 bis 72 h.
pub const ERASE_SLA_B_BOUNDS: Bounds = Bounds {
    min: HOUR_MS,
    max: 72 * HOUR_MS,
};
/// A.4 `multi_review_max` (8.9): Startwert 10 Werktage.
pub const MULTI_REVIEW_MAX_WORKDAYS: u64 = 10;
/// A.4 `multi_review_max`: Bereich 1 bis 20 Werktage.
pub const MULTI_REVIEW_MAX_BOUNDS: Bounds = Bounds { min: 1, max: 20 };
/// A.4 `measure_retries` (8.5): Startwert 3.
pub const MEASURE_RETRIES: u64 = 3;
/// A.4 `measure_retries`: Bereich 1 bis 10.
pub const MEASURE_RETRIES_BOUNDS: Bounds = Bounds { min: 1, max: 10 };
/// A.4 `hold_review_interval` (8.4): Startwert 90 Tage.
pub const HOLD_REVIEW_INTERVAL_MS: u64 = 90 * DAY_MS;
/// A.4 `hold_review_interval`: Bereich 30 bis 365 Tage.
pub const HOLD_REVIEW_INTERVAL_BOUNDS: Bounds = Bounds {
    min: 30 * DAY_MS,
    max: 365 * DAY_MS,
};
/// A.4 `archive_wait_max` (8.4): Startwert 30 Tage.
pub const ARCHIVE_WAIT_MAX_MS: u64 = 30 * DAY_MS;
/// A.4 `archive_wait_max`: Bereich 7 bis 180 Tage.
pub const ARCHIVE_WAIT_MAX_BOUNDS: Bounds = Bounds {
    min: 7 * DAY_MS,
    max: 180 * DAY_MS,
};
/// A.4 `outbox_ttl` (8.6): Startwert 90 Tage.
pub const OUTBOX_TTL_MS: u64 = 90 * DAY_MS;
/// A.4 `outbox_ttl`: Bereich 30 bis 365 Tage.
pub const OUTBOX_TTL_BOUNDS: Bounds = Bounds {
    min: 30 * DAY_MS,
    max: 365 * DAY_MS,
};

/// A.5 `K_MIN` (7.5): Startwert und Untergrenze 5.
pub const K_MIN: u64 = 5;

/// A.7 Eingabegroesse je Dokument (9.1): 512 MiB.
pub const INGEST_INPUT_MAX_BYTES: u64 = 512 * MIB;
/// A.7 CPU-Zeit je Dokument (9.1): 60 s.
pub const INGEST_CPU_MAX_MS: u64 = 60 * SECOND_MS;
/// A.7 Speicher des Workers (9.1): 2 GiB.
pub const WORKER_MEMORY_MAX_BYTES: u64 = 2 * GIB;
/// A.7 Expansionsfaktor (9.1): 100:1.
pub const INGEST_EXPANSION_MAX: u64 = 100;
/// A.7 Verschachtelungstiefe (9.1): 8.
pub const INGEST_NESTING_MAX: u64 = 8;
/// A.7 Seiten je Dokument (9.1): 10 000.
pub const INGEST_PAGES_MAX: u64 = 10_000;
/// A.7 Rahmen im Worker-Protokoll (9.1): hoechstens 64 MiB.
pub const WORKER_FRAME_MAX_BYTES: u64 = 64 * MIB;

const _: () = assert!(SAGA_BACKOFF_MIN_MS < SAGA_BACKOFF_MAX_MS);
// R-RETMAX (15.4): Startwert der Schlüsselaufbewahrung liegt unter der Obergrenze.
const _: () = assert!(COMMIT_KEY_RETENTION_START_MS <= COMMIT_KEY_RETENTION_MAX_MS);

#[cfg(test)]
mod tests {
    use super::*;

    // SPEC: Anhang A
    #[test]
    fn start_values_lie_inside_their_bounds() {
        let pairs = [
            (CLOCK_SKEW_MAX_MS, CLOCK_SKEW_MAX_BOUNDS),
            (MAX_SUBJECTS_PER_DOC, MAX_SUBJECTS_PER_DOC_BOUNDS),
            (DEK_CACHE_MAX, DEK_CACHE_MAX_BOUNDS),
            (CHECKPOINT_EVERY, CHECKPOINT_EVERY_BOUNDS),
            (WAL_MAX_BYTES, WAL_MAX_BYTES_BOUNDS),
            (SEGMENT_MAX_BYTES, SEGMENT_MAX_BYTES_BOUNDS),
            (GROUP_COMMIT_WINDOW_MS, GROUP_COMMIT_WINDOW_BOUNDS),
            (READER_DRAIN_MAX_MS, READER_DRAIN_MAX_BOUNDS),
            (BRIDGE_QUEUE_MAX, BRIDGE_QUEUE_MAX_BOUNDS),
            (MIN_FREE_BYTES, MIN_FREE_BYTES_BOUNDS),
            (ERASE_RETURN_TIMEOUT_MS, ERASE_RETURN_TIMEOUT_BOUNDS),
            (SAGA_RETRY_MAX, SAGA_RETRY_MAX_BOUNDS),
            (QUARANTINE_MAX_MS, QUARANTINE_MAX_BOUNDS),
            (QUARANTINE_PURGE_AFTER_MS, QUARANTINE_PURGE_AFTER_BOUNDS),
            (
                COMMIT_KEY_RETENTION_START_MS,
                Bounds {
                    min: 0,
                    max: COMMIT_KEY_RETENTION_MAX_MS,
                },
            ),
        ];
        for (v, b) in pairs {
            assert!(b.min <= b.max);
            assert!(b.contains(v), "{v} nicht in {b:?}");
        }
    }

    // SPEC: Anhang A
    #[test]
    fn bounds_are_inclusive_and_reject_outside() {
        let b = Bounds { min: 2, max: 5 };
        assert!(b.contains(2) && b.contains(5));
        assert!(!b.contains(1) && !b.contains(6));
    }

    // SPEC: Anhang A
    #[test]
    fn unit_constants() {
        assert_eq!(DAY_MS, 86_400_000);
        assert_eq!(GIB, 1_073_741_824);
    }
}
