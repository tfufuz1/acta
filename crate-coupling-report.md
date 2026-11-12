# Crate-Kopplungsbericht

- Erzeugt: 2026-11-06 13:17
- Workspace-Root: `/home/freddy/Projekte/acta`
- Analysierte Crates: 16 · Komponenten-Tiefe: 1 · Zeilen (ohne Leerzeilen): 140.798
- Legende: **prod** = src/bin/build ohne `cfg(test)`; **dev** = tests/benches/examples/fuzz und `cfg(test)`-Code.
- Hinweis: Textuelle Analyse ohne Compiler. Glob-Importe (`*`) sind nicht auflösbar. Siehe Kopf des Skripts für Grenzen.

## 1. Überblick

| Crate | Ring | Zeilen | Fan-out prod | Fan-in prod | Cargo-Deps intern (n/d/b) | Pub-Items | API-Lecks | Zyklus |
|---|---|---:|---:|---:|---|---:|---:|---|
| [acta](#acta) | 4 | 1867 | 5 | 0 | 3/0/0 | 29 | 8 | – |
| [acta-arch-tests](#acta-arch-tests) | tooling | 2882 | 0 | 0 | 0/0/0 | 34 | 0 | – |
| [acta-crypto](#acta-crypto) | 1 | 3021 | 2 | 4 | 2/0/0 | 82 | 0 | – |
| [acta-durable-fs](#acta-durable-fs) | 1 | 3563 | 2 | 1 | 3/0/0 | 34 | 0 | – |
| [acta-engine](#acta-engine) | 3 | 1079 | 2 | 1 | 10/1/0 | 18 | 3 | – |
| [acta-erasure](#acta-erasure) | 3 | 12879 | 3 | 1 | 3/0/0 | 194 | 24 | – |
| [acta-lab](#acta-lab) | tooling | 4152 | 0 | 0 | 0/0/0 | 75 | 0 | – |
| [acta-mvcc](#acta-mvcc) | 2 | 7715 | 2 | 1 | 2/0/0 | 30 | 2 | – |
| [acta-ports](#acta-ports) | 0 | 3934 | 1 | 8 | 1/0/0 | 77 | 2 | – |
| [acta-rank](#acta-rank) | 2 | 1125 | 0 | 0 | 1/0/0 | 8 | 0 | – |
| [acta-simd](#acta-simd) | 1 | 3219 | 1 | 1 | 1/0/0 | 35 | 15 | – |
| [acta-store](#acta-store) | 2 | 46059 | 5 | 1 | 5/0/0 | 130 | 9 | – |
| [acta-sys](#acta-sys) | 1 | 1934 | 0 | 2 | 0/0/0 | 29 | 0 | – |
| [acta-text](#acta-text) | 2 | 14290 | 3 | 0 | 3/0/0 | 78 | 8 | – |
| [acta-types](#acta-types) | 0 | 9747 | 0 | 10 | 0/0/0 | 119 | 0 | – |
| [acta-vector](#acta-vector) | 2 | 23332 | 4 | 0 | 4/0/0 | 39 | 4 | – |

## 2. Workspace-weite Auffälligkeiten

### 2.1 Zyklen

- Kein Zyklus im prod-Code-Graphen gefunden.
- Kein Zyklus in den normalen Cargo-Abhängigkeiten.

### 2.2 Abweichungen Cargo.toml ↔ Code

| Crate | Ziel | Deklariert als | Befund |
|---|---|---|---|
| `acta` | `acta-ports` | – | genutzt, aber nicht in Cargo.toml deklariert (cfg-/Feature-Pfad oder Fehler) |
| `acta` | `acta-store` | – | genutzt, aber nicht in Cargo.toml deklariert (cfg-/Feature-Pfad oder Fehler) |
| `acta-durable-fs` | `acta-types` | normal | nur in Tests/Benches genutzt, aber als normale Abhängigkeit deklariert → dev-dependency? |
| `acta-engine` | `acta-crypto` | normal | kein Code-Zugriff gefunden (prüfen: Makro, Feature, cfg, oder entfernen) |
| `acta-engine` | `acta-mvcc` | normal | kein Code-Zugriff gefunden (prüfen: Makro, Feature, cfg, oder entfernen) |
| `acta-engine` | `acta-ports` | normal | kein Code-Zugriff gefunden (prüfen: Makro, Feature, cfg, oder entfernen) |
| `acta-engine` | `acta-rank` | normal | kein Code-Zugriff gefunden (prüfen: Makro, Feature, cfg, oder entfernen) |
| `acta-engine` | `acta-store` | normal | kein Code-Zugriff gefunden (prüfen: Makro, Feature, cfg, oder entfernen) |
| `acta-engine` | `acta-sys` | normal | kein Code-Zugriff gefunden (prüfen: Makro, Feature, cfg, oder entfernen) |
| `acta-engine` | `acta-text` | normal | kein Code-Zugriff gefunden (prüfen: Makro, Feature, cfg, oder entfernen) |
| `acta-engine` | `acta-vector` | dev/normal | kein Code-Zugriff gefunden (prüfen: Makro, Feature, cfg, oder entfernen) |
| `acta-rank` | `acta-types` | normal | kein Code-Zugriff gefunden (prüfen: Makro, Feature, cfg, oder entfernen) |

### 2.3 Schwache Kanten (billig zu trennen)

Kanten im prod-Code mit höchstens 3 verschiedenen Symbolen. Reihenfolge: wenigste Symbole, dann wenigste Fundstellen.

| Von | Nach | Symbole | Fundstellen | Dateien | Symbole (prod) |
|---|---|---:|---:|---:|---|
| `acta-durable-fs` | `acta-sys` | 1 | 1 | 1 | `flush_dir` |
| `acta-crypto` | `acta-types` | 1 | 1 | 1 | `ActaError` |
| `acta-store` | `acta-sys` | 1 | 1 | 1 | `set_restrictive_file_acl` |
| `acta-text` | `acta-types` | 1 | 1 | 1 | `ActaError` |
| `acta-engine` | `acta-erasure` | 1 | 1 | 1 | `vault::FileVault` |
| `acta` | `acta-durable-fs` | 1 | 1 | 1 | `RealVfs` |
| `acta-ports` | `acta-types` | 1 | 2 | 2 | `ids::ErasureId`×2 |
| `acta-mvcc` | `acta-ports` | 1 | 3 | 1 | `MetricsSink`×3 |
| `acta-engine` | `acta-types` | 1 | 3 | 3 | `Result`×3 |
| `acta-durable-fs` | `acta-ports` | 2 | 2 | 2 | `clock`, `vfs` |
| `acta-crypto` | `acta-ports` | 2 | 2 | 1 | `BoxFuture`, `report_signer` |
| `acta-vector` | `acta-crypto` | 2 | 2 | 1 | `hierarchy::IdxKey`, `secret::SecretKey` |
| `acta` | `acta-store` | 2 | 2 | 1 | `tenant_codec::TenantScopedStorage`, `LsmStorage` |
| `acta-vector` | `acta-simd` | 2 | 3 | 3 | `validate_vector`×2, `kernels::scalar` |
| `acta-store` | `acta-crypto` | 2 | 5 | 2 | `verify_chain_complete`×4, `verify_wal_chain_completeness` |
| `acta-simd` | `acta-types` | 2 | 7 | 2 | `Result`×6, `ActaError` |
| `acta-text` | `acta-ports` | 2 | 7 | 5 | `StorageEngine`×6, `erasure` |
| `acta-mvcc` | `acta-types` | 2 | 28 | 3 | `TOMBSTONE_BIT`×27, `DocId` |

**Nur in Tests/Benches/Beispielen** (für das Produkt irrelevant, aber sie koppeln den Test-Build):

| Von | Nach | Fundstellen | Symbole |
|---|---|---:|---|
| `acta-durable-fs` | `acta-types` | 15 | `ActaError::Io`×12, `Result`, `ActaError::PolicyViolation`, `ActaError::Storage` |

### 2.4 Hub-Symbole (Kandidaten für types/ports)

Symbole, die aus einem Nicht-Basis-Crate von mindestens 3 verschiedenen Crates im prod-Code importiert werden. Wer sie in ein schmales Basis-Crate verschiebt, kann viele Kanten kappen.

Keine.

### 2.5 API-Lecks

Öffentliche Items, deren Signatur oder öffentliche Felder Typen anderer Workspace-Crates enthalten, sowie `pub use` fremder Crates. Diese Stellen machen eine Entkopplung am schwersten, weil Nutzer des Crates die fremde Abhängigkeit mitsehen.

| Crate | Komponente | Item | Fremde Crates | Fundstelle |
|---|---|---|---|---|
| `acta` | `api` | `struct Acta` | `acta-engine` | `src/api.rs:352` |
| `acta` | `api` | `struct Collection` | `acta-engine` | `src/api.rs:445` |
| `acta` | `api` | `pub use acta_types::secret::Secret` | `acta-types` | `src/api.rs:19` |
| `acta` | `error` | `enum ConfigDetails` | `acta-types` | `src/error.rs:81` |
| `acta` | `error` | `enum ConflictDetails` | `acta-types` | `src/error.rs:104` |
| `acta` | `error` | `enum LimitDetails` | `acta-types` | `src/error.rs:130` |
| `acta` | `error` | `enum Error` | `acta-types` | `src/error.rs:158` |
| `acta` | `wiring` | `struct SystemDefaults` | `acta-ports` | `src/wiring.rs:42` |
| `acta-engine` | `hardening` | `fn apply_process_hardening` | `acta-types` | `src/hardening.rs:6` |
| `acta-engine` | `startup` | `fn execute_startup_sequence` | `acta-types` | `src/startup.rs:10` |
| `acta-engine` | `wiring` | `fn create_vault` | `acta-erasure`, `acta-types` | `src/wiring.rs:9` |
| `acta-erasure` | `io` | `fn write_all_at` | `acta-ports` | `src/io.rs:11` |
| `acta-erasure` | `io` | `fn read_exact_at` | `acta-ports` | `src/io.rs:29` |
| `acta-erasure` | `io` | `fn zero_at` | `acta-ports` | `src/io.rs:54` |
| `acta-erasure` | `ledger` | `struct FinalizeParams` | `acta-types` | `src/ledger.rs:25` |
| `acta-erasure` | `ledger` | `fn finalize_erasure` | `acta-ports` | `src/ledger.rs:67` |
| `acta-erasure` | `measure::field` | `fn read_file_verified` | `acta-ports` | `src/measure/field.rs:202` |
| `acta-erasure` | `measure::field` | `fn collect_all_files` | `acta-ports` | `src/measure/field.rs:236` |
| `acta-erasure` | `measure::field` | `fn scan_mk1_structures` | `acta-ports` | `src/measure/field.rs:264` |
| `acta-erasure` | `measure::field` | `fn scan_mk2_raw` | `acta-ports` | `src/measure/field.rs:312` |
| `acta-erasure` | `measure::positive_control` | `fn run_positive_control` | `acta-types` | `src/measure/positive_control.rs:18` |
| `acta-erasure` | `measure::positive_control` | `fn run_positive_control_vfs` | `acta-ports`, `acta-types` | `src/measure/positive_control.rs:71` |
| `acta-erasure` | `measure::scanner_flow` | `fn scan_with_control` | `acta-ports` | `src/measure/scanner_flow.rs:29` |
| `acta-erasure` | `report::build` | `fn build_report_with_options` | `acta-types` | `src/report/build.rs:333` |
| `acta-erasure` | `report::build` | `fn build_report` | `acta-types` | `src/report/build.rs:484` |
| `acta-erasure` | `restore::replay` | `fn replay_erasures` | `acta-types` | `src/restore/replay.rs:21` |
| `acta-erasure` | `restricted` | `struct RestrictedReadAudit` | `acta-types` | `src/restricted.rs:125` |
| `acta-erasure` | `saga::core` | `struct ErasureDeps` | `acta-ports` | `src/saga/core.rs:74` |
| `acta-erasure` | `saga::driver` | `enum SagaEvent` | `acta-types` | `src/saga/driver.rs:13` |
| `acta-erasure` | `saga::driver` | `enum StepOutcome` | `acta-types` | `src/saga/driver.rs:24` |
| `acta-erasure` | `saga::machine` | `struct EraseRequest` | `acta-ports` | `src/saga/machine.rs:18` |
| `acta-erasure` | `scope::file` | `fn store_scope_file` | `acta-ports` | `src/scope/file.rs:30` |
| `acta-erasure` | `scope::file` | `fn load_scope_file` | `acta-ports` | `src/scope/file.rs:112` |
| `acta-erasure` | `vault::durable` | `pub use acta_ports::vfs::VerifiedVia` | `acta-ports` | `src/vault/durable.rs:10` |
| `acta-erasure` | `vault::rehash` | `fn rehash_durable_shard` | `acta-ports` | `src/vault/rehash.rs:72` |
| `acta-mvcc` | `seq_log` | `struct SeqLogEntry` | `acta-types` | `src/seq_log.rs:47` |
| `acta-mvcc` | `seq_log` | `enum SeqLogChange` | `acta-types` | `src/seq_log.rs:84` |
| `acta-ports` | `observer` | `struct ErasureEvent` | `acta-types` | `src/observer.rs:48` |
| `acta-ports` | `pre_gate` | `enum PreGateIntent` | `acta-types` | `src/pre_gate.rs:16` |
| `acta-simd` | `dispatch` | `fn cosine_distance` | `acta-types` | `src/dispatch.rs:102` |
| `acta-simd` | `dispatch` | `fn euclidean_distance` | `acta-types` | `src/dispatch.rs:126` |
| `acta-simd` | `dispatch` | `fn dot_product_distance` | `acta-types` | `src/dispatch.rs:150` |
| `acta-simd` | `dispatch` | `fn cosine_distance_f32_bytes` | `acta-types` | `src/dispatch.rs:174` |
| `acta-simd` | `dispatch` | `fn euclidean_distance_f32_bytes` | `acta-types` | `src/dispatch.rs:199` |
| `acta-simd` | `dispatch` | `fn dot_product_distance_f32_bytes` | `acta-types` | `src/dispatch.rs:224` |
| `acta-simd` | `dispatch` | `fn dot_product_u8` | `acta-types` | `src/dispatch.rs:249` |
| `acta-simd` | `dispatch` | `fn euclidean_distance_sq_u8` | `acta-types` | `src/dispatch.rs:261` |
| `acta-simd` | `dispatch` | `fn cosine_similarity_parts_u8` | `acta-types` | `src/dispatch.rs:273` |
| `acta-simd` | `(crate-root)` | `fn validate_vector` | `acta-types` | `src/lib.rs:41` |
| `acta-simd` | `(crate-root)` | `fn distance` | `acta-types` | `src/lib.rs:57` |
| `acta-simd` | `(crate-root)` | `fn distance_prevalidated` | `acta-types` | `src/lib.rs:65` |
| `acta-simd` | `(crate-root)` | `fn compute_distance` | `acta-types` | `src/lib.rs:82` |
| `acta-simd` | `(crate-root)` | `fn compute_distance_trusted` | `acta-types` | `src/lib.rs:98` |
| `acta-simd` | `(crate-root)` | `fn compute_distance_f32_bytes_trusted` | `acta-types` | `src/lib.rs:120` |
| `acta-store` | `lsm::engine` | `struct LsmStorage` | `acta-mvcc` | `src/lsm/engine.rs:34` |
| `acta-store` | `lsm::observer` | `struct CommitContext` | `acta-types` | `src/lsm/observer.rs:66` |
| `acta-store` | `lsm::tests` | `pub use acta_types::TOMBSTONE_BIT` | `acta-types` | `src/lsm/tests.rs:7` |
| `acta-store` | `purge::backend` | `struct WalFloor` | `acta-ports` | `src/purge/backend.rs:22` |
| `acta-store` | `purge::backend` | `struct RewriteOutcome` | `acta-ports` | `src/purge/backend.rs:30` |
| `acta-store` | `purge::rewrite` | `fn rewrite_sstable_without` | `acta-ports` | `src/purge/rewrite.rs:9` |
| `acta-store` | `purge::unlink` | `fn unlink_verified_sstable` | `acta-ports`, `acta-types` | `src/purge/unlink.rs:7` |
| `acta-store` | `purge::wal_seal` | `fn seal_active_wal` | `acta-types` | `src/purge/wal_seal.rs:8` |
| `acta-store` | `wal::chain_check` | `fn verify_wal_chain_completeness` | `acta-crypto` | `src/wal/chain_check.rs:13` |
| `acta-text` | `analyzer` | `fn compute_term_tag` | `acta-crypto` | `src/analyzer.rs:134` |
| `acta-text` | `buffer` | `fn seal_binding_envelope` | `acta-crypto` | `src/buffer.rs:9` |
| `acta-text` | `buffer` | `fn open_binding_envelope` | `acta-crypto` | `src/buffer.rs:51` |
| `acta-text` | `inverted::purge` | `struct TextPurger` | `acta-ports` | `src/inverted/purge.rs:133` |
| `acta-text` | `(crate-root)` | `struct Bm25Scorer` | `acta-ports` | `src/lib.rs:84` |
| `acta-text` | `member` | `fn compute_member_hash` | `acta-crypto` | `src/member.rs:12` |
| `acta-text` | `stream` | `struct Bm25CandidateStream` | `acta-ports` | `src/stream.rs:42` |
| `acta-text` | `wand` | `fn block_max_wand_search` | `acta-ports` | `src/wand.rs:261` |
| `acta-vector` | `candidate_stream` | `struct VectorCandidateStream` | `acta-ports` | `src/candidate_stream.rs:40` |
| `acta-vector` | `hnsw::arena` | `struct HnswArena` | `acta-ports` | `src/hnsw/arena.rs:53` |
| `acta-vector` | `persistence::snapshot` | `fn save_snapshot_vfs` | `acta-ports` | `src/persistence/snapshot.rs:138` |
| `acta-vector` | `persistence::snapshot` | `fn load_snapshot_vfs` | `acta-ports` | `src/persistence/snapshot.rs:148` |

## 3. Kantenliste

| Von | Nach | prod Fundstellen | prod Symbole | dev Fundstellen | Cargo-Art |
|---|---|---:|---:|---:|---|
| `acta-vector` | `acta-types` | 57 | 8 | 47 | normal |
| `acta-erasure` | `acta-ports` | 56 | 25 | 92 | normal |
| `acta-store` | `acta-ports` | 51 | 13 | 148 | normal |
| `acta-erasure` | `acta-types` | 32 | 12 | 30 | normal |
| `acta-store` | `acta-types` | 32 | 13 | 103 | normal |
| `acta` | `acta-types` | 29 | 26 | 18 | normal |
| `acta-mvcc` | `acta-types` | 28 | 2 | 13 | normal |
| `acta-vector` | `acta-ports` | 26 | 11 | 71 | normal |
| `acta-store` | `acta-mvcc` | 10 | 6 | 7 | normal |
| `acta-erasure` | `acta-crypto` | 9 | 5 | 0 | normal |
| `acta-text` | `acta-crypto` | 8 | 7 | 11 | normal |
| `acta-simd` | `acta-types` | 7 | 2 | 0 | normal |
| `acta-text` | `acta-ports` | 7 | 2 | 65 | normal |
| `acta` | `acta-engine` | 5 | 4 | 0 | normal |
| `acta-store` | `acta-crypto` | 5 | 2 | 0 | normal |
| `acta` | `acta-ports` | 4 | 4 | 0 | – |
| `acta-engine` | `acta-types` | 3 | 1 | 4 | normal |
| `acta-mvcc` | `acta-ports` | 3 | 1 | 7 | normal |
| `acta-vector` | `acta-simd` | 3 | 2 | 7 | normal |
| `acta` | `acta-store` | 2 | 2 | 0 | – |
| `acta-crypto` | `acta-ports` | 2 | 2 | 1 | normal |
| `acta-durable-fs` | `acta-ports` | 2 | 2 | 24 | normal |
| `acta-ports` | `acta-types` | 2 | 1 | 0 | normal |
| `acta-vector` | `acta-crypto` | 2 | 2 | 0 | normal |
| `acta` | `acta-durable-fs` | 1 | 1 | 0 | normal |
| `acta-crypto` | `acta-types` | 1 | 1 | 0 | normal |
| `acta-durable-fs` | `acta-sys` | 1 | 1 | 3 | normal |
| `acta-engine` | `acta-erasure` | 1 | 1 | 0 | normal |
| `acta-store` | `acta-sys` | 1 | 1 | 2 | normal |
| `acta-text` | `acta-types` | 1 | 1 | 113 | normal |
| `acta-durable-fs` | `acta-types` | 0 | 0 | 15 | normal |

## 4. Crates im Detail

### acta

- Pfad: `crates/acta` · Zeilen: 1867 · Rust-Dateien: 19 · Ring: 4
- Interne Cargo-Abhängigkeiten: `acta-durable-fs` (normal), `acta-engine` (normal), `acta-types` (normal)

**Ausgehend (Importe aus anderen Workspace-Crates)**

| Ziel | prod | dev | Dateien (prod) | Symbole (prod) | Symbole (nur dev) |
|---|---:|---:|---:|---|---|
| `acta-types` | 29 | 18 | 2 | `TenantId`×2, `error::Error`×2, `ActaError`×2, `secret::Secret`, `erasure`, `ids`, `refs`, `error`, `error::Error::Config`, `error::Error::Io` … (+16) | `ids::Timestamp::from_secs`×5, `refs::OperatorRef::try_new`×4, `ids::Timestamp::now`×4, `ids::Timestamp`, `TenantId::from_bytes` … (+2) |
| `acta-engine` | 5 | 0 | 1 | `Acta`×2, `ActaConfig`, `Acta::open_with_config`, `Collection` | – |
| `acta-ports` | 4 | 0 | 1 | `clock::Clock`, `clock::SystemClock`, `rng::Rng`, `vfs::Vfs` | – |
| `acta-store` | 2 | 0 | 1 | `tenant_codec::TenantScopedStorage`, `LsmStorage` | – |
| `acta-durable-fs` | 1 | 0 | 1 | `RealVfs` | – |

**Pro Komponente**

- `api` (546 Zeilen)
  - → `acta-engine`: prod: `Acta`×2, `ActaConfig`, `Acta::open_with_config`, `Collection`
  - → `acta-store`: prod: `tenant_codec::TenantScopedStorage`, `LsmStorage`
  - → `acta-types`: prod: `TenantId`×2, `secret::Secret`, `erasure`, `ids`, `refs`
- `error` (307 Zeilen)
  - → `acta-types`: prod: `error::Error`×2, `ActaError`×2, `error`, `error::Error::Config`, `error::Error::Io`, `error::Error::Corrupt`, `error::Error::Auth`, `error::Error::Erased`, `error::Error::Restricted`, `error::Error::NotFound` … (+11)
- `wiring` (55 Zeilen)
  - → `acta-durable-fs`: prod: `RealVfs`
  - → `acta-ports`: prod: `clock::Clock`, `clock::SystemClock`, `rng::Rng`, `vfs::Vfs`
- `tests/api_signature_test.rs` (135 Zeilen)
  - → `acta-types`: dev: `refs::OperatorRef::try_new`×4, `ids::Timestamp::now`×4
- `tests/deletion_e2e.rs` (36 Zeilen)
  - → `acta-types`: dev: `ids::Timestamp`, `refs`
- `tests/no_silent_success.rs` (265 Zeilen)
  - → `acta-types`: dev: `ids::Timestamp::from_secs`×5, `TenantId::from_bytes`, `ids::ClusterTag::new`, `ids::DocTag::new`

**Öffentliche Schnittstelle** (erreichbar über `pub mod`-Kette oder `pub use`)

- `api`
  - 3 enum, 17 struct
    - `struct SignedReport`
    - `struct CollectionOptions`
    - `struct Document`
    - `struct Hit`
    - `struct Filter` · 2 Methoden
    - `struct SearchPage`
    - `struct SnapshotId`
    - `enum ScanRange`
    - `enum Supersede`
    - `struct UpdateOptions`
    - `struct UpdateReceipt`
    - `enum RestrictedPurpose`
    - `struct RestrictedAccess`
    - `struct HealthReport`
    - `struct Missing`
    - `struct Set`
    - `struct EngineBuilder` · 9 Methoden
    - `struct Acta` · 10 Methoden ⚠ fremd: `acta-engine`
    - `struct Tenant` · 2 Methoden
    - `struct Collection` · 14 Methoden ⚠ fremd: `acta-engine`
- `error`
  - 5 enum, 1 type
    - `enum ErrorCode` · 1 Methoden
    - `enum ConfigDetails` ⚠ fremd: `acta-types`
    - `enum ConflictDetails` ⚠ fremd: `acta-types`
    - `enum LimitDetails` ⚠ fremd: `acta-types`
    - `enum Error` · 1 Methoden ⚠ fremd: `acta-types`
    - `type Result`
- `wiring`
  - 1 fn, 2 struct
    - `struct OsRng` · 1 Methoden
    - `struct SystemDefaults` ⚠ fremd: `acta-ports`
    - `fn system_defaults`
- ⚠ Re-Exporte **fremder Crates**: `acta_types::secret::Secret`

**Eingehend (wer nutzt dieses Crate)**

Wird von keinem anderen analysierten Workspace-Crate importiert.

**Nach außen ungenutzte öffentliche Items (Heuristik, 29 von 29)**: `SignedReport`, `CollectionOptions`, `Document`, `Hit`, `Filter`, `SearchPage`, `SnapshotId`, `ScanRange`, `Supersede`, `UpdateOptions`, `UpdateReceipt`, `RestrictedPurpose`, `RestrictedAccess`, `HealthReport`, `Missing`, `Set`, `EngineBuilder`, `Acta`, `Tenant`, `Collection`, `ErrorCode`, `ConfigDetails`, `ConflictDetails`, `LimitDetails`, `Error`, `Result`, `OsRng`, `SystemDefaults`, `system_defaults`

**Auffälligkeiten**

- acta-ports: genutzt, aber nicht in Cargo.toml deklariert (cfg-/Feature-Pfad oder Fehler)
- acta-store: genutzt, aber nicht in Cargo.toml deklariert (cfg-/Feature-Pfad oder Fehler)

### acta-arch-tests

- Pfad: `tests/arch` · Zeilen: 2882 · Rust-Dateien: 3 · Ring: tooling
- Interne Cargo-Abhängigkeiten: keine

**Ausgehend (Importe aus anderen Workspace-Crates)**

Keine.

**Pro Komponente**

Keine Importe aus anderen Workspace-Crates.

**Öffentliche Schnittstelle** (erreichbar über `pub mod`-Kette oder `pub use`)

- `(crate-root)`
  - 6 const, 1 enum, 23 fn, 1 struct
    - `enum Ring`
    - `const RING_TABLE`
    - `const UNSAFE_ISLANDS`
    - `const SPEC_5_3_ALLOWED_EDGES`
    - `const ALLOWLIST`
    - `struct Krate`
    - `fn ring_of`
    - `fn load_crates`
    - `fn check_layering`
    - `fn check_dependency_matrix_5_3`
    - `fn check_types_purity`
    - `fn check_network_dependencies`
    - `fn check_raw_storage_isolation`
    - `const WORDING_ALLOWLIST`
    - `fn normalize_wording_text`
    - `fn check_wording`
    - `const DIRECT_FS_ALLOWLIST`
    - `fn check_store_fs_usage`
    - `fn check_feature_hygiene`
    - `fn check_unsafe`
    - `fn check_crate_headers_and_descriptions`
    - `fn check_exports_and_dev_dependencies`
    - `fn check_naming_and_facade_structure`
    - `fn check_clean_source_tree`
    - `fn check_session_marks_and_timestamps`
    - `fn check_trademark_and_session_marks`
    - `fn check_plan_traceability`
    - `fn check_obsolete_evidence_terms`
    - `fn check_test_traceability`
    - `fn check_double_modules`
    - `fn check_ratchet_hygiene`
- `known_deviations`
  - 1 const, 1 fn, 1 struct
    - `struct Deviation`
    - `const KNOWN_DEVIATIONS`
    - `fn is_deviation_allowed`

**Eingehend (wer nutzt dieses Crate)**

Wird von keinem anderen analysierten Workspace-Crate importiert.

**Nach außen ungenutzte öffentliche Items (Heuristik, 34 von 34)**: `Deviation`, `KNOWN_DEVIATIONS`, `is_deviation_allowed`, `Ring`, `RING_TABLE`, `UNSAFE_ISLANDS`, `SPEC_5_3_ALLOWED_EDGES`, `ALLOWLIST`, `Krate`, `ring_of`, `load_crates`, `check_layering`, `check_dependency_matrix_5_3`, `check_types_purity`, `check_network_dependencies`, `check_raw_storage_isolation`, `WORDING_ALLOWLIST`, `normalize_wording_text`, `check_wording`, `DIRECT_FS_ALLOWLIST`, `check_store_fs_usage`, `check_feature_hygiene`, `check_unsafe`, `check_crate_headers_and_descriptions`, `check_exports_and_dev_dependencies`, `check_naming_and_facade_structure`, `check_clean_source_tree`, `check_session_marks_and_timestamps`, `check_trademark_and_session_marks`, `check_plan_traceability` …

### acta-crypto

- Pfad: `crates/acta-crypto` · Zeilen: 3021 · Rust-Dateien: 18 · Ring: 1
- Interne Cargo-Abhängigkeiten: `acta-ports` (normal), `acta-types` (normal)

**Ausgehend (Importe aus anderen Workspace-Crates)**

| Ziel | prod | dev | Dateien (prod) | Symbole (prod) | Symbole (nur dev) |
|---|---:|---:|---:|---|---|
| `acta-ports` | 2 | 1 | 1 | `BoxFuture`, `report_signer` | `report_signer::ReportSigner` |
| `acta-types` | 1 | 0 | 1 | `ActaError` | – |

**Pro Komponente**

- `error` (41 Zeilen)
  - → `acta-types`: prod: `ActaError`
- `sign` (147 Zeilen)
  - → `acta-ports`: prod: `BoxFuture`, `report_signer`
- `tests/wp32_spec_tests.rs` (1025 Zeilen)
  - → `acta-ports`: dev: `report_signer::ReportSigner`

**Öffentliche Schnittstelle** (erreichbar über `pub mod`-Kette oder `pub use`)

- `aead`
  - 5 fn, 4 struct, 1 trait
    - `struct SlotAad` · 1 Methoden
    - `fn wrap_master`
    - `fn unwrap_master`
    - `struct Block`
    - `struct BlockSealer` · 4 Methoden
    - `trait NonceSource` · Methoden: fill_nonce
    - `struct OsCsprng`
    - `fn seal_record`
    - `fn seal_record_with_nonce_source`
    - `fn open_record`
- `chain`
  - 5 fn
    - `fn chain_next`
    - `fn wal_mac`
    - `fn ledger_mac`
    - `fn verify_chain_complete`
    - `fn verify_wal_chain_completeness`
- `dek`
  - 1 fn, 2 struct, 2 type
    - `struct Dek`
    - `type Salt`
    - `type DocTag`
    - `struct AncestorSalts`
    - `fn derive_dek`
- `error`
  - 1 enum, 1 type
    - `enum CryptoError`
    - `type Result`
- `header`
  - 5 const, 2 struct
    - `const HEADER_SIZE`
    - `const HEADER_MAGIC`
    - `const HEADER_KIND`
    - `const HEADER_FORMAT_VERSION`
    - `const HEADER_ALG_IDS`
    - `struct HeaderSlot` · 3 Methoden
    - `struct Header` · 5 Methoden
- `hierarchy`
  - 1 enum, 12 struct
    - `struct MasterKey` · 3 Methoden
    - `struct EncKey`
    - `struct IdxKey`
    - `struct UnitKey`
    - `struct SignSeed`
    - `struct WalKey`
    - `struct LedgerKey`
    - `struct HdrKey`
    - `struct CommitKey`
    - `struct VaultMacKey`
    - `struct FileKey` · 2 Methoden
    - `enum FileKind`
    - `struct KeyHierarchy` · 13 Methoden
- `kdf`
  - 7 const, 2 fn, 1 struct
    - `const MIN_M_KIB`
    - `const MAX_M_KIB`
    - `const MIN_T`
    - `const MAX_T`
    - `const MIN_P`
    - `const MAX_P`
    - `const MIN_PASSPHRASE_LEN`
    - `struct KdfParams` · 3 Methoden
    - `fn derive_kek`
    - `fn hkdf_expand`
- `labels`
  - 23 const
    - `const TENANT_TAG`
    - `const COLL_TAG`
    - `const SUBJECT_TAG`
    - `const DOC_TAG`
    - `const TERM_TAG`
    - `const NAME_TAG`
    - `const COMMIT`
    - `const ENC`
    - `const IDX`
    - `const UNIT`
    - `const SIGN`
    - `const WAL`
    - `const LEDGER`
    - `const FILE`
    - `const FKMASK`
    - `const DEK`
    - `const DEK_SALT`
    - `const REC_AAD`
    - `const SLOT_AAD`
    - `const VAULT_MAC`
    - `const HDR`
    - `const TBIND_AAD`
    - `const REPORT_PREFIX`
- `secret`
  - 2 struct
    - `struct SecretKey` · 2 Methoden
    - `struct Secret` · 2 Methoden
- `sign`
  - 2 fn, 1 struct
    - `struct DerivedReportSigner` · 2 Methoden
    - `fn sign_report`
    - `fn verify_report`
- `tag`
  - 1 fn, 1 struct
  - … 2 weitere Items nicht aufgelistet (`--max-items`)
- Zusätzlich 1 `pub`-Items in privaten Modulen ohne Re-Export (nicht Teil der echten API).

**Eingehend (wer nutzt dieses Crate)**

| Nutzer | Symbole prod | Symbole dev |
|---|---|---|
| `acta-erasure` | `dek::Dek`×2, `secret::SecretKey`×2, `aead`×2, `hkdf_expand`×2, `SecretKey::new` | – |
| `acta-text` | `hierarchy::IdxKey`×2, `labels::TERM_TAG`, `tag`, `dek::Dek`, `labels::TBIND_AAD`, `aead::BlockSealer`, `hierarchy` | `secret::SecretKey`×3, `hierarchy::FileKey`×2 |
| `acta-store` | `verify_chain_complete`×4, `verify_wal_chain_completeness` | – |
| `acta-vector` | `hierarchy::IdxKey`, `secret::SecretKey` | – |

**Nach außen ungenutzte öffentliche Items (Heuristik, 71 von 82)**: `SlotAad`, `wrap_master`, `unwrap_master`, `Block`, `NonceSource`, `OsCsprng`, `seal_record`, `seal_record_with_nonce_source`, `open_record`, `chain_next`, `wal_mac`, `ledger_mac`, `Salt`, `DocTag`, `AncestorSalts`, `derive_dek`, `CryptoError`, `Result`, `HEADER_SIZE`, `HEADER_MAGIC`, `HEADER_KIND`, `HEADER_FORMAT_VERSION`, `HEADER_ALG_IDS`, `HeaderSlot`, `Header`, `MasterKey`, `EncKey`, `UnitKey`, `SignSeed`, `WalKey` …

### acta-durable-fs

- Pfad: `crates/acta-durable-fs` · Zeilen: 3563 · Rust-Dateien: 19 · Ring: 1
- Interne Cargo-Abhängigkeiten: `acta-ports` (normal), `acta-sys` (normal), `acta-types` (normal)

**Ausgehend (Importe aus anderen Workspace-Crates)**

| Ziel | prod | dev | Dateien (prod) | Symbole (prod) | Symbole (nur dev) |
|---|---:|---:|---:|---|---|
| `acta-ports` | 2 | 24 | 2 | `clock`, `vfs` | `Result`×14, `vfs::Vfs`, `Vfs`, `VfsOpenOptions`, `VfsFile` … (+5) |
| `acta-sys` | 1 | 3 | 1 | `flush_dir` | `FsClass`×2, `fs_class` |
| `acta-types` | 0 | 15 | 0 | – | `ActaError::Io`×12, `Result`, `ActaError::PolicyViolation`, `ActaError::Storage` |

**Pro Komponente**

- `(crate-root)` (102 Zeilen)
  - → `acta-sys`: prod: `flush_dir`
- `probe` (557 Zeilen)
  - → `acta-ports`: prod: `clock`
- `real_vfs` (322 Zeilen)
  - → `acta-ports`: prod: `vfs`
- `tests/df_arch_tests.rs` (153 Zeilen)
  - → `acta-ports`: dev: `vfs::Vfs`
- `tests/df_conformance_tests.rs` (89 Zeilen)
  - → `acta-ports`: dev: `Result`×14, `Vfs`, `VfsOpenOptions`, `VfsFile` … (+1)
- `tests/df_probe_tests.rs` (303 Zeilen)
  - → `acta-sys`: dev: `FsClass`
- `tests/durability_tests.rs` (128 Zeilen)
  - → `acta-ports`: dev: `vfs::VfsFile`
  - → `acta-sys`: dev: `FsClass`, `fs_class`
- `tests/recovery_aries_matrix.rs` (368 Zeilen)
  - → `acta-types`: dev: `ActaError::Io`×12, `Result`, `ActaError::PolicyViolation`, `ActaError::Storage`
- `tests/t_verify_tests.rs` (49 Zeilen)
  - → `acta-ports`: dev: `vfs`, `vfs::VerifiedVia::Direct`, `vfs::VerifiedVia::CacheDropped`, `vfs::VerifiedVia::Cached`

**Öffentliche Schnittstelle** (erreichbar über `pub mod`-Kette oder `pub use`)

- `(crate-root)`
  - 2 fn, 2 struct, 1 trait
    - `struct ScrubReceipt`
    - `trait FaultInjector` · Methoden: on_write, on_fsync, on_rename, on_dir_fsync
    - `struct NoFault`
    - `fn durable_remove_with_injector`
    - `fn durable_remove`
- `atomic`
  - 6 fn, 1 struct, 1 trait
    - `trait AtomicFaultInjector` · Methoden: on_write, on_sync_data, on_rename, on_sync_dir, on_unlink
    - `struct NoFault`
    - `fn create_dir_all_0700`
    - `fn write_atomic`
    - `fn atomic_replace`
    - `fn write_atomic_with_injector`
    - `fn clean_tmp`
    - `fn clean_tmp_with_injector`
- `dir_lock`
  - 1 const, 1 enum, 1 struct
    - `const DIR_LOCKED_MSG`
    - `enum LockMode`
    - `struct DirLock` · 5 Methoden
- `probe`
  - 4 enum, 3 fn, 1 struct
    - `enum Tier`
    - `enum UnsupportedFilesystem`
    - `enum ProbeVerdict`
    - `enum DetectedFs`
    - `struct FsProbe`
    - `fn probe_dir`
    - `fn free_space`
    - `fn verdict`
- `real_vfs`
  - 2 struct
    - `struct RealFile` · 3 Methoden
    - `struct RealVfs` · 1 Methoden
- `scrub`
  - 3 enum, 4 fn, 1 struct
    - `struct ScrubReceipt`
    - `enum ScrubPolicy`
    - `enum ScrubRefusal`
    - `enum ScrubOutcome`
    - `fn scrub_before_unlink`
    - `fn scrub_before_unlink_with_injector`
    - `fn scrub_if_safe`
    - `fn scrub_if_safe_with_injector`
- Zusätzlich 8 `pub`-Items in privaten Modulen ohne Re-Export (nicht Teil der echten API).

**Eingehend (wer nutzt dieses Crate)**

| Nutzer | Symbole prod | Symbole dev |
|---|---|---|
| `acta` | `RealVfs` | – |

**Nach außen ungenutzte öffentliche Items (Heuristik, 33 von 34)**: `AtomicFaultInjector`, `NoFault`, `create_dir_all_0700`, `write_atomic`, `atomic_replace`, `write_atomic_with_injector`, `clean_tmp`, `clean_tmp_with_injector`, `DIR_LOCKED_MSG`, `LockMode`, `DirLock`, `ScrubReceipt`, `FaultInjector`, `NoFault`, `durable_remove_with_injector`, `durable_remove`, `Tier`, `UnsupportedFilesystem`, `ProbeVerdict`, `DetectedFs`, `FsProbe`, `probe_dir`, `free_space`, `verdict`, `RealFile`, `ScrubReceipt`, `ScrubPolicy`, `ScrubRefusal`, `ScrubOutcome`, `scrub_before_unlink` …

**Auffälligkeiten**

- acta-types: nur in Tests/Benches genutzt, aber als normale Abhängigkeit deklariert → dev-dependency?

### acta-engine

- Pfad: `crates/acta-engine` · Zeilen: 1079 · Rust-Dateien: 17 · Ring: 3
- Interne Cargo-Abhängigkeiten: `acta-vector` (dev), `acta-crypto` (normal), `acta-erasure` (normal), `acta-mvcc` (normal), `acta-ports` (normal), `acta-rank` (normal), `acta-store` (normal), `acta-sys` (normal), `acta-text` (normal), `acta-types` (normal), `acta-vector` (normal)

**Ausgehend (Importe aus anderen Workspace-Crates)**

| Ziel | prod | dev | Dateien (prod) | Symbole (prod) | Symbole (nur dev) |
|---|---:|---:|---:|---|---|
| `acta-types` | 3 | 4 | 3 | `Result`×3 | `TenantId::try_new`×2, `erasure` |
| `acta-erasure` | 1 | 0 | 1 | `vault::FileVault` | – |

**Pro Komponente**

- `hardening` (13 Zeilen)
  - → `acta-types`: prod: `Result`
- `startup` (30 Zeilen)
  - → `acta-types`: prod: `Result`
- `wiring` (15 Zeilen)
  - → `acta-erasure`: prod: `vault::FileVault`
  - → `acta-types`: prod: `Result`
- `tests/builder_validation_tests.rs` (88 Zeilen)
  - → `acta-types`: dev: `TenantId::try_new`
- `tests/transaction_status_spec_tests.rs` (35 Zeilen)
  - → `acta-types`: dev: `Result`, `erasure`, `TenantId::try_new`

**Öffentliche Schnittstelle** (erreichbar über `pub mod`-Kette oder `pub use`)

- `builder`
  - 1 struct
    - `struct EngineBuilder` · 13 Methoden
- `collection`
  - 1 enum, 4 struct
    - `struct CollectionOptions`
    - `struct Document`
    - `enum ScanRange`
    - `struct StripedDocLocks` · 3 Methoden
    - `struct Collection` · 13 Methoden
- `hardening`
  - 1 fn
    - `fn apply_process_hardening` ⚠ fremd: `acta-types`
- `health`
  - 2 enum, 1 struct
    - `enum EngineState`
    - `enum DegradedReason`
    - `struct HealthReport`
- `instance`
  - 1 struct
    - `struct Acta` · 12 Methoden
- `search`
  - 1 enum, 2 struct
    - `enum HitSource`
    - `struct Hit`
    - `struct SearchPage`
- `startup`
  - 1 fn
    - `fn execute_startup_sequence` ⚠ fremd: `acta-types`
- `tenant`
  - 1 struct
    - `struct Tenant` · 3 Methoden
- `wiring`
  - 1 fn
    - `fn create_vault` ⚠ fremd: `acta-erasure`, `acta-types`
- `workers`
  - 1 struct
    - `struct BackgroundWorker` · 4 Methoden

**Eingehend (wer nutzt dieses Crate)**

| Nutzer | Symbole prod | Symbole dev |
|---|---|---|
| `acta` | `Acta`×2, `ActaConfig`, `Acta::open_with_config`, `Collection` | – |

**Nach außen ungenutzte öffentliche Items (Heuristik, 16 von 18)**: `EngineBuilder`, `CollectionOptions`, `Document`, `ScanRange`, `StripedDocLocks`, `apply_process_hardening`, `EngineState`, `DegradedReason`, `HealthReport`, `HitSource`, `Hit`, `SearchPage`, `execute_startup_sequence`, `Tenant`, `create_vault`, `BackgroundWorker`

**Auffälligkeiten**

- acta-crypto: kein Code-Zugriff gefunden (prüfen: Makro, Feature, cfg, oder entfernen)
- acta-mvcc: kein Code-Zugriff gefunden (prüfen: Makro, Feature, cfg, oder entfernen)
- acta-ports: kein Code-Zugriff gefunden (prüfen: Makro, Feature, cfg, oder entfernen)
- acta-rank: kein Code-Zugriff gefunden (prüfen: Makro, Feature, cfg, oder entfernen)
- acta-store: kein Code-Zugriff gefunden (prüfen: Makro, Feature, cfg, oder entfernen)
- acta-sys: kein Code-Zugriff gefunden (prüfen: Makro, Feature, cfg, oder entfernen)
- acta-text: kein Code-Zugriff gefunden (prüfen: Makro, Feature, cfg, oder entfernen)
- acta-vector: kein Code-Zugriff gefunden (prüfen: Makro, Feature, cfg, oder entfernen)

### acta-erasure

- Pfad: `crates/acta-erasure` · Zeilen: 12879 · Rust-Dateien: 80 · Ring: 3
- Interne Cargo-Abhängigkeiten: `acta-crypto` (normal), `acta-ports` (normal), `acta-types` (normal)

**Ausgehend (Importe aus anderen Workspace-Crates)**

| Ziel | prod | dev | Dateien (prod) | Symbole (prod) | Symbole (nur dev) |
|---|---:|---:|---:|---|---|
| `acta-ports` | 56 | 92 | 24 | `vfs::Vfs`×11, `ActaError::Io`×6, `gate`×5, `BoxFuture`×4, `vfs`×3, `keys`×3, `gate::Seq`×2, `gate::KeyBuf::new`×2, `clock`×2, `observer`×2 … (+15) | `Result`×51, `ActaError::NotFound`×4, `ledger::LedgerSink`, `report_signer::ReportSigner` |
| `acta-types` | 32 | 30 | 19 | `ids::ErasureId`×11, `erasure`×9, `erasure::LayerWitness`×2, `erasure::PendingReason`×2, `erasure::ControlEvidence`, `erasure::Residue`, `erasure::MeasureClass::Mk1`, `erasure::MeasureClass::Mk2`, `erasure::MeasureClass::Mk3`, `ids` … (+2) | `ActaError::NotFound`×6, `erasure::RetentionAttestation`×2, `erasure::Scanner::new`, `erasure::LayerId::new`, `erasure::ControlEvidence::observed` … (+1) |
| `acta-crypto` | 9 | 0 | 5 | `dek::Dek`×2, `secret::SecretKey`×2, `aead`×2, `hkdf_expand`×2, `SecretKey::new` | – |

**Pro Komponente**

- `gate` (829 Zeilen)
  - → `acta-ports`: prod: `gate`×3, `gate::KeyBuf::new`×2, `barrier::CommitBarrier`, `gate::Seq`
- `io` (59 Zeilen)
  - → `acta-ports`: prod: `vfs::VfsFile`
- `journal` (835 Zeilen)
  - → `acta-crypto`: prod: `dek::Dek`, `secret::SecretKey`, `aead`
  - → `acta-ports`: prod: `vfs`, `ActaError`, `vfs::Vfs`
  - → `acta-types`: prod: `ids::ErasureId`×2
- `ledger` (320 Zeilen)
  - → `acta-ports`: prod: `vfs::Vfs`, `gate`, `ledger`
  - → `acta-types`: prod: `erasure::LayerWitness`, `ids::ErasureId`
- `measure` (786 Zeilen)
  - → `acta-ports`: prod: `vfs::Vfs`×2, `vfs`, `clock`
  - → `acta-types`: prod: `erasure`×4, `erasure::ControlEvidence`, `erasure::Residue`
- `purge` (199 Zeilen)
  - → `acta-ports`: prod: `clock`, `observer`
- `replace` (126 Zeilen)
  - → `acta-types`: prod: `ids::ErasureId`
- `report` (1166 Zeilen)
  - → `acta-ports`: prod: `vfs::Vfs`, `BoxFuture`, `report_signer`
  - → `acta-types`: prod: `erasure::LayerWitness`, `ids::ErasureId`, `erasure::MeasureClass::Mk1`, `erasure::MeasureClass::Mk2`, `erasure::MeasureClass::Mk3`
- `restore` (107 Zeilen)
  - → `acta-ports`: prod: `gate::Seq`
  - → `acta-types`: prod: `ids::ErasureId`
- `restricted` (190 Zeilen)
  - → `acta-types`: prod: `ids::ErasureId`
- `saga` (1291 Zeilen)
  - → `acta-ports`: prod: `erasure`×2, `clock::Clock`, `id_gen::IdGen`, `barrier`, `observer`, `erasure::EraseTarget`, `gate::GateDecision`, `BoxFuture`
  - → `acta-types`: prod: `erasure`×5, `ids::ErasureId`×3, `erasure::PendingReason`×2, `ids`, `refs::OperatorRef::try_new`
- `scope` (306 Zeilen)
  - → `acta-crypto`: prod: `dek::Dek`, `secret::SecretKey`, `aead`
  - → `acta-ports`: prod: `vfs::Vfs`, `keys`
- `supersede` (153 Zeilen)
  - → `acta-types`: prod: `erasure::KeyCoverage`, `ids::ErasureId`
- `vault` (2439 Zeilen)
  - → `acta-crypto`: prod: `hkdf_expand`×2, `SecretKey::new`
  - → `acta-ports`: prod: `ActaError::Io`×6, `vfs::Vfs`×5, `BoxFuture`×2, `keys`×2, `vfs::VerifiedVia`, `vfs`, `vfs::VerifiedVia::Direct`, `vfs::VerifiedVia::CacheDropped`, `vfs::VerifiedVia::Cached`, `gate`
- `tests/crash_matrix_recovery.rs` (250 Zeilen)
  - → `acta-ports`: dev: `clock::Clock`, `gate::GateDecision`, `id_gen::IdGen`, `BoxFuture` … (+2)
  - → `acta-types`: dev: `erasure::RetentionAttestation`×2, `erasure::PendingReason`, `ids::ErasureId`
- `tests/gate_linearization_test.rs` (190 Zeilen)
  - → `acta-ports`: dev: `BoxFuture`, `barrier`, `gate`
- `tests/journal_durable_spec_tests.rs` (327 Zeilen)
  - → `acta-ports`: dev: `Result`×9, `ActaError::NotFound`×2, `vfs`
- `tests/ledger_spec_tests.rs` (124 Zeilen)
  - → `acta-ports`: dev: `Result`×8, `vfs::Vfs`×2, `ledger::LedgerSink`
  - → `acta-types`: dev: `ids::ErasureId`, `ActaError::NotFound`
- `tests/loom_gate.rs` (134 Zeilen)
  - → `acta-ports`: dev: `gate`
- `tests/measure_real_scan_tests.rs` (301 Zeilen)
  - → `acta-ports`: dev: `Result`×9, `vfs`
  - → `acta-types`: dev: `ActaError::NotFound`×2, `erasure`
- `tests/measure_spec_tests.rs` (237 Zeilen)
  - → `acta-ports`: dev: `Result`×9, `vfs`
  - → `acta-types`: dev: `ActaError::NotFound`×2, `erasure`
- `tests/report_canonical_spec_tests.rs` (180 Zeilen)
  - → `acta-ports`: dev: `report_signer::ReportSigner`
  - → `acta-types`: dev: `ids::ErasureId`, `erasure::Scanner::new`, `erasure::LayerId::new`, `erasure::MeasureClass::Mk1` … (+1)
- `tests/report_spec_tests.rs` (79 Zeilen)
  - → `acta-ports`: dev: `Result`×8, `vfs::Vfs`×2
  - → `acta-types`: dev: `ids::ErasureId`, `ActaError::NotFound`
- `tests/saga_core_orchestration.rs` (477 Zeilen)
  - → `acta-ports`: dev: `clock::Clock`, `gate::GateDecision`, `id_gen::IdGen`, `BoxFuture` … (+3)
  - → `acta-types`: dev: `refs::OperatorRef`, `erasure`, `ids`
- `tests/scope_codec_spec_tests.rs` (154 Zeilen)
  - → `acta-ports`: dev: `Result`×8, `ActaError::NotFound`×2, `vfs::Vfs`, `BoxFuture` … (+1)
- `tests/special_targets_test.rs` (247 Zeilen)
  - → `acta-types`: dev: `erasure::KeyCoverage`, `ids::ErasureId`
- `tests/t_gate.rs` (61 Zeilen)
  - → `acta-ports`: dev: `gate`
- `tests/t_measure.rs` (71 Zeilen)
  - → `acta-types`: dev: `erasure`
- `tests/t_observer.rs` (58 Zeilen)
  - → `acta-ports`: dev: `BoxFuture`
  - → `acta-types`: dev: `ids::ErasureId`
- `tests/t_report_ledger.rs` (63 Zeilen)
  - → `acta-ports`: dev: `gate`, `ledger`
  - → `acta-types`: dev: `ids::ErasureId`
- `tests/t_saga_journal.rs` (155 Zeilen)
  - → `acta-types`: dev: `erasure::PendingReason`, `ids::ErasureId`
- `tests/t_saga_spec_extensions.rs` (112 Zeilen)
  - → `acta-types`: dev: `ids::ErasureId`
- `tests/t_sealer_keyring.rs` (43 Zeilen)
  - → `acta-ports`: dev: `gate`, `keys`
- `tests/vault_spec_conformance.rs` (503 Zeilen)
  - → `acta-ports`: dev: `keys`, `vfs`

**Öffentliche Schnittstelle** (erreichbar über `pub mod`-Kette oder `pub use`)

- `error`
  - 1 enum
    - `enum ErasureError`
- `gate`
  - 2 enum, 3 fn, 6 struct
    - `struct BatchGateRequest` · 2 Methoden
    - `struct BatchGateOutcome`
    - `struct GateBatchCoordinator` · 2 Methoden
    - `fn bind_global_erased_set`
    - `fn add_batch`
    - `fn evaluate_restricted_access`
    - `struct ErasedSet` · 19 Methoden
    - `enum GatePredicate`
    - `enum GateLogEntry`
    - `struct GateLog` · 9 Methoden
    - `struct GateGuard` · 3 Methoden
- `io`
  - 3 fn
    - `fn write_all_at` ⚠ fremd: `acta-ports`
    - `fn read_exact_at` ⚠ fremd: `acta-ports`
    - `fn zero_at` ⚠ fremd: `acta-ports`
- `journal`
  - 2 enum, 2 fn, 6 struct, 1 trait
    - `struct DurableJournal` · 5 Methoden
    - `struct AppendLog` · 6 Methoden
    - `trait EpochKeys` · Methoden: commit_key
    - `fn compute_commitment`
    - `struct Epoch` · 2 Methoden
    - `enum RecordCode` · 3 Methoden
    - `enum TargetType` · 2 Methoden
    - `struct JournalRecord` · 3 Methoden
    - `struct JournalEntry` · 3 Methoden
    - `fn reduce`
    - `struct ErasureJournal` · 5 Methoden
- `ledger`
  - 1 const, 1 fn, 5 struct
    - `struct LedgerEntry` · 2 Methoden
    - `const GENESIS_HASH`
    - `struct InMemoryLedger` · 8 Methoden
    - `struct FinalizeParams` ⚠ fremd: `acta-types`
    - `struct FinalizeOutcome`
    - `fn finalize_erasure` ⚠ fremd: `acta-ports`
    - `struct InMemoryLedger` · 1 Methoden
- `measure`
  - 2 enum, 12 fn, 10 struct, 1 trait
    - `enum VerifiedVia` · 1 Methoden
    - `trait StructureDecoder` · Methoden: name, can_decode, decode_and_check
    - `struct ScanPlan` · 6 Methoden
    - `struct Mk3StatusInput`
    - `struct Mk1Result`
    - `struct Mk2Result`
    - `struct Mk3Result` · 1 Methoden
    - `fn scan_mk3_keys`
    - `fn read_file_verified` ⚠ fremd: `acta-ports`
    - `fn collect_all_files` ⚠ fremd: `acta-ports`
    - `fn scan_mk1_structures` ⚠ fremd: `acta-ports`
    - `fn scan_mk2_raw` ⚠ fremd: `acta-ports`
    - `fn run_positive_control` ⚠ fremd: `acta-types`
    - `fn run_positive_control_vfs` ⚠ fremd: `acta-ports`, `acta-types`
    - `fn verify_scanner_integrity`
    - `fn scan_with_control` ⚠ fremd: `acta-ports`
    - `fn run_scanner_flow`
    - `fn arm_scanner`
    - `fn finish_scan`
    - `enum EvidenceKind`
    - `struct LocalControlEvidence` · 1 Methoden
    - `struct UnarmedScanner` · 3 Methoden
    - `struct ArmedScanner` · 1 Methoden
    - `struct TestWitness` · 1 Methoden
    - `struct MeasurementCoordinator` · 1 Methoden
- `purge`
  - 1 enum, 4 fn, 4 struct
    - `struct PurgeCx` · 4 Methoden
    - `struct PurgePriority`
    - `struct LayerPurgeOrder` · 1 Methoden
    - `fn sort_by_priority`
    - `enum ReserveCheck`
    - `fn check_disk_reserve`
    - `struct WindowOutcome` · 2 Methoden
    - `fn run_window`
    - `fn run_core_window`
- `replace`
  - 1 enum, 1 struct
    - `enum ReplaceStep`
    - `struct ReplacePlan` · 9 Methoden
- `report`
  - 2 enum, 11 fn, 11 struct, 1 type
    - `enum ReachStatus` · 1 Methoden
    - `struct LayerWitnessReport` · 1 Methoden
    - `type WitnessEntry`
    - `struct FieldMeasurementSection`
    - `struct LayerKeyCoverage`
    - `struct ProcessHardeningStatus`
    - `struct ErasureReport` · 2 Methoden
    - `struct ReportBuildOptions`
    - `fn build_report_with_options` ⚠ fremd: `acta-types`
    - `fn build_report` ⚠ fremd: `acta-types`
    - `fn canonicalize_report`
- `restore`
  - 1 enum, 1 fn, 3 struct
- `restricted`
  - 2 enum, 3 struct
- `saga`
  - 4 enum, 4 fn, 8 struct, 7 trait
- `scope`
  - 5 fn, 2 struct, 1 type
- `supersede`
  - 1 enum, 2 struct
- `vault`
  - 19 const, 3 enum, 20 fn, 12 struct, 1 trait, 1 type
  - … 114 weitere Items nicht aufgelistet (`--max-items`)
- Zusätzlich 4 `pub`-Items in privaten Modulen ohne Re-Export (nicht Teil der echten API).
- Interne Re-Exporte (`pub use`): `SagaState`, `SagaState`
- ⚠ Re-Exporte **fremder Crates**: `acta_ports::vfs::VerifiedVia`

**Eingehend (wer nutzt dieses Crate)**

| Nutzer | Symbole prod | Symbole dev |
|---|---|---|
| `acta-engine` | `vault::FileVault` | – |

**Nach außen ungenutzte öffentliche Items (Heuristik, 193 von 194)**: `ErasureError`, `BatchGateRequest`, `BatchGateOutcome`, `GateBatchCoordinator`, `bind_global_erased_set`, `add_batch`, `evaluate_restricted_access`, `ErasedSet`, `GatePredicate`, `GateLogEntry`, `GateLog`, `GateGuard`, `write_all_at`, `read_exact_at`, `zero_at`, `DurableJournal`, `AppendLog`, `EpochKeys`, `compute_commitment`, `Epoch`, `RecordCode`, `TargetType`, `JournalRecord`, `JournalEntry`, `reduce`, `ErasureJournal`, `LedgerEntry`, `GENESIS_HASH`, `InMemoryLedger`, `FinalizeParams` …

### acta-lab

- Pfad: `crates/acta-lab` · Zeilen: 4152 · Rust-Dateien: 33 · Ring: tooling
- Interne Cargo-Abhängigkeiten: keine

**Ausgehend (Importe aus anderen Workspace-Crates)**

Keine.

**Pro Komponente**

Keine Importe aus anderen Workspace-Crates.

**Öffentliche Schnittstelle** (erreichbar über `pub mod`-Kette oder `pub use`)

- `crypto_ref`
  - 8 fn, 1 type
    - `type HmacSha256`
    - `fn aes_256_gcm_siv_decrypt`
    - `fn aes_256_gcm_siv_encrypt`
    - `fn derive_argon2id`
    - `fn derive_hkdf_sha256`
    - `fn blake3_keyed_mac_16`
    - `fn blake3_keyed_mac_8`
    - `fn hmac_sha256`
    - `fn verify_ed25519_strict`
- `error`
  - 1 enum
    - `enum LabError`
- `fixture`
  - 2 struct
    - `struct LabDocumentFixture` · 1 Methoden
    - `struct LabStoreFixture` · 3 Methoden
- `format`
  - 18 const, 7 enum, 28 struct
    - `struct AppendLogEntry`
    - `struct AppendLogFile` · 2 Methoden
    - `const FILE_HEADER_SIZE`
    - `const FILE_MAGIC`
    - `const FORMAT_VERSION_V1`
    - `const STANDARD_ALG_IDS`
    - `enum FileKind` · 2 Methoden
    - `struct FileHeader` · 3 Methoden
    - `const HEADER_AB_SIZE`
    - `const HEADER_AB_MAGIC`
    - `const HEADER_AB_KIND`
    - `struct HeaderSlot` · 2 Methoden
    - `struct HeaderAb` · 2 Methoden
    - `const MIN_RECORD_ENVELOPE_SIZE`
    - `const RECORD_ENVELOPE_V2`
    - `const ALG_AES_256_GCM_SIV`
    - `struct RecordEnvelopeV2` · 3 Methoden
    - `enum ClassStatus`
    - `enum KeyCoverage`
    - `struct LayerKeyCoverage`
    - `struct ErasureReport` · 4 Methoden
    - `struct ScopeEntry`
    - `struct ScopeFile` · 2 Methoden
    - `const SSTABLE_FOOTER_SIZE`
    - `const SSTABLE_FOOTER_MAGIC`
    - `struct SsTableFooter` · 1 Methoden
    - `struct SsTableEntry` · 1 Methoden
    - `struct SsTableFile` · 1 Methoden
    - `const TEXT_SEGMENT_HEADER_SIZE`
    - `const TERM_ENTRY_SIZE`
    - `struct TextSegmentHeader` · 1 Methoden
    - `struct TermEntry` · 1 Methoden
    - `struct DocTableEntry` · 1 Methoden
    - `struct TextSegmentFile` · 1 Methoden
    - `const VAULT_SLOT_BASE`
    - `const VAULT_SLOT_SIZE`
    - `const VAULT_METABLOCK_SIZE`
    - `enum VaultSlotType` · 1 Methoden
    - `enum VaultSlotStatus` · 1 Methoden
    - `struct VaultSlot` · 2 Methoden
    - `struct VaultMetaBlock` · 3 Methoden
    - `struct VaultFile` · 1 Methoden
    - `const VECTOR_SNAPSHOT_HEADER_SIZE`
    - `enum VectorMetric` · 1 Methoden
    - `struct VectorSnapshotHeader` · 1 Methoden
    - `struct VectorSlotEntry`
    - `struct VectorSnapshotFile` · 1 Methoden
    - `struct WalFrame`
    - `struct WalSegmentHeader`
    - `struct WalSegment` · 1 Methoden
    - `enum ManifestEntryPayload`
    - `struct ManifestEntry`
    - `struct ManifestLog` · 1 Methoden
- `scanner`
  - 1 enum, 9 struct
    - `struct StructureTarget` · 1 Methoden
    - `struct Mk1StructureScanner` · 2 Methoden
    - `struct Mk2RawScanner` · 3 Methoden
    - `struct Mk3KeyProbeScanner` · 2 Methoden
    - `enum MeasurementClass`
    - `struct ControlEvidence` · 1 Methoden
    - `struct Unarmed`
    - `struct Armed`
    - `struct Scanner` · 4 Methoden
    - `struct LayerWitness`

**Eingehend (wer nutzt dieses Crate)**

Wird von keinem anderen analysierten Workspace-Crate importiert.

**Nach außen ungenutzte öffentliche Items (Heuristik, 75 von 75)**: `HmacSha256`, `aes_256_gcm_siv_decrypt`, `aes_256_gcm_siv_encrypt`, `derive_argon2id`, `derive_hkdf_sha256`, `blake3_keyed_mac_16`, `blake3_keyed_mac_8`, `hmac_sha256`, `verify_ed25519_strict`, `LabError`, `LabDocumentFixture`, `LabStoreFixture`, `AppendLogEntry`, `AppendLogFile`, `FILE_HEADER_SIZE`, `FILE_MAGIC`, `FORMAT_VERSION_V1`, `STANDARD_ALG_IDS`, `FileKind`, `FileHeader`, `HEADER_AB_SIZE`, `HEADER_AB_MAGIC`, `HEADER_AB_KIND`, `HeaderSlot`, `HeaderAb`, `MIN_RECORD_ENVELOPE_SIZE`, `RECORD_ENVELOPE_V2`, `ALG_AES_256_GCM_SIV`, `RecordEnvelopeV2`, `ClassStatus` …

### acta-mvcc

- Pfad: `crates/acta-mvcc` · Zeilen: 7715 · Rust-Dateien: 37 · Ring: 2
- Interne Cargo-Abhängigkeiten: `acta-ports` (normal), `acta-types` (normal)

**Ausgehend (Importe aus anderen Workspace-Crates)**

| Ziel | prod | dev | Dateien (prod) | Symbole (prod) | Symbole (nur dev) |
|---|---:|---:|---:|---|---|
| `acta-types` | 28 | 13 | 3 | `TOMBSTONE_BIT`×27, `DocId` | `TxId`×8, `DocId::from` |
| `acta-ports` | 3 | 7 | 1 | `MetricsSink`×3 | `Clock`×2, `SystemClock::new`×2, `gate`×2 |

**Pro Komponente**

- `seq_log` (695 Zeilen)
  - → `acta-types`: prod: `TOMBSTONE_BIT`×16, `DocId`
- `snapshot` (661 Zeilen)
  - → `acta-types`: prod: `TOMBSTONE_BIT` · dev: `TOMBSTONE_BIT`
- `ssi` (959 Zeilen)
  - → `acta-ports`: prod: `MetricsSink`×3
  - → `acta-types`: prod: `TOMBSTONE_BIT`×10
- `benches/bm_mvcc_seq.rs` (93 Zeilen)
  - → `acta-ports`: dev: `Clock`, `SystemClock::new`
- `benches/bm_mvcc_ssi.rs` (114 Zeilen)
  - → `acta-types`: dev: `TxId`
- `benches/mvcc_bench.rs` (72 Zeilen)
  - → `acta-types`: dev: `DocId`
- `tests/audit_regression_tests.rs` (96 Zeilen)
  - → `acta-types`: dev: `DocId::from`
- `tests/bench_latency_report.rs` (112 Zeilen)
  - → `acta-ports`: dev: `Clock`, `gate`, `SystemClock::new`
- `tests/campaign_cov_ssi_coarsening.rs` (50 Zeilen)
  - → `acta-types`: dev: `TxId`
- `tests/campaign_cov_tx_buffer_reap.rs` (60 Zeilen)
  - → `acta-types`: dev: `TxId`
- `tests/history_anomaly_checker.rs` (201 Zeilen)
  - → `acta-types`: dev: `TxId`, `TOMBSTONE_BIT`
- `tests/mv_audit_tests.rs` (117 Zeilen)
  - → `acta-types`: dev: `TxId`
- `tests/mv_ssi_validation_test.rs` (197 Zeilen)
  - → `acta-types`: dev: `TxId`
- `tests/race_write_purge.rs` (139 Zeilen)
  - → `acta-ports`: dev: `gate`
- `tests/reference_model_proptest.rs` (186 Zeilen)
  - → `acta-types`: dev: `TxId`, `DocId`
- `tests/ssi_hardening.rs` (160 Zeilen)
  - → `acta-types`: dev: `TxId`
- `tests/ssi_metrics_observability.rs` (130 Zeilen)
  - → `acta-ports`: dev: `MetricsSink`

**Öffentliche Schnittstelle** (erreichbar über `pub mod`-Kette oder `pub use`)

- `error`
  - 1 enum
    - `enum MvccError`
- `seq`
  - 1 struct
    - `struct SeqAllocator` *(re-exportiert)* · 6 Methoden
- `seq_log`
  - 1 const, 1 enum, 2 struct
    - `const DEFAULT_MAX_PIN_DURATION`
    - `struct SeqLogEntry` · 1 Methoden ⚠ fremd: `acta-types`
    - `enum SeqLogChange` · 2 Methoden ⚠ fremd: `acta-types`
    - `struct SequenceLog` · 24 Methoden
- `snapshot`
  - 2 const, 3 struct, 4 type
    - `type TenantTag`
    - `type Seq`
    - `type RevokedCount`
    - `type Matcher`
    - `const DEFAULT_TENANT_TAG`
    - `const MAX_SNAPSHOTS_PER_TENANT`
    - `struct SnapshotRegistry` · 21 Methoden
    - `struct SnapshotHandle` · 2 Methoden
    - `struct SnapshotGuard` · 3 Methoden
- `ssi`
  - 1 const, 3 struct, 1 trait
    - `const DEFAULT_MAX_TRACKED_COMMIT_KEYS`
    - `struct ReadSet` · 13 Methoden
    - `trait SsiValidator` · Methoden: validate
    - `struct PruningBlockerInfo`
    - `struct SequenceLogSsiValidator` · 23 Methoden
- `tx_buffer`
  - 6 const, 1 enum, 2 struct, 1 trait
    - `const STAGING_ENTRY_OVERHEAD_BYTES`
    - `const DEFAULT_SHARD_COUNT`
    - `const DEFAULT_MAX_OPS_PER_TX`
    - `const DEFAULT_MAX_READ_SET_KEYS`
    - `const DEFAULT_MAX_TX_STAGED_BYTES`
    - `const DEFAULT_MAX_TOTAL_STAGED_BYTES`
    - `struct TxBufferConfig`
    - `enum IndexOp` · 2 Methoden
    - `trait StagedOpSize` · Methoden: staged_bytes
    - `struct TxBuffer` · 35 Methoden
- Zusätzlich 2 `pub`-Items in privaten Modulen ohne Re-Export (nicht Teil der echten API).

**Eingehend (wer nutzt dieses Crate)**

| Nutzer | Symbole prod | Symbole dev |
|---|---|---|
| `acta-store` | `SsiValidator`×4, `tx_buffer::STAGING_ENTRY_OVERHEAD_BYTES`×2, `SnapshotRegistry`, `SequenceLogSsiValidator`, `IndexOp`, `SequenceLogSsiValidator::new_with_bounds` | `SnapshotRegistry::new` |

**Nach außen ungenutzte öffentliche Items (Heuristik, 25 von 30)**: `MvccError`, `SeqAllocator`, `DEFAULT_MAX_PIN_DURATION`, `SeqLogEntry`, `SeqLogChange`, `SequenceLog`, `TenantTag`, `Seq`, `RevokedCount`, `Matcher`, `DEFAULT_TENANT_TAG`, `MAX_SNAPSHOTS_PER_TENANT`, `SnapshotHandle`, `SnapshotGuard`, `DEFAULT_MAX_TRACKED_COMMIT_KEYS`, `ReadSet`, `PruningBlockerInfo`, `DEFAULT_SHARD_COUNT`, `DEFAULT_MAX_OPS_PER_TX`, `DEFAULT_MAX_READ_SET_KEYS`, `DEFAULT_MAX_TX_STAGED_BYTES`, `DEFAULT_MAX_TOTAL_STAGED_BYTES`, `TxBufferConfig`, `StagedOpSize`, `TxBuffer`

### acta-ports

- Pfad: `crates/acta-ports` · Zeilen: 3934 · Rust-Dateien: 22 · Ring: 0
- Interne Cargo-Abhängigkeiten: `acta-types` (normal)

**Ausgehend (Importe aus anderen Workspace-Crates)**

| Ziel | prod | dev | Dateien (prod) | Symbole (prod) | Symbole (nur dev) |
|---|---:|---:|---:|---|---|
| `acta-types` | 2 | 0 | 2 | `ids::ErasureId`×2 | – |

**Pro Komponente**

- `observer` (141 Zeilen)
  - → `acta-types`: prod: `ids::ErasureId`
- `pre_gate` (92 Zeilen)
  - → `acta-types`: prod: `ids::ErasureId`

**Öffentliche Schnittstelle** (erreichbar über `pub mod`-Kette oder `pub use`)

- `(crate-root)`
  - 2 type
    - `type BoxFuture`
    - `type BoxStream`
- `barrier`
  - 4 struct, 2 trait
    - `struct BarrierShared` · 1 Methoden
    - `struct BarrierExclusive` · 1 Methoden
    - `struct CompactionPermit` · 1 Methoden
    - `struct PurgePermit` · 1 Methoden
    - `trait CommitBarrier` · Methoden: shared, exclusive
    - `trait PurgeBarrier` · Methoden: compaction, purge, purge_waiting
- `clock`
  - 1 struct, 1 trait
    - `trait Clock` · Methoden: now_unix_nanos, monotonic_nanos
    - `struct SystemClock` · 1 Methoden
- `erasure`
  - 2 enum, 7 struct, 1 trait
    - `struct LayerId` · 2 Methoden
    - `enum EraseTarget`
    - `struct LayerScope` · 4 Methoden
    - `struct PurgeCx` · 1 Methoden
    - `struct PurgeOutcome` · 1 Methoden
    - `struct Armed`
    - `struct Scanner` · 1 Methoden
    - `struct LayerWitness` · 3 Methoden
    - `enum KeyCoverage`
    - `trait ErasureLayer` · Methoden: id, resolve, on_gate, purge, witness, key_coverage
- `gate`
  - 3 enum, 3 struct, 1 trait
    - `struct Seq` · 2 Methoden
    - `struct KeyBuf` · 2 Methoden
    - `enum KeyBound` · 1 Methoden
    - `struct KeyRange` · 2 Methoden
    - `enum GateDecision`
    - `enum RangeGate`
    - `trait ReadGate` · Methoden: check, check_bound, variant_for_range, gated_seq, gate_epoch, covers…
- `id_gen`
  - 1 struct, 1 trait
    - `trait IdGen` · Methoden: next_id
    - `struct SequentialIdGen` · 1 Methoden
- `keys`
  - 1 enum, 4 struct, 2 trait
    - `struct FileId` · 2 Methoden
    - `enum FileKind`
    - `struct FileKey` · 3 Methoden
    - `struct SealCtx` · 1 Methoden
    - `struct Envelope` · 2 Methoden
    - `trait RecordSealer` · Methoden: seal, open, evict, make_slots_durable
    - `trait FileKeyring` · Methoden: create, open, destroy
- `ledger`
  - 3 struct, 1 trait
    - `struct Timestamp` · 2 Methoden
    - `struct RangeSet` · 3 Methoden
    - `struct LedgerEntry` · 3 Methoden
    - `trait LedgerSink` · Methoden: append, read_all, prune
- `memory`
  - 1 struct, 1 trait
    - `struct MemoryLockGuard` · 1 Methoden
    - `trait ArenaGuard` · Methoden: lock_memory, protect_against_dumping
- `metrics`
  - 1 enum, 6 struct, 2 trait
    - `trait MetricsSink` · Methoden: record_counter, record_gauge, record_histogram
    - `struct NoopMetricsSink`
    - `enum MetricName` · 1 Methoden
    - `trait Metrics` · Methoden: record_counter, record_gauge, record_histogram
    - `struct MetricsSinkAdapter` · 2 Methoden
    - `struct MetricKey` · 1 Methoden
    - `struct HistogramSnapshot`
    - `struct MetricSnapshot`
    - `struct InMemoryMetricsSink` · 2 Methoden
- `observer`
  - 3 enum, 1 struct, 1 trait, 1 type
    - `type Timestamp`
    - `enum TargetKind`
    - `enum ObservedPhase`
    - `struct ErasureEvent` ⚠ fremd: `acta-types`
    - `enum ObserverError`
    - `trait ErasureObserver` · Methoden: on_erased
- `pre_gate`
  - 1 enum, 1 struct, 1 trait
    - `enum PreGateIntent` ⚠ fremd: `acta-types`
    - `struct PreGateOutcome`
    - `trait PreGate` · Methoden: run
- `report_signer`
  - 1 enum, 1 struct, 1 trait, 1 type
    - `type SignerId`
    - `struct ReportSignature`
    - `enum SignerError`
    - `trait ReportSigner` · Methoden: signer_id, sign_report
- `rng`
  - 1 struct, 1 trait
    - `trait Rng` · Methoden: next_u64, fill_bytes, next_unit_f64
    - `struct SeededRng` · 1 Methoden
- `storage`
  - 1 const, 1 struct, 1 trait
    - `struct StorageStats`
    - `const MAX_SCAN_MERGE_ACCUMULATOR`
    - `trait StorageEngine` · Methoden: get, get_at_seq, get_tracked, get_at_seq_tracked, scan_prefix_tracked, supports_ssi_tracking…
- `text_index`
  - 1 struct, 1 trait
    - `struct TextIndexStats`
    - `trait TextIndex` · Methoden: search, search_at, insert, delete, commit, rollback…
- `vector_index`
  - 1 struct, 1 trait
    - `struct VectorIndexStats`
    - `trait VectorIndex` · Methoden: insert, all_doc_ids, insert_batch, search, search_at, search_filtered…
- `vfs`
  - 1 enum, 1 struct, 2 trait
    - `enum VerifiedVia`
    - `struct VfsOpenOptions` · 2 Methoden
    - `trait VfsFile` · Methoden: read_at, read_verified, write_at, set_len, len, is_empty…
    - `trait Vfs` · Methoden: open, read_at, read_verified, read, write_atomic, create_dir_all…

**Eingehend (wer nutzt dieses Crate)**

| Nutzer | Symbole prod | Symbole dev |
|---|---|---|
| `acta-erasure` | `vfs::Vfs`×11, `ActaError::Io`×6, `gate`×5, `BoxFuture`×4, `vfs`×3, `keys`×3, `gate::Seq`×2, `gate::KeyBuf::new`×2, `clock`×2, `observer`×2 … (+15) | `Result`×51, `ActaError::NotFound`×4, `ledger::LedgerSink`, `report_signer::ReportSigner` |
| `acta-store` | `BoxFuture`×8, `Clock`×7, `keys::FileId`×7, `MetricsSink`×6, `storage::MAX_SCAN_MERGE_ACCUMULATOR`×6, `StorageStats`×5, `vfs::VfsFile`×3, `ResourceTracker`×2, `SystemClock::new`×2, `StorageEngine`×2 … (+3) | `ResourceBudget`×27, `ResourceTracker::new`×26, `storage::StorageEngine` |
| `acta-vector` | `MetricsSink`×6, `Clock`×4, `Rng`×3, `VectorIndex`×2, `Vfs`×2, `SystemClock`×2, `NoopMetricsSink`×2, `SeededRng::new`×2, `VfsOpenOptions::read_only`, `ArenaGuard` … (+1) | `SeededRng`×5, `PurgeCx::new`×2, `vector_index::VectorIndex`, `erasure` … (+1) |
| `acta-text` | `StorageEngine`×6, `erasure` | `BoxFuture`×34, `StorageStats`×31 |
| `acta` | `clock::Clock`, `clock::SystemClock`, `rng::Rng`, `vfs::Vfs` | – |
| `acta-mvcc` | `MetricsSink`×3 | `Clock`×2, `SystemClock::new`×2, `gate`×2 |
| `acta-crypto` | `BoxFuture`, `report_signer` | `report_signer::ReportSigner` |
| `acta-durable-fs` | `clock`, `vfs` | `Result`×14, `vfs::Vfs`, `Vfs`, `VfsOpenOptions` … (+6) |

**Nach außen ungenutzte öffentliche Items (Heuristik, 53 von 77)**: `BarrierShared`, `BarrierExclusive`, `CompactionPermit`, `PurgePermit`, `PurgeBarrier`, `LayerId`, `LayerScope`, `PurgeCx`, `PurgeOutcome`, `Armed`, `Scanner`, `LayerWitness`, `KeyCoverage`, `ErasureLayer`, `KeyBuf`, `KeyBound`, `KeyRange`, `RangeGate`, `ReadGate`, `SequentialIdGen`, `FileKind`, `FileKey`, `SealCtx`, `Envelope`, `RecordSealer`, `FileKeyring`, `Timestamp`, `RangeSet`, `LedgerEntry`, `BoxStream` …

### acta-rank

- Pfad: `crates/acta-rank` · Zeilen: 1125 · Rust-Dateien: 9 · Ring: 2
- Interne Cargo-Abhängigkeiten: `acta-types` (normal)

**Ausgehend (Importe aus anderen Workspace-Crates)**

Keine.

**Pro Komponente**

Keine Importe aus anderen Workspace-Crates.

**Öffentliche Schnittstelle** (erreichbar über `pub mod`-Kette oder `pub use`)

- `bound`
  - 1 fn
    - `fn unseen_bound`
- `error`
  - 1 enum
    - `enum RankError`
- `rrf`
  - 1 const, 1 fn, 4 struct
    - `const K_RRF`
    - `struct DocHandle`
    - `struct Candidate`
    - `struct RrfScore`
    - `struct Fused`
    - `fn rrf_fuse`

**Eingehend (wer nutzt dieses Crate)**

Wird von keinem anderen analysierten Workspace-Crate importiert.

**Nach außen ungenutzte öffentliche Items (Heuristik, 8 von 8)**: `unseen_bound`, `RankError`, `K_RRF`, `DocHandle`, `Candidate`, `RrfScore`, `Fused`, `rrf_fuse`

**Auffälligkeiten**

- acta-types: kein Code-Zugriff gefunden (prüfen: Makro, Feature, cfg, oder entfernen)

### acta-simd

- Pfad: `crates/acta-simd` · Zeilen: 3219 · Rust-Dateien: 14 · Ring: 1
- Interne Cargo-Abhängigkeiten: `acta-types` (normal)

**Ausgehend (Importe aus anderen Workspace-Crates)**

| Ziel | prod | dev | Dateien (prod) | Symbole (prod) | Symbole (nur dev) |
|---|---:|---:|---:|---|---|
| `acta-types` | 7 | 0 | 2 | `Result`×6, `ActaError` | – |

**Pro Komponente**

- `(crate-root)` (120 Zeilen)
  - → `acta-types`: prod: `Result`×6
- `dispatch` (259 Zeilen)
  - → `acta-types`: prod: `ActaError`

**Öffentliche Schnittstelle** (erreichbar über `pub mod`-Kette oder `pub use`)

- `(crate-root)`
  - 1 const, 6 fn
    - `const MAX_VECTOR_DIMENSION`
    - `fn validate_vector` ⚠ fremd: `acta-types`
    - `fn distance` ⚠ fremd: `acta-types`
    - `fn distance_prevalidated` ⚠ fremd: `acta-types`
    - `fn compute_distance` ⚠ fremd: `acta-types`
    - `fn compute_distance_trusted` ⚠ fremd: `acta-types`
    - `fn compute_distance_f32_bytes_trusted` ⚠ fremd: `acta-types`
- `dispatch`
  - 2 enum, 12 fn
    - `enum SimdPath`
    - `fn active_path`
    - `enum SimdPathOverride`
    - `fn set_dispatch_override`
    - `fn get_dispatch_override`
    - `fn cosine_distance` ⚠ fremd: `acta-types`
    - `fn euclidean_distance` ⚠ fremd: `acta-types`
    - `fn dot_product_distance` ⚠ fremd: `acta-types`
    - `fn cosine_distance_f32_bytes` ⚠ fremd: `acta-types`
    - `fn euclidean_distance_f32_bytes` ⚠ fremd: `acta-types`
    - `fn dot_product_distance_f32_bytes` ⚠ fremd: `acta-types`
    - `fn dot_product_u8` ⚠ fremd: `acta-types`
    - `fn euclidean_distance_sq_u8` ⚠ fremd: `acta-types`
    - `fn cosine_similarity_parts_u8` ⚠ fremd: `acta-types`
- `kernels`
  - 13 fn, 1 struct
    - `fn canonical_reduce_16`
    - `fn cosine_distance_scalar`
    - `fn euclidean_distance_scalar`
    - `fn dot_product_scalar`
    - `fn cosine_distance_f32_bytes_scalar`
    - `fn euclidean_distance_f32_bytes_scalar`
    - `fn dot_product_f32_bytes_scalar`
    - `fn normalize_inplace`
    - `struct CosineSimilarityPartsU8`
    - `fn dot_product_u8_scalar`
    - `fn euclidean_distance_sq_u8_scalar`
    - `fn cosine_similarity_parts_u8_scalar`
    - `fn dot_product_f32_u8`
    - `fn euclidean_distance_sq_f32_u8`
- Interne Re-Exporte (`pub use`): `dot_product_f32_u8`, `euclidean_distance_sq_f32_u8`

**Eingehend (wer nutzt dieses Crate)**

| Nutzer | Symbole prod | Symbole dev |
|---|---|---|
| `acta-vector` | `validate_vector`×2, `kernels::scalar` | `kernels::scalar::dot_product_scalar`, `kernels::scalar::euclidean_distance_scalar`, `kernels::scalar::cosine_distance_scalar`, `kernels::scalar::dot_product_u8_scalar` … (+2) |

**Nach außen ungenutzte öffentliche Items (Heuristik, 28 von 35)**: `SimdPath`, `active_path`, `SimdPathOverride`, `set_dispatch_override`, `get_dispatch_override`, `cosine_distance`, `euclidean_distance`, `dot_product_distance`, `cosine_distance_f32_bytes`, `euclidean_distance_f32_bytes`, `dot_product_distance_f32_bytes`, `dot_product_u8`, `euclidean_distance_sq_u8`, `cosine_similarity_parts_u8`, `canonical_reduce_16`, `cosine_distance_f32_bytes_scalar`, `euclidean_distance_f32_bytes_scalar`, `dot_product_f32_bytes_scalar`, `normalize_inplace`, `CosineSimilarityPartsU8`, `dot_product_f32_u8`, `euclidean_distance_sq_f32_u8`, `MAX_VECTOR_DIMENSION`, `distance`, `distance_prevalidated`, `compute_distance`, `compute_distance_trusted`, `compute_distance_f32_bytes_trusted`

### acta-store

- Pfad: `crates/acta-store` · Zeilen: 46059 · Rust-Dateien: 228 · Ring: 2
- Interne Cargo-Abhängigkeiten: `acta-crypto` (normal), `acta-mvcc` (normal), `acta-ports` (normal), `acta-sys` (normal), `acta-types` (normal)

**Ausgehend (Importe aus anderen Workspace-Crates)**

| Ziel | prod | dev | Dateien (prod) | Symbole (prod) | Symbole (nur dev) |
|---|---:|---:|---:|---|---|
| `acta-ports` | 51 | 148 | 22 | `BoxFuture`×8, `Clock`×7, `keys::FileId`×7, `MetricsSink`×6, `storage::MAX_SCAN_MERGE_ACCUMULATOR`×6, `StorageStats`×5, `vfs::VfsFile`×3, `ResourceTracker`×2, `SystemClock::new`×2, `StorageEngine`×2 … (+3) | `ResourceBudget`×27, `ResourceTracker::new`×26, `storage::StorageEngine` |
| `acta-types` | 32 | 103 | 17 | `Result`×5, `ActaError::Storage`×5, `TOMBSTONE_BIT`×4, `TxId`×4, `ActaError::MemoryBudgetExceeded`×2, `ActaError::Internal`×2, `TxId::INTERNAL_BASE`×2, `key`×2, `tags::TenantTag`×2, `ActaError::Sandbox` … (+3) | `tags`×2, `ActaError::InvalidInput`×2, `TenantId`, `*` |
| `acta-mvcc` | 10 | 7 | 6 | `SsiValidator`×4, `tx_buffer::STAGING_ENTRY_OVERHEAD_BYTES`×2, `SnapshotRegistry`, `SequenceLogSsiValidator`, `IndexOp`, `SequenceLogSsiValidator::new_with_bounds` | `SnapshotRegistry::new` |
| `acta-crypto` | 5 | 0 | 2 | `verify_chain_complete`×4, `verify_wal_chain_completeness` | – |
| `acta-sys` | 1 | 2 | 1 | `set_restrictive_file_acl` | `verify_file_acl_owner_only`, `set_fd_limit` |

**Pro Komponente**

- `compaction` (3315 Zeilen)
  - → `acta-mvcc`: prod: `SnapshotRegistry` · dev: `SnapshotRegistry`
  - → `acta-ports`: prod: `StorageStats`×2, `ResourceTracker`×2, `Clock`×2, `SystemClock::new` · dev: `ResourceTracker::new`×26, `ResourceBudget`×26, `StorageEngine`×2
  - → `acta-types`: prod: `ActaError::MemoryBudgetExceeded`×2, `ActaError::Internal`×2, `ActaError::Sandbox`, `Result`, `TOMBSTONE_BIT` · dev: `TxId::new`×3, `TOMBSTONE_BIT`×2, `ActaError::MemoryBudgetExceeded`×2, `TxId`
- `erase` (95 Zeilen)
  - → `acta-ports`: prod: `BoxFuture`, `erasure`
  - → `acta-types`: prod: `Result`
- `lsm` (6291 Zeilen)
  - → `acta-mvcc`: prod: `SsiValidator`×4, `tx_buffer::STAGING_ENTRY_OVERHEAD_BYTES`, `SequenceLogSsiValidator`, `IndexOp`, `SequenceLogSsiValidator::new_with_bounds` · dev: `tx_buffer::STAGING_ENTRY_OVERHEAD_BYTES`
  - → `acta-ports`: prod: `storage::MAX_SCAN_MERGE_ACCUMULATOR`×6, `Clock`×5, `MetricsSink`×5, `StorageStats`×2, `StorageEngine`×2, `NoopMetricsSink`, `SystemClock::new` · dev: `Clock`
  - → `acta-types`: prod: `ActaError::Storage`×5, `TxId`×4, `ActaError`, `ActaError::DurabilityConfig`, `Result`, `TxId::new`, `TxId::INTERNAL_BASE` · dev: `TOMBSTONE_BIT`
- `manifest` (2328 Zeilen)
  - → `acta-ports`: prod: `keys::FileId`×2 · dev: `keys::FileId`×4
- `memtable` (1009 Zeilen)
  - → `acta-mvcc`: prod: `tx_buffer::STAGING_ENTRY_OVERHEAD_BYTES`
  - → `acta-types`: dev: `TOMBSTONE_BIT`
- `purge` (241 Zeilen)
  - → `acta-ports`: prod: `BoxFuture`×6, `keys::FileId`×4
  - → `acta-types`: prod: `Result`×2, `key` · dev: `key`, `tags`
- `sstable` (3383 Zeilen)
  - → `acta-ports`: prod: `vfs::VfsFile`×3
  - → `acta-types`: prod: `TOMBSTONE_BIT`×3, `TxId::INTERNAL_BASE` · dev: `ActaError`
- `system_pressure` (330 Zeilen)
  - → `acta-ports`: prod: `MetricsSink`
- `tenant_codec` (821 Zeilen)
  - → `acta-ports`: prod: `StorageStats`
- `tenant_scoped` (83 Zeilen)
  - → `acta-ports`: prod: `keys::FileId`, `BoxFuture`
  - → `acta-types`: prod: `tags::TenantTag`×2, `key`
- `vfs` (147 Zeilen)
  - → `acta-ports`: prod: `vfs`
- `wal` (7580 Zeilen)
  - → `acta-crypto`: prod: `verify_chain_complete`×4, `verify_wal_chain_completeness`
  - → `acta-sys`: prod: `set_restrictive_file_acl` · dev: `verify_file_acl_owner_only`
  - → `acta-types`: dev: `TxId`×7
- `benches/lsm_concurrency_bench.rs` (129 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `benches/redb_comparative_bench.rs` (508 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `benches/wal_bench.rs` (143 Zeilen)
  - → `acta-types`: dev: `TxId`
- `examples/chaos_writer.rs` (55 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
- `fuzz/fuzz_targets/fuzz_compaction_interleave.rs` (105 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `fuzz/fuzz_targets/fuzz_recovery_arbitrary_state.rs` (52 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
- `fuzz/fuzz_targets/wal_mutation_chaos.rs` (270 Zeilen)
  - → `acta-types`: dev: `TxId`
- `tests/adaptive_compaction_determinism.rs` (35 Zeilen)
  - → `acta-ports`: dev: `StorageStats`
- `tests/adaptive_compaction_regression.rs` (35 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/adaptive_compaction_tombstone.rs` (40 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/amplification_benchmark.rs` (342 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/blocking_util_pool_measurement.rs` (90 Zeilen)
  - → `acta-ports`: dev: `MetricsSink`
- `tests/budget_drift.rs` (54 Zeilen)
  - → `acta-mvcc`: dev: `tx_buffer::STAGING_ENTRY_OVERHEAD_BYTES`
- `tests/campaign_fuzzregress_memtable_concurrent.rs` (105 Zeilen)
  - → `acta-types`: dev: `TxId`
- `tests/campaign_j11_memory_deletion.rs` (48 Zeilen)
  - → `acta-ports`: dev: `storage::StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/campaign_j21_hypotheses.rs` (438 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
- `tests/campaign_j41_store.rs` (36 Zeilen)
  - → `acta-types`: dev: `TenantId`
- `tests/campaign_prop_lsm_ops.rs` (397 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/campaign_stress_store.rs` (294 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/chaos_bitflip_sstable.rs` (355 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
- `tests/chaos_dropped_write.rs` (121 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
- `tests/chaos_matrix.rs` (278 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
- `tests/chaos_memory_pressure.rs` (217 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
- `tests/chaos_power_cut.rs` (159 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `ActaError`
- `tests/chaos_task_massacre.rs` (163 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/commit_cancel_chain_consistency.rs` (49 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/commit_cancellation_atomicity.rs` (147 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/commit_core_batch_rollback.rs` (43 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/commit_core_cancel_safety.rs` (54 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/commit_paths_equivalence.rs` (95 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/commit_read_lock_concurrency.rs` (61 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/commit_rollback_batch_scoped.rs` (97 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/compaction_contiguous_selection.rs` (154 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`, `StorageStats`
  - → `acta-types`: dev: `TxId`
- `tests/compaction_correctness_and_pinning.rs` (80 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
- `tests/compaction_key_range_selection.rs` (101 Zeilen)
  - → `acta-mvcc`: dev: `SnapshotRegistry::new`
  - → `acta-ports`: dev: `ResourceTracker`, `ResourceBudget`
- `tests/compaction_merge_operator.rs` (481 Zeilen)
  - → `acta-mvcc`: dev: `SnapshotRegistry`
- `tests/compaction_noncontiguous_shadowing.rs` (78 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
- `tests/compaction_overlap_scan_and_tombstone.rs` (216 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/crash_kill_durability.rs` (450 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/crash_prefix_enumeration.rs` (133 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/crash_recovery.rs` (111 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/differential/redb_operation_sequence.rs` (856 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/disk_full_during_write.rs` (76 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/docid_128_kv_engine.rs` (64 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
- `tests/encryption_test.rs` (129 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/fault_injection_recovery.rs` (120 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/filename_uniqueness_test.rs` (88 Zeilen)
  - → `acta-mvcc`: dev: `SnapshotRegistry`
  - → `acta-types`: dev: `TxId`
- `tests/flush_crash_simulation.rs` (176 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
- `tests/flush_durability.rs` (64 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
- `tests/flush_mvcc_versions.rs` (184 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/flush_recovery_lifecycle.rs` (184 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/flush_threshold_boundary.rs` (68 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
- `tests/fsync_syscall_verification.rs` (62 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/fsync_timeout_rollback.rs` (67 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/group_commit_fault_injection.rs` (96 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
- `tests/group_commit_flush_race.rs` (171 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/group_commit_follower_timeout_chain.rs` (69 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/group_commit_leader_cancel_chain.rs` (65 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/group_commit_overlapping_batch_rollback.rs` (73 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/group_commit_stress.rs` (129 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/group_commit_test.rs` (223 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `ActaError`
- `tests/group_commit_timeout.rs` (171 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
- `tests/lsm_robustness_fixes.rs` (139 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/lsm_storage_merge_operator.rs` (112 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `ActaError::InvalidInput`×2
- `tests/manifest_compaction_audit.rs` (117 Zeilen)
  - → `acta-mvcc`: dev: `SnapshotRegistry`
  - → `acta-types`: dev: `TOMBSTONE_BIT`
- `tests/manifest_crash_no_resurrection.rs` (205 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
- `tests/manifest_replace_rank_compat.rs` (68 Zeilen)
  - → `acta-types`: dev: `Result`
- `tests/memory_only_crash_semantics.rs` (50 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/metrics_sink_test.rs` (278 Zeilen)
  - → `acta-ports`: dev: `MetricsSink`, `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/model_based_mvcc_flush.rs` (185 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/observer_blocking_failopen.rs` (169 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/observer_deadline_no_runtime_block.rs` (67 Zeilen)
  - → `acta-types`: dev: `TxId`
- `tests/proptest_model_based.rs` (237 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`, `Result`
- `tests/read_path_audit.rs` (209 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
- `tests/recovery_fd_exhaustion_regression.rs` (93 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-sys`: dev: `set_fd_limit`
  - → `acta-types`: dev: `TxId`
- `tests/resource_leaks.rs` (88 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
- `tests/rollback_sstables.rs` (62 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/scan_limits_and_ranges.rs` (59 Zeilen)
  - → `acta-types`: dev: `TxId`
- `tests/spec_store_tests.rs` (146 Zeilen)
  - → `acta-ports`: dev: `keys::FileId`×3, `StorageEngine`
  - → `acta-types`: dev: `key`, `tags`
- `tests/ssi_tracked_read_seq_clamp.rs` (122 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
- `tests/ssi_tracked_reads_via_trait.rs` (120 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
- `tests/ssi_unbounded_snapshot_seq.rs` (163 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
- `tests/ssi_write_skew_integration.rs` (274 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
- `tests/sstable_format_v3_to_v4_compat.rs` (158 Zeilen)
  - → `acta-types`: dev: `TOMBSTONE_BIT`
- `tests/support/mod.rs` (79 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
- `tests/tenant_scoped_storage.rs` (131 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
- `tests/test_wp02_flush_mvcc_bug.rs` (39 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`, `Result`
- `tests/toctou_put_if_absent.rs` (209 Zeilen)
  - → `acta-types`: dev: `ActaError`, `Result`, `*`
- `tests/tombstone_scan.rs` (120 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/wal_backpressure.rs` (294 Zeilen)
  - → `acta-types`: dev: `TxId`
- `tests/wal_flusher_rollback.rs` (92 Zeilen)
  - → `acta-types`: dev: `Result`×2, `TxId`
- `tests/wal_hmac_chain_ordering.rs` (178 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/wal_hmac_rollback_race.rs` (122 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/wal_no_hmac_replay.rs` (52 Zeilen)
  - → `acta-types`: dev: `TxId`
- `tests/wal_observer.rs` (178 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/wal_observer_contract.rs` (149 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/wal_observer_integration.rs` (268 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/wal_observer_test.rs` (181 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `TxId`
- `tests/wal_recovery_runtime_stall.rs` (116 Zeilen)
  - → `acta-types`: dev: `TxId`
- `tests/wal_systematic_crash.rs` (199 Zeilen)
  - → `acta-ports`: dev: `StorageEngine`
  - → `acta-types`: dev: `ActaError::Storage`×2, `TxId`
- `tests/wal_torn_write_fault_injection.rs` (113 Zeilen)
  - → `acta-types`: dev: `TxId`
- `tests/wal_truncate_ordering.rs` (257 Zeilen)
  - → `acta-types`: dev: `ActaError`

**Öffentliche Schnittstelle** (erreichbar über `pub mod`-Kette oder `pub use`)

- `compaction`
  - 1 enum, 1 fn, 7 struct, 2 trait
    - `struct WorkloadMetrics` · 4 Methoden
    - `struct WorkloadMetricsSnapshot`
    - `enum CompactionStrategy`
    - `struct AdaptiveCompactionPlan`
    - `trait AdaptiveCompactionPlanner` · Methoden: plan_compaction
    - `struct CostBasedAdaptivePlanner` · 2 Methoden
    - `struct CompactionConfig` *(re-exportiert)*
    - `struct CompactionEngine` *(re-exportiert)* · 14 Methoden
    - `trait MergeOperator` · Methoden: merge
    - `fn retain_key_versions`
    - `struct TtlMetadata`
- `envelope`
  - 5 const, 1 struct
    - `const ENVELOPE_V2_VERSION`
    - `const ENVELOPE_V2_ALG_ID`
    - `const NONCE_LEN`
    - `const AEAD_TAG_LEN`
    - `const HEADER_LEN`
    - `struct RecordEnvelopeV2` · 3 Methoden
- `erase`
  - 1 const, 2 struct
    - `const DOC_STORE_LAYER_ID`
    - `struct DocStoreLayer` · 1 Methoden
    - `struct RestoreProbe` · 3 Methoden
- `fsync`
  - 3 fn
    - `fn sync_file`
    - `fn sync_parent_dir`
    - `fn fsync_parent_dir`
- `health`
  - 1 const, 2 struct
    - `struct PoisonState` · 4 Methoden
    - `const MIN_DISK_RESERVE_BYTES`
    - `struct DiskReserve` · 4 Methoden
- `lsm`
  - 7 const, 4 enum, 7 struct, 1 trait
    - `enum DurabilityMode`
    - `enum DurabilityConfigError`
    - `struct LsmConfig`
    - `enum StorageHealth` *(re-exportiert)*
    - `struct LsmStorage` *(re-exportiert)* · 35 Methoden ⚠ fremd: `acta-mvcc`
    - `const MAX_KEY_SIZE`
    - `const MAX_VALUE_SIZE`
    - `const MAX_BATCH_SIZE`
    - `const MAX_INTERNAL_MERGE_ENTRIES_FACTOR`
    - `const MAX_GROUP_COMMIT_BATCH_SIZE`
    - `const MIN_ENTRIES_FOR_SSTABLE_REBUILD`
    - `const DEFAULT_MAX_OBSERVER_LATENCY`
    - `enum WriteOrigin`
    - `struct WalEntryRef` · 6 Methoden
    - `struct CommitContext` ⚠ fremd: `acta-types`
    - `struct CommittedBatch`
    - `trait WalObserver` · Methoden: requires_durability, on_commit
    - `struct ObserverRegistry` · 14 Methoden
    - `struct AsyncObserverAdapter` · 1 Methoden
- `manifest`
  - 2 const
    - `const MAX_MANIFEST_ENTRY_SIZE`
    - `const DEFAULT_ROLLOVER_THRESHOLD_BYTES`
- `memtable`
  - 1 const, 2 fn, 1 struct
    - `fn is_tombstone`
    - `fn is_valid_range_bounds`
    - `const RESERVED_PREFIXES`
    - `struct MemTable` · 17 Methoden
- `purge`
  - 4 fn, 4 struct, 1 trait
    - `struct DrainStats`
    - `struct SealPoint`
    - `struct WalFloor` ⚠ fremd: `acta-ports`
    - `struct RewriteOutcome` ⚠ fremd: `acta-ports`
    - `trait PurgePrimitives` · Methoden: drain_range, seal_wal_at, drop_wal_until, rewrite_without, unlink_verified
    - `fn drain_memtable`
    - `fn rewrite_sstable_without` ⚠ fremd: `acta-ports`
    - `fn unlink_verified_sstable` ⚠ fremd: `acta-ports`, `acta-types`
    - `fn seal_active_wal` ⚠ fremd: `acta-types`
- `sstable`
  - 3 const, 8 fn, 10 struct, 1 trait, 2 type
    - `const BLOCK_CACHE_SHARDS` *(re-exportiert)*
    - `struct SieveCacheBackend` *(re-exportiert)*
    - `trait BlockCacheBackend` *(re-exportiert)* · Methoden: new, get, insert, len, is_empty, contains
    - `struct LruBlockCacheBackend` *(re-exportiert)*
    - `struct QuickCacheBlockCacheBackend` *(re-exportiert)*
    - `type BlockCacheShard` *(re-exportiert)*
    - `type BlockCacheShard` *(re-exportiert)*
    - `struct BlockCache` *(re-exportiert)* · 8 Methoden
    - `const SSTABLE_MAGIC_MFSX` *(re-exportiert)*
    - `const SSTABLE_MAGIC_LEGACY` *(re-exportiert)*
    - `fn create_block_cache` *(re-exportiert)*
    - `fn create_block_cache_with_shards` *(re-exportiert)*
    - `fn get_entry_at_index` *(re-exportiert)*
    - `fn get_entry_key_at_index` *(re-exportiert)*
    - `fn binary_search_first_index_in_block` *(re-exportiert)*
    - `fn binary_search_index_in_block` *(re-exportiert)*
    - `fn binary_search_entry_in_block` *(re-exportiert)*
    - `fn block_binary_search` *(re-exportiert)*
    - `struct BloomFilter` *(re-exportiert)* · 6 Methoden
    - `struct BlockBuilder` *(re-exportiert)* · 6 Methoden
- `system_pressure`
  - 6 const, 1 enum, 2 struct
- `tenant_codec`
  - 2 struct, 1 trait
- `tenant_scoped`
  - 2 struct
- `vfs`
  - 1 fn, 2 struct
- `wal`
  - 5 const, 4 enum, 2 fn, 6 static, 10 struct, 1 trait, 1 type
  - … 50 weitere Items nicht aufgelistet (`--max-items`)
- Zusätzlich 23 `pub`-Items in privaten Modulen ohne Re-Export (nicht Teil der echten API).

**Eingehend (wer nutzt dieses Crate)**

| Nutzer | Symbole prod | Symbole dev |
|---|---|---|
| `acta` | `tenant_codec::TenantScopedStorage`, `LsmStorage` | – |

**Nach außen ungenutzte öffentliche Items (Heuristik, 127 von 130)**: `WorkloadMetrics`, `WorkloadMetricsSnapshot`, `CompactionStrategy`, `AdaptiveCompactionPlan`, `AdaptiveCompactionPlanner`, `CostBasedAdaptivePlanner`, `CompactionConfig`, `CompactionEngine`, `MergeOperator`, `retain_key_versions`, `TtlMetadata`, `ENVELOPE_V2_VERSION`, `ENVELOPE_V2_ALG_ID`, `NONCE_LEN`, `AEAD_TAG_LEN`, `HEADER_LEN`, `RecordEnvelopeV2`, `DOC_STORE_LAYER_ID`, `DocStoreLayer`, `RestoreProbe`, `sync_file`, `sync_parent_dir`, `fsync_parent_dir`, `PoisonState`, `MIN_DISK_RESERVE_BYTES`, `DiskReserve`, `DurabilityMode`, `DurabilityConfigError`, `LsmConfig`, `StorageHealth` …

### acta-sys

- Pfad: `crates/acta-sys` · Zeilen: 1934 · Rust-Dateien: 18 · Ring: 1
- Interne Cargo-Abhängigkeiten: keine

**Ausgehend (Importe aus anderen Workspace-Crates)**

Keine.

**Pro Komponente**

Keine Importe aus anderen Workspace-Crates.

**Öffentliche Schnittstelle** (erreichbar über `pub mod`-Kette oder `pub use`)

- `durability`
  - 1 enum, 9 fn
    - `enum FlushKind`
    - `fn flush_file`
    - `fn flush_file_sync`
    - `fn flush_dir`
    - `fn flush_dir_sync`
    - `fn rename_durable`
    - `fn rename_durable_sync`
    - `fn write_all_durable`
    - `fn durable_flush_file`
    - `fn durable_flush_dir`
- `error`
  - 3 enum
    - `enum SyncFailure`
    - `enum UnsupportedKind`
    - `enum SysError`
- `fs_probe`
  - 2 enum, 2 fn
    - `enum FsKind`
    - `enum FsClass` · 1 Methoden
    - `fn fs_class`
    - `fn fs_class_spec`
- `hardening`
  - 1 enum, 6 fn, 1 struct
    - `enum HardeningPolicy`
    - `struct HardeningReport`
    - `fn apply_process_hardening`
    - `fn mark_dont_dump`
    - `fn secure_zero_volatile`
    - `fn lock_pages`
    - `fn ensure_fd_limit`
    - `fn set_fd_limit`
- `perm`
  - 4 fn
    - `fn set_mode`
    - `fn verify_owner_only`
    - `fn set_restrictive_file_acl`
    - `fn verify_file_acl_owner_only`

**Eingehend (wer nutzt dieses Crate)**

| Nutzer | Symbole prod | Symbole dev |
|---|---|---|
| `acta-durable-fs` | `flush_dir` | `FsClass`×2, `fs_class` |
| `acta-store` | `set_restrictive_file_acl` | `verify_file_acl_owner_only`, `set_fd_limit` |

**Nach außen ungenutzte öffentliche Items (Heuristik, 23 von 29)**: `FlushKind`, `flush_file`, `flush_file_sync`, `flush_dir_sync`, `rename_durable`, `rename_durable_sync`, `write_all_durable`, `durable_flush_file`, `durable_flush_dir`, `SyncFailure`, `UnsupportedKind`, `SysError`, `FsKind`, `fs_class_spec`, `HardeningPolicy`, `HardeningReport`, `apply_process_hardening`, `mark_dont_dump`, `secure_zero_volatile`, `lock_pages`, `ensure_fd_limit`, `set_mode`, `verify_owner_only`

### acta-text

- Pfad: `crates/acta-text` · Zeilen: 14290 · Rust-Dateien: 61 · Ring: 2
- Interne Cargo-Abhängigkeiten: `acta-crypto` (normal), `acta-ports` (normal), `acta-types` (normal)

**Ausgehend (Importe aus anderen Workspace-Crates)**

| Ziel | prod | dev | Dateien (prod) | Symbole (prod) | Symbole (nur dev) |
|---|---:|---:|---:|---|---|
| `acta-crypto` | 8 | 11 | 4 | `hierarchy::IdxKey`×2, `labels::TERM_TAG`, `tag`, `dek::Dek`, `labels::TBIND_AAD`, `aead::BlockSealer`, `hierarchy` | `secret::SecretKey`×3, `hierarchy::FileKey`×2 |
| `acta-ports` | 7 | 65 | 5 | `StorageEngine`×6, `erasure` | `BoxFuture`×34, `StorageStats`×31 |
| `acta-types` | 1 | 113 | 1 | `ActaError` | `TOMBSTONE_BIT`×51, `Result`×50, `TxId`×7, `TxId::new`×2, `ActaError::CapabilityUnsupported`×2 … (+1) |

**Pro Komponente**

- `(crate-root)` (273 Zeilen)
  - → `acta-ports`: prod: `StorageEngine`×3 · dev: `BoxFuture`
- `analyzer` (124 Zeilen)
  - → `acta-crypto`: prod: `hierarchy::IdxKey`, `labels::TERM_TAG`, `tag`
- `buffer` (122 Zeilen)
  - → `acta-crypto`: prod: `dek::Dek`, `labels::TBIND_AAD`
- `erasure` (47 Zeilen)
  - → `acta-ports`: prod: `erasure`
- `inverted` (2553 Zeilen)
  - → `acta-ports`: prod: `StorageEngine` · dev: `StorageStats`×3
  - → `acta-types`: dev: `TOMBSTONE_BIT`×6, `ContextChunk`
- `member` (78 Zeilen)
  - → `acta-crypto`: prod: `hierarchy::IdxKey`
- `postings` (172 Zeilen)
  - → `acta-types`: prod: `ActaError`
- `segment` (457 Zeilen)
  - → `acta-crypto`: prod: `aead::BlockSealer`, `hierarchy`
- `stream` (162 Zeilen)
  - → `acta-ports`: prod: `StorageEngine`
- `wand` (803 Zeilen)
  - → `acta-ports`: prod: `StorageEngine` · dev: `StorageStats`×2, `BoxFuture`
  - → `acta-types`: dev: `TxId`×7, `TOMBSTONE_BIT`×6, `TxId::new`×2
- `tests/alloc_profiler.rs` (581 Zeilen)
  - → `acta-ports`: dev: `StorageStats`×2
  - → `acta-types`: dev: `Result`×16
- `tests/audit_inspection.rs` (590 Zeilen)
  - → `acta-ports`: dev: `BoxFuture`×32, `StorageStats`×4
  - → `acta-types`: dev: `Result`×34, `TOMBSTONE_BIT`×10
- `tests/bm25_candidate_stream.rs` (333 Zeilen)
  - → `acta-ports`: dev: `StorageStats`×2
- `tests/concurrent_metadata.rs` (208 Zeilen)
  - → `acta-ports`: dev: `StorageStats`×2
- `tests/pipeline_identity_regression.rs` (283 Zeilen)
  - → `acta-ports`: dev: `StorageStats`×2
  - → `acta-types`: dev: `TOMBSTONE_BIT`×6
- `tests/proptest_bm25_invariants.rs` (347 Zeilen)
  - → `acta-ports`: dev: `StorageStats`×2
  - → `acta-types`: dev: `TOMBSTONE_BIT`×6
- `tests/t_text_acceptance_tests.rs` (84 Zeilen)
  - → `acta-crypto`: dev: `hierarchy::FileKey`, `secret::SecretKey`, `dek::Dek`
- `tests/t_text_d121_tests.rs` (104 Zeilen)
  - → `acta-crypto`: dev: `dek::Dek`×2, `hierarchy::IdxKey`, `secret::SecretKey`
- `tests/t_text_spec_tests.rs` (175 Zeilen)
  - → `acta-crypto`: dev: `dek::Dek`, `hierarchy::FileKey`, `secret::SecretKey`, `hierarchy::IdxKey`
- `tests/text_search_at_wrappers.rs` (343 Zeilen)
  - → `acta-ports`: dev: `StorageStats`×2
  - → `acta-types`: dev: `TOMBSTONE_BIT`×6, `ActaError::CapabilityUnsupported`×2
- `tests/tombstone_read_your_writes.rs` (279 Zeilen)
  - → `acta-ports`: dev: `StorageStats`×2
  - → `acta-types`: dev: `TOMBSTONE_BIT`×6
- `tests/tombstone_update.rs` (233 Zeilen)
  - → `acta-ports`: dev: `StorageStats`×2
- `tests/wand_differential.rs` (445 Zeilen)
  - → `acta-ports`: dev: `StorageStats`×2
  - → `acta-types`: dev: `TOMBSTONE_BIT`×5
- `tests/wand_tie_break_threshold.rs` (242 Zeilen)
  - → `acta-ports`: dev: `StorageStats`×2
- `tests/write_amplification.rs` (231 Zeilen)
  - → `acta-ports`: dev: `StorageStats`×2

**Öffentliche Schnittstelle** (erreichbar über `pub mod`-Kette oder `pub use`)

- `(crate-root)`
  - 1 struct
    - `struct Bm25Scorer` · 1 Methoden ⚠ fremd: `acta-ports`
- `analyzer`
  - 2 fn, 2 struct, 1 trait, 1 type
    - `trait Analyzer` · Methoden: id, version, lang, analyze, term_tag
    - `struct GermanAnalyzer` · 1 Methoden
    - `struct UndeterminedAnalyzer`
    - `type UndAnalyzer`
    - `fn get_analyzer`
    - `fn compute_term_tag` ⚠ fremd: `acta-crypto`
- `bm25`
  - 2 const, 3 fn, 3 struct, 1 type
    - `const BM25_K1`
    - `const BM25_B`
    - `struct BM25` · 2 Methoden
    - `fn score_term`
    - `fn score_term_with_params`
    - `type FieldId`
    - `struct FieldWeight` · 1 Methoden
    - `struct BM25F` · 2 Methoden
    - `fn score_term_bm25f`
- `buffer`
  - 2 fn, 2 struct
    - `fn seal_binding_envelope` ⚠ fremd: `acta-crypto`
    - `fn open_binding_envelope` ⚠ fremd: `acta-crypto`
    - `struct TextBufRecord`
    - `struct TextBuffer` · 5 Methoden
- `domain`
  - 4 fn, 4 static, 2 struct, 1 trait
    - `struct LegalDomainVocabulary`
    - `fn get_legal_compound_stems`
    - `fn get_legal_protected_terms`
    - `static LEGAL_COMPOUND_STEMS`
    - `static LEGAL_PROTECTED_TERMS`
    - `struct MedicalDomainVocabulary`
    - `fn get_medical_compound_stems`
    - `fn get_medical_protected_terms`
    - `static MEDICAL_COMPOUND_STEMS`
    - `static MEDICAL_PROTECTED_TERMS`
    - `trait DomainVocabulary` · Methoden: compound_stems, protected_terms, domain_id
- `erasure`
  - 1 struct
    - `struct TextErasureLayer` · 1 Methoden
- `inverted`
  - 1 enum, 6 struct
    - `struct InvertedIndex` *(re-exportiert)* · 12 Methoden
    - `struct BM25MorphIndex` *(re-exportiert)* · 2 Methoden
    - `struct PurgeStats`
    - `struct PurgeReport` · 3 Methoden
    - `struct ScanTarget` · 1 Methoden
    - `enum ParsedTextTarget` · 1 Methoden
    - `struct TextPurger` · 5 Methoden ⚠ fremd: `acta-ports`
- `member`
  - 1 fn, 1 struct
    - `fn compute_member_hash` ⚠ fremd: `acta-crypto`
    - `struct MembershipBlock` · 8 Methoden
- `morphology`
  - 2 struct
    - `struct TokenReductionMetrics` *(re-exportiert)* · 1 Methoden
    - `struct PassthroughTokenizer` *(re-exportiert)* · 1 Methoden
- `phrase`
  - 1 fn
    - `fn match_phrase_positions`
- `posting_list`
  - 2 const, 4 struct, 1 type
    - `const POSTING_LIST_V2_MAGIC`
    - `const BLOCK_SIZE`
    - `type DocIdRaw`
    - `struct PostingBlockInfo`
    - `struct Posting` · 2 Methoden
    - `struct PostingList` · 10 Methoden
    - `struct ResidentPostingIndex` · 7 Methoden
- `postings`
  - 1 const, 2 fn, 1 struct
    - `const BLOCK_SIZE`
    - `fn encode_vbyte`
    - `fn decode_vbyte`
    - `struct PostingBlock` · 3 Methoden
- `segment`
  - 6 const, 5 struct
    - `const POSTING_BLOCK_SIZE`
    - `const MAGIC`
    - `const FORMAT_VERSION`
    - `const FILE_KIND_TEXT`
    - `const ALG_IDS`
    - `const FLAG_POSITIONS`
    - `struct SegmentHeader` · 2 Methoden
    - `struct DictEntry`
    - `struct DocEntry`
    - `struct SegmentWriter` · 7 Methoden
    - `struct SegmentReader` · 3 Methoden
- `stats`
  - 1 struct
    - `struct SegmentStats` · 2 Methoden
- `stream`
  - 1 const, 1 struct
    - `const DEFAULT_STREAM_BATCH_SIZE`
    - `struct Bm25CandidateStream` · 7 Methoden ⚠ fremd: `acta-ports`
- `tokenizer`
  - 3 fn, 2 struct, 1 trait
    - `fn compile_protected_regex`
    - `fn get_protected_regex`
    - `trait Tokenizer` · Methoden: tokenize
    - `struct DefaultTokenizer` · 2 Methoden
    - `struct GermanMorphTokenizer` · 2 Methoden
    - `fn tokenize`
- `wand`
  - 1 fn, 2 struct
    - `struct BoundedTopK` · 4 Methoden
    - `struct WandSearchResult`
    - `fn block_max_wand_search` ⚠ fremd: `acta-ports`
- Zusätzlich 9 `pub`-Items in privaten Modulen ohne Re-Export (nicht Teil der echten API).

**Eingehend (wer nutzt dieses Crate)**

Wird von keinem anderen analysierten Workspace-Crate importiert.

**Nach außen ungenutzte öffentliche Items (Heuristik, 78 von 78)**: `Analyzer`, `GermanAnalyzer`, `UndeterminedAnalyzer`, `UndAnalyzer`, `get_analyzer`, `compute_term_tag`, `BM25_K1`, `BM25_B`, `BM25`, `score_term`, `score_term_with_params`, `FieldId`, `FieldWeight`, `BM25F`, `score_term_bm25f`, `seal_binding_envelope`, `open_binding_envelope`, `TextBufRecord`, `TextBuffer`, `LegalDomainVocabulary`, `get_legal_compound_stems`, `get_legal_protected_terms`, `LEGAL_COMPOUND_STEMS`, `LEGAL_PROTECTED_TERMS`, `MedicalDomainVocabulary`, `get_medical_compound_stems`, `get_medical_protected_terms`, `MEDICAL_COMPOUND_STEMS`, `MEDICAL_PROTECTED_TERMS`, `DomainVocabulary` …

### acta-types

- Pfad: `crates/acta-types` · Zeilen: 9747 · Rust-Dateien: 41 · Ring: 0
- Interne Cargo-Abhängigkeiten: keine

**Ausgehend (Importe aus anderen Workspace-Crates)**

Keine.

**Pro Komponente**

Keine Importe aus anderen Workspace-Crates.

**Öffentliche Schnittstelle** (erreichbar über `pub mod`-Kette oder `pub use`)

- `erasure`
  - 14 enum, 10 struct
    - `enum EraseTarget`
    - `enum Timing`
    - `enum EraseMode`
    - `enum RetentionCode`
    - `struct RetentionAttestation`
    - `struct EraseRequest`
    - `struct EraseReceipt`
    - `struct LayerId` · 2 Methoden
    - `enum PendingReason`
    - `enum RefusalReason`
    - `enum Phase`
    - `enum ErasureStatus`
    - `enum KeyCoverage`
    - `enum GateVariant`
    - `enum MeasureClass`
    - `enum ScanMethod` · 1 Methoden
    - `enum RiskCondition`
    - `enum ResidualRisk` · 2 Methoden
    - `struct Unarmed`
    - `struct Armed`
    - `struct ControlEvidence` · 1 Methoden
    - `struct Residue`
    - `struct Scanner` · 4 Methoden
    - `struct LayerWitness` · 8 Methoden
- `error`
  - 7 enum, 1 struct, 1 type
    - `enum HnswDeletionError`
    - `struct StepId` · 2 Methoden
    - `enum CacheDirective`
    - `type Result`
    - `enum ActaError` · 14 Methoden
    - `enum ConfigError`
    - `enum Conflict`
    - `enum Limit`
    - `enum Error`
- `error_dto`
  - 1 struct
    - `struct ActaErrorDto` · 2 Methoden
- `file`
  - 1 enum, 2 struct
    - `struct FormatVersion`
    - `struct AlgIds`
    - `enum FileKind` · 2 Methoden
- `filter`
  - 1 enum
    - `enum FilterExpr` · 1 Methoden
- `ids`
  - 1 const, 8 struct
    - `const MAX_REF_LEN`
    - `struct DocRef`
    - `struct DocHandle` · 3 Methoden
    - `struct ErasureId` · 2 Methoden
    - `struct FileId` · 2 Methoden
    - `struct StoreId` · 2 Methoden
    - `struct GateId` · 2 Methoden
    - `struct Seq` · 2 Methoden
    - `struct Timestamp` · 2 Methoden
- `key`
  - 2 const, 3 enum, 4 struct
    - `const KEY_BOUND_LEN`
    - `const KEY_LEN`
    - `enum Region` · 1 Methoden
    - `enum Family` · 1 Methoden
    - `struct KeyBound` · 2 Methoden
    - `struct KeyBuf` · 6 Methoden
    - `struct KeyRange` · 6 Methoden
    - `enum MatcherItem`
    - `struct Matcher` · 4 Methoden
- `labels`
  - 22 const
    - `const LABEL_TENANT_TAG`
    - `const LABEL_COLL_TAG`
    - `const LABEL_SUBJECT_TAG`
    - `const LABEL_DOC_TAG`
    - `const LABEL_TERM_TAG`
    - `const LABEL_NAME_TAG`
    - `const LABEL_COMMIT`
    - `const LABEL_ENC`
    - `const LABEL_IDX`
    - `const LABEL_UNIT`
    - `const LABEL_SIGN`
    - `const LABEL_WAL`
    - `const LABEL_LEDGER`
    - `const LABEL_FILE`
    - `const LABEL_FKMASK`
    - `const LABEL_DEK`
    - `const LABEL_DEK_SALT`
    - `const LABEL_RECORD_AAD`
    - `const LABEL_KEYSLOT_AAD`
    - `const LABEL_VAULT_MAC`
    - `const LABEL_HEADER_MAC`
    - `const LABEL_TEXT_BINDING_AAD`
- `limits`
  - 7 const
    - `const MAX_DOCS_PER_STORE`
    - `const MAX_DOC_BYTES_DEFAULT`
- `model_fingerprint`
  - 1 struct
- `retrieval_strategy`
  - 1 enum
- `schema`
  - 2 enum
- `secret`
  - 1 struct
- `tags`
  - 2 struct
- `tenant_scope`
  - 1 enum, 2 struct
- `tombstone`
  - 1 struct, 1 trait
- `types`
  - 6 enum, 14 struct, 2 type
  - … 39 weitere Items nicht aufgelistet (`--max-items`)
- Zusätzlich 26 `pub`-Items in privaten Modulen ohne Re-Export (nicht Teil der echten API).
- Interne Re-Exporte (`pub use`): `TenantContext`, `filter`

**Eingehend (wer nutzt dieses Crate)**

| Nutzer | Symbole prod | Symbole dev |
|---|---|---|
| `acta-vector` | `ActaError::invalid_input`×19, `MAX_SEARCH_K`×15, `Result`×8, `DocId`×4, `ActaError::HnswConnectivityDegraded`×4, `ActaError::Index`×4, `DistanceMetric`×2, `ActaError::CapabilityUnsupported` | `ActaError::InvalidInput`×6, `ActaError::EmbeddingDimensionMismatch`×4, `ActaError::Storage`×4, `DistanceMetric::Euclidean`×3 … (+5) |
| `acta-erasure` | `ids::ErasureId`×11, `erasure`×9, `erasure::LayerWitness`×2, `erasure::PendingReason`×2, `erasure::ControlEvidence`, `erasure::Residue`, `erasure::MeasureClass::Mk1`, `erasure::MeasureClass::Mk2`, `erasure::MeasureClass::Mk3`, `ids` … (+2) | `ActaError::NotFound`×6, `erasure::RetentionAttestation`×2, `erasure::Scanner::new`, `erasure::LayerId::new` … (+2) |
| `acta-store` | `Result`×5, `ActaError::Storage`×5, `TOMBSTONE_BIT`×4, `TxId`×4, `ActaError::MemoryBudgetExceeded`×2, `ActaError::Internal`×2, `TxId::INTERNAL_BASE`×2, `key`×2, `tags::TenantTag`×2, `ActaError::Sandbox` … (+3) | `tags`×2, `ActaError::InvalidInput`×2, `TenantId`, `*` |
| `acta` | `TenantId`×2, `error::Error`×2, `ActaError`×2, `secret::Secret`, `erasure`, `ids`, `refs`, `error`, `error::Error::Config`, `error::Error::Io` … (+16) | `ids::Timestamp::from_secs`×5, `refs::OperatorRef::try_new`×4, `ids::Timestamp::now`×4, `ids::Timestamp` … (+3) |
| `acta-mvcc` | `TOMBSTONE_BIT`×27, `DocId` | `TxId`×8, `DocId::from` |
| `acta-simd` | `Result`×6, `ActaError` | – |
| `acta-engine` | `Result`×3 | `TenantId::try_new`×2, `erasure` |
| `acta-ports` | `ids::ErasureId`×2 | – |
| `acta-crypto` | `ActaError` | – |
| `acta-text` | `ActaError` | `TOMBSTONE_BIT`×51, `Result`×50, `TxId`×7, `TxId::new`×2 … (+2) |
| `acta-durable-fs` | – | `ActaError::Io`×12, `Result`, `ActaError::PolicyViolation`, `ActaError::Storage` |

*Hinweis: Mindestens ein Crate importiert per Glob (`*`); ungenutzte Items sind deshalb nicht bestimmbar.*

### acta-vector

- Pfad: `crates/acta-vector` · Zeilen: 23332 · Rust-Dateien: 109 · Ring: 2
- Interne Cargo-Abhängigkeiten: `acta-crypto` (normal), `acta-ports` (normal), `acta-simd` (normal), `acta-types` (normal)

**Ausgehend (Importe aus anderen Workspace-Crates)**

| Ziel | prod | dev | Dateien (prod) | Symbole (prod) | Symbole (nur dev) |
|---|---:|---:|---:|---|---|
| `acta-types` | 57 | 47 | 14 | `ActaError::invalid_input`×19, `MAX_SEARCH_K`×15, `Result`×8, `DocId`×4, `ActaError::HnswConnectivityDegraded`×4, `ActaError::Index`×4, `DistanceMetric`×2, `ActaError::CapabilityUnsupported` | `ActaError::InvalidInput`×6, `ActaError::EmbeddingDimensionMismatch`×4, `ActaError::Storage`×4, `DistanceMetric::Euclidean`×3, `ActaError`×3 … (+4) |
| `acta-ports` | 26 | 71 | 8 | `MetricsSink`×6, `Clock`×4, `Rng`×3, `VectorIndex`×2, `Vfs`×2, `SystemClock`×2, `NoopMetricsSink`×2, `SeededRng::new`×2, `VfsOpenOptions::read_only`, `ArenaGuard` … (+1) | `SeededRng`×5, `PurgeCx::new`×2, `vector_index::VectorIndex`, `erasure`, `Scanner::new` |
| `acta-simd` | 3 | 7 | 3 | `validate_vector`×2, `kernels::scalar` | `kernels::scalar::dot_product_scalar`, `kernels::scalar::euclidean_distance_scalar`, `kernels::scalar::cosine_distance_scalar`, `kernels::scalar::dot_product_u8_scalar`, `kernels::scalar::euclidean_distance_sq_u8_scalar` … (+1) |
| `acta-crypto` | 2 | 0 | 1 | `hierarchy::IdxKey`, `secret::SecretKey` | – |

**Pro Komponente**

- `acorn` (180 Zeilen)
  - → `acta-types`: prod: `DocId`
- `candidate_stream` (147 Zeilen)
  - → `acta-ports`: prod: `VectorIndex`
- `diskann` (3936 Zeilen)
  - → `acta-crypto`: prod: `hierarchy::IdxKey`, `secret::SecretKey`
  - → `acta-ports`: prod: `Vfs`, `VfsOpenOptions::read_only`, `VectorIndex` · dev: `VectorIndex`×2
  - → `acta-simd`: prod: `validate_vector`
  - → `acta-types`: prod: `MAX_SEARCH_K`×9
- `distance` (216 Zeilen)
  - → `acta-simd`: prod: `kernels::scalar` · dev: `kernels::scalar::dot_product_scalar`, `kernels::scalar::euclidean_distance_scalar`, `kernels::scalar::cosine_distance_scalar`, `kernels::scalar::dot_product_u8_scalar` … (+2)
  - → `acta-types`: dev: `ActaError::EmbeddingDimensionMismatch`×4, `DistanceMetric`
- `hnsw` (5401 Zeilen)
  - → `acta-ports`: prod: `MetricsSink`×6, `Clock`×4, `Rng`×3, `SystemClock`×2, `NoopMetricsSink`×2, `SeededRng::new`×2, `ArenaGuard`, `SystemClock::new` · dev: `SeededRng::new`×3, `Rng`×2, `VectorIndex`, `MetricsSink`
  - → `acta-simd`: prod: `validate_vector`
  - → `acta-types`: prod: `MAX_SEARCH_K`×6, `ActaError::HnswConnectivityDegraded`×4, `ActaError::Index`×4, `DocId`×3, `Result`×2, `DistanceMetric`, `ActaError::CapabilityUnsupported` · dev: `DistanceMetric`
- `persistence` (1261 Zeilen)
  - → `acta-ports`: prod: `Vfs` · dev: `VectorIndex`×3
  - → `acta-types`: dev: `Result`×2
- `quantize` (976 Zeilen)
  - → `acta-types`: prod: `ActaError::invalid_input`×19, `Result`×6, `DistanceMetric` · dev: `ActaError::InvalidInput`×6, `Result`×2
- `benches/acorn_vs_post_filter.rs` (61 Zeilen)
  - → `acta-types`: dev: `DocId`
- `benches/audit_benchmarks.rs` (438 Zeilen)
  - → `acta-ports`: dev: `VectorIndex`
- `benches/distance_bench.rs` (42 Zeilen)
  - → `acta-simd`: dev: `kernels::scalar`
- `benches/distance_nan_impact_bench.rs` (32 Zeilen)
  - → `acta-types`: dev: `DistanceMetric`
- `benches/filtered_search_selectivity_bench.rs` (174 Zeilen)
  - → `acta-ports`: dev: `VectorIndex`
- `benches/flush_threshold_amplification.rs` (185 Zeilen)
  - → `acta-ports`: dev: `VectorIndex`
- `benches/hnsw_bench.rs` (98 Zeilen)
  - → `acta-ports`: dev: `VectorIndex`
- `benches/hnsw_delete_search_bench.rs` (94 Zeilen)
  - → `acta-ports`: dev: `VectorIndex`
- `benches/overfetch_selectivity.rs` (87 Zeilen)
  - → `acta-ports`: dev: `SeededRng`, `VectorIndex`
- `benches/partial_rebuild_recall_bench.rs` (144 Zeilen)
  - → `acta-ports`: dev: `VectorIndex`
- `benches/sq8_bench.rs` (48 Zeilen)
  - → `acta-types`: dev: `DistanceMetric`
- `benches/vector_bench.rs` (102 Zeilen)
  - → `acta-ports`: dev: `PurgeCx::new`
- `fuzz/fuzz_targets/fuzz_hnsw_persistence.rs` (119 Zeilen)
  - → `acta-ports`: dev: `VectorIndex`
- `fuzz/fuzz_targets/hnsw_insert_search.rs` (79 Zeilen)
  - → `acta-ports`: dev: `VectorIndex`
- `tests/acorn_hnsw_differential.rs` (261 Zeilen)
  - → `acta-ports`: dev: `VectorIndex`
- `tests/acorn_naive_recall.rs` (78 Zeilen)
  - → `acta-types`: dev: `DocId`
- `tests/adaptive_ef_recall.rs` (286 Zeilen)
  - → `acta-ports`: dev: `VectorIndex`
- `tests/campaign_hnsw_backoff.rs` (57 Zeilen)
  - → `acta-ports`: dev: `VectorIndex`
- `tests/campaign_j23_vector_index.rs` (486 Zeilen)
  - → `acta-ports`: dev: `VectorIndex`
  - → `acta-types`: dev: `ActaError::Index`×6
- `tests/campaign_j24_test.rs` (617 Zeilen)
  - → `acta-ports`: dev: `VectorIndex`
- `tests/deleted_nodes_lock_contention.rs` (184 Zeilen)
  - → `acta-ports`: dev: `VectorIndex`
- `tests/determinism_test.rs` (119 Zeilen)
  - → `acta-ports`: dev: `VectorIndex`
- `tests/differential_testing.rs` (86 Zeilen)
  - → `acta-ports`: dev: `VectorIndex`
- `tests/diskann_corruption_fallback_test.rs` (111 Zeilen)
  - → `acta-ports`: dev: `VectorIndex`
  - → `acta-types`: dev: `TxId::new`
- `tests/diskann_docid128_tombstones.rs` (75 Zeilen)
  - → `acta-ports`: dev: `vector_index::VectorIndex`
  - → `acta-types`: dev: `ActaError::Io`
- `tests/diskann_fault_injection_test.rs` (81 Zeilen)
  - → `acta-ports`: dev: `VectorIndex`
- `tests/diskann_recall_test.rs` (227 Zeilen)
  - → `acta-ports`: dev: `VectorIndex`
  - → `acta-types`: dev: `TxId`
- `tests/diskann_search_filtered.rs` (179 Zeilen)
  - → `acta-ports`: dev: `VectorIndex`
  - → `acta-types`: dev: `MAX_SEARCH_K`
- `tests/distance_determinism.rs` (72 Zeilen)
  - → `acta-types`: dev: `DistanceMetric`
- `tests/docid_128_hnsw.rs` (78 Zeilen)
  - → `acta-ports`: dev: `VectorIndex`
  - → `acta-types`: dev: `Result`×2
- `tests/filtered_selectivity_recall.rs` (161 Zeilen)
  - → `acta-ports`: dev: `SeededRng`, `VectorIndex`
- `tests/hnsw_edge_cases.rs` (329 Zeilen)
  - → `acta-ports`: dev: `VectorIndex`
- `tests/hnsw_layer_determinism.rs` (118 Zeilen)
  - → `acta-ports`: dev: `VectorIndex`
- `tests/hnsw_rebuild_search_consistency.rs` (141 Zeilen)
  - → `acta-ports`: dev: `VectorIndex`
  - → `acta-types`: dev: `DistanceMetric::Euclidean`
- `tests/hnsw_seed_determinism.rs` (302 Zeilen)
  - → `acta-ports`: dev: `SeededRng`, `VectorIndex`
- `tests/hnsw_snapshot_delete_test.rs` (137 Zeilen)
  - → `acta-ports`: dev: `VectorIndex`
- `tests/k02_snapshot_rebuild_determinism.rs` (160 Zeilen)
  - → `acta-ports`: dev: `VectorIndex`
  - → `acta-types`: dev: `DistanceMetric::Euclidean`
- `tests/loom_quantizer_race_test.rs` (123 Zeilen)
  - → `acta-ports`: dev: `VectorIndex`×2
  - → `acta-types`: dev: `ActaError`
- `tests/nan_validation_policy.rs` (179 Zeilen)
  - → `acta-ports`: dev: `VectorIndex`
- `tests/partial_rebuild_recall.rs` (154 Zeilen)
  - → `acta-ports`: dev: `VectorIndex`
- `tests/partial_rebuild_recall_regression.rs` (336 Zeilen)
  - → `acta-ports`: dev: `VectorIndex`
- `tests/partial_rebuild_veto_f02_guard.rs` (105 Zeilen)
  - → `acta-ports`: dev: `SeededRng`, `VectorIndex`
- `tests/poisoning.rs` (117 Zeilen)
  - → `acta-ports`: dev: `VectorIndex`
- `tests/proptest_distance_quantize.rs` (97 Zeilen)
  - → `acta-types`: dev: `DistanceMetric`
- `tests/quantize_persistence_audit.rs` (310 Zeilen)
  - → `acta-ports`: dev: `VectorIndex`
  - → `acta-types`: dev: `ActaError`
- `tests/quantizer_encapsulation_regression.rs` (43 Zeilen)
  - → `acta-ports`: dev: `VectorIndex`
- `tests/r1_vector_repair_test.rs` (88 Zeilen)
  - → `acta-ports`: dev: `VectorIndex`
- `tests/rabitq_recall_calibration.rs` (125 Zeilen)
  - → `acta-types`: dev: `ActaError`
- `tests/ram_reduction.rs` (67 Zeilen)
  - → `acta-ports`: dev: `VectorIndex`
- `tests/recalibration.rs` (64 Zeilen)
  - → `acta-ports`: dev: `VectorIndex`
  - → `acta-types`: dev: `DistanceMetric::Euclidean`
- `tests/recall.rs` (75 Zeilen)
  - → `acta-ports`: dev: `VectorIndex`
- `tests/recall_after_deletion_gates.rs` (286 Zeilen)
  - → `acta-ports`: dev: `VectorIndex`, `erasure`
  - → `acta-types`: dev: `ScoredDocument`
- `tests/recall_audit.rs` (413 Zeilen)
  - → `acta-ports`: dev: `VectorIndex`
- `tests/recall_vs_brute_force.rs` (163 Zeilen)
  - → `acta-ports`: dev: `VectorIndex`
- `tests/rollback.rs` (63 Zeilen)
  - → `acta-ports`: dev: `VectorIndex`
- `tests/simd_numerical_audit.rs` (179 Zeilen)
  - → `acta-types`: dev: `DistanceMetric`
- `tests/sq8_bias_calibration.rs` (28 Zeilen)
  - → `acta-types`: dev: `DistanceMetric`
- `tests/sq8_rerank_recall.rs` (170 Zeilen)
  - → `acta-ports`: dev: `SeededRng`, `VectorIndex`
- `tests/toctou_test.rs` (236 Zeilen)
  - → `acta-ports`: dev: `VectorIndex`×2
- `tests/vector_contract_tests.rs` (148 Zeilen)
  - → `acta-ports`: dev: `PurgeCx::new`, `Scanner::new`
  - → `acta-types`: dev: `ActaError::Storage`×4

**Öffentliche Schnittstelle** (erreichbar über `pub mod`-Kette oder `pub use`)

- `acorn`
  - 1 enum, 1 fn, 1 struct, 1 trait
    - `fn compute_gamma_edge_budget`
    - `trait FilteredIndex` · Methoden: search_knn_acorn
    - `enum AcornError`
    - `struct NaiveReferenceIndex` · 3 Methoden
- `candidate_stream`
  - 1 const, 1 struct
    - `const DEFAULT_VECTOR_STREAM_BATCH_SIZE`
    - `struct VectorCandidateStream` · 8 Methoden ⚠ fremd: `acta-ports`
- `compute_pool`
  - 2 struct
    - `struct TaskHandle` · 1 Methoden
    - `struct ComputePool` · 5 Methoden
- `diskann`
  - 2 struct, 1 type
    - `struct TestVfs` · 1 Methoden
    - `type InMemoryVfs`
    - `struct DiskAnnIndex` *(re-exportiert)* · 2 Methoden
- `hnsw`
  - 2 const, 1 enum, 8 struct, 1 trait, 1 type
    - `struct AdaptiveEfPolicy` · 2 Methoden
    - `struct AdaptiveEfStats`
    - `struct AdaptiveEfStateMachine` · 6 Methoden
    - `const ARENA_ALIGNMENT_BYTES`
    - `const ARENA_ALIGNMENT_U32`
    - `type BacklinkKey`
    - `struct BacklinkTable` · 4 Methoden
    - `struct HnswArena` · 12 Methoden ⚠ fremd: `acta-ports`
    - `struct DeletionStats`
    - `trait GhostFreeVectorIndex` · Methoden: remove_with_graph_repair
    - `struct Sq8Bias` · 2 Methoden
    - `enum LocalVectorOp`
    - `struct LocalTxBuffer` · 4 Methoden
- `partial_rebuild`
  - 1 fn, 2 struct
    - `struct PartialRebuildConfig`
    - `struct TraversalTracker` · 4 Methoden
    - `fn should_trigger_partial_rebuild`
- `persistence`
  - 3 const, 2 fn, 3 struct
    - `struct BufferIndex` *(re-exportiert)* · 5 Methoden
    - `struct NodeRecord` *(re-exportiert)* · 2 Methoden
    - `const SNAPSHOT_MAGIC`
    - `const SNAPSHOT_KIND_VECTOR`
    - `const SNAPSHOT_FORMAT_VERSION`
    - `struct VectorSnapshotHeader` · 2 Methoden
    - `fn save_snapshot_vfs` ⚠ fremd: `acta-ports`
    - `fn load_snapshot_vfs` ⚠ fremd: `acta-ports`
- `quantize`
  - 2 const, 1 struct
    - `const DEFAULT_P_LOW`
    - `const DEFAULT_P_HIGH`
    - `struct ScalarQuantizer` · 17 Methoden
- `quantize_rabitq`
  - 1 struct
    - `struct RaBitQQuantizer` · 5 Methoden
- Zusätzlich 25 `pub`-Items in privaten Modulen ohne Re-Export (nicht Teil der echten API).

**Eingehend (wer nutzt dieses Crate)**

Wird von keinem anderen analysierten Workspace-Crate importiert.

**Nach außen ungenutzte öffentliche Items (Heuristik, 39 von 39)**: `compute_gamma_edge_budget`, `FilteredIndex`, `AcornError`, `NaiveReferenceIndex`, `DEFAULT_VECTOR_STREAM_BATCH_SIZE`, `VectorCandidateStream`, `TaskHandle`, `ComputePool`, `TestVfs`, `InMemoryVfs`, `DiskAnnIndex`, `AdaptiveEfPolicy`, `AdaptiveEfStats`, `AdaptiveEfStateMachine`, `ARENA_ALIGNMENT_BYTES`, `ARENA_ALIGNMENT_U32`, `BacklinkKey`, `BacklinkTable`, `HnswArena`, `DeletionStats`, `GhostFreeVectorIndex`, `Sq8Bias`, `LocalVectorOp`, `LocalTxBuffer`, `PartialRebuildConfig`, `TraversalTracker`, `should_trigger_partial_rebuild`, `BufferIndex`, `NodeRecord`, `SNAPSHOT_MAGIC` …

