# Acta – Normative Spezifikation (Fassung 3.0)

*Ersetzt `docs/SPEC.md` 2.0-E. Stand 2026-11-14. Bezug: Repository `tfufuz1/acta`, HEAD `bbfad85`. Toolchain: Rust 1.89.0 (conda-forge), Messung auf Linux x86_64, 1 vCPU, 3 GB RAM.*

Normative Schlüsselwörter nach RFC 2119: **MUSS**, **DARF NICHT**, **SOLL**, **KANN**. Diese Spezifikation hat zwei Teile:

- **Teil I (Ist):** Gemessene Eigenschaften des Repositorys. Nur Messergebnisse, keine Absichten.
- **Teil II bis IX (Soll):** Verbindliche Anforderungen an die Implementierung. Jede Anforderung hat eine ID und mindestens einen Abnahmetest.

Beweismarken: **[M]** gemessen (Befehl und Ergebnis genannt), **[C]** am Quelltext geprüft (Pfad genannt), **[E]** Entscheidung dieser Spezifikation, **[R]** Rechtsfrage, extern zu klären.

---

## Teil I – Ist-Zustand (Messprotokoll)

### I.1 Messumgebung [M]

| Größe | Wert |
|---|---|
| Rust | 1.89.0 (`rustc 1.89.0 (29483883e 2025-08-04)`), installiert über `micromamba` 2.10.0, Paket `rust=1.89` aus `conda-forge` |
| Netz | `crates.io`, `conda.anaconda.org` erreichbar; `static.rust-lang.org` nicht |
| Eingriffe für die Messung (nicht im Repository) | (a) `crates/acta-erasure` in `[workspace] members` eingetragen; (b) `Cargo.lock`: `generator` 0.8.12 → 0.8.11 (benötigt rustc 1.95, Dev-Abhängigkeit von `loom`) |

### I.2 Build- und Testergebnisse

| Ziel | Ergebnis [M] |
|---|---|
| `acta-erasure` lib (`cargo check`) | kompiliert |
| `acta-erasure` lib-Tests | 40 bestanden, 0 fehlgeschlagen |
| `acta-erasure` Integrationstests (37 Targets, einzeln ausgeführt) | 35 bestanden |
| `golden_erasure_test` | 1 fehlgeschlagen: Checksumme `canonical_report_v1.golden` (Test `test_golden_manifest_checksums`) |
| `p7_report_chain_tests` | 1 fehlgeschlagen: `canonical_form_is_deterministic`, Vektor `tests/p7_vectors/sample_report_v1.*` stimmt nicht mit dem Code überein |
| `proof_crash_resume_matrix` | nicht kompilierbar, 67 Fehler (fehlende Exporte `orchestrator::MemoryKeyStore`, `MemoryResidueScanner`, `types_local::ErasureTarget`; `Clock::system_time_nanos` nicht im Trait) |
| `proof_fail_closed_and_consistency` | nicht kompilierbar, 13 Fehler (`WalSink` privat; `FenceGateError::{TokenMismatch, NotFound}`, `MemoryWalSink::{record_wal_entry, dispose_scope_wal}` fehlen) |
| `acta-store` lib-Tests | Bau erfolgreich (1 m 28 s, 267 Warnungen). Testausführung: siehe I.5 |
| Workspace-Default-Build (`cargo test` im Root) | führt `acta-erasure` nicht aus (kein Mitglied) [C] |

**Folgerung:** Die Löschkette ist als Bibliothek in sich konsistent, aber ihre Abnahmetests zur Crash-Konsistenz und zur Fail-Closed-Semantik laufen nicht. Golden-Vektoren driften vom Format.

### I.3 Experimente [M]

Datei `tests/zz_audit_v01.rs` (Messwerkzeug, nicht Teil des Zielzustands). Alle drei Tests laufen durch und dokumentieren den Ist-Zustand:

| Test | Beobachtung | Befund |
|---|---|---|
| `v01_destroyed_slot_yields_garbage_not_erased` | Nach `destroy_slot(Document, tag)` und neuer Sealer-Instanz liefert `open` `Ok(33 Bytes)`, Bytes ≠ Klartext. Erwartet wäre `Err(Erased)`. | V-01 |
| `v01_default_sealer_is_keyless` | Mit `VaultSealer::new()` (k_unit = `[0x55; 32]`, kein Tresor) rekonstruiert ein Angreifer mit dem Schlüssel allein den Klartext. | V-01 |
| `v02_xor_two_records_same_key_leaks_xor_of_plaintexts` | `Chiffre(a) ⊕ Chiffre(b) = a ⊕ b` bei gleichem Schlüssel. | V-02 |

### I.4 Befundregister

| ID | Schw. | Befund | Status | Beleg |
|---|---|---|---|---|
| V-01 | K | Sealer fällt bei Ableitungsfehler (auch „Slot gelöscht“) auf `keyed_hash(k_unit, key)` zurück; `new()` nutzt Konstante | bestätigt [M] | `vault/sealer.rs` `resolve_dek`, Test I.3 |
| V-02 | K | Siegel = XOR mit Strom aus `keyed_hash(dek, label)`, keine Nonce, kein Tag, `SealCtx` ignoriert | bestätigt [M] | `vault/sealer.rs` `seal`/`open`, Test I.3 |
| V-03 | K | `acta-erasure` nicht Workspace-Mitglied; einziger `ErasureLayer`-Implementierer ist Test-Mock | bestätigt [C] | `Cargo.toml`, `tests/saga_core_orchestration.rs` |
| V-04 | K | Tresor-Payloads T/C/S/D unverschlüsselt in der Datei; `mask_epoch_key` existiert ohne Aufrufer | bestätigt [C][M] | `vault/slot.rs` `encode_slot_body` (Payload-Offset 24 ohne Maskierung), Grep |
| V-05 | H | Fehlerpfade im Schlüsselpfad: `unwrap_or_default`, `let _ =` auf HKDF, `[0; 32]` als Salt-Ersatz, `k_idx` Default, `[0x42; 32]` im In-Memory-Keyring, BLAKE3-Fallback in `mask_file_key` | bestätigt [C] | `acta-crypto/src/aead.rs`, `dek.rs:68-89`, `vault/keyring.rs:40-78` |
| V-06 | H | Store verschlüsselt keine Werte; `RecordSealer` nur Trait mit Dummy | bestätigt [C] | `acta-store/src/util.rs` |
| V-07 | M | Retention-Policy ist in Compaction verdrahtet; keine Partitionen N/P/D im Manifest (`AddSst` fehlt) | teilweise [C] | `acta-store/src/history.rs`, `lsm/ops/compaction.rs:154`, `manifest/entry.rs` |
| V-08 | H | `FlushPurgeFence::enter_purge` ohne Aufrufer | bestätigt [C] | `acta-store/src/lsm/fence.rs:59` |
| V-09 | H | Lineage-Konzepte (`source_tags`, `ArtKey`, `CopyRegistry`, `tag_set`) fehlen | bestätigt [C] (Grep-Treffer `art_key` waren `start_key`) | `crates/` |
| V-10 | H | Checkpoints hardlinken/kopieren ohne Registrierung | bestätigt [C] | `acta-checkpoint/src/hardlink_cloner.rs` |
| V-11 | – | Grabstein nullt Tag und Payload. Richtig. SPEC 2.0-E §9.5 war falsch. | korrigiert [C] | `vault/slot.rs` `tombstone_image` |
| V-12 | M | `destroy` synchronisiert je Slot; kein Batch | bestätigt [C] | `vault/durable.rs` `destroy` |
| V-13 | H | Golden-Vektoren driften vom Format (Report-Kanonisierung) | neu [M] | I.2 |
| V-14 | H | Crash-Matrix und Fail-Closed-Tests nicht kompilierbar | neu [M] | I.2 |
| V-15 | H | Rohe numerische Record-IDs (`u64`) werden als `String` in das signierte, append-only Widerrufslog geschrieben (`kv_shredding.rs` Z. 485) | neu [C] | `revocation_log.rs`, `kv_shredding.rs` |
| V-16 | M | `DocDek`-Ableitung kollabiert T, C und D auf denselben Tag (`resolve_dek`), S fehlt | neu [C] | `vault/sealer.rs` `resolve_dek` |

### I.5 Offene Punkte aus der Vorstufe

| ID | Ergebnis |
|---|---|
| A-1 `revocation_log` | Keine Schlüsselbytes im Typ, aber rohe Identifikatoren → V-15 |
| A-2 Schreibweg Slot-Payload | Unmaskiert bestätigt → V-04 |
| A-3 HistoryFloor | Durabel und monoton (`lsm/history.rs:42` schreibt Manifest vor Sichtbarkeit). Partitionen fehlen im Manifest → V-07 |
| A-4 ArtKey | Konzept existiert nicht → V-09 |
| A-5 Build und Tests | Durchgeführt, Ergebnisse I.2 |
| A-6 Interface-Spezifikation (910 KB, generiert) | Nicht Grundlage dieser Spezifikation. Maßgeblich ist der Quelltext. Die generierte Datei ist nur Navigationshilfe |
| A-7 Rechtsquellen | Mit der Netzfreigabe nicht gegenprüfbar. Übernommen aus `docs/dsgvo/`. Siehe Abschnitt IX.2 |
| I.5-S / V-17 | `acta-store` lib-Tests: 8 bestanden, `test_compaction_stress_and_gc` hängt über 10 min. Siehe Anhang B |

---

## Teil II – Zweck, Geltungsbereich, Zusagen

### II.1 Mission [E]

Personenbezogene Daten MÜSSEN von Anfang an so angelegt werden, dass sie atomar und rückstandsfrei aus dem RAG-System entfernbar sind (Speicher, Indizes, Ableitungen, Caches, Acta-eigene Kopien). Eine Zeitreise MUSS nachvollziehbar machen, auf welcher Grundlage (Normen, Richtlinien, Grenzwerte, Fristen, Modell- und Promptstand) ein LLM entschieden hat. Beides ist nur vereinbar, wenn personenbezogene Daten von der Zeitreise entkoppelt sind.

Nicht Gegenstand: Löschung auf dem Gerät des Nutzers oder in Quelldokumenten des Verantwortlichen.

### II.2 Zusagen

Jede Zusage MUSS mindestens einen Test haben. Eine Zusage ohne bestandenen Test gilt nicht als erfüllt.

| ID | Zusage | Abnahme |
|---|---|---|
| S1 | Nach Rückkehr von `erase()` mit Zustand `Gated` liefert kein Lesepfad Klartext des Ziels, auch nicht über `as_of`. Schreibvorgänge auf das Ziel werden abgelehnt. | T-DEL-1, T-CONC-1, T-TT-1 bis 8 |
| S2 | Jeder persistierte Klartext eines Betroffenen liegt in AEAD-Umschlägen unter Schlüsselmaterial, das nach Vernichtung nicht mehr existiert. Verschlüsselung geschieht vor dem WAL. | T-CRY-1, T-SEAL-1 bis 4, T-WAL-ENC |
| S3 | Ein Dokument mit mehreren Betroffenen wird unlesbar, sobald einer gelöscht wird (Modus `DeleteAll`, Mitbetroffene im Bericht). | T-MULTI-1 |
| S4 | Absturz an jedem Durabilitätspunkt führt nach Neustart zu Vollendung oder sichtbarem `Pending` mit bestätigtem Gate. Ein bestätigtes Gate geht nie verloren. | T-CRASH-C1 bis C11 |
| S5 | Die Chronik (N) enthält keine direkt oder indirekt identifizierenden Daten. Ein Eintrag mit Personenbezug wird abgelehnt (fail-closed). | T-CHR-1, T-ANON-1 bis 3 |
| S6 | Zeitreise über N liefert für gleiche Parameter gleiche Ergebnisse. Löschung ändert N-Einträge byte-genau nicht. | T-CHR-2, T-RANK-1 |
| S7 | Kein Fehler im Schlüssel-, Zufalls-, Ableitungs- oder Siegelpfad wird zu einem gültigen Wert. Es gibt keinen Ersatzschlüssel. | T-NOFALLBACK-*, T-RNG-1, G-CRY-1 (neu: Lint-basiert, Abschnitt VIII.3) |
| S8 | Ein Nachweis entsteht nur aus einer Messung mit bestandener Positivkontrolle und nennt seine Klasse und Grenzen. Nicht Implementiertes liefert `NotImplemented`, nie Erfolg. | T-EVID-1, T-STUB-1 |
| S9 | Ein Restore aus Sicherung kann gelöschte Daten nicht wiederbeleben. | T-REST-1 bis 3 |
| S10 | Abgeleitete Strukturen tragen Herkunft. Löschung der Quelle entfernt sie oder macht sie unlesbar. Ein Ableitungsobjekt ohne Herkunft ist nicht konstruierbar (Typzwang). | T-DERIV-1, Kompilierzeit-Test `derived_requires_lineage` |
| S11 | (neu) Ein Datensatz ist nur mit dem Schlüssel seiner Löschgranularität lesbar. Zwei Verschlüsselungseinheiten teilen keinen Schlüssel in einem persistierten Container. | I-17, Property-Test |
| S12 | (neu) Kein Schlüsselmaterial und keine personenbezogene Kennung erscheint im Klartext in einem append-only Protokoll. Protokolleinträge verwenden keyed Commitments. | T-LOG-1 (Bytes-Grep über Widerrufslog und Ledger) |

### II.3 Nicht-Zusagen [E]

Verboten in jedem Bericht, jeder Oberfläche und jeder Dokumentation: „DSGVO-konform“, „garantiert gelöscht“, „unwiederbringlich“, „zertifiziert“, „revisionssicher“, „fehlerfrei“, „löscht aus allen Sicherungen“. Prüfung über G-WORD.

Nicht erfasst (immer im Bericht ausgewiesen): Klartext in RAM, Swap, Core-Dumps (außer nach Härtung, Annahme A-RAM); Sicherungen außerhalb Acta; Daten bei externen Verarbeitern (Zone Z4); Quelldateien (Zone Z5); Logs und Prompts des Aufrufers; SSD-Restdaten (Annahme A-FLASH).

---

## Teil III – Begriffe und Datenklassen

### III.1 Ebenen [E]

| Ebene | Inhalt | Zeitreise | Löschung |
|---|---|---|---|
| **N** Chronik | Normen, Richtlinien, Konfig-, Modell-, Promptstände, Entscheidungsprotokolle, Receipts. Keine PII. | unbegrenzt, bitemporal | nur Takedown (Vier-Augen) |
| **P** Tresor | Personenbezogene Einheiten, je Einheit AEAD unter Schlüssel der Hierarchie | nur Horizont, solange Schlüssel lebt | Schlüsselvernichtung, dann Purge |
| **D** Ableitung | Embeddings-Segmente, Postings, Graph, Caches, Summaries | keine eigene; aus P oder neu berechnet | Herkunftspurge oder Neuaufbau |

Embeddings sind **P-Wahrheit** und **D-Ableitung** zugleich: Die Quelle liegt verschlüsselt in P, der Index ist D (SPEC 2.0-E §6.12, bestätigt).

### III.2 Datenklassen [E]

| Klasse | Ebene | Horizont | Löschung | Retention |
|---|---|---|---|---|
| Norm | N | unbegrenzt | Takedown | `KeepAll` |
| Evidence | N | unbegrenzt | nie (ohne PII) | `KeepAll` |
| Personal | P | 30 Tage oder `retain_until` | Crypto-Shred, dann Purge | `LiveWhileKeyAlive` |
| Special (Art. 9/10) | P | 0 | wie Personal ohne Karenz | kürzestmöglich |
| Derived | D | keine | Herkunftspurge | `Window` |
| Unclassified | P (Quarantäne) | keine | wie Personal | bis Klärung |

**Regel R-CLS:** Die Klasse ist Pflichtparameter jedes Schreibvorgangs. Fehlende Deklaration ist `ClassViolation`. Im Zweifel ist ein Datum personenbezogen.

---

## Teil IV – Architektur

### IV.1 Schichtung [E]

Abhängigkeiten zeigen nur nach unten. Gleiche Ebene: keine Querkante. Nur Adapter-Crates kennen zwei Peers.

| Ring | Crates | Regel |
|---|---|---|
| R0 Fundament | `acta-ids`, `acta-env`, `acta-sys`, `acta-durable-fs`, `acta-simd`, `acta-wire`, `acta-crypto` | kein tokio, kein `std::fs` in Kernpfaden (über Ports) |
| R1 Persistenz/Index | `acta-store`, `acta-mvcc`, `acta-checkpoint`, `acta-vector`, `acta-text`, `acta-graph`, `acta-rank` | Löschwissen nur über Ports |
| R2 Verträge | `acta-types`, `acta-ports` | nur Verträge, keine Logik |
| R3 Löschkern | `acta-erasure` | keine internen Abhängigkeiten außer R0/R2 |
| R4 Brücke | `acta-erasure-adapters` | einziges Crate, das Löschkern und R1 gleichzeitig kennt |
| R5 Orchestrierung | `acta-chronicle`, `acta-engine` | |
| R6 Composition Root | `acta-node` | einzige konkrete Adapterwahl |
| R7 Oberfläche | `acta` (Fassade), `acta-mcp`, `acta-cli` | |
| Test | `acta-lab` (nur dev) | |

**R-DEP:** Der Gate-Lint G1 prüft die Kante jedes Workspace-Crates gegen diese Tabelle (`cargo metadata`).

### IV.2 Workspace [E]

- Jedes Crate MUSS in `[workspace] members` **und** `default-members` stehen (Gate G-DEF-1). **Messung:** `acta-erasure` fehlt heute in beiden (V-03).
- Features sind rein additiv. Kein `default = [workspace]` (V-16 der Vorstufe, I.4 B-16).
- `default` des Löschkerns: `signing`, `vault`. `async` ist opt-in.
- Lints: `unwrap_used`, `expect_used`, `panic`, `indexing_slicing`, `let_underscore_must_use`, `await_holding_lock` = `deny` für alle Produktionscrates. Keine lokale Aufhebung.
- `unsafe` nur in `acta-sys`, `acta-simd`, generiertem Teil von `acta-wire`. Jeder Block braucht `// SAFETY:` (Lint `undocumented_unsafe_blocks`).

### IV.3 Ports [E]

Ports gehören dem konsumierenden Crate. Gemeinsame Typen nur aus `acta-ids` und `acta-types`. Die doppelte Trait-Welt (`acta-erasure::types_local` gegen `acta-ports`) wird aufgelöst; Löschtypen leben ausschließlich in `acta-erasure`.

Kernports des Löschkerns:

| Port | Zweck | Produktionsimplementierung |
|---|---|---|
| `Clock`, `Rng`, `IdGen` | Zeit, Zufall, IDs | `acta-env` (`OsRng` im Produktionspfad) |
| `Vfs` | Dateizugriff | `DurableVfs` in `acta-durable-fs` |
| `CryptoPrimitives` | AEAD, KDF, MAC | `acta-crypto` |
| `KeyAnchor` | Epochenschlüssel verwahren und vernichten | `FileAnchor` (Dev), `OsKeyringAnchor` (Dev), `HsmAnchor` (Prod, A1) |
| `ErasureLayer` | je Schicht: `resolve`, `on_gate`, `purge`, `witness` | `StoreLayer`, `VectorLayer`, `TextLayer`, `GraphLayer`, `CheckpointLayer`, `CacheLayer` |
| `CommitBarrier`, `PurgeBarrier`, `FenceGate`, `WalSink` | Sperren und Durabilität | `StoreBarriers` |
| `SnapshotRevoker` | Snapshots widerrufen | `MvccRevoker` |
| `ResidueScanner` | Rohbyte-Scan | `ForensicScanner` |
| `ReportSigner`, `Notary` | Signatur, Zeitstempel | Ed25519 (Dev) / HSM (Prod) |
| `RecipientNotifier`, `Authz` | Art. 19, Rollen | Adapter je Deployment |

**R-PORT:** Jeder Port hat genau eine Produktionsimplementierung und eine Aufrufstelle (Gate G2, Abschnitt VIII).

---

## Teil V – Schlüsselmodell (normativ)

### V.1 Primitive [E]

| Zweck | Verfahren | Bemerkung |
|---|---|---|
| Datensatz-AEAD | XChaCha20-Poly1305, 192-Bit-Zufallsnonce | Nonce kommt ausschließlich aus `Rng`, ist kein Parameter der öffentlichen API (I-15) |
| KDF | HKDF-SHA256 | Fehler propagieren; kein Fallback |
| Passphrase | Argon2id, m ≥ 19 456 KiB, t ≥ 2, p ≥ 1 | Header `KdfHeader` |
| MAC (WAL, Slots) | HMAC-SHA256 | |
| Commitments, Tags | BLAKE3 keyed | |
| Berichtssignatur | Ed25519, Domänenpräfix `ctx/report/v1` | |

Zufall: Schlägt der `Rng` fehl, schlägt die Operation fehl. Es gibt keinen Standardwert.

### V.2 Hierarchie [E] (eine, D-05)

```
Anker (KeyAnchor: HSM | KMS | OS-Keyring | Passphrase)
  Epochenschlüssel E_e  = (K_enc, K_vm, K_commit)              e = 1, 2, …
    Tresor-Slots, Payload maskiert unter K_enc (HKDF-Strom), MAC unter K_vm
      U  K_unit       je Mandant, zufällig
      T  Salt_T       je Mandant
      C  Salt_C       je Collection
      S  Salt_S       je Betroffenem (Subject)
      D  Salt_D       je Dokument
      F  FileKey      je Datei (Ingest-Modus), maskiert
      E  ArtKey       je Ableitungssegment (nur falls Segmentmodus `Sealed`, V.6)
    DEK(doc) = HKDF-SHA256(ikm = K_unit,
                            salt = SHA-256(label ‖ Salt_T ‖ Salt_C ‖ n_u16_be ‖ Salt_S_1 … Salt_S_n ‖ Salt_D),
                            info = label ‖ doc_tag)
```

**Hierarchieregeln:**

- H1: `T`, `C`, `S`, `D` sind **verschiedene** Slotschlüssel. `resolve_dek` MUSS `tenant_tag`, `coll_tag`, `subject_tag`, `doc_tag` getrennt setzen. (Behebt V-16.)
- H2: Fehlt ein Slot, ist das Ergebnis `Erased(Slot)` oder ein Fehler. Es darf kein Ersatzwert eingesetzt werden. (Behebt V-01.)
- H3: Fehlt `S` bei einem Dokument mit Subject, ist das ein Integritätsfehler, kein `None`.
- H4: Vernichtung wirkt nach oben ableitbar. S vernichtet ⇒ alle DEKs der Dokumente des Betroffenen unableitbar. D vernichtet ⇒ ein Dokument. C ⇒ eine Collection. T und U ⇒ der Mandant.
- H5: `K_unit` liegt als eigener Slot-Typ **U** (vorher offen [A]). Entscheidung: Typ U, Tag = Mandanten-ID.

### V.3 Epochenrotation [E]

1. Alle `T_ROT` (24 h) oder nach `N_SHRED` (10 000) Vernichtungen: `E_{e+1}` erzeugen, im Anker wrappen.
2. Alle lebenden Slots unter `E_{e+1}` neu maskieren (neue Datei, `fsync`, `rename`, `fsync(dir)`). DEKs ändern sich nicht.
3. `E_e` im Anker vernichten (`KeyAnchor::destroy`).
4. Journal `EpochSealed{e}`.

**Garantie:** Eine Kopie des Tresors aus Epoche ≤ e ist ohne `E_e` nicht entschlüsselbar. Dies ist die Voraussetzung, damit Vernichtung backup-fest wird (V-04).

### V.4 Umschlag v3 [E]

```
[ver u8 = 3][alg u8 = 1][doc_tag 16][nonce 24][ciphertext][tag 16]
AAD = "acta/rec/v3" ‖ tenant ‖ class u8 ‖ keyspace u16 ‖ user_key ‖ seq u64 be
```

- `seal(ctx: &SealCtx, key: &KeyBuf, plain: &[u8]) -> Result<Envelope, SealError>`. Kein Ersatzwert.
- `open` liefert `Zeroizing<Vec<u8>>`. Alle Fehler (Länge, Version, Tag, fehlender Slot) werden zu einer Fehlerklasse ohne Orakel. Gelöscht ist davon als `Erased` unterscheidbar (S1).
- Umschlag v2 (XOR, AES-GCM-SIV mit Aufrufer-Nonce) wird nur bis zur Migration (IX.4) gelesen und nie geschrieben.
- **Einheit (I-17):** Ein Umschlag je Datensatz. Ein Block, SST-Segment oder WAL-Frame enthält keine Umschläge verschiedener Verschlüsselungseinheiten unter demselben Schlüssel.

### V.5 Tresor-Slots [E]

Format (Ergänzung zu 2.0-E §9.5, normativ):

- Slot 64 Byte: `slot_type u8`, `status u8`, `gen u48`, `tag 16`, `payload 32`, `mac 8`.
- **Payload ist immer maskiert:** `payload = Salt ⊕ HKDF(K_enc, "ctx/slotmask/v1" ‖ epoch ‖ tag)`. Behebt V-04.
- **Grabstein-Abbild** (Bestätigung V-11): alle Bytes 0 außer `status = 0x02`, MAC gültig. Das Tag wird entfernt, damit kein dauerhaftes Pseudonym des Gelöschten bleibt. Die lineare Sondierung überspringt Grabsteine per `status`.
- **Schreibpfad:** `put_slot` nimmt nur bereits maskierte Payloads (Typ `MaskedPayload`). Ein Klartext-Payload ist nicht konstruierbar.

### V.6 Ableitungssegmente [E]

Zwei Modi, pro Index konfigurierbar, Standard `Rebuild`:

- **`Rebuild` (Standard):** Segmente tragen keinen eigenen Schlüssel. Sie sind Ableitung und werden bei Löschung nach dem Gate neu aufgebaut (Abschnitt VI.5, Schritt E6). Während des Fensters sind Treffer der Zielmenge durch die Bitmaske (VII.3) ausgeblendet.
- **`Sealed`:** Segment unter `ArtKey` (Slot E). Vernichtung des ArtKey ist sofortige Wirkung. Zu wählen nur bei Segmenten, die nach Erzeugung nicht mehr verändert werden. Verfügbarkeitsbedingt auf ≤ 50 000 Vektoren begrenzt.

Begründung der Standardwahl: Ein Segment unter gemeinsamem Schlüssel würde mit einer Quelle die Vektoren fremder Betroffener unlesbar machen (Verfügbarkeitsschaden). `Rebuild` vermeidet das.

---

## Teil VI – Löschprotokoll (normativ)

### VI.1 Löscharten [E]

| Ziel (`KeyScope`) | Auslöser | Vernichteter Slot | Umfang |
|---|---|---|---|
| `Subject` | Art. 17, Widerruf | S | alle Dokumente des Betroffenen |
| `Document` | gezielt | D | ein Dokument, alle Versionen |
| `Collection` | Auflösung | C | alle Dokumente der Collection |
| `Tenant` | Off-Boarding | T und U | alle Daten, N-Verweise werden `Erased` |
| `Expired` | Sweeper | je Einheit | `retain_until` oder Horizont abgelaufen |
| `Restrict` | Art. 18 | keiner | Sperre (`Restricted`) |
| `Supersede` | Art. 16 | keiner (gleicher DEK) | alte Versionen physisch entfernen, `SupersededWithSameKey` |
| `Replace` | Ersetzen | D (alt) | neu zuerst dauerhaft, dann alt |

**Mehrbetroffene (S3):** Standard `SplitStrict` (Einheit je Betroffenem beim Ingest). Nicht trennbare Einheiten tragen mehrere S-Salts. Löschung eines Betroffenen macht sie für alle unlesbar; `collateral[]` im Bericht listet die Mitbetroffenen. Modus `DeleteAll` erfordert Attestierung des Verantwortlichen.

### VI.2 Zustände [E]

`SagaState` (extern sichtbar): `Requested → Gated → KeyDestroyed → Purging → Verifying → Completed`, Nebenzustände `Refused`, `Restricted`, `Pending(Grund)`, `Escalated(Grund)`. **Kein `Failed`.** Monotonie: Zustände schreiten nur vorwärts. Ein persistiertes Gate wird nie aufgehoben, auch nicht bei Fehlern.

### VI.3 Schritte [E] (korrigiert gegenüber 2.0-E §7.3)

| Schritt | Aktion | Durabilität | Ergebnis |
|---|---|---|---|
| E0 | Validierung, `RetentionAttestation`, Auflösung `Subject → Dokumente` (P-Record `SubjectDocs`) | keine | `Refused` oder weiter |
| E1 | Scope-Datei verschlüsselt schreiben (`fsync` Datei und Verzeichnis), dann Journal `Requested+Intent`. Single-Flight über `derive_open_key`. | fsync #1 | `Requested` |
| E2 | **Gate zuerst.** Unter `CommitBarrier::exclusive`: `gate_epoch += 1`, `GateLog`-Eintrag mit `gated_seq` und Ordinal-Bitmap (VII.3), `ErasedOrds` aktualisieren. Snapshots widerrufen (`revoke_covering`). Gruppen-fsync. | fsync #2 | `Requested` mit Gate |
| E2a | **Nadelmenge (begrenzt).** Chiffretext-Reste und Index-Terme aus Lineage (VII.3). Klartextfenster höchstens `NEEDLE_MAX = 4096`, stratifiziert, **nach** dem Gate entnommen. Nadeln nur in `mlock`-Speicher. | keine | `Requested` |
| E2b | **Ack-Pflicht.** Jede registrierte Schicht bestätigt `on_gate` (RAM-Zeroing, Cache-Invalidierung, DEK-Cache leeren). Fehlt ein Ack, bleibt der Vorgang in `Pending(LayerNotAcked)`. | keine | `Requested` |
| E3 | **Schlüsselvernichtung (kryptografischer Commit).** Höchsten Slot (S, D, C oder T/U) als Grabstein-Abbild schreiben, `fsync`, am Medium zurücklesen und verifizieren (`VerifiedVia::Direct`). Journal `KeyDestroyed`. | fsync #3, #4 | `Gated` ⇒ `KeyDestroyed` |
| E4 | Receipt: Ledger-Eintrag (Hash-Kette) und Chronik-Receipt in N (`erasure_id`, keyed Commitment des Ziels, `erase_seq`, Zeit, gebucketete Mengen; keine PII). | Ledger-fsync | `KeyDestroyed` |
| E5 | Purge-Fenster (VI.5). Plattenreserve prüfen, `PurgeBarrier` exklusiv gegen Compaction. | je Phase im Journal | `Purging` |
| E6 | Ableitungen: Segmente `Rebuild` neu aufbauen bzw. `Sealed` vernichten, Graph und Caches bereinigen. | je Schicht idempotent | `Purging` |
| E7 | Messung (VIII) MK1 bis MK3 mit Positivkontrolle, `LayerWitness` je Schicht. | Journal | `Verifying` |
| E8 | `ErasureReport` kanonisch bauen und signieren; Ledger; Journal `Completed`; Scope-Datei scrubben und löschen, **danach** Intent entfernen. | fsync | `Completed` |
| E9 | Art.-19-Benachrichtigung, `SourceProbe`, Gate-Kompaktion nach `gate_compaction_delay`. | Ledger | `Completed` |

**Regeln:**

- **R-E2-E3:** E3 darf nur nach E2 und E2b laufen (Lesesperre steht dauerhaft, bevor der Schlüssel verschwindet).
- **R-O1:** Die Zeit bis `Gated` ist unabhängig von der Datenmenge. Im Anfragepfad liegen nur E2 und E3 (siehe VIII.4 für Messung).
- **R-RET:** `erase()` kehrt bei `Gated` zurück, spätestens nach `erase_return_timeout` (500 ms) mit `Pending` und aktivem Gate.
- **R-IDEM:** Wiederholte Anfragen mit gleichem Ziel und Modus liefern dieselbe `erasure_id`.

**Begründung der Änderung (gegenüber 2.0-E):** In 2.0-E lag E1 mit Nadelerzeugung über allen Dokumenten **vor** dem Gate. Bei großen Betroffenen überschritt das die 500-ms-Grenze, ohne dass ein Gate gesetzt war. Die Nadel-Stichprobe ist nach dem Gate zulässig, weil nach E2 nur der Kern selbst liest.

### VI.4 Durabilitätspunkte und Crash-Matrix [E]

| Nr. | Absturz | Zustand | Wiederaufnahme |
|---|---|---|---|
| C1 | Scope-Datei, kein Intent | Waise | `scope_purge` entfernt; Löschung gilt als nie beantragt |
| C2 | Intent, kein Gate | `Requested` | E2 |
| C3 | Gate geschrieben, nicht gefsynct | Gate fehlt | wie C2 |
| C4 | Gate gefsynct, Schlüssel lebt | Gate dauerhaft | E2b, dann E3 |
| C5 | Slot-Grabstein nicht verifiziert | unbestimmt | Slot am Medium lesen; fehlt Grabstein, E3 wiederholen |
| C6 | Vernichtung, kein Journal `KeyDestroyed` | Grabstein ohne Journal | Recovery ergänzt `KeyDestroyed` |
| C7 | Im Purge-Fenster, Tmp-SST vorhanden | Tmp | Tmp löschen, Phase wiederholen |
| C8 | Manifest-Batch, Altdatei noch da | beide Dateien | Journal-Liste obsoleter Dateien gegen Manifest abgleichen (J1); neue Datei nie löschen |
| C9 | Messung findet Rest | `Verifying`, `matches > 0` | `Pending(WitnessMissing)`, Purge wiederholen |
| C10 | vor Bericht oder Ledger | `Verifying` bestanden | Bericht aus Zeugen im Journal neu bauen |
| C11 | Epochenrotation, E_e noch da | zwei Epochen | Rotation fortsetzen, `E_e` vernichten |
| C12 | (neu) E2b-Ack fehlt nach Neustart | `Pending(LayerNotAcked)` | `on_gate` für alle Schichten erneut ausführen |

### VI.5 Purge-Fenster (E5, Reihenfolge fest, `PurgePriority`) [E]

| Phase | Schicht | Aktion | Messung |
|---|---|---|---|
| a | Snapshots | `revoke_covering`; Checkpoint-Pins lösen | `SnapshotRevoked` |
| b | Text | Segmente ohne Zielmenge neu schreiben | Needle-Scan Terme |
| c | Memtable, WAL | Drain; WAL-Segmente bis `gated_seq` entsorgen (Scrub, Unlink) | MK2 über WAL |
| d | SSTables | gefilterter Rewrite nur betroffener Dateien; bei `cluster`-Lokalität Datei-Drop (VII.2) | MK2, Manifest-Abgleich |
| e | Vektor | Segmente `Rebuild` bzw. ArtKey-Vernichtung; Kanten reparieren | Residue-Scan |
| f | Tresor | D-Slots entfernen, Waisen prüfen, Rehash bei Last > 0,6 | MK3 |
| g | Caches | Block-, Prompt-, KV-, Embedding-Cache evictieren und nullen | Heap-Scan |
| h | Kopien | `CopyRegistry`: Einträge des Ziels entfernen oder als Chiffrat dokumentieren | Registry leer |

Compaction filtert unabhängig davon: Jede Compaction von P fragt den `ErasedView` (VII.4) und verwirft Versionen gelöschter Ziele.

---

## Teil VII – Datenmodell und Lineage

### VII.1 Schlüsselraum KeyV2 [E] (ersetzt ASCII-Keys, B-04)

```
Präfix 65 Byte = [kind u8][tenant_tag 16][collection_tag 16][cluster 16][doc_tag 16]
Suffix 13 Byte = [!seq u64][flags u8][reserved 4]
tag = BLAKE3-keyed(K_scope, label ‖ ids)[0..16]         Rohkennungen erscheinen nie auf Platte
```

- `kind`: 0x01 DATA, 0x02 DOCMAP, 0x03 REL, 0x04 INTENT, 0x05 TEXT, 0x06 GRAPH, 0x07 VECTOR, 0x08 CHRONICLE, 0x09 JOURNAL. Exhaustives Enum, kein `non_exhaustive`.
- **`cluster` ist definiert [E]:** `cluster = subject_tag` des Hauptbetroffenen, sonst `doc_tag`. Begründung: Records eines Betroffenen liegen lokal beieinander, und ein Betroffener allein in einer Datei ermöglicht Datei-Drop statt Rewrite (VI.5 d).
- `horizon_class` in `flags` Bits 0 bis 1. Die Partition (N, P, D) steht nicht im Schlüssel.
- `K_scope` ist epochal (VII.5).

### VII.2 Partitionen und Manifest [E]

- Partitionen N, P, D je Mandant mit eigenem Manifestabschnitt (`AddSst{path, max_tx, partition, tag_min, tag_max}`) und eigenem Picker. Compaction mischt nie Partitionen (I-7).
- Ein Commit-Strang je Mandant (WAL). Der WAL enthält für P nur Chiffretext.
- Manifest v3: `AddSst`, `SeqHwm{seq, tx}`, `HistoryFloor{partition, seq}`, `SeqTimeAnchor`, `HorizonUntil{doc_tag, date}`. Atomarer Batch `Add+Remove` für jeden Rewrite. Schreibfehler vergiftet den Store.
- `HistoryFloor` und `SeqHwm` sind monoton und werden **vor** dem Verwerfen von Versionen durabel geschrieben (I-4).

### VII.3 Lineage [E] (ersetzt Tag-Listen)

- Jedes Dokument erhält beim Ingest ein **Ordinal** `doc_ord: u32`, je Mandant monoton. Ordinale werden nicht wiederverwendet, bis der betroffene Index neu gebaut ist (analog I-6).
- Jede abgeleitete Struktur führt eine `RoaringBitmap` ihrer `doc_ord`-Menge (SST-Footer, Posting-Segment, Graph-Knoten, Cache-Eintrag). Elementweise: `doc_ord`.
- `ErasedOrds: RoaringBitmap` ist die einzige Gate-Struktur. `ReadGate::check` ist `contains(ord)` in O(1) ohne I/O.
- Purge-Entscheidung je Datei oder Segment: `source ∩ target ≠ ∅`. Exakt, ohne Falsch-Positive.
- Die Zuordnung `doc_ord ↔ doc_tag` liegt in P unter dem D-Slot und verschwindet mit ihm. Ordinale erscheinen nie in N oder Berichten.
- **Typzwang (S10):** `Derived<T>` trägt `Lineage` (nicht leer). `put_derived` akzeptiert nur `Derived<T>`.

### VII.4 Gate und Lesepfade [E]

- `ReadGate::admit(key) -> Result<Admitted<'g>, Erased>`. `decrypt` und Index-Ausgabe verlangen `Admitted<'g>`.
- `LsmStorage` ist `pub(crate)`. Der einzige öffentliche Lesepfad ist der gegatete Leser in `acta-engine`. Architekturtest G-READ-1 listet alle öffentlichen Lesefunktionen.
- `GateGuard::linearized_read` prüft `gate_epoch` vor und nach dem Lesen.
- `RetentionPolicy::may_drop_version(v: VersionRef, gate: &dyn ErasedView) -> bool`. `ErasedView` ist read-only und O(1). Der Store kennt keine Löschlogik, nur diese Sicht.
- SSI: Eine Transaktion, die ein Ziel gelesen hat, das vor dem Commit gelöscht wird, bricht mit `Erased` ab. Schreibende Transaktionen auf ein gegatetes Ziel werden abgelehnt (`FenceGate`).

### VII.5 Revocation und Protokolle [E] (neu; behebt V-15)

- Protokolleinträge tragen **keine** rohen Kennungen. `RevocationTarget::Record(id)` wird ersetzt durch `RevocationTarget::Record(Commitment)` mit `Commitment = HMAC-SHA256(K_commit[e], canonical(id))`.
- `K_commit[e]` wird mit `E_e` rotiert und nach `gate_compaction_delay` vernichtet. Danach sind Einträge nicht mehr zu Personen auflösbar.
- Das Widerrufslog enthält nachweislich keine Schlüsselbytes (T-LOG-1, Bytes-Grep mit bekanntem DEK).
- `K_scope` (erzeugt Tags) ist epochal; nach `gate_compaction_delay` vernichtet.
- Berichtsfelder mit Personenbezug (Zeitstempel, Mengen) werden gebucketet; Mengen unter `K_MIN = 5` werden nicht ausgegeben.

### VII.6 Chronik (N) [E]

- `ChronicleEntry { id u128, kind, matter, valid [from,to), recorded_seq, recorded_at, content_hash 32, payload_ref (Blob unter NormKey), supersedes?, authority_ref, prev_hash 32 }`.
- Hash-Kette über kanonisch kodierte Einträge. Signierter Checkpoint alle 1 024 Einträge (Ed25519, optional RFC-3161).
- Korrekturen überschreiben nie (`supersedes`).
- **N-Validator (fail-closed):** (1) Schema-Whitelist, kein Freitext; (2) PII-Detektor mit Zeitlimit, Treffer oder Timeout ⇒ Ablehnung; (3) k-Schwelle `K_MIN` für Aggregate; (4) zufällige Entscheidungs-Handles `H_d`.
- Takedown (Ausnahme zu I-5): NormKey vernichten, Vier-Augen, Ledger-Eintrag. Die Kette bleibt prüfbar.

---

## Teil VIII – Messung, Nachweis, Verifikation

### VIII.1 Messklassen [E]

| Klasse | Aussage | Beleg |
|---|---|---|
| MK1 | logische Strukturen (Indizes, Manifest, Tabellen) | strukturelle Prüfung |
| MK2 | Rohbyte-Scan ohne Formatparser über Dateien und Heap | Positivkontrolle |
| MK3 | physische Schlüsselprüfung (Slot am Medium) | `VerifiedVia::Direct` |

### VIII.2 Nachweisklassen [E] und Abbildung der Vorstufe

| SPEC-Klasse | Aussage | Abbildung aus Synthese (L/C/P/E) |
|---|---|---|
| G | Lesepfade gesperrt | L |
| K | Schlüsselmaterial vernichtet (MK3) | C (Teil) |
| P | Klartext physisch entfernt (MK2 über WAL, SST, Manifest, Journal, Heap) | P |
| D | Ableitungen sauber (`LayerWitness` je Index und Cache) | P (Teil) |
| C | Kopien bereinigt (`CopyRegistry`) | C (Teil) |

Klasse **E** der Synthese („technisch vollständig“) gilt, wenn G, K, P, D und C je `reached` sind. Der Bericht nennt jede Klasse einzeln.

### VIII.3 Typestate und Gates [E]

- `Scanner<Unarmed>` → Positivkontrolle (Muster findet sich, mutiertes Muster nicht, Trefferzahl exakt) → `Scanner<Armed>` → `LayerWitness` nur bei `matches == 0`. `arm` und Konstruktor sind nur in `measure/` aufrufbar. Compile-fail-Tests erzwingen das.
- Attestierungsklassen: A0 Selbstsignatur (nur Entwicklung, ohne Beweiswirkung), A1 HSM-Signatur außerhalb des Prozesses (Produktionsmindestwert), A2 A1 plus qualifizierter Zeitstempel plus Prüfung durch `acta-verify`.
- `acta-verify`: eigenes Werkzeug, ≤ 800 Zeilen, liest Tresor nach öffentlichem Layout, prüft Grabsteine, MACs, Signatur, Ledger-Kette und führt den Kanarien-Test aus.

**Gates** (alle über `cargo xtask gates`, ohne `|| true`):

| Gate | Prüfung | Mittel |
|---|---|---|
| G1 | Schichtung nach IV.1 | `cargo metadata` |
| G2 | Jeder Port hat Produktionsimplementierung und Aufrufstelle | Test `every_port_wired` |
| G3 | Keine `SystemTime`, `thread_rng`, `std::fs` in Kernpfaden | `clippy.toml` `disallowed_methods` |
| G-CRY-1 | Kein Fallback im Kryptopfad (ersetzt grep-Prüfung) | `clippy.toml` `disallowed_methods`/`disallowed_types`, `let_underscore_must_use` |
| G-FS-1 | `remove_file` für P und Tresor nur nach `scrub_if_safe` | Lint, Ausnahmeliste |
| G-ERA-1 | Löschen nur über `ErasureCore::execute` | Sichtbarkeit `pub(crate)` |
| G-READ-1 | Lesepfade nur über gegateten Leser | Architekturtest |
| G-DEF-1 | Alle Crates in `default-members` | Test |
| G-DOC | Keine hängenden Verweise | Link-Test |
| G-WORD | Keine verbotene Formulierung (II.3) | Wortliste über Berichtsvorlagen und Oberflächentexte |
| G-SUPPLY | Keine Abhängigkeit mit inkompatiblem MSRV oder verbotener Lizenz | `cargo deny`, `cargo +1.89 check` |
| G4 | Externe Kryptoprüfung | Vor Produktion, extern |
| G5 | Rechtliche Freigabe | Vor Produktion, extern (IX.2) |

### VIII.4 Leistungsziele [Ziel, ungemessen]

| Operation | Ziel |
|---|---|
| Zeit bis `Gated` | < 500 ms, unabhängig von Datenmenge |
| Schlüsselvernichtung Subject | 1 Slot-Schreibvorgang, 1 `sync`, 1 Rücklesen |
| Gate-Prüfung beim Lesen | O(1), kein I/O |
| Boot-Selbsttest | < 2 s |

Messung: Benchmark `bm_erase` mit Datenmengen 10³, 10⁵, 10⁶ Dokumente. Gemessen wird die Konstanz von E2 und E3.

---

## Teil IX – Recht, Betrieb, Grenzen

### IX.1 Technische Grenzen (im Bericht) [E]

| ID | Annahme |
|---|---|
| A-FLASH | Wear-Leveling, Dateisystem-Journal und Snapshots können Altblöcke behalten. Ohne Schlüssel sind sie wertlos (D-02, D-15). |
| A-RAM | RAM, Swap, Core-Dumps liegen außerhalb der Messung. Schlüssel in `mlock`-Speicher, `MADV_DONTDUMP`, `RLIMIT_CORE=0`. |
| A-ANCHOR | Der Anker vernichtet Schlüssel wie zugesichert (HSM/KMS). |
| A-CRYPTO | AEAD und KDF bleiben im Prüfzeitraum sicher (G4). |
| A-EXT | Zonen Z4 (Empfänger) und Z5 (Quelldateien) liegen nicht im Besitz von Acta. Nur Benachrichtigungsstand wird berichtet. |

### IX.2 Rechtliche Einordnung und offene Fragen [R]

Quelle: `docs/dsgvo/`. Keine Rechtsberatung. Nicht extern gegengeprüft (A-7).

| ID | Frage | Stand laut Quellen |
|---|---|---|
| R-1 | Ist Schlüsselvernichtung eine Löschung nach Art. 17? | offen; keine Behörden- oder Gerichtsaussage |
| R-2 | Entfällt der Personenbezug nach Vernichtung für jede Stelle? (EuGH C-413/23 P) | offen |
| R-3 | Gelten Embeddings in RAG-Datenbanken als personenbezogen? | offen; vorsorglich ja (III.1) |
| R-4 | Zulässige Dauer personenbezogener Lösch-IDs im Nachweis | umstritten (AG Lörrach 3 C 1099/24) |
| R-5 | Zulässige Backup-Fristen | nicht belegt; Acta setzt Konfigurationsparameter, keine Behördenfrist |
| R-6 | Anwendbarkeit § 35 BDSG auf Vektor-DB und Backups | nein (nur nicht automatisiert) |

**Konsequenz für Umsetzung:** Acta liefert die technischen Bedingungen (Synthese 3.5: Granularität, alle Kopien, kein Klartext-Parallelbestand, Metadatenschutz, Algorithmenwechsel, KDF-Stärke, echte Vernichtung, Nachweis). Die rechtliche Qualifikation liegt beim Verantwortlichen und seiner Datenschutzberatung.

### IX.3 Außenkommunikation [E]

Zulässig: Beschreibung der Mechanismen, Voraussetzungen, Klassen mit `reached`/`not_reached`, deklarierte Annahmen. Unzulässig: II.3.

### IX.4 Migration [E]

1. Baseline: Teil I, Tests grün (Stufe R0).
2. Neuer Code liest v2 und v3 Umschläge und beide Schlüsselräume, schreibt nur v3 und KeyV2.
3. Hintergrundjob `ReSeal` (journaliert, wiederaufnehmbar): P-Records lesen, unter DEK v3 neu siegeln, in KeyV2 neu schreiben, Alt-SSTs entfernen.
4. Abschluss: Messungen `no_v2_envelope_in_P` und `no_ascii_keys` ergeben null. Danach Entfernen von v2-Lesepfad, ASCII-Codec, `legacy-wal-key`.
5. Rückfall nur vor Schritt 4.

### IX.5 Betreiberpflichten [E]

- Schlüsselmaterial im Anker (HSM/KMS für Produktion).
- Datensicherungen nur Chiffretext, Aufbewahrung höchstens 90 Tage (Empfehlung). Schlüsselmaterialsicherung höchstens `T_ROT_BACKUP` = 72 h. Tresor-Wiederherstellung aus Live-Replik.
- Core-Dumps aus, Volumes verschlüsselt, Zeitquelle synchronisiert.
- Logs und Traces enthalten nie PII.

---

## Teil X – Konfiguration

| Parameter | Default | Bemerkung |
|---|---|---|
| `erase_return_timeout` | 500 ms | |
| `erase_sla` | 24 h | E5 bis E8 |
| `escalate_after` | 72 h | `Escalated(SlaExceeded)` |
| `legal_max` | 30 Tage | Vorfallsprozess danach |
| `batch_window` | 60 s | Purge-Bündelung |
| `min_free_bytes` | 1 GiB | Plattenreserve |
| `T_ROT` | 24 h | Epochenrotation |
| `N_SHRED` | 10 000 | |
| `T_ROT_BACKUP` | 72 h | |
| `K_MIN` | 5 | k-Schwelle |
| `NEEDLE_MAX` | 4 096 | E2a |
| `history_default_days` | 30 | |
| `checkpoint_every` | 1 024 | |
| `segment_max_vectors` | 50 000 | Obergrenze für `Sealed` |
| `derived_mode` | `Rebuild` | V.6 |
| `vault_shards` | 16 | fest |
| `gate_compaction_delay` | 90 Tage | ≥ längste Sicherungsdauer |
| `boot_selftest` | an | nicht abschaltbar in Release |
| `k_idx` | **kein Default** | Start ohne Wert wird verweigert |

---

## Teil XI – Teststrategie

| Art | Umfang |
|---|---|
| Property (proptest) | Übergangsfunktion, Codec-Roundtrip, AEAD-Manipulation je Byte, Retention, KeyV2-Sortierung, Roaring-Ordinale |
| Loom | Gate-Epoche, Fence, Flush gegen Rollover gegen Purge, Snapshot-Revoke |
| Differenziell | Store gegen redb; Store mit Löschung gegen `ReferenceModel` |
| Fuzzing | WAL-Frames, Tresor-Slots, Ledger-Records, JSON-RPC, Journal |
| Golden | Bericht kanonisch, Slot-Layout, Umschlag v3, Chronik-Kette. **Regel G-GOLD:** Golden-Dateien werden ausschließlich durch `regen.sh` erzeugt und in CI gegen Code geprüft; manuelle Änderung ist verboten |
| Bytes-Grep | Marker in P schreiben, nach Erase und Purge im Datenverzeichnis suchen: null Treffer außerhalb unlesbarer Epochen |
| Kanarien | Start, CI, zyklisch im Betrieb |
| Crash | Matrix C1 bis C12 mit `FaultVfs` über den echten Store |
| Architektur | G1 bis G-SUPPLY |

### XI.1 Zusage-zu-Test-Zuordnung

| Zusage | Tests |
|---|---|
| S1 | T-DEL-1, T-CONC-1, T-TT-1 bis 8 |
| S2 | T-CRY-1, T-SEAL-1 bis 4, T-WAL-ENC |
| S3 | T-MULTI-1 |
| S4 | T-CRASH-C1 bis C12 |
| S5 | T-CHR-1, T-ANON-1 bis 3 |
| S6 | T-CHR-2, T-RANK-1 |
| S7 | T-NOFALLBACK-*, T-RNG-1, G-CRY-1, `vault_missing_slot_is_erased` (neu, behebt V-01) |
| S8 | T-EVID-1, T-STUB-1 |
| S9 | T-REST-1 bis 3 |
| S10 | T-DERIV-1, `derived_requires_lineage` |
| S11 | I-17 Property-Test |
| S12 | T-LOG-1 |

### XI.2 Pflichttests für Ist-Defekte

| Defekt | Test (muss vor Merge grün sein) |
|---|---|
| V-01 | `vault_missing_slot_is_erased`: destroy, neue Instanz, `open` ⇒ `Err(Erased)` |
| V-01 | `no_default_sealer_in_release`: `VaultSealer::new()` nur in `cfg(test)` |
| V-02 | `envelope_is_aead`: Manipulation jedes Bytes ⇒ Fehler; zwei Chiffrate unter gleichem Schlüssel haben keinen gemeinsamen XOR-Strom |
| V-04 | `slot_payload_masked_at_rest`: Bytes-Grep auf Tresordatei ohne Salt-Klartext |
| V-13 | `golden_match_code`: alle Golden-Vektoren gegen aktuellen Code |
| V-14 | `proof_*`-Targets kompilieren und laufen |
| V-15 | `revocation_log_has_no_raw_ids` |

---

## Teil XII – Umsetzungsplan

Jede Stufe endet mit grünen Abnahmetests **und** bestandener Messung, nicht mit Behauptung. Aufwand: Schätzung, nicht gemessen.

| Stufe | Inhalt | Befunde | Abnahme | Aufwand |
|---|---|---|---|---|
| **R0** Stop-the-line | `acta-erasure` in Workspace; V-01-Fallback, Konstanten-Sealer, `[0x42;32]`, `k_idx`-Default entfernen; Kryptopfad fehlerfrei (Lints); Golden regenerieren oder Vektor reparieren; `proof_*` kompilierbar | V-01, V-03, V-05, V-13, V-14 | XI.2 vollständig | 1–2 Wochen |
| **R1** Krypto | Umschlag v3 (XChaCha20-Poly1305); Hierarchie H1–H5; Slot-Maskierung und Epochenrotation; `destroy_many` | V-02, V-04, V-12, V-16 | Testvektoren, C11, Bytes-Grep Tresor | 3–4 Wochen |
| **R2** Walking Skeleton | Ingest mit `Sealed<P>` → WAL/Memtable → ein Index (`acta-text`) mit Lineage → `execute` → E2, E2a, E2b, E3 → MK2 mit Positivkontrolle → Bericht → Kanarien | V-03, V-14 (vollständig), V-15 | T-DEL-1, Kanarien Ende zu Ende, C1 bis C6 | 4–6 Wochen |
| **R3** Datenmodell | `doc_ord`-Lineage; KeyV2 mit `cluster`; Partitionen N/P/D im Manifest; Wert-Siegel vor WAL; ReSeal-Migration | V-06, V-07, V-09 | I-5, I-6, I-7, `no_ascii_keys`, `no_v2_envelope_in_P` | 4–6 Wochen |
| **R4** Store | `enter_purge` verdrahtet, asymmetrisches Fence; `rewrite_filtered` mit Datei-Drop; WAL-Frame v2; Scrub einbinden | V-08, B-19 | Loom-Modelle; C7, C8 | 3–4 Wochen |
| **R5** Schichten | `VectorLayer`, `TextLayer`, `GraphLayer`, `CacheLayer`, `CheckpointLayer`; `CopyRegistry`; `Rebuild`-Modus | V-10, V-09 | T-VEC, T-TEXT, T-GRAPH, T-REST, `hnsw_no_residue_after_erase` | 4–6 Wochen |
| **R6** Chronik und Node | `acta-chronicle`, `acta-engine`, `acta-node`; Wiring mit Pflichtfeldern; Boot-Selbsttest | V-14 (Rest) | G1, G2, G-ERA-1, G-READ-1, T-CHR | 4 Wochen |
| **R7** Nachweis | `acta-verify`; Notary; HSM-Signer; Leistungsmessung; Replik; G4 extern; G5 extern | D-12 | unabhängige Prüfung ohne Acta-Code besteht | 4–6 Wochen |

**Abhängigkeiten:** R0 vor allem. R1 vor R3. R2 vor R3 bis R5 (jede Breitenarbeit gegen laufenden Durchstich). R4 und R5 parallel nach R3. R6 nach R2, R7 zuletzt.

---

## Anhang A – Gegenüberstellung zu SPEC 2.0-E

| 2.0-E | Fassung 3.0 | Grund |
|---|---|---|
| §5.3: Slot-Payloads „maskiert unter K_enc“ | V.5: Maskierung verbindlich, Schreibpfad typgeprüft | V-04 |
| §7.3 E1: Nadeln vor Gate | VI.3 E2 Gate zuerst, E2a nach Gate | Zusage O(1) |
| §7.3 E3 ohne Ack-Vorbedingung | VI.3 E2b Ack-Pflicht | Fenster „Schlüssel weg, RAM-Klartext offen“ |
| §9.1 `cluster` undefiniert | VII.1 `cluster = subject_tag` | F-4 |
| §9.3 `tag_set` mit 4 096 Tags | VII.3 Roaring-Ordinale | Skalierung, Falsch-Positive |
| §9.5 Grabstein behält `tag` | V.5 Tag wird entfernt | V-11 |
| §6.12 ArtKey je Segment | V.6 `Rebuild` als Standard, `Sealed` optional | Verfügbarkeit |
| §6.11 Checkpoints mit Registry | VI.5 Phase h, `CopyRegistry` | V-10 |
| §9.6 Ledger mit Record-ID | VII.5 keyed Commitments | V-15 |
| §11.5 S1–S10 | II.2 S1–S12 | Neue Zusagen S11, S12 |
| §7.8 Klassen G/K/P/D/C | VIII.2 mit Abbildung zu L/C/P/E | Gemeinsames Vokabular |
| §12 Roadmap, Phase 0 | XII R0 bis R7, Aufwand als Schätzung | Erster Durchstich vorgezogen |

---

## Anhang B – Ausstehende Messung

| Gegenstand | Stand |
|---|---|
| `acta-store` lib-Tests | Bau erfolgreich. 8 Tests bestanden, 0 fehlgeschlagen. Der Lauf blieb bei `compaction::tests::advanced::test_compaction_stress_and_gc` über mehr als 10 Minuten ohne Ausgabe hängen (1 vCPU). Ursache nicht geklärt: Stresstest oder Deadlock. Diese Lücke ist Befund V-17 und muss vor R4 geschlossen werden. |
| Benchmark `bm_erase` | nicht ausgeführt |
| `acta-vector`, `acta-text`, `acta-graph` Tests | nicht ausgeführt |
| Interface-Spezifikation (A-6) | nicht als Quelle verwendet |

---

## Anhang C – Messwerkzeug (nicht Teil des Zielzustands)

Datei `zz_audit_v01.rs` (drei Tests, siehe I.3, beiliegend). Sie MUSS vor R0 entfernt und durch die Tests XI.2 ersetzt werden.

## Anhang D – Quellen

- Repository `tfufuz1/acta` HEAD `bbfad85`, Pfade wie genannt
- `docs/SPEC.md` 2.0-E (Vorgänger)
- `docs/Crypto-Shredding_RAG_Rechts-und-Architektursynthese.md` (identisch zu `docs/dsgvo/…`)
- `docs/dsgvo/` weitere Analysen (Teilmengen und Duplikate, siehe Befund in der Architekturanalyse)
- `AGENTS.md`
