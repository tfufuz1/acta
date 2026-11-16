## 3. Systemarchitektur

### 3.1 Prinzipien [E]

1. **Anforderung vor Mechanismus.** Kein Bauteil ohne Zusage oder Produktanforderung.
2. **Eigenbau nur dort, wo physische Kontrolle erforderlich ist** (Segmentspeicher, Schlüsseltresor, Löschkern, Versionskontrolle). Sonst bewährte Bibliotheken aus der Whitelist (3.8).
3. **Fehler unmöglich machen statt verbieten:** Typzwang (`Sealed<P>`, `Derived<T>`, `Validated<T>`, Schlüsseltypen ohne `Default`, `Clone`, `Debug`, `Serialize`).
4. **Eine Lösung je Problem:** ein Hash-Ketten-Typ (mehrere Instanzen), ein Zeit- und Zufallsport, ein Fehlerklassifikator.
5. **Kleine Crates, harte Größenbudgets, keine Sammel-Crates.**
6. **`unsafe` nur in `acta-sys`.** Abhängigkeiten mit `unsafe` sind erlaubt und werden mit `cargo geiger` im Bericht geführt.
7. **Kern synchron.** `tokio` nur an den Rändern (Level 8 und höher), nie in Level 0 bis 7.
8. **Nicht vertrauenswürdige Eingaben laufen nie im Prozess der Schlüssel** (13.2).
9. **Ein Schreiber je Mandant, Leser lock-frei gegen unveränderliche Schnappschüsse** (3.6).

### 3.2 Level-Modell [E]

Eine Abhängigkeit `A → B` ist erlaubt genau dann, wenn `level(A) > level(B)`. Die Level stehen in `rings.toml`; Gate G1 prüft sie gegen `cargo metadata`. Querkanten auf gleichem Level sind verboten.

### 3.3 Crate-Verteilung [E]

Größenbudgets sind harte Obergrenzen für Produktionscode (ohne Tests). Überschreitung erfordert einen ADR-Eintrag.

|Level|Crate|Aufgabe|Budget (LOC)|Interne Abhängigkeiten|
|---|---|---|---|---|
|L0|`acta-base`|Newtypes (Ids, Ordinale), `DataClass`, `ErrorClass`, `ClassifiedError`, Ports `Clock`, `CryptoRng` (fallibel), kanonische Kodierung (geprüfte Big-Endian-Felder), Festkomma-Arithmetik `fx` (Q32.32, `fx_ln`, `isqrt`), Grenzwerte|2 500|–|
|L1|`acta-crypto`|AEAD (XChaCha20-Poly1305), HKDF, HMAC, Ed25519, `Sealed<P>`, Schlüsseltypen (`Zeroizing`), `HashChain`, Auffüllung (Padding)|4 200|base|
|L1|`acta-fs`|`Vfs`-Port, `DurableVfs` (fsync, atomares Ersetzen, Scrub, `O_DIRECT`-Rücklesen), `FaultVfs` (Fehlerinjektion)|2 500|base|
|L1|`acta-sys`|Einziges eigenes `unsafe`: `mlock`, `MADV_DONTDUMP`, `RLIMIT_CORE`, `PR_SET_DUMPABLE`, `SecureBuf`; Sandbox (Landlock, seccomp); Hostprüfungen; `SystemClock`, `OsCryptoRng`|1 600|base|
|L1|`acta-gate`|`ErasedView`, `GateWriter`, Roaring-Mengen, Epoche, Traits `ErasureLayer`, `LayerAck`|1 200|base|
|L1|`acta-access`|Mandanten, Rollen, Attribute (ABAC), Richtlinienauswertung, Sichtklassen, Zugriffsmengen|3 000|base|
|L1|`acta-infer`|Verträge `Embedder`, `Reranker`, `Generator`, `Verifier`, `ModelFingerprint`, `ContextPurge`, Kostenbeschreibung|700|base|
|L1|`acta-pii`|Erkennung personenbezogener Muster (Regeln, Wörterbücher, Prüfsummen), N-Validator-Bausteine, `Validated<T>`|4 000|base|
|L1|`acta-ingest`|Chunking, Normalisierung, Ressourcenlimits, Worker-Protokoll (Rahmen, Client und Server-Seite), **ohne** Parser-Abhängigkeiten|3 000|base|
|L1|`acta-parse`|Parser für PDF, Office, HTML, Text, Archive (nicht vertrauenswürdige Eingaben); **einziger** Crate mit Parser-Abhängigkeiten|5 000|base|
|L2|`acta-ledger`|Append-only-Protokolle mit Hash-Kette (K-, E-, A-Ledger), signierte Checkpoints, keyed Commitments, Trait `RegistrySink`, `DirSink`|2 800|base, crypto, fs|
|L2|`acta-vault`|Mandantentresore, `KeyAnchor`-Trait, `FileAnchor`, Wurzelrotation, Sicherungsexport, Vernichtung, Schlüsselableitungen|5 500|base, crypto, fs|
|L2|`acta-seg`|Versiegelter, **schlüsselfreier** Segmentspeicher: Journal, Segmente, Manifest, Aufräumen, Kopienregister. Schlüssel werden als Parameter übergeben|6 500|base, crypto, fs, gate|
|L3|`acta-pstore`|P-Speicher: Dokumentkopf, Versionen, Betroffenenverzeichnis, Handle-Register, Vorgänge, Fallindex (DP), Auflösung von Schlüsseln und DEK|5 000|base, crypto, fs, gate, ledger, vault, seg|
|L3|`acta-history`|Versionskontrolle K: Objekte, Refs, Sichtbarkeitsmengen, Diff, Merge, Takedown|6 500|base, crypto, pii, ledger, vault, seg, gate|
|L3|`acta-text`|Segmentierter Volltextindex (BM25 Festkomma, sichtklassenrelative Statistik) mit Lineage, DE/EN-Normalisierung|5 000|base, crypto, gate, vault, seg|
|L3|`acta-vec`|Segmentierter Vektorindex (exakt, `i8`), Lineage|4 000|base, crypto, gate, vault, seg|
|L3|`acta-graph`|Segmentierter Graphindex (CSR, Festkomma-PPR Q32.32, Pfad-RAG, Entitätsextraktion), Lineage|4 000|base, crypto, gate, vault, seg|
|L4|`acta-erasure`|Löschkern: Gate-Journal, Zustandsmaschine, Messung, Bericht-Rohdaten, Restore-Schutz, Mehrbetroffenen-Plan|9 000|base, crypto, fs, gate, ledger, vault, seg, pstore|
|L4|`acta-decision`|Entscheidungsprotokoll, Szenarien, Vergleich, Akteure|4 500|base, crypto, ledger, vault, seg, history, pstore, gate, access, infer|
|L4|`acta-rag`|Abrufpipeline: Hybridsuche (Text, Vektor, Graph), Fusion, Reranking, Kontextaufbau, Belegprüfung, Bewertungswerkzeug; implementiert `ErasureLayer` für Antwort- und Sitzungs-Caches|7 000|base, crypto, ledger, text, vec, graph, history, pstore, gate, access, infer|
|L5|`acta-reports`|Löschbericht (JCS), Verzeichnis nach Art. 30, DSFA-Entwurf, Fristen-/Aufbewahrungs-/Krypto-/Betriebsberichte, Wortfilter|3 500|base, crypto, erasure, ledger, history|
|L6|`acta-engine`|Anwendungsfälle: Ingest, Antwort, Fachfunktionen, Löschung, Szenario, Aufbewahrung, Wartung|12 000|alle darunter|
|L7|`acta-node`|Composition Root der Kernports: Konfiguration, Verdrahtung, Selbsttest, Hintergrundaufgaben, Thread-Pools|3 000|engine und alle Kernadapter|
|L8|`acta-infer-http`|Client für lokale und freigegebene Inferenzdienste (OpenAI-kompatible Schnittstelle); `tokio` erlaubt|3 000|base, infer|
|L8|`acta-sink-net`|Netz-Implementierung von `RegistrySink`; `tokio` erlaubt|600|base, ledger|
|L8|`acta-api`|HTTP/JSON-Schnittstelle, OIDC, mTLS, Streaming; `tokio` erlaubt|6 000|node|
|L9|`acta-server` (Binary)|Prozessstart, Signalbehandlung; injiziert Kantenports (Inferenz, Netzsenke, Notifier, ArchiveHook)|700|api, node, infer-http, sink-net|
|L9|`acta-cli` (Binary)|Administration: Anfragen, Szenarien, Berichte, Sicherung, Reparatur, Prüfung, Support-Paket|3 000|node, infer-http, sink-net|
|L9|`acta-parse-worker` (Binary)|Sandbox-Prozess für Parser; keine Schlüssel, kein Netz|600|ingest, parse, sys|
|–|`acta-verify` (Binary)|Unabhängige Prüfung von Berichten, Ledgern und (V2) Tresoreinträgen; **keine** interne Abhängigkeit|1 200|–|
|dev|`acta-testkit`|`ManualClock`, `SeededRng`, Fehlerinjektion, Konformitätstests, Testdaten|5 000|base, crypto, fs|
|dev|`xtask`|`cargo xtask gates`: Architektur- und Hygieneprüfungen|3 000|–|

Zusammen sind es **31 Produktions- und 2 Entwicklungs-Crates** (ca. 121 100 Zeilen Produktionscode); Binaries: `acta-server`, `acta-cli`, `acta-parse-worker`, `acta-verify`. Der Löschkern erreicht Segmente, Indizes und Caches **ausschließlich** über `ErasureLayer` aus `acta-gate`.

### 3.4 Hub-Regeln [E]

Es gibt **keinen** Sammel-Crate für „Typen“ oder „Ports“. Gemeinsame Verträge sind kleine, einem Zweck gewidmete Crates (`acta-base`, `acta-gate`, `acta-infer`).

|Regel|Inhalt|
|---|---|
|HR-1|Ein Vertrags-Crate hat genau einen Zweck und bleibt unter seinem Budget|
|HR-2|Ein Trait gehört dem Crate, das ihn **braucht**, wenn der Erbringer in einem höheren Level liegt; liegt der Erbringer tiefer, besitzt der Erbringer ihn|
|HR-3|Nutzen Erbringer und Nutzer einen Vertrag aus verschiedenen Richtungen, entsteht ein eigenes kleines Vertrags-Crate auf dem niedrigsten tragfähigen Level (`acta-gate` für `ErasureLayer`)|
|HR-4|Ein Fehlertyp je Crate (`thiserror`, `#[non_exhaustive]`, kein `Other(String)`); Aggregation erst ab L6|
|HR-5|`default = []`; Features sind additiv und haben keine Rückkante nach oben|
|HR-6|Keine Alias-Re-Exports, kein `include!`, kein `#[deprecated]`, keine Brücken-Module|
|HR-7|`acta-node` ist die einzige Stelle, die konkrete **Kern**implementierungen verdrahtet; Kantenports (Inferenz, Netzsenke, Notifier, ArchiveHook) injiziert das Binary über den Builder. Jeder Port hat im Betrieb genau eine Implementierung|
|HR-8|Fan-in-Limit: Von `acta-engine` hängt höchstens `acta-node` ab|
|HR-9|`acta-seg` kennt keine Schlüsselverwaltung: Schlüssel gelangen als Parameter (`&SegmentKey`) hinein. Schlüsselauflösung liegt in `acta-vault`/`acta-pstore`|

### 3.5 Fehlerarchitektur [E]

```rust
#[non_exhaustive]
pub enum ErrorClass { Transient, Integrity, Crypto, Erased, Policy, Invalid }

pub trait ClassifiedError: std::error::Error {
    fn class(&self) -> ErrorClass;
    fn code(&self) -> &'static str; // stabil, ohne Nutzdaten und Kennungen
}
```

`Integrity` und `Crypto` lösen Vergiftung aus (15.2). `Display` und `code()` DÜRFEN keine Personendaten oder Roh-Kennungen enthalten (Z15). `Erased` entsteht **nur** aus dem Gate oder einem Registerbeleg, nie aus der bloßen Abwesenheit eines Schlüssels (Z10).

### 3.6 Laufzeit- und Nebenläufigkeitsmodell [E]

**Synchroner Kern.** Level 0 bis 7 sind synchron und nutzen Threads. **Begründung:** Durabilitätspunkte lassen sich in synchronem Code deterministisch durch Fehlerinjektion prüfen; es entfällt der Nachweis der Abbruchsicherheit an jedem `fsync`; Modelltests (`loom`) sind direkt anwendbar. Netzwerk und Inferenzclients laufen asynchron (L8) und rufen den Kern über begrenzte Thread-Pools. **Revisionstrigger:** Messung zeigt, dass `erase_return_timeout` unter Last verletzt wird.

**Threads.**

|Pool|Größe|Aufgabe|
|---|---|---|
|`writer(t)`|1 je Mandant|alle schreibenden Anwendungsfälle und Saga-Schritte des Mandanten, seriell|
|`read`|Kernzahl|Anfragen, Suche, Entschlüsselung; nie blockierend auf `writer`|
|`rewrite`|`max(2, Kernzahl/4)`|Segmentersatz, Rewrap, Reindex, GC (niedrige Priorität)|
|`maint`|2|Rotation, Export, Sicherung, Epochenverwaltung, Fristenjob|
|`bridge`|Konfiguration (`bridge_threads`, Start 2 × Kernzahl)|Aufrufe aus L8; begrenzte Warteschlange `bridge_queue_max`; bei Überlauf `Transient(Overloaded)`|

**Schnappschüsse.** Das Manifest, die Gate-Mengen und die Sichtbarkeitsmengen sind unveränderliche Strukturen hinter `Arc`; Schreiber tauschen den Zeiger atomar, Leser laden einmal je Anfrage und arbeiten auf diesem Stand (Epoche-basierte Freigabe). Eine Datei wird physisch erst entfernt, wenn kein Leser-Schnappschuss mehr auf sie zeigt, spätestens nach `reader_drain_max` (30 s); verbliebene Leser werden dann mit `Erased` bzw. `Transient(Retry)` abgebrochen. Die Gate-Prüfung bleibt davon unberührt und wirkt sofort (Z1).

**Sperrreihenfolge (Lock-Order), strikt von oben nach unten:**
1. `rotation_lock` (global, schließt Mandantenanlage und Rotation gegeneinander aus)
2. `writer(t)` (Mandant)
3. `vault_freeze(t)` (`RwLock`: Schlüsseloperationen halten Lesesperre, Freeze-Phase der Rotation die Schreibsperre)
4. Manifest-Zeigertausch (kurz, ohne E/A unter der Sperre)
5. Gate-Zeigertausch (kurz)
6. Ledger-Anhängen (eigener Mutex je Kette)

Keine Sperre wird über einen `fsync` gehalten, außer `writer(t)`. Rückwärtsnahme ist verboten; `loom`-Tests prüfen die Ordnung.

**Gruppen-Commit.** Journal-, Gate-Log- und Tresor-Anfügungen werden innerhalb eines Fensters von `group_commit_window` (2 ms) gebündelt und mit **einem** `fsync` bestätigt; jeder Aufrufer erhält seine Bestätigung erst nach dem `fsync`.

**Speicherbudget.** Alle Puffer in Anfragepfaden sind begrenzt (Anfragegröße, Kandidatenmenge, Kontextgröße); es gibt keine unbeschränkte Allokation aus Eingaben. Schlüsselmaterial liegt nur in `SecureBuf`; der DEK-Cache (`dek_cache_max` = 4 096 Einträge, LRU) ist selbst Teil des `ErasureLayer`-Satzes und wird bei Sperre und Vernichtung der betroffenen Einträge geleert. Entschlüsselte Chunk-Texte im Chunk-Cache tragen Lineage, werden beim Verdrängen genullt (`Zeroizing`) und bei Gate-Treffer verworfen.

### 3.7 Normative Port-Signaturen (Kernverträge) [E]

```rust
// acta-base
pub trait Clock: Send + Sync { fn now_ms(&self) -> u64; }
pub trait CryptoRng: Send + Sync { fn try_fill(&self, dst: &mut [u8]) -> Result<(), EntropyError>; }

// acta-gate
pub enum GateState { Live, Restricted(RestrictReason), Superseded, Erased }
pub trait ErasedView: Send + Sync {            // nur lesend; von allen Lesepfaden genutzt
    fn check_ver(&self, v: VerOrd) -> GateState;           // O(1), keine E/A
    fn is_redacted_k(&self, k: KOrd) -> bool;              // O(1)
    fn epoch(&self) -> u64;
    fn lineage_hit(&self, l: &Lineage) -> LineageHit;      // Live | Hit{restricted|erased|superseded}
}
pub struct TargetSet { pub subjects: RoaringBitmap, pub vers: RoaringBitmap, pub kords: RoaringTreemap }
pub trait ErasureLayer: Send + Sync {          // implementiert von seg, pstore, text, vec, rag, Schlüssel-Cache
    fn name(&self) -> &'static str;
    fn plan(&self, t: TenantOrd, target: &TargetSet) -> Result<LayerPlan, LayerError>;
    fn purge(&self, t: TenantOrd, plan: &LayerPlan) -> Result<LayerAck, LayerError>;   // idempotent
    fn residue(&self, t: TenantOrd, target: &TargetSet) -> Result<Vec<Residue>, LayerError>; // Messung
}

// acta-fs
pub trait Vfs: Send + Sync {
    fn create_new(&self, p: &Path) -> io::Result<Box<dyn VFile>>;
    fn open_read(&self, p: &Path) -> io::Result<Box<dyn VFile>>;
    fn open_direct_read(&self, p: &Path) -> io::Result<Box<dyn VFile>>;  // O_DIRECT, ausgerichtet auf 4096
    fn rename_atomic(&self, from: &Path, to: &Path) -> io::Result<()>;   // rename + fsync(Verzeichnis)
    fn remove_scrub(&self, p: &Path) -> io::Result<()>;                  // Nullen überschreiben, fsync, unlink
    fn list(&self, dir: &Path) -> io::Result<Vec<PathBuf>>;
}
pub trait VFile: Send + Sync {
    fn write_all_at(&self, off: u64, buf: &[u8]) -> io::Result<()>;
    fn read_exact_at(&self, off: u64, buf: &mut [u8]) -> io::Result<()>;
    fn sync_data(&self) -> Result<(), FsyncFailed>;   // FsyncFailed ist endgültig für diese Datei
    fn len(&self) -> io::Result<u64>;
}

// acta-crypto
pub fn seal<P>(key: &DataKey, ctx: &SealCtx, plaintext: &[u8], rng: &dyn CryptoRng)
    -> Result<Sealed<P>, CryptoError>;                       // Nonce intern, nie Parameter
pub fn open<P>(key: &DataKey, ctx: &SealCtx, s: &Sealed<P>) -> Result<Zeroizing<Vec<u8>>, CryptoError>;
```

**Fsync-Regel [E]:** Nach `FsyncFailed` DARF dieselbe Datei NICHT erneut synchronisiert und als stabil angenommen werden (das Betriebssystem kann fehlgeschlagene Seiten verwerfen). Der Inhalt wird aus dem Journal oder dem Speicher in eine **neue** Datei geschrieben; scheitert auch das, wird der betroffene Bereich vergiftet (15.2). Kantenports (`Embedder`, `Reranker`, `Generator`, `Verifier`, `Notifier`, `ArchiveHook`, `RegistrySink`, `KeyAnchor`) stehen in 11.1, 8.4, 8.6, 6.2 und 4.4.

### 3.8 Abhängigkeits-Whitelist [E]

Externe Crates sind nur aus der in `deny.toml` geführten Whitelist zulässig: `chacha20poly1305`, `hkdf`, `hmac`, `sha2`, `ed25519-dalek`, `argon2`, `zeroize`, `subtle`, `roaring`, `crc32c`, `thiserror`, `unicode-normalization`, `unicode-segmentation`, `regex` (linear, ohne Rückverfolgung), `rustix` (Features `mm`, `process`, `rand`), `getrandom`, `serde` und `serde_json` (nur API und Berichte, **nie** Plattenformate), eine RFC-8785-Bibliothek (JCS; Auswahl per ADR). Tests: `proptest`, `loom`, `cargo-fuzz`, `trybuild`, `cargo-mutants`. Rand (L8 und höher): `rustls` (mTLS), `hyper`/`axum`, `tokio`. Sandbox: `landlock`, `seccompiler`. Parser-Abhängigkeiten (PDF, Office, Archive) sind **nur** im Abhängigkeitsbaum von `acta-parse` und `acta-parse-worker` zulässig und werden getrennt geführt. Jede Aufnahme verlangt Lizenz-, Wartungs- und Schwachstellenprüfung (G-SUPPLY) und einen ADR-Eintrag.
---

