# IMPLEMENTATION_SPEC.md — Umbau `crates/contextra-store` → `crates/contextra-storage`

Status: normativ für die Umsetzung, Gegenstück zu `SPEC.md` (Revision R2). Bei Widerspruch gilt `SPEC.md`.
Stand der Prüfung: Struktur und Zahlen stammen aus dem geklonten Repository (`tfufuz1/acta`, `crates/contextra-store`: 32 015 Zeilen `src/`, 25 563 Zeilen in 160 Testdateien, 159 Dateien unter `tests/`, 13 Fuzz-Ziele, 6 Benches). **Nichts wurde kompiliert oder gemessen.**

## 1. Auftrag und Leitentscheidungen

1. Der Kern wird **neu und autark** gebaut (nur `std` + `contextra-ports`). Der Altstand ist Referenz, kein Ausgangscode.
2. Es entsteht **nichts** für Systeme oberhalb des Kerns. Dafür gelten die Flächen aus `SPEC.md` §7.8; jede ist klein, getestet und semver-stabil (SD-52).
3. Mechanismus im Kern, Politik außerhalb (SD-51, `SPEC.md` §2.3). Wer eine Funktion „vorsorglich“ ergänzt, verletzt diese Spezifikation.
4. Altformate werden nicht gelesen (SD-04). Es gibt keine produktiven Altdaten (A-01).

## 2. Repository-Umbau (Phase F0, mechanisch)

```
git mv crates/contextra-store legacy/contextra-store        # Historie bleibt
mkdir -p crates/contextra-ports crates/contextra-storage docs/archive
```

Workspace-`Cargo.toml`: `members = ["crates/contextra-ports", "crates/contextra-storage"]`, `resolver = "3"`, `legacy/` und `archive/` **nicht** im Workspace. Pflichtdateien: `rust-toolchain.toml` (MSRV, O-1), `rustfmt.toml`, `clippy.toml`, `deny.toml`.

`crates/contextra-ports/Cargo.toml`: keine Abhängigkeiten. `crates/contextra-storage/Cargo.toml`: `[dependencies] contextra-ports`; `[dev-dependencies] proptest, loom, arbitrary`; Feature `testing` (einziges). Lints als Crate-Attribute (P-13): `forbid(unsafe_code)`, `deny(unwrap_used, expect_used, panic, todo, unimplemented, unreachable, indexing_slicing)`.

Der Altbestand hat 36 Contextra-Crates im Workspace, der Kern hängt dort an `ids, env, types, ports, sys, mvcc, crypto, tokio, bytes, blake3, crc32fast, parking_lot, memmap2, lru, uuid, rand, quick_cache, serde`. **Keine** davon ist Abhängigkeit des neuen Kerns. Prüfung: `cargo tree -e normal -p contextra-storage` zeigt ausschließlich `contextra-ports`.

## 3. Modul-Abbildung Alt → Neu

| Alt (`src/`, Zeilen) | Neu (`SPEC.md` §3.2) | Behandlung |
|---|---|---|
| `wal/` (8 371) | `redo/` | **Neu geschrieben.** Ein Frame je `Change`, Siegel, `chain_seed`, HWM (§9.4). Szenarien wandern in Tests, Code nicht. HMAC-Kette, Legacy-Schlüsselmigration, async-Flusher entfallen. |
| `lsm/` (7 638) | `engine/`, `api/` | **Neu.** Tx-Puffer, SSI, Intent-Locks, `rollback_to_tx`, Observer-Registry (→ `Listener`), Group-Commit (→ `committer.rs`, Flat-Combining), Recovery (→ `recovery.rs`, 11 Schritte §13.1). |
| `sstable/` (4 288) | `run/`, `blob/` | **Neu.** Formate v0–v4 entfallen; ein Format §9.5. Aus dem Altcode übernommen werden nur Ideen: Block-Suche (`block_search.rs`) als Algorithmus; Bloom wird nach §9.5 (Doppel-Hashing, XXH64) neu. Drei Block-Caches → ein CLOCK-`BlockCache`. |
| `compaction/` (3 964) | `engine/compaction/` | **Neu.** Adaptiver Planer und Kostenmodell entfallen (SD-11); `MergeOperator` zurückgestellt (SD-12). Token-Bucket-Drosselung (`throttle.rs`) → `Governor`. |
| `manifest/` (1 605) | `catalog/` | **Neu.** Pfad-String-Log → numerische IDs, ein Edit = ein Record (§9.7). |
| `memtable/` (1 004) | `engine/memtable.rs` | **Neu.** Kein 16-fach-Sharding; `RwLock<BTreeMap>` nach SD-10, Abbruchkriterium H-S7. Tombstone-Bit 63 entfällt (Tag-Kind). |
| `io_seam/`, `fsync.rs` (≈ 500) | `env/`, `contextra-ports::Fs` | **Neu** über den Port `Fs` samt `FsCaps`. Fsync-Poison-Regel (`INV-LOG-5`) bleibt Prinzip. |
| `system_pressure/` (279) | `env/governor.rs`, Port `PressureSource` | Neu; kein watch-Channel, keine `tokio`. |
| `budget.rs`, `stats.rs` | `env/governor.rs`, `api/storage.rs::stats` | Neu. |
| `key_source.rs`, `util.rs` (Sealer) | `keys/`, Ports `Cipher/Mac/Kdf/KeyProvider/Entropy` | Neu; Primitive entfernt (keine `blake3`, kein AEAD im Kern). |
| `tenant_codec*`, `compat/` (2 650), `scanner/`, `engine.rs` | – | **Entfällt** (NZ-3, NZ-18, NZ-14). `compat/scanner/*` (Purger, Scan-Sichten) wird durch `erase/` und `scan_physical` ersetzt. |
| `compaction/merge_operator.rs`, `TtlMetadata` | – | Entfällt/zurückgestellt; `CompactionFilter` (§12.2) deckt Ablauf ab. |

Entfernte externe Abhängigkeiten ergeben sich aus der Tabelle: `tokio`, `tokio-util`, `bytes` (eigener `Bytes`), `parking_lot` (→ `std::sync` hinter `sync.rs`), `memmap2`, `crc32fast`, `blake3`, `lru`, `quick_cache`, `uuid`, `rand`, `serde`, `tracing`, `thiserror`.

## 4. Neue Module (nicht im Altbestand)

`api/submit.rs` (Pending), `api/inspect.rs` (`erase_status`, `scan_physical`), `api/segment.rs` (SegmentWriter/-Reader), `engine/holds.rs` (dauerhafte Holds), `keys/epoch.rs` (Schlüsselepoche), `erase/physical_scan.rs`, `testing/conformance.rs` (`fs_suite`, Flächen-Suiten), `testing/{mem_fs,fault_fs,manual_runtime,hooks,model,oracle,linearize}.rs`.

## 5. Phasen und Gates

| Phase | Inhalt | Abnahme (Gate) |
|---|---|---|
| **F0** | Repo-Umbau (§2), Ports-Crate, Toolchain, Referenz-`Cipher`/`Mac` (Dev-Crate `contextra-crypto-ref`, O-6), Goldens Anhang A.8 inkl. neuer Felder, Python-Referenzcodierer ergänzen | **G-0**: Build grün, `layering`-Test, Goldens existieren |
| **F1** | `codec/` (Varint, CRC32C portabel, XXH64, Bloom, Tag, KeyRange), `MemFs`, `FaultFs`, `RealFs`, `fs_suite` | Selbsttests von `FaultFs` (§19.2); Golden-Test `golden_anhang_a` |
| **F2** | `keys/` (Hierarchie, Nonces, Epoche, `Secret32`), `FORMAT`, `KEYS` | Nonce-Property (`INV-KEY-1`), Downgrade-Test (F-15) |
| **F3** | `run/` (Builder, Reader, Footer, Index, Filter, RangeDel), `TableCache`, `BlockCache` | H-S1, H-S9, H-S12; Bitflip auf Run |
| **F4** | `catalog/` (alle Feld-IDs inkl. `0x0012–0x0014`), Rollover, Recovery (§13.1) | H-S4, H-S5; Absturzmatrix Catalog/Redo |
| **F5** | `redo/`, `engine/` (committer, memtable, views, retention, registry), `apply`, Gegendruck, Listener, `submit` | H-S2, H-S7, H-S8, H-S13; `loom`-Modelle 1–4 |
| **F6** | Flush, Compaction, Blob-GC/Konsolidierung, Claims, Graveyard | H-S3, H-S10, H-S17; Modelltest |
| **F7** | `erase/` (E0–E7, Sweep, `erase_status`, `scan_physical`), `drop_space` | `ErasureOracle`, Absturzmatrix E1–E7 |
| **F8** | Modi `Authenticated`/`Encrypted`, `domain_sealed`, `Erase::domain`, `KeyProvider`-Vertrag, Epochen-Klon | H-S14, H-S18; `INV-KEYP-1`, `INV-EPOCH-1` |
| **F9** | `verify`, `checkpoint`, `open_readonly`, `hibernate`, dauerhafte Holds, Ressourcentests | H-S16; `INV-HOLD-1` |
| **F10** | Segment-Rahmen (`INV-SEG-1`), `Extension`, `ColdTier`, `RollbackAnchor` | Segment-Absturz-/Bitflip-Tests |
| **G-F** | Format-Freeze (§9.8 Punkte 1–14) nach F3–F8 | zwei Reviewer, alle Goldens in allen Modi |
| **G-P** | Leistungsbericht H-S1…H-S19 auf Referenzhardware (A-09) | Bericht, Abbruchkriterien §20 |

Reihenfolge ist verbindlich: kein Format wird vor G-F als stabil bezeichnet; G-F vor jeder Anbindung aufsetzender Schichten.

## 6. Neue Schnittstellen im Wortlaut (Signaturen, normativ in Semantik)

```rust
// contextra-ports
pub struct FsCaps { pub durable_file_sync: bool, pub durable_dir_sync: bool, pub atomic_rename: bool,
                    pub private_perm: bool, pub no_follow_root: bool }
pub trait KeyProvider: Send + Sync {
    fn kek(&self) -> io::Result<Secret32>;
    fn domain_key(&self, d: KeyDomain) -> io::Result<Option<Secret32>>;
    fn domain_key_create(&self, d: KeyDomain) -> io::Result<Option<Secret32>>;
    fn make_durable(&self) -> io::Result<()>;
}
pub trait SecretAlloc: Send + Sync { fn alloc(&self, b: [u8; 32]) -> io::Result<Box<dyn SecretCell>>; }

// contextra-storage
impl Storage {
    pub fn submit(&self, c: Change) -> Pending;                                   // 7.8.6
    pub fn hold_durable(&self, s: Stamp, o: HoldOwner, ttl: Option<Duration>) -> Result<HoldId>;
    pub fn hold_drop(&self, id: HoldId) -> Result<()>;
    pub fn holds(&self, owner_prefix: &str) -> Result<Vec<HoldInfo>>;
    pub fn erase_status(&self, id: ErasureId) -> Result<EraseStatus>;             // 7.8.4
    pub fn scan_physical(&self, spec: ScanSpec) -> Result<ScanReport>;            // rein lesend
    pub fn segment_writer(&self, kind: u16, reference: [u8; 16]) -> Result<SegmentWriter>;
    pub fn segment_reader(&self, kind: u16, id: u64) -> Result<SegmentReader>;
}
pub struct Pending { /* poll(), wait(), on_ready(cb) */ }
pub struct ScanReport { pub files: Vec<(FileRef, u64 /*hits*/)> }                 // nie Inhalte (P-12)
```

## 7. Test-Überführung (160 Dateien, 25 563 Zeilen)

Klassifikation des Altbestands (heuristisch nach Dateinamen, Stichproben nötig):

| Klasse | Dateien | Zeilen | Ziel-Netz |
|---|---|---|---|
| Absturz / Fehlerklassen | 40 | 6 608 | Absturzmatrix, Fehlerklassen-Matrix |
| Commit/Group/Listener/Krypto | 19 | 3 524 | Sichtbarkeit, Modell, Modi |
| Leistung/Ressourcen/Kampagnen | 23 | 3 428 | Benchmarks H-S, Ressourcentests |
| Modell/Property | 13 | 3 336 | Modelltest, Gesetze |
| Tx-/SSI-/Rollback | 19 | 2 353 | als Guard-Szenarien neu |
| Legacy-Formate, adaptiver Planer | 21 | 3 022 | **verworfen** |
| Bitflip/Fuzz | 9 | 1 031 | Bitflip, Fuzz |
| Merge-Operator | 2 | 691 | zurückgestellt |
| loom | 4 | 507 | `loom`-Modelle §16.7 |
| Mandanten-Codec | 3 | 266 | obere Schicht |
| Hilfscode | 4 + 1 | 797 | neu (`testing/`) |

Übernommen werden **Szenarien**, nicht Code. Jede `INV-*` wird per ID in einem Test zitiert (`tests/invariants.rs`). Fuzz-Ziele: bestehende 13 Namen dienen als Vorlage; neue Ziele für Segment-Rahmen, `hold_set`, `ScanSpec`.

## 8. Risiken und Gegenmaßnahmen

| Risiko | Gegenmaßnahme |
|---|---|
| Alleiniger Rewrite ≈ 60 000 Altzeilen Szenarienumfang | harte Phasen/Gates (§5); Altsuite nur als Szenariensteinbruch |
| `BTreeMap`-Memtable zu langsam | H-S7 früh in F5; Skip-Liste nur als isolierter Beschluss |
| `make_durable` blockiert Schreibpfad | liegt **vor** dem Ticket (1b), nie im Committer; Cache der Domänenschlüssel |
| Plattformen ohne Dir-Sync | `FsPolicy::Strict` verweigert; Relaxed nur Test |
| Nonce-Wiederverwendung über Klone | Epoche (SD-63) + Property `INV-EPOCH-1` |
| Spätere Formatwünsche | Freeze-Liste §9.8, Reservierungen §7.8.7 |
| Unverifizierte Hypothesen | alle H-S sind Hypothesen bis G-P |

## 9. Definition of Done je Phase

Code ≤ 400 Zeilen/Datei, `clippy -D warnings`, `cargo doc` ohne Warnungen, `cargo deny`, `miri` für `codec`/`keys`, neue Invarianten zitiert, Absturzmatrix für jeden neuen Persistenzschritt, Goldens aktualisiert (bei Formaten, zwei Reviewer).
