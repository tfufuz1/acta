# Acta — Globale Mikrofeingranulare Schnittstellenspezifikation

**Repository:** `acta` (Workspace-Root) · **Generiert am:** 2026-11-14
**Erzeugt durch:** `scripts/generate_interface_spec.py` (automatisierte, reproduzierbare Vollanalyse aller Workspace-Crates)
**Methodik:** Statische Extraktion aller `pub`-Symbole (Traits, Structs, Enums, Funktionen, Typ-Aliase, Konstanten) aus jedem `src/**/*.rs` je Crate, angereichert mit den Original-Doc-Kommentaren (`///`, `//!`), den `Cargo.toml`-Metadaten (Abhängigkeiten, Feature-Flags) und der `lib.rs`-Re-Export-Oberfläche. Test-Module (`#[cfg(test)] mod tests`) sind ausgeschlossen.

> **Hinweis zur Reproduzierbarkeit:** Dieses Dokument wird bei jedem Lauf von `scripts/generate_interface_spec.py` vollständig neu aus dem aktuellen Quellcode-Stand erzeugt. Es spiegelt exakt den Commit wider, gegen den es generiert wurde — bei Architekturänderungen (neue Crates, geänderte Traits/Ports) einfach das Skript erneut ausführen.

## Umfang der Analyse

| Kennzahl | Wert |
|---|---|
| Analysierte Fach-Crates (`crates/*`) | 18 |
| Extrahierte öffentliche Symbole gesamt | **4137** |
| — davon Public Traits (Schnittstellen im engeren Sinn) | 165 |
| — davon Public Structs | 703 |
| — davon Public Enums | 242 |
| — davon Public Funktionen | 2750 |
| — davon Typ-Aliase | 72 |
| — davon Konstanten | 205 |

---

## 1. Ring-Architektur (Layering-Modell, DAG-Invariante)

Acta erzwingt ein striktes 5-Ring-Schichtenmodell. Abhängigkeiten dürfen **ausschließlich von höheren auf niedrigere Ringe** verlaufen (keine Aufwärtskanten, P5-Invariante).

| Ring | Name | Verantwortung | Merkmal |
|---|---|---|---|
| **0** | Foundation & Pure Numerics/Types | Domänentypen, Traits/Ports, Kryptographie, Mathematik, Vektor-/Text-/Graph-Primitive | **Kein `tokio`** — Sync-Kern, async-Schale (P26) |
| **1** | Persistence & Cache | LSM-Tree-Storage, WAL, Block-Cache, KV-Cache-Segmente, Time-Travel-Checkpoints | Async-I/O nur an den Rändern |
| **2** | External Adapters & Sandboxing | Candle/GGUF-, Ollama-, ONNX-Inferenz, WASM-Sandbox | Blätter im Abhängigkeitsgraphen |
| **3** | Orchestration & Cognition | Transaktions-Engine, Collection-Engine, Kompaktierung, Privacy-Gateway, Routing, Agent-Workflows | Geschäfts-/Orchestrierungslogik |
| **4** | Boundary & Composition Roots | Öffentliche Fassade (`acta`), MCP-Server, Python-Bindings | Einzige Composition Root |

**Unsafe-Inseln:** `acta-sys`, `acta-simd`, `acta-wire` (einzige Crates ohne `#![forbid(unsafe_code)]`).

**Locking-Disziplin:** `collections (RwLock) → kv_locks (KvKeyLocks) → embedder (RwLock)`, Shard-Indizes strikt aufsteigend akquirieren, kein `.await` unter gehaltenem Sync-Lock.

---

## 2. Crate-Inventar nach Ring (Navigationsübersicht)

**Ring 0 — Foundation:** `acta-wire`, `acta-sys`, `acta-simd`, `acta-durable-fs`, `acta-env`, `acta-erasure`, `acta-ids`

**Ring 1 — Persistence & Cache:** `acta-mvcc`, `acta-crypto`, `acta-vector`, `acta-text`, `acta-graph`, `acta-rank`, `acta-store`, `acta-checkpoint`

**Ring 2 — External Adapters & Sandboxing:** `acta-types`, `acta-ports`

**Ring 3 — Orchestration & Cognition:** `acta-mcp`

**Ring 4 — Boundary & Composition Roots:** 

**Tooling:** 

---

## 3. Legende der Crate-Abschnitte

1. **Metadatentabelle** — Ring, Migrationsstatus, Zweck, Unsafe-Deklaration, Abhängigkeiten.
2. **Architekturbeschreibung** — Original-`//!`-Kopfkommentar aus `lib.rs`.
3. **Modul- und Re-Export-Oberfläche** — exakte `pub mod`/`pub use`-Deklarationen; definiert die von außen sichtbare API-Grenze.
4. **Feature-Flags** — Cargo-Features und Aktivierungsketten.
5. **Schnittstellen (Public Traits)** — vollständige Methodensignaturen.
6. **Öffentliche Datentypen** — Structs (Felder) und Enums (Varianten).
7. **Typ-Aliase / Konstanten**.
8. **Öffentliche Funktionen** — vollständige Signaturen mit Kurzbeschreibung.

---

## `acta-types`

| Attribut | Wert |
|---|---|
| **Ring** | 2 — Ebene E2 — Integration, Domäne & Orchestrierung |
| **Status** | Fertig |
| **Zweck** | Kanonische Domänenmodelle, Budgets, Filter-AST und Fehlertypen. |
| **Unsafe-Code** | Nein (`#![forbid(unsafe_code)]`) |
| **Interne Abhängigkeiten** | `acta-ids` |
| **Externe Abhängigkeiten (Auswahl)** | `ahash`, `bincode`, `blake3`, `bytes`, `serde`, `serde_json`, `sha2`, `thiserror` |

### Modul- und Re-Export-Oberfläche (lib.rs)

```rust
pub mod error;
pub mod error_dto;
pub mod model_fingerprint;
pub mod retrieval_strategy;
pub mod schema;
pub mod tenant_scope;
pub mod tombstone;
pub mod types;
pub mod erasure;
pub mod ids;
pub mod refs;
pub mod witness;
pub use erasure::*;
pub use ids::{ErasureId, FileId, GateId, Seq, StoreId, Timestamp};
pub use refs::{CollectionRef, DocKey, OperatorRef, SubjectRef, TenantRef, MAX_REF_LEN};
pub use error::{ActaError, HnswDeletionError, Result};
pub use error_dto::ActaErrorDto;
pub use model_fingerprint::ModelFingerprint;
pub use retrieval_strategy::RetrievalStrategy;
pub use schema::{DocIdWidth, ManifestSchemaVersion};
pub use tenant_scope::{TenantScopeViolation, TenantScoped};
pub use tombstone::{SeqBitTombstone, TombstoneSemanticsCheck};
pub use types::*;
pub use witness::{RequestContext, SagaRecord, ScanMethod};
```

### Feature-Flags

| Flag | Aktiviert |
|---|---|
| `default` | `[]` |
| `docid-128` | `["acta-ids/docid-128"]` |
| `auto-extraction-opt-out` | `[]` |

### Schnittstellen — Public Traits (1)

**`TombstoneSemanticsCheck`** — ** (`src/tombstone.rs`)

```rust
pub trait TombstoneSemanticsCheck {
/// Returns `true` if the sequence number or flags indicate a soft-deleted tombstone.
    fn is_tombstone(seq: u64, flags: u8) -> bool;

    /// Encodes a sequence number as an 8-byte little-endian tombstone payload with `TOMBSTONE_BIT` set.
    fn make_tombstone(seq: u64) -> Vec<u8>;
}
```

### Öffentliche Datentypen — Structs (42)

| Struct | Felder / Form | Beschreibung | Datei |
|---|---|---|---|
| `ActaErrorDto` | `{ /// Stable string identifier for the error variant (e.g. `"NotFound"`, `"PolicyViolation"`). pub kind: String, /// Human-readable error message. pub message: S… }` | — | `src/error_dto.rs` |
| `Armed` | `(tuple/unit)` | — | `src/erasure.rs` |
| `ConfigFingerprint` | `{ /// LLM-Modell-ID (z.B. "llama-3.2-3b-instruct") pub model_id: String, /// Quantisierungsgrad als String: "Q4_K_M", "Q8_0", "F16", "BF16". /// EXPLIZIT Teil de… }` | — | `src/types/domain/misc.rs` |
| `ContextChunk` | `{ /// Document identifier associated with this chunk. pub doc_id: DocId, /// Text content of the chunk. pub content: String, /// Computed relevance score. pub re… }` | — | `src/types/saos.rs` |
| `ContextWindow` | `{ /// List of included context chunks. pub chunks: Vec<ContextChunk>, /// Total aggregated token count. pub total_tokens: usize, /// Indicates whether context ch… }` | — | `src/types/saos.rs` |
| `ControlEvidence` | `(tuple/unit)` | — | `src/erasure.rs` |
| `Edge` | `{ /// Source entity identifier. pub from: EntityId, /// Destination entity identifier. pub to: EntityId, /// Relationship label. pub label: Arc<str>, /// Numeric… }` | — | `src/types/domain/misc.rs` |
| `Embedding` | `{ /// Raw vector data elements. pub data: Vec<f32>, }` | — | `src/types/domain/document.rs` |
| `Entity` | `{ /// Unique entity ID. pub id: EntityId, /// Canonical human-readable name. pub name: Arc<str>, /// Categorical entity type. pub entity_type: Arc<str>, /// Flex… }` | — | `src/types/domain/misc.rs` |
| `EraseReceipt` | `{ /// Eindeutige Kennung des Löschvorgangs. pub id: ErasureId, /// Status des Vorgangs. pub status: ErasureStatus, /// Zeitstempel der Sperrung im Gate. pub gate… }` | — | `src/erasure.rs` |
| `EraseRequest` | `{ /// Löschziel. pub target: EraseTarget, /// Timing (sofort oder gebündelt). pub timing: Timing, /// Aufbewahrungs-Attestierung. pub retention: RetentionAttesta… }` | — | `src/erasure.rs` |
| `ErasureId` | `(tuple/unit)` | — | `src/ids.rs` |
| `FusionWeights` | `{ vector: f32, text: f32, graph: f32, /// Reserved for future metadata-signal fusion; currently always 0.0. Do not expose a public constructor parameter for this… }` | — | `src/types/saos.rs` |
| `GateId` | `(tuple/unit)` | — | `src/ids.rs` |
| `HybridQuery` | `{ /// Optional full-text query string. pub text_query: Option<String>, /// Optional vector query embedding. pub vector_query: Option<Vec<f32>>, /// Optional star… }` | — | `src/types/saos.rs` |
| `HybridQueryBuilder` | `{ text_query: Option<String>, vector_query: Option<Vec<f32>>, graph_start_node: Option<String>, graph_strategy: Option<GraphTraversalStrategy>, fusion_weights: O… }` | — | `src/types/saos.rs` |
| `ImportanceScore` | `(tuple/unit)` | — | `src/types/importance.rs` |
| `LayerId` | `(tuple/unit)` | — | `src/erasure.rs` |
| `LayerWitness` | `{ layer: LayerId, class: MeasureClass, method_id: u16, files: u64, bytes: u64, duration: Duration, hash: [u8; 32], }` | — | `src/erasure.rs` |
| `MemoryImportance` | `{ /// Base LLM or heuristic evaluation score. pub base_score: ImportanceScore, /// Recency decay function strategy. pub decay: DecayFunction, /// Transaction ID… }` | — | `src/types/importance.rs` |
| `MemoryLink` | `{ /// The target document ID. pub target: DocId, /// The type of relationship. pub relation: LinkRelation, /// The transaction ID when this link was created. pub… }` | — | `src/types/domain/misc.rs` |
| `ModelFingerprint` | `{ /// SHA-256 hash digest over model weight blob and quantization tier string. pub hash: [u8; 32], /// Model identifier or filename (e.g. "llama-3.2-3b-instruct"… }` | Identifiziert eine Modelldatei und deren Quantisierungsstufe eindeutig. | `src/model_fingerprint.rs` |
| `PprConfig` | `{ /// Damping factor (probability of continuing random walk vs restarting). Default: 0.85. pub damping_factor: f32, /// Maximum power-iteration steps before term… }` | — | `src/types/domain/misc.rs` |
| `QueryRewriteOutput` | `{ /// Textual reformulation intended for lexical keyword search. pub text_query: Option<String>, /// Semantic reformulation intended for dense vector embedding s… }` | — | `src/types/saos.rs` |
| `RequestContext` | `{ /// Tenant identifier for authorization and scope isolation. pub tenant_id: crate::TenantId, /// Transaction ID allocated for the operation. pub tx_id: crate::… }` | — | `src/witness.rs` |
| `RerankResult` | `{ /// Rank position after reranking (0-based). pub index: usize, /// Original index in the input candidate array. pub original_index: usize, /// Raw model output… }` | — | `src/types/domain/document.rs` |
| `Reservation` | `{ budget: &'a TokenBudget, amount: usize, settled: AtomicBool, }` | — | `src/types/budget.rs` |
| `Residue` | `{ /// Anzahl gefundener Treffer / Rückstände. pub matches: u64, }` | — | `src/erasure.rs` |
| `ResourceBudget` | `{ /// Maximum allowed memory usage in bytes. pub memory_limit: u64, }` | — | `src/types/budget.rs` |
| `ResourceTracker` | `{ /// The configured budget. budget: ResourceBudget, /// Current memory usage in bytes. memory_used: std::sync::atomic::AtomicU64, }` | — | `src/types/budget.rs` |
| `RetentionAttestation` | `{ /// Aufbewahrungscode. pub code: RetentionCode, /// Opake Betreiberkennung (nicht im Klartext im Bericht). pub attested_by: OperatorRef, /// Zeitstempel der At… }` | — | `src/erasure.rs` |
| `SagaRecord` | `{ /// BLAKE3 hash of deleted keys. pub deleted_keys_hash: [u8; 32], }` | — | `src/witness.rs` |
| `Scanner` | `{ layer: LayerId, class: MeasureClass, method_id: u16, files: u64, bytes: u64, _state: S, }` | — | `src/erasure.rs` |
| `ScoredDocument` | `{ /// Identifier of the scored document. pub doc_id: DocId, /// Relevance or similarity score. pub score: f32, }` | — | `src/types/domain/document.rs` |
| `ScoredEntry` | `{ /// Unique result identifier. pub id: String, /// Combined final score after signal fusion. pub final_score: f32, /// Associated entry metadata. pub metadata:… }` | — | `src/types/saos.rs` |
| `SeqBitTombstone` | `(tuple/unit)` | — | `src/tombstone.rs` |
| `SignalFusionStrategies` | `{ /// Strategy used for vector search signal fusion. pub vector: FusionStrategy, /// Strategy used for full-text search signal fusion. pub text: FusionStrategy,… }` | — | `src/types/saos.rs` |
| `StepId` | `(tuple/unit)` | — | `src/error.rs` |
| `TlHfdParams` | `{ /// Regularization parameter $\sigma > 0$ controlling diffusion scale (default: 0.1). #[serde(default = "default_tl_hfd_sigma")] pub sigma: f32, /// Seed injec… }` | — | `src/types/domain/misc.rs` |
| `TokenBudget` | `{ /// Maximum total token limit. pub limit: usize, /// Strategy for calculating usable context window. pub strategy: BudgetStrategy, /// Reserved tokens for syst… }` | — | `src/types/budget.rs` |
| `Unarmed` | `(tuple/unit)` | — | `src/erasure.rs` |
| `WorkflowState` | `{ /// Associated transaction. pub tx: TxId, /// Agent memory graph state footprint (BLAKE3 hash digest). #[serde(with = "hex")] pub graph_hash: [u8; 32], }` | — | `src/types/domain/misc.rs` |

### Öffentliche Datentypen — Enums (33)

| Enum | Varianten | Beschreibung | Datei |
|---|---|---|---|
| `ActaError` | `// ═══ Core & Logic ═══ /// Internal engine logic error. #[error("Internal error: {0}")] Internal(String), /// Invalid user or API argument input. #[error("Invalid input: {0}")] InvalidInput(String),…` | — | `src/error.rs` |
| `AutoExtractionMode` | `/// Automatic entity extraction is enabled. Enabled, /// Automatic entity extraction is disabled. Disabled,` | — | `src/types/domain/misc.rs` |
| `BudgetStrategy` | `/// Konservativ: 80% des Limits als sicherer Puffer (Standard) Conservative, /// Aggressiv: 95% des Limits nutzen Aggressive, /// Exakt: Angabe des verfügbaren Fensters direkt Exact(usize),` | — | `src/types/budget.rs` |
| `CacheDirective` | `/// Segment is explicitly pinned in memory until TTL expiration or explicit release. Pin { /// Optional maximum duration for which the segment remains pinned. `None` indicates indefinite pinning. ttl…` | — | `src/error.rs` |
| `DecayFunction` | `/// No recency decay (factor remains 1.0). #[default] None, /// Exponential decay measured in `TxId` distance (not wall-clock time). Exponential { /// Half-life in transaction count. half_life_tx: u6…` | — | `src/types/importance.rs` |
| `DistanceMetric` | `/// Cosine distance (`1.0 - cos(angle)`). #[default] Cosine, /// Euclidean (L2) distance. Euclidean, /// Dot product distance (negated inner product for f32). DotProduct,` | — | `src/types/domain/document.rs` |
| `DocIdWidth` | `/// 64-bit DocId format (8-byte BLAKE3 truncation). Bit64 = 64, /// 128-bit DocId format (16-byte BLAKE3 truncation, ADR-082). Bit128 = 128,` | Gibt die von einer Schemaversion erwartete oder erzeugte Bitbreite einer `DocId` an. | `src/schema.rs` |
| `EraseMode` | `/// Vollständige physische Bereinigung und Schlüsselvernichtung. Erase, /// Einschränkung der Verarbeitung. Restrict,` | — | `src/erasure.rs` |
| `EraseTarget` | `/// Einzelnes Dokument. Document { /// Mandanten-Referenz. tenant: TenantRef, /// Collection-Referenz. collection: CollectionRef, /// Dokument-Referenz (inklusive Subject). doc: DocRef, }, /// Betrof…` | — | `src/erasure.rs` |
| `ErasureStatus` | `/// Auftrag empfangen und validiert. Requested, /// Im Gate persistiert, Lesepfade gesperrt. Gated, /// Schlüssel in Keyslots dauerhaft vernichtet. KeyDestroyed, /// Physisches Umschreiben und Berein…` | — | `src/erasure.rs` |
| `FilterExpr` | `/// Exact match: field == value Eq { /// Target metadata field name. field: String, /// Expected field value. value: serde_json::Value, }, /// Not equal: field != value Ne { /// Target metadata field…` | — | `src/types/filter.rs` |
| `FusionStrategy` | `/// Reciprocal Rank Fusion (default strategy per ADR-003). /// Robust against score scale mismatch and partial signal degradation. #[default] Rrf, /// Score-normalized fusion (CombSUM / Z-Score). ///…` | — | `src/types/saos.rs` |
| `GateVariant` | `/// Klasse P: Physisch gelöscht und gesperrt. Erased, /// Einschränkung der Verarbeitung aktiv. Restricted,` | — | `src/erasure.rs` |
| `GraphTraversalStrategy` | `/// Multi-hop BFS traversal with hop score decay (default max_hops = 3). Hops { /// Maximum traversal hop depth. max_hops: usize, }, /// Personalized PageRank power iteration starting from seed nodes…` | — | `src/types/saos.rs` |
| `HnswDeletionError` | `/// Target node with doc_id not found in graph. #[error("Node for doc_id {0:?} not found in HNSW index")] NodeNotFound(DocId), /// Graph repair resulted in a disconnected graph component. #[error("Gr…` | — | `src/error.rs` |
| `KeyCoverage` | `/// Vollständige kryptografische Unlesbarkeit aller Records/Payloads. Full, /// Struktur sichtbar, aber Verknüpfung unlesbar gemacht. Unlinked, /// Keine Schlüsselvernichtung auf dieser Schichtebene.…` | — | `src/erasure.rs` |
| `LinkRelation` | `/// Target memory provides more detail or context. Elaborates, /// Target memory contradicts this memory. Contradicts, /// Target memory supersedes this memory, replacing its context. Supersedes, ///…` | — | `src/types/domain/misc.rs` |
| `ManifestSchemaVersion` | `/// Schema Version 1 (v0.x legacy 64-bit DocId layout). #[default] V1 = 1, /// Schema Version 2 (ADR-082 128-bit DocId layout). V2 = 2,` | Schema-Versionskennung für SSTable- und WAL-Manifeste. | `src/schema.rs` |
| `MeasureClass` | `/// MK1: Logische/funktionale Prüfung. Mk1, /// MK2: Speicherebenen-Rohdatenprüfung. Mk2, /// MK3: Physische Medieninspektion. Mk3,` | — | `src/erasure.rs` |
| `MemoryType` | `/// Episodisches Gedächtnis: Erlebnisse, Unterhaltungen, zeitlich verankerte Ereignisse. /// Retrieval: Zeitliche Nähe und Relevanz zur aktuellen Session. /// Decay: Hohe Recency-Decay-Rate (Informat…` | — | `src/types/domain/misc.rs` |
| `OnSignalFailure` | `/// Abort hybrid search and return an error on signal failure (Default per Spec Teil 6.5 P1). #[default] Fail, /// Degrade search gracefully using remaining active signals and report failed signals i…` | — | `src/types/saos.rs` |
| `PendingReason` | `/// I/O-Fehler bei der Schichtbearbeitung. Io, /// SLA-Zeitfenster überschritten. SlaExceeded, /// Eine Schicht ist fehlgeschlagen. LayerFailed(LayerId), /// Nachweis/Zeuge fehlt noch. WitnessMissing…` | — | `src/erasure.rs` |
| `Phase` | `/// Intent-Registrierung. Intent, /// Gate-Setzung. Gate, /// Schlüsselvernichtung. KeyDestroy, /// Physisches Bereinigen. Purge, /// Unabhängige Nachweismessung. Verify, /// Dokument-Ersetzung (D-11…` | — | `src/erasure.rs` |
| `PprAlgorithm` | `/// Auto heuristic dispatch based on seed count (ForwardPush for <= 100 seeds, DensePowerIteration otherwise). #[default] Auto, /// Dense power-iteration algorithm (matrix-vector multiplication over…` | — | `src/types/domain/misc.rs` |
| `RefusalReason` | `/// Gesetzliche Aufbewahrungspflicht steht der Bereinigung entgegen. RetentionObligation, /// Unzulässige Parameterkombination. InvalidParameters,` | — | `src/erasure.rs` |
| `ResidualRisk` | `/// Flash-Remanenz auf FTL-Ebene. FlashRemanence, /// CoW-Dateisysteme oder Dateisystemjournale. FsJournalOrCow, /// Reste verpackter Schlüsselkopien. WrappedKeyRemanence, /// Reste in freigegebenen…` | — | `src/erasure.rs` |
| `RetentionCode` | `/// Es besteht keine Aufbewahrungspflicht. NoObligation, /// Aufbewahrungspflicht bestand und ist abgelaufen. ObligationExpired, /// Aufbewahrungspflicht besteht aktiv. ObligationActive, /// Rechtsan…` | — | `src/erasure.rs` |
| `RetrievalStrategy` | `/// Vektorbasiertes semantisches Signal. Vector, /// Textbasiertes lexikalisches/sparse Signal. Text, /// Graphbasiertes Beziehungs-Signal. Graph, /// Hybrides Verbund-Signal. Hybrid, /// Nutzt LeanR…` | Semantische Retrieval-Strategie für Signal-Fusion (§8/§9). | `src/retrieval_strategy.rs` |
| `RiskCondition` | `/// Bedingung trifft immer zu. Always, /// Trifft zu bei CoW-Dateisystemen oder Journaling. CowOrJournaling, /// Trifft zu bis zum nächsten Kompaktions- und Purge-Durchlauf. UntilPurgeComplete,` | — | `src/erasure.rs` |
| `ScanMethod` | `/// Direct structural scan of storage files, indices, or tables. Structural, /// Layer was not measured during this execution with specified reason. NotMeasured(String), /// Layer measurement is disa…` | — | `src/witness.rs` |
| `ScanMethod` | `/// MK1: Logische Scanmethode (z. B. Iteration über Indexeinträge). Mk1, /// MK2: Physische Scanmethode auf Block-/Dateiebene. Mk2, /// MK3: Rohmedien-Inspektion. Mk3,` | — | `src/erasure.rs` |
| `Signal` | `/// Dense vector embedding search signal. Vector, /// Lexical full-text search signal. Text, /// Graph traversal / PageRank signal. Graph,` | — | `src/types/saos.rs` |
| `Timing` | `/// Sofortige Ausführung im Vordergrund. Now, /// Gebündelte Ausführung im Batch-Fenster. Batched,` | — | `src/erasure.rs` |

### Typ-Aliase (3)

| Alias | Definition | Beschreibung |
|---|---|---|
| `QueryOptions` | `type QueryOptions= HybridQuery` | Alias for [`HybridQuery`] representing query options for hybrid retrieval operations (Spec Teil 6.5). |
| `QueryOptionsBuilder` | `type QueryOptionsBuilder= HybridQueryBuilder` | Alias for [`HybridQueryBuilder`] representing a query options builder for hybrid retrieval operations (Spec Teil 6.5). |
| `Result` | `type Result<T> = std::result::Result<T, ActaError>` | — |

### Konstanten (2)

| Konstante | Typ | Beschreibung |
|---|---|---|
| `EXPIRY_METADATA_KEY` | `&str` | — |
| `MAX_SEARCH_K` | `usize` | — |

### Öffentliche Funktionen (121)

| Signatur | Beschreibung | Datei |
|---|---|---|
| `pub fn arm(self, _ev: ControlEvidence) -> Scanner<Armed>` | Einzige Brücke nach Armed; erfordert Positivkontrollbeleg (TY-10). | `src/erasure.rs` |
| `pub fn bytes(&self) -> u64` | Liefert die Anzahl geprüfter Bytes. | `src/erasure.rs` |
| `pub fn class(&self) -> MeasureClass` | Liefert die Messklasse (MK1-MK3). | `src/erasure.rs` |
| `pub fn condition(&self) -> RiskCondition` | Statische Bedingung für Audit- und Verifikationswerkzeuge (AR-4). | `src/erasure.rs` |
| `pub fn duration(&self) -> Duration` | Liefert die Scandauer. | `src/erasure.rs` |
| `pub fn files(&self) -> u64` | Liefert die Anzahl geprüfter Dateien. | `src/erasure.rs` |
| `pub fn finish( self, matches: u64, duration: Duration, hash: [u8; 32], ) -> Result<LayerWitness, Residue>` | Schließt die Messung ab. matches != 0 => Err(Residue). LayerWitness wird ausschließlich bei matches == 0 ausgegeben (TY-10, I-19, D-72). | `src/erasure.rs` |
| `pub fn hash(&self) -> &[u8` | Liefert den kryptografischen Hash des Prüflaufs. | `src/erasure.rs` |
| `pub fn id(&self) -> &'static str` | Feste Identifikator-Zeichenkette nach SPEC Anhang D. | `src/erasure.rs` |
| `pub fn layer(&self) -> LayerId` | Liefert die Schicht-ID. | `src/erasure.rs` |
| `pub fn method_id(&self) -> u16` | Liefert die Methoden-ID. | `src/erasure.rs` |
| `pub fn new(layer: LayerId, class: MeasureClass, method_id: u16) -> Self` | Erzeugt einen neuen unbewaffneten Scanner. | `src/erasure.rs` |
| `pub fn record(&mut self, files: u64, bytes: u64)` | Zeichnet geprüfte Dateien und Bytes auf. | `src/erasure.rs` |
| `pub fn capability_unsupported( capability: impl Into<String>, reason: impl Into<String>, ) -> Self` | — | `src/error.rs` |
| `pub fn checksum_mismatch(path: impl Into<String>, block_id: u64) -> Self` | — | `src/error.rs` |
| `pub fn cross_device_link( source_path: impl Into<String>, target_path: impl Into<String>, ) -> Self` | — | `src/error.rs` |
| `pub fn durability_config(msg: impl Into<String>) -> Self` | — | `src/error.rs` |
| `pub fn invalid_input(msg: impl Into<String>) -> Self` | — | `src/error.rs` |
| `pub fn is_occ_conflict(&self) -> bool` | — | `src/error.rs` |
| `pub fn kv_quantization(msg: impl Into<String>) -> Self` | — | `src/error.rs` |
| `pub fn limit_exceeded(limit: usize, context: impl Into<String>) -> Self` | — | `src/error.rs` |
| `pub fn model_load(path: impl Into<String>, reason: impl Into<String>) -> Self` | — | `src/error.rs` |
| `pub fn orphaned_vector_reference( doc_id: impl Into<String>, index_id: impl Into<String>, ) -> Self` | — | `src/error.rs` |
| `pub fn pin_budget_exceeded(msg: impl Into<String>) -> Self` | — | `src/error.rs` |
| `pub fn snapshot_unsupported_for_signal(strategy: impl Into<String>) -> Self` | — | `src/error.rs` |
| `pub fn wal_corruption(offset: u64, reason: impl Into<String>) -> Self` | — | `src/error.rs` |
| `pub fn wal_truncation_detected(expected_hmac: [u8; 32], actual_hmac: [u8; 32]) -> Self` | — | `src/error.rs` |
| `pub fn new(kind: impl Into<String>, message: impl Into<String>) -> Self` | — | `src/error_dto.rs` |
| `pub fn with_details( kind: impl Into<String>, message: impl Into<String>, details: serde_json::Value, ) -> Self` | — | `src/error_dto.rs` |
| `pub fn new( hash: [u8; 32], model_id: impl Into<String>, quantization: impl Into<String>, ) -> Self` | Creates a new `ModelFingerprint`. # Ablauf Übernimmt Hash, Modellbezeichner und Quantisierungsstring ohne Nebenwirkungen. | `src/model_fingerprint.rs` |
| `pub fn as_u8(self) -> u8` | Returns the numeric version byte. | `src/schema.rs` |
| `pub fn bytes_len(self) -> usize` | Returns the size in bytes of a single `DocId` value under this width. | `src/schema.rs` |
| `pub fn current() -> Self` | Returns the active compile-time `DocIdWidth`. # Ablauf Prüft zur Compile-Zeit via `cfg!(feature = "docid-128")`, welche Breite gewählt ist. | `src/schema.rs` |
| `pub fn doc_id_width(self) -> DocIdWidth` | Returns the corresponding `DocIdWidth` for this schema version. | `src/schema.rs` |
| `pub fn from_u8(version_byte: u8) -> Result<Self>` | Parses a version from a numeric byte indicator. # Fehler Liefert `ActaError::Serialization`, falls das übergebene Byte keiner bekannten Ver… | `src/schema.rs` |
| `pub fn is_compatible_with_current_build(self) -> bool` | Returns `true` if this schema version is compatible with the compile-time DocId feature flags. | `src/schema.rs` |
| `pub fn available(&self) -> usize` | Returns tokens still available after subtracting reserved and consumed tokens. | `src/types/budget.rs` |
| `pub fn budget(&self) -> &ResourceBudget` | Returns a reference to the configured resource budget. | `src/types/budget.rs` |
| `pub fn consume(&self, tokens: usize)` | Records `tokens` as consumed, reducing future availability. | `src/types/budget.rs` |
| `pub fn consume_memory(&self, bytes: u64) -> Result<()>` | Attempts to consume `bytes` of memory from the tracked budget. # Errors Returns `Err(ActaError::MemoryBudgetExceeded)` if consuming `bytes`… | `src/types/budget.rs` |
| `pub fn consumed(&self) -> usize` | Returns total tokens consumed so far. | `src/types/budget.rs` |
| `pub fn effective_limit(&self) -> usize` | Calculates the effective token limit based on strategy. | `src/types/budget.rs` |
| `pub fn for_model(model: &str) -> Self` | Erstellt ein Budget für gängige Modelle. # Maintenance Note Model name patterns use string matching. Explicit model family patterns (e.g. `… | `src/types/budget.rs` |
| `pub fn has_memory_capacity(&self) -> bool` | Returns true if memory usage is below 95% of the limit. | `src/types/budget.rs` |
| `pub fn memory_used(&self) -> u64` | Returns current memory usage in bytes. | `src/types/budget.rs` |
| `pub fn new(max_tokens: usize, reserve_tokens: usize) -> Self` | Creates a new token budget with an exact max limit and reserve tokens. | `src/types/budget.rs` |
| `pub fn new(budget: ResourceBudget) -> Self` | Creates a new ResourceTracker with the given budget. | `src/types/budget.rs` |
| `pub fn refund(&self, tokens: usize)` | Refunds `tokens` back to the available budget if execution fails or consumes less than estimated. | `src/types/budget.rs` |
| `pub fn release_memory(&self, bytes: u64)` | Releases memory from the budget. Uses saturating subtraction to prevent atomic underflow (wrapping to `u64::MAX`) which would permanently b… | `src/types/budget.rs` |
| `pub fn reserve(&self, amount: usize) -> Result<Reservation<'_>>` | Reserviert `amount` Tokens atomar. Schlägt fehl, wenn das verfügbare Budget (limit - reserved - consumed) unterschritten würde. | `src/types/budget.rs` |
| `pub fn settle(self)` | Bestätigt den Verbrauch endgültig. Nach `settle()` erfolgt bei Drop KEINE Rückerstattung mehr. # Compile-time Duplicate Call Prevention `se… | `src/types/budget.rs` |
| `pub fn try_reserve(&self, tokens: usize) -> Result<()>` | Tries to reserve `tokens` from the available budget before step execution. Returns `Err(ActaError::Internal(...))` if available budget is 0… | `src/types/budget.rs` |
| `pub fn with_reserved(mut self, system_tokens: usize, answer_tokens: usize) -> Self` | Reserviert Token für System-Prompt und Antwort. | `src/types/budget.rs` |
| `pub fn with_strategy(mut self, strategy: BudgetStrategy) -> Self` | Sets the budget strategy. | `src/types/budget.rs` |
| `pub fn as_slice(&self) -> &[f32]` | Returns the vector data as an `f32` slice. | `src/types/domain/document.rs` |
| `pub fn compute(&self, a: &[f32], b: &[f32]) -> Result<f32>` | Computes the distance between two f32 vectors using this metric. | `src/types/domain/document.rs` |
| `pub fn compute_u8(&self, a: &[u8], b: &[u8]) -> Result<u32>` | — | `src/types/domain/document.rs` |
| `pub fn dim(&self) -> usize` | Returns the dimension (length) of the embedding vector. | `src/types/domain/document.rs` |
| `pub fn l2_norm(&self) -> f32` | Computes the Euclidean L2 norm of the vector. | `src/types/domain/document.rs` |
| `pub fn new(data: Vec<f32>) -> Self` | Creates a new vector embedding from `f32` slice or vector. | `src/types/domain/document.rs` |
| `pub fn new(doc_id: DocId, score: f32) -> Self` | Creates a new `ScoredDocument` with document ID and score. | `src/types/domain/document.rs` |
| `pub fn normalize(&self) -> Self` | Returns an L2-unit-normalized clone of this embedding. Guarded against zero and subnormal/near-zero vector norms (`norm < 1e-12`). Vectors… | `src/types/domain/document.rs` |
| `pub fn as_metadata_key(&self) -> &'static str` | Gibt den kanonischen Metadaten-Key zurück (für JSON-Serialisierung). | `src/types/domain/misc.rs` |
| `pub fn default_decay(&self) -> crate::types::importance::DecayFunction` | Gibt den Standard-Decay-Typ für diesen Gedächtnistyp zurück. | `src/types/domain/misc.rs` |
| `pub fn default_ttl_tx(&self) -> Option<u64>` | Gibt die empfohlene TTL (in Transaktionen) für Session-scoped Working Memory zurück. None bedeutet kein automatisches Ablaufen. | `src/types/domain/misc.rs` |
| `pub fn deserialize<'de, D>(deserializer: D) -> Result<[u8` | — | `src/types/domain/misc.rs` |
| `pub fn is_enabled(self) -> bool` | Returns `true` if auto extraction mode is enabled. | `src/types/domain/misc.rs` |
| `pub fn new(id: EntityId, name: impl Into<String>, entity_type: impl Into<String>) -> Self` | Creates a new `Entity` with id, name, and type. | `src/types/domain/misc.rs` |
| `pub fn new(from: EntityId, to: EntityId, label: impl Into<String>) -> Self` | Creates a new `Edge` between source and target entities with a label. | `src/types/domain/misc.rs` |
| `pub fn new( model_id: impl Into<String>, quantization: impl Into<String>, prompt_template: &str, temperature: f32, ) -> Self` | Erstellt einen neuen `ConfigFingerprint`. | `src/types/domain/misc.rs` |
| `pub fn serialize<S>(bytes: &[u8; 32], serializer: S) -> Result<S::Ok, S::Error> where S: Serializer,` | — | `src/types/domain/misc.rs` |
| `pub fn temperature(&self) -> f32` | Extrahiert Temperatur als f32 (verlustfrei da via to_bits gespeichert). | `src/types/domain/misc.rs` |
| `pub fn threshold(&self) -> f32` | Extrahiert Schwellenwert/Threshold als f32 (verlustfrei da via to_bits gespeichert). | `src/types/domain/misc.rs` |
| `pub fn try_new( id: EntityId, name: impl Into<String>, entity_type: impl Into<String>, ) -> Result<Self>` | Creates a new `Entity`, validating that `name` and `entity_type` are non-empty. # Errors Returns `ActaError::InvalidInput` if `name` or `en… | `src/types/domain/misc.rs` |
| `pub fn try_new( from: EntityId, to: EntityId, label: impl Into<String>, weight: f32, ) -> Result<Self>` | Creates a new `Edge`, validating non-empty label and finite non-negative weight. # Errors Returns `ActaError::InvalidInput` if `label` is e… | `src/types/domain/misc.rs` |
| `pub fn with_business_validity(mut self, from: Option<i64>, to: Option<i64>) -> Self` | Sets business validity window (business time in Unix ms) on the edge. | `src/types/domain/misc.rs` |
| `pub fn with_source_doc_id(mut self, doc_id: DocId) -> Self` | Sets the optional source document ID from which this edge was derived for provenance tracking. | `src/types/domain/misc.rs` |
| `pub fn with_threshold(mut self, threshold: f32) -> Self` | Setzt die Schwellenwert-Komponente (Threshold) für diesen Fingerabdruck (Builder Pattern). | `src/types/domain/misc.rs` |
| `pub fn with_tx_validity(mut self, from: Option<TxId>, to: Option<TxId>) -> Self` | Sets transaction validity window (system time / MVCC) on the edge. | `src/types/domain/misc.rs` |
| `pub fn with_validity(self, from: Option<TxId>, to: Option<TxId>) -> Self` | Sets transaction validity window on the edge (alias for `with_tx_validity` for backward compatibility). | `src/types/domain/misc.rs` |
| `pub fn with_weight(mut self, weight: f32) -> Self` | Sets a custom weight on the edge. | `src/types/domain/misc.rs` |
| `pub fn evaluate(&self, metadata: &serde_json::Value) -> bool` | Evaluates the filter expression against a JSON metadata object. | `src/types/filter.rs` |
| `pub fn decay_factor(&self, created_at_tx: TxId, now_tx: TxId) -> f32` | Pure non-failing decay factor calculation. Returns a factor in `[0.0, 1.0]`. # Invarianten & Edge Cases - If `now_tx < created_at_tx` (e.g.… | `src/types/importance.rs` |
| `pub fn decay_factor_wallclock( &self, created_at_epoch_secs: u64, now_epoch_secs: u64, created_at_tx: TxId, now_tx: TxId, ) -> f32` | Wall-clock based decay factor calculation. Returns a factor in `[0.0, 1.0]` based on elapsed wall-clock seconds. For non-WallClockExponenti… | `src/types/importance.rs` |
| `pub fn effective_score(&self, now_tx: TxId) -> f32` | Computes the effective importance score: `base_score * decay_factor`. # Zero-Panic Guarantee Always returns a valid `f32` in `[0.0, 1.0]`. | `src/types/importance.rs` |
| `pub fn new(raw: f32) -> Self` | Creates a new `ImportanceScore` clamped strictly within `[0.0, 1.0]`. # Zero-Panic Guarantee NaN inputs return `ImportanceScore(0.0)`. Posi… | `src/types/importance.rs` |
| `pub fn new(base_score: ImportanceScore, decay: DecayFunction, created_at_tx: TxId) -> Self` | Creates a new `MemoryImportance` configuration. | `src/types/importance.rs` |
| `pub fn value(&self) -> f32` | Returns the raw `f32` importance score value. | `src/types/importance.rs` |
| `pub fn build(self) -> Result<HybridQuery>` | Builds the `HybridQuery` instance. # Errors Returns `Err` if query parameters or fusion weights fail validation. | `src/types/saos.rs` |
| `pub fn builder() -> HybridQueryBuilder` | Creates a new `HybridQueryBuilder`. | `src/types/saos.rs` |
| `pub fn combined_text_owned(&self) -> String` | Kombinierten Text für Text-Indizierung und Embedding. Entspricht dem etablierten Contextual-Schema: prefix + "\n\n" + content. Allokiert nu… | `src/types/saos.rs` |
| `pub fn combined_token_count(&self) -> usize` | Kombinierter Token-Count (Heuristik: chars/4 ≈ Tokens). Verwendet für Budget-Management in `acta-db/src/context.rs`. | `src/types/saos.rs` |
| `pub fn graph(&self) -> f32` | Returns the graph signal weight component. | `src/types/saos.rs` |
| `pub fn has_context_prefix(&self) -> bool` | Gibt true zurück wenn contextual_prefix gesetzt und nicht leer. | `src/types/saos.rs` |
| `pub fn is_empty(&self) -> bool` | Returns `true` if all reformulation components are empty or None. | `src/types/saos.rs` |
| `pub fn metadata(&self) -> f32` | Returns the metadata signal weight component. Reserved for future metadata-signal fusion; currently always 0.0. Do not expose a public cons… | `src/types/saos.rs` |
| `pub fn new(vector: f32, text: f32, graph: f32) -> Result<Self>` | Creates 3-signal fusion weights (vector, text, graph) summing to 1.0. | `src/types/saos.rs` |
| `pub fn new( text_query: Option<String>, semantic_query: Option<String>, anchor_entities: Vec<String>, ) -> Self` | Creates a new `QueryRewriteOutput` with text, semantic, and anchor entity reformulations. | `src/types/saos.rs` |
| `pub fn new() -> Self` | Creates a new `HybridQueryBuilder` with empty default options. | `src/types/saos.rs` |
| `pub fn strategy_for_signal(&self, signal_name: &str) -> FusionStrategy` | Returns the fusion strategy configured for a specific signal name ("vector", "text", "graph"). | `src/types/saos.rs` |
| `pub fn text(&self) -> f32` | Returns the text signal weight component. | `src/types/saos.rs` |
| `pub fn text_only(text: impl Into<String>) -> Self` | Creates a simple text-only `QueryRewriteOutput`. | `src/types/saos.rs` |
| `pub fn uniform(strategy: FusionStrategy) -> Self` | Creates a uniform configuration where all signals use the specified strategy. | `src/types/saos.rs` |
| `pub fn vector(&self) -> f32` | Returns the vector signal weight component. | `src/types/saos.rs` |
| `pub fn with_filter(mut self, filter: FilterExpr) -> Self` | Sets a metadata expression filter on the query. | `src/types/saos.rs` |
| `pub fn with_fusion_strategy(mut self, strategy: impl Into<SignalFusionStrategies>) -> Self` | Sets search fusion strategy per signal or globally (`FusionStrategy` or `SignalFusionStrategies`). | `src/types/saos.rs` |
| `pub fn with_fusion_weights(mut self, weights: FusionWeights) -> Self` | Sets custom signal fusion weights. | `src/types/saos.rs` |
| `pub fn with_graph_start_node(mut self, start: impl Into<String>) -> Self` | Sets the graph start node for graph traversal search. | `src/types/saos.rs` |
| `pub fn with_graph_strategy(mut self, strategy: GraphTraversalStrategy) -> Self` | Sets the graph traversal strategy (Hops or PersonalizedPageRank). | `src/types/saos.rs` |
| `pub fn with_include_provenance(mut self, include: bool) -> Self` | Sets whether to calculate and attach ProvenanceRecord to output results. | `src/types/saos.rs` |
| `pub fn with_include_superseded(mut self, include: bool) -> Self` | Sets whether to include superseded memory chunks (ADR-038). | `src/types/saos.rs` |
| `pub fn with_k(mut self, k: usize) -> Self` | Sets the top-K limit for the query. | `src/types/saos.rs` |
| `pub fn with_memory_type_filter(mut self, types: Vec<MemoryType>) -> Self` | Sets the memory_type pre-RRF filter. | `src/types/saos.rs` |
| `pub fn with_on_signal_failure(mut self, policy: OnSignalFailure) -> Self` | Sets the signal failure policy ([`OnSignalFailure`]) for the query (Spec Teil 6.5). | `src/types/saos.rs` |
| `pub fn with_rerank_pool_max(mut self, max: usize) -> Self` | Sets the upper bound cap for pre-reranking candidate pool expansion. | `src/types/saos.rs` |
| `pub fn with_rerank_pool_multiplier(mut self, multiplier: usize) -> Self` | Sets the candidate pool multiplier for pre-reranking retrieval. | `src/types/saos.rs` |
| `pub fn with_same_community_as(mut self, entity_id: EntityId) -> Self` | Sets the community context entity filter/boost. | `src/types/saos.rs` |
| `pub fn with_text_query(mut self, q: impl Into<String>) -> Self` | Sets the full-text search query string. | `src/types/saos.rs` |
| `pub fn with_vector_query(mut self, v: Vec<f32>) -> Self` | Sets the vector query embedding. | `src/types/saos.rs` |
| `pub fn new(tenant_id: crate::TenantId, tx_id: crate::TxId) -> Self` | Creates a new `RequestContext`. # Parameter - `tenant_id`: Mandantenidentifikator. - `tx_id`: Zugeordnete Transaktions-ID. | `src/witness.rs` |
| `pub fn new(deleted_keys_hash: [u8; 32]) -> Self` | Creates a new `SagaRecord`. # Parameter - `deleted_keys_hash`: BLAKE3-Hash der zu löschenden Schlüssel. | `src/witness.rs` |


---

## `acta-ports`

| Attribut | Wert |
|---|---|
| **Ring** | 2 — Ebene E2 — Integration, Domäne & Orchestrierung |
| **Status** | Fertig |
| **Zweck** | dyn-kompatible Trait-Definitionen (Ports) für Storage, Indizes und Embedder. |
| **Unsafe-Code** | Nein (`#![forbid(unsafe_code)]`) |
| **Interne Abhängigkeiten** | `acta-env`, `acta-types` |
| **Externe Abhängigkeiten (Auswahl)** | `ahash`, `blake3`, `bytes`, `futures-util`, `serde`, `serde_json`, `thiserror` |

### Architekturbeschreibung

Core trait definitions for Acta subsystems.  These traits define the abstract interfaces that concrete implementations must fulfill, enabling modularity and testability.

### Modul- und Re-Export-Oberfläche (lib.rs)

```rust
pub mod attention;
pub mod checkpoint;
pub mod clock;
pub mod conformance;
pub mod embedding;
pub mod graph;
pub mod graph_index;
pub mod id_gen;
pub mod kv;
pub mod kv_bridge_port;
pub mod license;
pub mod lifecycle;
pub mod metrics;
pub mod observability;
pub mod plugin;
pub mod query_rewriter;
pub mod reranker;
pub mod rng;
pub mod scanner;
pub mod storage;
pub mod text_index;
pub mod vector_index;
pub mod vfs;
pub mod key_material;
pub mod env;
pub mod erasure_legacy;
pub mod fs;
pub mod index;
pub mod inference;
pub mod policy;
pub mod contract_tests;
pub mod barrier;
pub mod erasure;
pub mod gate;
pub mod keys;
pub mod ledger;
pub mod observer;
pub mod report_signer;
pub use acta_types::{ error, ids, model_fingerprint, refs, tenant_scope, types, witness, CollectionRef, ConfigFingerprint, ContextChunk, ContextWindow, ActaError, DocId, DocKey, EntityId, ErasureId, FileId, GateId, ModelFingerprint, OperatorRef, QueryRewriteOutput, RequestContext, RerankResult, ResourceBudget, ResourceTracker, Result, RetrievalStrategy, SagaRecord, ScanMethod, ScoredEntry, Seq, StoreId, SubjectRef, TenantId, TenantRef, TenantScopeViolation, TenantScoped, Timestamp, TokenBudget, TxId, WorkflowState, MAX_REF_LEN, };
pub use attention::*;
pub use checkpoint::*;
pub use clock::*;
pub use conformance::*;
pub use embedding::*;
pub use graph::*;
pub use graph_index::*;
pub use id_gen::*;
pub use key_material::KeyMaterialSource;
pub use kv::*;
pub use kv_bridge_port::*;
pub use license::*;
pub use lifecycle::*;
pub use metrics::*;
pub use observability::*;
pub use plugin::*;
pub use query_rewriter::*;
pub use reranker::*;
pub use rng::*;
pub use scanner::{ LayerScanner, LayerWitness, PurgeStats, Purger, ScanSession, ScanTarget, ScanTechnique, WitnessLayer, };
pub use storage::{StorageEngine, StorageStats, MAX_SCAN_MERGE_ACCUMULATOR};
pub use text_index::*;
pub use vector_index::*;
pub use vfs::*;
pub use metrics::{ HistogramSnapshot, InMemoryMetricsSink, MetricKey, MetricSnapshot, NoopMetricsSink, };
pub use barrier::{ BarrierExclusive, BarrierShared, CommitBarrier, CompactionPermit, PurgeBarrier, PurgePermit, };
pub use erasure::{ EraseTarget as PortEraseTarget, ErasureLayer, ErasureTarget, LayerScope, PurgeCx, PurgeOutcome, StorageBackend, };
pub use gate::{GateDecision, KeyBound, KeyRange, RangeGate, ReadGate};
pub use keys::{Envelope, FileKey, FileKeyring, FileKind, RecordSealer, SealCtx};
pub use ledger::{LedgerEntry, LedgerSink, RangeSet};
pub use observer::{ErasureEvent, ErasureObserver, ObservedPhase, TargetKind};
pub use report_signer::ReportSigner;
```

### Feature-Flags

| Flag | Aktiviert |
|---|---|
| `default` | `[]` |
| `conformance` | `["contract-tests"]` |
| `contract-tests` | `[]` |
| `testkit` | `[]` |

### Schnittstellen — Public Traits (43)

**`AttentionExporter`** — *Port implemented by inference backends (Ring 2) to export attention weights from the latest prefill step to the KV-cache eviction worker (Ring 1).* (`src/attention.rs`)

```rust
pub trait AttentionExporter : Send + Sync {
/// Returns accumulated attention weights per token position for the given request ID.
    ///
    /// Returns `None` if no attention weights are tracked or available for the request.
    ///
    /// # Fehler
    /// Keine; liefert `None` falls keine Aufmerksamkeitsgewichte verfügbar sind.
    /// # Crash
    /// Reiner flüchtiger RAM-Statusabruf ohne I/O oder persistente Mutation.
    fn export_attention_weights(&self, request_id: RequestId) -> Option<Vec<f32>>;
}
```

**`Checkpoint`** — *Abstract contract for generating consistent checkpoints.* (`src/checkpoint.rs`)

```rust
pub trait Checkpoint : Send + Sync + 'static {
/// Takes a deterministic snapshot of the current state.
    fn take_snapshot<'a>(&'a self, tx: TxId) -> BoxFuture<'a, Result<WorkflowState>>;

    /// Rolls the state back to the specified checkpoint.
    fn restore<'a>(&'a self, state: &'a WorkflowState) -> BoxFuture<'a, Result<()>>;
}
```

**`CheckpointCoordinator`** — *Unified Checkpoint Coordinator Trait combining named, TxId+seq_no-scoped, persistent checkpoints. # Dyn-Safety Note This trait is intentionally NOT dyn-compatible without specifying the associated type `Meta` (e.g. `Arc<dyn CheckpointCoordinator<Meta = ...>>`…* (`src/checkpoint.rs`)

```rust
pub trait CheckpointCoordinator : Send + Sync + 'static {
/// Type representing checkpoint metadata.
    type Meta: Send + Sync;

    /// Creates and persists a new named checkpoint.
    fn create_named_checkpoint(
        &self,
        name: &str,
        collection_id: &str,
        seq_no: u64,
        tx_id: TxId,
        metadata: serde_json::Value,
    ) -> impl Future<Output = Result<Self::Meta>> + Send;

    /// Restores database state to a named checkpoint.
    fn restore_named_checkpoint(
        &self,
        name: &str,
    ) -> impl Future<Output = Result<Self::Meta>> + Send;

    /// Deletes a checkpoint by name.
    ///
    /// # Fehler
    /// Liefert `ActaError::NotFound` falls der Checkpoint nicht existiert, oder I/O-Fehler.
    /// # Crash
    /// Atomare Manifest-/VFS-Operation; bei Crash vor Abschluss bleibt der Checkpoint erhalten.
    fn drop_named_checkpoint(&self, name: &str) -> impl Future<Output = Result<()>> + Send;

    /// Lists all active checkpoints.
    ///
    /// # Fehler
    /// Liefert I/O- oder Deserialisierungsfehler beim Lesen der Checkpoint-Metadaten.
    /// # Crash
    /// Reiner Lesevorgang aus Manifest oder Verzeichnis; keine Seiteneffekte.
    fn list_named_checkpoints(&self) -> impl Future<Output = Result<Vec<Self::Meta>>> + Send;
}
```

**`CommitBarrier`** — *Barrier trait managing commit phases and exclusive gate operations (SPEC §11.3, D-57, D-78, L-7).* (`src/barrier.rs`)

```rust
pub trait CommitBarrier : Send + Sync {
/// Acquires a shared permit for standard group commit operations.
    ///
    /// # Fehler
    /// Unfehlbar; wartet asynchron auf Commit-Freigabe.
    ///
    /// # Crash
    /// Reines Synchronisations-Token im RAM; cancel-safe.
    fn shared(&self) -> BoxFuture<'_, BarrierShared>;

    /// Acquires an exclusive permit for gate group transitions.
    ///
    /// # Fehler
    /// Unfehlbar; blockiert nachfolgende Commits exklusiv.
    ///
    /// # Crash
    /// Exklusiver Token-Inhaber; cancel-safe.
    fn exclusive(&self) -> BoxFuture<'_, BarrierExclusive>;
}
```

**`CommunityResolver`** — *Contract for resolving graph community assignments for entities.* (`src/graph_index.rs`)

```rust
pub trait CommunityResolver : Send + Sync {
/// Resolves the optional community ID for a given entity.
    fn get_community<'a>(&'a self, entity_id: EntityId) -> BoxFuture<'a, Result<Option<u64>>>;
}
```

**`ContextPreparer`** — *Contract for trimming and preparing context windows tailored to token budgets.* (`src/lifecycle.rs`)

```rust
pub trait ContextPreparer : Send + Sync {
/// Prepares and trims context chunks according to the provided token budget and relevance threshold.
    fn prepare_context(
        &self,
        chunks: Vec<ContextChunk>,
        budget: &TokenBudget,
        relevance_threshold: f32,
    ) -> Result<ContextWindow>;
}
```

**`DriftStatusProvider`** — *Trait for querying Lyapunov drift status from an attached router engine without creating a cyclic dependency (ADR-080).* (`src/observability.rs`)

```rust
pub trait DriftStatusProvider : Send + Sync {
/// Returns the overall drift status string ("stabil", "warnung", "kritisch", or "unbekannt").
    ///
    /// # Fehler
    /// Keine; statischer oder synchron berechneter Statusabruf.
    /// # Crash
    /// Reiner RAM-Abruf ohne persistente Mutation.
    fn overall_drift_status(&self) -> String;
}
```

**`EgressPolicy`** — *Dynamic port trait for evaluating data egress policies.* (`src/policy.rs`)

```rust
pub trait EgressPolicy : Send + Sync {
/// Evaluates if the outgoing content complies with data governance and egress rules.
    ///
    /// # Fehler
    /// Keine; fehlerfrei über `EgressDecision` abgebildet.
    /// # Crash
    /// Reiner Rechenvorgang im Arbeitsspeicher; keine persistente Mutation.
    fn evaluate(&self, ctx: &acta_types::RequestContext, payload: &str) -> EgressDecision;
}
```

**`EmbeddingProvider`** — *Abstraction for all embedding providers (e.g. OllamaEmbedder, OnnxEmbedder).* (`src/embedding.rs`)

```rust
pub trait EmbeddingProvider : Send + Sync + 'static {
/// Returns the provider identifier/name (e.g. for logging/debugging).
    ///
    /// # Fehler
    /// Keine; statische Identifikator-Abfrage.
    /// # Crash
    /// Reiner Statusabruf ohne Mutation oder Seiteneffekte.
    fn provider_name(&self) -> &str;

    /// Generates an embedding vector for the given text.
    fn embed<'a>(&'a self, text: &'a str) -> BoxFuture<'a, Result<Vec<f32>, EmbeddingError>>;

    /// Returns the dimensionality of embeddings generated by this provider.
    ///
    /// # Fehler
    /// Keine; statische Dimensionsabfrage.
    /// # Crash
    /// Reiner Statusabruf ohne Mutation oder Seiteneffekte.
    fn embedding_dim(&self) -> usize;

    /// Generates embeddings for multiple texts.
    fn embed_batch<'a>(
        &'a self,
        texts: &'a [&'a str],
    ) -> BoxFuture<'a, Result<Vec<Vec<f32>>, EmbeddingError>> {
        Box::pin(async move {
            let mut results = Vec::with_capacity(texts.len());
            for text in texts {
                results.push(self.embed(text).await?);
            }
            Ok(results)
        })
    }
}
```

**`ErasureLayer`** — *Primary trait for storage and index subsystems participating in the erasure saga. Follows SPEC §8.2 verbatim. # Invariants - **PO-05 (Read-only witness)**: `witness` accepts only `&Scanner<Armed>` and performs no mutations. - **PO-05 (Idempotency)**: `purge`…* (`src/erasure.rs`)

```rust
pub trait ErasureLayer : Send + Sync {
/// Returns the unique layer identifier.
    ///
    /// # Fehler
    /// Keine; statische Identifikator-Abfrage.
    ///
    /// # Crash
    /// Reiner Statusabruf ohne persistente Seiteneffekte.
    fn id(&self) -> LayerId;

    /// Resolves the concrete scope affected by the erase target before gating.
    ///
    /// # Fehler
    /// Liefert einen Fehler-String falls das EraseTarget im Layer nicht aufgelöst werden kann.
    ///
    /// # Crash
    /// Reiner Berechnungsschritt vor dem Gating; modifiziert keine Daten auf Festplatte.
    fn resolve(&self, t: &EraseTarget) -> BoxFuture<'_, Result<LayerScope, String>>;

    /// Immediate actions prior to gating (e.g. vector surrogate marking).
    ///
    /// # Fehler
    /// Liefert einen Fehler-String falls Vorbereitungsmaßnahmen vor dem Gate fehlschlagen.
    ///
    /// # Crash
    /// Bereitet RAM-Zustand vor; persistente Markierung erfolgt im Gruppen-Commit.
    fn on_gate(&self, s: &LayerScope) -> BoxFuture<'_, Result<(), String>>;

    /// Executes the physical purge for the given scope within the operational context.
    ///
    /// Must be idempotent and repeatable (PO-05).
    ///
    /// # Fehler
    /// Liefert Fehler bei I/O- oder Kompaktionsfehlern während des physischen Löschens.
    ///
    /// # Crash
    /// Idempotent (PO-05). Jeder Teilschritt ist wiederholbar. fsync-Fehler führen zu Poisoning.
    fn purge(&self, s: &LayerScope, cx: &PurgeCx) -> BoxFuture<'_, Result<PurgeOutcome, String>>;

    /// Verifies residue and produces a witness. Read-only (PO-05).
    ///
    /// # Fehler
    /// Liefert Fehler falls Residuen des gelöschten Umfangs gefunden werden oder I/O fehlschlägt.
    ///
    /// # Crash
    /// Reiner Lesezugriff (PO-05); modifiziert niemals persistenten Zustand.
    fn witness(
        &self,
        s: &LayerScope,
        sc: &Scanner<Armed>,
    ) -> BoxFuture<'_, Result<LayerWitness, String>>;

    /// Reports the key coverage classification (full | unlinked | none).
    ///
    /// # Fehler
    /// Keine; statische Klassifikationsabfrage.
    ///
    /// # Crash
    /// Reiner Statusabruf ohne Mutation.
    fn key_coverage(&self) -> KeyCoverage;
}
```

**`ErasureObserver`** — *Port trait for asynchronous delivery of erasure notifications (SPEC §3.1, §8.2, D-118). Implementations must be object-safe and dyn-compatible as `Arc<dyn ErasureObserver>`.* (`src/observer.rs`)

```rust
pub trait ErasureObserver : Send + Sync + 'static {
/// Notifies the observer about an erasure event.
    ///
    /// # Fehler
    /// Liefert [`ObserverError`], wenn die Benachrichtigung scheitert. Die aufrufende
    /// Hintergrundaufgabe führt Wiederholungen mit exponentiellem Backoff durch (D-118).
    ///
    /// # Crash
    /// Quittungen (`ObserverAck`) werden erst nach erfolgreicher Rückgabe im Journal
    /// persistiert; nicht quittierte Ereignisse werden nach Neustart erneut zugestellt.
    fn on_erased(
        &self,
        ev: ErasureEvent,
    ) -> BoxFuture<'static, std::result::Result<(), ObserverError>>;
}
```

**`ErasureTarget`** — *Generisches Erasure-Target für Bereinigungsanweisungen.* (`src/erasure.rs`)

```rust
pub trait ErasureTarget : Send + Sync + 'static {

}
```

**`FileKeyring`** — *File keyring interface managing per-file cryptographic keys (SPEC §4.1, §8.2, D-85, PO-06). # Contract (PO-06) `destroy` zeroes the key slot, executes `sync_data`, and verifies the key's non-presence via a read-check; failure leaves the slot in `Pending` requ…* (`src/keys.rs`)

```rust
pub trait FileKeyring : Send + Sync {
/// Allocates and records a new file key for the given kind and ID.
    ///
    /// # Fehler
    /// Liefert Fehler falls Slot-Erzeugung oder CSPRNG fehlschlägt.
    ///
    /// # Crash
    /// Persistiert Slot vor Dateierzeugung (I-9b).
    fn create(&self, kind: FileKind, id: FileId) -> BoxFuture<'_, Result<FileKey, String>>;

    /// Opens and retrieves the file key for the given ID.
    ///
    /// # Fehler
    /// Liefert Fehler falls Dateischlüssel fehlt oder vernichtet wurde.
    ///
    /// # Crash
    /// Reines Laden des Dateischlüssels aus dem Tresor.
    fn open(&self, id: FileId) -> BoxFuture<'_, Result<FileKey, String>>;

    /// Securely destroys the file key slot (PO-06).
    ///
    /// # Fehler
    /// Liefert Fehler falls Slot-Nullung oder Sync fehlschlägt.
    ///
    /// # Crash
    /// Nullt Slot, führt sync_data aus und prüft Fehlen; bei fsync-Fehler verbleibt Slot in Pending (PO-06, D-55).
    fn destroy(&self, id: FileId) -> BoxFuture<'_, Result<(), String>>;
}
```

**`GraphCollectionMutation`** — *Trait defining n-ary hyperedge collection mutations for graph indexing (Anhang B §B.5.1.3). # Dyn-Kompatibilität Dieser Trait ist durch synchrone Rückgabetypen vtable-kompatibel (dyn-safe).* (`src/graph.rs`)

```rust
pub trait GraphCollectionMutation : Send + Sync + 'static {
/// Relates multiple entities into an n-ary hyperedge associated with a document ID.
    fn relate_n_ary(
        &self,
        predicate_tag: u32,
        participants: &[RoleBinding],
        doc_id: DocId,
    ) -> Result<HyperEdgeId, GraphMutationError>;
}
```

**`GraphIndex`** — *Graph-Index Trait — CSR-basierte Entity-Relation-Traversal. # Dyn-Kompatibilität Dieser Trait ist durch explizite `BoxFuture`-Rückgabetypen vtable-kompatibel (dyn-safe). # TxId-Origin-Invariant (AGT-GRAPH-001) **Aufrufer MÜSSEN tx entweder aus der Collection-…* (`src/graph_index.rs`)

```rust
pub trait GraphIndex : Send + Sync + 'static {
/// Traverses the entity graph using BFS up to a maximum number of hops.
    /// Distributes traversing decay weights across related entities.
    fn traverse<'a>(
        &'a self,
        start_node: EntityId,
        max_hops: usize,
    ) -> BoxFuture<'a, Result<Vec<(EntityId, f32)>>>;

    /// Returns direct (1-hop) neighbor EntityIds for the given entity.
    fn neighbors<'a>(&'a self, start_node: EntityId) -> BoxFuture<'a, Result<Vec<EntityId>>> {
        Box::pin(async move {
            let results = self.traverse(start_node, 1).await?;
            Ok(results.into_iter().map(|(id, _)| id).collect())
        })
    }

    /// Removes an edge between two entities.
    fn remove_edge<'a>(
        &'a self,
        tx: TxId,
        from: EntityId,
        to: EntityId,
    ) -> BoxFuture<'a, Result<()>> {
        Box::pin(async move {
            let _ = (tx, from, to);
            Ok(())
        })
    }

    /// Adds a bidirectional edge between two entities.
    fn add_bidirectional<'a>(
        &'a self,
        tx: TxId,
        from: EntityId,
        to: EntityId,
        label: &'a str,
    ) -> BoxFuture<'a, Result<()>> {
        Box::pin(async move {
            self.add_edge(tx, Edge::new(from, to, label)).await?;
            self.add_edge(tx, Edge::new(to, from, label)).await?;
            Ok(())
        })
    }

    /// Traverses the entity graph starting from multiple anchor entities up to max_hops.
    /// Aggregates decay weights (keeping max score per entity) across anchors.
    fn multi_traverse<'a>(
        &'a self,
        start_nodes: &'a [EntityId],
        max_hops: usize,
    ) -> BoxFuture<'a, Result<Vec<(EntityId, f32)>>> {
        Box::pin(async move {
            let mut combined: AHashMap<EntityId, f32> = AHashMap::default();
            for &start in start_nodes {
                let results = self.traverse(start, max_hops).await?;
                for (entity_id, score) in results {
                    combined
                        .entry(entity_id)
                        .and_modify(|s| *s = s.max(score))
                        .or_insert(score);
                }
            }
            let mut results: Vec<(EntityId, f32)> = combined.into_iter().collect();
            results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
            Ok(results)
        })
    }

    /// Traverses the entity graph starting from multiple anchor entities up to max_hops at a specific sequence number.
    /// Aggregates decay weights (keeping max score per entity) across anchors.
    fn multi_traverse_at<'a>(
        &'a self,
        start_nodes: &'a [EntityId],
        max_hops: usize,
        seq_no: u64,
    ) -> BoxFuture<'a, Result<Vec<(EntityId, f32)>>> {
        Box::pin(async move {
            let mut combined: AHashMap<EntityId, f32> = AHashMap::default();
            for &start in start_nodes {
                let results = self.traverse_at(start, max_hops, seq_no).await?;
                for (entity_id, score) in results {
                    combined
                        .entry(entity_id)
                        .and_modify(|s| *s = s.max(score))
                        .or_insert(score);
                }
            }
            let mut results: Vec<(EntityId, f32)> = combined.into_iter().collect();
            results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
            Ok(results)
        })
    }

    /// Executes PathRAG graph traversal starting from multiple anchor entities up to `max_hops` at a specific sequence number (`seq_no`).
    ///
    /// # Contract
    /// - The result is the set of entities along with their relevance scores visible at sequence number `seq_no`.
    /// - Results are sorted by score descending, with ties broken by EntityId ascending.
    /// - A backend implementation without snapshot support returns the default [`ActaError::SnapshotUnsupportedForSignal`][crate::error::ActaError::SnapshotUnsupportedForSignal] error.
    ///
    /// # Errors
    /// Returns [`ActaError::SnapshotUnsupportedForSignal`][crate::error::ActaError::SnapshotUnsupportedForSignal]
    /// if snapshot-isolated PathRAG traversal is not implemented by the backend.
    fn path_rag_at<'a>(
        &'a self,
        _start_nodes: &'a [EntityId],
        _max_hops: usize,
        _seq_no: u64,
    ) -> BoxFuture<'a, Result<Vec<(EntityId, f32)>>> {
        Box::pin(async move {
            Err(
                crate::error::ActaError::snapshot_unsupported_for_signal(
                    "PathRag strategy does not support snapshot-isolated retrieval",
                ),
            )
        })
    }

    /// Traverses the entity graph using BFS up to a maximum number of hops at a specific sequence number.
    ///
    /// # Errors
    /// Returns [`ActaError::CapabilityUnsupported`][crate::ActaError::CapabilityUnsupported]
    /// with capability `"graph_traverse_at"` if snapshot-isolated graph traversal is not implemented.
    /// Tested via `capability_coverage` test module.
    fn traverse_at<'a>(
        &'a self,
        _start_node: EntityId,
        _max_hops: usize,
        _seq_no: u64,
    ) -> BoxFuture<'a, Result<Vec<(EntityId, f32)>>> {
        Box::pin(async move {
            Err(crate::error::ActaError::capability_unsupported(
                "graph_traverse_at",
                "Graph traversal snapshot isolation (traverse_at) is not supported by default — tracked in ADR-024",
            ))
        })
    }

    /// Calculates Personalized PageRank (PPR) starting from seed nodes on the graph state visible at a specific sequence number (`seq_no`).
    ///
    /// # Errors
    /// Returns [`ActaError::CapabilityUnsupported`][crate::ActaError::CapabilityUnsupported]
    /// with capability `"graph_ppr_snapshot"` if snapshot-isolated Personalized PageRank is not implemented.
    /// Tested via `capability_coverage` test module.
    fn personalized_page_rank_at<'a>(
        &'a self,
        _seed_nodes: &'a [EntityId],
        _config: &'a PprConfig,
        _seq_no: u64,
    ) -> BoxFuture<'a, Result<Vec<(EntityId, f32)>>> {
        Box::pin(async move {
            Err(crate::error::ActaError::capability_unsupported(
                "graph_ppr_snapshot",
                "Personalized PageRank snapshot isolation (personalized_page_rank_at) is not supported by default — tracked in ADR-024",
            ))
        })
    }

    /// Traverses the entity graph using BFS at a specific point in time (bi-temporal edge filtering).
    ///
    /// # Errors
    /// Returns [`ActaError::CapabilityUnsupported`][crate::ActaError::CapabilityUnsupported]
    /// with capability `"graph_traverse_at_time"` if bi-temporal graph traversal is not implemented.
    /// Tested via `capability_coverage` test module.
    fn traverse_at_time<'a>(
        &'a self,
        _start_node: EntityId,
        _max_hops: usize,
        _as_of: TxId,
    ) -> BoxFuture<'a, Result<Vec<(EntityId, f32)>>> {
        Box::pin(async move {
            Err(crate::error::ActaError::capability_unsupported(
                "graph_traverse_at_time",
                "Bi-temporal graph traversal (traverse_at_time) is not supported by default",
            ))
        })
    }

    /// Traverses the entity graph using BFS with independent system time and business time constraints.
    ///
    /// # Errors
    /// Returns [`ActaError::CapabilityUnsupported`][crate::ActaError::CapabilityUnsupported]
    /// with capability `"graph_traverse_at_bitemporal"` if bitemporal graph traversal is not implemented.
    fn traverse_at_bitemporal<'a>(
        &'a self,
        _start_node: EntityId,
        _max_hops: usize,
        _as_of_tx: TxId,
        _as_of_business: Option<i64>,
    ) -> BoxFuture<'a, Result<Vec<(EntityId, f32)>>> {
        Box::pin(async move {
            Err(crate::error::ActaError::capability_unsupported(
                "graph_traverse_at_bitemporal",
                "Bi-temporal graph traversal (traverse_at_bitemporal) is not supported by default",
            ))
        })
    }

    /// Calculates Personalized PageRank (PPR) starting from seed nodes.
    ///
    /// # Convergence Behavior
    /// Power iteration terminates when the L1 norm difference between iterations drops below `config.convergence_epsilon`,
    /// or when `config.max_iterations` is reached. If `config.max_iterations` is reached without full convergence,
    /// the function returns the best-effort intermediate ranking state (no `Err`) and emits a `tracing::warn!` log entry.
    ///
    /// # Errors
    /// Returns [`ActaError::CapabilityUnsupported`][crate::ActaError::CapabilityUnsupported]
    /// with capability `"graph_ppr"` if Personalized PageRank is not supported by this implementation.
    /// Tested via `capability_coverage` test module.
    fn personalized_page_rank<'a>(
        &'a self,
        _seed_nodes: &'a [EntityId],
        _config: &'a PprConfig,
    ) -> BoxFuture<'a, Result<Vec<(EntityId, f32)>>> {
        Box::pin(async move {
            Err(crate::error::ActaError::capability_unsupported(
                "graph_ppr",
                "Personalized PageRank (personalized_page_rank) is not supported by default for this GraphIndex implementation",
            ))
        })
    }

    /// Inserts or updates a node entity.
    fn add_entity<'a>(&'a self, tx: TxId, entity: Entity) -> BoxFuture<'a, Result<()>>;

    /// Inserts or updates an edge between two entities.
    fn add_edge<'a>(&'a self, tx: TxId, edge: Edge) -> BoxFuture<'a, Result<()>>;

    /// Commits a transaction.
    fn commit<'a>(&'a self, tx: TxId) -> BoxFuture<'a, Result<()>>;

    /// Rolls back a transaction.
    fn rollback<'a>(&'a self, tx: TxId) -> BoxFuture<'a, Result<()>>;

    /// Rolls back the entire graph state to a specific transaction ID.
    fn rollback_to_tx<'a>(&'a self, tx_id: TxId) -> BoxFuture<'a, Result<()>>;

    /// Returns the last transaction ID processed by the index.
    fn last_tx_id<'a>(&'a self) -> BoxFuture<'a, Result<TxId>>;

    /// Returns the number of entities in the index.
    fn len<'a>(&'a self) -> BoxFuture<'a, usize>;

    /// Returns true if the index is empty.
    fn is_empty<'a>(&'a self) -> BoxFuture<'a, bool> {
        Box::pin(async move { self.len().await == 0 })
    }

    /// Collects statistics for the Graph.
    fn stats<'a>(&'a self) -> BoxFuture<'a, Result<GraphIndexStats>>;
}
```

**`GroundingValidator`** — *Abstract contract for post-hoc hallucination / grounding validation.* (`src/lifecycle.rs`)

```rust
pub trait GroundingValidator : Send + Sync {
/// Validates an LLM-generated response against retrieval context chunks.
    ///
    /// Returns `Ok(GroundingAssessment)` if confidence meets threshold,
    /// or `Err(ActaError)` (e.g. `ActaError::PolicyViolation` or low-confidence signal)
    /// on grounding failure or abstention.
    fn validate_grounding<'a>(
        &'a self,
        response: &'a str,
        context_chunks: &'a [ContextChunk],
    ) -> BoxFuture<'a, Result<GroundingAssessment>>;
}
```

**`HybridSearchProvider`** — *Contract for executing hybrid (vector + text) queries for profile routing.* (`src/vector_index.rs`)

```rust
pub trait HybridSearchProvider : Send + Sync {
/// Executes a hybrid query returning matched context chunks with relevance scores.
    fn search_hybrid<'a>(
        &'a self,
        query_text: &'a str,
        query_embedding: &'a [f32],
        top_k: usize,
    ) -> BoxFuture<'a, Result<Vec<ContextChunk>>>;
}
```

**`KeyMaterialSource`** — *Port-Trait für kryptografisch sichere Schlüssel- und Salt-Generierung. Implementierungen **müssen** fail-closed sein: Bei Entropieerschöpfung oder OS-Fehler wird ein Fehler zurückgegeben, nie ein unsicherer Fallback verwendet. # Trennung von `Rng` Dieser Trai…* (`src/key_material.rs`)

```rust
pub trait KeyMaterialSource : Send + Sync + Debug + 'static {
/// Füllt `dest` mit kryptografisch sicheren Zufallsbytes.
    ///
    /// # Errors
    /// Gibt `ActaError::Storage` zurück bei OS-Entropieerschöpfung oder Plattformfehler.
    /// Kein Fallback auf schwächere Quellen.
    fn fill_key_bytes(&self, dest: &mut [u8]) -> Result<()>;
}
```

**`KvBridgeStorage`** — *Abstrahiertes Persistence-Interface für KV-Bridge Tier-2 Spill-over operations. Implementierbar von Ring-1 Storage Engines (`acta-store::LsmStorage`) oder Test-Mocks.* (`src/kv_bridge_port.rs`)

```rust
pub trait KvBridgeStorage : Send + Sync {
/// Versucht, einen Wert unter dem angegebenen Schlüssel abzufragen.
    fn get<'a>(&'a self, key: &'a [u8]) -> BoxFuture<'a, Result<Option<Bytes>>>;

    /// Schreibt einen Schlüssel-Wert-Paar im Kontext einer Transaktion (`tx_id`).
    fn put<'a>(&'a self, tx_id: TxId, key: &'a [u8], value: &'a [u8]) -> BoxFuture<'a, Result<()>>;

    /// Committet die Transaktion mit der ID `tx_id`.
    fn commit<'a>(&'a self, tx_id: TxId) -> BoxFuture<'a, Result<()>>;
}
```

**`KvPrefixStore`** — *Trait defining the contract for KV prefix caching and block reuse (Spec §9.2). # Dyn-Kompatibilität Dieser Trait ist durch synchrone Schnittstellen vtable-kompatibel (dyn-safe).* (`src/kv.rs`)

```rust
pub trait KvPrefixStore : Send + Sync + 'static {
/// Returns the longest exact matching prefix in blocks for the prompt tokens, if any.
    ///
    /// # Fehler
    /// Keine; liefert `None` falls kein übereinstimmendes Präfix gefunden wurde.
    /// # Crash
    /// Reiner Lesevorgang aus dem Speicher oder Cache ohne persistente Mutation.
    fn lookup(&self, tenant: TenantId, key: &PrefixKey, tokens: &[u32]) -> Option<KvPrefixHit>;

    /// Inserts KV blocks for a given token prefix into the store.
    fn insert(
        &self,
        tenant: TenantId,
        key: &PrefixKey,
        tokens: &[u32],
        blocks: Vec<KvBlock>,
    ) -> Result<(), ActaError>;

    /// Evicts KV prefix entries for the given tenant and prefix key.
    /// Returns the number of blocks evicted.
    ///
    /// # Fehler
    /// Liefert `ActaError` bei I/O- oder Freigabefehlern.
    /// # Crash
    /// Idempotente Eviction; Teilausfall hinterlässt entweder alten oder eviktierten Zustand.
    fn evict(&self, tenant: TenantId, key: &PrefixKey) -> Result<u64, ActaError>;
}
```

**`LayerScanner`** — *Trait für Storage-Scanner zur Durchführung von Restmengen-Messungen. # Doctests (Compile Fail Checks) (a) `LayerWitness` lässt sich außerhalb des Moduls nicht direkt konstruieren: ```compile_fail use acta_ports::scanner::{LayerWitness, WitnessLayer, ScanTechn…* (`src/scanner/mod.rs`)

```rust
pub trait LayerScanner : Send + Sync {
/// Gibt die von diesem Scanner bediente Schicht zurück.
    ///
    /// # Fehler
    /// Keine; statische Schichtabfrage.
    /// # Crash
    /// Reiner Statusabruf ohne Mutation.
    fn layer(&self) -> WitnessLayer;

    /// Führt das Scannen von Einheiten auf der übergebenen `ScanSession` aus.
    ///
    /// # Fehler
    /// Liefert `ActaError` bei I/O- oder Verifizierungsfehlern.
    /// # Crash
    /// Reiner Lese- und Messzugriff auf Speicherschichten; verändert niemals persistente Daten.
    fn scan_units(&self, session: &mut ScanSession, ctx: &crate::RequestContext) -> Result<()>;

    /// Default-Implementierung von `scan`, die `begin`, `scan_units` und `finish` ausführt.
    fn scan(
        &self,
        ctx: &acta_types::RequestContext,
        target: ScanTarget,
        technique: ScanTechnique,
        clock: &dyn Clock,
    ) -> Result<LayerWitness> {
        let mut session = ScanSession::begin(self.layer(), target, technique);
        self.scan_units(&mut session, ctx)?;
        session.finish(clock)
    }
}
```

**`LedgerSink`** — *Durable audit ledger sink located outside the primary data directory (SPEC §4.2, §11.8).* (`src/ledger.rs`)

```rust
pub trait LedgerSink : Send + Sync {
/// Appends a new verified erasure entry to the ledger.
    ///
    /// # Fehler
    /// Liefert Fehler bei Schreib- oder MAC-Validierungsfehlern.
    ///
    /// # Crash
    /// Schreibt dauerhaft in das Ledger-Log; fsync stellt Dauerhaftigkeit sicher.
    fn append(&self, e: &LedgerEntry) -> Result<(), String>;

    /// Reads all entries recorded in the ledger for verification or auditing.
    ///
    /// # Fehler
    /// Liefert Fehler bei I/O- oder Integritätsbrüchen der Hashkette.
    ///
    /// # Crash
    /// Reiner Lesezugriff zur Audit-Prüfung.
    fn read_all(&self) -> Result<Vec<LedgerEntry>, String>;

    /// Prunes entries older than the specified retention threshold.
    ///
    /// # Fehler
    /// Liefert Fehler bei I/O-Fehlern beim Kürzen.
    ///
    /// # Crash
    /// Kürzt veraltete Ledger-Einträge atomar.
    fn prune(&self, older_than: Timestamp) -> Result<usize, String>;
}
```

**`LicenseGate`** — *Dyn-kompatibler Port (P27) für Lizenzprüfung — Implementierung (Lizenzserver-Anbindung, Ed25519-Signaturprüfung analog zum Signaturprüfpfad §14.1) ist NICHT Teil dieses Tickets und folgt in einem eigenen, separat zu beauftragenden Schritt.* (`src/license.rs`)

```rust
pub trait LicenseGate : Send + Sync {
/// Checks if the requested feature ring is activated and accessible.
    ///
    /// # Fehler
    /// Liefert `LicenseError::NotActivated`, `LicenseError::InvalidSignature` oder `LicenseError::Expired`.
    /// # Crash
    /// Reiner Lesevorgang von Lizenz-Zertifikaten oder RAM-Zustand; keine Seiteneffekte.
    fn check_ring(&self, ring: FeatureRing) -> Result<(), LicenseError>;

    /// Authorizes the requested feature ring and yields an [`AuthorizedRing`] token on success.
    ///
    /// # Fehler
    /// Liefert `LicenseError` falls die Autorisierungsprüfung fehlschlägt.
    /// # Crash
    /// Reiner Statusabruf und Token-Ausstellung ohne persistente Mutation.
    fn authorize(&self, requested: FeatureRing) -> Result<AuthorizedRing, LicenseError> {
        self.check_ring(requested)?;
        Ok(AuthorizedRing::new(requested))
    }
}
```

**`LlmTextGenerator`** — *Abstract contract for LLM text generation (summarization, importance evaluation, query expansion).* (`src/embedding.rs`)

```rust
pub trait LlmTextGenerator : Send + Sync + 'static {
/// Generates text for a given prompt using an LLM.
    fn generate<'a>(&'a self, prompt: &'a str) -> BoxFuture<'a, crate::Result<String>>;

    /// Generates text for a given sequence of context segments using an LLM.
    ///
    /// Default implementation naively concatenates segment texts with `"\n\n"` and invokes `self.generate(&concatenated)`.
    fn generate_with_context<'a>(
        &'a self,
        tenant: TenantId,
        segments: &'a [ContextSegment<'a>],
    ) -> BoxFuture<'a, crate::Result<String>> {
        Box::pin(async move {
            let _ = tenant;
            let concatenated = segments
                .iter()
                .map(|s| s.text)
                .collect::<Vec<_>>()
                .join("\n\n");
            self.generate(&concatenated).await
        })
    }
}
```

**`LlmTextGeneratorStreaming`** — *Abstract contract for streaming LLM text generation (token by token / chunk by chunk).* (`src/embedding.rs`)

```rust
pub trait LlmTextGeneratorStreaming : LlmTextGenerator {
/// Generates text stream for a given prompt using an LLM configuration.
    fn generate_stream<'a>(
        &'a self,
        prompt: &'a str,
        config: &'a ConfigFingerprint,
    ) -> super::BoxStream<'a, crate::Result<String>>;
}
```

**`PluginManifest`** — *Manifest trait implemented by dynamic or static plugins.* (`src/plugin.rs`)

```rust
pub trait PluginManifest : Send + Sync {
/// Returns the name of the plugin.
    ///
    /// # Fehler
    /// Keine; statische Namensabfrage.
    /// # Crash
    /// Reiner Statusabruf ohne Mutation.
    fn name(&self) -> &str;

    /// Returns the list of plugin names required prior to activation.
    ///
    /// # Fehler
    /// Keine; statische Referenzliste.
    /// # Crash
    /// Reiner Statusabruf ohne Mutation.
    fn requires(&self) -> &[&str] {
        &[]
    }

    /// Returns the list of plugin names that conflict with this plugin.
    ///
    /// # Fehler
    /// Keine; statische Referenzliste.
    /// # Crash
    /// Reiner Statusabruf ohne Mutation.
    fn conflicts(&self) -> &[&str] {
        &[]
    }

    /// Returns the required feature ring for this plugin.
    ///
    /// # Fehler
    /// Keine; statischer Feature-Ring-Abruf.
    /// # Crash
    /// Reiner Statusabruf ohne Mutation.
    fn feature_ring_required(&self) -> FeatureRing {
        FeatureRing::Fast
    }

    /// Returns the static capability metadata for this plugin.
    ///
    /// # Fehler
    /// Keine; liefert statischen Deskriptor.
    /// # Crash
    /// Reiner Statusabruf ohne Mutation.
    fn capability(&self) -> PluginCapability;

    /// Activates the plugin instance.
    ///
    /// # Fehler
    /// Liefert `PluginError` bei Initialisierungs-, Berechtigungs- oder Ressourcenfehlern.
    /// # Crash
    /// Idempotente Aktivierung im RAM; Fehler machen Zustandsänderungen rückgängig.
    fn activate(&self) -> Result<(), PluginError>;
}
```

**`PurgeBarrier`** — *Barrier trait managing LSM compaction and physical purge mutual exclusion (SPEC §11.3, §11.4, I-16, L-7).* (`src/barrier.rs`)

```rust
pub trait PurgeBarrier : Send + Sync {
/// Acquires a shared permit for compaction.
    ///
    /// # Fehler
    /// Unfehlbar; wartet auf Purge-Freigabe.
    ///
    /// # Crash
    /// Kooperative Kompaktions-Sperre im RAM.
    fn compaction(&self) -> BoxFuture<'_, CompactionPermit>;

    /// Acquires an exclusive permit for physical purging.
    ///
    /// # Fehler
    /// Unfehlbar; exklusiver Purge-Zugriff.
    ///
    /// # Crash
    /// Erzwingt Abbruch kooperativer Kompaktionen; cancel-safe.
    fn purge(&self) -> BoxFuture<'_, PurgePermit>;

    /// Queries whether an exclusive purge operation is waiting for access.
    ///
    /// Background compactions inspect this periodically to cooperatively yield or abort.
    ///
    /// # Fehler
    /// Keine; nicht-blockierende atomare Abfrage.
    ///
    /// # Crash
    /// Reiner RAM-Zustandsprüfer.
    fn purge_waiting(&self) -> bool;
}
```

**`Purger`** — *Trait für physische Bereinigungsaktionen (Purger) einer Schicht.* (`src/scanner/layer.rs`)

```rust
pub trait Purger : Send + Sync {
/// Führt die physische Bereinigung für das gegebene Ziel aus.
    ///
    /// # Fehler
    /// Liefert `ActaError` bei I/O- oder Freigabefehlern während der physischen Bereinigung.
    /// # Crash
    /// Idempotent; Teilausfälle hinterlassen konsistente Zwischenzustände ohne Datenverlust lebender Einheiten.
    fn purge(&self, ctx: &crate::RequestContext, target: &ScanTarget) -> Result<PurgeStats>;
}
```

**`QueryRewriter`** — *Trait for query rewriting in multi-step iterative retrieval (LLM-agnostic).* (`src/query_rewriter.rs`)

```rust
pub trait QueryRewriter : Send + Sync {
/// Generates alternative text sub-queries based on original query and previous search results.
    ///
    /// For backward compatibility, default implementation calls `rewrite_structured` and collects non-empty text queries.
    fn rewrite<'a>(
        &'a self,
        original_query: &'a str,
        current_results: &'a [ScoredEntry],
    ) -> BoxFuture<'a, Result<Vec<String>>> {
        Box::pin(async move {
            let outputs = self
                .rewrite_structured(original_query, current_results)
                .await?;
            let text_queries = outputs
                .into_iter()
                .filter_map(|out| out.text_query)
                .filter(|q| !q.trim().is_empty())
                .collect();
            Ok(text_queries)
        })
    }

    /// Generates multi-signal query reformulations (text, semantic, and graph anchors) for multi-step retrieval.
    ///
    /// Default implementation delegates to `rewrite` and wraps the resulting string queries as `QueryRewriteOutput::text_only`.
    fn rewrite_structured<'a>(
        &'a self,
        original_query: &'a str,
        current_results: &'a [ScoredEntry],
    ) -> BoxFuture<'a, Result<Vec<QueryRewriteOutput>>> {
        Box::pin(async move {
            let queries = self.rewrite(original_query, current_results).await?;
            let outputs = queries
                .into_iter()
                .map(QueryRewriteOutput::text_only)
                .collect();
            Ok(outputs)
        })
    }
}
```

**`ReadGate`** — *Synchronous, memory-only read gate port interface (SPEC §4.2, §11.3, PO-07). # Invariants - **PO-07**: `ReadGate` is synchronous, performs zero I/O, and executes in $O(\log n)$ time.* (`src/gate.rs`)

```rust
pub trait ReadGate : Send + Sync {
/// Checks point status of an individual key.
    ///
    /// # Fehler
    /// Synchron, keine I/O, panikfrei; liefert GateDecision.
    ///
    /// # Crash
    /// Reiner RAM-Lookup in O(log n); keine Zustandsänderung.
    fn check(&self, k: &KeyBuf) -> GateDecision;

    /// Checks boundary status of a file or partition block for compaction or scans.
    ///
    /// # Fehler
    /// Synchron, keine I/O; liefert GateDecision.
    ///
    /// # Crash
    /// Reiner RAM-Lookup; keine Seiteneffekte.
    fn check_bound(&self, b: &KeyBound) -> GateDecision;

    /// Evaluates range policy for compaction (I-18: Erased may be dropped, Restricted never).
    ///
    /// # Fehler
    /// Synchron, keine I/O; liefert RangeGate.
    ///
    /// # Crash
    /// Reiner RAM-Lookup für Kompaktions-Entscheidungen.
    fn variant_for_range(&self, r: &KeyRange) -> RangeGate;

    /// Retrieves the sequence number at which the key was gated, if any (Restore-Guard, commit rejection).
    ///
    /// # Fehler
    /// Synchron, liefert `Option<Seq>`.
    ///
    /// # Crash
    /// Reiner RAM-Lookup; keine Mutation.
    fn gated_seq(&self, k: &KeyBuf) -> Option<Seq>;

    /// Returns the monotonic gate epoch counter (SPEC §11.3, D-120, I-21).
    ///
    /// Incremented before key destruction to allow linearized read verification.
    ///
    /// # Fehler
    /// Keine; synchroner atomarer Abruf.
    /// # Crash
    /// Reiner RAM-Zugriff ohne Mutation.
    fn gate_epoch(&self) -> u64 {
        0
    }

    /// Evaluates whether a 16-byte `KeyBuf` is covered by an active gate (Erased or Restricted) (SPEC I-2).
    ///
    /// Used by replay filters and commit leader phase.
    ///
    /// # Fehler
    /// Keine; synchroner ram-basierter Lookup.
    /// # Crash
    /// Reiner RAM-Zugriff ohne Mutation.
    fn covers(&self, k: &KeyBuf) -> bool {
        match self.check(k) {
            GateDecision::Erased | GateDecision::Restricted => true,
            GateDecision::Open => false,
        }
    }

    /// Evaluates point gate status of a canonical 78-byte raw key (SPEC 9.1, D-123).
    ///
    /// # Fehler
    /// Keine; synchroner ram-basierter Lookup.
    /// # Crash
    /// Reiner RAM-Zugriff ohne Mutation.
    fn check_raw(&self, _raw: &[u8; 78]) -> GateDecision {
        GateDecision::Open
    }

    /// Evaluates whether a canonical 78-byte raw key is covered by an active gate (SPEC I-2, I-21, D-123).
    ///
    /// # Fehler
    /// Keine; synchroner ram-basierter Lookup.
    /// # Crash
    /// Reiner RAM-Zugriff ohne Mutation.
    fn covers_raw(&self, raw: &[u8; 78]) -> bool {
        match self.check_raw(raw) {
            GateDecision::Erased | GateDecision::Restricted => true,
            GateDecision::Open => false,
        }
    }

    /// Retrieves the sequence number at which a 78-byte raw key was gated, if any.
    ///
    /// # Fehler
    /// Keine; synchroner ram-basierter Lookup.
    /// # Crash
    /// Reiner RAM-Zugriff ohne Mutation.
    fn gated_seq_raw(&self, _raw: &[u8; 78]) -> Option<Seq> {
        None
    }
}
```

**`RecordSealer`** — *Record sealer contract for crypto envelope management (SPEC §4.2, D-53).* (`src/keys.rs`)

```rust
pub trait RecordSealer : Send + Sync {
/// Encrypts and seals plaintext with slot allocation if missing.
    ///
    /// # Fehler
    /// Liefert Fehler falls Verschlüsselung oder Slot-Allokation fehlschlägt.
    ///
    /// # Crash
    /// Vorübergehende Daten im RAM; Persistierung der Slots erfolgt vor dem Commit.
    fn seal<'a>(
        &'a self,
        ctx: &'a SealCtx,
        key: &'a KeyBuf,
        plain: &'a [u8],
    ) -> BoxFuture<'a, Result<Envelope, String>>;

    /// Decrypts envelope. If slot was destroyed/missing, returns error indicating erasure.
    ///
    /// # Fehler
    /// Liefert Fehler falls Schlüssel vernichtet wurde oder Entschlüsselung fehlschlägt.
    ///
    /// # Crash
    /// Reiner kryptografischer Entschlüsselungszugriff ohne Seiteneffekte.
    fn open<'a>(&'a self, key: &'a KeyBuf, env: &'a [u8])
        -> BoxFuture<'a, Result<Vec<u8>, String>>;

    /// Evicts key range from DEK cache, zeroizing RAM keys (D-79).
    ///
    /// # Fehler
    /// Synchrone Operation ohne Fehlerzustände.
    ///
    /// # Crash
    /// Entfernt und nullt Schlüsselmaterial flüchtig im RAM.
    fn evict(&self, scope: &KeyRange);

    /// Forces persistence of vault slots (`sync_data`) before WAL frame flush (I-9, L-9).
    ///
    /// # Fehler
    /// Liefert Fehler bei I/O- oder fsync-Fehlern.
    ///
    /// # Crash
    /// Erzwingt Slot-Dauerhaftigkeit via sync_data; bei fsync-Fehler tritt Poisoning ein (I-9, L-9).
    fn make_slots_durable(&self) -> BoxFuture<'_, Result<(), String>>;
}
```

**`ReportSigner`** — *Abstract port trait for signing audit reports (SPEC §8.2, D-130, T-SIG-1).* (`src/report_signer.rs`)

```rust
pub trait ReportSigner : Send + Sync {
/// Returns the unique numeric signer identifier according to SPEC F.10.
    ///
    /// # Fehler
    /// Keine; statische Signer-ID-Abfrage.
    /// # Crash
    /// Reiner Statusabruf ohne persistente Mutation.
    fn signer_id(&self) -> SignerId;

    /// Signs the canonical report bytes.
    ///
    /// # Parameter
    /// - `canonical_report_bytes`: canonical JSON representation according to SPEC F.10.
    ///
    /// # Fehler
    /// Liefert [`SignerError`], wenn das kryptografische Signieren scheitert.
    ///
    /// # Crash
    /// Reines Berechnungs- oder HSM-Protokoll ohne ungesicherten Seiteneffekt.
    fn sign_report<'a>(
        &'a self,
        canonical_report_bytes: &'a [u8],
    ) -> BoxFuture<'a, std::result::Result<ReportSignature, SignerError>>;
}
```

**`Reranker`** — *Trait for post-retrieval cross-encoder reranking models.* (`src/reranker.rs`)

```rust
pub trait Reranker : Send + Sync {
/// Reranks candidates for a given query text.
    fn rerank<'a>(
        &'a self,
        query: &'a str,
        candidates: &'a [String],
    ) -> BoxFuture<'a, Result<Vec<RerankResult>>>;

    /// Records implicit feedback or outcome labels for calibration.
    ///
    /// # Fehler
    /// Keine; panikfrei und nicht-blockierend.
    /// # Crash
    /// Reiner flüchtiger RAM-Zustand oder No-Op; keine Seiteneffekte.
    fn record_implicit_feedback(&self, _results: &[RerankResult], _implicit_relevant_k: usize) {}

    /// Returns the maximum allowed execution time in milliseconds before timing out, if configured.
    ///
    /// # Fehler
    /// Keine; statische Deadline-Abfrage.
    /// # Crash
    /// Reiner Statusabruf ohne persistente Mutation.
    fn rerank_deadline_ms(&self) -> Option<u64> {
        Some(500)
    }
}
```

**`ResponseGroundingValidator`** — *Abstract contract for grounding validation of LLM-generated responses against raw source strings.* (`src/lifecycle.rs`)

```rust
pub trait ResponseGroundingValidator : Send + Sync {
/// Evaluates the grounding score for an LLM-generated response against source text slices.
    /// Returns a float score in [0.0, 1.0].
    ///
    /// # Fehler
    /// Liefert `ActaError` bei Evaluierungsfehlern oder ungültigen Eingaben.
    /// # Crash
    /// Reiner Rechenvorgang im Arbeitsspeicher; keine Seiteneffekte oder I/O.
    fn score_grounding(&self, response: &str, sources: &[&str]) -> Result<f32>;
}
```

**`SegmentSynthesizer`** — *Trait-Abstraktion für LLM-Synthesizer zur Segment-Zusammenfassung (REM-Phase).* (`src/text_index.rs`)

```rust
pub trait SegmentSynthesizer : Send + Sync {
/// Synthetisiert ein Segment von Texten zu einer abstrakten Zusammenfassung.
    fn synthesize_segment<'a>(
        &'a self,
        segment_texts: &'a [&'a str],
    ) -> BoxFuture<'a, Result<String>>;
    /// Gibt die Modell-ID des Synthesizers zurück.
    ///
    /// # Fehler
    /// Keine; statische Identifikator-Abfrage.
    /// # Crash
    /// Reiner Statusabruf ohne persistente Mutation.
    fn model_id(&self) -> &str;
}
```

**`StorageBackend`** — *Generisches Storage-Backend für modulare Datenzugriffe im Löschkern.* (`src/erasure.rs`)

```rust
pub trait StorageBackend : Send + Sync + 'static {

}
```

**`StorageEngine`** — *Unified Storage Engine trait — retained for backward compatibility across higher layers. # Dyn-Kompatibilität Dieser Trait ist durch explizite `BoxFuture`-Rückgabetypen vtable-kompatibel (dyn-safe).* (`src/storage.rs`)

```rust
pub trait StorageEngine : Send + Sync + 'static {
// TOCTOU-ACCEPTED: StorageEngine provides default batching/tracking delegate methods.
    /// Retrieves a value by key.
    fn get<'a>(&'a self, key: &'a [u8]) -> BoxFuture<'a, Result<Option<Bytes>>>;

    /// Retrieves a value by key at a specific sequence number (MVCC).
    fn get_at_seq<'a>(&'a self, key: &'a [u8], seq: u64) -> BoxFuture<'a, Result<Option<Bytes>>>;

    /// Retrieves a value by key tracked within transaction `tx_id` for SSI read-set validation.
    ///
    /// # ReadSet Tracking
    /// Untracked: Default implementation delegates to [`StorageEngine::get`] and registriert KEINE Lesezugriffe im ReadSet.
    /// Concrete SSI-enabled engines (e.g. `LsmStorage`) override this method to record keys in the transaction's `ReadSet`.
    fn get_tracked<'a>(
        &'a self,
        _tx_id: TxId,
        key: &'a [u8],
    ) -> BoxFuture<'a, Result<Option<Bytes>>> {
        self.get(key)
    }

    /// Retrieves a value by key at `seq` tracked within transaction `tx_id` for SSI read-set validation.
    ///
    /// # ReadSet Tracking
    /// Untracked: Default implementation delegates to [`StorageEngine::get_at_seq`] and registriert KEINE Lesezugriffe im ReadSet.
    /// Concrete SSI-enabled engines (e.g. `LsmStorage`) override this method to record keys in the transaction's `ReadSet`.
    fn get_at_seq_tracked<'a>(
        &'a self,
        _tx_id: TxId,
        key: &'a [u8],
        seq: u64,
    ) -> BoxFuture<'a, Result<Option<Bytes>>> {
        self.get_at_seq(key, seq)
    }

    /// Scans a range of keys with given prefix tracked within transaction `tx_id` for SSI read-set validation.
    ///
    /// # ReadSet Tracking
    /// Untracked: Default implementation delegates to [`StorageEngine::scan_prefix`] and registriert KEINE Lesezugriffe im ReadSet.
    /// Concrete SSI-enabled engines (e.g. `LsmStorage`) override this method to record keys/prefixes in the transaction's `ReadSet`.
    #[allow(clippy::type_complexity)]
    fn scan_prefix_tracked<'a>(
        &'a self,
        _tx_id: TxId,
        prefix: &'a [u8],
    ) -> BoxFuture<'a, Result<Vec<(Vec<u8>, Vec<u8>)>>> {
        self.scan_prefix(prefix)
    }

    /// Returns `true` if this storage engine implementation actively tracks read-sets for SSI validation.
    ///
    /// Default implementation returns `false`.
    ///
    /// # Fehler
    /// Keine; statische Fähigkeitsabfrage.
    /// # Crash
    /// Reiner Statusabruf ohne persistente Mutation.
    fn supports_ssi_tracking(&self) -> bool {
        false
    }

    /// Stores a key-value pair as part of a transaction.
    fn put<'a>(&'a self, tx_id: TxId, key: &'a [u8], value: &'a [u8]) -> BoxFuture<'a, Result<()>>;

    /// Atomically writes `value` under `key` within transaction ONLY IF no value exists.
    fn put_if_absent<'a>(
        &'a self,
        _tx_id: TxId,
        _key: &'a [u8],
        _value: &'a [u8],
    ) -> BoxFuture<'a, Result<bool>> {
        Box::pin(async move {
            Err(crate::error::ActaError::capability_unsupported(
                "put_if_absent",
                "Atomic put_if_absent is not supported by default — concrete storage engines must implement it atomically.",
            ))
        })
    }

    /// Stores multiple key-value pairs as part of a transaction.
    fn put_batch<'a>(
        &'a self,
        tx_id: TxId,
        entries: &'a [(Vec<u8>, Vec<u8>)],
    ) -> BoxFuture<'a, Result<()>> {
        Box::pin(async move {
            for (key, value) in entries {
                self.put(tx_id, key, value).await?;
            }
            Ok(())
        })
    }

    /// Deletes a key as part of a transaction.
    fn delete<'a>(&'a self, tx_id: TxId, key: &'a [u8]) -> BoxFuture<'a, Result<()>>;

    /// Deletes multiple keys as a single logical batch operation.
    fn delete_many<'a>(&'a self, tx_id: TxId, keys: Vec<Vec<u8>>) -> BoxFuture<'a, Result<u64>> {
        Box::pin(async move {
            let mut deleted = 0u64;
            for key in keys {
                self.delete(tx_id, &key).await?;
                deleted += 1;
            }
            Ok(deleted)
        })
    }

    /// Deletes all key-value pairs starting with `prefix` as part of a transaction.
    // TOCTOU-ACCEPTED
    fn delete_prefix<'a>(&'a self, tx_id: TxId, prefix: &'a [u8]) -> BoxFuture<'a, Result<u64>> {
        Box::pin(async move {
            let matching_keys: Vec<Vec<u8>> = self
                .scan_prefix(prefix)
                .await?
                .into_iter()
                .map(|(key, _)| key)
                .collect();
            self.delete_many(tx_id, matching_keys).await
        })
    }

    // ANCHOR[acta-ports:STORAGE-TX-001]
    /// Commits a transaction.
    fn commit<'a>(&'a self, tx_id: TxId) -> BoxFuture<'a, Result<()>>;

    /// Rolls back a transaction.
    fn rollback<'a>(&'a self, tx_id: TxId) -> BoxFuture<'a, Result<()>>;

    /// Rolls back the entire storage state to a specific transaction ID.
    fn rollback_to_tx<'a>(&'a self, tx_id: TxId) -> BoxFuture<'a, Result<()>>;

    /// Flushes memtable to disk.
    fn flush<'a>(&'a self) -> BoxFuture<'a, Result<()>>;

    /// Returns storage statistics.
    fn stats<'a>(&'a self) -> BoxFuture<'a, Result<StorageStats>>;

    /// Returns the last sequence number committed to storage.
    fn last_seq_no<'a>(&'a self) -> BoxFuture<'a, Result<u64>>;

    /// Returns the last transaction ID committed to storage.
    fn last_tx_id<'a>(&'a self) -> BoxFuture<'a, Result<TxId>>;

    // ANCHOR[acta-ports:STORAGE-CKPT-001]
    /// Pins a checkpoint for sequence number.
    fn pin_checkpoint<'a>(&'a self, seq_no: u64) -> BoxFuture<'a, Result<()>>;

    /// Unpins a checkpoint for sequence number.
    fn unpin_checkpoint<'a>(&'a self, seq_no: u64) -> BoxFuture<'a, Result<()>>;

    // ANCHOR[acta-ports:STORAGE-SCAN-001]
    /// Scans a range of keys with given prefix.
    #[allow(clippy::type_complexity)]
    fn scan_prefix<'a>(
        &'a self,
        prefix: &'a [u8],
    ) -> BoxFuture<'a, Result<Vec<(Vec<u8>, Vec<u8>)>>>;

    /// Scans prefix bounded.
    #[allow(clippy::type_complexity)]
    fn scan_prefix_bounded<'a>(
        &'a self,
        prefix: &'a [u8],
        limit: usize,
        cursor: Option<&'a [u8]>,
    ) -> BoxFuture<'a, Result<(Vec<(Vec<u8>, Vec<u8>)>, Option<Vec<u8>>)>> {
        Box::pin(async move {
            let all = self.scan_prefix(prefix).await?;
            let mut results = Vec::new();

            for (k, v) in all {
                if let Some(cur_bytes) = cursor {
                    if k.as_slice() <= cur_bytes {
                        continue;
                    }
                }
                results.push((k, v));
                if results.len() == limit {
                    break;
                }
            }

            let next_cursor = if results.len() == limit {
                results.last().map(|(k, _)| k.clone())
            } else {
                None
            };

            Ok((results, next_cursor))
        })
    }

    /// Scans prefix visible at sequence number.
    #[allow(clippy::type_complexity)]
    fn scan_prefix_at<'a>(
        &'a self,
        _prefix: &'a [u8],
        _seq_no: u64,
    ) -> BoxFuture<'a, Result<Vec<(Vec<u8>, Vec<u8>)>>> {
        Box::pin(async move {
            Err(crate::error::ActaError::capability_unsupported(
                "snapshot_read_at",
                "Storage-level snapshot-isolated prefix scan (scan_prefix_at) is not supported by default — implementors must override this method to guarantee MVCC snapshot isolation.",
            ))
        })
    }

    /// Scans range between start and end bounds.
    #[allow(clippy::type_complexity)]
    fn scan<'a>(
        &'a self,
        start: std::ops::Bound<&'a [u8]>,
        end: std::ops::Bound<&'a [u8]>,
        limit: Option<usize>,
    ) -> BoxFuture<'a, Result<Vec<(Vec<u8>, Vec<u8>)>>>;

    /// Scans range bounded.
    #[allow(clippy::type_complexity)]
    fn scan_bounded<'a>(
        &'a self,
        _start: std::ops::Bound<&'a [u8]>,
        _end: std::ops::Bound<&'a [u8]>,
        _limit: usize,
        _cursor: Option<&'a [u8]>,
    ) -> BoxFuture<'a, Result<(Vec<(Vec<u8>, Vec<u8>)>, Option<Vec<u8>>)>> {
        Box::pin(async move {
            Err(crate::error::ActaError::capability_unsupported(
                "scan_bounded",
                "Bounded range scan (scan_bounded) is not supported by default",
            ))
        })
    }
}
```

**`TextEmbeddingEngine`** — *Text embedding engine trait.* (`src/embedding.rs`)

```rust
pub trait TextEmbeddingEngine : Send + Sync + 'static {
/// Generates an embedding for the given text.
    fn embed<'a>(&'a self, text: &'a str) -> BoxFuture<'a, crate::Result<Vec<f32>>>;

    /// Generates embeddings for multiple texts.
    /// Default implementation executes sequential calls.
    fn embed_batch<'a>(
        &'a self,
        texts: &'a [&'a str],
    ) -> BoxFuture<'a, crate::Result<Vec<Vec<f32>>>> {
        Box::pin(async move {
            let mut results = Vec::with_capacity(texts.len());
            for text in texts {
                results.push(self.embed(text).await?);
            }
            Ok(results)
        })
    }
}
```

**`TextGenerator`** — ** (`src/embedding.rs`)

```rust
pub trait TextGenerator : Send + Sync + 'static {
/// Generates text response for a given prompt.
    fn generate_text<'a>(&'a self, prompt: &'a str) -> BoxFuture<'a, crate::Result<String>>;
}
```

**`TextIndex`** — *Text-Index Trait — abstrahiert BM25/Inverted-Index-Operationen. # Dyn-Kompatibilität Verwendet native `async fn` (AFIT) für statischen Dispatch.* (`src/text_index.rs`)

```rust
pub trait TextIndex : Send + Sync + 'static {
// ANCHOR[acta-ports:TXT-SEARCH-001]
    /// Searches for documents matching the query.
    fn search(
        &self,
        query: &str,
        k: usize,
    ) -> impl Future<Output = Result<Vec<ScoredDocument>>> + Send;

    /// Searches for documents matching the query at a specific sequence number.
    ///
    /// # Errors
    /// Returns [`ActaError::CapabilityUnsupported`][crate::ActaError::CapabilityUnsupported]
    /// with capability `"snapshot_read_at"` if snapshot-isolated text search is not implemented.
    /// Tested via `capability_coverage` test module.
    fn search_at(
        &self,
        query: &str,
        k: usize,
        seq_no: u64,
    ) -> impl Future<Output = Result<Vec<ScoredDocument>>> + Send {
        async move {
            let _ = (query, k, seq_no);
            Err(crate::error::ActaError::capability_unsupported(
                "snapshot_read_at",
                "Text search snapshot isolation (search_at) is not supported by default — tracked in ADR-024",
            ))
        }
    }

    /// Inserts or updates a document in the index.
    ///
    /// # Fehler
    /// Liefert `ActaError` bei I/O-, Tokenisierungs- oder Speicherfehlern.
    /// # Crash
    /// Transaktional gepuffert bis zum Commit; vor Commit kein persistenter Schaden.
    fn insert(&self, tx: TxId, id: DocId, text: &str) -> impl Future<Output = Result<()>> + Send;

    /// Deletes a document from the index.
    ///
    /// # Fehler
    /// Liefert `ActaError` bei Transaktions- oder I/O-Fehlern.
    /// # Crash
    /// Transaktional isoliert; bei Crash vor Commit bleibt der Vorzustand erhalten.
    fn delete(&self, tx: TxId, id: DocId) -> impl Future<Output = Result<()>> + Send;

    /// Commits a transaction.
    ///
    /// # Fehler
    /// Liefert `ActaError` bei WAL-, Flush- oder Fsync-Fehlern.
    /// # Crash
    /// WAL-first; Commit ist nach erfolgreichem Fsync dauerhaft wiederherstellbar.
    fn commit(&self, tx: TxId) -> impl Future<Output = Result<()>> + Send;

    /// Rolls back a transaction.
    ///
    /// # Fehler
    /// Liefert `ActaError` bei Transaktionsverwaltungsproblemen.
    /// # Crash
    /// Verwirft uncommittete Schreibpuffer im RAM; Zustand bleibt konsistent.
    fn rollback(&self, tx: TxId) -> impl Future<Output = Result<()>> + Send;

    /// Rolls back the entire index state to a specific transaction ID.
    ///
    /// # Fehler
    /// Liefert `ActaError` falls der Zielzustand nicht auffindbar oder korrupt ist.
    /// # Crash
    /// Atomarer Rollback; bei Unterbrechung stellt Recovery den sauberen Zustand her.
    fn rollback_to_tx(&self, tx_id: TxId) -> impl Future<Output = Result<()>> + Send;

    /// Returns the last transaction ID processed by the index.
    ///
    /// # Fehler
    /// Liefert `ActaError` bei Lesefehlern des Manifests/Indexheaders.
    /// # Crash
    /// Reiner Statusabruf ohne Mutation.
    fn last_tx_id(&self) -> impl Future<Output = Result<TxId>> + Send;

    /// Returns the number of documents in the index.
    ///
    /// # Fehler
    /// Keine; panikfrei.
    /// # Crash
    /// Reiner Statusabruf aus Header oder RAM-Zustand.
    fn len(&self) -> impl Future<Output = usize> + Send;

    /// Returns true if the index is empty.
    ///
    /// # Fehler
    /// Keine; delegiert panikfrei an `len`.
    /// # Crash
    /// Reiner Statusabruf ohne Seiteneffekte.
    fn is_empty(&self) -> impl Future<Output = bool> + Send {
        async { self.len().await == 0 }
    }

    /// Returns index statistics.
    ///
    /// # Fehler
    /// Liefert `ActaError` falls Statistiken nicht aggregiert werden können.
    /// # Crash
    /// Reiner Lesevorgang ohne Zustandsänderung.
    fn stats(&self) -> impl Future<Output = Result<TextIndexStats>> + Send;
}
```

**`VectorIndex`** — *Vector Index Trait — abstrahiert die HNSW-Vektorsuche. # Dyn-Kompatibilität Verwendet native `async fn` (AFIT) für statischen Dispatch.* (`src/vector_index.rs`)

```rust
pub trait VectorIndex : Send + Sync + 'static {
// ANCHOR[acta-ports:VEC-MUT-001]
    /// Inserts a vector with an associated document ID.
    fn insert(
        &self,
        tx: TxId,
        id: DocId,
        embedding: &[f32],
    ) -> impl Future<Output = Result<()>> + Send;

    /// Returns all active (non-deleted) document IDs in the index.
    ///
    /// # Fehler
    /// Liefert `ActaError` bei Lesefehlern der Node-Arena.
    /// # Crash
    /// Reiner Lesevorgang ohne Modifikation.
    fn all_doc_ids(&self) -> impl Future<Output = Result<Vec<DocId>>> + Send {
        async { Ok(Vec::new()) }
    }

    /// Inserts multiple vectors with associated document IDs.
    fn insert_batch(
        &self,
        tx: TxId,
        vectors: &[(DocId, &[f32])],
    ) -> impl Future<Output = Result<()>> + Send {
        async move {
            for (id, embedding) in vectors {
                self.insert(tx, *id, embedding).await?;
            }
            Ok(())
        }
    }

    // ANCHOR[acta-ports:VEC-SEARCH-001]
    /// Searches for the k nearest neighbors to a query vector.
    fn search(
        &self,
        query: &[f32],
        k: usize,
    ) -> impl Future<Output = Result<Vec<ScoredDocument>>> + Send;

    /// Searches for the k nearest neighbors to a query vector at a specific sequence number.
    ///
    /// # Errors
    /// Returns [`ActaError::CapabilityUnsupported`][crate::ActaError::CapabilityUnsupported]
    /// with capability `"snapshot_read_at"` if snapshot-isolated vector search is not implemented.
    /// Tested via `capability_coverage` test module.
    fn search_at(
        &self,
        query: &[f32],
        k: usize,
        seq_no: u64,
    ) -> impl Future<Output = Result<Vec<ScoredDocument>>> + Send {
        async move {
            let _ = (query, k, seq_no);
            Err(crate::error::ActaError::capability_unsupported(
                "snapshot_read_at",
                "Vector search snapshot isolation (search_at) is not supported by default — tracked in ADR-024",
            ))
        }
    }

    /// Searches with an optional filter predicate.
    ///
    /// # Default Behaviour
    /// Returns an error if a filter is provided. Implementors **MUST** override
    /// this method if filtered search is supported by their vector engine.
    ///
    /// # Errors
    /// Returns [`ActaError::CapabilityUnsupported`][crate::ActaError::CapabilityUnsupported]
    /// with capability `"vector_filtered_search"` if a filter predicate is passed to an engine without filter support.
    /// Tested via `capability_coverage` test module.
    ///
    /// # Note
    /// This default exists solely for backward compatibility.
    fn search_filtered(
        &self,
        query: &[f32],
        k: usize,
        filter: Option<&(dyn Fn(DocId) -> bool + Send + Sync)>,
    ) -> impl Future<Output = Result<Vec<ScoredDocument>>> + Send {
        async move {
            if filter.is_some() {
                return Err(crate::error::ActaError::capability_unsupported(
                    "vector_filtered_search",
                    "Filtered vector search is not supported by default for this vector engine",
                ));
            }
            self.search(query, k).await
        }
    }

    /// Deletes a vector by its document ID.
    ///
    /// # Fehler
    /// Liefert `ActaError` bei Transaktions- oder Arena-Fehlern.
    /// # Crash
    /// Gemäß VETO-F02 strictly tombstone + slot-zeroization; Teilausfälle vor Commit verwerfen Puffer.
    fn delete(&self, tx: TxId, id: DocId) -> impl Future<Output = Result<()>> + Send;

    /// Commits a transaction.
    ///
    /// # Fehler
    /// Liefert `ActaError` bei WAL-, Flush- oder Fsync-Fehlern.
    /// # Crash
    /// WAL-first; nach erfolgreichem fsync atomar persistent.
    fn commit(&self, tx: TxId) -> impl Future<Output = Result<()>> + Send;

    /// Rolls back a transaction.
    ///
    /// # Fehler
    /// Liefert `ActaError` bei Transaktionsverwaltungsproblemen.
    /// # Crash
    /// Verwirft uncommittete Änderungen im RAM; Graph bleibt unverändert konsistent.
    fn rollback(&self, tx: TxId) -> impl Future<Output = Result<()>> + Send;

    /// Rolls back the entire index state to a specific transaction ID.
    ///
    /// # Fehler
    /// Liefert `ActaError` falls der Ziel-Transaktionszustand nicht auffindbar ist.
    /// # Crash
    /// Atomarer Rollback; Recovery stellt bei Unterbrechung den sauberen Zustand her.
    fn rollback_to_tx(&self, tx_id: TxId) -> impl Future<Output = Result<()>> + Send;

    /// Returns the last transaction ID processed by the index.
    ///
    /// # Fehler
    /// Liefert `ActaError` bei Lesefehlern der Index-Metadaten.
    /// # Crash
    /// Reiner Statusabruf ohne Mutation.
    fn last_tx_id(&self) -> impl Future<Output = Result<TxId>> + Send;

    /// Returns the number of vectors in the index.
    ///
    /// # Fehler
    /// Keine; panikfrei.
    /// # Crash
    /// Reiner Statusabruf aus Header oder RAM-Zustand.
    fn len(&self) -> impl Future<Output = usize> + Send;

    /// Returns true if the index is empty.
    ///
    /// # Fehler
    /// Keine; delegiert panikfrei an `len`.
    /// # Crash
    /// Reiner Statusabruf ohne Seiteneffekte.
    fn is_empty(&self) -> impl Future<Output = bool> + Send {
        async { self.len().await == 0 }
    }

    /// Returns index statistics.
    ///
    /// # Fehler
    /// Liefert `ActaError` falls Statistiken nicht ermittelt werden können.
    /// # Crash
    /// Reiner Lesevorgang ohne Zustandsänderung.
    fn stats(&self) -> impl Future<Output = Result<VectorIndexStats>> + Send;

    /// Returns true if the index requires a background rebuild (e.g. due to tombstone accumulation).
    ///
    /// # Fehler
    /// Keine; panikfrei.
    /// # Crash
    /// Reiner Statusabruf ohne Mutation.
    fn is_rebuild_required(&self) -> bool {
        false
    }

    // ANCHOR[acta-ports:VEC-REBUILD-001]
    /// Triggers an asynchronous background rebuild of the index if supported.
    ///
    /// # Fehler
    /// Keine synchronen Fehler; asynchroner Hintergrundtask.
    /// # Crash
    /// Snapshot-isoliert; parallele Leser sehen bit-identische Ergebnisse während und nach Rebuild.
    fn trigger_rebuild_async(&self) {}
}
```

**`Vfs`** — *Abstract object-safe Virtual File System interface. Implements required file operations for LSM storage engine, SSTables, WAL sidecars, vault slot storage, and vector index persistence (SPEC 3.1, D-113, DF-01..DF-03).* (`src/vfs.rs`)

```rust
pub trait Vfs : Send + Sync + 'static {
/// Opens a file according to the specified options.
    ///
    /// # Contract V-VFS
    /// Default implementation delegates to `std::io::ErrorKind::Unsupported` unless overridden.
    ///
    /// # Fehler
    /// Liefert [`ActaError::Io`](crate::error::ActaError::Io) bei Berechtigungs-, Nicht-Gefunden- oder I/O-Fehlern.
    ///
    /// # Crash
    /// Öffnet Dateideskriptor; erzeugt ggf. eine leere Datei bei `create_new`/`create`. Keine implizite Datenpersistenz.
    fn open(&self, _path: &Path, _opts: &VfsOpenOptions) -> Result<Box<dyn VfsFile>> {
        Err(crate::ActaError::Io(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "positional VfsFile open is not supported by default in this Vfs implementation",
        )))
    }

    /// Reads data from byte offset `offset` into `buf` for file at `path`.
    ///
    /// # Fehler
    /// Liefert [`ActaError::Io`](crate::error::ActaError::Io) bei I/O-Fehlern.
    ///
    /// # Crash
    /// Reiner Lesezugriff ohne persistente Mutation.
    fn read_at(&self, path: &Path, offset: u64, buf: &mut [u8]) -> Result<usize> {
        let file = self.open(path, &VfsOpenOptions::read_only())?;
        file.read_at(buf, offset).map_err(crate::ActaError::Io)
    }

    /// Reads verified bytes directly from media or cache-dropped layer (SPEC D-113).
    ///
    /// Returns the verified byte payload and the verification classification `VerifiedVia`.
    ///
    /// # Fehler
    /// Liefert [`ActaError::Io`](crate::error::ActaError::Io) bei Lesefehlern.
    fn read_verified(
        &self,
        path: &Path,
        offset: u64,
        len: usize,
    ) -> Result<(Vec<u8>, VerifiedVia)> {
        let file = self.open(path, &VfsOpenOptions::read_only())?;
        file.read_verified(offset, len)
            .map_err(crate::ActaError::Io)
    }

    /// Reads the entire contents of a file at `path`.
    ///
    /// # Fehler
    /// Liefert [`ActaError::Io`](crate::error::ActaError::Io) oder [`ActaError::NotFound`](crate::error::ActaError::NotFound)
    /// falls die Datei fehlt oder nicht gelesen werden kann.
    ///
    /// # Crash
    /// Reiner Lesezugriff; keine Zustandsänderung auf dem Speichermedium.
    fn read(&self, path: &Path) -> Result<Vec<u8>>;

    /// Atomically writes `data` to `path` with crash-consistency guarantees.
    ///
    /// # Crash-Consistency Contract
    /// Writes to a temporary file, calls `fsync` on the file handle, renames the temporary
    /// file over `path`, and syncs the parent directory (`sync_dir`).
    ///
    /// # Fehler
    /// Liefert [`ActaError::Io`](crate::error::ActaError::Io) falls Schreiben, Synchronisieren oder Umbenennen fehlschlägt.
    ///
    /// # Crash
    /// Crash-konsistente All-or-Nothing-Garantie: Bricht der Vorgang vor atomarem Rename ab, bleibt die Originaldatei unberührt.
    fn write_atomic(&self, path: &Path, data: &[u8]) -> Result<()>;

    /// Creates a directory and all parent directories along `path` if missing.
    ///
    /// # Fehler
    /// Liefert [`ActaError::Io`](crate::error::ActaError::Io) falls Verzeichnisanlage fehlschlägt.
    ///
    /// # Crash
    /// Idempotente Verzeichnisanlage; Durability der Verzeichnisstruktur wird über `sync_dir` sichergestellt.
    fn create_dir_all(&self, path: &Path) -> Result<()>;

    /// Removes a file at `path`.
    ///
    /// # Fehler
    /// Liefert [`ActaError::Io`](crate::error::ActaError::Io) oder [`ActaError::NotFound`](crate::error::ActaError::NotFound)
    /// falls das Entfernen fehlschlägt oder die Datei fehlt.
    ///
    /// # Crash
    /// Entfernt Verzeichniseintrag; Persistenz erfordert `sync_dir` des übergeordneten Verzeichnisses.
    fn remove_file(&self, path: &Path) -> Result<()>;

    /// Canonical alias for removing a file (SPEC D-113 / Purge-Primitive).
    ///
    /// # Fehler
    /// Liefert [`ActaError::Io`](crate::error::ActaError::Io) oder [`ActaError::NotFound`](crate::error::ActaError::NotFound).
    /// # Crash
    /// Entfernt Verzeichniseintrag; Persistenz erfordert `sync_dir`.
    fn unlink(&self, path: &Path) -> Result<()> {
        self.remove_file(path)
    }

    /// Atomically renames a file from `from` to `to`.
    ///
    /// # Fehler
    /// Liefert [`ActaError::Io`](crate::error::ActaError::Io) falls das Umbenennen fehlschlägt.
    ///
    /// # Crash
    /// Atomarer Dateisystem-Rename; bei Systemabsturz existiert entweder der Quell- oder der Zielpfad.
    fn rename(&self, _from: &Path, _to: &Path) -> Result<()> {
        Err(crate::ActaError::Io(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "atomic rename is not supported by default in this Vfs implementation",
        )))
    }

    /// Reads all entry paths within directory `path`.
    ///
    /// # Fehler
    /// Liefert [`ActaError::Io`](crate::error::ActaError::Io) bei Lesefehlern des Verzeichnisses.
    ///
    /// # Crash
    /// Reiner Lesezugriff; keine Verzeichnisänderung.
    fn read_dir(&self, path: &Path) -> Result<Vec<PathBuf>> {
        let entries = self.list_dir(path)?;
        Ok(entries.into_iter().map(|name| path.join(name)).collect())
    }

    /// Returns `true` if a file or directory exists at `path`.
    ///
    /// # Fehler
    /// Liefert [`ActaError::Io`](crate::error::ActaError::Io) falls Pfadstatus nicht ermittelt werden kann.
    ///
    /// # Crash
    /// Reiner Statusabruf ohne Mutation.
    fn exists(&self, path: &Path) -> Result<bool>;

    /// Returns the length in bytes of the file at `path`.
    ///
    /// # Fehler
    /// Liefert [`ActaError::Io`](crate::error::ActaError::Io) oder [`ActaError::NotFound`](crate::error::ActaError::NotFound)
    /// falls Metadaten nicht abgefragt werden können.
    ///
    /// # Crash
    /// Reines Abfragen der Dateilänge ohne Seiteneffekte.
    fn len(&self, path: &Path) -> Result<u64>;

    /// Lists entry names (file and directory names, excluding `.` and `..`) inside directory `path`.
    ///
    /// # Fehler
    /// Liefert [`ActaError::Io`](crate::error::ActaError::Io) oder [`ActaError::NotFound`](crate::error::ActaError::NotFound)
    /// falls Pfad kein Verzeichnis ist oder nicht gelesen werden kann.
    ///
    /// # Crash
    /// Reiner Lesezugriff auf Verzeichniseinträge.
    fn list_dir(&self, path: &Path) -> Result<Vec<String>>;

    /// Canonical alias for listing entries in directory `path` (SPEC 3.1).
    ///
    /// # Fehler
    /// Liefert [`ActaError::Io`](crate::error::ActaError::Io) oder [`ActaError::NotFound`](crate::error::ActaError::NotFound).
    /// # Crash
    /// Reiner Lesezugriff auf Verzeichniseinträge.
    fn list(&self, path: &Path) -> Result<Vec<String>> {
        self.list_dir(path)
    }

    /// Flushes directory metadata to persistent storage for durable file additions and renames.
    ///
    /// # Fehler
    /// Liefert [`ActaError::Io`](crate::error::ActaError::Io) falls Directory-Sync fehlschlägt.
    ///
    /// # Crash
    /// Schreibt Verzeichnis-Metadaten dauerhaft auf Festplatte; schützt vor Datenverlust bei Stromausfall.
    fn sync_dir(&self, path: &Path) -> Result<()>;
}
```

**`VfsFile`** — *Abstract file handle providing positional I/O without modifying internal file offsets.* (`src/vfs.rs`)

```rust
pub trait VfsFile : Send + Sync {
/// Reads data from byte offset `offset` into `buf` without moving the file pointer.
    ///
    /// # Fehler
    /// Liefert `std::io::Error` bei Lesefehlern oder ungültigem Offset.
    ///
    /// # Crash
    /// Reiner positioneller Lesezugriff; verändert keine Dateimetadaten oder persistente Zustände.
    fn read_at(&self, buf: &mut [u8], offset: u64) -> std::io::Result<usize>;

    /// Reads verified data from storage medium bypassing or dropping the page cache (SPEC D-113).
    ///
    /// In accordance with SPEC D-113:
    /// Returns read bytes and classification `VerifiedVia ∈ {Direct, CacheDropped, Cached}`.
    /// Default implementation reads positionally via `read_at` classified as `VerifiedVia::Cached`.
    ///
    /// # Fehler
    /// Liefert `std::io::Error` bei I/O-Fehlern oder Leseproblemen.
    ///
    /// # Crash
    /// Reiner Lesezugriff zur Verifikation von Slots oder Medienblöcken.
    fn read_verified(&self, offset: u64, len: usize) -> std::io::Result<(Vec<u8>, VerifiedVia)> {
        let mut buf = vec![0u8; len];
        let n = self.read_at(&mut buf, offset)?;
        buf.truncate(n);
        Ok((buf, VerifiedVia::Cached))
    }

    /// Writes data from `buf` to byte offset `offset` without moving the file pointer.
    ///
    /// # Fehler
    /// Liefert `std::io::Error` bei Schreibfehlern oder Speicherplatzmangel.
    ///
    /// # Crash
    /// Schreibt Datenblöcke; Persistenz erfordert nachfolgendes `sync_data` oder `sync_all`.
    fn write_at(&self, buf: &[u8], offset: u64) -> std::io::Result<usize>;

    /// Truncates or extends the underlying file length to `len` bytes.
    ///
    /// # Fehler
    /// Liefert `std::io::Error` bei Kürzungs- oder Allokationsfehlern.
    ///
    /// # Crash
    /// Modifiziert die Dateigröße; wird erst durch `sync_all` crash-resistent festgeschrieben.
    fn set_len(&self, len: u64) -> std::io::Result<()>;

    /// Queries the current physical file length in bytes.
    ///
    /// # Fehler
    /// Liefert `std::io::Error` falls Dateimetadaten nicht abgefragt werden können.
    ///
    /// # Crash
    /// Reines Abfragen von Dateimetadaten ohne Seiteneffekte.
    fn len(&self) -> std::io::Result<u64>;

    /// Returns `true` if the file length is zero.
    ///
    /// # Fehler
    /// Liefert `std::io::Error` falls die Längenabfrage fehlschlägt.
    ///
    /// # Crash
    /// Reine Längenprüfung ohne Seiteneffekte.
    fn is_empty(&self) -> std::io::Result<bool> {
        Ok(self.len()? == 0)
    }

    /// Flushes data blocks to disk (`fdatasync`).
    ///
    /// # Fehler
    /// Liefert `std::io::Error` bei fehlschlagendem Flush. Bei Sync-Fehlern muss das Handle vergiftet werden (DF-03, D-55, T-FSYNC-1).
    ///
    /// # Crash
    /// Schreibt Datenblöcke bis zum Aufrufzeitpunkt persistent auf das Speichermedium.
    fn sync_data(&self) -> std::io::Result<()>;

    /// Flushes data blocks and metadata to disk (`fsync`).
    ///
    /// # Fehler
    /// Liefert `std::io::Error` bei Synchronisationsfehlern. Scheitert `sync_all`, ist das Handle dauerhaft vergiftet und darf nicht weiterverwendet werden (D-55, I-10).
    ///
    /// # Crash
    /// Bewirkt Haltbarkeit aller Datenblöcke und Inode-Metadaten (z. B. Dateilänge).
    fn sync_all(&self) -> std::io::Result<()>;
}
```

### Öffentliche Datentypen — Structs (47)

| Struct | Felder / Form | Beschreibung | Datei |
|---|---|---|---|
| `Armed` | `(tuple/unit)` | Phantom state for an armed scanner verifying deletion residue. | `src/erasure.rs` |
| `AuthorizedRing` | `{ ring: FeatureRing, }` | Token representing an authorized feature ring granting access to engine features (§15). `AuthorizedRing` cannot be directly instantiated outside of t… | `src/license.rs` |
| `BarrierExclusive` | `{ _token: Option<Arc<()>>, }` | RAII token guard representing an exclusive gate phase permit. | `src/barrier.rs` |
| `BarrierShared` | `{ _token: Option<Arc<()>>, }` | RAII token guard representing a shared commit phase permit. | `src/barrier.rs` |
| `CompactionPermit` | `{ _token: Option<Arc<()>>, }` | RAII token guard representing a compaction permit. | `src/barrier.rs` |
| `ContextSegment` | `{ /// Unique identifier for the chunk. pub chunk_id: u64, /// Text content of the segment. pub text: &'a str, /// Optional model fingerprint reference. pub model… }` | Representation of a context segment passed to context-aware text generation. | `src/embedding.rs` |
| `Envelope` | `{ payload: Vec<u8>, }` | Sealed record envelope containing encrypted ciphertext and tag. | `src/keys.rs` |
| `ErasureEvent` | `{ /// Unique identifier of the erasure operation. pub id: ErasureId, /// Kind of target affected. pub target_kind: TargetKind, /// Current phase reached. pub pha… }` | Event published to an [`ErasureObserver`] upon reaching an erasure lifecycle phase (SPEC D-118). | `src/observer.rs` |
| `FileId` | `(tuple/unit)` | Unique identifier for a durable storage file or SSTable. | `src/keys.rs` |
| `FileKey` | `{ raw: Vec<u8>, file_id: FileId, }` | Wrapper around a cryptographic file key with zeroizing on drop and masked debug display (PO-08). | `src/keys.rs` |
| `GraphIndexStats` | `{ /// Number of active nodes (Entities). pub num_entities: usize, /// Number of active edges. pub num_edges: usize, /// Total bytes allocated by CSR representati… }` | Statistics for the GraphIndex layer. | `src/graph_index.rs` |
| `GroundingAssessment` | `{ /// Raw or calibrated confidence score in [0.0, 1.0] indicating attribution quality. pub score: f32, /// Indicates whether the grounding score meets the requir… }` | Result of a post-hoc grounding / attribution validation check. | `src/lifecycle.rs` |
| `HyperEdgeId` | `(tuple/unit)` | — | `src/graph.rs` |
| `KeyBuf` | `(tuple/unit)` | Fixed or bounded key buffer for storage operations (SPEC §8.1). | `src/gate.rs` |
| `KeyRange` | `{ /// Lower boundary inclusive. pub start: KeyBuf, /// Upper boundary exclusive. pub end: KeyBuf, }` | Half-open key range `[start, end)` for scan and compaction boundaries. | `src/gate.rs` |
| `KvBlock` | `{ /// Block sequence identifier. pub block_id: u64, /// Encrypted or raw block tensor bytes. #[serde( serialize_with = "serialize_bytes", deserialize_with = "des… }` | Represents an exported KV block segment. | `src/kv.rs` |
| `KvLayout` | `{ /// Number of transformer layers. pub n_layer: u32, /// Number of KV heads. pub n_kv_head: u32, /// Dimension per head. pub head_dim: u32, /// Data type repres… }` | KV cache layer and dimensions layout configuration. | `src/kv.rs` |
| `KvPrefixHit` | `{ /// Number of tokens matched by the prefix. pub matched_tokens: usize, /// Extracted KV blocks for the matched prefix. pub blocks: Vec<KvBlock>, }` | Represents a prefix match hit in the KV store. | `src/kv.rs` |
| `LayerId` | `(tuple/unit)` | Layer identifier representing a specific subsystem participating in erasure (e.g. store, text, vector). | `src/erasure.rs` |
| `LayerScope` | `{ /// Opaque scope identifier or token. scope_id: [u8; 16], /// Segment or file identifiers affected. affected_units: Vec<u64>, }` | Scope resolved by an erasure layer prior to gating (SPEC §8.2, D-96). Records the concrete file sets, segment IDs, or key intervals that this layer m… | `src/erasure.rs` |
| `LayerWitness` | `{ layer: WitnessLayer, scanned: u64, remaining: u64, technique: ScanTechnique, method: ScanMethod, digest: [u8; 32], measured_at: u64, }` | Verifiziertes Messergebnis eines Storage-Scanners (SPEC §12.7.4). Unter INV-WITNESS-MINT-1 kann ein `LayerWitness` ausschließlich in `ScanSession::fi… | `src/scanner/witness.rs` |
| `LedgerEntry` | `{ /// Commitment hash over the target erasure definition. pub commitment: [u8; 32], /// Key ranges covered by this erasure. pub range_set: RangeSet, /// Sequence… }` | A tamper-evident entry in the erasure ledger (SPEC §4.2, §11.8, F.9). Contains `(commitment, range_set, gated_seq, completed_at)` along with `chain_h… | `src/ledger.rs` |
| `MemVfs` | `{ files: RwLock<BTreeMap<PathBuf, Vec<u8>>>, dirs: RwLock<BTreeMap<PathBuf, ()>>, }` | In-memory VFS reference implementation used strictly for contract self-testing. | `src/contract_tests.rs` |
| `OpenFastGate` | `(tuple/unit)` | Default open activation gate allowing `Fast` ring features without restriction. | `src/license.rs` |
| `PluginCapability` | `{ /// Unique plugin name. pub name: &'static str, /// Plugin version tuple (major, minor, patch). pub version: (u32, u32, u32), /// Architecture ring in which th… }` | Capability descriptor for a plugin. | `src/plugin.rs` |
| `PluginRegistry` | `{ active: BTreeMap<String, PluginCapability>, active_conflicts: BTreeMap<String, Vec<String>>, gate: Arc<dyn LicenseGate>, }` | Registry managing plugin activations, license gating, and dependency resolution. | `src/plugin.rs` |
| `PrefixKey` | `{ /// Target model fingerprint. pub model: ModelFingerprint, /// SHA256/BLAKE3 hash of the tokenizer vocabulary. pub tokenizer_hash: [u8; 32], /// KV tensor memo… }` | Key identifying a model KV cache prefix structure. | `src/kv.rs` |
| `PurgeCx` | `{ /// Budget limit or deadline in milliseconds. pub deadline_ms: u64, }` | Context passed into [`ErasureLayer::purge`] providing operational bounds and budgets. | `src/erasure.rs` |
| `PurgeOutcome` | `{ /// Number of records or items successfully removed. pub purged_count: u64, /// Number of bytes freed or compacted. pub bytes_reclaimed: u64, }` | Outcome of a layer purge execution (SPEC §8.2, WP-3.1). | `src/erasure.rs` |
| `PurgePermit` | `{ _token: Option<Arc<()>>, }` | RAII token guard representing an exclusive purge permit. | `src/barrier.rs` |
| `PurgeStats` | `{ /// Anzahl physisch bereinigter Einheiten. pub purged_units: u64, /// Anzahl freigegebener Bytes. pub bytes_freed: u64, }` | Statistiken über bereinigte Einheiten. | `src/scanner/layer.rs` |
| `RangeSet` | `{ ranges: Vec<(KeyBuf, KeyBuf)>, }` | Set of key ranges or tokens included in an erasure ledger entry. | `src/ledger.rs` |
| `ReportSignature` | `{ /// Signer identifier (u8: 0 = internal, 1 = operator / HSM). pub signer_id: SignerId, /// Base64url-encoded signature string without padding (RFC 4648 §5, SPE… }` | Digital signature over canonical report bytes (SPEC F.10). | `src/report_signer.rs` |
| `RequestId` | `(tuple/unit)` | Unique identifier for an inference request within a process. | `src/attention.rs` |
| `RoleBinding` | `{ /// The role assigned to the entity in the hyperedge. pub role: RoleId, /// The bound entity. pub entity: EntityId, }` | Binding of an entity to a specific role within an n-ary hyperedge. | `src/graph.rs` |
| `RoleId` | `(tuple/unit)` | — | `src/graph.rs` |
| `RopeConfig` | `{ /// Base frequency. pub base: f32, /// Optional frequency scaling factor. pub scaling: Option<f32>, }` | Rotary position embedding (RoPE) configuration. | `src/kv.rs` |
| `ScanSession` | `{ layer: WitnessLayer, target: ScanTarget, technique: ScanTechnique, scanned: u64, remaining: u64, control_matches: u64, unit_hashes: Vec<[u8; 32]>, }` | Laufende Scan-Session zur kontrollierten Erzeugung eines `LayerWitness`. | `src/scanner/session.rs` |
| `ScanTarget` | `{ /// Bezeichner des Scan-Ziels (z. B. Tenant, Collection, Path). pub target_id: String, /// BLAKE3 Hash des Scan-Ziels. pub target_hash: [u8; 32], }` | Zielbeschreibung des Scans inklusive Hash. | `src/scanner/layer.rs` |
| `Scanner` | `{ _state: PhantomData<State>, }` | Residue scanner type passed to [`ErasureLayer::witness`]. Must be purely read-only (PO-05). | `src/erasure.rs` |
| `SealCtx` | `{ /// Epoch sequence or slot identifier. pub epoch: u64, }` | Sealing context containing metadata and epoch tokens for envelope creation. | `src/keys.rs` |
| `Seq` | `(tuple/unit)` | Monotonic sequence number for transactions and gate markers. | `src/gate.rs` |
| `StorageStats` | `{ /// Number of SSTable segments. pub num_segments: usize, /// Total size of all SSTables in bytes. pub total_size_bytes: u64, /// Total size of memtables in byt… }` | Statistics for the storage engine. | `src/storage.rs` |
| `TextIndexStats` | `{ /// Total number of documents indexed. pub num_documents: usize, /// Total number of tokens across all documents. pub num_tokens: usize, /// Estimated memory u… }` | Statistics for a text index. | `src/text_index.rs` |
| `Timestamp` | `(tuple/unit)` | Discrete point in time measured in epoch milliseconds or nanoseconds. | `src/ledger.rs` |
| `VectorIndexStats` | `{ /// Number of active (non-deleted) vectors. pub num_vectors: usize, /// Estimated memory usage in bytes. pub memory_usage_bytes: usize, /// Number of HNSW laye… }` | Statistics for a vector index. | `src/vector_index.rs` |
| `VfsOpenOptions` | `{ /// Opens the file with read access. pub read: bool, /// Opens the file with write access. pub write: bool, /// Creates a new file if it does not exist. pub cr… }` | Options configuring how a file is opened via [`Vfs::open`]. | `src/vfs.rs` |

### Öffentliche Datentypen — Enums (20)

| Enum | Varianten | Beschreibung | Datei |
|---|---|---|---|
| `EgressDecision` | `/// Request is permitted to leave without modification. Allow, /// Request is permitted but payload must be redacted or sanitized. Redact(String), /// Request is denied. Deny(String),` | Egress evaluation decision. | `src/policy.rs` |
| `EmbeddingError` | `/// Provider is unreachable or unavailable. #[error("Provider unavailable: {0}")] Unavailable(String), /// Computation failed during embedding generation. #[error("Embedding failed: {0}")] Computatio…` | Error types encountered during embedding operations. | `src/embedding.rs` |
| `EraseTarget` | `/// Target a specific key or document identifier prefix. KeyRange { /// Lower key bound inclusive. start: Vec<u8>, /// Upper key bound exclusive. end: Vec<u8>, }, /// Target a discrete set of documen…` | Target of an erasure request specifying what needs to be erased. | `src/erasure.rs` |
| `FeatureRing` | `/// Fast ring (open source core). Fast, /// Sovereign ring (enterprise data sovereignty). Sovereign, /// Compliance ring (strict auditability and regulatory rules). Compliance,` | Feature-Ring gemäß Spec §15. | `src/license.rs` |
| `FileKind` | `/// SSTable data file. SSTable, /// Write-Ahead-Log (WAL) file. Wal, /// Inverted index segment. TextSegment, /// Vector HNSW segment. VectorSegment, /// General auxiliary file. Auxiliary,` | Kind of file managed by [`FileKeyring`] (SPEC §8.2, D-85). | `src/keys.rs` |
| `GateDecision` | `/// Key or boundary is open for reading. Open, /// Key or boundary has been erased. Access must be blocked immediately. Erased, /// Key or boundary is temporarily restricted (e.g. pending gate sequen…` | Gate decision for point queries or file boundaries (SPEC §4.2, §11.3). | `src/gate.rs` |
| `GraphMutationError` | `/// Lock acquisition timed out during concurrent mutation. #[error("Lock acquisition timed out: {0}")] LockAcquisitionTimeout(String), /// Duplicate hyperedge ID provided. #[error("Duplicate hyperedg…` | Error variants for graph mutation operations. | `src/graph.rs` |
| `KeyBound` | `/// Exact key match boundary. Exact(KeyBuf), /// Unbounded lower boundary. UnboundedLower, /// Unbounded upper boundary. UnboundedUpper,` | Boundary key used for compaction or range scanning. | `src/gate.rs` |
| `KeyCoverage` | `/// Layer maintains full key separation and slot-zeroing capability. Full, /// Layer holds keys unlinked from the main vault hierarchy. Unlinked, /// Layer maintains no direct key slots (relies entir…` | Key coverage capability of a layer (SPEC §8.2, D-69). | `src/erasure.rs` |
| `LicenseError` | `/// No valid activation found for the requested feature ring. #[error("no valid activation found for feature ring {0:?}")] NotActivated(FeatureRing), /// Activation signature verification failed. #[e…` | Fehlerklasse für Lizenz-/Aktivierungsprüfung — crate-lokal, an der Grenze in `ActaError` konvertierbar (P6-konform, siehe Spec §17). | `src/license.rs` |
| `ObservedPhase` | `/// Read-gate is established; all future reads will see target as erased. Gated, /// Physical purge and verification complete. Completed, /// Access restriction policy is activated (D-116). Restricte…` | Observed lifecycle phase of an erasure operation (SPEC D-118). | `src/observer.rs` |
| `ObserverError` | `/// Delivery failed due to transient network or backend error. #[error("transient delivery failure: {0}")] Transient(String), /// Delivery rejected permanently by the receiver. #[error("permanent del…` | Error returned by an [`ErasureObserver`] during notification delivery (SPEC D-118). | `src/observer.rs` |
| `PluginError` | `/// Dependency cycle or unresolvable missing dependency detected among plugins. #[error("dependency cycle detected among plugins: {0:?}")] DependencyCycle(Vec<String>), /// Conflicting plugins reques…` | Errors during plugin registration, activation, or dependency resolution. | `src/plugin.rs` |
| `RangeGate` | `/// Range contains erased data: records may be safely discarded. ContainsErased, /// Range is restricted: records must never be discarded until release. RestrictedNeverDiscard, /// Entire range is co…` | Decision variant for range operations during compaction (SPEC §4.2, I-18). | `src/gate.rs` |
| `ScanTechnique` | `/// Direkter struktureller Scan. Structural, /// Scan über entschlüsselte Ansicht. DecryptedView, /// Index-Introspektion. IndexIntrospection, /// Nadel-Scan (Needle Scan). NeedleScan, /// Schicht wu…` | Angewandte Methode/Technik während der Verifikation. | `src/scanner/layer.rs` |
| `SignerError` | `/// Signing key unavailable or corrupted. #[error("signing key unavailable: {0}")] KeyUnavailable(String), /// Hardware security module or external signature provider error. #[error("external signer…` | Error class for report signing operations (SPEC D-130, 7.5). | `src/report_signer.rs` |
| `TargetKind` | `/// Single document erasure. Document, /// Subject-level erasure (all documents belonging to a subject). Subject, /// Collection-level erasure. Collection, /// Tenant-level erasure. Tenant, /// Docum…` | Target kind indicating which hierarchy level was targeted (SPEC D-118, P13). | `src/observer.rs` |
| `VectorDeleteMode` | `/// Synchronous graph repair during vector deletion. SynchronousRepair, /// Asynchronous background graph repair during vector deletion. BackgroundRepair,` | Vector deletion mode for vector index maintenance (§15). | `src/license.rs` |
| `VerifiedVia` | `/// Read bypassed page cache via Direct I/O (`O_DIRECT` or `F_NOCACHE`). Direct, /// Read performed after page cache eviction hint (`posix_fadvise(POSIX_FADV_DONTNEED)`). CacheDropped, /// Read perfo…` | Method via which medium verification was performed (SPEC D-113). | `src/vfs.rs` |
| `WitnessLayer` | `/// LSM Memtable LsmMemtable, /// SsTable Dateien über alle Level SsTableAllLevels, /// HNSW Vektorgraph-Index HnswIndex, /// Write-Ahead-Log Segmente WalAllSegments, /// Compressed Sparse Row Graph…` | Identifikator für die physische Storage-Schicht. | `src/scanner/layer.rs` |

### Typ-Aliase (4)

| Alias | Definition | Beschreibung |
|---|---|---|
| `BoxFuture` | `type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>` | Type alias for a pinned, heap-allocated `Future` that is `Send` and dyn-compatible. |
| `BoxStream` | `type BoxStream<'a, T> = Pin<Box<dyn futures_util::stream::Stream<Item = T> + Send + 'a>>` | Type alias for a pinned, heap-allocated `Stream` that is `Send` and dyn-compatible. |
| `SignerId` | `type SignerId= u8` | Signer ID per SPEC F.10 (0 = internal/derived K_sign, 1 = operator signer). |
| `Timestamp` | `type Timestamp= u64` | Type representing a millisecond or microsecond timestamp (SPEC D-118). |

### Konstanten (1)

| Konstante | Typ | Beschreibung |
|---|---|---|
| `MAX_SCAN_MERGE_ACCUMULATOR` | `usize` | Harte Obergrenze für die Anzahl distinkter Keys, die eine StorageEngine- Implementierung während eines einzelnen scan()/scan_prefix_bounded()-Aufrufs… |

### Öffentliche Funktionen (75)

| Signatur | Beschreibung | Datei |
|---|---|---|
| `pub fn new(token: Option<Arc<()>>) -> Self` | Creates a new `BarrierShared` token. | `src/barrier.rs` |
| `pub fn new(token: Option<Arc<()>>) -> Self` | Creates a new `BarrierExclusive` token. | `src/barrier.rs` |
| `pub fn new(token: Option<Arc<()>>) -> Self` | Creates a new `CompactionPermit` token. | `src/barrier.rs` |
| `pub fn new(token: Option<Arc<()>>) -> Self` | Creates a new `PurgePermit` token. | `src/barrier.rs` |
| `pub fn assert_attention_exporter_contract<T: AttentionExporter>(t: &T, req: RequestId)` | Verifies that [`AttentionExporter`] methods can be invoked. | `src/conformance.rs` |
| `pub fn assert_checkpoint_contract<T: Checkpoint>(t: &T, tx: TxId, state: &WorkflowState)` | Verifies that [`Checkpoint`] methods can be invoked in generic contexts. | `src/conformance.rs` |
| `pub async fn assert_checkpoint_coordinator_contract<T: CheckpointCoordinator>( t: &T, tx: TxId, metadata: serde_json::Value, )` | Verifies that [`CheckpointCoordinator`] methods can be invoked in generic contexts. | `src/conformance.rs` |
| `pub fn assert_egress_policy_contract<T: crate::policy::EgressPolicy>( t: &T, ctx: &RequestContext, payload: &str, )` | Verifies dynamic object-safety and evaluation conformance of [`crate::policy::EgressPolicy`]. | `src/conformance.rs` |
| `pub fn assert_embedding_contract< T: EmbeddingProvider + LlmTextGeneratorStreaming + TextEmbeddingEngine + LlmTextGenerator + TextGenerator, >( t: &T, tenant: TenantId, seg: &[ContextSegment], )` | Verifies that [`EmbeddingProvider`], [`LlmTextGeneratorStreaming`], [`LlmTextGenerator`], and [`TextGenerator`] can be invoked. | `src/conformance.rs` |
| `pub fn assert_erasure_layer_contract<E: crate::erasure::ErasureLayer>( e: &E, target: &crate::erasure::EraseTarget, scope: &crate::erasure::LayerScope, )` | Verifies that [`crate::erasure::ErasureLayer`] methods can be invoked. | `src/conformance.rs` |
| `pub fn assert_graph_index_contract<T: GraphIndex + CommunityResolver>( t: &T, tx: TxId, e1: EntityId, e2: EntityId, )` | Verifies that [`GraphIndex`] and [`CommunityResolver`] methods can be invoked. | `src/conformance.rs` |
| `pub fn assert_keys_contract<S: crate::keys::RecordSealer, K: crate::keys::FileKeyring>( s: &S, k: &K, file_id: &crate::keys::FileId, ctx: &crate::keys::SealCtx, key: &crate::gate::KeyBuf, )` | Verifies that [`crate::keys::RecordSealer`] and [`crate::keys::FileKeyring`] methods can be invoked. | `src/conformance.rs` |
| `pub fn assert_ledger_contract<L: crate::ledger::LedgerSink>(l: &L)` | Verifies that [`crate::ledger::LedgerSink`] methods can be invoked. | `src/conformance.rs` |
| `pub fn assert_license_contract<T: LicenseGate>(t: &T, ring: FeatureRing)` | Verifies that [`LicenseGate`] methods can be invoked. | `src/conformance.rs` |
| `pub fn assert_lifecycle_contract< T: GroundingValidator + ResponseGroundingValidator + ContextPreparer, >( t: &T, chunks: &[ContextChunk], assessment: &GroundingAssessment, budget: &TokenBudget, )` | Verifies that [`GroundingValidator`], [`ResponseGroundingValidator`], and [`ContextPreparer`] can be invoked. | `src/conformance.rs` |
| `pub fn assert_observability_contract<T: DriftStatusProvider>(t: &T)` | Verifies that [`DriftStatusProvider`] methods can be invoked. | `src/conformance.rs` |
| `pub fn assert_purger_contract<T: Purger>(t: &T, ctx: &RequestContext, target: &ScanTarget)` | Verifies that [`Purger`] methods can be invoked. | `src/conformance.rs` |
| `pub fn assert_read_gate_contract<G: crate::gate::ReadGate>( g: &G, b: &crate::gate::KeyBound, r: &crate::gate::KeyRange, k: &crate::gate::KeyBuf, )` | Verifies that [`crate::gate::ReadGate`] methods can be invoked in generic contexts. | `src/conformance.rs` |
| `pub fn assert_reranker_contract<T: Reranker>(t: &T, results: &[RerankResult])` | Verifies that [`Reranker`] methods can be invoked. | `src/conformance.rs` |
| `pub fn assert_storage_engine_contract<T: StorageEngine>( t: &T, key: &[u8], val: &[u8], tx: TxId, )` | Verifies that [`StorageEngine`] methods can be invoked. | `src/conformance.rs` |
| `pub fn assert_text_index_contract<T: TextIndex + SegmentSynthesizer>(t: &T)` | Verifies that [`SegmentSynthesizer`] methods can be invoked. | `src/conformance.rs` |
| `pub async fn assert_vector_index_contract<T: VectorIndex + HybridSearchProvider>( t: &T, doc: DocId, )` | Verifies that [`VectorIndex`] and [`HybridSearchProvider`] methods can be invoked. | `src/conformance.rs` |
| `pub fn assert_vfs_file_contract<F: crate::vfs::VfsFile>(f: &F, buf: &mut [u8])` | Verifies that [`crate::vfs::VfsFile`] methods can be invoked in generic contexts. | `src/conformance.rs` |
| `pub fn assert_visitor_methods()` | Internal visitor conformance verification for KV deserialization. | `src/conformance.rs` |
| `pub fn new() -> Self` | Creates a new [`MemVfs`]. | `src/contract_tests.rs` |
| `pub fn verify_clock_contract<C: Clock>(clock: &C)` | Contract test helper verifying [`Clock`] trait implementation compliance. Checks monotonic non-decreasing behavior and valid wall-clock epo… | `src/contract_tests.rs` |
| `pub fn verify_layer_scanner_contract<L: LayerScanner>(scanner: &L, clock: &dyn Clock)` | Contract test helper verifying [`LayerScanner`] trait implementation compliance. Verifies positive control, scanned unit registration, and… | `src/contract_tests.rs` |
| `pub fn verify_metrics_sink_contract<M: MetricsSink>(sink: &M)` | Contract test helper verifying [`MetricsSink`] trait implementation compliance. | `src/contract_tests.rs` |
| `pub fn verify_purger_contract<P: Purger>(purger: &P)` | Contract test helper verifying [`Purger`] trait implementation compliance. Verifies idempotency and safe fail-closed execution. | `src/contract_tests.rs` |
| `pub fn verify_rng_contract<R: Rng>(rng: &mut R)` | Contract test helper verifying [`Rng`] trait implementation compliance. Checks non-zero outputs, correct buffer filling, and reproducible s… | `src/contract_tests.rs` |
| `pub async fn verify_storage_engine_contract<S: StorageEngine>(storage: &S)` | Contract test helper verifying [`StorageEngine`] trait implementation compliance. Verifies put/get, isolation, prefix scanning, delete, and… | `src/contract_tests.rs` |
| `pub async fn verify_text_index_contract<T: TextIndex>(index: &T)` | Contract test helper verifying [`TextIndex`] trait implementation compliance. Verifies text insertion, search, delete, len, and commit. | `src/contract_tests.rs` |
| `pub async fn verify_vector_index_contract<V: VectorIndex>(index: &V)` | Contract test helper verifying [`VectorIndex`] trait implementation compliance. Verifies insertion, search, delete, len, and transaction li… | `src/contract_tests.rs` |
| `pub fn verify_vfs_contract<V: Vfs>(vfs: &V, base_dir: &Path)` | Contract test helper verifying [`Vfs`] trait implementation compliance. Verifies atomic replacement, read roundtrips, directory creation, f… | `src/contract_tests.rs` |
| `pub fn new(chunk_id: u64, text: &'a str) -> Self` | Creates a new `ContextSegment` with default optional metadata set to `None`. | `src/embedding.rs` |
| `pub fn with_fingerprint(mut self, fingerprint: &'a ModelFingerprint) -> Self` | Builder method to set the model fingerprint. | `src/embedding.rs` |
| `pub fn with_rope_offset(mut self, offset: usize) -> Self` | Builder method to set the RoPE offset. | `src/embedding.rs` |
| `pub fn affected_units(&self) -> &[u64]` | Returns affected units count. | `src/erasure.rs` |
| `pub fn as_bytes(&self) -> &[u8` | Returns a slice of raw bytes. | `src/erasure.rs` |
| `pub fn new(scope_id: [u8; 16], affected_units: Vec<u64>) -> Self` | Creates a new `LayerScope` with specific units. | `src/erasure.rs` |
| `pub fn scope_id(&self) -> &[u8` | Returns the unique scope ID. | `src/erasure.rs` |
| `pub fn as_bytes(&self) -> &[u8` | Returns raw bytes slice. | `src/gate.rs` |
| `pub fn contains(&self, key: &KeyBuf) -> bool` | Returns `true` if `key` falls within `[start, end)`. | `src/gate.rs` |
| `pub fn new(role: RoleId, entity: EntityId) -> Self` | Creates a new `RoleBinding`. | `src/graph.rs` |
| `pub fn as_bytes(&self) -> &[u8]` | Returns a reference to the raw secret bytes. | `src/keys.rs` |
| `pub fn file_id(&self) -> FileId` | Associated file ID. | `src/keys.rs` |
| `pub fn new(raw: Vec<u8>, file_id: FileId) -> Self` | Creates a new `FileKey`. | `src/keys.rs` |
| `pub fn new(payload: Vec<u8>) -> Self` | Creates a new envelope. | `src/keys.rs` |
| `pub fn payload(&self) -> &[u8]` | Returns payload bytes. | `src/keys.rs` |
| `pub fn compute_chained( commitment: [u8; 32], range_set: RangeSet, gated_seq: Seq, completed_at: Timestamp, prev_hash: [u8; 32], ledger_mac: [u8; 32], ) -> Self` | Constructs a ledger entry and computes its chained BLAKE3 hash from the prior entry hash. | `src/ledger.rs` |
| `pub fn new(ranges: Vec<(KeyBuf, KeyBuf)>) -> Self` | Creates a range set from intervals. | `src/ledger.rs` |
| `pub fn ranges(&self) -> &[(KeyBuf, KeyBuf)]` | Returns the slice of ranges. | `src/ledger.rs` |
| `pub fn verify_chain_hash(&self, prev_hash: &[u8; 32]) -> bool` | Verifies the entry's chain hash against the supplied previous block hash. | `src/ledger.rs` |
| `pub fn ring(&self) -> FeatureRing` | Returns the authorized [`FeatureRing`]. | `src/license.rs` |
| `pub fn activate_all( &mut self, plugins: Vec<Box<dyn PluginManifest>>, ) -> Result<(), PluginError>` | Activates a batch of plugins in an all-or-nothing manner (INV-PLUGIN-DEPENDENCY). Sequence: 1. Conflict checks across requested and active… | `src/plugin.rs` |
| `pub fn current_feature_ring(&self) -> FeatureRing` | Returns the highest currently active `FeatureRing` according to the license gate. | `src/plugin.rs` |
| `pub fn is_active(&self, name: &str) -> bool` | Checks whether a plugin with the given name is currently active. | `src/plugin.rs` |
| `pub fn new( name: &'static str, version: (u32, u32, u32), ring: u8, feature_ring_required: FeatureRing, ) -> Self` | Creates a new `PluginCapability`. | `src/plugin.rs` |
| `pub fn new(gate: Arc<dyn LicenseGate>) -> Self` | Creates a new `PluginRegistry` bounded by the given `LicenseGate`. | `src/plugin.rs` |
| `pub fn snapshot(&self) -> Vec<PluginCapability>` | Takes a snapshot of all active plugin capabilities. | `src/plugin.rs` |
| `pub fn as_str(&self) -> &str` | Gibt den String-Namen der Schicht zurück. | `src/scanner/layer.rs` |
| `pub fn new(target_id: impl Into<String>, target_hash: [u8; 32]) -> Self` | Erzeugt ein neues ScanTarget. | `src/scanner/layer.rs` |
| `pub fn record_control_match(&mut self)` | Registriert eine bestandene Positivkontrolle (Control Match). | `src/scanner/session.rs` |
| `pub fn record_unit(&mut self, unit_hash: [u8; 32], is_live_residual: bool)` | Registriert eine gescannte Einheit. | `src/scanner/session.rs` |
| `pub fn digest(&self) -> &[u8` | Gibt den BLAKE3-Digest der Messung zurück. | `src/scanner/witness.rs` |
| `pub fn layer(&self) -> &str` | Gibt die gescannte Storage-Schicht als &str zurück. | `src/scanner/witness.rs` |
| `pub fn measured_at(&self) -> u64` | Gibt den Zeitstempel der Messung (Unix Nanos) zurück. | `src/scanner/witness.rs` |
| `pub fn method(&self) -> &ScanMethod` | Gibt die ScanMethod-Entsprechung zurück. | `src/scanner/witness.rs` |
| `pub fn mint_witness_for_testkit( layer: WitnessLayer, scanned: u64, remaining: u64, technique: ScanTechnique, digest: [u8; 32], measured_at: u64, ) -> LayerWitness` | Hilfsfunktion zur Witness-Erzeugung ausschließlich für Testzwecke (`testkit`-Feature). | `src/scanner/witness.rs` |
| `pub fn remaining(&self) -> u64` | Gibt die Anzahl der verbleibenden Lebendeinträge zurück. | `src/scanner/witness.rs` |
| `pub fn scanned(&self) -> u64` | Gibt die Anzahl der während der Verifikation gescannten Einheiten zurück. | `src/scanner/witness.rs` |
| `pub fn technique(&self) -> &ScanTechnique` | Gibt die angewandte Scantechnik zurück. | `src/scanner/witness.rs` |
| `pub fn witness_layer(&self) -> &WitnessLayer` | Gibt die gescannte Storage-Schicht als &WitnessLayer zurück. | `src/scanner/witness.rs` |
| `pub fn read_only() -> Self` | Creates options configured for read-only access. | `src/vfs.rs` |
| `pub fn write_create() -> Self` | Creates options configured for write-only access, creating the file if missing. | `src/vfs.rs` |


---

## `acta-mvcc`

| Attribut | Wert |
|---|---|
| **Ring** | 1 — Ebene E1 — Speicherschicht, Indizes & KI-Kerne |
| **Status** | Fertig |
| **Zweck** | SeqLog, SnapshotRegistry und transaktionaler TxBuffer (MVCC-Isolation). |
| **Unsafe-Code** | Nein (`#![forbid(unsafe_code)]`) |
| **Interne Abhängigkeiten** | `acta-env`, `acta-ids`, `acta-ports`, `acta-types` |
| **Externe Abhängigkeiten (Auswahl)** | `ahash`, `bytes`, `parking_lot`, `serde`, `thiserror`, `tracing` |

### Modul- und Re-Export-Oberfläche (lib.rs)

```rust
pub mod compat;
pub mod mvcc_error;
pub mod seq_log;
pub mod snapshot;
pub mod ssi;
pub mod tombstone;
pub mod tx_buffer;
pub mod types { .. }
pub use crate::mvcc_error::{MvccError, Result as MvccResult};
pub use acta_ids::{DocId, TenantId, TxId, TOMBSTONE_BIT};
pub use acta_ids::{DocId, TenantId, TxId, TOMBSTONE_BIT};
pub use crate::compat::{workspace_error as error, ActaError, Result};
pub use crate::mvcc_error::Result;
pub use seq_log::{SeqLogChange, SeqLogEntry, SequenceLog};
pub use snapshot::{SnapshotGuard, SnapshotRegistry};
pub use ssi::{PruningBlockerInfo, ReadSet, SequenceLogSsiValidator, SsiValidator};
pub use tombstone::{SeqBitTombstone, TombstoneSemanticsCheck};
pub use tx_buffer::{IndexOp, TxBuffer};
```

### Feature-Flags

| Flag | Aktiviert |
|---|---|
| `default` | `["workspace"]` |
| `workspace` | `["dep:acta-types", "dep:acta-ports"]` |
| `types-compat` | `["workspace"]` |
| `ports-compat` | `["workspace"]` |
| `docid-128` | `["acta-ids/docid-128"]` |

### Schnittstellen — Public Traits (3)

**`SsiValidator`** — *Schnittstelle (Trait) für die Validierung von Transaktionen unter Serializable Snapshot Isolation (SSI). Prüft vor dem Festschreiben (Commit) einer Transaktion, ob nebenläufige Transaktionen Schlüssel oder Präfixe verändert haben, die im [`ReadSet`] dieser Tr…* (`src/ssi.rs`)

```rust
pub trait SsiValidator {
/// Validiert das [`ReadSet`] der Transaktion `tx_id`.
    ///
    /// ### Fehlerfälle:
    /// Gibt `Err(MvccError::Conflict(...))` zurück, falls:
    /// - Ein Schlüssel im ReadSet nach dem Lese-Snapshot von einer anderen Transaktion committet wurde (Write-Skew).
    /// - Ein neuer Schlüssel committet wurde, der auf einen gelesenen Bereichsscan-Präfix zutrifft (Phantom-Read).
    /// - Der Snapshot-Wasserstand der Transaktion älter ist als die bereits bereinigte Historie (`pruned_through`).
    ///
    /// ### Nebenläufigkeits-Invariante:
    /// Aufrufer koordinieren Validierung und Commit-Registrierung (z. B. via Commit-Gate-Lock),
    /// um atomare, konfliktfreie Zustandsübergänge sicherzustellen.
    fn validate(&self, tx_id: TxId, read_set: &ReadSet) -> Result<()>;
}
```

**`StagedOpSize`** — *Trait zur Schätzung des Speicherverbrauchs in Bytes für gepufferte Nutzdaten. Ermöglicht genaue Buchführung des verbrauchten Hauptspeichers pro Transaktion und global.* (`src/tx_buffer.rs`)

```rust
pub trait StagedOpSize {
/// Liefert den geschätzten Speicherbedarf der Nutzdaten in Bytes.
    fn staged_bytes(&self) -> usize;
}
```

**`TombstoneSemanticsCheck`** — *Trait zur Bestimmung und Erzeugung von Tombstones in Sequenzprotokollen und Indizes. NOTE: In Acta wird das höchstwertige Bit (`TOMBSTONE_BIT` = 1 << 63) als Indikator für gelöschte Versionen verwendet. Zusätzlich können Low-Level-Flags (z. B. Bit 0x01) signa…* (`src/tombstone.rs`)

```rust
pub trait TombstoneSemanticsCheck {
/// Gibt `true` zurück, falls die Sequenznummer oder die Flags eine gelöschte Version (Tombstone) anzeigen.
    fn is_tombstone(seq: u64, flags: u8) -> bool;

    /// Kodiert eine logische Sequenznummer als 8-Byte Little-Endian Bytevektor mit gesetztem `TOMBSTONE_BIT`.
    fn make_tombstone(seq: u64) -> Vec<u8>;
}
```

### Öffentliche Datentypen — Structs (11)

| Struct | Felder / Form | Beschreibung | Datei |
|---|---|---|---|
| `PruningBlockerInfo` | `{ /// Kleinste Sequenznummer, die noch unbereinigt im Register liegt. pub min_unpruned_seq: u64, /// Aktueller `pruned_through`-Wasserstand. pub pruned_through_s… }` | Diagnoseinformationen über aktive Snapshots oder Sequenznummern, die das Pruning im Validator blockieren. Wird erzeugt, wenn der Speicher für Commit-… | `src/ssi.rs` |
| `ReadSet` | `{ /// Zuordnung von gelesenen Schlüsseln (Bytefolgen) zur zugehörigen Snapshot-Sequenznummer. keys: AHashMap<Vec<u8>, u64>, /// Zuordnung von Scan-Präfixen zu de… }` | Protokolliert gelesene Einzelschlüssel, Präfixe und deren jeweilige Snapshot-Sequenznummer für SSI. ### Speicher- und Risikoanalyse: Der Speicherbeda… | `src/ssi.rs` |
| `SeqBitTombstone` | `(tuple/unit)` | Standard-Referenzimplementierung für [`TombstoneSemanticsCheck`]. Nutzt bitweise ODER-Verknüpfung mit [`TOMBSTONE_BIT`] sowie Flag-Prüfung auf Bit 0. | `src/tombstone.rs` |
| `SeqLogEntry` | `{ /// Eindeutige Kennung des Dokuments. pub doc_id: DocId, /// Logische Sequenznummer, bei der dieser Dokumenteintrag eingefügt wurde. pub insert_seq: u64, /// L… }` | Ein versionierter Protokolleintrag für die Snapshot-Isolation (`_at`-Familie). Jeder Eintrag repräsentiert die Lebensspanne einer Dokumentversion: vo… | `src/seq_log.rs` |
| `SequenceLog` | `{ /// Liste aller historischen Dokumentversionen. entries: Vec<SeqLogEntry>, /// Zuordnung von gepinnter Sequenznummer zu Liste von Erstellungs-Zeitstempeln (in… }` | Verwaltet das chronologische Sequenz-Protokoll und Sichtbarkeitsfilter für Indexstrukturen. ### Funktionsweise: `SequenceLog` speichert alle Einfüge-… | `src/seq_log.rs` |
| `SequenceLogSsiValidator` | `{ committed_writes: Arc<RwLock<CommittedWrites>>, sequence_log: Option<Arc<RwLock<SequenceLog>>>, snapshot_registry: Option<Arc<SnapshotRegistry>>, pruned_throug… }` | Referenz-Implementierung von [`SsiValidator`] auf Basis von [`SequenceLog`] und In-Memory-Commit-Tracking. Arbeitet vollständig ohne Festplatten-I/O… | `src/ssi.rs` |
| `SnapshotGuard` | `{ registry: Arc<SnapshotRegistry>, seq_no: u64, created_at: Option<u64>, tenant: Option<TenantId>, }` | RAII-Guard (Resource Acquisition Is Initialization) für einen aktiven Lese-Snapshot. Hält einen Snapshot in der [`SnapshotRegistry`] gepinnt, solange… | `src/snapshot.rs` |
| `SnapshotRegistry` | `{ active: Mutex<BTreeMap<u64, Vec<SnapshotMeta>>>, min_active_seqno: AtomicU64, clock: Arc<dyn Clock>, config: SnapshotRegistryConfig, }` | Zentrale Registrierung für aktive Lese-Snapshots im System. ### Synchronisationsstrategie: - `active`: Ein sortierter `BTreeMap`, geschützt durch ein… | `src/snapshot.rs` |
| `SnapshotRegistryConfig` | `{ /// Maximale Anzahl gleichzeitig aktiver Snapshots (Default: 100.000). pub max_active_snapshots: usize, /// Maximales Alter eines Snapshots (Default: 300 s / 5… }` | Konfigurationsoptionen für die [`SnapshotRegistry`]. | `src/snapshot.rs` |
| `TxBuffer` | `{ shards: Vec<RwLock<TxShard<T>>>, key_shards: Vec<RwLock<KeyShard>>, tx_timeout: Duration, config: TxBufferConfig, global_staged_bytes: AtomicUsize, clock: Arc<… }` | Verwaltet gepufferte Transaktionen, deren Staging-Operationen und ReadSets. ### Synchronisations- und Sperrstrategie: Jeder Shard besitzt ein eigenes… | `src/tx_buffer.rs` |
| `TxBufferConfig` | `{ /// Timeout-Dauer, nach der verwaiste (inaktive) Transaktionen automatisch abgeräumt werden. pub tx_timeout: Duration, /// Maximale empfohlene Anzahl gleichzei… }` | Konfigurationsoptionen für den [`TxBuffer`]. | `src/tx_buffer.rs` |

### Öffentliche Datentypen — Enums (3)

| Enum | Varianten | Beschreibung | Datei |
|---|---|---|---|
| `IndexOp` | `/// Einfügen eines Dokuments mit Nutzlast (z. B. Einbettungs-Vektoren oder Schlüssel-Werte). Insert { /// Kennung des Dokuments. doc_id: DocId, /// Assoziierte Nutzdaten. data: T, }, /// Entfernen ei…` | Eine im Puffer vorgemerkte Operation auf einem Index. | `src/tx_buffer.rs` |
| `MvccError` | `/// Nebenläufigkeits- oder SSI-Konflikt (Write-Skew, Anti-Dependency, Phantom-Read). #[error("Conflict: {0}")] Conflict(String), /// Ressourcen- oder Kapazitätsgrenze überschritten (z. B. maximale ak…` | Lokaler Fehlertyp für Multi-Version Concurrency Control und Transaktionsverwaltung. | `src/mvcc_error.rs` |
| `SeqLogChange` | `/// Einfügen eines Dokuments zu einer bestimmten Sequenznummer. Insert { /// Eindeutige Kennung des Dokuments. doc_id: DocId, /// Sequenznummer des Einfügens. seq: u64, }, /// Löschen eines Dokuments…` | Repräsentiert eine historische Änderung (Einfügen oder Löschen) im Sequenz-Log für Delta-Replays. Wird von Abfragen wie [`SequenceLog::changes_since`… | `src/seq_log.rs` |

### Typ-Aliase (1)

| Alias | Definition | Beschreibung |
|---|---|---|
| `Result` | `type Result<T, E = MvccError> = std::result::Result<T, E>` | Lokaler Result-Typ für MVCC-Operationen. |

### Konstanten (8)

| Konstante | Typ | Beschreibung |
|---|---|---|
| `DEFAULT_MAX_OPS_PER_TX` | `usize` | Empfohlene Maximalanzahl an Operationen pro Einzelschreibtransaktion (10.000 Operationen). |
| `DEFAULT_MAX_PIN_DURATION` | `Duration` | Standard-Lebensdauer für Snapshot-Pins (5 Minuten), bevor Diagnose-Warnungen für blockierende oder verwaiste Pins ausgelöst werden. |
| `DEFAULT_MAX_READ_SET_KEYS` | `usize` | Standard-Maximalanzahl an getrackten Lese-Schlüsseln pro Transaktion (100.000 Schlüssel). |
| `DEFAULT_MAX_TOTAL_STAGED_BYTES` | `usize` | Standard-Maximalgröße für den globalen Puffer über alle Transaktionen hinweg (256 MiB). |
| `DEFAULT_MAX_TRACKED_COMMIT_KEYS` | `usize` | Standard-Höchstgrenze für im Speicher gehaltene bestätigte Schreib-Schlüssel (1.000.000 Schlüssel). |
| `DEFAULT_MAX_TX_STAGED_BYTES` | `usize` | Standard-Maximalgröße für gepufferte Rohdaten pro Einzelschreibtransaktion (16 MiB). |
| `DEFAULT_SHARD_COUNT` | `usize` | Standardanzahl an Partitionen (Shards) im Transaktionspuffer (64 Shards). |
| `STAGING_ENTRY_OVERHEAD_BYTES` | `usize` | Fester Speicher-Overhead pro gepufferter Operation (32 Bytes). |

### Öffentliche Funktionen (137)

| Signatur | Beschreibung | Datei |
|---|---|---|
| `pub fn limit_exceeded(limit: usize, context: impl Into<String>) -> Self` | Hilfskonstruktor zur Erzeugung eines [`MvccError::LimitExceeded`]. | `src/mvcc_error.rs` |
| `pub fn changes_since(&self, snapshot_seq: u64) -> Vec<SeqLogChange>` | Liefert alle Änderungen (Einfügungen und Entfernungen), die strikt nach `snapshot_seq` stattgefunden haben, aufsteigend sortiert nach Seque… | `src/seq_log.rs` |
| `pub fn clock(&self) -> &Arc<dyn Clock>` | Gibt eine Referenz auf den aktiven [`Clock`]-Port zurück. | `src/seq_log.rs` |
| `pub fn compact(&mut self, min_active_seqno: u64)` | Verdichtet (kompaktiert) das Sequenz-Log und entfernt historische Versionen, deren Entfernung strikt kleiner als `min_active_seqno` ist. ##… | `src/seq_log.rs` |
| `pub fn deletion_seq(&self, doc_id: DocId) -> Option<u64>` | Liefert die Lösch-Sequenznummer des Dokuments, falls es aktuell als gelöscht verzeichnet ist. | `src/seq_log.rs` |
| `pub fn doc_id(&self) -> DocId` | Liefert die Kennung des betroffenen Dokuments. | `src/seq_log.rs` |
| `pub fn expired_pins(&self) -> Vec<u64>` | Liefert alle abgelaufenen Snapshot-Pins bezogen auf die aktuelle Systemuhr. | `src/seq_log.rs` |
| `pub fn expired_pins_at(&self, now_nanos: u64) -> Vec<u64>` | Findet alle gepinnten Sequenznummern, deren Haltezeit `max_pin_duration` überschreitet, bezogen auf den übergebenen Monotonie-Zeitstempel `… | `src/seq_log.rs` |
| `pub fn is_empty(&self) -> bool` | Gibt `true` zurück, wenn das Sequenz-Log keine Einträge enthält. | `src/seq_log.rs` |
| `pub fn is_visible(&self, as_of: u64) -> bool` | Prüft in O(1), ob dieser Protokolleintrag zu einem bestimmten Snapshot-Stand `as_of` sichtbar ist. ### Formale Sichtbarkeitsinvariante (INV… | `src/seq_log.rs` |
| `pub fn is_visible(&self, doc_id: DocId, seq_no: u64) -> bool` | Prüft, ob ein Dokument zu einer bestimmten Sequenznummer `seq_no` im Index sichtbar ist. Ein Dokument ist sichtbar, wenn es vor oder bei `s… | `src/seq_log.rs` |
| `pub fn len(&self) -> usize` | Gibt die Anzahl der aktuell im Log vorgehaltenen Einträge zurück. | `src/seq_log.rs` |
| `pub fn max_pin_duration(&self) -> Duration` | Liefert die aktuell konfigurierte Maximaldauer für Pins. | `src/seq_log.rs` |
| `pub fn min_retention_seq(&self) -> Option<u64>` | Berechnet die kleinste aktuell gepinnte Sequenznummer bezogen auf die aktuelle Systemuhr. | `src/seq_log.rs` |
| `pub fn min_retention_seq_at(&self, now_nanos: u64) -> Option<u64>` | Berechnet die kleinste aktuell gepinnte Sequenznummer für Aufbewahrungs- und Pruning-Zwecke. Gibt `None` zurück, falls aktuell kein einzige… | `src/seq_log.rs` |
| `pub fn new() -> Self` | Erstellt ein neues, leeres `SequenceLog` mit der Standard-Systemuhr [`SystemClock`] und einer maximalen Pin-Dauer von 5 Minuten. | `src/seq_log.rs` |
| `pub fn pin_snapshot(&mut self, seq_no: u64)` | Pinnt eine historische Sequenznummer, um zu verhindern, dass die Hintergrund-Bereinigung (Rebuild/Compaction) gelöschte Knoten entfernt, di… | `src/seq_log.rs` |
| `pub fn pin_snapshot_at(&mut self, seq_no: u64, at_nanos: u64)` | Pinnt eine Sequenznummer zu einem expliziten Nanosekunden-Zeitstempel (für deterministische Tests). | `src/seq_log.rs` |
| `pub fn reap_expired_pins(&mut self) -> Vec<u64>` | Bereinigt alle abgelaufenen Snapshot-Pins bezogen auf die aktuelle Uhrzeit. | `src/seq_log.rs` |
| `pub fn reap_expired_pins_at(&mut self, now_nanos: u64) -> Vec<u64>` | Bereinigt (entpinnt) alle Pins, die die Maximaldauer bezüglich `now_nanos` überschritten haben. ### Ablauf: 1. Abgelaufene Pins via [`Seque… | `src/seq_log.rs` |
| `pub fn record_delete(&mut self, doc_id: DocId, seq: u64)` | Zeichnet eine Löschoperation für ein Dokument zu einer Sequenznummer auf. ### Ablauf: 1. Tombstone-Bit der Sequenznummer ausmaskieren. 2. D… | `src/seq_log.rs` |
| `pub fn record_insert(&mut self, doc_id: DocId, seq: u64)` | Zeichnet eine Einfügeoperation für ein Dokument zu einer Sequenznummer auf. ### Ablauf: 1. Tombstone-Bit der Sequenznummer ausmaskieren. 2.… | `src/seq_log.rs` |
| `pub fn seq(&self) -> u64` | Liefert die logische Sequenznummer dieser Änderung. | `src/seq_log.rs` |
| `pub fn set_clock(&mut self, clock: Arc<dyn Clock>)` | Ersetzt die aktive [`Clock`]-Instanz für dieses Log. | `src/seq_log.rs` |
| `pub fn set_max_pin_duration(&mut self, duration: Duration)` | Setzt die maximale Pin-Dauer für Diagnosezwecke. | `src/seq_log.rs` |
| `pub fn unpin_snapshot(&mut self, seq_no: u64)` | Gibt einen gepinnten Snapshot wieder frei. ### Ablauf: 1. Tombstone-Bit der Sequenznummer ausmaskieren. 2. Den letzten Zeitstempel vom Stac… | `src/seq_log.rs` |
| `pub fn with_clock(clock: Arc<dyn Clock>) -> Self` | Erstellt ein neues, leeres `SequenceLog` mit einer benutzerdefinierten Implementierung des [`Clock`]-Ports (z. B. `ManualClock` in determin… | `src/seq_log.rs` |
| `pub fn with_max_pin_duration(mut self, duration: Duration) -> Self` | Konfiguriert die maximale Pin-Dauer als Fluent-Builder und gibt `self` zurück. | `src/seq_log.rs` |
| `pub fn with_time_fn<F>(time_fn: F) -> Self where F: Fn() -> u64 + Send + Sync + 'static,` | Erstellt ein `SequenceLog` mit einer anpassbaren Zeitstempel-Closure (State-Passing per `Fn() -> u64`). | `src/seq_log.rs` |
| `pub fn clock(&self) -> &Arc<dyn Clock>` | Liefert eine Referenz auf den aktiven Zeitport ([`Clock`]). | `src/snapshot.rs` |
| `pub fn config(&self) -> SnapshotRegistryConfig` | Liefert die aktuelle Konfiguration der Registry. | `src/snapshot.rs` |
| `pub fn longest_active_pin(&self) -> Option<(u64, Duration)>` | Findet den am längsten aktiven Snapshot bezogen auf die aktuelle Systemuhr. | `src/snapshot.rs` |
| `pub fn longest_active_pin_at(&self, now_nanos: u64) -> Option<(u64, Duration)>` | Findet den am längsten aktiven Snapshot bezüglich des Nanosekunden-Zeitstempels `now_nanos`. Liefert `Some((seq_no, duration))` oder `None`… | `src/snapshot.rs` |
| `pub fn min_active_seqno(&self) -> u64` | Liefert die kleinste Sequenznummer aller derzeit aktiven Snapshots (`u64::MAX`, falls keine aktiv). | `src/snapshot.rs` |
| `pub fn min_for_compaction(&self, current_seq: u64) -> u64` | Berechnet die minimale Sequenznummer für die Kompaktierung (GC-Grenze) ohne Sentinel. | `src/snapshot.rs` |
| `pub fn min_read_snapshot(&self) -> Option<u64>` | Liefert die kleinste Snapshot-Sequenznummer als `Option<u64>` (ohne Sentinel). | `src/snapshot.rs` |
| `pub fn new() -> Self` | Erstellt eine neue, leere Snapshot-Registry mit Standard-[`SystemClock`] und Standardkonfiguration. | `src/snapshot.rs` |
| `pub fn oldest_active_snapshot(&self) -> Option<u64>` | Liefert die Untergrenze für Garbage Collection (älteste aktive Snapshot-Sequenz). Entspricht dem Vertrag für Store-Kompaktierung und ist äq… | `src/snapshot.rs` |
| `pub fn pin(&self, seq_no: u64)` | Dauerhaftes Pinnen einer Sequenznummer (z. B. für SAOS Checkpoints oder Backups), ohne einen RAII-Guard zu halten. Muss manuell über [`Snap… | `src/snapshot.rs` |
| `pub fn pin_at(&self, seq_no: u64, at_nanos: u64)` | Pinnt eine Sequenznummer zu einem expliziten Zeitstempel. | `src/snapshot.rs` |
| `pub fn pin_for_tenant(&self, tenant: TenantId, seq_no: u64)` | Pinnt eine Sequenznummer für einen Mandanten bezogen auf die aktuelle Systemuhr. | `src/snapshot.rs` |
| `pub fn pin_for_tenant_at(&self, tenant: TenantId, seq_no: u64, at_nanos: u64)` | Pinnt eine Sequenznummer für einen Mandanten zu einem expliziten Zeitstempel. | `src/snapshot.rs` |
| `pub fn reap_expired_pins(&self, max_pin_duration: Duration) -> Vec<u64>` | Bereinigt alle überalterten Snapshot-Pins bezogen auf die aktuelle Uhrzeit. | `src/snapshot.rs` |
| `pub fn reap_expired_pins_at(&self, max_pin_duration: Duration, now_nanos: u64) -> Vec<u64>` | Bereinigt und entfernt alle aktiven Snapshot-Pins, die älter als `max_pin_duration` sind. ### Ablauf: 1. Berechne Schwellenwert in Nanoseku… | `src/snapshot.rs` |
| `pub fn register(self: &Arc<Self>, seq_no: u64) -> SnapshotGuard` | Registriert einen Lese-Snapshot und liefert einen RAII-Guard ([`SnapshotGuard`]) zurück. Sobald der Guard den Gültigkeitsbereich verlässt (… | `src/snapshot.rs` |
| `pub fn register_at(self: &Arc<Self>, seq_no: u64, at_nanos: u64) -> SnapshotGuard` | Registriert einen Lese-Snapshot mit explizitem Zeitstempel ohne Mandantenzuordnung. | `src/snapshot.rs` |
| `pub fn register_for_tenant(self: &Arc<Self>, tenant: TenantId, seq_no: u64) -> SnapshotGuard` | Registriert einen Lese-Snapshot für einen bestimmten Mandanten. | `src/snapshot.rs` |
| `pub fn register_for_tenant_at( self: &Arc<Self>, tenant: TenantId, seq_no: u64, at_nanos: u64, ) -> SnapshotGuard` | Registriert einen Lese-Snapshot für einen bestimmten Mandanten mit explizitem Zeitstempel. | `src/snapshot.rs` |
| `pub fn revoke_covering( &self, tenant: TenantId, range: std::ops::Range<u64>, gated_seq: u64, ) -> usize` | Widerruft Snapshots im Bereich für einen Mandanten (Tenant-Isolation). | `src/snapshot.rs` |
| `pub fn seq_no(&self) -> u64` | Liefert die Sequenznummer, die durch diesen Guard geschützt wird. | `src/snapshot.rs` |
| `pub fn set_config(&mut self, config: SnapshotRegistryConfig)` | Setzt eine neue Konfiguration für die Registry. | `src/snapshot.rs` |
| `pub fn tenant(&self) -> Option<TenantId>` | Liefert den zugeordneten Mandanten, falls angegeben. | `src/snapshot.rs` |
| `pub fn try_register(self: &Arc<Self>, seq_no: u64) -> Result<SnapshotGuard>` | Versucht, einen Lese-Snapshot zu registrieren. Im Gegensatz zu `register` schlägt diese Methode fail-closed mit `MvccError::LimitExceeded`… | `src/snapshot.rs` |
| `pub fn try_register_at(self: &Arc<Self>, seq_no: u64, at_nanos: u64) -> Result<SnapshotGuard>` | Versucht, einen Lese-Snapshot zu einem expliziten Nanosekunden-Zeitstempel zu registrieren. | `src/snapshot.rs` |
| `pub fn try_register_for_tenant( self: &Arc<Self>, tenant: TenantId, seq_no: u64, ) -> Result<SnapshotGuard>` | Versucht, einen Lese-Snapshot für einen bestimmten Mandanten zu registrieren. | `src/snapshot.rs` |
| `pub fn try_register_for_tenant_at( self: &Arc<Self>, tenant: Option<TenantId>, seq_no: u64, at_nanos: u64, ) -> Result<SnapshotGuard>` | Versucht, einen Lese-Snapshot für einen Mandanten zu einem expliziten Zeitstempel zu registrieren. | `src/snapshot.rs` |
| `pub fn unpin(&self, seq_no: u64)` | Hebt einen dauerhaften Pin auf und bereinigt den minimal aktiven Wasserstand. | `src/snapshot.rs` |
| `pub fn with_clock(clock: Arc<dyn Clock>) -> Self` | Erstellt eine neue Snapshot-Registry mit benutzerdefiniertem [`Clock`]-Port (für Tests). | `src/snapshot.rs` |
| `pub fn with_clock_and_config(clock: Arc<dyn Clock>, config: SnapshotRegistryConfig) -> Self` | Erstellt eine neue Snapshot-Registry mit benutzerdefiniertem [`Clock`]-Port und Konfiguration. | `src/snapshot.rs` |
| `pub fn with_time_fn<F>(time_fn: F) -> Self where F: Fn() -> u64 + Send + Sync + 'static,` | Erstellt eine neue Snapshot-Registry mit einer Zeitstempel-Closure (State-Passing per `Fn() -> u64`). | `src/snapshot.rs` |
| `pub fn clear(&mut self)` | Leert das ReadSet vollständig und setzt die kleinste Sequenznummer zurück. | `src/ssi.rs` |
| `pub fn clock(&self) -> &Arc<dyn Clock>` | Liefert eine Referenz auf den aktiven [`Clock`]-Port. | `src/ssi.rs` |
| `pub fn diagnose_pruning_blocker(&self) -> Option<PruningBlockerInfo>` | Diagnostiziert, welcher unbereinigte Stand oder aktiver Snapshot das Pruning im Validator blockiert. | `src/ssi.rs` |
| `pub fn diagnose_pruning_blocker_at(&self, now_nanos: u64) -> Option<PruningBlockerInfo>` | Diagnostiziert Blockaden bezüglich eines expliziten Zeitstempels `now_nanos`. | `src/ssi.rs` |
| `pub fn forget_from(&self, first_seq: u64) -> usize` | Entfernt alle Commit-Registrierungen mit `commit_seq >= first_seq` (z. B. nach Transaktionsabbruch). | `src/ssi.rs` |
| `pub fn get(&self, key: &[u8]) -> Option<u64>` | Liefert die registrierte Snapshot-Sequenznummer für einen Schlüssel, falls vorhanden. | `src/ssi.rs` |
| `pub fn is_empty(&self) -> bool` | Liefert `true`, falls weder Einzelschlüssel noch Präfixe im ReadSet enthalten sind. | `src/ssi.rs` |
| `pub fn iter(&self) -> impl Iterator<Item = (&Vec<u8>, &u64)>` | Liefert einen Iterator über alle gelesenen Schlüssel und deren Snapshot-Sequenzen. | `src/ssi.rs` |
| `pub fn keys_map(&self) -> &AHashMap<Vec<u8>, u64>` | Liefert die interne Schlüssel-Map als Referenz. | `src/ssi.rs` |
| `pub fn len(&self) -> usize` | Liefert die Gesamtzahl aller getrackten Schlüssel und Präfixe. | `src/ssi.rs` |
| `pub fn max_tracked_keys(&self) -> usize` | Liefert die konfigurierte Obergrenze für Schlüssel. | `src/ssi.rs` |
| `pub fn min_snapshot_seq(&self) -> Option<u64>` | Liefert die kleinste Snapshot-Sequenznummer aller gelesenen Schlüssel und Präfixe. | `src/ssi.rs` |
| `pub fn new() -> Self` | Erstellt ein neues, leeres [`ReadSet`]. | `src/ssi.rs` |
| `pub fn new() -> Self` | Erstellt einen neuen [`SequenceLogSsiValidator`] mit Standard-Obergrenze (1.000.000 Schlüssel) und Standard-[`SystemClock`]. | `src/ssi.rs` |
| `pub fn new_with_bounds(max_tracked_keys: usize) -> Self` | Erstellt einen neuen Validator mit benutzerdefinierter Kapazitätsgrenze für Schlüssel. | `src/ssi.rs` |
| `pub fn new_with_clock_and_bounds(clock: Arc<dyn Clock>, max_tracked_keys: usize) -> Self` | Erstellt einen neuen Validator mit benutzerdefiniertem [`Clock`]-Port und Kapazitätsgrenze. | `src/ssi.rs` |
| `pub fn new_with_time_fn_and_bounds<F>(time_fn: F, max_tracked_keys: usize) -> Self where F: Fn() -> u64 + Send + Sync + 'static,` | Erstellt einen neuen Validator mit einer Zeitstempel-Closure (State-Passing per `Fn() -> u64`) und Kapazitätsgrenze. | `src/ssi.rs` |
| `pub fn prefixes_iter(&self) -> impl Iterator<Item = (&Vec<u8>, &u64)>` | Liefert einen Iterator über alle Bereichs-Präfixe und deren Snapshot-Sequenzen. | `src/ssi.rs` |
| `pub fn prefixes_map(&self) -> &AHashMap<Vec<u8>, u64>` | Liefert die interne Präfix-Map als Referenz. | `src/ssi.rs` |
| `pub fn prune_through(&self, bound_seq: u64) -> usize` | Bereinigt (prunt) bestätigte Schreib-Einträge mit `commit_seq <= bound_seq`. Aktualisiert den atomaren `pruned_through`-Wasserstand. ### Fa… | `src/ssi.rs` |
| `pub fn prune_through_checked(&self, bound: Option<u64>) -> Result<usize>` | Bereinigt bestätigte Schreib-Einträge bis zu einem optionalen Grenzwert mit expliziter Fehlerbehandlung. ### Fehlerfälle: Liefert `Err(Mvcc… | `src/ssi.rs` |
| `pub fn pruned_through(&self) -> u64` | Liefert den aktuellen `pruned_through`-Wasserstand. | `src/ssi.rs` |
| `pub fn record_commit_key(&self, key: &[u8], commit_seq: u64)` | Zeichnet einen committeten Schlüssel zu einer Sequenznummer auf. ### Ablauf: 1. Tombstone-Bit der Commit-Sequenznummer ausmaskieren. 2. Sch… | `src/ssi.rs` |
| `pub fn record_commit_keys<'a>( &self, keys: impl IntoIterator<Item = &'a [u8]>, commit_seq: u64, )` | Zeichnet eine Menge committeter Schlüssel stapelweise (Batch) zu einer Commit-Sequenznummer auf. | `src/ssi.rs` |
| `pub fn record_prefix(&mut self, prefix: impl Into<Vec<u8>>, snapshot_seq: u64)` | Zeichnet einen Bereichsscan-Präfix zu `snapshot_seq` auf, um Phantom-Reads zu erkennen. Wenn nach diesem Lesezugriff eine andere Transaktio… | `src/ssi.rs` |
| `pub fn record_read(&mut self, key: impl Into<Vec<u8>>, snapshot_seq: u64)` | Zeichnet einen Lesezugriff auf `key` zu einer bestimmten Snapshot-Sequenznummer `snapshot_seq` auf. ### Ablauf: 1. Maskiere das Tombstone-B… | `src/ssi.rs` |
| `pub fn register_read(&mut self, key: impl Into<Vec<u8>>, snapshot_seq: u64)` | Alias-Funktion für [`ReadSet::record_read`]. | `src/ssi.rs` |
| `pub fn set_clock(&mut self, clock: Arc<dyn Clock>)` | Ersetzt die aktive [`Clock`]-Instanz. | `src/ssi.rs` |
| `pub fn set_metrics_sink(&mut self, sink: Arc<parking_lot::RwLock<Arc<dyn MetricsSink>>>)` | Aktualisiert den Metrik-Empfänger. | `src/ssi.rs` |
| `pub fn set_snapshot_registry(&mut self, registry: Arc<SnapshotRegistry>)` | Setzt die verknüpfte [`SnapshotRegistry`]. | `src/ssi.rs` |
| `pub fn tracked_commit_keys(&self) -> usize` | Liefert die Anzahl aktuell gehaltener Commit-Schlüssel. | `src/ssi.rs` |
| `pub fn validate_and_record<'a>( &self, tx_id: TxId, read_set: &ReadSet, write_keys: impl IntoIterator<Item = &'a [u8]>, commit_seq: u64, ) -> Result<()>` | Atomically validates `read_set` and records `write_keys` at `commit_seq` under a single write lock. Führt die Validierung und das Eintragen… | `src/ssi.rs` |
| `pub fn with_clock(mut self, clock: Arc<dyn Clock>) -> Self` | Bindet eine benutzerdefinierte Implementierung des [`Clock`]-Ports an den Validator. | `src/ssi.rs` |
| `pub fn with_max_pin_duration(mut self, duration: Duration) -> Self` | Konfiguriert die maximale Pin-Dauer für Diagnosealarme. | `src/ssi.rs` |
| `pub fn with_metrics_sink( mut self, sink: Arc<parking_lot::RwLock<Arc<dyn MetricsSink>>>, ) -> Self` | Bindet einen Metrik-Empfänger ([`MetricsSink`]) für Monitoring an. | `src/ssi.rs` |
| `pub fn with_sequence_log(seq_log: Arc<RwLock<SequenceLog>>) -> Self` | Erstellt einen Validator, der an ein bestehendes [`SequenceLog`] angebunden ist. | `src/ssi.rs` |
| `pub fn with_snapshot_registry(mut self, registry: Arc<SnapshotRegistry>) -> Self` | Verknüpft eine [`SnapshotRegistry`] zur Überwachung aktiver Snapshot-Pins. | `src/ssi.rs` |
| `pub fn with_time_fn<F>(self, time_fn: F) -> Self where F: Fn() -> u64 + Send + Sync + 'static,` | Bindet eine Zeitstempel-Closure (State-Passing per `Fn() -> u64`) an den Validator. | `src/ssi.rs` |
| `pub fn begin(&self, tx: TxId)` | Registriert eine neue Transaktion mit der aktuellen Uhrzeit. | `src/tx_buffer.rs` |
| `pub fn begin_at(&self, tx: TxId, at_nanos: u64)` | Registriert eine neue Transaktion im Puffer zu einem bestimmten Nanosekunden-Zeitstempel. ### Ablauf: 1. Shard-Index für `tx` per Hash bere… | `src/tx_buffer.rs` |
| `pub fn clock(&self) -> &Arc<dyn Clock>` | Liefert eine Referenz auf den aktiven [`Clock`]-Port. | `src/tx_buffer.rs` |
| `pub fn discard(&self, tx: TxId)` | Verwirft alle gepufferten Operationen und das ReadSet einer Transaktion (Rollback). | `src/tx_buffer.rs` |
| `pub fn discard_kv(&self, tx: TxId)` | Verwirft alle KV-Operationen einer Transaktion und bereinigt das Key-Staging. | `src/tx_buffer.rs` |
| `pub fn doc_id(&self) -> DocId` | Liefert die Kennung des Dokuments (`DocId`), das von dieser Operation betroffen ist. | `src/tx_buffer.rs` |
| `pub fn drain(&self, tx: TxId) -> Vec<IndexOp<T>>` | Entnimmt alle gepufferten Operationen der Transaktion `tx` für den anschließenden Commit und gibt den reservierten Speicherplatz frei. | `src/tx_buffer.rs` |
| `pub fn drain_kv(&self, tx: TxId) -> Vec<IndexOp<(Vec<u8>, Vec<u8>)>>` | Entnimmt alle gepufferten KV-Operationen einer Transaktion und bereinigt das Key-Staging. | `src/tx_buffer.rs` |
| `pub fn estimated_bytes(&self) -> usize` | Schätzt den gesamten Speicherbedarf der Operation in Bytes (Overhead + Nutzdaten). | `src/tx_buffer.rs` |
| `pub fn get_ops(&self, tx: TxId) -> Option<Vec<IndexOp<T>>>` | Liefert eine Kopie der gepufferten Operationen einer Transaktion. | `src/tx_buffer.rs` |
| `pub fn get_read_set(&self, tx: TxId) -> Option<ReadSet>` | Liefert eine Kopie des angesammelten [`ReadSet`] für Transaktion `tx`. | `src/tx_buffer.rs` |
| `pub fn has_tx(&self, tx: TxId) -> bool` | Prüft, ob die gegebene Transaktion im Puffer registriert ist. | `src/tx_buffer.rs` |
| `pub fn is_empty(&self) -> bool` | Gibt `true` zurück, wenn in keinem einzigen Shard gepufferte Transaktionen liegen. | `src/tx_buffer.rs` |
| `pub fn is_key_staged_for_tx(&self, tx_id: TxId, key: &[u8]) -> bool` | Prüft, ob ein Schlüssel `key` innerhalb der Transaktion `tx_id` als Einfügung vorgemerkt ist. | `src/tx_buffer.rs` |
| `pub fn len(&self) -> usize` | Liefert die Gesamtzahl aller aktuell gepufferten Transaktionen über alle Shards. | `src/tx_buffer.rs` |
| `pub fn min_read_snapshot(&self) -> Option<u64>` | Findet die kleinste Snapshot-Sequenznummer über alle aktiven ReadSets aller Shards hinweg. Dient der Bestimmung des globalen GC-Schwellenwe… | `src/tx_buffer.rs` |
| `pub fn new() -> Self` | Erstellt einen neuen Puffer mit 64 Shards, 30 Sekunden Timeout und [`SystemClock`]. | `src/tx_buffer.rs` |
| `pub fn new_with_clock_and_config( clock: Arc<dyn Clock>, shard_count: usize, config: TxBufferConfig, ) -> Self` | Erstellt einen Puffer mit benutzerdefinierter [`Clock`], Shard-Anzahl und Konfiguration. | `src/tx_buffer.rs` |
| `pub fn new_with_config(shard_count: usize, tx_timeout: Duration) -> Self` | Erstellt einen neuen Puffer mit konfigurierter Shard-Anzahl und Transaktions-Timeout. | `src/tx_buffer.rs` |
| `pub fn new_with_config_ext( shard_count: usize, _tx_timeout: Duration, config: TxBufferConfig, ) -> Self` | Erstellt einen neuen Puffer mit expliziter Konfiguration ([`TxBufferConfig`]). | `src/tx_buffer.rs` |
| `pub fn new_with_time_fn_and_config<F>( time_fn: F, shard_count: usize, config: TxBufferConfig, ) -> Self where F: Fn() -> u64 + Send + Sync + 'static,` | Erstellt einen Puffer mit einer Zeitstempel-Closure (State-Passing per `Fn() -> u64`), Shard-Anzahl und Konfiguration. | `src/tx_buffer.rs` |
| `pub fn read_set(&self, tx: TxId) -> Option<ReadSet>` | Alias-Funktion für [`TxBuffer::get_read_set`]. | `src/tx_buffer.rs` |
| `pub fn reap_orphans(&self) -> Vec<TxId>` | Räumt verwaiste Transaktionen unter Verwendung der aktuellen Uhrzeit ab. | `src/tx_buffer.rs` |
| `pub fn reap_orphans_at(&self, now_nanos: u64) -> Vec<TxId>` | Räumt abgelaufene (verwaiste) Transaktionen bezüglich `now_nanos` über alle Shards ab. | `src/tx_buffer.rs` |
| `pub fn reap_orphans_bounded(&self, max: usize) -> Vec<TxId>` | Räumt bis zu `max` verwaiste Transaktionen bezogen auf die aktuelle Uhrzeit ab. | `src/tx_buffer.rs` |
| `pub fn reap_orphans_bounded_at(&self, max: usize, now_nanos: u64) -> Vec<TxId>` | Räumt maximal `max` verwaiste Transaktionen ab, um Latenzspitzen zu vermeiden. ### Ablauf: 1. Iteriere aufsteigend über die Shards (0 bis $… | `src/tx_buffer.rs` |
| `pub fn record_read(&self, tx: TxId, key: impl Into<Vec<u8>>, snapshot_seq: u64)` | Alias-Funktion für [`TxBuffer::register_read`]. | `src/tx_buffer.rs` |
| `pub fn register_read(&self, tx: TxId, key: impl Into<Vec<u8>>, snapshot_seq: u64)` | Registriert einen Lesezugriff im ReadSet der Transaktion `tx`. | `src/tx_buffer.rs` |
| `pub fn stage(&self, tx: TxId, op: IndexOp<T>) -> Result<()> where T: StagedOpSize,` | Puffert eine Indexoperation für die Transaktion `tx` unter Prüfung aller Speichergrenzen. | `src/tx_buffer.rs` |
| `pub fn stage_bounded(&self, tx: TxId, op: IndexOp<T>) -> Result<()> where T: StagedOpSize,` | Puffert eine Operation und erzwingt Obergrenzen für Operationsanzahl und Byte-Budgets. ### Ablauf: 1. Berechne geschätzten Speicherbedarf d… | `src/tx_buffer.rs` |
| `pub fn stage_kv(&self, tx: TxId, op: IndexOp<(Vec<u8>, Vec<u8>)>) -> Result<()>` | Puffert eine Schlüssel-Wert-Operation und aktualisiert die Key-Staging-Map. Bei Fehlschlag von `stage()` (z. B. Budget-Überlauf) wird das K… | `src/tx_buffer.rs` |
| `pub fn stage_many(&self, tx: TxId, ops: impl IntoIterator<Item = IndexOp<T>>) -> Result<()> where T: StagedOpSize,` | Puffert mehrere Operationen stapelweise mit einer einzigen Sperrung des Shards. | `src/tx_buffer.rs` |
| `pub fn staged_bytes(&self) -> usize` | Liefert den aktuell global über alle Transaktionen verbrauchten Staging-Speicher in Bytes. | `src/tx_buffer.rs` |
| `pub fn staged_status(&self, key: &[u8]) -> Option<bool>` | Ermittelt atomar den Staging-Status eines Schlüssels über alle uncommitted Transaktionen. Liefert `Some(true)` bei Einfügung, `Some(false)`… | `src/tx_buffer.rs` |
| `pub fn try_register_read( &self, tx: TxId, key: impl Into<Vec<u8>>, snapshot_seq: u64, max_keys: usize, ) -> Result<()>` | Registriert einen gelesenen Schlüssel für Transaktion `tx` mit Obergrenzen-Prüfung (`max_keys`). Verhindert unbegrenztes Speicherwachstum b… | `src/tx_buffer.rs` |
| `pub fn tx_timeout(&self) -> Duration` | Liefert die konfigurierte Transaktions-Timeout-Dauer. | `src/tx_buffer.rs` |
| `pub fn validate_pending_ops(&self, tx: TxId) -> Result<()>` | Prüft, ob für eine registrierte Transaktion tatsächlich anhängige Operationen existieren. | `src/tx_buffer.rs` |
| `pub fn with_clock(mut self, clock: Arc<dyn Clock>) -> Self` | Bindet eine benutzerdefinierte [`Clock`]-Instanz an diesen Puffer an. | `src/tx_buffer.rs` |
| `pub fn with_time_fn<F>(self, time_fn: F) -> Self where F: Fn() -> u64 + Send + Sync + 'static,` | Bindet eine Zeitstempel-Closure (State-Passing per `Fn() -> u64`) an diesen Puffer an. | `src/tx_buffer.rs` |


---

## `acta-wire`

| Attribut | Wert |
|---|---|
| **Ring** | 0 — Ebene E0 — Autarke Bausteine & Fundamente |
| **Status** | Fertig |
| **Zweck** | FlatBuffers-IPC-Generat und Zero-Copy-Adapter (Unsafe-Insel). |
| **Unsafe-Code** | Ja (deklarierte Unsafe-Insel) |
| **Interne Abhängigkeiten** | — |
| **Externe Abhängigkeiten (Auswahl)** | `bytes`, `flatbuffers`, `serde`, `serde_json`, `thiserror`, `tracing` |

### Architekturbeschreibung

# Crate `acta-wire`  **Ring & Klasse:** Ring 0 (Unsafe-Insel 3 von 3), Klasse A.  ## Zweck `acta-wire` stellt performante Zero-Copy-Serialisierungs- und Deserialisierungs-Primitiven für die Interprozesskommunikation (IPC = Inter-Process Communication) bereit. Es kapselt: 1. Generierten FlatBuffers-Code (`acta_generated`), der direkt auf Byte-Puffern operiert. 2. Gehärtete Verifikationsfunktionen (`verification`), die Puffer vor dem Zugriff strikt validieren. 3. Zero-Copy-Adapter (`adapter`), wie den referenzgezählten `WireBuffer`. 4. Einen robusten, angriffssicheren JSON-RPC-2.0-Parser (`jsonrpc`) ohne Panics. 5. Einen einheitlichen Fehlertyp `Error` (`Result<T>`) gemäß SPEC v4.0 B4.  ## Einbindung und Abhängigkeiten - **Abhängigkeiten (normal):** `bytes`, `flatbuffers`, `serde`, `serde_json`, `thiserror`. - **Wird eingebunden von:** `acta-mcp`, `acta-py`, `acta-router`, `acta-core`. - **Ring-0-Restriktionen:** Gemäß Regel 3 dürfen Zeit, Zufall und IDs nicht direkt erzeugt werden; I/O erfolgt über übergebene Puffer.  ## Sicherheitsinvariante & Zero-Panic-Garantie Die Crate erzwingt `#![deny(clippy::unwrap_used)]`, `#![deny(clippy::expect_used)]` und `#![deny(clippy::panic)]`. Fehler werden konsequent über `Result` gemeldet (Fail-Closed).

### Modul- und Re-Export-Oberfläche (lib.rs)

```rust
pub mod generated;
pub mod adapter;
pub mod jsonrpc;
pub mod verification;
pub use adapter::*;
pub use generated::acta_generated;
pub use generated::acta_generated::acta::ipc::*;
pub use jsonrpc::*;
pub use verification::*;
```

### Feature-Flags

| Flag | Aktiviert |
|---|---|
| `default` | `["std"]` |
| `std` | `[]` |
| `serde` | `[]` |
| `tracing` | `["dep:tracing"]` |
| `inspect` | `[]` |

### Schnittstellen — Public Traits (1)

**`VerifiableRoot`** — *Helper trait providing higher-ranked follow and verification for root FlatBuffers tables.* (`src/adapter.rs`)

```rust
pub trait VerifiableRoot {
/// Associated inner table type with lifetime `'a`.
    type Table<'a>: flatbuffers::Follow<'a> + flatbuffers::Verifiable + 'a;
}
```

### Öffentliche Datentypen — Structs (32)

| Struct | Felder / Form | Beschreibung | Datei |
|---|---|---|---|
| `Embedding` | `{ pub _tab: flatbuffers::Table<'a>, }` | Öffentliches FlatBuffers-Item `Embedding` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `EmbeddingArgs` | `{ pub data: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, f32>>>, pub quantized_u8: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>, pub metr… }` | Öffentliches FlatBuffers-Item `EmbeddingArgs` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `EmbeddingBuilder` | `{ fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>, start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>, }` | Öffentliches FlatBuffers-Item `EmbeddingBuilder` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `FlatBufferConfig` | `{ /// Maximum allowed buffer size in bytes (default: 10 MiB). /// Maximal erlaubte Puffergröße in Bytes (Standard: 10 MiB). pub max_message_size: usize, /// Maxi… }` | Configuration parameters for FlatBuffers verification. Definiert Schutzgrenzen gegen Denial-of-Service-Angriffe (DoS) durch übergroße oder tief gesch… | `src/verification.rs` |
| `HyperEdge` | `{ pub _tab: flatbuffers::Table<'a>, }` | Öffentliches FlatBuffers-Item `HyperEdge` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `HyperEdgeArgs` | `{ pub id: u64, pub predicate: u8, pub participants: Option< flatbuffers::WIPOffset< flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<RoleBinding<'a>>>, >, >,… }` | Öffentliches FlatBuffers-Item `HyperEdgeArgs` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `HyperEdgeBuilder` | `{ fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>, start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>, }` | Öffentliches FlatBuffers-Item `HyperEdgeBuilder` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `HyperEdgeId` | `{ pub _tab: flatbuffers::Table<'a>, }` | Öffentliches FlatBuffers-Item `HyperEdgeId` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `HyperEdgeIdArgs` | `{ pub id: u64, }` | Öffentliches FlatBuffers-Item `HyperEdgeIdArgs` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `HyperEdgeIdBuilder` | `{ fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>, start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>, }` | Öffentliches FlatBuffers-Item `HyperEdgeIdBuilder` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `JsonRpcConfig` | `{ /// Maximum allowed payload size in bytes (default: 10 MiB). /// Maximal erlaubte Nachrichtengröße in Bytes (Standard: 10 MiB). pub max_message_size: usize, //… }` | Configurable limits and security parameters for JSON-RPC message processing. Konfigurierbare Obergrenzen und Sicherheitsparameter für die JSON-RPC-Ve… | `src/jsonrpc.rs` |
| `JsonRpcError` | `{ /// Numeric error code per JSON-RPC 2.0 specification. /// Numerischer standardisierter Fehlercode gemäß JSON-RPC 2.0 Spezifikation. pub code: i32, /// Brief e… }` | JSON-RPC 2.0 Error object. Standardisiertes JSON-RPC 2.0 Fehlerobjekt. | `src/jsonrpc.rs` |
| `JsonRpcRequest` | `{ /// Always "2.0". /// Protokollversion: Muss strikt `"2.0"` sein. pub jsonrpc: String, /// `None` for notifications (no response expected). /// Anforderungs-ID… }` | Incoming JSON-RPC 2.0 message (Request or Notification). Eingehende JSON-RPC 2.0-Nachricht (Anfrage oder Benachrichtigung). Handelt es sich um eine B… | `src/jsonrpc.rs` |
| `JsonRpcResponse` | `{ /// Always "2.0". /// Protokollversion: Immer `"2.0"`. pub jsonrpc: String, /// Request ID (`None` for parse/batch errors). /// Zugehörige Anforderungs-ID der… }` | Outgoing JSON-RPC 2.0 response. Ausgehende JSON-RPC 2.0-Antwort (Erfolg oder Fehler). | `src/jsonrpc.rs` |
| `RoleBinding` | `{ pub _tab: flatbuffers::Table<'a>, }` | Öffentliches FlatBuffers-Item `RoleBinding` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `RoleBindingArgs` | `{ pub role: u32, pub entity: u64, }` | Öffentliches FlatBuffers-Item `RoleBindingArgs` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `RoleBindingBuilder` | `{ fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>, start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>, }` | Öffentliches FlatBuffers-Item `RoleBindingBuilder` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `RoleId` | `{ pub _tab: flatbuffers::Table<'a>, }` | Öffentliches FlatBuffers-Item `RoleId` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `RoleIdArgs` | `{ pub id: u32, }` | Öffentliches FlatBuffers-Item `RoleIdArgs` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `RoleIdBuilder` | `{ fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>, start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>, }` | Öffentliches FlatBuffers-Item `RoleIdBuilder` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `ScoredDocument` | `{ pub _tab: flatbuffers::Table<'a>, }` | Öffentliches FlatBuffers-Item `ScoredDocument` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `ScoredDocumentArgs` | `{ pub id: Option<flatbuffers::WIPOffset<&'a str>>, pub score: f32, pub metadata: Option<flatbuffers::WIPOffset<&'a str>>, pub embedding: Option<flatbuffers::WIPO… }` | Öffentliches FlatBuffers-Item `ScoredDocumentArgs` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `ScoredDocumentBuilder` | `{ fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>, start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>, }` | Öffentliches FlatBuffers-Item `ScoredDocumentBuilder` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `SearchResponse` | `{ pub _tab: flatbuffers::Table<'a>, }` | Öffentliches FlatBuffers-Item `SearchResponse` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `SearchResponseArgs` | `{ pub results: Option< flatbuffers::WIPOffset< flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<ScoredDocument<'a>>>, >, >, pub total_hits: u32, pub processi… }` | Öffentliches FlatBuffers-Item `SearchResponseArgs` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `SearchResponseBuilder` | `{ fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>, start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>, }` | Öffentliches FlatBuffers-Item `SearchResponseBuilder` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `Unverified` | `(tuple/unit)` | Typzustands-Marker: Unverifizierter Puffer. Rohpuffer, dessen FlatBuffers-Offsets, VTables und Bounds noch nicht validiert wurden. Aus diesem Zustand… | `src/adapter.rs` |
| `VectorIndexUpdate` | `{ pub _tab: flatbuffers::Table<'a>, }` | Öffentliches FlatBuffers-Item `VectorIndexUpdate` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `VectorIndexUpdateArgs` | `{ pub id: Option<flatbuffers::WIPOffset<&'a str>>, pub embedding: Option<flatbuffers::WIPOffset<Embedding<'a>>>, pub metadata: Option<flatbuffers::WIPOffset<&'a… }` | Öffentliches FlatBuffers-Item `VectorIndexUpdateArgs` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `VectorIndexUpdateBuilder` | `{ fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>, start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>, }` | Öffentliches FlatBuffers-Item `VectorIndexUpdateBuilder` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `Verified` | `{ _phantom: PhantomData<fn() -> T>, }` | Typzustands-Marker: Verifizierter Puffer für FlatBuffers-Wurzeltabelle `T`. Garantiert durch vorangegangene Verifikation gegen `FlatBufferConfig`, da… | `src/adapter.rs` |
| `WireBuffer` | `{ data: Bytes, config: FlatBufferConfig, _state: PhantomData<State>, }` | Zero-Copy Byte-Puffer-Wrapper für FlatBuffers-Nachrichten mit Typzustand. Kapselt eine Instanz von `bytes::Bytes`, um ein sicheres, referenzgezähltes… | `src/adapter.rs` |

### Öffentliche Datentypen — Enums (11)

| Enum | Varianten | Beschreibung | Datei |
|---|---|---|---|
| `EmbeddingOffset` | `` | Öffentliches FlatBuffers-Item `EmbeddingOffset` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `Error` | `/// Fehler bei der FlatBuffers-Validierung oder Strukturprüfung. #[error("flatbuffers error: {0}")] FlatBuffer(#[from] FlatBufferError), /// Fehler beim Parsen oder Validieren eines JSON-RPC 2.0 Payl…` | Zentraler, typisierter Fehlertyp für `acta-wire` gemäß SPEC v4.0 (B4). Fasst FlatBuffers-Verifikationsfehler und JSON-RPC-Protokollfehler zusammen. | `src/lib.rs` |
| `FlatBufferError` | `/// Buffer is empty. /// Der übergebene Byte-Puffer ist leer. EmptyBuffer, /// Buffer exceeds configured maximum message size. /// Der Puffer überschreitet die maximal erlaubte Nachrichtengröße (`max…` | Error type for FlatBuffers verification failures. Signalisiert Verletzungen der Spezifikation oder Sicherheitsgrenzen beim Einlesen von Byte-Puffern. | `src/verification.rs` |
| `HyperEdgeIdOffset` | `` | Öffentliches FlatBuffers-Item `HyperEdgeIdOffset` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `HyperEdgeOffset` | `` | Öffentliches FlatBuffers-Item `HyperEdgeOffset` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `JsonRpcPayload` | `/// Single JSON-RPC request. /// Einzelne JSON-RPC-Anfrage (`JsonRpcRequest`). Single(JsonRpcRequest), /// Batch of JSON-RPC requests. /// Batch-Array mehrerer JSON-RPC-Anfragen (`Vec<JsonRpcRequest>…` | Parsed JSON-RPC payload representing either a single request or a batch of requests. Geparste JSON-RPC-Payload: Repräsentiert entweder eine Einzelanf… | `src/jsonrpc.rs` |
| `RoleBindingOffset` | `` | Öffentliches FlatBuffers-Item `RoleBindingOffset` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `RoleIdOffset` | `` | Öffentliches FlatBuffers-Item `RoleIdOffset` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `ScoredDocumentOffset` | `` | Öffentliches FlatBuffers-Item `ScoredDocumentOffset` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `SearchResponseOffset` | `` | Öffentliches FlatBuffers-Item `SearchResponseOffset` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `VectorIndexUpdateOffset` | `` | Öffentliches FlatBuffers-Item `VectorIndexUpdateOffset` für IPC-Nachrichten. | `src/acta_generated.rs` |

### Typ-Aliase (1)

| Alias | Definition | Beschreibung |
|---|---|---|
| `Result` | `type Result<T> = core::result::Result<T, Error>` | Standard-Result-Typ für `acta-wire`. |

### Konstanten (27)

| Konstante | Typ | Beschreibung |
|---|---|---|
| `VT_BUSINESS_VALID_FROM` | `flatbuffers::VOffsetT` | Öffentliches FlatBuffers-Item `VT_BUSINESS_VALID_FROM` für IPC-Nachrichten. |
| `VT_BUSINESS_VALID_TO` | `flatbuffers::VOffsetT` | Öffentliches FlatBuffers-Item `VT_BUSINESS_VALID_TO` für IPC-Nachrichten. |
| `VT_CHILD_EDGE_IDS` | `flatbuffers::VOffsetT` | Öffentliches FlatBuffers-Item `VT_CHILD_EDGE_IDS` für IPC-Nachrichten. |
| `VT_DATA` | `flatbuffers::VOffsetT` | Öffentliches FlatBuffers-Item `VT_DATA` für IPC-Nachrichten. |
| `VT_EMBEDDING` | `flatbuffers::VOffsetT` | Öffentliches FlatBuffers-Item `VT_EMBEDDING` für IPC-Nachrichten. |
| `VT_EMBEDDING` | `flatbuffers::VOffsetT` | Öffentliches FlatBuffers-Item `VT_EMBEDDING` für IPC-Nachrichten. |
| `VT_ENTITY` | `flatbuffers::VOffsetT` | Öffentliches FlatBuffers-Item `VT_ENTITY` für IPC-Nachrichten. |
| `VT_ID` | `flatbuffers::VOffsetT` | Öffentliches FlatBuffers-Item `VT_ID` für IPC-Nachrichten. |
| `VT_ID` | `flatbuffers::VOffsetT` | Öffentliches FlatBuffers-Item `VT_ID` für IPC-Nachrichten. |
| `VT_ID` | `flatbuffers::VOffsetT` | Öffentliches FlatBuffers-Item `VT_ID` für IPC-Nachrichten. |
| `VT_ID` | `flatbuffers::VOffsetT` | Öffentliches FlatBuffers-Item `VT_ID` für IPC-Nachrichten. |
| `VT_ID` | `flatbuffers::VOffsetT` | Öffentliches FlatBuffers-Item `VT_ID` für IPC-Nachrichten. |
| `VT_METADATA` | `flatbuffers::VOffsetT` | Öffentliches FlatBuffers-Item `VT_METADATA` für IPC-Nachrichten. |
| `VT_METADATA` | `flatbuffers::VOffsetT` | Öffentliches FlatBuffers-Item `VT_METADATA` für IPC-Nachrichten. |
| `VT_METRIC` | `flatbuffers::VOffsetT` | Öffentliches FlatBuffers-Item `VT_METRIC` für IPC-Nachrichten. |
| `VT_PARTICIPANTS` | `flatbuffers::VOffsetT` | Öffentliches FlatBuffers-Item `VT_PARTICIPANTS` für IPC-Nachrichten. |
| `VT_PREDICATE` | `flatbuffers::VOffsetT` | Öffentliches FlatBuffers-Item `VT_PREDICATE` für IPC-Nachrichten. |
| `VT_PROCESSING_TIME_MS` | `flatbuffers::VOffsetT` | Öffentliches FlatBuffers-Item `VT_PROCESSING_TIME_MS` für IPC-Nachrichten. |
| `VT_QUANTIZED_U8` | `flatbuffers::VOffsetT` | Öffentliches FlatBuffers-Item `VT_QUANTIZED_U8` für IPC-Nachrichten. |
| `VT_RESULTS` | `flatbuffers::VOffsetT` | Öffentliches FlatBuffers-Item `VT_RESULTS` für IPC-Nachrichten. |
| `VT_ROLE` | `flatbuffers::VOffsetT` | Öffentliches FlatBuffers-Item `VT_ROLE` für IPC-Nachrichten. |
| `VT_SCORE` | `flatbuffers::VOffsetT` | Öffentliches FlatBuffers-Item `VT_SCORE` für IPC-Nachrichten. |
| `VT_SOURCE_DOC_ID` | `flatbuffers::VOffsetT` | Öffentliches FlatBuffers-Item `VT_SOURCE_DOC_ID` für IPC-Nachrichten. |
| `VT_TOTAL_HITS` | `flatbuffers::VOffsetT` | Öffentliches FlatBuffers-Item `VT_TOTAL_HITS` für IPC-Nachrichten. |
| `VT_TX_VALID_FROM` | `flatbuffers::VOffsetT` | Öffentliches FlatBuffers-Item `VT_TX_VALID_FROM` für IPC-Nachrichten. |
| `VT_TX_VALID_TO` | `flatbuffers::VOffsetT` | Öffentliches FlatBuffers-Item `VT_TX_VALID_TO` für IPC-Nachrichten. |
| `VT_WEIGHT` | `flatbuffers::VOffsetT` | Öffentliches FlatBuffers-Item `VT_WEIGHT` für IPC-Nachrichten. |

### Öffentliche Funktionen (121)

| Signatur | Beschreibung | Datei |
|---|---|---|
| `pub fn add_business_valid_from(&mut self, business_valid_from: i64)` | Öffentliches FlatBuffers-Item `add_business_valid_from` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn add_business_valid_to(&mut self, business_valid_to: i64)` | Öffentliches FlatBuffers-Item `add_business_valid_to` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn add_child_edge_ids( &mut self, child_edge_ids: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u64>>, )` | Öffentliches FlatBuffers-Item `add_child_edge_ids` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn add_data(&mut self, data: flatbuffers::WIPOffset<flatbuffers::Vector<'b, f32>>)` | Öffentliches FlatBuffers-Item `add_data` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn add_embedding(&mut self, embedding: flatbuffers::WIPOffset<Embedding<'b>>)` | Öffentliches FlatBuffers-Item `add_embedding` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn add_embedding(&mut self, embedding: flatbuffers::WIPOffset<Embedding<'b>>)` | Öffentliches FlatBuffers-Item `add_embedding` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn add_entity(&mut self, entity: u64)` | Öffentliches FlatBuffers-Item `add_entity` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn add_id(&mut self, id: flatbuffers::WIPOffset<&'b str>)` | Öffentliches FlatBuffers-Item `add_id` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn add_id(&mut self, id: flatbuffers::WIPOffset<&'b str>)` | Öffentliches FlatBuffers-Item `add_id` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn add_id(&mut self, id: u32)` | Öffentliches FlatBuffers-Item `add_id` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn add_id(&mut self, id: u64)` | Öffentliches FlatBuffers-Item `add_id` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn add_id(&mut self, id: u64)` | Öffentliches FlatBuffers-Item `add_id` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn add_metadata(&mut self, metadata: flatbuffers::WIPOffset<&'b str>)` | Öffentliches FlatBuffers-Item `add_metadata` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn add_metadata(&mut self, metadata: flatbuffers::WIPOffset<&'b str>)` | Öffentliches FlatBuffers-Item `add_metadata` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn add_metric(&mut self, metric: i8)` | Öffentliches FlatBuffers-Item `add_metric` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn add_participants( &mut self, participants: flatbuffers::WIPOffset< flatbuffers::Vector<'b, flatbuffers::ForwardsUOffset<RoleBinding<'b>>>, >, )` | Öffentliches FlatBuffers-Item `add_participants` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn add_predicate(&mut self, predicate: u8)` | Öffentliches FlatBuffers-Item `add_predicate` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn add_processing_time_ms(&mut self, processing_time_ms: f32)` | Öffentliches FlatBuffers-Item `add_processing_time_ms` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn add_quantized_u8( &mut self, quantized_u8: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u8>>, )` | Öffentliches FlatBuffers-Item `add_quantized_u8` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn add_results( &mut self, results: flatbuffers::WIPOffset< flatbuffers::Vector<'b, flatbuffers::ForwardsUOffset<ScoredDocument<'b>>>, >, )` | Öffentliches FlatBuffers-Item `add_results` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn add_role(&mut self, role: u32)` | Öffentliches FlatBuffers-Item `add_role` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn add_score(&mut self, score: f32)` | Öffentliches FlatBuffers-Item `add_score` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn add_source_doc_id(&mut self, source_doc_id: u64)` | Öffentliches FlatBuffers-Item `add_source_doc_id` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn add_total_hits(&mut self, total_hits: u32)` | Öffentliches FlatBuffers-Item `add_total_hits` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn add_tx_valid_from(&mut self, tx_valid_from: u64)` | Öffentliches FlatBuffers-Item `add_tx_valid_from` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn add_tx_valid_to(&mut self, tx_valid_to: u64)` | Öffentliches FlatBuffers-Item `add_tx_valid_to` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn add_weight(&mut self, weight: f32)` | Öffentliches FlatBuffers-Item `add_weight` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn business_valid_from(&self) -> i64` | Öffentliches FlatBuffers-Item `business_valid_from` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn business_valid_to(&self) -> i64` | Öffentliches FlatBuffers-Item `business_valid_to` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn child_edge_ids(&self) -> Option<flatbuffers::Vector<'a, u64>>` | Öffentliches FlatBuffers-Item `child_edge_ids` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>( _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>, args: &'args EmbeddingArgs<'args>, ) -> flatbuffers::WIPOffset<Embedding<'bldr>>` | Öffentliches FlatBuffers-Item `create` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>( _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>, args: &'args ScoredDocumentArgs<'args>, ) -> flatbuffers::WIPOffset<ScoredDocument<'bldr>>` | Öffentliches FlatBuffers-Item `create` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>( _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>, args: &'args SearchResponseArgs<'args>, ) -> flatbuffers::WIPOffset<SearchResponse<'bldr>>` | Öffentliches FlatBuffers-Item `create` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>( _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>, args: &'args VectorIndexUpdateArgs<'args>, ) -> flatbuffers::WIPOffset<VectorIndexUpdate<'bldr>>` | Öffentliches FlatBuffers-Item `create` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>( _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>, args: &'args RoleIdArgs, ) -> flatbuffers::WIPOffset<RoleId<'bldr>>` | Öffentliches FlatBuffers-Item `create` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>( _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>, args: &'args HyperEdgeIdArgs, ) -> flatbuffers::WIPOffset<HyperEdgeId<'bldr>>` | Öffentliches FlatBuffers-Item `create` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>( _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>, args: &'args RoleBindingArgs, ) -> flatbuffers::WIPOffset<RoleBinding<'bldr>>` | Öffentliches FlatBuffers-Item `create` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>( _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>, args: &'args HyperEdgeArgs<'args>, ) -> flatbuffers::WIPOffset<HyperEdge<'bldr>>` | Öffentliches FlatBuffers-Item `create` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn data(&self) -> Option<flatbuffers::Vector<'a, f32>>` | Öffentliches FlatBuffers-Item `data` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn embedding(&self) -> Option<Embedding<'a>>` | Öffentliches FlatBuffers-Item `embedding` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn embedding(&self) -> Option<Embedding<'a>>` | Öffentliches FlatBuffers-Item `embedding` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn entity(&self) -> u64` | Öffentliches FlatBuffers-Item `entity` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn finish(self) -> flatbuffers::WIPOffset<Embedding<'a>>` | Öffentliches FlatBuffers-Item `finish` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn finish(self) -> flatbuffers::WIPOffset<ScoredDocument<'a>>` | Öffentliches FlatBuffers-Item `finish` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn finish(self) -> flatbuffers::WIPOffset<SearchResponse<'a>>` | Öffentliches FlatBuffers-Item `finish` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn finish(self) -> flatbuffers::WIPOffset<VectorIndexUpdate<'a>>` | Öffentliches FlatBuffers-Item `finish` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn finish(self) -> flatbuffers::WIPOffset<RoleId<'a>>` | Öffentliches FlatBuffers-Item `finish` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn finish(self) -> flatbuffers::WIPOffset<HyperEdgeId<'a>>` | Öffentliches FlatBuffers-Item `finish` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn finish(self) -> flatbuffers::WIPOffset<RoleBinding<'a>>` | Öffentliches FlatBuffers-Item `finish` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn finish(self) -> flatbuffers::WIPOffset<HyperEdge<'a>>` | Öffentliches FlatBuffers-Item `finish` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn finish_search_response_buffer<'a, 'b>( fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<SearchResponse<'a>>, )` | Öffentliches FlatBuffers-Item `finish_search_response_buffer` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn finish_size_prefixed_search_response_buffer<'a, 'b>( fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<SearchResponse<'a>>, )` | Öffentliches FlatBuffers-Item `finish_size_prefixed_search_response_buffer` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn id(&self) -> Option<&'a str>` | Öffentliches FlatBuffers-Item `id` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn id(&self) -> Option<&'a str>` | Öffentliches FlatBuffers-Item `id` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn id(&self) -> u32` | Öffentliches FlatBuffers-Item `id` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn id(&self) -> u64` | Öffentliches FlatBuffers-Item `id` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn id(&self) -> u64` | Öffentliches FlatBuffers-Item `id` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self` | Öffentliches FlatBuffers-Item `init_from_table` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self` | Öffentliches FlatBuffers-Item `init_from_table` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self` | Öffentliches FlatBuffers-Item `init_from_table` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self` | Öffentliches FlatBuffers-Item `init_from_table` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self` | Öffentliches FlatBuffers-Item `init_from_table` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self` | Öffentliches FlatBuffers-Item `init_from_table` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self` | Öffentliches FlatBuffers-Item `init_from_table` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self` | Öffentliches FlatBuffers-Item `init_from_table` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn metadata(&self) -> Option<&'a str>` | Öffentliches FlatBuffers-Item `metadata` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn metadata(&self) -> Option<&'a str>` | Öffentliches FlatBuffers-Item `metadata` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn metric(&self) -> i8` | Öffentliches FlatBuffers-Item `metric` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn new( _fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, ) -> EmbeddingBuilder<'a, 'b>` | Öffentliches FlatBuffers-Item `new` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn new( _fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, ) -> ScoredDocumentBuilder<'a, 'b>` | Öffentliches FlatBuffers-Item `new` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn new( _fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, ) -> SearchResponseBuilder<'a, 'b>` | Öffentliches FlatBuffers-Item `new` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn new( _fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, ) -> VectorIndexUpdateBuilder<'a, 'b>` | Öffentliches FlatBuffers-Item `new` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> RoleIdBuilder<'a, 'b>` | Öffentliches FlatBuffers-Item `new` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn new( _fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, ) -> HyperEdgeIdBuilder<'a, 'b>` | Öffentliches FlatBuffers-Item `new` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn new( _fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, ) -> RoleBindingBuilder<'a, 'b>` | Öffentliches FlatBuffers-Item `new` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn new( _fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, ) -> HyperEdgeBuilder<'a, 'b>` | Öffentliches FlatBuffers-Item `new` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn participants( &self, ) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<RoleBinding<'a>>>>` | Öffentliches FlatBuffers-Item `participants` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn predicate(&self) -> u8` | Öffentliches FlatBuffers-Item `predicate` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn processing_time_ms(&self) -> f32` | Öffentliches FlatBuffers-Item `processing_time_ms` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn quantized_u8(&self) -> Option<flatbuffers::Vector<'a, u8>>` | Öffentliches FlatBuffers-Item `quantized_u8` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn results( &self, ) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<ScoredDocument<'a>>>>` | Öffentliches FlatBuffers-Item `results` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn role(&self) -> u32` | Öffentliches FlatBuffers-Item `role` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn root_as_search_response( buf: &[u8], ) -> Result<SearchResponse, flatbuffers::InvalidFlatbuffer>` | Öffentliches FlatBuffers-Item `root_as_search_response` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub unsafe fn root_as_search_response_unchecked(buf: &[u8]) -> SearchResponse` | Öffentliches FlatBuffers-Item `root_as_search_response_unchecked` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn root_as_search_response_with_opts<'b, 'o>( opts: &'o flatbuffers::VerifierOptions, buf: &'b [u8], ) -> Result<SearchResponse<'b>, flatbuffers::InvalidFlatbuffer>` | Öffentliches FlatBuffers-Item `root_as_search_response_with_opts` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn score(&self) -> f32` | Öffentliches FlatBuffers-Item `score` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn size_prefixed_root_as_search_response( buf: &[u8], ) -> Result<SearchResponse, flatbuffers::InvalidFlatbuffer>` | Öffentliches FlatBuffers-Item `size_prefixed_root_as_search_response` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub unsafe fn size_prefixed_root_as_search_response_unchecked( buf: &[u8], ) -> SearchResponse` | Öffentliches FlatBuffers-Item `size_prefixed_root_as_search_response_unchecked` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn size_prefixed_root_as_search_response_with_opts<'b, 'o>( opts: &'o flatbuffers::VerifierOptions, buf: &'b [u8], ) -> Result<SearchResponse<'b>, flatbuffers::InvalidFlatbuffer>` | Öffentliches FlatBuffers-Item `size_prefixed_root_as_search_response_with_opts` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn source_doc_id(&self) -> u64` | Öffentliches FlatBuffers-Item `source_doc_id` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn total_hits(&self) -> u32` | Öffentliches FlatBuffers-Item `total_hits` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn tx_valid_from(&self) -> u64` | Öffentliches FlatBuffers-Item `tx_valid_from` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn tx_valid_to(&self) -> u64` | Öffentliches FlatBuffers-Item `tx_valid_to` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn weight(&self) -> f32` | Öffentliches FlatBuffers-Item `weight` für IPC-Nachrichten. | `src/acta_generated.rs` |
| `pub fn as_slice(&self) -> &[u8]` | Gibt eine unveränderliche Slice-Referenz auf die zugrundeliegenden Rohdaten zurück. # Ablauf 1. Zugriff auf den internen Byte-Puffer. 2. Rü… | `src/adapter.rs` |
| `pub fn into_inner(self) -> Bytes` | Konsumiert den Wrapper und gibt das zugrundeliegende `Bytes`-Objekt frei. # Ablauf 1. Entpacken des internen `data`-Felds. 2. Rückgabe des… | `src/adapter.rs` |
| `pub fn is_empty(&self) -> bool` | Gibt an, ob der Puffer leer ist. | `src/adapter.rs` |
| `pub fn len(&self) -> usize` | Gibt die Länge des Puffers in Bytes zurück. | `src/adapter.rs` |
| `pub fn new(data: Bytes) -> Self` | Erzeugt einen neuen `WireBuffer` im Zustand `Unverified`, der das übergebene `Bytes`-Objekt umhüllt. # Ablauf 1. Übernahme des `Bytes`-Obje… | `src/adapter.rs` |
| `pub fn root<'a>( &'a self, ) -> Result<<T::Table<'a> as flatbuffers::Follow<'a>>::Inner, FlatBufferError>` | Liefert die verifizierte FlatBuffers-Wurzel für diesen Puffer unter Verwendung der bei der Verifikation geprüften Grenzwerte. # Garantien (… | `src/adapter.rs` |
| `pub fn root_with_config<'a>( &'a self, config: &FlatBufferConfig, ) -> Result<<T::Table<'a> as flatbuffers::Follow<'a>>::Inner, FlatBufferError>` | Liefert die verifizierte FlatBuffers-Wurzel für diesen Puffer unter Verwendung der angegebenen Schutzgrenzen. # Garantien (`INV-WIRE-BOUNDS… | `src/adapter.rs` |
| `pub fn verify<T>( self, config: &FlatBufferConfig, ) -> Result<WireBuffer<Verified<T>>, FlatBufferError> where T: VerifiableRoot,` | Verifiziert den Puffer gegen das angegebene FlatBuffers-Root-Schema `T` und die Schutzgrenzen `config`. # Ablauf (`INV-WIRE-BOUNDS-1`) 1. Ü… | `src/adapter.rs` |
| `pub fn err(id: Option<Value>, code: i32, message: impl Into<String>) -> Self` | Constructs an error JSON-RPC response. Konstruiert eine JSON-RPC 2.0-Fehlerantwort. # Parameter - `id`: ID der Anfrage (oder `None`, falls… | `src/jsonrpc.rs` |
| `pub fn internal_error(message: impl Into<String>) -> Self` | Internal error (-32603): Internal JSON-RPC error. Erzeugt einen Internal error (-32603): Interner JSON-RPC Serverfehler. | `src/jsonrpc.rs` |
| `pub fn invalid_params(message: impl Into<String>) -> Self` | Invalid params (-32602): Invalid method parameter(s). Erzeugt einen Invalid params (-32602): Ungültige Methodenparameter übergeben. | `src/jsonrpc.rs` |
| `pub fn invalid_request(message: impl Into<String>) -> Self` | Invalid Request (-32600): JSON sent is not a valid Request object. Erzeugt einen Invalid Request (-32600): Gesendetes JSON ist keine gültig… | `src/jsonrpc.rs` |
| `pub fn method_not_found(message: impl Into<String>) -> Self` | Method not found (-32601): The method does not exist / is not available. Erzeugt einen Method not found (-32601): Methode existiert nicht o… | `src/jsonrpc.rs` |
| `pub fn ok(id: Option<Value>, result: Value) -> Self` | Constructs a successful JSON-RPC response. Konstruiert eine erfolgreiche JSON-RPC 2.0-Antwort mit Ergebnisdaten. # Parameter - `id`: ID der… | `src/jsonrpc.rs` |
| `pub fn parse_error(message: impl Into<String>) -> Self` | Parse error (-32700): Invalid JSON received by server. Erzeugt einen Parse error (-32700): Ungültiges JSON empfangen. | `src/jsonrpc.rs` |
| `pub fn parse_jsonrpc_payload(raw: &[u8]) -> Result<JsonRpcPayload, JsonRpcError>` | Parses raw bytes into a `JsonRpcPayload` using default configuration limits. Parst rohe Bytes in ein `JsonRpcPayload` unter Verwendung der… | `src/jsonrpc.rs` |
| `pub fn parse_jsonrpc_payload_with_config( raw: &[u8], config: &JsonRpcConfig, ) -> Result<JsonRpcPayload, JsonRpcError>` | Parses raw bytes into a `JsonRpcPayload` with strict security and size validation. Parst rohe Bytes in ein `JsonRpcPayload` mit strikter Si… | `src/jsonrpc.rs` |
| `pub fn validate(&self) -> Result<(), JsonRpcError>` | Validates that the request complies with the JSON-RPC 2.0 specification (fail-closed). # Ablauf 1. Prüfen, ob `jsonrpc` exakt `"2.0"` entsp… | `src/jsonrpc.rs` |
| `pub fn verify_and_parse_embedding<'a>( buf: &'a [u8], config: &FlatBufferConfig, ) -> Result<Embedding<'a>, FlatBufferError>` | Verifies and parses an `Embedding` root table from bytes. # Ablauf von `verify_and_parse_embedding`: 1. Validiert Puffergrenzen und Tabelle… | `src/verification.rs` |
| `pub fn verify_and_parse_hyper_edge<'a>( buf: &'a [u8], config: &FlatBufferConfig, ) -> Result<HyperEdge<'a>, FlatBufferError>` | Verifies and parses a `HyperEdge` root table from bytes. # Ablauf von `verify_and_parse_hyper_edge`: 1. Validiert Puffergrenzen und Tabelle… | `src/verification.rs` |
| `pub fn verify_and_parse_hyper_edge_id<'a>( buf: &'a [u8], config: &FlatBufferConfig, ) -> Result<HyperEdgeId<'a>, FlatBufferError>` | Verifies and parses a `HyperEdgeId` root table from bytes. # Ablauf von `verify_and_parse_hyper_edge_id`: 1. Validiert Puffergrenzen und Ta… | `src/verification.rs` |
| `pub fn verify_and_parse_role_binding<'a>( buf: &'a [u8], config: &FlatBufferConfig, ) -> Result<RoleBinding<'a>, FlatBufferError>` | Verifies and parses a `RoleBinding` root table from bytes. # Ablauf von `verify_and_parse_role_binding`: 1. Validiert Puffergrenzen und Tab… | `src/verification.rs` |
| `pub fn verify_and_parse_role_id<'a>( buf: &'a [u8], config: &FlatBufferConfig, ) -> Result<RoleId<'a>, FlatBufferError>` | Verifies and parses a `RoleId` root table from bytes. # Ablauf von `verify_and_parse_role_id`: 1. Validiert Puffergrenzen und Tabellenstruk… | `src/verification.rs` |
| `pub fn verify_and_parse_scored_document<'a>( buf: &'a [u8], config: &FlatBufferConfig, ) -> Result<ScoredDocument<'a>, FlatBufferError>` | Verifies and parses a `ScoredDocument` root table from bytes. # Ablauf von `verify_and_parse_scored_document`: 1. Validiert Puffergrenzen u… | `src/verification.rs` |
| `pub fn verify_and_parse_search_response<'a>( buf: &'a [u8], config: &FlatBufferConfig, ) -> Result<SearchResponse<'a>, FlatBufferError>` | Verifies and parses a `SearchResponse` root table from bytes. # Ablauf von `verify_and_parse_search_response`: 1. Übergibt Puffer und Konfi… | `src/verification.rs` |
| `pub fn verify_and_parse_vector_index_update<'a>( buf: &'a [u8], config: &FlatBufferConfig, ) -> Result<VectorIndexUpdate<'a>, FlatBufferError>` | Verifies and parses a `VectorIndexUpdate` root table from bytes. # Ablauf von `verify_and_parse_vector_index_update`: 1. Validiert Puffergr… | `src/verification.rs` |
| `pub fn verify_flatbuffer<'a, T>( buf: &'a [u8], config: &FlatBufferConfig, ) -> Result<T::Inner, FlatBufferError> where T: flatbuffers::Follow<'a> + flatbuffers::Verifiable + 'a,` | Generic verified root accessor for any FlatBuffers root table `T`. # Ablauf von `verify_flatbuffer`: 1. Puffer prüfen: Ist der Puffer leer,… | `src/verification.rs` |


---

## `acta-sys`

| Attribut | Wert |
|---|---|
| **Ring** | 0 — Ebene E0 — Autarke Bausteine & Fundamente |
| **Status** | Fertig |
| **Zweck** | System-Abstraktionen für mmap, mlock und Win32-ACLs (Unsafe-Insel). |
| **Unsafe-Code** | Ja (deklarierte Unsafe-Insel) |
| **Interne Abhängigkeiten** | — |
| **Externe Abhängigkeiten (Auswahl)** | `memmap2`, `thiserror`, `tracing` |

### Modul- und Re-Export-Oberfläche (lib.rs)

```rust
pub mod acl_win32;
pub mod contract;
pub mod durability;
pub mod error;
pub mod mlock;
pub mod mmap;
pub mod posix;
pub mod vault;
pub mod mem { .. }
pub mod fsmeta { .. }
pub mod map { .. }
pub use crate::contract::{MemoryLocker, NoopMemoryLocker, OsMemoryLocker};
pub use crate::mlock::{ apply_dont_dump, mem_lock, mem_lock_typed, mem_lock_with_policy, mem_unlock, MemLockPolicy, };
pub use crate::vault::{LockedRegions, VaultBuffer};
pub use crate::acl_win32::{set_restrictive_file_acl, verify_file_acl_owner_only};
pub use crate::mmap::{mmap_readonly, SealedFile, SealedMap, SealedReadOnly, Unsealed};
pub use acl_win32::{set_restrictive_file_acl, verify_file_acl_owner_only};
pub use contract::{MemoryLocker, NoopMemoryLocker, OsMemoryLocker};
pub use durability::{durable_flush_dir, durable_flush_file, write_all_durable, DurableFile};
pub use error::{Error, Result, SysError};
pub use mlock::{ apply_dont_dump, mem_lock, mem_lock_typed, mem_lock_with_policy, mem_unlock, MemLockPolicy, };
pub use mmap::{mmap_readonly, SealedFile, SealedMap, SealedReadOnly, Unsealed};
pub use posix::{ensure_fd_limit, reopen_and_dup2, set_fd_limit};
pub use vault::{LockedRegions, VaultBuffer};
```

### Schnittstellen — Public Traits (1)

**`MemoryLocker`** — *Abstraktion für Memory-Locking im Arbeitsspeicher.* (`src/contract.rs`)

```rust
pub trait MemoryLocker : Send + Sync {
/// Sperrt eine Speicherregion im RAM.
    fn lock(&self, ptr: *const u8, len: usize, policy: MemLockPolicy) -> Result<bool>;

    /// Entsperrt eine zuvor gesperrte Speicherregion.
    fn unlock(&self, addr: usize, len: usize);
}
```

### Öffentliche Datentypen — Structs (9)

| Struct | Felder / Form | Beschreibung | Datei |
|---|---|---|---|
| `DurableFile` | `{ file: File, poisoned: bool, }` | Ein sicherer RAII-Wrapper um [`std::fs::File`], der die Invariante **INV-SYS-FSYNC-1** erzwingt: Sobald ein `fsync`/`fdatasync` oder Flush-Aufruf feh… | `src/durability.rs` |
| `LockedRegions` | `{ regions: Vec<(usize, usize)>, }` | Kapselt und verfolgt im RAM gesperrte Speicherbereiche (Adressen und Längen) für sensible Schlüsseldaten. | `src/vault.rs` |
| `NoopMemoryLocker` | `(tuple/unit)` | Referenzimplementierung eines Memory-Lockers ohne echte E/A / ohne Syscalls (No-op). | `src/contract.rs` |
| `OsMemoryLocker` | `(tuple/unit)` | Standard-Implementierung über die nativen OS-Syscalls von `acta-sys`. | `src/contract.rs` |
| `SealedFile` | `{ file: File, expected_len: u64, _marker: std::marker::PhantomData<State>, }` | Ein typisierter Dateiwächter, der den Versiegelungszustand einer Datei zur Compilezeit und Laufzeit abbildet. | `src/mmap.rs` |
| `SealedMap` | `{ inner: memmap2::Mmap, }` | Sichere Nur-Lese-Speicherabbildung für versiegelte, unveränderliche Dateien (INV-IO-1). | `src/mmap.rs` |
| `SealedReadOnly` | `(tuple/unit)` | Typzustands-Marker: Zeigt an, dass eine Datei vollständig versiegelt, unveränderlich und prüfsummenverifiziert ist (gemäß INV-IO-1 und INV-ENCRYPT-RE… | `src/mmap.rs` |
| `Unsealed` | `(tuple/unit)` | Typzustands-Marker: Zeigt an, dass eine Datei noch unversiegelt oder potenziell veränderlich ist. | `src/mmap.rs` |
| `VaultBuffer` | `{ data: Vec<u8>, is_locked: bool, }` | Ein im RAM verankerter, geschützter Speicherpuffer für hochsensible kryptografische Schlüsseldaten (INV-SYS-MEMLOCK-1). | `src/vault.rs` |

### Öffentliche Datentypen — Enums (2)

| Enum | Varianten | Beschreibung | Datei |
|---|---|---|---|
| `Error` | `/// Ein Speicherpuffer konnte nicht im physischen RAM verankert werden (`mlock`/`VirtualLock`). #[error("memory locking failed (errno {errno}): {reason}")] MemoryLockFailed { /// Betriebssystemspezif…` | Typisierter Systemfehler für Low-Level-OS-Operationen. | `src/error.rs` |
| `MemLockPolicy` | `/// Fehlschläge beim Sperren werden als fataler Fehler behandelt und brechen die Operation ab. /// Einsatz z. B. bei hochsensiblen kryptografischen Masterschlüsseln (Fail-Closed). FailClosed, /// Feh…` | Konfigurierbare Richtlinie für das Sperren von Speicherregionen (`mlock` / `VirtualLock`). | `src/mlock.rs` |

### Typ-Aliase (2)

| Alias | Definition | Beschreibung |
|---|---|---|
| `Result` | `type Result<T> = std::result::Result<T, Error>` | Spezifischer Result-Typ für `acta-sys`. |
| `SysError` | `type SysError= Error` | Typ-Alias für das Crate-Fehlermodell gemäß Baukasten-Spezifikation (docs/SPEC.md §3.1 / R3). |

### Öffentliche Funktionen (53)

| Signatur | Beschreibung | Datei |
|---|---|---|
| `pub fn set_restrictive_file_acl(path: &Path) -> std::io::Result<()>` | Beschränkt die Zugriffsrechte der angegebenen Datei unter Unix auf den Besitzer (`0600` / `-rw-------`). # Ablauf 1. Konvertiert den oktale… | `src/acl_win32.rs` |
| `pub fn set_restrictive_file_acl(path: &Path) -> std::io::Result<()>` | Beschränkt unter Windows die Zugriffsrechte der angegebenen Datei auf den Prozessbesitzer mittels Win32-DACL. # Ablauf 1. Ruft die Besitzer… | `src/acl_win32.rs` |
| `pub fn set_restrictive_file_acl(path: &Path) -> std::io::Result<()>` | Best-Effort-Fallback für Zielarchitekturen, die weder Unix noch Windows sind (No-Op). | `src/acl_win32.rs` |
| `pub fn verify_file_acl_owner_only(path: &Path) -> std::io::Result<()>` | Verifiziert unter Unix, dass eine Datei strikt auf den Besitzer beschränkt ist (`0600`) und dem aktuellen Prozess gehört. # Ablauf 1. Liest… | `src/acl_win32.rs` |
| `pub fn verify_file_acl_owner_only(path: &Path) -> std::io::Result<()>` | Verifiziert unter Windows, dass die Datei über eine geschützte DACL verfügt, die ausschließlich dem aktuellen Prozessbesitzer Zugriff gewäh… | `src/acl_win32.rs` |
| `pub fn verify_file_acl_owner_only(path: &Path) -> std::io::Result<()>` | Best-Effort-Fallback für Zielarchitekturen, die weder Unix noch Windows sind (No-Op). | `src/acl_win32.rs` |
| `pub fn durable_flush_dir(path: &Path) -> IoResult<()>` | Flasht den Verzeichniseintrag (übergeordnetes Verzeichnis) einer neu erstellten oder umbenannten Datei. | `src/durability.rs` |
| `pub fn durable_flush_file(file: &File, metadata_changed: bool) -> IoResult<()>` | Flasht Dateiinhalt und optional Metadaten auf den physischen Datenträger gemäß den Eigenschaften der Zielplattform. | `src/durability.rs` |
| `pub fn get_ref(&self) -> Result<&File>` | Gibt eine Referenz auf die innere [`File`] zurück, sofern das Handle nicht vergiftet ist. | `src/durability.rs` |
| `pub fn is_poisoned(&self) -> bool` | Gibt `true` zurück, wenn das Datei-Handle infolge eines früheren Flush-Fehlers vergiftet ist. | `src/durability.rs` |
| `pub fn new(file: File) -> Self` | Kapselt eine geöffnete [`File`] in einem gesunden, unvergifteten Zustand. | `src/durability.rs` |
| `pub fn set_fail_fsync(fail: bool, errno: i32)` | — | `src/durability.rs` |
| `pub fn should_fail_fsync() -> Option<i32>` | — | `src/durability.rs` |
| `pub fn sync_all(&mut self) -> Result<()>` | Flasht Dateiinhalt und Metadaten auf das physische Speichermedium. | `src/durability.rs` |
| `pub fn sync_data(&mut self) -> Result<()>` | Flasht den Dateiinhalt (ohne Metadaten, z. B. via `fdatasync` unter Linux). | `src/durability.rs` |
| `pub fn write_all(&mut self, buf: &[u8]) -> Result<()>` | Schreibt Daten dauerhaft in die Datei. Schlägt sofort fehl, wenn das Handle vergiftet ist. | `src/durability.rs` |
| `pub fn write_all_durable(file: &mut File, mut buf: &[u8]) -> IoResult<()>` | Schreibt die gesamten Bytes aus `buf` zuverlässig in `file`, unter robuster Behandlung partieller Schreibvorgänge und Signal-Unterbrechunge… | `src/durability.rs` |
| `pub fn apply_dont_dump(ptr: *const u8, len: usize) -> Result<()>` | Schließt eine Speicherregion per `madvise(MADV_DONTDUMP)` aus Core-Dumps aus (Unix). Auf Nicht-Unix-Systemen ist dies ein no-op. # Argument… | `src/mlock.rs` |
| `pub fn mem_lock(ptr: *const u8, len: usize) -> bool` | Sperrt eine Speicherregion im RAM mit der Standard-Richtlinie [`MemLockPolicy::Tolerant`]. | `src/mlock.rs` |
| `pub fn mem_lock_typed(ptr: *const u8, len: usize, policy: MemLockPolicy) -> Result<bool>` | Sperrt eine Speicherregion im RAM gemäß Richtlinie und liefert ein typisiertes [`Result<bool>`]. Liefert `Ok(true)`, wenn das Sperren erfol… | `src/mlock.rs` |
| `pub fn mem_lock_with_policy(ptr: *const u8, len: usize, policy: MemLockPolicy) -> IoResult<bool>` | Sperrt unter POSIX/Windows eine Speicherregion im RAM, um das Auslagern (Swapping) zu verhindern. | `src/mlock.rs` |
| `pub fn mem_unlock(addr: usize, len: usize)` | Entsperrt eine zuvor gesperrte Speicherregion im RAM. | `src/mlock.rs` |
| `pub fn set_fail_mlock(fail: bool, errno: i32)` | — | `src/mlock.rs` |
| `pub fn should_fail_mlock() -> Option<i32>` | — | `src/mlock.rs` |
| `pub fn as_file(&self) -> &File` | Gibt eine Referenz auf die zugrundeliegende [`File`] zurück. | `src/mmap.rs` |
| `pub fn as_slice(&self) -> &[u8]` | Gibt den gesamten Speicherinhalt als Byte-Slice zurück. | `src/mmap.rs` |
| `pub fn get_subslice(&self, offset: usize, len: usize) -> Result<&[u8]>` | Liest einen Teilbereich der Speicherabbildung mit sicherer Grenzenprüfung. # Fehler Liefert [`crate::Error::InvalidInput`], falls Bereichsg… | `src/mmap.rs` |
| `pub fn is_empty(&self) -> bool` | Prüft, ob die Speicherabbildung leer ist (0 Bytes). | `src/mmap.rs` |
| `pub fn len(&self) -> usize` | Gibt die Länge der Speicherabbildung in Bytes zurück. | `src/mmap.rs` |
| `pub fn map_readonly(&self) -> io::Result<SealedMap>` | Mappt die versiegelte Datei über die typgesicherte [`SealedMap`]-Abstraktion. | `src/mmap.rs` |
| `pub fn map_sealed(file: &SealedFile<SealedReadOnly>) -> io::Result<Self>` | Mappt eine nachweislich versiegelte Datei in den virtuellen Adressraum. # Fehler Liefert [`std::io::ErrorKind::InvalidData`], wenn die gema… | `src/mmap.rs` |
| `pub fn mmap_readonly(file: &File) -> io::Result<memmap2::Mmap>` | Erstellt eine unveränderliche Nur-Lese-Speicherabbildung (Read-Only Memory Map) für eine geöffnete Datei. # Vorbedingungen * `file`: Gültig… | `src/mmap.rs` |
| `pub fn new_verified(file: File, expected_len: u64) -> io::Result<Self>` | Erstellt eine versiegelte Dateireferenz nach erfolgreicher Integritäts- und Längenprüfung. # Fehler Liefert [`std::io::ErrorKind::InvalidDa… | `src/mmap.rs` |
| `pub fn verified_len(&self) -> u64` | Gibt die verifizierte Dateilänge in Bytes zurück. | `src/mmap.rs` |
| `pub fn ensure_fd_limit(min_fds: u64) -> bool` | Stellt sicher, dass das weiche Limit für offene Dateideskriptoren (`RLIMIT_NOFILE`) mindestens `min_fds` beträgt. Versucht das Limit bis zu… | `src/posix.rs` |
| `pub fn reopen_and_dup2(path: &Path, target_fd: i32, flags: i32) -> std::io::Result<()>` | Öffnet den Pfad `path` mit den angegebenen POSIX-Flags (z. B. `O_RDONLY`) und ersetzt atomar `target_fd` via `dup2`. # Argumente * `path`:… | `src/posix.rs` |
| `pub fn reopen_and_dup2(_path: &Path, _target_fd: i32, _flags: i32) -> std::io::Result<()>` | Fallback für Nicht-Linux-Systeme, auf denen `reopen_and_dup2` nicht unterstützt wird. | `src/posix.rs` |
| `pub fn set_fd_limit(limit: u64) -> bool` | Setzt das weiche und harte Dateideskriptorlimit (`RLIMIT_NOFILE`) des aktuellen Prozesses exakt auf `limit`. Liefert `true`, wenn das Limit… | `src/posix.rs` |
| `pub fn allocate(size: usize) -> Result<Self>` | Allokiert einen neuen Puffer der Größe `size` mit Nullen, verankert ihn im RAM (`MemLockPolicy::FailClosed`) und schließt ihn von Core-Dump… | `src/vault.rs` |
| `pub fn allocate_with_policy(size: usize, policy: MemLockPolicy) -> Result<Self>` | Allokiert einen neuen Puffer der Größe `size` gemäß der übergebenen Sperr-Richtlinie. | `src/vault.rs` |
| `pub fn as_mut_slice(&mut self) -> &mut [u8]` | Liefert die veränderbare Sicht auf den Pufferinhalt. | `src/vault.rs` |
| `pub fn as_slice(&self) -> &[u8]` | Liefert die unveränderliche Sicht auf den Pufferinhalt. | `src/vault.rs` |
| `pub fn from_slice(source: &[u8]) -> Result<Self>` | Allokiert einen Puffer und initialisiert ihn sofort mit einer Kopie der übergebenen sensiblen Daten. | `src/vault.rs` |
| `pub fn is_empty(&self) -> bool` | Gibt `true` zurück, wenn aktuell keine gesperrten Speicherregionen verwaltet werden. | `src/vault.rs` |
| `pub fn is_empty(&self) -> bool` | Prüft, ob der Puffer 0 Bytes lang ist. | `src/vault.rs` |
| `pub fn is_locked(&self) -> bool` | Gibt `true` zurück, wenn der Speicherbereich aktiv im physischen RAM verankert ist. | `src/vault.rs` |
| `pub fn len(&self) -> usize` | Gibt die Anzahl der aktuell nachverfolgten, gesperrten Regionen zurück. | `src/vault.rs` |
| `pub fn len(&self) -> usize` | Gibt die Kapazität/Länge des Puffers in Bytes zurück. | `src/vault.rs` |
| `pub fn lock_slice(&mut self, slice: &[u8], attempt_mlock: bool) -> bool` | Versucht einen Byte-Slice im RAM zu sperren, falls `attempt_mlock` wahr ist, unter Verwendung der Standardrichtlinie [`MemLockPolicy::Toler… | `src/vault.rs` |
| `pub fn lock_slice_with_policy( &mut self, slice: &[u8], attempt_mlock: bool, policy: MemLockPolicy, ) -> std::io::Result<bool>` | Versucht einen Byte-Slice im RAM gemäß der angegebenen [`MemLockPolicy`] zu sperren, falls `attempt_mlock == true`. | `src/vault.rs` |
| `pub fn new() -> Self` | Erstellt eine neue, leere Nachverfolgungsinstanz für gesperrte Speicherregionen. | `src/vault.rs` |
| `pub fn unlock_all(&mut self)` | Entsperrt alle registrierten Speicherregionen und leert die interne Nachverfolgungsliste. | `src/vault.rs` |
| `pub fn zeroize_slice(slice: &mut [u8])` | Überschreibt einen veränderbaren Byte-Slice sicher mit Nullen unter Verwendung von flüchtigen Schreibzugriffen (`write_volatile`) und einer… | `src/vault.rs` |


---

## `acta-simd`

| Attribut | Wert |
|---|---|
| **Ring** | 0 — Ebene E0 — Autarke Bausteine & Fundamente |
| **Status** | Fertig |
| **Zweck** | AVX2/AVX-512/NEON SIMD-Distanzberechnungskerne mit Laufzeit-Dispatch (Unsafe-Insel). |
| **Unsafe-Code** | Ja (deklarierte Unsafe-Insel) |
| **Interne Abhängigkeiten** | `acta-types` |
| **Externe Abhängigkeiten (Auswahl)** | `bytemuck`, `serde`, `thiserror`, `tracing` |

### Architekturbeschreibung

# Crate `acta-simd`  **Ring-Zugehörigkeit:** Ring 0 (Kern, unsafe-Insel).  **Zweck:** Diese Crate stellt hochoptimierte Vektor-Distanzberechnungen bereit. SIMD (Single Instruction, Multiple Data) nutzt spezielle Prozessorregister (z. B. 128-Bit bei ARM NEON, 256-Bit bei x86 AVX2, 512-Bit bei x86 AVX-512), um mehrere Gleitkomma- oder Ganzzahl-Operationen parallel mit einem einzigen CPU-Befehl auszuführen.  **Verbindung im System:** - **Abhängigkeiten (normal):** `bytemuck`, `thiserror`. Optional: `acta-types` (nur transitional via Feature `types-compat`), `serde`, `tracing`. - **Wird eingebunden von:** `acta-vector` (Ring 1) zur Beschleunigung von Ähnlichkeitssuchen in Vektorindizes (z. B. HNSW = Hierarchical Navigable Small World).  **Architektur:** 1. `kernels/`: Enthält isolierte Implementierungen für Skalar (Referenz), AVX2, AVX-512 und ARM NEON. 2. `dispatch.rs`: Ermittelt beim Start dynamisch die CPU-Fähigkeiten und wählt den schnellsten sicheren Pfad. 3. `lib.rs`: Bietet die sichere öffentliche Schnittstelle mit Vorab-Validierung gegen ungültige Werte (NaN/Unendlich).

### Modul- und Re-Export-Oberfläche (lib.rs)

```rust
pub mod contract;
pub mod dispatch;
pub mod error;
pub mod kernels;
pub mod metric;
pub mod compat;
pub use contract::{DistanceFn, MetricDistanceFn};
pub use dispatch::{ cosine_distance, cosine_distance_f32_bytes, cosine_similarity_parts_u8, dot_product_distance, dot_product_distance_f32_bytes, dot_product_u8, euclidean_distance, euclidean_distance_f32_bytes, euclidean_distance_sq_u8, Dispatcher, SimdTarget, };
pub use dispatch::{get_dispatch_override, set_dispatch_override, SimdPathOverride};
pub use error::{Error, Result, SimdError};
pub use kernels::scalar::{ cosine_distance_f32_bytes_scalar, cosine_distance_scalar, cosine_similarity_parts_f32_u8, cosine_similarity_parts_u8_scalar, dot_product_f32_bytes_scalar, dot_product_f32_u8, dot_product_scalar, dot_product_u8_scalar, euclidean_distance_f32_bytes_scalar, euclidean_distance_scalar, euclidean_distance_sq_f32_u8, euclidean_distance_sq_u8_scalar, normalize_inplace, CosineSimilarityPartsF32U8, CosineSimilarityPartsU8, };
pub use metric::DistanceMetric;
```

### Feature-Flags

| Flag | Aktiviert |
|---|---|
| `default` | `["std"]` |
| `std` | `[]` |
| `serde` | `["dep:serde"]` |
| `workspace` | `["dep:acta-types"]` |
| `types-compat` | `["workspace"]` |
| `simd-avx512` | `[]` |

### Schnittstellen — Public Traits (1)

**`DistanceFn`** — *Abstrakte Distanzfunktions-Schnittstelle zur dynamischen oder externen Injektion von Berechnungen. Ermöglicht es Vektor-Konsumenten (wie `acta-vector`), beliebige Distanz-Backends zustandslos und `Send + Sync` einzubinden.* (`src/contract.rs`)

```rust
pub trait DistanceFn : Send + Sync {
/// Berechnet die Distanz zwischen zwei gleich langen `f32`-Vektorslices.
    ///
    /// # Fehler
    /// Liefert `Err(SimdError)`, falls Dimensionen nicht übereinstimmen oder NaN/Inf vorliegt.
    fn distance(&self, a: &[f32], b: &[f32]) -> Result<f32>;
}
```

### Öffentliche Datentypen — Structs (4)

| Struct | Felder / Form | Beschreibung | Datei |
|---|---|---|---|
| `CosineSimilarityPartsF32U8` | `{ /// Accumulated dot product $\sum a_i (b_i \text{ as } f32)$. pub dot: f32, /// Accumulated squared L2 norm $\sum a_i^2$. pub norm_a_sq: f32, /// Accumulated s… }` | Struct holding raw float sum components for f32/u8 mixed cosine similarity calculation. | `src/kernels/scalar.rs` |
| `CosineSimilarityPartsU8` | `{ /// Accumulated dot product $\sum a_i b_i$. pub dot: u32, /// Accumulated squared L2 norm $\sum a_i^2$. pub norm_a_sq: u32, /// Accumulated squared L2 norm $\s… }` | Struct holding raw integer sum components for u8 cosine similarity calculation. | `src/kernels/scalar.rs` |
| `Dispatcher` | `{ target: SimdTarget, }` | Injectable and thread-safe SIMD dispatcher (Design by Contract). Encapsulates SIMD hardware target selection and execution without relying on mutable… | `src/dispatch.rs` |
| `MetricDistanceFn` | `{ /// Die auszuführende Distanzmetrik. pub metric: DistanceMetric, /// Der zugrundeliegende Dispatcher. pub dispatcher: crate::dispatch::Dispatcher, }` | Implementierung von [`DistanceFn`] für das Dispatcher-Gesamtwerkzeug mit konfigurierter Metrik. | `src/contract.rs` |

### Öffentliche Datentypen — Enums (3)

| Enum | Varianten | Beschreibung | Datei |
|---|---|---|---|
| `DistanceMetric` | `/// Kosinus-Distanz ($1.0 - \text{Kosinus-Ähnlichkeit}$). Wertebereich: $[0.0, 2.0]$. Cosine, /// Euklidische Distanz ($\\|a - b\\|_2 = \sqrt{\sum (a_i - b_i)^2}$). Wertebereich: $[0.0, \infty)$. Eucli…` | Unterstützte Distanzmetriken für Vektorberechnungen. Definiert als geschlossenes Wert-Enum (B09-Entscheidung). Besitzt bitidentische Serialisierung z… | `src/metric.rs` |
| `SimdError` | `/// Ungültige Vektoreingaben, beispielsweise `NaN` oder `±Inf` (INV-SIMD-FINITE-1). #[error("invalid input: {0}")] InvalidInput(&'static str), /// Ungleiche Vektordimensionen zwischen linkem und rech…` | Aufzählungs-Typ für alle im Crate auftretenden Fehlerzustände. Alle Varianten sind Fail-Closed konstruiert. | `src/error.rs` |
| `SimdTarget` | `/// Automatically detects CPU features at runtime (default). #[default] Auto, /// Forces scalar fallback execution without unsafe SIMD intrinsics. Scalar, /// Forces AVX2 + FMA SIMD execution path on…` | Explicit hardware target configuration for the SIMD dispatcher. | `src/dispatch.rs` |

### Typ-Aliase (3)

| Alias | Definition | Beschreibung |
|---|---|---|
| `Error` | `type Error= SimdError` | Typ-Alias `Error` für B4-Konformität (genau ein primärer Fehlertyp `Error`). |
| `Result` | `type Result<T> = core::result::Result<T, SimdError>` | Crate-lokales Result-Alias mit `SimdError` als Standard-Fehlertyp. |
| `SimdPathOverride` | `type SimdPathOverride= SimdTarget` | SIMD path override options for testing purposes. |

### Öffentliche Funktionen (40)

| Signatur | Beschreibung | Datei |
|---|---|---|
| `pub fn new(metric: DistanceMetric) -> Self` | Erstellt eine neue Distanzfunktion mit der angegebenen Metrik und Auto-Dispatch. | `src/contract.rs` |
| `pub fn auto() -> Self` | Creates a new `Dispatcher` with automatic CPU feature detection. | `src/dispatch.rs` |
| `pub fn cosine_distance(&self, a: &[f32], b: &[f32]) -> Result<f32>` | Computes cosine distance ($1.0 - \text{cosine\_similarity}$) between two `f32` vectors. | `src/dispatch.rs` |
| `pub fn cosine_distance(a: &[f32], b: &[f32]) -> Result<f32>` | Computes cosine distance ($1.0 - \text{cosine\_similarity}$) between two `f32` vectors. Dynamically dispatches to AVX-512, AVX2+FMA, or NEO… | `src/dispatch.rs` |
| `pub fn cosine_distance_f32_bytes(&self, a: &[f32], b_bytes: &[u8]) -> Result<f32>` | Computes cosine distance between an `f32` slice and raw little-endian `f32` byte slice. | `src/dispatch.rs` |
| `pub fn cosine_distance_f32_bytes(a: &[f32], b_bytes: &[u8]) -> Result<f32>` | Computes cosine distance between an `f32` slice and raw little-endian `f32` byte slice. Dynamically dispatches to AVX-512 or AVX2+FMA based… | `src/dispatch.rs` |
| `pub fn cosine_similarity_parts_u8( &self, a: &[u8], b: &[u8], ) -> Result<CosineSimilarityPartsU8>` | Computes cosine similarity component parts (dot product, norm_a_sq, norm_b_sq) for two `u8` byte slices. | `src/dispatch.rs` |
| `pub fn cosine_similarity_parts_u8(a: &[u8], b: &[u8]) -> Result<CosineSimilarityPartsU8>` | Computes cosine similarity component parts (dot product, norm_a_sq, norm_b_sq) for two `u8` byte slices. Dynamically dispatches to AVX-512… | `src/dispatch.rs` |
| `pub fn detect_best_target() -> SimdTarget` | Detects the best supported SIMD target on the host CPU, with support for caching and env override. | `src/dispatch.rs` |
| `pub fn dot_product_distance(&self, a: &[f32], b: &[f32]) -> Result<f32>` | Computes dot product distance (negated dot product $-a \cdot b$) between two `f32` vectors. | `src/dispatch.rs` |
| `pub fn dot_product_distance(a: &[f32], b: &[f32]) -> Result<f32>` | Computes dot product distance (negated dot product $-a \cdot b$) between two `f32` vectors. Dynamically dispatches to AVX-512, AVX2+FMA, or… | `src/dispatch.rs` |
| `pub fn dot_product_distance_f32_bytes(&self, a: &[f32], b_bytes: &[u8]) -> Result<f32>` | Computes dot product distance between an `f32` slice and raw little-endian `f32` byte slice. | `src/dispatch.rs` |
| `pub fn dot_product_distance_f32_bytes(a: &[f32], b_bytes: &[u8]) -> Result<f32>` | Computes dot product distance between an `f32` slice and raw little-endian `f32` byte slice. Dynamically dispatches to AVX-512 or AVX2+FMA… | `src/dispatch.rs` |
| `pub fn dot_product_u8(&self, a: &[u8], b: &[u8]) -> Result<u32>` | Computes integer dot product between two `u8` byte slices. | `src/dispatch.rs` |
| `pub fn dot_product_u8(a: &[u8], b: &[u8]) -> Result<u32>` | Computes integer dot product between two `u8` byte slices. Dynamically dispatches to AVX-512 VNNI (`dpbusd`) or AVX2 (`madd_epi16`), fallin… | `src/dispatch.rs` |
| `pub fn euclidean_distance(&self, a: &[f32], b: &[f32]) -> Result<f32>` | Computes Euclidean distance ($\|a - b\|_2$) between two `f32` vectors. | `src/dispatch.rs` |
| `pub fn euclidean_distance(a: &[f32], b: &[f32]) -> Result<f32>` | Computes Euclidean distance ($\|a - b\|_2$) between two `f32` vectors. Dynamically dispatches to AVX-512, AVX2+FMA, or NEON based on runtim… | `src/dispatch.rs` |
| `pub fn euclidean_distance_f32_bytes(&self, a: &[f32], b_bytes: &[u8]) -> Result<f32>` | Computes Euclidean distance between an `f32` slice and raw little-endian `f32` byte slice. | `src/dispatch.rs` |
| `pub fn euclidean_distance_f32_bytes(a: &[f32], b_bytes: &[u8]) -> Result<f32>` | Computes Euclidean distance between an `f32` slice and raw little-endian `f32` byte slice. Dynamically dispatches to AVX-512 or AVX2+FMA ba… | `src/dispatch.rs` |
| `pub fn euclidean_distance_sq_u8(&self, a: &[u8], b: &[u8]) -> Result<u32>` | Computes squared Euclidean distance ($\|a - b\|_2^2$) between two `u8` byte slices. | `src/dispatch.rs` |
| `pub fn euclidean_distance_sq_u8(a: &[u8], b: &[u8]) -> Result<u32>` | Computes squared Euclidean distance ($\|a - b\|_2^2$) between two `u8` byte slices. Dynamically dispatches to AVX-512 or AVX2 SIMD implemen… | `src/dispatch.rs` |
| `pub fn get_dispatch_override() -> Option<SimdPathOverride>` | Retrieves the current thread-local SIMD dispatch path override. | `src/dispatch.rs` |
| `pub fn set_dispatch_override(path: Option<SimdPathOverride>)` | Sets thread-local SIMD dispatch path override for testing. | `src/dispatch.rs` |
| `pub fn cosine_distance_f32_bytes_scalar(a: &[f32], b_bytes: &[u8]) -> f32` | Scalar reference implementation for cosine distance between `f32` slice and raw LE `f32` byte slice. # Semantics Decodes `f32` values from… | `src/kernels/scalar.rs` |
| `pub fn cosine_distance_scalar(a: &[f32], b: &[f32]) -> f32` | Scalar reference implementation for cosine distance on `f32` slices. # Semantics Calculates $1.0 - \text{cosine\_similarity}(a, b)$. Cosine… | `src/kernels/scalar.rs` |
| `pub fn cosine_similarity_parts_f32_u8(a: &[f32], b: &[u8]) -> CosineSimilarityPartsF32U8` | Scalar reference implementation for mixed `f32`/`u8` cosine similarity component parts. # Semantics Returns [`CosineSimilarityPartsF32U8`]… | `src/kernels/scalar.rs` |
| `pub fn cosine_similarity_parts_u8_scalar(a: &[u8], b: &[u8]) -> CosineSimilarityPartsU8` | Scalar reference implementation for `u8` cosine similarity component parts. # Semantics Returns [`CosineSimilarityPartsU8`] containing dot… | `src/kernels/scalar.rs` |
| `pub fn dot_product_f32_bytes_scalar(a: &[f32], b_bytes: &[u8]) -> f32` | Scalar reference implementation for dot product distance between `f32` slice and raw LE `f32` byte slice. # Semantics Decodes `f32` values… | `src/kernels/scalar.rs` |
| `pub fn dot_product_f32_u8(a: &[f32], b: &[u8]) -> f32` | Scalar reference implementation for mixed `f32`/`u8` dot product. # Semantics Calculates $\sum a_i (b_i \text{ as } f32)$. # Complexity $O(… | `src/kernels/scalar.rs` |
| `pub fn dot_product_scalar(a: &[f32], b: &[f32]) -> f32` | Scalar reference implementation for dot product distance on `f32` slices. # Semantics Calculates negated dot product $- \sum a_i b_i$. # Co… | `src/kernels/scalar.rs` |
| `pub fn dot_product_u8_scalar(a: &[u8], b: &[u8]) -> u32` | Scalar reference implementation for unsigned integer dot product between two `u8` byte slices. # Semantics Calculates $\sum a_i b_i$ with 3… | `src/kernels/scalar.rs` |
| `pub fn euclidean_distance_f32_bytes_scalar(a: &[f32], b_bytes: &[u8]) -> f32` | Scalar reference implementation for Euclidean distance between `f32` slice and raw LE `f32` byte slice. # Semantics Decodes `f32` values fr… | `src/kernels/scalar.rs` |
| `pub fn euclidean_distance_scalar(a: &[f32], b: &[f32]) -> f32` | Scalar reference implementation for Euclidean distance on `f32` slices. # Semantics Calculates $\sqrt{\sum (a_i - b_i)^2}$. # Complexity $O… | `src/kernels/scalar.rs` |
| `pub fn euclidean_distance_sq_f32_u8(a: &[f32], b: &[u8], alphas: &[f32], mins: &[f32]) -> f32` | Scalar reference implementation for mixed `f32`/`u8` scalar dequantized squared Euclidean distance. # Semantics Calculates $\sum (a_i - (b_… | `src/kernels/scalar.rs` |
| `pub fn euclidean_distance_sq_u8_scalar(a: &[u8], b: &[u8]) -> u32` | Scalar reference implementation for squared Euclidean distance between two `u8` byte slices. # Semantics Calculates $\sum (a_i - b_i)^2$ wi… | `src/kernels/scalar.rs` |
| `pub fn normalize_inplace(v: &mut [f32])` | Normalizes a vector in-place to unit L2 length. # Semantics Divides each element by $\sqrt{\sum v_i^2}$. Zero-norm vectors remain unmodifie… | `src/kernels/scalar.rs` |
| `pub fn compute_distance(a: &[f32], b: &[f32], metric: DistanceMetric) -> Result<f32>` | Berechnet die Distanz zwischen zwei Vektoren unter Verwendung der angegebenen Metrik inklusive Validierung. Diese Funktion ist der primäre… | `src/lib.rs` |
| `pub fn compute_distance_f32_bytes_trusted( a: &[f32], b_bytes: &[u8], metric: DistanceMetric, ) -> Result<f32>` | Berechnet die Distanz zwischen einem `f32`-Vektor und einem rohen Byte-Puffer (`&[u8]`), der `f32`-Werte darstellt. Dies ist besonders nütz… | `src/lib.rs` |
| `pub fn compute_distance_trusted(a: &[f32], b: &[f32], metric: DistanceMetric) -> Result<f32>` | Berechnet die Distanz zwischen zwei bereits validierten `f32`-Vektoren ohne erneute NaN-Prüfung. Wird intern verwendet, wenn Vektoren berei… | `src/lib.rs` |
| `pub fn validate_vector(vec: &[f32]) -> Result<()>` | Prüft, ob ein Vektor gültige endliche Zahlen enthält (kein NaN und kein Unendlich). NaN (Not a Number) und Unendlich führen in SIMD-Registe… | `src/lib.rs` |


---

## `acta-crypto`

| Attribut | Wert |
|---|---|
| **Ring** | 1 — Ebene E1 — Speicherschicht, Indizes & KI-Kerne |
| **Status** | Fertig |
| **Zweck** | AES-256-GCM-SIV, WAL-HMAC-Integritätsketten, Zeroize und KeyManager. |
| **Unsafe-Code** | Nein (`#![forbid(unsafe_code)]`) |
| **Interne Abhängigkeiten** | `acta-env`, `acta-ids`, `acta-ports`, `acta-types` |
| **Externe Abhängigkeiten (Auswahl)** | `aes-gcm-siv`, `ahash`, `argon2`, `bincode`, `blake3`, `hkdf`, `hmac`, `lru`, `parking_lot`, `rand`, `regex`, `serde`, `serde_json`, `sha2`, `subtle`, `thiserror`, `tracing`, `zeroize` |

### Modul- und Re-Export-Oberfläche (lib.rs)

```rust
pub mod aead;
pub mod anti_tamper;
pub mod audit_chain;
pub mod contract;
pub mod compat;
pub mod crypto;
pub mod dek;
pub mod error;
pub mod kdf;
pub mod kv_cipher;
pub mod kv_segment;
pub mod secret;
pub mod wal_completeness;
pub mod wal_crypto;
pub use aead::*;
pub use contract::*;
pub use dek::{derive_dek, AncestorSalts, Dek, UnitKey};
pub use kdf::hkdf_expand;
pub use secret::SecretKey;
pub use audit_chain::{ compute_record_commitment, AuditChain, AuditChainEntry, AuditChainHeadSignature, DataClass, EncryptedCommitmentSalt, };
pub use crypto::KeyManager as CryptoKey;
pub use crypto::KeyManager;
pub use error::{CryptoError, Result};
pub use kdf::{derive_key_argon2id, KdfHeader, KdfParams};
pub use kv_cipher::KvSegmentCipher;
pub use kv_cipher::{EncryptedKvLayer, KvCipher, ModelFingerprint};
pub use kv_segment::{ delete_mode::KvDeleteMode, eviction_worker::{emergency_wipe, EvictionWorker}, manager::{KvSegmentConfig, KvSegmentManager, KvSegmentPayload}, segment::KvSegment, store::TenantIsolatedKvStore, CURRENT_KV_KEY_DERIVATION_VERSION, };
pub use wal_completeness::verify_wal_chain_completeness;
pub use wal_crypto::CloudEgressGuard;
pub use wal_crypto::{ EncryptedWal, IntegrityVerifier, KmsProvider, LocalKeyManagerKms, WalEntrySnapshot, WalHmac, };
```

### Feature-Flags

| Flag | Aktiviert |
|---|---|
| `default` | `["kv-encryption", "workspace"]` |
| `workspace` | `["dep:acta-types", "dep:acta-ports"]` |
| `types-compat` | `["workspace"]` |
| `ports-compat` | `["workspace"]` |
| `test-utils` | `[]` |
| `kv-encryption` | `[]` |
| `cloud-egress-guard` | `[]` |

### Schnittstellen — Public Traits (4)

**`CryptoEntropy`** — *Trait zur Bereitstellung kryptographischer Entropie (Zufallsbytes). Ermöglicht die Injektion deterministischer Test-Zufallsquellen oder sicherer CSPRNGs.* (`src/contract.rs`)

```rust
pub trait CryptoEntropy : Send + Sync {
/// Füllt den übergebenen Puffer mit pseudozufälligen Bytes.
    fn fill_bytes(&mut self, dest: &mut [u8]);
}
```

**`KmsContract`** — *Schnittstelle für Key Management Services (KMS) zur sicheren Schlüsselableitung und Hüllkurven-Verschlüsselung.* (`src/contract.rs`)

```rust
pub trait KmsContract : Send + Sync {
/// Liefert den aktiven Datenverschlüsselungsschlüssel (DEK), genullt bei Freigabe.
    fn get_key(&self) -> Result<Zeroizing<Vec<u8>>>;

    /// Liefert die Kennung (ID) des aktiven Schlüssels.
    fn active_key_id(&self) -> &str;

    /// Leitet einen abgeleiteten Schlüssel anhand einer `key_id` ab.
    fn derive_data_key(&self, key_id: &[u8]) -> Result<Zeroizing<Vec<u8>>>;

    /// Wickelt (verschlüsselt) Daten mit dem angegebenen Schlüssel ein.
    fn wrap_key(&self, key_id: &[u8], plaintext: &[u8]) -> Result<Vec<u8>>;

    /// Entpackt (entschlüsselt) Daten mit dem angegebenen Schlüssel.
    fn unwrap_key(&self, key_id: &[u8], wrapped: &[u8]) -> Result<Zeroizing<Vec<u8>>>;
}
```

**`KmsProvider`** — *Key Management Strategy interface for data key derivation, key wrapping/unwrapping, and key rotation.* (`src/wal_crypto.rs`)

```rust
pub trait KmsProvider : Send + Sync {
/// Retrieves the Data Encryption Key (DEK) zeroized on drop.
    fn get_key(&self) -> Result<Zeroizing<Vec<u8>>>;

    /// Returns the active key identifier or version alias.
    fn active_key_id(&self) -> &str;

    /// Derives a raw data key bound to the specified `key_id`.
    fn derive_data_key(&self, key_id: &[u8]) -> Result<Zeroizing<Vec<u8>>>;

    /// Wraps (encrypts) plaintext data or a raw key using the key bound to `key_id`.
    fn wrap_key(&self, key_id: &[u8], plaintext: &[u8]) -> Result<Vec<u8>>;

    /// Unwraps (decrypts) wrapped ciphertext data or key using the key bound to `key_id`.
    fn unwrap_key(&self, key_id: &[u8], wrapped: &[u8]) -> Result<Zeroizing<Vec<u8>>>;

    /// Rotates the active master key or key version and returns the new active key ID.
    fn rotate_key(&mut self) -> Result<String>;
}
```

**`KvCipher`** — *Abstract cipher trait for KV block encryption and seal/open operations.* (`src/kv_cipher.rs`)

```rust
pub trait KvCipher : Send + Sync {
/// Encrypts (seals) plaintext byte payload into authenticated ciphertext bytes.
    fn seal(&self, plaintext: &[u8]) -> Result<Vec<u8>>;
    /// Decrypts (opens) authenticated ciphertext bytes back to plaintext byte payload.
    fn open(&self, ciphertext: &[u8]) -> Result<Vec<u8>>;
}
```

### Öffentliche Datentypen — Structs (34)

| Struct | Felder / Form | Beschreibung | Datei |
|---|---|---|---|
| `AncestorSalts` | `{ pub t: Salt, pub c: Salt, pub s: Option<Salt>, pub d: Salt, }` | — | `src/dek.rs` |
| `AuditChain` | `{ /// Öffentlicher Zugriff auf die Liste der Ketten-Einträge für Prüfungen/Tests. pub entries: Vec<AuditChainEntry>, head_hash: [u8; 32], }` | Manipulationssichere Audit-Kette mit BLAKE3-Hash-Verkettung. | `src/audit_chain.rs` |
| `AuditChainEntry` | `{ /// Fortlaufender Index in der Audit-Kette (0-basiert). pub index: u64, /// Identifikator des Datenschemas. pub schema_id: String, /// Blake3-Hash des zum Zeit… }` | Einzelner Eintrag in der manipulationssicheren BLAKE3-Audit-Hash-Kette. | `src/audit_chain.rs` |
| `AuditChainHeadSignature` | `{ /// Index des signierten Kettenkopfs. pub chain_index: u64, /// Blake3-Hash des Kettenkopfs. pub head_hash: [u8; 32], /// Signatur-Bytes. pub signature: Vec<u8… }` | Periodisch erzeugte Signatur über den aktuellen Kettenkopf. | `src/audit_chain.rs` |
| `AuditHeadAnchor` | `{ /// Sequenznummer / Kettenindex des extern verankerten Kettenkopfs. pub seq_no: u64, /// Blake3-Hash des Kettenkopfs an der Position `seq_no`. pub head_hash: [… }` | Externer Kopf-Anker zur Rollback-Erkennung für die Audit-Hash-Kette. | `src/audit_chain.rs` |
| `Blake3Chain` | `{ current_hash: [u8; 32], step_count: u64, }` | — | `src/anti_tamper.rs` |
| `CloudEgressGuard` | `{ max_egress_bytes: usize, }` | Egress guard for inspecting and sanitizing encrypted payload blocks prior to cloud egress. | `src/wal_crypto.rs` |
| `Dek` | `(tuple/unit)` | — | `src/dek.rs` |
| `DerivedKey` | `(tuple/unit)` | Wrapper für abgeleitetes 256-Bit Schlüsselmaterial, das bei Drop automatisch aus dem Speicher gelöscht wird. | `src/kdf.rs` |
| `DeterministicEntropy` | `{ state: u64, }` | Standard-Implementierung von [`CryptoEntropy`] basierend auf einem deterministischen Generator. | `src/contract.rs` |
| `EncryptedCommitmentSalt` | `{ /// Shredding Group ID zur Verwaltung des Sub-Keys. pub group_id: u64, /// Verschlüsselter Salt. pub ciphertext: Vec<u8>, /// AES-GCM-SIV Nonce (12 Bytes). pub… }` | Verschlüsselter Salt-Wert für ein Record-Commitment. | `src/audit_chain.rs` |
| `EncryptedKvLayer` | `{ /// Serialized format version. Must equal [`CURRENT_KV_FORMAT_VERSION`] on decrypt. #[zeroize(skip)] pub format_version: u8, /// AES-256-GCM-SIV ciphertext con… }` | Container for an encrypted KV-cache segment layer. Plaintext data and nonce are zeroized on drop. Public metadata (`tenant_id`, `model_fingerprint`,… | `src/kv_cipher.rs` |
| `EncryptedSegmentPayload` | `{ /// Encrypted KV layer container from acta-crypto. Zeroized on drop. pub layer: EncryptedKvLayer, }` | Encrypted layer representation stored inside a KvSegment when encryption is active. | `src/kv_segment/segment.rs` |
| `EncryptedWal` | `{ kms: Arc<dyn KmsProvider>, }` | Encrypted WAL chunk provider that handles transparent encryption/decryption of WAL payloads via KMS. # Invariants - Delegates encryption and key mana… | `src/wal_crypto.rs` |
| `EvictionWorker` | `{ sender: parking_lot::Mutex<mpsc::Sender<EvictionCommand>>, handle: parking_lot::Mutex<Option<std::thread::JoinHandle<()>>>, }` | Hintergrund-Worker für die Speicherfreigabe nach dem Least-Recently-Used-Prinzip (LRU). | `src/kv_segment/eviction_worker.rs` |
| `InMemoryKms` | `{ active_id: String, master_key: Zeroizing<[u8; 32]>, }` | I/O-freie In-Memory Referenzimplementierung von [`KmsContract`]. | `src/contract.rs` |
| `IntegrityVerifier` | `{ last_hmac: [u8; 32], integrity_key: Zeroizing<Vec<u8>>, last_seq_no: Option<u64>, }` | Stateful verifier for WAL HMAC-SHA256 hash chains. # Invariants - Maintains running `last_hmac` state across entries to enforce sequential hash-chain… | `src/wal_crypto.rs` |
| `KdfHeader` | `{ pub version: u8, pub kdf_id: u8, pub params: KdfParams, pub salt: Vec<u8>, }` | Versionierter KDF-Metadaten-Header zur deterministischen Reproduktion von Schlüsseln. Speichert die Argon2id-Kostenparameter (m_cost, t_cost, p_cost)… | `src/kdf.rs` |
| `KdfParams` | `{ pub m_cost_kib: u32, pub t_cost: u32, pub p_cost: u32, }` | Konfiguration der Argon2id KDF-Kostenparameter. | `src/kdf.rs` |
| `KeyManager` | `{ key: VolatileEncryptionKey, cipher_cache: OnceLock<Aes256GcmSiv>, nonce_prefix: [u8; 4], nonce_counter: AtomicU64, }` | Zentraler kryptographischer Schlüsselmanager für symmetrische Verschlüsselung. Hält den Master-Key im RAM, leitet per HKDF-Expand datei- und mandante… | `src/crypto.rs` |
| `KvSegment` | `{ #[zeroize(skip)] // Metadaten, keine sensiblen Tensor-Daten pub tenant_id: TenantId, #[zeroize(skip)] pub segment_id: u64, /// Versionsnummer der HKDF-Schlüsse… }` | — | `src/kv_segment/segment.rs` |
| `KvSegmentCipher` | `{ key_manager: KeyManager, }` | Chiffre für segmentweise verschlüsselte KV-Einträge. | `src/kv_cipher.rs` |
| `KvSegmentConfig` | `{ /// Delete mode specifying whether segments are tombstone-only or crypto-shredded. pub delete_mode: KvDeleteMode, }` | Configuration for KV segment storage behavior. | `src/kv_segment/manager.rs` |
| `KvSegmentManager` | `{ config: KvSegmentConfig, master_key: Option<Arc<KeyManager>>, }` | — | `src/kv_segment/manager.rs` |
| `KvSegmentPayload` | `{ /// Group ID corresponding to the segment/group. pub group_id: u64, /// Physical payload stored on disk (ciphertext or raw plaintext). pub payload: Vec<u8>, //… }` | Representation of a written KV segment (encrypted or raw payload with group_id and nonce). | `src/kv_segment/manager.rs` |
| `LocalKeyManagerKms` | `{ key_manager: KeyManager, key_id: String, rotation_counter: u64, }` | Standard local implementation of [`KmsProvider`] wrapping [`KeyManager`]. Provides thread-safe, zeroized key derivation, wrapping, unwrapping, and ro… | `src/wal_crypto.rs` |
| `ModelFingerprint` | `{ /// SHA-256 hash digest over model weight blob and quantization tier string. pub hash: [u8; 32], /// Model identifier or filename (e.g. "llama-3.2-3b-instruct"… }` | Uniquely identifies a model weight file and its quantization tier. | `src/kv_cipher.rs` |
| `Secret` | `{ inner: T, }` | — | `src/secret.rs` |
| `SecretKey` | `{ bytes: [u8; N], }` | — | `src/secret.rs` |
| `TenantIsolatedKvStore` | `{ /// 32 cache-line-gepaddete RwLock-Shards. Box<[_]> für flexible Shard-Anzahl. shards: Box<[Shard]>, shard_count: usize, /// Globaler Offset für Shard-Rotation… }` | — | `src/kv_segment/store.rs` |
| `UnitKey` | `(tuple/unit)` | — | `src/dek.rs` |
| `VolatileEncryptionKey` | `{ key_bytes: Zeroizing<[u8; 32]>, // AES-256 Key }` | — | `src/anti_tamper.rs` |
| `WalEntrySnapshot` | `{ pub tx_id: u64, pub seq_no: u64, pub op_type: u8, // 0: Put, 1: Delete, 2: TxEnd pub key: Vec<u8>, pub value: Vec<u8>, pub checksum: [u8; 32], pub prev_hmac: [… }` | Immutable snapshot of a WAL entry used for cryptographic integrity verification. # Invariants - Captures entry fields (`seq_no`, `op_type`, `key`, `v… | `src/wal_crypto.rs` |
| `WalHmac` | `{ mac: Hmac<Sha256>, }` | Stateful wrapper around HMAC-SHA256 initialized with WAL domain separation. # Invariants - Initialized with `b"acta-wal-v1"` domain separation string… | `src/wal_crypto.rs` |

### Öffentliche Datentypen — Enums (3)

| Enum | Varianten | Beschreibung | Datei |
|---|---|---|---|
| `CryptoError` | `#[error("key derivation failed: {0}")] KeyDerivation(String), #[error("encryption failed: {0}")] Encryption(String), #[error("decryption failed: {0}")] Decryption(String), #[error("AEAD operation fai…` | — | `src/error.rs` |
| `DataClass` | `/// Öffentliche Daten. Public, /// Interne Betriebsdaten. Internal, /// Vertrauliche Geschäftsdaten. Confidential, /// Streng vertrauliche / reglementierte Daten. Restricted, /// Personenbezogene Dat…` | Datenklassifikation für Audit-Einträge (keine Freitext-Strings). | `src/audit_chain.rs` |
| `KvDeleteMode` | `TombstoneOnly, CryptoShred,` | — | `src/kv_segment/delete_mode.rs` |

### Typ-Aliase (4)

| Alias | Definition | Beschreibung |
|---|---|---|
| `DocTag` | `type DocTag= [u8` | — |
| `Result` | `type Result<T> = std::result::Result<T, CryptoError>` | Result type alias for cryptographic operations in `acta-crypto`. |
| `Salt` | `type Salt= [u8` | — |
| `SpillHandler` | `type SpillHandler= Arc<dyn Fn(TenantId, u64, Vec<u8>) + Send + Sync>` | Optionaler Callback-Hook für Tier-2-LSM-Spill bei Eviction aus dem In-Memory LRU Cache. |

### Konstanten (18)

| Konstante | Typ | Beschreibung |
|---|---|---|
| `CURRENT_KV_FORMAT_VERSION` | `u8` | Current serialized format version for [`EncryptedKvLayer`]. Increment when the serialized layout changes in a breaking way. |
| `CURRENT_KV_KEY_DERIVATION_VERSION` | `u8` | Aktuelle Version der KV-Segment-Schlüsselableitung. Erhöhe diesen Wert, wenn sich der HKDF-Info-String oder der Salt-Aufbau ändert. |
| `DEFAULT_M_COST_KIB` | `u32` | OWASP Standardempfehlungen für Argon2id |
| `DEFAULT_P_COST` | `u32` | Standard-Parallelitätsgrad für Argon2id. |
| `DEFAULT_SEGMENT_CAPACITY_PER_TENANT` | `usize` | — |
| `DEFAULT_SHARD_COUNT` | `usize` | — |
| `DEFAULT_T_COST` | `u32` | Standard-Zeitkosten (Iterationen) für Argon2id. |
| `DEK_LABEL` | `&[u8]` | — |
| `DEK_SALT_LABEL` | `&[u8]` | — |
| `KDF_HEADER_MAGIC` | `&[u8` | Magischer Header-Marker "MFKD" (Acta Key Derivation) |
| `KDF_HEADER_VERSION_1` | `u8` | Aktuelle KDF Header Version |
| `KDF_ID_ARGON2ID` | `u8` | Identifikator für Argon2id |
| `MIN_M_COST_KIB` | `u32` | OWASP Mindestanforderungen für Argon2id |
| `MIN_P_COST` | `u32` | Minimaler Parallelitätsgrad für Argon2id gemäß OWASP. |
| `MIN_SALT_LEN` | `usize` | Min. empfohlener Salt-Umfang (16 Bytes) |
| `MIN_T_COST` | `u32` | Minimale Anzahl an Iterationen (Zeitkosten) für Argon2id gemäß OWASP. |
| `REC_AAD` | `&[u8]` | — |
| `WRAP_AAD` | `&[u8]` | — |

### Öffentliche Funktionen (134)

| Signatur | Beschreibung | Datei |
|---|---|---|
| `pub fn open_record(dek: &Dek, key_bytes: &[u8], env: &[u8]) -> Result<Zeroizing<Vec<u8>>>` | Opens an encrypted record envelope, verifying the associated data and authentication tag. Fails with a uniform `CryptoError::Aead` on any a… | `src/aead.rs` |
| `pub fn seal_record(dek: &Dek, doc: [u8; 16], key_bytes: &[u8], plain: &[u8]) -> Vec<u8>` | — | `src/aead.rs` |
| `pub fn seal_record_with_nonce( dek: &Dek, doc: [u8; 16], key_bytes: &[u8], plain: &[u8], nonce: [u8; 12], ) -> Result<Vec<u8>>` | Seals a plaintext record into an envelope using an explicit, caller-provided 12-byte nonce. Binds `doc` and `key_bytes` into the Associated… | `src/aead.rs` |
| `pub fn seal_record_with_rng<R: RngCore>( dek: &Dek, doc: [u8; 16], key_bytes: &[u8], plain: &[u8], rng: &mut R, ) -> Result<Vec<u8>>` | — | `src/aead.rs` |
| `pub fn as_bytes(&self) -> &[u8]` | Accessor for cryptographic operations. | `src/anti_tamper.rs` |
| `pub fn constant_time_compare(a: &[u8], b: &[u8]) -> bool` | Konstantzeit-Verifikation zweier Byte-Slices (`INV-CRYPTO-CONSTANT-TIME-1`). Verhindert Timing-Orakel bei MAC-, Tag- und Prüfsummen-Validie… | `src/anti_tamper.rs` |
| `pub fn current_hash(&self) -> &[u8` | Gibt den aktuellen Ketten-Hash zurück. | `src/anti_tamper.rs` |
| `pub fn emergency_wipe(&mut self)` | Emergency Trigger: Explicitly wipes the key from memory. This is to be hooked into hardware sensors (e.g. crash/voltage). | `src/anti_tamper.rs` |
| `pub fn extend(&mut self, context: &[u8], data: &[u8]) -> [u8` | Aktualisiert den Zustand der Kette mit einem neuen Block und Kontext in AAD-Disziplin. | `src/anti_tamper.rs` |
| `pub fn inspect_key_bytes_for_test(&self) -> &[u8` | Test-only method to inspect key bytes. | `src/anti_tamper.rs` |
| `pub fn new(raw: [u8; 32]) -> Self` | Creates a new volatile key from a raw 32-byte array. | `src/anti_tamper.rs` |
| `pub fn new(genesis: [u8; 32]) -> Self` | Initialisiert eine neue BLAKE3-Kette mit einem Seed/Genesis-Hash. | `src/anti_tamper.rs` |
| `pub fn step_count(&self) -> u64` | Gibt die bisherige Anzahl an Kettengliedern zurück. | `src/anti_tamper.rs` |
| `pub fn verify_current_hash(&self, expected: &[u8; 32]) -> Result<()>` | Verifiziert in konstanter Zeit, ob der aktuelle Hash mit einem erwarteten Wert übereinstimmt. | `src/anti_tamper.rs` |
| `pub fn verify_tag_constant_time(expected: &[u8], actual: &[u8]) -> Result<()>` | — | `src/anti_tamper.rs` |
| `pub fn append( &mut self, schema_id: impl Into<String>, rules_hash: [u8; 32], data_class: DataClass, doc_id: DocId, tx_id: TxId, commitment: Option<[u8; 32]>, ) -> Result<&AuditChainEntry>` | Fügt einen neuen Eintrag zur Audit-Kette hinzu und aktualisiert den Kettenkopf-Hash. | `src/audit_chain.rs` |
| `pub fn compute_hash( index: u64, schema_id: &str, rules_hash: &[u8; 32], data_class: DataClass, doc_id: DocId, tx_id: TxId, prev_hash: &[u8; 32], commitment: Option<&[u8; 32]>, ) -> Result<[u8` | Berechnet den Blake3-Hash für die Felder eines Audit-Ketten-Eintrags. | `src/audit_chain.rs` |
| `pub fn compute_record_commitment(salt: &[u8], attr: &[u8]) -> [u8` | Berechnet das Commitment `Blake3(salt || attr)`. | `src/audit_chain.rs` |
| `pub fn entries(&self) -> &[AuditChainEntry]` | Gibt eine Referenz auf alle Einträge der Kette zurück. | `src/audit_chain.rs` |
| `pub fn head_hash(&self) -> [u8` | Liefert den aktuellen Kettenkopf-Hash. | `src/audit_chain.rs` |
| `pub fn is_empty(&self) -> bool` | Prüft, ob die Kette leer ist. | `src/audit_chain.rs` |
| `pub fn len(&self) -> usize` | Liefert die Anzahl der Einträge in der Kette. | `src/audit_chain.rs` |
| `pub fn new(seq_no: u64, head_hash: [u8; 32]) -> Self` | Erstellt einen neuen nicht signierten `AuditHeadAnchor`. | `src/audit_chain.rs` |
| `pub fn new() -> Self` | Erzeugt eine neue, leere Audit-Kette. | `src/audit_chain.rs` |
| `pub fn sign_head_with_bytes( &self, signature_bytes: Vec<u8>, ) -> Result<AuditChainHeadSignature>` | Periodischer Aufruf: Erzeugt eine Kettenkopf-Signatur / Snapshot. | `src/audit_chain.rs` |
| `pub fn verify_chain(&self) -> Result<bool>` | Verifiziert die vollständige Kette auf Integrität. # Warnhinweis / Security Warning Diese Methode prüft lediglich die **interne kryptograph… | `src/audit_chain.rs` |
| `pub fn verify_chain_with_anchor( &self, anchor: &AuditHeadAnchor, ) -> Result<bool>` | Verifiziert die Kette gegen einen externen Kopf-Anker ([`AuditHeadAnchor`]). Schlägt fehl (`Ok(false)`), wenn: 1. Die Kette kürzer ist als… | `src/audit_chain.rs` |
| `pub fn with_signature( seq_no: u64, head_hash: [u8; 32], head_signature: AuditChainHeadSignature, ) -> Self` | Erstellt einen neuen signierten `AuditHeadAnchor`. | `src/audit_chain.rs` |
| `pub fn new(seed: u64) -> Self` | Erzeugt eine neue deterministische Entropiequelle mit festem Startwert (Seed). | `src/contract.rs` |
| `pub fn new(active_id: impl Into<String>, master_key: [u8; 32]) -> Self` | Erzeugt eine neue `InMemoryKms`-Instanz mit einem 32-Byte Master-Schlüssel. | `src/contract.rs` |
| `pub fn cipher_for(&self, tenant_id: acta_ids::TenantId) -> Result<Self>` | Derives a tenant-isolated `KeyManager` instance for a specific `TenantId`. | `src/crypto.rs` |
| `pub fn cipher_for_bytes( &self, tenant_id: acta_ids::TenantId, fingerprint: &[u8; 32], ) -> Result<Self>` | Derives a tenant-isolated `KeyManager` instance for a specific `TenantId` with raw 32-byte fingerprint. | `src/crypto.rs` |
| `pub fn cipher_for_scoped<T>( &self, scoped_key: acta_ids::TenantScoped<T>, expected_tenant_id: &acta_ids::TenantId, ) -> Result<Self>` | Derives a tenant-isolated `KeyManager` instance enforcing compile-time `TenantScoped` verification. Unpacks `scoped_key` only if the bound… | `src/crypto.rs` |
| `pub fn cipher_for_scoped_bytes<T>( &self, scoped_key: acta_ids::TenantScoped<T>, expected_tenant_id: &acta_ids::TenantId, fingerprint: &[u8; 32], ) -> Result<Self>` | Derives a tenant-isolated `KeyManager` instance enforcing compile-time `TenantScoped` verification with raw 32-byte fingerprint. | `src/crypto.rs` |
| `pub fn decrypt_auto_nonce(&self, ciphertext: &[u8], nonce_bytes: &[u8; 12]) -> Result<Vec<u8>>` | Decrypts a block of data using a full 12-byte nonce. | `src/crypto.rs` |
| `pub fn derive_file_key(&self, file_id: &[u8]) -> Result<Self>` | Derives a sub-key for a specific file or logical stream. This prevents nonce-reuse when multiple files use the same master key. | `src/crypto.rs` |
| `pub fn derive_kv_key( &self, tenant_id: acta_ids::TenantId, model_fingerprint: &crate::kv_cipher::ModelFingerprint, ) -> Result<Self>` | All KV-cache segments encrypted before v2 are cryptographically unreadable under v2 keys and will fail at the `format_version` check in `Kv… | `src/crypto.rs` |
| `pub fn derive_kv_key_bytes( &self, tenant_id: acta_ids::TenantId, fingerprint: &[u8; 32], ) -> Result<Self>` | Derives a sub-key for KV-cache segment encryption using raw 32-byte model fingerprint. | `src/crypto.rs` |
| `pub fn derive_kv_key_scoped( &self, scoped_fingerprint: acta_ids::TenantScoped<&crate::kv_cipher::ModelFingerprint>, expected_tenant_id: &acta_ids::TenantId, ) -> Result<Self>` | Derives a sub-key for KV-cache segment encryption with strict compile-time tenant scope enforcement. Unpacks `scoped_fingerprint` only if t… | `src/crypto.rs` |
| `pub fn derive_kv_key_scoped_bytes( &self, scoped_fingerprint: acta_ids::TenantScoped<&[u8; 32]>, expected_tenant_id: &acta_ids::TenantId, ) -> Result<Self>` | Derives a sub-key for KV-cache segment encryption with strict compile-time tenant scope enforcement using raw 32-byte model fingerprint. | `src/crypto.rs` |
| `pub fn derive_segment_key(&self, info: &str) -> Result<Self>` | Derives a sub-key specifically for a KV-cache segment based on a versioned info string. | `src/crypto.rs` |
| `pub fn emergency_wipe(&mut self)` | Emergency Trigger: Explicitly wipes the key from memory and clears cipher cache. | `src/crypto.rs` |
| `pub fn encrypt_auto_nonce(&self, data: &[u8]) -> Result<(Vec<u8>, [u8` | Encrypts a block of data with a deterministically generated 12-byte nonce. Returns the ciphertext and the full 12-byte nonce used. | `src/crypto.rs` |
| `pub fn export_dek(&self) -> Zeroizing<Vec<u8>>` | Exports the 32-byte raw Data Encryption Key (DEK) wrapped in [`Zeroizing<Vec<u8>>`]. | `src/crypto.rs` |
| `pub fn inspect_key_bytes_for_test(&self) -> &[u8` | Provides access to the key bytes ONLY during testing. | `src/crypto.rs` |
| `pub fn integrity_key(&self) -> Result<[u8` | Derives an integrity key for HMAC-SHA256. | `src/crypto.rs` |
| `pub fn try_new(passphrase: &str, salt: &[u8]) -> Result<Self>` | Creates a new KeyManager by deriving a key from a passphrase using HKDF-SHA256. # WARNHINWEIS Diese Funktion nutzt HKDF-SHA256 und setzt vo… | `src/crypto.rs` |
| `pub fn try_new_random_salt(passphrase: &str) -> Result<(Self, [u8` | Creates a new KeyManager with a cryptographically secure random salt. | `src/crypto.rs` |
| `pub fn try_new_with_kdf(passphrase: &str, header: &KdfHeader) -> Result<Self>` | Creates a new KeyManager using Argon2id password-based key derivation. Nutzt Argon2id mit den im [`KdfHeader`] definierten Kostenparametern… | `src/crypto.rs` |
| `pub fn derive_dek(unit: &UnitKey, a: &AncestorSalts, doc: DocTag) -> Dek` | Derives a DEK using AncestorSalts and DocTag. | `src/dek.rs` |
| `pub fn expose_secret(&self) -> &[u8` | — | `src/dek.rs` |
| `pub fn expose_secret(&self) -> &[u8` | — | `src/dek.rs` |
| `pub fn new(bytes: [u8; 32]) -> Self` | — | `src/dek.rs` |
| `pub fn new(bytes: [u8; 32]) -> Self` | — | `src/dek.rs` |
| `pub fn unwrap_dek(kek: &UnitKey, wrapped: &[u8]) -> Result<Dek>` | — | `src/dek.rs` |
| `pub fn wrap_dek(kek: &UnitKey, dek: &Dek, nonce: [u8; 12]) -> Result<Vec<u8>>` | — | `src/dek.rs` |
| `pub fn storage(reason: impl Into<String>) -> Self` | Helper to construct a `Storage` error. | `src/error.rs` |
| `pub fn wal_corruption(offset: u64, reason: impl Into<String>) -> Self` | Helper to construct a `WalCorruption` error. | `src/error.rs` |
| `pub fn derive_key_argon2id(passphrase: &str, header: &KdfHeader) -> Result<DerivedKey>` | Leitet einen kryptographisch starken Schlüssel mittels Argon2id aus einer Passphrase ab. # Ablauf der Argon2id-Schlüsselableitung: 1. Passp… | `src/kdf.rs` |
| `pub fn from_bytes(bytes: &[u8]) -> Result<Self>` | Parst einen `KdfHeader` aus Binärdaten. Panic-Freiheit (total function) bei beliebigen Byte-Eingaben. | `src/kdf.rs` |
| `pub fn generate_default() -> Result<Self>` | Erstellt einen neuen `KdfHeader` v1 mit zufälligem 32-Byte-Salt und OWASP-Standardparametern. | `src/kdf.rs` |
| `pub fn hkdf_expand(prk: &[u8], info: &[u8], out: &mut [u8]) -> Result<()>` | HKDF-SHA256 expand. | `src/kdf.rs` |
| `pub fn new(m_cost_kib: u32, t_cost: u32, p_cost: u32) -> Result<Self>` | Erstellt neue `KdfParams` mit Validierung der Untergrenzen. | `src/kdf.rs` |
| `pub fn new(params: KdfParams, salt: Vec<u8>) -> Result<Self>` | Erstellt einen `KdfHeader` v1 mit vorgegebenen Parametern und Salt. | `src/kdf.rs` |
| `pub fn new_for_test(m_cost_kib: u32, t_cost: u32, p_cost: u32) -> Self` | Erstellt `KdfParams` mit kleineren Kostenparametern ausschließlich für Testzwecke. | `src/kdf.rs` |
| `pub fn to_bytes(&self) -> Vec<u8>` | Serialisiert den `KdfHeader` in Binärform. Layout: `magic (4B "MFKD") | version (1B = 1) | kdf_id (1B = 1 Argon2id) | m_cost_kib (4B BE) |… | `src/kdf.rs` |
| `pub fn decrypt(&self, encrypted: &EncryptedKvLayer) -> Result<Vec<u8>>` | Decrypts an `EncryptedKvLayer` back to its original plaintext. Checks that `encrypted.format_version == CURRENT_KV_FORMAT_VERSION`. Derives… | `src/kv_cipher.rs` |
| `pub fn decrypt_with_version( &self, encrypted: &EncryptedKvLayer, _segment_id: u64, _version: u8, ) -> Result<Vec<u8>>` | Decrypts an `EncryptedKvLayer` for a given segment and version. | `src/kv_cipher.rs` |
| `pub fn encrypt( &self, tenant_id: TenantId, model_fingerprint: ModelFingerprint, plaintext: &[u8], ) -> Result<EncryptedKvLayer>` | Encrypts KV-cache plaintext payload for a given `(tenant_id, model_fingerprint)` pair. Derives an isolated sub-key via HKDF-SHA256 through… | `src/kv_cipher.rs` |
| `pub fn encrypt_with_version( &self, tenant_id: TenantId, _segment_id: u64, _version: u8, model_fingerprint: ModelFingerprint, plaintext: &[u8], ) -> Result<EncryptedKvLayer>` | Encrypts KV-cache plaintext payload for a given tenant, segment, and model fingerprint. | `src/kv_cipher.rs` |
| `pub fn new( hash: [u8; 32], model_id: impl Into<String>, quantization: impl Into<String>, ) -> Self` | Creates a new `ModelFingerprint`. | `src/kv_cipher.rs` |
| `pub fn new(key_manager: KeyManager) -> Self` | Creates a new `KvSegmentCipher` wrapping the workspace master `KeyManager`. | `src/kv_cipher.rs` |
| `pub fn emergency_wipe(store: &TenantIsolatedKvStore)` | SEPARATER Pfad für Notfall-Bereinigung (z.B. Prozess-Shutdown, expliziter Sicherheits-Trigger). Synchron, blockierend, vor Rückkehr abgesch… | `src/kv_segment/eviction_worker.rs` |
| `pub fn shutdown(&self)` | Beendet den Worker-Thread geordnet. | `src/kv_segment/eviction_worker.rs` |
| `pub fn spawn(store: Arc<TenantIsolatedKvStore>) -> Self` | Spawnt den Eviction-Worker auf einem dedizierten OS-Thread. | `src/kv_segment/eviction_worker.rs` |
| `pub fn trigger_eviction(&self, target_free_bytes: usize)` | Nicht-blockierender Trigger vom Hot-Path aus. | `src/kv_segment/eviction_worker.rs` |
| `pub fn delete_mode(&self) -> KvDeleteMode` | Returns the current delete mode. | `src/kv_segment/manager.rs` |
| `pub fn delete_segment(&self, _group_id: u64) -> Result<bool, CryptoError>` | Deletes a KV segment. | `src/kv_segment/manager.rs` |
| `pub fn new( config: KvSegmentConfig, master_key: Option<Arc<KeyManager>>, ) -> Self` | Creates a new manager with specified config and master key. | `src/kv_segment/manager.rs` |
| `pub fn read_segment(&self, segment: &KvSegmentPayload) -> Result<Vec<u8>, CryptoError>` | Reads/decrypts a KV segment payload. | `src/kv_segment/manager.rs` |
| `pub fn write_segment( &self, group_id: u64, plaintext: &[u8], ) -> Result<KvSegmentPayload, CryptoError>` | Writes a KV segment payload according to the configured mode. | `src/kv_segment/manager.rs` |
| `pub fn as_bytes(&self) -> &[u8]` | Read-Only-Zugriff. Kein Klartext-Export nach außen ohne expliziten Call. | `src/kv_segment/segment.rs` |
| `pub fn decrypt_data(&self, cipher: &KvSegmentCipher) -> Result<Vec<u8>, crate::CryptoError>` | Entschlüsselt die Daten des Segments, falls es verschlüsselt ist. Gibt bei Klartext-Segmenten direkt einen Klon der `data`-Bytes zurück. | `src/kv_segment/segment.rs` |
| `pub fn is_empty(&self) -> bool` | Prüft ob das Segment leer ist. | `src/kv_segment/segment.rs` |
| `pub fn len(&self) -> usize` | Länge der Tensor-Bytes in Bytes. | `src/kv_segment/segment.rs` |
| `pub fn new(tenant_id: TenantId, segment_id: u64, data: Vec<u8>) -> Self` | Erstellt ein neues Klartext-KV-Cache-Segment (Default / Zero-Config, P12-konform). | `src/kv_segment/segment.rs` |
| `pub fn new_encrypted( cipher: &KvSegmentCipher, tenant_id: TenantId, segment_id: u64, model_fingerprint: ModelFingerprint, rope_offset: Option<usize>, plaintext: &[u8], ) -> Result<Self, crate::CryptoError>` | Erstellt ein verschlüsseltes KV-Cache-Segment aus einem Klartext-Tensor. | `src/kv_segment/segment.rs` |
| `pub fn new_with_metadata( tenant_id: TenantId, segment_id: u64, data: Vec<u8>, #[cfg(feature = "kv-encryption")] model_fingerprint: Option<ModelFingerprint>, rope_offset: Option<usize>, ) -> Self` | Erstellt ein neues Klartext-KV-Cache-Segment mit optionalen Metadaten. | `src/kv_segment/segment.rs` |
| `pub fn to_spill_bytes(&self) -> Vec<u8>` | Serialisiert das Segment bzw. dessen verschlüsselte Layer für Tier-2-LSM-Spill. | `src/kv_segment/segment.rs` |
| `pub fn clear_all(&self)` | Leert alle Segmente aus allen Shards vollständig und löscht deren Speicher via Zeroize. | `src/kv_segment/store.rs` |
| `pub fn evict_lru_fair(&self, target_free_bytes: usize) -> usize` | Evictiert KV-Segmente unter Erhaltung von Tenant-Fairness via Round-Robin über alle aktiven Tenants. Im Gegensatz zu `evict_lru_global()` v… | `src/kv_segment/store.rs` |
| `pub fn evict_lru_fair_with_hook<F>(&self, target_free_bytes: usize, mut hook: F) -> usize where F: FnMut(),` | — | `src/kv_segment/store.rs` |
| `pub fn get_decrypted_segment( &self, cipher: &crate::KvSegmentCipher, tenant: TenantId, segment_id: u64, ) -> Result<Option<Vec<u8>>, crate::CryptoError>` | Liest ein bestimmtes Segment eines Mandanten und entschlüsselt es falls nötig. | `src/kv_segment/store.rs` |
| `pub fn get_segment_bytes(&self, tenant: TenantId, segment_id: u64) -> Option<Vec<u8>>` | Liefert unverschlüsselte Segment-Bytes für einen Tenant (Klartext-Modus). | `src/kv_segment/store.rs` |
| `pub fn get_segments(&self, tenant: TenantId) -> Vec<u64>` | INV-TENANT-Analogon für KV-Bridge: Ein Tenant kann niemals Segmente eines anderen Tenants lesen. Strukturell erzwungen durch getrennte Maps. | `src/kv_segment/store.rs` |
| `pub fn get_tenant_segment_len(&self, tenant: TenantId) -> usize` | Gibt die Anzahl der gespeicherten Segmente für einen bestimmten Tenant zurück. | `src/kv_segment/store.rs` |
| `pub fn insert_encrypted_segment( &self, cipher: &crate::KvSegmentCipher, tenant: TenantId, segment_id: u64, model_fingerprint: crate::ModelFingerprint, rope_offset: Option<usize>, plaintext: &[u8], ) -> Result<(), crate::CryptoError>` | Verschlüsselt einen Klartext-Tensor und fügt ein verschlüsseltes Segment ein. | `src/kv_segment/store.rs` |
| `pub fn insert_segment(&self, tenant: TenantId, segment: KvSegment)` | Fügt ein Segment für einen bestimmten Tenant ein. | `src/kv_segment/store.rs` |
| `pub fn new() -> Self` | Erstellt einen neuen tenant-isolierten KV-Store mit Standard-Shard-Anzahl (32). | `src/kv_segment/store.rs` |
| `pub fn on_rollback(&self, tenant: TenantId, chunk_ids: &[u64])` | Bereinigt assoziierte KV-Segmente bei einem transaktionalen Rollback. Delegiert direkt an `remove_segments_for_rollback`, um verwaiste KV-S… | `src/kv_segment/store.rs` |
| `pub fn remove_segment(&self, tenant: TenantId, segment_id: u64)` | Entfernt ein einzelnes Segment. Convenience-Wrapper um `remove_segments_for_rollback`. | `src/kv_segment/store.rs` |
| `pub fn remove_segments_for_rollback(&self, tenant: TenantId, segment_ids: &[u64])` | Entfernt alle Segmente mit den angegebenen `segment_ids` für den gegebenen Tenant. Muss bei transaktionalem Rollback aufgerufen werden, wen… | `src/kv_segment/store.rs` |
| `pub fn set_spill_handler(&self, handler: SpillHandler)` | Registriert einen Spill-Handler für evictierte Segmente. | `src/kv_segment/store.rs` |
| `pub fn with_capacity(segment_capacity_per_tenant: usize) -> Self` | Erstellt einen Store mit konfigurierter Segment-Kapazität pro Tenant. | `src/kv_segment/store.rs` |
| `pub fn with_shard_count(n: usize) -> Self` | Erstellt einen Store mit angegebener Shard-Anzahl. Die Shard-Anzahl wird automatisch auf die nächste Zweierpotenz aufgerundet. | `src/kv_segment/store.rs` |
| `pub fn as_bytes(&self) -> &[u8` | Exposes a reference to the secret bytes (compatibility alias). | `src/secret.rs` |
| `pub fn expose_secret(&self) -> &[u8` | Exposes a reference to the secret bytes. | `src/secret.rs` |
| `pub fn expose_secret(&self) -> &T` | Exposes a reference to the inner secret. | `src/secret.rs` |
| `pub fn expose_secret_mut(&mut self) -> &mut T` | Exposes a mutable reference to the inner secret. | `src/secret.rs` |
| `pub fn new(bytes: [u8; N]) -> Self` | Creates a new secret key wrapping raw bytes. | `src/secret.rs` |
| `pub fn new(inner: T) -> Self` | Creates a new `Secret` wrapping sensitive data. | `src/secret.rs` |
| `pub fn verify_wal_chain_completeness( wal_tail_hmac: &[u8; 32], manifest_high_water_mark: &[u8; 32], ) -> Result<(), CryptoError>` | Verifiziert, dass der WAL-Tail nicht abgeschnitten wurde, indem der letzte bekannte WAL-HMAC mit der im Checkpoint-Manifest persistierten H… | `src/wal_completeness.rs` |
| `pub fn decrypt_chunk(&self, data: &[u8]) -> Result<Vec<u8>>` | Decrypts the WAL chunk by extracting the prepended 12-byte nonce from the data. | `src/wal_crypto.rs` |
| `pub fn encrypt_chunk(&self, payload: &[u8]) -> Result<Vec<u8>>` | Wraps the internal WAL chunk in AES-256-GCM-SIV stream. Prepends the 12-byte nonce to the encrypted ciphertext. | `src/wal_crypto.rs` |
| `pub fn finalize(self) -> [u8` | Schließt die HMAC-Berechnung ab und liefert den 32-Byte-Digest. | `src/wal_crypto.rs` |
| `pub fn key_manager(&self) -> &KeyManager` | Accesses the underlying `KeyManager`. | `src/wal_crypto.rs` |
| `pub fn kms(&self) -> &Arc<dyn KmsProvider>` | Returns a reference to the inner KMS provider. | `src/wal_crypto.rs` |
| `pub fn last_hmac_snapshot(&self) -> [u8` | Gibt den aktuellen `last_hmac`-Kettenzustand zurück (für Verifier-Handoff). | `src/wal_crypto.rs` |
| `pub fn last_seq_no_snapshot(&self) -> Option<u64>` | Gibt die aktuell bekannte `last_seq_no` zurück (für Verifier-Handoff). | `src/wal_crypto.rs` |
| `pub fn new(key_manager: KeyManager, key_id: impl Into<String>) -> Self` | Creates a new `LocalKeyManagerKms` wrapping an existing `KeyManager`. | `src/wal_crypto.rs` |
| `pub fn new() -> Self` | Creates a new `CloudEgressGuard` with a default maximum payload limit of 10 MB. | `src/wal_crypto.rs` |
| `pub fn new(key_manager: KeyManager, file_id: &[u8]) -> Result<Self>` | Creates a new EncryptedWal with per-file key derivation via LocalKeyManagerKms to prevent nonce-reuse. `file_id` (e.g., filename) is used t… | `src/wal_crypto.rs` |
| `pub fn new(integrity_key: &[u8]) -> Result<Self>` | — | `src/wal_crypto.rs` |
| `pub fn new(integrity_key: &[u8]) -> Self` | — | `src/wal_crypto.rs` |
| `pub fn set_last_hmac(&mut self, hmac: [u8; 32])` | Übernimmt einen bereits bekannten `last_hmac`-Kettenzustand, z. B. beim Wechsel von einem Normal- auf einen Legacy-Key-Verifier MITTEN in e… | `src/wal_crypto.rs` |
| `pub fn set_last_seq_no(&mut self, seq_no: Option<u64>)` | Übernimmt eine bereits bekannte `last_seq_no` für den Verifier-Handoff. | `src/wal_crypto.rs` |
| `pub fn skip_hmac_verify_legacy(&mut self, entry: &WalEntrySnapshot)` | Updates the chain state for legacy V1 entries without HMAC verification. | `src/wal_crypto.rs` |
| `pub fn update(&mut self, data: &[u8])` | Führt neue Daten in die laufende HMAC-Berechnung ein. | `src/wal_crypto.rs` |
| `pub fn validate_egress_payload(&self, payload: &[u8]) -> Result<()>` | Validates that an egress payload meets size and structural integrity bounds. | `src/wal_crypto.rs` |
| `pub fn verify_and_update(&mut self, entry: &WalEntrySnapshot, offset: u64) -> Result<()>` | Default verification delegating to V3 verification. | `src/wal_crypto.rs` |
| `pub fn verify_and_update_v2(&mut self, entry: &WalEntrySnapshot, offset: u64) -> Result<()>` | Verifiziert und aktualisiert die HMAC-Kette im V2-Format (Rückwärtskompatibilität). | `src/wal_crypto.rs` |
| `pub fn verify_and_update_v3(&mut self, entry: &WalEntrySnapshot, offset: u64) -> Result<()>` | Verifies a V3 entry (with tx_id and length prefixes) and updates the chain state. | `src/wal_crypto.rs` |
| `pub fn with_kms(kms: Arc<dyn KmsProvider>) -> Self` | Creates a new EncryptedWal using an arbitrary KmsProvider. | `src/wal_crypto.rs` |
| `pub fn with_max_bytes(max_egress_bytes: usize) -> Self` | Creates a custom `CloudEgressGuard` with a specified maximum payload size. | `src/wal_crypto.rs` |


---

## `acta-vector`

| Attribut | Wert |
|---|---|
| **Ring** | 1 — Ebene E1 — Speicherschicht, Indizes & KI-Kerne |
| **Status** | Fertig |
| **Zweck** | HNSW- und DiskANN-Vektorindex-Implementierungen. |
| **Unsafe-Code** | Nein (`#![forbid(unsafe_code)]`) |
| **Interne Abhängigkeiten** | `acta-env`, `acta-ids`, `acta-mvcc`, `acta-ports`, `acta-simd`, `acta-sys`, `acta-types` |
| **Externe Abhängigkeiten (Auswahl)** | `ahash`, `blake3`, `memmap2`, `parking_lot`, `rand`, `roaring`, `serde`, `subtle`, `thiserror`, `tracing`, `zeroize` |

### Architekturbeschreibung

Acta Vector — HNSW vector index with SIMD distance computation.

### Modul- und Re-Export-Oberfläche (lib.rs)

```rust
pub mod casting;
pub mod compat;
pub mod contract;
pub mod error;
pub mod types;
pub mod diskann;
pub mod distance;
pub mod hnsw;
pub mod persistence;
pub mod quantize;
pub mod acorn;
pub mod compute_pool;
pub mod fixed_hasher;
pub mod partial_rebuild;
pub mod quantize_rabitq;
pub use compat::candidate_stream::{VectorCandidateStream, DEFAULT_VECTOR_STREAM_BATCH_SIZE};
pub use compute_pool::ComputePool;
pub use contract::{ BlockTransform, Clock as ContractClock, DistanceFn, Error as ContractError, ExternalId, Index as ContractIndex, Maintain as ContractMaintain, Metric as ContractMetric, Result as ContractResult, Rng as ContractRng, ScoredPoint, };
pub use diskann::{DiskAnnConfig, DiskAnnFallbackPolicy, DiskAnnIndex};
pub use hnsw::{hnsw_purger, hnsw_scanner, HnswPurger, HnswScanner};
pub use hnsw::{HnswConfig, HnswIndex, RebuildStatus};
pub use persistence::{HnswHeader, MmapIndex};
pub use quantize::ScalarQuantizer;
pub use quantize_rabitq::RaBitQQuantizer;
pub use types::{Clock, DistanceMetric, DocId, Maintain, MetricsSink, Rng, ScoredDocument, TxId};
```

### Feature-Flags

| Flag | Aktiviert |
|---|---|
| `default` | `["std"]` |
| `std` | `[]` |
| `simd` | `["dep:acta-simd", "acta-simd?/serde"]` |
| `serde` | `[]` |
| `tracing` | `[]` |
| `inspect` | `[]` |
| `experimental` | `["experimental-diskann", "experimental-rabitq", "experimental-predicate-augmented-search"]` |
| `mmap` | `["dep:acta-sys"]` |
| `diskann` | `["experimental-diskann"]` |
| `quantize-rabitq` | `["experimental-rabitq"]` |
| `ports-compat` | `["workspace"]` |
| `docid-128` | `["acta-ids/docid-128"]` |
| `experimental-diskann` | `[]` |
| `experimental-rabitq` | `[]` |
| `experimental-predicate-augmented-search` | `[]` |
| `graph` | `[]` |
| `experimental-all` | `["experimental-diskann", "experimental-rabitq", "experimental-predicate-augmented-search"]` |
| `partial-index-rebuild` | `[]` |

### Schnittstellen — Public Traits (14)

**`BlockTransform`** — *Generischer Transformations-Hook (z. B. für optionale Verschlüsselung/Transformation).* (`src/contract.rs`)

```rust
pub trait BlockTransform : Send + Sync {
/// Transformiert einen Datenblock vor der Persistierung.
    fn transform(&self, block: &mut [u8]) -> Result<()>;
    /// Rekonstruiert einen transformierten Datenblock beim Lesen.
    fn inverse_transform(&self, block: &mut [u8]) -> Result<()>;
}
```

**`Clock`** — *Trait zur Zeitabfrage.* (`src/contract.rs`)

```rust
pub trait Clock : Send + Sync {
/// Liefert die aktuelle Zeit in Unix-Millisekunden.
    fn now_unix_ms(&self) -> u64;
}
```

**`DistanceFn`** — *Distanzfunktions-Schnittstelle zur externen Injektion von Berechnungen (z.B. SIMD).* (`src/contract.rs`)

```rust
pub trait DistanceFn : Send + Sync {
/// Berechnet die Distanz zwischen zwei gleich langen Vektoren.
    fn distance(&self, a: &[f32], b: &[f32]) -> f32;
}
```

**`DistanceKernel`** — *Schnittstelle für Distanzberechnungskerne auf f32- und quantisierten Vektoren.* (`src/distance/kernel.rs`)

```rust
pub trait DistanceKernel : Send + Sync + 'static {
/// Berechnet die Distanz zwischen zwei Slices nach Endlichkeitsvalidierung.
    fn distance(&self, metric: DistanceMetric, a: &[f32], b: &[f32]) -> Result<f32>;

    /// Berechnet die Distanz zwischen einer f32-Query und einem Byte-Slice (LE f32).
    fn distance_f32_bytes(&self, metric: DistanceMetric, a: &[f32], b_bytes: &[u8]) -> Result<f32>;

    /// Berechnet die Distanz auf vorvalidierten f32-Slices.
    fn distance_trusted(&self, metric: DistanceMetric, a: &[f32], b: &[f32]) -> Result<f32>;

    /// Berechnet die Distanz zwischen vorvalidiertem f32-Slice und LE f32 Byte-Slice.
    fn distance_f32_bytes_trusted(
        &self,
        metric: DistanceMetric,
        a: &[f32],
        b_bytes: &[u8],
    ) -> Result<f32>;

    /// Kosinus-Distanz zwischen zwei f32-Slices.
    fn cosine_distance(&self, a: &[f32], b: &[f32]) -> Result<f32>;

    /// Skalarprodukt-Distanz (invertiertes Skalarprodukt) zwischen zwei f32-Slices.
    fn dot_product_distance(&self, a: &[f32], b: &[f32]) -> Result<f32>;

    /// Euklidische Distanz zwischen zwei f32-Slices.
    fn euclidean_distance(&self, a: &[f32], b: &[f32]) -> Result<f32>;

    /// Quadratische Euklidische Distanz zwischen zwei u8-Slices.
    fn euclidean_distance_sq_u8(&self, a: &[u8], b: &[u8]) -> u32;

    /// Gemischte f32/u8 quadratische Euklidische Distanz mit Rekonstruktionsparametern.
    fn euclidean_distance_sq_f32_u8(
        &self,
        a: &[f32],
        b: &[u8],
        alphas: &[f32],
        mins: &[f32],
    ) -> f32;

    /// Ganzzahliges Skalarprodukt zwischen zwei u8-Byte-Slices.
    fn dot_product_u8(&self, a: &[u8], b: &[u8]) -> u32;

    /// Gemischtes f32/u8 Skalarprodukt.
    fn dot_product_f32_u8(&self, a: &[f32], b: &[u8]) -> f32;

    /// Rohkomponenten für u8 Kosinus-Ähnlichkeit.
    fn cosine_similarity_parts_u8(&self, a: &[u8], b: &[u8]) -> CosineSimilarityPartsU8;

    /// Normalisiert einen Vektor in-place auf Einheitslänge.
    fn normalize_inplace(&self, v: &mut [f32]);
}
```

**`FilteredIndex`** — *Prädikatsagnostischer Suchpfad (ACORN-Muster, Patel et al. 2024). Im Gegensatz zu Post-Filtering wird das Prädikat WÄHREND der Graph-Traversal ausgewertet, mit erhöhter Kantenzahl (γ-Augmentierung) zur Konnektivitätssicherung bei hoher Selektivität.* (`src/acorn/mod.rs`)

```rust
pub trait FilteredIndex {
type Predicate: ?Sized;

    fn search_knn_acorn(
        &self,
        query: &[f32],
        k: usize,
        predicate: &Self::Predicate,
        gamma: u32,
    ) -> Result<Vec<(DocId, f32)>, AcornError>;
}
```

**`GhostFreeVectorIndex`** — *Trait providing ghost-free vector index deletion with synchronous neighborhood graph repair.* (`src/hnsw/deletion.rs`)

```rust
pub trait GhostFreeVectorIndex {
/// Deletes `doc_id` and synchronously repairs all neighborhood pointers across all layers
    /// before returning.
    ///
    /// # Invariant
    /// `INV-DELETION-2`: After `Ok(stats)` returns, no neighborhood pointer in the entire index
    /// references `doc_id`.
    fn remove_with_graph_repair(&mut self, doc_id: DocId) -> Result<DeletionStats>;
}
```

**`Index`** — *Standardisierter Index-Trait für Klasse A Bausteine (SPEC §4.3).* (`src/contract.rs`)

```rust
pub trait Index<Id, Query> {
/// Typ eines einzelnen Suchtreffers.
    type Hit;

    /// Fügt einen Vektordatenpunkt ein.
    fn insert(&mut self, id: Id, payload: &[u8]) -> Result<()>;

    /// Entfernt einen Vektordatenpunkt anhand seiner ID.
    fn remove(&mut self, id: &Id) -> Result<bool>;

    /// Sucht die nächsten Nachbarn für eine gegebene Abfrage.
    fn search(&self, query: &Query, limit: usize) -> Result<Vec<Self::Hit>>;
}
```

**`Index`** — *Normative Index-Schnittstelle gemäß SPEC §4.3 (Klasse A).* (`src/compat/hnsw_vector_index.rs`)

```rust
pub trait Index<Id, Query: ?Sized> {
/// Treffertyp der Suchoperation.
    type Hit;

    /// Fügt ein Dokument mit gegebener ID und Byte-Nutzlast in den Index ein.
    fn insert(&mut self, id: Id, payload: &[u8]) -> ContractResult<()>;

    /// Entfernt ein Dokument anhand seiner ID aus dem Index.
    fn remove(&mut self, id: &Id) -> ContractResult<bool>;

    /// Führt eine Suche anhand der Query aus und liefert bis zu `limit` Treffer zurück.
    fn search(&self, query: &Query, limit: usize) -> ContractResult<Vec<Self::Hit>>;
}
```

**`IntoDocId`** — *Trait for types convertible to [`DocId`] for deletion APIs.* (`src/hnsw/deletion.rs`)

```rust
pub trait IntoDocId {
/// Converts the type into a [`DocId`].
    fn into_doc_id(self) -> DocId;
}
```

**`Maintain`** — *Neutraler Wartungs-Trait für Klasse A Bausteine (SPEC §4.3).* (`src/contract.rs`)

```rust
pub trait Maintain {
/// Typ der Wartungsstatistiken.
    type Stats;

    /// Kompaktiert den Index und gibt freigegebene Ressourcen zurück.
    fn compact(&mut self) -> Result<Self::Stats>;

    /// Führt physische Bereinigungen durch.
    fn vacuum(&mut self) -> Result<()>;
}
```

**`Maintain`** — *Neutraler Wartungs-Trait für Speicherfreigaben und Kompaktierung (SPEC §6.1).* (`src/types.rs`)

```rust
pub trait Maintain : Send + Sync {
/// Führt physische Bereinigung und Slot-Freigaben durch.
    fn compact(&self) -> crate::error::Result<()>;
    /// Prüft den Rebuild- bzw. Reorganisationsbedarf.
    fn is_rebuild_required(&self) -> bool;
}
```

**`Maintain`** — *Normative Wartungsschnittstelle gemäß SPEC §4.3 (Klasse A).* (`src/compat/hnsw_vector_index.rs`)

```rust
pub trait Maintain {
/// Statistiktyp der Kompaktierungsoperation.
    type Stats;

    /// Führt eine Kompaktierung durch und gibt gelöschte Einträge tatsächlich frei.
    fn compact(&mut self) -> ContractResult<Self::Stats>;

    /// Bereinigt fragmentierte interne Strukturen.
    fn vacuum(&mut self) -> ContractResult<()>;
}
```

**`Rng`** — *Trait für deterministischen Zufall.* (`src/contract.rs`)

```rust
pub trait Rng : Send {
/// Liefert das nächste Pseudozufalls-`u64`.
    fn next_u64(&mut self) -> u64;
}
```

**`VectorIndex`** — ** (`src/compat/candidate_stream.rs`)

```rust
pub trait VectorIndex : Send + Sync {

}
```

### Öffentliche Datentypen — Structs (47)

| Struct | Felder / Form | Beschreibung | Datei |
|---|---|---|---|
| `AdaptiveEfPolicy` | `{ /// Initial search depth ef for layer 0. pub min_ef: usize, /// Maximum search depth ef limit. pub max_ef: usize, /// Growth factor for increasing ef in each e… }` | Configuration policy for adaptive ef_search (Ada-ef). | `src/hnsw/adaptive_ef.rs` |
| `AdaptiveEfStateMachine` | `{ policy: AdaptiveEfPolicy, k: usize, current_ef: usize, previous_top_k: Vec<DocId>, stable_rounds: usize, round_count: usize, is_terminated: bool, converged: bo… }` | Deterministic state machine managing Ada-ef expansion rounds. | `src/hnsw/adaptive_ef.rs` |
| `AdaptiveEfStats` | `{ /// Final ef depth reached at termination. pub final_ef: usize, /// Total expansion rounds executed. pub rounds: usize, /// Whether search terminated due to ca… }` | Execution statistics for an adaptive search run. | `src/hnsw/adaptive_ef.rs` |
| `BacklinkTable` | `{ pub map: BTreeMap<BacklinkKey, Vec<u32>>, }` | Deterministic backlink connection table using `BTreeMap`. | `src/hnsw/arena.rs` |
| `BackoffTracker` | `{ base_delay: Duration, max_delay: Duration, alert_threshold: u64, last_rebuild_nanos: Option<u64>, consecutive_rapid_rebuilds: u64, }` | Autarker Backoff-Zustandsautomat zur Ratenbegrenzung von Rebuild-Vorgängen. | `src/hnsw/core_rebuild/backoff.rs` |
| `BatchContext` | `{ pub running_max_layer: usize, pub has_entry_point: bool, pub has_ram_entry_point: bool, pub backlink_map: BacklinkTable, }` | Batch tracking context for running state across multi-operation transaction commits. | `src/hnsw/batch.rs` |
| `Candidate` | `{ pub index: usize, pub distance: f32, }` | Search candidate. | `src/hnsw/types/data.rs` |
| `ComputePool` | `{ inner: Arc<PoolState>, }` | Begrenzter Compute-Threadpool für Hintergrundberechnungen und rechenintensive Indizierung. Erzwingt eine strikte Obergrenze für Worker-Threads und ei… | `src/compute_pool.rs` |
| `CosineSimilarityPartsU8` | `{ /// Akkumuliertes Skalarprodukt $\sum a_i b_i$. pub dot: u32, /// Akkumulierte quadratische L2-Norm $\sum a_i^2$. pub norm_a_sq: u32, /// Akkumulierte quadrati… }` | Rohkomponenten zur Berechnung der ganzzahligen Kosinus-Ähnlichkeit auf u8-Vektoren. | `src/distance/kernel.rs` |
| `DeletionStats` | `{ /// Document ID that was deleted. pub doc_id: DocId, /// Number of neighborhood edges repaired (backlinks removed/replaced). pub repaired_edges: usize, /// Num… }` | Statistics returned after a ghost-free node deletion and neighborhood repair. | `src/hnsw/deletion.rs` |
| `DiskAnnConfig` | `{ /// Path to the on-disk index file. pub index_path: PathBuf, /// Vector dimension. pub dimension: usize, /// Maximum graph degree (R). pub max_degree: usize, /… }` | Configuration for DiskANN index. | `src/diskann/config.rs` |
| `DiskAnnHmac` | `{ hasher: blake3::Hasher, }` | Standalone keyed BLAKE3 hasher for DiskANN file integrity. | `src/diskann/integrity.rs` |
| `DiskAnnIndex` | `{ pub(crate) inner: Arc<DiskAnnIndexInner>, }` | DiskANN out-of-core vector index. This index uses `memmap2` to offload vector data and graph edges to disk. Note that mmap operations and page faults… | `src/diskann/types.rs` |
| `ExternalId` | `(tuple/unit)` | Externe Identifikationsnummer (128-Bit für breite Kompatibilität). | `src/contract.rs` |
| `GenericHnswIndex` | `{ pub(crate) inner: std::sync::Arc<HnswIndexCore>, pub(crate) _kernel: std::marker::PhantomData<K>, }` | Generischer HNSW (Hierarchical Navigable Small World) Vektorindex parametrisiert ueber DistanceKernel. | `src/hnsw/types/index.rs` |
| `HnswArena` | `{ pub arena: RwLock<Vec<u32>>, pub offsets: RwLock<Vec<usize>>, pub capacities: RwLock<Vec<usize>>, pub count_offsets: RwLock<Vec<usize>>, pub counts: RwLock<Vec… }` | Contiguous 64-byte aligned arena allocator for HNSW node connections with free-list slot reuse. | `src/hnsw/arena.rs` |
| `HnswColdCore` | `{ pub config: HnswConfig, pub validation_error: Option<String>, pub tx_buffer: crate::hnsw::staging::VectorTxBuffer, pub quantizer: RwLock<Option<crate::quantize… }` | Kalter Kern des HNSW-Index (Konfiguration, Puffer, Metriken, Quantisierer). | `src/hnsw/types/core.rs` |
| `HnswCompactStats` | `{ /// Anzahl physisch bereinigter Tombstones. pub purged_docs: usize, /// Anzahl verbliebener aktiver Dokumente. pub active_docs: usize, }` | Statistiken über den Kompaktierungsvorgang des HNSW-Vektorindex. | `src/compat/hnsw_vector_index.rs` |
| `HnswConfig` | `{ /// Vector dimensionality. pub dimension: usize, /// Maximum number of elements. pub max_elements: usize, /// Number of connections per element (M parameter).… }` | Konfiguration für den HNSW-Graph-Index. | `src/hnsw/config.rs` |
| `HnswConfigBuilder` | `{ config: HnswConfig, }` | Builder for HnswConfig with resource limit enforcements to prevent OOM. | `src/hnsw/config.rs` |
| `HnswHeader` | `{ magic: u32, version: u16, dimension: u32, m: u32, metric: u8, quantized: u8, q_min: f32, // Legacy v1 field q_max: f32, // Legacy v1 field node_count: u64, ent… }` | The header of an HNSW persistent file. | `src/persistence/header.rs` |
| `HnswHotCore` | `{ pub nodes: RwLock<Vec<HnswNode>>, #[cfg(not(feature = "docid-128"))] pub doc_to_node: RwLock<BTreeMap<u64, usize>>, #[cfg(feature = "docid-128")] pub doc_to_no… }` | Heisser Kern des HNSW-Index (fuer Lese- und Schreibpfade im Graph optimiert). | `src/hnsw/types/core.rs` |
| `HnswIndexCore` | `{ pub hot: HnswHotCore, pub cold: HnswColdCore, }` | Die Kern-Implementierung des HNSW-Index aus Hot- und Cold-Strukturen. | `src/hnsw/types/core.rs` |
| `HnswNode` | `{ pub(crate) doc_id: DocId, pub(crate) vector: VectorData, pub(crate) max_layer: usize, pub(crate) committed_tx: u64, }` | A node in the HNSW graph. | `src/hnsw/types/data.rs` |
| `HnswPurger` | `{ index: Arc<HnswIndex>, }` | — | `src/compat/purge.rs` |
| `HnswScanner` | `{ index: Arc<HnswIndex>, }` | — | `src/compat/scanner.rs` |
| `InjectedDistanceKernel` | `{ pub distance_fn: D, }` | Adapter, der einen [`DistanceFn`]-Typ in das [`DistanceKernel`]-Interface einbettet. | `src/distance/inject.rs` |
| `MmapIndex` | `{ pub mmap: std::sync::Arc<memmap2::Mmap>, pub header: HnswHeader, pub path: Option<std::path::PathBuf>, }` | A reader for memory-mapped HNSW indices. | `src/persistence/mmap.rs` |
| `MonotonicCounterClock` | `{ nanos: AtomicU64, }` | Deterministic monotonic counter clock for HNSW vector index. Implements [`Clock`] without any reference to system wall-clock or non-deterministic OS… | `src/hnsw/clock.rs` |
| `NaiveReferenceIndex` | `{ vectors: Vec<(DocId, Vec<f32>)>, metric: DistanceMetric, }` | Naive reference index performing brute-force search over all stored points. **NICHT für Produktion — O(n) Referenzimplementierung** Serves exclusivel… | `src/acorn/naive_reference.rs` |
| `NeighborBacklink` | `{ pub neighbor_ram_idx: usize, pub layer: usize, pub updated_connections: Vec<u32>, }` | Back-link connection update for a neighbor node at a specific layer. | `src/hnsw/batch.rs` |
| `NodeRecord` | `{ #[cfg(not(feature = "docid-128"))] pub doc_id: u64, #[cfg(feature = "docid-128")] pub doc_id: u128, pub max_layer: u8, pub vector_offset: u64, pub connections_… }` | Represents a node's metadata in the flat file. | `src/persistence/node.rs` |
| `PartialRebuildConfig` | `{ /// Kritische Übersättigungs-Schwelle θ_c. Default: 3.0 (3× globale Dichte). pub critical_ratio: f32, /// Ringpuffer-Größe N für Traversal-Statistik. Default:… }` | Konfiguration für den Partial-Rebuild-Trigger. | `src/partial_rebuild.rs` |
| `PreparedInsert` | `{ pub doc_id: DocId, pub vector_data: VectorData, pub new_layer: usize, pub new_idx: usize, pub final_connections: Vec<Vec<u32>>, pub neighbor_backlinks: Vec<Nei… }` | — | `src/hnsw/batch.rs` |
| `RaBitQQuantizer` | `{ dimension: usize, /// Row-major orthogonal rotation matrix R of size dimension x dimension. rotation_matrix: Vec<f32>, }` | RaBitQ Quantizer with random orthogonal rotation and scalar metadata. | `src/quantize_rabitq.rs` |
| `RebuildGuard` | `(tuple/unit)` | Guard, der das `rebuilding`-Flag auf `false` zuruecksetzt und wartende Threads weckt. | `src/hnsw/types/guards.rs` |
| `ResidualReport` | `{ /// Ob die ID noch in der ID-zu-Knoten-Zuordnung auffindbar ist. pub in_id_map: bool, /// Ob im zugehörigen Slot noch Vektordaten ungleich Null liegen. pub non… }` | Struktur für die lesende Inspektionssicht (`inspect`-Sicht). Ermittelt, ob Spuren einer Dokument-ID bzw. eines Vektors noch in ID-Mappings, Slots (Ni… | `src/compat/scanner.rs` |
| `ScalarDistance` | `{ pub metric: ContractMetric, }` | Schnittstellenadapter für [`DistanceFn`] über [`ScalarKernel`]. | `src/distance/scalar.rs` |
| `ScalarKernel` | `(tuple/unit)` | Vollständige skalare Referenz-Implementierung mit sicheren Iteratoren. | `src/distance/scalar.rs` |
| `ScoredDocument` | `{ /// Dokument-ID des Treffers. pub doc_id: DocId, /// Ähnlichkeitswert (Score, höher ist ähnlicher). pub score: f32, /// Distanzwert (niedriger ist näher). pub… }` | Suchergebnis mit Dokument-ID und Relevanz-Score / Distanz. | `src/types.rs` |
| `ScoredPoint` | `{ /// Externe Identifikationsnummer des Punkts. pub id: ExternalId, /// Ähnlichkeitswert (höher = besser, total geordnet). pub score: f32, }` | Trefferpunkt mit Distanz-/Ähnlichkeitsbewertung. | `src/contract.rs` |
| `SnapshotPinGuard` | `(tuple/unit)` | Guard, der einen Snapshot im `SequenceLog` pinnt und beim Drop wieder freigibt. | `src/hnsw/types/guards.rs` |
| `Sq8Bias` | `{ /// Mean quantization error across distance computations. pub mean_bias: f32, /// Variance of quantization error across distance computations. pub variance_bia… }` | Measured quantization bias and variance statistics for SQ8 distance calculations. | `src/hnsw/sq8_bias.rs` |
| `TaskHandle` | `{ rx: Receiver<VectorResult<T>>, }` | Handle zum Abwarten des Ergebnisses eines an den [`ComputePool`] übergebenen Tasks. | `src/compute_pool.rs` |
| `TraversalTracker` | `{ config: PartialRebuildConfig, visited_ring: VecDeque<Vec<u64>>, // Ein Vec<u64> pro Traversierungsaufruf }` | Ringpuffer für besuchte HNSW-Knoten-IDs während ef_search-Traversierungen. | `src/partial_rebuild.rs` |
| `VectorCandidateStream` | `{ index: &'a V, query: Vec<f32>, seq_no: Option<u64>, batch_size: usize, buffer: VecDeque<ScoredDocument>, seen: BTreeSet<DocId>, fetched_k: usize, yielded: usiz… }` | On-demand loading async pull-stream for vector search candidates. # Candidate Ordering & Determinism Vector search scores represent relevance (higher… | `src/compat/candidate_stream.rs` |
| `VectorTxBuffer` | `{ buffers: RwLock<BTreeMap<u64, Vec<VectorStagingOp>>>, capacity: usize, #[allow(dead_code)] timeout: Duration, }` | Transaction staging buffer for vector operations. Encapsulates transactional staging locally, isolating from direct coupling to storage transaction b… | `src/hnsw/staging.rs` |

### Öffentliche Datentypen — Enums (9)

| Enum | Varianten | Beschreibung | Datei |
|---|---|---|---|
| `AcornError` | `/// Dimension mismatch between query and vector database points. #[error("Invalid query dimension: expected {expected}, got {got}")] DimensionMismatch { /// Expected dimension. expected: usize, /// P…` | Errors occurring during ACORN search or graph traversal operations. | `src/acorn/mod.rs` |
| `DiskAnnFallbackPolicy` | `/// Transparently fall back to in-memory HNSW index on load failure or detected corruption. #[default] UseHnswOnFailure, /// Immediately return a typed error on load failure or index corruption. Fail…` | Fallback behavior policy when DiskANN loading, integrity checks, or reads fail. | `src/diskann/config.rs` |
| `DistanceMetric` | `/// Kosinus-Distanz ($1 - \text{sim}$). #[default] Cosine, /// Euklidische Distanz ($L_2$). Euclidean, /// Skalarprodukt (Dot-Produkt, invertiert für Minimierung). DotProduct,` | — | `src/types.rs` |
| `Error` | `/// Ungültige Eingabedaten. #[error("ungültige Eingabedaten: {0}")] InvalidInput(&'static str), /// Kapazitätsgrenze überschritten. #[error("Kapazitätsgrenze überschritten: gefordert {requested}, max…` | Lokales Fehler-Enum für Vertragsoperationen. | `src/contract.rs` |
| `Metric` | `/// Kosinus-Distanz. Cosine, /// Euklidische Distanz. Euclidean, /// Skalarprodukt. DotProduct,` | Unterstützte Distanzmetriken. | `src/contract.rs` |
| `RebuildStatus` | `/// Kein Rebuild laeuft oder geplant. Idle, /// Rebuild laeuft gerade im Hintergrund. Running, /// Rebuild wurde getriggert, startet bald. Pending,` | Status eines Index-Rebuilds. | `src/hnsw/types/core.rs` |
| `VectorData` | `/// Standard 32-bit floating point vectors. F32(Vec<f32>), /// 8-bit quantized vectors (SQ8). U8(Vec<u8>),` | — | `src/hnsw/types/data.rs` |
| `VectorError` | `/// Ungültige Eingabe oder Konfigurationsparameter. #[error("Invalid input: {0}")] InvalidInput(String), /// Strukturelle Beschädigung oder Inkonsistenz des Index. #[error("Index error: {0}")] Index(…` | Fehlerzustände von `acta-vector`. | `src/error.rs` |
| `VectorStagingOp` | `/// Insertion or update of a vector for a document. Insert { doc_id: DocId, embedding: Vec<f32> }, /// Deletion of a document vector. Delete { doc_id: DocId },` | Operation within a transactional staging buffer. | `src/hnsw/staging.rs` |

### Typ-Aliase (6)

| Alias | Definition | Beschreibung |
|---|---|---|
| `BacklinkKey` | `type BacklinkKey= (usize, usize)` | Composite key (neighbor_ram_idx, layer) for backlink lookups. |
| `FixedHashMap` | `type FixedHashMap<K, V> = HashMap<K, V, RandomState>` | Deterministische `HashMap` mit fixem Hasher. |
| `FixedHashSet` | `type FixedHashSet<T> = HashSet<T, RandomState>` | Deterministisches `HashSet` mit fixem Hasher. |
| `HnswIndex` | `type HnswIndex= GenericHnswIndex<ScalarKernel>` | Der Standard-HNSW-Vektorindex mit dem skalaren Referenz-Distanzkernel. |
| `Result` | `type Result<T> = core::result::Result<T, Error>` | Standard-Result-Typ für Vertragsoperationen. |
| `Result` | `type Result<T> = std::result::Result<T, VectorError>` | Standardmäßiger Result-Typ für `acta-vector`. |

### Konstanten (13)

| Konstante | Typ | Beschreibung |
|---|---|---|
| `ARENA_ALIGNMENT_BYTES` | `usize` | Slot alignment in bytes (64-byte alignment = 16 u32 elements for SIMD cache-line compatibility). |
| `ARENA_ALIGNMENT_U32` | `usize` | Alignment in u32 elements (64 bytes / 4 bytes = 16 u32 elements). |
| `DEFAULT_LAYER_SEED` | `u64` | Default-Seed fuer die deterministische HNSW-Ebenenzuweisung (wie in hnswlib ueblich). |
| `DEFAULT_QUEUE_CAPACITY_PER_WORKER` | `usize` | Standard-Kapazität für die synchrone Job-Warteschlange pro Worker-Thread. |
| `DEFAULT_VECTOR_STREAM_BATCH_SIZE` | `usize` | Default batch size for vector candidate retrieval. |
| `HNSW_MAGIC` | `u32` | Magic number for HNSW files (0x484E5357 = "HNSW"). |
| `HNSW_REBUILD_DELETION_RATIO` | `f64` | Standard-Loeschanteil (0.10 = 10 % geloeschte Knoten), ab dem ein Rebuild getriggert wird. |
| `HNSW_VERSION` | `u16` | Current file format version. |
| `MAX_SEARCH_K` | `usize` | Maximal zulässiges `k` für Nachbarschaftssuchen. |
| `SENTINEL_NO_ENTRY_POINT` | `u32` | Sentinel-Wert fuer ungueltigen / nicht gesetzten Entry-Point in HNSW. |
| `SIZE` | `usize` | — |
| `SIZE` | `usize` | — |
| `SIZE` | `usize` | — |

### Öffentliche Funktionen (293)

| Signatur | Beschreibung | Datei |
|---|---|---|
| `pub fn compute_gamma_edge_budget(base_degree: usize, predicate_selectivity: f32) -> usize` | Computes the augmented edge budget factor $\gamma$ for graph expansion in ACORN search. Based on Patel et al. 2024 (ACORN: Performant and P… | `src/acorn/gamma_augmentation.rs` |
| `pub fn from_vectors(vectors: Vec<(DocId, Vec<f32>)>) -> Self` | Creates a `NaiveReferenceIndex` pre-populated with vectors using Euclidean distance. | `src/acorn/naive_reference.rs` |
| `pub fn insert(&mut self, id: DocId, vector: Vec<f32>)` | Inserts a document vector into the index. | `src/acorn/naive_reference.rs` |
| `pub fn new(metric: DistanceMetric) -> Self` | Creates a new empty `NaiveReferenceIndex` with the specified distance metric. | `src/acorn/naive_reference.rs` |
| `pub fn f32_to_usize(val: f32) -> Result<usize>` | Konvertiert `f32` defensiv zu `usize` mit Validierung gegen Nicht-Endlichkeit und negative Werte. | `src/casting.rs` |
| `pub fn f64_to_usize(val: f64) -> Result<usize>` | Konvertiert `f64` defensiv zu `usize` mit Validierung gegen Nicht-Endlichkeit und negative Werte. | `src/casting.rs` |
| `pub fn to_u32(val: usize) -> Result<u32>` | Konvertiert `usize` zu `u32`, liefert `VectorError::InvalidInput` bei Überlauf. | `src/casting.rs` |
| `pub fn to_u64(val: usize) -> u64` | Konvertiert `usize` zu `u64`. Unfehlbar auf 32-Bit und 64-Bit Systemen. | `src/casting.rs` |
| `pub fn to_u8(val: usize) -> Result<u8>` | Konvertiert `usize` zu `u8`, liefert `VectorError::InvalidInput` bei Überschreitung von 255. | `src/casting.rs` |
| `pub fn u32_to_usize(val: u32) -> usize` | Konvertiert `u32` zu `usize`. Unfehlbar auf 32-Bit und 64-Bit Zielarchitekturen. | `src/casting.rs` |
| `pub fn u64_to_u32(val: u64) -> Result<u32>` | Konvertiert `u64` zu `u32`, liefert `VectorError::InvalidInput` bei Überschreitung von `u32::MAX`. | `src/casting.rs` |
| `pub fn u64_to_usize(val: u64) -> Result<usize>` | Konvertiert `u64` zu `usize`, liefert `VectorError::InvalidInput` bei Überlauf auf 32-Bit. | `src/casting.rs` |
| `pub fn fetched_depth(&self) -> usize` | Returns the last $k$ search depth requested from the vector index. | `src/compat/candidate_stream.rs` |
| `pub fn is_exhausted(&self) -> bool` | Returns whether the candidate stream is exhausted. | `src/compat/candidate_stream.rs` |
| `pub fn new(index: &'a V, query: impl Into<Vec<f32>>, seq_no: Option<u64>) -> Self` | — | `src/compat/candidate_stream.rs` |
| `pub async fn next_batch(&mut self) -> Result<Vec<ScoredDocument>>` | — | `src/compat/candidate_stream.rs` |
| `pub async fn next_doc(&mut self) -> Result<Option<DocId>>` | — | `src/compat/candidate_stream.rs` |
| `pub fn with_batch_size(mut self, n: usize) -> Self` | Sets a custom batch size (minimum 1). | `src/compat/candidate_stream.rs` |
| `pub fn yielded(&self) -> usize` | Returns the total number of items yielded so far. | `src/compat/candidate_stream.rs` |
| `pub fn hnsw_purger(index: Arc<HnswIndex>) -> Arc<dyn Purger>` | — | `src/compat/purge.rs` |
| `pub fn new(index: Arc<HnswIndex>) -> Self` | Erzeugt einen neuen `HnswPurger`. | `src/compat/purge.rs` |
| `pub fn has_residual(&self) -> bool` | Gibt an, ob noch irgendwelche Restspuren (ID, Vektorbytes, Kanten, Staging) existieren. | `src/compat/scanner.rs` |
| `pub fn hnsw_scanner(index: Arc<HnswIndex>) -> Arc<dyn LayerScanner>` | — | `src/compat/scanner.rs` |
| `pub fn inspect_residual(&self, doc_id: DocId) -> ResidualReport` | Lesende Inspektionssicht: Prüft, ob `doc_id` noch in Slots, Kanten oder Puffern vorkommt. | `src/compat/scanner.rs` |
| `pub fn new(index: Arc<HnswIndex>) -> Self` | Erzeugt einen neuen `HnswScanner`. | `src/compat/scanner.rs` |
| `pub fn scan_units(&self, session: &mut ScanSession, _ctx: &RequestContext) -> Result<()>` | Führt den Schicht-Scan für den HNSW-Index aus. | `src/compat/scanner.rs` |
| `pub fn active_jobs(&self) -> usize` | Gibt die Anzahl aktuell auf Worker-Threads aktiver Jobs zurück. | `src/compute_pool.rs` |
| `pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static,` | Reicht eine Fire-and-Forget-Closure ein. Blockiert bei voller Warteschlange (Backpressure). | `src/compute_pool.rs` |
| `pub fn join(self) -> VectorResult<T>` | Wartet auf den Abschluss des Tasks. Panics im Task werden als [`VectorError`] propagiert. | `src/compute_pool.rs` |
| `pub fn max_workers(&self) -> usize` | Gibt die maximale Worker-Kapazität des Pools zurück. | `src/compute_pool.rs` |
| `pub fn new(max_workers: usize) -> Self` | — | `src/compute_pool.rs` |
| `pub fn spawn<F, T>(&self, f: F) -> TaskHandle<T> where F: FnOnce() -> T + Send + 'static, T: Send + 'static,` | Reicht eine Funktion ein und liefert ein [`TaskHandle`], um das Ergebnis abzuwarten. | `src/compute_pool.rs` |
| `pub fn with_queue_capacity(max_workers: usize, queue_capacity: usize) -> Self` | Erzeugt einen neuen [`ComputePool`] mit konfigurierbarer Pufferkapazität (begrenzter Kanal). | `src/compute_pool.rs` |
| `pub async fn build(&self, vectors: &[Vec<f32>], ids: &[DocId]) -> Result<()>` | Builds the index from a set of vectors. | `src/diskann/build.rs` |
| `pub fn build_sync(&self, vectors: &[Vec<f32>], ids: &[DocId]) -> Result<()>` | — | `src/diskann/build.rs` |
| `pub fn build_to_path_sync( &self, target_path: &std::path::Path, vectors: &[Vec<f32>], ids: &[DocId], ) -> Result<()>` | — | `src/diskann/build.rs` |
| `pub fn recover_pending_delta( &self, ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<usize>> + Send + '_>>` | Liest uncommittete Pending-Vektoren aus `pending.wal` und fügt sie in den Index ein. | `src/diskann/build.rs` |
| `pub fn recover_pending_delta_sync(&self) -> Result<usize>` | — | `src/diskann/build.rs` |
| `pub async fn build(&self, vectors: &[Vec<f32>], ids: &[DocId]) -> Result<()>` | Builds the index from a set of vectors. | `src/diskann/build/mod.rs` |
| `pub fn build_sync(&self, vectors: &[Vec<f32>], ids: &[DocId]) -> Result<()>` | — | `src/diskann/build/mod.rs` |
| `pub fn build_to_path_sync( &self, target_path: &std::path::Path, vectors: &[Vec<f32>], ids: &[DocId], ) -> Result<()>` | — | `src/diskann/build/mod.rs` |
| `pub fn recover_pending_delta( &self, ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<usize>> + Send + '_>>` | Liest uncommittete Pending-Vektoren aus `pending.wal` und fügt sie in den Index ein. | `src/diskann/build/pending_wal.rs` |
| `pub fn recover_pending_delta_sync(&self) -> Result<usize>` | — | `src/diskann/build/pending_wal.rs` |
| `pub fn finalize(self) -> [u8` | Finalizes the calculation and yields the 32-byte digest. | `src/diskann/integrity.rs` |
| `pub fn new(integrity_key: &[u8]) -> Result<Self>` | Creates a new `DiskAnnHmac` using the provided integrity key. | `src/diskann/integrity.rs` |
| `pub fn update(&mut self, data: &[u8])` | Feeds new data into the integrity calculation. | `src/diskann/integrity.rs` |
| `pub async fn load(&self) -> Result<()>` | Loads the index from the configured path. | `src/diskann/persistence.rs` |
| `pub fn load_sync(&self) -> Result<()>` | — | `src/diskann/persistence.rs` |
| `pub async fn persist_delta(&self) -> Result<()>` | — | `src/diskann/persistence.rs` |
| `pub fn persist_delta_sync(&self) -> Result<()>` | — | `src/diskann/persistence.rs` |
| `pub fn trigger_background_persist_delta(&self)` | — | `src/diskann/persistence.rs` |
| `pub async fn persist_delta(&self) -> Result<()>` | — | `src/diskann/persistence/delta.rs` |
| `pub fn persist_delta_sync(&self) -> Result<()>` | — | `src/diskann/persistence/delta.rs` |
| `pub fn trigger_background_persist_delta(&self)` | — | `src/diskann/persistence/delta.rs` |
| `pub async fn load(&self) -> Result<()>` | — | `src/diskann/persistence/loader.rs` |
| `pub fn load_sync(&self) -> Result<()>` | — | `src/diskann/persistence/loader.rs` |
| `pub async fn search_predicate_augmented( &self, query: &[f32], k: usize, filter: &(dyn Fn(DocId) -> bool + Send + Sync), gamma: usize, ) -> Result<Vec<ScoredDocument>>` | Executes a predicate-augmented vector search using ACORN 2-hop neighbor expansion during graph traversal. # Parameters - `query`: Query vec… | `src/diskann/predicate_augmented.rs` |
| `pub async fn search_internal(&self, query: &[f32], k: usize) -> Result<Vec<ScoredDocument>>` | — | `src/diskann/search.rs` |
| `pub fn compute_distance<M: Into<DistanceMetric>>(a: &[f32], b: &[f32], metric: M) -> Result<f32>` | Berechnet die Distanz zwischen zwei Vektoren nach Endlichkeitsvalidierung. | `src/distance/dispatch.rs` |
| `pub fn compute_distance_f32_bytes_trusted<M: Into<DistanceMetric>>( a: &[f32], b_bytes: &[u8], metric: M, ) -> Result<f32>` | Berechnet die Distanz zwischen vorvalidiertem f32-Slice und Little-Endian f32 Byte-Slice. | `src/distance/dispatch.rs` |
| `pub fn compute_distance_trusted<M: Into<DistanceMetric>>( a: &[f32], b: &[f32], metric: M, ) -> Result<f32>` | Berechnet die Distanz zwischen zwei vorvalidierten f32-Slices. | `src/distance/dispatch.rs` |
| `pub fn cosine_distance(a: &[f32], b: &[f32]) -> Result<f32>` | Bequemlichkeitsfunktion für Kosinus-Distanz mit automatischer Validierung. | `src/distance/dispatch.rs` |
| `pub fn dot_product_distance(a: &[f32], b: &[f32]) -> Result<f32>` | Bequemlichkeitsfunktion für Skalarprodukt-Distanz mit automatischer Validierung. | `src/distance/dispatch.rs` |
| `pub fn euclidean_distance(a: &[f32], b: &[f32]) -> Result<f32>` | Bequemlichkeitsfunktion für Euklidische Distanz mit automatischer Validierung. | `src/distance/dispatch.rs` |
| `pub fn new(distance_fn: D) -> Self` | Erzeugt einen neuen injizierten Kernel aus einer [`DistanceFn`]. | `src/distance/inject.rs` |
| `pub fn compute(&self, a: &[f32], b: &[f32]) -> core::result::Result<f32, ContractError>` | Berechnet die Distanz zwischen zwei Vektoren und liefert bei ungültigen Eingaben Fehler. | `src/distance/scalar.rs` |
| `pub fn cosine_distance(&self, a: &[f32], b: &[f32]) -> Result<f32>` | Kosinus-Distanz zwischen zwei f32-Slices. | `src/distance/scalar.rs` |
| `pub fn cosine_distance_scalar(a: &[f32], b: &[f32]) -> f32` | Skalare Kosinus-Distanz als Referenz. | `src/distance/scalar.rs` |
| `pub fn cosine_similarity_parts_u8(&self, a: &[u8], b: &[u8]) -> CosineSimilarityPartsU8` | Rohkomponenten für u8 Kosinus-Ähnlichkeit. | `src/distance/scalar.rs` |
| `pub fn distance<M: Into<DistanceMetric>>( &self, metric: M, a: &[f32], b: &[f32], ) -> Result<f32>` | Inhärente Distanzberechnung mit Unterstützung für alle Typen, die `Into<DistanceMetric>` implementieren. | `src/distance/scalar.rs` |
| `pub fn distance_f32_bytes<M: Into<DistanceMetric>>( &self, metric: M, a: &[f32], b_bytes: &[u8], ) -> Result<f32>` | Inhärente Distanzberechnung auf Byte-Slices mit flexibler Metrik. | `src/distance/scalar.rs` |
| `pub fn distance_f32_bytes_trusted<M: Into<DistanceMetric>>( &self, metric: M, a: &[f32], b_bytes: &[u8], ) -> Result<f32>` | Inhärente Distanzberechnung auf vorvalidierten Byte-Slices. | `src/distance/scalar.rs` |
| `pub fn distance_trusted<M: Into<DistanceMetric>>( &self, metric: M, a: &[f32], b: &[f32], ) -> Result<f32>` | Inhärente Distanzberechnung auf vorvalidierten Slices. | `src/distance/scalar.rs` |
| `pub fn dot_product_distance(&self, a: &[f32], b: &[f32]) -> Result<f32>` | Dot-Produkt-Distanz (invertiertes Skalarprodukt) zwischen zwei f32-Slices. | `src/distance/scalar.rs` |
| `pub fn dot_product_f32_u8(&self, a: &[f32], b: &[u8]) -> f32` | Gemischtes f32/u8 Skalarprodukt. | `src/distance/scalar.rs` |
| `pub fn dot_product_scalar(a: &[f32], b: &[f32]) -> f32` | Skalares Dot-Produkt (invertiert) als Referenz. | `src/distance/scalar.rs` |
| `pub fn dot_product_u8(&self, a: &[u8], b: &[u8]) -> u32` | Ganzzahliges Skalarprodukt zwischen zwei u8-Byte-Slices. | `src/distance/scalar.rs` |
| `pub fn euclidean_distance(&self, a: &[f32], b: &[f32]) -> Result<f32>` | Euklidische Distanz zwischen zwei f32-Slices. | `src/distance/scalar.rs` |
| `pub fn euclidean_distance_scalar(a: &[f32], b: &[f32]) -> f32` | Skalare Euklidische Distanz als Referenz. | `src/distance/scalar.rs` |
| `pub fn euclidean_distance_sq_f32_u8( &self, a: &[f32], b: &[u8], alphas: &[f32], mins: &[f32], ) -> f32` | Gemischte f32/u8 quadratische Euklidische Distanz mit Rekonstruktionsparametern. | `src/distance/scalar.rs` |
| `pub fn euclidean_distance_sq_u8(&self, a: &[u8], b: &[u8]) -> u32` | Quadratische Euklidische Distanz zwischen zwei u8-Slices. | `src/distance/scalar.rs` |
| `pub fn normalize_inplace(&self, v: &mut [f32])` | Normalisiert einen Vektor in-place auf Einheitslänge. | `src/distance/scalar.rs` |
| `pub fn cosine_similarity_parts_u8(a: &[u8], b: &[u8]) -> CosineSimilarityPartsU8` | Berechnet die ganzzahligen Summen-Komponenten für Kosinus-Ähnlichkeit auf u8-Vektoren. | `src/distance/u8_ops.rs` |
| `pub fn dot_product_f32_u8(a: &[f32], b: &[u8]) -> f32` | Gemischtes Skalarprodukt zwischen f32-Query und quantisiertem u8-Vektor. | `src/distance/u8_ops.rs` |
| `pub fn dot_product_u8(a: &[u8], b: &[u8]) -> u32` | Ganzzahliges Skalarprodukt zwischen zwei u8-Vektoren. | `src/distance/u8_ops.rs` |
| `pub fn euclidean_distance_sq_f32_u8(a: &[f32], b: &[u8], alphas: &[f32], mins: &[f32]) -> f32` | Gemischte quadratische Euklidische Distanz zwischen f32-Query und quantisiertem u8-Vektor. | `src/distance/u8_ops.rs` |
| `pub fn euclidean_distance_sq_u8(a: &[u8], b: &[u8]) -> u32` | Quadratische Euklidische Distanz zwischen zwei u8-Vektoren. | `src/distance/u8_ops.rs` |
| `pub fn normalize_inplace(v: &mut [f32])` | In-place Normalisierung eines Vektors auf Einheitslänge. | `src/distance/u8_ops.rs` |
| `pub fn validate_vector(vec: &[f32]) -> Result<()>` | Prüft, ob ein Vektor frei von NaN oder Unendlichkeitswerten ist (INV-SIMD-FINITE-1). | `src/distance/validate.rs` |
| `pub fn invalid_input(msg: impl Into<String>) -> Self` | Hilfsmethode zur Konstruktion von `InvalidInput`. | `src/error.rs` |
| `pub fn fixed_hasher() -> RandomState` | Liefert einen deterministischen `RandomState` Build-Hasher mit fixierten Seeds. | `src/fixed_hasher.rs` |
| `pub fn new_fixed_hashmap<K, V>(capacity: usize) -> FixedHashMap<K, V>` | Erzeugt eine neue deterministische `FixedHashMap` mit vorgegebener Kapazität. | `src/fixed_hasher.rs` |
| `pub fn new_fixed_hashset<T>(capacity: usize) -> FixedHashSet<T>` | Erzeugt ein neues deterministisches `FixedHashSet` mit vorgegebener Kapazität. | `src/fixed_hasher.rs` |
| `pub fn current_ef(&self) -> usize` | Returns the active `ef_search` for the current expansion round. | `src/hnsw/adaptive_ef.rs` |
| `pub fn is_terminated(&self) -> bool` | Returns whether the state machine has reached a termination condition. | `src/hnsw/adaptive_ef.rs` |
| `pub fn new( min_ef: usize, max_ef: usize, growth_factor: f32, convergence_window: usize, stability_threshold: f32, ) -> Result<Self>` | Validating constructor for `AdaptiveEfPolicy`. | `src/hnsw/adaptive_ef.rs` |
| `pub fn new(policy: AdaptiveEfPolicy, k: usize) -> Self` | Initializes a new state machine for adaptive search with target top-k. | `src/hnsw/adaptive_ef.rs` |
| `pub fn round_count(&self) -> usize` | Returns the number of expansion rounds completed. | `src/hnsw/adaptive_ef.rs` |
| `pub fn stats(&self) -> AdaptiveEfStats` | Returns the current execution statistics. | `src/hnsw/adaptive_ef.rs` |
| `pub fn step(&mut self, current_top_k: &[DocId]) -> bool` | Evaluates current top-k candidate set, updating state and determining if search should terminate. Returns `true` if search should terminate… | `src/hnsw/adaptive_ef.rs` |
| `pub fn validate(&self) -> Result<()>` | Validates policy invariants. | `src/hnsw/adaptive_ef.rs` |
| `pub fn align_capacity(raw_capacity: usize) -> usize` | Rounds up an allocation size in u32s to the nearest 64-byte (16 x u32) boundary. | `src/hnsw/arena.rs` |
| `pub fn allocate_node( &self, max_layer: usize, m: usize, final_connections: &[Vec<u32>], ) -> Result<usize>` | — | `src/hnsw/arena.rs` |
| `pub fn clear(&mut self)` | Clears all stored backlinks. | `src/hnsw/arena.rs` |
| `pub fn free_node(&self, ram_idx: usize)` | Marks a node's slot as free for future slot reuse. | `src/hnsw/arena.rs` |
| `pub fn get(&self, neighbor_ram_idx: usize, layer: usize) -> Option<&Vec<u32>>` | Gets connection list for (neighbor_ram_idx, layer) in O(1) time. | `src/hnsw/arena.rs` |
| `pub fn get_ram_node_connections(&self, ram_idx: usize, layer: usize, m: usize) -> Vec<u32>` | — | `src/hnsw/arena.rs` |
| `pub fn insert(&mut self, neighbor_ram_idx: usize, layer: usize, connections: Vec<u32>)` | Inserts or updates connection list for (neighbor_ram_idx, layer) in O(1) time. | `src/hnsw/arena.rs` |
| `pub fn layer_offset(node_offset: usize, layer: usize, m: usize) -> usize` | Calculates layer offset within a node's allocated arena block. | `src/hnsw/arena.rs` |
| `pub fn new() -> Self` | Creates a new `BacklinkTable`. | `src/hnsw/arena.rs` |
| `pub fn new() -> Self` | Creates a new `HnswArena`. | `src/hnsw/arena.rs` |
| `pub fn try_relink_pruned_neighbors( &self, ram_idx: usize, layer: usize, m: usize, kept_neighbors: &[u32], ) -> bool` | — | `src/hnsw/arena.rs` |
| `pub fn update_backlink( &self, ram_idx: usize, layer: usize, m: usize, updated_connections: &[u32], ) -> Result<()>` | — | `src/hnsw/arena.rs` |
| `pub fn zeroize_and_free_node(&self, ram_idx: usize, max_layer: usize, m: usize)` | Zeroizes connection memory for a node slot across all layers, clears counts, and then marks the node's slot as free in the free_list. | `src/hnsw/arena.rs` |
| `pub fn zeroize_node_connections(&self, ram_idx: usize, max_layer: usize, m: usize)` | — | `src/hnsw/arena.rs` |
| `pub fn new(core: &HnswIndexCore) -> Self` | — | `src/hnsw/batch.rs` |
| `pub fn advance_nanos(&self, delta_nanos: u64)` | Advances the clock by `delta_nanos`. | `src/hnsw/clock.rs` |
| `pub fn new(initial_nanos: u64) -> Self` | Creates a new [`MonotonicCounterClock`] starting at the given nanosecond timestamp. | `src/hnsw/clock.rs` |
| `pub fn backoff_alert_threshold(mut self, threshold: u32) -> Self` | Sets the threshold of consecutive rapid rebuilds that trigger warning alerts. | `src/hnsw/config.rs` |
| `pub fn backoff_base_delay(mut self, delay: std::time::Duration) -> Self` | Sets the base delay for rebuild exponential backoff cooldown. | `src/hnsw/config.rs` |
| `pub fn backoff_max_delay(mut self, delay: std::time::Duration) -> Self` | Sets the maximum cap for rebuild exponential backoff cooldown. | `src/hnsw/config.rs` |
| `pub fn build(self) -> Result<HnswConfig>` | — | `src/hnsw/config.rs` |
| `pub fn distance_metric(mut self, metric: DistanceMetric) -> Self` | Use a specific distance metric. | `src/hnsw/config.rs` |
| `pub fn ef_construction(mut self, ef: usize) -> Self` | Set dynamic candidate list size for construction. | `src/hnsw/config.rs` |
| `pub fn ef_search(mut self, ef: usize) -> Self` | Set dynamic candidate list size for search. | `src/hnsw/config.rs` |
| `pub fn m(mut self, m: usize) -> Self` | Set the number of connections per element (M). | `src/hnsw/config.rs` |
| `pub fn max_elements(mut self, max: usize) -> Self` | Set max elements with a hardcap limit to avoid OOM. | `src/hnsw/config.rs` |
| `pub fn new(dimension: usize) -> Self` | Creates a new builder with the chosen dimensionality. | `src/hnsw/config.rs` |
| `pub fn partial_rebuild_config( mut self, config: crate::partial_rebuild::PartialRebuildConfig, ) -> Self` | Sets the partial rebuild configuration for local hot-path rebuilds (F-02). | `src/hnsw/config.rs` |
| `pub fn quantize(mut self, quantize: bool) -> Self` | Enable or disable scalar quantization (SQ8) to reduce footprint. | `src/hnsw/config.rs` |
| `pub fn quantizer_drift_threshold(mut self, threshold: f32) -> Self` | Sets the drift ratio threshold for ScalarQuantizer recalibration and rebuilds. | `src/hnsw/config.rs` |
| `pub fn quantizer_recalibration_sample_size(mut self, size: usize) -> Self` | Sets the sample size used for ScalarQuantizer recalibration during rebuilds. | `src/hnsw/config.rs` |
| `pub fn rebuild_threshold(mut self, threshold: f64) -> Self` | Sets the rebuild threshold. | `src/hnsw/config.rs` |
| `pub fn validate(&self) -> Result<()>` | — | `src/hnsw/config.rs` |
| `pub fn apply_insert(&self, prepared: PreparedInsert, tx_id: u64)` | — | `src/hnsw/core_insert/apply.rs` |
| `pub fn compute_insert(&self, id: DocId, vector: &[f32]) -> Result<PreparedInsert>` | — | `src/hnsw/core_insert/compute.rs` |
| `pub fn compute_insert_with_context( &self, id: DocId, vector: &[f32], offset: usize, prior_prepared: &mut [PreparedInsert], batch_ctx: &mut BatchContext, ) -> Result<PreparedInsert>` | — | `src/hnsw/core_insert/compute.rs` |
| `pub fn check_connectivity(&self) -> crate::error::Result<()>` | — | `src/hnsw/core_insert/connectivity.rs` |
| `pub fn connectivity_score(&self) -> f64` | — | `src/hnsw/core_insert/connectivity.rs` |
| `pub fn check_and_update(&mut self, now_nanos: u64) -> Result<()>` | Prüft, ob ein Rebuild zum Zeitpunkt `now_nanos` erlaubt ist, und aktualisiert den internen Zustand. | `src/hnsw/core_rebuild/backoff.rs` |
| `pub fn consecutive_rapid_rebuilds(&self) -> u64` | Liefert die Anzahl der aufeinanderfolgenden schnellen Rebuild-Versuche. | `src/hnsw/core_rebuild/backoff.rs` |
| `pub fn current_cooldown(&self) -> Duration` | Berechnet die aktuelle Cooldown-Dauer anhand der Anzahl aufeinanderfolgender Rebuilds. | `src/hnsw/core_rebuild/backoff.rs` |
| `pub fn last_rebuild_nanos(&self) -> Option<u64>` | Liefert den Zeitstempel des letzten erfolgreichen Rebuilds. | `src/hnsw/core_rebuild/backoff.rs` |
| `pub fn new(base_delay: Duration, max_delay: Duration, alert_threshold: u64) -> Self` | Erstellt einen neuen BackoffTracker mit den konfigurierten Parametern. | `src/hnsw/core_rebuild/backoff.rs` |
| `pub fn record_success(&mut self, now_nanos: u64)` | Markiert einen erfolgreichen Rebuild zum Zeitpunkt `now_nanos` und setzt den Zähler zurück. | `src/hnsw/core_rebuild/backoff.rs` |
| `pub fn create_backoff_tracker(&self) -> super::backoff::BackoffTracker` | Liefert den aktuellen Backoff-Tracker Zustand für Diagnose- und Testzwecke. | `src/hnsw/core_rebuild/full.rs` |
| `pub fn deleted_ratio(&self) -> f64` | — | `src/hnsw/core_rebuild/full.rs` |
| `pub fn is_rebuild_required(&self) -> bool` | — | `src/hnsw/core_rebuild/full.rs` |
| `pub fn rebuild(&self) -> Result<()>` | — | `src/hnsw/core_rebuild/full.rs` |
| `pub fn rebuild_count(&self) -> u64` | — | `src/hnsw/core_rebuild/full.rs` |
| `pub fn rebuild_status(&self) -> RebuildStatus` | — | `src/hnsw/core_rebuild/full.rs` |
| `pub fn rebuild_sync(&self) -> Result<()>` | — | `src/hnsw/core_rebuild/full.rs` |
| `pub fn rebuild_sync_with_clock(&self, clock: &dyn crate::types::Clock) -> Result<()>` | — | `src/hnsw/core_rebuild/full.rs` |
| `pub fn rebuild_sync_with_clock(&self, clock: &dyn crate::types::Clock) -> Result<()>` | Führt den synchronen Rebuild mit der übergebenen Uhr-Instanz aus. | `src/hnsw/core_rebuild/full.rs` |
| `pub fn rebuild_with_clock(&self, clock: &dyn crate::types::Clock) -> Result<()>` | — | `src/hnsw/core_rebuild/full.rs` |
| `pub fn rebuild_with_clock(&self, clock: &dyn crate::types::Clock) -> Result<()>` | Führt den Rebuild mit der übergebenen Uhr-Instanz aus. | `src/hnsw/core_rebuild/full.rs` |
| `pub fn verify_full_connectivity(&self) -> bool` | Führt Graphtraversierung durch zur Überprüfung von `INV-VEC-CONNECTIVITY-1`. | `src/hnsw/core_rebuild/full.rs` |
| `pub fn visited_dead_nodes(&self) -> u64` | — | `src/hnsw/core_rebuild/full.rs` |
| `pub fn wait_for_rebuild(&self) -> bool` | — | `src/hnsw/core_rebuild/full.rs` |
| `pub fn wait_for_rebuild_blocking( &self, timeout: std::time::Duration, _clock: &dyn crate::types::Clock, ) -> bool` | — | `src/hnsw/core_rebuild/full.rs` |
| `pub fn wait_for_rebuild_with_clock(&self, clock: &dyn crate::types::Clock) -> bool` | — | `src/hnsw/core_rebuild/full.rs` |
| `pub fn wait_for_rebuild_with_timeout(&self, timeout: std::time::Duration) -> bool` | — | `src/hnsw/core_rebuild/full.rs` |
| `pub fn wait_for_rebuild_with_timeout_and_clock( &self, timeout: std::time::Duration, clock: &dyn crate::types::Clock, ) -> bool` | — | `src/hnsw/core_rebuild/full.rs` |
| `pub fn wait_for_rebuild_with_timeout_and_clock( &self, timeout: std::time::Duration, clock: &dyn crate::types::Clock, ) -> bool` | Wartet auf den Abschluss eines laufenden Rebuilds mit Timeout und Uhr. | `src/hnsw/core_rebuild/full.rs` |
| `pub fn rebuild_region(&self, region_node_ids: Vec<u64>) -> Result<()>` | Startet einen partiellen Rebuild für eine bestimmte Menge von Knoten. | `src/hnsw/core_rebuild/region.rs` |
| `pub fn rebuild_region_sync(&self, region_node_ids: Vec<u64>) -> Result<()>` | Führt die lokale Reorganisation einer Region synchron durch. | `src/hnsw/core_rebuild/region.rs` |
| `pub async fn search_adaptive( &self, query: &[f32], k: usize, policy: &AdaptiveEfPolicy, ) -> Result<Vec<ScoredDocument>>` | Führt eine adaptive Suche (Ada-ef) auf dem HNSW-Index aus. | `src/hnsw/core_search/adaptive.rs` |
| `pub async fn search_adaptive_filtered( &self, query: &[f32], k: usize, policy: &AdaptiveEfPolicy, filter: Option<&(dyn Fn(DocId) -> bool + Send + Sync)>, ) -> Result<Vec<ScoredDocument>>` | Führt eine adaptive Suche mit Prädikatsfilter aus. | `src/hnsw/core_search/adaptive.rs` |
| `pub async fn search_adaptive_with_stats( &self, query: &[f32], k: usize, policy: &AdaptiveEfPolicy, filter: Option<&(dyn Fn(DocId) -> bool + Send + Sync)>, ) -> Result<(Vec<ScoredDocument>, AdaptiveEfStats)>` | — | `src/hnsw/core_search/adaptive.rs` |
| `pub fn remove<D: IntoDocId>(&self, doc_id: D) -> Result<bool>` | — | `src/hnsw/deletion.rs` |
| `pub fn remove_with_repair<D: IntoDocId>(&mut self, id: D) -> Result<bool>` | Removes a document from the vector index by its 64-bit ID. Executes tombstone insertion, immediate slot zeroization (`INV-VEC-ZEROIZE-1`),… | `src/hnsw/deletion.rs` |
| `pub fn calibrate( batch: &[&[f32]], quantizer: &crate::quantize::ScalarQuantizer, metric: crate::types::DistanceMetric, ) -> Self` | Calibrates quantization error statistics across a representative batch of vectors. | `src/hnsw/sq8_bias.rs` |
| `pub fn new(mean_bias: f32, variance_bias: f32, sample_count: u64) -> Self` | Creates a new `Sq8Bias` instance with calibrated parameters. | `src/hnsw/sq8_bias.rs` |
| `pub fn discard(&self, tx: TxId)` | Discards staged operations on transaction rollback. | `src/hnsw/staging.rs` |
| `pub fn drain(&self, tx: TxId) -> Vec<VectorStagingOp>` | Drains and returns all staged operations for a given transaction. | `src/hnsw/staging.rs` |
| `pub fn new(capacity: usize, timeout: Duration) -> Self` | Creates a new vector staging buffer with the specified concurrency ceiling and timeout. | `src/hnsw/staging.rs` |
| `pub fn stage(&self, tx: TxId, op: VectorStagingOp) -> crate::error::Result<()>` | Stages an operation into the transaction buffer. | `src/hnsw/staging.rs` |
| `pub fn new(config: HnswConfig) -> Self` | — | `src/hnsw/types/builder.rs` |
| `pub fn try_new(config: HnswConfig) -> Result<Self>` | Creates a new HNSW index, validating configuration upfront. Uses an internal, non-cryptographic, reproducible default RNG seeded with `DEFA… | `src/hnsw/types/builder.rs` |
| `pub fn try_new_with_kernel( config: HnswConfig, _kernel: std::sync::Arc<dyn DistanceKernel>, ) -> Result<Self>` | Creates a new HNSW index with a custom distance kernel. | `src/hnsw/types/builder.rs` |
| `pub fn try_new_with_rng( config: HnswConfig, rng: std::sync::Arc<dyn crate::types::Rng>, ) -> Result<Self>` | Creates a new HNSW index with an explicit random number generator for deterministic testing or custom seeding. Uses [`MonotonicCounterClock… | `src/hnsw/types/builder.rs` |
| `pub fn try_new_with_rng_and_clock( config: HnswConfig, rng: std::sync::Arc<dyn crate::types::Rng>, clock: std::sync::Arc<dyn crate::types::Clock>, ) -> Result<Self>` | — | `src/hnsw/types/builder.rs` |
| `pub fn clock(&self) -> std::sync::Arc<dyn crate::types::Clock>` | Returns the clock instance used by this index. | `src/hnsw/types/core.rs` |
| `pub fn get_entry_point(&self) -> Option<usize>` | — | `src/hnsw/types/core.rs` |
| `pub fn get_ram_entry_point(&self) -> Option<usize>` | — | `src/hnsw/types/core.rs` |
| `pub fn get_ram_node_connections(&self, ram_idx: usize, layer: usize, m: usize) -> Vec<u32>` | — | `src/hnsw/types/core.rs` |
| `pub fn metrics_sink(&self) -> std::sync::Arc<dyn crate::types::MetricsSink>` | Returns the metrics sink. | `src/hnsw/types/core.rs` |
| `pub fn set_clock(&self, clock: std::sync::Arc<dyn crate::types::Clock>)` | Sets the clock instance. | `src/hnsw/types/core.rs` |
| `pub fn set_entry_point(&self, ep: Option<usize>)` | — | `src/hnsw/types/core.rs` |
| `pub fn set_layer_seed(&self, seed: u64)` | Sets the random seed used for HNSW layer selection during insertions. | `src/hnsw/types/core.rs` |
| `pub fn set_metrics_sink(&self, sink: std::sync::Arc<dyn crate::types::MetricsSink>)` | Sets the metrics sink. | `src/hnsw/types/core.rs` |
| `pub fn set_ram_entry_point(&self, ep: Option<usize>)` | — | `src/hnsw/types/core.rs` |
| `pub fn all_doc_ids_and_layers(&self) -> Vec<(DocId, usize)>` | Returns all non-deleted document IDs and their assigned HNSW max layers. | `src/hnsw/types/index.rs` |
| `pub fn all_doc_ids_from_map(&self) -> Vec<DocId>` | — | `src/hnsw/types/index.rs` |
| `pub fn check_and_trigger_partial_rebuild(&self)` | — | `src/hnsw/types/index.rs` |
| `pub fn check_connectivity(&self) -> crate::error::Result<()>` | — | `src/hnsw/types/index.rs` |
| `pub fn clock(&self) -> std::sync::Arc<dyn crate::types::Clock>` | Returns the clock instance used by this index. | `src/hnsw/types/index.rs` |
| `pub fn connectivity_score(&self) -> f64` | — | `src/hnsw/types/index.rs` |
| `pub fn deleted_ratio(&self) -> f64` | — | `src/hnsw/types/index.rs` |
| `pub fn entry_point(&self) -> Option<usize>` | Gibt den aktuellen globalen Entry-Point-Knotenindex zurueck (falls vorhanden). | `src/hnsw/types/index.rs` |
| `pub fn is_rebuild_required(&self) -> bool` | — | `src/hnsw/types/index.rs` |
| `pub fn layer0_connections(&self, node_idx: usize) -> Vec<u32>` | Gibt die Layer-0-Verbindungen fuer einen Knotenindex zurueck. | `src/hnsw/types/index.rs` |
| `pub fn metrics_sink(&self) -> std::sync::Arc<dyn crate::types::MetricsSink>` | Returns the metrics sink. | `src/hnsw/types/index.rs` |
| `pub fn node_idx_for_doc(&self, doc_id: DocId) -> Option<usize>` | Gibt den internen Knotenindex fuer eine `DocId` zurueck (falls vorhanden und nicht geloescht). | `src/hnsw/types/index.rs` |
| `pub fn quantizer(&self) -> Option<crate::quantize::ScalarQuantizer>` | Returns a snapshot clone of the current quantizer if trained. | `src/hnsw/types/index.rs` |
| `pub fn rebuild(&self) -> Result<()>` | — | `src/hnsw/types/index.rs` |
| `pub fn rebuild_count(&self) -> u64` | — | `src/hnsw/types/index.rs` |
| `pub fn rebuild_region(&self, region_node_ids: Vec<u64>) -> Result<()>` | — | `src/hnsw/types/index.rs` |
| `pub fn rebuild_region_sync(&self, region_node_ids: Vec<u64>) -> Result<()>` | — | `src/hnsw/types/index.rs` |
| `pub fn rebuild_status(&self) -> RebuildStatus` | — | `src/hnsw/types/index.rs` |
| `pub fn rebuild_sync(&self) -> Result<()>` | — | `src/hnsw/types/index.rs` |
| `pub fn set_clock(&self, clock: std::sync::Arc<dyn crate::types::Clock>)` | Sets the clock instance. | `src/hnsw/types/index.rs` |
| `pub fn set_fault_injection_insert_target(&self, target: u64)` | Sets the target insertion count for simulating compute_insert fault injection in tests. | `src/hnsw/types/index.rs` |
| `pub fn set_layer_seed(&self, seed: u64)` | Sets the random seed used for HNSW layer selection during insertions. | `src/hnsw/types/index.rs` |
| `pub fn set_metrics_sink(&self, sink: std::sync::Arc<dyn crate::types::MetricsSink>)` | Sets the metrics sink. | `src/hnsw/types/index.rs` |
| `pub fn sq8_bias(&self) -> Sq8Bias` | Returns the calibrated SQ8 quantization bias statistics. | `src/hnsw/types/index.rs` |
| `pub fn trigger_rebuild_async(&self)` | — | `src/hnsw/types/index.rs` |
| `pub fn visited_dead_nodes(&self) -> u64` | — | `src/hnsw/types/index.rs` |
| `pub fn wait_for_rebuild(&self) -> bool` | — | `src/hnsw/types/index.rs` |
| `pub fn wait_for_rebuild_blocking( &self, timeout: std::time::Duration, clock: &dyn crate::types::Clock, ) -> bool` | — | `src/hnsw/types/index.rs` |
| `pub fn wait_for_rebuild_with_timeout(&self, timeout: std::time::Duration) -> bool` | — | `src/hnsw/types/index.rs` |
| `pub fn with_clock(self, clock: std::sync::Arc<dyn crate::types::Clock>) -> Self` | Builder method to set the clock port. | `src/hnsw/types/index.rs` |
| `pub fn with_metrics_sink(self, sink: std::sync::Arc<dyn crate::types::MetricsSink>) -> Self` | Builder method to set the metrics sink. | `src/hnsw/types/index.rs` |
| `pub fn load_mmap(&self, path: impl AsRef<std::path::Path>) -> Result<()>` | — | `src/hnsw/types/persist.rs` |
| `pub fn load_mmap_sync(&self, path: impl AsRef<std::path::Path>) -> Result<()>` | — | `src/hnsw/types/persist.rs` |
| `pub fn save(&self, path: impl AsRef<std::path::Path>) -> Result<()>` | — | `src/hnsw/types/persist.rs` |
| `pub fn save_sync(&self, path: impl AsRef<std::path::Path>) -> Result<()>` | — | `src/hnsw/types/persist.rs` |
| `pub fn compact_seq_log(&self, min_active_seqno: u64)` | — | `src/hnsw/types/seqlog.rs` |
| `pub fn pin_snapshot(&self, seq_no: u64)` | — | `src/hnsw/types/seqlog.rs` |
| `pub fn unpin_snapshot(&self, seq_no: u64)` | — | `src/hnsw/types/seqlog.rs` |
| `pub fn find_oversaturated_regions( &self, tombstone_map: &BTreeMap<u64, bool>, // node_id → is_tombstoned ) -> Vec<u64>` | — | `src/partial_rebuild.rs` |
| `pub fn hot_path_nodes(&self) -> impl Iterator<Item = u64> + '_` | Gibt die N zuletzt besuchten eindeutigen Knoten-IDs zurück. | `src/partial_rebuild.rs` |
| `pub fn new(config: PartialRebuildConfig) -> Self` | Erstellt einen neuen TraversalTracker mit der gegebenen Konfiguration. | `src/partial_rebuild.rs` |
| `pub fn record_traversal(&mut self, visited_node_ids: Vec<u64>)` | — | `src/partial_rebuild.rs` |
| `pub fn should_trigger_partial_rebuild( tracker: &TraversalTracker, tombstone_map: &BTreeMap<u64, bool>, global_tombstone_ratio: f32, config: &PartialRebuildConfig, ) -> Option<Vec<u64>>` | — | `src/partial_rebuild.rs` |
| `pub fn write_atomic(destination: &Path, data: &[u8]) -> Result<()>` | Writes data to `destination` atomically via a temporary file in the same parent directory, ensuring both file data and parent directory ent… | `src/persistence/atomic.rs` |
| `pub fn checksum(&self) -> [u8` | — | `src/persistence/header.rs` |
| `pub fn connections_offset(&self) -> u64` | — | `src/persistence/header.rs` |
| `pub fn dimension(&self) -> u32` | — | `src/persistence/header.rs` |
| `pub fn entry_point(&self) -> i64` | — | `src/persistence/header.rs` |
| `pub fn is_quantized(&self) -> bool` | — | `src/persistence/header.rs` |
| `pub fn last_tx_id(&self) -> u64` | — | `src/persistence/header.rs` |
| `pub fn m(&self) -> u32` | — | `src/persistence/header.rs` |
| `pub fn magic(&self) -> u32` | — | `src/persistence/header.rs` |
| `pub fn metric(&self) -> u8` | — | `src/persistence/header.rs` |
| `pub fn new( dimension: u32, m: u32, metric: u8, quantized: u8, q_min: f32, q_max: f32, node_count: u64, entry_point: i64, nodes_offset: u64, connections_offset: u64, last_tx_id: u64, ) -> Self` | — | `src/persistence/header.rs` |
| `pub fn new_v2( dimension: u32, m: u32, metric: u8, quantized: u8, q_min: f32, q_max: f32, node_count: u64, entry_point: i64, nodes_offset: u64, connections_offset: u64, last_tx_id: u64, quant_calibration_offset: u64, quant_calibration_len: u32, ) -> Self` | — | `src/persistence/header.rs` |
| `pub fn new_v2_with_bias( dimension: u32, m: u32, metric: u8, quantized: u8, q_min: f32, q_max: f32, node_count: u64, entry_point: i64, nodes_offset: u64, connections_offset: u64, last_tx_id: u64, quant_calibration_offset: u64, quant_calibration_len: u32, sq8_…` | — | `src/persistence/header.rs` |
| `pub fn node_count(&self) -> u64` | — | `src/persistence/header.rs` |
| `pub fn nodes_offset(&self) -> u64` | — | `src/persistence/header.rs` |
| `pub fn q_max(&self) -> f32` | — | `src/persistence/header.rs` |
| `pub fn q_min(&self) -> f32` | — | `src/persistence/header.rs` |
| `pub fn q_range(&self) -> (f32, f32)` | — | `src/persistence/header.rs` |
| `pub fn quant_calibration_len(&self) -> u32` | — | `src/persistence/header.rs` |
| `pub fn quant_calibration_offset(&self) -> u64` | — | `src/persistence/header.rs` |
| `pub fn quantized(&self) -> u8` | — | `src/persistence/header.rs` |
| `pub fn set_connections_offset(&mut self, offset: u64)` | — | `src/persistence/header.rs` |
| `pub fn sq8_bias(&self) -> Sq8Bias` | — | `src/persistence/header.rs` |
| `pub fn sq8_bias_mean(&self) -> f32` | — | `src/persistence/header.rs` |
| `pub fn sq8_bias_variance(&self) -> f32` | — | `src/persistence/header.rs` |
| `pub fn to_bytes(&self) -> [u8` | — | `src/persistence/header.rs` |
| `pub fn try_from_bytes(bytes: &[u8]) -> Result<Self>` | — | `src/persistence/header.rs` |
| `pub fn verify_checksum(&self, expected: &[u8; 32]) -> Result<()>` | Verifies that the serialized bytes match the expected BLAKE3 checksum. | `src/persistence/header.rs` |
| `pub fn version(&self) -> u16` | — | `src/persistence/header.rs` |
| `pub fn get_connections(&self, record: &NodeRecord, layer: usize) -> Result<Vec<u32>>` | — | `src/persistence/mmap.rs` |
| `pub fn get_node_record(&self, index: usize) -> Result<NodeRecord>` | — | `src/persistence/mmap.rs` |
| `pub fn get_vector(&self, record: &NodeRecord) -> Result<&[u8]>` | — | `src/persistence/mmap.rs` |
| `pub fn open(path: impl AsRef<std::path::Path>) -> Result<Self>` | — | `src/persistence/mmap.rs` |
| `pub async fn open_async(path: impl AsRef<std::path::Path> + Send) -> Result<Self>` | Opens an HNSW file without requiring external runtime blocking task spawns. | `src/persistence/mmap.rs` |
| `pub fn connections_offset(&self) -> u64` | — | `src/persistence/node.rs` |
| `pub fn from_bytes(bytes: &[u8]) -> Result<Self>` | — | `src/persistence/node.rs` |
| `pub fn to_bytes(&self) -> Vec<u8>` | — | `src/persistence/node.rs` |
| `pub fn vector_offset(&self) -> u64` | — | `src/persistence/node.rs` |
| `pub fn dequantize(&self, vector: &[u8]) -> Result<Vec<f32>>` | — | `src/quantize/decode.rs` |
| `pub fn asymmetric_dist( &self, query: &[f32], quantized: &[u8], metric: DistanceMetric, ) -> Result<f32>` | — | `src/quantize/distance.rs` |
| `pub fn symmetric_dist(&self, q1: &[u8], q2: &[u8], metric: DistanceMetric) -> Result<f32>` | — | `src/quantize/distance.rs` |
| `pub fn quantize(&self, vector: &[f32]) -> Result<Vec<u8>>` | — | `src/quantize/encode.rs` |
| `pub fn calculate_vector_drift(&self, vector: &[f32]) -> f32` | — | `src/quantize/train.rs` |
| `pub fn check_drift(&self, vector: &[f32]) -> f32` | Prüft den Drift und aktualisiert interne Metriken. | `src/quantize/train.rs` |
| `pub fn drift_ratio(&self) -> f32` | Gibt das Verhältnis der Anfragen außerhalb des trainierten Bereichs zurück. | `src/quantize/train.rs` |
| `pub fn is_rebuild_required(&self, threshold: f32) -> bool` | Prüft, ob eine Neukalibrierung empfohlen wird. | `src/quantize/train.rs` |
| `pub fn train(batch: &[&[f32]], dimension: usize) -> Self` | Trainiert den Quantizer mit Standard-Parametern. | `src/quantize/train.rs` |
| `pub fn train_with_percentiles( batch: &[&[f32]], dimension: usize, p_low: f32, p_high: f32, ) -> Self` | Panik-freie Trainingsmethode für Tests und Legacy-Pfade. | `src/quantize/train.rs` |
| `pub fn try_train(batch: &[&[f32]], dimension: usize) -> Result<Self>` | Versucht das Training eines `ScalarQuantizer` mit Standard-Perzentilen. | `src/quantize/train.rs` |
| `pub fn try_train_with_percentiles( batch: &[&[f32]], dimension: usize, p_low: f32, p_high: f32, ) -> Result<Self>` | — | `src/quantize/train.rs` |
| `pub fn asymmetric_distance(&self, query: &[f32], code: &[u8]) -> Result<f32>` | Computes the asymmetric Euclidean distance between an unquantized query vector and a quantized bitcode. # Mathematical Derivation & Bias Co… | `src/quantize_rabitq.rs` |
| `pub fn code_length(&self) -> usize` | Returns the expected byte length of quantized codes (`dimension.div_ceil(8) + 8` bytes). | `src/quantize_rabitq.rs` |
| `pub fn dimension(&self) -> usize` | Returns the vector dimension supported by this quantizer. | `src/quantize_rabitq.rs` |
| `pub fn quantize(&self, vector: &[f32]) -> Result<Vec<u8>>` | Quantizes a single $f32$ vector into a RaBitQ packed bitcode buffer. The returned code consists of: - $\lceil D / 8 \rceil$ bytes of packed… | `src/quantize_rabitq.rs` |
| `pub fn try_train(vectors: &[Vec<f32>], dimension: usize) -> Result<Self>` | Safely trains a `RaBitQQuantizer` for vectors of the specified `dimension` with a default seed. | `src/quantize_rabitq.rs` |
| `pub fn try_train_with_seed(vectors: &[Vec<f32>], dimension: usize, seed: u64) -> Result<Self>` | Safely trains a `RaBitQQuantizer` for vectors of the specified `dimension` with an explicit injected `seed`. Generates a deterministic orth… | `src/quantize_rabitq.rs` |
| `pub fn from_score(doc_id: DocId, score: f32) -> Self` | Erzeugt ein ScoredDocument mit abgeleiteter Distanz (`1.0 - score`). | `src/types.rs` |
| `pub fn new(doc_id: DocId, score: f32, distance: f32) -> Self` | Erzeugt ein neues ScoredDocument mit explizitem Score und Distanzwert. | `src/types.rs` |


---

## `acta-text`

| Attribut | Wert |
|---|---|
| **Ring** | 1 — Ebene E1 — Speicherschicht, Indizes & KI-Kerne |
| **Status** | Fertig |
| **Zweck** | BM25/BM25F-Volltextindexierung und deutsche Kompositazerlegung. |
| **Unsafe-Code** | Nein (`#![forbid(unsafe_code)]`) |
| **Interne Abhängigkeiten** | `acta-env`, `acta-ids`, `acta-ports`, `acta-types` |
| **Externe Abhängigkeiten (Auswahl)** | `bincode`, `blake3`, `bytes`, `futures`, `itoa`, `parking_lot`, `regex`, `serde`, `serde_json`, `thiserror`, `tracing`, `unicode-segmentation` |

### Modul- und Re-Export-Oberfläche (lib.rs)

```rust
pub mod compat;
pub mod contract;
pub mod error;
pub mod memory_index;
pub mod storage;
pub mod traits;
pub mod types;
pub mod bm25;
pub mod domain;
pub mod inverted;
pub mod morphology;
pub mod posting_list;
pub mod stream;
pub mod tokenizer;
pub mod wand;
pub use contract::{Clock, DocKey, Epoch, Error as ContractError, Result as ContractResult, Rng};
pub use error::{Result as LocalResult, TextError};
pub use memory_index::{CompactStats, MemoryTextIndex};
pub use storage::PostingStore;
pub use traits::{Index, Maintain};
pub use types::{DocId as LocalDocId, TextHit, MAX_SEARCH_K as LOCAL_MAX_SEARCH_K};
pub use bm25::{score_term_bm25f, FieldId, FieldWeight, BM25, BM25F};
pub use domain::{DomainVocabulary, LegalDomainVocabulary, MedicalDomainVocabulary};
pub use inverted::{InvertedIndex, Language};
pub use inverted::{ text_purger, text_scanner, BM25MorphIndex, PurgeReport, TextPurger, TextScanner, };
pub use morphology::{normalize_umlauts, GermanCompoundSplitter, MorphologicalTokenizer};
pub use posting_list::{Posting, PostingList, ResidentPostingIndex};
pub use stream::{Bm25CandidateStream, DEFAULT_STREAM_BATCH_SIZE};
pub use tokenizer::{DefaultTokenizer, GermanMorphTokenizer, Tokenizer};
pub use crate::compat::scorer::Bm25Scorer;
```

### Feature-Flags

| Flag | Aktiviert |
|---|---|
| `default` | `["std", "workspace"]` |
| `std` | `[]` |
| `serde` | `[]` |
| `tracing` | `[]` |
| `inspect` | `[]` |
| `experimental` | `[]` |
| `workspace` | `["types-compat", "ports-compat"]` |
| `types-compat` | `["dep:acta-types", "dep:acta-ports"]` |
| `ports-compat` | `["types-compat"]` |
| `bm25f` | `[]` |
| `docid-128` | `["acta-ids/docid-128"]` |

### Schnittstellen — Public Traits (10)

**`Analyzer`** — *Trait für vollständige Textanalyse-Pipelines (`Tokenizer -> Filter* -> Stemmer`). Definiert den identischen Ergebnisvertrag für Dokumenten-Indexierung und Abfrage.* (`src/tokenizer.rs`)

```rust
pub trait Analyzer : Send + Sync {
/// Analysiert den gegebenen Text und liefert eine strukturierte Token-Sequenz.
    fn analyze(&self, text: &str) -> Result<Vec<Token>>;

    /// Liefert die stabile, kryptographisch eindeutige Fingerprint-Zeichenkette
    /// bestehend aus Name, Version und Konfigurations-Hash.
    fn fingerprint(&self) -> String;
}
```

**`Clock`** — *Minimale Zeituhr-Schnittstelle zur Bereitstellung monotoner oder Unix-Millisekunden.* (`src/contract.rs`)

```rust
pub trait Clock : Send + Sync {
/// Liefert den aktuellen Unix-Zeitstempel in Millisekunden.
    fn now_unix_ms(&self) -> u64;
}
```

**`DomainVocabulary`** — *Trait für domänenspezifische Vokabularpakete (z. B. Recht und Medizin).* (`src/domain/mod.rs`)

```rust
pub trait DomainVocabulary : Send + Sync {
/// Domänenspezifische Komposita-Bausteine (Ergänzung zum KMU-Basiswörterbuch).
    fn compound_stems(&self) -> &'static [&'static str];
    /// Fachbegriffe, die NICHT als Stopwort behandelt werden dürfen, selbst wenn sie kurz/häufig sind
    /// (z. B. "Klage", "Akte" im juristischen Kontext).
    fn protected_terms(&self) -> &'static [&'static str];
    /// Kurzname für Logging/Konfiguration, z. B. "legal_de", "medical_de".
    fn domain_id(&self) -> &'static str;
}
```

**`Index`** — *Kern-Index-Schnittstelle (Klasse A / SPEC §4.3).* (`src/traits.rs`)

```rust
pub trait Index<Id, Query: ?Sized> {
/// Treffertyp der Suchoperation.
    type Hit;

    /// Fügt ein Dokument mit gegebener ID und Byte-Nutzlast in den Index ein.
    fn insert(&mut self, id: Id, payload: &[u8]) -> Result<()>;

    /// Entfernt ein Dokument anhand seiner ID aus dem Index.
    ///
    /// Gibt `true` zurück, wenn das Dokument vorhanden war und gelöscht wurde.
    fn remove(&mut self, id: &Id) -> Result<bool>;

    /// Führt eine Suche anhand der Query aus und liefert bis zu `limit` Treffer zurück.
    fn search(&self, query: &Query, limit: usize) -> Result<Vec<Self::Hit>>;
}
```

**`Maintain`** — *Neutrale Wartungsschnittstelle (Klasse A / SPEC §4.3).* (`src/traits.rs`)

```rust
pub trait Maintain {
/// Statistiktyp der Kompaktierungsoperation.
    type Stats;

    /// Führt eine Kompaktierung durch und gibt Segmente bzw. gelöschte Einträge tatsächlich frei.
    fn compact(&mut self) -> Result<Self::Stats>;

    /// Bereinigt fragmentierte interne Strukturen.
    fn vacuum(&mut self) -> Result<()>;
}
```

**`MorphologicalTokenizer`** — *Trait für morphologische Zerlegung von Wörtern in ihre Grundbestandteile. # Eingabevertrag Eingegebene Wörter MÜSSEN vor dem Aufruf von `decompose` in Kleinbuchstaben vorliegen und idealerweise mit [`normalize_umlauts`] vorbereitet sein. Bei Großbuchstaben od…* (`src/morphology/compound_splitter.rs`)

```rust
pub trait MorphologicalTokenizer : Send + Sync {
/// Zerlegt ein Token in morphologische Komponenten (Teilwörter).
    fn decompose<'a>(&self, token: &'a str) -> Vec<&'a str>;

    /// Liefert den Sprachcode (z. B. "de").
    fn language(&self) -> &str;
}
```

**`PostingStore`** — *Lokale Speicherabstraktion für den invertierten Index. Enthält exakt die im Kern genutzten Methoden aus `StorageEngine` (Inventar aus TX6.0).* (`src/storage.rs`)

```rust
pub trait PostingStore : Send + Sync {
/// Liest den Wert zu einem Schlüssel.
    fn get<'a>(&'a self, key: &'a [u8]) -> BoxFuture<'a, crate::error::Result<Option<Bytes>>>;

    /// Liest den Wert zu einem Schlüssel unter Snapshot-Isolation zur Sequenznummer `seq`.
    fn get_at_seq<'a>(
        &'a self,
        key: &'a [u8],
        seq: u64,
    ) -> BoxFuture<'a, crate::error::Result<Option<Bytes>>>;

    /// Schreibt einen Schlüssel/Wert-Eintrag im Kontext der Transaktion `tx`.
    fn put<'a>(
        &'a self,
        tx: TxId,
        key: &'a [u8],
        value: &'a [u8],
    ) -> BoxFuture<'a, crate::error::Result<()>>;

    /// Löscht einen Schlüssel im Kontext der Transaktion `tx`.
    fn delete<'a>(&'a self, tx: TxId, key: &'a [u8]) -> BoxFuture<'a, crate::error::Result<()>>;

    /// Bestätigt die Transaktion `tx`.
    fn commit<'a>(&'a self, tx: TxId) -> BoxFuture<'a, crate::error::Result<()>>;

    /// Rollt die Transaktion `tx` zurück.
    fn rollback<'a>(&'a self, tx: TxId) -> BoxFuture<'a, crate::error::Result<()>>;

    /// Rollt alle Transaktionen bis einschließlich `tx_id` zurück.
    fn rollback_to_tx<'a>(&'a self, tx_id: TxId) -> BoxFuture<'a, crate::error::Result<()>>;

    /// Liefert die höchste im Speicher vergebene Transaktions-ID.
    fn last_tx_id<'a>(&'a self) -> BoxFuture<'a, crate::error::Result<TxId>>;

    /// Liefert die aktuelle MVCC-Sequenznummer.
    fn last_seq_no<'a>(&'a self) -> BoxFuture<'a, crate::error::Result<u64>>;

    /// Scannt Schlüssel/Werte mit dem gegebenen Präfix.
    fn scan_prefix<'a>(
        &'a self,
        prefix: &'a [u8],
    ) -> BoxFuture<'a, crate::error::Result<Vec<(Vec<u8>, Vec<u8>)>>>;

    /// Scannt Schlüssel/Werte mit dem gegebenen Präfix unter Snapshot-Isolation.
    fn scan_prefix_at<'a>(
        &'a self,
        prefix: &'a [u8],
        seq: u64,
    ) -> BoxFuture<'a, crate::error::Result<Vec<(Vec<u8>, Vec<u8>)>>>;
}
```

**`Rng`** — *Minimale Zufallsquelle für Baukasten-Operationen.* (`src/contract.rs`)

```rust
pub trait Rng : Send {
/// Liefert die nächste Zufallszahl als `u64`.
    fn next_u64(&mut self) -> u64;
}
```

**`TokenFilter`** — *Trait für Token-Filter in einer Analyse-Pipeline.* (`src/tokenizer.rs`)

```rust
pub trait TokenFilter : Send + Sync {
/// Wendet den Filter auf eine Sequenz von Tokens an.
    fn filter(&self, tokens: Vec<Token>) -> Vec<Token>;
    /// Eindeutiger Bezeichner des Filters für die Fingerprint-Generierung.
    fn filter_id(&self) -> &str;
}
```

**`Tokenizer`** — *Trait für Text-Tokenizer mit unterschiedlichen Filter- oder Zerlegungsstrategien.* (`src/tokenizer.rs`)

```rust
pub trait Tokenizer : Send + Sync {
/// Zerlegt den übergebenen Text in kleingeschriebene Wort-Token und filtert Stoppwörter heraus.
    fn tokenize(&self, text: &str) -> Vec<String>;
}
```

### Öffentliche Datentypen — Structs (39)

| Struct | Felder / Form | Beschreibung | Datei |
|---|---|---|---|
| `AnalyzerPipeline` | `{ name: String, version: u32, limits: PipelineLimits, normalization: NormalizationPolicy, stopwords: Option<HashSet<String>>, filters: Vec<Arc<dyn TokenFilter>>,… }` | Modulare, konfigurierbare Textanalyse-Pipeline. | `src/tokenizer.rs` |
| `BM25` | `{ /// Sättigungsparameter für Termhäufigkeit (k1 >= 0). pub k1: f32, /// Dämpfungsparameter für Dokumentenlänge (0.0 <= b <= 1.0). pub b: f32, }` | Konfigurationsparameter für das BM25-Scoring-Modell. - `k1`: Steuert die Sättigung der Termhäufigkeit (TF). Standardwert: 1.5. - `b`: Steuert den Ein… | `src/bm25.rs` |
| `BM25F` | `{ /// Sättigungsparameter k1. pub k1: f32, /// Konfigurierte Feldgewichtungen. pub field_weights: Vec<FieldWeight>, cached_weights: Vec<(FieldId, f32, f32)>, }` | Konfiguration für feldgewichtetes BM25F-Scoring (§11.3). | `src/bm25.rs` |
| `BM25MorphIndex` | `{ inner: InvertedIndex<S>, tokenizer: Arc<dyn MorphologicalTokenizer>, }` | Invertierter Index mit morphologischer Zerlegung für deutsche Volltextsuche. | `src/compat/morph_index.rs` |
| `Bm25CandidateStream` | `{ index: &'a InvertedIndex<S>, query: String, max_seq: Option<u64>, batch_size: usize, buffer: VecDeque<(DocId, f32)>, seen: BTreeSet<DocId>, fetched_k: usize, y… }` | Asynchroner Pull-Stream zum bedarfsgerechten Nachladen von BM25-Kandidaten. | `src/stream.rs` |
| `Bm25Scorer` | `{ index: InvertedIndex<S>, }` | — | `src/compat/scorer.rs` |
| `BoundedTopK` | `{ heap: BinaryHeap<Bm25Candidate>, capacity: usize, }` | Begrenzter Min-Heap zur effizienten Verwaltung der K besten Treffer. | `src/wand.rs` |
| `CompactStats` | `{ /// Anzahl physisch bereinigter Tombstones. pub purged_docs: usize, /// Anzahl verbliebener aktiver Dokumente. pub active_docs: usize, }` | Statistiken über den Kompaktierungsvorgang. | `src/memory_index.rs` |
| `DefaultTokenizer` | `(tuple/unit)` | Standard-Tokenizer basierend auf Unicode-Wortgrenzen und generischen Stoppwörtern. | `src/tokenizer.rs` |
| `DocId` | `(tuple/unit)` | Standard 64-Bit Dokument-Identifikator. | `src/types.rs` |
| `DocId` | `(tuple/unit)` | 128-Bit Dokument-Identifikator bei aktiviertem `docid-128` Feature. | `src/types.rs` |
| `DocKey` | `(tuple/unit)` | Autarker Dokumentenschlüssel (128-Bit fähig). | `src/contract.rs` |
| `Epoch` | `(tuple/unit)` | Opake, monotone Sichtbarkeitsmarke zur Abstraktion von Transaktions- und MVCC-Ständen. | `src/contract.rs` |
| `FieldWeight` | `{ /// Feld-Identifikator. pub field_id: FieldId, /// Gewichtungsfaktor des Feldes. pub weight: f32, /// Längendämpfungsparameter des Feldes. pub b: f32, }` | Feldgewichtungskonfiguration für ein bestimmtes Feld in BM25F. | `src/bm25.rs` |
| `GermanCompoundSplitter` | `{ /// Minimum component length for splitting. min_component_len: usize, /// Maximum decomposition depth (number of segments). max_depth: usize, /// Trie data str… }` | Deutscher Komposita-Zerleger basierend auf Dynamischer Programmierung und Fugenanalyse. # Funktionsweise & Algorithmus 1. **Eingebettetes Wörterbuch*… | `src/morphology/compound_splitter.rs` |
| `GermanMorphTokenizer` | `{ splitter: Arc<crate::morphology::GermanCompoundSplitter>, }` | Deutscher Tokenizer mit morphologischer Zerlegung von Wortzusammensetzungen. | `src/tokenizer.rs` |
| `IndexSnapshot` | `{ /// Opake, monotone Epochenmarke dieses Snapshots. pub epoch: Epoch, /// Invertiertes Wörterbuch mit sortierten Posting-Listen. pub postings: BTreeMap<String,… }` | Unveränderlicher Snapshot des Index für lock-freie, konsistente Lesezugriffe. | `src/memory_index.rs` |
| `InvertedIndex` | `{ storage: Arc<S>, prefix: Vec<u8>, tokenizer: Arc<dyn Tokenizer>, pub(crate) total_docs: Arc<AtomicU64>, pub(crate) total_tokens: Arc<AtomicU64>, pub(crate) avg… }` | Persistenter invertierter Index über einer Kollektion im Storage-Backend. # Nebenläufigkeit & Sperrhierarchie: 1. `commit_lock` (`parking_lot::Mutex`… | `src/inverted/index_struct.rs` |
| `KeyInspection` | `{ /// Ob der Schlüssel in den aktiven Dokumentlängen vorkommt. pub in_doc_lengths: bool, /// Ob der Schlüssel aktuell als Tombstone markiert ist. pub is_tombston… }` | Prüfergebnis einer Schlüssel-Inspektion (Feature `inspect`). | `src/memory_index.rs` |
| `LegalDomainVocabulary` | `(tuple/unit)` | Deutsches Juristisches Fachvokabular zur Anbindung an [`DomainVocabulary`]. | `src/domain/legal_de.rs` |
| `MedicalDomainVocabulary` | `(tuple/unit)` | Deutsches Medizinisches Fachvokabular zur Anbindung an [`DomainVocabulary`]. | `src/domain/medical_de.rs` |
| `MemoryTextIndex` | `{ tokenizer: Arc<dyn Tokenizer>, scorer: BM25, snapshot: RwLock<Arc<IndexSnapshot>>, }` | Autarker, thread-sicherer In-Memory Volltextindex (Klasse A). Bietet die standardisierte Index-Schnittstelle gemäß SPEC §4.3 (`insert`, `remove`, `se… | `src/memory_index.rs` |
| `PassthroughTokenizer` | `{ lang: String, }` | Passthrough-Tokenizer für Sprachen, die keine Zerlegung von Zusammensetzungen benötigen. Gibt eingegebene Token unverändert als Einzelkomponente zurü… | `src/morphology/passthrough.rs` |
| `PipelineLimits` | `{ /// Maximale Eingabelänge in Bytes (Default: 1 MiB). pub max_input_length: usize, /// Maximale Tokenlänge in Bytes (Default: 256 Bytes). pub max_token_length:… }` | Konfigurierbare Ressourcen- und Sicherheitsgrenzen der Analyse-Pipeline. | `src/tokenizer.rs` |
| `Posting` | `{ /// Dokument-ID des Eintrags. pub doc_id: DocIdRaw, /// Termfrequenz (Häufigkeit des Terms in diesem Dokument). pub tf: u32, /// Gesamtlänge des Dokuments in T… }` | Kompakte Repräsentation eines einzelnen Vorkommens (Posting) im Index. 8-Byte ausgerichtet (`repr(C, align(8))`) für optimale CPU-Cacheline-Ausnutzun… | `src/posting_list.rs` |
| `PostingBlockInfo` | `{ /// Höchste Dokument-ID in diesem Block (ermöglicht schnelles Überspringen). pub max_doc_id: DocIdRaw, /// Maximale Termfrequenz innerhalb dieses Blocks. pub m… }` | Metadaten eines 64er-Posting-Blocks zur Berechnung oberer BM25-Schranken in Block-Max WAND. | `src/posting_list.rs` |
| `PostingList` | `{ postings: Vec<Posting>, blocks: Vec<PostingBlockInfo>, }` | Zusammenhängende, nach DocId sortierte Posting-Liste für einen Begriff. | `src/posting_list.rs` |
| `PurgeReport` | `{ /// Anzahl erfolgreich physisch gelöschter Einheiten. pub units_purged: u64, /// Anzahl fehlgeschlagener Einheiten während der Ausführung. pub units_failed: u6… }` | — | `src/compat/purge.rs` |
| `ResidentPostingIndex` | `{ index: RwLock<BTreeMap<String, Arc<PostingList>>>, }` | Speicherresidenter Lese-Cache für Posting-Listen mit RCU-Verwaltung. | `src/posting_list.rs` |
| `StorageBridge` | `(tuple/unit)` | Adapter-Newtype für `StorageEngine` zur Vermeidung von Blanket-Impl-Kollisionen. | `src/compat/storage.rs` |
| `TextHit` | `{ /// Eindeutige ID des Dokuments. pub doc_id: DocId, /// Berechneter BM25-Relevanz-Score. pub score: f32, }` | Ergebnisdokument mit Relevanz-Score für BM25-Suchen. | `src/types.rs` |
| `TextIndexMetadata` | `{ /// Gesamtzahl der aktiven Dokumente im Index. pub total_docs: u64, /// Gesamtzahl aller indizierten Token über alle Dokumente. pub total_tokens: u64, /// Fest… }` | Konsolidierte Metadaten des Textindex zur persistenten Speicherung im Schlüssel `meta:stats`. | `src/inverted/types.rs` |
| `TextPurger` | `{ index: Arc<InvertedIndex<S>>, }` | — | `src/compat/purge.rs` |
| `TextScanner` | `{ index: Arc<InvertedIndex<S>>, target: Arc<parking_lot::Mutex<Option<ScanTarget>>>, }` | — | `src/compat/scanner.rs` |
| `Token` | `{ /// Normalisierter Termwert. pub term: String, /// Start-Byte-Offset im Originaltext (auf gültiger Zeichengrenze). pub start_offset: usize, /// End-Byte-Offset… }` | Repräsentiert ein strukturiertes Token aus der Textanalyse-Pipeline. Ein Token umfasst den extrahierten Term, Byte-Offsets in den Originaltext (stets… | `src/tokenizer.rs` |
| `TokenReductionMetrics` | `{ /// Anzahl ursprünglicher Token vor der morphologischen Zerlegung. pub original_tokens: usize, /// Anzahl resultierender Token nach Zerlegung von Komposita. pu… }` | Metriken zur Erfassung der Token-Expansions- und Zerlegungswirkung. | `src/morphology/metrics.rs` |
| `Trie` | `{ root: TrieNode, }` | Präfixbaum (Trie) für schnelle O(m)-Wörterbuchabfragen (m = Wortlänge). | `src/morphology/trie.rs` |
| `TrieNode` | `{ /// Zeigt an, ob an diesem Knoten ein vollständiges Wort aus dem Wörterbuch endet. pub is_terminal: bool, /// Kindknoten, indiziert nach Folgezeichen. pub chil… }` | Knoten im Präfixbaum zur Repräsentation von Buchstabenfolgen deutscher Wörter. | `src/morphology/trie.rs` |
| `WandSearchResult` | `{ /// Gefundene Trefferdokumente mit Relevanzwert. pub results: Vec<(DocId, f32)>, /// Anzahl tatsächlich voll berechneter Dokumente (Pruning-Effektivitätsmetrik… }` | Ergebnis einer Block-Max-WAND-Suche inklusive Top-K-Treffern und Metadaten über evaluierte Dokumente. | `src/wand.rs` |

### Öffentliche Datentypen — Enums (6)

| Enum | Varianten | Beschreibung | Datei |
|---|---|---|---|
| `Error` | `/// Ungültige Eingabedaten. #[error("ungültige Eingabedaten: {0}")] InvalidInput(&'static str), /// Kapazitätsgrenze überschritten. #[error("Kapazitätsgrenze überschritten: gefordert {requested}, max…` | Minimaler Fehlertyp für Umgebungs- und Kontraktoperationen. | `src/contract.rs` |
| `Language` | `/// Englischer Standard-Tokenizer (Unicode-Wortgrenzen + Stoppwortfilterung). #[default] English, /// Deutscher morphologischer Tokenizer (Komposita-Zerlegung + Umlaut-Normalisierung). German, /// Be…` | Unterstützte Sprachen und Tokenisierungsstrategien für den invertierten Index. Steuert, welcher Tokenizer für die Dokumentenindizierung und Suchabfra… | `src/inverted/types.rs` |
| `NormalizationPolicy` | `/// Nur Unicode Lowercasing. #[default] Lowercase, /// NFKC-Normalisierung (Ligaturen auflösen, ZWJ/Formatzeichen entfernen, Fullwidth normalisieren) + Lowercasing. Nfkc, /// Spezifische deutsche Uml…` | Konfigurierbare Normalisierungsrichtlinie für Textanalyse. | `src/tokenizer.rs` |
| `OverflowPolicy` | `/// Überschüssige Eingaben oder Token werden deterministisch abgeschnitten. #[default] Truncate, /// Eine Überschreitung führt zu einem Fail-Closed Fehler. Error,` | Strategie bei Überschreitung konfigurierter Kapazitätsgrenzen. | `src/tokenizer.rs` |
| `ParsedTextTarget` | `/// Einzeldokument-Ziel. Document(DocId), /// Kollektions-Namespace-Ziel. Collection(String),` | — | `src/compat/scanner.rs` |
| `TextError` | `/// Ungültige Eingabedaten (z. B. leere/ungültige Abfrage, ungültige Scoring-Parameter). #[error("ungültige Eingabedaten: {0}")] InvalidInput(String), /// Fehler bei Textanalyse, Regex oder Tokenisie…` | Typisierter Fehler für Volltextindexierung, Tokenisierung und Scoring. | `src/error.rs` |

### Typ-Aliase (6)

| Alias | Definition | Beschreibung |
|---|---|---|
| `BoxFuture` | `type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>` | Typ-Alias für asynchrone Ergebnisse. |
| `DocIdRaw` | `type DocIdRaw= u64` | Primitiver Rohdatentyp für Dokument-IDs im Postings-Format. |
| `DocIdRaw` | `type DocIdRaw= u128` | Primitiver Rohdatentyp für 128-Bit Dokument-IDs im Postings-Format. |
| `FieldId` | `type FieldId= u32` | Eindeutige Kennung für Dokumentenfelder im feldgewichteten BM25F-Scoring. |
| `Result` | `type Result<T> = core::result::Result<T, Error>` | Einheitlicher Result-Typ für Kontraktoperationen. |
| `Result` | `type Result<T> = core::result::Result<T, TextError>` | Einheitlicher Result-Typ für `acta-text`. |

### Konstanten (11)

| Konstante | Typ | Beschreibung |
|---|---|---|
| `BLOCK_SIZE` | `usize` | Blockgröße für die Block-Max-WAND-Zerlegung (64 Postings pro Block). |
| `BM25_B` | `f32` | Standard-b-Parameter für die Dokumentlängen-Normalisierung in BM25 (0.75). |
| `BM25_K1` | `f32` | Standard-k1-Parameter für die Termfrequenz-Sättigung in BM25 (1.5). |
| `DEFAULT_STREAM_BATCH_SIZE` | `usize` | Standard-Batch-Größe für das Nachladen von Kandidaten (16 Dokumente). |
| `MAX_DOC_BYTES` | `usize` | Maximale Dokumentgröße in Bytes (fail-closed, 10 MB). |
| `MAX_DOC_TERMS` | `usize` | Maximale Anzahl von Begriffen (Tokens) pro Dokument (fail-closed). |
| `MAX_POSTINGS_PER_TERM` | `usize` | Maximale Anzahl von Postings pro Term (fail-closed). |
| `MAX_SEARCH_K` | `usize` | Maximale Anzahl von Suchtreffern pro Anfrage (SPEC §11.3). |
| `MAX_STAGED_TRANSACTIONS` | `usize` | Maximale Anzahl gleichzeitig ungecommitteter Transaktionen (10.000). |
| `MAX_TEXT_BYTES` | `usize` | Maximale Textgröße pro Dokument (10 MiB) zur Begrenzung von Speicherverbrauch und DoS-Schutz. |
| `POSTING_LIST_V2_MAGIC` | `&[u8` | Magische Bytes für das kompakte V2-Posting-Listen-Format (`PL\x02\x00`). |

### Öffentliche Funktionen (127)

| Signatur | Beschreibung | Datei |
|---|---|---|
| `pub fn new(k1: f32, b: f32) -> Result<Self>` | Erzeugt eine neue `BM25`-Instanz mit validierten Hyperparametern. # Fehler Liefert [`TextError::InvalidInput`], falls `k1 < 0.0` bzw. NaN i… | `src/bm25.rs` |
| `pub fn new(field_id: FieldId, weight: f32, b: f32) -> Result<Self>` | Erzeugt eine neue `FieldWeight`-Konfiguration nach Validierung von `weight` und `b`. | `src/bm25.rs` |
| `pub fn new(k1: f32, field_weights: Vec<FieldWeight>) -> Result<Self>` | Erzeugt ein neues BM25F-Modell mit Feldgewichtungen. | `src/bm25.rs` |
| `pub fn score_term(&self, tf: u32, doc_len: u32, avg_doc_len: f32, df: u32, n: u32) -> f32` | Berechnet den BM25-Score für einen Begriff mit den Parametern dieser Instanz. | `src/bm25.rs` |
| `pub fn score_term(tf: u32, doc_len: u32, avg_doc_len: f32, df: u32, n: u32) -> f32` | Berechnet den BM25-Score für einen Begriff mit den Standardparametern (`k1 = 1.5`, `b = 0.75`). # Parameter * `tf` - Termhäufigkeit (Term F… | `src/bm25.rs` |
| `pub fn score_term( &self, field_term_frequencies: &[(FieldId, u32, u32, f32)], df: u32, n: u32, ) -> f32` | Berechnet den BM25F-Score über mehrere Felder hinweg. | `src/bm25.rs` |
| `pub fn score_term_bm25f( field_term_frequencies: &[(FieldId, u32, u32, f32)], field_weights: &[(FieldId, f32, f32)], k1: f32, df: u32, n: u32, ) -> f32` | Berechnet den feldgewichteten BM25F-Score nach Robertson & Zaragoza (2004). Berechnet zunächst die gewichtete Pseudo-Termfrequenz über alle… | `src/bm25.rs` |
| `pub fn score_term_with_params( tf: u32, doc_len: u32, avg_doc_len: f32, df: u32, n: u32, k1: f32, b: f32, ) -> f32` | Berechnet den BM25-Score mit frei konfigurierbaren Parametern `k1` und `b`. # Ablauf 1. Schnellausstieg mit 0.0, wenn $N = 0$, $df = 0$ ode… | `src/bm25.rs` |
| `pub fn new( storage: Arc<S>, namespace: &str, tokenizer: Arc<dyn MorphologicalTokenizer>, ) -> Self` | Erzeugt einen neuen morphologischen Index mit deutschem Standard-Tokenizer. | `src/compat/morph_index.rs` |
| `pub fn tokenizer(&self) -> &dyn MorphologicalTokenizer` | Liefert eine Referenz auf den konfigurierten morphologischen Tokenizer. | `src/compat/morph_index.rs` |
| `pub fn is_success(&self) -> bool` | Liefert `true`, wenn die Bereinigung fehlerfrei und vollständig war. | `src/compat/purge.rs` |
| `pub fn new() -> Self` | Erzeugt einen leeren Purge-Bericht. | `src/compat/purge.rs` |
| `pub fn new(index: Arc<InvertedIndex<S>>) -> Self` | Creates a new `TextPurger` for the given index. | `src/compat/purge.rs` |
| `pub fn purge_collection(&self, tx: TxId, namespace: &str) -> Result<PurgeStats>` | Fast-path purge for an entire collection namespace (`__txt:{collection}:`). | `src/compat/purge.rs` |
| `pub fn purge_collection_report(&self, tx: TxId, namespace: &str) -> PurgeReport` | Fast-path purge for an entire collection namespace (`__txt:{collection}:`), returning a `PurgeReport`. | `src/compat/purge.rs` |
| `pub fn purge_document(&self, tx: TxId, doc_id: DocId) -> Result<PurgeStats>` | Purges a single document from the inverted index and recalculates corpus statistics. | `src/compat/purge.rs` |
| `pub fn purge_document_report(&self, tx: TxId, doc_id: DocId) -> PurgeReport` | Purges a single document from the inverted index and recalculates corpus statistics, returning a detailed `PurgeReport`. | `src/compat/purge.rs` |
| `pub fn text_purger<S: StorageEngine + 'static>(index: Arc<InvertedIndex<S>>) -> Arc<dyn Purger>` | — | `src/compat/purge.rs` |
| `pub fn to_purge_stats(&self) -> Result<PurgeStats>` | Konvertiert den Bericht in [`PurgeStats`] oder liefert einen Fehler bei Teilabbrüchen. | `src/compat/purge.rs` |
| `pub fn new(index: Arc<InvertedIndex<S>>) -> Self` | Erzeugt einen neuen `TextScanner` für den übergebenen invertierten Index. | `src/compat/scanner.rs` |
| `pub fn new_with_target(index: Arc<InvertedIndex<S>>, target: ScanTarget) -> Self` | Erzeugt einen neuen `TextScanner` mit vorab konfiguriertem Scan-Ziel. | `src/compat/scanner.rs` |
| `pub fn parse(target_id: &str) -> Self` | Parses a target ID string into document ID or collection namespace. | `src/compat/scanner.rs` |
| `pub fn set_target(&self, target: ScanTarget)` | Weist dem Scanner ein neues Zielobjekt zu. | `src/compat/scanner.rs` |
| `pub fn text_scanner<S: StorageEngine + 'static>( index: Arc<InvertedIndex<S>>, ) -> Arc<dyn LayerScanner>` | — | `src/compat/scanner.rs` |
| `pub fn with_target(self, target: ScanTarget) -> Self` | Konfiguriert das Scan-Ziel im Builder-Muster. | `src/compat/scanner.rs` |
| `pub fn new(storage: std::sync::Arc<S>, namespace: &str) -> Self` | Erzeugt eine neue `Bm25Scorer`-Instanz über der angegebenen Speicher-Engine. # Ablauf 1. Übernahme der Referenz auf die `StorageEngine`. 2.… | `src/compat/scorer.rs` |
| `pub fn get_legal_compound_stems() -> &'static [&'static str]` | Liefert den statischen Slice aller juristischen Komposita-Wortstämme. | `src/domain/legal_de.rs` |
| `pub fn get_legal_protected_terms() -> &'static [&'static str]` | Liefert den statischen Slice aller juristischen geschützten Begriffe. | `src/domain/legal_de.rs` |
| `pub fn get_medical_compound_stems() -> &'static [&'static str]` | Liefert den statischen Slice aller medizinischen Komposita-Wortstämme. | `src/domain/medical_de.rs` |
| `pub fn get_medical_protected_terms() -> &'static [&'static str]` | Liefert den statischen Slice aller medizinischen geschützten Fachbegriffe. | `src/domain/medical_de.rs` |
| `pub async fn commit(&self, tx: TxId) -> Result<()>` | Delegiert das Commit auf der Storage-Engine nach `commit_stats`. | `src/inverted/index_struct.rs` |
| `pub fn commit_lock(&self) -> &parking_lot::Mutex<()>` | Accessor for single-writer commit lock. | `src/inverted/index_struct.rs` |
| `pub async fn commit_stats(&self, tx: TxId) -> Result<()>` | Überträgt die gepufferten statistischen Änderungen einer Transaktion dauerhaft. | `src/inverted/index_struct.rs` |
| `pub async fn compact_terms(&self, tx: TxId) -> Result<u64>` | Kompaktiert Terme und bereinigt verwaiste Metadaten im Inverted Index (Maintenance / Compaction API). | `src/inverted/index_struct.rs` |
| `pub async fn delete_document(&self, tx: TxId, doc_id: DocId) -> Result<()>` | Löscht ein Dokument logisch aus dem Index und markiert alle Terme mit Tombstones. | `src/inverted/index_struct.rs` |
| `pub async fn last_tx_id(&self) -> Result<TxId>` | Delegiert `last_tx_id` auf der Storage-Engine. | `src/inverted/index_struct.rs` |
| `pub fn len_sync(&self) -> usize` | Liefert die Anzahl aktiver Dokumente im Index. | `src/inverted/index_struct.rs` |
| `pub async fn load_stats(&self) -> Result<()>` | Lädt die persistenten Statistiken (`meta:stats`) aus dem Storage in die atomaren In-Memory-Felder. | `src/inverted/index_struct.rs` |
| `pub fn new(storage: Arc<S>, namespace: &str) -> Self` | Erzeugt einen neuen invertierten Index mit englischem Standard-Tokenizer. | `src/inverted/index_struct.rs` |
| `pub fn new_with_language(storage: Arc<S>, namespace: &str, language: Language) -> Self` | Erzeugt einen neuen invertierten Index mit expliziter Sprachkonfiguration. # Language Selection Use `Language::from_iso("de")` for German,… | `src/inverted/index_struct.rs` |
| `pub fn prefix(&self) -> &[u8]` | Accessor for index storage prefix bytes. | `src/inverted/index_struct.rs` |
| `pub async fn resolve_tombstones(&self, tx: TxId) -> Result<u64>` | Bereinigt Tombstones im Inverted Index für gegebene Transaktion. | `src/inverted/index_struct.rs` |
| `pub async fn rewrite_posting_lists(&self, tx: TxId, terms: &[&str]) -> Result<usize>` | Schreibt Postinglisten für gegebene Terme neu (Maintenance / Rewrite API). | `src/inverted/index_struct.rs` |
| `pub async fn rollback(&self, tx: TxId) -> Result<()>` | Delegiert das Rollback auf der Storage-Engine nach `rollback_stats`. | `src/inverted/index_struct.rs` |
| `pub async fn rollback_stats(&self, tx: TxId) -> Result<()>` | Verwirft die gepufferten statistischen Änderungen einer Transaktion bei Rollback. | `src/inverted/index_struct.rs` |
| `pub async fn rollback_to_tx(&self, tx_id: TxId) -> Result<()>` | Delegiert `rollback_to_tx` auf der Storage-Engine. | `src/inverted/index_struct.rs` |
| `pub async fn search_bm25( &self, query: &str, k: usize, max_seq: Option<u64>, ) -> Result<Vec<(DocId, f32)>>` | Führt eine Volltextsuche mit BM25 aus. | `src/inverted/index_struct.rs` |
| `pub async fn search_bm25_at( &self, query: &str, k: usize, max_seq: Option<u64>, ) -> Result<Vec<(DocId, f32)>>` | Führt eine Volltextsuche mit BM25 zu einer bestimmten MVCC-Snapshot-Sequenznummer aus. | `src/inverted/index_struct.rs` |
| `pub fn storage(&self) -> &Arc<S>` | Accessor for underlying storage engine. | `src/inverted/index_struct.rs` |
| `pub async fn upsert_document(&self, tx: TxId, doc_id: DocId, text: &str) -> Result<()>` | Fügt ein Dokument in den Index ein oder aktualisiert ein bestehendes. | `src/inverted/index_struct.rs` |
| `pub fn with_tokenizer(mut self, tokenizer: Arc<dyn Tokenizer>) -> Self` | Konfiguriert einen benutzerdefinierten Tokenizer. | `src/inverted/index_struct.rs` |
| `pub fn from_iso(code: &str) -> Self` | Parst einen ISO-639-1 Sprachcode ("de", "de-AT", "en", "en-US"). Bewertet nur das primäre Subtag vor dem ersten Bindestrich. Unbekannte Cod… | `src/inverted/types.rs` |
| `pub fn current_epoch(&self) -> Epoch` | Liefert die aktuelle Epoche des Index. | `src/memory_index.rs` |
| `pub fn inspect_doc_lengths(&self) -> BTreeMap<DocKey, u32>` | Rohsicht-Inspektion der Dokumentlängen unter dem Feature `inspect`. | `src/memory_index.rs` |
| `pub fn inspect_key(&self, key: &DocKey) -> KeyInspection` | Rohsicht-Inspektion unter dem Feature `inspect`. | `src/memory_index.rs` |
| `pub fn inspect_postings(&self) -> BTreeMap<String, Vec<Posting>>` | Rohsicht-Inspektion aller Postings unter dem Feature `inspect`. | `src/memory_index.rs` |
| `pub fn is_empty(&self) -> bool` | Gibt an, ob der Index keine aktiven Dokumente enthält. | `src/memory_index.rs` |
| `pub fn len(&self) -> usize` | Liefert die Anzahl aktiver Dokumente im Index. | `src/memory_index.rs` |
| `pub fn new() -> Self` | Erzeugt einen neuen `MemoryTextIndex` mit Standard-Tokenizer und BM25-Scorer. | `src/memory_index.rs` |
| `pub fn search(&self, query_tokens: &[String], scorer: &BM25, limit: usize) -> Vec<TextHit>` | Sucht im unveränderlichen Snapshot anhand von BM25. | `src/memory_index.rs` |
| `pub fn snapshot(&self) -> Arc<IndexSnapshot>` | Liefert einen unveränderlichen Snapshot des Index (O(1) Lesezugriff). | `src/memory_index.rs` |
| `pub fn with_tokenizer(mut self, tokenizer: Arc<dyn Tokenizer>) -> Self` | Konfiguriert einen spezifischen Tokenizer. | `src/memory_index.rs` |
| `pub fn extend_vocabulary(&mut self, words: impl IntoIterator<Item = impl AsRef<str>>)` | Erweitert das Wörterbuch des Zerlegers dynamisch um zusätzliche Begriffe. | `src/morphology/compound_splitter.rs` |
| `pub fn max_depth(&self) -> usize` | Liefert die maximale Segmenttiefe. | `src/morphology/compound_splitter.rs` |
| `pub fn min_component_len(&self) -> usize` | Liefert die Mindestlänge für Teilkomponenten. | `src/morphology/compound_splitter.rs` |
| `pub fn new() -> Self` | Erzeugt einen neuen Zerleger mit der Standard-Mindestlänge (3 Zeichen) und Basis-Wortschatz. | `src/morphology/compound_splitter.rs` |
| `pub fn new_with_all_domains() -> Self` | Erzeugt einen Zerleger mit allen eingebauten Fachdomänen (Recht & Medizin). | `src/morphology/compound_splitter.rs` |
| `pub fn with_dictionary(min_len: usize, custom_words: HashSet<String>) -> Self` | Erzeugt einen Zerleger mit benutzerdefiniertem Wörterbuch. | `src/morphology/compound_splitter.rs` |
| `pub fn with_domains(min_len: usize, domains: &[&dyn DomainVocabulary]) -> Self` | Erzeugt einen Zerleger erweitert um spezifische Fachvokabulare (z. B. Recht, Medizin). | `src/morphology/compound_splitter.rs` |
| `pub fn with_max_depth(mut self, max_depth: usize) -> Self` | Setzt die maximale Rekursions-/Segmenttiefe für Zerlegungen. | `src/morphology/compound_splitter.rs` |
| `pub fn with_min_length(min_len: usize) -> Self` | Erzeugt einen Zerleger mit konfigurierbarer Mindest-Segmentlänge. | `src/morphology/compound_splitter.rs` |
| `pub fn expansion_ratio(&self) -> f32` | Berechnet das Expansionsverhältnis der Token-Menge. Ein Wert > 1,0 bedeutet eine Erhöhung der Tokenanzahl durch Komposita-Splitting. # Ziel… | `src/morphology/metrics.rs` |
| `pub fn new(lang: impl Into<String>) -> Self` | Erzeugt einen neuen Durchleitungs-Tokenizer für den angegebenen ISO-Sprachcode (z. B. "en"). | `src/morphology/passthrough.rs` |
| `pub fn get_german_stopwords() -> &'static HashSet<String>` | Liefert die statische deutsche Stoppwort-Liste. Wird per [`OnceLock`] beim ersten Zugriff lazy initialisiert. Erweitert um hochfrequente de… | `src/morphology/stopwords.rs` |
| `pub fn is_german_stopword(word: &str) -> bool` | Prüft, ob ein kleingeschriebenes, normalisiertes Wort ein deutsches Stoppwort ist. | `src/morphology/stopwords.rs` |
| `pub fn normalize_umlauts(input: &str) -> String` | Normalisiert deutsche Umlaute und Sonderzeichen für robusten Volltextabgleich. # Ersetzungen - `ä` -> `"ae"` - `ö` -> `"oe"` - `ü` -> `"ue"… | `src/morphology/stopwords.rs` |
| `pub fn contains(&self, word: &str) -> bool` | Prüft, ob ein Wort exakt als terminaler Eintrag im Baum enthalten ist. | `src/morphology/trie.rs` |
| `pub fn insert(&mut self, word: &str)` | Fügt ein Wort zeichenweise in den Baum ein und markiert den Endknoten als terminal. | `src/morphology/trie.rs` |
| `pub fn new() -> Self` | Erzeugt einen leeren Präfixbaum. | `src/morphology/trie.rs` |
| `pub fn starts_with(&self, prefix: &str) -> bool` | Prüft, ob im Baum mindestens ein Wort existiert, das mit dem übergebenen Präfix beginnt. | `src/morphology/trie.rs` |
| `pub fn as_slice(&self) -> &[Posting]` | Liefert die Postings als Slice. | `src/posting_list.rs` |
| `pub fn blocks(&self) -> &[PostingBlockInfo]` | Liefert die Block-Max-Metadaten. | `src/posting_list.rs` |
| `pub fn clear(&self)` | Leert den gesamten Cache (z. B. bei vollständigem Index-Neuladen). | `src/posting_list.rs` |
| `pub fn decode_compact(bytes: &[u8]) -> Result<Self, TextError>` | Dekodiert eine Posting-Liste aus dem Delta-Varint-Format mit Fallback auf Legacy-Bincode. # Defensives Verhalten Weist unvollständige, abge… | `src/posting_list.rs` |
| `pub fn doc_id(&self) -> DocId` | Liefert die `DocId` dieses Eintrags. | `src/posting_list.rs` |
| `pub fn empty() -> Self` | Erzeugt eine leere Posting-Liste. | `src/posting_list.rs` |
| `pub fn encode_compact(&self) -> Result<Vec<u8>, TextError>` | Kodiert die Posting-Liste in das kompakte Binärformat mit Delta-Varint-Kompression. | `src/posting_list.rs` |
| `pub fn get(&self, term: &str) -> Option<Arc<PostingList>>` | Liefert eine referenzgezählte Posting-Liste für den Begriff, falls im Cache vorhanden. | `src/posting_list.rs` |
| `pub fn insert_list(&self, term: String, list: PostingList)` | Fügt eine Posting-Liste in den Index ein oder ersetzt sie. | `src/posting_list.rs` |
| `pub fn is_empty(&self) -> bool` | Prüft, ob die Liste leer ist. | `src/posting_list.rs` |
| `pub fn len(&self) -> usize` | Liefert die Anzahl der Einträge in der Liste. | `src/posting_list.rs` |
| `pub fn new(doc_id: DocId, tf: u32, doc_len: u32) -> Self` | Erzeugt einen neuen `Posting`-Eintrag. | `src/posting_list.rs` |
| `pub fn new(mut postings: Vec<Posting>) -> Self` | Erzeugt eine sortierte und deduplizierte Posting-Liste und berechnet die Block-Metadaten. | `src/posting_list.rs` |
| `pub fn new() -> Self` | Erzeugt einen leeren residenten Posting-Index. | `src/posting_list.rs` |
| `pub fn remove(&self, doc_id: DocId) -> Self` | Entfernt ein Posting per Copy-On-Write (RCU). | `src/posting_list.rs` |
| `pub fn remove_posting_from_terms(&self, terms: &[String], doc_id: DocId)` | Entfernt eine Dokument-ID aus den Posting-Listen mehrerer Begriffe per RCU. | `src/posting_list.rs` |
| `pub fn remove_terms(&self, terms: &[String])` | Entfernt die angegebenen Begriffe aus dem Cache (z. B. bei Rollback). | `src/posting_list.rs` |
| `pub fn upsert(&self, posting: Posting) -> Self` | Fügt ein Posting per Copy-On-Write (RCU) ein oder aktualisiert einen bestehenden Eintrag. | `src/posting_list.rs` |
| `pub fn upsert_posting(&self, term: &str, posting: Posting)` | Fügt ein einzelnes Posting per RCU (Copy-On-Write) in die Posting-Liste des Begriffs ein. | `src/posting_list.rs` |
| `pub fn is_exhausted(&self) -> bool` | Prüft, ob der Stream vollständig erschöpft ist (keine gepufferten oder nachladbaren Elemente mehr). | `src/stream.rs` |
| `pub fn new( index: &'a InvertedIndex<S>, query: impl Into<String>, max_seq: Option<u64>, ) -> Self` | Erzeugt einen neuen Kandidaten-Stream für die übergebene Abfrage und Index-Snapshot. | `src/stream.rs` |
| `pub async fn next_batch(&mut self) -> Result<Vec<(DocId, f32)>>` | Holt den nächsten Batch von Kandidaten-Dokumenten (bis zu `batch_size`). | `src/stream.rs` |
| `pub async fn next_doc(&mut self) -> Result<Option<DocId>>` | Holt die nächste einzelne Dokument-ID aus dem Stream. | `src/stream.rs` |
| `pub fn with_batch_size(mut self, n: usize) -> Self` | Konfiguriert eine benutzerdefinierte Batch-Größe (mindestens 1). | `src/stream.rs` |
| `pub fn yielded(&self) -> usize` | Liefert die Gesamtzahl aller bisher an den Aufrufer herausgegebenen Treffer. | `src/stream.rs` |
| `pub fn compile_protected_regex(pattern: &str) -> Result<Regex>` | Kompiliert ein reguläres Muster für geschützte Token mit Fail-Closed-Fehlerbehandlung. # Fehler Gibt [`TextError::Text`] zurück, falls der… | `src/tokenizer.rs` |
| `pub fn default_english() -> Self` | Erzeugt die Standard-Pipeline für englischen und generischen Text. | `src/tokenizer.rs` |
| `pub fn default_german() -> Self` | Erzeugt die Standard-Pipeline für deutsche Texte inklusive Komposita-Zerlegung und Umlauten. | `src/tokenizer.rs` |
| `pub fn get_protected_regex() -> Result<&'static Regex>` | Liefert den gecachten regulären Standardausdruck für geschützte URLs und E-Mail-Adressen. | `src/tokenizer.rs` |
| `pub fn new() -> Self` | — | `src/tokenizer.rs` |
| `pub fn new() -> Self` | — | `src/tokenizer.rs` |
| `pub fn new( term: impl Into<String>, start_offset: usize, end_offset: usize, position_increment: usize, ) -> Self` | Erzeugt ein neues Token mit Term, Offsets und Positionsinkrement. | `src/tokenizer.rs` |
| `pub fn new(name: impl Into<String>, version: u32) -> Self` | Erzeugt eine neue leere Analyse-Pipeline mit Standard-Grenzen. | `src/tokenizer.rs` |
| `pub fn normalize(&self, text: &str) -> String` | Führt die Normalisierung deterministisch und idempotent auf dem String aus. | `src/tokenizer.rs` |
| `pub fn tokenize(text: &str) -> Vec<String>` | Hilfsfunktion zur Tokenisierung mit dem [`DefaultTokenizer`]. | `src/tokenizer.rs` |
| `pub fn try_new() -> Result<Self>` | Fallible Konstruktion eines `DefaultTokenizer` mit Validierung des regulären Schutzmusters. | `src/tokenizer.rs` |
| `pub fn try_new() -> Result<Self>` | Erzeugt eine neue Instanz des deutschen morphologischen Tokenizers. | `src/tokenizer.rs` |
| `pub fn with_filter(mut self, filter: Arc<dyn TokenFilter>) -> Self` | Fügt einen Token-Filter am Ende der Filterkette hinzu. | `src/tokenizer.rs` |
| `pub fn with_german_morphology( mut self, splitter: Arc<crate::morphology::GermanCompoundSplitter>, ) -> Self` | Aktiviert die deutsche Komposita-Zerlegung. | `src/tokenizer.rs` |
| `pub fn with_limits(mut self, limits: PipelineLimits) -> Self` | Konfiguriert die Ressourcen- und Sicherheitsgrenzen. | `src/tokenizer.rs` |
| `pub fn with_normalization(mut self, norm: NormalizationPolicy) -> Self` | Konfiguriert die Normalisierungsrichtlinie. | `src/tokenizer.rs` |
| `pub fn with_stopwords(mut self, stopwords: HashSet<String>) -> Self` | Konfiguriert die Stoppwortliste. | `src/tokenizer.rs` |
| `pub async fn block_max_wand_search<S: PostingStore>( storage: &S, terms: &[String], resident_index: &crate::posting_list::ResidentPostingIndex, prefix_helper: impl Fn(&str) -> Vec<u8>, tombstone_key_helper: impl Fn(DocId, &str) -> Vec<u8>, doc_len_key_helper:…` | Führt eine dynamisch geprunte Block-Max WAND Volltextsuche über den gegebenen Termen aus. | `src/wand.rs` |
| `pub fn into_sorted_vec(self) -> Vec<(DocId, f32)>` | Konvertiert den Heap in einen nach Relevanz absteigend sortierten Ergebnisvektor. | `src/wand.rs` |
| `pub fn min_threshold(&self) -> f32` | Liefert den aktuellen Minimal-Score (Schwelle), den ein Kandidat übertreffen muss. | `src/wand.rs` |
| `pub fn new(capacity: usize) -> Self` | Erzeugt einen neuen `BoundedTopK`-Puffer mit maximaler Kapazität (`MAX_SEARCH_K`). | `src/wand.rs` |
| `pub fn push(&mut self, doc_id: DocId, score: f32)` | Fügt einen neuen Kandidaten ein, wenn er besser als der schlechteste bisherige Treffer ist. | `src/wand.rs` |


---

## `acta-graph`

| Attribut | Wert |
|---|---|
| **Ring** | 1 — Ebene E1 — Speicherschicht, Indizes & KI-Kerne |
| **Status** | Fertig |
| **Zweck** | Compressed Sparse Row (CSR) Graph, Forward-Push PPR, Leiden und Hyperkanten. |
| **Unsafe-Code** | Nein (`#![forbid(unsafe_code)]`) |
| **Interne Abhängigkeiten** | `acta-env`, `acta-ids`, `acta-ports`, `acta-types` |
| **Externe Abhängigkeiten (Auswahl)** | `ahash`, `arc-swap`, `bincode`, `blake3`, `bytes`, `parking_lot`, `scc`, `serde`, `serde_json`, `thiserror`, `tracing` |

### Modul- und Re-Export-Oberfläche (lib.rs)

```rust
pub mod apprh;
pub mod arc_slice;
pub mod cascade;
pub mod community;
pub mod compat;
pub mod consistency_enforcement;
pub mod csr;
pub mod edge_reinforcement;
pub mod edge_reinforcement_buffer;
pub mod entity_extraction;
pub mod error;
pub mod hyperedge;
pub mod hyperedge_suggest;
pub mod path_rag;
pub mod percolation;
pub mod ppr;
pub mod ppr_stream;
pub mod provenance;
pub mod provider;
pub mod purger;
pub mod session_dag;
pub mod test_doubles;
pub mod types;
pub mod tl_hfd;
pub mod storage;
pub mod graph_view;
pub mod text_gen;
pub use apprh::{ apprh_local, forward_push_apprh, shadow_compare_forward_push_vs_apprh, ApprhError, ApprhFlipGate, ApprhMode, ApprhParams, ApprhSelector, ApprhSelectorConfig, ApprhShadowComparison, DefaultApprhFlipGate, };
pub use arc_slice::ArcSlice;
pub use cascade::{ cascade_invalidate_edges_for_superseded_doc, cascade_invalidate_hyperedges_for_superseded_doc, CascadeDeletionRecord, CascadeInvalidationReport, CascadeReport, CascadeStatus, CascadeTicket, GraphGarbageCollection, HyperedgeCascadeReport, MAX_HYPEREDGE_CASCADE_FANOUT, };
pub use community::{ detect_communities, CommunityAssignment, CommunityDetectionConfig, StarExpansionIterator, VirtualHyperedgeNode, };
pub use consistency_enforcement::{ ConflictPattern, ConsistencyEnforcer, ContradictionDetector, EdgeAssertion, EdgeId, ExactPredicateConflictDetector, };
pub use csr::CsrGraph;
pub use edge_reinforcement::{ apply_cooccurrence_reinforcement, apply_traversal_reinforcement, apply_weight_normalization, compute_edge_weight, EdgeReinforcementConfig, };
pub use edge_reinforcement_buffer::{CooccurrenceSignal, EdgeReinforcementBuffer, TraversalSignal};
pub use entity_extraction::{ ingest_extracted_relations, EntityExtractor, ExtractedRelation, RuleBasedEntityExtractor, };
pub use error::GraphError as Error;
pub use error::{GraphError, GraphMutationError, Result};
pub use hyperedge::{ sort_dedup_entities, star_weight, ConsolidationNodesGuard, HyperEdge, HyperEdgeId, HyperEdgeView, RoleBinding, RoleId, RoleInterner, HYPEREDGE_BY_ENTITY_PREFIX, HYPEREDGE_PREFIX, };
pub use hyperedge_suggest::{ compute_co_occurrence_candidates, validate_candidates_with_llm, HyperEdgeCandidate, HyperedgeSuggestError, ValidatedHyperEdgeCandidate, MAX_RELATE_PARTICIPANTS, };
pub use path_rag::{GraphPath, PathGraph, PathRAGConfig, PathRAGEngine};
pub use path_rag::{KPathConfig, KPathDiffusion, KPathResult};
pub use percolation::{ compute_percolation_health, find_rebonding_candidates, should_trigger_rebonding, PercolationConfig, };
pub use ppr::compute_ppr_with_apprh_selector;
pub use ppr::{compute_ppr, compute_ppr_at, DeletedView, PprContext};
pub use ppr_stream::{PprCandidateStream, DEFAULT_PPR_STREAM_BATCH_SIZE};
pub use provenance::{DocEdgeIndex, EdgeProvenance};
pub use provider::{ compute_pagerank, detect_communities_on_provider, GraphProvider, InMemoryGraphProvider, };
pub use purger::{graph_purger, GraphPurger};
pub use session_dag::{ AgentStateNode, DagEdge, NodeIdx, NodesGuard, NodesWriteGuard, SessionBranchTree, };
pub use tl_hfd::{ shadow_compare_forward_push_vs_tl_hfd, tl_hfd_local, DefaultFlipGate, ShadowComparison, ShadowDiscrepancyReport, TlHfdError, TlHfdFlipGate, TlHfdParams, MIN_AGREEMENT_THRESHOLD, MIN_SHADOW_SAMPLES, };
pub use storage::{GraphFuture, GraphStorage, InMemoryGraphStorage};
pub use graph_view::GraphView;
pub use text_gen::TextGenerator;
pub use types::{EntityId, PprAlgorithm, PprConfig};
```

### Feature-Flags

| Flag | Aktiviert |
|---|---|
| `workspace` | `["dep:acta-types", "dep:acta-ports", "types-compat", "ports-compat"]` |
| `session-dag` | `[]` |
| `entity-extraction` | `[]` |
| `hyperedge-suggest` | `[]` |
| `apprh-diffusion` | `[]` |
| `k-path-diffusion` | `[]` |
| `graph-connectivity-health` | `[]` |
| `edge-reinforcement-learning` | `[]` |
| `docid-128` | `[]` |
| `test-support` | `[]` |
| `types-compat` | `["dep:acta-types", "dep:acta-ports"]` |
| `ports-compat` | `["dep:acta-types", "dep:acta-ports"]` |
| `experimental-graph` | `["session-dag"]` |
| `physio-percolation` | `["graph-connectivity-health"]` |
| `ppr-forward-push` | `[]` |

### Schnittstellen — Public Traits (14)

**`ApprhFlipGate`** — ** (`src/apprh/shadow.rs`)

```rust
pub trait ApprhFlipGate {
/// Evaluates whether APPRH meets default flip criteria.
    fn should_flip(&self, report: &ApprhShadowComparison) -> bool;
}
```

**`ContradictionDetector`** — ** (`src/consistency_enforcement.rs`)

```rust
pub trait ContradictionDetector {
/// Prüft, ob zwei Aussagen im Widerspruch zueinander stehen.
    fn conflicts(&self, a: &EdgeAssertion, b: &EdgeAssertion) -> bool;
}
```

**`DefaultFlipGate`** — ** (`src/tl_hfd/shadow.rs`)

```rust
pub trait DefaultFlipGate {
/// Returns `true` iff:
    /// 1. `sample_count >= MIN_SHADOW_SAMPLES`
    /// 2. `mean_topk_jaccard >= MIN_AGREEMENT_THRESHOLD`
    /// 3. `p99_latency_delta_us <= 0.0`
    fn should_flip(&self, report: &ShadowDiscrepancyReport) -> bool {
        report.sample_count >= MIN_SHADOW_SAMPLES
            && report.mean_topk_jaccard >= MIN_AGREEMENT_THRESHOLD
            && report.p99_latency_delta_us <= 0.0
    }
}
```

**`EntityExtractor`** — ** (`src/entity_extraction.rs`)

```rust
pub trait EntityExtractor : Send + Sync {
/// Extrahiert Relationen aus dem gegebenen Freitext.
    fn extract(&self, text: &str) -> Vec<ExtractedRelation>;
}
```

**`GraphGarbageCollection`** — ** (`src/cascade.rs`)

```rust
pub trait GraphGarbageCollection {
/// Sweeps orphaned hyperedges whose participants no longer exist or are fully tombstoned.
    fn sweep_orphans(&self, wal_tx: TxId) -> Result<usize>;
}
```

**`GraphIndexExt`** — ** (`src/lib.rs`)

```rust
pub trait GraphIndexExt : acta_ports::GraphIndex {
/// Removes an entity node and all its incident (outgoing and incoming) edges from the graph.
    fn remove_entity<'a>(
        &'a self,
        tx: acta_types::TxId,
        entity: EntityId,
    ) -> acta_ports::BoxFuture<'a, acta_types::Result<()>>;
}
```

**`GraphIndexExt`** — ** (`src/compat/graph_index_ext.rs`)

```rust
pub trait GraphIndexExt : acta_ports::GraphIndex {
fn remove_entity<'a>(
        &'a self,
        tx: acta_types::TxId,
        entity: EntityId,
    ) -> acta_ports::BoxFuture<'a, acta_types::Result<()>>;
}
```

**`GraphProvider`** — ** (`src/compat/provider.rs`)

```rust
pub trait GraphProvider : Send + Sync {
/// Returns all active node IDs in the graph.
    fn node_ids(&self) -> Vec<EntityId>;
    /// Returns the number of active nodes.
    fn node_count(&self) -> usize {
        self.node_ids().len()
    }
    /// Checks if a node exists.
    fn contains_node(&self, node: EntityId) -> bool;
    /// Returns outgoing neighbors of `node` with their edge weights.
    fn outgoing_edges(&self, node: EntityId) -> Vec<(EntityId, f32)>;
    /// Returns incoming predecessors of `node` with their edge weights.
    fn incoming_edges(&self, node: EntityId) -> Vec<(EntityId, f32)>;
    /// Returns out-degree.
    fn out_degree(&self, node: EntityId) -> usize {
        self.outgoing_edges(node).len()
    }
    /// Returns in-degree.
    fn in_degree(&self, node: EntityId) -> usize {
        self.incoming_edges(node).len()
    }
    /// Returns hyperedge IDs for entity.
    fn hyperedges_for_entity(&self, _node: EntityId) -> Vec<HyperEdgeId> {
        Vec::new()
    }
    /// Resolves HyperEdgeId.
    fn get_hyperedge(&self, _id: HyperEdgeId) -> Option<Arc<HyperEdge>> {
        None
    }
    /// Returns role bindings.
    fn hyperedge_participants(&self, id: HyperEdgeId) -> Arc<[RoleBinding]> {
        self.get_hyperedge(id)
            .map(|he| he.participants.clone())
            .unwrap_or_else(|| Arc::from([]))
    }
}
```

**`GraphStorage`** — ** (`src/storage.rs`)

```rust
pub trait GraphStorage : Send + Sync + 'static {
fn get<'a>(&'a self, key: &'a [u8]) -> GraphFuture<'a, Result<Option<Bytes>>>;
    fn put<'a>(&'a self, tx: TxId, key: &'a [u8], value: &'a [u8]) -> GraphFuture<'a, Result<()>>;
    fn delete<'a>(&'a self, tx: TxId, key: &'a [u8]) -> GraphFuture<'a, Result<()>>;
    fn scan_prefix<'a>(&'a self, prefix: &'a [u8]) -> GraphFuture<'a, Result<Vec<(Vec<u8>, Vec<u8>)>>>;
    fn commit<'a>(&'a self, tx: TxId) -> GraphFuture<'a, Result<()>>;
    fn flush<'a>(&'a self) -> GraphFuture<'a, Result<()>>;
    fn last_tx_id<'a>(&'a self) -> GraphFuture<'a, Result<TxId>>;
}
```

**`GraphView`** — ** (`src/graph_view.rs`)

```rust
pub trait GraphView : Send + Sync {
fn neighbors<'a>(&'a self, entity: EntityId) -> GraphFuture<'a, Result<Vec<EntityId>>>;
    fn personalized_page_rank<'a>(
        &'a self,
        seeds: &'a [EntityId],
        config: &'a PprConfig,
    ) -> GraphFuture<'a, Result<Vec<(EntityId, f32)>>>;
}
```

**`KPathDiffusion`** — ** (`src/path_rag/k_path.rs`)

```rust
pub trait KPathDiffusion : Send + Sync {
/// Discovers up to `k` paths between `source` and `target` bounded by `config`.
    ///
    /// Budget exhaustion (`budget_exhausted: true`) is returned as a regular `Ok(KPathResult)`.
    fn find_k_paths(
        &self,
        source: EntityId,
        target: EntityId,
        config: &KPathConfig,
    ) -> acta_types::Result<KPathResult>;
}
```

**`PathGraph`** — ** (`src/path_rag/mod.rs`)

```rust
pub trait PathGraph : Send + Sync {
fn neighbors_with_weights(&self, node: EntityId) -> Vec<(EntityId, f32)>;
    fn predecessors_with_weights(&self, node: EntityId) -> Vec<(EntityId, f32)>;

    /// Returns hyperedge IDs associated with the given entity (default: empty).
    fn hyperedges_for_entity(&self, _node: EntityId) -> Vec<HyperEdgeId> {
        Vec::new()
    }

    /// Resolves a HyperEdgeId to its full HyperEdge details (default: None).
    fn get_hyperedge(&self, _id: HyperEdgeId) -> Option<Arc<HyperEdge>> {
        None
    }

    /// Returns participant role bindings for a given hyperedge ID (default: resolves via get_hyperedge).
    fn hyperedge_participants(&self, id: HyperEdgeId) -> Arc<[RoleBinding]> {
        match self.get_hyperedge(id) {
            Some(he) => he.participants.clone(),
            None => Arc::from([]),
        }
    }
}
```

**`ShadowSink`** — *Dyn-kompatibler Port für Diskrepanz-Senken.* (`src/ppr/shadow_hook.rs`)

```rust
pub trait ShadowSink<T> : Send + Sync {
/// Zeichnet einen Diskrepanz-Eintrag auf.
    fn record(&self, discrepancy: ShadowDiscrepancy<T>);
}
```

**`TextGenerator`** — ** (`src/text_gen.rs`)

```rust
pub trait TextGenerator : Send + Sync + 'static {
fn generate<'a>(&'a self, prompt: &'a str) -> GraphFuture<'a, Result<String>>;
}
```

### Öffentliche Datentypen — Structs (87)

| Struct | Felder / Form | Beschreibung | Datei |
|---|---|---|---|
| `AgentStateNode` | `{ pub step_id: NodeIdx, /// LLM prompt of this step. pub prompt: String, /// LLM response of this step. pub response: String, /// Optional MVCC snapshot referenc… }` | — | `src/session_dag.rs` |
| `ApprhGateMonitor` | `{ config: ApprhGateMonitorConfig, gate: Box<dyn ApprhFlipGate + Send + Sync>, state: Mutex<MonitorState>, }` | — | `src/apprh/gate_monitor.rs` |
| `ApprhGateMonitorConfig` | `{ /// Maximum capacity of the sliding window for historical observations. pub window_size: usize, /// Number of consecutive valid observations required for produ… }` | — | `src/apprh/gate_monitor.rs` |
| `ApprhGateMonitorSnapshot` | `{ /// Total number of observations received since creation or last reset. pub total_observations: u64, /// Current count of consecutive passed observations (stre… }` | — | `src/apprh/gate_monitor.rs` |
| `ApprhObservationRecord` | `{ /// Maximum absolute diff from report. pub max_abs_diff: f32, /// Top-k Jaccard overlap ratio from report. pub top_k_overlap: f32, /// Boolean indicator whethe… }` | — | `src/apprh/gate_monitor.rs` |
| `ApprhParams` | `{ /// Base PPR parameters ($\alpha$ teleport, $\epsilon$ convergence threshold). pub ppr: PprParams, /// Hyperedge decay factor ($\beta \in (0.0, 1.0]$, default:… }` | — | `src/apprh/params.rs` |
| `ApprhSelector` | `{ config: ApprhSelectorConfig, monitor: Arc<ApprhGateMonitor>, mode: RwLock<ApprhMode>, call_counter: AtomicU64, }` | — | `src/apprh/selector.rs` |
| `ApprhSelectorConfig` | `{ /// Sampling rate for shadow evaluations: 1 = every call, N = every N-th call. pub sample_interval: u64, /// Maximum number of seed nodes allowed for shadow ev… }` | — | `src/apprh/selector.rs` |
| `ApprhShadowComparison` | `{ /// Maximum absolute score difference across top-k union nodes. pub max_abs_diff: f32, /// Jaccard overlap ratio between top-k entity ID sets [0.0, 1.0]. pub t… }` | — | `src/apprh/shadow.rs` |
| `ArcSlice` | `{ shared: Arc<[T]>, offset: usize, len: usize, }` | — | `src/arc_slice.rs` |
| `CascadeDeletionRecord` | `{ pub doc_id: DocId, pub tombstoned_count: usize, pub wal_seq: u64, }` | — | `src/cascade.rs` |
| `CascadeInvalidationReport` | `{ /// Number of edges tombstoned as a result of the superseded document. pub tombstoned_edge_count: usize, /// Unique node (entity) IDs affected by the tombstone… }` | — | `src/cascade.rs` |
| `CascadeTicket` | `{ pub id: u64, pub doc_id: DocId, pub status: CascadeStatus, pub proof: Option<CascadeDeletionRecord>, }` | — | `src/cascade.rs` |
| `CommunityAssignment` | `{ /// Entity ID. pub entity_id: EntityId, /// Community ID (represented as a 64-bit integer, initialized from the seed EntityId). pub community_id: u64, /// Comm… }` | — | `src/community.rs` |
| `CommunityDetectionConfig` | `{ /// Maximum number of propagation / modularity optimization iterations. pub max_iterations: u32, /// Seed for deterministic node traversal shuffling. pub seed:… }` | — | `src/community.rs` |
| `ConflictPattern` | `{ /// Blake3-Hash über die normalisierte (subject, predicate, object)-Tripel-Signatur der widersprüchlichen Aussage. pub pattern_hash: [u8; 32], /// Anzahl der b… }` | — | `src/consistency_enforcement.rs` |
| `ConsistencyEnforcer` | `{ patterns: HashMap<[u8; 32], ConflictPattern>, suppression_threshold: u32, }` | — | `src/consistency_enforcement.rs` |
| `ConsolidationNodesGuard` | `{ entities: Vec<EntityId>, }` | — | `src/hyperedge.rs` |
| `CooccurrenceSignal` | `{ pub from: EntityId, pub to: EntityId, pub co_activation: f32, }` | — | `src/edge_reinforcement_buffer.rs` |
| `CsrEdge` | `{ pub target: EntityId, pub weight: f32, pub edge_type: EdgeType, #[cfg(feature = "edge-reinforcement-learning")] pub cooccurrence_weight: f32, // w_ij, initiali… }` | — | `src/csr/types.rs` |
| `CsrGraph` | `{ pub config: CsrGraphConfig, pub inner: Arc<ArcSwap<GraphInner>>, pub write_state: Arc<Mutex<GraphInner>>, #[cfg(feature = "ports-compat")] pub storage: Option<… }` | — | `src/csr/graph_write/mod.rs` |
| `CsrGraphConfig` | `{ pub rebuild_threshold: usize, pub max_compaction_peak_memory_mb: Option<usize>, pub resource_tracker: Option<Arc<ResourceTracker>>, }` | — | `src/csr/types.rs` |
| `DagEdge` | `{ pub parent: NodeIdx, pub child: NodeIdx, /// Label (e.g. "explore", "reject", "main") pub label: String, }` | — | `src/session_dag.rs` |
| `DefaultApprhFlipGate` | `(tuple/unit)` | — | `src/apprh/shadow.rs` |
| `DeletedView` | `{ deleted_nodes: HashSet<usize>, }` | — | `src/ppr/types.rs` |
| `DocEdgeIndex` | `{ index: RwLock<AHashMap<DocId, AHashSet<EdgeId>>>, }` | — | `src/provenance.rs` |
| `DocId` | `(tuple/unit)` | — | `src/types.rs` |
| `Edge` | `{ pub from: EntityId, pub to: EntityId, pub label: Arc<str>, pub weight: f32, #[serde(default, alias = "valid_from")] pub tx_valid_from: Option<TxId>, #[serde(de… }` | — | `src/types.rs` |
| `EdgeAssertion` | `{ /// Subjekt-Knoten der Aussage. pub subject: NodeIdx, /// Hash des Prädikats/Relationsnamens. pub predicate_hash: [u8; 32], /// Repräsentation des Objekts/Ziel… }` | — | `src/consistency_enforcement.rs` |
| `EdgeProvenance` | `{ /// Identifikator der Kante (from_entity_id, to_entity_id). pub edge_id: EdgeId, /// Dokumente deren Inhalt zu dieser Kante beigetragen hat. pub source_doc_ids… }` | — | `src/provenance.rs` |
| `EdgeReinforcementBuffer` | `{ cooccurrence: Mutex<Vec<CooccurrenceSignal>>, traversal: Mutex<Vec<TraversalSignal>>, }` | — | `src/edge_reinforcement_buffer.rs` |
| `EdgeReinforcementConfig` | `{ pub eta: f32, pub delta: f32, pub w_max: f32, pub rho: f32, pub q: f32, pub alpha: f32, }` | — | `src/edge_reinforcement.rs` |
| `EngineHandle` | `(tuple/unit)` | — | `src/compat/storage_bridge.rs` |
| `Entity` | `{ pub id: EntityId, pub name: Arc<str>, pub entity_type: Arc<str>, #[serde(default)] pub attributes: ahash::AHashMap<String, serde_json::Value>, }` | — | `src/types.rs` |
| `EntityId` | `(tuple/unit)` | — | `src/types.rs` |
| `ExactPredicateConflictDetector` | `(tuple/unit)` | — | `src/consistency_enforcement.rs` |
| `ExtractedRelation` | `{ /// Name / Identifikator der Quell-Entität (Subjekt). pub source: String, /// Name / Identifikator der Ziel-Entität (Objekt). pub target: String, /// Typ / Bez… }` | — | `src/entity_extraction.rs` |
| `FaultConfig` | `{ /// If true, all file operations fail. pub fail_all: bool, }` | Fault configuration for crash simulation. | `src/test_doubles.rs` |
| `FaultVfs` | `{ files: Arc<RwLock<BTreeMap<std::path::PathBuf, Vec<u8>>>>, failing: Arc<parking_lot::Mutex<bool>>, }` | — | `src/test_doubles.rs` |
| `GraphInner` | `{ pub(crate) id_map: HashMap<EntityId, InternalIndex>, pub(crate) reverse_map: Vec<EntityId>, pub(crate) entities: Vec<Entity>, pub(crate) communities: HashMap<E… }` | — | `src/csr/inner/mod.rs` |
| `GraphPath` | `{ /// Knoten-Sequenz vom Start zum Ziel. pub nodes: Vec<EntityId>, /// Kantengewichte entlang des Pfades (len = nodes.len() - 1). pub edge_weights: Vec<f32>, ///… }` | — | `src/path_rag/mod.rs` |
| `GraphPurger` | `{ graph: Arc<CsrGraph>, }` | — | `src/compat/purger.rs` |
| `GraphStats` | `{ /// Number of active/committed nodes \|V\|. pub node_count: usize, /// Number of directed edges \|E\|. pub edge_count: usize, /// Average outgoing degree \|E\| / \|V\|… }` | — | `src/ppr/cost.rs` |
| `HyperEdge` | `{ /// Eindeutige ID der Hyperkante. pub id: HyperEdgeId, /// Prädikat/Typ der Hyperkante. pub predicate: EdgeType, /// Liste der Teilnehmer mit ihren Rollen. ///… }` | — | `src/hyperedge.rs` |
| `HyperEdgeCandidate` | `{ /// Canonical sorted list of participating entity IDs. pub participants: Vec<EntityId>, /// Number of distinct documents in which this exact entity combination… }` | — | `src/hyperedge_suggest.rs` |
| `HyperEdgeId` | `(tuple/unit)` | — | `src/types.rs` |
| `HyperEdgeView` | `{ pub edge: &'a HyperEdge, pub participants: ArcSlice<RoleBinding>, }` | — | `src/hyperedge.rs` |
| `HyperedgeCascadeReport` | `{ /// Hyperedges tombstoned synchronously in this cascade run (up to MAX_HYPEREDGE_CASCADE_FANOUT). pub invalidated: Vec<crate::hyperedge::HyperEdgeId>, /// Hype… }` | — | `src/cascade.rs` |
| `InMemoryGraphProvider` | `{ nodes: BTreeSet<EntityId>, outgoing: BTreeMap<EntityId, Vec<(EntityId, f32)>>, incoming: BTreeMap<EntityId, Vec<(EntityId, f32)>>, hyperedges: BTreeMap<HyperEd… }` | Standalone in-memory reference implementation of `GraphProvider`. | `src/compat/provider.rs` |
| `InMemoryGraphStorage` | `{ map: Arc<RwLock<BTreeMap<Vec<u8>, Vec<u8>>>>, last_tx: Arc<AtomicU64>, }` | — | `src/storage.rs` |
| `InMemoryStorageEngine` | `{ data: Arc<RwLock<BTreeMap<Vec<u8>, Vec<u8>>>>, last_tx: Arc<AtomicU64>, }` | In-memory storage engine double implementing `acta_ports::StorageEngine`. | `src/test_doubles.rs` |
| `KPathConfig` | `{ /// Target number of paths to discover. pub k: usize, /// Maximum number of distinct graph nodes visited across forward/backward search. pub max_visited_nodes:… }` | — | `src/path_rag/k_path.rs` |
| `KPathResult` | `{ /// Up to `k` discovered paths between source and target. pub paths: Vec<Vec<EntityId>>, /// Indicates whether the search was halted due to `max_visited_nodes`… }` | — | `src/path_rag/k_path.rs` |
| `MemoryEstimate` | `{ pub shared_payload_bytes: usize, pub private_bytes: usize, }` | — | `src/csr/inner/estimate.rs` |
| `NodesGuard` | `{ tree: &'a SessionBranchTree, _nodes_guard: RwLockReadGuard<'a, HashMap<NodeIdx, AgentStateNode>>, }` | — | `src/session_dag.rs` |
| `NodesWriteGuard` | `{ tree: &'a SessionBranchTree, _nodes_guard: RwLockWriteGuard<'a, HashMap<NodeIdx, AgentStateNode>>, }` | — | `src/session_dag.rs` |
| `PathRAGConfig` | `{ /// Maximale Suchtiefe (Hop-Limit). pub max_hops: usize, /// Sufficiency-Schwelle: Pfade unter dieser Konfidenz werden gefiltert. pub sufficiency_threshold: f6… }` | — | `src/path_rag/mod.rs` |
| `PathRAGEngine` | `{ graph: G, pub config: PathRAGConfig, }` | — | `src/path_rag/mod.rs` |
| `PercolationConfig` | `{ /// Kritischer Perkolationsschwellenwert θ_perc. Default: 0.7. pub critical_threshold: f32, /// Similarity-Schwellenwert für Re-Bonding θ_bond. Default: 0.85.… }` | — | `src/percolation.rs` |
| `PersistedEdgePayload` | `{ pub weight: f32, #[serde(default, alias = "valid_from")] pub tx_valid_from: Option<TxId>, #[serde(default, alias = "valid_to")] pub tx_valid_to: Option<TxId>,… }` | — | `src/csr/types.rs` |
| `PprCandidateStream` | `{ graph: &'a G, seeds: Vec<EntityId>, config: PprConfig, batch_size: usize, exclude_seeds: bool, ranked: Option<VecDeque<(EntityId, f32)>>, yielded: usize, }` | — | `src/ppr_stream.rs` |
| `PprConfig` | `{ pub damping_factor: f32, pub max_iterations: u32, pub convergence_epsilon: f32, #[serde(default)] pub algorithm: PprAlgorithm, #[serde(default = "default_warn_… }` | — | `src/types.rs` |
| `PprContext` | `{ pub(crate) valid_seeds: Vec<usize>, pub(crate) out_weight_sums: Vec<f32>, pub(crate) ranks: Vec<f32>, pub(crate) next_ranks: Vec<f32>, }` | — | `src/ppr/types.rs` |
| `PprCostCalibrator` | `{ samples: Arc<Mutex<Vec<PprCostSample>>>, }` | — | `src/ppr/cost.rs` |
| `PprCostEstimate` | `{ /// Estimated edge accesses for Andersen-Chung-Lang ForwardPush: $O(1 / (\epsilon \cdot \alpha))$. pub forward_push: f64, /// Estimated edge accesses for Dense… }` | — | `src/ppr/cost.rs` |
| `PprCostSample` | `{ /// Algorithm evaluated. pub algorithm: PprAlgorithm, /// Estimated cost in edge access units. pub estimated_cost: f64, /// Measured edge accesses (counted ope… }` | — | `src/ppr/cost.rs` |
| `PprParams` | `{ /// Teleport probability ($\alpha$, default: 0.15). pub alpha: f32, /// Convergence error tolerance threshold ($\epsilon$, default: 1e-4). pub epsilon: f32, //… }` | — | `src/path_rag/mod.rs` |
| `ResourceBudget` | `{ pub memory_limit: u64, }` | — | `src/types.rs` |
| `ResourceTracker` | `{ pub budget: ResourceBudget, pub memory_used: AtomicU64, }` | — | `src/types.rs` |
| `RoleBinding` | `{ pub role: RoleId, pub entity: EntityId, }` | — | `src/types.rs` |
| `RoleId` | `(tuple/unit)` | — | `src/types.rs` |
| `RoleInterner` | `{ forward: HashMap<String, RoleId>, backward: HashMap<RoleId, String>, next_id: AtomicU32, }` | — | `src/hyperedge.rs` |
| `RuleBasedEntityExtractor` | `{ /// Minimale Konfidenzschwelle. Ergebnisse unterhalb dieser Schwelle werden nicht zurückgegeben. min_confidence: f32, /// Registrierte Extraktionsmuster. patte… }` | — | `src/entity_extraction.rs` |
| `SessionBranchTree` | `{ nodes: RwLock<HashMap<NodeIdx, AgentStateNode>>, edges: RwLock<Vec<DagEdge>>, active_head: RwLock<NodeIdx>, next_id: Arc<std::sync::atomic::AtomicU64>, }` | — | `src/session_dag.rs` |
| `SessionDagStepOptions` | `{ pub prompt: String, pub response: String, pub snapshot_tx_id: Option<TxId>, pub tool_outputs: Vec<String>, pub label: &'a str, }` | — | `src/session_dag.rs` |
| `ShadowComparison` | `{ /// Maximum absolute score difference across top-k union nodes. pub max_abs_diff: f32, /// Jaccard overlap ratio between top-k entity ID sets [0.0, 1.0]. pub t… }` | — | `src/tl_hfd/shadow.rs` |
| `ShadowDiscrepancy` | `{ /// Das Ergebnis des etablierten Baseline-Algorithmus. pub baseline: T, /// Das Ergebnis des neuen Kandidaten-Algorithmus. pub candidate: T, /// Eindeutige Kon… }` | — | `src/ppr/shadow_hook.rs` |
| `ShadowDiscrepancyReport` | `{ /// Number of evaluated query pairs (ForwardPush vs TL-HFD). pub sample_count: u64, /// Mean Jaccard similarity of top-k entity ID sets between algorithms. pub… }` | — | `src/tl_hfd/shadow.rs` |
| `SnapshotPathGraph` | `{ inner: &'a GraphInner, as_of_tx: TxId, }` | — | `src/path_rag/snapshot.rs` |
| `StarExpansionIterator` | `{ inner: arc_swap::Guard<std::sync::Arc<crate::csr::inner::GraphInner>>, hyperedge_keys: Vec<crate::hyperedge::HyperEdgeId>, current_hyperedge_idx: usize, curren… }` | — | `src/community.rs` |
| `TlHfdFlipGate` | `(tuple/unit)` | — | `src/tl_hfd/shadow.rs` |
| `TlHfdParams` | `{ #[serde(default = "default_tl_hfd_sigma")] pub sigma: f32, #[serde(default = "default_tl_hfd_delta")] pub delta: f32, #[serde(default = "default_tl_hfd_gamma")… }` | — | `src/types.rs` |
| `TlHfdParams` | `{ /// Regularization parameter $\sigma > 0$ controlling diffusion scale (default: 0.1). pub sigma: f32, /// Seed injection multiplier $\delta \ge 2.0$ (default:… }` | — | `src/tl_hfd/params.rs` |
| `TraversalSignal` | `{ pub from: EntityId, pub to: EntityId, pub path_length: usize, }` | — | `src/edge_reinforcement_buffer.rs` |
| `TxId` | `(tuple/unit)` | — | `src/types.rs` |
| `ValidatedHyperEdgeCandidate` | `{ /// The underlying co-occurrence candidate. pub candidate: HyperEdgeCandidate, /// Suggested semantic predicate string. pub predicate: String, /// LLM validati… }` | — | `src/hyperedge_suggest.rs` |
| `VirtualHyperedgeNode` | `{ /// ID der zugrundeliegenden Hyperkante. pub hyperedge_id: crate::hyperedge::HyperEdgeId, }` | — | `src/community.rs` |

### Öffentliche Datentypen — Enums (9)

| Enum | Varianten | Beschreibung | Datei |
|---|---|---|---|
| `ApprhError` | `/// Invalid parameter value provided to APPRH. #[error("Invalid parameter: {0}")] InvalidParameter(String), /// Empty seed list provided to APPRH. #[error("Seed list cannot be empty")] EmptySeeds, //…` | — | `src/apprh/error.rs` |
| `ApprhMode` | `/// APPRH disabled; authoritative Forward-Push PPR is returned. Off, /// APPRH shadow mode (default): returns authoritative Forward-Push PPR result, /// and periodically evaluates shadow comparisons…` | — | `src/apprh/selector.rs` |
| `CascadeStatus` | `/// Invalidation task is queued and pending. Pending, /// Invalidation task is currently processing. InFlight, /// Invalidation completed successfully with a deletion proof. Completed, /// Invalidati…` | — | `src/cascade.rs` |
| `EdgeType` | `#[default] Default,` | — | `src/csr/types.rs` |
| `GraphError` | `/// Invalid input parameter or argument. #[error("Invalid input: {0}")] InvalidInput(String), /// Requested node, edge, or hyperedge not found. #[error("Not found: {0}")] NotFound(String), /// Graph…` | — | `src/error.rs` |
| `GraphMutationError` | `/// Lock acquisition timed out. #[error("Lock acquisition timed out: {0}")] LockAcquisitionTimeout(String), /// Invalid role binding for hyperedge. #[error("Invalid role binding: {0}")] RoleBindingIn…` | — | `src/error.rs` |
| `HyperedgeSuggestError` | `/// Candidate count exceeds the maximum LLM calls allowed per cycle. #[error("LLM validation budget exceeded: {0} candidates exceed limit")] LlmValidationBudgetExceeded(usize),` | — | `src/hyperedge_suggest.rs` |
| `PprAlgorithm` | `#[default] Auto, DensePowerIteration, ForwardPush, ShadowMode, TlHfd(TlHfdParams), ShadowModeTlHfd(TlHfdParams),` | — | `src/types.rs` |
| `TlHfdError` | `/// Invalid parameter value provided to TL-HFD. #[error("Invalid parameter: {0}")] InvalidParameter(String), /// Non-finite value encountered during diffusion calculation. #[error("Non-finite value e…` | — | `src/tl_hfd/error.rs` |

### Typ-Aliase (6)

| Alias | Definition | Beschreibung |
|---|---|---|
| `CascadeReport` | `type CascadeReport= HyperedgeCascadeReport` | — |
| `EdgeId` | `type EdgeId= (EntityId, EntityId)` | — |
| `GraphFuture` | `type GraphFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>` | — |
| `NodeIdx` | `type NodeIdx= u64` | — |
| `NodeIdx` | `type NodeIdx= u64` | — |
| `Result` | `type Result<T, E = GraphError> = std::result::Result<T, E>` | — |

### Konstanten (18)

| Konstante | Typ | Beschreibung |
|---|---|---|
| `CASCADE_QUEUE_PREFIX` | `&str` | — |
| `DEFAULT_HYPEREDGE_CASCADE_FANOUT_LIMIT` | `usize` | — |
| `DEFAULT_PPR_STREAM_BATCH_SIZE` | `usize` | — |
| `DEFAULT_SUFFICIENCY_THRESHOLD` | `f64` | — |
| `DEFAULT_SUPPRESSION_THRESHOLD` | `u32` | — |
| `HYPEREDGE_BY_ENTITY_PREFIX` | `&str` | — |
| `HYPEREDGE_PREFIX` | `&str` | — |
| `MAX_DAG_STRING_BYTES` | `usize` | — |
| `MAX_DAG_TRAVERSAL_DEPTH` | `usize` | — |
| `MAX_HYPEREDGE_CASCADE_FANOUT` | `usize` | — |
| `MAX_RELATE_PARTICIPANTS` | `usize` | — |
| `MAX_SEARCH_K` | `usize` | — |
| `MAX_TRAVERSAL_HOPS` | `u8` | — |
| `MAX_VISITED_NODES` | `usize` | — |
| `MIN_AGREEMENT_THRESHOLD` | `f32` | — |
| `MIN_SHADOW_SAMPLES` | `u64` | — |
| `SCORE_DECAY` | `f32` | — |
| `WALLCLOCK_TX_HEURISTIC_MIN` | `u64` | — |

### Öffentliche Funktionen (262)

| Signatur | Beschreibung | Datei |
|---|---|---|
| `pub fn run_apprh_push<G: PathGraph>( graph: &G, seeds: &[EntityId], params: &ApprhParams, ) -> Result<AHashMap<EntityId, f32>, ApprhError>` | — | `src/apprh/diffusion.rs` |
| `pub fn config(&self) -> &ApprhGateMonitorConfig` | — | `src/apprh/gate_monitor.rs` |
| `pub fn new(config: ApprhGateMonitorConfig) -> Result<Self, ApprhError>` | — | `src/apprh/gate_monitor.rs` |
| `pub fn observe(&self, report: &ApprhShadowComparison)` | — | `src/apprh/gate_monitor.rs` |
| `pub fn production_ready(&self) -> bool` | — | `src/apprh/gate_monitor.rs` |
| `pub fn reset(&self)` | — | `src/apprh/gate_monitor.rs` |
| `pub fn snapshot(&self) -> ApprhGateMonitorSnapshot` | — | `src/apprh/gate_monitor.rs` |
| `pub fn validate(&self) -> Result<(), ApprhError>` | — | `src/apprh/gate_monitor.rs` |
| `pub fn with_gate<G>(config: ApprhGateMonitorConfig, gate: G) -> Result<Self, ApprhError> where G: ApprhFlipGate + Send + Sync + 'static,` | — | `src/apprh/gate_monitor.rs` |
| `pub fn apprh_diffusion( &self, seeds: &[EntityId], params: &ApprhParams, ) -> Result<AHashMap<EntityId, f32>, ApprhError>` | — | `src/apprh/mod.rs` |
| `pub fn apprh_local<G: PathGraph>( graph: &G, seeds: &[EntityId], params: &ApprhParams, ) -> Result<AHashMap<EntityId, f32>, ApprhError>` | — | `src/apprh/mod.rs` |
| `pub fn apprh_shadow_compare( &self, seeds: &[EntityId], ppr: &PprParams, apprh_params: &ApprhParams, top_k: usize, ) -> Result<ApprhShadowComparison, ApprhError>` | — | `src/apprh/mod.rs` |
| `pub fn forward_push_apprh<G: PathGraph>( graph: &G, seeds: &[EntityId], params: &PprParams, hyperedge_decay_factor: f32, ) -> Result<AHashMap<EntityId, f32>, acta_types::ActaError>` | — | `src/apprh/mod.rs` |
| `pub fn validate(&self) -> Result<(), ApprhError>` | — | `src/apprh/params.rs` |
| `pub fn demote_to_shadow(&self)` | — | `src/apprh/selector.rs` |
| `pub fn evaluate_and_run<G: PathGraph>( &self, graph: &G, seeds: &[EntityId], ppr_params: &PprParams, apprh_params: &ApprhParams, ) -> Result<Vec<(EntityId, f32)>, ApprhError>` | — | `src/apprh/selector.rs` |
| `pub fn mode(&self) -> ApprhMode` | — | `src/apprh/selector.rs` |
| `pub fn monitor(&self) -> &ApprhGateMonitor` | — | `src/apprh/selector.rs` |
| `pub fn new(config: ApprhSelectorConfig, monitor: Arc<ApprhGateMonitor>) -> Self` | — | `src/apprh/selector.rs` |
| `pub fn promote_to_production(&self) -> Result<(), ApprhError>` | — | `src/apprh/selector.rs` |
| `pub fn set_mode(&self, new_mode: ApprhMode) -> Result<(), ApprhError>` | — | `src/apprh/selector.rs` |
| `pub fn with_default_monitor(config: ApprhSelectorConfig) -> Result<Self, ApprhError>` | — | `src/apprh/selector.rs` |
| `pub fn shadow_compare_forward_push_vs_apprh<G: PathGraph>( graph: &G, seeds: &[EntityId], ppr: &PprParams, apprh_params: &ApprhParams, top_k: usize, ) -> Result<ApprhShadowComparison, ApprhError>` | — | `src/apprh/shadow.rs` |
| `pub fn as_arc(&self) -> &Arc<[T]>` | — | `src/arc_slice.rs` |
| `pub fn as_slice(&self) -> &[T]` | — | `src/arc_slice.rs` |
| `pub fn from_vec(vec: Vec<T>) -> Self` | — | `src/arc_slice.rs` |
| `pub fn is_empty(&self) -> bool` | — | `src/arc_slice.rs` |
| `pub fn len(&self) -> usize` | — | `src/arc_slice.rs` |
| `pub fn new(shared: Arc<[T]>) -> Self` | — | `src/arc_slice.rs` |
| `pub fn offset(&self) -> usize` | — | `src/arc_slice.rs` |
| `pub fn slice(&self, range: impl RangeBounds<usize>) -> Self` | — | `src/arc_slice.rs` |
| `pub fn try_slice(&self, range: impl RangeBounds<usize>) -> Option<Self>` | — | `src/arc_slice.rs` |
| `pub async fn cascade_invalidate_edges_for_superseded_doc( graph: &CsrGraph, superseded_doc_id: DocId, wal_seq: u64, ) -> Result<CascadeInvalidationReport>` | Tombstoniert alle Kanten, die von `superseded_doc_id` abgeleitet wurden, als Reaktion auf ein Supersedes-Ereignis. Idempotent: mehrfacher A… | `src/cascade.rs` |
| `pub async fn cascade_invalidate_hyperedges_for_superseded_doc( graph: &CsrGraph, superseded_doc_id: DocId, wal_seq: u64, ) -> Result<HyperedgeCascadeReport>` | Cascade invalidates hyperedges derived from `superseded_doc_id` with fan-out protection and recursive super-edge ancestor invalidation. Up… | `src/cascade.rs` |
| `pub fn topological_sort_hyperedge_closure( graph: &CsrGraph, closure_nodes: &[crate::hyperedge::HyperEdgeId], ) -> Vec<crate::hyperedge::HyperEdgeId>` | — | `src/cascade.rs` |
| `pub fn verify_doc_hyperedge_deletion_completeness( graph: &CsrGraph, target_doc_id: DocId, ) -> Result<()>` | — | `src/cascade.rs` |
| `pub async fn detect_communities( graph: &CsrGraph, config: &CommunityDetectionConfig, ) -> Result<Vec<CommunityAssignment>>` | Detects semantic communities in the given CSR graph using the deterministic Leiden algorithm. # Determinism and Reproducibility Guarantees… | `src/community.rs` |
| `pub fn new(hyperedge_id: u64) -> Self` | — | `src/community.rs` |
| `pub fn new(graph: &CsrGraph) -> Self` | — | `src/community.rs` |
| `pub fn next_with_weight(&mut self) -> Option<(EntityId, VirtualHyperedgeNode, f32)>` | — | `src/community.rs` |
| `pub fn add_edge(&mut self, source: EntityId, target: EntityId, weight: f32)` | — | `src/compat/provider.rs` |
| `pub fn add_hyperedge(&mut self, hyperedge: HyperEdge)` | — | `src/compat/provider.rs` |
| `pub fn add_node(&mut self, node: EntityId)` | — | `src/compat/provider.rs` |
| `pub fn compute_pagerank<G: GraphProvider>( graph: &G, damping: f32, max_iter: usize, tol: f32, ) -> HashMap<EntityId, f32>` | Generic PageRank computation executing against any `GraphProvider`. | `src/compat/provider.rs` |
| `pub fn detect_communities_on_provider<G: GraphProvider>( graph: &G, config: &crate::community::CommunityDetectionConfig, ) -> Result<Vec<crate::community::CommunityAssignment>>` | Generic Community Detection on any `GraphProvider`. | `src/compat/provider.rs` |
| `pub fn new() -> Self` | — | `src/compat/provider.rs` |
| `pub fn graph_purger(graph: Arc<CsrGraph>) -> Arc<dyn Purger>` | — | `src/compat/purger.rs` |
| `pub fn new(graph: Arc<CsrGraph>) -> Self` | Erzeugt einen neuen [`GraphPurger`]. | `src/compat/purger.rs` |
| `pub fn active_patterns(&self) -> impl Iterator<Item = &ConflictPattern>` | — | `src/consistency_enforcement.rs` |
| `pub fn check_before_insert(&mut self, assertion: &EdgeAssertion) -> Option<ConflictPattern>` | — | `src/consistency_enforcement.rs` |
| `pub fn detect_contradiction<D: ContradictionDetector>( &self, detector: &D, a: &EdgeAssertion, b: &EdgeAssertion, ) -> bool` | — | `src/consistency_enforcement.rs` |
| `pub fn get_pattern(&self, pattern_hash: &[u8; 32]) -> Option<&ConflictPattern>` | — | `src/consistency_enforcement.rs` |
| `pub fn is_suppressed(&self, pattern_hash: [u8; 32]) -> bool` | — | `src/consistency_enforcement.rs` |
| `pub fn new(suppression_threshold: u32) -> Self` | — | `src/consistency_enforcement.rs` |
| `pub fn pattern_hash(&self) -> [u8` | — | `src/consistency_enforcement.rs` |
| `pub fn record_contradiction( &mut self, pattern_hash: [u8; 32], at_tx: TxId, ) -> &ConflictPattern` | — | `src/consistency_enforcement.rs` |
| `pub fn suggest_tombstone_candidates( &self, csr_edges_matching_pattern: &[EdgeId], ) -> Vec<EdgeId>` | — | `src/consistency_enforcement.rs` |
| `pub fn suppression_threshold(&self) -> u32` | — | `src/consistency_enforcement.rs` |
| `pub async fn delete_edge_persistence<S: StorageEngine + ?Sized>( &self, storage: &S, tx: TxId, from: &EntityId, to: &EntityId, ) -> Result<()>` | Löscht eine einzelne Edge aus dem übergebenen Storage. | `src/csr/graph_persist.rs` |
| `pub async fn load_from_storage<S: StorageEngine + ?Sized>(storage: &S) -> Result<Self>` | Lädt den kompletten Graph-Zustand aus dem Storage (beim Startup). | `src/csr/graph_persist.rs` |
| `pub async fn persist_edge<S: StorageEngine + ?Sized>( &self, storage: &S, tx: TxId, from: &EntityId, to: &EntityId, payload: &PersistedEdgePayload, ) -> Result<()>` | Persistiert eine einzelne Edge in den übergebenen Storage. | `src/csr/graph_persist.rs` |
| `pub async fn persist_entity<S: StorageEngine + ?Sized>( &self, storage: &S, tx: TxId, entity: &Entity, ) -> Result<()>` | Persistiert eine einzelne Entity in den übergebenen Storage. | `src/csr/graph_persist.rs` |
| `pub async fn get_communities_batch( &self, entity_ids: &[EntityId], ) -> Result<HashMap<EntityId, u64>>` | — | `src/csr/graph_read/communities.rs` |
| `pub fn set_communities_batch(&self, assignments: &[crate::CommunityAssignment])` | — | `src/csr/graph_read/communities.rs` |
| `pub fn compact(&self)` | — | `src/csr/graph_read/mod.rs` |
| `pub async fn compact_async(&self) -> Result<()>` | — | `src/csr/graph_read/mod.rs` |
| `pub fn edge_count(&self) -> usize` | — | `src/csr/graph_read/mod.rs` |
| `pub fn entity_count(&self) -> usize` | — | `src/csr/graph_read/mod.rs` |
| `pub fn entity_exists(&self, id: EntityId) -> bool` | — | `src/csr/graph_read/mod.rs` |
| `pub async fn is_entity_deleted(&self, entity: EntityId) -> bool` | — | `src/csr/graph_read/mod.rs` |
| `pub async fn neighbors(&self, start: EntityId) -> Result<Vec<EntityId>>` | — | `src/csr/graph_read/mod.rs` |
| `pub async fn remove_entity(&self, tx: TxId, entity: EntityId) -> Result<()>` | — | `src/csr/graph_read/mod.rs` |
| `pub async fn deleted_view(&self) -> crate::DeletedView` | — | `src/csr/graph_read/pagerank.rs` |
| `pub async fn get_deleted_node_indices(&self) -> HashSet<usize>` | — | `src/csr/graph_read/pagerank.rs` |
| `pub async fn pagerank( &self, damping_factor: f32, max_iterations: usize, tolerance: f32, ) -> HashMap<EntityId, f32>` | — | `src/csr/graph_read/pagerank.rs` |
| `pub async fn personalized_page_rank_with_context_async( &self, seed_nodes: &[EntityId], config: &acta_types::PprConfig, ctx: &mut crate::PprContext, ) -> Vec<(EntityId, f32)>` | — | `src/csr/graph_read/pagerank.rs` |
| `pub fn path_rag_at<'a>( &'a self, start_nodes: &'a [EntityId], max_hops: usize, seq_no: u64, ) -> acta_ports::BoxFuture<'a, Result<Vec<(EntityId, f32)>>>` | — | `src/csr/graph_read/path_rag.rs` |
| `pub async fn add_edge( self: &Arc<Self>, from: EntityId, to: EntityId, weight: f32, tx_valid_from: Option<TxId>, tx_valid_to: Option<TxId>, business_valid_from: Option<i64>, business_valid_to: Option<i64>, source_doc_id: Option<DocId>, predicate_hash: Option<…` | — | `src/csr/graph_write/edge_mutation.rs` |
| `pub async fn insert_edge_direct( self: &Arc<Self>, from: EntityId, to: EntityId, weight: f32, ) -> Result<()>` | — | `src/csr/graph_write/edge_mutation.rs` |
| `pub async fn insert_edge_direct_with_bitemporal_validity( self: &Arc<Self>, from: EntityId, to: EntityId, weight: f32, tx_valid_from: Option<TxId>, tx_valid_to: Option<TxId>, business_valid_from: Option<i64>, business_valid_to: Option<i64>, source_doc_id: Opt…` | — | `src/csr/graph_write/edge_mutation.rs` |
| `pub async fn insert_edge_direct_with_validity( self: &Arc<Self>, from: EntityId, to: EntityId, weight: f32, tx_valid_from: Option<TxId>, tx_valid_to: Option<TxId>, ) -> Result<()>` | — | `src/csr/graph_write/edge_mutation.rs` |
| `pub fn load_edge_direct( &self, from: EntityId, to: EntityId, weight: f32, tx_valid_from: Option<TxId>, tx_valid_to: Option<TxId>, business_valid_from: Option<i64>, business_valid_to: Option<i64>, source_doc_id: Option<DocId>, ) -> Result<()>` | — | `src/csr/graph_write/edge_mutation.rs` |
| `pub fn insert_entity_direct(&self, entity: Entity) -> Result<()>` | — | `src/csr/graph_write/entity.rs` |
| `pub fn load_entity_direct(&self, entity: Entity) -> Result<()>` | — | `src/csr/graph_write/entity.rs` |
| `pub fn commit_super_edge_batch( &self, tombstone_ids: &[crate::hyperedge::HyperEdgeId], new_edges: Vec<crate::hyperedge::HyperEdge>, wal_tx: TxId, ) -> std::result::Result<(), GraphMutationError>` | — | `src/csr/graph_write/hyperedge.rs` |
| `pub fn enqueue_cascade_deferred( &self, doc_id: DocId, hyperedge_ids: &[crate::hyperedge::HyperEdgeId], )` | — | `src/csr/graph_write/hyperedge.rs` |
| `pub fn get_hyperedge( &self, id: crate::hyperedge::HyperEdgeId, ) -> Option<Arc<crate::hyperedge::HyperEdge>>` | — | `src/csr/graph_write/hyperedge.rs` |
| `pub fn hyperedges_for_doc(&self, doc_id: DocId) -> Vec<crate::hyperedge::HyperEdgeId>` | — | `src/csr/graph_write/hyperedge.rs` |
| `pub fn hyperedges_for_entity(&self, entity_id: EntityId) -> Vec<crate::hyperedge::HyperEdgeId>` | — | `src/csr/graph_write/hyperedge.rs` |
| `pub fn insert_hyperedge(&self, edge: crate::hyperedge::HyperEdge)` | — | `src/csr/graph_write/hyperedge.rs` |
| `pub fn insert_hyperedge_direct(&self, hyperedge: crate::hyperedge::HyperEdge)` | — | `src/csr/graph_write/hyperedge.rs` |
| `pub fn max_hyperedge_id(&self) -> u64` | — | `src/csr/graph_write/hyperedge.rs` |
| `pub fn parent_hyperedges_of( &self, child: crate::hyperedge::HyperEdgeId, ) -> Vec<crate::hyperedge::HyperEdgeId>` | — | `src/csr/graph_write/hyperedge.rs` |
| `pub fn pending_cascade_queue_len(&self) -> usize` | — | `src/csr/graph_write/hyperedge.rs` |
| `pub async fn persist_hyperedge( &self, tx: TxId, hyperedge: &crate::hyperedge::HyperEdge, ) -> Result<()>` | — | `src/csr/graph_write/hyperedge.rs` |
| `pub async fn process_cascade_queue(&self, batch_size: usize, wal_tx: TxId) -> Result<usize>` | — | `src/csr/graph_write/hyperedge.rs` |
| `pub fn relate_n_ary( &self, id: crate::hyperedge::HyperEdgeId, predicate: crate::csr::EdgeType, participants: Vec<crate::hyperedge::RoleBinding>, weight: f32, doc_id: Option<DocId>, ) -> std::result::Result<crate::hyperedge::HyperEdge, GraphMutationError>` | — | `src/csr/graph_write/hyperedge.rs` |
| `pub fn edges_for_doc(&self, doc_id: DocId) -> Vec<(EntityId, EntityId)>` | — | `src/csr/graph_write/mod.rs` |
| `pub fn estimate_compaction_peak_bytes(&self) -> usize` | — | `src/csr/graph_write/mod.rs` |
| `pub fn estimate_memory(&self, presized: bool) -> MemoryEstimate` | — | `src/csr/graph_write/mod.rs` |
| `pub fn estimate_memory_bytes(&self) -> usize` | — | `src/csr/graph_write/mod.rs` |
| `pub fn get_source_doc_id(&self, index: usize) -> Option<DocId>` | — | `src/csr/graph_write/mod.rs` |
| `pub fn inner_read(&self) -> arc_swap::Guard<Arc<GraphInner>>` | — | `src/csr/graph_write/mod.rs` |
| `pub fn new() -> Self` | — | `src/csr/graph_write/mod.rs` |
| `pub fn personalized_page_rank_at<'a>( &'a self, seed_nodes: &'a [acta_types::EntityId], config: &'a acta_types::PprConfig, seq_no: u64, ) -> acta_ports::BoxFuture< 'a, acta_types::Result<Vec<(acta_types::EntityId, f32)>>, >` | Delegiert an [`GraphIndex::personalized_page_rank_at`]. | `src/csr/graph_write/mod.rs` |
| `pub fn rollback_to_tx<'a>( &'a self, tx_id: acta_types::TxId, ) -> acta_ports::BoxFuture<'a, acta_types::Result<()>>` | Delegiert an [`GraphIndex::rollback_to_tx`]. | `src/csr/graph_write/mod.rs` |
| `pub fn set_storage(&mut self, storage: Arc<dyn StorageEngine>)` | — | `src/csr/graph_write/mod.rs` |
| `pub fn source_doc_id_at(&self, from: EntityId, to: EntityId) -> Option<DocId>` | — | `src/csr/graph_write/mod.rs` |
| `pub fn storage(&self) -> Option<Arc<dyn StorageEngine>>` | — | `src/csr/graph_write/mod.rs` |
| `pub fn with_config(config: CsrGraphConfig) -> Self` | — | `src/csr/graph_write/mod.rs` |
| `pub fn with_config_and_storage( config: CsrGraphConfig, storage: Arc<dyn StorageEngine>, ) -> Self` | — | `src/csr/graph_write/mod.rs` |
| `pub fn with_consistency_enforcer(suppression_threshold: u32) -> Self` | — | `src/csr/graph_write/mod.rs` |
| `pub fn with_storage(storage: Arc<dyn StorageEngine>) -> Self` | — | `src/csr/graph_write/mod.rs` |
| `pub async fn tombstone_edge( &self, edge_id: crate::consistency_enforcement::EdgeId, tx: TxId, ) -> Result<()>` | — | `src/csr/graph_write/tombstone.rs` |
| `pub fn tombstone_hyperedge(&self, id: crate::hyperedge::HyperEdgeId, wal_tx: TxId) -> bool` | — | `src/csr/graph_write/tombstone.rs` |
| `pub fn compact(&mut self)` | — | `src/csr/inner/compaction.rs` |
| `pub fn get_or_create_index(&mut self, id: EntityId) -> InternalIndex` | — | `src/csr/inner/compaction.rs` |
| `pub fn recompute_node_out_weight_sum(&mut self, node_idx: usize)` | — | `src/csr/inner/compaction.rs` |
| `pub fn total_bytes(&self) -> usize` | — | `src/csr/inner/estimate.rs` |
| `pub fn new() -> Self` | — | `src/csr/inner/mod.rs` |
| `pub fn new(target: EntityId, weight: f32) -> Self` | — | `src/csr/types.rs` |
| `pub fn is_edge_visible( tx_valid_from: Option<TxId>, tx_valid_to: Option<TxId>, as_of: TxId, ) -> bool` | — | `src/csr/visibility.rs` |
| `pub fn is_edge_visible_bitemporal( tx_valid_from: Option<TxId>, tx_valid_to: Option<TxId>, as_of_tx: TxId, business_valid_from: Option<i64>, business_valid_to: Option<i64>, as_of_business: Option<i64>, ) -> bool` | — | `src/csr/visibility.rs` |
| `pub fn is_edge_visible_business( business_valid_from: Option<i64>, business_valid_to: Option<i64>, business_as_of: i64, ) -> bool` | — | `src/csr/visibility.rs` |
| `pub fn is_suspicious_tx_id(tx: TxId) -> bool` | — | `src/csr/visibility.rs` |
| `pub fn cooccurrence_count(&self) -> usize` | — | `src/edge_reinforcement_buffer.rs` |
| `pub fn flush_to_graph(&self, graph: &CsrGraph, config: &EdgeReinforcementConfig)` | — | `src/edge_reinforcement_buffer.rs` |
| `pub fn new() -> Self` | — | `src/edge_reinforcement_buffer.rs` |
| `pub fn push_cooccurrence(&self, from: EntityId, to: EntityId, co_activation: f32)` | — | `src/edge_reinforcement_buffer.rs` |
| `pub fn push_traversal(&self, from: EntityId, to: EntityId, path_length: usize)` | — | `src/edge_reinforcement_buffer.rs` |
| `pub fn traversal_count(&self) -> usize` | — | `src/edge_reinforcement_buffer.rs` |
| `pub fn ingest_extracted_relations( graph: &CsrGraph, relations: &[ExtractedRelation], min_confidence_threshold: f32, doc_id: Option<DocId>, ) -> Result<Vec<HyperEdge>>` | — | `src/entity_extraction.rs` |
| `pub fn new( source: impl Into<String>, target: impl Into<String>, relation_type: impl Into<String>, confidence: f32, ) -> Self` | — | `src/entity_extraction.rs` |
| `pub fn new(min_confidence: f32) -> Self` | — | `src/entity_extraction.rs` |
| `pub fn acquire(entities: &[EntityId]) -> Self` | — | `src/hyperedge.rs` |
| `pub fn contains_id(&self, id: RoleId) -> bool` | — | `src/hyperedge.rs` |
| `pub fn contains_role(&self, name: &str) -> bool` | — | `src/hyperedge.rs` |
| `pub fn deserialize(bytes: &[u8]) -> Result<Self>` | — | `src/hyperedge.rs` |
| `pub fn entities(&self) -> &[EntityId]` | — | `src/hyperedge.rs` |
| `pub fn get_or_intern(&self, name: &str) -> RoleId` | — | `src/hyperedge.rs` |
| `pub fn is_empty(&self) -> bool` | — | `src/hyperedge.rs` |
| `pub fn len(&self) -> usize` | — | `src/hyperedge.rs` |
| `pub fn new( id: HyperEdgeId, predicate: EdgeType, participants: impl Into<Arc<[RoleBinding]>>, weight: f32, ) -> Self` | — | `src/hyperedge.rs` |
| `pub fn new(edge: &'a HyperEdge) -> Self` | — | `src/hyperedge.rs` |
| `pub fn new() -> Self` | — | `src/hyperedge.rs` |
| `pub fn resolve<F, R>(&self, id: RoleId, f: F) -> Option<R> where F: FnOnce(&str) -> R,` | — | `src/hyperedge.rs` |
| `pub fn resolve_string(&self, id: RoleId) -> Option<String>` | — | `src/hyperedge.rs` |
| `pub fn serialize(&self) -> Result<Vec<u8>>` | — | `src/hyperedge.rs` |
| `pub fn slice_participants( &self, range: impl std::ops::RangeBounds<usize>, ) -> ArcSlice<RoleBinding>` | — | `src/hyperedge.rs` |
| `pub fn sort_dedup_entities(entities: &[EntityId]) -> Vec<EntityId>` | — | `src/hyperedge.rs` |
| `pub fn star_weight(weight: f32, cardinality: usize) -> f32` | — | `src/hyperedge.rs` |
| `pub fn try_acquire(entities: &[EntityId]) -> std::result::Result<Self, GraphMutationError>` | — | `src/hyperedge.rs` |
| `pub fn validate(&self) -> std::result::Result<(), GraphMutationError>` | — | `src/hyperedge.rs` |
| `pub fn view(&self) -> HyperEdgeView<'_>` | — | `src/hyperedge.rs` |
| `pub fn with_business_validity( mut self, valid_from: Option<i64>, valid_to: Option<i64>, ) -> Self` | — | `src/hyperedge.rs` |
| `pub fn with_child_edge_ids(mut self, ids: impl Into<Arc<[HyperEdgeId]>>) -> Self` | — | `src/hyperedge.rs` |
| `pub fn with_entities<F, R>(&self, f: F) -> R where F: FnOnce(&[EntityId]) -> R,` | — | `src/hyperedge.rs` |
| `pub fn with_source_doc_id(mut self, source_doc_id: Option<DocId>) -> Self` | — | `src/hyperedge.rs` |
| `pub fn with_tx_validity(mut self, valid_from: Option<TxId>, valid_to: Option<TxId>) -> Self` | — | `src/hyperedge.rs` |
| `pub fn compute_co_occurrence_candidates( entities_per_document: &[(DocId, Vec<EntityId>)], min_co_occurrence: usize, ) -> Vec<HyperEdgeCandidate>` | — | `src/hyperedge_suggest.rs` |
| `pub async fn validate_candidates_with_llm( candidates: &[HyperEdgeCandidate], entity_labels: &dyn Fn(EntityId) -> Option<String>, generator: &dyn LlmTextGenerator, max_llm_calls_per_cycle: usize, ) -> Result<Vec<ValidatedHyperEdgeCandidate>, GraphMutationErro…` | Validates hyperedge candidates using an [`LlmTextGenerator`]. For each candidate (up to `max_llm_calls_per_cycle`), queries the LLM to conf… | `src/hyperedge_suggest.rs` |
| `pub fn find_all_paths(&self, source: EntityId) -> Vec<GraphPath>` | — | `src/path_rag/mod.rs` |
| `pub fn find_path(&self, source: EntityId, target: EntityId) -> Option<GraphPath>` | — | `src/path_rag/mod.rs` |
| `pub fn forward_push_ppr<G: PathGraph>( graph: &G, seeds: &[EntityId], params: &PprParams, ) -> AHashMap<EntityId, f32>` | — | `src/path_rag/mod.rs` |
| `pub fn max_hops(&self) -> usize` | — | `src/path_rag/mod.rs` |
| `pub fn new(graph: G, max_hops: usize, sufficiency_threshold: f64) -> Self` | — | `src/path_rag/mod.rs` |
| `pub fn sufficiency_check(&self, path: &GraphPath) -> bool` | — | `src/path_rag/mod.rs` |
| `pub fn sufficiency_threshold(&self) -> f64` | — | `src/path_rag/mod.rs` |
| `pub fn to_rrf_signal(&self, paths: &[GraphPath]) -> Vec<(DocId, f32)>` | — | `src/path_rag/mod.rs` |
| `pub fn with_config(graph: G, config: PathRAGConfig) -> Self` | — | `src/path_rag/mod.rs` |
| `pub fn with_defaults(graph: G) -> Self` | — | `src/path_rag/mod.rs` |
| `pub fn new(inner: &'a GraphInner, as_of_tx: TxId) -> Self` | — | `src/path_rag/snapshot.rs` |
| `pub fn compute_percolation_health( active_node_count: usize, active_edge_count: usize, // nach Tombstone-Abzug ) -> Option<f32>` | — | `src/percolation.rs` |
| `pub async fn find_rebonding_candidates<G: acta_ports::GraphIndex>( graph: &G, embeddings: &HashMap<EntityId, Vec<f32>>, config: &PercolationConfig, ) -> Vec<(EntityId, EntityId, f32)>` | Findet Kandidaten-Paare für Re-Bonding. Gibt `(EntityId, EntityId, similarity)`-Tupel zurück, sortiert absteigend nach Similarity. | `src/percolation.rs` |
| `pub fn should_trigger_rebonding(health: f32, config: &PercolationConfig) -> bool` | — | `src/percolation.rs` |
| `pub fn cheapest_strategy(&self) -> PprAlgorithm` | — | `src/ppr/cost.rs` |
| `pub fn clear(&self)` | — | `src/ppr/cost.rs` |
| `pub fn estimate_ppr_cost( stats: Option<&GraphStats>, seed_count: usize, config: &PprConfig, ) -> (PprCostEstimate, PprAlgorithm)` | — | `src/ppr/cost.rs` |
| `pub fn new(node_count: usize, edge_count: usize) -> Self` | — | `src/ppr/cost.rs` |
| `pub fn new() -> Self` | — | `src/ppr/cost.rs` |
| `pub fn recommended_multiplier(&self) -> Option<f64>` | — | `src/ppr/cost.rs` |
| `pub fn record(&self, sample: PprCostSample)` | — | `src/ppr/cost.rs` |
| `pub fn sample_count(&self) -> usize` | — | `src/ppr/cost.rs` |
| `pub fn compute_ppr( inner: &GraphInner, seed_nodes: &[EntityId], config: &PprConfig, deleted_nodes: &DeletedView, ) -> Vec<(EntityId, f32)>` | — | `src/ppr/mod.rs` |
| `pub fn compute_ppr_with_apprh_selector( inner: &GraphInner, seed_nodes: &[EntityId], config: &PprConfig, deleted_nodes: &DeletedView, ctx: &mut PprContext, selector: &crate::apprh::ApprhSelector, apprh_params: &crate::apprh::ApprhParams, ) -> Vec<(EntityId, f…` | — | `src/ppr/mod.rs` |
| `pub fn log_tl_hfd_vs_baseline_discrepancy( baseline: &AHashMap<EntityId, f32>, candidate: &AHashMap<EntityId, f32>, context_id: u64, sink: &dyn ShadowSink<f64>, )` | — | `src/ppr/shadow_hook.rs` |
| `pub fn compute_ppr_at( inner: &GraphInner, seed_nodes: &[EntityId], config: &PprConfig, deleted_nodes: &DeletedView, as_of_tx: TxId, ) -> Vec<(EntityId, f32)>` | — | `src/ppr/snapshot.rs` |
| `pub fn contains(&self, idx: usize) -> bool` | — | `src/ppr/types.rs` |
| `pub fn is_empty(&self) -> bool` | — | `src/ppr/types.rs` |
| `pub fn len(&self) -> usize` | — | `src/ppr/types.rs` |
| `pub fn new() -> Self` | — | `src/ppr/types.rs` |
| `pub fn is_exhausted(&self) -> bool` | — | `src/ppr_stream.rs` |
| `pub fn new(graph: &'a G, seeds: impl Into<Vec<EntityId>>, config: PprConfig) -> Self` | — | `src/ppr_stream.rs` |
| `pub async fn next_batch(&mut self) -> Result<Vec<(EntityId, f32)>>` | Fetches the next batch of candidate entity-score pairs (up to `batch_size`). Returns an empty `Vec` when exhausted. | `src/ppr_stream.rs` |
| `pub async fn next_batch_docs<F>(&mut self, mut resolve: F) -> Result<Vec<DocId>> where F: FnMut(EntityId) -> Option<DocId>,` | Fetches the next batch of resolved [`DocId`]s (up to `batch_size`). Entities for which `resolve` returns `None` are skipped. | `src/ppr_stream.rs` |
| `pub async fn next_entity(&mut self) -> Result<Option<EntityId>>` | Fetches the next single candidate entity ID. Returns `Ok(None)` when exhausted. | `src/ppr_stream.rs` |
| `pub fn with_batch_size(mut self, n: usize) -> Self` | — | `src/ppr_stream.rs` |
| `pub fn with_exclude_seeds(mut self, exclude: bool) -> Self` | — | `src/ppr_stream.rs` |
| `pub fn yielded(&self) -> usize` | — | `src/ppr_stream.rs` |
| `pub fn edges_for_doc(&self, doc_id: DocId) -> Vec<EdgeId>` | — | `src/provenance.rs` |
| `pub fn new(edge_id: EdgeId, source_doc_ids: Vec<DocId>, created_at_tx: TxId) -> Self` | — | `src/provenance.rs` |
| `pub fn new() -> Self` | — | `src/provenance.rs` |
| `pub fn record(&self, doc_id: DocId, edge_id: EdgeId)` | — | `src/provenance.rs` |
| `pub fn record_provenance(&self, provenance: &EdgeProvenance)` | — | `src/provenance.rs` |
| `pub fn remove_doc(&self, doc_id: DocId)` | — | `src/provenance.rs` |
| `pub fn active_head(&self) -> RwLockReadGuard<'_, NodeIdx>` | — | `src/session_dag.rs` |
| `pub fn active_head(&self) -> RwLockReadGuard<'_, NodeIdx>` | — | `src/session_dag.rs` |
| `pub fn active_head(&self) -> NodeIdx` | — | `src/session_dag.rs` |
| `pub fn active_head_write(&self) -> RwLockWriteGuard<'_, NodeIdx>` | — | `src/session_dag.rs` |
| `pub fn active_head_write(&self) -> RwLockWriteGuard<'_, NodeIdx>` | — | `src/session_dag.rs` |
| `pub fn append_step( &self, prompt: String, response: String, snapshot_tx_id: Option<TxId>, tool_outputs: Vec<String>, label: &str, ) -> Result<NodeIdx>` | — | `src/session_dag.rs` |
| `pub fn branch_from( &self, parent_node: NodeIdx, prompt: String, response: String, snapshot_tx_id: Option<TxId>, tool_outputs: Vec<String>, label: &str, ) -> Result<NodeIdx>` | — | `src/session_dag.rs` |
| `pub fn children_of(&self, node_idx: NodeIdx) -> Vec<NodeIdx>` | — | `src/session_dag.rs` |
| `pub fn edges(&self) -> RwLockReadGuard<'_, Vec<DagEdge>>` | — | `src/session_dag.rs` |
| `pub fn edges(&self) -> RwLockReadGuard<'_, Vec<DagEdge>>` | — | `src/session_dag.rs` |
| `pub fn edges_write(&self) -> RwLockWriteGuard<'_, Vec<DagEdge>>` | — | `src/session_dag.rs` |
| `pub fn edges_write(&self) -> RwLockWriteGuard<'_, Vec<DagEdge>>` | — | `src/session_dag.rs` |
| `pub fn get_node(&self, node_idx: NodeIdx) -> Option<AgentStateNode>` | — | `src/session_dag.rs` |
| `pub async fn load<S: StorageEngine + ?Sized>(storage: &S, namespace: &str) -> Result<Self>` | Loads and reconstructs a `SessionBranchTree` from storage for the specified namespace. | `src/session_dag.rs` |
| `pub fn lock_nodes(&self) -> NodesGuard<'_>` | — | `src/session_dag.rs` |
| `pub fn lock_nodes_write(&self) -> NodesWriteGuard<'_>` | — | `src/session_dag.rs` |
| `pub fn new(root_prompt: String, root_response: String) -> Self` | — | `src/session_dag.rs` |
| `pub fn node_count(&self) -> usize` | — | `src/session_dag.rs` |
| `pub fn nodes(&self) -> &HashMap<NodeIdx, AgentStateNode>` | — | `src/session_dag.rs` |
| `pub fn nodes(&self) -> &HashMap<NodeIdx, AgentStateNode>` | — | `src/session_dag.rs` |
| `pub fn nodes_mut(&mut self) -> &mut HashMap<NodeIdx, AgentStateNode>` | — | `src/session_dag.rs` |
| `pub fn path_to_head(&self) -> Vec<AgentStateNode>` | — | `src/session_dag.rs` |
| `pub async fn save<S: StorageEngine + ?Sized>( &self, storage: &S, namespace: &str, tx: TxId, ) -> Result<()>` | Persists all nodes, edges, active head, and next_id of the Session-DAG to the storage engine under the given namespace prefix. | `src/session_dag.rs` |
| `pub fn set_active_head(&self, node_idx: NodeIdx) -> Result<()>` | — | `src/session_dag.rs` |
| `pub fn new() -> Self` | — | `src/storage.rs` |
| `pub async fn run_all<S: GraphStorage>(storage: S)` | — | `src/storage.rs` |
| `pub fn new() -> Self` | Creates a new `FaultVfs`. | `src/test_doubles.rs` |
| `pub fn new() -> Self` | Creates a new empty `InMemoryStorageEngine`. | `src/test_doubles.rs` |
| `pub fn read_file(&self, path: &std::path::Path) -> std::result::Result<Vec<u8>, String>` | Reads data from a virtual path. | `src/test_doubles.rs` |
| `pub fn reset_counters(&self)` | Resets fault counters and failure state. | `src/test_doubles.rs` |
| `pub fn set_config(&self, config: FaultConfig)` | Sets the fault injection config. | `src/test_doubles.rs` |
| `pub fn sync(&self) -> std::result::Result<(), String>` | Flushes virtual files to disk. | `src/test_doubles.rs` |
| `pub fn trigger_crash(&self)` | Triggers an immediate crash state. | `src/test_doubles.rs` |
| `pub fn write_file( &self, path: &std::path::Path, data: &[u8], ) -> std::result::Result<(), String>` | Writes data to a virtual path. | `src/test_doubles.rs` |
| `pub fn run_diffusion<G: PathGraph>( graph: &G, seeds: &[EntityId], params: &TlHfdParams, ) -> Result<AHashMap<EntityId, f32>, TlHfdError>` | — | `src/tl_hfd/diffusion.rs` |
| `pub fn compute_lovasz_extension( participants: &[EntityId], get_x: impl Fn(EntityId) -> f32, ) -> (f32, Option<EntityId>, Option<EntityId>)` | — | `src/tl_hfd/lovasz.rs` |
| `pub fn truncate_participants( participants: &[EntityId], get_x: impl Fn(EntityId) -> f32, max_sort_size: usize, ) -> Vec<EntityId>` | — | `src/tl_hfd/lovasz.rs` |
| `pub fn thresholded_local_hfd( &self, seeds: &[EntityId], params: &TlHfdParams, ) -> Result<AHashMap<EntityId, f32>, TlHfdError>` | — | `src/tl_hfd/mod.rs` |
| `pub fn tl_hfd_local<G: PathGraph>( graph: &G, seeds: &[EntityId], params: &TlHfdParams, ) -> Result<AHashMap<EntityId, f32>, TlHfdError>` | — | `src/tl_hfd/mod.rs` |
| `pub fn validate(&self) -> Result<(), TlHfdError>` | — | `src/tl_hfd/params.rs` |
| `pub fn shadow_compare_forward_push_vs_tl_hfd<G: PathGraph>( graph: &G, seeds: &[EntityId], ppr: &PprParams, tl: &TlHfdParams, top_k: usize, ) -> Result<ShadowComparison, TlHfdError>` | — | `src/tl_hfd/shadow.rs` |
| `pub fn consume_memory(&self, bytes: u64) -> Result<(), crate::error::GraphError>` | — | `src/types.rs` |
| `pub fn from_key(key: &str) -> Result<Self, std::num::ParseIntError>` | — | `src/types.rs` |
| `pub fn from_key(key: &str) -> Result<Self, std::num::ParseIntError>` | — | `src/types.rs` |
| `pub fn has_memory_capacity(&self) -> bool` | — | `src/types.rs` |
| `pub fn is_exhausted(&self) -> bool` | — | `src/types.rs` |
| `pub fn memory_used(&self) -> u64` | — | `src/types.rs` |
| `pub fn new(id: EntityId, name: impl Into<String>, entity_type: impl Into<String>) -> Self` | — | `src/types.rs` |
| `pub fn new(from: EntityId, to: EntityId, label: impl Into<String>) -> Self` | — | `src/types.rs` |
| `pub fn new(role: RoleId, entity: EntityId) -> Self` | — | `src/types.rs` |
| `pub fn new(budget: ResourceBudget) -> Self` | — | `src/types.rs` |
| `pub fn record_allocation(&self, bytes: usize)` | — | `src/types.rs` |
| `pub fn release_memory(&self, bytes: u64)` | — | `src/types.rs` |
| `pub fn unlimited() -> Self` | — | `src/types.rs` |
| `pub fn with_limit(max_bytes: usize) -> Self` | — | `src/types.rs` |
| `pub fn with_source_doc_id(mut self, doc_id: Option<DocId>) -> Self` | — | `src/types.rs` |
| `pub fn with_weight(mut self, weight: f32) -> Self` | — | `src/types.rs` |


---

## `acta-rank`

| Attribut | Wert |
|---|---|
| **Ring** | 1 — Ebene E1 — Speicherschicht, Indizes & KI-Kerne |
| **Status** | Fertig |
| **Zweck** | 4-Signal-Retrieval-Fusion, Score-Kalibrierung und Drift-Messung. |
| **Unsafe-Code** | Nein (`#![forbid(unsafe_code)]`) |
| **Interne Abhängigkeiten** | `acta-ports`, `acta-types` |
| **Externe Abhängigkeiten (Auswahl)** | `ahash`, `serde`, `serde_json`, `thiserror`, `tracing` |

### Architekturbeschreibung

# Acta Rank (`acta-rank`)  **Ring:** Ring 0 (Ranking & Kalibrierung / Kernlogik) **Klasse:** Rand `[R]` **Reifegrad:** Experimentell / Aktiv (Kern v1)  ## Zweck des Crates Dieses Crate implementiert Methoden zur Fusion, Kalibrierung und Erklärung von Retrieval-Ergebnissen im Acta-System. Dazu gehören: - **4-Signal-Fusion:** Verknüpfung von Vektor-, Text- (BM25), Graph- und Rerank-Scores mittels reziproker Rangfusion (RRF = Reciprocal Rank Fusion) und score-basierter Normalisierung. - **Kalibrierung:** Umwandlung von Roh-Scores in Wahrscheinlichkeiten via Platt-Scaling (logistische Regression) und Isotonische Regression (PAVA = Pool-Adjacent Violators Algorithm). - **Konforme Prädiktion:** Finite-Sample-Abdeckungsgarantien und adaptive Schwellenwertanpassung bei Verteilungsverschiebung (Covariate Shift). - **Drift-Erkennung:** Überwachung von Score-Verteilungen zur Laufzeit. - **Erklärbarkeit (Explainability):** Umwandlung maschinenlesbarer Herkunftsnachweise (`ProvenanceRecord`) in strukturierte und deutschsprachige Erklärungen. - **DiBud (Discrete Budgeted):** Jederzeit abbrechbare Streaming-Fusion unter Zeit- und Kanalbudgets.  ## Einbindung und Abhängigkeiten - **Konsumiert (Ring 0):** `acta-types` (Domain-Typen), `acta-ports` (Hardware-Abstraktionen). - **Wird eingebunden von:** `acta`, `acta-db`, `acta-engine`, `acta-infer-candle`, `acta-infer-ollama`, `acta-infer-onnx`, `acta-mcp`, `acta-py`.  ## Minimalbeispiel (RRF-Doctest mit handgerechnetem Wert) ```rust use acta_rank::fusion::{rrf_fuse, ScoredItem};  // Liste 1 (Text):   [doc_1 (Rang 1), doc_2 (Rang 2)] // Liste 2 (Vektor): [doc_2 (Rang 1), doc_1 (Rang 2)] let text = vec![ScoredItem::new("doc_1", 10.0), ScoredItem::new("doc_2", 5.0)]; let vector = vec![ScoredItem::new("doc_2", 0.9), ScoredItem::new("doc_1", 0.8)];  // Mit k = 60: // doc_1: 1/(60+1) + 1/(60+2) = 1/61 + 1/62 ≈ 0.032522475 // doc_2: 1/(60+2) + 1/(60+1) = 1/62 + 1/61 ≈ 0.032522475 // Tie-Break: "doc_1" < "doc_2" (lexikografisch aufsteigend) let fused = rrf_fuse(&[text, vector], 60).unwrap_or_default(); assert_eq!(fused.len(), 2); assert_eq!(fused[0].id, "doc_1"); assert_eq!(fused[1].id, "doc_2"); let expected = (1.0 / 61.0_f32) + (1.0 / 62.0_f32); assert!((fused[0].score - expected).abs() < 1e-6); ```

### Modul- und Re-Export-Oberfläche (lib.rs)

```rust
pub mod compat;
pub mod calibration;
pub mod contract;
pub mod drift;
pub mod error;
pub mod explain;
pub mod fusion;
pub mod local_types;
pub mod metrics;
pub mod temporal;
pub mod dibud;
pub use error::{RankError, Result};
pub use local_types::{ ConfigFingerprint, DocId, FusionStrategy, FusionWeights, SignalFailurePolicy, SignalFusionStrategies, MAX_SEARCH_K, };
pub use calibration::{ calibrate_score, AdaptiveConformalCalibrator, Calibrator, Clock, ConformalCalibrator, ConformalError, IsotonicCalibrator, PlattScaler, SystemClock, };
pub use dibud::{ fuse_exact_prefix, fuse_exact_prefix_async, BudgetedChannel, DiBudFusionState, DiBudOutcome, DiBudStep, FusionBudget, };
pub use drift::{DriftDetector, DriftStatus};
pub use explain::{explain, ExplanationEntry, RetrievalExplanation};
pub use fusion::{ fuse_search_results_with_strategy, g, modulate_and_renormalize_weights, reciprocal_rank_fusion, rrf_fuse, score_normalized_fusion_with_options, weighted_reciprocal_rank_fusion, weighted_reciprocal_rank_fusion_mrrf, weighted_reciprocal_rank_fusion_with_options, weighted_reciprocal_rank_fusion_with_priority, weights_to_signal_factors, BoundedTopK, GlobalFusionConfig, GlobalFusionStrategy, MetadataMergePriority, ProvenanceBuilder, ProvenanceRecord, ScoredItem, SearchResult, SignalCalibrationContext, SignalContribution, SignalKind, };
pub use metrics::{mean_reciprocal_rank, ndcg_at_k, precision_at_k, recall_at_k, reciprocal_rank};
pub use temporal::{ apply_temporal_validity_filter, apply_temporal_validity_filter_at, apply_temporal_validity_filter_with_clock, extract_validity_window, sort_candidates_deterministic, FusionResult, ValidityWindow, };
pub use fusion::{apply_resonance_bonus, fuse_signals, ResonanceConfig};
pub use fusion::ResonanceModifier;
```

### Feature-Flags

| Flag | Aktiviert |
|---|---|
| `default` | `["std", "workspace", "exp-dibud", "exp-resonance", "experimental-rank"]` |
| `std` | `[]` |
| `workspace` | `["dep:acta-types", "dep:acta-ports"]` |
| `types-compat` | `["workspace"]` |
| `ports-compat` | `["workspace"]` |
| `dibud` | `["exp-dibud"]` |
| `exp-dibud` | `[]` |
| `exp-resonance` | `[]` |
| `experimental-all` | `["exp-dibud", "exp-resonance"]` |
| `experimental-rank` | `["experimental-all"]` |

### Schnittstellen — Public Traits (5)

**`Clock`** — *Minimaler Clock-Port für deterministische Zeitmessung im Kern.* (`src/contract.rs`)

```rust
pub trait Clock : Send + Sync {
/// Gibt den aktuellen UNIX-Zeitstempel in Nanosekunden zurück.
    fn now_unix_nanos(&self) -> u64;

    /// Gibt eine monotone Zeit in Nanosekunden zurück.
    fn monotonic_nanos(&self) -> u64 {
        self.now_unix_nanos()
    }
}
```

**`Clock`** — *Lokaler Clock-Port für deterministische Zeitstempel (B11-Konformität).* (`src/calibration/isotonic.rs`)

```rust
pub trait Clock : Send + Sync {
/// Gibt den aktuellen UNIX-Zeitstempel in Nanosekunden zurück.
    fn now_unix_nanos(&self) -> u64;

    /// Gibt eine monotone Zeit in Nanosekunden zurück.
    fn monotonic_nanos(&self) -> u64 {
        self.now_unix_nanos()
    }
}
```

**`ConformalCalibrator`** — *Trait for deterministic, online conformal calibration and threshold estimation. Trait für deterministische konforme Schwellenwertschätzung.* (`src/calibration/conformal.rs`)

```rust
pub trait ConformalCalibrator {
/// Deterministically updates the threshold given a non-conformity score and propensity weight.
    ///
    /// # Parameters
    /// - `non_conformity_score`: The non-conformity score $S_t$ of the current sample.
    /// - `propensity_weight`: The propensity weight $w_t > 0$ correcting for covariate shift.
    ///
    /// # Returns
    /// - `Ok(true)` if the update altered the threshold value.
    /// - `Ok(false)` if the threshold value remained unchanged.
    /// - `Err(ConformalError)` if any input parameter is non-finite or non-positive.
    // Ablauf des Online-Pinball-Updates:
    // 1. Validierung: Prüfen, ob propensity_weight > 0.0 und non_conformity_score endlich sind.
    // 2. Schrittweite berechnen:
    //    - Bei Überschreitung (S_t > threshold): Delta = +gamma * w_t * (1 - alpha)
    //    - Bei Unterschreitung (S_t <= threshold): Delta = -gamma * w_t * alpha
    // 3. Klemmen: Neuer Schwellenwert wird auf [min_threshold, max_threshold] beschränkt.
    // 4. Inkrementieren: Zähler der Beobachtungen erhöhen und Änderungsstatus zurückgeben.
    fn update(
        &mut self,
        non_conformity_score: f32,
        propensity_weight: f32,
    ) -> Result<bool, ConformalError>;

    /// Returns the currently valid threshold $\hat{q}$.
    fn threshold(&self) -> f32;
}
```

**`IsotonicPortsClockExt`** — *Extension-Trait für `IsotonicCalibrator` zur Ports-Clock-Konfiguration.* (`src/compat/clock.rs`)

```rust
pub trait IsotonicPortsClockExt {
/// Konfiguriert den Kalibrator mit einer externen `acta_ports::Clock`.
    fn with_ports_clock(self, clock: Arc<dyn acta_ports::Clock>) -> Self;
}
```

**`ResonanceModifier`** — *Trait für Signal-Resonanz und Kohärenz-Modulation.* (`src/fusion/ext.rs`)

```rust
pub trait ResonanceModifier {
/// Wendet Resonanz- oder Kohärenzmodifikationen auf die Ergebnisliste an.
    fn modify_results(&self, results: Vec<SearchResult>, valid_signal_count: usize) -> Vec<SearchResult>;
}
```

### Öffentliche Datentypen — Structs (29)

| Struct | Felder / Form | Beschreibung | Datei |
|---|---|---|---|
| `AdaptiveConformalCalibrator` | `{ alpha: f32, threshold: f32, learning_rate: f32, min_threshold: f32, max_threshold: f32, observation_count: u64, fingerprint: Option<ConfigFingerprint>, }` | Adaptive Conformal Calibrator for quantile-based online threshold estimation under covariate shift. Implements online stochastic gradient updates on… | `src/calibration/conformal.rs` |
| `BoundedTopK` | `{ heap: std::collections::BinaryHeap<T>, capacity: usize, }` | Size-bounded min-heap for Top-K candidate selection during Reciprocal Rank Fusion. # Abgrenzung Dieser Typ (`acta_rank::fusion::BoundedTopK`) dient a… | `src/fusion/topk.rs` |
| `ConfigFingerprint` | `{ /// Identifikator oder Hash der Modell-/Promptkonfiguration. pub hash: [u8; 32], }` | Konfigurations-Fingerprint zur Validierung und Cache-Invalidierung. | `src/local_types.rs` |
| `DiBudFusionState` | `{ channel_depths: [usize; 3], exhausted: [bool; 3], accesses: usize, observed_docs: AHashMap<DocId, ObservedDocState>, er_cache: AHashMap<DocId, f32>, provenance… }` | Core state machine tracking channel depths, observed document scores, and prefix certification. Zustandsautomat für die budgetierte Streaming-Rangfus… | `src/dibud/state.rs` |
| `DiBudOutcome` | `{ /// Ranked candidate document IDs sorted descending by lower bound $F^-(x)$ (tie-breaking by ascending `DocId`). pub ranked: Vec<DocId>, /// Number of prefix e… }` | Final outcome of a DiBud fusion execution. Endergebnis der DiBud-Fusion inklusive zertifiziertem Präfix. | `src/dibud/types.rs` |
| `DocId` | `(tuple/unit)` | Opake Dokumentidentität für `acta-rank`. | `src/local_types.rs` |
| `DriftDetector` | `{ window_size: usize, threshold: f32, baseline_mean: Option<f32>, recent_scores: VecDeque<f32>, }` | Drift detector monitoring score shifts over sliding windows. Detektor zur Überwachung von Score-Verschiebungen über ein gleitendes Fenster. | `src/drift.rs` |
| `ExplanationEntry` | `{ /// Identified search signal type. pub signal: SignalKind, /// Raw unnormalized score or distance. pub raw_score: f32, /// 1-based rank in the signal's result… }` | Breakdown entry representing the contribution of a single search signal. Einzelner Beitrags-Eintrag für eine spezifische Signalquelle innerhalb der E… | `src/explain.rs` |
| `FusionBudget` | `{ /// Maximum total channel access budget (reads). pub max_total_accesses: usize, /// Minimum required certified prefix length before stopping. pub min_certified… }` | Dynamic budget configuration for DiBud fusion. Ressourcen- und Abbruchbudget für die Streaming-Rangfusion. | `src/dibud/types.rs` |
| `FusionWeights` | `{ /// Vektor-Gewicht. pub vector: f32, /// Text-Gewicht. pub text: f32, /// Graph-Gewicht. pub graph: f32, /// Zusätzliche Gewichte. pub metadata: f32, }` | Fusionsgewichte für die Normalisierung. | `src/local_types.rs` |
| `GlobalFusionConfig` | `{ /// Maximum number of community nodes to include in the final result set. pub max_community_nodes: Option<usize>, /// Minimum size (member count) required for… }` | Configuration for `GlobalFusionStrategy`. Konfigurationsparameter für die globale Community-Fusionsstrategie. | `src/fusion/global.rs` |
| `GlobalFusionStrategy` | `{ config: GlobalFusionConfig, }` | Community-aware fusion strategy for corpus-wide topic retrieval (LeanRAG-backed). Globale Fusionsstrategie für hierarchische und korpusweite Retrieva… | `src/fusion/global.rs` |
| `IsotonicCalibrator` | `{ observations: VecDeque<(f32, bool)>, warmup_required: u32, max_observations: usize, cached_model: Option<Vec<(f32, f32)>>, // (max_score_in_block, calibrated_p… }` | Isotonic Calibrator using Pool-Adjacent Violators Algorithm. Nicht-parametrischer Kalibrierer auf Basis von PAVA. | `src/calibration/isotonic.rs` |
| `PlattScaler` | `{ a: f32, b: f32, fingerprint: Option<ConfigFingerprint>, }` | Platt Scaler for parametric logit/score calibration. Parametrischer Kalibrierer nach Platt (1999). | `src/calibration/platt.rs` |
| `PortsClockAdapter` | `(tuple/unit)` | Adapter für einen dynamischen `acta_ports::Clock` Referenzzeiger auf das lokale `Clock`-Trait. | `src/compat/clock.rs` |
| `ProvenanceBuilder` | `{ vector_distance: Option<f32>, vector_rank: Option<u32>, vector_weight: Option<f32>, bm25_score: Option<f32>, bm25_rank: Option<u32>, text_weight: Option<f32>,… }` | Fluent builder for constructing `ProvenanceRecord` instances. Fluent-Builder zum deterministischen Erstellen von `ProvenanceRecord`-Strukturen. | `src/fusion/provenance.rs` |
| `ProvenanceRecord` | `{ /// Distance score from vector search signal. #[serde(default, skip_serializing_if = "Option::is_none")] pub vector_distance: Option<f32>, /// BM25 score from… }` | Provenance record for fused search result. Auditierbarer Herkunftsnachweis (Provenance) über alle beteiligten Retrieval-Kanäle. | `src/fusion/types.rs` |
| `ResonanceConfig` | `{ /// Exponent β for coherence bonus. Default: 0.5. pub beta: f32, /// Boost strength γ. Default: 0.3. pub gamma: f32, }` | Configuration for Resonance Coherence Bonus. Konfigurationsparameter für die Berechnung des Resonanz-Kohärenz-Bonus. | `src/fusion/resonance.rs` |
| `ResonanceConfig` | `{ /// Dummy field pub enabled: bool, }` | Stub-Konfiguration für Resonanz-Bonus, wenn `exp-resonance` nicht aktiviert ist. | `src/fusion/ext.rs` |
| `RetrievalExplanation` | `{ /// Individual signal contribution entries, sorted descending by `contribution_share`. pub entries: Vec<ExplanationEntry>, /// Dominant search signal contribut… }` | Structured explanation detailing why a document was retrieved with its given rank. Strukturierte Gesamterklärung zur Nachvollziehbarkeit eines Retrie… | `src/explain.rs` |
| `ScoredItem` | `{ /// Eindeutiger Dokument- oder Elementbezeichner. pub id: String, /// Punktwert / Ähnlichkeitswert / Fusionswert. pub score: f32, /// Optionaler strukturierter… }` | Crate-lokaler Typ für gerankte/fusionierte Elemente. | `src/fusion/types.rs` |
| `SearchReport` | `{ /// List of signals that failed during query execution when using Degrade policy. pub degraded_signals: Vec<Signal>, /// Warnings captured during query process… }` | Execution report for search query execution. Detaillierter Audit-Report über den Fusions- und Suchablauf. | `src/fusion/types.rs` |
| `SearchResult` | `{ /// Document or memory entry identifier. pub id: String, /// Fused score value. pub score: f32, /// Document metadata payload. pub metadata: Option<serde_json:… }` | Individual search result candidate for fusion. Standard-Suchergebnis eines einzelnen Retrieval-Signals oder einer Fusionsstufe. | `src/fusion/types.rs` |
| `SignalCalibrationContext` | `{ /// Signal identifier matching the result set signal name (e.g., "vector", "text", "graph"). pub signal_name: String, /// Indicates whether the calibrator for… }` | Calibration and drift context for a search signal during mRRF weight modulation. Kontext zu Modellkalibrierung und Driftzustand eines Signals für die… | `src/fusion/signal.rs` |
| `SignalContribution` | `{ /// Raw score before normalization/fusion. pub raw_score: f32, /// 1-based rank in the signal's result list. pub rank: u32, /// Calculated RRF or normalized co… }` | Signal contribution details. Beitrag eines einzelnen Signals zum aggregierten RRF-Score. | `src/fusion/types.rs` |
| `SignalFusionStrategies` | `{ /// Strategie für Vektor. pub vector: FusionStrategy, /// Strategie für Text. pub text: FusionStrategy, /// Strategie für Graph. pub graph: FusionStrategy, }` | Signal-Fusionsstrategien pro Kanal. | `src/local_types.rs` |
| `SystemClock` | `(tuple/unit)` | Standard SystemClock basierend auf std::time::SystemTime. | `src/calibration/isotonic.rs` |
| `TxId` | `(tuple/unit)` | Transaktions-Identität für zeitliche und MVCC-Gültigkeit. | `src/local_types.rs` |
| `ValidityWindow` | `{ /// Start of transaction validity (system time / MVCC). pub tx_valid_from: Option<TxId>, /// End of transaction validity (system time / MVCC, exclusive upper b… }` | Extracted bi-temporal validity window representation for a document or search candidate. | `src/temporal.rs` |

### Öffentliche Datentypen — Enums (11)

| Enum | Varianten | Beschreibung | Datei |
|---|---|---|---|
| `BudgetedChannel` | `/// Vector (semantic) search channel (index 0). Vector = 0, /// Text (BM25) search channel (index 1). Text = 1, /// Graph search channel (index 2). Graph = 2,` | Identifies the channels managed by DiBud fusion budget. Order of variants is defined such that `Vector < Text < Graph`, matching the tie-breaking pri… | `src/dibud/types.rs` |
| `Calibrator` | `/// Platt-Scaling (logistische Regression) mit Format-Version 1. #[serde(rename = "platt_v1")] Platt { /// Format-Versionsfeld für defensive Dekodierung. format_version: u8, /// Platt-Scaler-Instanz.…` | Versionierter Container für Kalibrierungsmodelle (Klassenvertrag RANK-1). | `src/calibration/mod.rs` |
| `ConformalError` | `/// Target miscoverage level `alpha` is not finite or outside (0.0, 1.0). #[error("Invalid target miscoverage alpha: {0} (must be finite and in (0.0, 1.0))")] Alpha(f32), /// Propensity weight is not…` | Error type for conformal calibration and threshold updates. Fehlertypen für die konforme Kalibrierung. | `src/calibration/conformal.rs` |
| `DiBudStep` | `/// Request reading the next item from the specified channel. Poll(BudgetedChannel), /// Fusion is complete; outcome contains certified prefix and ranked documents. Done(DiBudOutcome),` | Represents the step action requested by `DiBudFusionState::next_request`. Nächste durchzuführende Aktion im DiBud-Zustandsautomaten. | `src/dibud/state.rs` |
| `DriftStatus` | `/// Distribution is stable. Stable { /// Mean score shift relative to baseline. mean_shift: f32, }, /// Drift detected exceeding configured tolerance threshold. DriftDetected { /// Mean score shift r…` | Drift status summary representation. Zusammenfassung des aktuellen Drift-Status einer Signalverteilung. | `src/drift.rs` |
| `FusionStrategy` | `/// Standard Reciprocal Rank Fusion mit k=60. #[default] Rrf, /// Score-normalisierte gewichtete lineare Kombination. ScoreNormalized,` | Fusionsstrategie für hybride Suchen. | `src/local_types.rs` |
| `MetadataMergePriority` | `/// Vector metadata is processed first (default behavior). Order: Vector, Text, Graph. #[default] VectorFirst, /// Text metadata is processed first. Order: Text, Vector, Graph. TextFirst, /// Graph m…` | Configures signal priority order for metadata merging during Reciprocal Rank Fusion. Prioritätsstrategie bei der Zusammenführung kollidierender Metad… | `src/fusion/signal.rs` |
| `RankError` | `/// Ungültige Eingabedaten oder Konfiguration. #[error("invalid input: {0}")] InvalidInput(String), /// Fehler bei der Kalibrierung oder Wahrscheinlichkeitsberechnung. #[error("calibration error: {0}…` | Fehlerzustände von `acta-rank`. | `src/error.rs` |
| `Signal` | `/// Dense vector similarity signal. Vector, /// Full-text BM25 keyword signal. Text, /// Graph entity relationship signal. Graph,` | Query retrieval signal identifier. # Abgrenzung Dieser Typ (`acta_rank::fusion::Signal`) bezeichnet die spezifische Suchsignal-Art (z. B. `Vector`, `… | `src/fusion/types.rs` |
| `SignalFailurePolicy` | `/// Bricht die Suche ab. #[default] Fail, /// Setzt die Suche mit reduzierten Signalen fort. Degrade,` | Richtlinie bei Signal-Fehlern. | `src/local_types.rs` |
| `SignalKind` | `/// Vector (semantic k-NN) search signal. Vector, /// Text (BM25 keyword) search signal. Text, /// Graph (traversal / PageRank) search signal. Graph, /// Edge-reinforcement weight signal / Bandit rec…` | Identifies the kind of search signal used during fusion. Klassifikation der unterschiedlichen Retrieval-Signalarten im Acta-System. | `src/fusion/signal.rs` |

### Typ-Aliase (3)

| Alias | Definition | Beschreibung |
|---|---|---|
| `FusedScore` | `type FusedScore= SearchResult` | Type alias for fused score output item. Aggregierter Fusionswert eines Dokuments (Typ-Alias auf SearchResult). |
| `FusionResult` | `type FusionResult= crate::fusion::SearchResult` | Type alias for search & fusion results in post-retrieval pipelines. |
| `Result` | `type Result<T> = core::result::Result<T, RankError>` | Standard-Result-Typ für `acta-rank`. |

### Konstanten (6)

| Konstante | Typ | Beschreibung |
|---|---|---|
| `DEFAULT_INITIAL_THRESHOLD` | `f32` | Default initial threshold $\hat{q}_0$. |
| `DEFAULT_LEARNING_RATE` | `f32` | Default learning rate $\gamma$ for online quantile updates. |
| `DEFAULT_MAX_THRESHOLD` | `f32` | Default maximum threshold bound. |
| `DEFAULT_MIN_THRESHOLD` | `f32` | Default minimum threshold bound. |
| `MAX_SEARCH_K` | `usize` | Maximale Anzahl an Suchergebnissen für Top-K-Buffer. |
| `ZERO` | `Self` | Null-Fingerprint. |

### Öffentliche Funktionen (118)

| Signatur | Beschreibung | Datei |
|---|---|---|
| `pub fn alpha(&self) -> f32` | Returns the target error rate $\alpha$. | `src/calibration/conformal.rs` |
| `pub fn invalidate_on_config_change(&mut self, new_fingerprint: impl Into<ConfigFingerprint>)` | Resets state if ConfigFingerprint changes (P8 Compliance). | `src/calibration/conformal.rs` |
| `pub fn learning_rate(&self) -> f32` | Returns the learning rate $\gamma$. | `src/calibration/conformal.rs` |
| `pub fn new(alpha: f32) -> Result<Self, ConformalError>` | Creates a new `AdaptiveConformalCalibrator` with target error rate `alpha` in `(0.0, 1.0)`. Uses default initial threshold (0.5) and defaul… | `src/calibration/conformal.rs` |
| `pub fn observation_count(&self) -> u64` | Returns the total number of observations processed. | `src/calibration/conformal.rs` |
| `pub fn with_bounds( mut self, min_threshold: f32, max_threshold: f32, ) -> Result<Self, ConformalError>` | Sets bounding constraints `[min_threshold, max_threshold]` for the estimated threshold. | `src/calibration/conformal.rs` |
| `pub fn with_params( alpha: f32, initial_threshold: f32, learning_rate: f32, ) -> Result<Self, ConformalError>` | Creates an `AdaptiveConformalCalibrator` with custom parameters. | `src/calibration/conformal.rs` |
| `pub fn cached_ece(&self) -> Option<f32>` | Returns cached Expected Calibration Error (ECE). | `src/calibration/isotonic.rs` |
| `pub fn calibrated_probability(&mut self, raw_score: f32) -> Option<f32>` | Returns calibrated probability if warm, or `None` if below warmup requirement. Liefert die kalibrierte Wahrscheinlichkeit für einen Roh-Sco… | `src/calibration/isotonic.rs` |
| `pub fn expected_calibration_error(&mut self) -> Option<f32>` | Calculates Expected Calibration Error over 10 bins. | `src/calibration/isotonic.rs` |
| `pub fn force_rebuild(&mut self)` | Forces immediate PAVA model rebuild. | `src/calibration/isotonic.rs` |
| `pub fn invalidate_on_config_change(&mut self, new_fingerprint: impl Into<ConfigFingerprint>)` | Resets observations if ConfigFingerprint changes (P8 Compliance). | `src/calibration/isotonic.rs` |
| `pub fn is_calibrated(&self) -> bool` | Checks if warmup observations threshold has been reached. | `src/calibration/isotonic.rs` |
| `pub fn last_calibration_at(&self) -> Option<u64>` | Returns timestamp (UNIX epoch in seconds) of last model rebuild. | `src/calibration/isotonic.rs` |
| `pub fn new() -> Self` | Erzeugt eine neue `SystemClock`. | `src/calibration/isotonic.rs` |
| `pub fn new(warmup_required: u32, max_observations: usize) -> Self` | Creates a new `IsotonicCalibrator`. | `src/calibration/isotonic.rs` |
| `pub fn observation_count(&self) -> usize` | Returns current observation count. | `src/calibration/isotonic.rs` |
| `pub fn record_outcome(&mut self, raw_score: f32, outcome: bool)` | Records a new observation `(raw_score, outcome)`. Non-finite `raw_score` values are safely ignored. Nimmt eine neue Beobachtung `(raw_score… | `src/calibration/isotonic.rs` |
| `pub fn with_clock(mut self, clock: Arc<dyn Clock>) -> Self` | Sets a custom clock port for deterministic time access. | `src/calibration/isotonic.rs` |
| `pub fn with_defaults() -> Self` | Creates an `IsotonicCalibrator` with defaults (Warmup: 50, Max Obs: 2000). | `src/calibration/isotonic.rs` |
| `pub fn calibrate_score(raw: f32, calibrator: &Calibrator) -> Result<f32>` | Kalibriert einen Roh-Score anhand eines übergebenen Kalibrierers (Klassenvertrag RANK-1). # Fehler Gibt `RankError::InvalidInput` zurück, w… | `src/calibration/mod.rs` |
| `pub fn conformal(calibrator: AdaptiveConformalCalibrator) -> Self` | Erzeugt einen neuen `Calibrator` basierend auf einem `AdaptiveConformalCalibrator`. | `src/calibration/mod.rs` |
| `pub fn isotonic(calibrator: IsotonicCalibrator) -> Self` | Erzeugt einen neuen `Calibrator` basierend auf einem `IsotonicCalibrator`. | `src/calibration/mod.rs` |
| `pub fn platt(scaler: PlattScaler) -> Self` | Erzeugt einen neuen `Calibrator` basierend auf einem `PlattScaler`. | `src/calibration/mod.rs` |
| `pub fn apply(&self, logit: f32) -> f32` | Alias for `transform(logit)`. | `src/calibration/platt.rs` |
| `pub fn fit(observations: &[(f32, bool)]) -> Self` | Fits parameters `A` and `B` via Negative Log-Likelihood minimization with L2 regularization and target smoothing (Platt, 1999). Trainiert d… | `src/calibration/platt.rs` |
| `pub fn identity() -> Self` | Uncalibrated default fallback ($A=1.0, B=0.0$). | `src/calibration/platt.rs` |
| `pub fn invalidate_on_config_change(&mut self, new_fingerprint: impl Into<ConfigFingerprint>)` | Resets scaler to identity if ConfigFingerprint changes (P8 Compliance). | `src/calibration/platt.rs` |
| `pub fn is_fitted(&self) -> bool` | Checks if model has been fitted. | `src/calibration/platt.rs` |
| `pub fn is_identity(&self) -> bool` | Checks if this scaler is identical to uncalibrated default ($A=1.0, B=0.0$). | `src/calibration/platt.rs` |
| `pub fn new(a: f32, b: f32) -> Self` | Creates a new `PlattScaler` with specified parameters `a` and `b`. Falls back to identity if non-finite. | `src/calibration/platt.rs` |
| `pub fn params(&self) -> (f32, f32)` | Returns current parameters `(a, b)`. | `src/calibration/platt.rs` |
| `pub fn predict(&self, raw_score: f32) -> f32` | Calculates calibrated probability for raw score. | `src/calibration/platt.rs` |
| `pub fn transform(&self, logit: f32) -> f32` | Applies Platt Scaling `sigmoid(A * logit + B)`. Führt die logistische Transformation `sigmoid(A * logit + B)` aus. | `src/calibration/platt.rs` |
| `pub fn apply_temporal_validity_filter_with_ports_clock( results: Vec<crate::fusion::SearchResult>, current_tx: TxId, clock: &dyn acta_ports::Clock, ) -> Vec<crate::fusion::SearchResult>` | Bi-temporal filter helper mit Ports-Clock. | `src/compat/types.rs` |
| `pub fn signal_contribution_synthesized_from_types(source_doc_ids: &[acta_types::DocId]) -> crate::fusion::ProvenanceRecord` | Helper für Synthese aus `acta_types::DocId`. | `src/compat/types.rs` |
| `pub fn weights_to_signal_factors_types_compat( weights: Option<&acta_types::FusionWeights>, ) -> (f32, f32, f32)` | Konvertiert optionale `acta_types::FusionWeights`. | `src/compat/types.rs` |
| `pub fn fuse_exact_prefix<I1, I2, I3>( mut state: DiBudFusionState, vector: &mut I1, text: &mut I2, graph: &mut I3, edge_reinforcement: impl Fn(DocId) -> f32, budget: &FusionBudget, ) -> Result<DiBudOutcome, RankError> where I1: Iterator<Item = DocId>, I2: Ite…` | Synchronously executes DiBud fusion using three channel iterators. Führt die DiBud-Streaming-Fusion synchron über Iterator-Kanäle aus. | `src/dibud/driver.rs` |
| `pub async fn fuse_exact_prefix_async<P, Fut>( mut state: DiBudFusionState, mut poll: P, edge_reinforcement: impl Fn(DocId) -> f32, budget: &FusionBudget, ) -> Result<DiBudOutcome, RankError> where P: FnMut(BudgetedChannel) -> Fut, Fut: Future<Output = Result<…` | Asynchronously executes DiBud fusion using a channel poll closure returning a std `Future`. This function relies purely on `core::future::F… | `src/dibud/driver.rs` |
| `pub fn accesses(&self) -> usize` | Returns total read accesses performed. | `src/dibud/state.rs` |
| `pub fn channel_depths(&self) -> [usize` | Returns current channel depths `[Vector, Text, Graph]`. | `src/dibud/state.rs` |
| `pub fn exhausted(&self) -> [bool` | Returns exhausted status for each channel `[Vector, Text, Graph]`. | `src/dibud/state.rs` |
| `pub fn feed( &mut self, ch: BudgetedChannel, item: Option<DocId>, er: impl Fn(DocId) -> f32, budget: &FusionBudget, ) -> Result<(), RankError>` | Feeds the result of reading a channel into the state machine. Raw scores are unknown in pure `DocId` streams and are set to 0.0 in `Provena… | `src/dibud/state.rs` |
| `pub fn new(capacity: usize) -> Self` | Creates a new `DiBudFusionState` with pre-allocated map capacities. | `src/dibud/state.rs` |
| `pub fn next_request(&self, budget: &FusionBudget) -> DiBudStep` | Determines whether fusion is done or returns the next channel to poll. Bestimmt die nächste Abfrageaktion oder schließt die Fusion ab. | `src/dibud/state.rs` |
| `pub fn provenance_of(&self, doc_id: &DocId) -> Option<&ProvenanceRecord>` | Returns reference to provenance record for a given document if present. | `src/dibud/state.rs` |
| `pub fn validate(&self) -> Result<(), RankError>` | Validates budget parameter constraints. Ensures all weights are finite and non-negative, and minimum accesses/certified lengths are at leas… | `src/dibud/types.rs` |
| `pub fn analyze(&self) -> DriftStatus` | Analyzes current observations against baseline. | `src/drift.rs` |
| `pub fn as_str(&self) -> &'static str` | Returns human-readable summary string ("stabil", "warnung", "kritisch", or "unbekannt"). | `src/drift.rs` |
| `pub fn new(window_size: usize, threshold: f32) -> Self` | Creates a new `DriftDetector` with window size and sensitivity threshold. | `src/drift.rs` |
| `pub fn observe(&mut self, score: f32) -> DriftStatus` | Records a new score observation and calculates updated drift status. | `src/drift.rs` |
| `pub fn overall_drift_status(&self) -> &'static str` | Returns human-readable overall drift status string. | `src/drift.rs` |
| `pub fn set_baseline(&mut self, baseline: f32)` | Sets explicit baseline mean score. | `src/drift.rs` |
| `pub fn with_defaults() -> Self` | Creates a `DriftDetector` with default parameters (window size 50, threshold 0.15). | `src/drift.rs` |
| `pub fn explain(record: &ProvenanceRecord) -> RetrievalExplanation` | Generates a structured `RetrievalExplanation` from a `ProvenanceRecord`. Computes per-signal `contribution_share` normalized such that all… | `src/explain.rs` |
| `pub fn to_human_readable_de(&self) -> String` | Generates a human-readable German explanation string for audit logging or CLI output. Formats all signal contributions using exact ranks an… | `src/explain.rs` |
| `pub fn apply_resonance_bonus( results: Vec<SearchResult>, _valid_signal_count: usize, _config: &ResonanceConfig, ) -> Vec<SearchResult>` | No-Op Fallback für `apply_resonance_bonus`. | `src/fusion/ext.rs` |
| `pub fn config(&self) -> &GlobalFusionConfig` | Returns a reference to the configuration. | `src/fusion/global.rs` |
| `pub fn fuse( &self, result_sets: Vec<(String, Vec<SearchResult>, f32)>, max_results: usize, ) -> Result<Vec<SearchResult>, RankError>` | Fuses multi-signal search results using community-aware RRF. Candidates are grouped by community ID (extracted from candidate metadata `"co… | `src/fusion/global.rs` |
| `pub fn new(config: GlobalFusionConfig) -> Self` | Creates a new `GlobalFusionStrategy` with the given configuration. | `src/fusion/global.rs` |
| `pub fn modulate_and_renormalize_weights( result_sets: &[(String, Vec<SearchResult>, f32)], calibration_contexts: Option<&[SignalCalibrationContext]>, beta: f32, ) -> Vec<f32>` | Modulates signal weights using mRRF margin formula: $m_s = S_1 - S_2$ (when signal is calibrated AND `drift_status` is `Stable` and $\ge 2$… | `src/fusion/mrrf.rs` |
| `pub fn weighted_reciprocal_rank_fusion_mrrf( mut result_sets: Vec<(String, Vec<SearchResult>, f32)>, max_results: usize, priority: MetadataMergePriority, include_provenance: bool, resonance_config: Option<&ResonanceConfig>, calibration_contexts: Option<&[Sign…` | Weighted Reciprocal Rank Fusion with mRRF margin-based calibration weight modulation. Gewichtete Rangfusion mit margin-basierter mRRF-Gewic… | `src/fusion/mrrf.rs` |
| `pub fn score_normalized_fusion_with_options( mut result_sets: Vec<(String, Vec<SearchResult>, f32)>, max_results: usize, priority: MetadataMergePriority, include_provenance: bool, resonance_config: Option<&ResonanceConfig>, ) -> Vec<SearchResult>` | Score-normalized fusion (CombSUM with MinMax normalization). Führt eine score-normalisierte lineare Fusion mehrerer Signal-Ergebnismengen d… | `src/fusion/normalized.rs` |
| `pub fn weights_to_signal_factors( weights: Option<&crate::local_types::FusionWeights>, ) -> (f32, f32, f32)` | Converts optional `FusionWeights` into (vector, text, graph) weight tuple. Konvertiert relative Signalgewichte in Normalisierungsfaktoren. | `src/fusion/normalized.rs` |
| `pub fn bm25(mut self, score: f32, rank: u32, weight: impl Into<Option<f32>>) -> Self` | Attaches BM25 text signal score, rank, and optional weight. | `src/fusion/provenance.rs` |
| `pub fn build(self) -> ProvenanceRecord` | Builds the resulting `ProvenanceRecord`. Schließt die Konstruktion ab und berechnet die RRF-Beiträge je Signal. | `src/fusion/provenance.rs` |
| `pub fn expected_total(mut self, total: f32) -> Self` | Sets expected ground truth RRF score for debug invariant checking. | `src/fusion/provenance.rs` |
| `pub fn explain_human_readable(&self) -> String` | Generates a deterministic human-readable explanation string from the built provenance record. | `src/fusion/provenance.rs` |
| `pub fn explain_human_readable(&self) -> String` | Generates a concise, deterministic human-readable explanation from retrieval signal contributions. | `src/fusion/provenance.rs` |
| `pub fn explain_human_readable_with_params( &self, fusion_mode: Option<&str>, calibrated_threshold: Option<f32>, ) -> String` | Generates a concise, deterministic human-readable explanation with optional fusion mode and calibrated threshold. | `src/fusion/provenance.rs` |
| `pub fn graph(mut self, score: f32, rank: u32, weight: impl Into<Option<f32>>) -> Self` | Attaches graph signal score, rank, and optional weight. | `src/fusion/provenance.rs` |
| `pub fn index_type(mut self, index_type: impl Into<String>) -> Self` | Attaches index type string. | `src/fusion/provenance.rs` |
| `pub fn new(rrf_k: f32) -> Self` | Creates a new `ProvenanceBuilder` with the specified RRF parameter `k`. | `src/fusion/provenance.rs` |
| `pub fn rerank_score(mut self, score: f32) -> Self` | Attaches optional cross-encoder rerank score. | `src/fusion/provenance.rs` |
| `pub fn source_collection(mut self, collection: impl Into<String>) -> Self` | Attaches source collection name. | `src/fusion/provenance.rs` |
| `pub fn vector(mut self, dist: f32, rank: u32, weight: impl Into<Option<f32>>) -> Self` | Attaches vector signal distance, rank, and optional weight. | `src/fusion/provenance.rs` |
| `pub fn apply_resonance_bonus( results: Vec<SearchResult>, valid_signal_count: usize, config: &ResonanceConfig, ) -> Vec<SearchResult>` | Applies Resonance Coherence Bonus to fused search results. Wendet den Resonanz-Kohärenz-Bonus auf bereits fusionierte Suchergebnisse an. | `src/fusion/resonance.rs` |
| `pub fn fuse_signals( result_sets: Vec<(String, Vec<SearchResult>, f32)>, max_results: usize, ) -> Vec<FusedScore>` | Convenience API to fuse multi-signal search results. Komfortfunktion zur Zusammenführung mehrerer Signal-Ergebnismengen mittels gewichteter… | `src/fusion/resonance.rs` |
| `pub fn fuse_search_results_with_signal_strategies( mut result_sets: Vec<(String, Vec<SearchResult>, f32)>, max_results: usize, priority: MetadataMergePriority, include_provenance: bool, resonance_config: Option<&ResonanceConfig>, strategies: crate::local_type…` | Fuses search result sets using per-signal fusion strategies (`SignalFusionStrategies`). Führt Teilergebnisse mit individuellen Strategien j… | `src/fusion/rrf.rs` |
| `pub fn fuse_search_results_with_strategy( result_sets: Vec<(String, Vec<SearchResult>, f32)>, max_results: usize, priority: MetadataMergePriority, include_provenance: bool, resonance_config: Option<&ResonanceConfig>, strategy: FusionStrategy, ) -> Vec<SearchR…` | Fuses search result sets using the specified `FusionStrategy`. Universeller Einstiegspunkt zur Signal-Fusion basierend auf einer globalen `… | `src/fusion/rrf.rs` |
| `pub fn g(x: f32) -> f32` | Bounded smooth influence function $g(x) = \frac{x}{1 + |x|}$. Returns $x / (1 + |x|)$, bounded in $(-1.0, 1.0)$ for all finite $x$. Returns… | `src/fusion/rrf.rs` |
| `pub fn reciprocal_rank_fusion( result_sets: Vec<Vec<SearchResult>>, max_results: usize, ) -> Vec<SearchResult>` | Fuses multiple sets of ranked search results into a single ranked list using Reciprocal Rank Fusion (RRF). Führt mehrere ungewichtete Signa… | `src/fusion/rrf.rs` |
| `pub fn rrf_fuse(ranked_lists: &[Vec<ScoredItem>], k: usize) -> Result<Vec<ScoredItem>>` | Standard RRF-Fusion gemäß Klassenvertrag RANK-1: `rrf_fuse(ranked_lists: &[Vec<ScoredItem>], k: usize) -> Result<Vec<ScoredItem>>`. Berechn… | `src/fusion/rrf.rs` |
| `pub fn weighted_reciprocal_rank_fusion( result_sets: Vec<(String, Vec<SearchResult>, f32)>, max_results: usize, ) -> Vec<SearchResult>` | Weighted Reciprocal Rank Fusion with default signal metadata priority (`VectorFirst`). Gewichtete reziproke Rangfusion (Weighted RRF) mit S… | `src/fusion/rrf.rs` |
| `pub fn weighted_reciprocal_rank_fusion_with_options( result_sets: Vec<(String, Vec<SearchResult>, f32)>, max_results: usize, priority: MetadataMergePriority, include_provenance: bool, resonance_config: Option<&ResonanceConfig>, ) -> Vec<SearchResult>` | Weighted Reciprocal Rank Fusion with options. Vollständig konfigurierbare gewichtete Rangfusion mit Resonanz- und Provenance-Optionen. | `src/fusion/rrf.rs` |
| `pub fn weighted_reciprocal_rank_fusion_with_priority( result_sets: Vec<(String, Vec<SearchResult>, f32)>, max_results: usize, priority: MetadataMergePriority, ) -> Vec<SearchResult>` | Weighted Reciprocal Rank Fusion with explicit metadata merge priority. Gewichtete reziproke Rangfusion mit expliziter Metadaten-Zusammenfüh… | `src/fusion/rrf.rs` |
| `pub fn as_str(&self) -> &'static str` | Converts `SignalKind` to its canonical string representation. | `src/fusion/signal.rs` |
| `pub fn from_name(name: &str) -> Option<Self>` | Identifies `SignalKind` from a signal name string. | `src/fusion/signal.rs` |
| `pub fn name_de(&self) -> &'static str` | Returns the German display name for the signal. | `src/fusion/signal.rs` |
| `pub fn new( signal_name: impl Into<String>, is_calibrated: bool, drift_status: DriftStatus, ) -> Self` | Creates a new `SignalCalibrationContext`. | `src/fusion/signal.rs` |
| `pub fn should_use_calibrated_scores(&self) -> bool` | Evaluates whether calibrated scores should be used for margin calculation. Returns `true` if and only if `is_calibrated` is true AND `drift… | `src/fusion/signal.rs` |
| `pub fn signal_rank(&self, signal_name: &str) -> usize` | Returns the precedence rank for a given signal name. | `src/fusion/signal.rs` |
| `pub fn capacity(&self) -> usize` | Returns maximum allowed capacity `k`. | `src/fusion/topk.rs` |
| `pub fn into_sorted_vec(self) -> Vec<T>` | Consumes the container and returns items sorted from best to worst. | `src/fusion/topk.rs` |
| `pub fn is_empty(&self) -> bool` | Returns `true` if the container holds zero items. | `src/fusion/topk.rs` |
| `pub fn len(&self) -> usize` | Returns current number of items held in the container. | `src/fusion/topk.rs` |
| `pub fn new(capacity: usize) -> Self` | Creates a new `BoundedTopK` container with maximum capacity `k`. | `src/fusion/topk.rs` |
| `pub fn push(&mut self, item: T)` | Pushes an item into top-K container, evicting the worst candidate if size exceeds capacity `k`. | `src/fusion/topk.rs` |
| `pub fn map_is_empty<K, V, S>(m: &ahash::AHashMap<K, V, S>) -> bool` | — | `src/fusion/types.rs` |
| `pub fn new(id: impl Into<String>, score: f32) -> Self` | Erzeugt ein neues `ScoredItem` ohne Provenienz. | `src/fusion/types.rs` |
| `pub fn synthesized_from(source_doc_ids: &[DocId]) -> Self` | Constructs a `ProvenanceRecord` synthesized from source document IDs. | `src/fusion/types.rs` |
| `pub fn with_provenance(mut self, provenance: ProvenanceRecord) -> Self` | Fügt dem Element Provenienzinformationen hinzu. | `src/fusion/types.rs` |
| `pub fn as_bytes(&self) -> &[u8` | Gibt die rohen Bytes des Fingerprints zurück. | `src/local_types.rs` |
| `pub fn from_key(key: &str) -> Result<Self, String>` | Erzeugt eine `DocId` aus einer hexadezimalen oder numerischen Zeichenkette. | `src/local_types.rs` |
| `pub fn new( model_id: impl Into<String>, quantization: impl Into<String>, prompt_template: &str, temperature: f32, ) -> Self` | Erzeugt einen Fingerprint aus Modell, Quantisierung, Template und Temperatur. | `src/local_types.rs` |
| `pub fn strategy_for_signal(&self, signal_name: &str) -> FusionStrategy` | Gibt die Fusionsstrategie für ein bestimmtes Signal ("vector", "text", "graph") zurück. | `src/local_types.rs` |
| `pub fn mean_reciprocal_rank<'a, I>(queries: &[(&[I], &AHashSet<&str>)]) -> f32 where I: AsRef<str> + 'a,` | Berechnet den Mean Reciprocal Rank (MRR) über mehrere Abfragen hinweg. Liefert das arithmetische Mittel der Reciprocal Ranks aller Abfragen… | `src/metrics.rs` |
| `pub fn ndcg_at_k<'a, I>(ranked_ids: &[I], relevance_scores: &AHashMap<&str, f32>, k: usize) -> f32 where I: AsRef<str> + 'a,` | Berechnet den Normalized Discounted Cumulative Gain (nDCG@k). Setzt den tatsächlichen DCG@k ins Verhältnis zum idealen DCG@k (IDCG@k) bei p… | `src/metrics.rs` |
| `pub fn precision_at_k<'a, I>(ranked_ids: &[I], relevant_ids: &AHashSet<&str>, k: usize) -> f32 where I: AsRef<str> + 'a,` | Berechnet Precision@k (Genauigkeit der Top-k-Ergebnisse). Anteil der relevanten Dokumente unter den ersten `k` Treffern: `|relevant ∩ top_k… | `src/metrics.rs` |
| `pub fn recall_at_k<'a, I>(ranked_ids: &[I], relevant_ids: &AHashSet<&str>, k: usize) -> f32 where I: AsRef<str> + 'a,` | Berechnet Recall@k (Trefferquote bezogen auf alle bekannten relevanten Dokumente). Anteil der gefundenen relevanten Dokumente unter den ers… | `src/metrics.rs` |
| `pub fn reciprocal_rank<'a, I>(ranked_ids: &[I], relevant_ids: &AHashSet<&str>) -> f32 where I: AsRef<str> + 'a,` | Berechnet den Reciprocal Rank (RR) für eine geordnete Liste von Treffer-IDs. Findet die 1-basierte Rangposition des ersten relevanten Dokum… | `src/metrics.rs` |
| `pub fn apply_temporal_validity_filter( results: Vec<crate::fusion::SearchResult>, current_tx: TxId, query_timestamp: Option<u64>, ) -> Vec<crate::fusion::SearchResult>` | Applies bi-temporal validity filtering to candidates post-RRF fusion. Filters out candidate results that are invalid relative to the curren… | `src/temporal.rs` |
| `pub fn apply_temporal_validity_filter_at( results: Vec<crate::fusion::SearchResult>, as_of_timestamp: u64, ) -> Vec<crate::fusion::SearchResult>` | Applies historical "as-of" bi-temporal validity filtering to candidates post-RRF fusion. Evaluates candidate validity at historical point-i… | `src/temporal.rs` |
| `pub fn apply_temporal_validity_filter_with_clock( results: Vec<crate::fusion::SearchResult>, current_tx: TxId, clock: &dyn crate::calibration::Clock, ) -> Vec<crate::fusion::SearchResult>` | Applies bi-temporal validity filtering using an injected `Clock` port trait (B11). | `src/temporal.rs` |
| `pub fn extract_validity_window(metadata: Option<&Value>) -> ValidityWindow` | Extracts bi-temporal validity window metadata from candidate document metadata. | `src/temporal.rs` |
| `pub fn is_empty(&self) -> bool` | Returns `true` if no validity window metadata constraints are set. Candidates without validity window metadata (legacy data) are treated as… | `src/temporal.rs` |
| `pub fn is_valid_bitemporal(&self, as_of_tx: TxId, as_of_business: Option<u64>) -> bool` | Evaluates bi-temporal validity using strict AND logic across system transaction time and business time. Intervall-Semantik: `[valid_from, v… | `src/temporal.rs` |
| `pub fn sort_candidates_deterministic(results: &mut [crate::fusion::SearchResult])` | Sorts candidates deterministically by score descending, breaking ties by document ID ascending (B11). | `src/temporal.rs` |


---

## `acta-store`

| Attribut | Wert |
|---|---|
| **Ring** | 1 — Ebene E1 — Speicherschicht, Indizes & KI-Kerne |
| **Status** | Fertig |
| **Zweck** | LSM-Tree Storage Engine mit WAL-Group-Commit und HMAC-Integritätsprüfung. |
| **Unsafe-Code** | Nein (`#![forbid(unsafe_code)]`) |
| **Interne Abhängigkeiten** | `acta-crypto`, `acta-env`, `acta-ids`, `acta-mvcc`, `acta-ports`, `acta-sys`, `acta-types` |
| **Externe Abhängigkeiten (Auswahl)** | `ahash`, `blake3`, `bytes`, `crc32fast`, `lru`, `memmap2`, `parking_lot`, `quick_cache`, `rand`, `serde`, `thiserror`, `tokio`, `tokio-util`, `tracing`, `uuid` |

### Architekturbeschreibung

# acta-store (Ring 1)  Die Crate `acta-store` implementiert das persistente LSM-Tree-Speichertriebwerk (Log-Structured Merge-tree) für die Acta-Plattform. Sie stellt die primäre Datenspeicherebene bereit und bietet atomare Schreibvorgänge, Konsistenz über Snapshots sowie Ausfallsicherheit durch Write-Ahead-Logging (WAL).  ## Architektur & Datenpfad - **Client / Transaktion:** Sendet atomare Schreiboperationen (Put, Delete) über Transaktionspuffer. - **WAL (Write-Ahead-Log):** Persistiert alle Mutationen sequentiell auf Festplatte inklusive CRC32C- und HMAC-SHA256-Integritätsprüfung, bevor sie im RAM bestätigt werden. - **MemTable:** Haltebereich im Arbeitsspeicher (geschardete BTreeMap), der schnelle Suchen und Einfügungen bei geordneten Sequenznummern ermöglicht. - **Flush & SSTable:** Bei Erreichen von Größenschwellen wird die aktive MemTable eingefroren und asynchron als unveränderliche SSTable (Sorted String Table, Format V4 Standard) auf Platte geschrieben. - **Compaction:** Hintergrundprozess, der SSTables hierarchisch (Leveled / Adaptive) zusammenführt, Tombstones bereinigt und Lese- sowie Speicherverstärkung minimiert.  ## Sicherheitsinvariante Die gesamte Crate erzwingt `#![forbid(unsafe_code)]` in Produktion. Es existieren keinerlei `unsafe`-Blöcke im Produktionscode.  ## Ring-Hierarchie & Abhängigkeiten - **Ring 1 / Klasse B:** Kernkomponente des Storage-Bundles (`types`, `ports`, `mvcc`, `crypto`, `sys`, `store`). - **Eingehende Abhängigkeiten:** Wird genutzt von `acta-engine`, `acta-db`, `acta-checkpoint`, `acta-agent` und der übergeordneten Fassade `acta`. - **Ausgehende Abhängigkeiten:** Nutzt `acta-types`, `acta-ports`, `acta-crypto`, `acta-sys`, `acta-mvcc`.

### Modul- und Re-Export-Oberfläche (lib.rs)

```rust
pub mod error;
pub mod traits;
pub mod compat;
pub mod budget;
pub mod stats;
pub mod compaction;
pub mod engine;
pub mod fsync;
pub mod history;
pub mod key_source;
pub mod lsm;
pub mod manifest;
pub mod memtable;
pub mod scanner;
pub mod sstable;
pub mod system_pressure;
pub mod tenant_codec;
pub mod util;
pub mod wal;
pub mod io_seam;
pub use error::{StoreError, StoreResult};
pub use util::{RecordAuthenticator, RecordSealer, SealedEnvelope};
pub use compaction::{ adaptive::{ AdaptiveCompactionPlan, AdaptiveCompactionPlanner, CompactionStrategy, CostBasedAdaptivePlanner, WorkloadMetrics, WorkloadMetricsSnapshot, }, CompactionConfig, CompactionEngine, MergeOperator, TtlMetadata, };
pub use key_source::{KeyMaterialSource, OsKeyMaterialSource, StaticKeyMaterialSource};
pub use lsm::{ CommittedBatch, LsmConfig, LsmStorage, ObserverRegistry, WalEntryRef, WalObserver, WriteOrigin, DEFAULT_MAX_OBSERVER_LATENCY, };
pub use manifest::{Manifest, ManifestEntry};
pub use system_pressure::{PressureLevel, SystemPressure, SystemPressureMonitor};
pub use tenant_codec::{TenantKeyCodec, TenantScopedStorage};
pub use io_seam::{RealFs, RealFsFile, StoreIo, Vfs, VfsFile, VfsOpenOptions};
```

### Feature-Flags

| Flag | Aktiviert |
|---|---|
| `default` | `[]` |
| `workspace` | `["dep:acta-types", "dep:acta-ports"]` |
| `test-hooks` | `[]` |
| `inspect` | `[]` |
| `memory-only-storage` | `[]` |
| `fault-injection` | `[]` |
| `block-cache-v2` | `["dep:quick_cache"]` |
| `sieve-cache` | `[]` |
| `docid-128` | `["acta-ids/docid-128"]` |
| `legacy-wal-key` | `[]` |

### Schnittstellen — Public Traits (17)

**`AdaptiveCompactionPlanner`** — *Trait defining adaptive compaction strategy selection.* (`src/compaction/adaptive/planner.rs`)

```rust
pub trait AdaptiveCompactionPlanner : Send + Sync {
/// Derives a compaction decision based on storage stats and operational workload metrics.
    fn plan_compaction(
        &self,
        stats: &StorageStats,
        metrics: &WorkloadMetricsSnapshot,
        sstables: &[Arc<SstableReader>],
        min_active_seqno: u64,
    ) -> Result<Option<AdaptiveCompactionPlan>>;
}
```

**`BlockCacheBackend`** — ** (`src/sstable/block_cache/traits.rs`)

```rust
pub trait BlockCacheBackend : Send + Sync {
/// Creates a new cache backend instance with the specified capacity.
    fn new(capacity: usize) -> Self
    where
        Self: Sized;

    /// Retrieves a cached block by `(file_id, offset)` key.
    fn get(&self, key: &(u64, u64)) -> Option<Bytes>;

    /// Inserts a block into the cache.
    fn insert(&self, key: (u64, u64), value: Bytes);

    /// Returns the number of cached blocks in this backend instance.
    fn len(&self) -> usize;

    /// Returns `true` if the backend contains no cached blocks.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Checks if a key is present in the cache.
    fn contains(&self, key: &(u64, u64)) -> bool {
        self.get(key).is_some()
    }

    /// Returns the current memory usage in bytes.
    fn current_bytes(&self) -> usize;

    /// Returns the capacity limit in bytes.
    fn capacity_bytes(&self) -> usize;
}
```

**`KeyMaterialSource`** — *Port trait for key and salt material generation.* (`src/key_source.rs`)

```rust
pub trait KeyMaterialSource : Send + Sync + Debug + 'static {
/// Fills dest with random bytes.
    fn fill_key_bytes(&self, dest: &mut [u8]) -> Result<()>;
}
```

**`MergeOperator`** — *Trait for merging two raw byte values (`existing_val` from older tier, `new_val` from newer tier) for the same key across different SSTable tiers.* (`src/compaction/merge_operator.rs`)

```rust
pub trait MergeOperator : Send + Sync {
/// Merges two raw byte values for the same key.
    ///
    /// # Fail-Safe Requirement
    /// Implementations running untrusted guest code (e.g. WASM via `acta-sandbox`) MUST
    /// implement fail-safe handling: if execution fails (e.g. fuel exhaustion or trap),
    /// the caller or operator must swallow the error for that single entry, log a `tracing::warn!`,
    /// and retain both original versions in the compacted SSTable without aborting the overall compaction.
    fn merge(&self, existing_val: &[u8], new_val: &[u8]) -> Result<Vec<u8>>;
}
```

**`RecordAuthenticator`** — *Lokaler Trait zur Authentisierung von Datensätzen und Blöcken.* (`src/util.rs`)

```rust
pub trait RecordAuthenticator : Send + Sync {
/// Berechnet ein Authentisierungs-Tag über gegebene Daten.
    ///
    /// # Errors
    /// Liefert einen Fehler zurück, falls das Tag nicht erzeugt werden kann.
    fn authenticate(&self, key: &[u8], data: &[u8]) -> Result<[u8; 32]>;

    /// Verifiziert ein Authentisierungs-Tag gegen gegebene Daten.
    ///
    /// # Errors
    /// Liefert einen Fehler zurück, falls die Verifikation fehlschlägt.
    fn verify(&self, key: &[u8], data: &[u8], expected_tag: &[u8; 32]) -> Result<bool>;
}
```

**`RecordSealer`** — *Lokaler Trait zur Versiegelung von Datensätzen (Ersatz für externe Krypto-Kopplungen).* (`src/util.rs`)

```rust
pub trait RecordSealer : Send + Sync {
/// Versiegelt Klartextdaten deterministisch oder mit Umschlag.
    ///
    /// # Errors
    /// Liefert einen Fehler zurück, falls die Versiegelung fehlschlägt.
    fn seal(&self, key: &[u8], plaintext: &[u8]) -> Result<SealedEnvelope>;

    /// Öffnet einen versiegelten Datensatz.
    ///
    /// # Errors
    /// Liefert einen Fehler zurück, falls die Entsiegelung fehlschlägt.
    fn open(&self, key: &[u8], envelope: &[u8]) -> Result<Vec<u8>>;
}
```

**`ReplayProgressSink`** — ** (`src/wal/replay.rs`)

```rust
pub trait ReplayProgressSink : Send + Sync {
fn on_entry_replayed(&self, seq: WalSeq, entries_total_estimate: Option<u64>);
}
```

**`RetentionPolicy`** — *Entscheidet, ob eine Version physisch verworfen werden darf.* (`src/history.rs`)

```rust
pub trait RetentionPolicy : Send + Sync + std::fmt::Debug {
/// Persistierter Floor der Partition (Zeitreise darunter ⇒ `HorizonExceeded`).
    fn floor(&self, partition: PartitionId) -> u64;

    /// `newer_exists`: eine neuere Version mit `seq <= snapshot_floor` bleibt erhalten
    /// (die Version ist also für keinen Snapshot mehr die sichtbare).
    /// `snapshot_floor`: kleinster gepinnter Snapshot-Seq.
    fn may_drop_version(
        &self,
        partition: PartitionId,
        seq: u64,
        newer_exists: bool,
        snapshot_floor: u64,
    ) -> bool;
}
```

**`StorageEngineHandle`** — *Adapter trait for types that can act as backend for `TenantScopedStorage`.* (`src/compat/engine/tenant_storage.rs`)

```rust
pub trait StorageEngineHandle : Send + Sync + 'static {
fn get_handle<'a>(&'a self, key: &'a [u8]) -> BoxFuture<'a, Result<Option<Bytes>>>;
    fn get_at_seq_handle<'a>(&'a self, key: &'a [u8], seq_no: u64) -> BoxFuture<'a, Result<Option<Bytes>>>;
    fn get_tracked_handle<'a>(&'a self, tx_id: TxId, key: &'a [u8]) -> BoxFuture<'a, Result<Option<Bytes>>>;
    fn get_at_seq_tracked_handle<'a>(&'a self, tx_id: TxId, key: &'a [u8], seq: u64) -> BoxFuture<'a, Result<Option<Bytes>>>;
    fn scan_prefix_tracked_handle<'a>(&'a self, tx_id: TxId, prefix: &'a [u8]) -> BoxFuture<'a, Result<Vec<(Vec<u8>, Vec<u8>)>>>;
    fn supports_ssi_tracking_handle(&self) -> bool;
    fn put_handle<'a>(&'a self, tx_id: TxId, key: &'a [u8], value: &'a [u8]) -> BoxFuture<'a, Result<()>>;
    fn put_if_absent_handle<'a>(&'a self, tx_id: TxId, key: &'a [u8], value: &'a [u8]) -> BoxFuture<'a, Result<bool>>;
    fn put_batch_handle<'a>(&'a self, tx_id: TxId, entries: &'a [(Vec<u8>, Vec<u8>)]) -> BoxFuture<'a, Result<()>>;
    fn delete_handle<'a>(&'a self, tx_id: TxId, key: &'a [u8]) -> BoxFuture<'a, Result<()>>;
    fn delete_many_handle<'a>(&'a self, tx_id: TxId, keys: Vec<Vec<u8>>) -> BoxFuture<'a, Result<u64>>;
    fn delete_prefix_handle<'a>(&'a self, tx_id: TxId, prefix: &'a [u8]) -> BoxFuture<'a, Result<u64>>;
    fn commit_handle<'a>(&'a self, tx_id: TxId) -> BoxFuture<'a, Result<()>>;
    fn rollback_handle<'a>(&'a self, tx_id: TxId) -> BoxFuture<'a, Result<()>>;
    fn rollback_to_tx_handle<'a>(&'a self, tx_id: TxId) -> BoxFuture<'a, Result<()>>;
    fn flush_handle<'a>(&'a self) -> BoxFuture<'a, Result<()>>;
    fn stats_handle<'a>(&'a self) -> BoxFuture<'a, Result<StorageStats>>;
    fn last_seq_no_handle<'a>(&'a self) -> BoxFuture<'a, Result<u64>>;
    fn last_tx_id_handle<'a>(&'a self) -> BoxFuture<'a, Result<TxId>>;
    fn pin_checkpoint_handle<'a>(&'a self, seq_no: u64) -> BoxFuture<'a, Result<()>>;
    fn unpin_checkpoint_handle<'a>(&'a self, seq_no: u64) -> BoxFuture<'a, Result<()>>;
    fn scan_prefix_handle<'a>(&'a self, prefix: &'a [u8]) -> BoxFuture<'a, Result<Vec<(Vec<u8>, Vec<u8>)>>>;
    fn scan_prefix_bounded_handle<'a>(&'a self, prefix: &'a [u8], limit: usize, cursor: Option<&'a [u8]>) -> BoxFuture<'a, Result<(Vec<(Vec<u8>, Vec<u8>)>, Option<Vec<u8>>)>>;
    fn scan_prefix_at_handle<'a>(&'a self, prefix: &'a [u8], seq_no: u64) -> BoxFuture<'a, Result<Vec<(Vec<u8>, Vec<u8>)>>>;
    fn scan_handle<'a>(&'a self, start: Bound<&'a [u8]>, end: Bound<&'a [u8]>, limit: Option<usize>) -> BoxFuture<'a, Result<Vec<(Vec<u8>, Vec<u8>)>>>;
    fn scan_bounded_handle<'a>(&'a self, start: Bound<&'a [u8]>, end: Bound<&'a [u8]>, limit: usize, cursor: Option<&'a [u8]>) -> BoxFuture<'a, Result<(Vec<(Vec<u8>, Vec<u8>)>, Option<Vec<u8>>)>>;
}
```

**`StoreFile`** — *Positional I/O-Zugriff auf eine geöffnete Datei.* (`src/traits.rs`)

```rust
pub trait StoreFile : Send + Sync + Debug + 'static {
/// Liest positionell bei `offset` in `buf`.
    fn read_at(&self, buf: &mut [u8], offset: u64) -> std::io::Result<usize>;

    /// Schreibt positionell bei `offset` aus `buf`.
    fn write_at(&self, buf: &[u8], offset: u64) -> std::io::Result<usize>;

    /// Setzt die Dateilänge.
    fn set_len(&self, len: u64) -> std::io::Result<()>;

    /// Liefert die Dateilänge in Bytes.
    fn len(&self) -> std::io::Result<u64>;

    /// Synchronisiert Datenblöcke auf Platte (`fdatasync`).
    fn sync_data(&self) -> std::io::Result<()>;

    /// Synchronisiert Datenblöcke und Metadaten auf Platte (`fsync`).
    fn sync_all(&self) -> std::io::Result<()>;
}
```

**`StoreFs`** — *Lokale VFS-Naht für den Speicherkern.* (`src/traits.rs`)

```rust
pub trait StoreFs : Send + Sync + 'static {
/// Öffnet eine Datei mit den gegebenen Optionen.
    fn open(&self, path: &Path, opts: &StoreFileOpenOptions) -> StoreResult<Box<dyn StoreFile>>;

    /// Liest eine Datei vollständig ein.
    fn read(&self, path: &Path) -> StoreResult<Vec<u8>>;

    /// Schreibt Daten atomar und crash-konsistent.
    fn write_atomic(&self, path: &Path, data: &[u8]) -> StoreResult<()>;

    /// Erzeugt ein Verzeichnis und alle Elternverzeichnisse.
    fn create_dir_all(&self, path: &Path) -> StoreResult<()>;

    /// Löscht eine Datei.
    fn remove_file(&self, path: &Path) -> StoreResult<()>;

    /// Benennt eine Datei atomar um.
    fn rename(&self, from: &Path, to: &Path) -> StoreResult<()>;

    /// Prüft, ob ein Pfad existiert.
    fn exists(&self, path: &Path) -> StoreResult<bool>;

    /// Liefert die Dateigröße.
    fn len(&self, path: &Path) -> StoreResult<u64>;

    /// Listet Dateinamen in einem Verzeichnis auf.
    fn list_dir(&self, path: &Path) -> StoreResult<Vec<String>>;

    /// Synchronisiert Verzeichnis-Metadaten (`fsync` auf Directory).
    fn sync_dir(&self, path: &Path) -> StoreResult<()>;
}
```

**`SystemPressureSignal`** — *Injektions-Trait für externe Speicherdruck-Signale (ohne globale Zustände).* (`src/system_pressure/signal.rs`)

```rust
pub trait SystemPressureSignal : Send + Sync {
/// Liefert den aktuellen Druckpegel.
    fn current_pressure(&self) -> PressureLevel;

    /// Gibt eine empfohlene Durchsatz-Reduktionsrate (0.0 bis 1.0) zurück.
    fn advise_reduction_ratio(&self) -> f32;
}
```

**`Vfs`** — *Abstract virtual file system for acta-store.* (`src/io_seam.rs`)

```rust
pub trait Vfs : Send + Sync + 'static {
fn open(&self, path: &Path, opts: &VfsOpenOptions) -> Result<Box<dyn VfsFile>>;
    fn write_atomic(&self, path: &Path, data: &[u8]) -> Result<()>;
    fn create_dir_all(&self, path: &Path) -> Result<()>;
    fn remove_file(&self, path: &Path) -> Result<()>;
    fn rename(&self, from: &Path, to: &Path) -> Result<()>;
    fn read_dir(&self, path: &Path) -> Result<Vec<PathBuf>>;
    fn exists(&self, path: &Path) -> Result<bool>;
    fn len(&self, path: &Path) -> Result<u64>;
    fn list_dir(&self, path: &Path) -> Result<Vec<String>>;
    fn sync_dir(&self, path: &Path) -> Result<()>;
    fn read(&self, path: &Path) -> Result<Vec<u8>> {
        std::fs::read(path).map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                StoreError::NotFound(path.display().to_string())
            } else {
                StoreError::from(e)
            }
        })
    }
}
```

**`Vfs`** — *Abstraktes virtuelles Dateisystem für `acta-store`.* (`src/io_seam/types.rs`)

```rust
pub trait Vfs : Send + Sync + 'static {
/// Öffnet eine Datei mit den übergebenen Optionen.
    fn open(&self, path: &Path, opts: &VfsOpenOptions) -> Result<Box<dyn VfsFile>>;
    /// Schreibt Daten atomar über temporäre Datei und Rename.
    fn write_atomic(&self, path: &Path, data: &[u8]) -> Result<()>;
    /// Erzeugt ein Verzeichnis rekursiv.
    fn create_dir_all(&self, path: &Path) -> Result<()>;
    /// Entfernt eine Datei.
    fn remove_file(&self, path: &Path) -> Result<()>;
    /// Benennt eine Datei atomar um.
    fn rename(&self, from: &Path, to: &Path) -> Result<()>;
    /// Liest Einträge eines Verzeichnisses als Pfade.
    fn read_dir(&self, path: &Path) -> Result<Vec<PathBuf>>;
    /// Prüft auf Existenz eines Pfads.
    fn exists(&self, path: &Path) -> Result<bool>;
    /// Liefert die Dateigröße.
    fn len(&self, path: &Path) -> Result<u64>;
    /// Listet Dateinamen in einem Verzeichnis als Strings auf.
    fn list_dir(&self, path: &Path) -> Result<Vec<String>>;
    /// Synchronisiert Verzeichnis-Metadaten.
    fn sync_dir(&self, path: &Path) -> Result<()>;
    /// Liest eine Datei vollständig ein.
    fn read(&self, path: &Path) -> Result<Vec<u8>> {
        std::fs::read(path).map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                StoreError::NotFound(path.display().to_string())
            } else {
                StoreError::from(e)
            }
        })
    }
}
```

**`VfsFile`** — *Abstract positional file handle.* (`src/io_seam.rs`)

```rust
pub trait VfsFile : Send + Sync + 'static {
fn read_at(&self, buf: &mut [u8], offset: u64) -> std::io::Result<usize>;
    fn write_at(&self, buf: &[u8], offset: u64) -> std::io::Result<usize>;
    fn set_len(&self, len: u64) -> std::io::Result<()>;
    fn len(&self) -> std::io::Result<u64>;
    fn sync_data(&self) -> std::io::Result<()>;
    fn sync_all(&self) -> std::io::Result<()>;
}
```

**`VfsFile`** — *Abstraktes positionelles Dateihandle.* (`src/io_seam/types.rs`)

```rust
pub trait VfsFile : Send + Sync + 'static {
/// Liest positionell ab `offset` in `buf`.
    fn read_at(&self, buf: &mut [u8], offset: u64) -> std::io::Result<usize>;
    /// Schreibt positionell ab `offset` aus `buf`.
    fn write_at(&self, buf: &[u8], offset: u64) -> std::io::Result<usize>;
    /// Setzt die Dateilänge.
    fn set_len(&self, len: u64) -> std::io::Result<()>;
    /// Liefert die Dateilänge in Bytes.
    fn len(&self) -> std::io::Result<u64>;
    /// Synchronisiert Datenblöcke auf Platte (`fdatasync`).
    fn sync_data(&self) -> std::io::Result<()>;
    /// Synchronisiert Datenblöcke und Metadaten auf Platte (`fsync`).
    fn sync_all(&self) -> std::io::Result<()>;
}
```

**`WalObserver`** — *Synchronous, deterministic observer interface for committed WAL batches.* (`src/lsm/observer/types.rs`)

```rust
pub trait WalObserver : Send + Sync {
/// Indicates whether this observer requires durable WAL commits.
    ///
    /// Returns `false` by default. Observers setting this to `false` will also receive notifications
    /// for `DurabilityMode::MemoryOnly` commits.
    fn requires_durability(&self) -> bool {
        false
    }

    /// Notification callback executed upon batch commit.
    ///
    /// Invoked via a bounded, non-blocking dispatcher mechanism. The commit path enforces a
    /// configurable maximum latency deadline (`max_observer_latency`). Observers taking longer
    /// than this deadline will time out on the commit path, increment drop counters, and trip
    /// the circuit breaker to prevent blocking subsequent transaction commits.
    fn on_commit(&self, batch: &CommittedBatch<'_>, seq_no: u64, tx_id: TxId);
}
```

### Öffentliche Datentypen — Structs (66)

| Struct | Felder / Form | Beschreibung | Datei |
|---|---|---|---|
| `AdaptiveCompactionPlan` | `{ /// The chosen compaction strategy. pub strategy: CompactionStrategy, /// Candidates SSTables to compact. pub candidates: Vec<Arc<SstableReader>>, /// Whether… }` | Resulting plan proposed by adaptive compaction planner. | `src/compaction/adaptive/planner.rs` |
| `AsyncObserverAdapter` | `{ inner: Arc<dyn WalObserver>, sender: tokio::sync::mpsc::Sender<(u64, TxId, WriteOrigin)>, }` | Non-blocking, bounded async adapter wrapper for `WalObserver` implementations. | `src/lsm/observer/adapter.rs` |
| `BlockBuilder` | `{ data: BytesMut, offsets: Vec<u32>, pub(crate) block_size: usize, key_hashes: Vec<(u64, u64)>, v3_bloom: [u64; 4], format_version: u16, }` | A builder for SSTable data blocks (Format v4 default, supports v3). | `src/sstable/builder/block.rs` |
| `BlockCache` | `{ shards: Vec<BlockCacheShard>, hash_builder: ahash::RandomState, hits: std::sync::atomic::AtomicU64, misses: std::sync::atomic::AtomicU64, }` | — | `src/sstable/block_cache/mod.rs` |
| `BloomFilter` | `{ pub(super) bits: Vec<u64>, pub(super) num_hashes: usize, pub(super) num_bits: usize, }` | SPECCED: Speichereffizienter Bloom-Filter für SSTable-Pre-Checks. Ziel: False-Positive-Rate p ≤ 0.01. Formel: m = -n * ln(p) / (ln(2)^2) ≈ 9.6 * n Ha… | `src/sstable/bloom.rs` |
| `CommitContext` | `{ pub tx_id: TxId, pub origin: WriteOrigin, pub durable: bool, }` | Context describing a committed transaction passed to observers. | `src/lsm/observer/types.rs` |
| `CommitGuard` | `{ pub(super) _lock: &'a tokio::sync::MutexGuard<'a, ()>, }` | Proof that `commit_mutex` is currently held by the calling task. Can only be constructed while holding the mutex guard. | `src/lsm/guard.rs` |
| `CommittedBatch` | `{ pub entries: Vec<WalEntryRef<'a>>, pub origin: WriteOrigin, }` | Batch of committed WAL entries passed to observers. | `src/lsm/observer/types.rs` |
| `CompactionConfig` | `{ /// Minimum number of SSTables in a size tier to trigger compaction. pub min_sstables_per_tier: usize, /// Number of SSTables in L0 that trigger compaction. pu… }` | Configuration for the compaction engine. | `src/compaction/config.rs` |
| `CompactionEngine` | `{ config: CompactionConfig, pub(super) snapshot_registry: Arc<SnapshotRegistry>, block_cache: Arc<BlockCache>, key_manager: Option<Arc<KeyManager>>, budget: Arc<… }` | Orchestriert die periodische Verdichtung und Bereinigung von SSTables. | `src/compaction/engine.rs` |
| `CostBasedAdaptivePlanner` | `{ read_ratio_threshold: f64, min_sstables_per_tier: usize, size_ratio: f64, }` | Reference implementation of cost-based adaptive compaction (EcoTune / ArceKV inspired). Adjusts compaction aggressiveness based on read-vs-write work… | `src/compaction/adaptive/cost_based.rs` |
| `ErasureRecord` | `{ pub id: [u8; 16], pub state: ErasureState, pub handle: [u8; 16], pub seq: u64, }` | Ein Journal-Eintrag der Löschkette: `(erasure_id, state, handle, seq)`. `seq` ist bei `Requested` der Antragszeitpunkt (Unix-Sekunden), ab `Shredded`… | `src/manifest/entry.rs` |
| `FlushPurgeFence` | `{ flush_active: AtomicBool, purge_active: AtomicBool, }` | Koordiniert das gegenseitige Fencing zwischen MemTable-Flush und Purge-Kompaktion. Atomares Fencing: Garantierter wechselseitiger Ausschluss über Seq… | `src/lsm/fence.rs` |
| `InMemoryStorageEngine` | `{ data: Arc<RwLock<StoreDataMap>>, staged: Arc<RwLock<StagedTxMap>>, seq: Arc<AtomicU64>, last_tx: Arc<RwLock<TxId>>, }` | An in-memory implementation of [`StorageEngine`] guaranteed to perform no disk I/O. | `src/compat/engine/memory_only.rs` |
| `LoomFile` | `{ pub pos: usize, pub buf: Vec<u8>, }` | — | `src/wal/fs.rs` |
| `LoomMetadata` | `(tuple/unit)` | — | `src/wal/fs.rs` |
| `LruBlockCacheBackend` | `{ state: RwLock<LruBlockCacheState>, }` | — | `src/sstable/block_cache/lru.rs` |
| `LsmConfig` | `{ /// Path to the data directory. pub path: PathBuf, /// Maximum size of the memtable before flushing to disk. pub memtable_size_limit: usize, /// Maximum RAM us… }` | Configuration for the LSM storage engine. | `src/lsm/config.rs` |
| `LsmPurger` | `{ storage: Arc<LsmStorage>, runtime: ScannerRuntime, }` | Purger für LSM Storage Engine. | `src/compat/scanner/purge.rs` |
| `LsmState` | `{ pub(super) memtable: Arc<MemTable>, pub(super) immutable_memtables: Vec<Arc<MemTable>>, }` | — | `src/lsm/guard.rs` |
| `LsmStorage` | `{ pub(super) config: LsmConfig, pub(super) key_manager: Option<Arc<KeyManager>>, pub(super) state: RwLock<LsmState>, /// SSTables stored separately for shared ac… }` | Primaere Schnittstelle fuer den LSM-Baum mit get, put, delete und scan. | `src/lsm/engine.rs` |
| `Manifest` | `{ pub(super) path: PathBuf, pub(super) file: tokio::sync::Mutex<tokio::fs::File>, pub(super) _dir_lock: Option<DirLock>, pub(super) is_poisoned: AtomicBool, }` | Append-only Manifest file handle for recording active SSTable set transitions. | `src/manifest/handle.rs` |
| `MemTableShard` | `{ pub entries: RwLock<MemTableMap>, }` | A single shard of the MemTable, holding a subset of the key space. | `src/memtable/shard.rs` |
| `MemtableScanner` | `{ view: StoreScanView, #[allow(dead_code)] runtime: ScannerRuntime, }` | Scanner für die `LsmMemtable`-Schicht. Hält [`StoreScanView`] statt `Arc<LsmStorage>` — keine direkte Kopplung an die Speicher-Engine. Kann daher mit… | `src/compat/scanner/memtable.rs` |
| `NoopReplayProgressSink` | `(tuple/unit)` | — | `src/wal/replay.rs` |
| `ObserverRegistry` | `{ observers: parking_lot::RwLock<Vec<Arc<BoundedObserverWorker>>>, historical_drops: Arc<AtomicU64>, max_observer_latency: parking_lot::RwLock<Duration>, clock:… }` | Configuration and handle for managing WAL observers with bounded delivery and circuit breaker. | `src/lsm/observer/registry.rs` |
| `OpenOptions` | `(tuple/unit)` | — | `src/wal/fs.rs` |
| `OsKeyMaterialSource` | `(tuple/unit)` | Default production implementation using OS CSPRNG. | `src/key_source.rs` |
| `PartitionRetention` | `{ per_partition: BTreeMap<PartitionId, Retention>, floors: BTreeMap<PartitionId, u64>, }` | Standard-Policy: `Retention` pro Partition, Rückfall auf `LiveWhileKeyAlive` (entspricht dem bisherigen Verhalten von `retain_key_versions`). | `src/history.rs` |
| `PreparedBatch` | `(tuple/unit)` | A batch of prepared WAL entries bound to a specific HMAC chain. `PreparedBatch` has a private inner field and cannot be constructed outside `wal.rs`.… | `src/wal/encode.rs` |
| `QuickCacheBlockCacheBackend` | `{ cache: quick_cache::sync::Cache<(u64, u64), Bytes, BlockWeighter>, }` | Lock-optimized block cache backend using `quick_cache::sync::Cache` (S3-FIFO / Clock-based eviction) with byte-based capacity. | `src/sstable/block_cache/quick.rs` |
| `RealFs` | `(tuple/unit)` | Default POSIX/OS filesystem implementation of [] for acta-store. | `src/io_seam.rs` |
| `RealFs` | `(tuple/unit)` | Standard POSIX/Betriebssystem-Dateisystemimplementierung für `acta-store`. | `src/io_seam/real_fs.rs` |
| `RealFsFile` | `{ file: std::fs::File, }` | Standard [] handle based on []. | `src/io_seam.rs` |
| `RealFsFile` | `(tuple/unit)` | Dateihandle für `RealFs`. | `src/io_seam/real_fs.rs` |
| `ResourceBudget` | `{ /// Memory limit in bytes (0 = unlimited). pub memory_limit: usize, }` | Resource budget configuration. | `src/budget.rs` |
| `ResourceTracker` | `{ budget: ResourceBudget, memory_used: AtomicU64, }` | Thread-safe tracker for memory allocations. | `src/budget.rs` |
| `ScannerRuntime` | `{ handle: Option<Handle>, }` | Laufzeit-Ausführungsstrategie für synchrone Scanner- und Purger-Ports. SPEC §3.4 (Sync-Kern / Async-Schale) & SPEC E-07 (Zero-Panic-Doktrin): Synchro… | `src/compat/scanner/runtime.rs` |
| `SealedEnvelope` | `{ payload: Vec<u8>, }` | Kapselung eines versiegelten Datensatzes. | `src/util.rs` |
| `SieveCacheBackend` | `{ state: RwLock<SieveCacheState>, }` | — | `src/sstable/block_cache/sieve.rs` |
| `SstableBuilder` | `{ path: PathBuf, tmp_path: PathBuf, file: Option<Box<dyn VfsFile>>, vfs: Arc<dyn Vfs>, block_builder: BlockBuilder, block_size: usize, index: Vec<(Bytes, u64)>,… }` | — | `src/sstable/builder/writer.rs` |
| `SstableFooter` | `{ /// Format-Version (z. B. 4 für MFSX v4, 0/1 für frühere MFSX, 3 für Legacy). pub format_version: u16, /// Gibt an, ob es sich um das moderne MFSX-Format hande… }` | Strukturierte Repräsentation des SSTable-Footers / Trailers. | `src/sstable/format.rs` |
| `SstableMetadata` | `{ pub first_key: Bytes, pub last_key: Bytes, pub file_size: u64, pub min_tx_id: u64, pub max_tx_id: u64, pub min_seq: u64, pub max_seq: u64, }` | — | `src/sstable/builder/meta.rs` |
| `SstableReader` | `{ pub(super) file: Arc<dyn VfsFile>, pub(super) index: Vec<(Bytes, u64)>, pub(super) metadata: SstableMetadata, /// Byte offset where the index data begins (= en… }` | Liest SSTable-Dateien mit Block-Cache und Bloom-Filter-Unterstuetzung. | `src/sstable/reader/mod.rs` |
| `SstableScanner` | `{ view: StoreScanView, runtime: ScannerRuntime, }` | Scanner für die `SsTableAllLevels`-Schicht. | `src/compat/scanner/sstable.rs` |
| `SstableStream` | `{ pub(super) reader: Arc<SstableReader>, pub(super) block_idx: usize, pub(super) entry_idx: usize, pub(super) current_block: Option<Bytes>, pub(super) num_offset… }` | — | `src/sstable/stream.rs` |
| `StaticKeyMaterialSource` | `{ key_stream: std::sync::Arc<parking_lot::Mutex<Vec<u8>>>, }` | Fixed deterministic key material source for reproducible testing and fault simulation. | `src/key_source.rs` |
| `StorageStats` | `{ /// Number of SSTable segments. pub num_segments: usize, /// Total size of all SSTables in bytes. pub total_size_bytes: u64, /// Total size of memtables in byt… }` | Detailed operational and storage statistics. | `src/stats.rs` |
| `StoreFileOpenOptions` | `{ /// Lesezugriff aktivieren. pub read: bool, /// Schreibzugriff aktivieren. pub write: bool, /// Datei anlegen, falls nicht vorhanden. pub create: bool, /// Dat… }` | Öffnungs-Optionen für `StoreFile`. | `src/traits.rs` |
| `StoreIo` | `{ vfs: Arc<dyn Vfs>, }` | Central Store I/O abstraction () wrapping . | `src/io_seam.rs` |
| `StoreIo` | `{ vfs: Arc<dyn Vfs>, }` | Konkrete asynchrone I/O-Hülle für den LSM-Kern. | `src/io_seam/store_io.rs` |
| `StoreScanView` | `{ /// Aktive MemTable (Snapshot zum Erstellungszeitpunkt). pub(crate) active_memtable: Arc<MemTable>, /// Immutable MemTables (Snapshot zum Erstellungszeitpunkt)… }` | Schlanke, entkoppelte Lesezugriffssicht auf LSM-Speicherschichten für Scanner und Purger. Kann direkt aus einem `Arc<LsmStorage>` erzeugt oder für Te… | `src/compat/scanner/store_scan_view.rs` |
| `SystemPressure` | `{ /// Warteschlangentiefe anstehender WAL-Schreiboperationen. pub wal_queue_depth: usize, /// Tokio Scheduler Queue-Verhältnis (0.0 bis 1.0). pub scheduler_queue… }` | Aggregierter Systemdruck-Zustand. | `src/system_pressure/calculation.rs` |
| `SystemPressureMonitor` | `{ pressure_tx: tokio::sync::watch::Sender<SystemPressure>, /// Empfänger-Kanal für Druckaktualisierungen. pub pressure_rx: tokio::sync::watch::Receiver<SystemPre… }` | Periodischer Monitor zur Erfassung von Systemdruck-Signalen. | `src/system_pressure/monitor.rs` |
| `TenantKeyCodec` | `{ tenant_id: TenantId, /// Vorberechnetes Scan-Präfix ohne Allokation im Hot-Path. scan_prefix_cache: Vec<u8>, }` | Encodes LSM storage keys with tenant isolation prefixes. | `src/tenant_codec.rs` |
| `TenantScopedStorage` | `{ #[allow(dead_code)] pub(crate) inner: S, pub(crate) codec: TenantKeyCodec, }` | — | `src/tenant_codec.rs` |
| `VfsOpenOptions` | `{ pub read: bool, pub write: bool, pub create: bool, pub create_new: bool, pub truncate: bool, }` | Options configuring how a file is opened via []. | `src/io_seam.rs` |
| `VfsOpenOptions` | `{ /// Lesezugriff aktivieren. pub read: bool, /// Schreibzugriff aktivieren. pub write: bool, /// Datei erzeugen, falls nicht existent. pub create: bool, /// Dat… }` | Optionen zum Öffnen von Dateien über das VFS. | `src/io_seam/types.rs` |
| `Wal` | `{ pub(crate) path: PathBuf, pub(crate) size: Arc<std::sync::atomic::AtomicU64>, pub(crate) header_written: Arc<std::sync::atomic::AtomicBool>, pub(crate) key_man… }` | Hauptstruktur fuer das Write-Ahead-Log mit Append- und Replay-Operationen. | `src/wal/core.rs` |
| `WalConfig` | `{ pub key_manager: Option<Arc<KeyManager>>, pub(crate) allow_legacy_integrity_key_fallback: bool, pub min_wal_version: WalVersion, pub flusher_config: WalFlusher… }` | Konfigurationsoptionen für das Write-Ahead-Log (WAL). Steuert Schlüsselmanager, Integritätsschlüssel-Rückfallebenen, Mindestversion und Flusher-Einst… | `src/wal/config.rs` |
| `WalEntry` | `{ /// The operation performed. pub op: WalOp, /// Sequence number assigned to the operation. pub seq_no: u64, /// HMAC of the current entry (includes previous HM… }` | A single entry in the Write-Ahead Log. | `src/wal/encode.rs` |
| `WalEntryRef` | `{ pub entry: &'a WalEntry, }` | Lightweight view reference into a `WalEntry`. | `src/lsm/observer/types.rs` |
| `WalFlusherConfig` | `{ /// Maximum time to wait for additional messages after receiving the first, /// before issuing `sync_all()`. Set to 0 to disable (immediate flush). /// /// Typ… }` | Configuration for the WAL background flusher actor. Controls how aggressively the flusher coalesces concurrent writes into a single `sync_all()` call… | `src/wal/flusher.rs` |
| `WalScanner` | `{ view: StoreScanView, runtime: ScannerRuntime, }` | Scanner für die `WalAllSegments`-Schicht. | `src/compat/scanner/wal.rs` |
| `WorkloadMetrics` | `{ read_count: AtomicU64, write_count: AtomicU64, last_seq_no: AtomicU64, }` | Operational workload metrics captured deterministically without wall-clock time (P28 compliance). All trigger decisions are strictly derived from tra… | `src/compaction/adaptive/metrics.rs` |
| `WorkloadMetricsSnapshot` | `{ /// Total read operations recorded. pub read_count: u64, /// Total write operations recorded. pub write_count: u64, /// Highest observed transaction sequence n… }` | Point-in-time deterministic snapshot of operational workload counters. | `src/compaction/adaptive/metrics.rs` |

### Öffentliche Datentypen — Enums (18)

| Enum | Varianten | Beschreibung | Datei |
|---|---|---|---|
| `CompactionStrategy` | `/// Strict leveled/tiered size-tiered compaction (fast writes, higher space/read amplification). WriteOptimizedSTCS, /// Aggressive compaction merging more tiers / full LSM to reduce read amplificati…` | Dynamic compaction strategy chosen by the adaptive planner. | `src/compaction/adaptive/planner.rs` |
| `DurabilityConfigError` | `#[error("DurabilityMode::{mode:?} is incompatible with feature '{feature}': {reason}")] IncompatibleCombination { mode: DurabilityMode, feature: &'static str, reason: &'static str, },` | — | `src/lsm/config.rs` |
| `DurabilityMode` | `Full, WalNoHmac, MemoryOnly,` | — | `src/lsm/config.rs` |
| `ErasureState` | `Requested = 1, Frozen = 2, /// Commit-Punkt der Löschung (D-2). Shredded = 3, Receipted = 4, Purged = 5, DerivedRebuilt = 6, Verified = 7, Closed = 8,` | Zustände der Erasure-State-Machine (Spec §6). Die Reihenfolge ist die zulässige Übergangsreihenfolge; Übergänge sind monoton. | `src/manifest/entry.rs` |
| `LegacyKeyStatus` | `/// A standard, secure integrity key (e.g. from `.wal_integrity_key` or `KeyManager`) is in use. Standard, /// The legacy obfuscated key is active and requires migration. LegacyActive,` | Status indicating whether a WAL segment or key configuration uses the obfuscated legacy key. | `src/wal/hmac.rs` |
| `ManifestEntry` | `/// A new SSTable file was created and fully fsynced. Add { path: PathBuf, max_tx: u64 }, /// An SSTable file was removed or superseded. Remove { path: PathBuf }, /// A transaction rollback operation…` | An entry in the SSTable manifest log. | `src/manifest/entry.rs` |
| `PoisonCause` | `/// Ein `fsync` (`sync_all` oder Verzeichnis-fsync) ist fehlgeschlagen. Irreversibel (fsyncgate). FsyncFailed = 1, /// Ein physischer Schreibaufruf (`write`/`write_all`/`flush`) ist fehlgeschlagen. I…` | Typisierte Ursache für das Vergiften eines WAL-Handles (SPEC §3.5, §12.1, INV-FSYNC-1). Nach einem Fsync-, Write-, Truncate- oder HMAC-Kettendivergen… | `src/wal/poison.rs` |
| `PressureLevel` | `/// Normaler Betriebszustand ohne Drosselung. Normal, /// System unter Last; leichte Verzögerungen empfohlen. Elevated, /// Backpressure aktiv; neue Schreibvorgänge müssen verlangsamt werden. Critica…` | Diskreter Systemdruck-Pegel für Backpressure-Steuerung. | `src/system_pressure/signal.rs` |
| `ReadOutcome` | `Found(T), /// Der Schlüssel existierte zu diesem Seq nachweislich nicht. NotFound, /// Subject geshreddet; wirkt unabhängig vom angefragten Seq (D-4). Erased { erase_seq: u64, erasure_id: [u8; 16] },…` | Ergebnis eines Zeitreise-Reads (Spec §5.2). | `src/history.rs` |
| `Retention` | `/// N (Chronik): nie eine Version verwerfen, auch Tombstones nicht. KeepAll, /// P (Tresor): alle Versionen oberhalb des Snapshot-Floors, darunter nur die jeweils /// neueste („Floor-Version"). Die L…` | Aufbewahrungsklasse einer Partition. | `src/history.rs` |
| `StorageHealth` | `Healthy, FlushFailing,` | Health status of the storage engine. | `src/lsm/engine.rs` |
| `StoreError` | `/// Speicherfehler im LSM-Kern. #[error("Storage error: {0}")] Storage(String), /// Standard I/O-Fehler. #[error("I/O error: {0}")] Io(#[from] std::io::Error), /// Korruption im Write-Ahead-Log. #[er…` | Lokaler Fehlertyp des LSM-Speicherkerns `acta-store`. | `src/error.rs` |
| `VerifiedVia` | `Direct, CacheDropped, Cached,` | Method via which medium verification was performed. | `src/io_seam.rs` |
| `VerifiedVia` | `/// Direkt vom Medium gelesen. Direct, /// Nach Cache-Invalidierung vom Medium gelesen. CacheDropped, /// Aus dem Betriebssystem-/Dateisystem-Cache bedient. Cached,` | Methode, über welche die Medienverifikation durchgeführt wurde. | `src/io_seam/types.rs` |
| `WalOp` | `/// Inserts or updates a key-value pair. Put { tx_id: TxId, key: Vec<u8>, value: Vec<u8>, }, /// Deletes a key. Delete { tx_id: TxId, key: Vec<u8> }, /// Transaction outcome marker (committed / abort…` | — | `src/wal/encode.rs` |
| `WalParseError` | `#[error("WAL entry too short for CRC header")] HeaderTooShort, #[error("CRC mismatch: stored={stored:#010x}, computed={computed:#010x}")] CrcMismatch { stored: u32, computed: u32 }, #[error("WAL payl…` | Typed parsing errors encountered when decoding WAL entries from raw bytes. | `src/wal/encode.rs` |
| `WalVersion` | `V1, // Legacy: kein HMAC V2, // Current: HMAC ohne tx_id V3, // New: HMAC mit tx_id` | — | `src/wal/encode.rs` |
| `WriteOrigin` | `/// Regular user/application write transaction. UserWrite,` | Origin tag indicating where a committed batch originated from. | `src/lsm/observer/types.rs` |

### Typ-Aliase (10)

| Alias | Definition | Beschreibung |
|---|---|---|
| `BlockCacheShard` | `type BlockCacheShard= LruBlockCacheBackend` | — |
| `BlockCacheShard` | `type BlockCacheShard= QuickCacheBlockCacheBackend` | — |
| `File` | `type File= LoomFile` | — |
| `MemTableEntry` | `type MemTableEntry= (SequenceNumber, Bytes, TransactionId)` | — |
| `MemTableMap` | `type MemTableMap= BTreeMap<Bytes, Vec<MemTableEntry>>` | — |
| `PartitionId` | `type PartitionId= u32` | Partitions-ID. Solange der Store nur eine physische Partition kennt, ist das `0`. |
| `SequenceNumber` | `type SequenceNumber= u64` | — |
| `StoreResult` | `type StoreResult<T> = std::result::Result<T, StoreError>` | Ergebnis-Typ fuer Speicheroperationen. |
| `TransactionId` | `type TransactionId= u64` | — |
| `WalSeq` | `type WalSeq= u64` | — |

### Konstanten (41)

| Konstante | Typ | Beschreibung |
|---|---|---|
| `BLOCKING_UTIL_CRITICAL` | `f32` | Kritischer Schwellenwert für Blocking-Pool Auslastung. |
| `BLOCKING_UTIL_ELEVATED` | `f32` | Erhöhter Schwellenwert für Blocking-Pool Auslastung. |
| `BLOCK_CACHE_SHARDS` | `usize` | Sharded block cache: 64 independent shards to reduce contention. Shard selection uses ahash tuple hashing on (file_id, offset). |
| `BLOCK_SIZE` | `usize` | Standard-Blockgröße für SSTable Data Blocks (4 KiB). |
| `CURRENT_MANIFEST_VERSION` | `u8` | — |
| `CURRENT_SSTABLE_FORMAT_VERSION` | `u16` | Aktuelle unterstützte SSTable-Formatversion (V4). |
| `DEFAULT_MAX_OBSERVER_LATENCY` | `Duration` | Default maximum latency allowed for WAL observer callback execution (1 millisecond). |
| `DEFAULT_PARTITION` | `PartitionId` | Die Standard-Partition des Stores. |
| `DEFAULT_WAL_QUEUE_CAPACITY` | `usize` | Default capacity for the bounded WAL flusher command channel. |
| `INTENT_MAGIC` | `&[u8]` | Magische Konstante fuer Rollback-Intent-Dateien. |
| `LEGACY_TRAILER_LEN` | `usize` | Trailer-Länge für Legacy MFST (54 Bytes). |
| `MANIFEST_HEADER_MAGIC` | `&[u8` | — |
| `MANIFEST_VERSION_V1` | `u8` | — |
| `MANIFEST_VERSION_V2` | `u8` | — |
| `MAX_SCAN_MERGE_ACCUMULATOR` | `usize` | — |
| `MAX_WAL_ENTRY_SIZE` | `u32` | Maximum size for a single WAL entry payload (64MB). |
| `MAX_WAL_SIZE` | `u64` | — |
| `MFSX_TRAILER_LEN_V0` | `usize` | Trailer-Länge für MFSX Format Version 0 (52 Bytes). |
| `MFSX_TRAILER_LEN_V1` | `usize` | Trailer-Länge für MFSX Format Version >= 1 (54 Bytes). |
| `MIN_ENTRIES_FOR_SSTABLE_REBUILD` | `usize` | Mindestanzahl an ueberlebenden Eintraegen, um eine neue SSTable zu bauen. |
| `RANK_BLOCK_CACHE` | `u32` | Rang 950: SSTable-Block-Cache-Sperre (`SieveCacheBackend::state`). |
| `RANK_COMMIT_MUTEX` | `u32` | Rang 200: Globaler Commit-Mutex des Stores (`commit_mutex`), entspricht Commit-Barriere. |
| `RANK_DOCUMENT_INTENT` | `u32` | Rang 100: Transaktionale Absichtssperren auf Dokument-/Schlüsselebene (`intent_locks`). |
| `RANK_FLUSH_MUTEX` | `u32` | Rang 300: Flush-Mutex des Stores (`flush_mutex`), serialisiert Flush-Vorgänge. |
| `RANK_MANIFEST_MUTEX` | `u32` | Rang 800: Manifest-Append-Sperre (`Manifest::file`). |
| `RANK_MEMTABLE_SHARDS` | `u32` | Rang 700: MemTable-Shards (`MemTable::shards`), atomare RAM-Modifikationen. |
| `RANK_OBSERVER_REGISTRY` | `u32` | Rang 900: Observer-Registry-Sperre (`ObserverRegistry::observers`). |
| `RANK_SSTABLES_RWLOCK` | `u32` | Rang 600: SSTable-Katalog (`sstables`), schützt die Liste aktiver L0..Ln SSTables. |
| `RANK_STATE_RWLOCK` | `u32` | Rang 400: LSM-Zustand (`state`), schützt MemTable-Zeiger und unveränderliche MemTables. |
| `RANK_WAL_TRUNCATE` | `u32` | Rang 500: WAL-Truncate-Sperre (`wal.truncate_lock`), serialisiert Append/Truncate im WAL. |
| `RESERVED_PREFIXES` | `&[&[u8]]` | List of known reserved system key prefixes. |
| `ROLLBACK_INLINE_THRESHOLD_ENTRIES` | `usize` | Maximum threshold for surviving entries during rollback below which entries are retained in memtable instead of creating a new SSTable (M-4 optimizat… |
| `SCHEDULER_QUEUE_CRITICAL` | `f32` | Kritischer Schwellenwert für die Tokio-Scheduler-Warteschlange (Verhältnis). |
| `SCHEDULER_QUEUE_ELEVATED` | `f32` | Erhöhter Schwellenwert für die Tokio-Scheduler-Warteschlange (Verhältnis). |
| `SHARD_COUNT` | `usize` | Number of independent shards. 16 provides sufficient parallelism for the 8-concurrent-embed pipeline while keeping memory overhead negligible. Must b… |
| `SSTABLE_MAGIC_LEGACY` | `u32` | Magic-Bytes für Legacy SSTable: `MFST` in Hex (0x4D46_5354). |
| `SSTABLE_MAGIC_MFSX` | `u32` | Magic-Bytes für V4 SSTable: `MFSX` in Hex (0x5853_464D). |
| `WAL_QUEUE_CRITICAL_THRESHOLD` | `usize` | Kritischer Schwellenwert für die WAL-Warteschlangentiefe. |
| `WAL_QUEUE_ELEVATED_THRESHOLD` | `usize` | Erhöhter Schwellenwert für die WAL-Warteschlangentiefe. |
| `WAL_V2_HEADER` | `[u8` | Magic header for V2 batch-encrypted WAL files (`b"MFW2"`). |
| `WAL_V3_HEADER` | `[u8` | Magic header for V3 WAL files (`b"MFW3"`). |

### Öffentliche Funktionen (441)

| Signatur | Beschreibung | Datei |
|---|---|---|
| `pub fn consume_memory(&self, bytes: u64) -> StoreResult<()>` | Consumes memory, failing if limit is exceeded. | `src/budget.rs` |
| `pub fn has_memory_capacity(&self) -> bool` | Returns true if memory limit is not reached. | `src/budget.rs` |
| `pub fn memory_used(&self) -> u64` | Returns the current memory used in bytes. | `src/budget.rs` |
| `pub fn new(budget: ResourceBudget) -> Self` | Creates a new ResourceTracker with the specified budget. | `src/budget.rs` |
| `pub fn release_memory(&self, bytes: u64)` | Releases previously consumed memory. | `src/budget.rs` |
| `pub fn compute_key_overlap_score(window: &[Arc<SstableReader>]) -> usize` | Computes a score representing key range contiguity/overlap among SSTables in a window. Returns the number of adjacent overlapping pairs whe… | `src/compaction/adaptive/cost_based.rs` |
| `pub fn new(read_ratio_threshold: f64, min_sstables_per_tier: usize, size_ratio: f64) -> Self` | Creates a new CostBasedAdaptivePlanner with configurable read ratio threshold. | `src/compaction/adaptive/cost_based.rs` |
| `pub fn select_stcs_candidates( ssts: &[Arc<SstableReader>], min_sstables_per_tier: usize, size_ratio: f64, ) -> Option<Vec<Arc<SstableReader>>>` | Helper function to perform standard Size-Tiered Compaction candidate selection. Prefers contiguous key ranges (overlapping or adjacent key… | `src/compaction/adaptive/cost_based.rs` |
| `pub fn validate_tombstone_safety( candidates: &[Arc<SstableReader>], min_active_seqno: u64, is_full_compaction: bool, ) -> bool` | Hard invariant verification enforcing INV-COMPACTION-ADAPTIVE-1. Validates that no candidate tombstone with `seq >= min_active_seqno` can e… | `src/compaction/adaptive/cost_based.rs` |
| `pub fn new() -> Self` | Creates a new WorkloadMetrics counter instance initialized to zero. | `src/compaction/adaptive/metrics.rs` |
| `pub fn read_write_ratio(&self) -> f64` | Computes the read/write operational ratio safely handling zero writes. | `src/compaction/adaptive/metrics.rs` |
| `pub fn record_read(&self)` | Records a read operation atomically. | `src/compaction/adaptive/metrics.rs` |
| `pub fn record_write(&self, seq_no: u64)` | Records a write operation and updates the latest sequence number atomically. | `src/compaction/adaptive/metrics.rs` |
| `pub fn snapshot(&self) -> WorkloadMetricsSnapshot` | Takes a deterministic snapshot of current metrics for compaction strategy evaluation. | `src/compaction/adaptive/metrics.rs` |
| `pub async fn compact_all( &self, sstables: &RwLock<Vec<Arc<SstableReader>>>, data_path: &std::path::Path, ) -> Result<bool>` | Forces a full compaction over all existing SSTables. This maintenance API merges all currently registered SSTables into a single consolidat… | `src/compaction/engine.rs` |
| `pub async fn compact_range( &self, range: std::ops::RangeInclusive<&[u8]>, sstables: &RwLock<Vec<Arc<SstableReader>>>, data_path: &std::path::Path, ) -> Result<bool>` | Compacts all SSTables that overlap with the specified key range `[start, end]`. Neutral maintenance API without domain-specific erasure/shr… | `src/compaction/engine.rs` |
| `pub fn generate_sst_path(&self, data_path: &std::path::Path) -> Result<PathBuf>` | Generates a unique SSTable file path using a monotonic counter. | `src/compaction/engine.rs` |
| `pub async fn maybe_compact( &self, sstables: &RwLock<Vec<Arc<SstableReader>>>, data_path: &std::path::Path, ) -> Result<bool>` | — | `src/compaction/engine.rs` |
| `pub async fn maybe_compact_with_cancel( &self, sstables: &RwLock<Vec<Arc<SstableReader>>>, data_path: &std::path::Path, cancel_token: Option<&tokio_util::sync::CancellationToken>, ) -> Result<bool>` | Evaluates whether compaction should run and performs it with an optional cancellation token. | `src/compaction/engine.rs` |
| `pub async fn merge_sstables( &self, inputs: &[Arc<SstableReader>], output_path: &std::path::Path, min_snapshot_seq: u64, is_full_compaction: bool, ) -> Result<()>` | Performs a multi-way merge of input SSTables into a single output SSTable. During merge: - Duplicate keys: newest sequence number wins - To… | `src/compaction/engine.rs` |
| `pub async fn merge_sstables_with_cancel( &self, inputs: &[Arc<SstableReader>], output_path: &std::path::Path, min_snapshot_seq: u64, is_full_compaction: bool, cancel_token: Option<&tokio_util::sync::CancellationToken>, ) -> Result<()>` | Performs a multi-way merge with an optional cancellation token. | `src/compaction/engine.rs` |
| `pub fn new( config: CompactionConfig, snapshot_registry: Arc<SnapshotRegistry>, block_cache: Arc<BlockCache>, key_manager: Option<Arc<KeyManager>>, budget: Arc<crate::budget::ResourceTracker>, manifest: Option<Arc<crate::manifest::Manifest>>, ) -> Self` | Creates a new compaction engine. | `src/compaction/engine.rs` |
| `pub fn record_read_op(&self)` | Records a read operation for workload tracking. | `src/compaction/engine.rs` |
| `pub fn record_write_op(&self, seq_no: u64)` | Records a write operation for workload tracking. | `src/compaction/engine.rs` |
| `pub fn retention(&self) -> Arc<dyn crate::history::RetentionPolicy>` | Configures a custom VFS implementation for compaction file operations. Aktuelle Retention-Policy. | `src/compaction/engine.rs` |
| `pub async fn run_loop( self: Arc<Self>, sstables: Arc<RwLock<Vec<Arc<SstableReader>>>>, data_path: PathBuf, shutdown: tokio_util::sync::CancellationToken, )` | Runs the background compaction loop. Periodically checks if compaction is needed and performs it. Designed to be spawned via `tokio::spawn`. | `src/compaction/engine.rs` |
| `pub fn select_compaction_candidates( &self, ssts: &[Arc<SstableReader>], ) -> Option<Vec<Arc<SstableReader>>>` | Selects SSTables to compact using Size-Tiered strategy. Groups by size class and returns the group that meets the threshold, prioritizing k… | `src/compaction/engine.rs` |
| `pub fn set_retention(&self, policy: Arc<dyn crate::history::RetentionPolicy>)` | Ersetzt die Retention-Policy (wirkt auf folgende Flushes/Compactions). | `src/compaction/engine.rs` |
| `pub fn with_adaptive_planner(mut self, planner: Arc<dyn AdaptiveCompactionPlanner>) -> Self` | Attaches a custom adaptive compaction planner. | `src/compaction/engine.rs` |
| `pub fn with_clock(mut self, clock: Arc<dyn acta_env::Clock>) -> Self` | Attaches a custom clock port for deterministic execution timing (P28). | `src/compaction/engine.rs` |
| `pub fn with_merge_operator(mut self, merge_operator: Arc<dyn MergeOperator>) -> Self` | Attaches a custom merge operator for compaction value merging (§4.12). | `src/compaction/engine.rs` |
| `pub fn with_pressure_rx( mut self, rx: tokio::sync::watch::Receiver<crate::system_pressure::SystemPressure>, ) -> Self` | Attaches a system pressure watch receiver to enable pressure-aware compaction backpressure. | `src/compaction/engine.rs` |
| `pub fn with_vfs(mut self, vfs: Arc<dyn Vfs>) -> Self` | — | `src/compaction/engine.rs` |
| `pub fn generate_sst_path( data_path: &Path, global_counter: &AtomicU64, instance_counter: &AtomicU64, ) -> Result<PathBuf>` | Generiert einen eindeutigen Pfad für eine neue SSTable-Datei unter Verwendung monotoner Zähler. | `src/compaction/engine/install.rs` |
| `pub fn estimate_compaction_peak_bytes(candidates: &[Arc<SstableReader>]) -> u64` | Schätzt den Spitzen-Speicherverbrauch einer Kompaktierung anhand der Kandidatengrößen. | `src/compaction/engine/select.rs` |
| `pub fn select_compaction_candidates( sstables: &[Arc<SstableReader>], max_count: usize, ) -> Vec<Arc<SstableReader>>` | Wählt die besten SSTable-Kandidaten für einen Kompaktierungslauf aus. Folgt chronologischer Reihenfolge (kleinste min_tx_id zuerst) und gar… | `src/compaction/engine/select.rs` |
| `pub fn calculate_throttle_delay( pressure_rx: Option<&Receiver<SystemPressure>>, ) -> Option<Duration>` | Berechnet eine Drossel-Verzögerung basierend auf dem aktuellen Systemdruck. | `src/compaction/engine/throttle.rs` |
| `pub fn retain_key_versions<T, F>(entries: Vec<T>, get_seq: F, floor_seq: u64) -> Vec<T> where F: Fn(&T) -> u64,` | Filters a sequence of entries for a SINGLE key (sorted by raw sequence DESCENDING) according to MVCC retention rules given a `floor` sequen… | `src/compaction/retention.rs` |
| `pub fn new() -> Self` | Creates a new, empty in-memory storage engine. | `src/compat/engine/memory_only.rs` |
| `pub fn validate_against_features( self, deletion_proof_active: bool, feature_ring: acta_ports::license::FeatureRing, ) -> Result<(), DurabilityConfigError>` | — | `src/compat/license.rs` |
| `pub fn lsm_purgers(storage: &Arc<LsmStorage>) -> Vec<Arc<dyn Purger>>` | Erzeugt die LSM-Purger unter Verwendung der aktuellen Tokio-Laufzeitumgebung. | `src/compat/scanner/factory.rs` |
| `pub fn lsm_purgers_with_handle( storage: &Arc<LsmStorage>, handle: tokio::runtime::Handle, ) -> Vec<Arc<dyn Purger>>` | Erzeugt die LSM-Purger unter Verwendung eines expliziten Tokio-Runtime-Handles. | `src/compat/scanner/factory.rs` |
| `pub fn lsm_purgers_with_runtime( storage: &Arc<LsmStorage>, runtime: ScannerRuntime, ) -> Vec<Arc<dyn Purger>>` | Erzeugt die LSM-Purger unter Verwendung einer übergebenen [`ScannerRuntime`]. | `src/compat/scanner/factory.rs` |
| `pub fn lsm_scanners(storage: &Arc<LsmStorage>) -> Vec<Arc<dyn LayerScanner>>` | Erzeugt die Standard-LSM-Scanner unter Verwendung der aktuellen Tokio-Laufzeitumgebung. | `src/compat/scanner/factory.rs` |
| `pub fn lsm_scanners_with_handle( storage: &Arc<LsmStorage>, handle: tokio::runtime::Handle, ) -> Vec<Arc<dyn LayerScanner>>` | Erzeugt die Standard-LSM-Scanner unter Verwendung eines expliziten Tokio-Runtime-Handles. | `src/compat/scanner/factory.rs` |
| `pub fn lsm_scanners_with_runtime( storage: &Arc<LsmStorage>, runtime: ScannerRuntime, ) -> Vec<Arc<dyn LayerScanner>>` | Erzeugt die Standard-LSM-Scanner unter Verwendung einer übergebenen [`ScannerRuntime`]. | `src/compat/scanner/factory.rs` |
| `pub fn from_storage_blocking(storage: Arc<LsmStorage>, runtime: ScannerRuntime) -> Result<Self>` | Erzeugt einen `MemtableScanner` aus einem `Arc<LsmStorage>` (Kompatibilitätspfad). Intern wird [`LsmStorage::scan_view`] aufgerufen — die K… | `src/compat/scanner/memtable.rs` |
| `pub fn new(view: StoreScanView) -> Self` | Erzeugt einen neuen `MemtableScanner` aus einer [`StoreScanView`]. | `src/compat/scanner/memtable.rs` |
| `pub fn with_runtime(view: StoreScanView, runtime: ScannerRuntime) -> Self` | Erzeugt einen neuen `MemtableScanner` aus einer [`StoreScanView`] mit expliziter Runtime. | `src/compat/scanner/memtable.rs` |
| `pub fn new(storage: Arc<LsmStorage>) -> Self` | Erzeugt einen neuen `LsmPurger` mit implizit erfasster Laufzeit. | `src/compat/scanner/purge.rs` |
| `pub fn with_runtime(storage: Arc<LsmStorage>, runtime: ScannerRuntime) -> Self` | Erzeugt einen neuen `LsmPurger` mit expliziter `ScannerRuntime`. | `src/compat/scanner/purge.rs` |
| `pub fn block_on<F, T>(&self, future: F) -> Result<T> where F: Future<Output = Result<T>> + Send, T: Send,` | Führt ein Future synchron aus. Strategie: 1. Kein Handle vorhanden: Sofortiger Fehler `ActaError::Storage`. 2. Multi-Thread Runtime: Ausfüh… | `src/compat/scanner/runtime.rs` |
| `pub fn capture() -> Self` | Erfasst das aktuelle Tokio-Runtime-Handle aus dem Aufrufkontext. | `src/compat/scanner/runtime.rs` |
| `pub fn empty() -> Self` | Erstellt eine leere `ScannerRuntime` (führt bei Port-Aufruf zu einem typisierten Fehler). | `src/compat/scanner/runtime.rs` |
| `pub fn handle(&self) -> Option<&Handle>` | Liefert eine Referenz auf das Handle, falls vorhanden. | `src/compat/scanner/runtime.rs` |
| `pub fn with_handle(handle: Handle) -> Self` | Erstellt eine `ScannerRuntime` mit explizit bereitgestelltem Handle. | `src/compat/scanner/runtime.rs` |
| `pub fn from_storage_blocking(storage: Arc<LsmStorage>, runtime: ScannerRuntime) -> Result<Self>` | Erzeugt einen `SstableScanner` aus einem `Arc<LsmStorage>` (Kompatibilitätspfad). | `src/compat/scanner/sstable.rs` |
| `pub fn new(view: StoreScanView) -> Self` | Erzeugt einen neuen `SstableScanner` mit implizit erfasster Laufzeit. | `src/compat/scanner/sstable.rs` |
| `pub fn with_runtime(view: StoreScanView, runtime: ScannerRuntime) -> Self` | Erzeugt einen neuen `SstableScanner` mit expliziter `ScannerRuntime`. | `src/compat/scanner/sstable.rs` |
| `pub fn active_memtable(&self) -> &Arc<MemTable>` | Gibt die aktive MemTable zurück. | `src/compat/scanner/store_scan_view.rs` |
| `pub fn all_memtables(&self) -> impl Iterator<Item = &Arc<MemTable>>` | Gibt einen Iterator über alle MemTables (aktiv + immutable) zurück. | `src/compat/scanner/store_scan_view.rs` |
| `pub fn block_cache(&self) -> &Arc<BlockCache>` | Gibt den gemeinsamen Block-Cache zurück. | `src/compat/scanner/store_scan_view.rs` |
| `pub fn data_path(&self) -> &std::path::Path` | Gibt den Pfad zum Datenverzeichnis zurück. | `src/compat/scanner/store_scan_view.rs` |
| `pub fn immutable_memtables(&self) -> &[Arc<MemTable>]` | Gibt alle immutablen MemTables zurück. | `src/compat/scanner/store_scan_view.rs` |
| `pub fn new_for_test( active_memtable: Arc<MemTable>, immutable_memtables: Vec<Arc<MemTable>>, sstables: Arc<RwLock<Vec<Arc<SstableReader>>>>, block_cache: Arc<BlockCache>, data_path: PathBuf, vfs: Arc<dyn Vfs>, snapshot_registry: Arc<SnapshotRegistry>, ) -> S…` | Erstellt einen `StoreScanView` für Tests mit vollständig isolierten Daten. Ermöglicht Unit-Tests ohne `LsmStorage`-Instanz. | `src/compat/scanner/store_scan_view.rs` |
| `pub fn snapshot_registry(&self) -> &Arc<SnapshotRegistry>` | Gibt die SnapshotRegistry zurück (für Purge-Sperrprüfung). | `src/compat/scanner/store_scan_view.rs` |
| `pub fn sstables_arc(&self) -> &Arc<RwLock<Vec<Arc<SstableReader>>>>` | Gibt die SSTable-Liste als `Arc<RwLock<...>>` zurück (für Purger-Schreibzugriff). | `src/compat/scanner/store_scan_view.rs` |
| `pub fn vfs(&self) -> &Arc<dyn Vfs>` | Gibt die VFS-Instanz zurück. | `src/compat/scanner/store_scan_view.rs` |
| `pub fn from_storage_blocking(storage: Arc<LsmStorage>, runtime: ScannerRuntime) -> Result<Self>` | Erzeugt einen `WalScanner` aus einem `Arc<LsmStorage>` (Kompatibilitätspfad). | `src/compat/scanner/wal.rs` |
| `pub fn new(view: StoreScanView) -> Self` | Erzeugt einen neuen `WalScanner` mit implizit erfasster Laufzeit. | `src/compat/scanner/wal.rs` |
| `pub fn with_runtime(view: StoreScanView, runtime: ScannerRuntime) -> Self` | Erzeugt einen neuen `WalScanner` mit expliziter `ScannerRuntime`. | `src/compat/scanner/wal.rs` |
| `pub fn checksum_mismatch(path: impl Into<String>, block_id: u64) -> Self` | Erzeugt einen neuen `StoreError::ChecksumMismatch`. | `src/error.rs` |
| `pub fn invalid_input(reason: impl Into<String>) -> Self` | — | `src/error.rs` |
| `pub fn wal_corruption(offset: u64, reason: impl Into<String>) -> Self` | Erzeugt einen neuen `StoreError::WalCorruption`. | `src/error.rs` |
| `pub fn wal_truncation_detected(expected_hmac: [u8; 32], actual_hmac: [u8; 32]) -> Self` | Erzeugt einen neuen `StoreError::InvalidInput`. | `src/error.rs` |
| `pub async fn fsync_parent_dir(path: &Path) -> Result<()>` | Alias for `sync_parent_dir` for backward compatibility across storage modules. | `src/fsync.rs` |
| `pub async fn sync_file(path: &Path) -> Result<()>` | Performs fsync on a single file at `path`. # Errors Returns `StoreError::Storage` on Unix if opening or syncing fails. Returns `StoreError:… | `src/fsync.rs` |
| `pub async fn sync_parent_dir(path: &Path) -> Result<()>` | Performs fsync on the parent directory of `path`. Directory fsync is required on POSIX filesystems to guarantee that newly created files or… | `src/fsync.rs` |
| `pub fn check_horizon(seq: u64, floor: u64) -> Option<ReadOutcome<()>>` | Prüft den Horizont: `seq < floor` ⇒ `HorizonExceeded`. | `src/history.rs` |
| `pub fn default_arc() -> Arc<dyn RetentionPolicy>` | Standard-Policy als `Arc<dyn RetentionPolicy>`. | `src/history.rs` |
| `pub fn is_found(&self) -> bool` | — | `src/history.rs` |
| `pub fn map<U>(self, f: impl FnOnce(T) -> U) -> ReadOutcome<U>` | — | `src/history.rs` |
| `pub fn new() -> Self` | — | `src/history.rs` |
| `pub fn retain_versions_with_policy<T, F>( entries: Vec<T>, get_seq: F, snapshot_floor: u64, partition: PartitionId, policy: &dyn RetentionPolicy, ) -> Vec<T> where F: Fn(&T) -> u64,` | Filtert die Versionen EINES Schlüssels (absteigend nach Roh-Seq) gemäß Policy. Regeln (Obermenge der alten `retain_key_versions`): 1. Alles… | `src/history.rs` |
| `pub fn retention(&self, partition: PartitionId) -> Retention` | — | `src/history.rs` |
| `pub fn with_floor(mut self, partition: PartitionId, floor: u64) -> Self` | Setzt den (persistierten) History-Floor einer Partition. | `src/history.rs` |
| `pub fn with_retention(mut self, partition: PartitionId, r: Retention) -> Self` | Setzt die Retention einer Partition. | `src/history.rs` |
| `pub async fn create_dir_all(&self, path: &Path) -> Result<()>` | Creates directories along . | `src/io_seam.rs` |
| `pub async fn exists(&self, path: &Path) -> Result<bool>` | Checks if a file exists. | `src/io_seam.rs` |
| `pub fn file(&self) -> &std::fs::File` | Returns a reference to the inner file. | `src/io_seam.rs` |
| `pub async fn len(&self, path: &Path) -> Result<u64>` | Returns the length in bytes of the file at . | `src/io_seam.rs` |
| `pub fn new(file: std::fs::File) -> Self` | Creates a new from an open file handle. | `src/io_seam.rs` |
| `pub fn new(vfs: Arc<dyn Vfs>) -> Self` | Creates a new with the specified . | `src/io_seam.rs` |
| `pub fn open(&self, path: &Path, opts: &VfsOpenOptions) -> Result<Box<dyn VfsFile>>` | Opens a file using positional VfsFile options. | `src/io_seam.rs` |
| `pub async fn read(&self, path: &Path) -> Result<Vec<u8>>` | Reads an entire file to a vector of bytes. | `src/io_seam.rs` |
| `pub async fn read_at(&self, path: &Path, offset: u64, len: usize) -> Result<Vec<u8>>` | Reads data positionally from into . | `src/io_seam.rs` |
| `pub async fn read_dir(&self, path: &Path) -> Result<Vec<PathBuf>>` | Reads directory entries as PathBufs. | `src/io_seam.rs` |
| `pub fn read_only() -> Self` | — | `src/io_seam.rs` |
| `pub async fn remove_file(&self, path: &Path) -> Result<()>` | Removes a file at . | `src/io_seam.rs` |
| `pub async fn rename(&self, from: &Path, to: &Path) -> Result<()>` | Atomically renames a file. | `src/io_seam.rs` |
| `pub async fn sync_dir(&self, path: &Path) -> Result<()>` | Syncs directory metadata. | `src/io_seam.rs` |
| `pub fn vfs(&self) -> &Arc<dyn Vfs>` | Returns a reference to the inner . | `src/io_seam.rs` |
| `pub async fn write_atomic(&self, path: &Path, data: Vec<u8>) -> Result<()>` | Atomically writes data to with crash-consistency. | `src/io_seam.rs` |
| `pub async fn create_dir_all(&self, path: &Path) -> Result<()>` | Erzeugt Verzeichnisse rekursiv. | `src/io_seam/store_io.rs` |
| `pub async fn exists(&self, path: &Path) -> Result<bool>` | Prüft auf Existenz einer Datei. | `src/io_seam/store_io.rs` |
| `pub async fn len(&self, path: &Path) -> Result<u64>` | Liefert die Dateilänge in Bytes. | `src/io_seam/store_io.rs` |
| `pub fn new(vfs: Arc<dyn Vfs>) -> Self` | Erzeugt ein neues `StoreIo` mit dem angegebenen VFS. | `src/io_seam/store_io.rs` |
| `pub fn open(&self, path: &Path, opts: &VfsOpenOptions) -> Result<Box<dyn VfsFile>>` | Öffnet eine Datei unter Verwendung von `VfsOpenOptions`. | `src/io_seam/store_io.rs` |
| `pub async fn read(&self, path: &Path) -> Result<Vec<u8>>` | Liest eine Datei vollständig asynchron ein. | `src/io_seam/store_io.rs` |
| `pub async fn read_at(&self, path: &Path, offset: u64, len: usize) -> Result<Vec<u8>>` | Liest Daten positionell ab `offset` mit Länge `len` asynchron ein. | `src/io_seam/store_io.rs` |
| `pub async fn read_dir(&self, path: &Path) -> Result<Vec<PathBuf>>` | Liest Verzeichniseinträge als Pfadliste. | `src/io_seam/store_io.rs` |
| `pub async fn remove_file(&self, path: &Path) -> Result<()>` | Löscht eine Datei. | `src/io_seam/store_io.rs` |
| `pub async fn rename(&self, from: &Path, to: &Path) -> Result<()>` | Benennt eine Datei atomar um. | `src/io_seam/store_io.rs` |
| `pub async fn sync_dir(&self, path: &Path) -> Result<()>` | Synchronisiert Verzeichnis-Metadaten. | `src/io_seam/store_io.rs` |
| `pub fn vfs(&self) -> &Arc<dyn Vfs>` | Liefert eine Referenz auf das innere VFS. | `src/io_seam/store_io.rs` |
| `pub async fn write_atomic(&self, path: &Path, data: Vec<u8>) -> Result<()>` | Schreibt Daten atomar und crash-konsistent. | `src/io_seam/store_io.rs` |
| `pub fn read_only() -> Self` | Standardoptionen für Nur-Lese-Zugriff. | `src/io_seam/types.rs` |
| `pub fn new(bytes: Vec<u8>) -> Self` | Creates a static key material source repeating or providing fixed byte slices. | `src/key_source.rs` |
| `pub fn cleanup_intent_locks_where<F>(&self, mut predicate: F) where F: FnMut(TxId) -> bool,` | Clears intent locks matching a predicate, gracefully recovering from poisoned locks. | `src/lsm/commit.rs` |
| `pub fn clear_intent_locks_above_tx(&self, target_tx: TxId)` | Removes all intent lock registrations associated with transactions strictly newer than target_tx. | `src/lsm/commit.rs` |
| `pub fn clear_intent_locks_for_tx(&self, tx_id: TxId)` | Clears intent locks for a specific transaction ID. | `src/lsm/commit.rs` |
| `pub fn validate_durability( self, integrity_chain_required: bool, require_persistence: bool, ) -> Result<(), DurabilityConfigError>` | Validates the durability mode against required integrity guarantees and persistence constraints. | `src/lsm/config.rs` |
| `pub fn with_clock(mut self, clock: std::sync::Arc<dyn acta_env::Clock>) -> Self` | Attaches an injected clock port fluently to `LsmConfig`. | `src/lsm/config.rs` |
| `pub fn with_key_source( mut self, key_source: std::sync::Arc<dyn crate::key_source::KeyMaterialSource>, ) -> Self` | Attaches an injected key material source fluently to `LsmConfig` (for deterministic testing). | `src/lsm/config.rs` |
| `pub fn block_cache_for_test(&self) -> Arc<BlockCache>` | — | `src/lsm/engine.rs` |
| `pub fn budget_for_test(&self) -> Arc<crate::budget::ResourceTracker>` | — | `src/lsm/engine.rs` |
| `pub fn budget_tracking_drift_bytes(&self) -> u64` | Returns the accumulated total memory budget tracking drift in bytes caused by unbudgeted memtable puts during commit when memory limit was… | `src/lsm/engine.rs` |
| `pub fn clock(&self) -> Arc<dyn acta_env::Clock>` | Returns the currently configured clock port. | `src/lsm/engine.rs` |
| `pub async fn close(&self) -> Result<()>` | Gracefully closes the storage engine, stopping background tasks and flushing active memtable to disk. # Errors Gibt `ActaError::Storage` zu… | `src/lsm/engine.rs` |
| `pub fn data_path(&self) -> &std::path::Path` | Returns the data directory path. | `src/lsm/engine.rs` |
| `pub fn deregister_observer(&self, observer: &Arc<dyn WalObserver>)` | Deregisters an observer matching the provided `Arc` reference via pointer equality. | `src/lsm/engine.rs` |
| `pub async fn get_at_seq_tracked( &self, tx_id: TxId, key: &[u8], snapshot_seq: u64, ) -> Result<Option<Bytes>>` | Performs a tracked read for transaction `tx_id` at an explicit `snapshot_seq`. Registers the read key in `TxBuffer` at `snapshot_seq` and r… | `src/lsm/engine.rs` |
| `pub async fn get_tracked(&self, tx_id: TxId, key: &[u8]) -> Result<Option<Bytes>>` | Performs a tracked read for transaction `tx_id`. Evaluates `snapshot_seq = next_seq_no - 1` ONCE (K2), registers the read key in `TxBuffer`… | `src/lsm/engine.rs` |
| `pub fn has_observers(&self) -> bool` | Returns true if at least one observer is registered. | `src/lsm/engine.rs` |
| `pub fn health(&self) -> StorageHealth` | Returns the current storage health status. | `src/lsm/engine.rs` |
| `pub fn last_applied_seq(&self) -> u64` | Returns the highest sequence number applied to the MemTable and published. | `src/lsm/engine.rs` |
| `pub fn manifest_for_test(&self) -> Arc<crate::manifest::Manifest>` | — | `src/lsm/engine.rs` |
| `pub fn max_observer_latency(&self) -> std::time::Duration` | Returns the current maximum latency threshold for observer callbacks. | `src/lsm/engine.rs` |
| `pub async fn maybe_compact_for_test(&self) -> Result<bool>` | — | `src/lsm/engine.rs` |
| `pub fn metrics_sink(&self) -> Arc<dyn acta_env::MetricsSink>` | Returns the currently configured [`acta_env::MetricsSink`]. | `src/lsm/engine.rs` |
| `pub fn next_seq_no_for_test(&self) -> u64` | Returns the current `next_seq_no` value for testing verification. | `src/lsm/engine.rs` |
| `pub async fn open(config: LsmConfig) -> Result<Self>` | Opens or recovers an LSM storage instance with the given configuration. # Errors Gibt `ActaError::Storage` zurück, falls I/O-Fehler, Verzei… | `src/lsm/engine.rs` |
| `pub async fn open_with_merge_operator( config: LsmConfig, merge_operator: Arc<dyn MergeOperator>, ) -> Result<Self>` | Opens or recovers an LSM storage instance with the given configuration and a `MergeOperator`. # Errors Gibt `ActaError::Storage` zurück bei… | `src/lsm/engine.rs` |
| `pub fn pressure_receiver( &self, ) -> tokio::sync::watch::Receiver<crate::system_pressure::SystemPressure>` | Returns a watch receiver for monitoring system pressure levels. | `src/lsm/engine.rs` |
| `pub fn register_observer(&self, observer: Arc<dyn WalObserver>)` | Registers a new WAL observer. | `src/lsm/engine.rs` |
| `pub fn request_flush(&self)` | Requests an asynchronous background flush without blocking the caller or holding commit locks. | `src/lsm/engine.rs` |
| `pub async fn restore_wal_file_handle_for_test(&self)` | — | `src/lsm/engine.rs` |
| `pub async fn scan_prefix_tracked( &self, tx_id: TxId, prefix: &[u8], ) -> Result<Vec<(Vec<u8>, Vec<u8>)>>` | Performs a tracked prefix scan for transaction `tx_id`. Evaluates `snapshot_seq = last_applied_seq` ONCE, scans all matching entries, and r… | `src/lsm/engine.rs` |
| `pub async fn scan_view(self: &Arc<Self>) -> crate::compat::scanner::StoreScanView` | Erzeugt eine entkoppelte [`StoreScanView`] für Scanner und Purger. Dies ist die einzige erlaubte Stelle, an der `LsmStorage` seine Interna… | `src/lsm/engine.rs` |
| `pub fn set_clock(&self, clock: Arc<dyn acta_env::Clock>)` | Sets the injected clock port for deterministic execution timing (P28). | `src/lsm/engine.rs` |
| `pub fn set_max_observer_latency(&self, latency: std::time::Duration)` | Sets the maximum latency threshold for observer callbacks. | `src/lsm/engine.rs` |
| `pub fn set_metrics_sink(&self, sink: Arc<dyn acta_env::MetricsSink>)` | Sets the [`acta_env::MetricsSink`] for recording operational metrics. | `src/lsm/engine.rs` |
| `pub fn set_ssi_validator( &mut self, validator: Arc<dyn acta_mvcc::SsiValidator + Send + Sync>, )` | Sets or replaces the [`acta_mvcc::SsiValidator`] port for transaction conflict detection. | `src/lsm/engine.rs` |
| `pub fn shutdown(&self)` | Signals the background compaction engine to stop. | `src/lsm/engine.rs` |
| `pub async fn simulate_wal_append_failure_for_test(&self)` | — | `src/lsm/engine.rs` |
| `pub async fn simulate_wal_append_failure_for_tx_for_test(&self, _tx_id: u64)` | — | `src/lsm/engine.rs` |
| `pub fn spawn_tracked<F>(&self, future: F) where F: std::future::Future<Output = ()> + Send + 'static,` | Spawns a background task tracked by this storage instance. | `src/lsm/engine.rs` |
| `pub fn sstables_for_test(&self) -> Arc<RwLock<Vec<Arc<SstableReader>>>>` | — | `src/lsm/engine.rs` |
| `pub fn vfs(&self) -> &std::sync::Arc<dyn crate::io_seam::Vfs>` | Returns the VFS instance. | `src/lsm/engine.rs` |
| `pub async fn wait_shutdown(&self)` | Waits for all spawned tasks to shut down fully. | `src/lsm/engine.rs` |
| `pub fn with_clock(self, clock: Arc<dyn acta_env::Clock>) -> Self` | Configures the clock port fluently on `self` and returns `self`. | `src/lsm/engine.rs` |
| `pub fn with_metrics_sink(self, sink: Arc<dyn acta_env::MetricsSink>) -> Self` | Configures the [`acta_env::MetricsSink`] fluently on `self` and returns `self`. | `src/lsm/engine.rs` |
| `pub fn with_ssi_validator( mut self, validator: Arc<dyn acta_mvcc::SsiValidator + Send + Sync>, ) -> Self` | Configures the [`acta_mvcc::SsiValidator`] fluently on `self` and returns `self`. | `src/lsm/engine.rs` |
| `pub async fn commit<'a>(&'a self, tx_id: TxId) -> Result<()>` | Transaktion committen. | `src/lsm/engine/facade.rs` |
| `pub async fn delete<'a>(&'a self, tx_id: TxId, key: &'a [u8]) -> Result<()>` | Loeschen eines einzelnen Schluessels. | `src/lsm/engine/facade.rs` |
| `pub async fn delete_many<'a>(&'a self, tx_id: TxId, keys: Vec<Vec<u8>>) -> Result<u64>` | Loeschen mehrerer Schluessel. | `src/lsm/engine/facade.rs` |
| `pub async fn delete_prefix<'a>(&'a self, tx_id: TxId, prefix: &'a [u8]) -> Result<u64>` | Loeschen aller Schluessel mit Praefix. | `src/lsm/engine/facade.rs` |
| `pub async fn flush<'a>(&'a self) -> Result<()>` | MemTable synchron flushen. | `src/lsm/engine/facade.rs` |
| `pub async fn get<'a>(&'a self, key: &'a [u8]) -> Result<Option<bytes::Bytes>>` | Inherent read operations for standalone usage. | `src/lsm/engine/facade.rs` |
| `pub async fn get_at_seq<'a>(&'a self, key: &'a [u8], seq_no: u64) -> Result<Option<bytes::Bytes>>` | Punktlesen zu einer bestimmten Sequenznummer. | `src/lsm/engine/facade.rs` |
| `pub async fn last_seq_no<'a>(&'a self) -> Result<u64>` | Letzte Sequenznummer abfragen. | `src/lsm/engine/facade.rs` |
| `pub async fn last_tx_id<'a>(&'a self) -> Result<TxId>` | Letzte Transaktions-ID abfragen. | `src/lsm/engine/facade.rs` |
| `pub async fn pin_checkpoint<'a>(&'a self, seq_no: u64) -> Result<()>` | Checkpoint pinnen. | `src/lsm/engine/facade.rs` |
| `pub async fn put<'a>(&'a self, tx_id: TxId, key: &'a [u8], value: &'a [u8]) -> Result<()>` | Inherent write and maintenance operations. | `src/lsm/engine/facade.rs` |
| `pub async fn put_if_absent<'a>(&'a self, tx_id: TxId, key: &'a [u8], value: &'a [u8]) -> Result<bool>` | Einfuegen nur wenn Schluessel nicht vorhanden. | `src/lsm/engine/facade.rs` |
| `pub async fn rollback<'a>(&'a self, tx_id: TxId) -> Result<()>` | Transaktion zurueckrollen. | `src/lsm/engine/facade.rs` |
| `pub async fn scan<'a>( &'a self, start: std::ops::Bound<&'a [u8]>, end: std::ops::Bound<&'a [u8]>, limit: Option<usize>, ) -> Result<Vec<(Vec<u8>, Vec<u8>)>>` | Bereichs-Scan ueber Schichten mit optionalem Limit. | `src/lsm/engine/facade.rs` |
| `pub async fn scan_bounded<'a>( &'a self, start: std::ops::Bound<&'a [u8]>, end: std::ops::Bound<&'a [u8]>, limit: usize, cursor: Option<&'a [u8]>, ) -> Result<(Vec<(Vec<u8>, Vec<u8>)>, Option<Vec<u8>>)>` | Bereichs-Scan mit Grenzen und Paginierungs-Cursor. | `src/lsm/engine/facade.rs` |
| `pub async fn scan_prefix<'a>(&'a self, prefix: &'a [u8]) -> Result<Vec<(Vec<u8>, Vec<u8>)>>` | Praefix-Scan ueber alle Schichten. | `src/lsm/engine/facade.rs` |
| `pub async fn scan_prefix_at<'a>( &'a self, prefix: &'a [u8], seq_no: u64, ) -> Result<Vec<(Vec<u8>, Vec<u8>)>>` | Praefix-Scan zu einer bestimmten Sequenznummer. | `src/lsm/engine/facade.rs` |
| `pub async fn scan_prefix_bounded<'a>( &'a self, prefix: &'a [u8], limit: usize, cursor: Option<&'a [u8]>, ) -> Result<(Vec<(Vec<u8>, Vec<u8>)>, Option<Vec<u8>>)>` | Begrenzter Praefix-Scan mit Paginierungs-Cursor. | `src/lsm/engine/facade.rs` |
| `pub async fn stats<'a>(&'a self) -> Result<crate::stats::StorageStats>` | Speicherstatistiken ermitteln. | `src/lsm/engine/facade.rs` |
| `pub async fn unpin_checkpoint<'a>(&'a self, seq_no: u64) -> Result<()>` | Checkpoint freigeben. | `src/lsm/engine/facade.rs` |
| `pub async fn has_pending_legacy_wal_migration(&self) -> Result<bool>` | Checks if the current storage instance's data directory contains any pending legacy WAL files. | `src/lsm/engine/migration.rs` |
| `pub async fn has_pending_legacy_wal_migration_path(path: &std::path::Path) -> Result<bool>` | Checks if a database directory contains any legacy WAL files requiring explicit migration. | `src/lsm/engine/migration.rs` |
| `pub async fn migrate_legacy_wal_keys( config: &LsmConfig, clock: Arc<dyn acta_env::Clock>, ) -> Result<usize>` | Explicitly migrates all legacy WAL files in the specified directory using the provided clock port. | `src/lsm/engine/migration.rs` |
| `pub fn enter_flush(&self) -> bool` | Markiert den Beginn eines Flush-Vorgangs. Gibt `true` zurück, wenn kein Purge aktiv ist und Flush erworben wurde. | `src/lsm/fence.rs` |
| `pub fn enter_purge(&self) -> bool` | Markiert den Beginn einer Purge-Kompaktion (Priorität). Gibt `true` zurück, wenn Purge exklusiven Zugriff erlangt hat. | `src/lsm/fence.rs` |
| `pub fn is_flush_active(&self) -> bool` | Prüft nicht-blockierend, ob ein Flush aktiv ist. | `src/lsm/fence.rs` |
| `pub fn is_purge_active(&self) -> bool` | Prüft nicht-blockierend, ob ein Purge ansteht oder aktiv ist. | `src/lsm/fence.rs` |
| `pub fn leave_flush(&self)` | Beendet den Flush-Abschnitt. | `src/lsm/fence.rs` |
| `pub fn leave_purge(&self)` | Beendet den Purge-Abschnitt. | `src/lsm/fence.rs` |
| `pub fn load_snapshot_tx(target: &AtomicU64) -> u64` | Lädt den aktuellen Sichtbarkeitshorizont für einen konsistenten Snapshot mit `Acquire`-Semantik. | `src/lsm/fence.rs` |
| `pub fn publish_visibility(target: &AtomicU64, new_tx: u64)` | Publiziert Sichtbarkeitsfortschritte (`last_committed_tx`) mit `Release`-Semantik. | `src/lsm/fence.rs` |
| `pub async fn force_flush(&self) -> Result<()>` | — | `src/lsm/flush.rs` |
| `pub async fn maybe_compact(&self) -> Result<bool>` | Evaluates whether compaction should run and performs it if needed. Uses the persistent CompactionEngine instance to maintain a stable count… | `src/lsm/flush.rs` |
| `pub async fn get_at_seq_checked( &self, partition: PartitionId, key: &[u8], seq: u64, ) -> Result<ReadOutcome<Bytes>>` | Zeitreise-Read mit Horizont-Prüfung: `seq < floor` ⇒ `HorizonExceeded` statt einer stillen, womöglich falschen `NotFound`-Antwort. | `src/lsm/history.rs` |
| `pub fn history_floor(&self, partition: PartitionId) -> u64` | Aktueller persistierter History-Floor der Partition (0 = unbegrenzt zeitreisefähig). | `src/lsm/history.rs` |
| `pub async fn raise_history_floor(&self, partition: PartitionId, floor: u64) -> Result<u64>` | Hebt den History-Floor an (monoton; ein niedrigerer Wert ist ein No-op). Liefert den danach gültigen Floor. Der Floor darf nicht über den z… | `src/lsm/history.rs` |
| `pub fn set_retention_policy(&self, policy: Arc<dyn RetentionPolicy>)` | Ersetzt die Retention-Policy (z. B. `KeepAll` für die Chronik). | `src/lsm/history.rs` |
| `pub fn new( inner: Arc<dyn WalObserver>, capacity: usize, ) -> (Self, tokio::task::JoinHandle<()>)` | — | `src/lsm/observer/adapter.rs` |
| `pub fn clear_circuit_breaker(&self, observer: &Arc<dyn WalObserver>)` | Resets the circuit breaker state for a specific observer. | `src/lsm/observer/registry.rs` |
| `pub fn deregister_observer(&self, observer: &Arc<dyn WalObserver>)` | Deregisters an observer matching the provided `Arc` reference via pointer equality. | `src/lsm/observer/registry.rs` |
| `pub fn dropped_count(&self) -> u64` | Returns the total number of dropped events across all observers (including deregistered ones). | `src/lsm/observer/registry.rs` |
| `pub fn dropped_count_for(&self, observer: &Arc<dyn WalObserver>) -> u64` | Returns the number of dropped events for a specific registered observer. | `src/lsm/observer/registry.rs` |
| `pub fn is_any_circuit_breaker_open(&self) -> bool` | Returns whether any registered observer currently has an open circuit breaker. | `src/lsm/observer/registry.rs` |
| `pub fn is_circuit_breaker_open(&self, observer: &Arc<dyn WalObserver>) -> bool` | Returns whether the circuit breaker is currently open for a specific observer. | `src/lsm/observer/registry.rs` |
| `pub fn is_empty(&self) -> bool` | Returns true if no observers are registered. | `src/lsm/observer/registry.rs` |
| `pub fn max_observer_latency(&self) -> Duration` | Returns the current maximum latency threshold. | `src/lsm/observer/registry.rs` |
| `pub fn new() -> Self` | Creates a new, empty `ObserverRegistry`. | `src/lsm/observer/registry.rs` |
| `pub fn notify(&self, entries: &[WalEntry], seq_no: u64, tx_id: TxId, origin: WriteOrigin)` | Synchronously notifies all registered observers of a committed batch. | `src/lsm/observer/registry.rs` |
| `pub fn notify_with_context(&self, entries: &[WalEntry], seq_no: u64, ctx: CommitContext)` | Synchronously notifies all registered observers of a committed batch with explicit CommitContext. | `src/lsm/observer/registry.rs` |
| `pub fn register_observer(&self, observer: Arc<dyn WalObserver>)` | Registers a new WAL observer. | `src/lsm/observer/registry.rs` |
| `pub fn set_clock(&self, clock: Arc<dyn Clock>)` | Sets the injected clock port for deterministic execution timing (P28). | `src/lsm/observer/registry.rs` |
| `pub fn set_max_observer_latency(&self, latency: Duration)` | Sets the maximum latency threshold for observer callbacks. | `src/lsm/observer/registry.rs` |
| `pub fn checksum(&self) -> &'a [u8` | Returns a reference to the entry's checksum. | `src/lsm/observer/types.rs` |
| `pub fn new(entry: &'a WalEntry) -> Self` | Creates a new `WalEntryRef` from a `WalEntry`. | `src/lsm/observer/types.rs` |
| `pub fn op(&self) -> &'a WalOp` | Returns a reference to the operation inside the entry. | `src/lsm/observer/types.rs` |
| `pub fn prev_hmac(&self) -> &'a [u8` | Returns a reference to the entry's previous HMAC checksum. | `src/lsm/observer/types.rs` |
| `pub fn seq_no(&self) -> u64` | Returns the sequence number of the entry. | `src/lsm/observer/types.rs` |
| `pub fn tx_id(&self) -> TxId` | Returns the transaction ID associated with this entry. | `src/lsm/observer/types.rs` |
| `pub async fn rollback_to_tx(&self, target_tx: TxId) -> Result<()>` | Fuehrt ein Rollback des Datenbestands auf `target_tx` aus. | `src/lsm/recovery/rollback.rs` |
| `pub async fn new(config: LsmConfig) -> Result<Self>` | Oeffnet oder erzeugt den LSM-Speicher mit der uebergebenen Konfiguration. | `src/lsm/recovery/startup.rs` |
| `pub async fn new_with_merge_operator( config: LsmConfig, merge_operator: Option<Arc<dyn MergeOperator>>, ) -> Result<Self>` | Oeffnet oder erzeugt den LSM-Speicher mit optionalem MergeOperator fuer Compaction. | `src/lsm/recovery/startup.rs` |
| `pub async fn point_lookup_metrics(&self, key: &[u8]) -> (usize, usize, usize, usize, bool)` | Evaluates detailed traversal metrics (evaluated_sstables, bloom_passes, range_passes, block_reads, found) for point lookups. | `src/lsm/scan.rs` |
| `pub fn derive_doc_id(key: &[u8]) -> DocId` | Derives a deterministic `DocId` from key bytes using BLAKE3. | `src/lsm/validate.rs` |
| `pub fn derive_doc_id(key: &[u8]) -> DocId` | Derives a deterministic 128-bit `DocId` from key bytes using BLAKE3. | `src/lsm/validate.rs` |
| `pub fn validate_key(key: &[u8]) -> Result<()>` | Validates that a key is non-empty and does not exceed `MAX_KEY_SIZE`. | `src/lsm/validate.rs` |
| `pub fn validate_value(value: &[u8]) -> Result<()>` | Validates that value bytes do not exceed `MAX_VALUE_SIZE`. | `src/lsm/validate.rs` |
| `pub async fn append(&self, entry: &ManifestEntry) -> StoreResult<()>` | Appends a new entry to the manifest and performs flush + fsync. | `src/manifest/apply.rs` |
| `pub async fn append_batch(&self, entries: &[ManifestEntry]) -> StoreResult<()>` | Appends multiple entries to the manifest in a single atomic batch with flush + fsync. | `src/manifest/apply.rs` |
| `pub fn carry_over_metadata(entries: &[ManifestEntry]) -> Vec<ManifestEntry>` | Kompakte Metadaten-Einträge, die ein Rollover übernehmen MUSS: letzter `WalCheckpoint`, `SeqHwm`, `HistoryFloor`s und Erasure-Journal. Ohne… | `src/manifest/cleanup.rs` |
| `pub fn extract_erasure_states( entries: &[ManifestEntry], ) -> std::collections::BTreeMap<[u8` | Letzter (höchster) Zustand je `erasure_id`. Übergänge sind monoton (I3): ein niedrigerer Zustand überschreibt nie einen höheren. | `src/manifest/cleanup.rs` |
| `pub fn extract_high_water_mark(entries: &[ManifestEntry]) -> Option<[u8` | Extracts the most recent WAL high-water-mark HMAC from a list of manifest entries. | `src/manifest/cleanup.rs` |
| `pub fn extract_history_floors(entries: &[ManifestEntry]) -> std::collections::BTreeMap<u32, u64>` | Persistierte History-Floors je Partition (monoton: höchster Wert gewinnt, I4). | `src/manifest/cleanup.rs` |
| `pub fn extract_seq_hwm(entries: &[ManifestEntry]) -> Option<(u64, u64)>` | Höchste persistierte `SeqHwm{seq, tx}` (Spec §4.3); beide Komponenten monoton. | `src/manifest/cleanup.rs` |
| `pub fn reconstruct_dead_sstables(entries: &[ManifestEntry]) -> HashSet<PathBuf>` | Reconstructs the set of SSTable file names (or path components) that are proven dead (explicitly removed via `Remove` or `Replace.removed`)… | `src/manifest/cleanup.rs` |
| `pub fn reconstruct_valid_sstables(entries: &[ManifestEntry]) -> Vec<(PathBuf, u64)>` | Reconstructs the ordered list of currently valid SSTable file names (or path components) and their rank (shadowing order: smaller rank = ol… | `src/manifest/cleanup.rs` |
| `pub async fn load(path: &Path) -> StoreResult<Vec<ManifestEntry>>` | Loads all valid manifest entries from `path`. | `src/manifest/core.rs` |
| `pub fn from_bytes(data: &[u8]) -> StoreResult<Self>` | Deserializes a manifest entry from a slice containing `[crc32: u32 LE] [payload...]`. | `src/manifest/entry.rs` |
| `pub fn from_u8(v: u8) -> Option<Self>` | — | `src/manifest/entry.rs` |
| `pub fn to_bytes(&self) -> StoreResult<Vec<u8>>` | Serializes the entry into binary format with a 4-byte total size prefix and a 4-byte CRC32. | `src/manifest/entry.rs` |
| `pub fn is_poisoned(&self) -> bool` | Checks if the manifest handle has been poisoned by a preceding fsync failure (INV-FSYNC-1). | `src/manifest/handle.rs` |
| `pub async fn open(path: impl AsRef<Path>) -> StoreResult<Self>` | Opens or creates an append-only Manifest file at `path`. | `src/manifest/handle.rs` |
| `pub fn path(&self) -> &Path` | Returns the filesystem path to the manifest file. | `src/manifest/handle.rs` |
| `pub fn poison(&self)` | Explicitly poisons the manifest handle (INV-FSYNC-1). | `src/manifest/handle.rs` |
| `pub async fn maybe_rollover( &self, live_entries: &[ManifestEntry], threshold_bytes: u64, ) -> Result<bool>` | Evaluates whether the MANIFEST file size exceeds `threshold_bytes` and performs an atomic rollover if needed. Rollover process: (a) Write c… | `src/manifest/rollover.rs` |
| `pub async fn rollover(&self, live_entries: &[ManifestEntry]) -> Result<()>` | Evaluates whether the MANIFEST file size exceeds `threshold_bytes` and performs an atomic rollover if needed. Rollover process: (a) Write c… | `src/manifest/rollover.rs` |
| `pub fn iter(&self) -> Vec<(Bytes, Bytes, u64, u64)>` | Iterates all entries across all shards in ascending key order. | `src/memtable/iter.rs` |
| `pub fn iter_latest(&self) -> Vec<(Bytes, Bytes, u64, u64)>` | Iterates over only the latest version of each key in sorted key order (including tombstones). | `src/memtable/iter.rs` |
| `pub fn scan_prefix_into( &self, prefix: &[u8], seq_no: u64, max_tx: TxId, target: &mut BTreeMap<Bytes, (Bytes, u64)>, )` | Scans entries matching a key prefix into `target`. Higher `seq_no` versions shadow lower `seq_no` versions. | `src/memtable/iter.rs` |
| `pub fn scan_prefix_into_matching<F>( &self, prefix: &[u8], seq_no: u64, max_tx: TxId, target: &mut BTreeMap<Bytes, (Bytes, u64)>, predicate: F, ) where F: Fn(&[u8], u64, u64) -> bool,` | Scans entries matching a key prefix into `target` with an additional matching predicate. | `src/memtable/iter.rs` |
| `pub fn scan_range_into( &self, start: Bound<&[u8]>, end: Bound<&[u8]>, seq_no: u64, max_tx: TxId, target: &mut BTreeMap<Bytes, (Bytes, u64)>, )` | Scans entries matching a key range into `target` according to MVCC visibility rules. | `src/memtable/iter.rs` |
| `pub fn scan_range_into_matching<F>( &self, start: Bound<&[u8]>, end: Bound<&[u8]>, seq_no: u64, max_tx: TxId, target: &mut BTreeMap<Bytes, (Bytes, u64)>, predicate: F, ) where F: Fn(&[u8], u64, u64) -> bool,` | Scans entries matching a key range into `target` with an additional matching predicate. | `src/memtable/iter.rs` |
| `pub fn get(&self, key: &[u8]) -> Option<(Bytes, u64)>` | Retrieves the latest value and sequence number for a key. | `src/memtable/mvcc.rs` |
| `pub fn get_at_seq(&self, key: &[u8], seq_no: u64, max_tx: u64) -> Option<(Bytes, u64, u64)>` | Retrieves a value, sequence number, and transaction ID by key at or below a specific sequence number and bounded by maximum transaction ID… | `src/memtable/mvcc.rs` |
| `pub fn put(&self, key: Bytes, value: Bytes, seq_no: u64, tx_id: u64)` | Inserts a key-value pair with sequence number and transaction id into the appropriate shard. | `src/memtable/mvcc.rs` |
| `pub fn rollback(&self, tx_id: u64)` | Rollback: Removes all entries belonging to `tx_id` across all shards. | `src/memtable/mvcc.rs` |
| `pub fn tx_range(&self) -> (u64, u64)` | Returns the minimum and maximum transaction IDs contained in this memtable. | `src/memtable/mvcc.rs` |
| `pub fn hash_remainder(remainder: &[u8]) -> usize` | Helper to compute a deterministic hash for reserved system key remainder. | `src/memtable/shard.rs` |
| `pub fn matching_reserved_prefix(key: &[u8]) -> Option<&'static [u8]>` | Helper to detect if a key matches a reserved system prefix. | `src/memtable/shard.rs` |
| `pub fn new() -> Self` | — | `src/memtable/shard.rs` |
| `pub fn shard_for(key: &[u8]) -> usize` | Maps a key to its owning shard index `0..SHARD_COUNT`. | `src/memtable/shard.rs` |
| `pub fn shard_range_for_bounds( start: Bound<&[u8]>, end: Bound<&[u8]>, ) -> std::ops::RangeInclusive<usize>` | Computes the contiguous shard range `[start, end]` that covers the given bounds. | `src/memtable/shard.rs` |
| `pub fn shard_range_for_prefix(prefix: &[u8]) -> std::ops::RangeInclusive<usize>` | Computes the contiguous shard range `[start, end]` that may contain keys with the given prefix. | `src/memtable/shard.rs` |
| `pub fn is_valid_range_bounds(start: Bound<&[u8]>, end: Bound<&[u8]>) -> bool` | Validates whether start and end key range bounds define a non-empty, valid interval (`start <= end`). | `src/memtable/tombstone.rs` |
| `pub fn contains(&self, file_id: u64, offset: u64) -> bool` | — | `src/sstable/block_cache/mod.rs` |
| `pub fn create_block_cache(capacity_mb: usize) -> Arc<BlockCache>` | Binary search for a key index inside a SSTable data block. Returns `Ok(Ok(index))` if the exact key is found, or `Ok(Err(index))` indicatin… | `src/sstable/block_cache/mod.rs` |
| `pub fn create_block_cache_with_shards(capacity_mb: usize, num_shards: usize) -> Arc<BlockCache>` | Creates a new block cache instance with configurable shard count. Capacity is in MB. | `src/sstable/block_cache/mod.rs` |
| `pub fn current_bytes(&self) -> usize` | Returns current memory usage in bytes across all shards. | `src/sstable/block_cache/mod.rs` |
| `pub fn get(&self, file_id: u64, offset: u64) -> Option<Bytes>` | — | `src/sstable/block_cache/mod.rs` |
| `pub fn hit_ratio(&self) -> f64` | Returns the cache hit ratio (0.0 .. 1.0). | `src/sstable/block_cache/mod.rs` |
| `pub fn hits(&self) -> u64` | Returns the total number of cache hits recorded. | `src/sstable/block_cache/mod.rs` |
| `pub fn insert(&self, file_id: u64, offset: u64, data: Bytes)` | — | `src/sstable/block_cache/mod.rs` |
| `pub fn is_empty(&self) -> bool` | — | `src/sstable/block_cache/mod.rs` |
| `pub fn len(&self) -> usize` | — | `src/sstable/block_cache/mod.rs` |
| `pub fn misses(&self) -> u64` | Returns the total number of cache misses recorded. | `src/sstable/block_cache/mod.rs` |
| `pub fn new(capacity_per_shard: usize) -> Self` | — | `src/sstable/block_cache/mod.rs` |
| `pub fn new_with_shards(capacity_per_shard: usize, num_shards: usize) -> Self` | — | `src/sstable/block_cache/mod.rs` |
| `pub fn reset_metrics(&self)` | Resets hit and miss metrics counters. | `src/sstable/block_cache/mod.rs` |
| `pub fn shard_idx(&self, file_id: u64, offset: u64) -> usize` | — | `src/sstable/block_cache/mod.rs` |
| `pub fn total_capacity_bytes(&self) -> usize` | Returns total configured byte capacity across all shards. | `src/sstable/block_cache/mod.rs` |
| `pub fn binary_search_entry_in_block( block_data: &[u8], offsets_start: usize, num_offsets: usize, key: &[u8], is_v3: bool, ) -> Result<Option<(usize, usize)>>` | — | `src/sstable/block_search.rs` |
| `pub fn binary_search_first_index_in_block( block_data: &[u8], offsets_start: usize, num_offsets: usize, key: &[u8], is_v3: bool, ) -> StoreResult<Option<usize>>` | — | `src/sstable/block_search.rs` |
| `pub fn binary_search_index_in_block( block_data: &[u8], offsets_start: usize, num_offsets: usize, key: &[u8], is_v3: bool, ) -> Result<std::result::Result<usize, usize>>` | — | `src/sstable/block_search.rs` |
| `pub fn block_binary_search( block_data: &[u8], offsets_start: usize, num_offsets: usize, key: &[u8], is_v3: bool, ) -> StoreResult<Option<usize>>` | — | `src/sstable/block_search.rs` |
| `pub fn get_entry_at_index( block_data: &[u8], offsets_start: usize, idx: usize, is_v3: bool, ) -> StoreResult<(usize, usize)>` | Entry offset accessor supporting both 16-bit (legacy v0..v2) and 32-bit (v3+) block formats. | `src/sstable/block_search.rs` |
| `pub fn get_entry_key_at_index( block_data: &[u8], offsets_start: usize, idx: usize, is_v3: bool, ) -> StoreResult<&[u8]>` | — | `src/sstable/block_search.rs` |
| `pub fn from_bytes(data: &[u8]) -> Result<Self>` | — | `src/sstable/bloom.rs` |
| `pub fn hash_pair(key: &[u8]) -> (u64, u64)` | Erzeugt zwei unabhängige 64-bit Hashes aus dem Blake3-Digest. h1 = erste 8 Bytes, h2 = Bytes 8-15 (oder fallback zu h1 ^ const) | `src/sstable/bloom.rs` |
| `pub fn insert(&mut self, key: &[u8])` | Inserts a key into the filter. | `src/sstable/bloom.rs` |
| `pub fn may_contain(&self, key: &[u8]) -> bool` | Checks if a key might be in the filter. | `src/sstable/bloom.rs` |
| `pub fn new(expected_elements: usize, fpr: f64) -> Self` | Creates a new Bloom filter for the expected number of elements and target false positive rate (fpr). ## FPR Trade-off - `fpr = 0.01` (1%) u… | `src/sstable/bloom.rs` |
| `pub fn to_bytes(&self) -> Vec<u8>` | — | `src/sstable/bloom.rs` |
| `pub fn add(&mut self, key: &[u8], value: &[u8], seq_no: u64, tx_id: u64) -> bool` | — | `src/sstable/builder/block.rs` |
| `pub fn build(mut self) -> Bytes` | Finalizes the block and returns the bytes. | `src/sstable/builder/block.rs` |
| `pub fn current_size(&self) -> usize` | — | `src/sstable/builder/block.rs` |
| `pub fn is_empty(&self) -> bool` | — | `src/sstable/builder/block.rs` |
| `pub fn new(block_size: usize) -> Self` | — | `src/sstable/builder/block.rs` |
| `pub fn new_with_version(block_size: usize, format_version: u16) -> Self` | — | `src/sstable/builder/block.rs` |
| `pub async fn add(&mut self, key: &[u8], value: &[u8], seq_no: u64, tx_id: u64) -> Result<()>` | Adds a key-value pair to the SSTable being built. | `src/sstable/builder/writer.rs` |
| `pub async fn create(path: impl AsRef<Path>) -> Result<Self>` | Creates a new SstableBuilder that writes to the given file path. | `src/sstable/builder/writer.rs` |
| `pub async fn create_with_key_manager( path: impl AsRef<Path>, key_manager: Option<Arc<KeyManager>>, ) -> Result<Self>` | — | `src/sstable/builder/writer.rs` |
| `pub async fn create_with_vfs( path: impl AsRef<Path>, key_manager: Option<Arc<KeyManager>>, vfs: Arc<dyn Vfs>, ) -> Result<Self>` | — | `src/sstable/builder/writer.rs` |
| `pub async fn finish(mut self) -> Result<SstableMetadata>` | Finalizes the SSTable and returns metadata. | `src/sstable/builder/writer.rs` |
| `pub fn set_block_size(&mut self, size: usize)` | Configures the target block size for data blocks. | `src/sstable/builder/writer.rs` |
| `pub fn set_bloom_filter(&mut self, expected_keys: usize, fpr: f64)` | Configures the whole-SSTable bloom filter parameters. | `src/sstable/builder/writer.rs` |
| `pub fn set_format_version(&mut self, version: u16)` | Explicitly configures the SSTable format version to write (default: 4). | `src/sstable/builder/writer.rs` |
| `pub fn with_block_size(mut self, size: usize) -> Self` | Sets the target block size for data blocks. | `src/sstable/builder/writer.rs` |
| `pub fn with_bloom_fpr(mut self, expected_keys: usize, fpr: f64) -> Self` | Sets the whole-SSTable bloom filter parameters. | `src/sstable/builder/writer.rs` |
| `pub fn parse(trailer_data: &[u8], path_display: &str) -> StoreResult<Self>` | Parst und validiert den SSTable-Trailer aus den letzten Bytes der Datei. | `src/sstable/format.rs` |
| `pub async fn iter(&self) -> Result<Vec<(Bytes, Bytes, u64)>>` | Iterates over all entries in sorted key order (allocates memory for all entries). | `src/sstable/reader/iter.rs` |
| `pub fn metadata(&self) -> &SstableMetadata` | — | `src/sstable/reader/iter.rs` |
| `pub async fn stream(self: &Arc<Self>) -> Result<SstableStream>` | — | `src/sstable/reader/iter.rs` |
| `pub async fn get(&self, key: &[u8]) -> Result<Option<(Bytes, u64, u64)>>` | Retrieves a value from the SSTable by key. | `src/sstable/reader/lookup.rs` |
| `pub async fn get_at( &self, key: &[u8], max_seq: u64, max_tx: u64, ) -> Result<Option<(Bytes, u64, u64)>>` | — | `src/sstable/reader/lookup.rs` |
| `pub async fn lookup_metrics(&self, key: &[u8]) -> (bool, bool, bool, bool)` | Metrics result for point lookup instrumentation. | `src/sstable/reader/lookup.rs` |
| `pub fn file_path(&self) -> &Path` | — | `src/sstable/reader/mod.rs` |
| `pub fn first_key(&self) -> &Bytes` | — | `src/sstable/reader/mod.rs` |
| `pub fn last_key(&self) -> &Bytes` | — | `src/sstable/reader/mod.rs` |
| `pub fn max_tx_id(&self) -> u64` | — | `src/sstable/reader/mod.rs` |
| `pub fn min_tx_id(&self) -> u64` | — | `src/sstable/reader/mod.rs` |
| `pub fn parse_block_trailer( block_data: &[u8], format_version: u16, ) -> Result<(usize, usize, &[u8])>` | Helper function to parse block trailer across format versions v0..v4. Returns `(num_offsets, offsets_start, bloom_bytes)`. | `src/sstable/reader/mod.rs` |
| `pub async fn open(path: impl AsRef<Path>, block_cache: Arc<BlockCache>) -> Result<Self>` | — | `src/sstable/reader/open.rs` |
| `pub async fn open_with_key_manager( path: impl AsRef<Path>, block_cache: Arc<BlockCache>, key_manager: Option<Arc<KeyManager>>, ) -> Result<Self>` | — | `src/sstable/reader/open.rs` |
| `pub async fn open_with_vfs( path: impl AsRef<Path>, block_cache: Arc<BlockCache>, key_manager: Option<Arc<KeyManager>>, vfs: Arc<dyn Vfs>, ) -> Result<Self>` | — | `src/sstable/reader/open.rs` |
| `pub async fn scan_prefix(&self, prefix: &[u8]) -> Result<Vec<(Bytes, Bytes, u64, u64)>>` | Scans the SSTable for keys starting with the given prefix. | `src/sstable/reader_ext.rs` |
| `pub async fn scan_range( &self, start: std::ops::Bound<&[u8]>, end: std::ops::Bound<&[u8]>, ) -> Result<Vec<(Bytes, Bytes, u64, u64)>>` | Scans all entries within a key range. | `src/sstable/reader_ext.rs` |
| `pub async fn next(&mut self) -> Result<Option<(Bytes, Bytes, u64, u64)>>` | — | `src/sstable/stream.rs` |
| `pub async fn next_entry(&mut self) -> Result<Option<(Bytes, Bytes, u64, u64)>>` | Optimized for compaction: avoids copying if possible. | `src/sstable/stream.rs` |
| `pub fn compute_pressure( wal_depth: usize, scheduler_queue_depth: f32, blocking_util: f32, embedding_queue: usize, max_permits: usize, ) -> SystemPressure` | Berechnet den Systemdruck anhand der Einzelsignale. | `src/system_pressure/calculation.rs` |
| `pub fn compute_pressure( &self, wal_depth: usize, scheduler_queue_depth: f32, blocking_util: f32, embedding_queue: usize, max_permits: usize, ) -> SystemPressure` | Berechnet den aktuellen Druckzustand. | `src/system_pressure/monitor.rs` |
| `pub fn new(sampling_interval: Duration) -> Self` | Erstellt einen neuen Monitor mit SystemClock. | `src/system_pressure/monitor.rs` |
| `pub fn new_with_clock( sampling_interval: Duration, clock: Arc<dyn acta_env::Clock>, ) -> Self` | Erstellt einen neuen Monitor mit einer injizierten Clock. | `src/system_pressure/monitor.rs` |
| `pub async fn run( self, cancellation: CancellationToken, wal_queue_depth_fn: impl Fn() -> usize + Send + 'static, embedding_permits_fn: impl Fn() -> usize + Send + 'static, max_embedding_permits: usize, )` | Startet die periodische Hintergrundüberwachung. | `src/system_pressure/monitor.rs` |
| `pub async fn run_with_metrics_sink( self, cancellation: CancellationToken, wal_queue_depth_fn: impl Fn() -> usize + Send + 'static, embedding_permits_fn: impl Fn() -> usize + Send + 'static, max_embedding_permits: usize, metrics_sink: Option<Arc<parking_lot::…` | Startet die periodische Hintergrundüberwachung mit optionalem MetricsSink. | `src/system_pressure/monitor.rs` |
| `pub fn with_clock(mut self, clock: Arc<dyn acta_env::Clock>) -> Self` | Ersetzt die Clock durch eine übergebene Instanz. | `src/system_pressure/monitor.rs` |
| `pub fn collection_prefix(&self, collection: &CollectionId) -> Vec<u8>` | Collection-spezifisches Scan-Präfix: `t:{tenant}:{col}:` | `src/tenant_codec.rs` |
| `pub fn decode_chunk_key(key: &[u8]) -> Option<(TenantId, CollectionId, DocId)>` | Dekodiert einen Chunk-Key vollständig in (TenantId, CollectionId, DocId). Invariante: decode(encode(x)) == x (Injektivität). | `src/tenant_codec.rs` |
| `pub fn decode_tenant_id(key: &[u8]) -> Option<TenantId>` | Dekodiert TenantId aus encoded Key. None bei ungültigem Format. Kein Unwrap — robuste Nutzung im Recovery-Pfad. | `src/tenant_codec.rs` |
| `pub fn encode_chunk_key(&self, collection: &CollectionId, doc_id: DocId) -> Vec<u8>` | Encodes a document chunk key: `t:{tenant}:{collection}:chunk:{doc}`. | `src/tenant_codec.rs` |
| `pub fn encode_graph_key(&self, collection: &CollectionId, entity_id: u64) -> Vec<u8>` | Encodes a graph entity key: `t:{tenant}:{collection}:graph:{entity}`. | `src/tenant_codec.rs` |
| `pub fn new(tenant_id: TenantId) -> Self` | Creates a new `TenantKeyCodec` for the given `TenantId`. | `src/tenant_codec.rs` |
| `pub fn new(inner: S, tenant_id: TenantId) -> Self` | Creates a new `TenantScopedStorage` wrapping an underlying `StorageEngine` or shared pointer. | `src/tenant_codec.rs` |
| `pub fn scan_prefix(&self) -> &[u8]` | Globales Scan-Präfix für diesen Tenant (alle Collections): `t:{tenant}:` Cached — keine Allokation im Hot-Path. | `src/tenant_codec.rs` |
| `pub fn tenant_id(&self) -> TenantId` | Returns the immutable `TenantId` bound to this storage wrapper. | `src/tenant_codec.rs` |
| `pub fn create_new() -> Self` | Standardoptionen für reines Erzeugen / Schreiben. | `src/traits.rs` |
| `pub fn read_only() -> Self` | Standardoptionen für reines Lesen. | `src/traits.rs` |
| `pub fn new(payload: Vec<u8>) -> Self` | Erzeugt einen neuen [`SealedEnvelope`]. | `src/util.rs` |
| `pub fn payload(&self) -> &[u8]` | Liefert die Rohdaten des Umschlags. | `src/util.rs` |
| `pub fn with_id_gen(mut self, id_gen: Arc<dyn acta_env::IdGen>) -> Self` | — | `src/wal/config.rs` |
| `pub fn with_legacy_fallback(mut self, allow: bool) -> Self` | — | `src/wal/config.rs` |
| `pub fn allow_legacy_fallback_for_test(&self) -> bool` | — | `src/wal/core.rs` |
| `pub fn is_poisoned(&self) -> bool` | Gibt an, ob das Handle vergiftet ist. | `src/wal/core.rs` |
| `pub fn is_sealed(&self) -> bool` | — | `src/wal/core.rs` |
| `pub fn legacy_key_used_for_test(&self) -> bool` | — | `src/wal/core.rs` |
| `pub async fn migrate_legacy_wal( path: impl AsRef<Path>, key_manager: Option<Arc<KeyManager>>, ) -> Result<bool>` | Explicitly migrates a legacy WAL segment to the V3 format signed with a secure integrity key. **INV-WAL-LEGACY-KEY-1**: Legacy migration re… | `src/wal/core.rs` |
| `pub async fn open(path: impl AsRef<Path>) -> Result<Self>` | — | `src/wal/core.rs` |
| `pub async fn open_for_legacy_migration( path: impl AsRef<Path>, key_manager: Option<Arc<KeyManager>>, ) -> Result<Self>` | Explicitly opens a legacy WAL file requiring migration using the legacy integrity key fallback. **INV-WAL-LEGACY-KEY-1**: This is the ONLY… | `src/wal/core.rs` |
| `pub fn open_loom() -> Self` | — | `src/wal/core.rs` |
| `pub async fn open_read_only( path: impl AsRef<Path>, key_manager: Option<Arc<KeyManager>>, ) -> Result<Self>` | Opens an existing WAL file solely for read-only replay, without spawning a background write flusher actor or keeping write handles open. | `src/wal/core.rs` |
| `pub async fn open_read_only_with_config( path: impl AsRef<Path>, config: WalConfig, ) -> Result<Self>` | Opens an existing WAL file solely for read-only replay with explicit configuration. | `src/wal/core.rs` |
| `pub async fn open_with_config(path: impl AsRef<Path>, config: WalConfig) -> Result<Self>` | Opens or creates a WAL file with explicit configuration options. | `src/wal/core.rs` |
| `pub async fn open_with_key_manager( path: impl AsRef<Path>, key_manager: Option<Arc<KeyManager>>, ) -> Result<Self>` | Opens or creates a WAL file with an optional KeyManager. | `src/wal/core.rs` |
| `pub fn path(&self) -> &Path` | — | `src/wal/core.rs` |
| `pub fn poison_cause(&self) -> Option<PoisonCause>` | Gibt die typisierte Ursache des Poisoning zurück, sofern vorhanden. | `src/wal/core.rs` |
| `pub fn poison_for_test(&self)` | — | `src/wal/core.rs` |
| `pub fn poison_with_cause(&self, cause: PoisonCause)` | Vergiftet das Handle mit der angegebenen typisierten Ursache. Das erste Setzen gewinnt (`compare_exchange`), um die primäre Ursache determi… | `src/wal/core.rs` |
| `pub async fn recover_from_poison(&self) -> Result<()>` | Stellt ein mit `PoisonCause::TornWrite` vergiftetes `Wal`-Handle nach einem partiellen Schreibabbruch wieder her. Re-verifiziert den physis… | `src/wal/core.rs` |
| `pub fn size(&self) -> u64` | Helper to expose integrity key for tests | `src/wal/core.rs` |
| `pub async fn truncate_uncommitted_tail(&self) -> Result<u64>` | Truncates any uncommitted entries appearing after the last committed `TxEnd` marker in the WAL. Returns the number of bytes truncated. **Po… | `src/wal/core.rs` |
| `pub fn was_legacy_rekeyed(&self) -> bool` | — | `src/wal/core.rs` |
| `pub fn compute_checksum( op: &WalOp, seq_no: u64, integrity_key: &[u8], prev_hmac: [u8; 32], ) -> StoreResult<[u8` | — | `src/wal/encode.rs` |
| `pub fn compute_checksum_v2( op: &WalOp, seq_no: u64, integrity_key: &[u8], prev_hmac: [u8; 32], ) -> StoreResult<[u8` | Legacy V2 checksum calculation (without tx_id and length-prefixes in HMAC). | `src/wal/encode.rs` |
| `pub fn compute_checksum_v3( op: &WalOp, seq_no: u64, integrity_key: &[u8], prev_hmac: [u8; 32], ) -> StoreResult<[u8` | Computes V3 checksum (includes tx_id and length prefixes for key/value). | `src/wal/encode.rs` |
| `pub fn entries(&self) -> &[WalEntry]` | Returns a slice of the inner WAL entries. | `src/wal/encode.rs` |
| `pub fn extend(&mut self, other: PreparedBatch)` | Extends this prepared batch with entries from another prepared batch. | `src/wal/encode.rs` |
| `pub fn from_bytes(data: &[u8]) -> StoreResult<Self>` | Deserializes a WAL entry from bytes, verifying CRC32. | `src/wal/encode.rs` |
| `pub fn from_bytes_classified(data: &[u8]) -> std::result::Result<Self, WalParseError>` | Deserializes a WAL entry from bytes, returning a classified `WalParseError` on failure. | `src/wal/encode.rs` |
| `pub fn into_inner(self) -> Vec<WalEntry>` | Consumes the `PreparedBatch` and returns the inner vector of entries. | `src/wal/encode.rs` |
| `pub fn is_empty(&self) -> bool` | Returns `true` if the batch contains no entries. | `src/wal/encode.rs` |
| `pub fn len(&self) -> usize` | Returns the number of entries in the batch. | `src/wal/encode.rs` |
| `pub fn to_bytes(&self) -> StoreResult<Vec<u8>>` | Serializes the entry to bytes. | `src/wal/encode.rs` |
| `pub fn try_new( op: WalOp, seq_no: u64, integrity_key: &[u8], prev_hmac: [u8; 32], ) -> StoreResult<Self>` | Creates a new WAL entry with HMAC-SHA256 checksum and chaining. | `src/wal/encode.rs` |
| `pub fn tx_id(&self) -> TxId` | — | `src/wal/encode.rs` |
| `pub fn tx_id(&self) -> TxId` | — | `src/wal/encode.rs` |
| `pub fn append(&mut self, _append: bool) -> &mut Self` | — | `src/wal/fs.rs` |
| `pub async fn copy<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> std::io::Result<u64>` | — | `src/wal/fs.rs` |
| `pub fn create(&mut self, _create: bool) -> &mut Self` | — | `src/wal/fs.rs` |
| `pub async fn create_dir_all<P: AsRef<Path>>(path: P) -> std::io::Result<()>` | — | `src/wal/fs.rs` |
| `pub fn create_new(&mut self, _create_new: bool) -> &mut Self` | — | `src/wal/fs.rs` |
| `pub async fn flush(&self) -> std::io::Result<()>` | — | `src/wal/fs.rs` |
| `pub async fn hard_link<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> std::io::Result<()>` | — | `src/wal/fs.rs` |
| `pub fn len(&self) -> u64` | — | `src/wal/fs.rs` |
| `pub async fn metadata<P: AsRef<Path>>(path: P) -> std::io::Result<LoomMetadata>` | — | `src/wal/fs.rs` |
| `pub async fn metadata(&self) -> std::io::Result<LoomMetadata>` | — | `src/wal/fs.rs` |
| `pub fn mode(&mut self, _mode: u32) -> &mut Self` | — | `src/wal/fs.rs` |
| `pub fn new() -> Self` | — | `src/wal/fs.rs` |
| `pub fn new() -> Self` | — | `src/wal/fs.rs` |
| `pub async fn open<P: AsRef<Path>>(&self, _path: P) -> std::io::Result<LoomFile>` | — | `src/wal/fs.rs` |
| `pub async fn open<P: AsRef<Path>>(_path: P) -> std::io::Result<Self>` | — | `src/wal/fs.rs` |
| `pub fn permissions(&self) -> std::fs::Permissions` | — | `src/wal/fs.rs` |
| `pub async fn read<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<u8>>` | — | `src/wal/fs.rs` |
| `pub fn read(&mut self, _read: bool) -> &mut Self` | — | `src/wal/fs.rs` |
| `pub async fn remove_file<P: AsRef<Path>>(path: P) -> std::io::Result<()>` | — | `src/wal/fs.rs` |
| `pub async fn rename<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> std::io::Result<()>` | — | `src/wal/fs.rs` |
| `pub async fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64>` | — | `src/wal/fs.rs` |
| `pub async fn set_len(&mut self, len: u64) -> std::io::Result<()>` | — | `src/wal/fs.rs` |
| `pub async fn set_permissions<P: AsRef<Path>>( path: P, perm: std::fs::Permissions, ) -> std::io::Result<()>` | — | `src/wal/fs.rs` |
| `pub async fn sync_all(&self) -> std::io::Result<()>` | — | `src/wal/fs.rs` |
| `pub async fn try_exists<P: AsRef<Path>>(path: P) -> std::io::Result<bool>` | — | `src/wal/fs.rs` |
| `pub async fn write<P: AsRef<Path>, C: AsRef<[u8]>>( path: P, contents: C, ) -> std::io::Result<()>` | — | `src/wal/fs.rs` |
| `pub fn write(&mut self, _write: bool) -> &mut Self` | — | `src/wal/fs.rs` |
| `pub async fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()>` | — | `src/wal/fs.rs` |
| `pub fn integrity_key_for_test(&self) -> Result<[u8` | Exposes the HMAC integrity key for testing. | `src/wal/hmac.rs` |
| `pub fn last_hmac_lock_for_test(&self) -> std::sync::Arc<tokio::sync::Mutex<[u8` | Exposes internal `last_hmac` lock for concurrency testing. | `src/wal/hmac.rs` |
| `pub fn legacy_integrity_key_for_test() -> Result<[u8` | — | `src/wal/hmac.rs` |
| `pub fn migrate_legacy_key(legacy: &[u8], new_key_material: &[u8]) -> Result<[u8` | Migrates a legacy integrity key to a new 32-byte key material state. # Chain Compatibility When migrating a legacy WAL segment to a new int… | `src/wal/hmac.rs` |
| `pub fn migrate_legacy_key(_legacy: &[u8], _new_key_material: &[u8]) -> Result<[u8` | — | `src/wal/hmac.rs` |
| `pub async fn prepare_batch(&self, ops: Vec<(WalOp, u64)>) -> Result<(PreparedBatch, [u8` | Prepares a batch of WAL operations with sequential sequence numbers and HMAC hash-chaining. | `src/wal/hmac.rs` |
| `pub async fn rollback_hmac(&self, prev_hmac: [u8; 32])` | Rolls back the in-memory HMAC state to `prev_hmac` asynchronously. | `src/wal/hmac.rs` |
| `pub fn rollback_hmac_sync(&self, prev_hmac: [u8; 32]) -> bool` | Synchronously rolls back the in-memory HMAC state to `prev_hmac` using non-blocking `try_lock`. Suitable for use within `Drop` implementati… | `src/wal/hmac.rs` |
| `pub async fn append_batch(&self, batch: PreparedBatch) -> Result<()>` | — | `src/wal/io/append.rs` |
| `pub async fn append_batch_locked( &self, batch: PreparedBatch, guard: &tokio::sync::MutexGuard<'_, ()>, ) -> Result<()>` | — | `src/wal/io/append.rs` |
| `pub async fn enqueue_append_batch_locked( &self, batch: PreparedBatch, _guard: &tokio::sync::MutexGuard<'_, ()>, ) -> Result<tokio::sync::oneshot::Receiver<Result<()>>>` | — | `src/wal/io/append.rs` |
| `pub async fn try_append_batch(&self, batch: PreparedBatch) -> Result<()>` | Non-blocking attempt to append a prepared batch. Returns `Err(StoreError::Storage("WAL queue full (backpressure)"))` if the flusher channel… | `src/wal/io/append.rs` |
| `pub async fn try_append_batch_locked( &self, batch: PreparedBatch, guard: &tokio::sync::MutexGuard<'_, ()>, ) -> Result<()>` | — | `src/wal/io/append.rs` |
| `pub async fn try_enqueue_append_batch_locked( &self, batch: PreparedBatch, _guard: &tokio::sync::MutexGuard<'_, ()>, ) -> Result<tokio::sync::oneshot::Receiver<Result<()>>>` | — | `src/wal/io/append.rs` |
| `pub fn arm_truncate_failure_for_test(&self)` | Aktiviert eine einmalige simulierte Truncate-I/O-Failure für Tests. Der Flusher gibt beim nächsten `WalCommand::Truncate` einen Fehler zurü… | `src/wal/io/truncate.rs` |
| `pub async fn find_tx_offset(&self, target_tx_id: TxId) -> Result<(u64, [u8` | — | `src/wal/io/truncate.rs` |
| `pub async fn last_hmac_snapshot(&self) -> [u8` | Returns a snapshot of the last HMAC written to the log. | `src/wal/io/truncate.rs` |
| `pub async fn rotate_and_seal(&self) -> Result<PathBuf>` | — | `src/wal/io/truncate.rs` |
| `pub async fn scan_entries_with_callback<F>( &self, file_size: u64, mut callback: F, ) -> Result<WalVersion> where F: FnMut(u64, WalEntry, u64) -> bool,` | Scans the WAL entry by entry, executing full HMAC chain validation, CRC checks, and key manager decryption. Invokes `callback(seq_no, entry… | `src/wal/io/truncate.rs` |
| `pub async fn truncate(&self, offset: u64, new_last_hmac: [u8; 32]) -> Result<()>` | — | `src/wal/io/truncate.rs` |
| `pub async fn replay(&self) -> Result<Vec<(u64, WalEntry, u64)>>` | Replays the WAL using memory mapping (`memmap2`) for zero-copy entry decoding. Falls back to stream replay if mmap or parsing fails. | `src/wal/replay.rs` |
| `pub async fn replay_from(&self, since_seq_no: u64) -> Result<Vec<WalEntry>>` | Replays the WAL and returns all entries with seq_no > since_seq_no. | `src/wal/replay.rs` |
| `pub async fn replay_mmap(&self) -> Result<(Vec<(u64, WalEntry, u64)>, WalVersion)>` | Replays the WAL using zero-copy memory mapping (`mmap`). Returns all valid entries with sequence numbers, entries, end offsets, and detecte… | `src/wal/replay.rs` |
| `pub async fn replay_mmap_with_sink<S: ReplayProgressSink>( &self, sink: &S, ) -> Result<(Vec<(u64, WalEntry, u64)>, WalVersion)>` | Replays the WAL using zero-copy memory mapping (`mmap`) with progress callback. | `src/wal/replay.rs` |
| `pub async fn replay_stream(&self) -> Result<Vec<(u64, WalEntry, u64)>>` | Replays the WAL using the stream reader (`BufReader`). | `src/wal/replay.rs` |
| `pub async fn replay_with_sink<S: ReplayProgressSink>( &self, sink: &S, ) -> Result<Vec<(u64, WalEntry, u64)>>` | Replays the WAL while notifying the provided `ReplayProgressSink` of progress. | `src/wal/replay.rs` |
| `pub async fn scan_entries_mmap<F>(&self, file_size: u64, mut callback: F) -> Result<WalVersion> where F: FnMut(u64, WalEntry, u64) -> bool,` | Scans the WAL using read-only memory mapping (`memmap2`) for zero-copy entry parsing. If mmap mapping or slice scanning encounters any erro… | `src/wal/replay.rs` |


---

## `acta-checkpoint`

| Attribut | Wert |
|---|---|
| **Ring** | 1 — Ebene E1 — Speicherschicht, Indizes & KI-Kerne |
| **Status** | Fertig |
| **Zweck** | Time-Travel-Registry und Checkpoint-Verwaltung ohne globalen Zustand. |
| **Unsafe-Code** | Nein (`#![forbid(unsafe_code)]`) |
| **Interne Abhängigkeiten** | `acta-env`, `acta-ids`, `acta-ports`, `acta-types` |
| **Externe Abhängigkeiten (Auswahl)** | `blake3`, `bytes`, `parking_lot`, `serde`, `serde_json`, `thiserror`, `tokio`, `tracing` |

### Modul- und Re-Export-Oberfläche (lib.rs)

```rust
pub mod compat;
pub mod contract;
pub use vfs::CheckpointVfs;
pub use error::{CheckpointError, CheckpointResult};
pub use fs_seam::{FsSeam, StdFsSeam};
pub use guard::{CheckpointGuard, PinGuard};
pub use hardlink_cloner::{ CheckpointHardlinkCloner, DefaultHardlinkCloner, HardlinkCloneResult, SnapshotPinner, };
pub use manifest::CheckpointManifest;
pub use meta::{CheckpointMeta, StateCheckpoint, CURRENT_ID_WIDTH_BITS};
pub use orphan::{ await_pending_rollbacks, clear_all_orphaned_checkpoints, clear_orphaned_checkpoint, get_orphaned_checkpoints, get_orphaned_checkpoints_for_namespace, global_orphan_registry, monotonic_timestamp_ms, monotonic_timestamp_ms_with, orphaned_checkpoint_count, pending_rollback_count, register_orphaned_checkpoint, register_pinned_seq_no_orphan, InstanceOrphanRegistry, OrphanRegistry, OrphanState, PinId, PinnedSeqNoOrphan, RecoveryReport, };
pub use snapshot_source::{BoxFuture as LocalBoxFuture, SnapshotSource};
pub use store::{CheckpointRegistry, PersistentCheckpointStore};
```

### Feature-Flags

| Flag | Aktiviert |
|---|---|
| `default` | `["workspace"]` |
| `workspace` | `["dep:acta-types", "dep:acta-ports"]` |
| `types-compat` | `["workspace"]` |
| `ports-compat` | `["workspace"]` |
| `docid-128` | `["acta-ids/docid-128"]` |

### Schnittstellen — Public Traits (6)

**`CheckpointHardlinkCloner`** — *Trait für die Schnittstelle zum physischen SSTable-Klonen via Hardlinks. # Zweck Abstrahiert das Dateisystem-Klonen für Tests und verschiedene Plattformen. # Windows NTFS Limit Unter Windows ist `CreateHardLink` NTFS-exklusiv und auf maximal **1023 Links** pr…* (`src/hardlink_cloner.rs`)

```rust
pub trait CheckpointHardlinkCloner : Send + Sync {
/// Klont alle für `seq_no` sichtbaren SSTable-Dateien im Quellverzeichnis ins Zielverzeichnis.
    ///
    /// Hält während der Operation einen aktiven `SnapshotRegistry`-Pin auf `seq_no`.
    ///
    /// # Parameter
    /// - `seq_no`: Sequenznummer des MVCC-Snapshots.
    /// - `source_dir`: Quellverzeichnis mit bestehenden `.sst`-Dateien.
    /// - `target_dir`: Zielverzeichnis für die Klone.
    /// - `snapshot_registry`: Referenz auf die MVCC-Snapshot-Registry zum Pinning.
    ///
    /// # Fehler
    /// Gibt einen Fehler zurück bei Dateisystemproblemen oder unauflösbaren Link-Konflikten.
    fn clone_sstable_hardlinks<'a>(
        &'a self,
        seq_no: u64,
        source_dir: &'a Path,
        target_dir: &'a Path,
        snapshot_pinner: &'a (dyn SnapshotPinner + 'a),
    ) -> BoxFuture<'a, Result<HardlinkCloneResult>>;
}
```

**`CheckpointRegistry`** — *Trait für die erweiterte Checkpoint-Verwaltung inklusive Orphan-Recovery. Definiert die Schnittstelle zum Speichern, Laden und Bereinigen von Checkpoints.* (`src/store.rs`)

```rust
pub trait CheckpointRegistry : Send + Sync {
/// Speichert die Metadaten eines Checkpoints asynchron ab.
    fn save_checkpoint<'a>(&'a self, meta: CheckpointMeta) -> BoxFuture<'a, Result<()>>;
    /// Lädt die Metadaten eines Checkpoints anhand der Sequenznummer.
    fn load_checkpoint<'a>(&'a self, seq_no: u64) -> BoxFuture<'a, Result<Option<CheckpointMeta>>>;
    /// Listet alle im Store registrierten Checkpoints auf.
    fn list_checkpoints<'a>(&'a self) -> BoxFuture<'a, Result<Vec<CheckpointMeta>>>;

    /// Liefert eine Referenz auf die zugeordnete Orphan-Registry, falls vorhanden.
    fn orphan_registry(&self) -> Option<&Arc<InstanceOrphanRegistry>> {
        None
    }

    /// Stellt verwaiste Pins wieder her und unpinnt sie in der Speicherengine.
    fn recover_orphaned_pins<'a>(&'a self) -> BoxFuture<'a, Result<Vec<PinId>>> {
        Box::pin(async move { Ok(Vec::new()) })
    }

    /// Führt für alle verwaisten Checkpoints ein kontrolliertes Rollback aus.
    fn recover_orphaned_checkpoints<'a>(&'a self) -> BoxFuture<'a, Result<Vec<TxId>>> {
        Box::pin(async move { Ok(Vec::new()) })
    }

    /// Schreibt ungespeicherte Orphan-Einträge auf die Festplatte.
    fn flush_orphan_registry<'a>(&'a self) -> BoxFuture<'a, Result<()>> {
        Box::pin(async move { Ok(()) })
    }
}
```

**`CheckpointVfs`** — *Lokale minimale VFS-Schnittstelle für `acta-checkpoint`.* (`src/vfs.rs`)

```rust
pub trait CheckpointVfs : Send + Sync {
/// Schreibt Daten atomar an den angegebenen Pfad.
    fn write_atomic(&self, path: &Path, data: &[u8]) -> std::io::Result<()>;

    /// Liest den gesamten Dateiinhalt an dem angegebenen Pfad.
    fn read(&self, path: &Path) -> std::io::Result<Vec<u8>>;
}
```

**`FsSeam`** — *Minimale Schnittstelle für Dateisystemzugriffe.* (`src/fs_seam.rs`)

```rust
pub trait FsSeam : Send + Sync {
/// Schreibt Daten atomar oder direkt in eine Datei.
    fn write_file(&self, path: &Path, content: &[u8]) -> io::Result<()>;

    /// Liest den gesamten Inhalt einer Datei.
    fn read_file(&self, path: &Path) -> io::Result<Vec<u8>>;

    /// Prüft, ob ein Pfad existiert.
    fn exists(&self, path: &Path) -> bool;

    /// Löscht eine Datei, falls sie existiert.
    fn remove_file(&self, path: &Path) -> io::Result<()>;

    /// Erzeugt ein Verzeichnis rekursiv.
    fn create_dir_all(&self, path: &Path) -> io::Result<()>;
}
```

**`SnapshotPinner`** — *Trait zur Abstraktion von Snapshot-Pinning-Mechanismen während des SSTable-Klonens.* (`src/hardlink_cloner.rs`)

```rust
pub trait SnapshotPinner : Send + Sync {
/// Pinnt eine Sequenznummer und liefert einen RAII-Guard zurück.
    fn pin_seq_no(&self, seq_no: u64) -> Box<dyn std::any::Any + Send + Sync>;
}
```

**`SnapshotSource`** — *Lokale Schnittstelle zur Speicher-Engine für Snapshot-, Checkpoint- und Rollback-Mechanik.* (`src/snapshot_source.rs`)

```rust
pub trait SnapshotSource : Send + Sync + 'static {
/// Liest einen Wert zu einem Schlüssel.
    fn get<'a>(&'a self, key: &'a [u8]) -> BoxFuture<'a, Result<Option<Bytes>>>;

    /// Schreibt ein Schlüssel-Wert-Paar im Transaktionskontext.
    fn put<'a>(&'a self, tx_id: TxId, key: &'a [u8], value: &'a [u8]) -> BoxFuture<'a, Result<()>>;

    /// Löscht einen Schlüssel im Transaktionskontext.
    fn delete<'a>(&'a self, tx_id: TxId, key: &'a [u8]) -> BoxFuture<'a, Result<()>>;

    /// Bestätigt eine Transaktion.
    fn commit<'a>(&'a self, tx_id: TxId) -> BoxFuture<'a, Result<()>>;

    /// Rollt eine Transaktion zurück.
    fn rollback<'a>(&'a self, tx_id: TxId) -> BoxFuture<'a, Result<()>>;

    /// Setzt die Engine auf den Zustand einer früheren Transaktion zurück.
    fn rollback_to_tx<'a>(&'a self, tx_id: TxId) -> BoxFuture<'a, Result<()>>;

    /// Flusht anstehende Daten auf die Festplatte.
    fn flush<'a>(&'a self) -> BoxFuture<'a, Result<()>>;

    /// Liefert die höchste vergebene Sequenznummer.
    fn last_seq_no<'a>(&'a self) -> BoxFuture<'a, Result<u64>>;

    /// Liefert die ID der letzten Transaktion.
    fn last_tx_id<'a>(&'a self) -> BoxFuture<'a, Result<TxId>>;

    /// Pinnt einen Snapshot bei der angegebenen Sequenznummer gegen GC.
    fn pin_checkpoint<'a>(&'a self, seq_no: u64) -> BoxFuture<'a, Result<()>>;

    /// Gibt den gepinnten Snapshot wieder frei.
    fn unpin_checkpoint<'a>(&'a self, seq_no: u64) -> BoxFuture<'a, Result<()>>;

    /// Scannt Schlüssel mit dem angegebenen Präfix.
    #[allow(clippy::type_complexity)]
    fn scan_prefix<'a>(
        &'a self,
        prefix: &'a [u8],
    ) -> BoxFuture<'a, Result<Vec<(Vec<u8>, Vec<u8>)>>>;
}
```

### Öffentliche Datentypen — Structs (19)

| Struct | Felder / Form | Beschreibung | Datei |
|---|---|---|---|
| `CheckpointGuard` | `{ pub(crate) checkpoint: Option<StateCheckpoint>, pub(crate) storage: Arc<S>, pub(crate) namespace: String, pub(crate) orphan_registry: Arc<InstanceOrphanRegistr… }` | RAII Guard, der explizit über [`commit`](Self::commit) oder [`rollback`](Self::rollback) finalisiert werden MUSS. # RAII-Kontrakt und Betriebshinweis… | `src/guard.rs` |
| `CheckpointManifest` | `{ /// Zugehörige Checkpoint-Metadaten. pub meta: CheckpointMeta, /// Liste der beteiligten Speicherkomponenten oder SSTable-Dateien. pub components: Vec<String>,… }` | Kryptografisch gesichertes Manifest eines gespeicherten Checkpoints. # Zweck Bindet Metadaten (`CheckpointMeta`) und zugehörige Komponenten-Dateien (… | `src/manifest.rs` |
| `CheckpointMeta` | `{ /// Eindeutiger Name des Checkpoints. pub name: String, /// ID der zugeordneten Collection. pub collection_id: String, /// LSM-Sequenznummer zum Erstellungszei… }` | Metadaten für einen persistenten, benannten Checkpoint. # Zweck Speichert alle Identifikations-, Versions- und Audit-Informationen zu einem Snapshot… | `src/meta.rs` |
| `DefaultHardlinkCloner` | `(tuple/unit)` | Produktive Implementierung von [`CheckpointHardlinkCloner`] basierend auf `tokio::fs`. Nutzt asynchrone Dateisystem-Operationen ohne den Haupt-Thread… | `src/hardlink_cloner.rs` |
| `HardlinkCloneResult` | `{ /// Die gepinnte und geklonte Quell-Sequenznummer. pub source_seq_no: u64, /// Liste der Zielpfade, die als Hardlinks oder Kopien erzeugt wurden. pub linked_fi… }` | Ergebnisstruktur einer durchgeführten Hardlink-Klon-Operation für SSTables. # Zweck Liefert dem Aufrufer die Sequenznummer sowie die Liste aller erfo… | `src/hardlink_cloner.rs` |
| `InMemorySnapshotSource` | `{ data: RwLock<VersionedKeyMap>, last_seq: AtomicU64, last_tx: AtomicU64, pinned: RwLock<std::collections::BTreeSet<u64>>, }` | Vollständige, I/O-freie Referenzimplementierung einer `SnapshotSource`. Geeignet für Standalone-Einsätze, Tests und In-Memory-Verwendung ohne Speiche… | `src/contract.rs` |
| `InstanceOrphanRegistry` | `{ pins: Mutex<Vec<PinnedSeqNoOrphan>>, checkpoints: Mutex<Vec<StateCheckpoint>>, persist_path: std::path::PathBuf, is_dirty: AtomicBool, clock: Arc<dyn Clock>, v… }` | Instanz-gebundene Orphan-Registry für Checkpoints und gepinnte Sequenznummern (ADR-053). # Thread-Sicherheit & Nebenläufigkeit Verwendet [`parking_lo… | `src/orphan.rs` |
| `LoomDirEntry` | `(tuple/unit)` | — | `src/hardlink_cloner.rs` |
| `LoomMetadata` | `(tuple/unit)` | — | `src/hardlink_cloner.rs` |
| `LoomReadDir` | `(tuple/unit)` | — | `src/hardlink_cloner.rs` |
| `OrphanRegistry` | `{ inner: InstanceOrphanRegistry, }` | — | `src/orphan.rs` |
| `OrphanState` | `{ /// Liste der verwaisten Checkpoints. pub checkpoints: Vec<StateCheckpoint>, /// Liste der verwaisten Sequenznummern-Pins. pub pinned_seq_nos: Vec<PinnedSeqNoO… }` | Serialisierbarer Gesamtzustand aller verwaisten Checkpoints und Pins einer Instanz. # Felder - `checkpoints`: Liste unvollendeter Transaktions-Checkp… | `src/orphan.rs` |
| `PersistentCheckpointStore` | `{ pub(crate) storage: Arc<S>, /// Registrierter Checkpoint-Index im Arbeitsspeicher — geschützt durch ein konsolidiertes RwLock (P-23) index: RwLock<CheckpointIn… }` | Registry für gespeicherte Checkpoints mit Thread-sicherem Zustand. # Zweck Verwaltet Checkpoint-Metadaten, koordiniert MVCC-Sequenznummern und stellt… | `src/store.rs` |
| `PinGuard` | `{ storage: Arc<S>, seq_no: Option<u64>, orphan_registry: Arc<InstanceOrphanRegistry>, clock: Arc<dyn Clock>, }` | RAII-Guard für gepinnte Checkpoint-Sequenznummern (gemäß ADR-015). # Zweck Stellt sicher, dass eine gepinnte Sequenznummer bei einem Fehler oder Pani… | `src/guard.rs` |
| `PinnedSeqNoOrphan` | `{ /// Die verwaiste LSM-Sequenznummer. pub seq_no: u64, /// Zeitstempel der Pin-Erstellung in Millisekunden. pub timestamp_ms: u64, }` | Verwaiste gepinnte Sequenznummer — wird beim Recovery verarbeitet. # Felder - `seq_no`: Die Sequenznummer, die entpinnt werden muss. - `timestamp_ms`… | `src/orphan.rs` |
| `RecoveryReport` | `{ /// Liste aller erfolgreich entpinnten Sequenznummern. pub recovered_pins: Vec<PinId>, /// Liste aller bereinigten Transaktions-IDs. pub recovered_checkpoints:… }` | Bericht über wiederhergestellte und aufgeräumte verwaiste Pins und Checkpoints. # Felder - `recovered_pins`: Liste aller erfolgreich entpinnten Seque… | `src/orphan.rs` |
| `StateCheckpoint` | `{ /// Transaktions-ID des Zustands-Snapshots. pub tx_id: TxId, /// Zeitstempel der Erstellung in Millisekunden. pub timestamp_ms: u64, /// Optionaler Namespace b… }` | Point-in-Time Checkpoint für einen Agenten-Schritt oder eine Transaktionsgrenze. # Zweck Repräsentiert einen leichten Transaktions-Zwischenstand zur… | `src/meta.rs` |
| `StdFsSeam` | `(tuple/unit)` | Standard-Implementierung basierend auf `std::fs`. | `src/fs_seam.rs` |
| `WorkflowState` | `{ /// Associated transaction. pub tx: TxId, /// Agent memory graph state footprint (BLAKE3 hash digest). pub graph_hash: [u8; 32], }` | Defines a frozen workflow state acting as a savepoint. | `src/meta.rs` |

### Öffentliche Datentypen — Enums (1)

| Enum | Varianten | Beschreibung | Datei |
|---|---|---|---|
| `CheckpointError` | `/// Ungültige Eingabeparameter (z. B. leere Namen oder Grenzwertüberschreitungen). #[error("Invalid input: {0}")] InvalidInput(String), /// Serialisierungs- oder Deserialisierungsfehler (JSON/BLAKE3)…` | Fehlerzustände im Checkpoint-Subsystem. | `src/error.rs` |

### Typ-Aliase (5)

| Alias | Definition | Beschreibung |
|---|---|---|
| `BoxFuture` | `type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>` | Lokaler Alias für pinning-boxed Futures ohne externe Abhängigkeit. |
| `CheckpointResult` | `type CheckpointResult<T> = std::result::Result<T, CheckpointError>` | Typalias für Resultate mit [`CheckpointError`]. |
| `PinId` | `type PinId= u64` | Typ-Alias für Sequenznummern, die als gepinnte Checkpoint-Identifikatoren verwaltet werden. |
| `Result` | `type Result<T> = CheckpointResult<T>` | Kurzform für interne Module, identisch zu [`CheckpointResult`]. |
| `VersionedKeyMap` | `type VersionedKeyMap= BTreeMap<Vec<u8>, BTreeMap<u64, Option<Vec<u8>>>>` | Vollständige, I/O-freie Referenzimplementierung einer `SnapshotSource`. Geeignet für Standalone-Einsätze, Tests und In-Memory-Verwendung ohne Speiche… |

### Konstanten (1)

| Konstante | Typ | Beschreibung |
|---|---|---|
| `CURRENT_ID_WIDTH_BITS` | `u16` | Aktuelle ID-Breite in Bits (SPEC §5.2). |

### Öffentliche Funktionen (116)

| Signatur | Beschreibung | Datei |
|---|---|---|
| `pub fn get_sync(&self, key: &[u8]) -> Option<Vec<u8>>` | Synchroner Datenzugriff für Tests oder Debugging. | `src/contract.rs` |
| `pub fn new() -> Self` | Erzeugt eine neue, leere In-Memory-Quelle. | `src/contract.rs` |
| `pub fn cross_device_link( source_path: impl Into<String>, target_path: impl Into<String>, ) -> Self` | Erzeugt einen `CrossDeviceLink`-Fehler. | `src/error.rs` |
| `pub fn internal(msg: impl Into<String>) -> Self` | Erzeugt einen `Internal`-Fehler. | `src/error.rs` |
| `pub fn invalid_input(msg: impl Into<String>) -> Self` | Erzeugt einen `InvalidInput`-Fehler. | `src/error.rs` |
| `pub fn new() -> Self` | Erzeugt eine neue Instanz von [`StdFsSeam`]. | `src/fs_seam.rs` |
| `pub fn checkpoint(&self) -> Result<&StateCheckpoint>` | Liefert eine Referenz auf den enthaltenen Checkpoint, sofern noch nicht konsumiert. | `src/guard.rs` |
| `pub fn clock(&self) -> &Arc<dyn Clock>` | Liefert eine Referenz auf den injizierten Clock-Port. | `src/guard.rs` |
| `pub fn clock(&self) -> &Arc<dyn Clock>` | Liefert den verwendeten Clock-Port. | `src/guard.rs` |
| `pub fn commit(mut self) -> Result<StateCheckpoint>` | Überführt den Guard in einen dauerhaften Zustand (Commit) und konsumiert ihn. # Ablauf von `commit`: 1. Checkpoint aus Option entnehmen (be… | `src/guard.rs` |
| `pub fn defuse(mut self)` | Entschärft den Guard nach erfolgreicher Persistierung, sodass die Sequenznummer gepinnt bleibt. | `src/guard.rs` |
| `pub async fn for_agent_step(storage: Arc<S>, tx: TxId) -> Result<Self>` | Erstellt einen neuen CheckpointGuard für einen Agenten-Schritt mit SystemClock. | `src/guard.rs` |
| `pub async fn for_agent_step_with_clock( storage: Arc<S>, tx: TxId, clock: Arc<dyn Clock>, ) -> Result<Self>` | Erstellt einen CheckpointGuard für einen Agenten-Schritt mit speziellem Clock-Port. | `src/guard.rs` |
| `pub async fn for_agent_step_with_registry( storage: Arc<S>, tx: TxId, orphan_registry: Arc<InstanceOrphanRegistry>, ) -> Result<Self>` | Erstellt einen CheckpointGuard für einen Agenten-Schritt mit spezifischer Orphan-Registry. | `src/guard.rs` |
| `pub fn new(checkpoint: StateCheckpoint, storage: Arc<S>, namespace: impl Into<String>) -> Self` | Erzeugt einen neuen `CheckpointGuard` mit Standard-SystemClock. | `src/guard.rs` |
| `pub async fn pin( storage: Arc<S>, seq_no: u64, orphan_registry: Arc<InstanceOrphanRegistry>, ) -> Result<Self>` | Pinnt eine Sequenznummer in der Storage-Engine unter Verwendung der Clock aus der Orphan-Registry. # Parameter - `storage`: Die zugrundelie… | `src/guard.rs` |
| `pub async fn pin_with_clock( storage: Arc<S>, seq_no: u64, orphan_registry: Arc<InstanceOrphanRegistry>, clock: Arc<dyn Clock>, ) -> Result<Self>` | Pinnt eine Sequenznummer mit explizit übergebenem Clock-Port. # Ablauf von `pin_with_clock`: 1. `storage.pin_checkpoint(seq_no)` asynchron… | `src/guard.rs` |
| `pub async fn rollback(mut self) -> Result<()>` | Führt ein manuelles, asynchrones Rollback des Checkpoints aus. Nach Aufruf von `rollback()` ist der Guard konsumiert, sodass beim Drop kein… | `src/guard.rs` |
| `pub fn rollback_blocking(mut self) -> Result<()>` | Führt ein synchrones, blockierendes Rollback des Checkpoints aus. # Einschränkung Diese Methode darf **nicht** aus einem laufenden asynchro… | `src/guard.rs` |
| `pub async fn unpin(mut self) -> Result<()>` | Entpinnt die Sequenznummer explizit und asynchron bei einem abgefangenen Fehler. # Ablauf von `unpin`: 1. `seq_no` entnehmen. 2. `storage.u… | `src/guard.rs` |
| `pub fn with_clock( checkpoint: StateCheckpoint, storage: Arc<S>, namespace: impl Into<String>, clock: Arc<dyn Clock>, ) -> Self` | Erzeugt einen neuen `CheckpointGuard` mit injiziertem Clock-Port. | `src/guard.rs` |
| `pub fn with_metrics_sink(mut self, sink: Arc<dyn MetricsSink>) -> Self` | Ergänzt den Guard um eine Metrik-Senke zur Erfassung von Ausführungsdauern. | `src/guard.rs` |
| `pub fn with_metrics_sink_opt(mut self, sink: Option<Arc<dyn MetricsSink>>) -> Self` | Ergänzt den Guard optional um eine Metrik-Senke. | `src/guard.rs` |
| `pub fn with_registry( checkpoint: StateCheckpoint, storage: Arc<S>, namespace: impl Into<String>, orphan_registry: Arc<InstanceOrphanRegistry>, ) -> Self` | Erzeugt einen neuen `CheckpointGuard` mit vorhandener Orphan-Registry. | `src/guard.rs` |
| `pub fn with_registry_and_counter( checkpoint: StateCheckpoint, storage: Arc<S>, namespace: impl Into<String>, orphan_registry: Arc<InstanceOrphanRegistry>, skipped_rollbacks: Arc<AtomicU64>, ) -> Self` | Erzeugt einen neuen `CheckpointGuard` mit vorhandener Orphan-Registry und Zähler für übersprungene Rollbacks. | `src/guard.rs` |
| `pub fn with_registry_counter_and_clock( checkpoint: StateCheckpoint, storage: Arc<S>, namespace: impl Into<String>, orphan_registry: Arc<InstanceOrphanRegistry>, skipped_rollbacks: Arc<AtomicU64>, clock: Arc<dyn Clock>, ) -> Self` | Vollständiger interner Konstruktor mit allen Parametern. | `src/guard.rs` |
| `pub async fn copy<P: AsRef<Path>, Q: AsRef<Path>>(_from: P, _to: Q) -> std::io::Result<u64>` | — | `src/hardlink_cloner.rs` |
| `pub async fn create_dir_all<P: AsRef<Path>>(_path: P) -> std::io::Result<()>` | — | `src/hardlink_cloner.rs` |
| `pub async fn hard_link<P: AsRef<Path>, Q: AsRef<Path>>( _from: P, _to: Q, ) -> std::io::Result<()>` | — | `src/hardlink_cloner.rs` |
| `pub async fn metadata<P: AsRef<Path>>(_path: P) -> std::io::Result<LoomMetadata>` | — | `src/hardlink_cloner.rs` |
| `pub fn new() -> Self` | Erzeugt eine neue Instanz von [`DefaultHardlinkCloner`]. | `src/hardlink_cloner.rs` |
| `pub async fn next_entry(&mut self) -> std::io::Result<Option<LoomDirEntry>>` | — | `src/hardlink_cloner.rs` |
| `pub fn path(&self) -> std::path::PathBuf` | — | `src/hardlink_cloner.rs` |
| `pub async fn read_dir<P: AsRef<Path>>(_path: P) -> std::io::Result<LoomReadDir>` | — | `src/hardlink_cloner.rs` |
| `pub async fn remove_file<P: AsRef<Path>>(_path: P) -> std::io::Result<()>` | — | `src/hardlink_cloner.rs` |
| `pub fn new(meta: CheckpointMeta, components: Vec<String>) -> Result<Self>` | Erzeugt ein neues [`CheckpointManifest`] und berechnet die BLAKE3-Prüfsumme. # Vorbedingungen - `meta.name` und `meta.collection_id` müssen… | `src/manifest.rs` |
| `pub fn verify(&self) -> Result<()>` | Verifiziert die Integrität des Manifests anhand der gespeicherten Prüfsumme. # Ablauf von `verify`: 1. Tupel `(&self.meta, &self.components… | `src/manifest.rs` |
| `pub fn into_workflow_state(&self) -> WorkflowState` | Konvertiert die Metadaten in einen [`WorkflowState`]. # Kompatibilität Behält `&self` für Rückwärtskompatibilität mit externen Aufrufern be… | `src/meta.rs` |
| `pub fn to_workflow_state(&self) -> WorkflowState` | Konvertiert die Metadaten in einen [`WorkflowState`]. # Ablauf: 1. Transaktions-ID aus `tx_id` übernehmen. 2. Deterministic Hash aus der Se… | `src/meta.rs` |
| `pub fn validate_id_width(&self) -> Result<()>` | Validiert die ID-Breite gegen die aktuelle Build-Konfiguration (SPEC §5.2). | `src/meta.rs` |
| `pub async fn await_pending_rollbacks()` | Rückwärtskompatibilität: Wartet auf ausstehende Rollbacks (No-op, da Hintergrund-Tasks entfernt wurden). | `src/orphan.rs` |
| `pub fn clear_all(&self)` | Löscht alle Einträge im Speicher und auf der Platte synchron. | `src/orphan.rs` |
| `pub fn clear_all(&self)` | Löscht alle Einträge (Pins und Checkpoints) und leert die Statusdatei. | `src/orphan.rs` |
| `pub fn clear_all_orphaned_checkpoints()` | — | `src/orphan.rs` |
| `pub fn clear_orphan_pin(&self, seq_no: u64)` | Entfernt eine einzelne gepinnte Sequenznummer aus dem Speicher und persistiert. | `src/orphan.rs` |
| `pub fn clear_orphaned_checkpoint(&self, tx_id: TxId)` | Entfernt einen einzelnen verwaisten Checkpoint anhand seiner Transaktions-ID und persistiert. | `src/orphan.rs` |
| `pub fn clear_orphaned_checkpoint(_tx_id: TxId)` | — | `src/orphan.rs` |
| `pub fn clock(&self) -> &Arc<dyn Clock>` | Liefert eine Referenz auf den injizierten Clock-Port. | `src/orphan.rs` |
| `pub fn drain_orphan_pins(&self) -> Vec<PinnedSeqNoOrphan>` | Entnimmt alle verwaisten Pins aus dem Speicher und aktualisiert die Statusdatei. | `src/orphan.rs` |
| `pub fn drain_orphaned_checkpoints(&self) -> Vec<StateCheckpoint>` | Entnimmt alle verwaisten Checkpoints aus dem Speicher und aktualisiert die Statusdatei. | `src/orphan.rs` |
| `pub async fn flush_orphan_registry(&self) -> std::io::Result<()>` | Flusht die In-Memory-Einträge asynchron auf Festplatte, sofern Änderungen vorliegen (`is_dirty`). # Ablauf von `flush_orphan_registry`: 1.… | `src/orphan.rs` |
| `pub fn get_orphan_pins(&self) -> Vec<PinnedSeqNoOrphan>` | Liefert eine Momentaufnahme aller verwaisten Pins im Speicher. | `src/orphan.rs` |
| `pub fn get_orphaned_checkpoints(&self) -> Vec<StateCheckpoint>` | Liefert eine Momentaufnahme aller verwaisten Checkpoints im Speicher. | `src/orphan.rs` |
| `pub fn get_orphaned_checkpoints() -> Vec<StateCheckpoint>` | — | `src/orphan.rs` |
| `pub fn get_orphaned_checkpoints_for_namespace(ns: &str) -> Vec<StateCheckpoint>` | Liefert verwaiste Checkpoints eines bestimmten Namespaces (veraltet). | `src/orphan.rs` |
| `pub fn get_orphans(&self) -> Vec<PinId>` | Liefert alle aktuell registrierten verwaisten Pin-Sequenznummern. | `src/orphan.rs` |
| `pub fn global_orphan_registry() -> &'static OrphanRegistry` | — | `src/orphan.rs` |
| `pub fn is_empty(&self) -> bool` | Liefert `true`, wenn keine verwaisten Einträge vorhanden waren oder bereinigt wurden. | `src/orphan.rs` |
| `pub fn load_sync(path: &std::path::Path) -> Self` | Lädt den Zustand synchron von der Festplatte oder liefert einen leeren Zustand bei Fehlern. | `src/orphan.rs` |
| `pub fn load_sync(path: &std::path::Path) -> Self` | Lädt den Zustand synchron von der Festplatte. | `src/orphan.rs` |
| `pub fn load_vfs(vfs: &dyn crate::vfs::CheckpointVfs, path: &std::path::Path) -> Self` | Lädt den Zustand über die Vfs-Schnittstelle. | `src/orphan.rs` |
| `pub fn monotonic_timestamp_ms() -> u64` | Liest den Millisekunden-Zeitstempel über die standardmäßige SystemClock. | `src/orphan.rs` |
| `pub fn monotonic_timestamp_ms_with(clock: &dyn Clock) -> u64` | Liest einen Millisekunden-Zeitstempel deterministisch über die übergebene Clock-Port-Instanz. # Parameter - `clock`: Referenz auf eine Impl… | `src/orphan.rs` |
| `pub fn new(file_path: impl Into<std::path::PathBuf>) -> Self` | Erzeugt eine neue Instanz von [`OrphanRegistry`]. | `src/orphan.rs` |
| `pub fn new(persist_path: impl Into<std::path::PathBuf>) -> Self` | Erzeugt eine neue instanzgebundene Registry für den angegebenen Dateipfad. | `src/orphan.rs` |
| `pub fn new_with_vfs( persist_path: impl Into<std::path::PathBuf>, vfs: Arc<dyn crate::vfs::CheckpointVfs>, ) -> Self` | Erzeugt eine neue instanzgebundene Registry über den VFS-Port. | `src/orphan.rs` |
| `pub fn orphaned_checkpoint_count() -> usize` | Liefert die Anzahl der aktuell registrierten verwaisten ("orphaned") Checkpoints. | `src/orphan.rs` |
| `pub fn pending_rollback_count() -> usize` | Rückwärtskompatibilität: Liefert die Anzahl ausstehender Rollback-Hintergrundaufgaben (immer 0). | `src/orphan.rs` |
| `pub fn persist_sync(&self) -> std::io::Result<()>` | Schreibt den Zustand synchron als formatierte JSON-Datei auf die Festplatte (ggf. über Vfs). | `src/orphan.rs` |
| `pub fn persist_sync(&self) -> std::io::Result<()>` | Persistiert den aktuellen Zustand synchron auf die Festplatte (bevorzugt über Vfs). | `src/orphan.rs` |
| `pub fn persist_vfs(&self, vfs: &dyn crate::vfs::CheckpointVfs) -> std::io::Result<()>` | Schreibt den Zustand über die Vfs-Schnittstelle crash-konsistent (`write_atomic`). | `src/orphan.rs` |
| `pub async fn recover_and_clean<S: crate::snapshot_source::SnapshotSource>( &self, storage: &S, ) -> Result<Vec<PinId>>` | Bereinigt alle verwaisten Pins durch Entpinnen in der Storage-Engine. # Ablauf von `recover_and_clean`: 1. Liste aller verwaisten Pins abfr… | `src/orphan.rs` |
| `pub fn register_checkpoint_sync(&self, cp: StateCheckpoint)` | Registriert einen verwaisten Checkpoint rein im Arbeitsspeicher ohne blockierendes Disk-I/O. # Crash-Safety & RAII Drop Semantik Wird synch… | `src/orphan.rs` |
| `pub fn register_orphan(&self, pin_id: PinId) -> std::io::Result<()>` | Registriert synchron einen verwaisten Pin. | `src/orphan.rs` |
| `pub fn register_orphan_sync(&self, orphan: PinnedSeqNoOrphan)` | Registriert einen verwaisten Sequenz-Pin rein im Arbeitsspeicher ohne blockierendes Disk-I/O. # Crash-Safety & RAII Drop Semantik Wird sync… | `src/orphan.rs` |
| `pub fn register_orphaned_checkpoint(cp: StateCheckpoint)` | — | `src/orphan.rs` |
| `pub fn register_pinned_seq_no_orphan(orphan: PinnedSeqNoOrphan)` | — | `src/orphan.rs` |
| `pub fn set_clock(&mut self, clock: Arc<dyn Clock>)` | Setzt die Clock-Implementierung nachträglich. | `src/orphan.rs` |
| `pub fn total_recovered(&self) -> usize` | Liefert die Gesamtzahl aller bereinigten Pins und Checkpoints. | `src/orphan.rs` |
| `pub fn vfs(&self) -> Option<&Arc<dyn crate::vfs::CheckpointVfs>>` | Liefert eine Referenz auf das injizierte VFS (falls vorhanden). | `src/orphan.rs` |
| `pub fn with_clock(mut self, clock: Arc<dyn Clock>) -> Self` | Konfiguriert die Registry mit einer spezifischen Clock-Implementierung. | `src/orphan.rs` |
| `pub fn with_vfs(mut self, vfs: Arc<dyn crate::vfs::CheckpointVfs>) -> Self` | Konfiguriert die Registry mit einer VFS-Implementierung. | `src/orphan.rs` |
| `pub async fn allocate_tx(&self) -> Result<TxId>` | Reserviert eine neue Transaktions-ID aus dem reservierten internen Bereich (`INTERNAL_BASE`). # Ablauf von `allocate_tx`: 1. Atomaren Zähle… | `src/store.rs` |
| `pub fn checkpoint_counter(&self) -> u64` | Liefert den Zählerstand für Checkpoint-Zeitstempel. | `src/store.rs` |
| `pub fn checkpoint_guard_skipped_rollback_count(&self) -> u64` | Alias für `skipped_rollback_count`. | `src/store.rs` |
| `pub fn clear_all_orphaned_checkpoints(&self)` | Löscht alle verwaisten Checkpoints. | `src/store.rs` |
| `pub fn clear_orphaned_checkpoint(&self, tx_id: TxId)` | Löscht einen verwaisten Checkpoint anhand seiner Transaktions-ID. | `src/store.rs` |
| `pub fn clock(&self) -> &Arc<dyn Clock>` | Liefert den aktuellen Clock-Port. | `src/store.rs` |
| `pub async fn close(&self) -> Result<()>` | Alias für `shutdown()`. Führt einen kontrollierten Flush beim Schließen aus. | `src/store.rs` |
| `pub async fn create_checkpoint( &self, name: &str, collection_id: &str, seq_no: u64, tx_id: TxId, metadata: serde_json::Value, ) -> Result<CheckpointMeta>` | Creates a new persistent checkpoint. # Ablauf von `create_checkpoint`: 1. Bezeichner `name` und `collection_id` validieren. 2. `write_lock`… | `src/store.rs` |
| `pub fn create_guard(&self, tx_id: TxId) -> Result<CheckpointGuard<S>>` | Creates an ephemeral transactional checkpoint RAII guard. If the returned guard is dropped without calling `.commit()`, the underlying stor… | `src/store.rs` |
| `pub async fn create_hardlink_clone( &self, seq_no: u64, source_dir: &Path, target_dir: &Path, snapshot_pinner: &(dyn SnapshotPinner + '_), ) -> Result<HardlinkCloneResult>` | Creates a physical hardlink clone of all SSTable files visible for `seq_no` into `target_dir`. Delegates to [`DefaultHardlinkCloner`] while… | `src/store.rs` |
| `pub async fn drop_checkpoint(&self, name: &str) -> Result<()>` | Deletes a persistent checkpoint by name. # Ablauf von `drop_checkpoint`: 1. Bezeichner validieren und `write_lock` holen. 2. Checkpoint lad… | `src/store.rs` |
| `pub async fn flush_orphan_registry(&self) -> Result<()>` | Flusht die Orphan-Registry asynchron auf Platte. | `src/store.rs` |
| `pub async fn get_checkpoint(&self, name: &str) -> Result<Option<CheckpointMeta>>` | Ruft einen Checkpoint nach Namen ab. | `src/store.rs` |
| `pub fn get_orphaned_checkpoints(&self) -> Vec<StateCheckpoint>` | Liefert alle verwaisten Checkpoints. | `src/store.rs` |
| `pub async fn list_checkpoints(&self) -> Result<Vec<CheckpointMeta>>` | Listet alle persistent gespeicherten Checkpoints im konfigurierten Namespace auf. # Ablauf von `list_checkpoints`: 1. Präfix-Scan auf der S… | `src/store.rs` |
| `pub fn monotonic_timestamp_ms(&self) -> u64` | Liefert einen monoton steigenden Zeitstempel in Millisekunden. | `src/store.rs` |
| `pub fn new(storage: Arc<S>, namespace: impl Into<String>) -> Result<Self>` | Synchroner Konstruktor für Umgebungen ohne asynchronen Tokio-Einstiegspunkt. | `src/store.rs` |
| `pub fn new_with_orphan_registry( storage: Arc<S>, namespace: impl Into<String>, orphan_registry: Arc<InstanceOrphanRegistry>, ) -> Result<Self>` | Synchroner Konstruktor mit injizierter Orphan Registry. | `src/store.rs` |
| `pub async fn open(storage: Arc<S>, namespace: impl Into<String>) -> Result<Self>` | Öffnet einen PersistentCheckpointStore asynchron mit Rekonstruktion und Monotonie-Garantie. | `src/store.rs` |
| `pub async fn open_and_recover( storage: Arc<S>, namespace: impl Into<String>, ) -> Result<(Self, RecoveryReport)>` | Öffnet einen PersistentCheckpointStore und führt eine vollständige Orphan-Recovery für Pins und Checkpoints durch. # Ablauf von `open_and_r… | `src/store.rs` |
| `pub async fn open_and_recover_with_orphan_registry( storage: Arc<S>, namespace: impl Into<String>, orphan_registry: Arc<InstanceOrphanRegistry>, ) -> Result<(Self, RecoveryReport)>` | Öffnet einen PersistentCheckpointStore mit einer spezifischen instanzgebundenen Orphan Registry und führt Orphan-Recovery durch. | `src/store.rs` |
| `pub async fn open_with_orphan_registry( storage: Arc<S>, namespace: impl Into<String>, orphan_registry: Arc<InstanceOrphanRegistry>, ) -> Result<Self>` | Öffnet einen PersistentCheckpointStore mit einer spezifischen instanzgebundenen Orphan Registry. | `src/store.rs` |
| `pub fn orphan_registry(&self) -> &Arc<InstanceOrphanRegistry>` | Liefert eine Referenz auf die Orphan-Registry. | `src/store.rs` |
| `pub async fn recover_orphaned_checkpoints(&self) -> Result<Vec<TxId>>` | Recovers all registered/persisted orphaned checkpoints during controlled startup or recovery. Checks the serialization barrier (`last_tx <=… | `src/store.rs` |
| `pub async fn recover_orphaned_pins(&self) -> Result<Vec<PinId>>` | Recovers all registered/persisted orphaned sequence pins (ADR-052). # Ablauf von `recover_orphaned_pins`: 1. Liste verwaister Pins aus der… | `src/store.rs` |
| `pub fn register_orphaned_checkpoint(&self, cp: StateCheckpoint)` | Registriert einen verwaisten Checkpoint. | `src/store.rs` |
| `pub fn register_pinned_seq_no_orphan(&self, orphan: PinnedSeqNoOrphan)` | Registriert einen verwaisten Sequenz-Pin. | `src/store.rs` |
| `pub async fn restore_checkpoint(&self, name: &str) -> Result<CheckpointMeta>` | Restores the system to a specific checkpoint by name. This will rollback the underlying storage to the transaction ID of the checkpoint. #… | `src/store.rs` |
| `pub fn set_metrics_sink(&mut self, sink: Arc<dyn MetricsSink>)` | Setzt die Metrik-Senke. | `src/store.rs` |
| `pub async fn shutdown(&self) -> Result<()>` | Führt einen expliziten Flush der Orphan-Registry beim Herunterfahren durch. | `src/store.rs` |
| `pub fn skipped_rollback_count(&self) -> u64` | Liefert die Anzahl übersprungener Rollbacks. | `src/store.rs` |
| `pub fn storage(&self) -> &Arc<S>` | Liefert eine Referenz auf die Orphan-Registry. Liefert eine Referenz auf die zugrundeliegende Speicher-Engine. | `src/store.rs` |
| `pub fn with_clock(mut self, clock: Arc<dyn Clock>) -> Self` | Setzt den Zeitgeber für den `PersistentCheckpointStore`. # Composition Root Der Composition Root setzt per `with_clock` eine injizierte Uhr… | `src/store.rs` |
| `pub fn with_metrics_sink(mut self, sink: Arc<dyn MetricsSink>) -> Self` | Konfiguriert den Store mit einer Metrik-Senke. | `src/store.rs` |


---

## `acta-mcp`

| Attribut | Wert |
|---|---|
| **Ring** | 3 — Ebene E3 — Anwendungen, Bindings & Tooling |
| **Status** | Fertig |
| **Zweck** | Stdio-JSON-RPC MCP-Server-Protokoll-Adapter. |
| **Unsafe-Code** | Nein (`#![forbid(unsafe_code)]`) |
| **Interne Abhängigkeiten** | `acta-crypto`, `acta-ports`, `acta-rank`, `acta-types`, `acta-wire` |
| **Externe Abhängigkeiten (Auswahl)** | `futures-util`, `parking_lot`, `rand`, `scc`, `serde`, `serde_json`, `thiserror`, `tokio`, `tokio-util`, `tracing`, `tracing-subscriber`, `unicode-normalization`, `zeroize` |

### Modul- und Re-Export-Oberfläche (lib.rs)

```rust
pub mod config;
pub mod io;
pub mod protocol;
pub mod validation;
pub use config::*;
pub use io::{read_line_bounded, MAX_RPC_BYTES, MAX_SEARCH_QUERY_BYTES};
pub use validation::{is_write_allowed_by_env, validate_collection_name};
```

### Feature-Flags

| Flag | Aktiviert |
|---|---|
| `default` | `[]` |
| `test-utils` | `[]` |

### Öffentliche Datentypen — Structs (2)

| Struct | Felder / Form | Beschreibung | Datei |
|---|---|---|---|
| `EmbeddingConfig` | `{ /// Provider-Kennung. Leer bedeutet: nicht konfiguriert. pub provider: String, /// Optionale erwartete Dimension zur Index-Alignment-Prüfung. pub expected_dim:… }` | Embedding-Provider-Konfiguration. | `src/config.rs` |
| `MockEmbedder` | `{ dimension: usize, all_zero: bool, }` | Fallback Mock Embedder (nur Test- und Dev-Builds). | `src/config.rs` |

### Öffentliche Datentypen — Enums (1)

| Enum | Varianten | Beschreibung | Datei |
|---|---|---|---|
| `McpError` | `#[error("{0}")] ParseError(String), #[error("{0}")] InvalidRequest(String), #[error("{0}")] MethodNotFound(String), #[error("{0}")] InvalidParams(String), #[error("{0}")] InternalError(String),` | MCP Error representation matching standard JSON-RPC 2.0 error codes. | `src/protocol.rs` |

### Konstanten (4)

| Konstante | Typ | Beschreibung |
|---|---|---|
| `MAX_BATCH_SIZE` | `usize` | Maximale Anzahl von Elementen in einem JSON-RPC Batch-Request (Schutz gegen Batch-DoS). |
| `MAX_JSON_DEPTH` | `usize` | Maximale Rekursionstiefe für eingehende JSON-RPC Payloads (Schutz gegen Stack-Overflow). |
| `MAX_RPC_BYTES` | `usize` | Maximum allowed single JSON-RPC message size via stdio (4 MB). |
| `MAX_SEARCH_QUERY_BYTES` | `usize` | Maximum allowed search query length in bytes (64 KB). |

### Öffentliche Funktionen (21)

| Signatur | Beschreibung | Datei |
|---|---|---|
| `pub fn available_compiled_providers() -> Vec<&'static str>` | Liste der einkompilierten Provider-Kennungen. Ohne Backend-Crates nur `mock` (Test/Dev). | `src/config.rs` |
| `pub fn build_provider(&self) -> Result<Arc<dyn EmbeddingProvider>, ActaError>` | Instanziiert den konfigurierten `EmbeddingProvider` und führt die Startup-Probe aus. | `src/config.rs` |
| `pub fn create_embedding_provider( provider_type: Option<&str>, ) -> Result<Arc<dyn EmbeddingProvider>, ActaError>` | Erzeugt einen `EmbeddingProvider` anhand der Kennung. Unbekannte oder nicht einkompilierte Provider schlagen fehl. | `src/config.rs` |
| `pub fn from_env() -> Self` | Lädt die Konfiguration aus Umgebungsvariablen. | `src/config.rs` |
| `pub fn new(dimension: usize) -> Self` | — | `src/config.rs` |
| `pub fn provider_opt(&self) -> Option<&str>` | — | `src/config.rs` |
| `pub fn verify_embedding_provider( provider: &dyn EmbeddingProvider, expected_dim: Option<usize>, ) -> Result<(), ActaError>` | Startup embedding self-test (probe) verifying provider functionality. | `src/config.rs` |
| `pub fn with_expected_dim(mut self, dim: usize) -> Self` | — | `src/config.rs` |
| `pub fn with_zero_vectors(dimension: usize) -> Self` | — | `src/config.rs` |
| `pub async fn read_line_bounded<R: tokio::io::AsyncBufRead + Unpin>( reader: &mut R, buf: &mut String, max_bytes: usize, ) -> std::io::Result<usize>` | Reads a single line from an async reader into `buf` up to `max_bytes`. If the line exceeds `max_bytes`, consumes and discards the remainder… | `src/io.rs` |
| `pub fn code(&self) -> i32` | — | `src/protocol.rs` |
| `pub fn internal_error(msg: impl Into<String>) -> Self` | — | `src/protocol.rs` |
| `pub fn invalid_params(msg: impl Into<String>) -> Self` | — | `src/protocol.rs` |
| `pub fn invalid_request(msg: impl Into<String>) -> Self` | — | `src/protocol.rs` |
| `pub fn measure_json_depth(val: &Value) -> usize` | Ermittelt die Schachtelungstiefe eines JSON-Value. | `src/protocol.rs` |
| `pub fn method_not_found(msg: impl Into<String>) -> Self` | — | `src/protocol.rs` |
| `pub fn parse_error(msg: impl Into<String>) -> Self` | — | `src/protocol.rs` |
| `pub fn response_from_error(id: Option<Value>, err: McpError) -> JsonRpcResponse` | Helper function to convert an `McpError` directly into a `JsonRpcResponse`. | `src/protocol.rs` |
| `pub fn validate_json_depth(val: &Value, max_depth: usize) -> Result<(), McpError>` | Validiert, dass ein JSON-Value die maximale Tiefe nicht überschreitet. | `src/protocol.rs` |
| `pub fn is_write_allowed_by_env() -> bool` | Checks whether database write access is explicitly enabled via environment variable `ACTA_MCP_ALLOW_WRITE`. | `src/validation.rs` |
| `pub fn validate_collection_name(name: &str) -> Result<(), McpError>` | MCP-Server mit stdio-Transport (JSON-RPC 2.0). stdout ist dem Protokoll vorbehalten — Logs gehen ausschließlich nach stderr. | `src/validation.rs` |


---

## `acta-durable-fs`

| Attribut | Wert |
|---|---|
| **Ring** | 0 — Ebene E0 — Autarke Bausteine & Fundamente |
| **Status** | Fertig |
| **Zweck** | Durable I/O, atomare Dateiverschiebungen und fsync-Garantien. |
| **Unsafe-Code** | Nein (`#![forbid(unsafe_code)]`) |
| **Interne Abhängigkeiten** | — |
| **Externe Abhängigkeiten (Auswahl)** | `crc32fast`, `serde`, `thiserror`, `tracing` |

### Modul- und Re-Export-Oberfläche (lib.rs)

```rust
pub mod error;
pub mod fault_injector;
pub mod health;
pub mod journal;
pub mod platform;
pub mod scrub;
pub mod vfs;
pub use error::{DurableFsError, Error, Result};
pub use fault_injector::{FaultInjector, NoFault};
pub use health::{AtomicHealthState, HealthEvent, HealthState};
pub use journal::{ AppendOnlyJournal, JournalSyncPolicy, PendingDeleteJournal, RecordRecoveryPolicy, };
pub use platform::FlushKind;
pub use scrub::{ scrub_file_with_rate_limit, scrub_if_safe, scrub_if_safe_with_injector, ScrubOutcome, ScrubPolicy, ScrubReceipt, ScrubRefusal, };
pub use vfs::{RealFs, StdVfs, Vfs};
```

### Feature-Flags

| Flag | Aktiviert |
|---|---|
| `default` | `["std", "crc"]` |
| `std` | `[]` |
| `crc` | `["dep:crc32fast"]` |
| `serde` | `["dep:serde"]` |
| `tracing` | `["dep:tracing"]` |
| `inspect` | `[]` |

### Schnittstellen — Public Traits (2)

**`FaultInjector`** — *Schnittstelle zur Injektion von I/O-Fehlern in Persistenzpfade.* (`src/fault_injector.rs`)

```rust
pub trait FaultInjector : Send + Sync {
/// Hook vor oder während eines Schreibvorgangs.
    fn on_write(&self, _path: Option<&Path>, _bytes: usize) -> io::Result<()> {
        Ok(())
    }

    /// Hook vor dem Hardware-Flush (`fsync`) einer Datei.
    fn on_fsync(&self, _path: Option<&Path>) -> io::Result<()> {
        Ok(())
    }

    /// Hook vor einer atomaren Umbenennung (`rename`).
    fn on_rename(&self, _from: &Path, _to: &Path) -> io::Result<()> {
        Ok(())
    }

    /// Hook vor dem Hardware-Flush (`fsync`) eines Verzeichnisses.
    fn on_dir_fsync(&self, _dir: &Path) -> io::Result<()> {
        Ok(())
    }
}
```

**`Vfs`** — *Synchroner Trait für virtuelle Dateisystemoperationen (Klasse A).* (`src/vfs.rs`)

```rust
pub trait Vfs : Send + Sync {
/// Liest den gesamten Inhalt einer Datei.
    fn read(&self, path: &Path) -> Result<Vec<u8>>;

    /// Schreibt Daten atomar und crash-sicher in eine Datei (`INV-VFS-ATOMIC-1`).
    fn write_atomic(&self, path: &Path, data: &[u8]) -> Result<()>;

    /// Hängt Daten an eine Datei an und flusht synchron.
    fn append(&self, path: &Path, data: &[u8]) -> Result<()>;

    /// Synchronisiert eine Datei auf stabilen Speicher.
    fn sync_file(&self, path: &Path) -> Result<()>;

    /// Synchronisiert die Metadaten eines Verzeichnisses auf stabilen Speicher.
    fn sync_dir(&self, path: &Path) -> Result<()>;

    /// Entfernt eine Datei dauerhaft und flusht das übergeordnete Verzeichnis.
    fn remove(&self, path: &Path) -> Result<()>;

    /// Benennt eine Datei atomar um und flusht das übergeordnete Verzeichnis.
    fn rename(&self, from: &Path, to: &Path) -> Result<()>;

    /// Listet alle Dateinamen in einem Verzeichnis lexikografisch sortiert auf.
    fn list(&self, path: &Path) -> Result<Vec<String>>;

    /// Prüft, ob ein Pfad existiert.
    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }

    /// Gibt die Größe einer Datei in Bytes zurück.
    fn len(&self, path: &Path) -> Result<u64> {
        fs::metadata(path)
            .map(|m| m.len())
            .map_err(|e| map_io_err(e, path))
    }

    /// Erstellt ein Verzeichnis inklusive aller Zwischenverzeichnisse.
    fn create_dir_all(&self, path: &Path) -> Result<()> {
        fs::create_dir_all(path).map_err(DurableFsError::Io)
    }
}
```

### Öffentliche Datentypen — Structs (12)

| Struct | Felder / Form | Beschreibung | Datei |
|---|---|---|---|
| `AppendOnlyJournal` | `{ file: File, path: PathBuf, next_seq: u64, policy: JournalSyncPolicy, unflushed_bytes: usize, health: Arc<AtomicHealthState>, }` | Universelles Append-only Journal mit Prüfsummen und Sequenznummern. | `src/journal.rs` |
| `AtomicHealthState` | `{ state: AtomicU8, }` | Threadsicherer, atomarer Wrapper für den aktuellen [`HealthState`]. | `src/health.rs` |
| `DirLock` | `{ lock_path: PathBuf, _file: Option<File>, mode: LockMode, }` | RAII-Guard für eine Verzeichnissperre (`.dirlock`). | `src/lib.rs` |
| `DurableHandle` | `{ file: Option<File>, is_poisoned: bool, injector: F, path: Option<PathBuf>, }` | Ein Wrapper um [`File`], der das Handle bei einem `fsync`- oder Schreibfehler dauerhaft vergiftet (`INV-FSYNC-1`). | `src/lib.rs` |
| `FaultInjectingVfs` | `{ inner: V, fail_next_write: AtomicBool, fail_next_fsync: AtomicBool, fail_next_rename: AtomicBool, fail_next_dir_fsync: AtomicBool, synced_snapshots: Mutex<Hash… }` | VFS-Dekorator zur präzisen Fehlerinjektion und Crash-Punkt-Simulation. | `src/fault_injector.rs` |
| `JournalRecord` | `{ /// Monoton steigende Sequenznummer des Records. pub seq_no: u64, /// Nutzdaten des Records. pub payload: Vec<u8>, }` | Ein gelesener Journal-Eintrag. | `src/journal.rs` |
| `NoFault` | `(tuple/unit)` | Null-Kosten-Standardinjektor für Produktivumgebungen. | `src/fault_injector.rs` |
| `PendingDeleteJournal` | `{ journal_path: PathBuf, entries: Vec<PathBuf>, health: Arc<AtomicHealthState>, }` | Journal zur dauerhaften Aufzeichnung von Entfernungsaufträgen über Prozessabstürze hinweg. | `src/journal.rs` |
| `PoisonedHandle` | `(tuple/unit)` | Markierungstyp zur Kennzeichnung eines durch `fsync`- oder Schreibfehler vergifteten Handles. | `src/lib.rs` |
| `ScrubProgress` | `{ /// Bisher verarbeitete Bytes. pub bytes_processed: u64, /// Gesamtgröße der zu prüfenden Datei / des Segments in Bytes. pub total_bytes: u64, }` | Fortschrittsbericht beim aufrufergesteuerten Scrubbing. | `src/scrub.rs` |
| `ScrubReceipt` | `{ /// Gesamtanzahl der vor dem Löschen mit Nullen überschriebenen Bytes. pub bytes_scrubbed: u64, }` | Quittung nach erfolgreichem Überschreiben und dauerhaftem Entfernen einer Datei. | `src/scrub.rs` |
| `StdVfs` | `(tuple/unit)` | Standard-Implementierung von [`Vfs`] auf Basis von `std::fs`. | `src/vfs.rs` |

### Öffentliche Datentypen — Enums (12)

| Enum | Varianten | Beschreibung | Datei |
|---|---|---|---|
| `DurableFsError` | `/// Dateisystem- oder I/O-Fehler des zugrundeliegenden Betriebssystems. #[error("I/O error: {0}")] Io(#[from] std::io::Error), /// Pfad wurde nicht gefunden. #[error("path not found: {path}")] NotFou…` | Primärer Fehlertyp für alle dauerhaften Dateisystemoperationen (`acta-durable-fs`). | `src/error.rs` |
| `FlushKind` | `/// Flusht nur die Dateinutzdaten auf stabile Speichermedien (`fdatasync`). DataOnly, /// Flusht Dateinutzdaten und Metadaten auf stabile Speichermedien (`fsync`). DataAndMetadata,` | Spezifikation des Flush-Umfangs für geöffnete Dateihandles. | `src/platform.rs` |
| `FsClass` | `/// Lokales Dateisystem mit unterstützten POSIX/Win32-Durability-Garantien. LocalSupported, /// Netzlaufwerk (NFS, SMB, CIFS). Network, /// Benutzerdefiniertes Dateisystem im Userspace (FUSE). Fuse,…` | Dateisystemklassifikation für Persistenz- und Locking-Garantien (§23.4). | `src/lib.rs` |
| `HealthEvent` | `/// Fataler Ausfall während eines synchronen Festplatten-Flushes (`fsync`). FsyncFailure, /// Speichermedium meldet fehlenden Speicherplatz (`ENOSPC`). Enospc, /// Nicht-fataler Lesefehler oder Warnu…` | Ereignisse und Auslöser für Statusübergänge im Speichersystem. | `src/health.rs` |
| `HealthState` | `/// Normaler Betriebszustand mit vollständigen Lese- und Schreibfähigkeiten. #[default] Healthy = 0, /// Nicht-fatale Lesefehler oder Prüfsummenabweichungen aufgetreten; Schreiben erlaubt, Lesen eing…` | Einstufung des Systemzustands für das Speichersubstrat (§18.4). | `src/health.rs` |
| `InjectedFault` | `/// Speichermedium voll (`ENOSPC`). DiskFull, /// Hardware- oder I/O-Fehler (`EIO`). IoError, /// Vorzeitiger Abbruch bei fsync. FsyncFailure, /// Fehler beim atomaren Umbenennen. RenameFailure,` | Skriptbarer Fehlerzustand für Tests und Chaos-Simulation. | `src/fault_injector.rs` |
| `JournalSyncPolicy` | `/// Jeder Record wird sofort synchron per `fsync` persistiert. Always, /// Batch-Synchronisation nach maximaler Bytezahl oder Zeitintervall. GroupCommit { /// Maximale ungeflushte Bytezahl vor einem…` | Synchronisations-Politik für das Append-only-Journal. | `src/journal.rs` |
| `LockMode` | `/// Standard-Sperre: Erfordert ein unterstütztes lokales Dateisystem. #[default] Normal, /// Reiner In-Memory-Modus: Umgeht Dateisystemprüfungen. MemoryOnly,` | Sperrmodus für Verzeichnissperren gemäß Spezifikation §7.3.1. | `src/lib.rs` |
| `RecordRecoveryPolicy` | `/// Bricht bei jedem Prüfsummen- oder Framing-Fehler sofort ab (Fail-Closed). #[default] FailClosed, /// Schneidet das Journal deterministisch vor dem ersten ungültigen Record ab (Truncate). Truncate…` | Recovery-Verhalten bei beschädigtem Journal-Ende (Torn-Write / Bit-Rot / Zero-Fill). | `src/journal.rs` |
| `ScrubOutcome` | `/// Datei wurde erfolgreich mit Nullen überschrieben, gegengelesen und dauerhaft entfernt. Scrubbed(ScrubReceipt), /// Datei wurde dauerhaft gelöscht, ohne zuvor überschrieben zu werden. RemovedUnscr…` | Ergebnis einer Bereinigungsoperation mittels [`scrub_if_safe`]. | `src/scrub.rs` |
| `ScrubPolicy` | `/// Überschreiben deaktiviert: Datei wird direkt ohne vorheriges Nullen entfernt. Disabled, /// Datei vor dem Entfernen vollständig mit binären Nullen (`0u8`) überschreiben. OverwriteZeros,` | Richtlinie zur Steuerung des Überschreibverhaltens vor dem Löschen. | `src/scrub.rs` |
| `ScrubRefusal` | `/// Scrubbing wurde per Richtlinie ([`ScrubPolicy::Disabled`]) deaktiviert. Disabled, /// Datei besitzt mehrere Hardlinks (`nlink > 1`). MultipleHardlinks, /// Datei liegt auf einem Copy-on-Write-Dat…` | Gründe für eine Verweigerung des Nullen-Überschreibens aus Sicherheits- oder Plattformgründen. | `src/scrub.rs` |

### Typ-Aliase (3)

| Alias | Definition | Beschreibung |
|---|---|---|
| `Error` | `type Error= DurableFsError` | Kanonischer Alias `Error` gemäß Klasse-A-Vertrag (B4). |
| `RealFs` | `type RealFs= StdVfs` | Alias für bestehende Workspace-Kompatibilität (`RealFs`). |
| `Result` | `type Result<T> = std::result::Result<T, DurableFsError>` | Einheitlicher Result-Typ für `acta-durable-fs`. |

### Konstanten (2)

| Konstante | Typ | Beschreibung |
|---|---|---|
| `JOURNAL_MAGIC` | `[u8` | Magic Bytes für den Header eines Journal-Records: `[0x43, 0x54, 0x58, 0x4A]` ("CTXJ"). |
| `RECORD_HEADER_SIZE` | `usize` | Header-Größe für jeden Journal-Record: 4 Bytes Magic + 8 Bytes SeqNo + 4 Bytes Payload-Länge + 4 Bytes CRC32 = 20 Bytes. |

### Öffentliche Funktionen (55)

| Signatur | Beschreibung | Datei |
|---|---|---|
| `pub fn kind(&self) -> std::io::ErrorKind` | Gibt den entsprechenden [`std::io::ErrorKind`] für Kompatibilität mit IO-Operationen zurück. | `src/error.rs` |
| `pub fn inject_dir_fsync_failure(&self)` | Konfiguriert, dass der nächste Verzeichnis-Sync fehlschlägt. | `src/fault_injector.rs` |
| `pub fn inject_fsync_failure(&self)` | Konfiguriert, dass der nächste `sync_file`-Aufruf mit EIO fehlschlägt. | `src/fault_injector.rs` |
| `pub fn inject_rename_failure(&self)` | Konfiguriert, dass das nächste `rename` fehlschlägt. | `src/fault_injector.rs` |
| `pub fn inject_write_disk_full(&self)` | Konfiguriert, dass der nächste Schreibaufruf mit `ENOSPC` fehlschlägt. | `src/fault_injector.rs` |
| `pub fn new(inner: V) -> Self` | Kapselt ein bestehendes [`Vfs`] in einen Injektor-Dekorator. | `src/fault_injector.rs` |
| `pub fn simulate_crash(&self) -> Result<()>` | Simuliert einen plötzlichen Prozessabsturz (Crash-Punkt-Simulation): Alle noch nicht synchronisierten Datenänderungen gehen unwiderruflich… | `src/fault_injector.rs` |
| `pub fn allows_writes(self) -> bool` | Gibt an, ob im aktuellen Zustand mutierende Schreiboperationen zulässig sind. | `src/health.rs` |
| `pub fn is_poisoned(self) -> bool` | Prüft, ob das Speichersubstrat dauerhaft vergiftet (`Poisoned`) ist. | `src/health.rs` |
| `pub fn load(&self) -> HealthState` | Liest den aktuellen [`HealthState`] mit `Acquire`-Speicherordnung aus. | `src/health.rs` |
| `pub fn new() -> Self` | Initialisiert eine neue atomare Zustandsverwaltung im Startzustand `Healthy`. | `src/health.rs` |
| `pub fn record_event(&self, event: HealthEvent) -> HealthState` | Führt einen atomaren Zustandsübergang anhand eines [`HealthEvent`] aus. | `src/health.rs` |
| `pub fn transition(self, event: HealthEvent) -> Self` | Berechnet den Folgezustand basierend auf einem eingehenden [`HealthEvent`]. | `src/health.rs` |
| `pub fn append(&mut self, payload: &[u8]) -> Result<u64>` | Hängt einen neuen Payload-Record an das Journal an. | `src/journal.rs` |
| `pub fn calculate_crc32(data: &[u8]) -> u32` | Hilfsfunktion zur Berechnung einer CRC32-Prüfsumme. | `src/journal.rs` |
| `pub fn flush(&mut self) -> Result<()>` | Flusht ausstehende Journal-Schreibvorgänge synchron auf stabilen Speicher (`INV-FSYNC-1`). | `src/journal.rs` |
| `pub fn health_state(&self) -> HealthState` | Gibt den aktuellen Gesundheitszustand zurück. | `src/journal.rs` |
| `pub fn health_state(&self) -> HealthState` | Fragt den aktuellen Gesundheitsstatus ([`HealthState`]) des Journals ab. | `src/journal.rs` |
| `pub fn open(path: impl Into<PathBuf>) -> Result<Self>` | Öffnet oder erstellt ein Journal mit Standard-Sync-Politik ([`JournalSyncPolicy::Always`]). | `src/journal.rs` |
| `pub fn open(journal_path: impl Into<PathBuf>) -> Result<Self>` | Öffnet oder erstellt ein `PendingDeleteJournal` am angegebenen Pfad mit neuem Health-State. | `src/journal.rs` |
| `pub fn open_with_health( journal_path: impl Into<PathBuf>, health: Arc<AtomicHealthState>, ) -> Result<Self>` | Öffnet oder erstellt ein `PendingDeleteJournal` mit einer geteilten [`AtomicHealthState`]-Instanz. | `src/journal.rs` |
| `pub fn open_with_policy( path: impl Into<PathBuf>, policy: JournalSyncPolicy, health: Arc<AtomicHealthState>, ) -> Result<Self>` | Öffnet oder erstellt ein Journal mit benutzerdefinierter Sync-Politik und Health-State. | `src/journal.rs` |
| `pub fn open_without_recovery( path: impl Into<PathBuf>, policy: JournalSyncPolicy, health: Arc<AtomicHealthState>, ) -> Result<Self>` | Öffnet ein Journal ohne automatische Initial-Recovery (nützlich für manuelle Recovery). | `src/journal.rs` |
| `pub fn path(&self) -> &std::path::Path` | Gibt den Pfad des Journals zurück. | `src/journal.rs` |
| `pub fn pending(&self) -> &[PathBuf]` | Liefert eine unveränderliche Sicht auf alle aktuell ausstehenden Pfade. | `src/journal.rs` |
| `pub fn process<F>(&mut self, mut remove_fn: F) -> Result<usize> where F: FnMut(&std::path::Path) -> Result<()>,` | Verarbeitet alle ausstehenden Einträge unter Verwendung der Entfernungsfunktion `remove_fn`. | `src/journal.rs` |
| `pub fn record(&mut self, path: &std::path::Path) -> Result<()>` | Fügt einen neuen Dateipfad dauerhaft zum Journal hinzu. | `src/journal.rs` |
| `pub fn recover(&mut self, recovery_policy: RecordRecoveryPolicy) -> Result<Vec<JournalRecord>>` | Führt die Recovery des Journals durch und liest gültige Records ein. | `src/journal.rs` |
| `pub fn acquire(dir: &Path, mode: LockMode) -> Result<Self>` | Erwirbt eine Sperre auf `dir` im angegebenen [`LockMode`]. | `src/lib.rs` |
| `pub fn atomic_replace(path: &Path, bytes: &[u8]) -> Result<()>` | Ersetzt den Inhalt der Datei unter `path` atomar mit `bytes` unter Verwendung des Standardinjektors [`NoFault`]. | `src/lib.rs` |
| `pub fn atomic_replace_with_injector<F: FaultInjector>( path: &Path, bytes: &[u8], injector: &F, ) -> Result<()>` | Ersetzt den Inhalt der Datei unter `path` atomar mit `bytes` unter Verwendung eines [`FaultInjector`] (`INV-VFS-ATOMIC-1`). | `src/lib.rs` |
| `pub fn durable_remove(path: &Path) -> Result<()>` | Entfernt eine Datei dauerhaft und flusht das Elternverzeichnis mit Standardinjektor [`NoFault`]. | `src/lib.rs` |
| `pub fn durable_remove_with_injector<F: FaultInjector>(path: &Path, injector: &F) -> Result<()>` | Entfernt eine Datei dauerhaft und flusht das Elternverzeichnis mit benutzerdefiniertem [`FaultInjector`]. | `src/lib.rs` |
| `pub fn flush(&mut self, kind: FlushKind) -> Result<()>` | Flusht Dateiinhalte synchron auf die Festplatte und führt Fehlerinjektions-Hooks aus. | `src/lib.rs` |
| `pub fn flush_dir(dir: &Path) -> Result<()>` | Flusht Verzeichniseinträge auf stabile Speichermedien. | `src/lib.rs` |
| `pub fn flush_file(file: &File, kind: FlushKind) -> Result<()>` | Flusht eine geöffnete Datei auf die Festplatte. | `src/lib.rs` |
| `pub fn fs_class(path: &Path) -> Result<FsClass>` | Ermittelt die Dateisystemklasse für den angegebenen Pfad (§23.4). | `src/lib.rs` |
| `pub fn get_mut(&mut self) -> Option<&mut File>` | Liefert eine veränderliche Referenz auf die zugrundeliegende Datei, oder `None`, falls das Handle vergiftet ist. | `src/lib.rs` |
| `pub fn get_ref(&self) -> Option<&File>` | Liefert eine unveränderliche Referenz auf die zugrundeliegende Datei, oder `None`, falls das Handle vergiftet ist. | `src/lib.rs` |
| `pub fn is_poisoned(&self) -> bool` | Prüft, ob dieses Handle aufgrund eines vorangegangenen Flush-Fehlers vergiftet ist (`INV-FSYNC-1`). | `src/lib.rs` |
| `pub fn mode(&self) -> LockMode` | Liefert den aktiven [`LockMode`] dieser Verzeichnissperre. | `src/lib.rs` |
| `pub fn new(file: File) -> Self` | Kapselt eine geöffnete [`File`] in ein neues [`DurableHandle`] mit standardmäßigem [`NoFault`]. | `src/lib.rs` |
| `pub fn new_with_injector(file: File, injector: F) -> Self` | Kapselt eine geöffnete [`File`] mit einem benutzerdefinierten [`FaultInjector`]. | `src/lib.rs` |
| `pub fn new_with_path_and_injector(file: File, path: PathBuf, injector: F) -> Self` | Kapselt eine geöffnete [`File`] mit zugehörigem Pfad und benutzerdefiniertem [`FaultInjector`]. | `src/lib.rs` |
| `pub fn poison(&mut self)` | Vergiftet dieses Handle explizit manuell. | `src/lib.rs` |
| `pub fn rename_durable(from: &Path, to: &Path) -> Result<()>` | Benennt eine Datei atomar um und flusht das übergeordnete Verzeichnis. | `src/lib.rs` |
| `pub fn scrub_and_remove(path: &Path) -> Result<ScrubReceipt>` | Überschreibt eine Datei vollständig mit Nullen und entfernt sie dauerhaft (mit [`NoFault`]). | `src/lib.rs` |
| `pub fn scrub_and_remove_with_injector<F: FaultInjector>( path: &Path, injector: &F, ) -> Result<ScrubReceipt>` | Überschreibt eine Datei mit Nullen und entfernt sie dauerhaft unter Verwendung eines [`FaultInjector`]. | `src/lib.rs` |
| `pub fn sync_dir(dir: &Path) -> Result<()>` | Synchronisiert die Verzeichnismetadaten eines Ordners mit Standard-Injektor [`NoFault`]. | `src/lib.rs` |
| `pub fn sync_dir_with_injector<F: FaultInjector>(dir: &Path, injector: &F) -> Result<()>` | Synchronisiert die Verzeichnismetadaten eines Ordners mit benutzerdefiniertem [`FaultInjector`]. | `src/lib.rs` |
| `pub fn sync_dir_path(dir: &Path) -> io::Result<()>` | Flusht Verzeichniseinträge synchron auf stabile Speichermedien. Unter Unix wird das Verzeichnis als Datei geöffnet und `sync_all` ausgeführ… | `src/platform.rs` |
| `pub fn sync_file(file: &File, kind: FlushKind) -> io::Result<()>` | Flusht eine geöffnete Datei synchron und crash-sicher auf das Speichermedium. # Errors Liefert einen [`io::Error`], falls der Betriebssyste… | `src/platform.rs` |
| `pub fn scrub_file_with_rate_limit<F>( path: &Path, bytes_per_sec: u64, mut on_progress: F, ) -> Result<u64> where F: FnMut(ScrubProgress),` | Prüft eine Datei segmentweise auf Prüfsummenintegrität mit konfigurierbarer Ratenbegrenzung. Wird vollständig vom aufrufenden Thread getrie… | `src/scrub.rs` |
| `pub fn scrub_if_safe( path: &Path, policy: ScrubPolicy, reader_alive: bool, not_obsolete: bool, ) -> Result<ScrubOutcome>` | Bereinigt und löscht eine Datei sicher, sofern alle Sicherheitsvorbedingungen erfüllt sind. | `src/scrub.rs` |
| `pub fn scrub_if_safe_with_injector<F: FaultInjector>( path: &Path, policy: ScrubPolicy, reader_alive: bool, not_obsolete: bool, injector: &F, ) -> Result<ScrubOutcome>` | Bereinigt und löscht eine Datei sicher unter Verwendung eines benutzerdefinierten [`FaultInjector`]. | `src/scrub.rs` |


---

## `acta-env`

| Attribut | Wert |
|---|---|
| **Ring** | 0 — Ebene E0 — Autarke Bausteine & Fundamente |
| **Status** | Fertig |
| **Zweck** | System-Umwelt, deterministische Clock, SeededRng und MetricsSink. |
| **Unsafe-Code** | Nein (`#![forbid(unsafe_code)]`) |
| **Interne Abhängigkeiten** | — |
| **Externe Abhängigkeiten (Auswahl)** | — |

### Architekturbeschreibung

Canonical environment abstractions for Acta (Clock, Rng, IdGen, Metrics).  # Architecture Role Zero internal workspace dependencies (E0 Foundation Micro-Core). Pure standard library environment contracts and deterministic test doubles.

### Modul- und Re-Export-Oberfläche (lib.rs)

```rust
pub mod clock;
pub mod env;
pub mod id_gen;
pub mod metrics;
pub mod rng;
pub mod manual_clock;
pub use clock::{Clock, SystemClock};
pub use env::*;
pub use id_gen::{IdGen, SequentialIdGen};
pub use metrics::{ HistogramSnapshot, InMemoryMetricsSink, MetricKey, MetricSnapshot, MetricsSink, NoopMetricsSink, };
pub use rng::{Rng, SeededRng};
pub use manual_clock::ManualClock;
```

### Feature-Flags

| Flag | Aktiviert |
|---|---|
| `default` | `["std"]` |
| `std` | `[]` |
| `test-utils` | `[]` |

### Schnittstellen — Public Traits (4)

**`Clock`** — *Port trait for time operations, enabling deterministic testing by decoupling system wall-clock and monotonic time sources. In production, [`SystemClock`] provides access to the real OS clocks. In unit tests or simulations, deterministic doubles like `ManualCl…* (`src/clock.rs`)

```rust
pub trait Clock : Send + Sync + 'static {
/// Returns the current wall-clock time as nanoseconds since the Unix epoch.
    ///
    /// Returns `0` if system time is before the epoch instead of panicking.
    ///
    /// # Invarianten
    /// - `INV-ENV-TIME-1`: Garantiert panikfrei; liefert bei Zeiten vor Unix-Epoche `0`.
    ///
    /// # Fehler
    /// Keine; garantiert panikfrei, liefert bei Zeit vor Unix-Epoche `0`.
    /// # Crash
    /// Reiner Statusabruf ohne persistente Mutation.
    fn now_unix_nanos(&self) -> u64;

    /// Returns nanoseconds elapsed since a monotonic baseline reference point.
    ///
    /// # Invarianten
    /// - `INV-ENV-TIME-2`: Garantiert panikfrei und monoton nicht fallend innerhalb des Prozesses.
    ///
    /// # Fehler
    /// Keine; garantiert panikfrei und monoton nicht fallend.
    /// # Crash
    /// Reiner Statusabruf ohne persistente Mutation.
    fn monotonic_nanos(&self) -> u64;
}
```

**`IdGen`** — *Port trait for unique ID generation.* (`src/id_gen.rs`)

```rust
pub trait IdGen : Send + Sync + 'static {
/// Generates and returns the next unique 64-bit unsigned identifier.
    ///
    /// # Invarianten
    /// - `INV-ENV-ID-1`: Keine Duplikate bei aufeinanderfolgenden Aufrufen; threadsichere Monotonie.
    ///
    /// # Fehler
    /// Keine; garantiert panikfrei durch atomare Operationen.
    /// # Crash
    /// Reiner flüchtiger RAM-Zustand; bei Crash wird der Generator neu initialisiert.
    fn next_id(&self) -> u64;
}
```

**`MetricsSink`** — *Metrics sink port trait for recording system counters, gauges, and histograms. Implemented by observability backends to collect operational metrics from domain components without creating direct dependencies on specific metrics libraries.* (`src/metrics.rs`)

```rust
pub trait MetricsSink : Send + Sync + 'static {
/// Records a counter metric increment.
    ///
    /// # Fehler
    /// Keine; garantiert panikfrei und nicht-blockierend.
    /// # Crash
    /// Reiner flüchtiger RAM-Zähler; keine I/O oder persistente Mutation.
    fn record_counter(&self, name: &str, value: u64, labels: &[(&str, &str)]);

    /// Records a gauge metric absolute value.
    ///
    /// # Fehler
    /// Keine; garantiert panikfrei und nicht-blockierend.
    /// # Crash
    /// Reiner flüchtiger RAM-Messwert; keine I/O oder persistente Mutation.
    fn record_gauge(&self, name: &str, value: f64, labels: &[(&str, &str)]);

    /// Records a histogram observation value.
    ///
    /// # Fehler
    /// Keine; garantiert panikfrei und nicht-blockierend.
    /// # Crash
    /// Reiner flüchtiger RAM-Histogrammwert; keine I/O oder persistente Mutation.
    fn record_histogram(&self, name: &str, value: f64, labels: &[(&str, &str)]);
}
```

**`Rng`** — *Port trait for random number generators, enabling deterministic testing by decoupling non-deterministic entropy sources.* (`src/rng.rs`)

```rust
pub trait Rng : Send + Sync + 'static {
/// Generates the next pseudorandom 64-bit unsigned integer.
    ///
    /// # Invarianten
    /// - `INV-ENV-RNG-1`: Deterministische Reproduzierbarkeit bei gleichem Seed.
    ///
    /// # Fehler
    /// Keine; garantiert panikfrei und nicht-blockierend.
    /// # Crash
    /// Reiner flüchtiger RAM-Zustand; bei Crash wird der Generator neu initialisiert.
    fn next_u64(&self) -> u64;

    /// Fills `dest` with pseudorandom bytes generated by this RNG.
    ///
    /// # Fehler
    /// Keine; garantiert panikfrei für alle Puffergrößen.
    /// # Crash
    /// Reiner flüchtiger RAM-Zustand; keine persistente Mutation.
    fn fill_bytes(&self, dest: &mut [u8]);

    /// Returns a pseudorandom `f64` value in the half-open interval `[0.0, 1.0)`
    /// with 53 bits of precision.
    ///
    /// # Invarianten
    /// - `INV-ENV-RNG-2`: Ergebnis liegt strikt im Bereich `[0.0, 1.0)`.
    ///
    /// # Fehler
    /// Keine; garantiert panikfrei.
    /// # Crash
    /// Reiner Statusabruf ohne persistente Mutation.
    fn next_unit_f64(&self) -> f64;
}
```

### Öffentliche Datentypen — Structs (9)

| Struct | Felder / Form | Beschreibung | Datei |
|---|---|---|---|
| `HistogramSnapshot` | `{ /// Sum of all observed values. pub sum: f64, /// Total count of observations. pub count: u64, /// Minimum observed value (None if empty). pub min: Option<f64>… }` | Snapshot of a recorded histogram metric. | `src/metrics.rs` |
| `InMemoryMetricsSink` | `{ counters: RwLock<BTreeMap<MetricKey, Arc<AtomicU64>>>, gauges: RwLock<BTreeMap<MetricKey, f64>>, histograms: RwLock<BTreeMap<MetricKey, HistogramState>>, }` | Production-ready local thread-safe implementation of [`MetricsSink`]. Stores metrics strictly in-process with no external network exports (privacy-pr… | `src/metrics.rs` |
| `ManualClock` | `{ /// Aktueller simulierter Unix-Zeitstempel in Nanosekunden. unix_nanos: AtomicU64, /// Aktueller simulierter monotoner Zeitstempel in Nanosekunden. mono_nanos:… }` | Controllable clock implementation for testing determinism and time regression. Permits precise manual stepping of unix and monotonic nanoseconds via… | `src/manual_clock.rs` |
| `MetricKey` | `{ /// Metric identifier (e.g. `lsm_commit_total`). pub name: String, /// Sorted label key-value pairs. pub labels: Vec<(String, String)>, }` | Key identifying a metric by name and sorted key-value labels. | `src/metrics.rs` |
| `MetricSnapshot` | `{ /// Counter metrics (accumulated values). pub counters: BTreeMap<MetricKey, u64>, /// Gauge metrics (last recorded f64 values). pub gauges: BTreeMap<MetricKey,… }` | In-memory snapshot of all metrics collected by [`InMemoryMetricsSink`]. | `src/metrics.rs` |
| `NoopMetricsSink` | `(tuple/unit)` | A no-op implementation of [`MetricsSink`] that silently discards all metrics events. | `src/metrics.rs` |
| `SeededRng` | `{ /// Interner 64-Bit-Zustand, atomar fortgeschaltet über `fetch_add(GOLDEN_RATIO_64)`. state: AtomicU64, }` | A fast, deterministic 64-bit pseudorandom number generator using SplitMix64 with atomic state transitions (`AtomicU64`). Lock-free and thread-safe wi… | `src/rng.rs` |
| `SequentialIdGen` | `{ /// Nächste zu vergebende ID im Prozessspeicher. next: AtomicU64, }` | A thread-safe ID generator that produces monotonically increasing 64-bit IDs. Wraps around safely on integer overflow (`u64::MAX` -> `0`) via atomic… | `src/id_gen.rs` |
| `SystemClock` | `{ /// Monotoner Referenzpunkt zur Initialisierungszeit der Instanz. start_instant: Instant, }` | Standard production implementation of [`Clock`] backed by [`SystemTime`] and [`Instant`]. | `src/clock.rs` |

### Öffentliche Funktionen (12)

| Signatur | Beschreibung | Datei |
|---|---|---|
| `pub fn new() -> Self` | Creates a new [`SystemClock`] using the current [`Instant`] as the monotonic baseline. # Didaktischer Hinweis Die Initialisierung von `star… | `src/clock.rs` |
| `pub fn new(start: u64) -> Self` | Creates a new [`SequentialIdGen`] starting at `start`. | `src/id_gen.rs` |
| `pub fn unix_ms(c: &dyn Clock) -> u64` | Converts clock time to unix milliseconds by rounding down (SPEC-v2 §4.2, B02). Ensures deterministic millisecond calculation across legacy… | `src/lib.rs` |
| `pub fn advance_nanos(&self, delta: u64)` | Advances the clock by the given delta in nanoseconds. Erhöht sowohl Unix- als auch Monotonie-Nanosekunden um `delta`. | `src/manual_clock.rs` |
| `pub fn new(unix_nanos: u64, mono_nanos: u64) -> Self` | Creates a new `ManualClock` initialized at the given unix and monotonic timestamps. | `src/manual_clock.rs` |
| `pub fn set_monotonic_nanos(&self, nanos: u64)` | Sets the monotonic timestamp to an absolute value in nanoseconds. | `src/manual_clock.rs` |
| `pub fn set_unix_nanos(&self, nanos: u64)` | Sets the unix timestamp to an absolute value in nanoseconds. | `src/manual_clock.rs` |
| `pub fn new(name: impl Into<String>, labels: &[(&str, &str)]) -> Self` | Creates a new normalized [`MetricKey`]. Sortiert Labels deterministisch lexikografisch, um Hash- und BTreeMap-Konsistenz zu wahren. | `src/metrics.rs` |
| `pub fn new() -> Self` | Creates a new empty [`InMemoryMetricsSink`]. | `src/metrics.rs` |
| `pub fn snapshot(&self) -> MetricSnapshot` | Captures a point-in-time [`MetricSnapshot`] of all recorded counters, gauges, and histograms. | `src/metrics.rs` |
| `pub fn from_entropy() -> Self` | Creates a new [`SeededRng`] seeded from process/runtime entropy. # Non-Cryptographic Usage This entropy source uses [`std::collections::has… | `src/rng.rs` |
| `pub fn new(seed: u64) -> Self` | Creates a new [`SeededRng`] initialized with the given seed. Garantiert exakte Bitgleichheit zu den kanonischen Golden-Testvektoren. | `src/rng.rs` |


---

## `acta-erasure`

| Attribut | Wert |
|---|---|
| **Ring** | 0 — Ebene E0 — Autarke Bausteine & Fundamente |
| **Status** | Fertig |
| **Zweck** | Löschungsprüfung, kryptografische Nachweise und Audit-Ledger. |
| **Unsafe-Code** | Nein (`#![forbid(unsafe_code)]`) |
| **Interne Abhängigkeiten** | — |
| **Externe Abhängigkeiten (Auswahl)** | `aes-gcm-siv`, `ahash`, `bincode`, `blake3`, `bytes`, `ed25519-dalek`, `futures-util`, `hmac`, `parking_lot`, `rand`, `serde`, `serde_json`, `sha2`, `subtle`, `thiserror`, `tokio`, `tracing`, `zeroize` |

### Architekturbeschreibung

Löschkern: Saga, Gate, Journal, Tresor, Messung, signierter Bericht.  Ring 3 (SPEC §5.1, §5.2, §11). Abhängigkeitsregel AR-3 (SPEC §5.3): Erlaubte Abhängigkeiten sind ausschließlich `acta-types`, `acta-ports` und `acta-crypto`.

### Modul- und Re-Export-Oberfläche (lib.rs)

```rust
pub mod backend;
pub mod crypto_local;
pub mod deletion_proof;
pub mod erasure_domain;
pub mod error;
pub mod gate;
pub mod io;
pub mod journal;
pub mod ledger;
pub mod measure;
pub mod purge;
pub mod replace;
pub mod report;
pub mod restore;
pub mod restricted;
pub mod saga;
pub mod scope;
pub mod supersede;
pub mod test_mocks;
pub mod types_local;
pub mod vault;
pub mod ed25519_proof;
pub mod kv_shredding;
pub mod orchestrator;
pub mod revocation_log;
```

### Feature-Flags

| Flag | Aktiviert |
|---|---|
| `default` | `["signing", "vault", "async"]` |
| `signing` | `["dep:ed25519-dalek"]` |
| `vault` | `["dep:aes-gcm-siv"]` |
| `async` | `["dep:tokio", "dep:futures-util"]` |

### Schnittstellen — Public Traits (39)

**`Clock`** — *Port trait for time operations.* (`src/types_local.rs`)

```rust
pub trait Clock : Send + Sync + 'static {
/// Returns the current wall-clock time as nanoseconds since the Unix epoch.
    fn now_unix_nanos(&self) -> u64;

    /// Returns nanoseconds elapsed since a monotonic baseline reference point.
    fn monotonic_nanos(&self) -> u64;
}
```

**`CommitBarrier`** — *CommitBarrier-Port (SPEC §11.3, D-57, D-78).* (`src/types_local.rs`)

```rust
pub trait CommitBarrier : Send + Sync {
/// Bezieht geteilten Commit-Zugriff.
    fn shared(&self) -> BoxFuture<'_, BarrierShared>;
    /// Bezieht exklusiven Gate-Zugriff.
    fn exclusive(&self) -> BoxFuture<'_, BarrierExclusive>;
}
```

**`EpochKeys`** — *Trait zur Bereitstellung von Epochenschlüsseln (SPEC D-125). Rotation vernichtet den alten Epochenschlüssel (Slot E im Tresor). Danach sind alte Commitments nicht mehr verknüpfbar.* (`src/journal/epoch.rs`)

```rust
pub trait EpochKeys : Send + Sync {
/// Liefert den 32-Byte-Epochenschlüssel `K_commit(epoch)`.
    fn commit_key(&self, epoch_id: u32) -> [u8; 32];
}
```

**`ErasureLayer`** — *Schichtvertrag für Speicher- und Index-Teilnehmer (SPEC §8.2).* (`src/types_local.rs`)

```rust
pub trait ErasureLayer : Send + Sync {
/// Schichtkennung.
    fn id(&self) -> LayerId;
    /// Löst Ziel vor dem Gating in den konkreten Bereich auf.
    fn resolve(&self, t: &EraseTarget) -> BoxFuture<'_, Result<LayerScope, String>>;
    /// Vorbereitungsmaßnahmen vor dem Gating.
    fn on_gate(&self, s: &LayerScope) -> BoxFuture<'_, Result<(), String>>;
    /// Führt physische Bereinigung durch.
    fn purge(&self, s: &LayerScope, cx: &PurgeCx) -> BoxFuture<'_, Result<PurgeOutcome, String>>;
    /// Prüft Rückstände und erzeugt Zeugen.
    fn witness(
        &self,
        s: &LayerScope,
        sc: &Scanner<Armed>,
    ) -> BoxFuture<'_, Result<LayerWitness, String>>;
    /// Meldet die Schlüsselabdeckung.
    fn key_coverage(&self) -> KeyCoverage;
}
```

**`ErasureObserver`** — *Port-Vertrag für Lösch-Ereignis-Benachrichtigungen.* (`src/types_local.rs`)

```rust
pub trait ErasureObserver : Send + Sync + 'static {
/// Benachrichtigt den Observer über ein Ereignis.
    fn on_erased(&self, ev: ErasureEvent) -> BoxFuture<'static, Result<(), ObserverError>>;
}
```

**`ErasureTarget`** — *Generisches Löschziel (ErasureTarget) für Mandanten, Collections oder Dokumente.* (`src/backend.rs`)

```rust
pub trait ErasureTarget : Send + Sync + Debug + 'static {
/// Eindeutiger Identifikator des Targets.
    fn target_id(&self) -> String;

    /// Art des Löschziels.
    fn target_type(&self) -> &'static str;
}
```

**`FenceGate`** — *Port-Vertrag für FenceGate (INV-ERA-5). Blockiert konkurrierende Schreiber und Lesezugriffe während eines physischen Purge.* (`src/types_local.rs`)

```rust
pub trait FenceGate : Send + Sync {
/// Aktiviert den Fence für einen bestimmten KeyScope und gibt ein RAII-Token zurück.
    fn acquire_fence<'a>(
        &'a self,
        scope: &'a KeyScope,
    ) -> BoxFuture<'a, Result<FencePermit, crate::error::ErasureError>>;

    /// Prüft synchron, ob ein Fence für den gegebenen Schlüssel oder Bereich aktiv ist.
    fn is_fenced(&self, key: &KeyBuf) -> bool;
}
```

**`FenceGate`** — *Port-Trait für das Fencing von Schlüsselbereichen während des physischen Purge (INV-ERA-5).* (`src/gate/fence.rs`)

```rust
pub trait FenceGate : Send + Sync {
/// Blockiert einen Schlüsselbereich für Schreibzugriffe und liefert einen Autorisierungs-Token.
    ///
    /// # Fehler
    /// - [`FenceGateError::OverlapConflict`]: Falls `range` mit einem bereits aktiven Fence überlappt.
    /// - [`FenceGateError::CorruptedState`]: Falls der Wächterzustand korrumpiert ist.
    /// - [`FenceGateError::DrainTimeout`]: Falls aktive Schreiber im Bereich nicht rechtzeitig abschließen.
    fn acquire_fence(&self, range: FenceRange) -> Result<FenceToken, FenceGateError>;

    /// Hebt einen aktiven Fence auf. Dies gelingt ausschließlich mit dem ursprünglichen Besitzer-Token.
    ///
    /// # Fehler
    /// - [`FenceGateError::InvalidToken`]: Falls der Token unbekannt oder abgelaufen ist.
    /// - [`FenceGateError::CorruptedState`]: Falls der Zustand korrumpiert ist.
    fn release_fence(&self, token: &FenceToken) -> Result<(), FenceGateError>;

    /// Prüft, ob für einen Einzelschlüssel ein aktiver Fence vorliegt.
    fn is_fenced(&self, key: &[u8]) -> bool;

    /// Prüft, ob ein Schreibzugriff auf einen Einzelschlüssel zulässig ist.
    ///
    /// Ist der Schlüssel frei, wird ein RAII-[`WriterGuard`] zurückgegeben.
    ///
    /// # Fehler
    /// - [`FenceGateError::Fenced`]: Falls der Schlüssel in einem gefencten Bereich liegt.
    /// - [`FenceGateError::CorruptedState`]: Falls der Zustand korrumpiert ist (Fail-Closed).
    fn check_write(&self, key: &[u8]) -> Result<WriterGuard, FenceGateError>;

    /// Prüft eine Menge von Schlüsseln atomar auf Schreibzulässigkeit.
    ///
    /// Wird mindestens ein Schlüssel blockiert, schlägt der gesamte Batch fehl.
    fn check_write_batch<'a, I>(&self, keys: I) -> Result<WriterGuard, FenceGateError>
    where
        I: IntoIterator<Item = &'a [u8]>;

    /// Gibt an, ob aktuell mindestens ein Fence aktiv ist.
    fn is_active(&self) -> bool;
}
```

**`FileKeyring`** — *Port-Vertrag für Dateischlüsselverwaltung.* (`src/types_local.rs`)

```rust
pub trait FileKeyring : Send + Sync {
/// Erzeugt einen neuen Dateischlüssel.
    fn create(&self, kind: FileKind, id: FileId) -> BoxFuture<'_, Result<FileKey, String>>;
    /// Öffnet einen existierenden Dateischlüssel.
    fn open(&self, id: FileId) -> BoxFuture<'_, Result<FileKey, String>>;
    /// Vernichtet den Dateischlüssel im Tresor dauerhaft.
    fn destroy(&self, id: FileId) -> BoxFuture<'_, Result<(), String>>;
}
```

**`IdGen`** — *Port trait for unique ID generation.* (`src/types_local.rs`)

```rust
pub trait IdGen : Send + Sync + 'static {
/// Generates and returns the next unique 64-bit unsigned identifier.
    fn next_id(&self) -> u64;
}
```

**`KeyStore`** — *Port-Vertrag für KeyStore (WP-K). Verwaltet Schlüsselhierarchien (KEK/DEK), Wrapping und unwiderrufliche Schlüsselvernichtung.* (`src/types_local.rs`)

```rust
pub trait KeyStore : Send + Sync {
/// Vernichtet alle Schlüssel (DEKs / SubKeys) eines Scopes unwiderruflich.
    fn destroy_scope_keys(
        &self,
        ctx: &RequestContext,
        scope: &KeyScope,
    ) -> Result<usize, crate::error::ErasureError>;

    /// Prüft, ob ein Schlüssel für die Entschlüsselung noch existiert.
    fn key_exists(&self, scope: &KeyScope) -> Result<bool, crate::error::ErasureError>;
}
```

**`KeyStore`** — *Synchroner Port-Trait für Schlüsselverwaltung und Vernichtung (WP-K). Folgt der synchronen Kernregel: Keine Async-Runtime im Löschpfad. Zeit und Zufall werden ausschließlich über Ports (`Clock`) injiziert.* (`src/vault/key_store.rs`)

```rust
pub trait KeyStore : Send + Sync {
/// Erzeugt oder lädt den Gruppen-KEK für einen Bereich `(group_id, epoch)`.
    fn get_or_derive_kek(&self, scope: GroupEpochKey) -> Result<[u8; 32], KeyStoreError>;

    #[cfg(feature = "vault")]
    /// Verschlüsselt und wrappt einen Datensatz unter Envelope Encryption.
    fn encrypt_record(
        &self,
        scope: GroupEpochKey,
        record_id: u64,
        plaintext: &[u8],
    ) -> Result<EncryptedEpochRecordPayload, KeyStoreError>;

    #[cfg(feature = "vault")]
    /// Entschlüsselt einen Datensatz unter Envelope Encryption.
    fn decrypt_record(
        &self,
        payload: &EncryptedEpochRecordPayload,
    ) -> Result<Vec<u8>, KeyStoreError>;

    /// Prüft ob ein Schlüsselbereich aktiv (nicht widerrufen) ist.
    fn is_active(&self, scope: GroupEpochKey) -> bool;

    /// Vernichtet alle Schlüssel eines Bereichs unwiderruflich (`purge_keys`).
    ///
    /// Nullt das Schlüsselmaterial im RAM mit Zeroize, markiert die Epoche
    /// als vernichtet und liefert ein Quittungsobjekt zur Ledger-Verankerung.
    /// Idempotent: Mehrfachaufrufe führen nicht zu Fehlern oder Reaktivierung (INV-NO-RESURRECTION-1).
    fn purge_keys(
        &self,
        scope: GroupEpochKey,
        clock: Option<&dyn Clock>,
    ) -> Result<KeyPurgeReceipt, KeyStoreError>;
}
```

**`LayerScanner`** — *Trait für Storage-Scanner zur Durchführung von Restmengen-Messungen.* (`src/types_local.rs`)

```rust
pub trait LayerScanner : Send + Sync {
/// Gibt die von diesem Scanner bediente Schicht zurück.
    fn layer(&self) -> WitnessLayer;

    /// Führt das Scannen von Einheiten auf der übergebenen `ScanSession` aus.
    fn scan_units(
        &self,
        session: &mut ScanSession,
        ctx: &RequestContext,
    ) -> Result<(), crate::error::ErasureError>;

    /// Default-Implementierung von `scan`.
    fn scan(
        &self,
        ctx: &RequestContext,
        target: ScanTarget,
        technique: ScanTechnique,
        clock: &dyn Clock,
    ) -> Result<ScanWitness, crate::error::ErasureError> {
        let mut session = ScanSession::begin(self.layer(), target, technique);
        self.scan_units(&mut session, ctx)?;
        session.finish(clock)
    }
}
```

**`LayerWitnessExt`** — *Extension trait providing accessors for `LayerWitness` without mutating `acta-ports`.* (`src/deletion_proof.rs`)

```rust
pub trait LayerWitnessExt {
/// Extracts the `wal_seq_after` if layer is `WalAllSegments`, or `None`.
    fn wal_seq_after(&self) -> Option<u64>;
    /// Extracts the positive control check match count.
    fn control_matches(&self) -> u64;
}
```

**`LedgerSink`** — *Port-Vertrag für das Lösch-Ledger.* (`src/types_local.rs`)

```rust
pub trait LedgerSink : Send + Sync {
/// Hängt einen Eintrag an das Ledger an.
    fn append(&self, e: &LedgerEntry) -> Result<(), String>;
    /// Liest alle Einträge aus.
    fn read_all(&self) -> Result<Vec<LedgerEntry>, String>;
    /// Bereinigt alte Einträge nach Aufbewahrungsfrist.
    fn prune(&self, older_than: Timestamp) -> Result<usize, String>;
}
```

**`LedgerStorage`** — *Abstrakter synchroner Speicher-Port für Append-Only Ledgers.* (`src/ledger/storage.rs`)

```rust
pub trait LedgerStorage : Send + Sync {
/// Prüft, ob das Ledger bereits existiert.
    fn exists(&self) -> bool;

    /// Prüft, ob der Speicher leer ist.
    fn is_empty(&self) -> Result<bool, LedgerError> {
        Ok(self.len()? == 0)
    }

    /// Liest den gesamten Speicherinhalt.
    fn read_all(&self) -> Result<Vec<u8>, LedgerError>;

    /// Hängt Daten atomar an und garantiert, dass bei einem Fehler vor dem Sync keine Teil-Daten verbleiben.
    fn append_atomic(&self, data: &[u8]) -> Result<(), LedgerError>;

    /// Synchronisiert die geschriebenen Daten dauerhaft auf das Speichermedium (fsync).
    fn sync(&self) -> Result<(), LedgerError>;

    /// Setzt den Speicherinhalt zurück oder kürzt ihn (z. B. für Rollback oder Prune).
    fn truncate(&self, len: usize) -> Result<(), LedgerError>;

    /// Liefert die aktuelle Bytegröße des Speichers.
    fn len(&self) -> Result<usize, LedgerError>;
}
```

**`ProofKeyProvider`** — *Injected provider interface for deletion proof signing keys.* (`src/orchestrator.rs`)

```rust
pub trait ProofKeyProvider : Send + Sync {
/// Resolves the deletion proof key for the specified tenant.
    fn resolve_proof_key(&self, tenant_id: TenantId) -> Result<Vec<u8>, ErasureError>;
}
```

**`PurgeBarrier`** — *PurgeBarrier-Port (SPEC §11.3, §11.4).* (`src/types_local.rs`)

```rust
pub trait PurgeBarrier : Send + Sync {
/// Bezieht geteilte Kompaktions-Erlaubnis.
    fn compaction(&self) -> BoxFuture<'_, CompactionPermit>;
    /// Bezieht exklusive Purge-Erlaubnis.
    fn purge(&self) -> BoxFuture<'_, PurgePermit>;
    /// Prüft ob ein exklusiver Purge wartet.
    fn purge_waiting(&self) -> bool;
}
```

**`Purger`** — *Trait für physische Bereinigungsaktionen (Purger) einer Schicht.* (`src/types_local.rs`)

```rust
pub trait Purger : Send + Sync {
/// Führt die physische Bereinigung für das gegebene Ziel aus.
    fn purge(
        &self,
        ctx: &RequestContext,
        target: &ScanTarget,
    ) -> Result<PurgeStats, crate::error::ErasureError>;
}
```

**`ReadGate`** — *Synchroner ReadGate-Port (SPEC §4.2, §11.3, PO-07).* (`src/types_local.rs`)

```rust
pub trait ReadGate : Send + Sync {
/// Prüft den Status eines einzelnen Schlüssels.
    fn check(&self, k: &KeyBuf) -> GateDecision;
    /// Prüft den Status einer Bereichsgrenze.
    fn check_bound(&self, b: &KeyBound) -> GateDecision;
    /// Liefert die Bereichsentscheidung für Kompaktionen.
    fn variant_for_range(&self, r: &KeyRange) -> RangeGate;
    /// Liefert die Gating-Sequenznummer, falls vorhanden.
    fn gated_seq(&self, k: &KeyBuf) -> Option<Seq>;
    /// Liefert den monotonen Gate-Epochenzähler.
    fn gate_epoch(&self) -> u64 {
        0
    }
    /// Prüft ob ein Schlüssel durch ein Gate abgedeckt ist.
    fn covers(&self, k: &KeyBuf) -> bool {
        match self.check(k) {
            GateDecision::Erased | GateDecision::Restricted => true,
            GateDecision::Open => false,
        }
    }
    /// Prüft Status eines kanonischen 78-Byte Schlüssels.
    fn check_raw(&self, _raw: &[u8; 78]) -> GateDecision {
        GateDecision::Open
    }
    /// Prüft ob ein 78-Byte Schlüssel abgedeckt ist.
    fn covers_raw(&self, raw: &[u8; 78]) -> bool {
        match self.check_raw(raw) {
            GateDecision::Erased | GateDecision::Restricted => true,
            GateDecision::Open => false,
        }
    }
    /// Liefert Sequenznummer eines 78-Byte Schlüssels.
    fn gated_seq_raw(&self, _raw: &[u8; 78]) -> Option<Seq> {
        None
    }
}
```

**`RecordSealer`** — *Port-Vertrag für RecordSealer.* (`src/types_local.rs`)

```rust
pub trait RecordSealer : Send + Sync {
/// Versiegelt Klartext.
    fn seal<'a>(
        &'a self,
        ctx: &'a SealCtx,
        key: &'a KeyBuf,
        plain: &'a [u8],
    ) -> BoxFuture<'a, Result<Envelope, String>>;

    /// Öffnet einen Envelope.
    fn open<'a>(&'a self, key: &'a KeyBuf, env: &'a [u8])
        -> BoxFuture<'a, Result<Vec<u8>, String>>;

    /// Invalidiert Schlüssel aus dem Cache.
    fn evict(&self, scope: &KeyRange);

    /// Erzwingt Slot-Dauerhaftigkeit.
    fn make_slots_durable(&self) -> BoxFuture<'_, Result<(), String>>;
}
```

**`ReportSigner`** — *Port-Vertrag für Berichtssignierung.* (`src/types_local.rs`)

```rust
pub trait ReportSigner : Send + Sync {
/// Liefert die Signer-ID.
    fn signer_id(&self) -> SignerId;
    /// Signiert kanonische Berichts-Bytes.
    fn sign_report<'a>(
        &'a self,
        canonical_report_bytes: &'a [u8],
    ) -> BoxFuture<'a, Result<ReportSignature, SignerError>>;
}
```

**`ResidueScanner`** — *Port-Vertrag für ResidueScanner (WP-12 / INV-CLASS-MEASURED-1). Untersucht Index-Artefakte (z.B. Vektorindex / Graph) auf unzulässige Restspuren.* (`src/types_local.rs`)

```rust
pub trait ResidueScanner : Send + Sync {
/// Führt den Restspuren-Scan für einen Scope auf einem Ziel-Artefakt durch.
    fn scan_residues(
        &self,
        ctx: &RequestContext,
        scope: &KeyScope,
        target: &ScanTarget,
    ) -> Result<u64, crate::error::ErasureError>;
}
```

**`ResidueScanner`** — *Synchroner Port für Index-spezifische Restspuren-Scanner (SPEC §12, WP-12). Implementiert durch Adapter in Vektor-, Text- oder Graph-Index-Crates. Arbeitet rein lesend ohne Modifikationen am Index.* (`src/measure/residue.rs`)

```rust
pub trait ResidueScanner : Send + Sync {
/// Liefert die Schichtkennung des Scanners.
    fn layer(&self) -> LayerId;

    /// Prüft, ob das injizierte Positivkontroll-Muster auffindbar ist.
    ///
    /// Gibt `Ok(true)` zurück, wenn das Kontrollmuster nachweisbar vorhanden ist.
    /// Gibt `Ok(false)` zurück, wenn es nicht detektiert wurde.
    fn canary_probe(&self, canary: &[u8]) -> Result<bool, ResidueMeasurementError>;

    /// Durchsucht den Index nach Spuren der spezifizierten Abfrage.
    ///
    /// Gibt ein `ResidueScanResult` zurück. Fehler führen fail-closed zum Abbruch.
    fn scan_residue(&self, query: &ResidueQuery) -> Result<ResidueScanResult, ResidueMeasurementError>;
}
```

**`SagaGate`** — *Naht für das Gate-Subsystem (SPEC §11.3, ER-11, ER-12, ER-13).* (`src/saga/seams.rs`)

```rust
pub trait SagaGate : Send + Sync {
/// Trägt eine Lesesperre ein und persistiert das Gate-Log (fsync #2).
    /// Liefert die `gated_seq`.
    fn add_gate<'a>(
        &'a self,
        erasure_id: ErasureId,
        target: &'a EraseTarget,
    ) -> BoxFuture<'a, Result<u64, ErasureError>>;

    /// Kompaktiert das Gate nach Abschluss der Löschung (SPEC §11.2 Schritt 6).
    fn compact_gate<'a>(&'a self, erasure_id: ErasureId)
        -> BoxFuture<'a, Result<(), ErasureError>>;

    /// Liefert die Gate-Entscheidung (`Erased`, `Restricted`, `Open`) für ein Ziel.
    fn decision_for_target<'a>(
        &'a self,
        target: &'a EraseTarget,
    ) -> BoxFuture<'a, Result<GateDecision, ErasureError>>;
}
```

**`SagaJournal`** — *Naht für das Intent-Journal (SPEC §11.6, ER-29).* (`src/saga/seams.rs`)

```rust
pub trait SagaJournal : Send + Sync {
/// Hängt den initialen `Requested+Intent`-Eintrag mit eingebettetem Umfang an (fsync #1).
    fn append_intent<'a>(
        &'a self,
        erasure_id: ErasureId,
        target: &'a EraseTarget,
        attestation: &'a RetentionAttestation,
    ) -> BoxFuture<'a, Result<(), ErasureError>>;

    /// Hängt einen Zustandsübergang an.
    fn append_state<'a>(
        &'a self,
        erasure_id: ErasureId,
        state: SagaState,
        reason: Option<&'a PendingReason>,
    ) -> BoxFuture<'a, Result<(), ErasureError>>;

    /// Rekonstruiert noch offene Sagas aus dem Journal.
    fn recover_open_sagas<'a>(&'a self) -> BoxFuture<'a, Result<Vec<EraseSaga>, ErasureError>>;
}
```

**`SagaLedger`** — *Naht für das Ledger und Restore-Absicherung (SPEC §11.8, ER-30).* (`src/saga/seams.rs`)

```rust
pub trait SagaLedger : Send + Sync {
/// Schreibt den Eintrag nach `Completed` in das Ledger außerhalb des Datenverzeichnisses.
    fn record_completed<'a>(
        &'a self,
        erasure_id: ErasureId,
        target: &'a EraseTarget,
        gated_seq: u64,
        completed_at: u64,
    ) -> BoxFuture<'a, Result<(), ErasureError>>;
}
```

**`SagaMeasure`** — *Naht für die Messung und Zeugenprüfung (SPEC §12, ER-25, ER-26).* (`src/saga/seams.rs`)

```rust
pub trait SagaMeasure : Send + Sync {
/// Führt die Positivkontrolle durch und verifiziert Residuen für eine Schicht.
    fn witness_layer<'a>(
        &'a self,
        layer_id: LayerId,
        target: &'a EraseTarget,
    ) -> BoxFuture<'a, Result<LayerWitness, ErasureError>>;
}
```

**`SagaReport`** — *Naht für die Berichterstellung und kryptografische Signatur (SPEC §11.7, ER-27).* (`src/saga/seams.rs`)

```rust
pub trait SagaReport : Send + Sync {
/// Erstellt den kanonischen, signierten Löschbericht.
    fn sign_report<'a>(
        &'a self,
        erasure_id: ErasureId,
        target: &'a EraseTarget,
        attestation: &'a RetentionAttestation,
        witnesses: &'a [LayerWitness],
    ) -> BoxFuture<'a, Result<Vec<u8>, ErasureError>>;
}
```

**`SagaVault`** — *Naht für den Schlüsseltresor (SPEC §11.5, ER-05, ER-06).* (`src/saga/seams.rs`)

```rust
pub trait SagaVault : Send + Sync {
/// Vernichtet den Slot der höchsten Ebene (`T`/`C`/`S`/`D`) und verifiziert das Nullen (fsync #3).
    fn destroy_highest_key<'a>(
        &'a self,
        target: &'a EraseTarget,
    ) -> BoxFuture<'a, Result<(), ErasureError>>;

    /// Prüft lesend, dass der Schlüssel nicht mehr existiert.
    fn verify_key_absent<'a>(
        &'a self,
        target: &'a EraseTarget,
    ) -> BoxFuture<'a, Result<bool, ErasureError>>;
}
```

**`SnapshotRevoker`** — *Naht für den Snapshot-Widerruf (SPEC §11.2 Schritt 5a, D-61, L-21).* (`src/saga/seams.rs`)

```rust
pub trait SnapshotRevoker : Send + Sync {
/// Widerruft Snapshots des Mandanten für den betroffenen Bereich bis `gated_seq`.
    fn revoke_covering<'a>(
        &'a self,
        target: &'a EraseTarget,
        gated_seq: u64,
    ) -> BoxFuture<'a, Result<u64, ErasureError>>;
}
```

**`StorageBackend`** — *Generisches Storage-Backend für modulare Datenzugriffe im Löschkern.* (`src/backend.rs`)

```rust
pub trait StorageBackend : Send + Sync + 'static {
/// Führt physische Bereinigung auf dem Backend aus.
    fn purge<'a>(&'a self, cx: &'a PurgeCx) -> BoxFuture<'a, Result<PurgeOutcome, ErasureError>>;

    /// Liefert den Schicht-Identifikator des Backends.
    fn layer_id(&self) -> LayerId;
}
```

**`StructureDecoder`** — *Strukturdecoder-Schnittstelle zur Prüfung schichtspezifischer Strukturen (SPEC 12.1, AR-3). Ermöglicht die Injektion von Decodern für Manifeste, SSTable-Indizes, Segmentwörterbücher und Slot-Tabellen, ohne dass `acta-erasure` von `store`, `text` oder `vector`…* (`src/measure/field.rs`)

```rust
pub trait StructureDecoder : Send + Sync {
/// Name oder Typ des Decoders für Berichte/Traceability.
    fn name(&self) -> &str;

    /// Prüft, ob dieser Decoder für die angegebene Datei zuständig ist.
    fn can_decode(&self, path: &Path) -> bool;

    /// Decodiert und prüft den Dateiinhalt auf Einträge des gesuchten Ziels.
    ///
    /// Gibt die Anzahl der gefundenen Treffer zurück (0 im fehlerfreien Zustand).
    fn decode_and_check(
        &self,
        path: &Path,
        content: &[u8],
        target_pattern: &[u8],
    ) -> Result<u64, ErasureError>;
}
```

**`VaultScrubContext`** — *Kontext oder Callback-Prüfer für Manifest- und Dateiexistenz.* (`src/vault/scrub.rs`)

```rust
pub trait VaultScrubContext {
/// Prüft ob die gegebene Datei existiert (für F-Slots).
    fn file_exists(&self, file_id: &[u8; 16]) -> bool;

    /// Prüft ob ein Dokument noch im Manifest aktiv ist (für D-Slots).
    fn document_in_manifest(&self, doc_tag: &[u8; 16]) -> bool;

    /// Aktuelle nächste Sequenznummer des Stores (next_seq).
    fn next_seq(&self) -> u64;
}
```

**`Vfs`** — *Abstract object-safe Virtual File System interface for `acta-erasure`.* (`src/types_local.rs`)

```rust
pub trait Vfs : Send + Sync + 'static {
/// Opens a file according to the specified options.
    fn open(
        &self,
        _path: &std::path::Path,
        _opts: &VfsOpenOptions,
    ) -> Result<Box<dyn VfsFile>, crate::error::ErasureError> {
        Err(crate::error::ErasureError::Io(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "positional VfsFile open is not supported by default",
        )))
    }

    /// Reads data from byte offset `offset` into `buf` for file at `path`.
    fn read_at(
        &self,
        path: &std::path::Path,
        offset: u64,
        buf: &mut [u8],
    ) -> Result<usize, crate::error::ErasureError> {
        let file = self.open(path, &VfsOpenOptions::read_only())?;
        file.read_at(buf, offset)
            .map_err(crate::error::ErasureError::Io)
    }

    /// Reads the entire contents of a file at `path`.
    fn read(&self, path: &std::path::Path) -> Result<Vec<u8>, crate::error::ErasureError>;

    /// Atomically writes `data` to `path` with crash-consistency guarantees.
    fn write_atomic(
        &self,
        path: &std::path::Path,
        data: &[u8],
    ) -> Result<(), crate::error::ErasureError>;

    /// Creates a directory and all parent directories along `path` if missing.
    fn create_dir_all(&self, path: &std::path::Path) -> Result<(), crate::error::ErasureError>;

    /// Removes a file at `path`.
    fn remove_file(&self, path: &std::path::Path) -> Result<(), crate::error::ErasureError>;

    /// Returns `true` if a file or directory exists at `path`.
    fn exists(&self, path: &std::path::Path) -> Result<bool, crate::error::ErasureError>;

    /// Returns the length in bytes of the file at `path`.
    fn len(&self, path: &std::path::Path) -> Result<u64, crate::error::ErasureError>;

    /// Lists entry names inside directory `path`.
    fn list_dir(&self, path: &std::path::Path) -> Result<Vec<String>, crate::error::ErasureError>;

    /// Flushes directory metadata to persistent storage.
    fn sync_dir(&self, path: &std::path::Path) -> Result<(), crate::error::ErasureError>;
}
```

**`VfsExt`** — *Erweiterungstraits für `Vfs` zum Öffnen von positionellen `VfsFile` Handles.* (`src/types_local.rs`)

```rust
pub trait VfsExt {
/// Öffnet eine Datei als VfsFile.
    fn open_file(
        &self,
        path: &std::path::Path,
        opts: &VfsOpenOptions,
    ) -> Result<Box<dyn VfsFile>, crate::error::ErasureError>;
}
```

**`VfsFile`** — *Port-Vertrag für geöffnete VFS-Dateihandles.* (`src/types_local.rs`)

```rust
pub trait VfsFile : Send + Sync {
/// Liest an einer bestimmten Position.
    fn read_at(&self, buf: &mut [u8], offset: u64) -> std::io::Result<usize>;
    /// Verifiziertes Lesen unter Umgehung des Caches.
    fn read_at_verified(
        &self,
        buf: &mut [u8],
        offset: u64,
    ) -> std::io::Result<(usize, VerifiedVia)> {
        let n = self.read_at(buf, offset)?;
        Ok((n, VerifiedVia::Cached))
    }
    /// Schreibt an einer bestimmten Position.
    fn write_at(&self, buf: &[u8], offset: u64) -> std::io::Result<usize>;
    /// Verifiziertes Lesen (SPEC D-113).
    fn read_verified(&self, offset: u64, len: usize) -> std::io::Result<(Vec<u8>, VerifiedVia)> {
        let mut buf = vec![0u8; len];
        let n = self.read_at(&mut buf, offset)?;
        buf.truncate(n);
        Ok((buf, VerifiedVia::Cached))
    }
    /// Schreibt Daten dauerhaft auf Festplatte.
    fn sync_data(&self) -> std::io::Result<()>;
    /// Schreibt alle Daten und Metadaten dauerhaft.
    fn sync_all(&self) -> std::io::Result<()> {
        self.sync_data()
    }
    /// Kürzt oder erweitert die Dateilänge.
    fn set_len(&self, _len: u64) -> std::io::Result<()> {
        Ok(())
    }
    /// Ermittelt die Dateigröße.
    fn len(&self) -> std::io::Result<u64>;
    /// Prüft ob die Datei leer ist.
    fn is_empty(&self) -> std::io::Result<bool> {
        self.len().map(|l| l == 0)
    }
}
```

**`WalSink`** — *Port-Vertrag für WalSink (INV-ERA-6). Verwaltet WAL-Entsorgung, Verankerung und Unlesbarmachung von WAL-Einträgen eines gelöschten Scopes.* (`src/types_local.rs`)

```rust
pub trait WalSink : Send + Sync {
/// Entsorgt oder überschreibt alle WAL-Segmente/Einträge, die zum Scope gehören.
    fn purge_scope_wal(
        &self,
        ctx: &RequestContext,
        scope: &KeyScope,
    ) -> Result<u64, crate::error::ErasureError>;

    /// Schreibt einen Commit-Marker zur dauerhaften Verankerung der WAL-Bereinigung.
    fn append_erasure_marker(
        &self,
        ctx: &RequestContext,
        erasure_id: &ErasureId,
        scope: &KeyScope,
    ) -> Result<(), crate::error::ErasureError>;
}
```

**`WalSink`** — *Synchroner Port zur physischen oder kryptographischen Entsorgung von WAL-Einträgen eines Scopes. Gemäß der synchronen Kernregel existieren im Löschpfad keine Threads und keine Async-Aufrufe.* (`src/journal/wal_sink.rs`)

```rust
pub trait WalSink : Send + Sync {
/// Enumeriert alle bekannten WAL-Einträge des angegebenen Scopes.
    fn enumerate_scope_entries(&self, scope: &WalScope) -> Result<Vec<WalEntryId>, WalSinkError>;

    /// Führt die physische Entsorgung (Truncate/Überschreiben) der angegebenen Einträge durch.
    ///
    /// Gibt die tatsächliche Anzahl der entsorgten Einträge zurück.
    fn dispose_entries(
        &self,
        scope: &WalScope,
        entries: &[WalEntryId],
    ) -> Result<usize, WalSinkError>;

    /// Verwirft und nullt den scope-spezifischen Integritäts- oder Verschlüsselungsschlüssel.
    ///
    /// Nach diesem Aufruf können verbliebene Einträge des Scopes kryptographisch nicht mehr
    /// validiert oder entschlüsselt werden.
    fn discard_scope_integrity_key(&self, scope: &WalScope) -> Result<(), WalSinkError>;

    /// Prüft, ob der Scope bereits als vollständig entsorgt gilt.
    fn is_scope_disposed(&self, scope: &WalScope) -> Result<bool, WalSinkError>;
}
```

### Öffentliche Datentypen — Structs (202)

| Struct | Felder / Form | Beschreibung | Datei |
|---|---|---|---|
| `ActiveScopeFlight` | `{ scope: KeyScope, coordinator: Arc<SingleFlightCoordinatorInner>, }` | RAII-Token für einen aktiven Flug eines Scopes. | `src/scope/flight.rs` |
| `AncestorSalts` | `{ /// Mandanten-Salt (T-Slot). pub t: [u8; 32], /// Collection-Salt (C-Slot). pub c: [u8; 32], /// Betroffenen-Salt (S-Slot, optional). pub s: Option<[u8; 32]>,… }` | Ahnen-Salts für die DEK-Ableitung (Tenant, Collection, Subject, Document). | `src/crypto_local.rs` |
| `AppendLog` | `{ entries: Vec<JournalEntry>, }` | In-Memory-Journal-Implementierung (SPEC §11.6, ER-29). | `src/journal/appendlog.rs` |
| `Armed` | `(tuple/unit)` | Typzustand: Scanner ist scharfgeschaltet nach erfolgreicher Positivkontrolle. | `src/types_local.rs` |
| `Armed` | `(tuple/unit)` | — | `src/erasure_domain.rs` |
| `ArmedScanner` | `(tuple/unit)` | Scanner im Zustand `Armed` (scharfgeschaltet für Residuenprüfung). | `src/measure.rs` |
| `BarrierExclusive` | `{ _token: Option<Arc<()>>, }` | RAII-Token für exklusive Gate-Barriere. | `src/types_local.rs` |
| `BarrierShared` | `{ _token: Option<Arc<()>>, }` | RAII-Token für geteilte Commit-Barriere. | `src/types_local.rs` |
| `BatchGateOutcome` | `{ /// Sequenznummer, unter der die Gates gesetzt wurden. pub gated_seq: Seq, /// Zeigt an, ob die Gates erfolgreich im `ErasedSet` registriert wurden. pub applie… }` | Ergebnis eines Gruppen-Commits. | `src/gate/batch.rs` |
| `BatchGateRequest` | `{ /// Gate-Kennung. pub gate_id: [u8; 16], /// Untere Grenze. pub lo: KeyBuf, /// Obere Grenze. pub hi: KeyBuf, /// Optionale 65-Byte Grenzen (SPEC 9.1). pub bou… }` | Anfrage für einen Gruppen-Commit. | `src/gate/batch.rs` |
| `BatchRecordItem` | `{ /// Record-Typ (z. B. Completed oder KeyDestroyed) pub record_type: RecordType, /// Commitment-Hash pub commitment: [u8; 32], /// Bereichs-Tags (opak) pub rang… }` | Eingabedaten für einen Batch-Append im Ledger. | `src/ledger/durable.rs` |
| `BlobRewriter` | `{ journal: RewriteJournal, manifest: RewriteManifest, backend: Arc<dyn StorageBackend>, clock: Arc<dyn Clock>, poisoned: Arc<AtomicBool>, }` | Absturzsicherer, fail-closed BlobRewriter über `StorageBackend`, `RewriteJournal` und `RewriteManifest`. | `src/vault/sealer.rs` |
| `CollectionId` | `(tuple/unit)` | — | `src/types_local.rs` |
| `CollectionRef` | `(tuple/unit)` | Collection-Referenz (SPEC 7.3). | `src/erasure_domain.rs` |
| `CompactionPermit` | `{ _token: Option<Arc<()>>, }` | RAII-Token für Kompaktions-Erlaubnis. | `src/types_local.rs` |
| `ControlEvidence` | `(tuple/unit)` | Beleg für eine beobachtete Positivkontrolle (D-72). | `src/types_local.rs` |
| `ControlEvidence` | `(tuple/unit)` | — | `src/erasure_domain.rs` |
| `Dek` | `(tuple/unit)` | Data Encryption Key (DEK). | `src/crypto_local.rs` |
| `DeletionProof` | `{ /// Version der Signatur-Payload-Konstruktion. /// /// Version 1 (Legacy): Signiert NUR `scope`, `deleted_keys_hash` und `deleted_after_tx`. /// WARNUNG: Versi… }` | — | `src/deletion_proof.rs` |
| `DeletionProofKeyPair` | `{ /// Privater Signierschlüssel. pub signing_key: ed25519_dalek::SigningKey, /// Öffentlicher Verifikationsschlüssel. #[zeroize(skip)] pub verifying_key: ed25519… }` | Schlüsselpaar für Ed25519-Attestationsbeweise. Signierschlüssel wird bei Drop automatisch zeroisiert (`ZeroizeOnDrop`). | `src/ed25519_proof.rs` |
| `DeletionProofKeyPair` | `{ signing_key: ed25519_dalek::SigningKey, /// Öffentlicher Verifikationsschlüssel. pub verifying_key: ed25519_dalek::VerifyingKey, }` | KeyPair for Ed25519 signing and verification of DeletionProofs (version 3). | `src/deletion_proof.rs` |
| `DerivedReportSigner` | `{ key: SigningKey, signer_id: SignerId, }` | Implementierung des `ReportSigner`-Ports für intern abgeleitete Schlüssel (signer_id = 0, D-130). | `src/report/signing.rs` |
| `Destroyed` | `{ /// Art der erfolgten Verifikation am Medium. pub verified_via: VerifiedVia, }` | Nachweis einer erfolgreichen Slot-Vernichtung (SPEC D-111, D-113). | `src/vault/table.rs` |
| `DocId` | `(tuple/unit)` | — | `src/types_local.rs` |
| `DocKey` | `(tuple/unit)` | Dokumentenschlüssel (SPEC 7.2). | `src/erasure_domain.rs` |
| `DocRef` | `{ /// Optionaler Betroffener. pub subject: Option<SubjectRef>, /// Eindeutiger Dokumentenschlüssel. pub key: DocKey, }` | Eindeutige Dokumentenadresse. | `src/erasure_domain.rs` |
| `DocumentHierarchyTags` | `{ /// Mandanten-Tag (T-Slot). pub tenant_tag: [u8; 16], /// Collection-Tag (C-Slot). pub coll_tag: [u8; 16], /// Betroffenen-Tag (S-Slot, optional). pub subject_… }` | Kontext für Hierarchie-Tags eines Dokuments (T, C, S, D). | `src/vault/sealer.rs` |
| `DurableJournal` | `{ vfs: Arc<dyn Vfs>, path: PathBuf, derived_key: [u8; 32], records: Vec<JournalRecord>, chain_tail: [u8; 32], }` | Dauerhaftes Journal auf Basis von Vfs (SPEC §11.6, Anhang F.1, F.9). | `src/journal/appendlog.rs` |
| `DurableVaultShard` | `{ file: Box<dyn VfsFile>, path: PathBuf, shard_no: u8, cap_log2: u8, capacity: usize, live: u64, tombstones: u64, generation: u64, rsv_cursor: u64, vault_id: [u8… }` | Dauerhafter Tresor-Shard über VFS. | `src/vault/durable.rs` |
| `EncryptedEpochRecordPayload` | `{ /// Numerische ID des Datensatzes. pub record_id: u64, /// Shredding-Gruppen-ID, zu der der Datensatz gehört. pub group_id: u64, /// Epoche, zu der der KEK geh… }` | Serialisierbarer Container für einen mit Envelope Encryption und Epochenbindung geschützten Datensatz. | `src/kv_shredding.rs` |
| `EncryptedRecordPayload` | `{ /// Numerische ID des Datensatzes. pub record_id: u64, /// Shredding-Gruppen-ID, zu der der Datensatz gehört. pub group_id: u64, /// Mit AES-256-GCM-SIV versch… }` | Serialisierbarer Container für einen mit Envelope Encryption geschützten Datensatz. | `src/kv_shredding.rs` |
| `Envelope` | `{ payload: Vec<u8>, }` | Versiegelter Datensatz-Umschlag. | `src/types_local.rs` |
| `Epoch` | `{ /// Epochenindex (monoton steigend). pub index: u32, /// Zeitstempel des Epochenbeginns (Mikrosekunden UTC). pub started_at: u64, }` | Zustand einer Journal-Epoche. | `src/journal/epoch.rs` |
| `EraseExecutionRequest` | `{ /// Eindeutige ID (oder abgeleitet). pub erasure_id: ErasureId, /// Ziel des Löschvorgangs. pub target: EraseTarget, /// Modus (Erase oder Restrict). pub mode:… }` | Antrag an den Löschkern. | `src/saga/core.rs` |
| `EraseReceipt` | `{ /// Eindeutige Kennung. pub id: ErasureId, /// Status. pub status: ErasureStatus, /// Zeitstempel der Sperrung. pub gated_at: Timestamp, }` | Löschquittung. | `src/types_local.rs` |
| `EraseReceipt` | `{ /// Eindeutige Kennung des Löschvorgangs. pub id: ErasureId, /// Status des Vorgangs. pub status: ErasureStatus, /// Zeitstempel der Sperrung im Gate. pub gate… }` | — | `src/erasure_domain.rs` |
| `EraseRequest` | `{ /// Löschziel. pub target: EraseTarget, /// Timing (sofort oder gebündelt). pub timing: Timing, /// Aufbewahrungs-Attestierung. pub retention: RetentionAttesta… }` | — | `src/erasure_domain.rs` |
| `EraseRequest` | `{ /// Ziel der Löschung pub target: PortEraseTarget, /// Eindeutige Lösch-ID pub erasure_id: [u8; 16], }` | Anfrage zur Löschung (Kompatibilität mit Ports & Tests). | `src/saga/machine.rs` |
| `EraseSaga` | `{ /// Eindeutige Kennung. erasure_id: ErasureId, /// Aktueller Zustand. state: SagaState, /// Letzter Pending-Grund (falls vorhanden). pending_reason: Option<Pen… }` | Zustandsmaschine für einen einzelnen Löschvorgang. | `src/saga/machine.rs` |
| `ErasedSet` | `{ gate_epoch: AtomicU64, erased: Vec<GateEntry>, restricted: Vec<GateEntry>, prefix_seq: Vec<PrefixSeqEntry>, }` | Sortierte Bereichsmenge aktiver Gates (ER-11, D-120). | `src/gate/erased_set.rs` |
| `ErasureConfig` | `{ /// Maximale Dauer bis zum Erreichen von Gated oder Pending. pub erase_return_timeout: Duration, /// SLA-Frist für die gesamte Löschung. pub erase_sla: Duratio… }` | Konfiguration für den Löschkern (SPEC §4, §11.2). | `src/saga/core.rs` |
| `ErasureCoordinator` | `{ single_flight: crate::scope::SingleFlightCoordinator, }` | Kanonischer einheitlicher Koordinator der Löschkette (INV-ERA-1 / WP-10). Stellt sicher, dass es genau einen koordinierten Löschpfad im Crate gibt. | `src/orchestrator.rs` |
| `ErasureCore` | `{ cfg: ErasureConfig, deps: Arc<ErasureDeps>, active_sagas: Arc<RwLock<BTreeMap<ErasureId, ActiveEntry>>>, open_keys: Arc<RwLock<BTreeMap<[u8; 32], ErasureId>>>, }` | Haupt-Orchestrator des Löschkerns (SPEC §11.2, AUFTRAG J-07). | `src/saga/core.rs` |
| `ErasureCoreStatus` | `{ /// Die ID. pub id: ErasureId, /// Der aktuelle Zustand. pub state: SagaState, /// Detaillierter Status gemäß SPEC §7.4. pub status: ErasureStatus, /// Gated-S… }` | Status eines laufenden oder abgeschlossenen Löschauftrags. | `src/saga/core.rs` |
| `ErasureDeps` | `{ /// Intent-Journal Naht. pub journal: Arc<dyn SagaJournal>, /// Gate Naht. pub gate: Arc<dyn SagaGate>, /// Schlüsseltresor Naht. pub vault: Arc<dyn SagaVault>… }` | Abhängigkeiten des Löschkerns (SPEC §4 Schnittstellen, Ports & Seams). | `src/saga/core.rs` |
| `ErasureEvent` | `{ /// ID der Löschoperation. pub id: ErasureId, /// Art des Ziels. pub target_kind: TargetKind, /// Erreichte Phase. pub phase: ObservedPhase, /// Zeitstempel. p… }` | Ereignis für den ErasureObserver. | `src/types_local.rs` |
| `ErasureFailure` | `{ /// Layer where verification or purging failed. pub layer: String, /// Detailed failure explanation. pub reason: String, }` | Detailed failure reason when erasure verification fails. | `src/orchestrator.rs` |
| `ErasureId` | `(tuple/unit)` | 16-Byte Lösch-Identifikator (SPEC §7.4, TY-09). | `src/types_local.rs` |
| `ErasureJournal` | `{ entries: Vec<JournalEntry>, }` | In-Memory- und Disk-Abstraktion des Journals (Appendlog nach F.9). | `src/journal.rs` |
| `ErasureLedger` | `{ vfs: Arc<dyn Vfs>, path: PathBuf, ledger_key: [u8; 32], entries: RwLock<Vec<LedgerEntry>>, last_hash: RwLock<[u8; 32]>, max_gated_seq: RwLock<u64>, }` | Persistentes, fehlertolerantes und fail-closed Ledger für Erasure-Nachweise. | `src/ledger/persistent.rs` |
| `ErasureLedger` | `{ storage: Box<dyn LedgerStorage>, ledger_key: [u8; 32], last_hash: [u8; 32], max_gated_seq: u64, records: Vec<FramedRecord>, }` | Persistentes, hash-verkettetes Ledger (WP-8, CTRALEDG v1). | `src/ledger/durable.rs` |
| `ErasureReport` | `{ /// Gibt an, ob alle tatsächlich gemessenen Schichten verifizierbar ohne Restbestände bereinigt wurden. pub verified: bool, /// Cryptographic DeletionProof iss… }` | Comprehensive erasure verification report. | `src/orchestrator.rs` |
| `ErasureReport` | `{ /// Format-Version des Berichts (immer 1 für v1.0.0). pub report_version: u32, /// Store-ID (Hexadezimal, kein Klartext). pub store_id: String, /// Lösch-ID (H… }` | Kanonischer Löschbericht (ER-27, SPEC §11.7, Anhang D). Keine Inhalte, Schlüssel oder Klartext-Kennungen (P13). | `src/report/build.rs` |
| `FencePermit` | `{ _token: Option<Arc<()>>, }` | RAII-Token für einen aktiven Lese-/Schreib-Fence (INV-ERA-5). | `src/types_local.rs` |
| `FenceToken` | `(tuple/unit)` | Eindeutiger Autorisierungs-Token für einen aktiven Fence. Nur der Inhaber dieses Tokens ist berechtigt, den Fence über [`FenceGate::release_fence`] a… | `src/gate/fence.rs` |
| `FieldMeasurementSection` | `{ /// MK1 Status. pub mk1: ReachStatus, /// MK2 Status. pub mk2: ReachStatus, /// MK3 Status. pub mk3: ReachStatus, }` | Feldmessungs-Abschnitt (MK1, MK2, MK3) für Abwärtskompatibilität. | `src/report/build.rs` |
| `FileId` | `(tuple/unit)` | Eindeutige Dateikennung. | `src/types_local.rs` |
| `FileKey` | `{ raw: Vec<u8>, file_id: FileId, }` | Flüchtiger Dateischlüssel mit Zeroize-on-Drop. | `src/types_local.rs` |
| `FileLedgerStorage` | `{ path: PathBuf, }` | Dateibasierte Implementierung von `LedgerStorage` im sicheren Dateisystem. | `src/ledger/storage.rs` |
| `FileRestoreChecker` | `{ ledger: &'a InMemoryLedger, }` | Dateibasierter Restore-Checker: prüft auf Zeitregression und Wiederauftauchen (ER-30). | `src/restore/guard.rs` |
| `FileVault` | `{ shards: Vec<ShardBackend>, vault_id: [u8; 16], mac_key: [u8; 32], vfs: Option<Arc<dyn crate::types_local::Vfs>>, vault_dir: Option<PathBuf>, }` | 16-Shard Tresor nach SPEC D-111. | `src/vault.rs` |
| `FinalizeOutcome` | `{ /// Der signierte und gespeicherte Bericht. pub report: ErasureReport, /// Pfad zum gespeicherten Bericht. pub report_path: PathBuf, /// Sequenznummer des Absc… }` | Ergebnis des Abschlusses. | `src/ledger.rs` |
| `FinalizeParams` | `{ /// Eindeutige Lösch-ID. pub erasure_id: ErasureId, /// Gate-Sequenznummer. pub gated_seq: u64, /// Store-Sequenz bei Abschluss. pub completed_seq: u64, /// Ze… }` | Parameter für den Abschluss eines Löschvorgangs (SPEC §11.8). | `src/ledger.rs` |
| `FramedRecord` | `{ /// Typ des Records pub record_type: RecordType, /// Sequenznummer (gated_seq) pub seq: u64, /// Zeitstempel (completed_at bzw. creation in Mikrosekunden) pub… }` | Strukturierter Record im CTRALEDG-Format (Framing). | `src/ledger/record.rs` |
| `GateBatchCoordinator` | `{ barrier: Arc<dyn CommitBarrier>, erased_set: Arc<RwLock<ErasedSet>>, gate_log: Arc<RwLock<GateLog>>, }` | Kontext für die koordinierte Ausführung von Gate-Gruppen-Commits. | `src/gate/batch.rs` |
| `GateGuard` | `{ inner: Arc<RwLock<ErasedSet>>, }` | Thread-sichere Implementierung des `ReadGate`-Ports (ER-11, D-120). | `src/gate/read_gate.rs` |
| `GateLog` | `{ entries: Vec<GateLogEntry>, }` | In-Memory- und Disk-Append-Log für Gates (SPEC §11.3, D-58). | `src/gate/log.rs` |
| `GraphRepairAttestation` | `{ /// Document ID whose neighborhood pointers were repaired. pub doc_id: DocId, /// Verified confirmation that no ghost pointers remain. pub verified_no_ghost_po… }` | Attestation confirming synchronous neighborhood graph repair for vector deletions. | `src/deletion_proof.rs` |
| `GroupEpochKey` | `{ /// Gruppen- bzw. Mandanten-ID. pub group_id: u64, /// Epochen-Nummer für Rotation und isolierte Schlüsselvernichtung. pub epoch: u64, }` | Thread-sichere Schlüssel-Registry für Envelope Encryption und O(1) Krypto-Shredding. Schlüsselbereich für Gruppen und Epochen (Entscheidung E5). | `src/kv_shredding.rs` |
| `GroupEpochKey` | `{ /// Gruppen- bzw. Mandanten-ID. pub group_id: u64, /// Epochen-Nummer für Rotation und isolierte Schlüsselvernichtung. pub epoch: u64, }` | Schlüsselbereich für Gruppen und Epochen (Entscheidung E5), falls Feature vault nicht aktiv ist. | `src/vault/key_store.rs` |
| `GroupKek` | `(tuple/unit)` | Group Key Encryption Key (KEK) - 256 Bit, mit automatischer Nullung beim Verwerfen (`ZeroizeOnDrop`). | `src/kv_shredding.rs` |
| `InMemoryFenceGate` | `{ active_flag: AtomicBool, corrupted_flag: AtomicBool, inflight_writers: Arc<AtomicU32>, next_id: AtomicU64, entries: RwLock<Vec<ActiveFenceEntry>>, }` | In-Memory-Referenzimplementierung des [`FenceGate`]-Ports (Klasse A, autark, synchron). Arbeitet synchron ohne externe Runtimes oder Threads. | `src/gate/fence.rs` |
| `InMemoryLedger` | `{ entries: Arc<RwLock<Vec<LedgerEntry>>>, }` | In-Memory-Implementierung von `LedgerSink` (für Ports-Kompatibilität). | `src/ledger.rs` |
| `InMemoryLedger` | `{ entries: Vec<LedgerEntry>, ledger_key: [u8; 32], last_hash: [u8; 32], /// Höchste gesehene gated_seq. max_gated_seq: u64, /// Optionales zugrundeliegendes Eras… }` | In-Memory-Ledger-Implementierung (für Tests und WP-4.3 als Basis). Ausgebaut um optionales persistentes Backend (`ErasureLedger`), behält aber alle b… | `src/ledger/sink.rs` |
| `InMemoryVectorIndexStub` | `{ layer: LayerId, active_docs: std::collections::BTreeSet<u64>, backlinks: std::collections::BTreeMap<u64, Vec<u64>>, quantized_entries: std::collections::BTreeS… }` | In-Memory Testdouble für Indexstrukturen zur Verifikation der ResidueScanner-Logik. | `src/measure/residue.rs` |
| `InMemoryWalEntry` | `{ /// ID des Eintrags. pub id: WalEntryId, /// Zugehöriger Scope. pub scope: WalScope, /// Nutzdaten / Klartext. pub payload: Vec<u8>, /// Ob der Eintrag physisc… }` | In-Memory-Eintrag für die Test- und Referenz-Implementierung. | `src/journal/wal_sink.rs` |
| `InMemoryWalSink` | `{ entries: Mutex<BTreeMap<WalEntryId, InMemoryWalEntry>>, integrity_keys: Mutex<BTreeMap<[u8; 16], [u8; 32]>>, disposed_scopes: Mutex<BTreeSet<[u8; 16]>>, fail_o… }` | Referenzimplementierung des `WalSink`-Ports im Speicher für Tests und Verifikation. | `src/journal/wal_sink.rs` |
| `JournalEntry` | `{ /// Lösch-ID. pub erasure_id: [u8; 16], /// Saga-Zustand. pub state: SagaState, /// Zeitstempel (Mikrosekunden UTC). pub timestamp: u64, /// Commitment-Hash (B… }` | Ein abwärtskompatibler Journal-Eintrag (SPEC §11.6, ER-29). | `src/journal/record.rs` |
| `JournalRecord` | `{ /// Lösch-Kennung. pub erasure_id: [u8; 16], /// Record-Code nach F.9. pub record_code: RecordCode, /// Zeitstempel in Mikrosekunden UTC. pub timestamp: u64, /… }` | Vollständiger Journal-Eintrag nach SPEC Anhang F.9. Klartext: `erasure_id(16) || record_code(u8) || time(u64) || commitment(16) || target_type(u8) ||… | `src/journal/record.rs` |
| `KeyBuf` | `(tuple/unit)` | Fester 16-Byte Schlüsselpuffer für Speicher- und Gate-Operationen (SPEC §8.1). | `src/types_local.rs` |
| `KeyPurgeReceipt` | `{ /// Bereinigte Gruppen-ID. pub group_id: u64, /// Bereinigte Epoche (Entscheidung E5). pub epoch: u64, /// Anzahl physisch genullter DEK-Schlüssel-Handles dies… }` | Vernichtungsergebnis nach `purge_keys` zur Verankerung im Ledger durch den Orchestrator. Herkunft: ffeb79555:crates/acta-crypto/src/scope_keyring.rs:… | `src/kv_shredding.rs` |
| `KeyPurgeReceipt` | `{ /// Bereinigte Gruppen-ID. pub group_id: u64, /// Bereinigte Epoche (Entscheidung E5). pub epoch: u64, /// Anzahl physisch genullter DEK-Schlüssel-Handles dies… }` | Vernichtungsergebnis nach `purge_keys` zur Verankerung im Ledger durch den Orchestrator. Herkunft: ffeb79555:crates/acta-crypto/src/scope_keyring.rs:… | `src/vault/key_store.rs` |
| `KeyRange` | `{ /// Untere Grenze (inklusiv). pub start: KeyBuf, /// Obere Grenze (exklusiv). pub end: KeyBuf, }` | Halboffenes Schlüsselintervall `[start, end)`. | `src/types_local.rs` |
| `KeyRegistry` | `{ groups: RwLock<BTreeMap<u64, GroupEntry>>, epoch_groups: RwLock<BTreeMap<(u64, u64), GroupEntry>>, group_keks: RwLock<BTreeMap<u64, Vec<u8>>>, record_deks: RwL… }` | Thread-sichere Schlüssel-Registry für Envelope Encryption und O(1) Krypto-Shredding. | `src/kv_shredding.rs` |
| `LayerCleanupProof` | `{ layer: DeletionLayer, _private: (), }` | Beweis, dass ein bestimmter DeletionLayer physisch bereinigt wurde. Kann NUR von den jeweiligen Bereinigungsfunktionen der Storage-Layer erzeugt werd… | `src/deletion_proof.rs` |
| `LayerId` | `(tuple/unit)` | Schicht-Kennung. | `src/types_local.rs` |
| `LayerId` | `(tuple/unit)` | — | `src/erasure_domain.rs` |
| `LayerKeyCoverage` | `{ /// Schicht-ID (hex, kein Klartext). pub layer_id_hex: String, /// Abdeckungsgrad ("full", "unlinked", "none", "nicht erfasst"). pub coverage: String, }` | Schlüsselabdeckung einer Schicht im Bericht. | `src/report/build.rs` |
| `LayerPurgeOrder` | `{ /// Schicht-ID (16 Bytes). pub layer_id: [u8; 16], /// Priorität. pub priority: PurgePriority, }` | Zuordnung einer Schicht zu ihrer Purge-Priorität. | `src/purge/order.rs` |
| `LayerScope` | `{ scope_id: [u8; 16], affected_units: Vec<u64>, }` | Schicht-Geltungsbereich vor dem Gating. | `src/types_local.rs` |
| `LayerScopeSet` | `{ /// Scopes je Schicht. pub scopes: Vec<ResolvedLayerScope>, }` | Gesamtmenge der Scope-Auflösungen aller Schichten für eine Löschung. | `src/scope/set.rs` |
| `LayerSummary` | `{ /// Layer name. pub layer: String, /// Applied scanning technique. pub technique: String, /// Number of units scanned. pub scanned: u64, /// Number of remainin… }` | Summary of a single storage layer's scan result. | `src/orchestrator.rs` |
| `LayerWitness` | `{ layer: LayerId, class: MeasureClass, method_id: u16, files: u64, bytes: u64, duration: Duration, witness_hash: [u8; 32], }` | Zeuge einer überprüften physischen Löschung. | `src/types_local.rs` |
| `LayerWitness` | `{ layer: LayerId, class: MeasureClass, method_id: u16, files: u64, bytes: u64, duration: Duration, hash: [u8; 32], }` | — | `src/erasure_domain.rs` |
| `LayerWitnessMeasurement` | `{ /// Layer that was measured. pub layer: DeletionLayer, /// Applied scan technique. #[serde(with = "scan_technique_serde")] pub technique: ScanTechnique, /// Nu… }` | Measurement details for a single witnessed layer in V5 proof. | `src/deletion_proof.rs` |
| `LayerWitnessReport` | `{ /// Schichtbezeichner (z. B. "doc_store", "vector_payload", "text_postings"). pub layer: String, /// Messklasse ("MK1", "MK2", "MK3"). pub class: String, /// M… }` | Strukturierter Zeugeneintrag für eine Schicht (SPEC §11.7, §12.2). | `src/report/build.rs` |
| `LedgerAnchorProof` | `{ /// Höchste Sequenznummer nach diesem Batch. pub last_seq: u64, /// Aktueller Ketten-Hash nach diesem Batch. pub chain_hash: [u8; 32], /// Anzahl der im Batch… }` | Bestätigung für einen atomaren Batch-Append als Verankerungsnachweis (WP-8, INV-ERA-4). | `src/ledger/durable.rs` |
| `LedgerConfirmation` | `{ /// Sequenznummer des Ledger-Eintrags. pub seq: u64, /// Hexadezimaler Kettenhash des Ledger-Eintrags. pub chain_hash: String, }` | Ledger-Verankerungsbestätigung für den Löschbericht (SPEC §11.8, ER-30). Opaker lokaler Typ, der die Bestätigung der Ledger-Verankerung festhält, ohn… | `src/report/build.rs` |
| `LedgerEntry` | `{ /// Commitment-Hash der Löschdefinition. pub commitment: [u8; 32], /// Abgedeckte Schlüsselbereiche. pub range_set: RangeSet, /// Gate-Schließsequenz. pub gate… }` | Revisionssicherer Ledger-Eintrag. | `src/types_local.rs` |
| `LedgerEntry` | `{ /// Commitment (BLAKE3-keyed über Ziel + Mandant). pub commitment: [u8; 32], /// Bereich-Tags (opak, kein Klartext). pub range_tags: Vec<u8>, /// Gate-Sequenzn… }` | Ein Ledger-Eintrag (SPEC §11.8, ER-30). | `src/ledger/entry.rs` |
| `LedgerFileHeader` | `{ /// Magic-Bytes pub magic: [u8; 8], /// Versionsnummer pub version: u32, /// Reservierte Flags pub flags: u32, }` | Dateikopf (Header) eines CTRALEDG v1 Ledgers (16 Bytes). | `src/ledger/record.rs` |
| `LocalControlEvidence` | `{ #[allow(deprecated)] kind: EvidenceKind, }` | Nachweis über erfolgreiche Positivkontrolle für Test-Scanner. | `src/measure.rs` |
| `LocalDek` | `(tuple/unit)` | Stark typisierter Daten-Schlüssel (DEK) mit automatischer Nullung im Speicher. Herkunft: 1364a54d5:crates/acta-crypto/src/hierarchy.rs:16 (MasterKey) | `src/vault/key_store.rs` |
| `ManifestEntry` | `{ /// Target-ID oder Blob-ID. pub target_id: String, /// Ob das Blob aktiv oder durch Rewrite genullt/bereinigt ist. pub is_erased: bool, /// Letzter verankerter… }` | Eintrag im Tresor-Manifest. | `src/vault/sealer.rs` |
| `MeasurementCoordinator` | `{ _private: (), }` | Koordinator für Feldmessungen (MK1, MK2, MK3, ER-26). | `src/measure.rs` |
| `MemLedgerStorage` | `{ data: Arc<RwLock<Vec<u8>>>, }` | In-Memory-Implementierung von `LedgerStorage` für Unit-Tests und In-Memory-Szenarien. | `src/ledger/storage.rs` |
| `MemoryVfsFile` | `{ data: std::sync::Arc<parking_lot::RwLock<Vec<u8>>>, vfs_raw: std::sync::Arc<dyn crate::types_local::Vfs>, path: std::path::PathBuf, }` | In-Memory-VfsFile Adapter für In-Memory Vfs. | `src/types_local.rs` |
| `MemoryWalSink` | `{ purged_scopes: RwLock<BTreeSet<Vec<u8>>>, erasure_markers: RwLock<Vec<(ErasureId, Vec<u8>)>>, }` | In-Memory Referenzimplementierung von [`WalSink`]. | `src/journal/wal.rs` |
| `MetablockParams` | `{ /// 16-Byte Tresor-ID. pub vault_id: &'a [u8; 16], /// Shard-Nummer (0..15). pub shard_no: u8, /// Zweierlogarithmus der Shard-Kapazität. pub cap_log2: u8, ///… }` | Parameter für einen Metablock (64 Bytes) nach SPEC F.3. | `src/vault/open.rs` |
| `Mk1Result` | `{ /// Anzahl geprüfter Strukturen. pub checked_structures: u64, /// Treffer (sollte 0 sein). pub matches: u64, /// Verifikationsmethode (D-113). pub verified_via… }` | Ergebnis einer MK1-Feldmessung (logische Prüfung). | `src/measure/field.rs` |
| `Mk2Result` | `{ /// Anzahl geprüfter Dateien. pub checked_files: u64, /// Geprüfte Bytes. pub checked_bytes: u64, /// Treffer (sollte 0 sein). pub matches: u64, /// Verifikati… }` | Ergebnis einer MK2-Feldmessung (Rohscan). | `src/measure/field.rs` |
| `Mk3Result` | `{ /// Slot der höchsten Ebene genullt. pub top_slot_zeroed: bool, /// DEK-Ableitung gescheitert. pub dek_derivation_fails: bool, /// F-Slots der Zieldateien nich… }` | Ergebnis einer MK3-Feldmessung (Schlüsselabwesenheit). | `src/measure/field.rs` |
| `Mk3StatusInput` | `{ /// Slot der höchsten Ebene genullt. pub top_slot_zeroed: bool, /// DEK-Ableitung gescheitert. pub dek_derivation_fails: bool, /// F-Slots der Zieldateien nich… }` | Eingabewerte für MK3-Schlüsselprüfung. | `src/measure/field.rs` |
| `MockErasureTarget` | `{ /// Eindeutige ID des Targets. pub id: String, /// Art des Löschziels. pub kind: &'static str, }` | Mock für ein konkretes ErasureTarget. | `src/test_mocks.rs` |
| `MockStorageBackend` | `{ layer: LayerId, purge_count: Arc<AtomicUsize>, }` | In-Memory Mock für ein generisches Storage-Backend. | `src/test_mocks.rs` |
| `OperatorAttestation` | `{ operator: String, case_commitment: [u8; 32], attested_at: u64, }` | Attestierung des beauftragten Betreibers für Art. 18 Abs. 2 Zugriff (SPEC D-116). | `src/restricted.rs` |
| `OperatorRef` | `(tuple/unit)` | Opake Betreiberkennung für Attestierungen (SPEC §7.3). | `src/types_local.rs` |
| `PrefixSeqMatcher` | `{ #[serde(with = "prefix_serde")] prefix: [u8; 65], seq_lt: u64, }` | Matcher für das Prädikat `PrefixSeq` (SPEC D-117, F.9). | `src/supersede.rs` |
| `ProcessHardeningStatus` | `{ /// Verschlüsselung erzwungen (immer true). pub encryption_mandatory: bool, /// WAL-Integrität erzwungen (immer true). pub wal_integrity_mandatory: bool, }` | Prozesshärtungsstatus (SPEC §11.7, Pflichtfeld). | `src/report/build.rs` |
| `ProtectedDek` | `{ bytes: [u8; 32], }` | Sicher geschützter 32-Byte DEK-Puffer mit automatischem Zeroize-on-Drop. | `src/vault/sealer.rs` |
| `PurgeCx` | `{ /// Deadline in Millisekunden. pub deadline_ms: u64, }` | Bereinigungs-Kontext mit Deadline. | `src/types_local.rs` |
| `PurgeCx` | `{ /// Deadline für das Purge-Fenster in Monotonic Nanos. pub deadline_nanos: Option<u64>, /// Mindest-freier Speicherplatz (Bytes) vor dem Rewrite (ER-22). pub m… }` | Ausführungskontext eines Purge-Fensters (ER-20, ER-22). | `src/purge/cx.rs` |
| `PurgeOutcome` | `{ /// Bereinigte Elemente. pub purged_count: u64, /// Freigegebene Bytes. pub bytes_reclaimed: u64, }` | Bereinigungs-Ergebnis. | `src/types_local.rs` |
| `PurgePermit` | `{ _token: Option<Arc<()>>, }` | RAII-Token für exklusive Purge-Erlaubnis. | `src/types_local.rs` |
| `PurgePriority` | `(tuple/unit)` | Priorität einer Schicht beim Purge (niedrigere Zahl = früher ausgeführt). | `src/purge/order.rs` |
| `PurgeProgressTracker` | `{ /// Eindeutige Kennung des Purge-Vorgangs. pub purge_id: [u8; 16], /// Aktuell erreichte und erfolgreich verifizierte Phase. pub current_phase: PurgePhase, ///… }` | Persistierbarer Fortschritts-Zustand eines Purge-Laufes für Wiederanlauf (P9). | `src/purge/resume.rs` |
| `PurgeStats` | `{ /// Anzahl physisch bereinigter Einheiten. pub purged_units: u64, /// Anzahl freigegebener Bytes. pub bytes_freed: u64, }` | Statistiken über bereinigte Einheiten. | `src/types_local.rs` |
| `RangeSet` | `{ ranges: Vec<(KeyBuf, KeyBuf)>, }` | Intervall-Menge für Ledger-Einträge. | `src/types_local.rs` |
| `RecordDek` | `(tuple/unit)` | Record Data Encryption Key (DEK) - 256 Bit, mit automatischer Nullung beim Verwerfen (`ZeroizeOnDrop`). | `src/kv_shredding.rs` |
| `RehashOutcome` | `{ /// Neue Shard-Kapazität. pub new_capacity: usize, /// Anzahl kopierter aktiver Slots. pub slots_copied: usize, /// Ob die Altdatei vollständig mit Nullen über… }` | Ergebnis einer Rehash-Operation (D-111). | `src/vault/rehash.rs` |
| `ReplacePlan` | `{ erasure_id: ErasureId, old_doc_tag: [u8; 16], new_doc_tag: [u8; 16], step: ReplaceStep, }` | Plan und Zustandsverwaltung für die Ersetzung eines Mehr-Betroffenen-Dokuments (SPEC D-115). | `src/replace.rs` |
| `ReplayOutcome` | `{ /// Anzahl wiederholter Löschungen. pub replayed_count: u64, /// Ob der Store nach dem Replay freigegeben werden kann. pub store_cleared: bool, }` | Ergebnis eines Replay-Vorgangs. | `src/restore/replay.rs` |
| `ReportBuildOptions` | `{ /// Store-ID (Hex). pub store_id: String, /// Zieltyp-Name. pub target_type: String, /// Commitment-Hex. pub commitment: String, /// Anfragezeitstempel (Mikros… }` | Optionen für die Berichterstellung. | `src/report/build.rs` |
| `ReportSignature` | `{ /// Signer ID (0 = abgeleitet, 1 = extern/Betreiber). pub signer_id: SignerId, /// Base64url-kodierte Signatur. pub signature_base64: String, /// SHA-256 Finge… }` | Signatur eines DeletionReports (SPEC F.10). | `src/types_local.rs` |
| `ReportSigner` | `(tuple/unit)` | Statische Hilfsfunktionen für Berichtssignierung und VFS-Persistierung (Abwärtskompatibilität). | `src/report/signing.rs` |
| `RequestContext` | `{ /// Tenant identifier for authorization and scope isolation. pub tenant_id: TenantId, /// Transaction ID allocated for the operation. pub tx_id: TxId, }` | RequestContext für Löschoperationen. | `src/types_local.rs` |
| `ResidualRiskContext` | `{ /// Physischer Purge vollständig abgeschlossen. pub purge_complete: bool, /// Medium ist rotierend (z. B. HDD). Wenn false oder unbekannt: FlashRemanence. pub… }` | Kontext zur Auswertung der dynamischen Bedingungen für Restrisiken (SPEC Anhang D). | `src/report/risk.rs` |
| `ResidualRiskEntry` | `{ /// Token nach SPEC Anhang D (z. B. "RISK-FLASH-REMANENCE"). pub id: String, /// Name / Bezeichner (z. B. "FlashRemanence"). pub name: String, /// Kurztext / B… }` | Serialisiert ein Restrisiko als maschinenlesbaren Eintrag. | `src/report/risk.rs` |
| `Residue` | `{ /// Anzahl gefundener Rückstände. pub matches: u64, }` | Rückstandsbefund bei fehlgeschlagener Verifikation. | `src/types_local.rs` |
| `Residue` | `{ /// Anzahl gefundener Treffer / Rückstände. pub matches: u64, }` | — | `src/erasure_domain.rs` |
| `ResidueFinding` | `{ /// Betroffene Schicht. pub layer: LayerId, /// Strukturelle Klasse des Funds. pub offset_class: OffsetClass, /// Anzahl der Fundstellen in dieser Strukturklas… }` | Fundstelle einer Restspur im Index. Enthält niemals vertrauliche Rohdaten oder Klartext (Datensparsamkeit). | `src/measure/residue.rs` |
| `ResidueMeasurementCoordinator` | `{ scanner: &'a dyn ResidueScanner, clock: &'a dyn Clock, }` | Koordiniert den normativen Messablauf für Index-Schichten. Ablauf: 1. Positivkontrolle: Canary-Prüfung (`canary_probe`). Scheitert dies -> `PositiveC… | `src/measure/residue.rs` |
| `ResidueMeasurementReport` | `{ /// Vor-Löschung Trefferanzahl (Bestätigung der Präsenz). pub pre_erase_matches: u64, /// Nach-Löschung Trefferanzahl (muss zwingend 0 sein). pub post_erase_ma… }` | Statusbericht eines vollständigen Restspuren-Messablaufs. | `src/measure/residue.rs` |
| `ResidueQuery` | `{ /// Dokument-ID des zu untersuchenden Datensatzes. pub doc_id: DocId, /// Fingerabdruck des Werts oder Zielobjekts (z. B. BLAKE3-Hash). pub target_fingerprint:… }` | Zielanfrage für einen Restspuren-Scan. | `src/measure/residue.rs` |
| `ResidueScanResult` | `{ /// Strukturierte Liste der Befunde. pub findings: Vec<ResidueFinding>, /// Gesamtanzahl der detektierten Treffer (saturating sum). pub total_matches: u64, ///… }` | Gesamtergebnis eines Restspuren-Scans. | `src/measure/residue.rs` |
| `ResidueWitnessDigest` | `{ /// Betroffene Schicht. pub layer: LayerId, /// Gesamtzeit der Verifikation. pub duration: Duration, /// Untersuchte Einheiten. pub units_checked: u64, /// BLA… }` | Kryptographischer Zeugen-Digest einer erfolgreichen Messung. Kann an `acta-erasure::report` oder `acta-crypto` übergeben werden. | `src/measure/residue.rs` |
| `ResolvedLayerScope` | `{ /// Schicht-ID. pub layer_id: ScopeLayerId, /// Anzahl betroffener Einheiten (Dateien, Segmente, etc.). pub unit_count: u64, /// Opake Einheitsliste (Schicht-s… }` | Auflösung einer einzelnen Schicht für eine Löschung (ER-17). | `src/scope/set.rs` |
| `RestoreGuard` | `{ ledger_floor: Seq, }` | Restore-Guard prüft Sequenznummern gegen den Ledger-Floor. | `src/restore.rs` |
| `RestrictedAccess` | `{ purpose: RestrictedPurpose, attestation: OperatorAttestation, }` | Autorisierungskontext für den Zugriff auf eingeschränkte Daten (SPEC D-116). | `src/restricted.rs` |
| `RestrictedReadAudit` | `{ /// Zugehörige Löschungs-/Restriktions-ID. pub erasure_id: ErasureId, /// Record-Code nach F.9 (Code 9). pub record_code: u8, /// Rechtsgrund / Zweck. pub purp… }` | Audit-Datensatz für Journal Code 9 `RestrictedRead` (SPEC D-116, F.9). | `src/restricted.rs` |
| `RetentionAttestation` | `{ /// Code. pub code: RetentionCode, /// Betreiberkennung. pub attested_by: OperatorRef, /// Zeitstempel. pub attested_at: Timestamp, }` | Attestierung zur Aufbewahrung. | `src/types_local.rs` |
| `RetentionAttestation` | `{ /// Aufbewahrungscode. pub code: RetentionCode, /// Opake Betreiberkennung (nicht im Klartext im Bericht). pub attested_by: OperatorRef, /// Zeitstempel der At… }` | — | `src/erasure_domain.rs` |
| `RevocationEntry` | `{ /// Monotone Sequenznummer im Log (0, 1, 2, ...). pub sequence_number: u64, /// Das widerrufene Target. pub target: RevocationTarget, /// Widerrufszeitpunkt in… }` | Ein einzelner Eintrag im manipulationssicheren Widerrufslog. | `src/revocation_log.rs` |
| `RevocationLog` | `{ storage: Option<(Arc<dyn Vfs>, PathBuf)>, signing_key: Option<SigningKey>, verifying_key: VerifyingKey, clock: Arc<dyn Clock>, entries: RwLock<Vec<RevocationEn… }` | Persistentes Protokoll über widerrufene Schlüsselgruppen. | `src/revocation_log.rs` |
| `RewriteIntent` | `{ /// Eindeutige Rewrite-ID. pub rewrite_id: u64, /// Betroffenes Target / Scope. pub target_id: String, /// Start-Zeitstempel (Unix-Nanos). pub timestamp_nanos:… }` | Intent-Datenstruktur für einen Blob-Rewrite. | `src/vault/sealer.rs` |
| `RewriteJournal` | `{ entries: Arc<RwLock<BTreeMap<u64, RewriteIntent>>>, vfs: Option<Arc<dyn Vfs>>, journal_path: Option<std::path::PathBuf>, }` | In-Memory- und VFS-fähiges Journal für Rewrite-Transaktionen. | `src/vault/sealer.rs` |
| `RewriteManifest` | `{ version: Arc<RwLock<u64>>, entries: Arc<RwLock<BTreeMap<String, ManifestEntry>>>, vfs: Option<Arc<dyn Vfs>>, manifest_path: Option<std::path::PathBuf>, }` | Manifest des Tresors zur Verankerung aktiver und bereinigter Blobs. | `src/vault/sealer.rs` |
| `RewriteOutcome` | `{ /// Eindeutige Rewrite-ID. pub rewrite_id: u64, /// Betroffenes Target. pub target_id: String, /// Ob das Manifest erfolgreich aktualisiert wurde. pub manifest… }` | Ergebnis eines Blob-Rewrite-Vorgangs. | `src/vault/sealer.rs` |
| `SagaDriver` | `{ #[cfg(feature = "async")] core: Option<Arc<super::core::ErasureCore>>, }` | Treiber-Kontext für einen einzelnen Saga-Schritt. | `src/saga/driver.rs` |
| `SagaRecord` | `{ /// BLAKE3 hash of deleted keys. pub deleted_keys_hash: [u8; 32], }` | Saga record containing deletion tracking state and digests. | `src/types_local.rs` |
| `ScanPlan` | `{ /// ID der zu scannenden Schicht. pub layer: LayerId, /// Messklasse (MK1, MK2 oder MK3). pub class: MeasureClass, /// Numerische Methoden-ID. pub method_id: u… }` | Konfiguration für einen verifizierten Mess-Scan mit Positivkontrolle (SPEC 12.1, 12.2, D-72). | `src/measure/field.rs` |
| `ScanSession` | `{ layer: WitnessLayer, target: ScanTarget, technique: ScanTechnique, scanned: u64, remaining: u64, control_matches: u64, unit_hashes: Vec<[u8; 32]>, }` | Laufende Scan-Session zur kontrollierten Erzeugung eines `ScanWitness`. | `src/types_local.rs` |
| `ScanTarget` | `{ /// Bezeichner des Scan-Ziels (z. B. Tenant, Collection, Path). pub target_id: String, /// BLAKE3 Hash des Scan-Ziels. pub target_hash: [u8; 32], }` | Zielbeschreibung des Scans inklusive Hash. | `src/types_local.rs` |
| `ScanWitness` | `{ layer: WitnessLayer, scanned: u64, remaining: u64, technique: ScanTechnique, method: ScanMethod, digest: [u8; 32], measured_at: u64, }` | Verifiziertes Messergebnis eines Storage-Scanners (SPEC §12.7.4). | `src/types_local.rs` |
| `Scanner` | `{ layer: LayerId, class: MeasureClass, method_id: u16, files: u64, bytes: u64, _state: State, }` | Typisierter Scanner. | `src/types_local.rs` |
| `Scanner` | `{ layer: LayerId, class: MeasureClass, method_id: u16, files: u64, bytes: u64, _state: S, }` | — | `src/erasure_domain.rs` |
| `ScopeLockGuard` | `{ vfs: &'a dyn Vfs, path: PathBuf, record: ScopeLockRecord, }` | Single-Flight Guard für einen Scope (INV-ERA-3). Hält das Lock auf einen Scope solange aktiv, bis der Guard gedroppt oder explizit via [`ScopeLockGua… | `src/scope/single_flight.rs` |
| `ScopeLockManager` | `{ vfs: &'a dyn Vfs, clock: &'a dyn Clock, default_lease_nanos: u64, }` | Verwaltet Single-Flight Scope-Locks (INV-ERA-3). | `src/scope/single_flight.rs` |
| `ScopeLockRecord` | `{ /// Eindeutige Kennung des gesperrten Scopes (z. B. ErasureId). pub scope_id: [u8; 16], /// Eindeutige Kennung des Besitzers (z. B. Instanz / Transaktion). pub… }` | Zustand eines Scope-Locks auf Platte oder im VFS. | `src/scope/single_flight.rs` |
| `ScrubOutcome` | `{ /// Anzahl geprüfter Slots. pub slots_scanned: usize, /// Anzahl genullter F-Slots (Waisen). pub f_orphans_cleaned: usize, /// Anzahl genullter D-Slots (Waisen… }` | Ergebnis eines Scrub-Schritts. | `src/vault/scrub.rs` |
| `SealCtx` | `{ /// Epoche oder Slot-Identifikator. pub epoch: u64, }` | Versiegelungskontext. | `src/types_local.rs` |
| `SecretKey` | `{ bytes: [u8; N], }` | 32-Byte SecretKey mit Zeroize-on-Drop. | `src/crypto_local.rs` |
| `Seq` | `(tuple/unit)` | Streng monoton steigende Sequenznummer (SPEC §8.7, I-12). | `src/types_local.rs` |
| `SequentialIdGen` | `{ next: std::sync::atomic::AtomicU64, }` | A thread-safe sequential ID generator. | `src/types_local.rs` |
| `SignedReport` | `{ /// Kanonischer JSON-Inhalt ohne Signaturfelder bzw. vollständiger Berichtskörper. pub canonical_body: String, /// Base64url-Signatur ohne Padding. pub signatu… }` | Signierter Löschbericht mit Base64url-Signatur und Prüffingerabdruck (ER-27, SPEC §11.7, F.10). | `src/report/signing.rs` |
| `SingleFlightCoordinator` | `{ inner: Arc<SingleFlightCoordinatorInner>, }` | Koordinator zur Durchsetzung von INV-ERA-3 (Single-Flight je Scope). | `src/scope/flight.rs` |
| `SlotData` | `{ /// Slot-Typ. pub slot_type: SlotType, /// 16-Byte Tag. pub tag: [u8; 16], /// 32-Byte Payload (Salt oder maskierter Schlüssel). pub payload: [u8; 32], /// Gen… }` | Entpackter Slot-Inhalt (in-memory). | `src/vault/slot.rs` |
| `SubKey` | `(tuple/unit)` | Sub-Key bzw. KEK-Bytes (256 Bit) mit automatischer Nullung beim Verwerfen (`ZeroizeOnDrop`). | `src/kv_shredding.rs` |
| `SubjectRef` | `(tuple/unit)` | Betroffenen-Referenz (SPEC 7.2, 7.3). | `src/erasure_domain.rs` |
| `SupersedePlan` | `{ erasure_id: ErasureId, matcher: PrefixSeqMatcher, intent_seq: u64, mode: SupersedeMode, }` | Plan für die Berichtigung mit Purge alter Versionen (SPEC D-117). | `src/supersede.rs` |
| `SystemClock` | `{ start_instant: std::time::Instant, }` | Standard implementation of [`Clock`]. | `src/types_local.rs` |
| `TenantId` | `(tuple/unit)` | — | `src/types_local.rs` |
| `TenantRef` | `(tuple/unit)` | Mandanten-Referenz (SPEC 7.3). | `src/erasure_domain.rs` |
| `TestWitness` | `{ hash: [u8; 32], }` | Hilfstyp für Zeugen-Prüfung in Tests. | `src/measure.rs` |
| `Timestamp` | `(tuple/unit)` | Diskreter Zeitstempel in Mikrosekunden oder Millisekunden seit Epoche. | `src/types_local.rs` |
| `TxId` | `(tuple/unit)` | — | `src/types_local.rs` |
| `Unarmed` | `(tuple/unit)` | Typzustand: Scanner ist noch nicht scharfgeschaltet. | `src/types_local.rs` |
| `Unarmed` | `(tuple/unit)` | — | `src/erasure_domain.rs` |
| `UnarmedScanner` | `(tuple/unit)` | Scanner im Zustand `Unarmed` (noch nicht scharfgeschaltet). | `src/measure.rs` |
| `UncoveredLayer` | `{ /// Layer that was not covered in measurement. pub layer: DeletionLayer, /// Justification why layer was not covered for the given scope. pub reason: String, }` | Uncovered layer explanation for scope. | `src/deletion_proof.rs` |
| `UnitKey` | `(tuple/unit)` | UnitKey Wrapper. | `src/crypto_local.rs` |
| `VaultKeyring` | `{ vault: Arc<RwLock<FileVault>>, k_enc: [u8; 32], }` | Tresor-gestützte `FileKeyring`-Implementierung nach SPEC D-85 / PO-06. | `src/vault/keyring.rs` |
| `VaultOpenOutcome` | `{ /// Vault-ID laut Metablock. pub vault_id: [u8; 16], /// Shard-Nummer (0..15). pub shard_no: u8, /// Kapazität (2^cap_log2). pub capacity: usize, /// Anzahl ak… }` | Ergebnis der Open-Prüfung für einen Tresor-Shard (D-111 Abs. 8). | `src/vault/open.rs` |
| `VaultSealer` | `{ vault: Option<Arc<RwLock<FileVault>>>, k_unit: [u8; 32], dek_cache: Arc<RwLock<BTreeMap<KeyBuf, ProtectedDek>>>, poisoned: Arc<AtomicBool>, }` | RecordSealer zur Kapselung von Datensätzen in Envelopes mit DEK-Cache und Gruppen-Sync. | `src/vault/sealer.rs` |
| `VaultTable` | `{ /// Slot-Rohdaten (capacity × VAULT_SLOT_SIZE Bytes). raw: Vec<u8>, /// Kapazität (Zweierpotenz). capacity: usize, /// Anzahl aktiver (live) Slots. live: usize… }` | In-Memory-Tresor-Shard (oder Tabelle) für Tests und als Kern-Implementierungsbasis. | `src/vault/table.rs` |
| `VfsOpenOptions` | `{ /// Lesend öffnen. pub read: bool, /// Schreibend öffnen. pub write: bool, /// Datei anlegen falls nicht vorhanden. pub create: bool, /// Neue Datei erzwingen.… }` | Optionen zum Öffnen von Dateien im VFS. | `src/types_local.rs` |
| `WalDisposalReceipt` | `{ /// Betroffener Scope. pub scope: WalScope, /// Anzahl physisch entsorgter Einträge. pub disposed_entries_count: usize, /// Angewandte Entsorgungsmethode. pub… }` | Beleg über die durchgeführte Entsorgung von WAL-Einträgen. | `src/journal/wal_sink.rs` |
| `WalEntryId` | `(tuple/unit)` | Identifikator eines einzelnen Eintrags oder Blocks im WAL. | `src/journal/wal_sink.rs` |
| `WalScope` | `{ /// 16-Byte Scope-ID (z. B. Tenant, Collection oder kombinierte ID). scope_id: [u8; 16], /// Optionales Schlüsselpräfix zur Eingrenzung. prefix: Vec<u8>, }` | Eindeutiger Identifikator für einen logischen Bereich (Scope) im WAL. | `src/journal/wal_sink.rs` |
| `WindowOutcome` | `{ /// Anzahl erfolgreich gelöschter Einheiten. pub purged_units: u64, /// Anzahl freigegebener Bytes. pub bytes_freed: u64, /// Verbleibende offene Löschungen. p… }` | Ergebnis eines Purge-Fensters. | `src/purge/window.rs` |
| `WitnessChainReport` | `{ /// Liste hexadezimaler Digests der Zeugen in kanonischer Reihenfolge. pub witness_digests: Vec<String>, /// Hexadezimaler Wurzel-Digest der Hash-Kette. pub ch… }` | Kryptografische Zeugenkette und Wurzel-Digest (SPEC §11.7, §12.2, INV-ERA-7). Jeder Zeuge wird über seinen deterministischen BLAKE3-Digest gebunden.… | `src/report/build.rs` |
| `WriterGuard` | `{ inflight: Arc<AtomicU32>, }` | RAII-Wächter für einen aktiven Schreiber im System. Solange dieser Wächter existiert, wird der Zähler aktiver Schreiber (`inflight_writers`) hochgeha… | `src/gate/fence.rs` |

### Öffentliche Datentypen — Enums (98)

| Enum | Varianten | Beschreibung | Datei |
|---|---|---|---|
| `AdmissionFailure` | `/// Scope-Spezifikation ist leer oder ungültig. EmptyScope, /// Unzureichende Plattenreserve bereits vor Beginn. InsufficientSpace { /// Freie Bytes. free_bytes: u64, /// Benötigte Bytes. required_by…` | Detaillierte Gründe für das Scheitern von `Admitted`. | `src/purge/phase.rs` |
| `CollectionStatus` | `/// Normal aktiv. #[default] Active, /// Zur physischen Bereinigung vorgemerkt/Fehlgeschlagen. PendingPurge,` | Status einer Collection im System. | `src/orchestrator.rs` |
| `CustodyLevel` | `/// Local custody. Local, /// Sovereign custody. Sovereign, /// Escrowed custody. Escrow,` | Custody level for cryptographic key management during deletion proof generation. | `src/deletion_proof.rs` |
| `DeletionLayer` | `/// In-memory memtable records cleared. LsmMemtable, /// Persistent SSTable files purged across all LSM levels. SsTableAllLevels, /// HNSW graph nodes and tombstone references purged. HnswIndex, ///…` | Layer-explizite Coverage-Deklaration. Jeder Layer MUSS physisch bereinigt sein bevor er hier deklariert wird. | `src/deletion_proof.rs` |
| `DeletionProofError` | `/// Ungültiger Ed25519 VerifyingKey (32 Bytes). #[error("invalid verifying key: {0}")] InvalidVerifyingKey(String), /// Ungültige Ed25519 Signatur (64 Bytes) oder Fehlschlag bei der Verifikation. #[e…` | Fehlerzustände bei der Verifikation von Ed25519-Attestationsbeweisen. | `src/ed25519_proof.rs` |
| `DeletionScope` | `/// Individual document deletion. Document { /// DocId of target document. doc_id: DocId, /// TenantId of owning tenant. tenant_id: TenantId, }, /// Collection-wide deletion. Collection { /// Collect…` | Target scope for physical deletion. | `src/deletion_proof.rs` |
| `DisposeFailure` | `/// Nullungs-Vorgang (Scrub) fehlgeschlagen. ScrubFailed, /// Löschen der physischen Datei (`unlink`) verweigert. UnlinkRefused,` | Detaillierte Gründe für das Scheitern von `Disposing`. | `src/purge/phase.rs` |
| `EraseMode` | `/// Vollständige physische Löschung und Schlüsselvernichtung. Erase, /// Einschränkung der Verarbeitung. Restrict,` | Löschmodus. | `src/types_local.rs` |
| `EraseMode` | `/// Vollständige physische Bereinigung und Schlüsselvernichtung. Erase, /// Einschränkung der Verarbeitung. Restrict,` | — | `src/erasure_domain.rs` |
| `EraseTarget` | `/// Schlüsselintervall. KeyRange { /// Beginn inklusiv. start: Vec<u8>, /// Ende exklusiv. end: Vec<u8>, }, /// Diskrete Menge von Schlüsseln. ExactKeys(Vec<Vec<u8>>),` | Lösch-Zielangabe. | `src/types_local.rs` |
| `EraseTarget` | `/// Einzelnes Dokument. Document { /// Mandanten-Referenz. tenant: TenantRef, /// Collection-Referenz. collection: CollectionRef, /// Dokument-Referenz (inklusive Subject). doc: DocRef, }, /// Betrof…` | Löschziel (SPEC 7.3). | `src/erasure_domain.rs` |
| `ErasureError` | `/// Nicht unterstützte Beweis-Version. #[error("unsupported proof version: {0}")] UnsupportedProofVersion(u8), /// Ungültige Beweis-Signatur. #[error("invalid proof signature")] InvalidProofSignature…` | Oberster Fehlertyp des Löschkerns. | `src/error.rs` |
| `ErasureStatus` | `/// Auftrag empfangen. Requested, /// Im Gate gesperrt. Gated, /// Schlüssel vernichtet. KeyDestroyed, /// Bereinigung läuft. Purging, /// Messung läuft. Verifying, /// Abgeschlossen. Completed, ///…` | Status einer Lösch-Saga (SPEC §11.2, D-124). | `src/types_local.rs` |
| `ErasureStatus` | `/// Auftrag empfangen und validiert. Requested, /// Im Gate persistiert, Lesepfade gesperrt. Gated, /// Schlüssel in Keyslots dauerhaft vernichtet. KeyDestroyed, /// Physisches Umschreiben und Berein…` | — | `src/erasure_domain.rs` |
| `EvidenceKind` | `/// Muster wurde erfolgreich in der Kontrolldatei nachgewiesen Observed,` | Art des Positivkontroll-Nachweises (ER-25). | `src/measure.rs` |
| `ExcludedScope` | `/// Wissen in Zusammenfassungen die als LLM-Fine-Tuning-Input dienten. ConsolidatedAndDistilled, /// LLM-Modellparameter (arXiv:2505.16831 — Unlearning Isn't Deletion). LlmParameterMemory,` | Explizite Nicht-Abdeckung — maschinenlesbar für Audit-Systeme. | `src/deletion_proof.rs` |
| `FenceFailure` | `/// Der Scope ist durch einen konkurrierenden Purge-Vorgang gesperrt (Single-Flight). ScopeAlreadyLocked, /// Compaction-Aktivität blockiert das Errichten des Fences. CompactionBusy, /// Ein aktiver…` | Detaillierte Gründe für das Scheitern von `Fenced`. | `src/purge/phase.rs` |
| `FenceGateError` | `/// Schreiber abgewiesen: Der Zielbereich ist durch einen aktiven Fence gesperrt. #[error("Schreibzugriff verweigert: Bereich ist gefenct")] Fenced { /// Der Bereich, der den Zugriff blockiert hat. r…` | Fehler im Lebenszyklus eines Fences oder bei Schreibzugriffen. | `src/gate/fence.rs` |
| `FenceRange` | `/// Ein exakter Einzelschlüssel. Exact(Vec<u8>), /// Ein Schlüsselpräfix (trifft auf alle Schlüssel zu, die mit diesem Byte-Präfix beginnen). Prefix(Vec<u8>), /// Ein geschlossenes Intervall `[lo, hi…` | Spezifikation des gefencten Schlüsselbereichs. | `src/gate/fence.rs` |
| `FileKind` | `/// SSTable-Datendatei. SSTable, /// WAL-Datei. Wal, /// Textindex-Segment. TextSegment, /// Vektor-HNSW-Segment. VectorSegment, /// Allgemeine Hilfsdatei. Auxiliary,` | Dateityp für den FileKeyring. | `src/types_local.rs` |
| `FinalizeFailure` | `/// Ledger-Append fehlgeschlagen. LedgerAppendFailed, /// Ledger-Commit/fsync fehlgeschlagen. LedgerSyncFailed,` | Detaillierte Gründe für das Scheitern von `Finalizing`. | `src/purge/phase.rs` |
| `FlushFailure` | `/// E/A-Fehler beim Schreiben des Memtable-Flushes. IoError, /// Timeout beim Warten auf In-Flight-Schreiber. Timeout,` | Detaillierte Gründe für das Scheitern von `Flushed`. | `src/purge/phase.rs` |
| `GateDecision` | `/// Schlüssel ist lesbar. Open, /// Schlüssel ist gelöscht. Erased, /// Zugriff vorübergehend eingeschränkt. Restricted,` | Gate-Entscheidung für Punktabfragen oder Dateigrenzen (SPEC §4.2, §11.3). | `src/types_local.rs` |
| `GateLogEntry` | `/// Ein neues Gate hinzufügen. Add { /// Gate-Kennung. gate_id: [u8; 16], /// Gate-Variante: true = Erased, false = Restricted. is_erased: bool, /// Untere Grenze (16 Bytes). lo: [u8; 16], /// Obere…` | Mögliche Eintragsarten im Gate-Log (SPEC §11.3, Anhang F.9). | `src/gate/log.rs` |
| `GatePredicate` | `/// Bereichsprädikat `[lo, hi]` (je 65 Bytes KeyBound als Hex oder Raw-Bytes). Range { /// Untere Grenze (65 Bytes als Vec für Serde-Kompatibilität). lo: Vec<u8>, /// Obere Grenze (65 Bytes als Vec f…` | Prädikat eines GateLog-Eintrags nach SPEC F.9. | `src/gate/log.rs` |
| `GateVariant` | `/// Klasse P: Physisch gelöscht und gesperrt. Erased, /// Einschränkung der Verarbeitung aktiv. Restricted,` | — | `src/erasure_domain.rs` |
| `GuardOutcome` | `/// Kein Wiederauftauchen erkannt, Store kann freigegeben werden. Clean, /// Zeitregression erkannt – Store gesperrt, Replay erforderlich. TimeRegression { /// Aktueller Store-Seq. store_seq: u64, //…` | Ergebnis des Restore-Guard-Checks. | `src/restore/guard.rs` |
| `HkdfError` | `/// Ausgabe-Puffer ueberschreitet die RFC 5869 Grenze (255 * Hash-Laenge). #[error("output length too large")] OutputTooLarge, /// Ungueltiger Schluessel fuer HMAC. #[error("invalid key")] InvalidKey…` | Führt HKDF-Expand gemäß RFC 5869 auf Basis von HMAC-SHA256 durch. Fehler bei HKDF-Operationen. | `src/crypto_local.rs` |
| `IndexCorruptionKind` | `/// Ungültige Kantenverweise / hängende Referenzen. DanglingBacklink, /// Ungültige Header-Prüfsumme. BadChecksum, /// Unvollständige oder asynchrone Mmap-Snapshot-Reste. PersistedSnapshotResidue, //…` | Klassifikation festgestellter Index-Korruptionen. | `src/measure/residue.rs` |
| `KeyBound` | `/// Exakte Schlüsselbegrenzung. Exact(KeyBuf), /// Unbegrenzte untere Grenze. UnboundedLower, /// Unbegrenzte obere Grenze. UnboundedUpper,` | Begrenzungs-Schlüssel für Scans und Kompaktion. | `src/types_local.rs` |
| `KeyCoverage` | `/// Vollständige Schlüsselabdeckung. Full, /// Unverknüpfte Schlüssel. Unlinked, /// Keine eigene Schlüsselvernichtung. None,` | Reichweite der Schlüsselabdeckung einer Schicht (SPEC §8.2, D-69). | `src/types_local.rs` |
| `KeyCoverage` | `/// Vollständige kryptografische Unlesbarkeit aller Records/Payloads. Full, /// Struktur sichtbar, aber Verknüpfung unlesbar gemacht. Unlinked, /// Keine Schlüsselvernichtung auf dieser Schichtebene.…` | — | `src/erasure_domain.rs` |
| `KeyScope` | `/// Mandanten-Löschung. Tenant(TenantId), /// Collection-weite Löschung. Collection { /// Mandant. tenant: TenantId, /// Collection. collection: CollectionId, }, /// Einzelnes Dokument. Document { //…` | Abstrakter Bezeichner für den zu bereinigenden Schlüsselbereich (SPEC v4.0 §6.2, B3). | `src/types_local.rs` |
| `KeyShredError` | `/// Gruppe nicht gefunden. #[error("Gruppe nicht gefunden: {group_id}")] GroupNotFound { /// Betroffene Gruppen-ID. group_id: u64, }, /// Epoche nicht gefunden. #[error("Epoche nicht gefunden für Gru…` | Lokale Fehlertaxonomie für Schlüsseloperationen und Krypto-Shredding. | `src/kv_shredding.rs` |
| `KeyStoreError` | `/// Gruppe oder Epoche wurde nicht gefunden. #[error("Schlüsselbereich nicht gefunden: Gruppe {group_id}, Epoche {epoch}")] NotFound { /// Betroffene Gruppen-ID. group_id: u64, /// Betroffene Epoche.…` | Lokale Fehlertaxonomie für den synchronen `KeyStore`-Port. | `src/vault/key_store.rs` |
| `LedgerError` | `/// Das angeforderte Ledger existiert nicht und `create_if_missing` war `false`. #[error("Ledger file/storage not found")] NotFound, /// Datenkorruption (ungültige Magic, ungültige Prüfsumme, abgeris…` | Fehlerzustände des Ledger-Subsystems. | `src/ledger/error.rs` |
| `MeasureClass` | `/// MK1: Logische Strukturen. Mk1, /// MK2: Rohdaten-Bytescan. Mk2, /// MK3: Physisches Nulling / Medienprüfung. Mk3,` | Messklasse für Kontrollscans (SPEC §12.1). | `src/types_local.rs` |
| `MeasureClass` | `/// MK1: Logische/funktionale Prüfung. Mk1, /// MK2: Speicherebenen-Rohdatenprüfung. Mk2, /// MK3: Physische Medieninspektion. Mk3,` | — | `src/erasure_domain.rs` |
| `ObservedPhase` | `/// Read-Gate eingerichtet. Gated, /// Bereinigung und Verifikation abgeschlossen. Completed, /// Einschränkung aktiv. Restricted, /// Einschränkung aufgehoben. Lifted,` | Beobachtete Lebenszyklus-Phase. | `src/types_local.rs` |
| `ObserverError` | `/// Vorübergehender Fehler. #[error("transient delivery failure: {0}")] Transient(String), /// Dauerhafter Fehler. #[error("permanent delivery failure: {0}")] Permanent(String), /// Zeitüberschreitun…` | Fehlerklasse für den Observer. | `src/types_local.rs` |
| `OffsetClass` | `/// Knotenrepräsentation (z. B. Vektorslot, Knotenheader). VectorNode, /// Rückverweis- / Backlink-Tabelle. BacklinkEntry, /// Nachbarschaftsliste (Graph-Ebenen). NeighborList, /// Quantisierte Daten…` | Klassifikation der Fundstelle/Speicherstruktur innerhalb der Indexschicht. | `src/measure/residue.rs` |
| `PendingReason` | `/// I/O-Fehler. Io, /// SLA überschritten. SlaExceeded, /// Schichtfehler. LayerFailed(LayerId), /// Zeuge fehlt. WitnessMissing, /// Plattenreserve unterschritten. DiskReserve, /// Unspezifischer Fe…` | Grund für Schwebezustand. | `src/types_local.rs` |
| `PendingReason` | `/// I/O-Fehler bei der Schichtbearbeitung. Io, /// SLA-Zeitfenster überschritten. SlaExceeded, /// Eine Schicht ist fehlgeschlagen. LayerFailed(LayerId), /// Nachweis/Zeuge fehlt noch. WitnessMissing…` | — | `src/erasure_domain.rs` |
| `Phase` | `/// Intent-Registrierung. Intent, /// Gate-Setzung. Gate, /// Schlüsselvernichtung. KeyDestroy, /// Physisches Bereinigen. Purge, /// Unabhängige Nachweismessung. Verify, /// Ersetzung. Replace, ///…` | Phase eines Löschablaufs (SPEC §7.4, D-124 Abs. 5). | `src/types_local.rs` |
| `Phase` | `/// Intent-Registrierung. Intent, /// Gate-Setzung. Gate, /// Schlüsselvernichtung. KeyDestroy, /// Physisches Bereinigen. Purge, /// Unabhängige Nachweismessung. Verify, /// Dokument-Ersetzung (D-11…` | — | `src/erasure_domain.rs` |
| `PurgePhase` | `/// 1: Purge-Auftrag angenommen, Intent & Scope persistent geschrieben. Admitted = 1, /// 2: Erasure-Fence auf betroffenen Scope errichtet, Schreiber blockiert. Fenced = 2, /// 3: Aktive Memtables un…` | Einheitliche Enum für den gesamten Lebenszyklus des physischen Purge (WP-2). Repräsentiert als `u8`-Codes (1..=9) zur sicheren Persistenz und Wiederh… | `src/purge/phase.rs` |
| `PurgePhaseError` | `/// Ungültiger Phasenübergang in der Zustandsmaschine. #[error("invalid purge phase transition from {from:?} to {to:?}")] InvalidTransition { /// Ausgangsphase. from: PurgePhase, /// Zielphase. to: P…` | Fehlerzustände während der einzelnen Purge-Phasen (WP-1, WP-2). Jeder Fehler besitzt einen klaren Phasenbezug; Freitext-Stringfehler sind verboten. | `src/purge/phase.rs` |
| `RangeGate` | `/// Bereich enthält gelöschte Daten. ContainsErased, /// Bereich ist gesperrt / darf nicht verworfen werden. RestrictedNeverDiscard, /// Vollständig lesbar. EntirelyOpen,` | Gate-Entscheidungsvariante für Bereichsoperationen (SPEC §4.2, I-18). | `src/types_local.rs` |
| `ReachStatus` | `/// Erreicht Reached, /// Nicht erreicht NotReached,` | Pflichtstatus einer Nachweisklasse (G, K, P laut SPEC D-36, §12.5). | `src/report/build.rs` |
| `RecordCode` | `/// 1: Requested+Intent RequestedIntent = 1, /// 2: Gated Gated = 2, /// 3: KeyDestroyed KeyDestroyed = 3, /// 4: Purging Purging = 4, /// 5: Verifying Verifying = 5, /// 6: Completed Completed = 6,…` | Record-Codes nach SPEC Anhang F.9. | `src/journal/record.rs` |
| `RecordType` | `/// Intent vor Beginn Intent = 0x01, /// Completed / Purged nach Ausführung Completed = 0x02, /// KeyDestroyed KeyDestroyed = 0x03,` | Typ eines Ledger-Records im CTRALEDG-Format. | `src/ledger/record.rs` |
| `RefusalReason` | `/// Aufbewahrungspflicht. RetentionObligation, /// Ungültige Parameter. InvalidParameters,` | Ablehnungsgrund vor Gate. | `src/types_local.rs` |
| `RefusalReason` | `/// Gesetzliche Aufbewahrungspflicht steht der Bereinigung entgegen. RetentionObligation, /// Unzulässige Parameterkombination. InvalidParameters,` | — | `src/erasure_domain.rs` |
| `ReleaseFailure` | `/// Fence-Freigabe konnte nicht atomar persistiert werden. FenceReleaseFailed, /// Bereinigung der Intent-Datei fehlgeschlagen. IntentCleanupFailed,` | Detaillierte Gründe für das Scheitern von `Released`. | `src/purge/phase.rs` |
| `ReplaceStep` | `/// R1: Initialer Zustand nach Eingang des Ersetzungsantrags. R1Requested, /// R2: Absicht registriert, PreGate-Ausführung für Neudokument aktiv. R2PreGate, /// R3: Neudokument dauerhaft bestätigt, A…` | Einzelschritte des Ersetzungsprotokolls (SPEC D-115). | `src/replace.rs` |
| `ReserveCheck` | `/// Ausreichend Speicherplatz vorhanden. Sufficient { /// Freie Bytes. free_bytes: u64, }, /// Zu wenig Speicherplatz (→ Pending(DiskReserve)). Insufficient { /// Freie Bytes. free_bytes: u64, /// Be…` | Ergebnis der Speicherplatzprüfung. | `src/purge/reserve.rs` |
| `ResidualRisk` | `/// Flash-Remanenz auf FTL-Ebene. FlashRemanence, /// CoW-Dateisysteme oder Dateisystemjournale. FsJournalOrCow, /// Reste verpackter Schlüsselkopien. WrappedKeyRemanence, /// Reste in freigegebenen…` | — | `src/erasure_domain.rs` |
| `ResidualRiskId` | `/// Flash-Remanenz auf FTL-Ebene. FlashRemanence, /// CoW-Dateisysteme oder Dateisystemjournale. FsJournalOrCow, /// Reste verpackter Schlüsselkopien. WrappedKeyRemanence, /// Reste in freigegebenen…` | Identifikatoren für Restrisiken nach SPEC Anhang D. | `src/report/risk.rs` |
| `ResidueMeasurementError` | `/// Positivkontrolle fehlgeschlagen: Kontrollwert wurde nicht im Index gefunden. #[error("positive control failed in phase {phase}: canary was not detected")] PositiveControlFailed { /// Phase, in de…` | Spezifische Fehlerklasse für Restspuren-Messungen. Folgt Regel 2 (keine Stringfehler, Phasenbezug) und Regel 4 (Fail-Closed). | `src/measure/residue.rs` |
| `ResiduePhase` | `/// Initialisierung der Messung. Init, /// Basisscan vor der Löschung. PreEraseScan, /// Durchführung der Positivkontrolle. PositiveControl, /// Prüfscan nach der Löschung (Negativkontrolle). PostEra…` | Spezifische Phase einer Restspuren-Messung. | `src/measure/residue.rs` |
| `RestrictedPurpose` | `/// Geltendmachung, Ausübung oder Verteidigung von Rechtsansprüchen. LegalClaims, /// Einwilligung der betroffenen Person. DataSubjectConsent,` | Gesetzlicher Ausnahmezweck nach Art. 18 Abs. 2 DSGVO (SPEC D-116). | `src/restricted.rs` |
| `RestrictedState` | `/// Einschränkung der Verarbeitung aktiv. Restricted, /// Einschränkung durch formalen Journal-Eintrag aufgehoben. Lifted, /// Restriktion durch nachfolgenden Löschauftrag abgelöst (D-119). Supersede…` | Zustandsmodell für eingeschränkte Daten (Art. 18 DSGVO, SPEC §11.2, D-116, D-119). | `src/restricted.rs` |
| `RetentionCode` | `/// Keine Aufbewahrungspflicht. NoObligation, /// Pflicht abgelaufen. ObligationExpired, /// Pflicht aktiv. ObligationActive, /// Rechtsansprüche anhängig. LegalClaimsPending, /// Betroffene Person h…` | Aufbewahrungscode. | `src/types_local.rs` |
| `RetentionCode` | `/// Es besteht keine Aufbewahrungspflicht. NoObligation, /// Aufbewahrungspflicht bestand und ist abgelaufen. ObligationExpired, /// Aufbewahrungspflicht besteht aktiv. ObligationActive, /// Rechtsan…` | — | `src/erasure_domain.rs` |
| `RetentionDecision` | `/// Löschung wird fortgesetzt. ProceedErase, /// Zugriffsbeschränkung wird gesetzt. ProceedRestrict, /// Antrag wird verweigert. Refuse(RefusalReason),` | Ergebnis der Retention-Prüfung. | `src/saga/core.rs` |
| `RevocationTarget` | `/// Shredding Group ID (z. B. für `KeyRegistry`) Group(u64), /// Key Encryption Key (KEK) Identifier Kek(String), /// Data Encryption Key (DEK) Identifier Dek(String), /// Specific Record Identifier…` | Das Ziel eines Widerruf-Eintrags im Log (Group ID, KEK ID, DEK ID oder Record ID). | `src/revocation_log.rs` |
| `RewriteError` | `/// Handle ist dauerhaft vergiftet (INV-VAULT-FAILCLOSED-1). #[error("rewriter handle is permanently poisoned")] Poisoned, /// Phase 1: Fehler beim Schreiben des Intent ins Journal. #[error("journal…` | Fehlerarten für Rewrite-Operationen mit Phasenbezug (WP-5). | `src/vault/sealer.rs` |
| `RewriteFailure` | `/// Plattenreserve unterschritten während des Rewrite-Vorgangs. DiskReserveExceeded, /// E/A- oder Integritätsfehler beim Lesen oder Schreiben von Segmenten. SegmentIoError, /// Dateikopf- oder Signa…` | Detaillierte Gründe für das Scheitern von `Rewriting`. | `src/purge/phase.rs` |
| `RewriteState` | `/// Phase 1: Intent persistent erfasst. Planned, /// Phase 2: Ziel-Blobs überschrieben / gepurgt. Overwritten, /// Phase 3: Überschriebene Daten synchronisiert. Synced, /// Phase 4: Manifest persiste…` | Zustand eines Rewrite-Eintrags im Journal. | `src/vault/sealer.rs` |
| `RiskCondition` | `/// Bedingung trifft immer zu. Always, /// Trifft zu bei CoW-Dateisystemen oder Journaling. CowOrJournaling, /// Trifft zu bis zum nächsten Kompaktions- und Purge-Durchlauf. UntilPurgeComplete,` | — | `src/erasure_domain.rs` |
| `SagaEvent` | `/// Nächsten Schritt ausführen. Advance(ErasureId), /// Zurück in Pending versetzen mit Grund. SetPending(ErasureId, PendingReason), /// Saga abschließen. Complete(ErasureId),` | Ereignis der Treiber-Pipeline (SPEC §11.2). | `src/saga/driver.rs` |
| `SagaState` | `/// Löschauftrag eingegangen. Requested, /// Gate gesetzt und Schlüssel vernichtet – Daten unlesbar. Gated, /// Schlüssel vernichtet (Intermediate-Zustand, Purge noch offen). KeyDestroyed, /// Purge-…` | Saga-Zustand (SPEC §11.2, §7.4, D-124 Abs. 5, ER-29). | `src/saga/machine.rs` |
| `ScanMethod` | `/// Direct structural scan of storage files, indices, or tables. Structural, /// Layer was not measured during this execution with specified reason. NotMeasured(String), /// Layer measurement is disa…` | Scan-Methode für Berichts- und Nachweiszwecke. | `src/types_local.rs` |
| `ScanMethod` | `/// MK1: Logische Scanmethode (z. B. Iteration über Indexeinträge). Mk1, /// MK2: Physische Scanmethode auf Block-/Dateiebene. Mk2, /// MK3: Rohmedien-Inspektion. Mk3,` | — | `src/erasure_domain.rs` |
| `ScanTechnique` | `/// Direkter struktureller Scan. Structural, /// Scan über entschlüsselte Ansicht. DecryptedView, /// Index-Introspektion. IndexIntrospection, /// Nadel-Scan (Needle Scan). NeedleScan, /// Schicht wu…` | Angewandte Methode/Technik während der Verifikation. | `src/types_local.rs` |
| `ScannerFaultKind` | `/// E/A- oder Medienfehler. IoFailure, /// Zugriff verweigert oder gesperrt. AccessDenied, /// Segment nicht erreichbar. SegmentUnavailable, /// Interner Fehler im Index-Adapter. AdapterFailure,` | Art des Fehlers im zugrundeliegenden Index-Scanner. | `src/measure/residue.rs` |
| `ScopeLockError` | `/// Scope ist bereits durch einen aktiven Purge gesperrt. #[error("scope {scope_id:?} is locked by active owner {owner_id:?} (epoch {epoch})")] ScopeLocked { /// Betroffener Scope bzw. Erasure-ID. sc…` | Fehler beim Erwerb oder der Freigabe eines Scope-Locks. | `src/scope/single_flight.rs` |
| `SealerError` | `/// Handle ist vergiftet (D-55, INV-VAULT-FAILCLOSED-1). #[error("sealer handle is permanently poisoned")] Poisoned, /// Puffer-Längen stimmen beim Speicher-Swap nicht überein. #[error("buffer length…` | Fehlerarten für Sealer- und Swap-Operationen mit Phasenbezug (WP-5a). | `src/vault/sealer.rs` |
| `ShardBackend` | `/// In-Memory Shard-Tabelle. InMemory(VaultTable), /// Dauerhafter Shard über VFS. Durable(DurableVaultShard),` | Backend-Speicher eines Shards (entweder persistent über VFS oder im Arbeitsspeicher). | `src/vault.rs` |
| `SignatureVersion` | `/// Version 1: HMAC-SHA256 über Scope, Key-Hash und TxId. V1 = 1, /// Version 2: HMAC-SHA256 mit Bindung an Covered Layers, Excluded Scopes und WAL-Receipt. V2 = 2, /// Version 3: Ed25519 asymmetrisc…` | `SignatureVersion`-Enum für typsichere Versionierung von DeletionProof-Signaturen. | `src/ed25519_proof.rs` |
| `SignerError` | `/// Schlüssel nicht verfügbar. #[error("signing key unavailable: {0}")] KeyUnavailable(String), /// Externer Signierer fehlgeschlagen. #[error("external signer failed: {0}")] ExternalSignerFailed(Str…` | Signierer-Fehlerklasse. | `src/types_local.rs` |
| `SigningKeyHandle` | `/// Symmetric HMAC key slice. Hmac(&'a [u8]), /// Ed25519 signing key reference. Ed25519(&'a ed25519_dalek::SigningKey),` | Handle to the key used for signing deletion proofs. | `src/deletion_proof.rs` |
| `SlotType` | `/// Mandanten-Stamm-Slot. Tenant = 0x01, /// Collection-Slot. Collection = 0x02, /// Betroffenen-Slot (Subject). Subject = 0x03, /// Dokument-Slot. Document = 0x04, /// Datei-Schlüssel-Slot. File = 0…` | Typen von Tresor-Slots (F.3, D-33, D-79, D-85, D-111, D-125). | `src/vault/slot.rs` |
| `StepOutcome` | `/// Zustand erfolgreich gewechselt. Advanced(SagaState), /// Zustand bleibt Pending (mit Grund). Pending(PendingReason), /// Saga ist abgeschlossen oder verweigert. Terminal(SagaState),` | Ergebnis eines Treiberschritts. | `src/saga/driver.rs` |
| `SupersedeMode` | `/// Standard: Alte Versionen bleiben in der Historie erhalten. Keep, /// Purge: Vorherige Versionen werden gegated und physisch entfernt (Art. 16 DSGVO). PurgeOldVersions,` | Modus der Berichtigung (SPEC D-117). | `src/supersede.rs` |
| `TargetKind` | `/// Dokument. Document, /// Betroffener. Subject, /// Collection. Collection, /// Mandant. Tenant, /// Ersetzung (`replace_erasing`). DocumentReplace, /// Überschreibung (`update_with(..., Purge)`).…` | Ziel-Ebene für Observer-Ereignisse. | `src/types_local.rs` |
| `TargetType` | `/// 1: Document Document = 1, /// 2: Subject Subject = 2, /// 3: Collection Collection = 3, /// 4: Tenant Tenant = 4, /// 5: Superseded Superseded = 5, /// 6: DocumentReplace DocumentReplace = 6,` | Zieltypen (target_type) nach SPEC Anhang F.9. | `src/journal/record.rs` |
| `Timing` | `/// Sofort. Now, /// Gebündelt. Batched,` | Timing der Löschung. | `src/types_local.rs` |
| `Timing` | `/// Sofortige Ausführung im Vordergrund. Now, /// Gebündelte Ausführung im Batch-Fenster. Batched,` | — | `src/erasure_domain.rs` |
| `VerificationKey` | `/// Key for HMAC-SHA256 signature verification (version 1 and 2). Hmac(&'a [u8]), /// Key for Ed25519 signature verification (version 3). Ed25519(&'a ed25519_dalek::VerifyingKey),` | Key parameter for verification (either HMAC-SHA256 byte slice or Ed25519 VerifyingKey). | `src/deletion_proof.rs` |
| `VerifiedVia` | `/// Direktes I/O ohne Puffer (`O_DIRECT`). Direct, /// Nach Puffer-Verwerfung (`posix_fadvise`). CacheDropped, /// Gewöhnlicher Cache-Pfad. Cached,` | Verifikationsmodus beim Lesen von Medien (SPEC D-113). | `src/types_local.rs` |
| `VerifiedVia` | `/// O_DIRECT-Lesen am Medium. Direct, /// Cache-Invalidierung (fadvise) und Lesen. CacheDropped, /// Cached-Lesen. Cached,` | Modus für Verifikation am Medium nach SPEC D-113. | `src/vault/table.rs` |
| `VerifiedVia` | `/// Direktes Lesen am Medium (O_DIRECT / F_NOCACHE). Direct, /// Cache vor dem Lesen verworfen (posix_fadvise DONTNEED). CacheDropped, /// Regulärer Cache-Lesepfad (Fallback bis D-113 voll in Ports g…` | Art der Leseverifikation gemäß SPEC D-113. | `src/measure/field.rs` |
| `WalDisposalMethod` | `/// Physische Tilgung (z. B. Überschreiben oder Truncate der WAL-Einträge). PhysicalPurge, /// Unlesbarmachung durch Vernichtung des scope-spezifischen Schlüssels (Krypto-Shredding). CryptoShredding,…` | Methode, mit der Einträge eines Scopes unlesbar oder getilgt wurden. | `src/journal/wal_sink.rs` |
| `WalRetireFailure` | `/// WAL-Segment konnte nicht rotiert oder genullt werden. WalSyncError, /// WAL-Integritätskette beschädigt. WalCorrupt,` | Detaillierte Gründe für das Scheitern von `WalRetiring`. | `src/purge/phase.rs` |
| `WalSinkError` | `/// E/A-Fehler bei Lese- oder Schreiboperationen im WAL. #[error("E/A-Fehler in WAL-Phase {phase}: {detail}")] Io { /// Betroffene Phase. phase: &'static str, /// Detailbeschreibung. detail: String,…` | Fehlertyp für den WAL-Sink-Port mit Phasenbezug (fail-closed, kein reiner String-Fehler). | `src/journal/wal_sink.rs` |
| `WitnessFailure` | `/// Positivkontrolle schlug fehl (Scanner unzuverlässig). PositiveControlFailed, /// Unerwartete Restspuren im gescannten Medium gefunden. ResidueDetected { /// Anzahl gefundener Spuren. count: u64,…` | Detaillierte Gründe für das Scheitern von `Witnessing`. | `src/purge/phase.rs` |
| `WitnessLayer` | `/// LSM Memtable LsmMemtable, /// SsTable Dateien über alle Level SsTableAllLevels, /// HNSW Vektorgraph-Index HnswIndex, /// Write-Ahead-Log Segmente WalAllSegments, /// Compressed Sparse Row Graph…` | Identifikator für die physische Storage-Schicht. | `src/types_local.rs` |

### Typ-Aliase (14)

| Alias | Definition | Beschreibung |
|---|---|---|
| `BoxFuture` | `type BoxFuture<'a, T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>` | Type-Alias für gepinnte ge-boxte Futures ohne Tokio/Futures-Util. |
| `BoxFuture` | `type BoxFuture<'a, T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>` | Type-Alias für gepinnte ge-boxte Futures ohne Tokio/Futures-Util. |
| `BoxFuture` | `type BoxFuture<'a, T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>` | Type-Alias für BoxFuture ohne tokio/futures-util. |
| `BoxFuture` | `type BoxFuture<'a, T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>` | Type-Alias für gepinnte ge-boxte Futures ohne Tokio/Futures-Util. |
| `BoxFuture` | `type BoxFuture<'a, T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>` | Type-Alias für gepinnte ge-boxte Futures ohne Tokio/Futures-Util. |
| `BoxFuture` | `type BoxFuture<'a, T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>` | Type-Alias für BoxFuture ohne tokio/futures-util. |
| `BoxFuture` | `type BoxFuture<'a, T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>` | Type-Alias für gepinnte ge-boxte Futures ohne Tokio/Futures-Util. |
| `CryptoError` | `type CryptoError= crate::error::ErasureError` | Typalias für CryptoError. |
| `InMemoryKeyring` | `type InMemoryKeyring= VaultKeyring` | In-Memory-Keyring für Tests und abwärtskompatible Validierung. |
| `Result` | `type Result<T> = std::result::Result<T, ErasureError>` | Typalias für Result. |
| `Result` | `type Result<T> = std::result::Result<T, crate::error::ErasureError>` | Typalias für Result. |
| `ScopeLayerId` | `type ScopeLayerId= [u8` | Kennung einer Schicht (für Scope-Zuordnung). |
| `SignerId` | `type SignerId= u8` | Signer-ID-Typalias. |
| `WitnessEntry` | `type WitnessEntry= LayerWitnessReport` | Zeuge einer einzelnen Schicht im Bericht (Abwärtskompatibilitäts-Alias). |

### Konstanten (35)

| Konstante | Typ | Beschreibung |
|---|---|---|
| `CACHE_EVICT` | `Self` | Cache-Eviction (niedrigste Priorität). |
| `CTRALEDG_MAGIC` | `&[u8` | CTRALEDG Magic (8 Bytes): "CTRALEDG" |
| `CTRALEDG_VERSION` | `u32` | Aktuelle Version des Ledger-Formats |
| `CTX_FILE_MAGIC` | `&[u8` | F.1 Magic-Bytes für Acta-Dateien. |
| `CTX_FORMAT_VERSION` | `u16` | F.1 Format-Version (v1 = 1). |
| `DEFAULT_EXCLUDED_SCOPES` | `&[ExcludedScope]` | Default excluded scopes for Acta deletion proofs (§20.4). |
| `DEFAULT_SHRED_KEY_GROUP_SIZE` | `usize` | Standardgröße einer Shredding-Schlüsselgruppe (Anzahl Records je KEK). |
| `FIXED_HEADER_LEN` | `usize` | Fixer Anteil der Frame-Länge (ohne range_tags und CRC): record_type(1) + seq(8) + timestamp(8) + commitment(32) + range_tags_len(4) + prev_hash(32) +… |
| `GEN` | `usize` | Offset des gen-Feldes (Bytes 2..8, u48 big-endian, 6 B). |
| `GENESIS_HASH` | `[u8` | Genesis-Hash für die erste Kette (Nullvektor). |
| `MAC` | `usize` | Offset des mac-Feldes (Bytes 56..64, 8 B). |
| `MIN_FRAME_LEN` | `usize` | Minimale Framelänge inklusive CRC32C. |
| `PAYLOAD` | `usize` | Offset des payload-Feldes (Bytes 24..56, 32 B). |
| `SIZE` | `usize` | Größe des Headers in Bytes. |
| `SLOT_CLEANUP` | `Self` | Slot-Bereinigung (D-Slots). |
| `SLOT_TYPE` | `usize` | Offset des slot_type-Feldes (Byte 0, 1 B). |
| `SNAPSHOT_REVOKE` | `Self` | Snapshot-Widerruf (höchste Priorität). |
| `SSTABLE_REWRITE` | `Self` | SSTable-Rewrite. |
| `STATUS` | `usize` | Offset des status-Feldes (Byte 1, 1 B). |
| `STATUS_ACTIVE` | `u8` | Slot-Status: aktiv. |
| `STATUS_EMPTY` | `u8` | Slot-Status-Werte (F.3). |
| `STATUS_TOMBSTONE` | `u8` | Slot-Status: Grabstein (für lineare Sondierung). |
| `TAG` | `usize` | Offset des tag-Feldes (Bytes 8..24, 16 B). |
| `TEXT_REWRITE` | `Self` | Textsegment-Rewrite. |
| `UNMEASURED_ERASURE_LAYERS` | `&[&str]` | Schichten, die in standardmäßigen Löschläufen derzeit ungemessen bleiben können (SPEC §9.4). |
| `VAULT_ALG_IDS` | `[u8` | Standard-Algorithmen-Satz v1: [aead=1, kdf=0, mac=1, hash=1]. |
| `VAULT_FILE_KIND` | `u8` | Dateityp (kind) für Tresor nach SPEC F.1: 10. |
| `VAULT_HEADER_SIZE` | `usize` | Größe des F.1-Dateikopfs in Bytes. |
| `VAULT_METABLOCK_SIZE` | `usize` | Größe des Metablocks nach SPEC F.3. |
| `VAULT_NUM_SHARDS` | `usize` | Anzahl Shards im Tresor nach SPEC D-111 Abs. 1. |
| `VAULT_SLOT_BASE` | `usize` | Byte-Offset des ersten Slots in der Vault-Datei (F.3, ER-03). |
| `VAULT_SLOT_BASE` | `usize` | Basis-Offset der Slots in einer Shard-Datei: 64 Bytes F.1-Kopf + 64 Bytes Metablock = Offset 128 (SPEC F.3). |
| `VAULT_SLOT_SIZE` | `usize` | Größe eines Tresor-Slots in Bytes (F.3, D-111, I-23). |
| `VECTOR_REPAIR` | `Self` | Vektor-Kantenreparatur. |
| `WAL_FLUSH` | `Self` | WAL-Segmente und Memtable. |

### Öffentliche Funktionen (606)

| Signatur | Beschreibung | Datei |
|---|---|---|
| `pub fn as_bytes(&self) -> &[u8` | Gibt eine Referenz auf die geschützten Bytes zurück. | `src/crypto_local.rs` |
| `pub fn derive_dek(unit: &UnitKey, a: &AncestorSalts, doc: [u8; 16]) -> Dek` | Leitet den DEK nach SPEC D-79 / §9.4 / §10.1 ab: `DEK = HKDF-SHA256(ikm = K_unit, salt = SHA-256("ctx/dek-salt/v2" || salt_T || salt_C || s… | `src/crypto_local.rs` |
| `pub fn hkdf_expand(prk: &[u8], info: &[u8], out: &mut [u8]) -> Result<(), HkdfError>` | Fuehrt HKDF-Expand gemaess RFC 5869 auf Basis von HMAC-SHA256 durch. | `src/crypto_local.rs` |
| `pub fn open_record( dek: &Dek, key_bytes: &[u8], env: &[u8], ) -> Result<zeroize::Zeroizing<Vec<u8>>, crate::error::ErasureError>` | Öffnet einen versiegelten Datensatz gemäß F.6. | `src/crypto_local.rs` |
| `pub fn seal_record(dek: &Dek, doc: [u8; 16], key_bytes: &[u8], plain: &[u8]) -> Vec<u8>` | Verschlüsselt und versiegelt einen Datensatz gemäß F.6 (Envelope-Format). Format: `version(1) || alg_id(1) || doc(16) || nonce(12) || ciphe… | `src/crypto_local.rs` |
| `pub fn compute_wal_delete_receipt( prev_hmac: &[u8; 32], delete_event_payload: &[u8], integrity_key: &[u8], ) -> Result<[u8` | Erzeugt eine WAL-Löschquittung H(hmac_prev || delete_event) für den Nachweis auf WAL-Ebene gemäß DSGVO Art. 17. | `src/deletion_proof.rs` |
| `pub fn create( scope: DeletionScope, deleted_keys: Vec<Vec<u8>>, deleted_after_tx: TxId, covered_layers: Vec<LayerCleanupProof>, excluded_scopes: Vec<ExcludedScope>, proof_key: &[u8], ) -> Result<Self>` | Erstellt und signiert einen DeletionProof nach Layer-Bereinigung. AUFRUFREIHENFOLGE (INV-DELETION-1): 1. Alle covered_layers physisch berei… | `src/deletion_proof.rs` |
| `pub fn create_full_v3( scope: DeletionScope, mut deleted_keys: Vec<Vec<u8>>, deleted_after_tx: TxId, covered_layers: Vec<LayerCleanupProof>, excluded_scopes: Vec<ExcludedScope>, wal_chain_receipt: Option<[u8; 32]>, audit_chain_position: Option<u64>, attested_…` | Erstellt und signiert einen DeletionProof (Version 3, Ed25519) mit allen optionalen Erweiterungen. | `src/deletion_proof.rs` |
| `pub fn create_v3( scope: DeletionScope, deleted_keys: Vec<Vec<u8>>, deleted_after_tx: TxId, covered_layers: Vec<LayerCleanupProof>, excluded_scopes: Vec<ExcludedScope>, attested_at: i64, graph_repair: &[GraphRepairAttestation], signing_key: &ed25519_dalek::Si…` | Erstellt und signiert einen DeletionProof (Version 3) mit Ed25519. | `src/deletion_proof.rs` |
| `pub fn create_v3_with_audit_position( scope: DeletionScope, deleted_keys: Vec<Vec<u8>>, deleted_after_tx: TxId, covered_layers: Vec<LayerCleanupProof>, excluded_scopes: Vec<ExcludedScope>, audit_chain_position: Option<u64>, attested_at: i64, graph_repair: &[G…` | Erstellt und signiert einen DeletionProof (Version 3, Ed25519) inklusive Verweis auf den Audit-Chain-Eintrag. | `src/deletion_proof.rs` |
| `pub fn create_with_wal_receipt( scope: DeletionScope, mut deleted_keys: Vec<Vec<u8>>, deleted_after_tx: TxId, covered_layers: Vec<LayerCleanupProof>, excluded_scopes: Vec<ExcludedScope>, wal_chain_receipt: Option<[u8; 32]>, proof_key: &[u8], ) -> Result<Self>` | Erstellt und signiert einen DeletionProof (Version 2) inklusive optionaler WAL-HMAC-Kettenquittung. | `src/deletion_proof.rs` |
| `pub fn create_with_wal_receipt_v3( scope: DeletionScope, deleted_keys: Vec<Vec<u8>>, deleted_after_tx: TxId, covered_layers: Vec<LayerCleanupProof>, excluded_scopes: Vec<ExcludedScope>, wal_chain_receipt: Option<[u8; 32]>, attested_at: i64, graph_repair: &[Gr…` | Erstellt und signiert einen DeletionProof (Version 3, Ed25519) inklusive optionaler WAL-HMAC-Kettenquittung. | `src/deletion_proof.rs` |
| `pub fn deserialize<'de, D>(deserializer: D) -> Result<ScanTechnique, D::Error> where D: Deserializer<'de>,` | — | `src/deletion_proof.rs` |
| `pub fn export_for_audit(&self) -> Result<String>` | Exportiert Proof als JSON für Compliance-Dokumentation. ExcludedScope-Liste ist maschinenlesbar enthalten. | `src/deletion_proof.rs` |
| `pub fn generate() -> Self` | Generates a new random Ed25519 keypair using OsRng. | `src/deletion_proof.rs` |
| `pub fn hash_deleted_keys_length_prefixed(deleted_keys: &[Vec<u8>]) -> [u8` | Berechnet den Blake3-Hash einer deterministisch sortierten Liste gelöschter Schlüssel mit Längenpräfix. DOKUMENTATION ZUM KOLLISIONSRISIKO:… | `src/deletion_proof.rs` |
| `pub fn issue<'a>( ctx: &crate::types_local::RequestContext, saga: &crate::types_local::SagaRecord, witnesses: &[LayerWitness], excluded: &[ExcludedScope], custody: CustodyLevel, key: impl Into<SigningKeyHandle<'a>>, clock: &dyn crate::types_local::Clock, ) ->…` | Issues a new `DeletionProof` based on verified `LayerWitness` measurements. # Fail-Closed Validation (INV-WITNESS-MINT-1 & REQ-PRF-2) Retur… | `src/deletion_proof.rs` |
| `pub fn layer(&self) -> &DeletionLayer` | Gibt den zugrundeliegenden DeletionLayer zurück. | `src/deletion_proof.rs` |
| `pub fn map_witness_layer_to_deletion_layer( layer: &WitnessLayer, witness: &LayerWitness, ) -> Result<DeletionLayer>` | Mappt `WitnessLayer` erschöpfend auf `DeletionLayer` ohne `_`-Zweig. | `src/deletion_proof.rs` |
| `pub fn new_after_verified_empty( layer: DeletionLayer, remaining_live_entries: usize, ) -> Result<Self>` | Erzeugt einen Proof NUR, wenn `remaining_live_entries == 0` — also nur dann, wenn der Aufrufer nachweislich (durch einen Re-Scan des betrof… | `src/deletion_proof.rs` |
| `pub fn serialize<S>(tech: &ScanTechnique, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer,` | — | `src/deletion_proof.rs` |
| `pub fn signature_version_typed( &self, ) -> std::result::Result<SignatureVersion, DeletionProofError>` | Parst und liefert die typsichere Signaturversion des Beweises. # Errors Gibt [`DeletionProofError::UnsupportedVersion`] zurück, wenn `signa… | `src/deletion_proof.rs` |
| `pub fn signing_key(&self) -> &ed25519_dalek::SigningKey` | Returns a reference to the signing key. | `src/deletion_proof.rs` |
| `pub fn tenant_id(&self) -> TenantId` | Gibt die Tenant-ID aus dem Scope zurück. | `src/deletion_proof.rs` |
| `pub fn verify<'a>(&self, key: impl Into<VerificationKey<'a>>) -> Result<bool>` | Verifiziert Signatur. Referenz: Befund v17 Teil 10.1 & INVARIANTE INV-DELETION-1. NICHT-GARANTIE: Prüft nur Signatur, nicht ob Storage tats… | `src/deletion_proof.rs` |
| `pub fn verify_and_create<F>(layer: DeletionLayer, verification: F) -> Result<Self> where F: FnOnce() -> Result<bool>,` | Erzeugt einen Proof für `layer` NUR, wenn `verification` bestätigt, dass die physische Bereinigung tatsächlich abgeschlossen ist. `verifica… | `src/deletion_proof.rs` |
| `pub fn verify_external( &self, public_key: &ed25519_dalek::VerifyingKey, ) -> std::result::Result<(), CryptoError>` | Verifies this proof using ONLY the provided Ed25519 public key. Does NOT require access to any `KeyManager` state — this is the verificatio… | `src/deletion_proof.rs` |
| `pub fn verify_wal_delete_receipt( receipt: &[u8; 32], prev_hmac: &[u8; 32], delete_event_payload: &[u8], integrity_key: &[u8], ) -> Result<bool>` | Verifiziert eine WAL-Löschquittung in O(1) konstanter Zeit ohne Klartextzugang. | `src/deletion_proof.rs` |
| `pub fn verifying_key_bytes(&self) -> [u8` | Returns the 32-byte representation of the verifying public key. | `src/deletion_proof.rs` |
| `pub fn from_signing_key(signing_key: ed25519_dalek::SigningKey) -> Self` | Erzeugt ein `DeletionProofKeyPair` aus einem bestehenden Signierschlüssel. | `src/ed25519_proof.rs` |
| `pub fn generate() -> Self` | Generiert ein neues Ed25519-Schlüsselpaar unter Nutzung des System-RNGs (`OsRng`). | `src/ed25519_proof.rs` |
| `pub fn generate_with_rng<R: RngCore + CryptoRng>(rng: &mut R) -> Self` | Generiert ein neues Ed25519-Schlüsselpaar unter Nutzung eines beliebigen RNGs. | `src/ed25519_proof.rs` |
| `pub fn hash_deleted_keys_length_prefixed(keys: &[Vec<u8>]) -> [u8` | Berechnet den BLAKE3-Hash über längenpräfixierte gelöschte Schlüssel. | `src/ed25519_proof.rs` |
| `pub fn sign_deletion_proof_v3(keypair: &DeletionProofKeyPair, proof_payload: &[u8]) -> [u8` | Signiert den Payload eines DeletionProofs unter Nutzung von Ed25519 (Version 3). | `src/ed25519_proof.rs` |
| `pub fn signing_key(&self) -> &ed25519_dalek::SigningKey` | Gibt eine Referenz auf den Signierschlüssel zurück. | `src/ed25519_proof.rs` |
| `pub fn verify_deletion_proof_v3( verifying_key_bytes: &[u8; 32], proof_payload: &[u8], signature_bytes: &[u8; 64], ) -> Result<(), DeletionProofError>` | Verifiziert die Ed25519-Signatur eines DeletionProofs unabhängig von internem Acta-State (Version 3). | `src/ed25519_proof.rs` |
| `pub fn verifying_key_bytes(&self) -> [u8` | Gibt die Bytes des öffentlichen Verifikationsschlüssels (32 Bytes) zurück. | `src/ed25519_proof.rs` |
| `pub fn arm(self, _ev: ControlEvidence) -> Scanner<Armed>` | Einzige Brücke nach Armed; erfordert Positivkontrollbeleg (TY-10). | `src/erasure_domain.rs` |
| `pub fn bytes(&self) -> u64` | Liefert die Anzahl geprüfter Bytes. | `src/erasure_domain.rs` |
| `pub fn class(&self) -> MeasureClass` | Liefert die Messklasse (MK1-MK3). | `src/erasure_domain.rs` |
| `pub fn condition(&self) -> RiskCondition` | Statische Bedingung für Audit- und Verifikationswerkzeuge (AR-4). | `src/erasure_domain.rs` |
| `pub fn duration(&self) -> Duration` | Liefert die Scandauer. | `src/erasure_domain.rs` |
| `pub fn files(&self) -> u64` | Liefert die Anzahl geprüfter Dateien. | `src/erasure_domain.rs` |
| `pub fn finish( self, matches: u64, duration: Duration, hash: [u8; 32], ) -> Result<LayerWitness, Residue>` | Schließt die Messung ab. matches != 0 => Err(Residue). LayerWitness wird ausschließlich bei matches == 0 ausgegeben (TY-10, I-19, D-72). | `src/erasure_domain.rs` |
| `pub fn hash(&self) -> &[u8` | Liefert den kryptografischen Hash des Prüflaufs. | `src/erasure_domain.rs` |
| `pub fn id(&self) -> &'static str` | Feste Identifikator-Zeichenkette nach SPEC Anhang D. | `src/erasure_domain.rs` |
| `pub fn layer(&self) -> LayerId` | Liefert die Schicht-ID. | `src/erasure_domain.rs` |
| `pub fn method_id(&self) -> u16` | Liefert die Methoden-ID. | `src/erasure_domain.rs` |
| `pub fn new(layer: LayerId, class: MeasureClass, method_id: u16) -> Self` | Erzeugt einen neuen unbewaffneten Scanner. | `src/erasure_domain.rs` |
| `pub fn record(&mut self, files: u64, bytes: u64)` | Zeichnet geprüfte Dateien und Bytes auf. | `src/erasure_domain.rs` |
| `pub fn add_batch( requests: Vec<BatchGateRequest>, current_seq: Seq, ) -> Result<BatchGateOutcome, ErasureError>` | Freistehende `add_batch`-Funktion (SPEC §11.3, D-57, D-78). Fügt Gate-Anfragen im Batch mit atomarer Inkrementierung von `gate_epoch` und A… | `src/gate/batch.rs` |
| `pub fn bind_global_erased_set(set: Arc<RwLock<ErasedSet>>)` | Initialisiert oder bindet das globale ErasedSet für freistehende `add_batch`-Aufrufe. | `src/gate/batch.rs` |
| `pub fn evaluate_restricted_access( purpose: &str, is_restricted_only: bool, batch_size: usize, ) -> Result<bool, ErasureError>` | Prüft den Zugriff auf `Restricted`-Daten nach Art. 18 Abs. 2 (SPEC D-116, T-RESTR-2..4). Stapelgrenze höchstens 1 000 Elemente (T-RESTR-4). | `src/gate/batch.rs` |
| `pub async fn execute_batch( &self, requests: Vec<BatchGateRequest>, gated_seq: Seq, ) -> Result<BatchGateOutcome, ErasureError>` | Führt den Gruppen-Commit für die übergebenen Anfragen unter `CommitBarrier::exclusive` aus. Invarianten und Zusicherungen: - `gate_epoch +=… | `src/gate/batch.rs` |
| `pub fn new( gate_id: [u8; 16], lo: KeyBuf, hi: KeyBuf, is_erased: bool, erasure_id: [u8; 16], ) -> Self` | Erstellt eine Standard-Gate-Anfrage aus 16-Byte-Schlüsseln. | `src/gate/batch.rs` |
| `pub fn new( barrier: Arc<dyn CommitBarrier>, erased_set: Arc<RwLock<ErasedSet>>, gate_log: Arc<RwLock<GateLog>>, ) -> Self` | Erstellt einen neuen Koordinator für Gate-Gruppen-Commits. | `src/gate/batch.rs` |
| `pub fn with_bounds_65( gate_id: [u8; 16], lo_65: [u8; 65], hi_65: [u8; 65], is_erased: bool, erasure_id: [u8; 16], ) -> Self` | Erstellt eine Gate-Anfrage mit exakten 65-Byte Grenzen (SPEC 9.1, D-123). | `src/gate/batch.rs` |
| `pub fn add_erased(&mut self, lo: KeyBuf, hi: KeyBuf, gated_seq: Seq)` | Fügt einen `Erased`-Bereich hinzu. | `src/gate/erased_set.rs` |
| `pub fn add_erased_with_bounds( &mut self, gate_id: [u8; 16], lo: [u8; 65], hi: [u8; 65], gated_seq: Seq, )` | Fügt einen `Erased`-Bereich mit 65-Byte-Grenzen hinzu. | `src/gate/erased_set.rs` |
| `pub fn add_prefix_seq(&mut self, prefix: [u8; 65], seq_lt: Seq, gated_seq: Seq)` | Fügt ein PrefixSeq-Gate hinzu (SPEC D-117). | `src/gate/erased_set.rs` |
| `pub fn add_restricted(&mut self, lo: KeyBuf, hi: KeyBuf, gated_seq: Seq)` | Fügt einen `Restricted`-Bereich hinzu. | `src/gate/erased_set.rs` |
| `pub fn add_restricted_with_bounds( &mut self, gate_id: [u8; 16], lo: [u8; 65], hi: [u8; 65], gated_seq: Seq, )` | Fügt einen `Restricted`-Bereich mit 65-Byte-Grenzen hinzu. | `src/gate/erased_set.rs` |
| `pub fn check(&self, k: &KeyBuf) -> GateDecision` | Prüft einen 16-Byte-Schlüssel (O(log n) mit binärer Suche auf sortierten Einträgen). | `src/gate/erased_set.rs` |
| `pub fn check_bound(&self, b: &KeyBound) -> GateDecision` | Prüft eine Grenze. | `src/gate/erased_set.rs` |
| `pub fn check_raw(&self, raw: &[u8; 78]) -> GateDecision` | Prüft einen kanonischen 78-Byte-Schlüssel (SPEC 9.1, D-123). | `src/gate/erased_set.rs` |
| `pub fn covers(&self, k: &KeyBuf) -> bool` | Prüft, ob ein 16-Byte KeyBuf gegated ist (Erased oder Restricted, für Replay/Commit, I-2). | `src/gate/erased_set.rs` |
| `pub fn covers_raw(&self, raw: &[u8; 78]) -> bool` | Prüft, ob ein 78-Byte Schlüssel gegated ist (Erased oder Restricted, für Replay/Commit, I-2). | `src/gate/erased_set.rs` |
| `pub fn gate_epoch(&self) -> u64` | Liefert die aktuelle Gate-Epoche. | `src/gate/erased_set.rs` |
| `pub fn gated_seq(&self, k: &KeyBuf) -> Option<Seq>` | Liefert die Sequenznummer, bei der ein Schlüssel eingesprrt wurde. | `src/gate/erased_set.rs` |
| `pub fn gated_seq_raw(&self, raw: &[u8; 78]) -> Option<Seq>` | Liefert die Sequenznummer, bei der ein 78-Byte Schlüssel eingesprrt wurde. | `src/gate/erased_set.rs` |
| `pub fn increment_epoch(&self) -> u64` | Erhöht die Gate-Epoche atomar (vor der Schlüsselvernichtung, I-21). | `src/gate/erased_set.rs` |
| `pub fn lift_restriction(&mut self, lo: &KeyBuf, hi: &KeyBuf)` | Hebt eine Restriktion auf (ER-15). | `src/gate/erased_set.rs` |
| `pub fn new() -> Self` | Erstellt einen leeren Gate-Satz. | `src/gate/erased_set.rs` |
| `pub fn remove_erased(&mut self, lo: &KeyBuf, hi: &KeyBuf)` | Entfernt ein Erased-Gate (bei Completed, ER-12 / L-24). | `src/gate/erased_set.rs` |
| `pub fn remove_gate(&mut self, gate_id: &[u8; 16])` | Entfernt ein Gate über dessen ID. | `src/gate/erased_set.rs` |
| `pub fn variant_for_range(&self, r: &KeyRange) -> RangeGate` | Bereichsrichtlinie für Kompaktion (I-18): Restricted > Erased > Open. | `src/gate/erased_set.rs` |
| `pub fn add_fenced_key(&self, key: [u8; 16])` | Fügt einen Einzelschlüssel zum Fence hinzu (inherente Hilfsmethode). | `src/gate/fence.rs` |
| `pub fn contains_key(&self, key: &[u8]) -> bool` | Prüft, ob ein gegebener Schlüssel in diesen Bereich fällt. | `src/gate/fence.rs` |
| `pub fn inflight_count(&self) -> u32` | Gibt die Anzahl aktuell registrierter Inflight-Schreiber zurück. | `src/gate/fence.rs` |
| `pub fn is_fenced(&self, key: impl AsRef<[u8]>) -> bool` | Synchroner Check auf einen Schlüssel (inherente Methode für Abwärtskompatibilität). | `src/gate/fence.rs` |
| `pub fn mark_corrupted(&self)` | Markiert den Fence-Zustand explizit als korrumpiert (Fail-Closed-Simulation und -Verifikation). | `src/gate/fence.rs` |
| `pub fn new() -> Self` | Erzeugt eine neue, unbeschädigte Instanz von `InMemoryFenceGate`. | `src/gate/fence.rs` |
| `pub fn overlaps(&self, other: &Self) -> bool` | Prüft, ob sich zwei Bereiche überschneiden. | `src/gate/fence.rs` |
| `pub fn append(&mut self, entry: GateLogEntry)` | Fügt einen Eintrag hinzu. | `src/gate/log.rs` |
| `pub fn append_batch(&mut self, new_entries: Vec<GateLogEntry>)` | Fügt mehrere Einträge im Batch hinzu. | `src/gate/log.rs` |
| `pub fn compact(&self) -> Self` | Kompaktiert das Gate-Log: Entfernt alle `Add`-Einträge, deren `gate_id` durch ein späteres `Remove` gelöscht wurde, und verwirft die `Remov… | `src/gate/log.rs` |
| `pub fn entries(&self) -> &[GateLogEntry]` | Liefert alle Einträge. | `src/gate/log.rs` |
| `pub fn from_entries(entries: Vec<GateLogEntry>) -> Self` | Lädt Einträge aus einer Liste. | `src/gate/log.rs` |
| `pub fn is_empty(&self) -> bool` | Prüft ob das Log leer ist. | `src/gate/log.rs` |
| `pub fn len(&self) -> usize` | Anzahl der Einträge. | `src/gate/log.rs` |
| `pub fn new() -> Self` | Erstellt ein leeres Gate-Log. | `src/gate/log.rs` |
| `pub fn replay_into(&self, set: &mut crate::gate::erased_set::ErasedSet)` | Wendet die Einträge des Gate-Logs auf ein `ErasedSet` an (Recovery I-2). | `src/gate/log.rs` |
| `pub fn linearized_read<T, F>(&self, k: &KeyBuf, mut f: F) -> Result<T, GateDecision> where F: FnMut() -> Result<T, GateDecision>,` | Führt einen linearisierten Lesevorgang aus (SPEC D-120, I-21). Prüft das Gate vor und nach der übergebenen Lese-/Entschlüsselungs-Operation… | `src/gate/read_gate.rs` |
| `pub fn linearized_read_raw<T, F>(&self, raw: &[u8; 78], mut f: F) -> Result<T, GateDecision> where F: FnMut() -> Result<T, GateDecision>,` | Führt einen linearisierten Lesevorgang auf einem 78-Byte-Schlüssel aus (SPEC D-120, D-123, I-21). | `src/gate/read_gate.rs` |
| `pub fn new(set: Arc<RwLock<ErasedSet>>) -> Self` | Erstellt einen neuen Gate-Wächter. | `src/gate/read_gate.rs` |
| `pub fn read_exact_at(f: &dyn VfsFile, buf: &mut [u8], offset: u64) -> Result<(), ErasureError>` | Liest einen vollständigen Puffer von einer Dateiposition. | `src/io.rs` |
| `pub fn write_all_at(f: &dyn VfsFile, buf: &[u8], offset: u64) -> Result<(), ErasureError>` | Schreibt einen vollständigen Puffer an eine Dateiposition und liefert einen Fehler, wenn weniger als `buf.len()` Bytes geschrieben wurden. | `src/io.rs` |
| `pub fn zero_at(f: &dyn VfsFile, offset: u64, len: usize) -> Result<(), ErasureError>` | Schreibt Nullen an eine Dateiposition ohne Heap-Allokation in einer Block-Schleife. Alle I/O-Fehler sind typisiert und enthalten keine Date… | `src/io.rs` |
| `pub fn append(&mut self, entry: JournalEntry)` | Fügt einen neuen Eintrag an. | `src/journal.rs` |
| `pub fn entries(&self) -> &[JournalEntry]` | Gibt alle Einträge zurück. | `src/journal.rs` |
| `pub fn new() -> Self` | Erstellt ein neues leeres In-Memory-Journal. | `src/journal.rs` |
| `pub fn open_durable( vfs: Arc<dyn Vfs>, path: &Path, derived_key: &[u8; 32], ) -> Result<DurableJournal, ErasureError>` | Öffnet ein dauerhaftes Journal auf Vfs. | `src/journal.rs` |
| `pub fn recover_open_sagas(&self) -> Vec<EraseSaga>` | Rekonstruiert den Zustand aller noch nicht abgeschlossenen Sagas aus dem Journal (SPEC §11.2, §11.6). Jede nicht-vollendete Löschung (nicht… | `src/journal.rs` |
| `pub fn append(&mut self, entry: JournalEntry)` | Fügt einen Eintrag hinzu. | `src/journal/appendlog.rs` |
| `pub fn append_record(&mut self, rec: &JournalRecord) -> Result<(), ErasureError>` | Fügt einen neuen Record an und synchronisiert (`fsync`). | `src/journal/appendlog.rs` |
| `pub fn entries(&self) -> &[JournalEntry]` | Liefert alle Einträge. | `src/journal/appendlog.rs` |
| `pub fn is_empty(&self) -> bool` | Prüft ob das Log leer ist. | `src/journal/appendlog.rs` |
| `pub fn len(&self) -> usize` | Anzahl der Einträge. | `src/journal/appendlog.rs` |
| `pub fn new() -> Self` | Erstellt ein leeres Log. | `src/journal/appendlog.rs` |
| `pub fn open_or_create( vfs: Arc<dyn Vfs>, path: &Path, derived_key: &[u8; 32], ) -> Result<Self, ErasureError>` | Öffnet ein existierendes Journal oder legt ein neues an. | `src/journal/appendlog.rs` |
| `pub fn records(&self) -> &[JournalRecord]` | Liefert alle Records. | `src/journal/appendlog.rs` |
| `pub fn recover_open_sagas(&self) -> Vec<EraseSaga>` | Rekonstruiert den Zustand aller noch nicht abgeschlossenen Sagas aus dem Journal (SPEC §11.2, §11.6). | `src/journal/appendlog.rs` |
| `pub fn recover_open_sagas(&self) -> Vec<EraseSaga>` | Rekonstruiert den Zustand aller noch nicht abgeschlossenen Sagas aus dem Journal (SPEC §11.2, §11.6). | `src/journal/appendlog.rs` |
| `pub fn reduce_completed(&mut self) -> Result<(), ErasureError>` | Führt die Reduktion nach Completed (R13) via atomarem Dateiaustausch durch. | `src/journal/appendlog.rs` |
| `pub fn compute_commitment(commit_key: &[u8; 32], subject: &[u8], tenant: &[u8]) -> [u8` | Berechnet das kryptografische Commitment für Journal und Bericht nach SPEC §11.6 und D-125: `BLAKE3-keyed(K_commit(epoch), "ctx/commit/v1"… | `src/journal/epoch.rs` |
| `pub fn first(started_at: u64) -> Self` | Erstellt die erste Epoche. | `src/journal/epoch.rs` |
| `pub fn next(&self, now: u64) -> Self` | Rotiert zur nächsten Epoche. | `src/journal/epoch.rs` |
| `pub fn as_u8(self) -> u8` | Gibt den numerischen Code zurück. | `src/journal/record.rs` |
| `pub fn as_u8(self) -> u8` | Gibt den numerischen Code zurück. | `src/journal/record.rs` |
| `pub fn decode(bytes: &[u8]) -> Result<Self, ErasureError>` | Deserialisiert den Klartext aus Bytes nach SPEC F.9. | `src/journal/record.rs` |
| `pub fn encode(&self) -> Vec<u8>` | Serialisiert den Klartext nach SPEC F.9 in Bytes: `erasure_id(16) ‖ record_code(u8) ‖ time(u64) ‖ commitment(16) ‖ target_type(u8) ‖ retent… | `src/journal/record.rs` |
| `pub fn from_record(rec: &JournalRecord) -> Self` | Konvertiert einen `JournalRecord` in einen `JournalEntry`. | `src/journal/record.rs` |
| `pub fn from_u8(val: u8) -> Result<Self, ErasureError>` | Konvertiert ein Byte in `RecordCode`. | `src/journal/record.rs` |
| `pub fn from_u8(val: u8) -> Result<Self, ErasureError>` | Konvertiert ein Byte in `TargetType`. | `src/journal/record.rs` |
| `pub fn new_v2( erasure_id: [u8; 16], record_code: RecordCode, timestamp: u64, commitment: [u8; 16], target_type: TargetType, retention_code: u8, phase: u8, reason: u8, scope_tags: Vec<u8>, counters: u64, witness_hashes: Vec<[u8; 32]>, ) -> Self` | Erzeugt einen neuen `JournalRecord`. | `src/journal/record.rs` |
| `pub fn new_v2( erasure_id: [u8; 16], record_code: RecordCode, timestamp: u64, commitment: [u8; 16], target_type: TargetType, retention_code: u8, phase: u8, reason: u8, scope_tags: Vec<u8>, counters: u64, witness_hashes: Vec<[u8; 32]>, ) -> JournalRecord` | Additiver Konstruktor `new_v2` für erweiterte Initialisierung. | `src/journal/record.rs` |
| `pub fn to_record(&self) -> JournalRecord` | Konvertiert `self` in einen `JournalRecord`. | `src/journal/record.rs` |
| `pub fn to_saga_state(self) -> SagaState` | Mappt RecordCode auf den entsprechenden SagaState. | `src/journal/record.rs` |
| `pub fn reduce(entries: &[JournalEntry]) -> Vec<JournalEntry>` | Reduziert Journal-Einträge: Behält nur den letzten Eintrag je `erasure_id` nach `Completed` (R13). | `src/journal/reduce.rs` |
| `pub fn dispose_scope_wal( sink: &dyn WalSink, scope: &WalScope, clock: &dyn Clock, ) -> Result<WalDisposalReceipt, WalSinkError>` | Führt die WAL-Entsorgung für den angegebenen Scope synchron durch. Gemäß INV-ERA-6 werden WAL-Einträge eines gelöschten Scopes entsorgt ode… | `src/journal/scope_purge.rs` |
| `pub fn is_scope_purged(&self, scope: &KeyScope) -> bool` | Prüft, ob ein Scope bereits als im WAL bereinigt markiert ist. | `src/journal/wal.rs` |
| `pub fn markers_count(&self) -> usize` | Gibt alle registrierten Marker zurück. | `src/journal/wal.rs` |
| `pub fn new() -> Self` | Erzeugt eine neue `MemoryWalSink`. | `src/journal/wal.rs` |
| `pub fn add_entry(&self, id: u64, scope: WalScope, payload: Vec<u8>)` | Fügt einen Test-WAL-Eintrag hinzu. | `src/journal/wal_sink.rs` |
| `pub fn get_integrity_key(&self, scope_id: &[u8; 16]) -> Option<[u8` | Liest den Integritätsschlüssel eines Scopes, sofern noch vorhanden. | `src/journal/wal_sink.rs` |
| `pub fn new() -> Self` | Erzeugt eine neue leere Instanz. | `src/journal/wal_sink.rs` |
| `pub fn prefix(&self) -> &[u8]` | Gibt das optionale Präfix zurück. | `src/journal/wal_sink.rs` |
| `pub fn read_entries_for_scope(&self, scope: &WalScope) -> Vec<Vec<u8>>` | Liest lesbare Einträge eines Scopes. Wenn der Schlüssel verworfen wurde oder Einträge getilgt wurden, ist kein Klartext verfügbar. | `src/journal/wal_sink.rs` |
| `pub fn scope_id(&self) -> &[u8` | Gibt die 16-Byte Scope-ID zurück. | `src/journal/wal_sink.rs` |
| `pub fn set_fail_on_dispose(&self, fail: bool)` | Simuliert einen Fehler bei `dispose_entries`. | `src/journal/wal_sink.rs` |
| `pub fn set_fail_on_enumerate(&self, fail: bool)` | Simuliert einen Fehler bei `enumerate_scope_entries`. | `src/journal/wal_sink.rs` |
| `pub fn set_fail_on_key_discard(&self, fail: bool)` | Simuliert einen Fehler bei `discard_scope_integrity_key`. | `src/journal/wal_sink.rs` |
| `pub fn set_integrity_key(&self, scope_id: [u8; 16], key: [u8; 32])` | Hinterlegt einen Integritätsschlüssel für einen Scope. | `src/journal/wal_sink.rs` |
| `pub fn with_prefix(scope_id: [u8; 16], prefix: Vec<u8>) -> Self` | Erzeugt einen neuen `WalScope` mit Kennung und Schlüsselpräfix. | `src/journal/wal_sink.rs` |
| `pub fn decrypt_envelope_record( &self, payload: &EncryptedRecordPayload, ) -> Result<Vec<u8>, ErasureError>` | Entschlüsselt einen Datensatz mittels Envelope Encryption. | `src/kv_shredding.rs` |
| `pub fn decrypt_envelope_record_epoch( &self, payload: &EncryptedEpochRecordPayload, ) -> Result<Vec<u8>, ErasureError>` | Entschlüsselt einen Datensatz unter Envelope Encryption mit Epochenprüfung. | `src/kv_shredding.rs` |
| `pub fn decrypt_record( &self, group_id: u64, record_id: &str, dek_bytes: &[u8; 32], payload: &[u8], ) -> Result<Vec<u8>, ErasureError>` | Entschlüsselt Daten für einen Record mit AES-256-GCM-SIV. | `src/kv_shredding.rs` |
| `pub fn decrypt_with_group( &self, group_id: u64, ciphertext: &[u8], nonce_bytes: &[u8; 12], ) -> Result<Vec<u8>, ErasureError>` | Entschlüsselt Chiffretext direkt unter dem Gruppen-KEK für `group_id`. | `src/kv_shredding.rs` |
| `pub fn derive_subkey(master_key: &[u8; 32], group_id: u64, epoch: u64) -> SubKey` | Leitet einen Sub-Key aus einem Masterschlüssel deterministisch via BLAKE3 KDF ab. | `src/kv_shredding.rs` |
| `pub fn encrypt_envelope_record( &self, group_id: u64, record_id: u64, plaintext: &[u8], ) -> Result<EncryptedRecordPayload, ErasureError>` | Verschlüsselt einen Datensatz mittels Envelope Encryption. | `src/kv_shredding.rs` |
| `pub fn encrypt_envelope_record_epoch( &self, group_id: u64, epoch: u64, record_id: u64, plaintext: &[u8], ) -> Result<EncryptedEpochRecordPayload, ErasureError>` | Verschlüsselt einen Datensatz unter Envelope Encryption gebunden an Gruppe und Epoche. | `src/kv_shredding.rs` |
| `pub fn encrypt_record( &self, group_id: u64, record_id: &str, plaintext: &[u8], ) -> Result<Vec<u8>, ErasureError>` | Verschlüsselt Daten für einen Record mit AES-256-GCM-SIV. | `src/kv_shredding.rs` |
| `pub fn encrypt_with_group( &self, group_id: u64, plaintext: &[u8], ) -> Result<(Vec<u8>, [u8` | Verschlüsselt direkt unter dem Gruppen-KEK für `group_id`. | `src/kv_shredding.rs` |
| `pub fn get_or_derive(&self, group_id: u64) -> Result<SubKey, ErasureError>` | Ermittelt oder erzeugt den Group KEK für `group_id`. | `src/kv_shredding.rs` |
| `pub fn get_or_derive_epoch(&self, group_id: u64, epoch: u64) -> Result<SubKey, ErasureError>` | Ermittelt oder erzeugt den Group-KEK für ein spezifisches `(group_id, epoch)`-Paar. | `src/kv_shredding.rs` |
| `pub fn get_wrapped_dek(&self, group_id: u64, record_id: u64) -> Option<(Vec<u8>, [u8` | Liefert den gewrappten DEK und die Nonce eines Records, falls vorhanden und aktiv. | `src/kv_shredding.rs` |
| `pub fn get_wrapped_kek(&self, group_id: u64) -> Option<(Vec<u8>, [u8` | Liefert den gewrappten KEK und die Nonce einer Gruppe, falls vorhanden und aktiv. | `src/kv_shredding.rs` |
| `pub fn is_group_active(&self, group_id: u64) -> bool` | Prüft ob eine Gruppe aktiv ist. | `src/kv_shredding.rs` |
| `pub fn is_group_epoch_active(&self, group_id: u64, epoch: u64) -> bool` | Prüft ob eine Gruppe in einer bestimmten Epoche aktiv ist. | `src/kv_shredding.rs` |
| `pub fn is_group_epoch_revoked(&self, group_id: u64, epoch: u64) -> bool` | Prüft ob eine Gruppe in einer bestimmten Epoche widerrufen ist. | `src/kv_shredding.rs` |
| `pub fn is_group_revoked(&self, group_id: u64) -> bool` | Prüft ob eine Gruppe widerrufen ist. | `src/kv_shredding.rs` |
| `pub fn is_key_active(&self, group_id: u64) -> bool` | Alias für `is_group_active`. | `src/kv_shredding.rs` |
| `pub fn is_record_active(&self, record_id: &str) -> bool` | Prüft ob ein Datensatz anhand seines String-Schlüssels aktiv ist. | `src/kv_shredding.rs` |
| `pub fn is_record_num_active(&self, group_id: u64, record_id: u64) -> bool` | Prüft ob ein numerischer Datensatz aktiv ist. | `src/kv_shredding.rs` |
| `pub fn new() -> Self` | Erzeugt eine neue `KeyRegistry` mit zufälligem Master-KEK. | `src/kv_shredding.rs` |
| `pub fn purge_keys( &self, group_id: u64, epoch: u64, clock: Option<&dyn crate::types_local::Clock>, ) -> Result<KeyPurgeReceipt, KeyShredError>` | Physische und kryptographische Schlüsselvernichtung (`purge_keys`) für eine Gruppe und Epoche. Herkunft: ffeb79555:crates/acta-crypto/src/s… | `src/kv_shredding.rs` |
| `pub fn revoke_group(&self, group_id: u64) -> Result<(), ErasureError>` | Widerruft eine gesamte Shredding-Gruppe in O(1) und nullt das Schlüsselmaterial im RAM. | `src/kv_shredding.rs` |
| `pub fn revoke_record(&self, record_id: &str) -> Result<(), ErasureError>` | Widerruft einen einzelnen Datensatz anhand seines String-Schlüssels. | `src/kv_shredding.rs` |
| `pub fn revoke_record_num(&self, group_id: u64, record_id: u64) -> bool` | Widerruft einen einzelnen Datensatz anhand seiner numerischen ID. | `src/kv_shredding.rs` |
| `pub fn revoke_subkey(&self, group_id: u64) -> Result<(), ErasureError>` | Alias für `revoke_group`. | `src/kv_shredding.rs` |
| `pub fn with_revocation_log(mut self, log: Arc<RevocationLog>) -> Self` | Bindet ein `RevocationLog` an und synchronisiert bereits protokollierte Widerrufe. | `src/kv_shredding.rs` |
| `pub fn finalize_erasure( vfs: &dyn Vfs, ledger_sink: Option<&dyn LedgerSink>, params: &FinalizeParams, ) -> Result<FinalizeOutcome, ErasureError>` | Führt die Abschlussreihenfolge nach SPEC §11.8 idempotent durch: 1. Zeugen prüfen / Bericht kanonisch serialisieren und signieren. 2. `eras… | `src/ledger.rs` |
| `pub fn new() -> Self` | Erstellt ein neues leeres In-Memory-Ledger. | `src/ledger.rs` |
| `pub fn append_batch( &mut self, items: &[BatchRecordItem], ) -> Result<LedgerAnchorProof, LedgerError>` | Führt einen atomaren Batch-Append von Einträgen durch (INV-ERA-4). Synchronisiert vor Rückgabe (`sync`) und liefert einen `LedgerAnchorProo… | `src/ledger/durable.rs` |
| `pub fn detect_time_regression(&self, store_current_seq: u64) -> bool` | Prüft auf Zeitregression: `store_current_seq < max(gated_seq)` (ER-30). | `src/ledger/durable.rs` |
| `pub fn last_hash(&self) -> &[u8` | Liefert den aktuellen Ketten-Hash. | `src/ledger/durable.rs` |
| `pub fn ledger_floor(&self) -> u64` | Liefert den ledger_floor: `max(gated_seq)` (ER-30). | `src/ledger/durable.rs` |
| `pub fn open( storage: Box<dyn LedgerStorage>, ledger_key: [u8; 32], create_if_missing: bool, ) -> Result<Self, LedgerError>` | Öffnet oder erstellt ein ErasureLedger über den angegebenen Speicher. Fail-Closed Verhalten: - `create_if_missing == false` und Speicher le… | `src/ledger/durable.rs` |
| `pub fn records(&self) -> &[FramedRecord]` | Alle geladenen Records. | `src/ledger/durable.rs` |
| `pub fn verify_chain(&self) -> Result<(), LedgerError>` | Verifiziert die gesamte Hash-Kette aller Records im Ledger. | `src/ledger/durable.rs` |
| `pub fn new( commitment: [u8; 32], range_tags: Vec<u8>, gated_seq: u64, completed_at: u64, prev_hash: &[u8; 32], ledger_key: &[u8; 32], ) -> Self` | Erstellt einen neuen Ledger-Eintrag mit Hash-Kette. | `src/ledger/entry.rs` |
| `pub fn verify(&self, prev_hash: &[u8; 32], ledger_key: &[u8; 32]) -> bool` | Prüft die Integrität dieses Eintrags (MAC-Verifikation). | `src/ledger/entry.rs` |
| `pub fn entries(&self) -> Vec<LedgerEntry>` | Liefert alle Einträge. | `src/ledger/persistent.rs` |
| `pub fn open_or_create( vfs: Arc<dyn Vfs>, path: &Path, ledger_key: &[u8; 32], ) -> Result<Self, ErasureError>` | Öffnet oder erstellt ein persistentes `ErasureLedger`. Fail-Closed: Wenn die Datei existiert, aber beschädigt ist (z. B. MAC-Mismatch oder… | `src/ledger/persistent.rs` |
| `pub fn record_key_destruction( &self, commitment: [u8; 32], range_tags: Vec<u8>, gated_seq: u64, completed_at: u64, ) -> Result<LedgerEntry, ErasureError>` | Verankert eine Schlüsselvernichtung dauerhaft vor dem Abschluss der Saga (INV-ERA-4). | `src/ledger/persistent.rs` |
| `pub fn verify_integrity(&self) -> bool` | Prüft die Kette auf Integrität. | `src/ledger/persistent.rs` |
| `pub fn from_u8(v: u8) -> Result<Self, LedgerError>` | Parst den Record-Typ aus einem u8. | `src/ledger/record.rs` |
| `pub fn new() -> Self` | Erstellt einen Standard-Header. | `src/ledger/record.rs` |
| `pub fn parse(buf: &[u8]) -> Result<Self, LedgerError>` | Parst und validiert den Header aus einem Byte-Slice. | `src/ledger/record.rs` |
| `pub fn parse(buf: &[u8]) -> Result<(Self, usize), LedgerError>` | Parst einen Record aus einem Slice. Gibt den geparsten Record und die gelesene Bytezahl zurück. | `src/ledger/record.rs` |
| `pub fn serialize(&self) -> Vec<u8>` | Serialisiert den Record inklusive CRC32C Prüfsumme. | `src/ledger/record.rs` |
| `pub fn to_bytes(&self) -> [u8` | Serialisiert den Header in 16 Bytes. | `src/ledger/record.rs` |
| `pub fn append( &mut self, commitment: [u8; 32], range_tags: Vec<u8>, gated_seq: u64, completed_at: u64, ) -> Result<(), ErasureError>` | Fügt einen Eintrag hinzu (ER-30). | `src/ledger/sink.rs` |
| `pub fn append_batch( &mut self, items: &[BatchRecordItem], ) -> Result<LedgerAnchorProof, LedgerError>` | Führt einen atomaren Batch-Append mit Verankerungsnachweis durch (WP-8, INV-ERA-4). | `src/ledger/sink.rs` |
| `pub fn detect_time_regression(&self, store_current_seq: u64) -> bool` | Prüft auf Zeitregression: `store_seq < max(gated_seq)` (ER-30). | `src/ledger/sink.rs` |
| `pub fn entries(&self) -> &[LedgerEntry]` | Alle Einträge. | `src/ledger/sink.rs` |
| `pub fn last_hash(&self) -> &[u8` | Gibt den aktuellen Ketten-Hash zurück. | `src/ledger/sink.rs` |
| `pub fn ledger_floor(&self) -> u64` | Gibt den ledger_floor zurück: `max(gated_seq)` (ER-30). | `src/ledger/sink.rs` |
| `pub fn max_gated_seq(&self) -> u64` | Gibt die höchste gesehene gate_seq zurück. | `src/ledger/sink.rs` |
| `pub fn new(ledger_key: [u8; 32]) -> Self` | Erstellt ein leeres Ledger mit dem gegebenen MAC-Schlüssel. | `src/ledger/sink.rs` |
| `pub fn prune(&mut self, cutoff_us: u64)` | Entfernt Einträge jenseits von `backup_retention` (ER-30). `cutoff_us`: Mikrosekunden UTC, Einträge mit `completed_at < cutoff_us` werden e… | `src/ledger/sink.rs` |
| `pub fn verify_chain(&self) -> bool` | Prüft alle Einträge auf Integrität (Ketten-Verifikation). | `src/ledger/sink.rs` |
| `pub fn with_storage( storage: Box<dyn LedgerStorage>, ledger_key: [u8; 32], create_if_missing: bool, ) -> Result<Self, LedgerError>` | Erstellt ein Ledger mit persistentem `LedgerStorage`-Backend (WP-8). | `src/ledger/sink.rs` |
| `pub fn new() -> Self` | Erstellt einen leeren In-Memory-Speicher. | `src/ledger/storage.rs` |
| `pub fn new<P: AsRef<Path>>(path: P) -> Self` | Erstellt einen neuen dateibasierten Storage am angegebenen Pfad. | `src/ledger/storage.rs` |
| `pub fn path(&self) -> &Path` | Gibt den Dateipfad zurück. | `src/ledger/storage.rs` |
| `pub fn with_bytes(bytes: Vec<u8>) -> Self` | Erstellt einen In-Memory-Speicher mit initialen Bytes. | `src/ledger/storage.rs` |
| `pub fn arm(self, _evidence: LocalControlEvidence) -> ArmedScanner` | Schaltet den Scanner mit dem Positivkontrollnachweis scharf (`arm`, L-5, ER-25). | `src/measure.rs` |
| `pub fn finish(&self, matches: u64) -> Result<TestWitness, ErasureError>` | Schließt den Scan ab. Gibt `TestWitness` NUR zurück, wenn `matches == 0` (ER-25). | `src/measure.rs` |
| `pub fn kind(&self) -> EvidenceKind` | Gibt die Art des Nachweises zurück. | `src/measure.rs` |
| `pub fn new() -> Self` | Erstellt einen neuen unbewaffneten Scanner. | `src/measure.rs` |
| `pub fn new() -> Self` | Erstellt einen neuen Koordinator. | `src/measure.rs` |
| `pub fn run_positive_control( &self, pattern: &[u8], ) -> Result<LocalControlEvidence, ErasureError>` | Führt die Positivkontrolle auf einem bekannten Muster aus (ER-25). | `src/measure.rs` |
| `pub fn witness_hash(&self) -> &[u8` | Liefert den Zeugen-Hash. | `src/measure.rs` |
| `pub fn as_str(&self) -> &'static str` | Identifikator-String für den Bericht. | `src/measure/field.rs` |
| `pub fn collect_all_files(vfs: &dyn Vfs, dir: &Path) -> Result<Vec<PathBuf>, ErasureError>` | Rekursives Sammeln aller Dateipfade über Vfs (ohne Verzeichnisse selbst als Dateien einzufügen). | `src/measure/field.rs` |
| `pub fn is_clean(&self) -> bool` | Prüft ob MK3 vollständig bestanden (kein Schlüssel mehr ableitbar). | `src/measure/field.rs` |
| `pub fn new( layer: LayerId, class: MeasureClass, method_id: u16, root_dir: impl Into<PathBuf>, control_pattern: impl Into<Vec<u8>>, ) -> Self` | Erzeugt einen neuen ScanPlan. | `src/measure/field.rs` |
| `pub fn read_file_verified( vfs: &dyn Vfs, path: &Path, ) -> Result<(Vec<u8>, VerifiedVia), ErasureError>` | Liest Bytes einer Datei verifiziert über Vfs (SPEC D-113 Brücke). | `src/measure/field.rs` |
| `pub fn scan_mk1_structures( vfs: &dyn Vfs, root: &Path, pattern: &[u8], ) -> Result<Mk1Result, ErasureError>` | Scannt MK1-Strukturen (Manifeste, SSTable-Indizes, WAL-Köpfe) auf ein Schlüsselmuster. | `src/measure/field.rs` |
| `pub fn scan_mk2_raw(vfs: &dyn Vfs, root: &Path, pattern: &[u8]) -> Result<Mk2Result, ErasureError>` | Scannt MK2-Rohdaten aller Dateien (inklusive tmp/, alte Segmente, WAL, scope/, Berichte). | `src/measure/field.rs` |
| `pub fn scan_mk3_keys( top_slot_zeroed: bool, dek_derivation_fails: bool, file_slots_absent: bool, ) -> Mk3Result` | Führt MK3-Schlüsselprüfung durch (SPEC §12.3, Anhang F.3, D-79, D-113). | `src/measure/field.rs` |
| `pub fn with_control_file(mut self, path: impl Into<PathBuf>) -> Self` | Setzt einen expliziten Pfad zu einer Kontrolldatei. | `src/measure/field.rs` |
| `pub fn with_decoder(mut self, decoder: std::sync::Arc<dyn StructureDecoder>) -> Self` | Fügt einen Strukturdecoder hinzu. | `src/measure/field.rs` |
| `pub fn with_mk3_status(mut self, status: Mk3StatusInput) -> Self` | Setzt MK3 Statuswerte. | `src/measure/field.rs` |
| `pub fn with_target_pattern(mut self, pattern: impl Into<Vec<u8>>) -> Self` | Fügt ein Zielmuster hinzu. | `src/measure/field.rs` |
| `pub fn with_target_patterns(mut self, patterns: impl IntoIterator<Item = Vec<u8>>) -> Self` | Fügt mehrere Zielmuster hinzu. | `src/measure/field.rs` |
| `pub fn run_positive_control(pattern: &[u8]) -> Option<ControlEvidence>` | Führt eine echte Positivkontrolle im Speicher durch: 1. Schreibt ein Kontrollmuster eingebettet in einen Scratch-Puffer mit Zufalls-/Paddin… | `src/measure/positive_control.rs` |
| `pub fn run_positive_control_vfs( vfs: &dyn Vfs, control_file: Option<&Path>, scratch_dir: &Path, pattern: &[u8], ) -> Option<ControlEvidence>` | Führt eine echte Positivkontrolle über das Vfs durch (SPEC 12.1, HR-10, T-RAW-2). Wenn `control_file` angegeben ist, wird diese Datei geles… | `src/measure/positive_control.rs` |
| `pub fn verify_scanner_integrity(pattern: &[u8], mutator: impl Fn(&[u8]) -> Vec<u8>) -> bool` | Kalibrierungs- und Validierungsfunktion für Scanner-Implementierungen. Verifiziert, dass ein Scanner, der blind arbeitet oder manipuliert i… | `src/measure/positive_control.rs` |
| `pub fn add_finding(&mut self, finding: ResidueFinding)` | Fügt einen Fund hinzu. | `src/measure/residue.rs` |
| `pub fn as_bytes(&self) -> &[u8` | Liefert den 32-Byte Digest als Slice. | `src/measure/residue.rs` |
| `pub fn insert_document(&mut self, doc_id: DocId, backlink_targets: Vec<u64>)` | Fügt ein Dokument mit Vektorknoten und Backlinks hinzu. | `src/measure/residue.rs` |
| `pub fn is_clean(&self) -> bool` | Prüft, ob keine Spuren gefunden wurden. | `src/measure/residue.rs` |
| `pub fn measure_after_erasure( &self, query: &ResidueQuery, canary: &[u8], pre_erase_matches: u64, ) -> Result<ResidueMeasurementReport, ResidueMeasurementError>` | Führt die vollständige Verifikationsmessung nach physischer Löschung durch. # Parameter - `query`: Suchkriterium des gelöschten Datensatzes… | `src/measure/residue.rs` |
| `pub fn new(scanner: &'a dyn ResidueScanner, clock: &'a dyn Clock) -> Self` | Erzeugt einen neuen Koordinator mit injiziertem Scanner und Clock. | `src/measure/residue.rs` |
| `pub fn new(layer: LayerId) -> Self` | Erstellt einen neuen leeren Stub-Index. | `src/measure/residue.rs` |
| `pub fn physical_erase(&mut self, doc_id: DocId)` | Führt physische Bereinigung/Nullung des Dokuments durch. | `src/measure/residue.rs` |
| `pub fn register_canary(&mut self, canary: Vec<u8>)` | Registriert ein Positivkontroll-Muster (Canary). | `src/measure/residue.rs` |
| `pub fn set_simulate_corrupt(&mut self, corrupt: Option<IndexCorruptionKind>)` | Konfiguriert die Simulation von Korruption. | `src/measure/residue.rs` |
| `pub fn set_simulate_fault(&mut self, fault: Option<ScannerFaultKind>)` | Konfiguriert die Simulation eines Scanner-Fehlers. | `src/measure/residue.rs` |
| `pub fn verify_positive_control(&self, canary: &[u8]) -> Result<(), ResidueMeasurementError>` | Führt die Positivkontrolle auf dem Scanner aus. | `src/measure/residue.rs` |
| `pub fn run_scanner_flow( _layer: LayerId, _class: MeasureClass, _method_id: u16, _control_pattern: &[u8], ) -> Result<LayerWitness, ErasureError>` | Führt einen delegierenden Scan-Fluss für Schichten aus (SPEC §12.1, §12.2, D-72, ER-25). Veraltet: Für echte Messungen muss `scan_with_cont… | `src/measure/scanner_flow.rs` |
| `pub fn scan_with_control(vfs: &dyn Vfs, plan: &ScanPlan) -> Result<LayerWitness, ErasureError>` | Führt einen echten Scan über alle Dateien mit vorangehender Positivkontrolle durch. 1. Positivkontrolle über Vfs. 2. Scanner scharfschalten… | `src/measure/scanner_flow.rs` |
| `pub fn arm_scanner(scanner: Scanner<Unarmed>, evidence: ControlEvidence) -> Scanner<Armed>` | Erstellt einen bewaffneten Scanner nach Positivkontrolle (ER-25). Nur aus `src/measure` aufzurufen (L-5). | `src/measure/witness.rs` |
| `pub fn compute_witness_digest(witness: &LayerWitness) -> [u8` | Berechnet einen kryptografischen BLAKE3-Digest eines `LayerWitness` (SPEC §12.2, INV-ERA-7). Bindet Schicht-ID, Messklasse, Methoden-ID, Da… | `src/measure/witness.rs` |
| `pub fn finish_scan( scanner: Scanner<Armed>, matches: u64, duration: Duration, scan_hash: [u8; 32], ) -> Result<LayerWitness, crate::types_local::Residue>` | Erstellt einen Zeugen nach erfolgreichem Scan (ER-25). `matches == 0` → Ok(LayerWitness) `matches != 0` → Err(Residue) | `src/measure/witness.rs` |
| `pub fn validate_witness(witness: &LayerWitness) -> Result<(), crate::error::ErasureError>` | Prüft, ob ein `LayerWitness` die strengen Invarianten für die Berichterstellung erfüllt (INV-CLASS-MEASURED-1). Ein Zeuge ist nur dann gült… | `src/measure/witness.rs` |
| `pub fn compute_collection_id(name: &str) -> CollectionId` | Erzeugt eine deterministische, kollisionsfreie [`CollectionId`] aus einem Namen. | `src/orchestrator.rs` |
| `pub fn coordinate_erasure( &self, scope: crate::types_local::KeyScope, ) -> Result<crate::scope::ActiveScopeFlight, ErasureError>` | Einheitlicher Einstiegspunkt für die Ausführung einer Lösch-Pipeline (INV-ERA-1). | `src/orchestrator.rs` |
| `pub fn new() -> Self` | Erstellt eine neue Instanz des einheitlichen Koordinators. | `src/orchestrator.rs` |
| `pub fn summary_from_witness(layer_name: String, witness: &ScanWitness) -> LayerSummary` | Erstellt ein [`LayerSummary`] aus einem [`ScanWitness`]. | `src/orchestrator.rs` |
| `pub fn is_expired(&self) -> bool` | Prüft ob die Deadline mit der Systemuhr überschritten ist. | `src/purge/cx.rs` |
| `pub fn is_expired_against(&self, clock: &dyn Clock) -> bool` | Prüft ob die Deadline überschritten ist bezüglich einer gegebenen Uhr. | `src/purge/cx.rs` |
| `pub fn new(min_free_bytes: u64, batch_window: Duration) -> Self` | Erstellt einen neuen Purge-Kontext. | `src/purge/cx.rs` |
| `pub fn with_deadline_nanos(mut self, deadline_nanos: u64) -> Self` | Erstellt einen Kontext mit Deadline (Monotonic Nanos). | `src/purge/cx.rs` |
| `pub fn new(layer_id: [u8; 16], priority: PurgePriority) -> Self` | Erstellt eine neue Zuordnung. | `src/purge/order.rs` |
| `pub fn sort_by_priority(mut layers: Vec<LayerPurgeOrder>) -> Vec<LayerPurgeOrder>` | Sortiert Schichten nach Purge-Priorität (niedrigste Zahl zuerst). | `src/purge/order.rs` |
| `pub fn from_code(code: u8) -> Result<Self, PurgePhaseError>` | Lädt eine Phase aus ihrem persistierten `u8`-Code (1..=9). | `src/purge/phase.rs` |
| `pub fn must_run(self, target: PurgePhase) -> bool` | Prüft, ob die angegebene Zielphase `target` noch ausgeführt werden muss (`self <= target`). | `src/purge/phase.rs` |
| `pub fn transition_to(&mut self, next: PurgePhase) -> Result<(), PurgePhaseError>` | Führt einen überprüften Phasenübergang durch. Schlägt mit [`PurgePhaseError::InvalidTransition`] fehl, wenn `can_transition_to` `false` ist. | `src/purge/phase.rs` |
| `pub fn check_disk_reserve( available_bytes: u64, required_bytes: u64, ) -> Result<ReserveCheck, ErasureError>` | Prüft ob genug Speicherplatz für den Purge-Rewrite vorhanden ist. | `src/purge/reserve.rs` |
| `pub fn advance_to_next(&mut self) -> Result<PurgePhase, PurgePhaseError>` | Schließt die aktuelle Phase erfolgreich ab und wechselt auf die Folgephase. Schlägt fail-closed fehl, wenn keine Folgephase existiert oder… | `src/purge/resume.rs` |
| `pub fn current_phase(&self) -> PurgePhase` | Gibt die aktuell erreichte Phase zurück. | `src/purge/resume.rs` |
| `pub fn is_completed(&self) -> bool` | Prüft, ob der Purge bereits terminal beendet ist. | `src/purge/resume.rs` |
| `pub fn new(purge_id: [u8; 16]) -> Self` | Initialisiert einen neuen Fortschrittstracker im Zustand `Admitted`. | `src/purge/resume.rs` |
| `pub fn next_phase(&self) -> Option<PurgePhase>` | Ermittelt die als nächstes auszuführende Phase. Liefert `None`, falls der Purge bereits vollständig im Zustand `Released` abgeschlossen ist. | `src/purge/resume.rs` |
| `pub fn record_failure(&mut self, err: PurgePhaseError)` | Zeichnet einen Phasenfehler auf. | `src/purge/resume.rs` |
| `pub fn recover( purge_id: [u8; 16], phase_code: u8, attempts: u32, ) -> Result<Self, PurgePhaseError>` | Stellt einen Tracker aus persistierten Rohdaten wieder her (P9 Resume). Validiert den Phasencode fail-closed. | `src/purge/resume.rs` |
| `pub fn empty() -> Self` | Erstellt ein leeres Ergebnis im Initialzustand `Admitted`. | `src/purge/window.rs` |
| `pub fn merge(&mut self, other: &WindowOutcome)` | Addiert ein weiteres Ergebnis. | `src/purge/window.rs` |
| `pub async fn run_core_window(core: &ErasureCore) -> Result<WindowOutcome, ErasureError>` | Führt ein vollständiges Purge-Fenster über den `ErasureCore` aus (SPEC §11.2, §11.4). | `src/purge/window.rs` |
| `pub fn run_window(cx: &PurgeCx) -> Result<WindowOutcome, ErasureError>` | Führt ein Purge-Fenster durch (ER-20). Quellkompatible Signatur für bestehende Aufrufer: prüft Plattenreserve. Schlägt fehl mit `DiskReserv… | `src/purge/window.rs` |
| `pub fn advance_to_r2_intent(&mut self) -> Result<(), ErasureError>` | Schaltet idempotent auf R2 (PreGate Intent) vor (D-115 R2). | `src/replace.rs` |
| `pub fn advance_to_r3_gate_old(&mut self) -> Result<(), ErasureError>` | Schaltet nach Bestätigung des Neudokuments auf R3 (Gate altes Dokument) vor (D-115 R3). | `src/replace.rs` |
| `pub fn advance_to_r4_key_destroyed(&mut self) -> Result<(), ErasureError>` | Schaltet auf R4 (D-Slot des Altdokuments vernichtet) vor (D-115 R4). | `src/replace.rs` |
| `pub fn erasure_id(&self) -> ErasureId` | Liefert die zugehörige Lösch-ID. | `src/replace.rs` |
| `pub fn new( erasure_id: ErasureId, old_doc_tag: [u8; 16], new_doc_tag: [u8; 16], ) -> Result<Self, ErasureError>` | Erstellt einen neuen Ersetzungsplan. # Fehler Liefert `ErasureError::SameIdentity`, wenn altes und neues Dokument denselben Tag tragen. | `src/replace.rs` |
| `pub fn old_doc_tag(&self) -> &[u8` | Liefert den Tag des alten Dokuments. | `src/replace.rs` |
| `pub fn replacement_tag(&self) -> &[u8` | Liefert den Ersatz-Tag (`replacement_tag`, SPEC D-115) für den Löschbericht. | `src/replace.rs` |
| `pub fn step(&self) -> ReplaceStep` | Liefert den aktuellen Protokollschritt (R1 bis R4). | `src/replace.rs` |
| `pub fn verify_replacement_present(&self, is_present: bool) -> Result<(), ErasureError>` | Prüft vor dem Gate-Schritt (R3), ob das Ersatzdokument dauerhaft vorhanden ist (I-25). Liefert `ErasureError::ReplacementMissing` (FM-19),… | `src/replace.rs` |
| `pub fn compute_witnesses_chain_hash(witnesses: &[LayerWitness]) -> [u8` | Berechnet den verketteten Hash über eine Liste von `LayerWitness`-Objekten. | `src/report/binding.rs` |
| `pub fn verify_witnesses_binding(witnesses: &[LayerWitness], expected_hash: &[u8; 32]) -> bool` | Verifiziert, ob die gegebenen Zeugen mit dem gebundenen Hash übereinstimmen. | `src/report/binding.rs` |
| `pub fn build_report( erasure_id: ErasureId, gated_at: u64, completed_at: u64, witnesses: Vec<LayerWitness>, segments_rewritten: u64, bytes_rewritten: u64, purge_complete: bool, ) -> ErasureReport` | Erstellt einen Löschbericht mit Standardoptionen (ER-27). | `src/report/build.rs` |
| `pub fn build_report_with_options( erasure_id: ErasureId, witnesses: Vec<LayerWitness>, segments_rewritten: u64, bytes_rewritten: u64, opts: ReportBuildOptions, ) -> ErasureReport` | Erstellt einen vollständigen, normativen Löschbericht nach SPEC §11.7. | `src/report/build.rs` |
| `pub fn canonicalize_report(report: &ErasureReport) -> Result<Vec<u8>, ErasureError>` | Kanonisiert einen Bericht als Bytes (für die Signatur). | `src/report/build.rs` |
| `pub fn compute_from_digests(digests: &[[u8; 32]]) -> Self` | Berechnet die Zeugenkette aus einer geordneten Liste von Zeugendigests (INV-ERA-7). Bei leerer Liste ist der Wurzel-Digest 32 Nullbytes. Be… | `src/report/build.rs` |
| `pub fn from_witness(w: &LayerWitness) -> Self` | Erstellt einen Report-Eintrag aus einem `LayerWitness`. | `src/report/build.rs` |
| `pub fn new(seq: u64, chain_hash: [u8; 32]) -> Self` | Erstellt eine neue Ledger-Verankerungsbestätigung. | `src/report/build.rs` |
| `pub fn new(schema_version: &str, timestamp_micros: u64) -> Self` | Abwärtskompatibler Konstruktor für bestehende Tests. | `src/report/build.rs` |
| `pub fn set_field(&mut self, key: &str, val: &str)` | Abwärtskompatible Hilfsmethode. | `src/report/build.rs` |
| `pub fn try_build_report_with_options( erasure_id: ErasureId, witnesses: Vec<LayerWitness>, segments_rewritten: u64, bytes_rewritten: u64, opts: ReportBuildOptions, ) -> Result<ErasureReport, ErasureError>` | Erstellt einen normativen Löschbericht mit strenger Validierung (fail-closed, INV-CLASS-MEASURED-1). Schlägt fehl (`Err`), wenn: - keine Ze… | `src/report/build.rs` |
| `pub fn canonicalize_json<T: Serialize>(val: &T) -> Result<String, ErasureError>` | Alias für `to_canonical_json_string` für Abwärtskompatibilität. | `src/report/canonical.rs` |
| `pub fn canonicalize_value(v: &Value) -> Result<Value, ErasureError>` | Kanonisiert einen `serde_json::Value` nach F.10. | `src/report/canonical.rs` |
| `pub fn to_canonical_json<T: Serialize>(value: &T) -> Result<Vec<u8>, ErasureError>` | Serialisiert einen Wert als kanonisches JSON (keine Leerzeichen, sortierte Schlüssel). | `src/report/canonical.rs` |
| `pub fn to_canonical_json_string<T: Serialize>(value: &T) -> Result<String, ErasureError>` | Serialisiert und kanonisiert einen Wert als String. | `src/report/canonical.rs` |
| `pub fn verify_canonical_nfc(s: &str) -> Result<(), ErasureError>` | Prüft, ob ein UTF-8-String die Anforderungen an Normalform C (NFC) erfüllt (SPEC F.10). Zur Vermeidung neuer Abhängigkeiten (AR-3, Regel 4)… | `src/report/canonical.rs` |
| `pub fn evaluate_residual_risks(purge_complete: bool) -> Vec<ResidualRiskId>` | Abwärtskompatible Auswertung aller Restrisiken (ER-28). Beinhaltet standardmäßig alle bedingungslosen und die Purge-abhängigen Risiken. | `src/report/risk.rs` |
| `pub fn evaluate_residual_risks_with_context(ctx: &ResidualRiskContext) -> Vec<ResidualRiskId>` | Bewertet alle Restrisiken mit vollständigem Kontext gemäß Anhang D. | `src/report/risk.rs` |
| `pub fn from_id(risk: ResidualRiskId) -> Self` | Erstellt einen Eintrag aus einer Risiko-ID. | `src/report/risk.rs` |
| `pub fn from_risk(risk: &ResidualRiskId) -> Self` | Abwärtskompatibler Alias. | `src/report/risk.rs` |
| `pub fn base64url_encode(data: &[u8]) -> String` | Hilfsfunktion für Base64url-Kodierung ohne Padding (F.10, RFC 4648 §5). | `src/report/signing.rs` |
| `pub fn new(key: SigningKey) -> Self` | Erzeugt einen neuen internen Signierer (signer_id = 0). | `src/report/signing.rs` |
| `pub fn persist_signed_report( vfs: &dyn Vfs, target_path: &Path, signed: &SignedReport, ) -> Result<(), ErasureError>` | Schreibt und synchronisiert den signierten Bericht atomar über Vfs (SPEC §11.8). | `src/report/signing.rs` |
| `pub fn sign(report: ErasureReport, key: &SigningKey) -> Result<Self, ErasureError>` | Signiert einen kanonischen Bericht mit einem `SigningKey`. | `src/report/signing.rs` |
| `pub fn sign_bytes( report_bytes: &[u8], signature: String, pub_fingerprint: String, ) -> SignedReport` | Erstellt einen signierten Bericht aus kanonischen Bytes. | `src/report/signing.rs` |
| `pub fn sign_report( report: &ErasureReport, key: &SigningKey, ) -> Result<SignedReport, ErasureError>` | Signiert einen kanonischen Bericht mit dem Ed25519-Schlüssel (SPEC §11.7, F.10, D-130). | `src/report/signing.rs` |
| `pub fn signature_base64(&self) -> &str` | Liefert die Base64url-Signatur. | `src/report/signing.rs` |
| `pub fn verify(&self) -> Result<(), ErasureError>` | Verifiziert die Integrität (erfordert bekannten Verifying-Key oder decodierten Fingerprint). | `src/report/signing.rs` |
| `pub fn verify_report_with_chain( &self, verifying_key: &VerifyingKey, ) -> Result<ErasureReport, ErasureError>` | Verifiziert sowohl die Ed25519-Signatur als auch die kryptografische Zeugenkette (INV-ERA-7, INV-CLASS-MEASURED-1). Validiert: 1. Ed25519 v… | `src/report/signing.rs` |
| `pub fn verify_with_key(&self, verifying_key: &VerifyingKey) -> Result<(), ErasureError>` | Verifiziert die Integrität und Signatur des Berichts gegen den VerifyingKey. | `src/report/signing.rs` |
| `pub fn with_signer_id(key: SigningKey, signer_id: SignerId) -> Self` | Erzeugt einen Signierer mit benutzerdefinierter Signer-ID (z. B. 1 für Betreiber). | `src/report/signing.rs` |
| `pub fn is_replay(&self, seq: Seq) -> bool` | Prüft, ob eine Sequenznummer kleiner oder gleich dem Floor ist (Replay / Altbestand). | `src/restore.rs` |
| `pub fn new(ledger_floor: Seq) -> Self` | Erstellt einen neuen RestoreGuard mit einem Ledger-Floor. | `src/restore.rs` |
| `pub fn check(&self, store_current_seq: u64) -> GuardOutcome` | Führt die Integritätsprüfung durch (ER-30). `store_current_seq`: aktuelle Sequenznummer des Stores. | `src/restore/guard.rs` |
| `pub fn new(ledger: &'a InMemoryLedger) -> Self` | Erstellt einen neuen Checker mit Ledger-Referenz. | `src/restore/guard.rs` |
| `pub fn next_safe_seq(&self, store_current_seq: u64) -> u64` | Berechnet die nächste sichere Sequenznummer: `max(current_seq, ledger_floor) + 1` (ER-30). | `src/restore/guard.rs` |
| `pub fn replay_erasures(affected_ids: &[ErasureId]) -> Result<ReplayOutcome, ErasureError>` | Führt einen Replay-Vorgang für betroffene Löschungen durch (ER-30 / INV-ERA-2). Ein Wiederanlauf (Resume) führt den Purge für jede nicht ab… | `src/restore/replay.rs` |
| `pub fn resume_and_execute_purge( saga: &mut crate::saga::machine::EraseSaga, driver: &crate::saga::driver::SagaDriver, ) -> Result<crate::saga::driver::StepOutcome, ErasureError>` | Führt die aktive Wiederaufnahme (Resume) einer unterbrochenen Saga aus (INV-ERA-2). | `src/restore/replay.rs` |
| `pub fn allows_access_to_restricted(&self, is_target_restricted: bool) -> bool` | Prüft, ob der Pfad ausschließlich auf `Restricted`-Bereiche angewandt wird (SPEC D-116, T-RESTR-3). Gegen `Erased`-Bereiche ist kein Zugrif… | `src/restricted.rs` |
| `pub fn attestation(&self) -> &OperatorAttestation` | Liefert die zugehörige Betreiber-Attestierung. | `src/restricted.rs` |
| `pub fn attested_at(&self) -> u64` | Liefert den Zeitstempel der Attestierung. | `src/restricted.rs` |
| `pub fn case_commitment(&self) -> &[u8` | Liefert das kryptografische Commitment des Fallbezugs. | `src/restricted.rs` |
| `pub fn create_entry( erasure_id: ErasureId, access: &RestrictedAccess, doc_count: u32, timestamp: u64, ) -> Result<Self, ErasureError>` | Erstellt einen auditierbaren Journal-Eintrag für Art. 18 Abs. 2 Zugriff. | `src/restricted.rs` |
| `pub fn is_lifted(&self) -> bool` | Prüft, ob die Einschränkung aufgehoben ist. | `src/restricted.rs` |
| `pub fn is_terminal(&self) -> bool` | Prüft, ob es sich um einen Endzustand handelt. | `src/restricted.rs` |
| `pub fn lift(&mut self, _reason: &str) -> Result<(), ErasureError>` | Hebt die Restriktion mit einer Begründung auf (Journal-Eintrag erforderlich). | `src/restricted.rs` |
| `pub fn new(operator: &str, case_ref: &[u8], attested_at: u64, epoch_key: &[u8; 32]) -> Self` | Erstellt eine neue Betreiber-Attestierung mit gehashtem Fallbezug. `case_ref` wird kryptografisch mit dem Epochenschlüssel gehasht (`case_c… | `src/restricted.rs` |
| `pub fn new(purpose: RestrictedPurpose, attestation: OperatorAttestation) -> Self` | Erstellt ein neues Zugriffsgesuch nach Art. 18 Abs. 2 DSGVO. | `src/restricted.rs` |
| `pub fn operator(&self) -> &str` | Liefert den Betreiber-Identifikator. | `src/restricted.rs` |
| `pub fn purpose(&self) -> RestrictedPurpose` | Liefert den rechtlichen Zweck. | `src/restricted.rs` |
| `pub fn supersede_by_erasure(&mut self) -> Result<(), ErasureError>` | Ersetzt die Restriktion durch einen nachfolgenden regulären Erase-Vorgang (D-119). | `src/restricted.rs` |
| `pub fn validate_batch_size(&self, batch_size: usize) -> Result<(), ErasureError>` | Validiert die Stapelgröße (SPEC D-116: höchstens 1 000 Dokumente). | `src/restricted.rs` |
| `pub fn append(&self, target: RevocationTarget) -> Result<RevocationEntry>` | Fügt einen neuen Widerrufs-Eintrag für `target` an den Log an. Nutzt den injizierten Clock-Port zur Generierung des Zeitstempels und den Si… | `src/revocation_log.rs` |
| `pub fn canonical_bytes(&self) -> Vec<u8>` | Gibt eine kanonische Byte-Repräsentation des Targets für das Hashing zurück. | `src/revocation_log.rs` |
| `pub fn compute_entry_hash( sequence_number: u64, target: &RevocationTarget, revoked_at_nanos: u64, prev_hash: &[u8; 32], ) -> [u8` | Berechnet den SHA-256-Hash eines Eintrags aus seinen kanonischen Feldern. | `src/revocation_log.rs` |
| `pub fn deserialize<'de, D>(deserializer: D) -> Result<[u8` | — | `src/revocation_log.rs` |
| `pub fn is_empty(&self) -> bool` | Prüft, ob der Log leer ist. | `src/revocation_log.rs` |
| `pub fn is_revoked(&self, target: &RevocationTarget) -> bool` | Prüft, ob ein gegebenes Target (KEK, DEK, Group oder Record) widerrufen wurde. | `src/revocation_log.rs` |
| `pub fn len(&self) -> usize` | Gibt die Anzahl der Widerrufe im Log zurück. | `src/revocation_log.rs` |
| `pub fn new_in_memory( clock: Arc<dyn Clock>, signing_key: Option<SigningKey>, verifying_key: VerifyingKey, ) -> Self` | Erstellt einen neuen In-Memory-Widerrufslog ohne Dateipersistenz. | `src/revocation_log.rs` |
| `pub fn open_or_create( vfs: Arc<dyn Vfs>, path: impl AsRef<Path>, clock: Arc<dyn Clock>, signing_key: Option<SigningKey>, verifying_key: VerifyingKey, ) -> Result<Self>` | Öffnet einen bestehenden Widerrufslog oder erzeugt einen neuen unter `path` über den Vfs-Port. Beim Öffnen einer bestehenden Datei wird die… | `src/revocation_log.rs` |
| `pub fn revoked_targets(&self) -> AHashSet<RevocationTarget>` | Gibt eine Kopie aller aktuell im Log widerrufenen Targets zurück. | `src/revocation_log.rs` |
| `pub fn serialize<S>(bytes: &[u8; 64], serializer: S) -> Result<S::Ok, S::Error> where S: Serializer,` | — | `src/revocation_log.rs` |
| `pub fn verify_integrity(&self) -> Result<()>` | Verifiziert die Integrität der gesamten Logeinträge-Kette (Sequenz, Hashes, Ed25519-Signaturen). | `src/revocation_log.rs` |
| `pub fn config(&self) -> &ErasureConfig` | Gibt die Konfiguration zurück. | `src/saga/core.rs` |
| `pub fn deps(&self) -> &Arc<ErasureDeps>` | Gibt die Abhängigkeiten zurück. | `src/saga/core.rs` |
| `pub async fn erase(&self, req: EraseExecutionRequest) -> Result<EraseReceipt, ErasureError>` | Nimmt eine Löschung an und führt sie bis mindestens `Gated` aus (SPEC §11.2, ER-23). Cancel-Safety (ER-24): Die Ausführung läuft in einer u… | `src/saga/core.rs` |
| `pub fn evaluate_retention( mode: EraseMode, attestation: &RetentionAttestation, ) -> Result<RetentionDecision, ErasureError>` | Führt die Validierung der Retention-Attestierung gemäß Matrix D-124 Abs. 6 durch. | `src/saga/core.rs` |
| `pub fn new(cfg: ErasureConfig, deps: ErasureDeps) -> Self` | Erstellt eine neue Instanz des Orchestrators. | `src/saga/core.rs` |
| `pub async fn recover_and_resume(&self) -> Result<Vec<ErasureId>, ErasureError>` | Wiederanlauf und Recovery unfertiger Sagas (SPEC §11.2 Recovery, I-15, FM-1..14). | `src/saga/core.rs` |
| `pub async fn run_purge_window( &self, ) -> Result<crate::purge::window::WindowOutcome, ErasureError>` | Führt ein Purge-Fenster über alle offenen Sagas im Zustand `KeyDestroyed` oder `Purging` aus. | `src/saga/core.rs` |
| `pub async fn status(&self, id: ErasureId) -> Result<ErasureCoreStatus, ErasureError>` | Fragt den Status einer Löschung ab. | `src/saga/core.rs` |
| `pub fn core(&self) -> Option<&Arc<super::core::ErasureCore>>` | Liefert die Core-Referenz, falls vorhanden. | `src/saga/driver.rs` |
| `pub fn new() -> Self` | Erstellt einen neuen Treiber. | `src/saga/driver.rs` |
| `pub fn process_event( &self, saga: &mut EraseSaga, event: SagaEvent, ) -> Result<StepOutcome, ErasureError>` | Verarbeitet ein Ereignis aus der Treiber-Pipeline (SPEC §11.2). | `src/saga/driver.rs` |
| `pub fn step(&self, saga: &mut EraseSaga) -> Result<StepOutcome, ErasureError>` | Führt den nächsten Schritt der Saga durch (idempotent, ER-18). Validiert den Übergang und vollzieht den Schritt in der Zustandsmaschine. | `src/saga/driver.rs` |
| `pub async fn step_async(&self, saga: &mut EraseSaga) -> Result<StepOutcome, ErasureError>` | Asynchrone Ausführung des nächsten Treiberschritts unter Nutzung des `ErasureCore`. | `src/saga/driver.rs` |
| `pub fn with_core(core: Arc<super::core::ErasureCore>) -> Self` | Erstellt einen Treiber mit Anbindung an den `ErasureCore`. | `src/saga/driver.rs` |
| `pub fn derive_open_key(k_idx: &[u8; 32], canonical_target: &[u8], mode_byte: u8) -> [u8` | Berechnet den Open-Lookup-Key nach D-119: `open_key = BLAKE3-keyed(K_idx, "ctx/erasure-open/v1" || kanonisches_Ziel || mode)` | `src/saga/idempotency.rs` |
| `pub fn is_same_request(a_erasure_id: [u8; 16], b_erasure_id: [u8; 16]) -> bool` | Prüft ob zwei `EraseRequest`-Anfragen dieselbe Löschung darstellen. Gleiches Ziel, gleicher Modus → gleiche ErasureId. | `src/saga/idempotency.rs` |
| `pub fn can_transition_to(self, next: SagaState) -> bool` | Prüft ob ein Übergang zu `next` erlaubt ist (SPEC §11.2, I-4, FW-9). | `src/saga/machine.rs` |
| `pub fn erasure_id(&self) -> ErasureId` | Gibt die Lösch-ID zurück. | `src/saga/machine.rs` |
| `pub fn is_terminal(self) -> bool` | Prüft ob dieser Zustand ein Endzustand ist. | `src/saga/machine.rs` |
| `pub fn is_terminal(&self) -> bool` | Prüft ob die Saga abgeschlossen ist. | `src/saga/machine.rs` |
| `pub fn new(erasure_id: ErasureId) -> Self` | Erstellt eine neue Saga mit gegebener ErasureId im Zustand `Requested`. | `src/saga/machine.rs` |
| `pub fn pending_reason(&self) -> Option<&PendingReason>` | Gibt den letzten Pending-Grund zurück (falls vorhanden). | `src/saga/machine.rs` |
| `pub fn state(&self) -> SagaState` | Gibt den aktuellen Zustand zurück. | `src/saga/machine.rs` |
| `pub fn transition_to(&mut self, next: SagaState) -> Result<(), crate::error::ErasureError>` | Führt einen einfachen Zustandsübergang durch (idempotent). | `src/saga/machine.rs` |
| `pub fn transition_with_reason( &mut self, next: SagaState, reason: Option<PendingReason>, ) -> Result<(), crate::error::ErasureError>` | Führt einen Zustandsübergang mit Begründung durch (ER-18: idempotent). Gibt `Err` zurück wenn der Übergang nicht erlaubt ist. | `src/saga/machine.rs` |
| `pub fn with_id(erasure_id: ErasureId) -> Self` | Erstellt eine neue Saga mit gegebener ErasureId im Zustand `Requested`. | `src/saga/machine.rs` |
| `pub fn with_state(erasure_id: ErasureId, state: SagaState) -> Self` | Erstellt eine Saga mit einem vorgegebenen Zustand (Recovery-Pfad). | `src/saga/machine.rs` |
| `pub fn is_restrict_allowed(att: &RetentionAttestation) -> bool` | Prüft ob der Modus `Restrict` zulässig ist. | `src/saga/retention.rs` |
| `pub fn is_retention_active(att: &RetentionAttestation) -> bool` | Prüft ob die Attestierung eine aktive Aufbewahrungspflicht anzeigt. | `src/saga/retention.rs` |
| `pub async fn query_target_gate( gate: &dyn SagaGate, target: &EraseTarget, ) -> Result<GateDecision, ErasureError>` | Öffentlicher Delegator für `decision_for_target`. | `src/saga/seams.rs` |
| `pub fn decode_scope(data: &[u8]) -> Result<LayerScopeSet, ErasureError>` | Deserialisiert einen `LayerScopeSet` aus dem festen Binärformat. Fail-Closed: Bei falschem Magic, unbekannter Version, Trunkierung, ungülti… | `src/scope/codec.rs` |
| `pub fn encode_scope(scope: &LayerScopeSet) -> Result<Vec<u8>, ErasureError>` | Serialisiert einen `LayerScopeSet` in ein festes Binärformat mit Versionsfeld und Prüfsumme (Big-Endian). | `src/scope/codec.rs` |
| `pub async fn load_scope_file( vfs: &dyn Vfs, keyring: &dyn FileKeyring, erasure_id: &[u8; 16], file_id: FileId, ) -> Result<LayerScopeSet, ErasureError>` | Lädt einen `LayerScopeSet` aus einer persistenten Scope-Datei (kind=9). Fail-Closed: Jede Beschädigung (Header-MAC, Record-AEAD, Magic, Ver… | `src/scope/file.rs` |
| `pub fn scope_file_path(erasure_id: &[u8; 16]) -> PathBuf` | Ermittelt den relativen Pfad der Scope-Datei nach SPEC D-75: `scope/<erasure_id_hex>.scp` | `src/scope/file.rs` |
| `pub async fn store_scope_file( vfs: &dyn Vfs, keyring: &dyn FileKeyring, erasure_id: &[u8; 16], file_id: FileId, scope: &LayerScopeSet, ) -> Result<(), ErasureError>` | Speichert einen `LayerScopeSet` als persistente, verschlüsselte Scope-Datei (kind=9). Schreibt atomar und synchronisiert das übergeordnete… | `src/scope/file.rs` |
| `pub fn is_active(&self, scope: &KeyScope) -> bool` | Prüft, ob für den Scope derzeit ein Purge aktiv ist. | `src/scope/flight.rs` |
| `pub fn new() -> Self` | Erzeugt einen neuen `SingleFlightCoordinator`. | `src/scope/flight.rs` |
| `pub fn try_acquire(&self, scope: KeyScope) -> Result<ActiveScopeFlight, ErasureError>` | Versucht, den exklusiven Flug für den gegebenen Scope zu reservieren. Liefert `Err(ErasureError::LimitExceeded(...))` falls bereits ein Flu… | `src/scope/flight.rs` |
| `pub fn empty(layer_id: ScopeLayerId) -> Self` | Erstellt einen leeren Scope für eine Schicht. | `src/scope/set.rs` |
| `pub fn is_embeddable(&self) -> bool` | Prüft ob der Scope klein genug ist um eingebettet zu werden (ER-17). Schwellwert: ≤ 16 Einheiten → Einbettung in Journal-Eintrag. | `src/scope/set.rs` |
| `pub fn is_embeddable(&self) -> bool` | Prüft ob der gesamte Scope einbettbar ist (ER-17). | `src/scope/set.rs` |
| `pub fn new() -> Self` | Erstellt eine leere Scope-Menge. | `src/scope/set.rs` |
| `pub fn push(&mut self, scope: ResolvedLayerScope)` | Fügt einen Scope hinzu. | `src/scope/set.rs` |
| `pub fn total_units(&self) -> u64` | Gesamtanzahl betroffener Einheiten über alle Schichten. | `src/scope/set.rs` |
| `pub fn with_units(layer_id: ScopeLayerId, units: Vec<u64>) -> Self` | Erstellt einen Scope mit Einheiten. | `src/scope/set.rs` |
| `pub fn acquire( &self, scope_id: [u8; 16], owner_id: [u8; 16], ) -> Result<ScopeLockGuard<'a>, ScopeLockError>` | Versucht, ein exklusives Single-Flight Lock auf den angegebenen Scope zu erwerben. - Falls keine Sperre existiert: Erwirbt Lock mit Epoche… | `src/scope/single_flight.rs` |
| `pub fn acquire_with_lease( &self, scope_id: [u8; 16], owner_id: [u8; 16], lease_nanos: u64, ) -> Result<ScopeLockGuard<'a>, ScopeLockError>` | Versucht, ein exklusives Single-Flight Lock mit benutzerdefinierter Lease-Dauer zu erwerben. | `src/scope/single_flight.rs` |
| `pub fn epoch(&self) -> u64` | Gibt die aktuelle Sperr-Epoche zurück. | `src/scope/single_flight.rs` |
| `pub fn from_bytes(bytes: &[u8]) -> Result<Self, ScopeLockError>` | Dekodiert und validiert den Lock-Record fail-closed. | `src/scope/single_flight.rs` |
| `pub fn is_locked(&self, scope_id: &[u8; 16]) -> Result<bool, ScopeLockError>` | Prüft, ob ein Scope aktuell gesperrt ist. | `src/scope/single_flight.rs` |
| `pub fn is_stale(&self, current_monotonic_nanos: u64) -> bool` | Prüft, ob das Lock relativ zur übergebenen Monotonic-Zeit abgelaufen (stale) ist. | `src/scope/single_flight.rs` |
| `pub fn new(vfs: &'a dyn Vfs, clock: &'a dyn Clock, default_lease_nanos: u64) -> Self` | Erstellt einen neuen `ScopeLockManager`. | `src/scope/single_flight.rs` |
| `pub fn owner_id(&self) -> &[u8` | Gibt die Besitzer-ID zurück. | `src/scope/single_flight.rs` |
| `pub fn release(self) -> Result<(), ScopeLockError>` | Gibt das Scope-Lock explizit frei. | `src/scope/single_flight.rs` |
| `pub fn scope_id(&self) -> &[u8` | Gibt die Scope-ID zurück. | `src/scope/single_flight.rs` |
| `pub fn scope_lock_path(scope_id: &[u8; 16]) -> PathBuf` | Pfad der Sperrdatei für einen Scope. | `src/scope/single_flight.rs` |
| `pub fn to_bytes(&self) -> Vec<u8>` | Kodiert den Lock-Record in ein festes Binärformat mit BLAKE3-Prüfsumme. | `src/scope/single_flight.rs` |
| `pub fn deserialize<'de, D>(deserializer: D) -> Result<[u8` | — | `src/supersede.rs` |
| `pub fn erasure_id(&self) -> ErasureId` | Liefert die zugehörige Lösch-ID. | `src/supersede.rs` |
| `pub fn intent_seq(&self) -> u64` | Liefert die Intent-Sequenz vor dem Update-Commit. | `src/supersede.rs` |
| `pub fn matcher(&self) -> &PrefixSeqMatcher` | Liefert den Matcher für Gate und Purge. | `src/supersede.rs` |
| `pub fn matches(&self, candidate_prefix: &[u8; 65], candidate_seq: u64) -> bool` | Prüft, ob ein gegebener Schlüssel mit dem Präfix übereinstimmt und die Sequenz unter `seq_lt` liegt. | `src/supersede.rs` |
| `pub fn mode(&self) -> SupersedeMode` | Liefert den Berichtigungsmodus. | `src/supersede.rs` |
| `pub fn new( erasure_id: ErasureId, prefix: [u8; 65], split_seq: u64, mode: SupersedeMode, ) -> Self` | Erstellt einen neuen Plan für `Superseded`. | `src/supersede.rs` |
| `pub fn serialize<S>(prefix: &[u8; 65], serializer: S) -> Result<S::Ok, S::Error> where S: Serializer,` | — | `src/supersede.rs` |
| `pub fn verify_updated_seq(&self, latest_committed_seq: u64) -> Result<(), ErasureError>` | Validiert, dass eine neue Version mit `seq > intent_seq` committet wurde (D-117). Liefert `ErasureError::UpdateMissing`, wenn die neue Vers… | `src/supersede.rs` |
| `pub fn new(layer: LayerId) -> Self` | Erzeugt ein neues Mock-Backend für den gegebenen Layer. | `src/test_mocks.rs` |
| `pub fn new(id: impl Into<String>, kind: &'static str) -> Self` | Erzeugt ein neues MockErasureTarget. | `src/test_mocks.rs` |
| `pub fn purge_count(&self) -> usize` | Liefert die Anzahl der aufgerufenen Purge-Operationen. | `src/test_mocks.rs` |
| `pub fn affected_units(&self) -> &[u64]` | Liefert die betroffenen Einheiten. | `src/types_local.rs` |
| `pub fn arm(self, _ev: ControlEvidence) -> Scanner<Armed>` | Schaltet Scanner über Positivkontrolle scharf. | `src/types_local.rs` |
| `pub fn armed(layer: LayerId, class: MeasureClass, method_id: u16) -> Self` | Erzeugt einen direkt scharfen Scanner (z.B. für Tests/Mocks). | `src/types_local.rs` |
| `pub fn as_bytes(&self) -> &[u8]` | Liefert die Bytes als Slice. | `src/types_local.rs` |
| `pub fn as_bytes(&self) -> &[u8` | Gibt eine Referenz auf die Bytes zurück. | `src/types_local.rs` |
| `pub fn as_bytes(&self) -> &[u8]` | Liefert Referenz auf die geschützten Bytes. | `src/types_local.rs` |
| `pub fn as_bytes(&self) -> &[u8` | Liefert die Bytes als Slice. | `src/types_local.rs` |
| `pub fn as_str(&self) -> &str` | Gibt den String-Namen der Schicht zurück. | `src/types_local.rs` |
| `pub fn begin(layer: WitnessLayer, target: ScanTarget, technique: ScanTechnique) -> Self` | Startet eine neue ScanSession. | `src/types_local.rs` |
| `pub fn bytes(&self) -> u64` | Liefert geprüfte Bytes. | `src/types_local.rs` |
| `pub fn class(&self) -> MeasureClass` | Liefert die Messklasse. | `src/types_local.rs` |
| `pub fn compute_chained( commitment: [u8; 32], range_set: RangeSet, gated_seq: Seq, completed_at: Timestamp, prev_hash: [u8; 32], ledger_mac: [u8; 32], ) -> Self` | Berechnet die verkettete Hash-Kette. | `src/types_local.rs` |
| `pub fn contains(&self, key: &KeyBuf) -> bool` | Prüft ob ein Schlüssel im Intervall liegt. | `src/types_local.rs` |
| `pub fn digest(&self) -> &[u8` | Gibt den BLAKE3-Digest der Messung zurück. | `src/types_local.rs` |
| `pub fn duration(&self) -> Duration` | Liefert Scandauer. | `src/types_local.rs` |
| `pub fn file_id(&self) -> FileId` | Liefert die Dateikennung. | `src/types_local.rs` |
| `pub fn files(&self) -> u64` | Liefert geprüfte Dateien. | `src/types_local.rs` |
| `pub fn finish( self, matches: u64, duration: Duration, hash: [u8; 32], ) -> Result<LayerWitness, Residue>` | Schließt Messung ab. | `src/types_local.rs` |
| `pub fn finish(mut self, clock: &dyn Clock) -> Result<ScanWitness, crate::error::ErasureError>` | Schließt die Session ab und erzeugt den `ScanWitness` (gesperrter Mint-Pfad). | `src/types_local.rs` |
| `pub fn from_key(key: &str) -> Result<Self, crate::error::ErasureError>` | Erzeugt eine `DocId` aus einem Schlüssel mittels BLAKE3. | `src/types_local.rs` |
| `pub fn hash(&self) -> &[u8` | Alias für witness_hash. | `src/types_local.rs` |
| `pub fn layer(&self) -> LayerId` | Liefert die Schicht-ID. | `src/types_local.rs` |
| `pub fn layer(&self) -> &str` | Gibt die gescannte Storage-Schicht als &str zurück. | `src/types_local.rs` |
| `pub fn measured_at(&self) -> u64` | Gibt den Zeitstempel der Messung (Unix Nanos) zurück. | `src/types_local.rs` |
| `pub fn method(&self) -> &ScanMethod` | Gibt die ScanMethod-Entsprechung zurück. | `src/types_local.rs` |
| `pub fn method_id(&self) -> u16` | Liefert die Methoden-ID. | `src/types_local.rs` |
| `pub fn new(token: Option<Arc<()>>) -> Self` | Erzeugt ein neues Shared-Token. | `src/types_local.rs` |
| `pub fn new(token: Option<Arc<()>>) -> Self` | Erzeugt ein neues Exclusive-Token. | `src/types_local.rs` |
| `pub fn new(token: Option<Arc<()>>) -> Self` | Erzeugt ein neues CompactionPermit-Token. | `src/types_local.rs` |
| `pub fn new(token: Option<Arc<()>>) -> Self` | Erzeugt ein neues PurgePermit-Token. | `src/types_local.rs` |
| `pub fn new(raw: Vec<u8>, file_id: FileId) -> Self` | Erzeugt einen neuen `FileKey`. | `src/types_local.rs` |
| `pub fn new(payload: Vec<u8>) -> Self` | Erzeugt einen neuen Envelope. | `src/types_local.rs` |
| `pub fn new(ranges: Vec<(KeyBuf, KeyBuf)>) -> Self` | Erzeugt eine Bereichsmenge aus Intervallen. | `src/types_local.rs` |
| `pub fn new(scope_id: [u8; 16], affected_units: Vec<u64>) -> Self` | Erzeugt einen neuen Geltungsbereich. | `src/types_local.rs` |
| `pub fn new(layer: LayerId, class: MeasureClass, method_id: u16) -> Self` | Erzeugt einen unbewaffneten Scanner. | `src/types_local.rs` |
| `pub fn new( data: Vec<u8>, vfs_raw: std::sync::Arc<dyn crate::types_local::Vfs>, path: std::path::PathBuf, ) -> Self` | Erzeugt ein MemoryVfsFile aus vorhandenen Bytes. | `src/types_local.rs` |
| `pub fn new(tenant_id: TenantId, tx_id: TxId) -> Self` | Erzeugt einen neuen `RequestContext`. | `src/types_local.rs` |
| `pub fn new(deleted_keys_hash: [u8; 32]) -> Self` | Erzeugt einen neuen `SagaRecord`. | `src/types_local.rs` |
| `pub fn new(target_id: impl Into<String>, target_hash: [u8; 32]) -> Self` | Erzeugt ein neues ScanTarget. | `src/types_local.rs` |
| `pub fn new( layer: WitnessLayer, scanned: u64, remaining: u64, technique: ScanTechnique, digest: [u8; 32], measured_at: u64, ) -> Self` | Erzeugt einen neuen `ScanWitness`. | `src/types_local.rs` |
| `pub fn new() -> Self` | Creates a new [`SystemClock`]. | `src/types_local.rs` |
| `pub fn new(start: u64) -> Self` | Creates a new [`SequentialIdGen`]. | `src/types_local.rs` |
| `pub fn new(token: Option<Arc<()>>) -> Self` | Erzeugt ein neues FencePermit. | `src/types_local.rs` |
| `pub fn open_vfs_file( vfs: std::sync::Arc<dyn crate::types_local::Vfs>, path: &std::path::Path, opts: &VfsOpenOptions, ) -> Result<Box<dyn VfsFile>, crate::error::ErasureError>` | Öffnet eine Datei im VFS als positionelles `VfsFile` Handle (In-Memory-Simulation). | `src/types_local.rs` |
| `pub fn payload(&self) -> &[u8]` | Gibt eine Referenz auf die Payload zurück. | `src/types_local.rs` |
| `pub fn ranges(&self) -> &[(KeyBuf, KeyBuf)]` | Liefert die Bereiche. | `src/types_local.rs` |
| `pub fn read_only() -> Self` | Konfiguriert Optionen für reinen Lesezugriff. | `src/types_local.rs` |
| `pub fn record(&mut self, files: u64, bytes: u64)` | Zeichnet geprüfte Dateien und Bytes auf. | `src/types_local.rs` |
| `pub fn record_control_match(&mut self)` | Registriert eine bestandene Positivkontrolle (Control Match). | `src/types_local.rs` |
| `pub fn record_unit(&mut self, unit_hash: [u8; 32], is_live_residual: bool)` | Registriert eine gescannte Einheit. | `src/types_local.rs` |
| `pub fn remaining(&self) -> u64` | Gibt die Anzahl der verbleibenden Lebendeinträge zurück. | `src/types_local.rs` |
| `pub fn scanned(&self) -> u64` | Gibt die Anzahl der während der Verifikation gescannten Einheiten zurück. | `src/types_local.rs` |
| `pub fn scope_id(&self) -> &[u8` | Liefert die Scope-ID. | `src/types_local.rs` |
| `pub fn technique(&self) -> &ScanTechnique` | Gibt die angewandte Scantechnik zurück. | `src/types_local.rs` |
| `pub fn try_new(bytes: Vec<u8>) -> Result<Self, String>` | Erzeugt eine geprüfte Instanz. | `src/types_local.rs` |
| `pub fn verify_chain_hash(&self, prev_hash: &[u8; 32]) -> bool` | Verifiziert den Ketten-Hash. | `src/types_local.rs` |
| `pub fn witness_hash(&self) -> &[u8` | Liefert den Hash. | `src/types_local.rs` |
| `pub fn witness_layer(&self) -> &WitnessLayer` | Gibt die gescannte Storage-Schicht als &WitnessLayer zurück. | `src/types_local.rs` |
| `pub fn write_create() -> Self` | Konfiguriert Optionen für Schreibzugriff mit Dateierzeugung. | `src/types_local.rs` |
| `pub fn capacity(&self) -> usize` | Gesamtkapazität über alle 16 Shards. | `src/vault.rs` |
| `pub fn create_dir( vfs: &Arc<dyn crate::types_local::Vfs>, vault_dir: &Path, vault_id: [u8; 16], mac_key: [u8; 32], planned_docs: usize, created_seq: u64, ) -> Result<Self, ErasureError>` | Erstellt einen neuen dauerhaften Tresor mit 16 Shards in `vault_dir` über VFS (SPEC D-111, D-112). | `src/vault.rs` |
| `pub fn destroy_slot( &mut self, slot_type: SlotType, tag: [u8; 16], ) -> Result<Destroyed, ErasureError>` | Vernichtet einen Slot (D-111 Abs. 3, D-113, I-23): | `src/vault.rs` |
| `pub fn durable_shard(&self, index: usize) -> Option<&DurableVaultShard>` | Zugriff auf dauerhaften Shard. | `src/vault.rs` |
| `pub fn durable_shard_mut(&mut self, index: usize) -> Option<&mut DurableVaultShard>` | Veränderlicher Zugriff auf dauerhaften Shard. | `src/vault.rs` |
| `pub fn get_slot( &self, slot_type: SlotType, tag: [u8; 16], ) -> Result<Option<SlotData>, ErasureError>` | Liest einen Slot (ER-02, D-111). | `src/vault.rs` |
| `pub fn initial_shard_capacity(planned_docs: usize) -> usize` | Berechnet die Anfangskapazität je Shard nach SPEC D-111 Abs. 7: `nextpow2(planned_docs * 1.3 / 16 / 0.5).max(1024)` | `src/vault.rs` |
| `pub fn is_poisoned(&self) -> bool` | Prüft ob mindestens ein Shard vergiftet ist (D-55, I-10). | `src/vault.rs` |
| `pub fn live_count(&self) -> usize` | Gesamtzahl aktiver Slots über alle 16 Shards. | `src/vault.rs` |
| `pub fn load_exceeded(&self) -> bool` | Prüft ob die Lastgrenze in mindestens einem Shard überschritten ist (λ > 0,90). | `src/vault.rs` |
| `pub fn mac_key(&self) -> &[u8` | MAC-Schlüssel. | `src/vault.rs` |
| `pub fn new_in_memory(total_capacity: usize) -> Self` | Erstellt einen leeren In-Memory-Tresor mit 16 Shards und gegebener Gesamtkapazität. | `src/vault.rs` |
| `pub fn new_in_memory_with_id_and_key( total_capacity: usize, vault_id: [u8; 16], mac_key: [u8; 32], ) -> Self` | Erstellt einen In-Memory-Tresor mit vault_id und MAC-Schlüssel. | `src/vault.rs` |
| `pub fn new_in_memory_with_key(total_capacity: usize, mac_key: [u8; 32]) -> Self` | Erstellt einen In-Memory-Tresor mit einem spezifischen MAC-Schlüssel. | `src/vault.rs` |
| `pub fn open_dir( vfs: &Arc<dyn crate::types_local::Vfs>, vault_dir: &Path, expected_vault_id: &[u8; 16], mac_key: &[u8; 32], ) -> Result<Self, ErasureError>` | Öffnet einen bestehenden dauerhaften Tresor über VFS nach SPEC D-111 Abs. 8 / D-112: | `src/vault.rs` |
| `pub fn put_slot( &mut self, slot_type: SlotType, tag: [u8; 16], payload: [u8; 32], ) -> Result<(), ErasureError>` | Speichert einen Slot (ER-04, D-111, I-9). | `src/vault.rs` |
| `pub fn put_slot_with_gen( &mut self, slot_type: SlotType, tag: [u8; 16], payload: [u8; 32], gen: u64, ) -> Result<(), ErasureError>` | Speichert einen Slot mit explizitem Generationszähler (u48). | `src/vault.rs` |
| `pub fn replace_durable_shard(&mut self, index: usize, shard: DurableVaultShard)` | Ersetzt einen dauerhaften Shard (z.B. nach Rehash). | `src/vault.rs` |
| `pub fn shard(&self, index: usize) -> Option<&VaultTable>` | Zugriff auf einzelnen Shard (In-Memory). | `src/vault.rs` |
| `pub fn shard_file_name(shard_no: u8) -> String` | Berechnet den Shard-Dateinamen `<nn>.vlt` für Shard-Index `0..15`. | `src/vault.rs` |
| `pub fn shard_index(&self, slot_type: SlotType, tag: &[u8; 16]) -> usize` | Ermittelt den zuständigen Shard nach SPEC D-111 Abs. 1. | `src/vault.rs` |
| `pub fn shard_mut(&mut self, index: usize) -> Option<&mut VaultTable>` | Veränderlicher Zugriff auf einzelnen Shard (In-Memory). | `src/vault.rs` |
| `pub fn sync_all_shards(&self) -> Result<(), ErasureError>` | Führt Gruppen-Sync für alle Shards aus (D-111 Abs. 9, I-9). | `src/vault.rs` |
| `pub fn tombstone_count(&self) -> usize` | Gesamtzahl Grabsteine über alle 16 Shards. | `src/vault.rs` |
| `pub fn vault_dir(&self) -> Option<&Path>` | Vault-Verzeichnis (falls dauerhaft). | `src/vault.rs` |
| `pub fn vault_id(&self) -> &[u8` | Vault-ID. | `src/vault.rs` |
| `pub fn vfs(&self) -> Option<&Arc<dyn crate::types_local::Vfs>>` | Vfs-Instanz (falls dauerhaft). | `src/vault.rs` |
| `pub fn capacity(&self) -> usize` | Shard-Kapazität. | `src/vault/durable.rs` |
| `pub fn collect_active_slots(&self) -> Result<Vec<SlotData>, ErasureError>` | Sammelt alle aktiven Slots für den Rehash (Grabsteine werden nicht übernommen, D-111 Abs. 5). | `src/vault/durable.rs` |
| `pub fn create_file( vfs: &Arc<dyn Vfs>, path: PathBuf, shard_no: u8, capacity: usize, vault_id: [u8; 16], mac_key: [u8; 32], created_seq: u64, ) -> Result<Self, ErasureError>` | Erstellt eine neue dauerhafte Shard-Datei über VFS. | `src/vault/durable.rs` |
| `pub fn destroy( &mut self, slot_type: SlotType, tag: &[u8; 16], ) -> Result<super::table::Destroyed, ErasureError>` | Vernichtet einen Slot nach SPEC D-111 Abs. 3, D-113 und I-23: 1. Schreibt in EINEM 64-Byte-Schreibvorgang das Grabstein-Abbild (tombstone_i… | `src/vault/durable.rs` |
| `pub fn find(&self, slot_type: SlotType, tag: &[u8; 16]) -> Result<Option<usize>, ErasureError>` | Findet den Slot-Index für (slot_type, tag) durch lineare Sondierung. | `src/vault/durable.rs` |
| `pub fn flush_meta(&self) -> Result<(), ErasureError>` | Aktualisiert den Metablock auf Platte. | `src/vault/durable.rs` |
| `pub fn generation(&self) -> u64` | Shard-Generation. | `src/vault/durable.rs` |
| `pub fn get( &self, slot_type: SlotType, tag: &[u8; 16], ) -> Result<Option<SlotData>, ErasureError>` | Liest und decodiert einen Slot. | `src/vault/durable.rs` |
| `pub fn is_poisoned(&self) -> bool` | Prüft ob der Shard vergiftet ist (D-55, I-10). | `src/vault/durable.rs` |
| `pub fn live_count(&self) -> usize` | Geschätzte Live-Anzahl laut Metablock. | `src/vault/durable.rs` |
| `pub fn load_exceeded(&self) -> bool` | Prüft ob Lastfaktor mit nächstem Insert > 0,90 wäre (D-111 Abs. 4). | `src/vault/durable.rs` |
| `pub fn load_factor(&self) -> f64` | Aktueller Lastfaktor λ = (live + tombstones) / capacity. | `src/vault/durable.rs` |
| `pub fn mac_key_ref(&self) -> &[u8` | MAC-Schlüssel Referenz. | `src/vault/durable.rs` |
| `pub fn must_force_rehash(&self) -> bool` | Prüft ob Rehash erzwungen werden muss (λ > 0,80). | `src/vault/durable.rs` |
| `pub fn open_file( vfs: &Arc<dyn Vfs>, path: PathBuf, expected_shard_no: u8, expected_vault_id: &[u8; 16], mac_key: &[u8; 32], ) -> Result<Self, ErasureError>` | Öffnet eine bestehende dauerhafte Shard-Datei über VFS. | `src/vault/durable.rs` |
| `pub fn overwrite_with_zeros(&mut self) -> Result<(), ErasureError>` | Überschreibt die Shard-Datei über das gehaltene Handle vollständig mit Nullen (D-111 Abs. 5). | `src/vault/durable.rs` |
| `pub fn path(&self) -> &Path` | Shard-Pfad. | `src/vault/durable.rs` |
| `pub fn poison(&self)` | Vergiftet den Shard irreversibel (D-55, I-10). | `src/vault/durable.rs` |
| `pub fn put(&mut self, sd: SlotData) -> Result<(), ErasureError>` | Schreibt einen Slot dauerhaft via VFS (genau ein 64-Byte-Schreibvorgang, I-23). | `src/vault/durable.rs` |
| `pub fn read_slot_raw_pub(&self, idx: usize) -> Result<[u8` | Liest rohe 64-Byte Slot-Bytes an Index `idx`. | `src/vault/durable.rs` |
| `pub fn rsv_cursor(&self) -> u64` | Scrub-Cursor (`rsv`). | `src/vault/durable.rs` |
| `pub fn set_generation(&mut self, gen: u64)` | Setzt die Shard-Generation. | `src/vault/durable.rs` |
| `pub fn set_path(&mut self, path: PathBuf)` | Setzt den Pfad. | `src/vault/durable.rs` |
| `pub fn set_rsv_cursor(&mut self, cursor: u64)` | Setzt den Scrub-Cursor (`rsv`). | `src/vault/durable.rs` |
| `pub fn shard_no(&self) -> u8` | Shard-Nummer (0..15). | `src/vault/durable.rs` |
| `pub fn should_background_rehash(&self) -> bool` | Prüft ob Rehash im Hintergrund ausgelöst werden soll (λ > 0,60). | `src/vault/durable.rs` |
| `pub fn sync_data(&self) -> Result<(), ErasureError>` | Synchronisiert die Datenblöcke der Datei (`sync_data`). Bei Fehler: Shard wird vergiftet (D-55, I-10). | `src/vault/durable.rs` |
| `pub fn tombstone_count(&self) -> usize` | Geschätzte Tombstone-Anzahl laut Metablock. | `src/vault/durable.rs` |
| `pub fn vault_id(&self) -> &[u8` | Vault-ID Referenz. | `src/vault/durable.rs` |
| `pub fn as_bytes(&self) -> &[u8` | Zugriff auf die rohen Schlüssel-Bytes als Referenz. | `src/vault/key_store.rs` |
| `pub fn file_id_to_tag(id: FileId) -> [u8` | Erzeugt das 16-Byte-Tag für eine `FileId` nach SPEC F.3. | `src/vault/keyring.rs` |
| `pub fn mask_file_key(raw_key: &[u8; 32], tag: &[u8; 16], k_enc: &[u8; 32]) -> [u8` | Maskiert oder demaskiert den Dateischlüssel mit HKDF-Expand nach SPEC F.3 / D-85. | `src/vault/keyring.rs` |
| `pub fn new() -> Self` | Erstellt einen neuen `VaultKeyring` mit eigener In-Memory-Tresor-Instanz. | `src/vault/keyring.rs` |
| `pub fn new_in_memory() -> Self` | Erstellt einen neuen In-Memory-Keyring mit eigener Tresor-Instanz. | `src/vault/keyring.rs` |
| `pub fn with_vault(vault: Arc<RwLock<FileVault>>, k_enc: [u8; 32]) -> Self` | Erstellt einen neuen `VaultKeyring` über einen bestehenden Tresor. | `src/vault/keyring.rs` |
| `pub fn compute_metablock_mac(mac_key: &[u8; 32], meta_pre_mac: &[u8]) -> [u8` | Berechnet den Metablock-MAC nach SPEC F.3: `HMAC-SHA256(K_vm, Bytes[0..50))[0..8]`. | `src/vault/open.rs` |
| `pub fn encode_metablock(params: MetablockParams<'_>) -> [u8` | Encodiert einen Metablock (64 Bytes) nach SPEC F.3. | `src/vault/open.rs` |
| `pub fn open_vault(header: &[u8]) -> Result<VaultOpenOutcome, ErasureError>` | Abwärtskompatible Hilfsfunktion für Header-Öffnung. | `src/vault/open.rs` |
| `pub fn open_vault_shard( raw_shard: &[u8], expected_vault_id: &[u8; 16], mac_key: &[u8; 32], ) -> Result<VaultOpenOutcome, ErasureError>` | Öffnet und prüft einen Tresor-Shard (Kopf + Metablock + Stichprobe, kein Vollscan, D-111 Abs. 8). | `src/vault/open.rs` |
| `pub fn rehash( old: &VaultTable, vault_id: [u8; 16], mac_key: [u8; 32], ) -> Result<VaultTable, ErasureError>` | Rückwärtskompatible Hilfsfunktion für Rehash einer Tabelle. | `src/vault/rehash.rs` |
| `pub fn rehash_durable_shard( old: &mut DurableVaultShard, vfs: &Arc<dyn Vfs>, erase_pending: Option<&AtomicBool>, ) -> Result<(DurableVaultShard, RehashOutcome), ErasureError>` | Führt Rehash für einen dauerhaften Shard über VFS durch nach SPEC D-111 Abs. 4 bis 6: - Prüft ob Abbruch durch wartendes erase angefordert… | `src/vault/rehash.rs` |
| `pub fn rehash_shard( old: &mut VaultTable, vault_id: [u8; 16], mac_key: [u8; 32], ) -> Result<(VaultTable, RehashOutcome), ErasureError>` | Führt Rehash für einen In-Memory-Shard durch. | `src/vault/rehash.rs` |
| `pub fn scrub_shard_step<C: VaultScrubContext>( vault: &mut FileVault, shard_idx: usize, batch_size: usize, ctx: &C, ) -> Result<ScrubOutcome, ErasureError>` | Führt einen Scrub-Schritt über bis zu `batch_size` Slots eines Shards durch. | `src/vault/scrub.rs` |
| `pub fn as_bytes(&self) -> &[u8` | Gibt eine Referenz auf die geschützten Bytes zurück. | `src/vault/sealer.rs` |
| `pub fn derive_dek_from_vault( &self, hierarchy: &DocumentHierarchyTags, ) -> Result<[u8` | Leitet den DEK aus den Ahnen-Salts im Tresor ab (D-79). | `src/vault/sealer.rs` |
| `pub async fn execute_rewrite( &self, rewrite_id: u64, target_id: &str, pre_digest: [u8; 32], crash_after_step: Option<u8>, ) -> Result<RewriteOutcome, RewriteError>` | Führt die 5-Phasen-Rewrite-Pipeline absturzsicher und fail-closed aus. 1. Phase 1: Intent ins Journal schreiben (`Planned`). 2. Phase 2: Üb… | `src/vault/sealer.rs` |
| `pub fn get_entry(&self, target_id: &str) -> Option<ManifestEntry>` | Liest einen Manifest-Eintrag. | `src/vault/sealer.rs` |
| `pub fn get_intent(&self, rewrite_id: u64) -> Option<RewriteIntent>` | Liest einen Intent anhand seiner ID. | `src/vault/sealer.rs` |
| `pub fn is_poisoned(&self) -> bool` | Prüft ob das Sealer-Handle vergiftet ist (INV-VAULT-FAILCLOSED-1). | `src/vault/sealer.rs` |
| `pub fn is_poisoned(&self) -> bool` | Prüft ob der Rewriter dauerhaft vergiftet ist (INV-VAULT-FAILCLOSED-1). | `src/vault/sealer.rs` |
| `pub fn journal(&self) -> &RewriteJournal` | Liefert eine Referenz auf das Journal. | `src/vault/sealer.rs` |
| `pub fn manifest(&self) -> &RewriteManifest` | Liefert eine Referenz auf das Manifest. | `src/vault/sealer.rs` |
| `pub fn new() -> Self` | Erstellt einen neuen `VaultSealer` im Speicher. | `src/vault/sealer.rs` |
| `pub fn new() -> Self` | Erstellt ein neues In-Memory RewriteJournal. | `src/vault/sealer.rs` |
| `pub fn new() -> Self` | Erstellt ein neues leeres In-Memory RewriteManifest. | `src/vault/sealer.rs` |
| `pub fn new( journal: RewriteJournal, manifest: RewriteManifest, backend: Arc<dyn StorageBackend>, clock: Arc<dyn Clock>, ) -> Self` | Erstellt einen neuen BlobRewriter. | `src/vault/sealer.rs` |
| `pub fn new_in_memory() -> Self` | Erstellt einen neuen `VaultSealer` im Speicher. | `src/vault/sealer.rs` |
| `pub fn poison(&self)` | Vergiftet das Sealer-Handle dauerhaft (Fail-Closed). | `src/vault/sealer.rs` |
| `pub fn poison(&self)` | Vergiftet das Rewriter-Handle dauerhaft. | `src/vault/sealer.rs` |
| `pub fn record_intent(&self, mut intent: RewriteIntent) -> Result<(), RewriteError>` | Erfasst den initialen RewriteIntent (Phase 1: Planned). | `src/vault/sealer.rs` |
| `pub async fn recover_and_reconcile(&self) -> Result<usize, RewriteError>` | Wiederanlauf nach Absturz (Crash Recovery): Prüft alle uncommitted Intents im Journal: - Wurde das Manifest vor dem Absturz bereits aktuali… | `src/vault/sealer.rs` |
| `pub fn safe_swap_buffers( &self, dest: &mut [u8], src: &mut [u8], fail_at_step: Option<u8>, ) -> Result<(), SealerError>` | Sicherer Speicher-Swap mit Fail-Closed Garantie (WP-5a, INV-VAULT-FAILCLOSED-1). Tauscht die Inhalte von `dest` und `src` byteweise aus. Fa… | `src/vault/sealer.rs` |
| `pub fn uncommitted_intents(&self) -> Vec<RewriteIntent>` | Liefert alle nicht abgeschlossenen Intents (zur Wiederanlaufprüfung). | `src/vault/sealer.rs` |
| `pub fn update_entry( &self, target_id: String, is_erased: bool, updated_at_nanos: u64, rewrite_id: u64, ) -> Result<u64, RewriteError>` | Aktualisiert einen Eintrag atomar im Manifest (Phase 4). | `src/vault/sealer.rs` |
| `pub fn update_state( &self, rewrite_id: u64, new_state: RewriteState, ) -> Result<(), RewriteError>` | Aktualisiert den Zustand eines Intents im Journal. | `src/vault/sealer.rs` |
| `pub fn version(&self) -> u64` | Aktuelle Manifest-Version. | `src/vault/sealer.rs` |
| `pub fn with_vault(vault: Arc<RwLock<FileVault>>, k_unit: [u8; 32]) -> Self` | Erstellt einen `VaultSealer` über einen gegebenen Tresor mit K_unit-Schlüssel. | `src/vault/sealer.rs` |
| `pub fn with_vfs(vfs: Arc<dyn Vfs>, journal_path: std::path::PathBuf) -> Self` | Erstellt ein neues VFS-gestütztes RewriteJournal. | `src/vault/sealer.rs` |
| `pub fn with_vfs(vfs: Arc<dyn Vfs>, manifest_path: std::path::PathBuf) -> Self` | Erstellt ein neues VFS-gestütztes RewriteManifest. | `src/vault/sealer.rs` |
| `pub fn compute_header_tag(mac_key: &[u8; 32], header_pre_tag: &[u8]) -> [u8` | Berechnet den 16-Byte Header-Tag für einen Tresor-Shard-Kopf nach SPEC F.1 / F.3: `HMAC-SHA256(K_vm, Bytes[0..48))[0..16]`. | `src/vault/slot.rs` |
| `pub fn compute_slot_mac(mac_key: &[u8; 32], vault_id: &[u8; 16], slot_pre_mac: &[u8]) -> [u8` | Berechnet den 8-Byte Slot-MAC nach SPEC F.3: `HMAC-SHA256(K_vm, vault_id ‖ Bytes[0..56))[0..8]`. | `src/vault/slot.rs` |
| `pub fn decode_slot( raw: &[u8; VAULT_SLOT_SIZE], vault_id: &[u8; 16], mac_key: &[u8; 32], ) -> Result<Option<SlotData>, ErasureError>` | Deserialisiert einen Slot aus 64 Bytes nach SPEC F.3. Gibt `Ok(None)` bei leeren oder Grabstein-Slots zurück, `Err(Corrupt)` bei ungültigem… | `src/vault/slot.rs` |
| `pub fn encode_slot( vault_id: &[u8; 16], slot_type: SlotType, status: u8, gen: u64, tag: &[u8; 16], payload: &[u8; 32], mac_key: &[u8; 32], ) -> [u8` | Serialisiert einen aktiven Slot in 64 Bytes inkl. MAC (F.3, D-111). | `src/vault/slot.rs` |
| `pub fn encode_slot_body(sd: &SlotData) -> [u8` | Serialisiert die Nutzlast eines aktiven Slots (ohne MAC). | `src/vault/slot.rs` |
| `pub fn encode_vault_header( file_id: &[u8; 16], created_seq: u64, mac_key: &[u8; 32], ) -> [u8` | Encodiert einen F.1-Dateikopf (64 Bytes) für einen Tresor-Shard (kind = 10, key_mode = 1) nach SPEC F.1 / F.3. | `src/vault/slot.rs` |
| `pub fn from_u8(b: u8) -> Option<Self>` | Konvertiert u8 in SlotType. | `src/vault/slot.rs` |
| `pub fn mask_epoch_key(raw_key: &[u8; 32], epoch_id: &[u8; 16], k_enc: &[u8; 32]) -> [u8` | Maskiert oder demaskiert einen Epochenschlüssel (Slot-Typ E) nach SPEC D-125 / F.3: `payload = K_epoch ⊕ HKDF-Expand(K_enc, "ctx/emask/v1"… | `src/vault/slot.rs` |
| `pub fn slot_probe_start(slot_type: SlotType, tag: &[u8; 16], capacity: usize) -> (u8, usize)` | Bestimmt Shard und Startindex für lineare Sondierung nach SPEC D-111 Abs. 1: `h = BLAKE3(slot_type ‖ tag)` `shard = h[0] >> 4` (0..15) `i =… | `src/vault/slot.rs` |
| `pub fn tombstone_image(vault_id: &[u8; 16], mac_key: &[u8; 32]) -> [u8` | Erstellt das kanonische Grabstein-Abbild (tombstone_image) nach SPEC D-111 Abs. 3 / F.3: Alle Bytes 0 außer `status = 0x02`, MAC über das A… | `src/vault/slot.rs` |
| `pub fn verify_vault_header( header_bytes: &[u8], mac_key: &[u8; 32], ) -> Result<([u8` | Prüft einen F.1-Dateikopf (64 Bytes) für einen Tresor-Shard. | `src/vault/slot.rs` |
| `pub fn capacity(&self) -> usize` | Kapazität. | `src/vault/table.rs` |
| `pub fn collect_active_slots(&self) -> Vec<SlotData>` | Liefert alle aktiven Slots für den Rehash (Grabsteine werden ausgelassen, D-111 Abs. 5). | `src/vault/table.rs` |
| `pub fn destroy( &mut self, slot_type: SlotType, tag: &[u8; 16], ) -> Result<Destroyed, ErasureError>` | Vernichtet einen Slot nach SPEC D-111 Abs. 3 und I-23: 1. Schreibt in EINEM 64-Byte-Schreibvorgang das Grabstein-Abbild (tombstone_image) 2… | `src/vault/table.rs` |
| `pub fn find(&self, slot_type: SlotType, tag: &[u8; 16]) -> Option<usize>` | Findet den Slot-Index für (slot_type, tag). Gibt `None` zurück wenn nicht gefunden. Stoppt bei `STATUS_EMPTY` (Sondierung endet dort, D-79,… | `src/vault/table.rs` |
| `pub fn generation(&self) -> u64` | Generation. | `src/vault/table.rs` |
| `pub fn get( &self, slot_type: SlotType, tag: &[u8; 16], ) -> Result<Option<SlotData>, ErasureError>` | Liest einen Slot (dekodiert mit MAC-Prüfung). | `src/vault/table.rs` |
| `pub fn is_poisoned(&self) -> bool` | Gibt zurück, ob der Shard vergiftet ist (D-55, I-10). | `src/vault/table.rs` |
| `pub fn live_count(&self) -> usize` | Anzahl aktiver Slots. | `src/vault/table.rs` |
| `pub fn load_exceeded(&self) -> bool` | Prüft, ob die Kapazität mit dem nächsten Einfügen erschöpft wäre (λ > 0,90, D-111 Abs. 4). | `src/vault/table.rs` |
| `pub fn load_factor(&self) -> f64` | Aktueller Lastfaktor λ = (live + tombstones) / capacity. | `src/vault/table.rs` |
| `pub fn mac_key_ref(&self) -> &[u8` | Referenz auf MAC-Schlüssel. | `src/vault/table.rs` |
| `pub fn must_force_rehash(&self) -> bool` | Prüft, ob Rehash erzwungen werden muss (λ > 0,80). | `src/vault/table.rs` |
| `pub fn new(capacity: usize, vault_id: [u8; 16], mac_key: [u8; 32]) -> Self` | Erstellt eine neue leere Tabelle mit gegebener Kapazität (muss Zweierpotenz sein). | `src/vault/table.rs` |
| `pub fn new_shard(shard_no: u8, capacity: usize, vault_id: [u8; 16], mac_key: [u8; 32]) -> Self` | Erstellt eine neue leere Shard-Tabelle mit gegebener Shard-Nummer. | `src/vault/table.rs` |
| `pub fn overwrite_with_zeros(&mut self)` | Überschreibt die gesamte Tabelle vollständig mit Nullen (D-111 Abs. 5). | `src/vault/table.rs` |
| `pub fn poison(&mut self)` | Vergiftet den Shard irreversibel (D-55, I-10). | `src/vault/table.rs` |
| `pub fn put(&mut self, sd: SlotData) -> Result<(), ErasureError>` | Schreibt einen Slot (ER-04, D-111). Gibt Fehler zurück wenn Kapazität voll oder vergiftet. | `src/vault/table.rs` |
| `pub fn rsv_cursor(&self) -> u64` | Scrub-Cursor (rsv) für vault_scrub (D-111 Abs. 8). | `src/vault/table.rs` |
| `pub fn set_generation(&mut self, gen: u64)` | Setzt die Generation. | `src/vault/table.rs` |
| `pub fn set_rsv_cursor(&mut self, cursor: u64)` | Aktualisiert den Scrub-Cursor (rsv). | `src/vault/table.rs` |
| `pub fn shard_no(&self) -> u8` | Shard-Nummer. | `src/vault/table.rs` |
| `pub fn should_background_rehash(&self) -> bool` | Prüft, ob Rehash im Hintergrund ausgelöst werden soll (λ > 0,60). | `src/vault/table.rs` |
| `pub fn slot_bytes_pub(&self, idx: usize) -> Option<&[u8` | Öffentlicher Zugriff auf rohe Slot-Bytes für Scrubbing. | `src/vault/table.rs` |
| `pub fn tombstone_count(&self) -> usize` | Anzahl Grabsteine. | `src/vault/table.rs` |
| `pub fn vault_id(&self) -> &[u8` | Vault-ID. | `src/vault/table.rs` |


---

## `acta-ids`

| Attribut | Wert |
|---|---|
| **Ring** | 0 — Ebene E0 — Autarke Bausteine & Fundamente |
| **Status** | Fertig |
| **Zweck** | Kanonische Identitätstypen, DocId, TenantId und ID_WIDTH_BITS. |
| **Unsafe-Code** | Nein (`#![forbid(unsafe_code)]`) |
| **Interne Abhängigkeiten** | — |
| **Externe Abhängigkeiten (Auswahl)** | `blake3`, `serde`, `thiserror` |

### Architekturbeschreibung

Canonical identity, tenant, and reference types for Acta.  # Architecture Role Zero internal dependencies. Standalone micro-kernel for identifiers.

### Modul- und Re-Export-Oberfläche (lib.rs)

```rust
```

### Feature-Flags

| Flag | Aktiviert |
|---|---|
| `default` | `["std"]` |
| `std` | `["blake3/std", "serde/std"]` |
| `docid-128` | `[]` |

### Öffentliche Datentypen — Structs (12)

| Struct | Felder / Form | Beschreibung | Datei |
|---|---|---|---|
| `CollectionId` | `(tuple/unit)` | Collection-ID. | `src/lib.rs` |
| `DocId` | `(tuple/unit)` | Dokument-ID. | `src/lib.rs` |
| `DocId` | `(tuple/unit)` | Dokument-ID (128-Bit). | `src/lib.rs` |
| `DocRef` | `{ /// Optionaler Betroffener (Subject). pub subject: Option<SubjectRef>, /// Eindeutiger Dokumentenschlüssel innerhalb des Clusters. pub key: DocKey, }` | Eindeutige Dokumentenadresse: Betroffener ist unveränderlicher Teil der Identität (SPEC 7.2, D-70). | `src/lib.rs` |
| `EntityId` | `(tuple/unit)` | Entity-ID für Graph- und semantische Knoten. | `src/lib.rs` |
| `FileId` | `(tuple/unit)` | 16-Byte Dateikennung (SPEC F.1). | `src/lib.rs` |
| `Seq` | `(tuple/unit)` | — | `src/lib.rs` |
| `StoreId` | `(tuple/unit)` | 16-Byte Speicherinstanz-Kennung (SPEC F.1). | `src/lib.rs` |
| `TenantId` | `(tuple/unit)` | Mandanten-ID. | `src/lib.rs` |
| `TenantScoped` | `{ tenant_id: TenantId, value: T, }` | Ein Wrapper-Typ, der einen Wert untrennbar an eine `TenantId` bindet. | `src/lib.rs` |
| `Timestamp` | `(tuple/unit)` | — | `src/lib.rs` |
| `TxId` | `(tuple/unit)` | Transaktions-Identifikator. | `src/lib.rs` |

### Öffentliche Datentypen — Enums (2)

| Enum | Varianten | Beschreibung | Datei |
|---|---|---|---|
| `IdError` | `/// Key string cannot be empty. #[error("Key cannot be empty")] EmptyKey, /// Identifier cannot be empty. #[error("empty identifier")] EmptyIdentifier, /// Identifier exceeds maximum allowed length.…` | Specific error type for identifier, reference, and tenant scope operations. | `src/lib.rs` |
| `TenantScopeViolation` | `/// Raised when unpacking a `TenantScoped<T>` value with an expected `TenantId` that differs /// from the value's actual bound `TenantId`. #[error("tenant scope mismatch: expected {expected:?}, got {…` | Errors raised when attempting an illegal or mismatched tenant scope unpacking operation. | `src/lib.rs` |

### Typ-Aliase (1)

| Alias | Definition | Beschreibung |
|---|---|---|
| `Result` | `type Result<T> = core::result::Result<T, IdError>` | Result alias for identifier operations. |

### Konstanten (18)

| Konstante | Typ | Beschreibung |
|---|---|---|
| `DEFAULT` | `Self` | Der implizite Default-Mandant für Single-Tenant-Deployments. |
| `ID_WIDTH_BITS` | `u32` | Bit width of `DocId` and associated identifiers in the current build. |
| `INTERNAL_BASE` | `u64` | Lower bound of the internal system transaction ID range. |
| `INVALID` | `Self` | Invalid tenant identifier sentinel value (`0`). |
| `INVALID` | `Self` | Invalid collection identifier sentinel value (`0`). |
| `INVALID` | `Self` | Invalid or uninitialized transaction identifier sentinel value (`0`). |
| `MAX` | `Self` | Maximum possible `DocId` value (`u64::MAX`). |
| `MAX` | `Self` | Maximum possible `DocId` value (`u128::MAX`). |
| `MAX_COLLECTION_SEQUENCE` | `u64` | Upper bound of the Collection-sequenced transaction ID range (`10^12`). |
| `MAX_REF_LEN` | `usize` | Maximale Bytelänge für Referenz-Strings / Byte-Identifier. |
| `MAX_SEARCH_K` | `usize` | Maximale Anzahl an Suchergebnissen (Top-K Begrenzung). |
| `MIN` | `Self` | Minimum possible `DocId` value (`0`). |
| `MIN` | `Self` | Minimum possible `DocId` value (`0`). |
| `SYSTEM` | `Self` | SYSTEM tenant identifier (`0`). |
| `SYSTEM` | `Self` | SYSTEM collection identifier (`0`). |
| `TOMBSTONE_BIT` | `u64` | Bit 63 einer Sequenznummer kennzeichnet einen Tombstone (Soft-Deletion). |
| `ZERO` | `Self` | Minimale Sequenznummer. |
| `ZERO` | `Self` | Null-Zeitstempel. |

### Öffentliche Funktionen (18)

| Signatur | Beschreibung | Datei |
|---|---|---|
| `pub fn as_bytes(&self) -> Vec<u8>` | Converts the entity identifier into its string byte representation. | `src/lib.rs` |
| `pub fn as_bytes(&self) -> &[u8]` | Liefert die Bytes als Slice. | `src/lib.rs` |
| `pub fn from_doc_id(doc_id: DocId) -> Self` | Creates an `EntityId` directly from a `DocId`. | `src/lib.rs` |
| `pub fn from_doc_id(doc_id: DocId) -> Self` | Creates an `EntityId` directly from a `DocId`. | `src/lib.rs` |
| `pub fn from_key(key: &str) -> Result<Self>` | Derives a `DocId` from a string key using the first 8 bytes (64 bits) of its BLAKE3 hash. | `src/lib.rs` |
| `pub fn from_key(key: &str) -> Result<Self>` | Derives a 128-bit `DocId` from a string key using the first 16 bytes (128 bits) of its BLAKE3 hash. | `src/lib.rs` |
| `pub fn from_key(key: &str) -> Result<Self>` | Derives an `EntityId` from a string key using the first 8 bytes of its BLAKE3 hash. | `src/lib.rs` |
| `pub fn into_inner(self) -> Vec<u8>` | Verbraucht den Wrapper und liefert den Vektor. | `src/lib.rs` |
| `pub fn into_inner_checked( self, expected: &TenantId, ) -> core::result::Result<T, TenantScopeViolation>` | Gibt den Wert nur zurück, wenn die aufrufende Seite die erwartete `TenantId` nennt. | `src/lib.rs` |
| `pub fn is_valid_origin(&self) -> bool` | Checks if the transaction ID originates from a valid range. | `src/lib.rs` |
| `pub fn map<U>(self, f: impl FnOnce(T) -> U) -> TenantScoped<U>` | Transforms the inner value `T` into `U` using `f` while preserving the bound `TenantId`. | `src/lib.rs` |
| `pub fn new(tenant_id: TenantId, value: T) -> Self` | Creates a new `TenantScoped` value bound to the specified `TenantId`. | `src/lib.rs` |
| `pub fn tenant_id(&self) -> &TenantId` | Returns a reference to the bound `TenantId`. | `src/lib.rs` |
| `pub fn try_from_internal_offset(offset: u64) -> Result<Self>` | Safely constructs an internal system TxId from an offset relative to `TxId::INTERNAL_BASE`. | `src/lib.rs` |
| `pub fn try_new(id: u64) -> Result<Self>` | Sicherer Konstruktor. Gibt Err wenn id == 0. | `src/lib.rs` |
| `pub fn try_new(id: u64) -> Result<Self>` | Creates a new `CollectionId`, ensuring `id != 0`. | `src/lib.rs` |
| `pub fn try_new(bytes: Vec<u8>) -> Result<Self>` | Erzeugt eine geprüfte Instanz. Verstoß führt zu `IdError`. | `src/lib.rs` |
| `pub fn value(&self) -> &T` | Referenz auf den inneren Wert. | `src/lib.rs` |


---

## Anhang A — Querschnittliche Invarianten (gelten crateübergreifend)

* **Zero-Panic-Doktrin:** `unwrap_used`, `expect_used`, `panic`, `todo`, `unimplemented`, `unreachable` sind workspace-weit als Clippy-Lint verboten.
* **`unsafe_code = "deny"`** ist Workspace-Default; nur `acta-sys`, `acta-simd`, `acta-wire` heben dies explizit auf.
* **Sync-Kern, Async-Schale (P26):** Ring-0-Crates verwenden kein `tokio`.
* **DAG-Integrität (P5):** Aufwärtskanten sind verboten; einzige dokumentierte Restausnahme: `acta-router → acta-db` (Migrationsartefakt).
* **`#[non_exhaustive]`-Disziplin** bei zentralen Fehler-Enums (u. a. `ActaError`).
* **Deletion-Proof-Kette:** kryptographisch extern verifizierbare Löschbeweise (`acta-crypto`).
* **Feature-Ring-Trennung:** `fast` / `sovereign` / `compliance` (Crate `acta`) trennen Lizenzstufen auf Kompilierungsebene.

## Anhang B — Nicht tiefenanalysierte Hilfs-Targets

| Target | Pfad | Rolle |
|---|---|---|
| `acta-bench` | `benchmarks/acta-bench` | Criterion-Benchmark-Harness, kein Teil der Produktions-API. |
| `xtask` | `xtask/` | Cargo-`xtask`-Automatisierungsbinary, von `[workspace] exclude` ausgenommen. |

## Anhang C — Referenzierte Primärquellen im Repository

* `README.md`, `ARCHITECTURE.md`, `CONSTITUTION.md`, `VETOES.md`, `AGENTS.md`
* `capabilities.toml`, `deny.toml`, `supply-chain/`
* `docs/spec/ACTA_FINALE_PRODUKTSPEZIFIKATION.md`
* `docs/refactor/router-db-edge-audit.md`, `docs/refactor/mcp-upward-edges-audit.md`

---

*Ende der globalen Schnittstellenspezifikation — automatisch generiert. Für Implementierungsdetails unterhalb der öffentlichen API-Grenze (private Module, `pub(crate)`-Symbole) den Quellcode direkt konsultieren.*
