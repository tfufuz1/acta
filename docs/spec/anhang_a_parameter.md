## Anhang A – Parameterverzeichnis

**Pflicht** = kein Standardwert; der Start verweigert den Betrieb ohne Wert (15.4). Alle Startwerte sind [Q] und werden nach Messbericht (16.5) bestätigt oder mit ADR geändert. Zeitangaben in UTC; Größen in Zweierpotenzen.

### A.1 Schlüssel, Tresor, Sicherung

|Parameter|Startwert|Bereich|Abschnitt|
|---|---|---|---|
|`T_ROT`|12 h|1 bis 24 h|4.3|
|`N_SHRED`|10 000|100 bis 1 000 000|4.3|
|`rotation_min_gap`|15 min|1 min bis `T_ROT`|4.3|
|`rotation_parallel`|4|1 bis 16|4.3|
|`backup_epoch`|24 h|≤ `T_BACKUP`|4.3|
|`T_BACKUP`|**Pflicht**, Start 14 Tage|7 bis 35 Tage|4.3, 8.7|
|`vault_export_interval`|1 h|bis 4 h|4.3|
|`data_backup_interval`|24 h|bis 7 Tage|14.4|
|`data_backup_retention`|= `T_BACKUP`|≤ `T_BACKUP`|14.4|
|`anchor_destroy_max`|**Pflicht**|je Anker|4.4|
|`crypto_review_interval`|12 Monate|3 bis 12 Monate|4.7|
|`clock_skew_max`|5 min|1 bis 30 min|4.6|
|`max_subjects_per_doc`|64|2 bis 256|4.2|
|`dek_cache_max`|4 096 Einträge|256 bis 65 536|3.6|
|Argon2id (`FileAnchor`)|m ≥ 65 536 KiB, t ≥ 3, p ≥ 4|nur heraufsetzbar|4.1|
|Passphrase-Entropie|≥ 80 Bit|nur heraufsetzbar|4.4|

### A.2 Ledger, Gate, Senke, Commitments

|Parameter|Startwert|Bereich|Abschnitt|
|---|---|---|---|
|`checkpoint_every`|1 024|64 bis 65 536|6.2|
|`commit_epoch`|30 Tage|7 bis 90 Tage|6.2|
|`commit_key_retention`|90 Tage|≥ `T_BACKUP + backup_epoch`, ≤ 3 Jahre|6.2, 15.4|
|`erase_registry_sink`|**Pflicht**|`DirSink` oder Netzsenke|6.2|
|`access_log_retention`|**Pflicht**|–|6.2, 13.5|
|`ops_log_retention`|90 Tage|7 bis 365 Tage|13.5|
|Gate-Log-Verdichtung|wöchentlich|täglich bis monatlich|5.3|
|`gate_propagation_max`|1 Rahmen|fest|9.5|

### A.3 Speicher

|Parameter|Startwert|Bereich|Abschnitt|
|---|---|---|---|
|`wal_max_bytes`|64 MiB|4 MiB bis 1 GiB|5.1|
|`wal_max_age`|30 s|1 s bis 10 min|5.1|
|`segment_max_bytes`|256 MiB|16 MiB bis 1 GiB|5.1|
|`cluster_seg_min`|4 MiB|256 KiB bis 64 MiB|5.1|
|`dp_cluster_min_bytes`|1 MiB|64 KiB bis 16 MiB|5.5|
|Segmentblock (Klartext)|64 KiB|fest je `format_version`|5.1|
|`group_commit_window`|2 ms|0 bis 20 ms|3.6|
|`reader_drain_max`|30 s|5 s bis 5 min|3.6|
|`ingest_idem_ttl`|7 Tage|1 bis 30 Tage|5.4|
|`bridge_threads`|2 × Kernzahl|1 bis 8 × Kernzahl|3.6|
|`bridge_queue_max`|1 024|64 bis 65 536|3.6|
|`min_free_bytes`|1 GiB|64 MiB bis 100 GiB|8.3|

### A.4 Löschkern

|Parameter|Startwert|Bereich|Abschnitt|
|---|---|---|---|
|`erase_return_timeout`|2 s|100 ms bis 10 s|8.3|
|`erase_sla_B`|24 h|1 h bis 72 h|8.7|
|`multi_review_max`|10 Werktage|1 bis 20|8.9|
|`measure_retries`|3|1 bis 10|8.5|
|`saga_retry_max`|8 (Backoff 1 s bis 1 h)|1 bis 100|8.3|
|`quarantine_max`|24 h|1 bis 72 h|2.2|
|`quarantine_purge_after`|7 Tage|2 bis 30 Tage|2.2|
|`history_horizon`|`Personal` 30 Tage; `Besonders` 0|0 bis 90 Tage; `Besonders` fest 0|6.6|
|`hold_review_interval`|90 Tage|30 bis 365 Tage|8.4|
|`archive_wait_max`|30 Tage|7 bis 180 Tage|8.4|
|`outbox_ttl`|90 Tage|30 bis 365 Tage|8.6|
|`backup_derived_dp`|false; bei `Besonders` fest false|false/true|5.5|

### A.5 Wissensebene und Entscheidungen

|Parameter|Startwert|Bereich|Abschnitt|
|---|---|---|---|
|`branch_retention`|90 Tage|7 Tage bis unbegrenzt|6.4, 6.7|
|`K_MIN`|5|≥ 5|7.5|
|`split_retest_interval`|12 Monate|3 bis 36 Monate|2.3|
|`decision_retention`, `payload_retention`|**Pflicht** je `DecisionKind`|–|7.1, 12.7|
|`actor_handle_epoch`|12 Monate|3 Monate bis aus|7.4|
|`index_gen_retention`|= `branch_retention`|–|11.4|
|`merge_main_four_eyes`|an|an/aus|6.4|
|`merge_main_eval_gate`|an|an/aus|9.7|

### A.6 Abruf und Antwort

|Parameter|Startwert|Bereich|Abschnitt|
|---|---|---|---|
|`k_text`, `k_vec`, `k_graph` (K)|100, 100, 50|10 bis 1 000|9.4|
|`k_text_p`, `k_vec_p`, `k_graph_p` (P)|50, 50, 25|10 bis 500|9.4|
|`rrf_k`|60|1 bis 1 000|9.4|
|`fuse_keep`, `rerank_keep`|40, 12|je 5 bis 200|9.4|
|`max_chunks_per_doc`|3|1 bis 10|9.4|
|`neighbor_expand`|1|0 bis 3|9.4|
|`overlap_min`|0,30|0,1 bis 0,8|9.4, 9.6|
|`support_min`|**Pflicht** (Messbericht)|–|9.6|
|`max_cases_per_query`|5|1 bis 20|9.4|
|`answer_cache_ttl`|24 h|0 bis 7 Tage|9.5|
|`cache_personal`|false|fest false|9.5|
|`session_ttl`|8 h|15 min bis 24 h|9.5|
|`rate_limit_per_min`|**Pflicht** je Rolle|–|9.4|
|`view_class_ttl`|60 s|1 bis 300 s|13.4|
|`token_max_age`|1 h|5 min bis 1 h|13.4|
|`chunk_cfg` v1|300 / 450 / 60 Wörter, Überlappung 40|siehe 9.1.1|9.1.1|
|BM25|`k1` 1,2; `b` 0,75|fest je `retrieval_cfg`|9.2|
|Regressionsgrenzen|Recall@10 −0,02; Zitiergenauigkeit −0,02; Enthaltungsgenauigkeit −0,03; Latenz p95 +20 %|enger setzbar|9.7|
|`goldset_interval`|7 Tage|1 bis 30 Tage|11.4|

### A.7 Ingest und Worker

|Parameter|Startwert|Abschnitt|
|---|---|---|
|Eingabegröße|512 MiB|9.1|
|CPU-Zeit je Dokument|60 s|9.1|
|Speicher des Workers|2 GiB|9.1|
|Expansionsfaktor|100:1|9.1|
|Verschachtelungstiefe|8|9.1|
|Seiten|10 000|9.1|
|Rahmen im Worker-Protokoll|höchstens 64 MiB|9.1|

Worker-Limits sind herabsetzbar, nicht heraufsetzbar ohne ADR.

### A.8 Inferenz

|Parameter|Startwert|Abschnitt|
|---|---|---|
|`infer_timeout`|Embedding 10 s, Rerank 10 s, Generierung 120 s|11.1|
|Wiederholungen|höchstens 1|11.1|
|`inference_budget`|**Pflicht** (Tokens je Stunde je Mandant)|11.5|
|`max_parallel_generations`|je Profil|11.5|
|`infer_queue_max`, `embed_queue_max`|256, 1 024|11.5|

### A.9 Betrieb

|Parameter|Startwert|Abschnitt|
|---|---|---|
|RTO|4 h (ohne DP-Neuaufbau)|14.6|
|RPO Schlüssel|≤ `vault_export_interval`|14.6|
|RPO Daten|≤ `data_backup_interval`|14.6|
|Restore-Test|quartalsweise|14.4|
|`ExportLate`|Export älter als 2 × `vault_export_interval`|14.3|
|`RequestDue`|70 % der Frist|8.2|
|`SlaBreach`|`erase_sla_B` überschritten|14.3|

---

