# Acta – Gesamtspezifikation

*Crates, Funktionsweise, Löschkette und Zeitreise · Fassung 2.0-E · Stand 2026-11-14 · Bezug: Repository tfufuz1/acta (HEAD ea1de2c), AGENTS.md, docs/ACTA\_SPEC.md, ACTA\_SPEZIFIKATION.md 1.0-E, ACTA\_ERASURE\_IMPL\_SPEC.md, docs/Loesch-Architekturen.md*

Normative Wörter: **MUSS**, **DARF NICHT**, **SOLL**, **KANN** (RFC 2119). Dieses Dokument ist die Arbeitsgrundlage für die Umsetzung. Es trennt strikt zwischen dem, was im Code geprüft wurde, und dem, was Zielzustand ist.

---

## 0. Status, Methode, Beweisstand

### 0.1 Beweismarken

| Marke | Bedeutung |
| --- | --- |
| **\[V\]** | Am Quelltext des geklonten Repositories geprüft. Der Pfad ist genannt. |
| **\[D\]** | Aus einem Dokument übernommen (Crate-SPEC, Docs, Vorbericht), nicht gegen den Code geprüft. |
| **\[E\]** | Festlegung dieser Spezifikation (Entscheidung, Zielzustand). |
| **\[A\]** | Annahme. Muss vor der Umsetzung gegen den Code geprüft werden. |

### 0.2 Beweisstand

| Aussage | Stand |
| --- | --- |
| Repository geklont | ja, HEAD ea1de2c, 18 Crate-Verzeichnisse, 1.276 Rust-Dateien, 260.039 Zeilen (inklusive Tests) |
| Gebaut, getestet, gefuzzt | **nein**. In der Arbeitsumgebung gibt es kein cargo. Aussagen wie „alle Tests grün“ in Crate-Dokumenten sind \[D\]. |
| Gelesen (vollständig oder im Kern) | acta-erasure (Saga, Gate, Vault-Köpfe, Purge, Messung, Restore, Journal, Ledger, Report-Köpfe), acta-crypto (dek, aead, kdf, audit\_chain, Köpfe), acta-store (lib, Retention, Fence, tenant\_codec, WAL-Köpfe, Manifest-Einträge), acta-mvcc (lib, Snapshot-Revoke, Tombstone, SeqLog), acta-checkpoint (lib, Meta, Hardlink-Cloner) |
| Nur Köpfe, Cargo.toml und Löschhooks gelesen | acta-vector, acta-text, acta-graph, acta-rank, acta-simd, acta-wire, acta-mcp, acta-types, acta-ports, acta-ids, acta-env, acta-sys, acta-durable-fs |
| Nicht gelesen | docs/Acta\_Spezifikation\_v3.0, v3.1 und Crypto-Shredding-Synthese (nur über die Variantentabelle in docs/Loesch-Architekturen.md bekannt). SST-Reader, Compaction-Engine, WAL-Replay im Detail. |
| Rechtswirkung | **fachlich recherchiert (Kapitel 13), anwaltliche Freigabe steht aus**. Die DSGVO-Bezüge sind technische Auslegung, keine Rechtsberatung. |

### 0.3 Geltung

Diese Spezifikation vereinheitlicht die vorhandenen Dokumente. Wo sie eine Entscheidung trifft (Kapitel 3), ersetzt sie die widersprechende Aussage der Quelle. Wo sie schweigt, gilt die Quelle weiter.

---

## 1. Ziele, Zusagen, Nicht-Ziele

### 1.1 Mission (aus AGENTS.md)

Personenbezogene Daten MÜSSEN von Anfang an so angelegt werden, dass sie jederzeit atomar, physisch und kryptografisch rückstandsfrei aus dem gesamten RAG-System entfernt werden können (Speicher, Indizes, Ableitungen, Kopien). Gleichzeitig MUSS ein Versionssystem mit Zeitreise nachvollziehbar machen, auf welcher Grundlage (Gesetze, Richtlinien, Leitlinien, Grenzwerte, Fristen, Modell- und Prompt-Version) ein LLM entschieden hat. Beides ist nur vereinbar, wenn personenbezogene Daten von der Zeitreise entkoppelt sind.

Nicht Aufgabe: Personendaten auf dem Rechner des Nutzers oder in dessen Quelldokumenten zu entfernen. Aufgabe: das RAG-System selbst zu 100 % sauber zu halten.

### 1.2 Zusagen

Jede Zusage MUSS mindestens einen Test haben (Kapitel 11). Ohne Test ist sie keine Zusage.

| ID | Zusage |
| --- | --- |
| **S1** | Ab Rückkehr von `erase()` mit Zustand `Gated` liefert kein Lesepfad mehr Klartext des Ziels, auch nicht über Zeitreise (`as_of` beliebig). Neue Schreibvorgänge auf das Ziel werden abgelehnt. |
| **S2** | Jeder persistierte Klartext eines Betroffenen liegt in AEAD-Umschlägen unter Schlüsselmaterial, das nach `KeyDestroyed` nicht mehr existiert. Vor dem WAL wird verschlüsselt. |
| **S3** | Ein Dokument mit mehreren Betroffenen wird unlesbar, sobald einer von ihnen gelöscht wird. |
| **S4** | Ein Absturz an jedem benannten Durabilitätspunkt (§7.6) führt nach dem Neustart zur Vollendung oder zu einem sichtbaren Zustand `Pending` mit bestätigtem Gate. Ein bestätigtes Gate geht nie verloren. |
| **S5** | Die Chronik (Ebene N) enthält keine direkt oder indirekt identifizierenden Daten. Ein Eintrag mit Personenbezug wird abgelehnt (fail-closed). |
| **S6** | Eine Zeitreise über N liefert für gleiche Parameter gleiche Ergebnisse. Eine Löschung ändert N-Einträge byte-genau nicht. |
| **S7** | Kein Fehler im Schlüssel-, Zufalls-, Ableitungs- oder Siegelpfad wird zu einem gültigen Wert (kein Nullschlüssel, kein leeres Envelope, kein Ersatzschlüssel). |
| **S8** | Ein Nachweis entsteht nur aus einer Messung mit bestandener Positivkontrolle. Er nennt seine Klasse und seine Grenzen. Nicht Implementiertes liefert `NotImplemented`, nie Erfolg. |
| **S9** | Ein Restore aus Sicherung kann gelöschte Daten nicht wiederbeleben (Restore-Guard, Replay der Löschungen). |
| **S10** | Abgeleitete Strukturen (Vektor, Volltext, Graph, Caches) tragen Herkunft (Lineage). Löschung der Quelle entfernt sie oder macht sie unlesbar. |

### 1.3 Zusagen, die Acta nicht macht

Verboten in jedem Bericht und jeder Oberfläche: „DSGVO-konform“, „garantiert gelöscht“, „unwiederbringlich gelöscht“, „zertifiziert“, „revisionssicher“, „fehlerfrei“, „Löschung aus allen Sicherungen“. Nicht gelöscht werden: Klartext in RAM, Swap, Core-Dumps; Backups außerhalb von Acta; Daten bei externen Verarbeitern; Logs und Prompts des Aufrufers; Quelldateien des Verantwortlichen. Die Prüfung von Aufbewahrungspflichten (Art. 17 Abs. 3) liegt beim Aufrufer vor `erase()` und wird als `RetentionAttestation` übergeben.

### 1.4 Anonymitätskriterien für Ebene N

N gilt nur dann als frei von Personenbezug, wenn die drei Kriterien der Artikel-29-Gruppe verneint werden: **Herausgreifen** (singling out), **Verknüpfbarkeit** (linkability), **Ableitung** (inference). Daraus folgen Pflichten (§8.4): zufällige, entscheidungsbezogene Handles, Schema-Whitelist, PII-Detektor, k-Schwelle für Aggregate. Ob die Kriterien im Einzelfall erfüllt sind, bewertet der Datenschutzbeauftragte.

---

## 2. Befunde am Ist-Zustand

### 2.1 Crate-Inventar

Zeilen = alle .rs-Dateien inklusive Tests (grobe Messung mit wc).

| Crate | Ring / Rolle | Dateien | Zeilen | Abhängigkeiten (intern) | Löschbezug im Code |
| --- | --- | --- | --- | --- | --- |
| acta-ids | R0 Kennungen, TOMBSTONE\_BIT | 6 | 1.020 | – | `TOMBSTONE_BIT` (Bit 63) |
| acta-env | R0 Clock, Rng, IdGen, Metrics | 9 | 1.341 | – | Determinismus |
| acta-sys | R0 unsafe-Insel (mmap, ACL, Durability) | 14 | 2.988 | – | Tresorpuffer, fsync |
| acta-durable-fs | R0 atomare Dateiersetzung, Poisoning, Scrub, Journal | 23 | 3.491 | – | `scrub` (ungenutzt, B-09) |
| acta-simd | R0 SIMD-Distanzen (unsafe) | 21 | 6.420 | – | – |
| acta-wire | R0 FlatBuffers, JSON-RPC-Parser | 17 | 6.211 | – | – |
| acta-types | R2 Domänentypen, Fehler | 40 | 9.574 | ids | veraltete Löschtypen (deprecated) |
| acta-ports | R2 Trait-Verträge | 46 | 7.184 | env, types | `scanner` (Purger, LayerScanner), `erasure_legacy` |
| acta-mvcc | R1 SeqLog, Snapshots, TxBuffer, SSI | 36 | 9.017 | ids, env | `revoke_covering` |
| acta-crypto | R0 AEAD, KDF, WAL-HMAC, Audit-Kette | 51 | 11.307 | ids, env | DEK, Audit-Commitments |
| acta-store | R1 LSM (WAL, Memtable, SST, Manifest, Compaction) | 335 | 57.558 | ids, env, sys, mvcc, crypto | Scanner/Purger im compat-Modul |
| acta-checkpoint | R1 Checkpoints, Hardlink-Forks | 39 | 10.551 | ids, env (types, ports optional) | keine |
| acta-erasure | R3 Löschkern | 128 | 32.361 | **keine** | Kern der Kette |
| acta-vector | R1 HNSW, DiskANN (experimentell) | 218 | 36.788 | ids, env, mvcc (simd, sys optional) | HnswPurger, zeroize |
| acta-text | R1 BM25, invertierter Index | 74 | 18.176 | ids, env | TextPurger |
| acta-graph | R1 CSR-Graph, Session-DAG | 155 | 34.510 | ids, env | GraphPurger |
| acta-rank | R0/R1 Fusion, Kalibrierung, Erklärbarkeit | 58 | 10.810 | types, ports (optional) | keine |
| acta-mcp | stdio-Protokollschicht | 6 | 732 | types, ports, wire, rank, crypto | keine |

### 2.2 Befunde

Schwere: **K** kritisch (Zusage S1 bis S9 nicht erfüllbar), **H** hoch, **M** mittel, **N** niedrig.

| ID | Schw. | Befund | Beleg |
| --- | --- | --- | --- |
| B-01 | K | **Der Löschkern ist isoliert.** Kein Crate im Workspace ruft `ErasureCore`, `FileVault`, `ErasureLedger` oder `RestoreGuard`. Außerhalb von acta-erasure kommt der Name nur in Hinweistexten von deprecated-Attributen vor. `default-members` im Root-Cargo.toml enthält weder acta-erasure noch acta-ids noch acta-env; ein einfaches `cargo test` im Root führt deren Tests nicht aus. | \[V\] grep acta\_erasure über crates/, Cargo.toml |
| B-02 | K | **Die Datensatz-Versiegelung des Tresors ist kein AEAD.** `VaultSealer::seal` XORt den Klartext byteweise mit einem wiederholten 32-Byte-Schlüsselstrom (`blake3::keyed_hash(dek, konstantes Label)`), ohne Nonce, ohne Tag, ohne AAD. `SealCtx` wird ignoriert. Gleiche DEK ergibt gleichen Strom für alle Records. | \[V\] acta-erasure/src/vault/sealer.rs (`impl RecordSealer for VaultSealer`) |
| B-03 | K | **Der Store verschlüsselt keine Werte.** `RecordSealer` und `RecordAuthenticator` sind in acta-store nur Traits ohne Aufrufer. Verschlüsselt werden ausschließlich WAL-Batches, und nur über einen optionalen, passphrasenbasierten `KeyManager` für den ganzen Store, nicht je Betroffenem. In sstable/, memtable/ und lsm/ gibt es keine Verschlüsselungsaufrufe. Crypto-Shredding hat im Primärpfad nichts zu vernichten. | \[V\] grep seal/encrypt in acta-store/src, util.rs, wal/flusher.rs |
| B-04 | H | **Schlüsselraum passt nicht zusammen.** Der Store kodiert Schlüssel als ASCII `t:{tenant}:{collection}:chunk:{doc}`. Das Gate des Löschkerns arbeitet mit 16-Byte-`KeyBuf`, 65-Byte-Datenpräfix und 78-Byte-Rohschlüssel. Gate-Bereiche würden keine Store-Schlüssel treffen. | \[V\] acta-store/src/tenant\_codec.rs, acta-erasure/src/gate/erased\_set.rs, supersede.rs |
| B-05 | H | **Zwei Trait-Welten.** `Purger`, `LayerScanner`, `ScanTarget`, `RequestContext`, `ErasureId`, `WitnessLayer` existieren in acta-erasure::types\_local und in acta-ports/acta-types. LsmPurger, WalScanner, MemtableScanner, SstableScanner, HnswPurger, HnswScanner, TextPurger, TextScanner und GraphPurger implementieren die ports-Variante. Der Kern konsumiert `ErasureLayer`, das niemand implementiert. Alle Implementierungen sind als deprecated markiert („wandern nach acta-erasure“), dort ist nichts angekommen. | \[V\] grep `impl .* for` in compat/purge\*.rs, types\_local.rs |
| B-06 | H | **Vier Schlüsselhierarchien nebeneinander:** (1) acta-crypto::dek (UnitKey + Ahnen-Salts), (2) acta-erasure::crypto\_local (Kopie von 1), (3) kv\_shredding (GroupKek je 64 Records, RecordDek, gewrappt), (4) Tresor-Slots T/C/S/D/F/E, dazu acta-crypto::kv\_segment. Kein gemeinsamer Anker, keine gemeinsame Epoche. | \[V\] dek.rs, kv\_shredding.rs, vault/slot.rs |
| B-07 | H | **Fence tot, Vorrang falsch.** `FlushPurgeFence::enter_purge` hat keinen Aufrufer. `flush()` kehrt bei aktivem Purge mit `Ok(())` zurück und tut nichts. Die Doku behauptet Vorrang des Purge; im Code weicht `enter_purge` zurück, sobald `flush_active` gesetzt ist, und beide Seiten können gleichzeitig zurückweichen. | \[V\] lsm/fence.rs, lsm/ops/compaction.rs |
| B-08 | H | **Retention ohne Policy.** `retain_key_versions` behält alle Versionen über dem Floor und die neueste darunter. Der Sentinel `u64::MAX` (kein aktiver Snapshot) lässt nur die neueste Version übrig. Zeitreise im Store ist damit gleichbedeutend mit gepinnten Snapshots. Es gibt keine Klassen, keinen History-Floor und keine Ergebnisarten `Erased` oder `HorizonExceeded`. | \[V\] compaction/retention.rs |
| B-09 | H | **Kein physisches Überschreiben.** `acta-durable-fs::scrub` wird von keinem Crate benutzt. acta-store entfernt Dateien mit `vfs.remove_file`, ohne zu überschreiben. | \[V\] grep scrub, compaction/engine.rs |
| B-10 | H | **Fehlerpfade im Schlüsselpfad (Verstoß gegen S7).** `seal_record` endet auf `unwrap_or_default()` und liefert bei Fehler ein leeres Envelope. `derive_dek` verwirft den HKDF-Fehler (`let _ =`); bei Fehlschlag bliebe der Ausgabepuffer ein Nullschlüssel. `ErasureConfig::default().k_idx` ist `[0u8; 32]`. `wrap_dek` und `seal_record_with_nonce` nehmen die 12-Byte-Nonce vom Aufrufer. Ein statischer, XOR-obfuskierter Legacy-WAL-Integritätsschlüssel steht im Quelltext (Feature `legacy-wal-key`). | \[V\] acta-crypto/src/aead.rs, dek.rs; acta-erasure/src/saga/core.rs; acta-store/src/wal/hmac.rs |
| B-11 | H | **Checkpoints sind ungeschützte Kopien.** `CheckpointHardlinkCloner` verlinkt oder kopiert SSTables (Fallback `fs::copy`). Es gibt keine Registrierung dieser Kopien für die Löschung. Der Snapshot-Pin über `SnapshotRegistry` hängt an `cfg(feature = mvcc)`, das in Cargo.toml nicht definiert ist; der Pfad wird nie kompiliert. | \[V\] acta-checkpoint/src/hardlink\_cloner.rs, Cargo.toml |
| B-12 | M | **Manifest kennt keine Löschung.** `ManifestEntry` hat nur `Add`, `Remove`, `RollbackComplete`, `Replace`, `WalCheckpoint`. Weder Klasse noch Partition noch Seq-High-Water-Mark noch Löschjournal. | \[V\] acta-store/src/manifest/entry.rs |
| B-13 | M | **Doppelte Nachweis- und Ledger-Formate** in acta-erasure: `DeletionProof` (deletion\_proof.rs, 2.743 Zeilen) und `ErasureReport` (report/); zwei `ErasureLedger` (ledger::persistent, ledger::durable); `VerifiedVia` mehrfach definiert. | \[V\] ledger.rs, deletion\_proof.rs, report.rs |
| B-14 | M | **Kein Composition Root, keine Engine, keine Fassade.** Crate-Dokumente nennen acta-engine, acta-db, acta, acta-agent, acta-privacy, acta-router. Keines liegt im Workspace. acta-mcp bestätigt das und ist auf Protokoll, Eingabevalidierung und bounded stdio reduziert. | \[V\] acta-mcp/src/lib.rs, Cargo.toml |
| B-15 | M | **Async im Kern.** `ErasureCore` nutzt tokio (`spawn`, `oneshot`, `RwLock`) hinter dem Default-Feature `async`. acta-store ist durchgehend async. Der Löschkern soll synchron sein (Entscheidung D-09). | \[V\] saga/core.rs, Cargo.toml |
| B-16 | M | **Feature-Default-Flips.** `default = [workspace]` in acta-checkpoint, acta-crypto, acta-mvcc, acta-graph, acta-text; `default` von acta-rank enthält experimentelle Features. Verstößt gegen additive Features (R6). | \[V\] Cargo.toml je Crate |
| B-17 | M | **Dangling Referenzen.** Crate-SPEC.md und AGENTS.md verweisen auf docs/SPEC.md (v2 bis v5) und docs/erasure/Loeschkette\_Implementierungsspezifikation.md. Beide fehlen im Repository. Zwei gegensätzliche Top-Level-Spezifikationen liegen nebeneinander (LSM versus SQLite). | \[V\] ls docs/, crates/\*/SPEC.md |
| B-18 | N | `run_window` ohne `ErasureCore` und `scan_with_control` liefern `NotImplemented`. Das ist korrekt fail-closed und gewollt (S8). | \[V\] purge/window.rs, measure/scanner\_flow.rs |
| B-19 | H | **Übernommen aus dem Vorbericht \[D\], nicht erneut geprüft:** WAL-Längenfeld nicht im Header-CRC (F1); SST wird nach Fehler im Manifest-Commit gelöscht (F2); Race Flush gegen WAL-Rollover (F3); Manifest-Schreibfehler vergiften den Store nicht (F4); Tmp-Dateien und Waisen nicht aufgeräumt (F5, F6); Anti-Rollback der WAL-Kette (F7); Bloom-Parameter (F9). | \[D\] ACTA\_ERASURE\_IMPL\_SPEC.md §4, §11 |

### 2.3 Tragfähige Bausteine

Diese Teile MÜSSEN erhalten und verdrahtet werden, nicht neu geschrieben:

- Saga-Zustandsautomat mit reiner Übergangsfunktion (`SagaState::can_transition_to`) und Single-Flight über `derive_open_key`.
- `ErasedSet` mit `gate_epoch` und `GateGuard::linearized_read` (Lese-Linearisierung), `GateLog` als Append-Log mit Gruppen-fsync, `CommitBarrier::exclusive`.
- Tresor als Slot-Tabelle (16 Shards, 64-Byte-Slots T/C/S/D/F/E, MAC je Slot, Rehash mit Überschreiben der Altdatei, Waisen-Scrub mit Cursor, Poisoning bei E/A-Fehler).
- `Scanner<Unarmed>` → Positivkontrolle → `Scanner<Armed>` → `LayerWitness`; Zeuge nur bei `matches == 0`.
- Kanonischer Bericht (NFC-JSON), Ed25519-Signatur mit Domain-Präfix, `ErasureLedger` mit Hash-Kette, `RestoreGuard` und `replay_erasures`.
- Purge-Phasen 1 bis 9 mit persistiertem Fortschritt (`PurgeProgressTracker`), Plattenreserve-Prüfung, feste Schichtreihenfolge.
- acta-store: WAL mit HMAC-Kette, Poison-Mechanik, Fault-Injection-Features, differenzielle Tests gegen redb.
- acta-mvcc: SSI, `SnapshotRegistry::revoke_covering` (Snapshot-Widerruf je Mandant), Sperrenordnung INV-LOCK-ORDER-1.
- acta-durable-fs: `write_atomic` (tmp, fsync, rename, Verzeichnis-fsync), Poisoning, `scrub_*`.
- acta-vector: `HnswPurger` mit Nullung der Vektoren und Graph-Reparatur (zeroize).
- acta-crypto: `AuditChain` mit Commitments `BLAKE3(salt || attr)`, deren Salt unter einem Shredding-Schlüssel liegt (Grundlage für Ebene N).

### 2.4 Gesamturteil

Die Einzelteile der Löschkette sind weitgehend vorhanden und gut abgegrenzt. **Die Kette selbst existiert nicht**: Es fehlen die Verbindung zwischen Kern und Speicher (B-01, B-04, B-05), die Verschlüsselung vor dem WAL (B-02, B-03), die Klassentrennung und Retention für die Zeitreise (B-08, B-12) und ein einziger Einstiegspunkt (B-14). Diese Spezifikation legt fest, wie die Lücken geschlossen werden, ohne die guten Bausteine zu verwerfen.

---

## 3. Normative Entscheidungen

**Status: final.** Alle Entscheidungen D-01 bis D-15 wurden am 2026-11-14 vom Projektverantwortlichen freigegeben. Sie sind verbindlich und ändern sich nur durch eine neue Fassung dieses Dokuments. Abweichungen von Quelldokumenten sind je Zeile benannt.

| ID | Entscheidung | Begründung |
| --- | --- | --- |
| D-01 **(final)** | Basis ist die LSM-Architektur mit drei Ebenen (docs/ACTA\_SPEC.md). Die SQLite-Fassung ACTA\_SPEZIFIKATION.md 1.0-E wird **als Speicherspezifikation verworfen**. Ihre Regeln zu Nachweis, Wortlaut, Auditor-Prüfung, Kanarien-Test und Testvektoren gelten weiter (§7.10). | Im Repository liegen 57.558 Zeilen LSM, MVCC und SSI. AGENTS.md verlangt die Vereinbarkeit von Löschkette und MVCC-Zeitreise; SQLite hat kein MVCC im Sinne der Zielsetzung. |
| D-02 | **Der Commit-Punkt der Löschung ist die dauerhafte Vernichtung des Schlüsselmaterials.** Physisches Umschreiben ist nachgelagerte Hygiene und nie Voraussetzung für „gelöscht“. | O(1)-Zeit bis `Gated`, unabhängig von der Datenmenge. |
| D-03 | Verschlüsselt wird **vor dem WAL**, je Betroffenem und Einheit, mit einem AEAD (XChaCha20-Poly1305, 192-Bit-Zufallsnonce; Alternative AES-256-GCM-SIV). Der XOR-Strom in `VaultSealer` wird entfernt (B-02). | S2, S7. |
| D-04 | Zeitreise gilt uneingeschränkt nur für Ebene N. P ist nur zeitreisefähig, solange der Schlüssel lebt und innerhalb des Horizonts. **Erase gewinnt rückwirkend** über jedes `as_of`. | Kernkonflikt Versionierung gegen Löschung (§8). |
| D-05 | Es gibt **eine** Schlüsselhierarchie: Anker → Epochenschlüssel → Tresor-Slots (Salts T, C, S, D) → DEK (HKDF). `kv_shredding` und `kv_segment` werden Verbraucher dieser Hierarchie. `crypto_local` wird gelöscht; acta-erasure bekommt Primitive über einen Port. | B-06. |
| D-06 | Es gibt **einen** Löschpfad: `ErasureCore::execute`. Rohes Löschen im Store ist nicht öffentlich. Ein Lint (Gate G-ERA-1) verbietet andere Aufrufer. | Lehre aus fünf Löschpfaden im Vorgängerprojekt. |
| D-07 | N, P und D liegen in **getrennten LSM-Instanzen** (je Mandant) mit eigenem Manifest-Abschnitt. Compaction mischt nie Klassen. | Purge-Rewrite bleibt auf P begrenzt, N wird nie angefasst (I-5). |
| D-08 | acta-erasure bleibt autark (Klasse A, keine internen Abhängigkeiten). Die Verbindung entsteht in einem **neuen Brückencrate `acta-erasure-adapters`**, das als einziges Erasure und Store/Indizes/Checkpoints gemeinsam kennt. | R2/R3; verhindert Rückkehr der Verdrahtungslücke. |
| D-09 | Der Löschkern ist **synchron** spezifiziert. Das Feature `async` ist opt-in und nicht Default. Adapter zu async-Teilen (acta-store) laufen über einen dedizierten Worker-Pool, nie über `block_on` in Runtime-Threads. | B-15, P8. |
| D-10 | **Ein** Nachweisformat: `ErasureReport` (kanonisch, Ed25519). `DeletionProof` wird in Phase 3 entweder auf eine Sicht des Reports reduziert oder entfernt \[A: Auswahl nach Vergleich beider Formate\]. Ein `ErasureLedger`. | B-13. |
| D-11 | **Ein binärer Schlüsselraum** (KeyV2, §9.1) ersetzt die ASCII-Schlüssel. Er übernimmt das 65-Byte-Datenpräfix, auf dem Gate, GateLog und Supersede-Matcher bereits beruhen. | B-04. |
| D-12 | **Kein Selbstbericht gilt als Beweis.** Beweis ist eine Messung mit Positivkontrolle, ergänzt um eine Prüfung ohne Acta-Code (§7.10). | S8. |

---

## 4. Zielarchitektur

### 4.1 Drei Ebenen und Datenklassen

```
Ebene N  CHRONIK     Normen, Richtlinien, Konfig-, Modell-, Prompt-Stände,
(KeepAll)            Entscheidungsprotokolle, Receipts. Keine PII.
                     bitemporal, hash-verkettet, signierte Checkpoints
                         ^ Brücke: nur zufällige Handles und Commitments
Ebene P  TRESOR      Personenbezogene Einheiten, je Einheit AEAD,
(LiveWhileKeyAlive)  Schlüssel über Tresor-Slots, ReadGate, Erase gewinnt
                         ^ Lineage (Pflicht)
Ebene D  ABLEITUNG   Embeddings, HNSW, Postings, Graph, Caches, Summaries
(Window)             nie Quelle der Wahrheit, immer neu aufbaubar
```

| Klasse | Beispiele | Ebene | Zeitreise | Löschung | Retention |
| --- | --- | --- | --- | --- | --- |
| Norm | Gesetzestext in Fassung, Richtlinie v7, Grenzwert, Frist, Modell- und Prompt-Version | N | unbegrenzt, bitemporal | nur Takedown (Vier-Augen, Ausnahme) | `KeepAll` |
| Evidence | `ErasureReport`, `ContextReceipt`, Audit-Kette | N (Unterkette) | unbegrenzt | nie (per Konstruktion ohne PII) | `KeepAll` |
| Personal | Stammdaten, Vertrag mit Namen, Prompt mit PII, LLM-Antwort mit PII | P | nur Horizont (Default 30 Tage oder bis `retain_until`) | Crypto-Shred, dann Purge | `LiveWhileKeyAlive` |
| Special | Art. 9/10 (Gesundheit, Strafurteile, Biometrie) | P | Horizont 0 | wie Personal, ohne Karenz | kürzestmöglich |
| Derived | Embeddings, ANN-Segmente, Chunk-Caches, Re-Ranker-Features | D | keine, wird aus Quelle zeitgefiltert oder neu berechnet | Lineage-Purge, Schlüsselwechsel | `Window` |
| Unclassified | Ergebnis fehlgeschlagener Klassifikation | P (Quarantäne) | keine | wie Personal | bis Klärung |

**Regel BR-2:** Im Zweifel personenbezogen. Der Weg nach N führt ausschließlich über den N-Validator (§8.4). Freitext mit möglicher PII (Prompts, LLM-Ausgaben) ist immer P.

### 4.2 Ringe und Abhängigkeitsregeln

| Regel | Inhalt |
| --- | --- |
| R1 | Abhängigkeiten zeigen nur nach unten (höherer Ring hängt von niedrigerem ab). Gleicher Ring: keine Querkante. |
| R2 | Ports gehören dem konsumierenden Crate. Gemeinsame Typen nur über acta-ids und acta-types. |
| R3 | Nur Brückencrates (`acta-erasure-adapters`, Adapter je Indexcrate) kennen zwei Peers gleichzeitig. |
| R4 | Nur `acta-node` nennt konkrete Adaptertypen. |
| R5 | Features sind rein additiv, kein Default-Flip (B-16). |
| R6 | Kern-Crates enthalten kein `std::fs`, kein `SystemTime`, kein `thread_rng`, kein tokio. Zugriff nur über `Clock`, `Rng`, `IdGen`, `Vfs`. |
| R7 | `#![forbid(unsafe_code)]` überall außer acta-sys, acta-simd und dem generierten Teil von acta-wire. |
| R8 | Workspace-Lints (`unwrap_used`, `expect_used`, `panic`, `indexing_slicing` = deny) gelten für Produktionscode ausnahmslos. `acta-crypto` hebt `indexing_slicing` derzeit lokal auf (`allow`); das MUSS entfallen. |
| R9 | Jedes Crate steht in `default-members`. |

### 4.3 Soll-Crate-Karte

```
R7  acta (Fassade) | acta-server | acta-cli | acta-mcp (Protokoll + Tools)
R6  acta-node            Composition Root: Wiring, Boot-Selbsttest, Resume
R5  acta-engine          Use Cases: Ingest-Gate, Query/AsOf, Erase
    acta-erasure-adapters  Brücke Erasure <-> Store, Indizes, Checkpoint
R4  acta-chronicle       Ebene N: bitemporale Chronik, N-Validator, Receipts
R3  acta-erasure         Saga, Gate, Tresor, Messung, Bericht, Ledger, Restore
R2  acta-types | acta-ports        (Verträge; Löschtypen nur noch in acta-erasure)
R1  acta-store | acta-mvcc | acta-checkpoint | acta-vector | acta-text | acta-graph | acta-rank
R0  acta-crypto | acta-durable-fs | acta-sys | acta-simd | acta-wire | acta-ids | acta-env
Test acta-lab            FaultVfs, ForensicScanner, CanaryFactory, ReferenceModel (nur dev)
```

Neu: `acta-chronicle`, `acta-engine`, `acta-erasure-adapters`, `acta-node`, `acta-lab`. Neue Crates sind klein zu halten (Budget §6.20).

### 4.4 Datenflüsse in einem Satz je Pfad

1. **Schreiben:** Aufrufer deklariert Klasse → Ingest-Gate (N-Validator bei N) → `ReadGate`/`FenceGate` Zulassung → AEAD unter DEK der Einheit → WAL (Chiffretext) → Memtable → SST der Partition der Klasse.
2. **Lesen:** `ReadGate.check(key)` **vor** jeder Versionssuche und vor der Entschlüsselung → Versionssuche (MVCC, `as_of`) → DEK über Tresor → Entschlüsselung → zweite Prüfung `gate_epoch` (linearisiert).
3. **Zeitreise:** `AsOf { valid_at, known_seq }` → Chronik (N) liefert Normstand → P-Treffer nur über Gate und Horizont → Ergebnis `Found`, `Erased`, `HorizonExceeded` oder `NotFound`.
4. **Löschen:** `erase(target, attestation)` → Intent → Gate (Commit unter `CommitBarrier::exclusive`) → Schlüsselvernichtung (Commit-Punkt) → Receipt in N → Purge-Fenster → Ableitungen → Messung → Bericht → Ledger → `Completed`.

---

## 5. Kryptografie und Schlüsselmodell (normativ)

### 5.1 Ergänzende Entscheidungen

| ID | Entscheidung |
| --- | --- |
| D-13 | Das **Erasure-Journal liegt im Löschkern** (`DurableJournal`/`AppendLog` in acta-erasure), nicht im Manifest von acta-store. Der Store bekommt kein Konzept von Löschung (ACTA\_SPEC §6.6), nur generische Fähigkeiten (§6.10). Das weicht von ACTA\_ERASURE\_IMPL\_SPEC §6 ab, das Löschzustände im Store-Manifest führt. |
| D-14 | Präzisierung zu D-07: N, P und D sind **Partitionen eines Store je Mandant** mit eigener Memtable, eigener SST-Menge, eigenem Manifest-Abschnitt und eigenem Compaction-Picker, aber **einem** Commit- und WAL-Strang. Sonst wären Commits über Klassen (Entscheidung in N, Eingabe in P) nicht atomar. Der WAL trägt für P nur Chiffretext, daher braucht die Löschung keinen WAL-Umbau. |
| D-15 | Überschreiben (`scrub`) ist **Hygiene, kein Beweis**. Auf SSD, Copy-on-Write-Dateisystemen und Snapshots kann es wirkungslos sein. Maßgeblich ist D-02: ohne Schlüssel sind Altblöcke wertlos. Der Bericht führt das als Annahme `A-FLASH`. |

### 5.2 Primitive

| Zweck | Verfahren | Stand |
| --- | --- | --- |
| Datensatz-AEAD | XChaCha20-Poly1305, 192-Bit-Zufallsnonce aus dem `Rng`-Port | **neu** (ersetzt Repeating-XOR, B-02, und AES-GCM-SIV mit Aufrufer-Nonce, B-10) |
| Schlüsselableitung | HKDF-SHA256 | vorhanden (`acta-crypto::kdf`) |
| Passphrase | Argon2id mit Untergrenzen m ≥ 19.456 KiB, t ≥ 2, p ≥ 1, Header `KdfHeader` | vorhanden |
| Integritätskette WAL, Slot-MAC | HMAC-SHA256 | vorhanden |
| Commitments, IDs, Tags | BLAKE3 (keyed) | vorhanden |
| Berichtssignatur | Ed25519, Domain-Präfix, kanonische Bytes | vorhanden (`report/signing.rs`) |

Zufall kommt ausschließlich aus dem `Rng`-Port. Schlägt die Zufallsquelle fehl, schlägt die Operation fehl. Es gibt keinen Ersatzwert.

### 5.3 Hierarchie (eine Hierarchie, D-05)

```
Anker (KeyAnchor: HSM, KMS, OS-Keyring, Passphrase-Argon2id)
  Epochenschlüssel E_e: K_enc, K_vm, K_commit        e = 1, 2, ...
    Tresor-Slots (maskiert/verschlüsselt unter K_enc, MAC unter K_vm)
      U  K_unit    je Mandant, zufällig, bei Rotation nur neu gewrappt
      T  Salt_T    je Mandant
      C  Salt_C    je Collection
      S  Salt_S    je Betroffenem (Subject)
      D  Salt_D    je Dokument (doc_tag)
      F  FileKey   je Datei (nur Ingest-Modus)
    DEK(doc) = HKDF-SHA256( ikm  = K_unit,
                            salt = SHA-256(label || Salt_T || Salt_C || n || Salt_S_1..n || Salt_D),
                            info = label || doc_tag )
```

- `n` ist die Zahl der Betroffenen als `u16` big-endian, `Salt_S_i` sortiert nach Handle. Für `n = 0` entfällt der S-Teil. Es gibt **keine** Nullen als Ersatz für fehlende Bestandteile; ein fehlender Slot ist `Erased` oder Fehler (S7). Das ändert den Ist-Zustand (`derive_dek` füllt `None` mit `[0u8; 32]`) und bricht die Kompatibilität der Ableitung. Migration: Neu-Versiegelung (Format v3, §9.4).
- **Vernichtung wirkt nach oben ableitbar:** Slot S weg ⇒ alle DEKs aller Dokumente dieses Betroffenen unableitbar (S3). Slot D ⇒ ein Dokument. Slot C ⇒ eine Collection. Slot T und U ⇒ der Mandant. Jede Vernichtung ist O(1) in der Datenmenge.
- Die Zuordnung der Slot-Typen U und T ist \[A\]: In vault/slot.rs sind T, C, S, D, F, E belegt. K\_unit liegt entweder in T oder in einem neuen Typ U. Vor Umsetzung prüfen.
- Kein Schlüsselmaterial wird je im Klartext persistiert. DEKs leben im RAM in `SecretKey` (Zeroize) und im `dek_cache` des Sealers, der bei Gate und Vernichtung invalidiert wird.

### 5.4 Epochenrotation (macht die Vernichtung backup-fest)

Problem: Kopien alter Tresor-Dateien (SSD-Reste, Snapshots, Backups) enthalten vernichtete Slots weiterhin, solange `K_enc` des alten Zustands noch existiert.

1. Alle `T_ROT` (Default 24 h) oder nach `N_SHRED` Vernichtungen: neuen Epochenschlüssel `E_{e+1}` erzeugen und im Anker wrappen.
2. Alle lebenden Slots unter `E_{e+1}` neu schreiben (neue Datei, fsync, rename, Verzeichnis-fsync, Altdatei überschreiben). Werte bleiben gleich, nur die Hülle wechselt. DEKs ändern sich nicht.
3. `E_e` im Anker **vernichten** (`KeyAnchor::destroy`), dann Keyring-Datei atomar ersetzen.
4. Journal-Eintrag `EpochSealed{e}` und Ledger-Eintrag.

Garantie: Jede Kopie des Tresors aus Epoche ≤ e ist ohne `E_e` unlesbar. Backups des Keyrings MÜSSEN auf `T_ROT_BACKUP` begrenzt sein (Betreiberpflicht, §10.5). Die vorhandene Rehash-Routine (`vault/rehash.rs`) ist Grundlage von Schritt 2.

### 5.5 Umschlag v3 (Datensatz)

```
[ver u8 = 3][alg u8 = 1][doc_tag 16][nonce 24][ciphertext][tag 16]
AAD = label 'acta/rec/v3' || tenant || class u8 || keyspace u16 || user_key || seq u64 BE
```

- Umkopieren oder Replay in einen anderen Kontext schlägt die Tag-Prüfung fehl.
- Die Nonce ist **kein Parameter** der öffentlichen API (I-23). `seal` hat die Signatur `fn seal(&self, ctx: &SealCtx, key: &KeyBuf, plain: &[u8]) -> Result<Envelope>`; `SealCtx` wird tatsächlich in die AAD gebunden.
- `open` liefert `Zeroizing<Vec<u8>>`. Jeder Fehler (Länge, Version, Tag, fehlender Slot) wird zu **einer** einheitlichen Fehlerklasse ohne Orakel-Details; ein gelöschter Slot ist davon unterscheidbar als `Erased` (S1).
- v2-Umschläge (`ctx/rec/v2`, AES-GCM-SIV) sind lesbar bis zum Abschluss der Migration, werden aber nie mehr geschrieben.

### 5.6 Brücke von P nach N: Handles und Commitments

- `SubjectHandle` (128 Bit) kommt aus dem CSPRNG und wird **nicht** aus einer Identität abgeleitet (sonst Wörterbuchangriff auf N). Die Zuordnung Identität zu Handle ist ein P-Record (Blind-Index `HMAC(tenant_pepper, normalisierte Kennung)`) und wird mit gelöscht.
- Je Entscheidung ein eigenes `DecisionHandle`; die Zuordnung Handle zu Betroffenem liegt in P unter dem Subject-Slot. So verbindet N nicht mehrere Entscheidungen derselben Person.
- `output_commitment = HMAC(salt_subject, canonical(output))`; `salt_subject` liegt in P. Vor der Vernichtung kann ein Auditor die Ausgabe prüfen, danach ist das Commitment weder auflösbar noch verknüpfbar. `acta-crypto::audit_chain::compute_record_commitment` ist die Grundlage.
- `SubjectCommitment = BLAKE3-keyed(K_commit[epoch], canonical(scope))`. Nach Vernichtung von `K_commit[e]` sind alte Commitments nicht mehr verknüpfbar.

### 5.7 Fehlerpfade und RAM

- Verboten im gesamten Kryptopfad: `unwrap_or_default`, `let _ =` auf `Result`, `unwrap_or(0)` auf Schlüsselbytes, `[0u8; N]` als Schlüsselersatz. Durchsetzung: Gate G-CRY-1 (grep-basierter Architekturtest über acta-crypto und acta-erasure) und Clippy `let_underscore_must_use`.
- `ErasureConfig::k_idx` hat **keinen Default**. Ein Start ohne explizit gesetzten Schlüssel wird verweigert.
- Der statische Legacy-WAL-Schlüssel (Feature `legacy-wal-key`) wird nach abgeschlossener Migration samt Feature entfernt. Bis dahin darf das Feature in keinem Release-Profil aktiv sein (Test `no_legacy_key_in_release`).
- `Clone` auf `SubKey`, `GroupKek`, `RecordDek` (kv\_shredding.rs) entfällt oder wird auf `Zeroizing`-Handles mit Referenzzähler umgestellt. Kernel-Seiten für Schlüsselpuffer: `mlock` und `MADV_DONTDUMP` über acta-sys.

---

## 6. Crate-Spezifikationen

Jedes Crate: **Rolle**, **Soll-Verträge**, **Invarianten**, **Änderungen gegenüber Ist** (mit Befund-ID), **Abnahme**. Kurzform; Details zur Löschkette stehen in §7, zur Zeitreise in §8.

### 6.1 acta-ids (R0)

- **Rolle:** Kennungen ohne Abhängigkeiten: `TenantId`, `CollectionId`, `DocId` (64 oder 128 Bit, Feature `docid-128`), `TxId`, `TOMBSTONE_BIT = 1 << 63`, `ID_WIDTH_BITS`, `MAX_SEARCH_K`.
- **Invarianten:** Layout-Prüfung zur Compile-Zeit (`size_of::<DocId>()`). Ein Tombstone-Bit in der Sequenznummer ist MVCC-Sichtbarkeit und **keine Löschung** im Sinne dieser Spezifikation.
- **Änderung:** Das 65-Byte-Datenpräfix (§9.1) arbeitet mit 128-Bit-Tags. Persistierte Formate führen `id_width_bits` (vorhanden in `CheckpointMeta`); Abweichung ergibt `IdWidthMismatch`.
- **Abnahme:** Layout-Tests, Proptest Roundtrip.

### 6.2 acta-env (R0)

- **Rolle:** `Clock` (Wanduhr und monoton), `Rng`, `IdGen`, `MetricsSink`, `ManualClock`, `SplitMix64` als deterministischer Test-Rng.
- **Invarianten:** Kern-Crates nutzen nur diese Ports (R6). Der Produktions-`Rng` ist `OsRng`; `SplitMix64` ist in Release-Builds nicht als Schlüsselquelle verdrahtbar (Typ-Trennung `KeyRng` versus `TestRng`).
- **Abnahme:** Gate G3 (kein `SystemTime`, kein `thread_rng` in Kernpfaden).

### 6.3 acta-sys (R0, unsafe-Insel)

- **Rolle:** mmap, ACL (Unix `0600` und UID-Prüfung, Win32-DACL), Durability-Primitive, RAII-Tresorpuffer (mlock).
- **Invarianten:** jeder `unsafe`-Block mit SAFETY-Kommentar (Lint `undocumented_unsafe_blocks = deny`); keine Geschäftslogik.
- **Änderung:** `mlock`/`MADV_DONTDUMP`/`RLIMIT_CORE=0` als Funktion `harden_process()`, deren Ergebnis im Bericht als `ProcessHardeningStatus` erscheint (Typ existiert im Report).
- **Abnahme:** Miri oder Sanitizer-Lauf über alle unsafe-Pfade (Annahme \[A\]: Tooling vorhanden).

### 6.4 acta-durable-fs (R0)

- **Rolle:** `write_atomic` (tmp, `sync_file`, rename, `sync_dir`), `AtomicHealthState` mit Poisoning (INV-FSYNC-1), Append-Journal mit Prüfsumme, Pending-Delete-Journal, `scrub_file_with_rate_limit`, `scrub_if_safe`, `FaultInjector`.
- **Änderung (B-09):** Wird zur **einzigen** Dateischicht für alles, was Löschung berührt: SST- und WAL-Entsorgung, Tresor-Rehash, Journal, Ledger, Keyring. acta-store und acta-erasure binden es über ihre `Vfs`-Ports ein. `remove_file` ohne vorheriges `scrub_if_safe` ist für Dateien der Klasse P und für Tresor-/Keyring-Dateien verboten (Gate G-FS-1).
- **Grenze (D-15):** `scrub_if_safe` prüft, ob Überschreiben sinnvoll ist (Dateisystem, SSD-Hinweis), und meldet das Ergebnis als `ScrubOutcome`. Es beweist keine physische Entfernung.
- **Abnahme:** Crash-Matrix der Dateioperationen mit `FaultInjector` (Absturz nach tmp, nach rename, nach sync\_dir).

### 6.5 acta-simd, acta-wire (R0)

- **acta-simd:** Distanzkerne (Skalar, AVX2, AVX-512, NEON) mit Laufzeit-Dispatch. Nicht Teil der Löschkette. Für Ranking-Reproduzierbarkeit (S6 für Receipts) MUSS der gewählte Kernel (`kernel_id`) in den Index-Metadaten und im `ContextReceipt` stehen; Ergebnisse verschiedener Kernel gelten nicht als gleich \[E\].
- **acta-wire:** FlatBuffers-Zero-Copy, `WireBuffer`, JSON-RPC-Parser ohne Panics, verifizierte Puffer. `unsafe` nur im generierten Teil. Kein Personenbezug im Wire-Format ohne Klassenfeld (siehe 6.17).

### 6.6 acta-types (R2)

- **Rolle:** Domänenmodell, `ActaError`, DTOs, `TenantScoped`, Budgets.
- **Änderung:** Neue reine Typen `Plane`, `DataClass` (Norm, Personal, Special, Derived, Evidence, Unclassified), `LegalBasis`, `Horizon`, `AsOf { valid_at, known_seq }`, `ReadOutcome<T>` (§8.2), `SubjectHandle`, `DecisionHandle`, `ClassTag`. Die als `deprecated` markierten Löschtypen (`erasure.rs`, `witness.rs`, Fehler) werden **entfernt**; Löschtypen leben nur in acta-erasure (D-08). `ActaError` bekommt Varianten `Erased`, `HorizonExceeded`, `ClassViolation`, `NotImplemented`.
- **Abnahme:** Serde-Roundtrip, Golden-Tests für `AsOf`.

### 6.7 acta-ports (R2)

- **Rolle:** Trait-Verträge für Indizes (`VectorIndex`, `TextIndex`, `GraphIndex`), `KvPort`, `Checkpoint`, `CheckpointCoordinator`, Scanner-Schnittstellen.
- **Änderung (B-05):** Das Modul `scanner` (`Purger`, `LayerScanner`, `ScanTarget`, `PurgeStats`) und `erasure_legacy` werden **entfernt**. Ihre Aufgabe übernehmen die Ports von acta-erasure; Implementierungen entstehen als Adapter (§6.19). Die Konformanzsuite (`contract-tests`) wird um `ErasureLayer`-Verträge erweitert (jede Schicht: `resolve`, `on_gate`, `purge`, `witness` idempotent, wiederholbar, crash-sicher).
- **Abnahme:** Jeder Port hat eine Produktionsimplementierung und eine Aufrufstelle (Dead-Port-Gate G2).

### 6.8 acta-mvcc (R1)

- **Rolle:** `SequenceLog`, `SnapshotRegistry` und `SnapshotGuard`, `TxBuffer`, SSI (`ReadSet`, `SequenceLogSsiValidator`), `SeqBitTombstone`.
- **Invarianten:** Sperrenordnung INV-LOCK-ORDER-1: Dokument-/Key-Lock, dann TxBuffer-Shards (nie zwei gleichzeitig), dann SnapshotRegistry, dann SeqLog/SSI. Erweiterung (§10.1): `ErasureFence` davor, Tresor-Lock nach den MVCC-Sperren, nie über I/O.
- **Änderung:** MVCC bleibt frei von Löschwissen. Zwei generische Hooks: (a) `revoke_covering(tenant, range, gated_seq)` bleibt die einzige Löschfähigkeit und wird als `SnapshotRevoker` im Adapter verwendet; (b) `min_for_compaction` liefert nur die Snapshot-Untergrenze. Die Retention-Entscheidung liegt nicht hier, sondern in der `RetentionPolicy` des Stores (§8.2). Ein widerrufener Snapshot liefert beim nächsten Zugriff `SnapshotRevoked`, nie Daten.
- **Abnahme:** vorhandene Loom- und Isolationstests; neuer Test: Snapshot vor dem Gate, Lesen nach dem Gate ⇒ `Erased`.

### 6.9 acta-crypto (R0)

- **Rolle:** Primitive, ohne Löschlogik: AEAD, KDF, `SecretKey`, WAL-HMAC (`WalHmac`, `EncryptedWal`, `verify_wal_chain_completeness`), `AuditChain` mit Commitments, `KmsContract` und `InMemoryKms` (Vorlage für `KeyAnchor`).
- **Änderung:** (1) `aead` auf Umschlag v3 (§5.5), Nonce nicht mehr Parameter; `seal_record` gibt `Result` zurück (B-10). (2) `derive_dek` propagiert Fehler und kodiert `n` Betroffene (§5.3). (3) `kv_segment` und `kv_cipher` (KV-/LLM-Kontext-Cache) werden hinter ein nicht-default Feature `kv-cache` verschoben; ein Cache MUSS sich als `DerivedParticipant`-Adapter registrieren oder wird beim Commit abgelehnt (BR-9). (4) `indexing_slicing = allow` im Crate entfällt. (5) Der Feature-Default `workspace` entfällt (B-16).
- **Abnahme:** Testvektoren (Anhang A), Property-Tests (Roundtrip, Manipulation je Byte), Wiederverwendung einer Nonce per Typ unmöglich; externe Kryptoprüfung vor Produktivbetrieb (Gate G4).

### 6.10 acta-store (R1)

- **Rolle:** LSM: WAL (HMAC-Kette, Poisoning, Gruppen-Commit), Memtable (Shards, MVCC), SSTable (Format V4), Manifest, Compaction (Leveled und adaptiv), Block-Cache, `TenantScopedStorage`, `io_seam`.
- **Invarianten:** Der Store kennt **kein** Konzept Erasure (ACTA\_SPEC §6.6). Er bietet generische Fähigkeiten (unten) und bleibt nach Klassen getrennt (D-14).
- **Neue generische Fähigkeiten:**
  1. **Partitionen N, P, D** je Mandant mit eigenem Manifest-Abschnitt (`AddSst{class, partition}`) und eigenem Picker; Compaction mischt nie Klassen (I-7).
  2. **`KeyV2`** (§9.1) und `TenantKeyCodec` v2; der ASCII-Codec wird nach einmaliger Migration entfernt (B-04).
  3. **`RetentionPolicy`** (§8.2) statt fester `retain_key_versions`-Regel; `HistoryFloor{partition, seq}` und `SeqHwm{seq, tx}` im Manifest, monoton steigend, **vor** dem Verwerfen der Versionen durabel.
  4. **`rewrite_filtered(partition, matcher, op_id)`:** streamender SST-Rewrite in O(Block), `tmp, fsync, rename, fsync(dir)`, Manifest-Batch `Add(neu)+Remove(alt)` atomar, danach `scrub_if_safe` und `unlink` der Alt-Datei. Fehler **nach** dem Manifest-Commit löschen nie die neue Datei (F2, B-19).
  5. **`compaction_barrier()`** (exklusiv gegen Flush und Compaction) auf Basis von `FlushPurgeFence`; `enter_purge` bekommt Aufrufer, der Vorrang wird mit **asymmetrischem** Protokoll umgesetzt (Purge setzt Flag und wartet mit Timeout auf `flush_active == false`; Flush gibt bei aktivem Purge `Err(Busy)` zurück statt `Ok(())`, B-07).
  6. **`retire_wal_segments_up_to(seq)`** mit Scrub und `unlink`; `rotate_and_seal`.
  7. **Block-Cache nur mit Chiffretext** für P; falls ein Klartext-Cache nötig ist, `invalidate_handle(tag)` und `zeroize`.
  8. **Value-Verschlüsselung vor dem WAL** über den `RecordSealer`-Port (B-03): für P ist `put` nur mit `SealedBytes` möglich (Typ-Zwang). N ist Klartext, D ist unter einem Ableitungsschlüssel gesiegelt.
  9. **`get_at(part, key, as_of)` → `ReadOutcome`** mit `HorizonExceeded` bei `as_of < floor(partition)`.
- **Änderung:** Phase-0-Arbeiten aus dem Vorbericht (B-19): WAL-Frame v2 (Länge im Header-CRC, Torn-Tail-Regel nur am EOF), Rollover und Flush serialisieren, Poison bei jedem Manifest- und WAL-Schreibfehler, Tmp-Cleanup und Waisen-GC, Anti-Rollback der WAL-Kette, Bloom-Größe aus Merge-Eingabe. Jede Änderung ist erst mit dem zugehörigen Fault-Injection-Test abgenommen (§11.3).
- **Abnahme:** vorhandene differenzielle Tests gegen redb bleiben grün; neue Crash-Matrix (§11.3); Loom-Modell Flush gegen Rollover gegen Purge.

### 6.11 acta-checkpoint (R1)

- **Rolle:** benannte Checkpoints (`CheckpointMeta` mit `seq_no`, `tx_id`, `metadata`), `PersistentCheckpointStore`, RAII-`CheckpointGuard` mit Rollback, `CheckpointHardlinkCloner` für Agent-Forks, Orphan-Registry.
- **Änderung (B-11):** (1) Jede Kopie (Hardlink oder `fs::copy`) wird in einer **`CopyRegistry`** verzeichnet (Pfad, Partition, `seq_no`, Erzeugungszeit). Hardlink-Klone gelten als Kopien, weil ein Rewrite der Originale sie nicht erreicht. (2) Checkpoints dürfen **nur Partition N und D (Metadaten)** hardlinken. P-Dateien eines Checkpoints werden **nicht** geklont; ein Fork der P-Daten ist nur als Chiffretext-Kopie zulässig, mit Registrierung und Löschbezug (`PurgeableCopy`). (3) Der Pin über `SnapshotRegistry` wird als echtes Feature `mvcc` mit Abhängigkeit definiert oder entfernt; kein toter `cfg`-Pfad. (4) `CheckpointMeta.metadata` (JSON) wird gegen den N-Validator geprüft, weil Freitext dort Personenbezug tragen kann.
- **Abnahme:** Test: P-Daten, Checkpoint, Erase, Restore des Checkpoints ⇒ `Erased` (S9). Test `time_travel_correctness` bleibt Regressionsbasis.

### 6.12 acta-vector (R1)

- **Rolle:** HNSW-Index mit SIMD-Distanzen, exakte Suche, experimentell DiskANN und RaBitQ. Löschfähigkeiten sind vorhanden: Nullung der Vektoren (f32 und u8) und der Arena-Slots, Kantenreparatur, Neuwahl des Einstiegspunkts (`HnswPurger`, Zeroize).
- **Endgültige Festlegung (Embeddings):** Ein Embedding ist personenbezogen (Inversionsangriffe, Membership Inference). Es liegt als **P-Record** unter dem DEK seines Quelldokuments und ist die Wahrheit. Der Index ist eine **Ableitung (D)**.
- **Persistente Segmente** (mmap, DiskANN) sind unveränderlich je (Mandant, Collection, Zeitfenster), höchstens 50.000 Vektoren, und unter einem **ArtKey** versiegelt, der als Tresor-Slot (Typ E) liegt. Jedes Segment trägt `source_tags` (sortierte Liste der Quell-`doc_tag`). Wird **eine** Quelle vernichtet, wird der ArtKey des Segments vernichtet; das Segment ist sofort unlesbar und wird aus den verbleibenden P-Embeddings neu gebaut. Ein Wrapped-Copy-Schema je Betroffenem ist verboten, weil das Segment sonst Information über den Gelöschten behielte.
- **RAM-Index:** Beim Gate (`on_gate`) werden die Vektoren der Zielmenge sofort genullt und per Bitset-Maske aus Kandidaten entfernt. Die Kantenreparatur folgt im Purge-Fenster (Schritt e). Slots werden erst nach dem Rebuild wiederverwendet (INV-HNSW-SLOT-1).
- **Feature `partial-index-rebuild`** bleibt aus, bis `test_partial_rebuild_recall_regression` 30 Tage stabil grün ist und ein ADR es freigibt (Vorgabe aus Cargo.toml). Bis dahin gilt der **vollständige Segment-Rebuild**; die Segmentgröße begrenzt die Kosten.
- **Änderung:** Die Typen `Purger`/`LayerScanner` aus acta-ports entfallen (B-05). `HnswPurger` und `HnswScanner` werden zu `VectorLayer` im Adapter (§6.18), die Kernlogik (Zeroize, Reparatur, Residue-Scan) bleibt in acta-vector als öffentliche, löschneutrale Funktionen (`zero_slots`, `repair_graph`, `scan_residue`).
- **Abnahme:** Test `hnsw_no_residue_after_erase` (Bytes-Grep im Arena-Dump und im Segment), Recall-Regression nach Rebuild, Konnektivitätstest (INV-VEC-CONNECTIVITY-1).

### 6.13 acta-text (R1)

- **Rolle:** Analyzer `a1` (deutsche Morphologie, Unicode-Segmentierung), invertierter Index mit Posting-Listen, BM25 und WAND, MVCC-Snapshots.
- **Festlegung:** Persistente Posting-Segmente sind D, unter ArtKey versiegelt, mit `source_tags`. Dokumentstatistiken (df, Längen) sind aus den Segmenten ableitbar und werden beim Rebuild neu berechnet. Der Index speichert keine Volltexte; Snippets entstehen aus P-Records über das `ReadGate`.
- **Änderung:** `TextPurger` und `TextScanner` werden Funktionen (`rewrite_segments_without`, `scan_terms`) und gehen im `TextLayer` des Adapters auf. Der Verweis auf einen nicht existierenden Satelliten `acta-erasure-text` wird entfernt.
- **Abnahme:** Needle-Scan nach eindeutigem Marker-Term im Segment und im RAM; Determinismus der Rangfolge bei gleicher Analyzer-Version.

### 6.14 acta-graph (R1)

- **Rolle:** CSR-Graph (BFS mit Score-Decay, Personalized PageRank, Communities, Cascade), Session-DAG für Agentenzweige.
- **Festlegung:** Knoten und Kanten tragen nur opake IDs und `source_tags`. Beschriftungen (Namen, Entitätstexte) liegen als P-Records. Die Session-DAG trennt Struktur (Handles, Zeitstempel, Modell-/Prompt-Version: N-fähig nach Validator) von Inhalt (Prompts und Antworten: P). Ein Zweig-Fork kopiert nur Struktur, nie Inhalt.
- **Änderung:** `GraphPurger` wird `GraphLayer`; `cascade.rs` (kaskadierende Entfernung) liefert die Reparaturlogik. Experimentelle Features (`experimental-graph`, `physio-percolation`) sind nicht Teil des Defaults (B-16).
- **Abnahme:** Nach Erase enthält weder CSR-Datei noch Delta-Puffer eine Kante zur Zielmenge; Zentralitätswerte werden neu berechnet.

### 6.15 acta-rank (R1)

- **Rolle:** RRF-Fusion von Vektor-, Text-, Graph- und Rerank-Signal, Kalibrierung (Platt, Isotonic), konforme Prädiktion, Drift, Erklärbarkeit (`ProvenanceRecord`).
- **Festlegung:** acta-rank ist zustandslos bezüglich Personendaten. Kalibrierdaten enthalten nur Scores und Labels, keine Texte oder Kennungen (Test `calibration_state_has_no_identifiers`). `ConfigFingerprint` und `kernel_id` fließen in den `ContextReceipt`. Seeds kommen aus dem `Rng`-Port.
- **Änderung:** `default` ohne experimentelle Features (`exp-dibud`, `exp-resonance`); sie bleiben opt-in. `ProvenanceRecord` bekommt das Feld `plane` und, für P-Treffer, nur Commitments statt Inhalten.
- **Abnahme:** Gleiche Eingabe, gleiche Konfiguration, gleicher Seed ⇒ identische Rangfolge.

### 6.16 acta-mcp (R7)

- **Rolle:** stdio-JSON-RPC-Protokollschicht (ADR-010, kein HTTP), begrenzte Eingabegrößen, Validierung.
- **Endgültige Festlegung der Tools:** `acta.search` und `acta.get` (jeweils mit **Pflichtfeld** `as_of`, kein stiller Default), `acta.chronicle` (Norm-Abfrage), `acta.erase_request` und `acta.verify`. Ein LLM darf **keine** Löschung ausführen: `acta.erase_request` legt nur ein Ticket mit Zustand `Requested` an, das ein Mensch mit Rolle `Controller` über die CLI freigibt (Vier-Augen bei Tenant- und Collection-Zielen). Schreibende Tools sind standardmäßig aus (`is_write_allowed_by_env`).
- **Änderung:** Kein hartcodierter Fallback-Schlüssel (Schlüssel kommen nur über den Anker). Antworten markieren gelöschte Treffer als `NotFound` für die Rolle `Reader` (Existenz gelöschter Daten ist selbst Information); nur `Auditor` und `Controller` sehen `Erased`.
- **Abnahme:** Protokoll-Fuzzing (vorhanden in acta-wire), Rollen-Matrix-Tests.

### 6.17 acta-erasure (R3, Löschkern)

Bleibt autark (Klasse A). Modulentscheidungen:

| Modul | Entscheidung |
| --- | --- |
| `saga/` (`machine`, `core`, `driver`, `idempotency`, `retention`, `seams`) | **behalten.** `ErasureCore` wird synchron (D-09). Cancel-Safety läuft über die Shell, nicht über `tokio::spawn` im Kern. `k_idx` ohne Default. |
| `gate/` (`ErasedSet`, `GateLog`, `GateGuard`, `batch`, `fence`) | **behalten.** Schlüsselraum auf KeyV2 (§9.1). `InMemoryFenceGate` wird Testdouble; der Produktions-Fence ist der Store-Fence über den Adapter. |
| `vault/` (Slot-Tabelle, Keyring, Rehash, Scrub, Open) | **behalten, erweitern:** Slot-Typ U oder Zuordnung (§5.3), Epochenrotation (§5.4), `VaultSealer` auf AEAD v3 (B-02). |
| `crypto_local.rs` | **löschen.** Primitive über Port `CryptoPrimitives`, implementiert von acta-crypto im Adapter (D-05). |
| `kv_shredding.rs`, `revocation_log.rs` | **umbauen:** Gruppen-KEKs und Widerrufslog werden Verbraucher der einen Hierarchie; der Widerrufslog wird zum signierten Teil des Ledgers. |
| `purge/` (Phasen 1 bis 9, Reihenfolge, Fenster, Reserve, Resume) | **behalten.** `run_window` ohne Kern bleibt `NotImplemented`. |
| `measure/` (Scanner-Typestate, Positivkontrolle, MK1 bis MK3, Residue) | **behalten, erweitern:** Needle-Scan über Heap und Dateien, Kanarien-Fabrik (§7.9). |
| `journal/`, `ledger/` | **vereinheitlichen:** ein `ErasureLedger` (Hash-Kette, außerhalb des Datenverzeichnisses), Journal bleibt `DurableJournal`. |
| `report/`, `ed25519_proof.rs`, `deletion_proof.rs` | **ein Format** (D-10): `ErasureReport` kanonisch. `DeletionProof` wird auf eine Sicht des Reports reduziert oder entfernt. |
| `restore/` | **behalten**, Anbindung an `CopyRegistry` und Backup-Manifest. |
| `restricted.rs`, `replace.rs`, `supersede.rs`, `scope/` | **behalten** (Art. 18, Ersetzen, Berichtigung, Scope-Dateien). |
| `test_mocks.rs` | nur `cfg(test)`. |

Default-Features: `signing`, `vault`. `async` ist opt-in (D-09).

### 6.18 Neue Crates

**acta-erasure-adapters (R5, Brücke).** Einziges Crate, das acta-erasure und Store, Indizes, Checkpoints, Crypto kennt (R3). Implementiert genau die Ports des Kerns:

| Kernport | Adapter | Wirkt auf |
| --- | --- | --- |
| `ErasureLayer` | `StoreLayer` (Memtable-Drain, SST-Rewrite, WAL-Retire, Manifest) | acta-store |
| `ErasureLayer` | `VectorLayer`, `TextLayer`, `GraphLayer` | acta-vector, acta-text, acta-graph |
| `ErasureLayer` | `CheckpointLayer` (CopyRegistry, Kopien purgen) | acta-checkpoint |
| `ErasureLayer` | `CacheLayer` (Block-Cache, Prompt-/KV-Caches mit `source_tags`) | acta-store, acta-crypto (kv-cache) |
| `CommitBarrier`, `PurgeBarrier`, `FenceGate`, `WalSink` | `StoreBarriers` auf `FlushPurgeFence` und Commit-Mutex | acta-store |
| `SnapshotRevoker` | `MvccRevoker` auf `SnapshotRegistry::revoke_covering` | acta-mvcc |
| `KeyStore`, `RecordSealer`, `FileKeyring` | `VaultKeyStore` mit AEAD v3 und Slot-Tabelle | acta-crypto |
| `Vfs` | `DurableVfs` auf acta-durable-fs | acta-durable-fs |
| `ResidueScanner` | `ForensicScanner` (Rohbyte-Scan ohne Formatparser) | alle Dateien, Heap |

**acta-chronicle (R4, Ebene N).** `Chronicle::append(entry) -> EntryId`, `Chronicle::as_of(AsOf, matter) -> ReadOutcome<Entry>`, `history(matter, valid_at)`, `checkpoint()`. Enthält den **N-Validator** (Schema-Whitelist, PII-Detektor, k-Schwelle), die Hash-Kette (`prev_hash`) und signierte Checkpoints alle 1.024 Einträge (Ed25519, optional RFC-3161-Zeitstempel über Port `Notary`). Baut auf `acta-crypto::audit_chain` auf und schreibt in die Partition N des Stores. Korrekturen überschreiben nie (`supersedes`).

**acta-engine (R5, Anwendungsfälle).** `Ingest` (Klasse deklarieren, Gate, Siegeln, Schreiben), `Query` (AsOf-Planer, `ReadGate`, Fusion, `ContextBundle`, `ContextReceipt`), `Erase` (ruft ausschließlich `ErasureCore::execute`), `Sweeper` (Ablauf von Horizont und `retain_until`). Enthält den **einzigen** `ReadGate`-Einhängepunkt für alle Lesepfade (D-06). Klassifikation von Rohdokumenten ist **nicht** Teil von v1: der Aufrufer erklärt die Klasse (Ingest-Gate fail-closed bei fehlender Deklaration). Ein optionaler `Classifier`-Port darf Vorschläge liefern, entscheidet aber nie allein.

**acta-node (R6, Composition Root).** Baut die einzige `Wiring`-Struktur; jeder Pflichtport ist ein nicht-optionales Feld, ein vergessener Adapter ist ein Kompilierfehler:

```
pub struct Wiring {
    anchor: Arc<dyn KeyAnchor>,          vault: Arc<FileVault>,
    store: Arc<PartitionedStore>,        barriers: Arc<StoreBarriers>,
    layers: NonEmptyVec<Arc<dyn ErasureLayer>>,   // Store, Vector, Text, Graph, Checkpoint, Cache
    revoker: Arc<dyn SnapshotRevoker>,   journal: Arc<DurableJournal>,
    ledger: Arc<ErasureLedger>,          signer: Arc<dyn ReportSigner>,
    scanner: Arc<dyn ResidueScanner>,    notary: Arc<dyn Notary>,
    notifier: Arc<dyn RecipientNotifier>, authz: Arc<dyn Authz>,
    clock: Arc<dyn Clock>, rng: Arc<dyn Rng>, ids: Arc<dyn IdGen>,
}
```

Pflichten beim Start, in dieser Reihenfolge: (1) Prozesshärtung (`harden_process`), (2) Anker öffnen, Tresor öffnen (Kopf, Metablock, Stichproben), (3) `RestoreGuard` prüfen (Zeitregression, Wiederauftauchen), (4) `resume_pending()` für offene Sagas **vor** dem ersten Lese- oder Schreibzugriff; ein offener Vorgang ohne lauffähige Schicht ist ein Startfehler, kein No-Op, (5) **Boot-Selbsttest** mit Kanarien-Löschung in einem flüchtigen Test-Mandanten (schreiben, ableiten, löschen, messen, Bericht verifizieren, Budget unter 2 s); schlägt er fehl, startet Acta nicht. (6) Erst danach Zulassung von Anfragen.

**acta-lab (Test).** `FaultVfs` (Absturz an benannten Punkten), `ForensicScanner`, `HeapScanner`, `CanaryFactory` (eindeutige Marker mit Positivkontrolle), `ReferenceModel` (naives In-Memory-Modell als Orakel für Property-Tests), `Differential` (Vergleich Store gegen Referenz). Nur dev-Abhängigkeit.

### 6.19 Budgets neuer Produktionscode

| Crate | Zeilen (Budget) |
| --- | --- |
| acta-erasure-adapters | 3.000 |
| acta-chronicle | 1.800 |
| acta-engine | 2.200 |
| acta-node | 900 |
| acta-lab | 2.500 |

Überschreitung braucht einen ADR. Der Löschkern (acta-erasure) schrumpft durch Wegfall von `crypto_local`, doppeltem Ledger und doppeltem Nachweisformat.

---

## 7. Die Löschkette (normativ)

### 7.1 Löschziele

Ein Löschauftrag trägt Ziel, Modus und `RetentionAttestation`. Der Kern kennt `KeyScope` (Tenant, Collection, Document) und `EraseTarget` (Schlüsselbereich, exakte Schlüssel). Neu ist `KeyScope::Subject(SubjectHandle)`; er wird in der Auflösung (`resolve`) auf die Dokumente des Betroffenen abgebildet (P-Record `SubjectDocs`, unter dem Subject-Slot).

| Ziel | Auslöser | Vernichteter Slot | Umfang |
| --- | --- | --- | --- |
| Subject | Art. 17, Widerruf der Einwilligung | S | alle Dokumente des Betroffenen, auch Mehrbetroffenen-Dokumente (S3) |
| Document | gezielte Löschung | D | ein Dokument, alle Versionen |
| Collection | Auflösung einer Collection | C | alle Dokumente der Collection |
| Tenant | Off-Boarding | T und U | alle Daten des Mandanten, auch Chronik-Verweise werden `Erased` |
| Expired | Sweeper, Speicherbegrenzung (Art. 5 Abs. 1 lit. e) | je nach Einheit | Einheiten mit abgelaufenem `retain_until` oder Horizont |
| Restrict | Art. 18 | keiner | Sperre statt Löschung (`Restricted`), Lesen nur mit Zweck und Audit-Eintrag |
| Supersede | Art. 16 Berichtigung mit Purge alter Versionen | keiner (gleicher DEK) | alte Versionen physisch weg, Klasse K nicht erreicht und so berichtet |
| Replace | Ersetzen eines Dokuments | D (alt) | neu zuerst dauerhaft, dann alt (I-25) |

**Mehrbetroffene.** Standard ist `SplitStrict`: Einheiten werden beim Ingest je Betroffenem getrennt. Nicht trennbare Einheiten tragen mehrere S-Salts und werden beim Löschen **eines** Betroffenen für alle unlesbar (S3). In v1 gilt dafür `DeleteAll` mit ausdrücklicher Liste der Mitbetroffenen im Bericht (`collateral`) und Pflicht-Attestierung des Verantwortlichen. `RedactAndRekey` (geschwärzte Neufassung vor der Vernichtung, Protokoll wie `replace_erasing`) wird nur aktiv, wenn ein `Redactor`-Port verdrahtet ist.

### 7.2 Verantwortungsgrenze (Custody Boundary)

| Zone | Ort | Mechanismus | Nachweisklasse |
| --- | --- | --- | --- |
| Z1 Kern | WAL, Memtable, SSTables, Manifest, Tresor, Journal, Ledger | Schlüsselvernichtung (Schnellpfad), gefilterter Rewrite (Hygiene) | gemessen (K, P) |
| Z2 Ableitungen | Vektor-, Text-, Graph-Segmente, RAM-Strukturen, Caches | ArtKey-Vernichtung, Rebuild, Nullung | gemessen (D) |
| Z3 Acta-eigene Kopien | Checkpoints, Forks, Exporte, Datensicherungen | `CopyRegistry`, Chiffretext-Kopien werden mit dem Schlüssel unlesbar, Klartext-Kopien gibt es nicht | gemessen (C) für Registrierte |
| Z4 Weitergegebene Kopien | Empfänger, externe Modellanbieter | `ReleaseLedger` und Benachrichtigung (Art. 19) | nur benachrichtigt, nie verifiziert |
| Z5 Quelldateien | Dateiserver, Postfach | Modus `Reference` (Standard: Acta hält nur Hash und URI) oder `Ingest` (verschlüsselte Kopie in Z1); `SourceProbe` meldet Reste | nicht im Besitz, Befund `SourceStillPresent` als Aufgabe |
| Z6 Umgebung | RAM, Swap, Core-Dumps, SSD-Wear-Leveling, Page-Cache | Verschlüsselung dominiert; `mlock`, `zeroize`, Core-Dump-Verbot | Annahmen `A-FLASH`, `A-RAM` im Bericht |

### 7.3 Zustandsautomat und Schritte

Der Kern hat zwei ineinandergreifende Automaten, beide mit reiner Übergangsfunktion: `SagaState` (außen sichtbar) und `PurgePhase` 1 bis 9 (Purge-Fenster). Es kommt **kein** dritter hinzu. Die Schritte E0 bis E9 sind das normative Protokoll; die Tabelle ordnet sie dem Code zu.

| Schritt | Aktion | SagaState danach | Durabilität |
| --- | --- | --- | --- |
| E0 | Validierung, `RetentionAttestation` (Matrix D-124), `resolve` je Schicht; bei Pflicht zur Aufbewahrung `Refused` oder `Restricted` | `Requested` oder `Refused` | keine |
| E1 | Scope-Datei verschlüsselt schreiben (`scope-<id>.scope`, fsync Datei und Verzeichnis), **danach** Journal `Requested+Intent`; Single-Flight über `derive_open_key`; Nadelmenge (§7.8) erzeugen | `Requested` | fsync #1 |
| E2 | Gate: unter `CommitBarrier::exclusive` zuerst `gate_epoch += 1`, dann GateLog-Eintrag mit `gated_seq`, dann `ErasedSet`; Snapshots widerrufen (`revoke_covering`); `on_gate` je Schicht (RAM-Nullung, Cache-Invalidierung, DEK-Cache leeren) | `Requested` (Gate gesetzt) | fsync #2 (Gruppen-Commit) |
| E3 | **Schlüsselvernichtung (kryptografischer Commit):** höchsten Slot (S, D, C oder T/U) auf Grabstein-Image nullen, fsync, am Medium zurücklesen und verifizieren (`VerifiedVia::Direct` oder `CacheDropped`); Journal `KeyDestroyed` | `Gated`, dann `KeyDestroyed` | fsync #3 und #4 |
| E4 | Receipt: Ledger-Eintrag (Hash-Kette) und Chronik-Receipt in N (`erasure_id`, Commitment des Ziels, `erase_seq`, Zeit, Mengen; keine PII) | `KeyDestroyed` | Ledger-fsync |
| E5 | Purge-Fenster (§7.5): Plattenreserve prüfen, `PurgeBarrier` exklusiv gegen Compaction, Schichten in fester Reihenfolge | `Purging` | je Phase im Journal |
| E6 | Ableitungen: ArtKey vernichten, Segmente neu bauen, Graph und Caches bereinigen | `Purging` | je Schicht idempotent |
| E7 | Messung (§7.8): MK1 bis MK3 mit Positivkontrolle, `LayerWitness` je Schicht | `Verifying` | Ergebnis im Journal |
| E8 | `ErasureReport` bauen, kanonisieren, signieren; Ledger; Journal `Completed`; Scope-Datei scrubben und löschen, **danach** Intent entfernen | `Completed` | fsync |
| E9 | Benachrichtigung der Empfänger (Art. 19), `SourceProbe`, Gate-Kompaktion nach Ablauf der Sperrfrist (§7.9) | `Completed` | Ledger |

Regeln:

- **E2 vor E3, immer.** Die Lesesperre steht dauerhaft, bevor der Schlüssel verschwindet. Dadurch gibt es kein Fenster, in dem Reader teilentschlüsselte oder falsch entschlüsselte Daten sehen.
- **Monotonie (I-3):** Zustände schreiten nur vorwärts. Ein persistiertes Gate wird nie aufgehoben (INV-NO-RESURRECTION-1), auch nicht bei Fehlern. Ein Fehler führt zu `Pending(Grund)`, nie zu `Failed`; die Löschung wird wiederholt, bis sie gelingt oder ein Mensch eingreift (`Escalated`).
- **`Pending` geht nie direkt nach `Completed`** (FW-9). `Completed` setzt `matches == 0` in jeder Schicht und bestandene Positivkontrolle voraus.
- **Rückgabe an den Aufrufer:** `erase()` kehrt bei `Gated` (nach E3) zurück, spätestens nach `erase_return_timeout` (Default 500 ms) mit `Pending` und aktivem Gate. Die Zeit bis `Gated` ist O(1) in der Zielgröße (E3 ist ein Slot, nicht ein Datenvolumen).
- **Idempotenz:** Jeder Schritt ist wiederholbar. Wiederholte Anfragen mit gleichem Ziel und Modus liefern dieselbe `erasure_id` (Single-Flight).

### 7.4 Durabilitätspunkte und Crash-Matrix

Jeder Punkt ist ein Test (§11.3). Nach dem Neustart führt `resume_pending()` vor jedem Zugriff die Wiederaufnahme aus.

| Nr. | Absturz | Zustand danach | Wiederaufnahme |
| --- | --- | --- | --- |
| C1 | nach Scope-Datei, vor Intent | Scope-Datei ohne Intent | Waise, wird von `scope_purge` entsorgt; Löschung gilt als nie beantragt |
| C2 | nach Intent, vor Gate | `Requested` ohne Gate | Schritt E2 wird ausgeführt |
| C3 | Gate geschrieben, nicht gefsynct | Gate nicht vorhanden | wie C2; kein Reader hat Gate-Wirkung nach außen zugesichert bekommen |
| C4 | nach Gate-fsync, vor Schlüsselvernichtung | Gate dauerhaft, Slot lebt | E3 wird ausgeführt; Reads liefern bereits `Erased` |
| C5 | Slot genullt, nicht gefsynct oder nicht verifiziert | unbestimmt | Recovery liest den Slot am Medium (`Direct`); Grabstein fehlt ⇒ E3 wiederholen; Gate bleibt |
| C6 | nach Schlüsselvernichtung, vor Journal `KeyDestroyed` | Slot Grabstein, Journal alt | Recovery erkennt Grabstein ohne Journal und ergänzt `KeyDestroyed` |
| C7 | im Purge-Fenster (SST neu, Manifest noch alt) | Tmp-Datei vorhanden | Tmp wird als Waise gelöscht, Phase wiederholt |
| C8 | nach Manifest-Batch, vor Scrub/Unlink der Alt-Datei | beide Dateien auf Platte, Manifest zeigt neu | Journal-Liste obsoleter Dateien wird gegen das Manifest abgeglichen, nur nicht lebende werden entsorgt (Regel J1); die neue Datei wird **nie** gelöscht |
| C9 | Messung findet Rest | `Verifying` mit `matches > 0` | `Pending(WitnessMissing)`, Purge der betroffenen Schicht wird wiederholt |
| C10 | vor Bericht oder Ledger | `Verifying` bestanden, kein Bericht | Bericht wird aus Zeugen im Journal neu gebaut, Ledger idempotent über `erasure_id` |
| C11 | Epochenrotation: nach neuer Generation, vor Vernichtung von `E_e` | zwei Epochen gültig | Rotation wird fortgesetzt; `E_e` wird vernichtet |

### 7.5 Purge-Fenster (E5, Reihenfolge a bis g)

Die Reihenfolge ist fest (ER-20) und im Code als `PurgePriority` kodiert. Exklusiv gegen Compaction (`PurgeBarrier`), Plattenreserve vorab (`min_free_bytes`, Default 1 GiB; sonst `Pending(DiskReserve)` bei aktivem Gate).

| Phase | Schicht | Aktion | Idempotenz und Messung |
| --- | --- | --- | --- |
| a | Snapshots | `SnapshotRegistry::revoke_covering(tenant, range, gated_seq)`; Pins aus Checkpoints werden gelöst | widerrufene Snapshots liefern `SnapshotRevoked` |
| b | Text | Segmente ohne die Zielmenge neu schreiben (`rewrite_segments_without`) | Needle-Scan der Terme |
| c | Memtable und WAL | Memtable-Drain und Flush der Partition P; WAL-Segmente bis `gated_seq` entsorgen (`retire_segments_up_to`, Scrub, Unlink); gemischte Alt-Segmente (vor Migration, Klartext) per `execute_scope_purge` mit Neuverkettung der HMAC-Kette | P im WAL ist Chiffretext, daher kein Umbau; MK2 über alle WAL-Dateien |
| d | SSTables | `rewrite_filtered(P, matcher)`: nur SSTs, deren `tag_set` die Zielmenge enthält; Block-Cache invalidieren; Alt-Datei scrubben und löschen; F-Slots nullen | MK2 über alle SSTs; Manifest-Abgleich |
| e | Vektor | Slots nullen (schon bei E2), Kanten reparieren, Segmente mit vernichtetem ArtKey neu bauen | Residue-Scan Arena und Segment |
| f | Tresor | D-Slots der Zielmenge entfernen, Waisen-F-Slots prüfen, bei Last über 0,6 Rehash mit Überschreiben der Altdatei | MK3 liest Slots direkt |
| g | Caches | Block-Cache, Prompt-/KV-Caches mit `source_tags`, Embedding-Cache evictieren und nullen | Heap-Scan |
| h | Kopien | `CopyRegistry`: Hardlink-Klone und Kopien der Partition P entfernen oder als reine Chiffretext-Kopie dokumentieren | Registry leer für das Ziel |

Compaction filtert unabhängig davon: jede Compaction der Partition P fragt `ReadGate` (über `RetentionPolicy::may_drop_version`) und verwirft Versionen gelöschter Ziele. Chiffretext verschwindet damit spätestens bei normaler Compaction, auch wenn ein Purge-Lauf ausfiele.

### 7.6 ReadGate (ein Gate, alle Lesepfade)

- `ReadGate::check(key) -> Open, Erased, Restricted` ist O(log n) im `ErasedSet`, ohne I/O. `GateGuard::linearized_read` prüft vor und nach der Leseoperation `gate_epoch`; hat sich die Epoche geändert, wird neu geprüft (I-21). Dadurch liefert kein Leser Daten eines Ziels, dessen Gate währenddessen aktiv wurde.
- **Strukturelle Durchsetzung:** `LsmStorage` ist `pub(crate)`. Der einzige öffentliche Zugang ist der gegatete Leser in `acta-engine`. Ein Architekturtest listet alle öffentlichen Lesefunktionen des Stores (`get`, `get_at_seq`, `scan_prefix`, `*_tracked`, `collect_visible_entries`, Checkpoint-Restore, Export) und schlägt fehl, wenn eine ohne Gate erreichbar ist.
- Alle Ableitungspfade prüfen das Gate vor der Ausgabe: Vektorkandidaten (Bitset-Maske), Posting-Treffer, Graph-Traversal, Rerank-Eingaben, Prompt-Bau, MCP-Antworten.
- **SSI:** Eine Transaktion, die ein Ziel gelesen hat, das vor ihrem Commit geshreddet wird, bricht mit `Erased` ab. Schreibende Transaktionen auf ein Ziel mit aktivem Gate werden abgelehnt (`FenceGate`).
- Nach außen wird `Erased` für die Rolle `Reader` als `NotFound` gezeigt; `Auditor` und `Controller` sehen den Unterschied.

### 7.7 Schreibpfad und Fence

Schreiber betreten `FenceGate::enter_write(range)` (Dekker-Muster: zuerst `inflight` erhöhen, RAII-Ticket, dann `active` und Tabelle prüfen). Ein aktiver Fence über dem Bereich lehnt ab. Im Store ersetzt das asymmetrische `FlushPurgeFence`-Protokoll (§6.10) die bisherige symmetrische Form. Ein Loom-Modell prüft: kein Schreiber committet in einen gefencten Bereich; Flush und Purge können nicht gleichzeitig laufen; kein Livelock.

### 7.8 Messung und Nachweis

**Messklassen.** MK1 logische Strukturen (Indizes, Manifest, Tabellen), MK2 Rohbyte-Scan ohne Formatparser über Dateien und Heap, MK3 physische Schlüsselprüfung (Slot am Medium, `Direct` oder nach `fadvise`-Drop).

**Typestate.** `Scanner<Unarmed>` führt zuerst die Positivkontrolle aus (Muster in Scratch-Puffer, echter Durchlauf, genaue Trefferzahl, mutiertes Muster wird nicht gefunden). Nur dann entsteht `ControlEvidence` und `Scanner<Armed>`. Ein `LayerWitness` entsteht ausschließlich bei `matches == 0` (`finish_scan`). Konstruktor und `arm` sind nur in `measure/` aufrufbar.

**Nadelmenge (Festlegung).** Nach der Schlüsselvernichtung ist der Klartext nicht mehr bekannt; deshalb entsteht die Nadelmenge in E1, solange er noch lesbar ist:

1. `doc_tag`, Umschlag-Header und Nonces der Zieldokumente (Chiffretext-Reste),
2. je Dokument drei Klartextfenster zu 24 Byte, deterministisch über BLAKE3-keyed gewählt, sowie Terme und Entitäts-IDs für die Ableitungsschichten,
3. Slot-Images der vernichteten Slots (für MK3 als Negativprüfung).

Die Nadeln liegen nur in `mlock`-Speicher und in der verschlüsselten Scope-Datei. Sie werden mit `Completed` gescrubbt und überleben nie die nächste Epochenrotation. Der Bericht enthält nur Anzahl und Hash der Nadelmenge.

**Klassen des Nachweises (je Löschung ausgewiesen als `reached` oder `not_reached`):**

| Klasse | Aussage | Beleg |
| --- | --- | --- |
| G | Lesepfade sind gesperrt | Gate-Probe: je Lesepfad ein Lesezugriff liefert `Erased` |
| K | Schlüsselmaterial vernichtet | MK3, Slot am Medium als Grabstein |
| P | physisch entfernt aus Z1 | MK2 über WAL, SST, Manifest, Journal, Heap mit Positivkontrolle |
| D | Ableitungen sauber | `LayerWitness` je Index und Cache |
| C | Kopien bereinigt | `CopyRegistry` ohne Eintrag oder nur Chiffretext |

Der Bericht nennt zusätzlich `key_coverage` je Schicht, `ProcessHardeningStatus`, die Annahmen `A-FLASH` und `A-RAM`, die Restrisiken (z. B. `SupersededWithSameKey` bei Berichtigung) und für Z4/Z5 nur Benachrichtigungsstand. Format: kanonisches NFC-JSON, Ed25519, Domain-Präfix `ctx/report/v1` (vorhanden).

**Attestierungsklassen (D-12).** A0: Bericht vom Acta-Prozess selbst signiert (Entwicklung, keine Beweiswirkung). A1: Signatur über einen Schlüssel in einem Operator-HSM außerhalb des Acta-Prozesses. A2: A1 plus qualifizierter Zeitstempel (RFC 3161 über `Notary`) plus Prüfung durch das unabhängige Werkzeug `acta-verify`. Produktivbetrieb verlangt mindestens A1.

**Unabhängige Prüfung.** `acta-verify` ist ein eigenes, kleines Werkzeug (Budget 800 Zeilen, keine Abhängigkeit zu acta-erasure außer dem öffentlichen Format): es liest die Tresor-Dateien nach dem öffentlichen Layout (Kopf 64 Byte, Metablock 64 Byte, Slots zu 64 Byte), prüft Grabsteine und Slot-MACs, verifiziert Berichtssignatur und Ledger-Kette und führt den **Kanarien-Test** auf der echten Konfiguration aus (eigenes Marker-Subject anlegen, löschen, Datenverzeichnis nach Marker und Marker-Chiffretext durchsuchen, `as_of`-Lesen versuchen). Erwartung: null Treffer außerhalb unlesbarer Epochen.

### 7.9 Restore, Backups, Anti-Wiederauferstehung (S9)

- **Zwei Spuren.** Datensicherungen enthalten nur Chiffretext und dürfen lange leben (Empfehlung des Betreibers: höchstens 90 Tage, Art. 5 Abs. 1 lit. e). Schlüsselmaterial (Tresor, Keyring, Anker-Sicherung) darf **nicht** länger als `T_ROT_BACKUP` = 72 Stunden gesichert werden; `T_ROT` = 24 Stunden. Der ungünstigste Zeitraum von der Vernichtung bis zur Unlesbarkeit in jeder Sicherung ist damit `T_ROT + T_ROT_BACKUP` = 96 Stunden.
- **Folge für die Notfallwiederherstellung (final):** Ein Tresor-Backup älter als 72 Stunden ist per Konstruktion unbrauchbar, weil `E_e` vernichtet ist. Wiederherstellung erfolgt aus einer **Live-Replik** des Tresors, die jede Vernichtung synchron erhält (Commit nach Ack aller konfigurierten Replikate), nicht aus einem alten Backup. Ohne Replik bedeutet Tresorverlust Verlust der Lesbarkeit aller P-Daten; das ist dokumentiertes Betreiberrisiko (§14).
- **RestoreGuard** beim Öffnen: (1) Zeitregression (`store.current_seq < max(gated_seq)` im Ledger), (2) Wiederauftauchen (Datei im Zielbereich mit `min_seq <= gated_seq`). Treffer: Store nicht freigegeben, Zustand `Recovering`, `replay_erasures` setzt Gates und Slot-Vernichtungen aus dem Ledger erneut, danach Purge und Messung.
- **Gate-Kompaktion** (E9) erst nach `Completed` **und** nach Ablauf der längsten Datensicherungs-Aufbewahrung des Betreibers. Das Ledger behält Einträge dauerhaft (nur gekeyte Hashes, keine PII).
- Der Anker erhält eine **Löschhistorie** (`erased_set_mirror`): Spiegel der Ledger-Wurzel auf einem zweiten Medium, damit auch das Ersetzen des gesamten Datenverzeichnisses erkannt wird (Anti-Rollback).

### 7.10 Zeitgesteuerte und besondere Löschungen

- **Sweeper:** prüft `retain_until` und Horizont, löst `Expired` aus; Läufe sind selbst Journal-Einträge. Fristen werden aus `LegalBasis` abgeleitet (Vertrag: Horizont 30 Tage; gesetzliche Pflicht: bis `retain_until`; Special: Horizont 0).
- **Restrict (Art. 18):** Zustände `Restricted` und `Lifted`; gesperrt sind Lesen, Suchen und Schreiben. Ein Lesepfad für Rechtsansprüche schreibt **vor** der Auslieferung den Journal-Eintrag `RestrictedRead` (Zweck, Zeit, Anzahl, `case_commitment`), höchstens 1.000 Dokumente je Aufruf. Ein `Erase` auf `Restricted` ist zulässig und hebt die Sperre durch Löschung auf.
- **Supersede:** Matcher `PrefixSeq{prefix[65], seq_lt}`; alte Versionen werden gegatet und gepurgt, nachdem die neue Version committet ist; fehlt sie nach einem Absturz, bleibt alles unberührt (`Pending(UpdateMissing)`).
- **Art. 19:** Jede Weitergabe von P-Inhalten (Export, externer Modellanbieter) wird im `ReleaseLedger` verzeichnet; bei Löschung benachrichtigt `RecipientNotifier` die Empfänger. Ergebnis `Notified` oder `Acknowledged`, nie `Verified`.

### 7.11 Grenzen (immer im Bericht)

`A-FLASH`: Wear-Leveling, Dateisystem-Journal und Snapshots können Altblöcke behalten; ohne Schlüssel sind sie wertlos (D-02, D-15). `A-RAM`: RAM, Swap und Core-Dumps liegen außerhalb der Messung; Schlüssel in `mlock`-Speicher, Core-Dumps aus. `A-ANCHOR`: Der Anker (HSM, KMS) vernichtet Schlüssel wie zugesichert. `A-CRYPTO`: AEAD und KDF bleiben im Prüfzeitraum sicher (Gate G4, periodische Überprüfung). Z4 und Z5 liegen nicht im Besitz von Acta.

---

## 8. Zeitreise und Löschung (normativ)

### 8.1 Drei Zeitachsen

| Achse | Gilt für | Bedeutung |
| --- | --- | --- |
| Sequenz (`Seq`) | Store, alle Partitionen | Commit-Reihenfolge je Mandant, monoton, **nie wiederverwendet** (`SeqHwm`). Grundlage von MVCC-Snapshots und `as_of`. |
| Bitemporal (`valid`, `recorded`) | Chronik (N) | `valid` = Geltungszeit in der Welt (halboffenes Datumsintervall), `recorded` = Wissenszeit (Seq und Zeitpunkt). Korrekturen überschreiben nie, sie legen einen Eintrag mit `supersedes` an. |
| Checkpoint | Mandant | benannter Fixpunkt (`seq_no`, `tx_id`, Metadaten). Zeitreise-Anker für Agenten und Audits. |

Die vier Fragen eines Unternehmens: Was gilt jetzt? (`valid_at = heute`, `known_seq = jetzt`.) Was galt am 01.03.2022? Was wussten wir am 15.04.2022 über den 01.03.2022? (`valid_at = 2022-03-01`, `known_seq` = Seq zum 15.04.2022 über `SeqTimeMap`.) Wie hat sich unser Wissen über den 01.03.2022 verändert? (`history(matter, valid_at)`.)

### 8.2 Retention und Ergebnisarten

```
pub enum Retention { KeepAll, LiveWhileKeyAlive, Window { min_seq_floor: u64 } }

pub trait RetentionPolicy: Send + Sync {
    fn floor(&self, part: PartitionId) -> u64;                       // persistiert, monoton
    fn may_drop_version(&self, key: &KeyRef, seq: u64, newer_exists: bool) -> bool;
}

pub enum ReadOutcome<T> {
    Found(T), NotFound,
    Erased { erase_seq: u64, erasure_id: ErasureId },
    Restricted, HorizonExceeded { floor: u64 }, SnapshotRevoked,
}
```

| Partition | Policy | Regel für `may_drop_version` |
| --- | --- | --- |
| N | `KeepAll` | immer `false`. Auch die bisherige Regel von `retain_key_versions` (nur die neueste Version unterhalb des Floors behalten) gilt hier **nicht**. |
| P | `LiveWhileKeyAlive` | `true`, wenn das Ziel gelöscht ist (`ReadGate` meldet `Erased`), auch oberhalb des Snapshot-Floors. Sonst `true` nur, wenn `seq` unterhalb des Horizonts der Einheit liegt, eine neuere Version existiert und kein aktiver Snapshot sie braucht. |
| D | `Window` | nach `min_seq_floor`; Ableitungen werden ohnehin aus Quelle und Chronik neu berechnet. |

- Der Horizont je Einheit steht als `horizon_class` im Schlüssel (0 = keiner, 1 = 30 Tage, 2 = bis `retain_until` aus der `HorizonTable` im Manifest). Der `HistoryFloor` je Partition wird aus `SeqTimeMap` (dünn besetzte Zuordnung Seq zu Zeit, alle 1.000 Seq oder je Minute) berechnet.
- **Commit-Reihenfolge:** erst `HistoryFloor` und `SeqHwm` durabel im Manifest, dann Compaction-Install, dann Verwerfen. So kann ein Absturz keine Version verwerfen, ohne dass der Floor sie schon ausschließt.
- `get_at(part, key, as_of)`: `as_of < floor(part)` ⇒ `HorizonExceeded` (nie eine stille falsche Antwort). Der Snapshot-Floor aus `min_active_seqno` bleibt zusätzliche Untergrenze, ist aber nicht mehr alleinige Wahrheit (B-08).

### 8.3 AsOf-Semantik

| Plane und Zustand | `valid_at` | `known_seq` | Ergebnis |
| --- | --- | --- | --- |
| Chronik | beliebig | beliebig | `Found` oder `NotFound` |
| Tresor, lebend, im Horizont | nicht anwendbar | Seq im Horizont | `Found` |
| Tresor, lebend, außerhalb des Horizonts | nicht anwendbar | zu alt | `HorizonExceeded` |
| Tresor, gegatet oder gelöscht | nicht anwendbar | **egal** | `Erased` (**Erase gewinnt** vor `HorizonExceeded` und vor jedem Snapshot) |
| Tresor, `Restricted` | nicht anwendbar | egal | `Restricted` nur für Rolle `LegalHold`, sonst `NotFound` |
| Ableitung | beliebig | beliebig | wird über die Quelle zeitgefiltert oder neu berechnet, nie eigenständige Wahrheit |

`AsOf` ist **Pflichtparameter** in jeder Abfrage-API; es gibt keinen stillen Default `jetzt` im Server.

### 8.4 Entscheidungsnachvollziehbarkeit ohne Personendaten

**Entscheidungsprotokoll (N):**

```
DecisionRecord { decision_id, tenant, ts, model_id, model_fingerprint, prompt_template_ver,
                 policy_set_ver[], params, deadline_refs[], kernel_id, config_fingerprint,
                 input_refs[ (decision_handle, p_record_ref) ], output_commitment }
```

- Es enthält Normstände, Parameter, Fristen, Modell- und Promptversion, Rechenkern, Konfigurationsfingerabdruck und **nur** Verweise auf P über zufällige Handles. Der Klartext der Ein- und Ausgabe steht in P.
- Nach der Löschung bleibt die Entscheidung nachvollziehbar (welche Regeln, welche Fristen, welches Modell), der personenbezogene Teil liefert `Ref::Erased`.
- **ContextReceipt (Evidence, N):** `AsOf`, Abfrage-Hash, Menge der verwendeten Chronik-Einträge (ID und Hash), für P-Treffer nur Commitments, Rankingparameter, Embedder- und Ranker-Version. Gleiche Anfrage mit gleichem `AsOf` liefert für N identische Ergebnisse (S6). Für P-Treffer entstehen nach der Löschung bewusst `Erased`-Lücken.
- Der Prompt beginnt mit einem **Stand-Header** (Rechtsstand und Wissensstand). Antworten MÜSSEN Zitate über `entry_id` und `content_hash` liefern können.

**N-Validator (Ingest-Gate nach N, fail-closed):**

1. Schema-Whitelist: nur deklarierte Felder, Typen Aufzählung, Zahl (gebucketet), Datum, Hash, Commitment, Handle. **Kein Freitext.**
2. PII-Detektor (Regeln und NER über Port) auf allen übrigen Textfeldern, mit Zeitlimit. Treffer, Fehler oder Zeitüberschreitung ⇒ Ablehnung oder Umleitung nach P.
3. k-Schwelle `K_MIN` = 5 für Aggregate: Aggregationen unter der Schwelle werden nicht in N materialisiert.
4. Entscheidungsbezogene Handles `H_d` (zufällig, je Entscheidung); N verbindet nicht mehrere Entscheidungen derselben Person.
5. Abnahmetests gegen die drei Kriterien (EDPB-Entwurf 02/2026, Artikel-29-Stellungnahme 05/2014): **Record Isolation:** kein Feldwertkombination in N ist für einen Handle eindeutig bestimmbar außer dem Handle selbst; **Linkability:** kein Wert kommt in zwei Entscheidungen derselben Person vor, kein 16-Byte-Wert in N entspricht einem Blind-Index oder einem HMAC einer Kennung (Scan über alle N-Einträge); **Inference:** keine Spalte erlaubt Rückschluss auf Attribute einer Person (Whitelist, Bucketing, k-Schwelle). Der Test läuft in CI auf einem synthetischen Korpus mit Personen als Gegenprobe.

### 8.5 Takedown in der Chronik (Ausnahme)

Enthält ein Norm-Eintrag fälschlich Personenbezug, ist ein **Takedown** zulässig (Vier-Augen, Ledger-Eintrag). Der Payload eines Chronik-Eintrags liegt verschlüsselt unter einem eigenen NormKey (Tresor-Slot D des Eintrags); `content_hash` und Metadaten stehen in der Hash-Kette. Takedown vernichtet den NormKey. Die Kette bleibt prüfbar (der Eintragshash deckt `content_hash`, nicht den Payload), der Inhalt ist unlesbar. Das ist die einzige Ausnahme von Invariante I-5.

### 8.6 Checkpoints, Forks und Zeitreise

- Ein Checkpoint pinnt `seq_no` (über `SnapshotRegistry`) und verzeichnet seine Dateien in der `CopyRegistry`. P-Daten werden nicht geklont (§6.11).
- Ein Restore ist ein **neuer Zweig**; ein Zweig sieht gelöschte Ziele weiterhin als `Erased` (S9).
- Ein Agent-Fork kopiert Struktur und N, nie P-Inhalte. Die Session-DAG trennt Struktur von Inhalt (§6.14).

### 8.7 Szenarien (Abnahmetests T-TT)

| Szenario | Erwartung |
| --- | --- |
| Lesen einer P-Version bei `as_of < erase_seq` nach der Löschung | `Erased`, nie `Found` |
| Chronik-Entscheidung verweist auf gelöschten P-Input | Entscheidung vollständig lesbar, Input `Ref::Erased` |
| Zeitreise hinter den Horizont, Ziel lebt | `HorizonExceeded` |
| Zeitreise hinter den Horizont, Ziel gelöscht | `Erased` (Vorrang) |
| Snapshot vor der Löschung geöffnet, Lesen danach | `SnapshotRevoked` oder `Erased`, nie Daten |
| Löschung, danach Wiederherstellung eines Checkpoints | `Erased` |
| Löschung eines Betroffenen | alle N-Einträge byte-identisch (I-5) |
| Berichtigung (Supersede) | alte Version physisch weg, Bericht nennt `SupersededWithSameKey`, Klasse K nicht erreicht |

---

## 9. Datenformate (normativ)

Alle Formate tragen Magic, Version und, wo IDs vorkommen, `id_width_bits`. Öffnen mit unbekannter Version ist ein Fehler ohne Teilmutation (fail-closed). Alle Ganzzahlen big-endian, sofern nicht anders genannt.

### 9.1 Schlüsselraum KeyV2 (D-11)

```
Präfix (65 Byte) = [kind u8][tenant_tag 16][collection_tag 16][cluster 16][doc_tag 16]
Suffix (13 Byte) = [!seq u64][flags u8][reserved 4]            Rohschlüssel = 78 Byte
tag  = BLAKE3-keyed(K_scope, label || ids)[0..16]                 (nie Rohkennungen auf Platte)
```

- Die Breiten 1 + 4 × 16 = 65 und 65 + 13 = 78 folgen den Maßen von Gate, GateLog und Supersede-Matcher (`prefix[65]`, Rohschlüssel 78 Byte) und sind \[A\], bis `erased_set.rs` und `supersede.rs` gegen diese Aufteilung geprüft sind. Weicht der Code ab, gilt der Code für die Gesamtbreite und diese Spezifikation für die Feldzuordnung.
- `kind`: 0x01 DATA, 0x02 DOCMAP, 0x03 REL, 0x04 INTENT, 0x05 TEXT, 0x06 GRAPH, 0x07 VECTOR, 0x08 CHRONICLE, 0x09 JOURNAL (spiegelt `KeyKind`, exhaustiv, ohne `non_exhaustive`; ein neuer Wert bricht den Build jedes Purgers).
- `!seq` (bitweise invertiert) sortiert neue Versionen zuerst. Alle Records eines Dokuments liegen zusammenhängend.
- `horizon_class` liegt in `flags` (Bits 0 bis 1), die Klasse (N, P, D) bestimmt die Partition und steht **nicht** im Schlüssel.

### 9.2 Manifest (Version 3)

Bestehende Einträge bleiben: `Add{path, max_tx}`, `Remove{path}`, `RollbackComplete{target_tx}`, `Replace{…}`, `WalCheckpoint{hmac}`. Neu: `AddSst{path, max_tx, partition, tag_min, tag_max}`, `SeqHwm{seq, tx}`, `HistoryFloor{partition, seq}`, `SeqTimeAnchor{seq, instant}`, `HorizonUntil{doc_tag, date}`. Atomarer Batch `Add+Remove` bleibt Pflicht für jeden Rewrite. Ein Manifest-Schreibfehler vergiftet den Store (kein Weiterschreiben nach Teilframe, Rollback auf den letzten vollständigen Frame beim Öffnen).

### 9.3 SSTable (Format 5)

Footer-Block `tag_set`: sortierte, deduplizierte `doc_tag`-Liste (bis 4.096 Tags) oder 16-Bit-XOR-Filter, dazu `tag_min` und `tag_max`. Der Bloom-Filter wird aus der tatsächlichen Schlüsselzahl der Merge-Eingabe dimensioniert (statt fest 100.000, F9).

### 9.4 WAL-Frame v2

```
[magic u8][flags u8 (Bit 0-1: Klasse N,P,D)][len u32][hdr_crc32c u32][payload len][chain_tag 32]
hdr_crc32c = CRC32C(magic || flags || len || prev_chain_tag)
chain_tag  = HMAC-SHA256(chain_key, prev_tag || header || payload)     // über die geschriebenen Bytes
```

Torn-Tail-Regel: Nur ein unvollständiger letzter Frame am EOF wird abgeschnitten. Gültiger Header mit ungültigem Tag oder ein gültiger Frame hinter einer Lücke ⇒ `Corruption`, Open verweigert. `len <= MAX_FRAME` und `offset + len <= file_len` sind Pflicht. Der Pfad zum Verlustakzeptieren existiert nur hinter einem expliziten Wiederherstellungsschalter mit Log. P-Payload im WAL ist Umschlag v3 (Chiffretext).

### 9.5 Tresor (vorhanden, mit Erweiterung)

Datei je Shard (16 Shards): Kopf 64 Byte (Magic `CTXFILE\0`, `format_version u16`, `kind = 10`, `alg_ids`, `file_id` 16 Byte zufällig, `created_seq`, `key_mode`, **`epoch u32` in den reservierten Bytes**, `header_tag` 16 Byte), Metablock 64 Byte (`vault_id`, `shard_no`, `cap_log2`, `live`, `tombstones`, `generation`, `rsv` Scrub-Cursor, MAC), danach Slots zu 64 Byte ab Offset 128: `slot_type`, `status` (leer, aktiv, Grabstein), `gen u48`, `tag` 16 Byte, `payload` 32 Byte, `mac` 8 Byte (HMAC-SHA256 unter `K_vm`, gekürzt). Slot-Typen T, C, S, D, F, E (und U nach §5.3). Grabstein-Image: `status = 0x02`, `tag` bleibt (Sondierung), `payload` genullt, `gen` erhöht, MAC gültig. Open prüft alle Köpfe und Metablöcke und je Shard 256 Zufallsslots (kein Vollscan). Lastfaktor über 0,60 ⇒ Hintergrund-Rehash, über 0,80 erzwungen, über 0,90 `Limit::Capacity`.

### 9.6 Journal, GateLog, Ledger, Bericht

- **Journal:** Append-only, CRC und Hash-Kette, Scope verschlüsselt. Codes (`Requested+Intent`, `Gated`, `KeyDestroyed`, `Purging`, `Verifying`, `Completed`, `RestrictedRead`, `SupersededByErasure`, …) stehen in `journal/record.rs` und werden **nur erweitert, nie wiederverwendet** \[A: vollständige Nummerntabelle nicht gelesen\]. Neu: `EpochSealed`, `ReSealProgress`, `CopyPurged`.
- **GateLog (`gates.log`):** `Add{gate_id, is_erased, lo, hi, predicate (Range oder PrefixSeq), gated_seq, erasure_id}` und `Remove`, AEAD, Hash-Kette, ein fsync je Gruppe.
- **Ledger:** Magic `CTRALEDG`, Hash-Kette ab `GENESIS_HASH`, Einträge mit `erasure_id`, gekeyten Commitments, `gated_seq`, Hash des Berichts; Speicherort **außerhalb** des Datenverzeichnisses; Wurzel gespiegelt (§7.9).
- **Bericht:** kanonisches NFC-JSON mit Feldern `erasure_id`, Zieltyp, Modus, Attestierungsklasse, Zeiten, `key_coverage` je Schicht, `witnesses[]`, `field_measurement`, Klassen G, K, P, D, C jeweils `reached` oder `not_reached`, `process_hardening`, `assumptions[]`, `residual_risks[]`, `collateral[]`, `signature` (Base64url), `pub_fingerprint`. Signatur über `ctx/report/v1 || kanonische Bytes ohne signature`.

### 9.7 Chronik-Eintrag

```
ChronicleEntry { id u128, kind (Norm|Policy|Config|Model|Rule|ContextReceipt|ErasureReceipt|Checkpoint),
                 matter, valid [from,to), recorded_seq, recorded_at, content_hash 32,
                 payload_ref (verschlüsselter Blob unter NormKey), supersedes?, authority_ref, prev_hash 32 }
```

Hash-Kette über kanonisch kodierte Einträge; alle 1.024 Einträge ein signierter Checkpoint (Ed25519) mit Merkle-Wurzel, optional RFC-3161-Zeitstempel. Zwei Scopes: `Global` (geteilter, nur lesbarer Rechtskorpus) und `Tenant`; ein Mandant schreibt nie in `Global`.

### 9.8 Verzeichnislayout

```
<data>/tenant-<tag>/{N,P,D}/*.sst          je Partition, Manifest-Abschnitt je Partition
<data>/tenant-<tag>/wal-*.log               ein WAL-Strang je Mandant
<data>/vault/shard-00 ... shard-15          Tresor
<data>/keyring/                             Epochen, atomar ersetzt
<data>/journal/{saga.journal, gates.log, scope-*.scope}
<data>/copies/registry                      CopyRegistry
<ledger_dir>/ledger.log                     außerhalb des Datenverzeichnisses
```

### 9.9 Migration (Reihenfolge)

1. Baseline: Phase-0-Fixes (§12) und Tests grün.
2. Neuer Code liest v2 und v3 (Umschläge) und beide Schlüsselräume, schreibt nur v3 und KeyV2.
3. Hintergrundjob `ReSeal` (journaliert, wiederaufnehmbar): P-Records lesen, unter DEK v3 neu siegeln, in KeyV2 neu schreiben, Alt-SSTs über `rewrite_filtered` entfernen.
4. Abschlusskriterium: Messung `no_v2_envelope_in_P` und `no_ascii_keys` ergibt null; danach Entfernen von v2-Lesepfad, ASCII-Codec, `legacy-wal-key`.
5. Rückfall ist nur vor Schritt 4 möglich; danach nicht mehr (Schlüsselableitung ändert sich, §5.3).

---

## 10. Nebenläufigkeit, Fehler, Betrieb

### 10.1 Sperrenordnung (total, deadlockfrei)

| Rang | Sperre | Hinweis |
| --- | --- | --- |
| 1 | `CommitBarrier` (Committer shared, Gate-Setzer exclusive) | Gate-Gruppen-Commit hält sie nur für `gate_epoch += 1`, GateLog-Append und `ErasedSet`-Einfügen, nie über fsync |
| 2 | `ErasureFence` (Schreib-Zulassung je Bereich) | Dekker-Muster, RAII-Ticket |
| 3 | Dokument-/Key-Lock | externe Shard-Locks |
| 4 | `TxBuffer`-Shards | nie zwei gleichzeitig, aufsteigend |
| 5 | `SnapshotRegistry` | nur O(1)-Operationen |
| 6 | `SequenceLog` und SSI | feingranular |
| 7 | Tresor (`FileVault`-Shard, `dek_cache`) | kurz, nie über I/O oder `.await` |
| 8 | `ErasedSet` (RwLock), `gate_epoch` (Atomic) | Leser ohne I/O |

Im Store gilt die vorhandene Ordnung `flush_mutex`, dann `commit_mutex`, dann `state`-RwLock \[V compaction.rs\]. `PurgeBarrier` steht **vor** `flush_mutex`. Kein Guard über `.await` (Lint `await_holding_lock = deny`). Ein Loom-Modell deckt Gate-Linearisierung, Fence und Flush gegen Rollover gegen Purge ab.

### 10.2 Fehlertaxonomie

| Klasse | Beispiele | Verhalten |
| --- | --- | --- |
| Transient | E/A-Fehler, Plattenreserve, Zeitüberschreitung | `Pending(Grund)` mit `attempt` und `next_retry_at`; Backoff 1 s, 2 s, 4 s bis höchstens 15 min; Gate bleibt |
| Verweigerung | Aufbewahrungspflicht, ungültige Parameter | `Refused` vor E2, terminal, mit Grund |
| Integrität | MAC-Fehler, Korruption, Poison | fail-closed: Handle vergiftet, Open verweigert, `Escalated(Reason)`, Alarm |
| Frist | SLA überschritten | `Escalated(SlaExceeded)` nach 72 h, Alarm; Löschung läuft weiter |

Kein Fehler im Schlüsselpfad wird zu einem gültigen Wert (S7). Es gibt `Escalated`, nie `Failed`. Nach außen werden Fehler auf `ActaError` abgebildet, ohne Orakeldetails.

### 10.3 Poisoning

Jeder Schreib- oder Sync-Fehler in WAL, Manifest, Tresor-Shard, Journal oder Ledger vergiftet das jeweilige Handle dauerhaft (INV-FSYNC-1, INV-VAULT-FAILCLOSED-1). Ein vergifteter Store nimmt keine Schreibvorgänge mehr an; lesen darf nur, was ohne das vergiftete Handle korrekt prüfbar ist (N). Aufhebung nur durch Neustart mit Recovery.

### 10.4 Konfiguration (Defaults, endgültig)

| Parameter | Default | Bemerkung |
| --- | --- | --- |
| `erase_return_timeout` | 500 ms | bis `Gated` oder `Pending` |
| `erase_sla` | 24 h | Ziel für E5 bis E8 |
| `escalate_after` | 72 h | Alarm, `Escalated(SlaExceeded)` |
| `legal_max` | 30 Tage | harte Obergrenze, danach Vorfallsprozess (Art. 12 Abs. 3) |
| `batch_window` | 60 s | Bündelung von Purge-Läufen |
| `min_free_bytes` | 1 GiB | Plattenreserve |
| `T_ROT` | 24 h | Epochenrotation |
| `N_SHRED` | 10.000 | Rotation nach so vielen Vernichtungen |
| `T_ROT_BACKUP` | 72 h | Höchstalter gesicherten Schlüsselmaterials |
| `K_MIN` | 5 | k-Schwelle für Aggregate in N |
| `history_default_days` | 30 | Horizont Personal ohne Rechtspflicht |
| `checkpoint_every` | 1.024 | Chronik-Checkpoint |
| `segment_max_vectors` | 50.000 | Segmentgröße (§6.12) |
| `vault_shards` | 16 | feste Zahl |
| `boot_selftest` | an | nicht abschaltbar in Release |
| `k_idx` | **keiner** | Start ohne Wert wird verweigert |
| `gate_compaction_delay` | 90 Tage | mindestens die längste Datensicherungsdauer des Betreibers |

### 10.5 Betreiberpflichten (außerhalb der Engine, im Nachweis ausgewiesen)

- Schlüsselmaterial nur im Anker (HSM oder KMS für Produktion, A1); Passphrase-Anker und Datei-Anker sind Entwicklungsstufen.
- Sicherungen: Datensicherungen nur Chiffretext, Aufbewahrung nach Betreiberrichtlinie (Empfehlung höchstens 90 Tage); Tresor, Keyring und Anker-Sicherungen höchstens 72 Stunden; Wiederherstellung des Tresors aus Live-Replik (§7.9).
- Core-Dumps aus, Swap verschlüsselt oder aus, Volumes mit Plattenverschlüsselung (LUKS, dm-crypt). Das ändert die Spezifikation nicht, stärkt aber `A-FLASH`.
- Zeitquelle synchronisiert (NTP); `Clock` ist monoton verdrahtet.
- Externe Auftragsverarbeiter und Embedding-APIs erhalten P-Inhalte nur mit Löschvertrag und Eintrag im `ReleaseLedger`.
- Logs und Traces enthalten nie PII; die Observability-Pipeline erhält nur N-Felder, Handles und Zähler.

### 10.6 Beobachtung (ohne Personenbezug)

Metriken: Dauer bis `Gated`, offene Sagas je Zustand, ältester Pending-Vorgang, Ergebnis der Messung je Schicht, Lastfaktor je Tresor-Shard, Alter der Epoche, Ledger-Floor gegen Store-Seq, Anzahl registrierter Kopien, Boot-Selbsttest-Ergebnis. `health()` meldet `Healthy`, `Degraded`, `Poisoned`, `Recovering`. Alarme: Pending über `erase_sla`, Epoche älter als 1,5 × `T_ROT`, Ledger-Floor über Store-Seq, Kopie ohne Registrierung.

---

## 11. Tests, Gates, Abnahme

### 11.1 Regeln

Jede Zusage S1 bis S10 hat mindestens einen Test. Gates laufen lokal und in der CI mit **einem** Befehl (`cargo xtask gates`, neues Dev-Crate unter 400 Zeilen); kein Gate darf mit `|| true` umgangen werden. Tests sind deterministisch (`ManualClock`, `SplitMix64`-Seeds, feste Schlüsselströme in `acta-lab`).

### 11.2 Invarianten

| ID | Invariante | Prüfung |
| --- | --- | --- |
| I-1 | Nach dem Commit von E3 liefert kein Lesepfad Klartext des Ziels, auch nicht mit `as_of < erase_seq` | Property-Test über alle Lesepfade und Loom |
| I-2 | Vor dem Commit von E2 ist der Zustand identisch zu nie beantragt; zwischen E2 und E3 ist nur das Gate sichtbar und Recovery führt E3 aus | Crash-Matrix C1 bis C5 |
| I-3 | Zustände sind monoton: kein `Live`-Slot nach Grabstein, kein Gate-Entfernen vor `Completed` und Sperrfrist | Proptest auf `can_transition_to`, Journal-Replay |
| I-4 | `HistoryFloor` und `SeqHwm` sinken nie | Manifest-Replay |
| I-5 | N-Einträge sind vor und nach einer Löschung byte-identisch (Ausnahme: Takedown, §8.5) | Hash-Vergleich |
| I-6 | Sequenznummern werden nie wiederverwendet, auch nicht nach Tombstone-GC | Test mit letzter Version per GC entfernt, Neustart |
| I-7 | Compaction mischt nie Klassen | Property-Test auf dem Picker |
| I-8 | Schlüsselmaterial wird nie im Klartext persistiert; nach Rotation liegt keine Datei unter `E_{e-1}` | Bytes-Grep, Verifier |
| I-9 | Jede Schicht (`KeyKind`) hat einen Purger | `every_key_kind_has_a_purger` (exhaustives Enum) |
| I-10 | Ein Nachweis entsteht nur aus einer Messung mit Positivkontrolle | Typestate, `compile_fail`-Tests |
| I-11 | Kein `unwrap`, `expect`, `panic`, ungeprüfter Index im Produktionscode | Clippy deny, `zero_panic_analyzer` |
| I-12 | Kein Geheimnis in Logs, `Debug`, Fehlermeldungen | Test auf `REDACTED`, Log-Grep |
| I-13 | Chronik-Abfragen sind bei gleichen Parametern identisch | Wiederholungstest, Golden |
| I-14 | Ein Restore kann Gelöschtes nicht wiederbeleben | Test T-REST |
| I-15 | Keine Nonce wird je wiederverwendet | Typ (kein Nonce-Parameter), Test mit 10^7 Siegeln |
| I-16 | Ein Absturz an jedem benannten Punkt führt zu Vollendung oder zu `Pending` mit Gate | gesamte Crash-Matrix |

### 11.3 Fehlerinjektion

`acta-lab::FaultVfs` (und `FaultInjector` in acta-durable-fs, Feature `fault-injection` im Store) beendet den Prozess an benannten Punkten. Pflicht-Punkte: C1 bis C11 (§7.4); Store: Absturz nach WAL-Append, nach Manifest-`Add`, nach `Remove`, nach `sync_dir`; `sync_dir` schlägt nach `append_batch` fehl (Daten müssen nach Recovery vorhanden sein, F2); ENOSPC mitten im Frame (nächster Open lädt ohne Mid-File-Korruption, F4); EMFILE beim Reopen nach Rollover (Store vergiftet, F3); Löschen der jüngsten WAL-Datei (Open schlägt fehl, F7); Crash im Startup-Flush (zweiter Start erfolgreich, F5); fehlendes MANIFEST in nicht pristinem Verzeichnis (klarer Fehler, F6).

### 11.4 Testarten

| Art | Umfang |
| --- | --- |
| Property (proptest) | Übergangsfunktion, Codec-Roundtrips, AEAD-Manipulation je Byte, Retention-Regeln, KeyV2-Sortierung |
| Loom | Gate-Epoche, Fence, Flush gegen Rollover gegen Purge, Snapshot-Revoke |
| Differenziell | Store gegen redb (vorhanden), Store mit Löschung gegen `ReferenceModel` |
| Fuzzing | WAL-Frames, Tresor-Slots, Ledger-Records (vorhanden: `fuzz_ledger_record`), JSON-RPC, Journal |
| Golden und Testvektoren | Bericht kanonisch, Slot-Layout, Envelope v3, Chronik-Hash-Kette (Vektoren mit zwei unabhängigen Wegen erzeugen, wie in ACTA\_SPEZIFIKATION Anhang B) |
| Mutation | `cargo-mutants` über acta-erasure und acta-crypto |
| Bytes-Grep (CI) | eindeutigen Marker in P schreiben, nach Erase und Purge im gesamten Datenverzeichnis nach Marker und nach Marker-Chiffretext suchen: null Treffer außerhalb unlesbarer Epochen |
| Kanarien | beim Start, in der CI und zyklisch im Betrieb (Test-Mandant: anlegen, schreiben, ableiten, löschen, messen, `as_of`-Lesen, Verifier) |
| Architektur | Gates G1 bis G-WORD (unten) |
| Leistung \[Ziel, ungemessen\] | Zeit bis `Gated` unabhängig von der Datenmenge (Ziel unter 500 ms), Purge-Durchsatz, Suchlatenz mit Gate, Boot-Selbsttest unter 2 s; Benchmark `bm_erase` ist vorhanden |

### 11.5 Zuordnung Zusage zu Test

| Zusage | Tests |
| --- | --- |
| S1 | T-DEL-1 (alle Lesepfade nach Gate), T-CONC-1 (Loom Gate-Linearisierung), T-TT-1 bis T-TT-8 (§8.7) |
| S2 | T-CRY-1 (Bytes-Grep), T-SEAL-1 bis 4 (Umschlag, AAD, Manipulation), T-WAL-ENC |
| S3 | T-MULTI-1 |
| S4 | T-CRASH-C1 bis C11 |
| S5 | T-CHR-1 (N-Validator, Whitelist, PII-Detektor), T-ANON-1 bis 3 (drei Kriterien) |
| S6 | T-CHR-2 (Determinismus), T-RANK-1 |
| S7 | T-NOFALLBACK-\*, T-RNG-1, G-CRY-1 |
| S8 | T-EVID-1 (Typestate), T-STUB-1 (`NotImplemented`), T-WORD-1 (verbotene Formulierungen) |
| S9 | T-REST-1 bis 3 (Zeitregression, Wiederauftauchen, Checkpoint) |
| S10 | T-DERIV-1 (Lineage vollständig), T-VEC-1, T-TEXT-1, T-GRAPH-1 |

### 11.6 Gates

| Gate | Inhalt |
| --- | --- |
| G1 | Schichtung: Abhängigkeiten nur nach unten, nur Adapter kennen zwei Peers |
| G2 | Dead-Port: jeder Port hat Produktionsimplementierung und Aufrufstelle |
| G3 | Determinismus: kein `SystemTime`, `thread_rng`, `std::fs` in Kernpfaden |
| G4 | Externe Kryptoprüfung vor Produktivbetrieb |
| G5 | Rechtliche Freigabe vor Produktivbetrieb (§13) |
| G-CRY-1 | keine Fallbacks im Kryptopfad (§5.7) |
| G-FS-1 | kein `remove_file` ohne `scrub_if_safe` für P und Tresor |
| G-ERA-1 | Löschen ausschließlich über `ErasureCore::execute` |
| G-READ-1 | jeder öffentliche Lesepfad des Stores nur über den gegateten Leser erreichbar |
| G-DEF-1 | alle Crates in `default-members`, Features additiv, kein `default = [workspace]` |
| G-DOC | keine hängenden Verweise in Dokumenten (B-17) |
| G-WORD | keine verbotene Formulierung in Berichten und Oberflächentexten |

---

## 12. Roadmap und Arbeitspakete

Aufwand ist **Schätzung, nicht gemessen** (Annahme: drei erfahrene Rust-Entwickler, Wochen). Jede Phase endet mit einem Gate und ist erst mit allen Abnahmetests abgeschlossen.

| Phase | Inhalt | Abnahme | Aufwand |
| --- | --- | --- | --- |
| **0 Baseline und Hygiene** (blockierend) | P0-1 Workspace: `default-members` vollständig, `xtask gates`, CI ohne \` |  | true`  (B-01, B-16); P0-2 erstmals bauen und testen, Ergebnis dokumentieren; P0-3 hängende Verweise und widersprüchliche Specs auflösen (B-17); P0-4 Kryptofehler B-10 beheben ( `seal\_record` ,  `derive\_dek` ,  `k\_idx` ); P0-5 Store-Fixes F1 bis F7, F9 mit Fault-Injection-Tests (B-19); P0-6  `SeqHwm` ; P0-7  `FlushPurgeFence`asymmetrisch,`enter\_purge`verdrahtet,`flush`meldet`Busy\` (B-07) |
| **1 Schlüssel und Tresor** | AEAD v3 (XChaCha20-Poly1305), Entfernen des XOR-Sealers (B-02), eine Hierarchie (D-05), `crypto_local` löschen, `derive_dek` mit `n` Betroffenen, Epochenrotation, `KeyAnchor`-Adapter (Passphrase, OS-Keyring), `harden_process` | T-SEAL, Testvektoren, Rotation mit Crash C11, Bytes-Grep auf Tresor-Dateien | 6 |
| **2 Klassen und Store-Fähigkeiten** | KeyV2 und Partitionen N, P, D (D-07, D-14), Wert-Verschlüsselung vor dem WAL (B-03), `RetentionPolicy`, `HistoryFloor`, `ReadOutcome`, `rewrite_filtered`, `compaction_barrier`, Scrub-Einbindung (B-09), SST-Format 5 mit `tag_set`, Manifest v3, Migration `ReSeal` | I-7, I-4, I-6, T-TT-3, T-TT-4; Marker nie im Datenverzeichnis | 8 |
| **3 Verdrahtung** | `acta-erasure-adapters`, `acta-node` mit Wiring, Boot-Selbsttest und Resume, `acta-engine` mit `ReadGate` (D-06), Entfernen der Trait-Welt in acta-ports/acta-types (B-05), ein Ledger und ein Berichtsformat (D-10), sync-Kern (D-09) | T-DEL-1, T-CONC-1, alle Crash-Tests C1 bis C11 über den echten Store, G2, G-ERA-1, G-READ-1 | 6 |
| **4 Chronik und Zeitreise** | `acta-chronicle`, N-Validator, `DecisionRecord`, `ContextReceipt`, AsOf-API, Takedown, Chronik-Receipt aus E4 | T-CHR, T-ANON, T-TT-1 bis 8, I-5 | 5 |
| **5 RAG-Integration** | Embeddings als P, ArtKey-Segmente, `VectorLayer`, `TextLayer`, `GraphLayer`, `CacheLayer`, `CopyRegistry` und Checkpoint-Umbau, MCP-Tools mit Pflicht-`as_of` und `erase_request` | T-VEC, T-TEXT, T-GRAPH, T-REST, T-DERIV | 8 |
| **6 Nachweis und Härtung** | `acta-verify`, Kanarien im Betrieb, `Notary`, HSM-Signer (A1), Replik-Commit, Backup-Werkzeuge, Leistungsmessung, **G4 externe Kryptoprüfung, G5 Rechtsfreigabe** | unabhängige Prüfung ohne Acta-Code besteht; Messwerte statt Zielwerte | 6 |

Abhängigkeiten: 0 vor allem; 1 vor 2 (Siegel vor Partitionen); 2 vor 3 (Adapter brauchen Store-Fähigkeiten); 3 vor 4 und 5; 6 zuletzt. Phase 4 und 5 sind nach Phase 3 parallel möglich.

---
