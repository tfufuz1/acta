# Acta Voll-Audit

- Commit: `e600e5bb2` (2026-11-04 22:34:38 +0100), Branch `main`, hinter origin: 0, ungespeicherte Aenderungen: True
- Start: 2026-11-04T21:44:34+00:00  Dauer: 383.1 min
- Toolchain-Pin: `1.97.0`; Preflight: **PASS** (cargo 1.97.0 (c980f4866 2026-06-30) | Pin 1.97.0)
- Host: Linux-7.2.7-1-liquorix-amd64-x86_64-with-glibc2.41 | 8 CPUs | MemTotal:       10121012 kB
- Nutzer: freddy

## Gesamt

- **FAIL**: 56
- **TIMEOUT**: 1
- **SKIPPED**: 1
- **INFO**: 20
- **PASS**: 114

Hinweis: Bei `befund_*`-Schritten bedeutet FAIL, dass der Befund **vorliegt**. `INFO` ist eine Messung oder ein Hinweis ohne Urteil.

## Rote und uebersprungene Schritte

| Schritt | Status | Sek | Detail | Log |
|---|---|---:|---|---|
| `fmt_check` | FAIL | 4.4 |  | `logs/008_fmt_check.log` |
| `check_alle_targets` | FAIL | 145.2 | errors=19 | `logs/009_check_alle_targets.log` |
| `check_no_default_features` | FAIL | 3.8 | errors=13 | `logs/010_check_no_default_features.log` |
| `doc` | FAIL | 5.2 |  | `logs/011_doc.log` |
| `clippy_lib__acta` | FAIL | 23.3 | errors=2 warnings=1 | `logs/012_clippy_lib__acta.log` |
| `clippy_alle_targets__acta` | FAIL | 29.7 | errors=2 warnings=1 | `logs/013_clippy_alle_targets__acta.log` |
| `clippy_lib__acta-engine` | FAIL | 2.6 | errors=2 warnings=0 | `logs/020_clippy_lib__acta-engine.log` |
| `clippy_alle_targets__acta-engine` | FAIL | 26.0 | errors=2 warnings=1 | `logs/021_clippy_alle_targets__acta-engine.log` |
| `clippy_alle_targets__acta-store` | FAIL | 29.9 | errors=1594 warnings=1 | `logs/035_clippy_alle_targets__acta-store.log` |
| `clippy_lib__acta-vector` | FAIL | 2.3 | errors=2 warnings=0 | `logs/046_clippy_lib__acta-vector.log` |
| `clippy_alle_targets__acta-vector` | FAIL | 15.7 | errors=103 warnings=1 | `logs/047_clippy_alle_targets__acta-vector.log` |
| `feature_matrix_hack` | FAIL | 190.9 |  | `logs/050_feature_matrix_hack.log` |
| `panic_flaeche_lib` | FAIL | 24.9 | errors=13 warnings=2 | `logs/053_panic_flaeche_lib.log` |
| `test_lib__acta` | FAIL | 86.5 | passed=0 failed=0 ignored=0 | COMPILE-FEHLER | `logs/055_test_lib__acta.log` |
| `test_integration__acta` | FAIL | 0.8 | passed=0 failed=0 ignored=0 | COMPILE-FEHLER | `logs/056_test_integration__acta.log` |
| `test_doc__acta` | FAIL | 0.6 | passed=0 failed=0 ignored=0 | COMPILE-FEHLER | `logs/057_test_doc__acta.log` |
| `nextest__acta` | FAIL | 1.3 | nextest: keine Summary-Zeile gefunden | `logs/058_nextest__acta.log` |
| `test_integration__acta-arch-tests` | FAIL | 5.0 | passed=51 failed=1 ignored=0 | FAILED: test_t_arch_6_exports_and_dev_dependencies | `logs/060_test_integration__acta-arch-tests.log` |
| `nextest__acta-arch-tests` | FAIL | 7.1 | passed=51 failed=1 | FAILED: (44/52) acta-arch-tests::arch test_t_arch_6_exports_and_dev_dependencies | `logs/062_nextest__acta-arch-tests.log` |
| `test_integration__acta-durable-fs` | FAIL | 4.5 | passed=31 failed=3 ignored=0 | FAILED: test_df_05_dir_lock_exclusive_same_process, test_df_05_dir_lock_permissions, test_t_lock_1_dir_lock_exclusive_subprocess | `logs/068_test_integration__acta-durable-fs.log` |
| `nextest__acta-durable-fs` | FAIL | 0.8 | passed=31 failed=3 | FAILED: (25/34) acta-durable-fs::df_lock_tests test_df_05_dir_lock_exclusive_same_process, (30/34) acta-durable-fs::df_lock_tests test_t_lock_1_dir_lock_exclusive_subprocess, (32/34) acta-durable-fs::df_lock_tests test_df_05_dir_lock_permissions | `logs/070_nextest__acta-durable-fs.log` |
| `test_integration__acta-engine` | FAIL | 9.1 | passed=0 failed=0 ignored=0 | COMPILE-FEHLER | `logs/072_test_integration__acta-engine.log` |
| `nextest__acta-engine` | FAIL | 1.3 | nextest: keine Summary-Zeile gefunden | `logs/074_nextest__acta-engine.log` |
| `test_integration__acta-store` | FAIL | 3416.4 | passed=485 failed=14 ignored=11 | FAILED: prop_model_based_lsm_simulation, prop_model_based_mvcc_flush_simulation, test_chaos_dropped_write_error_propagation_and_recovery, test_commit_cancel_safety_future_dropped_leaves_chain_or_state_corrupt, test_concurrent_commit_during_flush_is_durable, test_crash_kill_durability, test_follower_timeout_chain_integrity, test_group_commit_hmac_chain_no_bifurcation, test_h7_differential_btree_map_oracle, test_lsm_storage_with_merge_operator_sums_values_on_compaction, test_merge_operator_counter_across_sstables, test_observer_fail_open_timeout_and_latency_unaffected, test_v1_sst_compatibility_fixture, test_wal_recovery_linear_throughput_no_runtime_stall | `logs/100_test_integration__acta-store.log` |
| `nextest__acta-store` | FAIL | 2010.5 | passed=484 failed=15 | FAILED: (114/499) acta-store lsm::tests::recovery_tests::test_wal_uuid_sidecar_cleaned_up_on_startup, (271/499) acta-store::campaign_j21_hypotheses test_h7_differential_btree_map_oracle, (282/499) acta-store::chaos_dropped_write test_chaos_dropped_write_error_propagation_and_recovery, (291/499) acta-store::commit_core_cancel_safety test_commit_cancel_safety_future_dropped_leaves_chain_or_state_corrupt, (308/499) acta-store::compaction_merge_operator test_merge_operator_counter_across_sstables, (313/499) acta-store::crash_kill_durability test_crash_kill_durability, (345/499) acta-store::flush_mvcc_versions test_v1_sst_compatibility_fixture, (351/499) acta-store::group_commit_follower_timeout_chain test_follower_timeout_chain_integrity, (358/499) acta-store::lsm_robustness_fixes test_multi_cycle_crash_recovery_wal_cleanup, (368/499) acta-store::lsm_storage_merge_operator test_lsm_storage_with_merge_operator_sums_values_on_compaction, (377/499) acta-store::model_based_mvcc_flush prop_model_based_mvcc_flush_simulation, (392/499) acta-store::proptest_model_based prop_model_based_lsm_simulation, (462/499) acta-store::wal_hmac_chain_ordering test_group_commit_hmac_chain_no_bifurcation, (473/499) acta-store::wal_observer_integration test_observer_fail_open_timeout_and_latency_unaffected, (493/499) acta-store::wal_recovery_runtime_stall test_wal_recovery_linear_throughput_no_runtime_stall | `logs/102_nextest__acta-store.log` |
| `test_integration__acta-sys` | FAIL | 5.8 | passed=31 failed=1 ignored=0 | FAILED: sy_platform_04_t_plat_1_fs_class | `logs/104_test_integration__acta-sys.log` |
| `nextest__acta-sys` | FAIL | 1.4 | passed=31 failed=1 | FAILED: (28/32) acta-sys::sy_platform_tests sy_platform_04_t_plat_1_fs_class | `logs/106_nextest__acta-sys.log` |
| `test_integration__acta-vector` | FAIL | 34.2 | passed=0 failed=0 ignored=0 | COMPILE-FEHLER | `logs/124_test_integration__acta-vector.log` |
| `nextest__acta-vector` | FAIL | 8.2 | nextest: keine Summary-Zeile gefunden | `logs/126_nextest__acta-vector.log` |
| `nextest__ctx-verify` | FAIL | 7.0 | passed=25 failed=1 | FAILED: (26/26) ctx-verify::measurements t_raw_1_bench_field_scan_throughput | `logs/130_nextest__ctx-verify.log` |
| `test_gesamt_debug` | FAIL | 119.4 | passed=0 failed=0 ignored=0 | COMPILE-FEHLER | `logs/131_test_gesamt_debug.log` |
| `test_gesamt_release` | FAIL | 191.4 | passed=0 failed=0 ignored=0 | COMPILE-FEHLER | `logs/132_test_gesamt_release.log` |
| `testbarkeit_funktionen` | FAIL | 0.0 | 2938 Funktionen, 139 mit direktem Umweltzugriff; 5 komplexe pub-Funktionen ohne Testbezug | `logs/134_testbarkeit_funktionen.log` |
| `testbarkeit_verwaiste_dateien` | FAIL | 0.4 | 17 verwaiste Dateien (39 Tests laufen nie), 0 fehlende deklarierte Dateien | `logs/135_testbarkeit_verwaiste_dateien.log` |
| `liste_tests__acta` | FAIL | 3.2 | 0 Tests gelistet | `logs/136_liste_tests__acta.log` |
| `liste_tests__acta-engine` | FAIL | 6.4 | 0 Tests gelistet | `logs/140_liste_tests__acta-engine.log` |
| `liste_tests__acta-vector` | FAIL | 7.4 | 0 Tests gelistet | `logs/153_liste_tests__acta-vector.log` |
| `bench__acta-vector__hnsw_bench` | FAIL | 51.1 | keine Criterion-Zeilen erkannt | `logs/161_bench__acta-vector__hnsw_bench.log` |
| `bench__acta-vector__partial_rebuild_recall_bench` | FAIL | 0.2 | keine Criterion-Zeilen erkannt | `logs/165_bench__acta-vector__partial_rebuild_recall_bench.log` |
| `bench__acta-vector__filtered_search_selectivity_bench` | FAIL | 0.2 | keine Criterion-Zeilen erkannt | `logs/167_bench__acta-vector__filtered_search_selectivity_bench.log` |
| `fuzz__crypto__wal_hmac_chain_verify_fuzz` | FAIL | 33.1 |  | `logs/175_fuzz__crypto__wal_hmac_chain_verify_fuzz.log` |
| `fuzz__store__fuzz_compaction_interleave` | FAIL | 33.1 |  | `logs/176_fuzz__store__fuzz_compaction_interleave.log` |
| `fuzz__store__fuzz_manifest_load` | FAIL | 32.2 |  | `logs/177_fuzz__store__fuzz_manifest_load.log` |
| `fuzz__store__fuzz_memtable_concurrent` | FAIL | 32.9 |  | `logs/178_fuzz__store__fuzz_memtable_concurrent.log` |
| `fuzz__store__fuzz_recovery_arbitrary_state` | FAIL | 32.6 |  | `logs/179_fuzz__store__fuzz_recovery_arbitrary_state.log` |
| `fuzz__store__fuzz_sstable_binary_search` | FAIL | 32.4 |  | `logs/180_fuzz__store__fuzz_sstable_binary_search.log` |
| `fuzz__store__fuzz_wal_replay` | FAIL | 32.1 |  | `logs/181_fuzz__store__fuzz_wal_replay.log` |
| `fuzz__store__wal_mutation_chaos` | FAIL | 32.4 |  | `logs/182_fuzz__store__wal_mutation_chaos.log` |
| `fuzz__store__wal_roundtrip` | FAIL | 33.0 |  | `logs/183_fuzz__store__wal_roundtrip.log` |
| `fuzz__text__fuzz_bm25_tokenize` | FAIL | 33.1 |  | `logs/184_fuzz__text__fuzz_bm25_tokenize.log` |
| `fuzz__text__fuzz_tokenizer_smoke` | FAIL | 32.7 |  | `logs/185_fuzz__text__fuzz_tokenizer_smoke.log` |
| `fuzz__vector__fuzz_hnsw_persistence` | FAIL | 0.1 |  | `logs/186_fuzz__vector__fuzz_hnsw_persistence.log` |
| `fuzz__vector__hnsw_insert_search` | FAIL | 0.1 |  | `logs/187_fuzz__vector__hnsw_insert_search.log` |
| `mutation_tier0_1` | FAIL | 46.5 |  | `logs/188_mutation_tier0_1.log` |
| `miri_vector_scalar` | FAIL | 63.2 |  | `logs/190_miri_vector_scalar.log` |
| `asan_vector` | FAIL | 109.0 |  | `logs/191_asan_vector.log` |
| `coverage` | TIMEOUT | 10800.0 | >10800s (zaehlt als rot) | `logs/155_coverage.log` |
| `coverage_funktionen` | SKIPPED | 0.0 | coverage.json fehlt (Schritt `coverage` nicht gelaufen: --coverage oder --all) | `logs/156_coverage_funktionen.log` |

## Panic-Flaeche im Produktivcode (Clippy, lib, ohne Tests)

| Crate | unwrap | expect | panic | todo | index | Summe |
|---|---:|---:|---:|---:|---:|---:|
| `ctx-verify` | 0 | 0 | 0 | 0 | 1 | 1 |

## unsafe-Inventar gegen capabilities.toml

- `acta-erasure`: 0 Zeilen, island=False, nicht in capabilities.toml
- `acta-lab`: 0 Zeilen, island=False, nicht in capabilities.toml
- `acta-simd`: 81 Zeilen, island=True, ok
- `acta-sys`: 19 Zeilen, island=True, ok
- `acta-testmacros`: 0 Zeilen, island=False, nicht in capabilities.toml
- `ctx-verify`: 0 Zeilen, island=False, nicht in capabilities.toml

## Benchmarks: 81 Messwerte in `bench_results.csv`

## Testbarkeit je Crate (statisch)

| crate | tests_unit | tests_integration | doctests | ignoriert | tests_ohne_assertion | pub_fns | pub_mit_nachweis | pub_ohne_nachweis | nachweis_quote | urteil |
|---|---|---|---|---|---|---|---|---|---|---|
| acta | 0 | 9 | 0 | 0 | 3 | 40 | 28 | 2 | 0.7 | ok |
| acta-arch-tests | 32 | 20 | 0 | 0 | 2 | 24 | 21 | 3 | 0.88 | ok |
| acta-crypto | 0 | 46 | 1 | 0 | 2 | 57 | 40 | 5 | 0.7 | ok |
| acta-durable-fs | 0 | 34 | 0 | 0 | 0 | 24 | 16 | 6 | 0.67 | ok |
| acta-engine | 0 | 12 | 0 | 0 | 0 | 51 | 29 | 9 | 0.57 | ok |
| acta-erasure | 0 | 55 | 0 | 0 | 0 | 209 | 104 | 71 | 0.5 | ok |
| acta-lab | 0 | 28 | 0 | 0 | 0 | 59 | 45 | 11 | 0.76 | ok |
| acta-mvcc | 52 | 107 | 0 | 0 | 0 | 132 | 96 | 17 | 0.73 | ok |
| acta-ports | 28 | 8 | 0 | 0 | 2 | 51 | 21 | 3 | 0.41 | schwach |
| acta-rank | 0 | 19 | 0 | 0 | 0 | 2 | 2 | 0 | 1.0 | ok |
| acta-simd | 0 | 56 | 0 | 0 | 0 | 31 | 28 | 3 | 0.9 | ok |
| acta-store | 269 | 308 | 1 | 11 | 10 | 320 | 186 | 74 | 0.58 | ok |
| acta-sys | 0 | 32 | 0 | 0 | 0 | 22 | 19 | 3 | 0.86 | ok |
| acta-testkit | 21 | 70 | 0 | 0 | 0 | 124 | 57 | 27 | 0.46 | schwach |
| acta-testmacros | 0 | 1 | 0 | 0 | 1 | 2 | 1 | 1 | 0.5 | ok |
| acta-text | 106 | 112 | 0 | 1 | 8 | 132 | 73 | 22 | 0.55 | ok |
| acta-types | 120 | 79 | 0 | 0 | 1 | 217 | 153 | 14 | 0.71 | ok |
| acta-vector | 108 | 156 | 0 | 0 | 6 | 195 | 114 | 55 | 0.58 | ok |
| ctx-verify | 0 | 26 | 0 | 0 | 4 | 18 | 13 | 5 | 0.72 | ok |

## Statisch gezaehlte Tests gegen `cargo test -- --list`

Differenz > 0: Tests im Quelltext, die cargo nicht kennt (Feature/Plattform/cfg). Differenz < 0: Tests aus Makros oder nicht erkannten Attributen.

| crate | statisch | gelistet | differenz |
|---|---|---|---|
| acta-arch-tests | 52 | 52 | 0 |
| acta-crypto | 46 | 44 | 2 |
| acta-durable-fs | 34 | 34 | 0 |
| acta-erasure | 55 | 54 | 1 |
| acta-lab | 28 | 28 | 0 |
| acta-mvcc | 159 | 151 | 8 |
| acta-ports | 36 | 36 | 0 |
| acta-rank | 19 | 19 | 0 |
| acta-simd | 56 | 56 | 0 |
| acta-store | 577 | 510 | 67 |
| acta-sys | 32 | 32 | 0 |
| acta-testkit | 91 | 91 | 0 |
| acta-testmacros | 1 | 1 | 0 |
| acta-text | 218 | 218 | 0 |
| acta-types | 199 | 199 | 0 |
| ctx-verify | 26 | 26 | 0 |

## Reife-Behauptung vs. Beleg (capabilities.toml gegen diesen Lauf)

| Crate | behauptet | lib-Tests | Clippy lib | Urteil |
|---|---|---|---|---|
| `acta` | stable | FAIL | FAIL | **Behauptung ohne Beleg** |
| `acta-arch-tests` | stable | PASS | PASS | ok |
| `acta-crypto` | beta | PASS | PASS | ok |
| `acta-durable-fs` | stable | PASS | PASS | ok |
| `acta-engine` | beta | PASS | FAIL | **Behauptung ohne Beleg** |
| `acta-mvcc` | stable | PASS | PASS | ok |
| `acta-ports` | stable | PASS | PASS | ok |
| `acta-rank` | stable | PASS | PASS | ok |
| `acta-simd` | beta | PASS | PASS | ok |
| `acta-store` | beta | PASS | PASS | ok |
| `acta-sys` | stable | PASS | PASS | ok |
| `acta-testkit` | stable | PASS | PASS | ok |
| `acta-text` | beta | PASS | PASS | ok |
| `acta-types` | stable | PASS | PASS | ok |
| `acta-vector` | beta | PASS | FAIL | **Behauptung ohne Beleg** |

## Alle Schritte

| Gruppe | Schritt | Status | Sek | Detail |
|---|---|---|---:|---|
| B-befunde | `befund_cfg_features_undefiniert` | PASS | 0.2 | keine undefinierten Features |
| B-befunde | `befund_default_members` | INFO | 0.0 | members=19 default-members=16; nicht in default-members (von `cargo test` ohne --workspace nicht erfasst): ['acta-lab', 'acta-testmacros', 'ctx-verify'] |
| B-befunde | `befund_maturity_claims` | INFO | 0.0 | stable=9 beta=6 Eintraege ohne Crate=0 |
| B-befunde | `befund_bench_fuzz_dateien` | PASS | 0.0 | 0 fehlende Dateien/Verzeichnisse |
| 0-basis | `toolchain_preflight` | PASS | 0.1 | cargo 1.97.0 (c980f4866 2026-06-30) | Pin 1.97.0 |
| 0-basis | `toolchain_versionen` | INFO | 0.6 |  |
| 0-basis | `cargo_fetch` | PASS | 0.9 |  |
| 0-basis | `cargo_tree_duplicates` | INFO | 0.4 |  |
| 1-statisch | `fmt_check` | FAIL | 4.4 |  |
| 1-statisch | `check_alle_targets` | FAIL | 145.2 | errors=19 |
| 1-statisch | `check_no_default_features` | FAIL | 3.8 | errors=13 |
| 1-statisch | `doc` | FAIL | 5.2 |  |
| 1-statisch | `clippy_lib__acta` | FAIL | 23.3 | errors=2 warnings=1 |
| 1-statisch | `clippy_alle_targets__acta` | FAIL | 29.7 | errors=2 warnings=1 |
| 1-statisch | `clippy_lib__acta-arch-tests` | PASS | 7.0 | errors=0 warnings=0 |
| 1-statisch | `clippy_alle_targets__acta-arch-tests` | PASS | 1.1 | errors=0 warnings=0 |
| 1-statisch | `clippy_lib__acta-crypto` | PASS | 20.5 | errors=0 warnings=0 |
| 1-statisch | `clippy_alle_targets__acta-crypto` | PASS | 25.3 | errors=0 warnings=0 |
| 1-statisch | `clippy_lib__acta-durable-fs` | PASS | 7.4 | errors=0 warnings=0 |
| 1-statisch | `clippy_alle_targets__acta-durable-fs` | PASS | 16.5 | errors=0 warnings=0 |
| 1-statisch | `clippy_lib__acta-engine` | FAIL | 2.6 | errors=2 warnings=0 |
| 1-statisch | `clippy_alle_targets__acta-engine` | FAIL | 26.0 | errors=2 warnings=1 |
| 1-statisch | `clippy_lib__acta-erasure` | PASS | 7.7 | errors=0 warnings=0 |
| 1-statisch | `clippy_alle_targets__acta-erasure` | PASS | 16.4 | errors=0 warnings=0 |
| 1-statisch | `clippy_lib__acta-lab` | PASS | 15.2 | errors=0 warnings=0 |
| 1-statisch | `clippy_alle_targets__acta-lab` | PASS | 7.4 | errors=0 warnings=0 |
| 1-statisch | `clippy_lib__acta-mvcc` | PASS | 1.9 | errors=0 warnings=0 |
| 1-statisch | `clippy_alle_targets__acta-mvcc` | PASS | 10.9 | errors=0 warnings=0 |
| 1-statisch | `clippy_lib__acta-ports` | PASS | 6.3 | errors=0 warnings=0 |
| 1-statisch | `clippy_alle_targets__acta-ports` | PASS | 8.1 | errors=0 warnings=0 |
| 1-statisch | `clippy_lib__acta-rank` | PASS | 7.9 | errors=0 warnings=0 |
| 1-statisch | `clippy_alle_targets__acta-rank` | PASS | 5.4 | errors=0 warnings=0 |
| 1-statisch | `clippy_lib__acta-simd` | PASS | 4.9 | errors=0 warnings=0 |
| 1-statisch | `clippy_alle_targets__acta-simd` | PASS | 7.4 | errors=0 warnings=0 |
| 1-statisch | `clippy_lib__acta-store` | PASS | 19.7 | errors=0 warnings=0 |
| 1-statisch | `clippy_alle_targets__acta-store` | FAIL | 29.9 | errors=1594 warnings=1 |
| 1-statisch | `clippy_lib__acta-sys` | PASS | 6.8 | errors=0 warnings=0 |
| 1-statisch | `clippy_alle_targets__acta-sys` | PASS | 23.5 | errors=0 warnings=0 |
| 1-statisch | `clippy_lib__acta-testkit` | PASS | 5.9 | errors=0 warnings=0 |
| 1-statisch | `clippy_alle_targets__acta-testkit` | PASS | 11.1 | errors=0 warnings=0 |
| 1-statisch | `clippy_lib__acta-testmacros` | PASS | 2.8 | errors=0 warnings=0 |
| 1-statisch | `clippy_alle_targets__acta-testmacros` | PASS | 7.0 | errors=0 warnings=0 |
| 1-statisch | `clippy_lib__acta-text` | PASS | 8.7 | errors=0 warnings=0 |
| 1-statisch | `clippy_alle_targets__acta-text` | PASS | 13.5 | errors=0 warnings=0 |
| 1-statisch | `clippy_lib__acta-types` | PASS | 1.9 | errors=0 warnings=0 |
| 1-statisch | `clippy_alle_targets__acta-types` | PASS | 7.1 | errors=0 warnings=0 |
| 1-statisch | `clippy_lib__acta-vector` | FAIL | 2.3 | errors=2 warnings=0 |
| 1-statisch | `clippy_alle_targets__acta-vector` | FAIL | 15.7 | errors=103 warnings=1 |
| 1-statisch | `clippy_lib__ctx-verify` | PASS | 2.3 | errors=0 warnings=0 |
| 1-statisch | `clippy_alle_targets__ctx-verify` | PASS | 2.8 | errors=0 warnings=0 |
| 1-statisch | `feature_matrix_hack` | FAIL | 190.9 |  |
| 1-statisch | `deny` | PASS | 2.0 |  |
| 1-statisch | `audit` | PASS | 1.8 |  |
| 1-statisch | `panic_flaeche_lib` | FAIL | 24.9 | errors=13 warnings=2 |
| 1-statisch | `unsafe_inventar` | PASS | 0.2 | keine Verstoesse |
| 2-tests | `test_lib__acta` | FAIL | 86.5 | passed=0 failed=0 ignored=0 | COMPILE-FEHLER |
| 2-tests | `test_integration__acta` | FAIL | 0.8 | passed=0 failed=0 ignored=0 | COMPILE-FEHLER |
| 2-tests | `test_doc__acta` | FAIL | 0.6 | passed=0 failed=0 ignored=0 | COMPILE-FEHLER |
| 2-tests | `nextest__acta` | FAIL | 1.3 | nextest: keine Summary-Zeile gefunden |
| 2-tests | `test_lib__acta-arch-tests` | PASS | 12.4 | passed=32 failed=0 ignored=0 |
| 2-tests | `test_integration__acta-arch-tests` | FAIL | 5.0 | passed=51 failed=1 ignored=0 | FAILED: test_t_arch_6_exports_and_dev_dependencies |
| 2-tests | `test_doc__acta-arch-tests` | PASS | 0.3 | passed=0 failed=0 ignored=0 | KEINE TESTS AUSGEFUEHRT |
| 2-tests | `nextest__acta-arch-tests` | FAIL | 7.1 | passed=51 failed=1 | FAILED: (44/52) acta-arch-tests::arch test_t_arch_6_exports_and_dev_dependencies |
| 2-tests | `test_lib__acta-crypto` | PASS | 49.1 | passed=0 failed=0 ignored=0 | KEINE TESTS AUSGEFUEHRT |
| 2-tests | `test_integration__acta-crypto` | PASS | 47.2 | passed=44 failed=0 ignored=0 |
| 2-tests | `test_doc__acta-crypto` | PASS | 0.6 | passed=1 failed=0 ignored=0 |
| 2-tests | `nextest__acta-crypto` | PASS | 32.3 | passed=44 failed=0 |
| 2-tests | `test_lib__acta-durable-fs` | PASS | 30.5 | passed=0 failed=0 ignored=0 | KEINE TESTS AUSGEFUEHRT |
| 2-tests | `test_integration__acta-durable-fs` | FAIL | 4.5 | passed=31 failed=3 ignored=0 | FAILED: test_df_05_dir_lock_exclusive_same_process, test_df_05_dir_lock_permissions, test_t_lock_1_dir_lock_exclusive_subprocess |
| 2-tests | `test_doc__acta-durable-fs` | PASS | 0.4 | passed=0 failed=0 ignored=0 | KEINE TESTS AUSGEFUEHRT |
| 2-tests | `nextest__acta-durable-fs` | FAIL | 0.8 | passed=31 failed=3 | FAILED: (25/34) acta-durable-fs::df_lock_tests test_df_05_dir_lock_exclusive_same_process, (30/34) acta-durable-fs::df_lock_tests test_t_lock_1_dir_lock_exclusive_subprocess, (32/34) acta-durable-fs::df_lock_tests test_df_05_dir_lock_permissions |
| 2-tests | `test_lib__acta-engine` | PASS | 80.0 | passed=0 failed=0 ignored=0 | KEINE TESTS AUSGEFUEHRT |
| 2-tests | `test_integration__acta-engine` | FAIL | 9.1 | passed=0 failed=0 ignored=0 | COMPILE-FEHLER |
| 2-tests | `test_doc__acta-engine` | PASS | 1.3 | passed=0 failed=0 ignored=0 | KEINE TESTS AUSGEFUEHRT |
| 2-tests | `nextest__acta-engine` | FAIL | 1.3 | nextest: keine Summary-Zeile gefunden |
| 2-tests | `test_lib__acta-erasure` | PASS | 24.5 | passed=0 failed=0 ignored=0 | KEINE TESTS AUSGEFUEHRT |
| 2-tests | `test_integration__acta-erasure` | PASS | 9.8 | passed=50 failed=0 ignored=0 |
| 2-tests | `test_doc__acta-erasure` | PASS | 1.0 | passed=0 failed=0 ignored=0 | KEINE TESTS AUSGEFUEHRT |
| 2-tests | `nextest__acta-erasure` | PASS | 1.7 | passed=50 failed=0 |
| 2-tests | `test_lib__acta-lab` | PASS | 11.6 | passed=0 failed=0 ignored=0 | KEINE TESTS AUSGEFUEHRT |
| 2-tests | `test_integration__acta-lab` | PASS | 2.5 | passed=28 failed=0 ignored=0 |
| 2-tests | `test_doc__acta-lab` | PASS | 0.5 | passed=0 failed=0 ignored=0 | KEINE TESTS AUSGEFUEHRT |
| 2-tests | `nextest__acta-lab` | PASS | 1.0 | passed=28 failed=0 |
| 2-tests | `test_lib__acta-mvcc` | PASS | 21.6 | passed=52 failed=0 ignored=0 |
| 2-tests | `test_integration__acta-mvcc` | PASS | 44.7 | passed=151 failed=0 ignored=0 |
| 2-tests | `test_doc__acta-mvcc` | PASS | 0.6 | passed=0 failed=0 ignored=0 | KEINE TESTS AUSGEFUEHRT |
| 2-tests | `nextest__acta-mvcc` | PASS | 15.2 | passed=151 failed=0 |
| 2-tests | `test_lib__acta-ports` | PASS | 10.8 | passed=28 failed=0 ignored=0 |
| 2-tests | `test_integration__acta-ports` | PASS | 5.6 | passed=36 failed=0 ignored=0 |
| 2-tests | `test_doc__acta-ports` | PASS | 0.4 | passed=0 failed=0 ignored=0 | KEINE TESTS AUSGEFUEHRT |
| 2-tests | `nextest__acta-ports` | PASS | 0.7 | passed=36 failed=0 |
| 2-tests | `test_lib__acta-rank` | PASS | 6.0 | passed=0 failed=0 ignored=0 | KEINE TESTS AUSGEFUEHRT |
| 2-tests | `test_integration__acta-rank` | PASS | 3.8 | passed=19 failed=0 ignored=0 |
| 2-tests | `test_doc__acta-rank` | PASS | 0.3 | passed=0 failed=0 ignored=0 | KEINE TESTS AUSGEFUEHRT |
| 2-tests | `nextest__acta-rank` | PASS | 1.3 | passed=19 failed=0 |
| 2-tests | `test_lib__acta-simd` | PASS | 13.4 | passed=0 failed=0 ignored=0 | KEINE TESTS AUSGEFUEHRT |
| 2-tests | `test_integration__acta-simd` | PASS | 27.2 | passed=56 failed=0 ignored=0 |
| 2-tests | `test_doc__acta-simd` | PASS | 0.4 | passed=0 failed=0 ignored=0 | KEINE TESTS AUSGEFUEHRT |
| 2-tests | `nextest__acta-simd` | PASS | 17.3 | passed=56 failed=0 |
| 2-tests | `test_lib__acta-store` | PASS | 667.0 | passed=257 failed=0 ignored=0 |
| 2-tests | `test_integration__acta-store` | FAIL | 3416.4 | passed=485 failed=14 ignored=11 | FAILED: prop_model_based_lsm_simulation, prop_model_based_mvcc_flush_simulation, test_chaos_dropped_write_error_propagation_and_recovery, test_commit_cancel_safety_future_dropped_leaves_chain_or_state_corrupt, test_concurrent_commit_during_flush_is_durable, test_crash_kill_durability, test_follower_timeout_chain_integrity, test_group_commit_hmac_chain_no_bifurcation, test_h7_differential_btree_map_oracle, test_lsm_storage_with_merge_operator_sums_values_on_compaction, test_merge_operator_counter_across_sstables, test_observer_fail_open_timeout_and_latency_unaffected, test_v1_sst_compatibility_fixture, test_wal_recovery_linear_throughput_no_runtime_stall |
| 2-tests | `test_doc__acta-store` | PASS | 1.8 | passed=0 failed=0 ignored=0 | KEINE TESTS AUSGEFUEHRT |
| 2-tests | `nextest__acta-store` | FAIL | 2010.5 | passed=484 failed=15 | FAILED: (114/499) acta-store lsm::tests::recovery_tests::test_wal_uuid_sidecar_cleaned_up_on_startup, (271/499) acta-store::campaign_j21_hypotheses test_h7_differential_btree_map_oracle, (282/499) acta-store::chaos_dropped_write test_chaos_dropped_write_error_propagation_and_recovery, (291/499) acta-store::commit_core_cancel_safety test_commit_cancel_safety_future_dropped_leaves_chain_or_state_corrupt, (308/499) acta-store::compaction_merge_operator test_merge_operator_counter_across_sstables, (313/499) acta-store::crash_kill_durability test_crash_kill_durability, (345/499) acta-store::flush_mvcc_versions test_v1_sst_compatibility_fixture, (351/499) acta-store::group_commit_follower_timeout_chain test_follower_timeout_chain_integrity, (358/499) acta-store::lsm_robustness_fixes test_multi_cycle_crash_recovery_wal_cleanup, (368/499) acta-store::lsm_storage_merge_operator test_lsm_storage_with_merge_operator_sums_values_on_compaction, (377/499) acta-store::model_based_mvcc_flush prop_model_based_mvcc_flush_simulation, (392/499) acta-store::proptest_model_based prop_model_based_lsm_simulation, (462/499) acta-store::wal_hmac_chain_ordering test_group_commit_hmac_chain_no_bifurcation, (473/499) acta-store::wal_observer_integration test_observer_fail_open_timeout_and_latency_unaffected, (493/499) acta-store::wal_recovery_runtime_stall test_wal_recovery_linear_throughput_no_runtime_stall |
| 2-tests | `test_lib__acta-sys` | PASS | 26.6 | passed=0 failed=0 ignored=0 | KEINE TESTS AUSGEFUEHRT |
| 2-tests | `test_integration__acta-sys` | FAIL | 5.8 | passed=31 failed=1 ignored=0 | FAILED: sy_platform_04_t_plat_1_fs_class |
| 2-tests | `test_doc__acta-sys` | PASS | 0.4 | passed=0 failed=0 ignored=0 | KEINE TESTS AUSGEFUEHRT |
| 2-tests | `nextest__acta-sys` | FAIL | 1.4 | passed=31 failed=1 | FAILED: (28/32) acta-sys::sy_platform_tests sy_platform_04_t_plat_1_fs_class |
| 2-tests | `test_lib__acta-testkit` | PASS | 24.9 | passed=21 failed=0 ignored=0 |
| 2-tests | `test_integration__acta-testkit` | PASS | 12.2 | passed=91 failed=0 ignored=0 |
| 2-tests | `test_doc__acta-testkit` | PASS | 0.5 | passed=0 failed=0 ignored=0 | KEINE TESTS AUSGEFUEHRT |
| 2-tests | `nextest__acta-testkit` | PASS | 1.0 | passed=91 failed=0 |
| 2-tests | `test_lib__acta-testmacros` | PASS | 18.5 | passed=0 failed=0 ignored=0 | KEINE TESTS AUSGEFUEHRT |
| 2-tests | `test_integration__acta-testmacros` | PASS | 52.0 | passed=1 failed=0 ignored=0 |
| 2-tests | `test_doc__acta-testmacros` | PASS | 0.6 | passed=0 failed=0 ignored=0 | KEINE TESTS AUSGEFUEHRT |
| 2-tests | `nextest__acta-testmacros` | PASS | 1.7 | passed=1 failed=0 |
| 2-tests | `test_lib__acta-text` | PASS | 176.2 | passed=106 failed=0 ignored=0 |
| 2-tests | `test_integration__acta-text` | PASS | 245.8 | passed=217 failed=0 ignored=1 |
| 2-tests | `test_doc__acta-text` | PASS | 0.8 | passed=0 failed=0 ignored=0 | KEINE TESTS AUSGEFUEHRT |
| 2-tests | `nextest__acta-text` | PASS | 163.2 | passed=217 failed=0 |
| 2-tests | `test_lib__acta-types` | PASS | 7.0 | passed=120 failed=0 ignored=0 |
| 2-tests | `test_integration__acta-types` | PASS | 8.4 | passed=199 failed=0 ignored=0 |
| 2-tests | `test_doc__acta-types` | PASS | 0.9 | passed=0 failed=0 ignored=0 | KEINE TESTS AUSGEFUEHRT |
| 2-tests | `nextest__acta-types` | PASS | 1.5 | passed=199 failed=0 |
| 2-tests | `test_lib__acta-vector` | PASS | 22.8 | passed=66 failed=0 ignored=0 |
| 2-tests | `test_integration__acta-vector` | FAIL | 34.2 | passed=0 failed=0 ignored=0 | COMPILE-FEHLER |
| 2-tests | `test_doc__acta-vector` | PASS | 0.6 | passed=0 failed=0 ignored=0 | KEINE TESTS AUSGEFUEHRT |
| 2-tests | `nextest__acta-vector` | FAIL | 8.2 | nextest: keine Summary-Zeile gefunden |
| 2-tests | `test_lib__ctx-verify` | PASS | 10.6 | passed=0 failed=0 ignored=0 | KEINE TESTS AUSGEFUEHRT |
| 2-tests | `test_integration__ctx-verify` | PASS | 14.0 | passed=26 failed=0 ignored=0 |
| 2-tests | `test_doc__ctx-verify` | PASS | 0.3 | passed=0 failed=0 ignored=0 | KEINE TESTS AUSGEFUEHRT |
| 2-tests | `nextest__ctx-verify` | FAIL | 7.0 | passed=25 failed=1 | FAILED: (26/26) ctx-verify::measurements t_raw_1_bench_field_scan_throughput |
| 2-tests | `test_gesamt_debug` | FAIL | 119.4 | passed=0 failed=0 ignored=0 | COMPILE-FEHLER |
| 2-tests | `test_gesamt_release` | FAIL | 191.4 | passed=0 failed=0 ignored=0 | COMPILE-FEHLER |
| 3-testbarkeit | `testbarkeit_crates` | PASS | 3.7 | 19 Crates, 1914 Tests (statisch); ohne Tests: keine; schwach: ['acta-ports', 'acta-testkit'] |
| 3-testbarkeit | `testbarkeit_funktionen` | FAIL | 0.0 | 2938 Funktionen, 139 mit direktem Umweltzugriff; 5 komplexe pub-Funktionen ohne Testbezug |
| 3-testbarkeit | `testbarkeit_verwaiste_dateien` | FAIL | 0.4 | 17 verwaiste Dateien (39 Tests laufen nie), 0 fehlende deklarierte Dateien |
| 3-testbarkeit | `liste_tests__acta` | FAIL | 3.2 | 0 Tests gelistet |
| 3-testbarkeit | `liste_tests__acta-arch-tests` | INFO | 0.3 | 0 Tests gelistet |
| 3-testbarkeit | `liste_tests__acta-crypto` | INFO | 0.5 | 0 Tests gelistet |
| 3-testbarkeit | `liste_tests__acta-durable-fs` | INFO | 0.4 | 0 Tests gelistet |
| 3-testbarkeit | `liste_tests__acta-engine` | FAIL | 6.4 | 0 Tests gelistet |
| 3-testbarkeit | `liste_tests__acta-erasure` | INFO | 12.4 | 0 Tests gelistet |
| 3-testbarkeit | `liste_tests__acta-lab` | INFO | 0.4 | 0 Tests gelistet |
| 3-testbarkeit | `liste_tests__acta-mvcc` | INFO | 0.6 | 0 Tests gelistet |
| 3-testbarkeit | `liste_tests__acta-ports` | INFO | 0.4 | 0 Tests gelistet |
| 3-testbarkeit | `liste_tests__acta-rank` | INFO | 0.3 | 0 Tests gelistet |
| 3-testbarkeit | `liste_tests__acta-simd` | INFO | 0.4 | 0 Tests gelistet |
| 3-testbarkeit | `liste_tests__acta-store` | INFO | 1.6 | 0 Tests gelistet |
| 3-testbarkeit | `liste_tests__acta-sys` | INFO | 0.3 | 0 Tests gelistet |
| 3-testbarkeit | `liste_tests__acta-testkit` | INFO | 0.3 | 0 Tests gelistet |
| 3-testbarkeit | `liste_tests__acta-testmacros` | INFO | 0.2 | 0 Tests gelistet |
| 3-testbarkeit | `liste_tests__acta-text` | INFO | 0.5 | 0 Tests gelistet |
| 3-testbarkeit | `liste_tests__acta-types` | INFO | 0.3 | 0 Tests gelistet |
| 3-testbarkeit | `liste_tests__acta-vector` | FAIL | 7.4 | 0 Tests gelistet |
| 3-testbarkeit | `liste_tests__ctx-verify` | INFO | 0.3 | 0 Tests gelistet |
| 3-testbarkeit | `coverage` | TIMEOUT | 10800.0 | >10800s (zaehlt als rot) |
| 3-testbarkeit | `coverage_funktionen` | SKIPPED | 0.0 | coverage.json fehlt (Schritt `coverage` nicht gelaufen: --coverage oder --all) |
| 4-bench | `bench__acta-mvcc__mvcc_bench` | PASS | 150.5 | 7 Messpunkte |
| 4-bench | `bench__acta-crypto__crypto_benchmarks` | PASS | 158.9 | 7 Messpunkte |
| 4-bench | `bench__acta-vector__distance_bench` | PASS | 123.1 | 6 Messpunkte |
| 4-bench | `bench__acta-vector__sq8_bench` | PASS | 71.9 | 4 Messpunkte |
| 4-bench | `bench__acta-vector__hnsw_bench` | FAIL | 51.1 | keine Criterion-Zeilen erkannt |
| 4-bench | `bench__acta-vector__hnsw_delete_search_bench` | PASS | 57.9 | 2 Messpunkte |
| 4-bench | `bench__acta-vector__audit_benchmarks` | PASS | 140.6 | keine Criterion-Zeilen erkannt |
| 4-bench | `bench__acta-vector__flush_threshold_amplification` | PASS | 6.4 | keine Criterion-Zeilen erkannt |
| 4-bench | `bench__acta-vector__partial_rebuild_recall_bench` | FAIL | 0.2 | keine Criterion-Zeilen erkannt |
| 4-bench | `bench__acta-vector__distance_nan_impact_bench` | PASS | 50.9 | 2 Messpunkte |
| 4-bench | `bench__acta-vector__filtered_search_selectivity_bench` | FAIL | 0.2 | keine Criterion-Zeilen erkannt |
| 4-bench | `bench__acta-store__sstable_bench` | PASS | 101.6 | 2 Messpunkte |
| 4-bench | `bench__acta-store__memtable_bench` | PASS | 48.6 | 2 Messpunkte |
| 4-bench | `bench__acta-store__wal_bench` | PASS | 227.7 | 15 Messpunkte |
| 4-bench | `bench__acta-store__lsm_concurrency_bench` | PASS | 813.2 | 5 Messpunkte |
| 4-bench | `bench__acta-store__block_cache_bench` | PASS | 75.2 | 3 Messpunkte |
| 4-bench | `bench__acta-store__redb_comparative_bench` | PASS | 292.3 | 8 Messpunkte |
| 4-bench | `bench__acta-simd__simd_bench` | PASS | 258.7 | 18 Messpunkte |
| 5-tief | `fuzz__crypto__wal_hmac_chain_verify_fuzz` | FAIL | 33.1 |  |
| 5-tief | `fuzz__store__fuzz_compaction_interleave` | FAIL | 33.1 |  |
| 5-tief | `fuzz__store__fuzz_manifest_load` | FAIL | 32.2 |  |
| 5-tief | `fuzz__store__fuzz_memtable_concurrent` | FAIL | 32.9 |  |
| 5-tief | `fuzz__store__fuzz_recovery_arbitrary_state` | FAIL | 32.6 |  |
| 5-tief | `fuzz__store__fuzz_sstable_binary_search` | FAIL | 32.4 |  |
| 5-tief | `fuzz__store__fuzz_wal_replay` | FAIL | 32.1 |  |
| 5-tief | `fuzz__store__wal_mutation_chaos` | FAIL | 32.4 |  |
| 5-tief | `fuzz__store__wal_roundtrip` | FAIL | 33.0 |  |
| 5-tief | `fuzz__text__fuzz_bm25_tokenize` | FAIL | 33.1 |  |
| 5-tief | `fuzz__text__fuzz_tokenizer_smoke` | FAIL | 32.7 |  |
| 5-tief | `fuzz__vector__fuzz_hnsw_persistence` | FAIL | 0.1 |  |
| 5-tief | `fuzz__vector__hnsw_insert_search` | FAIL | 0.1 |  |
| 5-tief | `mutation_tier0_1` | FAIL | 46.5 |  |
| 5-tief | `loom_mvcc` | PASS | 288.0 |  |
| 5-tief | `miri_vector_scalar` | FAIL | 63.2 |  |
| 5-tief | `asan_vector` | FAIL | 109.0 |  |
