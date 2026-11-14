**Vier zielgerichtete Architekturvarianten** zur Implementierung der vollständigen Löschkette in _Acta_ sowie die **unbereinigte Legacy-Ausgangsbasis** extrahieren.

  

|**Variante**|**Architektur-Kern**|**Schlüssel- & Löschmechanismus**|**Saga / Zustandsmodell**|**Nachweis & Klassen**|**Primäre Ziel-Pfade**|
|---|---|---|---|---|---|
|**1. Multi-Crate-Engine-Store-Konsolidierung**|In-Place-Härtung der bestehenden Crates (`engine`, `store`, `crypto`, `ports`, `checkpoint`) ohne neues Crate|Bereichs-`ErasureFence` (Dekker-Muster), streamender SSTable-Rewrite (`purge_keys`), `ErasureLedger` (Keyed-Hashes)|9-Phasen-`PurgePhase` (`Admitted` bis `Released`) + 6-stufiger `ErasureCoordinator`|Zweistufiger Beweis (Phase A/B), kanonische TLV-Kodierung, `LayerWitness::from_scan`|`crates/acta-store/src/lsm/*`, `crates/acta-engine/src/deletion/*`, `crates/acta-crypto/src/*`|
|**2. Autarkes, synchrones Kern-Crate (`acta-erasure`)**|Vollständige Auslagerung in ein isoliertes Crate mit 0 internen Abhängigkeiten und Ports|Port-Entkopplung (`FenceGate`, `WalSink`, `KeyStore`, `ResidueScanner`), KEK/DEK-Hierarchie, Tresor-Rewrite (`vault`)|Deterministische `PurgePhase`-Enum + `EraseSaga` mit synchronem Journal und Resume|Deterministischer, kanonischer Löschbericht mit Hash-Ketten-Bindung an Scanner-Zeugen|`crates/acta-erasure/src/{saga, vault, gate, journal, ledger, purge, measure, report, restore, scope}`|
|**3. Zwei-Ebenen- & Drei-Spuren-Architektur mit `Epoch Seal` (v3.0 / v3.1)**|Trennung in Ebene A (Klartext-Purge), Ebene B (Crypto-Shredding) und Spur Backup (Daten vs. Schlüssel)|`DocDek` je Einheit, `ArtKey` je Index-Artefakt, `KEK`-Epochen mit **Re-Wrap-then-Destroy** (`Epoch Seal`), Custody `K1`–`K3`|10-Schritte-Saga: Fast-Path (Schritte 1–7) vor terminierter Hygiene (Schritte 8–10)|`DeletionProof` V5, 4 verkettete Siegel (L, P, C1–C3, M), `Verified` vs. `Validated`, eIDAS-Zeitanker|`crates/acta-ports/src/erasure.rs`, `crates/acta-crypto/src/{epoch_keys.rs, residual.rs}`, `crates/acta-store/src/erasure/*`|
|**4. Verschlankter v1-Neubau („Acta Core v1“) ohne KV-Cache**|Minimalistischer Neubau (< 2.500 LOC) unter Entfernung von KV-Cache und altem Löschpfad; Einfrieren von 15 Crates|Transaktionales `ErasureManifest`, `DocDek` mit 128-Bit-`scope_id`, zentraler `ReadGate` („Erase gewinnt“ bei Zeitreisen)|7-Zustände-Saga (`Requested` bis `Completed`) über vereinheitlichtes async `Vfs`-Journal|`DeletionProof` v4 (Ed25519) aus unabhängigem Rohscan (`ForensicScanner`) mit Klassen L, P, C, C-soft, E|`crates/acta-types/src/erasure/*`, `crates/acta-store/src/erase/*`, `crates/acta-engine/src/erasure/*`|

### Variante 1: Konsolidierte Multi-Crate-Pipeline (In-Place-Härtung der Bestands-Crates)

Diese Variante belässt die Löschkette verteilt über die bestehenden Workspace-Crates (`acta-engine`, `acta-store`, `acta-crypto`, `acta-ports`, `acta-checkpoint`) und härtet die vorhandene Multi-Crate-Pipeline ohne Auslagerung in ein neues Crate.

  

#### Funktionsweise

- **Einheitlicher Koordinator & Modussteuerung:** Sämtliche Löschaufrufe (`forget`, `forget_collection`, `forget_tenant`, `drop_collection`) laufen durch einen einzigen `ErasureCoordinator::execute(req: ErasureRequest)` mit den Zuständen `Requested → Tombstoned → Purged → WalRetired → Verified { witnesses } → Attested { proof }`. Über `ErasureMode` (`Off`, `Assured`, `Attested`) wird gesteuert, ob Scope-Persistenz, Ledger-Verankerung (vor dem Tombstone-Commit), physischer Purge, Schicht-Scans und kryptografische Attestierung ausgeführt werden.
    
      
    
- **9-Phasen-Zustandsmaschine (`PurgePhase`):** Der physische Purge folgt einer strikten Enum-Ordnung (`#[repr(u8)]` von `1` bis `9`):
    
      
    1. `Admitted (1)`: Schreibt den Scope verschlüsselt in eine eigene Datei `purge-<purge_id hex>.scope` über `ScopeSealer` (AEAD mit `aad = purge_id || scope_hash`, Schlüssel über `blake3::derive_key("acta/purge-scope/v1", master)`, oder `PlainSealer` mit CRC bei unverschlüsselten Datenbanken) samt `fsync` auf Datei und Verzeichnis, und persistiert danach Intent v2. Single-Flight wird über `purge_slot: tokio::sync::Mutex<()>` in `LsmStorage` erzwungen.
        
          
        
    2. `Fenced (2)`: Installiert eine bereichsbezogene Schreibschleuse (`ErasureFence::enter_write`) nach dem Dekker-Muster: Zuerst wird `inflight` atomar erhöht (`fetch_add(1, SeqCst)`) und sofort ein RAII-`WriteTicket` konstruiert; erst danach werden `active.load(SeqCst)` sowie die über `ArcSwap` verwaltete Exakt-/Präfix-Tabelle geprüft. Alle `IndexOp`-Varianten werden erschöpfend über `fenced_keys` abgebildet.
        
          
        
    3. `Flushed (3)`: Flusht die Memtable (`PurgeBackend::flush_memtable`).
        
          
        
    4. `Rewriting (4)`: `purge_keys(&fence, &scope, as_of_tx, probe)` schreibt betroffene SSTables streamend über einen lazy erzeugten `SstableBuilder` um (Speicherbedarf $O(\text{Block})$), schreibt die Liste der obsoleten Dateien **vor** dem Manifest-Tausch ins Journal (Regel J1), führt einen Verzeichnis-`fsync` nach `rename` aus und tauscht die Reader im RAM direkt positionserhaltend über `Vec<(PathBuf, Option<Arc<SstableReader>>)>` aus.
        
          
        
    5. `Disposing (5)`: `dispose_obsolete_files_with_policy` gleicht Journal-Einträge zuerst gegen die Live-Menge des Manifests ab (um nach einem Absturz zwischen Journal und Manifest keine lebenden Dateien zu löschen), wartet auf Reader-Drain (`Arc::strong_count == 1`), invalidiert den Block-Cache (`block_cache.invalidate_file(id)`), scrubbt und entfernt die Dateien.
        
          
        
    6. `WalRetiring (6)`: `PurgeBackend::retire_wal` rotiert das aktive WAL-Segment, schreibt zuerst einen Manifest-`WalCheckpoint { hmac }`, entsorgt vollständig überholte Segmente via `retire_segments_up_to` inklusive Scrubbing und schreibt gemischte Segmente über `execute_scope_purge` unter Neuverkettung der HMAC-Kette um.
        
          
        
    7. `Witnessing (7)`: Führt Schicht-Scans aus und erzeugt `LayerWitness::from_scan(ScanOutcome, ...)` (der direkte Konstruktor `LayerWitness::issue` wird auf `pub(crate)` eingeschränkt; bei `matches_found > 0` oder `units_scanned == 0` ohne `EmptyLayer`-Deklaration erfolgt eine Ablehnung). Keyed-Hashes (`compute_keyed_hash`) werden im `ErasureLedger` gespeichert.
        
          
        
    8. `Finalizing (8)`: Erstellt einen zweistufigen Beweis (Phase A nach Tombstone-Commit für `LsmMemtable`, Phase B nach Verifikation) mit kanonischer TLV-Kodierung (`Tag u16 LE || Länge u32 LE || Wert`) und einem zentralen `ProofSignerProvider` im Composition Root.
        
          
        
    9. `Released (9)`: Gibt den Fence frei, scrubbt und löscht zuerst die Scope-Datei und entfernt erst danach das Intent.
        
          
        
- **Crash-Recovery & Restore-Schutz:** `LsmStorage::open_internal` baut zuerst die vollständige `LsmStorage`-Instanz auf und ruft anschließend `recover_pending_purge()` auf. `PurgeOrchestrator<B: PurgeBackend>::resume()` führt die zuletzt betretene Phase idempotent erneut aus, prüft alle Live-SSTables per Streaming-Scan auf Restspuren (`PurgeError::ResidueFound`) und startet erst danach `run_orphan_gc()`. Beim Öffnen prüfen `verify_wal_anchor` und `reapply_tombstones_from_ledger` die WAL-Integrität, während `PersistentCheckpointStore` mit `RestorePolicy::{Filter, ForbidAfterErasure}` und `LedgerSource::{Direct, Handle}` Backups beim Restore filtert.
    
      
    

#### Pfade und Module

- **Engine-Koordinator & Treiber:** `crates/acta-engine/src/acta_impl/forget.rs`, `crates/acta-engine/src/acta_impl/collections.rs`, `crates/acta-engine/src/deletion/purge_driver.rs`, `crates/acta-engine/src/deletion/text_witness.rs`, `crates/acta-engine/src/deletion/relations_witness.rs`
    
      
    
- **Store-Purge, Recovery & Fence:** `crates/acta-store/src/lsm/recovery.rs`, `crates/acta-store/src/lsm/fence.rs`, `crates/acta-store/src/lsm/ops/purge.rs`, `crates/acta-store/src/lsm/ops/write.rs`, `crates/acta-store/src/lsm/ops/dispose.rs`, `crates/acta-store/src/lsm/purge/{orchestrator.rs, scope_store.rs, failpoint.rs}`, `crates/acta-store/src/compaction/purge_filter.rs`
    
      
    
- **WAL & Ledger:** `crates/acta-store/src/wal/{retire.rs, scope_purge.rs, replay_filter.rs, hmac.rs}`, `crates/acta-store/src/erasure_ledger/{mod.rs, entry.rs}`
    
      
    
- **Kryptografie, Ports & Checkpoints:** `crates/acta-crypto/src/canonical.rs`, `crates/acta-crypto/src/deletion_proof/{issuer.rs, proof_store.rs}`, `crates/acta-ports/src/erasure.rs`, `crates/acta-checkpoint/src/store.rs`
    
      
    

### Variante 2: Autarkes, synchrones Kern-Crate (`acta-erasure` Standalone-Architektur)

Diese Variante kapselt die gesamte Löschkette in einem **eigenständigen, autarken Rust-Crate** (`crates/acta-erasure`) mit `#![forbid(unsafe_code)]`, **null internen Crate-Abhängigkeiten** (keine Abhängigkeiten auf `acta-types`, `acta-ports`, `acta-crypto` oder `acta-store`) und strikt synchronem Kern.

  

#### Funktionsweise

- **Port-Adapter-Entkopplung (`types_local.rs`):** Alle Infrastrukturzugriffe werden im Crate selbst als Traits (Ports) definiert und außerhalb durch Infrastruktur-Crates als Adapter implementiert. Neben den Basis-Ports (`StorageBackend`, `Purger`, `LayerScanner`, `Clock`, `Rng`) definiert das Crate vier spezialisierte Lösch-Ports: `FenceGate` (Nebenläufigkeitsschutz beim Purge, Adapter in `acta-store`), `WalSink` (WAL-Entsorgung und Scope-Purge, Adapter in `acta-store`), `KeyStore` (Schlüsselvernichtung und -lesen, Adapter in `acta-crypto` / Vault) und `ResidueScanner` (Restspurenprüfung im Vektorindex, Adapter im Index-Crate).
    
      
    
- **Synchroner Kern ohne Async-Runtime-Kopplung:** Die Orchestrierung ist von Async-Runtime-Stacks entkoppelt; Hintergrund-Akteure mit eigenen Threads (wie der historische `wal/flusher.rs`) sind ausgeschlossen.
    
      
    
- **Schlüsselhierarchie & Tresor-Vernichtung (`vault`):** Eine zweistufige Schlüsselhierarchie verwaltet einen KEK pro Gruppe und gewrappte DEKs pro Datensatz (`kv_shredding.rs`, `vault/keyring.rs` über `KeyStore`). Die Vernichtung eines KEK macht alle abhängigen DEKs der Gruppe unlesbar, wobei DEKs nie ungelöscht im Klartextspeicher verbleiben. Die physische Schlüsselvernichtung (`purge_keys`) nutzt atomare Rewrites mit Journal, Manifest und Speicher-Swap-Absicherung (`vault/sealer.rs`), wobei jeder E/A- oder Synchronisationsfehler das Vault-Handle dauerhaft vergiftet (Fail-Closed-Invariante `INV-VAULT-FAILCLOSED-1`).
    
      
    
- **Zustandsmaschine, Scope-Persistenz & Resume:** Eine phasenspezifische Fehlertaxonomie (`error.rs`) und eine einheitliche Zustandsmaschine `PurgePhase` (`purge/`) steuern den Ablauf über explizite Übergangsprüfungen (`can_transition_to`). Der Lösch-Scope wird absturzsicher in `scope/` persistiert und mit Single-Flight-Garantie versehen (höchstens ein laufender Purge pro Scope). Nach einem Absturz setzt die Wiederaufnahme (`saga/driver.rs` und `restore/replay.rs`) den unterbrochenen Purge aktiv fort.
    
      
    
- **Ledger, Kanonische Berichte & Restore-Guard:** Jede Schlüsselvernichtung wird vor dem Ende der Saga in einem persistenten Fail-Closed-Ledger (`ledger.rs`, Ausbau von `InMemoryLedger` zu `ErasureLedger`) verankert. Deterministische, kanonische Löschberichte (`report/`) binden verifizierte Scanner-Zeugen (`measure/witness.rs`) über eine Hash-Kette (`INV-ERA-7`, `INV-CLASS-MEASURED-1`). Ein Restore-Guard (`restore/guard.rs`) verhindert die Reaktivierung gelöschter Schlüssel bei Backup-Wiederherstellungen.
    
      
    

#### Pfade und Module

- **Crate-Kern & Ports:** `crates/acta-erasure/Cargo.toml`, `crates/acta-erasure/src/lib.rs`, `crates/acta-erasure/src/types_local.rs`, `crates/acta-erasure/src/error.rs`
    
      
    
- **Saga & Orchestrierung:** `crates/acta-erasure/src/orchestrator/`, `crates/acta-erasure/src/saga/machine.rs`, `crates/acta-erasure/src/saga/driver.rs`
    
      
    
- **Tresor & Schlüsselverwaltung:** `crates/acta-erasure/src/vault/slot.rs`, `crates/acta-erasure/src/vault/sealer.rs`, `crates/acta-erasure/src/vault/keyring.rs`, `crates/acta-erasure/src/kv_shredding.rs`, `crates/acta-erasure/src/revocation_log.rs`
    
      
    
- **Scope, Purge & Sperren:** `crates/acta-erasure/src/scope/{codec.rs, file.rs, set.rs}`, `crates/acta-erasure/src/purge/{window.rs, order.rs, reserve.rs, cx.rs}`, `crates/acta-erasure/src/gate/{erased_set.rs, read_gate.rs}`
    
      
    
- **Journal, Ledger, Berichte & Restore:** `crates/acta-erasure/src/journal/`, `crates/acta-erasure/src/ledger.rs`, `crates/acta-erasure/src/measure/witness.rs`, `crates/acta-erasure/src/report/{build.rs, canonical.rs, signing.rs}`, `crates/acta-erasure/src/deletion_proof.rs`, `crates/acta-erasure/src/restore/{guard.rs, replay.rs}`
    
      
    

### Variante 3: Zwei-Ebenen- & Drei-Spuren-Architektur mit `Epoch Seal` (v3.0 / v3.1)

Diese Variante kombiniert die physische Bereinigung von Klartextstrukturen (**Ebene A**) mit der kryptografischen Entwertung persistenter Reste (**Ebene B**) und einer getrennten Backup-Führung (**Spur Backup**), um sowohl Flash-/Wear-Leveling-Reste als auch unstrukturierte Index-Artefakte und Backups abzudecken.

  

#### Funktionsweise

- **Schlüsselhierarchie, `ArtKey` & Custody-Stufen (`K0`–`K3`):** Über den Port `EffaceableKeySlot` wird eine Wurzel in einer von vier Custody-Stufen verwaltet: `K0` (unverschlüsselt, nur Entwicklung), `K1` (Argon2id aus Passphrase `m ≥ 64 MiB, t ≥ 3` + Software-Keystore), `K2` (manipulationsresistentes Modul wie TPM 2.0 / HSM) oder `K3` (externer KMS/HSM mit Wartefenster und Vernichtungsattest). Darunter hängen epochenbezogene `KEK[coll, epoche]`, die folgende Schlüssel wrappen:
    
      
    - `DocDek[einheit]`: 32-Byte-Schlüssel je Betroffenen-Einheit für Records in WAL, SSTables, Blob-Segmenten sowie Vektor-Payloads im Snapshot. Dokumente mit mehreren Betroffenen werden in Segmente zerlegt und über eine `Mixed-Policy` (`BlockAll`, `RedactAndRekey`, `DeleteAll`) behandelt.
        
          
        
    - `ArtKey[artefakt]`: Artefaktschlüssel für geteilte Index-Strukturen (je Text-Segment und je Graph-Snapshot-Generation).
        
          
        
    - `K_ref[tenant, epoche]`: Zufälliger Referenzschlüssel für `HMAC-SHA256`-Referenz-Hashes in Berichten und Logs.
        
          
        
- **KeyTable, Padmé-Padding & `Epoch Seal` (`Re-Wrap-then-Destroy`):** Gewrappte Schlüssel liegen getrennt vom Chiffretext in einer **KeyTable** mit Ersetzen-Semantik (nie in einem Append-Only-Log). Record-Header (`doc_key_id`) liegen verschlüsselt unter `K_eng`, Nutzdaten werden nach dem **Padmé-Schema** gepaddet (mindestens 256 Byte), und die 96-Bit-Wrap-Nonce dient als Suchmuster („Fahndungsnadel“). Beim Löschen wird der `DocDek` in der KeyTable genullt und die Segmentdatei atomar neu geschrieben. Spätestens beim **Epoch Seal** (zyklisch nach `kek_epoch_len` = 30 Tage, manuell via `seal-now` oder spätestens `seal_after_deletion_max` = 7 Tage nach einer Löschung) wird `KEK[coll, e+1]` erzeugt, alle _überlebenden_ `DocDek`s und `ArtKey`s werden unter `KEK[e+1]` neu gewrappt ($O(\text{Anzahl Schlüssel})$ ohne Daten-Rewrite), die KeyTable atomar ersetzt und die alte `KEK[e]` (inklusive optionaler Passphrasen-Rotation bei `K1` bzw. Hardware-`destroy()` bei `K2`/`K3`) vernichtet. Dadurch werden auch Flash-Reste früherer gewrappter `DocDek`s unlesbar.
    
      
    
- **10-Phasen-Saga (Fast-Path vor Hygiene) & 4 verkettete Siegel:** Während in Vorstufe v3.0 die Ebene-A-Bereinigung vor der Schlüsselvernichtung lag, zieht v3.1 die Schlüsselvernichtung durch record-granulare Verschlüsselung in den **Fast-Path (Minuten bis Stunden)** vor:
    
      
    1. `Frozen`: Eintrag in `ErasedSet` (`fsync`), Lesesperre, `legal_state = Eingeschränkt` → **Siegel 1 (Klasse `L`)**.
        
          
        
    2. `LogicallyDeleted`: WAL-Tombstone + Lineage-Kaskade.
        
          
        
    3. `PlaintextPurged`: Ebene A nullen/zeroizen (HNSW-Arena-Slots `fp32` und `SQ8` ohne Graph-Rewiring, Caches, Surrogat-Vault); Text-Segmente unter neuem `ArtKey` neu schreiben und alten `ArtKey` sofort vernichten.
        
          
        
    4. `PlaintextMeasured`: Ebene-A-Scanner + **Pre-Destruction-Inventar** aller Chiffratblöcke je `key_id` → **Siegel 2 (Klasse `P`)**.
        
          
        
    5. `KeyDestroyed`: Nach optionaler Karenz (`erasure_grace`, 0–72 h) Vernichtung des `DocDek` in der KeyTable.
        
          
        
    6. `CryptoMeasured`: **Nadelscan** (Suche nach der 96-Bit-Wrap-Nonce in allen Dateien), **Kanarien-Probe** und Chiffrat-Identitätsprüfung → vorläufige Klasse **`C*`**.
        
          
        
    7. `Epoch Seal`: Rotation `KEK[e] → KEK[e+1]` und Vernichtung von `KEK[e]` → **Siegel 3 (Klasse `C1`, `C2` oder `C3`)** mit expliziten Annahmecodes (`A-FLASH`, `A-ALGO`, `A-KEYLEAK`, `A-RAM`).
        
          
        
    8. _Hygiene (terminiert ≤ `purge_rebuild_max_days` = 14 Tage):_ Physische Kompaktion (`compact_range`), WAL-Scrub, Blob-Rewrite, Checkpoint-Rewrite.
        
          
        
    9. _Hygiene:_ Hintergrund-Neuaufbau des HNSW-Graphen (`g → g+1` ab Tombstone-Anteil ≥ 0,2 zur Beseitigung von `R-GRAPH`-Topologieresten) und Vernichtung des alten `ArtKey[g]`.
        
          
        
    10. `HygieneMeasured` & `Reported` → **Siegel 4 (Klasse `M` – Messvollständig)**.
        
          
        
- **Zwei-Spuren-Backup, `ErasedSetMirror` & `DeletionProof` V5:** Backups werden in **Spur D** (verschlüsselte Daten ohne KeyTable/Keystore, beliebige Lebensdauer) und **Spur K** (KeyTable-Snapshots mit signiertem Ablaufdatum `valid_until` / `key_backup_max_age` = 30 Tage) getrennt. Ein `ErasedSetMirror` spiegelt `ErasedSet`-Einträge auf ein Zweitmedium/WORM; beim Restore wendet der Restore-Guard `ErasedSet` vor dem ersten Lesezugriff an (`restore_reapplied`) und lehnt abgelaufene Schlüssel-Backups (`RestoreBeyondHorizon`) ab. Der Bericht (`DeletionProof` V5 / `ErasureReceiptV4`) verkettet Siegel 1–4 in der `AuditChain`, trennt `Verified` und `Validated` (Vier-Augen-Abnahme mit Concurrence-Signatur im Regulated-Tier), unterstützt qualifizierte Zeitstempel (RFC 3161 / eIDAS Art. 41, 42), führt ein `RecipientLedger` (Art. 17 Abs. 2 / Art. 19 DSGVO) und erzeugt Außentexte ausschließlich aus einer gemessenen Satzbibliothek.
    
      
    

#### Pfade und Module

- **Typen & Ports:** `crates/acta-types/src/...` (`ErasureId`, `KeyEpoch`, `KeyRef`, `SubjectCommitment`, `ErasureError`), `crates/acta-ports/src/erasure.rs` (`EffaceableKeySlot`, `KeyCustodian`, `ErasureParticipant`, `ErasureJournal`, `ErasedSetMirror`, `RecipientLedger`, `NotaryPort`)
    
      
    
- **Kryptografie & Berichte:** `crates/acta-crypto/src/epoch_keys.rs` (`EpochKeyStore`), `crates/acta-crypto/src/residual.rs` (`ResidualReport`, `ScanOutcome`), `crates/acta-crypto/src/receipt_v4.rs` (`DeletionProof` V5), `crates/acta-crypto/src/kv_shredding.rs`, `crates/acta-crypto/src/revocation_log.rs` (reine Hash-Kette ohne I/O)
    
      
    
- **Store & Schicht-Participants:** `crates/acta-store/src/erasure/{mod.rs, keystore.rs, lsm.rs}` (`AtomicFile`, `DurableAppendLog`, `RawFileScanner`, `KeyTable`), `crates/acta-store/src/lsm/ops/compaction.rs`, `crates/acta-vector/src/hnsw/*`, `crates/acta-text/src/inverted/*`, `crates/acta-graph/src/cascade.rs`, `crates/acta-checkpoint/src/store.rs`
    
      
    
- **Orchestrierung, CLI & Export:** `crates/acta-engine/src/erasure/*` (`ErasureOrchestrator`, `ErasedSet`), `crates/acta-mcp/*` (`acta_erase`), `crates/acta-audit-export/src/*`, CLI `doctor verify-report` / `doctor export-concept`
    
      
    

### Variante 4: Verschlankter v1-Neubau („Acta Core v1“) mit `ErasureManifest` & `ReadGate`

Diese Variante setzt auf einen **minimalistischen Neubau** unter einem Code-Budget von **unter 2.500 Zeilen Produktionscode** für alle Erase-Module, friert 15 Rand-Crates aus dem Workspace aus und **entfernt sowohl den gesamten alten Löschpfad als auch das komplette KV-Cache-Subsystem** (`acta-kvcache`, `acta-store::kv`, `acta-crypto::{kv_cipher, kv_segment, kv_shredding, revocation_log}`, Feature `content-addressed-kv-cache`, `kv_cache_tensor` in `acta-infer-candle`).

  

#### Funktionsweise

- **Öffentliche Oberfläche & transaktionales `ErasureManifest`:** Einziger Einstiegspunkt ist `erase(target: EraseTarget, on_retention: RetentionDecision, actor: &Authz) -> Result<ErasureTicket>` (ergänzt um `erasure_status` und `resume_pending`). Beim Einfügen eines Dokuments wird in derselben Transaktion ein **`ErasureManifest`** geschrieben, das sämtliche erzeugten Hilfsschlüssel über ein exhaustives `KeyKind`-Enum (`User`, `DocMap`, `Rel`, `Intent`, `Text`, `Graph`, `Vector`, `ErasureManifest`) auflistet und vor der Schlüsselvernichtung ausgelesen wird.
    
      
    
- **`DocDek`-Hülle mit 128-Bit-`scope_id`:** Aus `KEK_root` (Argon2id `m ≥ 64 MiB, t ≥ 3, p ≥ 1`) und `KEK_coll` (HKDF-SHA256) wird pro Dokument ein `DocDek` (32 Byte `OsRng`, `!Clone`, `!Copy`, `Zeroize`, Zugriff nur via `with_key`) über den async Port `KeyAnchor` (`Hardware`, `OsKeyring` oder `FileOnly`) und die `KeyTable` verwaltet. Die Verschlüsselung nutzt AES-256-GCM-SIV mit Zweierpotenz-Padding ab 256 Byte und AAD `tenant || collection || KeyKind || scope_id || fmt_ver`, wobei `scope_id` ein 128-Bit `BLAKE3-keyed(K_scope, tenant || collection || user_key)` ist, um 64-Bit-`DocId`-Kollisionen beim Löschen auszuschließen.
    
      
    
- **Zentraler `ReadGate` & Zeitreise-Semantik („Erase gewinnt“):** `TenantScopedStorage` (`crates/acta-store/src/tenant_codec.rs`) wird als einziger Zugang der Engine zum Store (`LsmStorage` wird `pub(crate)`) mit dem `ReadGate` ausgestattet. Vor jedem Lesen (`get`, `get_at_seq`, `scan_prefix`, `scan_prefix_at`) prüft der `ReadGate` gegen das durabel gespeicherte `ErasedSet` (`SubjectCommitment = BLAKE3-keyed(K_commit[tenant, epoch], canonical(scope))`) und liefert bei Treffer sofort `Erased` – auch bei historischen MVCC-Zeitreisen (`get_at_seq`), sodass offene Snapshots nur den physischen Unlink verzögern, aber keine Daten mehr liefern.
    
      
    
- **7-Zustände-Saga, async `Vfs` & Streamende Purge-Compaction (`erase_keys`):**
    
      
    - Die reine Zustandsmaschine (`types::erasure`) durchläuft `Requested → Frozen → LogicallyDeleted → PlaintextPurged → KeyDestroyed → Measured → Completed` (sowie `Restricted` bei Aufbewahrungspflicht und `Pending(attempt, next_retry_at)` bei Fehlern).
        
          
        
    - Persistenz erfolgt ausschließlich über einen vereinheitlichten async `Vfs`-Trait (direktes `std::fs`/`tokio::fs` ist in Erase-Modulen per Lint verboten). Der Scope im Saga-Journal ist unter einem Ephemeralschlüssel `K_saga[erasure_id]` verschlüsselt, der bei `Completed` vernichtet wird.
        
          
        
    - `LsmStorage::erase_keys` (`crates/acta-store/src/erase/rewrite.rs`) setzt eine Barrier exklusiv gegen Compaction (nicht gegen Reads/Writes), führt einen WAL-Drain (`rotate_and_seal`, Flush, Entfernen mit Existenzprüfung) durch, schreibt SSTables streamend via `SstableBuilder::create_like` unter Erhalt von `KeyManager`, `seq`, `tx_id` und Tombstone-Flags mit exaktem Matcher (`Exact(bytes)` / `Prefix{prefix, fixed_suffix_len}`) um, tauscht das Manifest atomar aus (`ersetze {alt} durch {neu}`), löscht alte Dateien verzögert über `PendingDeleteJournal` (inklusive Sweep über alle Hardlink-Checkpoints/Forks) und invalidiert den Block-Cache mit `zeroize`.
        
          
        
    - Ebene-A-Adapter (`LayerPurger` und `LayerScanner` getrennt) führen HNSW-Slot-Zeroing (`fp32` und `SQ8`) **ohne** Graph-Rewiring durch (`hnsw/deletion.rs` entfällt; Hintergrund-Rebuild bei Tombstone-Quote 0,2), schreiben Textindex-Postings anhand der Termliste aus dem `ErasureManifest` neu, entfernen Graph-Eingangskanten und bereinigen registrierte `Purgeable`-Caches.
        
          
        
- **Unabhängige Roh-Messung (`Erasure Lab`) & `DeletionProof` v4:** `ForensicScanner` (Rohbyte-Scan ohne SSTable-/WAL-Parser) und `HeapScanner` (`/proc/self/mem`) prüfen mit verpflichtender Positivkontrolle (`CanaryFactory`). Die reine Funktion `classify` erzeugt daraus den asymmetrisch signierten (Ed25519) `DeletionProof` v4 mit den Klassen **L** (Logisch), **P** (Physisch-Klartext), **C** (Kryptografisch mit `Hardware`/`OsKeyring`-Anker), **C-soft** (`FileOnly`-Anker unter Ausweis von `ResidualRisk::KeyStoreRemanence`) und **E** (Ebene-A-Index/Cache).
    
      
    

#### Pfade und Module

- **Reine Typen & Ports:** `crates/acta-types/src/erasure/{scope.rs, state.rs, class.rs, evidence.rs, catalog.rs}`, `crates/acta-ports/src/erasure.rs`, `crates/acta-ports/src/vfs.rs`
    
      
    
- **Kryptografie & Proof v4:** `crates/acta-crypto/src/{docdek.rs, envelope.rs, keytable.rs, commit_key.rs, proof_v4.rs, audit_chain.rs, kdf.rs}`
    
      
    
- **Store-Erase, ReadGate & Checkpoints:** `crates/acta-store/src/erase/{barrier.rs, rewrite.rs, scan_raw.rs, journal.rs, keytable_store.rs, read_gate.rs}`, `crates/acta-store/src/tenant_codec.rs`, `crates/acta-store/src/sstable/{builder.rs, reader.rs, block_cache.rs}`, `crates/acta-store/src/manifest/*`, `crates/acta-store/src/wal/*`, `crates/acta-durable-fs/src/scrub.rs`, `crates/acta-checkpoint/src/*`
    
      
    
- **Engine-Saga & Ebene-A-Adapter:** `crates/acta-engine/src/erasure/{saga.rs, gate.rs, recovery.rs}`, `crates/acta-engine/src/collection/crud/delete.rs`, `crates/acta-engine/src/collection/mod.rs`, `crates/acta-text/src/inverted/{purge.rs, scanner.rs}`, `crates/acta-vector/src/hnsw/purge.rs`
    
      
    
- **Erasure Lab (Testkit):** `crates/acta-testkit/src/erasure_lab/*` (`FaultVfs`, `ForensicScanner`, `HeapScanner`, `CanaryFactory`, `ReferenceModel`, `RecordingStorage`, `NeighborDigest`)
    
      
    

### Ausgangsbasis (Kontrast): Fragmentierter Legacy-Ist-Stand vor dem Umbau

Alle vier Zielvarianten grenzen sich explizit von der unbereinigten Bestandsimplementierung ab, in der keine durchgängige Löschkette existierte:

  

- **Fragmentierte Einstiegspunkte:** Standard-Löschungen über `Collection::delete` (`crates/acta-engine/src/collection/crud/delete.rs`) erzeugten lediglich MVCC-Tombstones (`IndexOp::Delete`) ohne Schlüsselvernichtung oder physisches Überschreiben und gaben den Key-Lock (`kv_locks`) vor der Transaktion frei. Daneben existierten getrennte Pfade über `Acta::execute_forget_pipeline` (`crates/acta-engine/src/acta_impl/forget.rs:167`), `DpiPurgeDriver::run_physical` (`crates/acta-engine/src/deletion/purge_driver.rs:129`), Inline-Löschungen in `collections.rs` sowie der MCP-Handler `crates/acta-mcp/src/tools_crud.rs` mit hartkodiertem Fallback-Schlüssel und ungemessenen `0`-Konstanten im Beweis.
    
      
    
- **Unverdrahtete Komponenten & Lücken:** Crypto-Shredding existierte nur flüchtig im RAM für KV-Cache-Segmente (`crates/acta-crypto/src/kv_shredding.rs`, wobei der gewrappte DEK im Payload verblieb), `ErasureLedger` (`crates/acta-store/src/erasure_ledger/mod.rs`) und WAL-Retire (`crates/acta-store/src/wal/retire.rs`, `wal/scope_purge.rs`) besaßen keine Produktionsaufrufer, und `PurgeOrchestrator::resume` (`crates/acta-store/src/lsm/purge/orchestrator.rs`) lief beim Neustart ohne Storage-Backend als No-Op.