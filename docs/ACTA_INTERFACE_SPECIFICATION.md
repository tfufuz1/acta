# Acta — Globale Mikrofeingranulare Schnittstellenspezifikation

**Repository:** `acta` (Workspace-Root) · **Generiert am:** 2026-11-06
**Erzeugt durch:** `scripts/generate_interface_spec.py` (automatisierte, reproduzierbare Vollanalyse aller Workspace-Crates)
**Methodik:** Statische Extraktion aller `pub`-Symbole (Traits, Structs, Enums, Funktionen, Typ-Aliase, Konstanten) aus jedem `src/**/*.rs` je Crate, angereichert mit den Original-Doc-Kommentaren (`///`, `//!`), den `Cargo.toml`-Metadaten (Abhängigkeiten, Feature-Flags) und der `lib.rs`-Re-Export-Oberfläche. Test-Module (`#[cfg(test)] mod tests`) sind ausgeschlossen.

> **Hinweis zur Reproduzierbarkeit:** Dieses Dokument wird bei jedem Lauf von `scripts/generate_interface_spec.py` vollständig neu aus dem aktuellen Quellcode-Stand erzeugt. Es spiegelt exakt den Commit wider, gegen den es generiert wurde — bei Architekturänderungen (neue Crates, geänderte Traits/Ports) einfach das Skript erneut ausführen.

## Umfang der Analyse

| Kennzahl | Wert |
|---|---|
| Analysierte Fach-Crates (`crates/*`) | 12 |
| Extrahierte öffentliche Symbole gesamt | **1775** |
| — davon Public Traits (Schnittstellen im engeren Sinn) | 38 |
| — davon Public Structs | 304 |
| — davon Public Enums | 95 |
| — davon Public Funktionen | 1161 |
| — davon Typ-Aliase | 23 |
| — davon Konstanten | 154 |

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

**Ring 0 — Foundation:** `acta-types`, `acta-ports`, `acta-mvcc`, `acta-wire`, `acta-sys`, `acta-simd`, `acta-crypto`, `acta-vector`, `acta-text`, `acta-graph`, `acta-rank`, `acta-adapt`, `acta-core`, `acta-audit-export`, `acta-avv-generator`, `acta-license`

**Ring 1 — Persistence & Cache:** `acta-store`, `acta-kvcache`, `acta-checkpoint`

**Ring 2 — External Adapters & Sandboxing:** `acta-infer-candle`, `acta-infer-ollama`, `acta-infer-onnx`, `acta-sandbox`

**Ring 3 — Orchestration & Cognition:** `acta-engine`, `acta-cognition`, `acta-privacy`, `acta-router`, `acta-agent`, `acta-db`

**Ring 4 — Boundary & Composition Roots:** `acta`, `acta-mcp`, `acta-py`

**Tooling:** `acta-testkit`

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
| **Ring** | 0 — Ring 0 — Foundation & Pure Numerics/Types |
| **Status** | Fertig |
| **Zweck** | Kanonische Domänenmodelle, IDs, Budgets, Filter-AST und Fehlertypen. |
| **Unsafe-Code** | Nein (`#![forbid(unsafe_code)]`) |
| **Interne Abhängigkeiten** | — |
| **Externe Abhängigkeiten (Auswahl)** | `ahash`, `bincode`, `blake3`, `serde`, `serde_json`, `sha2`, `thiserror`, `zeroize` |

### Architekturbeschreibung

Reine Datentypen: IDs, Tags, Schlüssel, Fehler, Löschtypen. Kein I/O.  # Architektur-Rolle: Ring 0 (Typen-Kern)  Gemäß SPEC Abschnitt 5.1 und 8 sowie `docs/specs/01-acta-types.md`.  - Reine Datentypen: IDs, Tags, Schlüssel, Fehler, Löschtypen. Kein I/O. - Ring: 0 - Erlaubte Workspace-Abhängigkeiten: keine. - Keine asynchronen Laufzeiten, kein I/O, kein `unsafe`. - Wird eingebunden von: `acta-engine`, `acta`, `acta-crypto`, `acta-mvcc`, `acta-store`, `acta-text`, `acta-vector`, `acta-rank`, `acta-ports`.

### Modul- und Re-Export-Oberfläche (lib.rs)

```rust
pub mod erasure;
pub mod error;
pub mod error_dto;
pub mod file;
pub mod filter;
pub mod ids;
pub mod key;
pub mod labels;
pub mod limits;
pub mod model_fingerprint;
pub mod refs;
pub mod retrieval_strategy;
pub mod schema;
pub mod secret;
pub mod tags;
pub mod tenant_scope;
pub mod tombstone;
pub mod types;
pub use erasure::{ Armed, ControlEvidence, EraseMode, EraseReceipt, EraseRequest, EraseTarget, ErasureStatus, GateVariant, KeyCoverage, LayerId, LayerWitness, MeasureClass, PendingReason, Phase, RefusalReason, ResidualRisk, Residue, RetentionAttestation, RetentionCode, RiskCondition, ScanMethod, Scanner, Timing, Unarmed, };
pub use error::{ CacheDirective, ConfigError, Conflict, ActaError, Error, HnswDeletionError, Limit, Result, StepId, };
pub use error_dto::ActaErrorDto;
pub use file::{AlgIds, FileKind, FormatVersion};
pub use filter::FilterExpr;
pub use ids::{ErasureId, FileId, GateId, Seq, StoreId, Timestamp};
pub use key::{ Family, KeyBound, KeyBuf, KeyRange, Matcher, MatcherItem, Region, KEY_BOUND_LEN, KEY_LEN, };
pub use labels::{ LABEL_COLL_TAG, LABEL_COMMIT, LABEL_DEK, LABEL_DEK_SALT, LABEL_DOC_TAG, LABEL_ENC, LABEL_FILE, LABEL_FKMASK, LABEL_HEADER_MAC, LABEL_IDX, LABEL_KEYSLOT_AAD, LABEL_LEDGER, LABEL_NAME_TAG, LABEL_RECORD_AAD, LABEL_SIGN, LABEL_SUBJECT_TAG, LABEL_TENANT_TAG, LABEL_TERM_TAG, LABEL_TEXT_BINDING_AAD, LABEL_UNIT, LABEL_VAULT_MAC, LABEL_WAL, };
pub use limits::{ MAX_DIMENSION, MAX_DOCS_PER_STORE, MAX_DOC_BYTES_DEFAULT, MAX_DOC_BYTES_HARD, MAX_K, MAX_QUERY_TERMS, MAX_SNAPSHOTS_PER_TENANT, };
pub use model_fingerprint::ModelFingerprint;
pub use refs::{ CollectionRef, DocHandle, DocKey, DocRef, OperatorRef, SubjectRef, TenantContext, TenantRef, MAX_REF_LEN, };
pub use retrieval_strategy::RetrievalStrategy;
pub use schema::{DocIdWidth, ManifestSchemaVersion};
pub use secret::Secret;
pub use tags::{ClusterTag, CollTag, DocTag, NameTag, SubjectTag, TagPreimage, TenantTag, TermTag};
pub use tenant_scope::{TenantScopeViolation, TenantScoped};
pub use tombstone::{SeqBitTombstone, TombstoneSemanticsCheck};
pub use types::{ budget, domain, importance, saos, AutoExtractionMode, BudgetStrategy, CollectionId, ConfigFingerprint, ContextChunk, ContextWindow, DecayFunction, DistanceMetric, DocId, Edge, Embedding, Entity, EntityId, FusionStrategy, FusionWeights, GraphTraversalStrategy, HybridQuery, HybridQueryBuilder, ImportanceScore, LinkRelation, MemoryImportance, MemoryLink, MemoryType, OnSignalFailure, PprAlgorithm, PprConfig, QueryOptions, QueryOptionsBuilder, QueryRewriteOutput, RerankResult, Reservation, ResourceBudget, ResourceTracker, ScoredDocument, ScoredEntry, Signal, SignalFusionStrategies, TenantId, TlHfdParams, TokenBudget, TxId, WorkflowState, EXPIRY_METADATA_KEY, MAX_SEARCH_K, TOMBSTONE_BIT, };
```

### Feature-Flags

| Flag | Aktiviert |
|---|---|
| `docid-128` | `[]` |
| `auto-extraction-opt-out` | `[]` |

### Schnittstellen — Public Traits (1)

**`TombstoneSemanticsCheck`** — *Trait defining tombstone semantics for index sequence numbers and key-value records. # Bit-Mask Convention (IP-07 / ADR-041) By convention in Acta, sequence numbers (`u64`) reserve the most significant bit (bit 63) as the `TOMBSTONE_BIT` (`1 << 63`). When bit…* (`src/tombstone.rs`)

```rust
pub trait TombstoneSemanticsCheck {
/// Returns `true` if the sequence number or flags indicate a soft-deleted tombstone.
    fn is_tombstone(seq: u64, flags: u8) -> bool;

    /// Encodes a sequence number as an 8-byte little-endian tombstone payload with `TOMBSTONE_BIT` set.
    fn make_tombstone(seq: u64) -> Vec<u8>;
}
```

### Öffentliche Datentypen — Structs (63)

| Struct | Felder / Form | Beschreibung | Datei |
|---|---|---|---|
| `ActaErrorDto` | `{ /// Stable string identifier for the error variant (e.g. `"NotFound"`, `"PolicyViolation"`). pub kind: String, /// Human-readable error message. pub message: S… }` | Serializable data transfer object representing a [`ActaError`]. Used across IPC and API boundaries (e.g. Tauri frontend IPC) to preserve structured e… | `src/error_dto.rs` |
| `AlgIds` | `(tuple/unit)` | Algorithmus-IDs eines Dateikopfs (SPEC F.1: `u8[4] = [aead, kdf_or_0, mac, hash]; v1: [1, 0, 1, 1]`). | `src/file.rs` |
| `Armed` | `(tuple/unit)` | Typzustand: Scanner ist scharfgeschaltet nach erfolgreicher Positivkontrolle. | `src/erasure.rs` |
| `ClusterTag` | `(tuple/unit)` | 16-Byte Cluster-Tag für Dokumenten-Clustering (SPEC D-70, TY-06). | `src/tags.rs` |
| `CollectionId` | `(tuple/unit)` | Internal collection identifier. | `src/types/domain/ids.rs` |
| `ConfigFingerprint` | `{ /// LLM-Modell-ID (z.B. "llama-3.2-3b-instruct") pub model_id: String, /// Quantisierungsgrad als String: "Q4_K_M", "Q8_0", "F16", "BF16". /// EXPLIZIT Teil de… }` | Konfigurations-Fingerabdruck für P8-Kalibrierungs-Integrität. INVARIANTE INV-P8-1: Jede Änderung an einem der Felder MUSS `IsotonicCalibrator::invali… | `src/types/domain/misc.rs` |
| `ContextChunk` | `{ /// Document identifier associated with this chunk. pub doc_id: DocId, /// Text content of the chunk. pub content: String, /// Computed relevance score. pub re… }` | A chunk of context for LLM budget allocation. | `src/types/saos.rs` |
| `ContextWindow` | `{ /// List of included context chunks. pub chunks: Vec<ContextChunk>, /// Total aggregated token count. pub total_tokens: usize, /// Indicates whether context ch… }` | An aggregated context window constrained by a token budget. | `src/types/saos.rs` |
| `ControlEvidence` | `(tuple/unit)` | Beleg für eine beobachtete Positivkontrolle (D-72). | `src/erasure.rs` |
| `DocHandle` | `{ cluster: ClusterTag, doc: DocTag, }` | Opaker Dokumenten-Handle als Suchtreffer (SPEC 7.2, D-70). | `src/ids.rs` |
| `DocId` | `(tuple/unit)` | Internal document identifier. Under the default configuration, `DocId` wraps a 64-bit primitive (`u64`). When feature `docid-128` is enabled (ADR-082… | `src/types/domain/ids.rs` |
| `DocId` | `(tuple/unit)` | Internal document identifier (128-bit variant). | `src/types/domain/ids.rs` |
| `DocRef` | `{ /// Optionaler Betroffener (Subject). pub subject: Option<SubjectRef>, /// Eindeutiger Dokumentenschlüssel innerhalb des Clusters. pub key: DocKey, }` | Eindeutige Dokumentenadresse: Betroffener ist unveränderlicher Teil der Identität (SPEC 7.2, D-70). | `src/ids.rs` |
| `Edge` | `{ /// Source entity identifier. pub from: EntityId, /// Destination entity identifier. pub to: EntityId, /// Relationship label. pub label: Arc<str>, /// Numeric… }` | Graph directed edge representation with explicit bitemporal axis separation. # Abgrenzung Dieser Typ `acta_types::Edge` ist die domänenzentrierte Hyp… | `src/types/domain/misc.rs` |
| `Embedding` | `{ /// Raw vector data elements. pub data: Vec<f32>, }` | Vector embedding representation. | `src/types/domain/document.rs` |
| `Entity` | `{ /// Unique entity ID. pub id: EntityId, /// Canonical human-readable name. pub name: Arc<str>, /// Categorical entity type. pub entity_type: Arc<str>, /// Flex… }` | Represents a canonical node in the knowledge graph. | `src/types/domain/misc.rs` |
| `EntityId` | `(tuple/unit)` | Internal entity identifier for graph nodes. | `src/types/domain/ids.rs` |
| `EraseReceipt` | `{ /// Eindeutige Kennung des Löschvorgangs. pub id: ErasureId, /// Status der Löschung. pub status: ErasureStatus, /// Zeitstempel der Sperrung im Gate. pub gate… }` | Quittung eines Löschauftrags (SPEC 7.4). | `src/erasure.rs` |
| `EraseRequest` | `{ /// Löschziel. pub target: EraseTarget, /// Timing (sofort oder gebündelt). pub timing: Timing, /// Aufbewahrungs-Attestierung. pub retention: RetentionAttesta… }` | Löschauftrag (SPEC 7.3). | `src/erasure.rs` |
| `ErasureId` | `(tuple/unit)` | 16-Byte Lösch-Identifikator (SPEC 7.4, TY-09). | `src/ids.rs` |
| `FileId` | `(tuple/unit)` | 16-Byte Dateikennung (SPEC F.1). | `src/ids.rs` |
| `FormatVersion` | `(tuple/unit)` | Format-Version des Speicherframeworks (SPEC F.1). | `src/file.rs` |
| `FusionWeights` | `{ vector: f32, text: f32, graph: f32, /// Reserved for future metadata-signal fusion; currently always 0.0. Do not expose a public constructor parameter for this… }` | Normalized fusion weights for hybrid search. | `src/types/saos.rs` |
| `GateId` | `(tuple/unit)` | 16-Byte Gate-Kennung für ReadGate-Einträge (SPEC F.9). | `src/ids.rs` |
| `HybridQuery` | `{ /// Optional full-text query string. pub text_query: Option<String>, /// Optional vector query embedding. pub vector_query: Option<Vec<f32>>, /// Optional star… }` | A unified query traversing multiple index signals. | `src/types/saos.rs` |
| `HybridQueryBuilder` | `{ text_query: Option<String>, vector_query: Option<Vec<f32>>, graph_start_node: Option<String>, graph_strategy: Option<GraphTraversalStrategy>, fusion_weights: O… }` | Builder for HybridQuery to improve DX. # Abgrenzung `acta_types::HybridQueryBuilder` ist ein speicherfreier, reiner Value-Builder in Ring 0, der eine… | `src/types/saos.rs` |
| `ImportanceScore` | `(tuple/unit)` | Scores the importance of a memory item on a normalized scale of `0.0` to `1.0`. | `src/types/importance.rs` |
| `KeyBound` | `(tuple/unit)` | 65-Byte-Bereichsgrenze für prefixfreie Bereichssuche (SPEC 9.1, D-10). Serde nutzt manuelle Impls, da `[u8; 65]` das Derive-Limit von 32 überschreite… | `src/key.rs` |
| `KeyBuf` | `(tuple/unit)` | 78-Byte-Schlüssel fester Länge (SPEC 9.1, D-10, TY-03). Serde nutzt manuelle Impls, da `[u8; 78]` das Derive-Limit von 32 überschreitet. | `src/key.rs` |
| `KeyRange` | `{ /// Untere Grenze (inklusiv). pub lo: KeyBound, /// Obere Grenze (inklusiv). pub hi: KeyBound, }` | Inklusiver Schlüsselbereich mit fester 65-Byte-Länge (SPEC 9.1, D-10). | `src/key.rs` |
| `LayerId` | `(tuple/unit)` | Schicht-ID einer beteiligten Speicherschicht (SPEC 8.2). | `src/erasure.rs` |
| `LayerWitness` | `{ layer: LayerId, class: MeasureClass, method_id: u16, files: u64, bytes: u64, duration: Duration, hash: [u8; 32], }` | Unveränderlicher Zeuge einer Schichtmessung (D-72, I-19). Felder sind privat; matches() ist per Konstruktion immer 0. | `src/erasure.rs` |
| `Matcher` | `(tuple/unit)` | Normalisierter Matcher für Löschbereiche und Exakt-Filter (SPEC 9.1, D-10, TY-05). | `src/key.rs` |
| `MemoryImportance` | `{ /// Base LLM or heuristic evaluation score. pub base_score: ImportanceScore, /// Recency decay function strategy. pub decay: DecayFunction, /// Transaction ID… }` | Tracks the importance score and decay parameters of a document. | `src/types/importance.rs` |
| `MemoryLink` | `{ /// The target document ID. pub target: DocId, /// The type of relationship. pub relation: LinkRelation, /// The transaction ID when this link was created. pub… }` | A directional link to another memory chunk in the Zettelkasten. | `src/types/domain/misc.rs` |
| `ModelFingerprint` | `{ /// SHA-256 hash digest over model weight blob and quantization tier string. pub hash: [u8; 32], /// Model identifier or filename (e.g. "llama-3.2-3b-instruct"… }` | Uniquely identifies a model weight file and its quantization tier. Dient dazu, Modellgewicht-Dateien und Quantisierungsstufen auf Systemebene unverwe… | `src/model_fingerprint.rs` |
| `PprConfig` | `{ /// Damping factor (probability of continuing random walk vs restarting). Default: 0.85. pub damping_factor: f32, /// Maximum power-iteration steps before term… }` | Configuration parameters for Personalized PageRank (PPR). | `src/types/domain/misc.rs` |
| `QueryRewriteOutput` | `{ /// Textual reformulation intended for BM25 keyword search. pub text_query: Option<String>, /// Semantic reformulation intended for dense vector embedding sear… }` | Output of a query rewriter containing text, semantic, and graph anchor reformulations. | `src/types/saos.rs` |
| `RerankResult` | `{ /// Rank position after reranking (0-based). pub index: usize, /// Original index in the input candidate array. pub original_index: usize, /// Raw model output… }` | Result of cross-encoder reranking operation for a candidate item. | `src/types/domain/document.rs` |
| `Reservation` | `{ budget: &'a TokenBudget, amount: usize, settled: AtomicBool, }` | RAII reservation structure for guaranteed atomicity & drop-refund against double-spend. | `src/types/budget.rs` |
| `Residue` | `{ /// Anzahl gefundener Treffer / Rückstände. pub matches: u64, }` | Rückstandsbefund bei fehlgeschlagener Verifikation (SPEC 12.2). | `src/erasure.rs` |
| `ResourceBudget` | `{ /// Maximum allowed memory usage in bytes. pub memory_limit: u64, }` | Resource budget for memory management. | `src/types/budget.rs` |
| `ResourceTracker` | `{ /// The configured budget. budget: ResourceBudget, /// Current memory usage in bytes. memory_used: std::sync::atomic::AtomicU64, }` | Tracks resource usage against a budget. | `src/types/budget.rs` |
| `RetentionAttestation` | `{ /// Aufbewahrungscode. pub code: RetentionCode, /// Opake Betreiberkennung (nicht im Klartext im Bericht). pub attested_by: OperatorRef, /// Zeitstempel der At… }` | Pflicht-Attestierung zur Aufbewahrung bei jedem Löschauftrag (SPEC 7.3, D-15). | `src/erasure.rs` |
| `Scanner` | `{ layer: LayerId, class: MeasureClass, method_id: u16, files: u64, bytes: u64, _state: S, }` | Typisierter Scanner im Typzustandsmuster (TY-10, L-5). | `src/erasure.rs` |
| `ScoredDocument` | `{ /// Identifier of the scored document. pub doc_id: DocId, /// Relevance or similarity score. pub score: f32, }` | A scored search result. | `src/types/domain/document.rs` |
| `ScoredEntry` | `{ /// Unique result identifier. pub id: String, /// Combined final score after signal fusion. pub final_score: f32, /// Associated entry metadata. pub metadata:… }` | Evaluated result for hybrid/4-signal search. | `src/types/saos.rs` |
| `Secret` | `{ bytes: Vec<u8>, }` | Zeroisierender Speicherpuffer für Schlüsselmaterial und sensible Daten (SPEC 7.1, TY-11). Kein Debug mit Klartext, kein Display, kein Clone. | `src/secret.rs` |
| `Seq` | `(tuple/unit)` | Streng monoton steigende Sequenznummer (SPEC 8.7, I-12, TY-08). Kein Sentinelwert und kein u64::MAX-Pfad; `Option<Seq>` modelliert „kein Snapshot“. | `src/ids.rs` |
| `SeqBitTombstone` | `(tuple/unit)` | Default reference implementation of [`TombstoneSemanticsCheck`]. Uses bit 63 (`TOMBSTONE_BIT = 1 << 63`) in `seq: u64` or bit 0 in `flags: u8` (`flag… | `src/tombstone.rs` |
| `SignalFusionStrategies` | `{ /// Strategy used for vector search signal fusion. pub vector: FusionStrategy, /// Strategy used for full-text search signal fusion. pub text: FusionStrategy,… }` | Strategy configuration mapping per retrieval signal (vector, text, graph). | `src/types/saos.rs` |
| `StepId` | `(tuple/unit)` | — | `src/error.rs` |
| `StoreId` | `(tuple/unit)` | 16-Byte Speicherinstanz-Kennung (SPEC F.1). | `src/ids.rs` |
| `TagPreimage` | `(tuple/unit)` | Längenpräfigierte Vorform zur Tag-Bildung (SPEC 9.3, TY-07). Verhindert Kollisionen durch unterschiedliche Feldaufteilungen: `label ‖ (u32 len ‖ feld… | `src/tags.rs` |
| `TenantContext` | `{ tenant: crate::ids::TenantRef, }` | Mandanten-Kontext für alle mandantenspezifischen Aufrufe (SPEC D-07, VETO-F10). | `src/tenant_scope.rs` |
| `TenantId` | `(tuple/unit)` | LAYER-BEGRÜNDUNG: TenantId ist Abhängigkeit für KV-Cache-Isolation (Layer 4). In Layer 1+ definiert → zyklische Crate-Abhängigkeit unvermeidbar. INVA… | `src/types/domain/ids.rs` |
| `TenantScoped` | `{ tenant_id: TenantId, value: T, }` | Ein Wrapper-Typ, der einen Wert untrennbar an eine `TenantId` bindet. Macht es auf Typ-Ebene unmöglich, einen Wert ohne explizite Tenant-Zuordnung au… | `src/tenant_scope.rs` |
| `Timestamp` | `(tuple/unit)` | Mikrosekunden-Zeitstempel in UTC (SPEC 7.4). | `src/ids.rs` |
| `TlHfdParams` | `{ /// Regularization parameter $\sigma > 0$ controlling diffusion scale (default: 0.1). #[serde(default = "default_tl_hfd_sigma")] pub sigma: f32, /// Seed injec… }` | Configuration parameters for Thresholded Local Hyper-Flow Diffusion (TL-HFD, Spec §21.1). # Ring Architecture Defined in `acta-types` (Ring 0). # Zwe… | `src/types/domain/misc.rs` |
| `TokenBudget` | `{ /// Maximum total token limit. pub limit: usize, /// Strategy for calculating usable context window. pub strategy: BudgetStrategy, /// Reserved tokens for syst… }` | Token budget configuration for LLM context management. | `src/types/budget.rs` |
| `TxId` | `(tuple/unit)` | Transaction identifier. | `src/types/domain/ids.rs` |
| `Unarmed` | `(tuple/unit)` | Typzustand: Scanner ist noch nicht scharfgeschaltet. | `src/erasure.rs` |
| `WorkflowState` | `{ /// Associated transaction. pub tx: TxId, /// Agent memory graph state footprint (BLAKE3 hash digest). #[serde(with = "hex")] pub graph_hash: [u8; 32], }` | Defines a frozen workflow state acting as a savepoint. | `src/types/domain/misc.rs` |

### Öffentliche Datentypen — Enums (37)

| Enum | Varianten | Beschreibung | Datei |
|---|---|---|---|
| `ActaError` | `// ═══ Core & Logic ═══ /// Internal engine logic error. #[error("Internal error: {0}")] Internal(String), /// Invalid user or API argument input. #[error("Invalid input: {0}")] InvalidInput(String),…` | Unified error type for all `Acta` operations across the entire workspace. # Non-Exhaustive Variant Guarantee This enum is marked `#[non_exhaustive]`… | `src/error.rs` |
| `AutoExtractionMode` | `/// Automatic entity extraction is enabled. Enabled, /// Automatic entity extraction is disabled. Disabled,` | Mode for automatic OpenIE entity extraction during document ingestion. | `src/types/domain/misc.rs` |
| `BudgetStrategy` | `/// Konservativ: 80% des Limits als sicherer Puffer (Standard) Conservative, /// Aggressiv: 95% des Limits nutzen Aggressive, /// Exakt: Angabe des verfügbaren Fensters direkt Exact(usize),` | Strategie für Token-Budget-Management. | `src/types/budget.rs` |
| `CacheDirective` | `/// Segment is explicitly pinned in memory until TTL expiration or explicit release. Pin { /// Optional maximum duration for which the segment remains pinned. `None` indicates indefinite pinning. ttl…` | Declarative agent directive controlling segment caching, pinning, and eviction lifecycle. | `src/error.rs` |
| `DecayFunction` | `/// No recency decay (factor remains 1.0). #[default] None, /// Exponential decay measured in `TxId` distance (not wall-clock time). Exponential { /// Half-life in transaction count. half_life_tx: u6…` | Recency-decay mathematical function for episodic relevance. | `src/types/importance.rs` |
| `DistanceMetric` | `/// Cosine distance (`1.0 - cos(angle)`). #[default] Cosine, /// Euclidean (L2) distance. Euclidean, /// Dot product distance (negated inner product for f32). DotProduct,` | Distance metric for vector comparison. | `src/types/domain/document.rs` |
| `DocIdWidth` | `/// 64-bit DocId format (8-byte BLAKE3 truncation). Bit64 = 64, /// 128-bit DocId format (16-byte BLAKE3 truncation, ADR-082). Bit128 = 128,` | Indicates the DocId bit-width expected or produced by a schema version. Gibt die von einer Schemaversion erwartete oder erzeugte Bitbreite einer `Doc… | `src/schema.rs` |
| `EraseMode` | `/// Vollständige physische Löschung und Schlüsselvernichtung. Erase, /// Einschränkung der Verarbeitung (Art. 18 DSGVO). Restrict,` | Löschmodus (SPEC 7.3). | `src/erasure.rs` |
| `EraseTarget` | `/// Einzelnes Dokument. Document { /// Mandanten-Referenz. tenant: TenantRef, /// Collection-Referenz. collection: CollectionRef, /// Dokument-Referenz (inklusive Subject). doc: DocRef, }, /// Betrof…` | Löschziel (SPEC 7.3). | `src/erasure.rs` |
| `ErasureStatus` | `/// Auftrag empfangen und validiert. Requested, /// Im Gate persistiert, Lesepfade gesperrt. Gated, /// Schlüssel in Keyslots dauerhaft vernichtet. KeyDestroyed, /// Physisches Umschreiben und Berein…` | Status der Lösch-Saga (SPEC 7.4, 11.2, D-124 Abs. 5). | `src/erasure.rs` |
| `Family` | `/// Chunks und Metadaten (0x01). Doc = 0x01, /// Chunk-Zuordnung und Vektor-Slot-Nummern (0x02). DocMap = 0x02, /// Manifest, Versionen, Identität (0x03). Manifest = 0x03, /// 0x04 reserviert (Locato…` | Speicherfamilie innerhalb einer Region (SPEC 9.2). | `src/key.rs` |
| `FileKind` | `/// 1: WAL-Segment. Wal = 1, /// 2: SSTable-Datendatei. SSTable = 2, /// 3: Manifest-Log. Manifest = 3, /// 4: Textsegment. TextSegment = 4, /// 5: Vektor-Snapshot. VectorSnapshot = 5, /// 6: Journal…` | Dateitypen laut SPEC Anhang F.1 (Werte 1..=10, TY-15). | `src/file.rs` |
| `FilterExpr` | `/// Exact match: field == value Eq { /// Target metadata field name. field: String, /// Expected field value. value: serde_json::Value, }, /// Not equal: field != value Ne { /// Target metadata field…` | Metadata filter expressions for pre/post filtering. | `src/filter.rs` |
| `FusionStrategy` | `/// Reciprocal Rank Fusion (default strategy per ADR-003). /// Robust against score scale mismatch and partial signal degradation. #[default] Rrf, /// Score-normalized fusion (CombSUM / Z-Score). ///…` | Strategy used for search result signal fusion. | `src/types/saos.rs` |
| `GateVariant` | `/// Klasse P: Physisch gelöscht und gesperrt. Erased, /// Einschränkung der Verarbeitung aktiv. Restricted,` | Gate-Zustandsvariante (SPEC 11.3). | `src/erasure.rs` |
| `GraphTraversalStrategy` | `/// Multi-hop BFS traversal with hop score decay (default max_hops = 3). Hops { /// Maximum traversal hop depth. max_hops: usize, }, /// Personalized PageRank power iteration starting from seed nodes…` | Strategy used for graph retrieval in hybrid search queries. | `src/types/saos.rs` |
| `HnswDeletionError` | `/// Target node with doc_id not found in graph. #[error("Node for doc_id {0:?} not found in HNSW index")] NodeNotFound(DocId), /// Graph repair resulted in a disconnected graph component. #[error("Gr…` | Errors that can occur during HNSW graph deletion and repair operations. | `src/error.rs` |
| `KeyCoverage` | `/// Vollständige kryptografische Unlesbarkeit aller Records/Payloads. Full, /// Struktur sichtbar, aber Verknüpfung unlesbar gemacht. Unlinked, /// Keine Schlüsselvernichtung auf dieser Schichtebene.…` | Reichweite der Schlüsselabdeckung einer Schicht (SPEC 8.2, D-69). | `src/erasure.rs` |
| `LinkRelation` | `/// Target memory provides more detail or context. Elaborates, /// Target memory contradicts this memory. Contradicts, /// Target memory supersedes this memory, replacing its context. Supersedes, ///…` | Relationship types between Zettelkasten memory chunks (A-MEM). | `src/types/domain/misc.rs` |
| `ManifestSchemaVersion` | `/// Schema Version 1 (v0.x legacy 64-bit DocId layout). #[default] V1 = 1, /// Schema Version 2 (ADR-082 128-bit DocId layout). V2 = 2,` | Schema-Versionskennung für SSTable- und WAL-Manifeste. | `src/schema.rs` |
| `MatcherItem` | `/// Bereich fester Länge (65 Bytes). Range(KeyRange), /// Exakter Schlüssel (78 Bytes). Exact(KeyBuf),` | Element eines Matchers: Entweder ein KeyRange oder ein exakter KeyBuf. | `src/key.rs` |
| `MeasureClass` | `/// MK1: Logische/funktionale Prüfung. Mk1, /// MK2: Speicherebenen-Rohdatenprüfung. Mk2, /// MK3: Physische Medieninspektion. Mk3,` | Messklasse für unabhängige Löschkontrollen (SPEC 12.1). | `src/erasure.rs` |
| `MemoryType` | `/// Episodisches Gedächtnis: Erlebnisse, Unterhaltungen, zeitlich verankerte Ereignisse. /// Retrieval: Zeitliche Nähe und Relevanz zur aktuellen Session. /// Decay: Hohe Recency-Decay-Rate (Informat…` | Klassifiziert den kognitiven Gedächtnistyp einer gespeicherten Einheit. # Persistence Wird als Teil der Dokument-Metadaten serialisiert (JSON-Feld "m… | `src/types/domain/misc.rs` |
| `OnSignalFailure` | `/// Abort hybrid search and return an error on signal failure (Default per Spec Teil 6.5 P1). #[default] Fail, /// Degrade search gracefully using remaining active signals and report failed signals i…` | Strategy for handling actual signal failures during hybrid search orchestration (Spec Teil 6.5). **Semantik**: - `Fail` (Default): If an individual s… | `src/types/saos.rs` |
| `PendingReason` | `/// I/O-Fehler bei der Schichtbearbeitung. Io, /// SLA-Zeitfenster überschritten. SlaExceeded, /// Eine Schicht ist fehlgeschlagen. LayerFailed(LayerId), /// Nachweis/Zeuge fehlt noch. WitnessMissing…` | Begründung für Verzögerungen bei der Löschabwicklung (SPEC 11.2). | `src/erasure.rs` |
| `Phase` | `/// Intent-Registrierung. Intent, /// Gate-Setzung. Gate, /// Schlüsselvernichtung. KeyDestroy, /// Physisches Bereinigen. Purge, /// Unabhängige Nachweismessung. Verify, /// Dokument-Ersetzung (D-11…` | Phase eines Löschablaufs (SPEC §7.4, D-124 Abs. 5). | `src/erasure.rs` |
| `PprAlgorithm` | `/// Auto heuristic dispatch based on seed count (ForwardPush for <= 100 seeds, DensePowerIteration otherwise). #[default] Auto, /// Dense power-iteration algorithm (matrix-vector multiplication over…` | Selection of algorithm strategy for Personalized PageRank (PPR). | `src/types/domain/misc.rs` |
| `RefusalReason` | `/// Gesetzliche Aufbewahrungspflicht steht der Löschung entgegen. RetentionObligation, /// Unzulässige Parameterkombination. InvalidParameters,` | Begründung für die Ablehnung eines Löschauftrags (SPEC 7.3, 7.4). | `src/erasure.rs` |
| `Region` | `/// Dokumentfamilien unter Cluster (0x01). Data = 0x01, /// 0x02 ist reserviert (frühere Region LOC entfällt laut D-70). ReservedLoc = 0x02, /// Collection-Katalog und Zähler (0x03). Meta = 0x03,` | Region im Schlüsselraum (SPEC 9.1). | `src/key.rs` |
| `ResidualRisk` | `/// Flash-Remanenz auf FTL-Ebene. FlashRemanence, /// CoW-Dateisysteme oder Dateisystemjournale. FsJournalOrCow, /// Reste verpackter Schlüsselkopien. WrappedKeyRemanence, /// Reste in freigegebenen…` | Abschließender Katalog von 15 Restrisiken (SPEC Anhang D). | `src/erasure.rs` |
| `RetentionCode` | `/// Es besteht keine Aufbewahrungspflicht. NoObligation, /// Aufbewahrungspflicht bestand und ist abgelaufen. ObligationExpired, /// Aufbewahrungspflicht besteht aktiv. ObligationActive, /// Rechtsan…` | Abschließender Aufbewahrungs-Code (SPEC 7.3, E-L12, TY-13). | `src/erasure.rs` |
| `RetrievalStrategy` | `/// Vektorbasiertes semantisches Signal. Vector, /// Textbasiertes sparse/BM25 Signal. Text, /// Graphbasiertes Beziehungs-Signal. Graph, /// Hybrides Verbund-Signal. Hybrid, /// Nutzt LeanRAG-aggreg…` | Semantische Retrieval-Strategie für Signal-Fusion (§8/§9). Semantische Retrieval-Strategie für Signal-Fusion (§8/§9). | `src/retrieval_strategy.rs` |
| `RiskCondition` | `/// Bedingung trifft immer zu. Always, /// Trifft zu bei CoW-Dateisystemen oder Journaling. CowOrJournaling, /// Trifft zu bis zum nächsten Kompaktions- und Purge-Durchlauf. UntilPurgeComplete,` | Risikobedingung zur Evaluierung von Restrisiken (SPEC Anhang D, AR-4). | `src/erasure.rs` |
| `ScanMethod` | `/// MK1: Logische Scanmethode (z. B. Iteration über Indexeinträge). Mk1, /// MK2: Physische Scanmethode auf Block-/Dateiebene. Mk2, /// MK3: Rohmedien-Inspektion. Mk3,` | Unabhängige Messmethode für Schichtprüfungen (SPEC 3.1, 12.1). | `src/erasure.rs` |
| `Signal` | `/// Dense vector embedding search signal. Vector, /// Lexical full-text BM25 search signal. Text, /// Graph traversal / PageRank signal. Graph,` | Individual retrieval signal (vector, text, graph) in multi-signal hybrid search. # Abgrenzung Dieser Typ `acta_types::Signal` beschreibt das ungewich… | `src/types/saos.rs` |
| `TenantScopeViolation` | `/// Raised when unpacking a `TenantScoped<T>` value with an expected `TenantId` that differs /// from the value's actual bound `TenantId`. #[error("tenant scope mismatch: expected {expected:?}, got {…` | Errors raised when attempting an illegal or mismatched tenant scope unpacking operation. | `src/tenant_scope.rs` |
| `Timing` | `/// Sofortige Löschung im Vordergrund. Now, /// Gebündelte Löschung im Batch-Fenster. Batched,` | Zeitliches Verhalten der Löschung (SPEC 7.3). | `src/erasure.rs` |

### Typ-Aliase (3)

| Alias | Definition | Beschreibung |
|---|---|---|
| `QueryOptions` | `type QueryOptions= HybridQuery` | Alias for [`HybridQuery`] representing query options for hybrid retrieval operations (Spec Teil 6.5). |
| `QueryOptionsBuilder` | `type QueryOptionsBuilder= HybridQueryBuilder` | Alias for [`HybridQueryBuilder`] representing a query options builder for hybrid retrieval operations (Spec Teil 6.5). |
| `Result` | `type Result<T> = std::result::Result<T, ActaError>` | Convenience alias for `Result<T, ActaError>`. |

### Konstanten (49)

| Konstante | Typ | Beschreibung |
|---|---|---|
| `CURRENT` | `Self` | Aktuelle Standard-Formatversion. |
| `DEFAULT` | `Self` | Der implizite Default-Mandant für alle bestehenden Single-Tenant-Deployments. |
| `EXPIRY_METADATA_KEY` | `&str` | Reserved metadata key for sequence-based document TTL expiration. |
| `INTERNAL_BASE` | `u64` | Lower bound of the internal system transaction ID range. Exact numeric value: `u64::MAX - 1_000_000` (`18_446_744_073_708_551_615`). Dieser Grenzwert… |
| `INVALID` | `Self` | Invalid tenant identifier sentinel value (`0`). |
| `INVALID` | `Self` | Invalid collection identifier sentinel value (`0`). |
| `INVALID` | `Self` | Invalid or uninitialized transaction identifier sentinel value (`0`). `TxId(0)` is reserved as an uninitialized sentinel or null identifier across th… |
| `KEY_BOUND_LEN` | `usize` | Länge des Präfix-Bereichsbegrenzers (65 Bytes). |
| `KEY_LEN` | `usize` | Feste Gesamtlänge des Speicherschlüssels (78 Bytes, TY-03). |
| `LABEL_COLL_TAG` | `&[u8]` | Label für CollTag-Erzeugung. |
| `LABEL_COMMIT` | `&[u8]` | Label für Epochen-Commitment. |
| `LABEL_DEK` | `&[u8]` | Label für Dokument-DEK-Ableitung. |
| `LABEL_DEK_SALT` | `&[u8]` | Label für Dokument-DEK-Salt-Kombination. |
| `LABEL_DOC_TAG` | `&[u8]` | Label für DocTag-Erzeugung. |
| `LABEL_ENC` | `&[u8]` | Label für Verschlüsselungsschlüssel (K_enc). |
| `LABEL_FILE` | `&[u8]` | Label für Steuerdatei-Schlüssel (K_file). |
| `LABEL_FKMASK` | `&[u8]` | Label für Maskierung von Dateischlüsseln (F-Slots). |
| `LABEL_HEADER_MAC` | `&[u8]` | Label für Header-MAC. |
| `LABEL_IDX` | `&[u8]` | Label für Index-Tag-Schlüssel (K_idx). |
| `LABEL_KEYSLOT_AAD` | `&[u8]` | Label für Keyslot-AAD-Bindung. |
| `LABEL_LEDGER` | `&[u8]` | Label für Ledger-Authentisierung (K_ledger). |
| `LABEL_NAME_TAG` | `&[u8]` | Label für NameTag-Erzeugung. |
| `LABEL_RECORD_AAD` | `&[u8]` | Label für Record-AAD-Bindung. |
| `LABEL_SIGN` | `&[u8]` | Label für Signatur-Schlüsselableitung (K_sign). |
| `LABEL_SUBJECT_TAG` | `&[u8]` | Label für SubjectTag-Erzeugung. |
| `LABEL_TENANT_TAG` | `&[u8]` | Label für TenantTag-Erzeugung. |
| `LABEL_TERM_TAG` | `&[u8]` | Label für TermTag-Erzeugung. |
| `LABEL_TEXT_BINDING_AAD` | `&[u8]` | Label für Textbindung-AAD. |
| `LABEL_UNIT` | `&[u8]` | Label für Einheiten-Schlüssel (K_unit). |
| `LABEL_VAULT_MAC` | `&[u8]` | Label für Vault-Slot-MAC. |
| `LABEL_WAL` | `&[u8]` | Label für WAL-HMAC-Kette (K_wal). |
| `MAX` | `Self` | Maximum possible `DocId` value (`u64::MAX`). |
| `MAX` | `Self` | Maximum possible `DocId` value (`u128::MAX`). |
| `MAX_COLLECTION_SEQUENCE` | `u64` | Upper bound of the Collection-sequenced transaction ID range (`10^12`). Transaction allocation sequences managed by `Collection::allocate_tx()` opera… |
| `MAX_DIMENSION` | `usize` | Maximale Dimension für Einbettungsvektoren (4096). |
| `MAX_DOCS_PER_STORE` | `u64` | Maximale Anzahl an Dokumenten je Store-Instanz (50.000.000). |
| `MAX_DOC_BYTES_DEFAULT` | `usize` | Standard-Obergrenze für Dokumentengröße in Bytes (4 MiB). |
| `MAX_DOC_BYTES_HARD` | `usize` | Harte maximale Dokumentengröße in Bytes (64 MiB). |
| `MAX_K` | `usize` | Maximaler Top-k-Wert für Retrieval (1000). |
| `MAX_QUERY_TERMS` | `usize` | Maximale Anzahl von Termen in einer Suchanfrage (256). |
| `MAX_REF_LEN` | `usize` | Maximale Bytelänge für Referenz-Strings / Byte-Identifier. |
| `MAX_SEARCH_K` | `usize` | Maximum number of search results that any hybrid/vector/text search may return. Callers enforce this limit before forwarding k to HNSW/BM25. Duplicat… |
| `MAX_SNAPSHOTS_PER_TENANT` | `usize` | Maximale Anzahl aktiver MVCC-Snapshots je Mandant (1024). |
| `MIN` | `Self` | Minimum possible `DocId` value (`0`). Note: Callers MUST propagate `from_key()` errors rather than silently producing `DocId(0)`. |
| `MIN` | `Self` | Minimum possible `DocId` value (`0`). |
| `SYSTEM` | `Self` | SYSTEM tenant identifier (`0`). |
| `TOMBSTONE_BIT` | `u64` | Bit mask for identifying tombstones in sequence numbers. |
| `V1` | `Self` | Standard-Formatversion v1 (SPEC F.1). |
| `V1` | `Self` | Standard-Algorithmen-IDs für v1 (SPEC F.1). |

### Öffentliche Funktionen (162)

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
| `pub fn capability_unsupported( capability: impl Into<String>, reason: impl Into<String>, ) -> Self` | Creates a `CapabilityUnsupported` error. | `src/error.rs` |
| `pub fn checksum_mismatch(path: impl Into<String>, block_id: u64) -> Self` | Creates a `ChecksumMismatch` error. | `src/error.rs` |
| `pub fn cross_device_link( source_path: impl Into<String>, target_path: impl Into<String>, ) -> Self` | Creates a `CrossDeviceLink` error. | `src/error.rs` |
| `pub fn durability_config(msg: impl Into<String>) -> Self` | Creates a `DurabilityConfig` error. | `src/error.rs` |
| `pub fn invalid_input(msg: impl Into<String>) -> Self` | Creates an `InvalidInput` error from any displayable value. | `src/error.rs` |
| `pub fn is_occ_conflict(&self) -> bool` | Returns `true` if this error represents an optimistic concurrency control (OCC) conflict or stale read. | `src/error.rs` |
| `pub fn kv_quantization(msg: impl Into<String>) -> Self` | Creates a `KvQuantization` error. | `src/error.rs` |
| `pub fn limit_exceeded(limit: usize, context: impl Into<String>) -> Self` | Creates a `LimitExceeded` error. | `src/error.rs` |
| `pub fn model_load(path: impl Into<String>, reason: impl Into<String>) -> Self` | Creates a `ModelLoad` error. | `src/error.rs` |
| `pub fn orphaned_vector_reference( doc_id: impl Into<String>, index_id: impl Into<String>, ) -> Self` | Creates an `OrphanedVectorReference` error. | `src/error.rs` |
| `pub fn pin_budget_exceeded(msg: impl Into<String>) -> Self` | Creates a `PinBudgetExceeded` error. | `src/error.rs` |
| `pub fn snapshot_unsupported_for_signal(strategy: impl Into<String>) -> Self` | Creates a `SnapshotUnsupportedForSignal` error. | `src/error.rs` |
| `pub fn wal_corruption(offset: u64, reason: impl Into<String>) -> Self` | Creates a `WalCorruption` error. | `src/error.rs` |
| `pub fn wal_truncation_detected(expected_hmac: [u8; 32], actual_hmac: [u8; 32]) -> Self` | Creates a `WalTruncationDetected` error. | `src/error.rs` |
| `pub fn new(kind: impl Into<String>, message: impl Into<String>) -> Self` | Creates a new `ActaErrorDto` with custom kind and message. | `src/error_dto.rs` |
| `pub fn with_details( kind: impl Into<String>, message: impl Into<String>, details: serde_json::Value, ) -> Self` | Creates a new `ActaErrorDto` with custom kind, message, and details payload. | `src/error_dto.rs` |
| `pub fn evaluate(&self, metadata: &serde_json::Value) -> bool` | Evaluates the filter expression against a JSON metadata object. | `src/filter.rs` |
| `pub fn as_bytes(&self) -> &[u8]` | Liefert die Bytes als Slice. | `src/ids.rs` |
| `pub fn into_inner(self) -> Vec<u8>` | Verbraucht den Wrapper und liefert den Vektor. | `src/ids.rs` |
| `pub fn try_new(bytes: Vec<u8>) -> Result<Self, Error>` | Erzeugt eine geprüfte Instanz. Verstoß führt zu `ConfigError` (I-11). | `src/ids.rs` |
| `pub fn bound(&self) -> KeyBound` | Liefert die 65-Byte-Bereichsgrenze dieses Schlüssels. | `src/key.rs` |
| `pub fn cluster(r: Region, t: TenantTag, c: CollTag, cl: ClusterTag) -> Self` | Bereich für ein Cluster (z. B. ein Betroffener). | `src/key.rs` |
| `pub fn collection(r: Region, t: TenantTag, c: CollTag) -> Self` | Bereich für eine Collection. | `src/key.rs` |
| `pub fn commit_seq(&self) -> u64` | Rekonstruiert die Commit-Sequenznummer ohne DEK (D-74, TY-04). | `src/key.rs` |
| `pub fn contains_bound(&self, b: &KeyBound) -> bool` | Prüft, ob ein 65-Byte-Bound innerhalb des Bereichs liegt. | `src/key.rs` |
| `pub fn contains_key(&self, k: &KeyBuf) -> bool` | Prüft, ob ein 78-Byte-KeyBuf innerhalb des Bereichs liegt. | `src/key.rs` |
| `pub fn doc(r: Region, t: TenantTag, c: CollTag, cl: ClusterTag, d: DocTag) -> Self` | Bereich für ein einzelnes Dokument. | `src/key.rs` |
| `pub fn items(&self) -> &[MatcherItem]` | Liefert die Elemente des Matchers. | `src/key.rs` |
| `pub fn matches(&self, k: &KeyBuf) -> bool` | Prüft, ob ein Schlüssel von einem der Kriterien getroffen wird. | `src/key.rs` |
| `pub fn meta( tenant: TenantTag, coll: CollTag, name: NameTag, sub: u32, commit_seq: u64, ) -> Self` | Erzeugt einen Schlüssel für die META-Region (L-8 Golden-Format). Format: cluster_tag = 0¹⁶, doc_tag = name_tag, family = Manifest (0x03). | `src/key.rs` |
| `pub fn new( region: Region, tenant: TenantTag, coll: CollTag, cluster: ClusterTag, doc: DocTag, family: Family, sub: u32, commit_seq: u64, ) -> Self` | Erzeugt einen neuen 78-Byte-Schlüssel für die DATA-Region (TY-03). | `src/key.rs` |
| `pub fn new(items: Vec<MatcherItem>) -> Self` | Erzeugt einen neuen Matcher. | `src/key.rs` |
| `pub fn normalize(mut self) -> Self` | Normalisiert den Matcher: sortiert und verschmilzt überlappende/angrenzende Bereiche (TY-05). | `src/key.rs` |
| `pub fn tenant(r: Region, t: TenantTag) -> Self` | Bereich für einen gesamten Mandanten. | `src/key.rs` |
| `pub fn new( hash: [u8; 32], model_id: impl Into<String>, quantization: impl Into<String>, ) -> Self` | Creates a new `ModelFingerprint`. # Ablauf Übernimmt Hash, Modellbezeichner und Quantisierungsstring ohne Nebenwirkungen. | `src/model_fingerprint.rs` |
| `pub fn as_u8(self) -> u8` | Returns the numeric version byte. | `src/schema.rs` |
| `pub fn bytes_len(self) -> usize` | Returns the size in bytes of a single `DocId` value under this width. | `src/schema.rs` |
| `pub fn current() -> Self` | Returns the active compile-time `DocIdWidth`. # Ablauf Prüft zur Compile-Zeit via `cfg!(feature = "docid-128")`, welche Breite gewählt ist. | `src/schema.rs` |
| `pub fn doc_id_width(self) -> DocIdWidth` | Returns the corresponding `DocIdWidth` for this schema version. | `src/schema.rs` |
| `pub fn from_u8(version_byte: u8) -> Result<Self>` | Parses a version from a numeric byte indicator. # Fehler Liefert `ActaError::Serialization`, falls das übergebene Byte keiner bekannten Ver… | `src/schema.rs` |
| `pub fn is_compatible_with_current_build(self) -> bool` | Returns `true` if this schema version is compatible with the compile-time DocId feature flags. | `src/schema.rs` |
| `pub fn expose_secret(&self) -> &[u8]` | Gewährt schreibgeschützten Einblick in die geschützten Bytes. | `src/secret.rs` |
| `pub fn is_empty(&self) -> bool` | Prüft, ob das Secret leer ist. | `src/secret.rs` |
| `pub fn len(&self) -> usize` | Liefert die Länge des geschützten Speichers. | `src/secret.rs` |
| `pub fn new(bytes: Vec<u8>) -> Self` | Erzeugt ein neues `Secret` aus einem Byte-Vektor. | `src/secret.rs` |
| `pub fn as_bytes(&self) -> &[u8]` | Liefert die serialisierten Bytes der Vorform. | `src/tags.rs` |
| `pub fn field(mut self, f: &[u8]) -> Self` | Fügt ein Feld mit vorangestellter 32-Bit-Länge (big-endian) hinzu (TY-07). | `src/tags.rs` |
| `pub fn new(label: &'static [u8]) -> Self` | Erzeugt eine neue Vorform mit einem Domain-Separation-Label. | `src/tags.rs` |
| `pub fn unassigned(d: DocTag) -> Self` | Unzugeordnetes Dokument: 0x00¹² ‖ doc_tag[0..4] (D-70, TY-06). | `src/tags.rs` |
| `pub fn into_inner_checked(self, expected: &TenantId) -> Result<T, TenantScopeViolation>` | Gibt den Wert nur zurück, wenn die aufrufende Seite die erwartete `TenantId` nennt — verhindert versehentliches Cross-Tenant-Leaking durch… | `src/tenant_scope.rs` |
| `pub fn map<U>(self, f: impl FnOnce(T) -> U) -> TenantScoped<U>` | Transforms the inner value `T` into `U` using `f` while preserving the bound `TenantId`. # Ablauf Wendet `f(self.value)` an und erzeugt ein… | `src/tenant_scope.rs` |
| `pub fn new(tenant_id: TenantId, value: T) -> Self` | Creates a new `TenantScoped` value bound to the specified `TenantId`. # Ablauf Kapselt den Wert `value` zusammen mit `tenant_id` ohne Allok… | `src/tenant_scope.rs` |
| `pub fn new(tenant: crate::ids::TenantRef) -> Self` | Erzeugt einen neuen `TenantContext`. | `src/tenant_scope.rs` |
| `pub fn tenant(&self) -> &crate::ids::TenantRef` | Liefert die Mandanten-Referenz. | `src/tenant_scope.rs` |
| `pub fn tenant_id(&self) -> &TenantId` | Returns a reference to the bound `TenantId`. | `src/tenant_scope.rs` |
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
| `pub fn as_bytes(&self) -> Vec<u8>` | Converts the entity identifier into its string byte representation. | `src/types/domain/ids.rs` |
| `pub fn from_doc_id(doc_id: DocId) -> Self` | Creates an `EntityId` directly from a `DocId`. | `src/types/domain/ids.rs` |
| `pub fn from_doc_id(doc_id: DocId) -> Self` | Creates an `EntityId` directly from a `DocId`. | `src/types/domain/ids.rs` |
| `pub fn from_key(key: &str) -> Result<Self>` | Derives a `DocId` from a string key using the first 8 bytes (64 bits) of its BLAKE3 hash. # Kollisionssicherheit & Entwurfsentscheidung (AD… | `src/types/domain/ids.rs` |
| `pub fn from_key(key: &str) -> Result<Self>` | Derives a 128-bit `DocId` from a string key using the first 16 bytes (128 bits) of its BLAKE3 hash. # ADR-082 128-Bit BLAKE3 Truncation BLA… | `src/types/domain/ids.rs` |
| `pub fn from_key(key: &str) -> Result<Self>` | Derives an `EntityId` from a string key using the first 8 bytes of its BLAKE3 hash. # Errors Returns `ActaError::InvalidInput` if `key` is… | `src/types/domain/ids.rs` |
| `pub fn from_key(key: &str) -> Result<Self>` | Derives an `EntityId` from a string key using the first 8 bytes of its BLAKE3 hash. # Errors Returns `ActaError::InvalidInput` if `key` is… | `src/types/domain/ids.rs` |
| `pub fn is_system(self) -> bool` | Returns `true` if this tenant ID is `SYSTEM` (0). | `src/types/domain/ids.rs` |
| `pub fn is_valid_origin(&self) -> bool` | Checks if the transaction ID originates from a valid range per AGT-GRAPH-001. Returns `true` if `self.0 <= MAX_COLLECTION_SEQUENCE` (Collec… | `src/types/domain/ids.rs` |
| `pub fn try_from_internal_offset(offset: u64) -> Result<Self>` | Safely constructs an internal system TxId from an offset relative to `TxId::INTERNAL_BASE`. # Domain Range Separation (ADR-028) Valid syste… | `src/types/domain/ids.rs` |
| `pub fn try_new(id: u64) -> Result<Self>` | Sicherer Konstruktor. Gibt Err wenn id == 0. | `src/types/domain/ids.rs` |
| `pub fn try_new(id: u64) -> Result<Self>` | Creates a new `CollectionId`, ensuring `id != 0`. | `src/types/domain/ids.rs` |
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
| `pub fn decay_factor(&self, created_at_tx: TxId, now_tx: TxId) -> f32` | Pure non-failing decay factor calculation. Returns a factor in `[0.0, 1.0]`. # Invarianten & Edge Cases - If `now_tx < created_at_tx` (e.g.… | `src/types/importance.rs` |
| `pub fn decay_factor_wallclock( &self, created_at_epoch_secs: u64, now_epoch_secs: u64, created_at_tx: TxId, now_tx: TxId, ) -> f32` | Wall-clock based decay factor calculation. Returns a factor in `[0.0, 1.0]` based on elapsed wall-clock seconds. For non-WallClockExponenti… | `src/types/importance.rs` |
| `pub fn effective_score(&self, now_tx: TxId) -> f32` | Computes the effective importance score: `base_score * decay_factor`. # Zero-Panic Guarantee Always returns a valid `f32` in `[0.0, 1.0]`. | `src/types/importance.rs` |
| `pub fn new(raw: f32) -> Self` | Creates a new `ImportanceScore` clamped strictly within `[0.0, 1.0]`. # Zero-Panic Guarantee NaN inputs return `ImportanceScore(0.0)`. Posi… | `src/types/importance.rs` |
| `pub fn new(base_score: ImportanceScore, decay: DecayFunction, created_at_tx: TxId) -> Self` | Creates a new `MemoryImportance` configuration. | `src/types/importance.rs` |
| `pub fn value(&self) -> f32` | Returns the raw `f32` importance score value. | `src/types/importance.rs` |
| `pub fn build(self) -> Result<HybridQuery>` | Builds the `HybridQuery` instance. # Errors Returns `Err` if query parameters or fusion weights fail validation. | `src/types/saos.rs` |
| `pub fn builder() -> HybridQueryBuilder` | Creates a new `HybridQueryBuilder`. | `src/types/saos.rs` |
| `pub fn combined_text_owned(&self) -> String` | Kombinierten Text für BM25-Indizierung und Embedding. Entspricht dem etablierten Contextual-BM25-Schema: prefix + "\n\n" + content. Allokie… | `src/types/saos.rs` |
| `pub fn combined_token_count(&self) -> usize` | Kombinierter Token-Count (Heuristik: chars/4 ≈ Tokens). Verwendet für Budget- und Token-Management. | `src/types/saos.rs` |
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


---

## `acta-ports`

| Attribut | Wert |
|---|---|
| **Ring** | 0 — Ring 0 — Foundation & Pure Numerics/Types |
| **Status** | Fertig |
| **Zweck** | dyn-kompatible Trait-Definitionen (Ports) für Storage, Indizes, Embedder und System-Uhren. |
| **Unsafe-Code** | Nein (`#![forbid(unsafe_code)]`) |
| **Interne Abhängigkeiten** | `acta-types` |
| **Externe Abhängigkeiten (Auswahl)** | `ahash`, `blake3`, `bytes`, `futures-util`, `serde`, `serde_json`, `thiserror`, `zeroize` |

### Architekturbeschreibung

Schnittstellen (Traits) zwischen den Schichten: Speicher, Index, Dateisystem, Zeit, Zufall, Löschschicht.  Ring 0 Komponente laut SPEC 5.1a, 5.2, 5.3 und 8.2. Erlaubte Abhängigkeiten laut SPEC §3.1 und §5.3: acta-types (Ring 0).  Diese Traits definieren die abstrakten Schnittstellen (Ports) zwischen den Schichten zur Realisierung einer hexagonalen Architektur.

### Modul- und Re-Export-Oberfläche (lib.rs)

```rust
pub mod barrier;
pub mod clock;
pub mod erasure;
pub mod gate;
pub mod id_gen;
pub mod keys;
pub mod ledger;
pub mod memory;
pub mod metrics;
pub mod observer;
pub mod pre_gate;
pub mod report_signer;
pub mod rng;
pub mod storage;
pub mod text_index;
pub mod vector_index;
pub mod vfs;
pub use acta_types::{ ActaError, DocId, ResourceBudget, ResourceTracker, Result, ScoredDocument, TxId, };
pub use barrier::{ BarrierExclusive, BarrierShared, CommitBarrier, CompactionPermit, PurgeBarrier, PurgePermit, };
pub use clock::{Clock, SystemClock};
pub use erasure::{ Armed, EraseTarget, ErasureLayer, KeyCoverage, LayerId, LayerScope, LayerWitness, PurgeCx, PurgeOutcome, Scanner, };
pub use gate::{GateDecision, KeyBound, KeyBuf, KeyRange, RangeGate, ReadGate, Seq};
pub use id_gen::{IdGen, SequentialIdGen};
pub use keys::{Envelope, FileId, FileKey, FileKeyring, FileKind, RecordSealer, SealCtx};
pub use ledger::{LedgerEntry, LedgerSink, RangeSet, Timestamp};
pub use memory::{ArenaGuard, MemoryLockGuard};
pub use metrics::{ HistogramSnapshot, InMemoryMetricsSink, MetricKey, MetricName, MetricSnapshot, Metrics, MetricsSink, MetricsSinkAdapter, NoopMetricsSink, };
pub use observer::{ErasureEvent, ErasureObserver, ObservedPhase, ObserverError, TargetKind};
pub use pre_gate::{PreGate, PreGateIntent, PreGateOutcome};
pub use report_signer::{ReportSignature, ReportSigner, SignerError, SignerId};
pub use rng::{Rng, SeededRng};
pub use storage::{StorageEngine, StorageStats, MAX_SCAN_MERGE_ACCUMULATOR};
pub use text_index::{TextIndex, TextIndexStats};
pub use vector_index::{VectorIndex, VectorIndexStats};
pub use vfs::{VerifiedVia, Vfs, VfsFile, VfsOpenOptions};
```

### Feature-Flags

| Flag | Aktiviert |
|---|---|
| `default` | `[]` |

### Schnittstellen — Public Traits (21)

**`ArenaGuard`** — *Abstract port for locking and hardening process memory against dumping and swapping. Follows PO-11 and SPEC D-88 (2).* (`src/memory.rs`)

```rust
pub trait ArenaGuard : Send + Sync {
/// Locks the specified memory slice into physical RAM (`mlock`), preventing swapping to disk.
    ///
    /// # Fehler
    /// Liefert [`ActaError`](crate::ActaError) falls das Sperren im RAM fehlschlägt oder durch OS-Limits verwehrt wird.
    ///
    /// # Crash
    /// Reiner Speicherschutz im RAM gegen Swapping; keine Festplattenmutation.
    fn lock_memory(&self, ptr: *const u8, len: usize) -> Result<MemoryLockGuard>;

    /// Excludes the specified memory slice from OS core dumps (`MADV_DONTDUMP`).
    ///
    /// # Fehler
    /// Liefert [`ActaError`](crate::ActaError) falls madvise fehlschlägt.
    ///
    /// # Crash
    /// Reiner Speicherschutz im RAM; keine Festplattenmutation.
    fn protect_against_dumping(&self, ptr: *const u8, len: usize) -> Result<()>;
}
```

**`Clock`** — *Port trait for time operations, enabling deterministic testing by decoupling system wall-clock and monotonic time sources.* (`src/clock.rs`)

```rust
pub trait Clock : Send + Sync + 'static {
/// Returns the current wall-clock time as nanoseconds since the Unix epoch.
    ///
    /// Returns `0` if system time is before the epoch instead of panicking.
    ///
    /// # Fehler
    /// Keine; Abfrage der Systemzeit ist unfehlbar.
    ///
    /// # Crash
    /// Reiner Zeitzugriff ohne Seiteneffekte.
    fn now_unix_nanos(&self) -> u64;

    /// Returns nanoseconds elapsed since a monotonic baseline reference point.
    ///
    /// # Fehler
    /// Keine; monotoner Takt ist unfehlbar.
    ///
    /// # Crash
    /// Monoton wachsender Zeitgeber; keine Zustandsänderung.
    fn monotonic_nanos(&self) -> u64;
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

**`IdGen`** — *Port trait for unique ID generation.* (`src/id_gen.rs`)

```rust
pub trait IdGen : Send + Sync + 'static {
/// Generates and returns the next unique 64-bit unsigned identifier.
    /// # Fehler
    /// Keine; atomare Inkrementierung ist unfehlbar.
    ///
    /// # Crash
    /// Monotoner Sequenzgenerator im RAM.
    fn next_id(&self) -> u64;
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

**`Metrics`** — *Static, privacy-preserving metrics port (SPEC §14.5, PO-09, D-64). Follows PO-09: Accepts exclusively static [`MetricName`] variants and forbids dynamic ID labels.* (`src/metrics.rs`)

```rust
pub trait Metrics : Send + Sync + 'static {
/// Records a counter metric increment.
    ///
    /// # Fehler
    /// Keine; unfehlbare Zähleraktualisierung im RAM.
    ///
    /// # Crash
    /// Reiner In-Memory-Metrikzustand; keine I/O- oder Festplattenmutation.
    fn record_counter(&self, metric: MetricName, value: u64);

    /// Records a gauge metric absolute value.
    ///
    /// # Fehler
    /// Keine; unfehlbare Pegelstandsmessung im RAM.
    ///
    /// # Crash
    /// Reiner In-Memory-Metrikzustand; keine I/O- oder Festplattenmutation.
    fn record_gauge(&self, metric: MetricName, value: f64);

    /// Records a histogram observation value in seconds or units.
    ///
    /// # Fehler
    /// Keine; unfehlbare Beobachtungsregistrierung im RAM.
    ///
    /// # Crash
    /// Reiner In-Memory-Metrikzustand; keine I/O- oder Festplattenmutation.
    fn record_histogram(&self, metric: MetricName, value: f64);
}
```

**`MetricsSink`** — *Metrics sink port trait for recording system counters, gauges, and histograms. Implemented by observability backends to collect operational metrics from domain components without creating direct dependencies on specific metrics libraries.* (`src/metrics.rs`)

```rust
pub trait MetricsSink : Send + Sync + 'static {
/// Records a counter metric increment.
    /// # Fehler
    /// Keine; Aufzeichnung erfolgt non-blocking.
    ///
    /// # Crash
    /// Flüchtige Metrikerfassung im RAM.
    fn record_counter(&self, name: &str, value: u64, labels: &[(&str, &str)]);

    /// Records a gauge metric absolute value.
    /// # Fehler
    /// Keine; atomare Aktualisierung.
    ///
    /// # Crash
    /// Flüchtige Metrikerfassung im RAM.
    fn record_gauge(&self, name: &str, value: f64, labels: &[(&str, &str)]);

    /// Records a histogram observation value.
    /// # Fehler
    /// Keine; Histogramm-Update.
    ///
    /// # Crash
    /// Flüchtige Metrikerfassung im RAM.
    fn record_histogram(&self, name: &str, value: f64, labels: &[(&str, &str)]);
}
```

**`PreGate`** — *Abstract port for coordinating atomic pre-gate state mutations (SPEC §8.2, D-115, D-117). Implemented by the storage and collection engine to execute atomic pre-commit operations before physical gates take effect.* (`src/pre_gate.rs`)

```rust
pub trait PreGate : Send + Sync {
/// Executes the pre-gate action, ensuring the new document or version is persisted.
    ///
    /// # Fehler
    /// Liefert einen Fehler-String bei Commit- oder I/O-Fehlern. Bei Scheitern bleibt
    /// das bestehende Ziel unberührt (I-25, Fail-Closed).
    ///
    /// # Crash
    /// Crash-konsistent; Recovery nimmt unterbrochene Operationen über das Journal wieder auf.
    fn run<'a>(
        &'a self,
        intent: &'a PreGateIntent,
    ) -> BoxFuture<'a, std::result::Result<PreGateOutcome, String>>;
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
    /// Synchron, liefert Option<Seq>.
    ///
    /// # Crash
    /// Reiner RAM-Lookup; keine Mutation.
    fn gated_seq(&self, k: &KeyBuf) -> Option<Seq>;

    /// Returns the monotonic gate epoch counter (SPEC §11.3, D-120, I-21).
    ///
    /// Incremented before key destruction to allow linearized read verification.
    /// # Fehler
    /// Keine; unfehlbare Epochenabfrage.
    ///
    /// # Crash
    /// Reiner flüchtiger RAM-Zähler; keine Festplattenmutation.
    fn gate_epoch(&self) -> u64 {
        0
    }

    /// Evaluates whether a 16-byte `KeyBuf` is covered by an active gate (Erased or Restricted) (SPEC I-2).
    ///
    /// Used by replay filters and commit leader phase.
    ///
    /// # Fehler
    /// Synchron, unfehlbar; liefert bool.
    ///
    /// # Crash
    /// Reiner RAM-Lookup in O(log n); keine Seiteneffekte.
    fn covers(&self, k: &KeyBuf) -> bool {
        match self.check(k) {
            GateDecision::Erased | GateDecision::Restricted => true,
            GateDecision::Open => false,
        }
    }

    /// Evaluates point gate status of a canonical 78-byte raw key (SPEC 9.1, D-123).
    ///
    /// # Fehler
    /// Synchron, keine I/O; liefert GateDecision.
    ///
    /// # Crash
    /// Reiner RAM-Lookup; keine Seiteneffekte.
    fn check_raw(&self, _raw: &[u8; 78]) -> GateDecision {
        GateDecision::Open
    }

    /// Evaluates whether a canonical 78-byte raw key is covered by an active gate (SPEC I-2, I-21, D-123).
    ///
    /// # Fehler
    /// Synchron, unfehlbar; liefert bool.
    ///
    /// # Crash
    /// Reiner RAM-Lookup; keine Seiteneffekte.
    fn covers_raw(&self, raw: &[u8; 78]) -> bool {
        match self.check_raw(raw) {
            GateDecision::Erased | GateDecision::Restricted => true,
            GateDecision::Open => false,
        }
    }

    /// Retrieves the sequence number at which a 78-byte raw key was gated, if any.
    ///
    /// # Fehler
    /// Synchron, liefert Option<Seq>.
    ///
    /// # Crash
    /// Reiner RAM-Lookup; keine Mutation.
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
    /// Liefert Fehler falls Schlüssel vernichtet wurde (Löschung) oder Entschlüsselung fehlschlägt.
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
    /// Keine; unfehlbare Abfrage der Signer-ID.
    ///
    /// # Crash
    /// Reiner In-Memory-Identifikator; keine Mutation.
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

**`Rng`** — *Port trait for random number generators, enabling deterministic testing by decoupling non-deterministic entropy sources.* (`src/rng.rs`)

```rust
pub trait Rng : Send + Sync + 'static {
/// Generates the next pseudorandom 64-bit unsigned integer.
    ///
    /// # Fehler
    /// Keine; PRNG-Zustandsübergang ist unfehlbar.
    ///
    /// # Crash
    /// Flüchtig im RAM; deterministisch reproduzierbar bei SeededRng.
    fn next_u64(&self) -> u64;

    /// Fills `dest` with pseudorandom bytes generated by this RNG.
    ///
    /// # Fehler
    /// Keine; Pufferbefüllung ist unfehlbar.
    ///
    /// # Crash
    /// RAM-Mutation des Puffers.
    fn fill_bytes(&self, dest: &mut [u8]);

    /// Returns a pseudorandom `f64` value in the half-open interval `[0.0, 1.0)`
    /// with 53 bits of precision.
    ///
    /// # Fehler
    /// Keine; Gleitkommagenerierung ist unfehlbar.
    ///
    /// # Crash
    /// RAM-Berechnung.
    fn next_unit_f64(&self) -> f64;
}
```

**`StorageEngine`** — *Unified Storage Engine trait — retained for backward compatibility across higher layers. # Dyn-Kompatibilität Dieser Trait ist durch explizite `BoxFuture`-Rückgabetypen vtable-kompatibel (dyn-safe).* (`src/storage.rs`)

```rust
pub trait StorageEngine : Send + Sync + 'static {
// TOCTOU-ACCEPTED: StorageEngine provides default batching/tracking delegate methods.
    /// Retrieves a value by key.
    ///
    /// # Fehler
    /// Liefert `ActaError::Io` bei Lesefehlern oder `ActaError::NotFound` falls Schlüssel fehlt.
    ///
    /// # Crash
    /// Reiner Lesezugriff; modifiziert keinen persistenten Zustand.
    fn get<'a>(&'a self, key: &'a [u8]) -> BoxFuture<'a, Result<Option<Bytes>>>;

    /// Retrieves a value by key at a specific sequence number (MVCC).
    ///
    /// # Fehler
    /// Liefert `ActaError::Io` bei I/O-Fehlern oder `ActaError::NotFound`.
    ///
    /// # Crash
    /// Lesezugriff auf historische Snapshot-Version; keine Zustandsmutation.
    fn get_at_seq<'a>(&'a self, key: &'a [u8], seq: u64) -> BoxFuture<'a, Result<Option<Bytes>>>;

    /// Retrieves a value by key tracked within transaction `tx_id` for SSI read-set validation.
    ///
    /// # ReadSet Tracking
    /// Untracked: Default implementation delegates to [`StorageEngine::get`] and registriert KEINE Lesezugriffe im ReadSet.
    /// Concrete SSI-enabled engines (e.g. `LsmStorage`) override this method to record keys in the transaction's `ReadSet`.
    ///
    /// # Fehler
    /// Liefert `ActaError::Io` bei Lesefehlern oder `ActaError::NotFound`.
    ///
    /// # Crash
    /// Reiner Lesezugriff mit Registrierung im flüchtigen Transaktions-ReadSet; keine Plattenmutation.
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
    ///
    /// # Fehler
    /// Liefert `ActaError::Io` bei Lesefehlern oder `ActaError::NotFound`.
    ///
    /// # Crash
    /// Reiner historischer Lesezugriff mit flüchtiger ReadSet-Registrierung; keine Plattenmutation.
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
    ///
    /// # Fehler
    /// Liefert `ActaError::Io` bei Lesefehlern.
    ///
    /// # Crash
    /// Reiner Scan mit flüchtiger Präfix-ReadSet-Registrierung; keine Plattenmutation.
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
    /// Keine; unfehlbare Fähigkeitsabfrage.
    ///
    /// # Crash
    /// Reiner Statusabruf ohne Mutation.
    fn supports_ssi_tracking(&self) -> bool {
        false
    }

    /// Stores a key-value pair as part of a transaction.
    ///
    /// # Fehler
    /// Liefert `ActaError::Io` bei Schreibfehlern oder `ActaError::Conflict` bei MVCC-Konflikten.
    ///
    /// # Crash
    /// Schreibt in die Transaktions-Staging-Puffer; Persistierung erfolgt erst beim Commit.
    fn put<'a>(&'a self, tx_id: TxId, key: &'a [u8], value: &'a [u8]) -> BoxFuture<'a, Result<()>>;

    /// Atomically writes `value` under `key` within transaction ONLY IF no value exists.
    fn put_if_absent<'a>(
        &'a self,
        _tx_id: TxId,
        _key: &'a [u8],
        _value: &'a [u8],
    ) -> BoxFuture<'a, Result<bool>> {
        Box::pin(async move {
            Err(ActaError::capability_unsupported(
                "put_if_absent",
                "Atomic put_if_absent is not supported by default — concrete storage engines must implement it atomically.",
            ))
        })
    }

    /// Stores multiple key-value pairs as part of a transaction.
    ///
    /// # Fehler
    /// Liefert `ActaError::Io` bei I/O-Fehlern oder Konflikt bei MVCC.
    ///
    /// # Crash
    /// Schreibt Einträge in den Transaktionspuffer; Persistierung erfolgt erst beim Commit.
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
    ///
    /// # Fehler
    /// Liefert `ActaError::Io` bei I/O-Fehlern.
    ///
    /// # Crash
    /// Schreibt einen Tombstone in den Transaktionspuffer; persistent erst nach Commit.
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
    ///
    /// # Fehler
    /// Liefert `ActaError::Io` bei Scan- oder Löschfehlern.
    ///
    /// # Crash
    /// Schreibt Tombstones in den Transaktionspuffer; persistent erst nach Commit.
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

    /// Commits a transaction.
    ///
    /// # Fehler
    /// Liefert `ActaError::Io` bei WAL- oder fsync-Fehlern.
    ///
    /// # Crash
    /// Scheitert ein fsync während des Commits, wechselt der Speicher unverzüglich in den Zustand `Poisoned` (P14, D-55).
    fn commit<'a>(&'a self, tx_id: TxId) -> BoxFuture<'a, Result<()>>;

    /// Rolls back a transaction.
    ///
    /// # Fehler
    /// Liefert `ActaError::Io` bei internen Fehlern.
    ///
    /// # Crash
    /// Verwirft unpersistierte Transaktionsdaten im RAM; atomar und crash-sicher.
    fn rollback<'a>(&'a self, tx_id: TxId) -> BoxFuture<'a, Result<()>>;

    /// Rolls back the entire storage state to a specific transaction ID.
    ///
    /// # Fehler
    /// Liefert Fehler falls Transaktion nicht existiert oder unzulässig ist.
    ///
    /// # Crash
    /// Stellt vorherigen persistenten Zustand wieder her.
    fn rollback_to_tx<'a>(&'a self, tx_id: TxId) -> BoxFuture<'a, Result<()>>;

    /// Flushes memtable to disk.
    ///
    /// # Fehler
    /// Liefert `ActaError::Io` bei Schreib- oder Sync-Fehlern.
    ///
    /// # Crash
    /// Führt durable sync auf der neuen SSTable und dem Manifest aus; bei fsync-Fehler tritt Poisoning ein.
    fn flush<'a>(&'a self) -> BoxFuture<'a, Result<()>>;

    /// Returns storage statistics.
    ///
    /// # Fehler
    /// Liefert Fehler bei Problemen beim Auslesen von Metriken.
    ///
    /// # Crash
    /// Reine Speicherabfrage ohne Seiteneffekte.
    fn stats<'a>(&'a self) -> BoxFuture<'a, Result<StorageStats>>;

    /// Returns the last sequence number committed to storage.
    ///
    /// # Fehler
    /// Liefert Fehler falls Sequenz nicht ermittelbar ist.
    ///
    /// # Crash
    /// Deterministische In-Memory / Manifest-Abfrage.
    fn last_seq_no<'a>(&'a self) -> BoxFuture<'a, Result<u64>>;

    /// Returns the last transaction ID committed to storage.
    ///
    /// # Fehler
    /// Liefert Fehler falls TxId nicht ermittelbar ist.
    ///
    /// # Crash
    /// Deterministische Zustandsabfrage.
    fn last_tx_id<'a>(&'a self) -> BoxFuture<'a, Result<TxId>>;

    /// Pins a sequence number snapshot for durability / retention.
    ///
    /// # Fehler
    /// Liefert Fehler bei ungültiger Sequenz.
    ///
    /// # Crash
    /// Registriert Snapshot im RAM / Manifest.
    fn pin_checkpoint<'a>(&'a self, seq_no: u64) -> BoxFuture<'a, Result<()>>;

    /// Unpins a sequence number snapshot.
    ///
    /// # Fehler
    /// Liefert Fehler falls Snapshot unbekannt ist.
    ///
    /// # Crash
    /// Entfernt Snapshot-Pin; gibt Ressourcen für GC frei.
    fn unpin_checkpoint<'a>(&'a self, seq_no: u64) -> BoxFuture<'a, Result<()>>;

    /// Scans a range of keys with given prefix.
    ///
    /// # Fehler
    /// Liefert `ActaError::Io` bei Lesefehlern.
    ///
    /// # Crash
    /// Reiner Lese-Scan ohne Mutation.
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
            Err(ActaError::capability_unsupported(
                "snapshot_read_at",
                "Storage-level snapshot-isolated prefix scan (scan_prefix_at) is not supported by default — implementors must override this method to guarantee MVCC snapshot isolation.",
            ))
        })
    }

    /// Scans range between start and end bounds.
    ///
    /// # Fehler
    /// Liefert `ActaError::Io` bei Lesefehlern.
    ///
    /// # Crash
    /// Reiner Bereichs-Scan ohne Mutation.
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
            Err(ActaError::capability_unsupported(
                "scan_bounded",
                "Bounded range scan (scan_bounded) is not supported by default",
            ))
        })
    }
}
```

**`TextIndex`** — *Text-Index Trait — abstrahiert BM25/Inverted-Index-Operationen. # Dyn-Kompatibilität Verwendet native `async fn` (AFIT) für statischen Dispatch.* (`src/text_index.rs`)

```rust
pub trait TextIndex : Send + Sync + 'static {
/// Searches for documents matching the query.
    ///
    /// # Fehler
    /// Liefert `ActaError::Io` bei Lesefehlern.
    ///
    /// # Crash
    /// Reiner Lesezugriff; keine Mutation auf Festplatte.
    fn search(
        &self,
        query: &str,
        k: usize,
    ) -> impl Future<Output = Result<Vec<ScoredDocument>>> + Send;

    /// Searches for documents matching the query at a specific sequence number.
    ///
    /// # Fehler
    /// Liefert `ActaError::CapabilityUnsupported` falls nicht implementiert oder `ActaError::Io` bei Lesefehlern.
    ///
    /// # Crash
    /// Reiner historischer Snapshot-Lesezugriff; keine Mutation auf Festplatte.
    fn search_at(
        &self,
        _query: &str,
        _k: usize,
        _seq_no: u64,
    ) -> impl Future<Output = Result<Vec<ScoredDocument>>> + Send {
        async move {
            Err(ActaError::capability_unsupported(
                "snapshot_read_at",
                "Text search snapshot isolation (search_at) is not supported by default — tracked in ADR-024",
            ))
        }
    }

    /// Inserts or updates a document in the index.
    ///
    /// # Fehler
    /// Liefert `ActaError::Io` bei Allokations- oder Tokenisierungsfehlern.
    ///
    /// # Crash
    /// Schreibt in die In-Memory-Postings; Persistierung erfolgt erst beim Commit.
    fn insert(&self, tx: TxId, id: DocId, text: &str) -> impl Future<Output = Result<()>> + Send;

    /// Deletes a document from the index.
    ///
    /// # Fehler
    /// Liefert `ActaError::Io` bei Fehlern während der Löschung.
    ///
    /// # Crash
    /// Vermerkt Löschung im Transaktionspuffer; persistent erst nach Commit.
    fn delete(&self, tx: TxId, id: DocId) -> impl Future<Output = Result<()>> + Send;

    /// Commits a transaction.
    ///
    /// # Fehler
    /// Liefert `ActaError::Io` bei Schreib- oder fsync-Fehlern.
    ///
    /// # Crash
    /// Schreibt Textsegmente persistent fest; fsync-Fehler führen zu Poisoning.
    fn commit(&self, tx: TxId) -> impl Future<Output = Result<()>> + Send;

    /// Rolls back a transaction.
    ///
    /// # Fehler
    /// Liefert `ActaError::Io` bei internen Rollback-Fehlern.
    ///
    /// # Crash
    /// Verwirft unpersistierte Posting-Listen der Transaktion im RAM; crash-sicher.
    fn rollback(&self, tx: TxId) -> impl Future<Output = Result<()>> + Send;

    /// Rolls back the entire index state to a specific transaction ID.
    ///
    /// # Fehler
    /// Liefert Fehler falls Transaktion nicht existiert oder unzulässig ist.
    ///
    /// # Crash
    /// Stellt vorherigen Indexzustand wieder her.
    fn rollback_to_tx(&self, tx_id: TxId) -> impl Future<Output = Result<()>> + Send;

    /// Returns the last transaction ID processed by the index.
    ///
    /// # Fehler
    /// Liefert Fehler falls TxId nicht ermittelt werden kann.
    ///
    /// # Crash
    /// Deterministischer RAM-/Manifest-Statusabruf ohne Seiteneffekte.
    fn last_tx_id(&self) -> impl Future<Output = Result<TxId>> + Send;

    /// Returns the number of documents in the index.
    ///
    /// # Fehler
    /// Keine; unfehlbare Zählung.
    ///
    /// # Crash
    /// Reiner Statusabruf ohne Mutation.
    fn len(&self) -> impl Future<Output = usize> + Send;

    /// Returns true if the index is empty.
    ///
    /// # Fehler
    /// Keine; unfehlbare Prüfung.
    ///
    /// # Crash
    /// Reiner Statusabruf ohne Mutation.
    fn is_empty(&self) -> impl Future<Output = bool> + Send {
        async { self.len().await == 0 }
    }

    /// Returns index statistics.
    ///
    /// # Fehler
    /// Liefert Fehler falls Statistiken nicht ermittelt werden können.
    ///
    /// # Crash
    /// Reines Abfragen von Zählern im RAM.
    fn stats(&self) -> impl Future<Output = Result<TextIndexStats>> + Send;
}
```

**`VectorIndex`** — *Vector Index Trait — abstrahiert die HNSW-Vektorsuche. # Dyn-Kompatibilität Verwendet native `async fn` (AFIT) für statischen Dispatch.* (`src/vector_index.rs`)

```rust
pub trait VectorIndex : Send + Sync + 'static {
/// Inserts a vector with an associated document ID.
    ///
    /// # Fehler
    /// Liefert Fehler bei Dimensions-Mismatch oder Allokationsfehlern.
    ///
    /// # Crash
    /// Schreibt Vektor in den RAM-Graphknoten; Persistierung erfolgt im Snapshot.
    fn insert(
        &self,
        tx: TxId,
        id: DocId,
        embedding: &[f32],
    ) -> impl Future<Output = Result<()>> + Send;

    /// Returns all active (non-deleted) document IDs in the index.
    ///
    /// # Fehler
    /// Liefert Fehler bei internen Zugriffsfehlern.
    ///
    /// # Crash
    /// Reiner In-Memory-Lesezugriff; keine Plattenmutation.
    fn all_doc_ids(&self) -> impl Future<Output = Result<Vec<DocId>>> + Send {
        async { Ok(Vec::new()) }
    }

    /// Inserts multiple vectors with associated document IDs.
    ///
    /// # Fehler
    /// Liefert Fehler bei Dimensions-Mismatch oder Allokationsfehlern.
    ///
    /// # Crash
    /// Schreibt Vektoren in den RAM-Graphen; Persistierung erfolgt im Snapshot.
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

    /// Searches for the k nearest neighbors to a query vector.
    ///
    /// # Fehler
    /// Liefert Fehler bei Dimensions-Mismatch oder unzulässigen Parametern.
    ///
    /// # Crash
    /// Reiner Lese-Graph-Traversierungsschritt im RAM; keine Festplattenmutation.
    fn search(
        &self,
        query: &[f32],
        k: usize,
    ) -> impl Future<Output = Result<Vec<ScoredDocument>>> + Send;

    /// Searches for the k nearest neighbors to a query vector at a specific sequence number.
    ///
    /// # Fehler
    /// Liefert `ActaError::CapabilityUnsupported` falls nicht implementiert oder Fehler bei Dimensionsmismatch.
    ///
    /// # Crash
    /// Reiner historischer Snapshot-Lesezugriff; keine Mutation auf Festplatte.
    fn search_at(
        &self,
        _query: &[f32],
        _k: usize,
        _seq_no: u64,
    ) -> impl Future<Output = Result<Vec<ScoredDocument>>> + Send {
        async move {
            Err(ActaError::capability_unsupported(
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
    /// # Fehler
    /// Liefert `ActaError::CapabilityUnsupported` falls Filterung nicht unterstützt wird.
    ///
    /// # Crash
    /// Reiner Graph-Suchschritt; keine Mutation auf Festplatte.
    fn search_filtered(
        &self,
        query: &[f32],
        k: usize,
        filter: Option<&(dyn Fn(DocId) -> bool + Send + Sync)>,
    ) -> impl Future<Output = Result<Vec<ScoredDocument>>> + Send {
        async move {
            if filter.is_some() {
                return Err(ActaError::capability_unsupported(
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
    /// Liefert Fehler falls Markierung im Index fehlschlägt.
    ///
    /// # Crash
    /// Markiert Vektorknoten als gelöscht (Tombstone); persistiert erst nach Snapshot/Commit.
    fn delete(&self, tx: TxId, id: DocId) -> impl Future<Output = Result<()>> + Send;

    /// Commits a transaction.
    ///
    /// # Fehler
    /// Liefert `ActaError::Io` bei WAL- oder Snapshot-Fehlern.
    ///
    /// # Crash
    /// Macht eingefügte/gelöschte Vektoren im Graphen sichtbar und persistent; fsync-Fehler führen zu Poisoning.
    fn commit(&self, tx: TxId) -> impl Future<Output = Result<()>> + Send;

    /// Rolls back a transaction.
    ///
    /// # Fehler
    /// Liefert Fehler bei internen Rollback-Fehlern.
    ///
    /// # Crash
    /// Verwirft unpersistierte Vektoränderungen der Transaktion im RAM; crash-sicher.
    fn rollback(&self, tx: TxId) -> impl Future<Output = Result<()>> + Send;

    /// Rolls back the entire index state to a specific transaction ID.
    ///
    /// # Fehler
    /// Liefert Fehler falls Transaktion nicht existiert oder unzulässig ist.
    ///
    /// # Crash
    /// Stellt vorherigen Indexzustand wieder her.
    fn rollback_to_tx(&self, tx_id: TxId) -> impl Future<Output = Result<()>> + Send;

    /// Returns the last transaction ID processed by the index.
    ///
    /// # Fehler
    /// Liefert Fehler falls TxId nicht ermittelt werden kann.
    ///
    /// # Crash
    /// Deterministischer RAM-/Manifest-Statusabruf ohne Seiteneffekte.
    fn last_tx_id(&self) -> impl Future<Output = Result<TxId>> + Send;

    /// Returns the number of vectors in the index.
    ///
    /// # Fehler
    /// Keine; unfehlbare Zählung.
    ///
    /// # Crash
    /// Reiner Statusabruf ohne Mutation.
    fn len(&self) -> impl Future<Output = usize> + Send;

    /// Returns true if the index is empty.
    ///
    /// # Fehler
    /// Keine; unfehlbare Prüfung.
    ///
    /// # Crash
    /// Reiner Statusabruf ohne Mutation.
    fn is_empty(&self) -> impl Future<Output = bool> + Send {
        async { self.len().await == 0 }
    }

    /// Returns index statistics.
    ///
    /// # Fehler
    /// Liefert Fehler falls Statistiken nicht ermittelt werden können.
    ///
    /// # Crash
    /// Reines Abfragen von Zählern im RAM.
    fn stats(&self) -> impl Future<Output = Result<VectorIndexStats>> + Send;

    /// Returns true if the index requires a background rebuild (e.g. due to tombstone accumulation).
    ///
    /// # Fehler
    /// Keine; unfehlbare Schwellwertprüfung im RAM.
    ///
    /// # Crash
    /// Reiner Statusabruf ohne Mutation.
    fn is_rebuild_required(&self) -> bool {
        false
    }

    /// Triggers an asynchronous background rebuild of the index if supported.
    ///
    /// # Fehler
    /// Keine direkte Fehlerpropagation; Rebuild läuft asynchron im Hintergrund.
    ///
    /// # Crash
    /// Startet Hintergrundtask zur Reorganisation des HNSW-Graphen.
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
    /// Liefert [`ActaError::Io`](crate::error::ActaError::Io) bei I/O-Fehlern oder Leseproblemen.
    ///
    /// # Crash
    /// Reiner positioneller Lesezugriff; modifiziert keine persistenten Dateiinhalte oder Metadaten.
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
    ///
    /// # Crash
    /// Reiner Lesezugriff zur Medien- und Cache-Verifikation; keine Modifikation auf dem Speichermedium.
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
    /// Liefert [`ActaError::Io`](crate::error::ActaError::Io) oder [`ActaError::NotFound`](crate::error::ActaError::NotFound)
    /// falls das Entfernen fehlschlägt oder die Datei fehlt.
    ///
    /// # Crash
    /// Entfernt Verzeichniseintrag; Persistenz erfordert `sync_dir` des übergeordneten Verzeichnisses.
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
    /// Liefert [`ActaError::Io`](crate::error::ActaError::Io) bei Lesefehlern des Verzeichnisses.
    ///
    /// # Crash
    /// Reiner Lesezugriff auf Verzeichniseinträge; keine Seiteneffekte.
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
    /// Garantiert die Haltbarkeit der geschriebenen Datenblöcke bis zum Aufrufzeitpunkt.
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

### Öffentliche Datentypen — Structs (38)

| Struct | Felder / Form | Beschreibung | Datei |
|---|---|---|---|
| `Armed` | `(tuple/unit)` | Phantom state for an armed scanner verifying deletion residue. | `src/erasure.rs` |
| `BarrierExclusive` | `{ _token: Option<Arc<()>>, }` | RAII token guard representing an exclusive gate phase permit. | `src/barrier.rs` |
| `BarrierShared` | `{ _token: Option<Arc<()>>, }` | RAII token guard representing a shared commit phase permit. | `src/barrier.rs` |
| `CompactionPermit` | `{ _token: Option<Arc<()>>, }` | RAII token guard representing a compaction permit. | `src/barrier.rs` |
| `Envelope` | `{ payload: Vec<u8>, }` | Sealed record envelope containing encrypted ciphertext and tag. | `src/keys.rs` |
| `ErasureEvent` | `{ /// Unique identifier of the erasure operation. pub id: ErasureId, /// Kind of target affected. pub target_kind: TargetKind, /// Current phase reached. pub pha… }` | Event published to an [`ErasureObserver`] upon reaching an erasure lifecycle phase (SPEC D-118). | `src/observer.rs` |
| `FileId` | `(tuple/unit)` | Unique identifier for a durable storage file or SSTable. | `src/keys.rs` |
| `FileKey` | `{ raw: Vec<u8>, file_id: FileId, }` | Wrapper around a cryptographic file key with zeroizing on drop and masked debug display (PO-08). | `src/keys.rs` |
| `HistogramSnapshot` | `{ /// Sum of all observed values. pub sum: f64, /// Total count of observations. pub count: u64, /// Minimum observed value (None if empty). pub min: Option<f64>… }` | Snapshot of a recorded histogram metric. | `src/metrics.rs` |
| `InMemoryMetricsSink` | `{ counters: RwLock<BTreeMap<MetricKey, Arc<AtomicU64>>>, gauges: RwLock<BTreeMap<MetricKey, f64>>, histograms: RwLock<BTreeMap<MetricKey, HistogramState>>, }` | Production-ready local thread-safe implementation of [`MetricsSink`]. Stores metrics strictly in-process with no external network exports (privacy-pr… | `src/metrics.rs` |
| `KeyBuf` | `(tuple/unit)` | Fixed or bounded key buffer for storage operations (SPEC §8.1). | `src/gate.rs` |
| `KeyRange` | `{ /// Lower boundary inclusive. pub start: KeyBuf, /// Upper boundary exclusive. pub end: KeyBuf, }` | Half-open key range `[start, end)` for scan and compaction boundaries. | `src/gate.rs` |
| `LayerId` | `(tuple/unit)` | Layer identifier representing a specific subsystem participating in erasure (e.g. store, text, vector). | `src/erasure.rs` |
| `LayerScope` | `{ /// Opaque scope identifier or token. scope_id: [u8; 16], /// Segment or file identifiers affected. affected_units: Vec<u64>, }` | Scope resolved by an erasure layer prior to gating (SPEC §8.2, D-96). Records the concrete file sets, segment IDs, or key intervals that this layer m… | `src/erasure.rs` |
| `LayerWitness` | `{ /// Digest or commitment over the verified non-presence. witness_hash: [u8; 32], }` | Cryptographic or structural witness produced after verifying deletion residue. | `src/erasure.rs` |
| `LedgerEntry` | `{ /// Commitment hash over the target erasure definition. pub commitment: [u8; 32], /// Key ranges covered by this erasure. pub range_set: RangeSet, /// Sequence… }` | A tamper-evident entry in the erasure ledger (SPEC §4.2, §11.8, F.9). Contains `(commitment, range_set, gated_seq, completed_at)` along with `chain_h… | `src/ledger.rs` |
| `MemoryLockGuard` | `{ _token: Option<Arc<()>>, }` | RAII token guard protecting a specific locked memory region. Ensures memory unlock or dump-restoration is performed cleanly upon drop. | `src/memory.rs` |
| `MetricKey` | `{ /// Metric identifier (e.g. `lsm_commit_total`). pub name: String, /// Sorted label key-value pairs. pub labels: Vec<(String, String)>, }` | Key identifying a metric by name and sorted key-value labels. | `src/metrics.rs` |
| `MetricSnapshot` | `{ /// Counter metrics (accumulated values). pub counters: BTreeMap<MetricKey, u64>, /// Gauge metrics (last recorded f64 values). pub gauges: BTreeMap<MetricKey,… }` | In-memory snapshot of all metrics collected by [`InMemoryMetricsSink`]. | `src/metrics.rs` |
| `MetricsSinkAdapter` | `{ sink: S, }` | Adapter implementing [`Metrics`] over any existing [`MetricsSink`]. | `src/metrics.rs` |
| `NoopMetricsSink` | `(tuple/unit)` | A no-op implementation of [`MetricsSink`] that silently discards all metrics events. | `src/metrics.rs` |
| `PreGateOutcome` | `{ /// Monotonic commit sequence number assigned to the newly committed document/version. pub committed_seq: u64, }` | Outcome of pre-gate execution indicating persisted version sequence (SPEC D-115, D-117). | `src/pre_gate.rs` |
| `PurgeCx` | `{ /// Budget limit or deadline in milliseconds. pub deadline_ms: u64, }` | Context passed into [`ErasureLayer::purge`] providing operational bounds and budgets. | `src/erasure.rs` |
| `PurgeOutcome` | `{ /// Number of records or items successfully removed. pub purged_count: u64, /// Number of bytes freed or compacted. pub bytes_reclaimed: u64, }` | Outcome of a layer purge execution (SPEC §8.2, WP-3.1). | `src/erasure.rs` |
| `PurgePermit` | `{ _token: Option<Arc<()>>, }` | RAII token guard representing an exclusive purge permit. | `src/barrier.rs` |
| `RangeSet` | `{ ranges: Vec<(KeyBuf, KeyBuf)>, }` | Set of key ranges or tokens included in an erasure ledger entry. | `src/ledger.rs` |
| `ReportSignature` | `{ /// Signer identifier (u8: 0 = internal, 1 = operator / HSM). pub signer_id: SignerId, /// Base64url-encoded signature string without padding (RFC 4648 §5, SPE… }` | Digital signature over canonical report bytes (SPEC F.10). | `src/report_signer.rs` |
| `Scanner` | `{ _state: PhantomData<State>, }` | Residue scanner type passed to [`ErasureLayer::witness`]. Must be purely read-only (PO-05). | `src/erasure.rs` |
| `SealCtx` | `{ /// Epoch sequence or slot identifier. pub epoch: u64, }` | Sealing context containing metadata and epoch tokens for envelope creation. | `src/keys.rs` |
| `SeededRng` | `{ state: AtomicU64, }` | A fast, deterministic 64-bit pseudorandom number generator using SplitMix64 with atomic state transitions (`AtomicU64`). Lock-free and thread-safe wi… | `src/rng.rs` |
| `Seq` | `(tuple/unit)` | Monotonic sequence number for transactions and gate markers. | `src/gate.rs` |
| `SequentialIdGen` | `{ next: AtomicU64, }` | A thread-safe ID generator that produces monotonically increasing 64-bit IDs. Wraps around safely on integer overflow (`u64::MAX` -> `0`) via atomic… | `src/id_gen.rs` |
| `StorageStats` | `{ /// Number of SSTable segments. pub num_segments: usize, /// Total size of all SSTables in bytes. pub total_size_bytes: u64, /// Total size of memtables in byt… }` | Statistics for the storage engine. | `src/storage.rs` |
| `SystemClock` | `{ start_instant: Instant, }` | Standard production implementation of [`Clock`] backed by [`SystemTime`] and [`Instant`]. | `src/clock.rs` |
| `TextIndexStats` | `{ /// Total number of documents indexed. pub num_documents: usize, /// Total number of tokens across all documents. pub num_tokens: usize, /// Estimated memory u… }` | Statistics for a text index. | `src/text_index.rs` |
| `Timestamp` | `(tuple/unit)` | Discrete point in time measured in epoch milliseconds or nanoseconds. | `src/ledger.rs` |
| `VectorIndexStats` | `{ /// Number of active (non-deleted) vectors. pub num_vectors: usize, /// Estimated memory usage in bytes. pub memory_usage_bytes: usize, /// Number of HNSW laye… }` | Statistics for a vector index. | `src/vector_index.rs` |
| `VfsOpenOptions` | `{ /// Opens the file with read access. pub read: bool, /// Opens the file with write access. pub write: bool, /// Creates a new file if it does not exist. pub cr… }` | Options configuring how a file is opened via [`Vfs::open`]. | `src/vfs.rs` |

### Öffentliche Datentypen — Enums (13)

| Enum | Varianten | Beschreibung | Datei |
|---|---|---|---|
| `EraseTarget` | `/// Target a specific key or document identifier prefix. KeyRange { /// Lower key bound inclusive. start: Vec<u8>, /// Upper key bound exclusive. end: Vec<u8>, }, /// Target a discrete set of documen…` | Target of an erasure request specifying what needs to be erased. | `src/erasure.rs` |
| `FileKind` | `/// SSTable data file. SSTable, /// Write-Ahead-Log (WAL) file. Wal, /// Inverted index segment. TextSegment, /// Vector HNSW segment. VectorSegment, /// General auxiliary file. Auxiliary,` | Kind of file managed by [`FileKeyring`] (SPEC §8.2, D-85). | `src/keys.rs` |
| `GateDecision` | `/// Key or boundary is open for reading. Open, /// Key or boundary has been erased. Access must be blocked immediately. Erased, /// Key or boundary is temporarily restricted (e.g. pending gate sequen…` | Gate decision for point queries or file boundaries (SPEC §4.2, §11.3). | `src/gate.rs` |
| `KeyBound` | `/// Exact key match boundary. Exact(KeyBuf), /// Unbounded lower boundary. UnboundedLower, /// Unbounded upper boundary. UnboundedUpper,` | Boundary key used for compaction or range scanning. | `src/gate.rs` |
| `KeyCoverage` | `/// Layer maintains full key separation and slot-zeroing capability. Full, /// Layer holds keys unlinked from the main vault hierarchy. Unlinked, /// Layer maintains no direct key slots (relies entir…` | Key coverage capability of a layer (SPEC §8.2, D-69). | `src/erasure.rs` |
| `MetricName` | `/// Latency until erasure reach Gated state (`ctx_erase_gated_seconds`). EraseGatedSeconds, /// Latency of durable fsync / flush operations (`ctx_fsync_seconds`). FsyncSeconds, /// Count of open/acti…` | Static enumeration of all authorized metric names in Acta (SPEC §14.5, PO-09, D-64). In accordance with D-64 and PO-09, metric names are strictly sta… | `src/metrics.rs` |
| `ObservedPhase` | `/// Read-gate is established; all future reads will see target as erased. Gated, /// Physical purge and verification complete. Completed, /// Access restriction per Art. 18 DSGVO is activated (D-116)…` | Observed lifecycle phase of an erasure operation (SPEC D-118). | `src/observer.rs` |
| `ObserverError` | `/// Delivery failed due to transient network or backend error. #[error("transient delivery failure: {0}")] Transient(String), /// Delivery rejected permanently by the receiver. #[error("permanent del…` | Error returned by an [`ErasureObserver`] during notification delivery (SPEC D-118). | `src/observer.rs` |
| `PreGateIntent` | `/// Replacement of a multi-subject document (D-115, TargetKind 6). Replace { /// Associated erasure identifier. erasure_id: ErasureId, /// BLAKE3 tag of the new replacement document. new_doc_tag: [u8…` | Operation context or intent for pre-gate execution (SPEC D-115, D-117). | `src/pre_gate.rs` |
| `RangeGate` | `/// Range contains erased data: records may be safely discarded. ContainsErased, /// Range is restricted: records must never be discarded until release. RestrictedNeverDiscard, /// Entire range is co…` | Decision variant for range operations during compaction (SPEC §4.2, I-18). | `src/gate.rs` |
| `SignerError` | `/// Signing key unavailable or corrupted. #[error("signing key unavailable: {0}")] KeyUnavailable(String), /// Hardware security module or external signature provider error. #[error("external signer…` | Error class for report signing operations (SPEC D-130, 7.5). | `src/report_signer.rs` |
| `TargetKind` | `/// Single document erasure. Document, /// Subject-level erasure (all documents belonging to a subject). Subject, /// Collection-level erasure. Collection, /// Tenant-level erasure. Tenant, /// Docum…` | Target kind indicating which hierarchy level was targeted (SPEC D-118, P13). | `src/observer.rs` |
| `VerifiedVia` | `/// Read bypassed page cache via Direct I/O (`O_DIRECT` or `F_NOCACHE`). Direct, /// Read performed after page cache eviction hint (`posix_fadvise(POSIX_FADV_DONTNEED)`). CacheDropped, /// Read perfo…` | Method via which medium verification was performed (SPEC D-113). | `src/vfs.rs` |

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

### Öffentliche Funktionen (31)

| Signatur | Beschreibung | Datei |
|---|---|---|
| `pub fn new(token: Option<Arc<()>>) -> Self` | Creates a new `BarrierShared` token. | `src/barrier.rs` |
| `pub fn new(token: Option<Arc<()>>) -> Self` | Creates a new `BarrierExclusive` token. | `src/barrier.rs` |
| `pub fn new(token: Option<Arc<()>>) -> Self` | Creates a new `CompactionPermit` token. | `src/barrier.rs` |
| `pub fn new(token: Option<Arc<()>>) -> Self` | Creates a new `PurgePermit` token. | `src/barrier.rs` |
| `pub fn new() -> Self` | Creates a new [`SystemClock`] using the current [`Instant`] as the monotonic baseline. | `src/clock.rs` |
| `pub fn affected_units(&self) -> &[u64]` | Returns affected units count. | `src/erasure.rs` |
| `pub fn as_bytes(&self) -> &[u8` | Returns a slice of raw bytes. | `src/erasure.rs` |
| `pub fn new(scope_id: [u8; 16], affected_units: Vec<u64>) -> Self` | Creates a new `LayerScope` with specific units. | `src/erasure.rs` |
| `pub fn scope_id(&self) -> &[u8` | Returns the unique scope ID. | `src/erasure.rs` |
| `pub fn witness_hash(&self) -> &[u8` | Returns the witness digest. | `src/erasure.rs` |
| `pub fn as_bytes(&self) -> &[u8` | Returns raw bytes slice. | `src/gate.rs` |
| `pub fn contains(&self, key: &KeyBuf) -> bool` | Returns `true` if `key` falls within `[start, end)`. | `src/gate.rs` |
| `pub fn new(start: u64) -> Self` | Creates a new [`SequentialIdGen`] starting at `start`. | `src/id_gen.rs` |
| `pub fn as_bytes(&self) -> &[u8]` | Returns a reference to the raw secret bytes. | `src/keys.rs` |
| `pub fn file_id(&self) -> FileId` | Associated file ID. | `src/keys.rs` |
| `pub fn new(raw: Vec<u8>, file_id: FileId) -> Self` | Creates a new `FileKey`. | `src/keys.rs` |
| `pub fn new(payload: Vec<u8>) -> Self` | Creates a new envelope. | `src/keys.rs` |
| `pub fn payload(&self) -> &[u8]` | Returns payload bytes. | `src/keys.rs` |
| `pub fn compute_chained( commitment: [u8; 32], range_set: RangeSet, gated_seq: Seq, completed_at: Timestamp, prev_hash: [u8; 32], ledger_mac: [u8; 32], ) -> Self` | Constructs a ledger entry and computes its chained BLAKE3 hash from the prior entry hash. | `src/ledger.rs` |
| `pub fn new(ranges: Vec<(KeyBuf, KeyBuf)>) -> Self` | Creates a range set from intervals. | `src/ledger.rs` |
| `pub fn ranges(&self) -> &[(KeyBuf, KeyBuf)]` | Returns the slice of ranges. | `src/ledger.rs` |
| `pub fn verify_chain_hash(&self, prev_hash: &[u8; 32]) -> bool` | Verifies the entry's chain hash against the supplied previous block hash. | `src/ledger.rs` |
| `pub fn new(token: Option<Arc<()>>) -> Self` | Creates a new `MemoryLockGuard`. | `src/memory.rs` |
| `pub fn inner(&self) -> &S` | Returns a reference to the inner sink. | `src/metrics.rs` |
| `pub fn new(sink: S) -> Self` | Creates a new adapter wrapping `sink`. | `src/metrics.rs` |
| `pub fn new(name: impl Into<String>, labels: &[(&str, &str)]) -> Self` | Creates a new normalized [`MetricKey`]. | `src/metrics.rs` |
| `pub fn new() -> Self` | Creates a new empty [`InMemoryMetricsSink`]. | `src/metrics.rs` |
| `pub fn snapshot(&self) -> MetricSnapshot` | Captures a point-in-time [`MetricSnapshot`] of all recorded counters, gauges, and histograms. | `src/metrics.rs` |
| `pub fn new(seed: u64) -> Self` | Creates a new [`SeededRng`] initialized with the given seed. | `src/rng.rs` |
| `pub fn read_only() -> Self` | Creates options configured for read-only access. | `src/vfs.rs` |
| `pub fn write_create() -> Self` | Creates options configured for write-only access, creating the file if missing. | `src/vfs.rs` |


---

## `acta-mvcc`

| Attribut | Wert |
|---|---|
| **Ring** | 0 — Ring 0 — Foundation & Pure Numerics/Types |
| **Status** | Fertig |
| **Zweck** | SeqLog, SnapshotRegistry und transaktionaler TxBuffer (MVCC-Isolation). |
| **Unsafe-Code** | Nein (`#![forbid(unsafe_code)]`) |
| **Interne Abhängigkeiten** | `acta-ports`, `acta-types` |
| **Externe Abhängigkeiten (Auswahl)** | `ahash`, `bytes`, `parking_lot`, `serde`, `thiserror`, `tracing` |

### Architekturbeschreibung

Sequenzen, Snapshots und SSI-Validierung für Mehrversions-Nebenläufigkeit.  Ring: 2 (Kernkomponente) Erlaubte Abhängigkeiten laut SPEC §5.3: acta-types, acta-ports. Siehe SPEC §8.7 für normative Anforderungen.  ## Lock-Ordnung (SPEC §5.4, §8.7, D-11, D-127) Zur Vermeidung von Deadlocks zwischen nebenläufigen Schreib-, Commit- und Purge-Pfaden gilt die strikte Lock-Hierarchie: `Dokumentlock -> Commit-Barriere -> Purge-Barriere -> Journal`  - **Dokumentlock**: Exklusiv pro Dokument während In-Memory-Modifikationen (keine Verschachtelung). - **Commit-Barriere**: Shared (`read`) bei regulären Transaktions-Commits; Exclusive (`write`) bei Barrieren-Flush. - **Purge-Barriere**: Shared (`read`) während regulärer Lese- und Schreiboperationen; Exclusive (`write`) während Purge-/Lösch-Sagas. - **Journal**: Serialisiertes WAL/Manifest-Schreiben.  Shard-Locks werden niemals verschachtelt und keine Sperre wird über `.await` gehalten (L-7, SPEC §5.4).

### Modul- und Re-Export-Oberfläche (lib.rs)

```rust
pub mod error;
pub mod seq_log;
pub mod snapshot;
pub mod ssi;
pub mod tx_buffer;
pub use acta_types::{ActaError, DocId, Result, TxId, TOMBSTONE_BIT};
pub use error::MvccError;
pub use seq_log::{SeqLogChange, SeqLogEntry, SequenceLog};
pub use snapshot::{SnapshotGuard, SnapshotRegistry};
pub use ssi::{PruningBlockerInfo, ReadSet, SequenceLogSsiValidator, SsiValidator};
pub use tx_buffer::{IndexOp, TxBuffer};
```

### Schnittstellen — Public Traits (2)

**`SsiValidator`** — *Schnittstelle (Trait) für die Validierung von Transaktionen unter SSI.* (`src/ssi.rs`)

```rust
pub trait SsiValidator {
/// Validiert das [`ReadSet`] der Transaktion `tx_id`.
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

### Öffentliche Datentypen — Structs (11)

| Struct | Felder / Form | Beschreibung | Datei |
|---|---|---|---|
| `PruningBlockerInfo` | `{ /// Kleinste Sequenznummer, die noch unbereinigt im Register liegt. pub min_unpruned_seq: u64, /// Aktueller `pruned_through`-Wasserstand. pub pruned_through_s… }` | Diagnoseinformationen über aktive Snapshots oder Sequenznummern, die das Pruning im Validator blockieren. Wird erzeugt, wenn der Speicher für Commit-… | `src/ssi.rs` |
| `ReadSet` | `{ /// Zuordnung von gelesenen Schlüsseln (Bytefolgen) zur zugehörigen Snapshot-Sequenznummer. keys: AHashMap<Vec<u8>, u64>, /// Zuordnung von Scan-Präfixen zu de… }` | Protokolliert gelesene Einzelschlüssel, Präfixe und deren Snapshot-Sequenznummer für SSI. | `src/ssi.rs` |
| `SeqAllocator` | `{ current_seq: AtomicU64, ledger_floor: AtomicU64, }` | Atomarer Sequenzallokator mit Restore-Sicherheit. Garantiert, dass generierte Sequenznummern sowohl strikt größer als die bisherige `current_seq` als… | `src/seq.rs` |
| `SeqLogEntry` | `{ /// Eindeutige Kennung des Dokuments. pub doc_id: DocId, /// Logische Sequenznummer, bei der dieser Dokumenteintrag eingefügt wurde. pub insert_seq: u64, /// L… }` | Ein versionierter Protokolleintrag für die Snapshot-Isolation (`_at`-Familie). Jeder Eintrag repräsentiert die Lebensspanne einer Dokumentversion: vo… | `src/seq_log.rs` |
| `SequenceLog` | `{ /// Liste aller historischen Dokumentversionen. entries: Vec<SeqLogEntry>, /// Zuordnung von gepinnter Sequenznummer zu Liste von Erstellungs-Zeitstempeln (in… }` | Verwaltet das chronologische Sequenz-Protokoll und Sichtbarkeitsfilter für Indexstrukturen. ### Funktionsweise: `SequenceLog` speichert alle Einfüge-… | `src/seq_log.rs` |
| `SequenceLogSsiValidator` | `{ committed_writes: Arc<RwLock<CommittedWrites>>, sequence_log: Option<Arc<RwLock<SequenceLog>>>, snapshot_registry: Option<Arc<SnapshotRegistry>>, pruned_throug… }` | Referenz-Implementierung von [`SsiValidator`] auf Basis von [`SequenceLog`] und In-Memory-Commit-Tracking. Arbeitet vollständig ohne Festplatten-I/O… | `src/ssi.rs` |
| `SnapshotGuard` | `{ registry: Arc<SnapshotRegistry>, seq_no: u64, created_at: Option<u64>, }` | Veralteter RAII-Guard für Abwärtskompatibilität. | `src/snapshot.rs` |
| `SnapshotHandle` | `{ registry: Arc<SnapshotRegistry>, tenant: TenantTag, seq: Seq, created_at: Option<u64>, revoked: Arc<AtomicBool>, }` | RAII-Handle für einen aktiven Lese-Snapshot eines Mandanten. | `src/snapshot.rs` |
| `SnapshotRegistry` | `{ tenants: Mutex<BTreeMap<TenantTag, TenantState>>, revoked_tags: Mutex<BTreeSet<(TenantTag, Seq)>>, active_count: AtomicUsize, min_active_seqno: AtomicU64, cloc… }` | Zentrale Registrierung für aktive Lese-Snapshots im System. | `src/snapshot.rs` |
| `TxBuffer` | `{ shards: Vec<RwLock<TxShard<T>>>, key_shards: Vec<RwLock<KeyShard>>, tx_timeout: Duration, config: TxBufferConfig, global_staged_bytes: AtomicUsize, clock: Arc<… }` | Verwaltet gepufferte Transaktionen, deren Staging-Operationen und ReadSets. ### Synchronisations- und Sperrstrategie: Jeder Shard besitzt ein eigenes… | `src/tx_buffer.rs` |
| `TxBufferConfig` | `{ /// Timeout-Dauer, nach der verwaiste (inaktive) Transaktionen automatisch abgeräumt werden. pub tx_timeout: Duration, /// Maximale empfohlene Anzahl gleichzei… }` | Konfigurationsoptionen für den [`TxBuffer`]. | `src/tx_buffer.rs` |

### Öffentliche Datentypen — Enums (3)

| Enum | Varianten | Beschreibung | Datei |
|---|---|---|---|
| `IndexOp` | `/// Einfügen eines Dokuments mit Nutzlast (z. B. Einbettungs-Vektoren oder Schlüssel-Werte). Insert { /// Kennung des Dokuments. doc_id: DocId, /// Assoziierte Nutzdaten. data: T, }, /// Löschen eine…` | Eine im Puffer vorgemerkte Operation auf einem Index. | `src/tx_buffer.rs` |
| `MvccError` | `/// Ungültige Sequenznummer. #[error("Invalid sequence number: {0}")] InvalidSeq(u64), /// Veraltete Variante für Abwärtskompatibilität. #[error("Invalid sequence number: {0}")] InvalidSequence(u64),…` | Typisierte Fehler für `acta-mvcc`. | `src/error.rs` |
| `SeqLogChange` | `/// Einfügen eines Dokuments zu einer bestimmten Sequenznummer. Insert { /// Eindeutige Kennung des Dokuments. doc_id: DocId, /// Sequenznummer des Einfügens. seq: u64, }, /// Löschen eines Dokuments…` | Repräsentiert eine historische Änderung (Einfügen oder Löschen) im Sequenz-Log für Delta-Replays. Wird von Abfragen wie [`SequenceLog::changes_since`… | `src/seq_log.rs` |

### Typ-Aliase (4)

| Alias | Definition | Beschreibung |
|---|---|---|
| `Matcher` | `type Matcher= ()` | Platzhalter für Matcher-Bereiche (WP-5.1: Fail-Closed Mandantenwiderruf bis `gated_seq`). |
| `RevokedCount` | `type RevokedCount= usize` | Anzahl widerrufener Snapshots. |
| `Seq` | `type Seq= u64` | Sequenznummer. |
| `TenantTag` | `type TenantTag= [u8` | Fester 16-Byte Mandanten-Tag gemäß D-61. |

### Konstanten (10)

| Konstante | Typ | Beschreibung |
|---|---|---|
| `DEFAULT_MAX_OPS_PER_TX` | `usize` | Empfohlene Maximalanzahl an Operationen pro Einzelschreibtransaktion (10.000 Operationen). |
| `DEFAULT_MAX_PIN_DURATION` | `Duration` | Standard-Lebensdauer für Snapshot-Pins (5 Minuten), bevor Diagnose-Warnungen für blockierende oder verwaiste Pins ausgelöst werden. |
| `DEFAULT_MAX_READ_SET_KEYS` | `usize` | Standard-Maximalanzahl an getrackten Lese-Schlüsseln pro Transaktion (100.000 Schlüssel). |
| `DEFAULT_MAX_TOTAL_STAGED_BYTES` | `usize` | Standard-Maximalgröße für den globalen Puffer über alle Transaktionen hinweg (256 MiB). |
| `DEFAULT_MAX_TRACKED_COMMIT_KEYS` | `usize` | Standard-Höchstgrenze für im Speicher gehaltene bestätigte Schreib-Schlüssel (1.000.000 Schlüssel). |
| `DEFAULT_MAX_TX_STAGED_BYTES` | `usize` | Standard-Maximalgröße für gepufferte Rohdaten pro Einzelschreibtransaktion (16 MiB). |
| `DEFAULT_SHARD_COUNT` | `usize` | Standardanzahl an Partitionen (Shards) im Transaktionspuffer (64 Shards). |
| `DEFAULT_TENANT_TAG` | `TenantTag` | Standard-Mandantentag für Abwärtskompatibilität. |
| `MAX_SNAPSHOTS_PER_TENANT` | `usize` | Obergrenze aktiver Snapshots je Mandant (D-87). |
| `STAGING_ENTRY_OVERHEAD_BYTES` | `usize` | Fester Speicher-Overhead pro gepufferter Operation (32 Bytes). |

### Öffentliche Funktionen (132)

| Signatur | Beschreibung | Datei |
|---|---|---|
| `pub fn current_seq(&self) -> u64` | Liefert die zuletzt vergebene oder gesetzte `current_seq`. | `src/seq.rs` |
| `pub fn ledger_floor(&self) -> u64` | Liefert den aktuellen `ledger_floor`. | `src/seq.rs` |
| `pub fn new(initial_seq: u64, ledger_floor: u64) -> Self` | Erzeugt einen neuen `SeqAllocator` mit initialen Sequenz- und Floor-Werten. | `src/seq.rs` |
| `pub fn next_seq(&self) -> Result<u64, MvccError>` | Berechnet und allokiert atomar die nächste Sequenznummer. ### Algorithmus (MV-06 / I-20): 1. Lade `current_seq` und `ledger_floor`. 2. Bere… | `src/seq.rs` |
| `pub fn reset_after_restore(&self, current: u64, floor: u64)` | Setzt nach einem Restore-Ereignis den Zustand neu. Setzt `current_seq` auf den Stand des restaurierten Snapshots und `ledger_floor` auf den… | `src/seq.rs` |
| `pub fn set_ledger_floor(&self, floor: u64)` | Passt den `ledger_floor` an (z. B. nach Wiederherstellung oder Ledger-Scan). Nutzt `fetch_max`, damit der Floor niemals verkleinert werden… | `src/seq.rs` |
| `pub fn changes_since(&self, snapshot_seq: u64) -> Vec<SeqLogChange>` | Liefert alle Änderungen (Einfügungen und Löschungen), die strikt nach `snapshot_seq` stattgefunden haben, aufsteigend sortiert nach Sequenz… | `src/seq_log.rs` |
| `pub fn clock(&self) -> &Arc<dyn Clock>` | Gibt eine Referenz auf den aktiven [`Clock`]-Port zurück. | `src/seq_log.rs` |
| `pub fn compact(&mut self, min_active_seqno: u64)` | Verdichtet (kompaktiert) das Sequenz-Log und entfernt historische Versionen, deren Löschung strikt kleiner als `min_active_seqno` ist. ###… | `src/seq_log.rs` |
| `pub fn deletion_seq(&self, doc_id: DocId) -> Option<u64>` | Liefert die Lösch-Sequenznummer des Dokuments, falls es aktuell als gelöscht verzeichnet ist. | `src/seq_log.rs` |
| `pub fn doc_id(&self) -> DocId` | Liefert die Kennung des betroffenen Dokuments. | `src/seq_log.rs` |
| `pub fn expired_pins(&self) -> Vec<u64>` | Liefert alle abgelaufenen Snapshot-Pins bezogen auf die aktuelle Systemuhr. | `src/seq_log.rs` |
| `pub fn expired_pins_at(&self, now_nanos: u64) -> Vec<u64>` | Findet alle gepinnten Sequenznummern, deren Haltezeit `max_pin_duration` überschreitet, bezogen auf den übergebenen Monotonie-Zeitstempel `… | `src/seq_log.rs` |
| `pub fn is_empty(&self) -> bool` | Gibt `true` zurück, wenn das Sequenz-Log keine Einträge enthält. | `src/seq_log.rs` |
| `pub fn is_visible(&self, as_of: u64) -> bool` | Prüft in O(1), ob dieser Protokolleintrag zu einem bestimmten Snapshot-Stand `as_of` sichtbar ist. ### Ablauf der Sichtbarkeitsprüfung: 1.… | `src/seq_log.rs` |
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
| `pub fn clock(&self) -> &Arc<dyn Clock>` | Liefert eine Referenz auf den aktiven Zeitport ([`Clock`]). | `src/snapshot.rs` |
| `pub fn is_revoked(&self, seq_no: u64) -> bool` | Prüft, ob ein Snapshot für den Standard-Mandanten widerrufen wurde. | `src/snapshot.rs` |
| `pub fn is_revoked(&self) -> bool` | Prüft, ob dieser Snapshot widerrufen wurde. | `src/snapshot.rs` |
| `pub fn is_revoked_tenant(&self, tenant: TenantTag, seq_no: u64) -> bool` | Prüft, ob ein Snapshot eines bestimmten Mandanten widerrufen wurde. | `src/snapshot.rs` |
| `pub fn longest_active_pin(&self) -> Option<(u64, Duration)>` | Findet den am längsten aktiven Snapshot bezogen auf die aktuelle Systemuhr. | `src/snapshot.rs` |
| `pub fn longest_active_pin_at(&self, now_nanos: u64) -> Option<(u64, Duration)>` | Findet den am längsten aktiven Snapshot bezüglich des Nanosekunden-Zeitstempels `now_nanos`. | `src/snapshot.rs` |
| `pub fn min_active_seqno(&self) -> u64` | Abwärtskompatible Hilfsmethode für bestehende Aufrufer. Gibt die kleinste Sequenznummer zurück, oder 0, falls keine Snapshots aktiv sind. K… | `src/snapshot.rs` |
| `pub fn min_for_compaction(&self, current: Seq) -> Seq` | Minimal erforderliche Sequenz für die Kompaktierung (MV-02). Liefert `min(min_read_snapshot, current)`, bei leerer Registry stets `current`. | `src/snapshot.rs` |
| `pub fn min_read_snapshot(&self) -> Option<Seq>` | Minimal aktive Snapshot-Sequenznummer über alle Mandanten. Liefert `None`, wenn keine Snapshots aktiv sind (MV-01). | `src/snapshot.rs` |
| `pub fn new() -> Self` | Erstellt eine neue, leere Snapshot-Registry mit Standard-[`SystemClock`]. | `src/snapshot.rs` |
| `pub fn open(self: &Arc<Self>, tenant: TenantTag, at: Seq) -> Result<SnapshotHandle, MvccError>` | Öffnet einen neuen Snapshot für den Mandanten `tenant` bei Sequenz `at`. Garantiert das Limit von maximal 1024 aktiven Snapshots je Mandant… | `src/snapshot.rs` |
| `pub fn pin(&self, seq_no: u64)` | Pinnt eine Sequenznummer dauerhaft. | `src/snapshot.rs` |
| `pub fn pin_at(&self, seq_no: u64, at_nanos: u64)` | Pinnt eine Sequenznummer zu einem spezifischen Zeitstempel. | `src/snapshot.rs` |
| `pub fn reap_expired_pins(&self, max_pin_duration: Duration) -> Vec<u64>` | Bereinigt alle überalterten Snapshot-Pins bezogen auf die aktuelle Uhrzeit. | `src/snapshot.rs` |
| `pub fn reap_expired_pins_at(&self, max_pin_duration: Duration, now_nanos: u64) -> Vec<u64>` | Bereinigt und entfernt alle aktiven Snapshot-Pins, die älter als `max_pin_duration` sind. | `src/snapshot.rs` |
| `pub fn register(self: &Arc<Self>, seq_no: u64) -> SnapshotGuard` | Abwärtskompatible Registrierung für Default-Mandant. | `src/snapshot.rs` |
| `pub fn register_at(self: &Arc<Self>, seq_no: u64, at_nanos: u64) -> SnapshotGuard` | Abwärtskompatible Registrierung mit explizitem Zeitstempel. | `src/snapshot.rs` |
| `pub fn revoke_covering( &self, tenant: TenantTag, _range: &Matcher, gated_seq: Seq, ) -> RevokedCount` | Widerruft alle Snapshots des Mandanten mit `seq <= gated_seq` (fail-closed, D-61). | `src/snapshot.rs` |
| `pub fn revoke_through(&self, erased_through_seq: u64) -> usize` | Widerruft alle Snapshots mit `seq <= erased_through_seq` (D-11). | `src/snapshot.rs` |
| `pub fn revoke_where<F>(&self, predicate: F) -> usize where F: Fn(u64) -> bool,` | Markiert Snapshots, die das Prädikat erfüllen, als widerrufen. | `src/snapshot.rs` |
| `pub fn seq(&self) -> Result<Seq, MvccError>` | Liefert die Sequenznummer des Snapshots oder `Err(MvccError::SnapshotRevoked)`, falls der Snapshot widerrufen wurde (MV-05). | `src/snapshot.rs` |
| `pub fn seq_no(&self) -> u64` | Liefert die Sequenznummer. | `src/snapshot.rs` |
| `pub fn tenant(&self) -> TenantTag` | Mandanten-Tag dieses Handles. | `src/snapshot.rs` |
| `pub fn unpin(&self, seq_no: u64)` | Hebt einen dauerhaften Pin auf. | `src/snapshot.rs` |
| `pub fn validate(&self) -> Result<(), MvccError>` | Validiert den Snapshot. | `src/snapshot.rs` |
| `pub fn with_clock(clock: Arc<dyn Clock>) -> Self` | Erstellt eine neue Snapshot-Registry mit benutzerdefiniertem [`Clock`]-Port. | `src/snapshot.rs` |
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
| `pub fn prefixes_iter(&self) -> impl Iterator<Item = (&Vec<u8>, &u64)>` | Liefert einen Iterator über alle Bereichs-Präfixe und deren Snapshot-Sequenzen. | `src/ssi.rs` |
| `pub fn prefixes_map(&self) -> &AHashMap<Vec<u8>, u64>` | Liefert die interne Präfix-Map als Referenz. | `src/ssi.rs` |
| `pub fn prune_through(&self, bound_seq: u64) -> usize` | — | `src/ssi.rs` |
| `pub fn prune_through_checked(&self, bound: Option<u64>) -> std::result::Result<(), MvccError>` | Bereinigt (prunt) bestätigte Schreib-Einträge mit `commit_seq <= bound`. Vertrag V-MVCC: - `bound == None`: Es gibt nichts zu prunen -> lie… | `src/ssi.rs` |
| `pub fn pruned_through(&self) -> u64` | Liefert den aktuellen `pruned_through`-Wasserstand. | `src/ssi.rs` |
| `pub fn record_commit_key(&self, key: &[u8], commit_seq: u64)` | Zeichnet einen committeten Schlüssel zu einer Sequenznummer auf. | `src/ssi.rs` |
| `pub fn record_commit_keys<'a>( &self, keys: impl IntoIterator<Item = &'a [u8]>, commit_seq: u64, )` | Zeichnet eine Menge committeter Schlüssel stapelweise (Batch) zu einer Commit-Sequenznummer auf. | `src/ssi.rs` |
| `pub fn record_prefix(&mut self, prefix: impl Into<Vec<u8>>, snapshot_seq: u64)` | Zeichnet einen Bereichsscan-Präfix zu `snapshot_seq` auf, um Phantom-Reads zu erkennen. Wenn nach diesem Lesezugriff eine andere Transaktio… | `src/ssi.rs` |
| `pub fn record_read(&mut self, key: impl Into<Vec<u8>>, snapshot_seq: u64)` | Zeichnet einen Lesezugriff auf `key` zu einer bestimmten Snapshot-Sequenznummer `snapshot_seq` auf. ### Ablauf: 1. Maskiere das Tombstone-B… | `src/ssi.rs` |
| `pub fn register_read(&mut self, key: impl Into<Vec<u8>>, snapshot_seq: u64)` | Alias-Funktion für [`ReadSet::record_read`]. | `src/ssi.rs` |
| `pub fn set_clock(&mut self, clock: Arc<dyn Clock>)` | Ersetzt die aktive [`Clock`]-Instanz. | `src/ssi.rs` |
| `pub fn set_metrics_sink( &mut self, sink: Arc<parking_lot::RwLock<Arc<dyn acta_ports::MetricsSink>>>, )` | Aktualisiert den Metrik-Empfänger. | `src/ssi.rs` |
| `pub fn set_snapshot_registry(&mut self, registry: Arc<SnapshotRegistry>)` | Setzt die verknüpfte [`SnapshotRegistry`]. | `src/ssi.rs` |
| `pub fn tracked_commit_keys(&self) -> usize` | Liefert die Anzahl aktuell gehaltener Commit-Schlüssel. | `src/ssi.rs` |
| `pub fn validate_and_record<'a>( &self, tx_id: TxId, read_set: &ReadSet, write_keys: impl IntoIterator<Item = &'a [u8]>, commit_seq: u64, ) -> Result<()>` | Führt Validierung und Eintragen der Commit-Schlüssel atomar unter demselben Schreib-Lock aus. | `src/ssi.rs` |
| `pub fn with_clock(mut self, clock: Arc<dyn Clock>) -> Self` | Bindet eine benutzerdefinierte Implementierung des [`Clock`]-Ports an den Validator. | `src/ssi.rs` |
| `pub fn with_max_pin_duration(mut self, duration: Duration) -> Self` | Konfiguriert die maximale Pin-Dauer für Diagnosealarme. | `src/ssi.rs` |
| `pub fn with_metrics_sink( mut self, sink: Arc<parking_lot::RwLock<Arc<dyn acta_ports::MetricsSink>>>, ) -> Self` | Bindet einen Metrik-Empfänger ([`acta_ports::MetricsSink`]) für Monitoring an. | `src/ssi.rs` |
| `pub fn with_sequence_log(seq_log: Arc<RwLock<SequenceLog>>) -> Self` | Erstellt einen Validator, der an ein bestehendes [`SequenceLog`] angebunden ist. | `src/ssi.rs` |
| `pub fn with_snapshot_registry(mut self, registry: Arc<SnapshotRegistry>) -> Self` | Verknüpft eine [`SnapshotRegistry`] zur Überwachung aktiver Snapshot-Pins. | `src/ssi.rs` |
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
| `pub fn read_set(&self, tx: TxId) -> Option<ReadSet>` | Alias-Funktion für [`TxBuffer::get_read_set`]. | `src/tx_buffer.rs` |
| `pub fn reap_orphans(&self) -> Vec<TxId>` | Räumt verwaiste Transaktionen unter Verwendung der aktuellen Uhrzeit ab. | `src/tx_buffer.rs` |
| `pub fn reap_orphans_at(&self, now_nanos: u64) -> Vec<TxId>` | Räumt abgelaufene (verwaiste) Transaktionen bezüglich `now_nanos` über alle Shards ab. | `src/tx_buffer.rs` |
| `pub fn reap_orphans_bounded(&self, max: usize) -> Vec<TxId>` | Räumt bis zu `max` verwaiste Transaktionen bezogen auf die aktuelle Uhrzeit ab. | `src/tx_buffer.rs` |
| `pub fn reap_orphans_bounded_at(&self, max: usize, now_nanos: u64) -> Vec<TxId>` | Räumt maximal `max` verwaiste Transaktionen ab, um Latenzspitzen zu vermeiden. ### Ablauf: 1. Iteriere aufsteigend über die Shards (0 bis $… | `src/tx_buffer.rs` |
| `pub fn record_read(&self, tx: TxId, key: impl Into<Vec<u8>>, snapshot_seq: u64)` | Alias-Funktion für [`TxBuffer::register_read`]. | `src/tx_buffer.rs` |
| `pub fn register_read(&self, tx: TxId, key: impl Into<Vec<u8>>, snapshot_seq: u64)` | Registriert einen Lesezugriff im ReadSet der Transaktion `tx`. | `src/tx_buffer.rs` |
| `pub fn stage(&self, tx: TxId, op: IndexOp<T>) -> Result<()> where T: StagedOpSize,` | Puffert eine Indexoperation für die Transaktion `tx` unter Prüfung aller Speichergrenzen. | `src/tx_buffer.rs` |
| `pub fn stage_bounded(&self, tx: TxId, op: IndexOp<T>) -> Result<()> where T: StagedOpSize,` | Puffert eine Operation und erzwingt Obergrenzen für Operationsanzahl und Byte-Budgets. | `src/tx_buffer.rs` |
| `pub fn stage_kv(&self, tx: TxId, op: IndexOp<(Vec<u8>, Vec<u8>)>) -> Result<()>` | Puffert eine Schlüssel-Wert-Operation und aktualisiert die Key-Staging-Map. Bei Fehlschlag von `stage()` (z. B. Budget-Überlauf) wird das K… | `src/tx_buffer.rs` |
| `pub fn stage_many(&self, tx: TxId, ops: impl IntoIterator<Item = IndexOp<T>>) -> Result<()> where T: StagedOpSize,` | Puffert mehrere Operationen stapelweise mit einer einzigen Sperrung des Shards. | `src/tx_buffer.rs` |
| `pub fn staged_bytes(&self) -> usize` | Liefert den aktuell global über alle Transaktionen verbrauchten Staging-Speicher in Bytes. | `src/tx_buffer.rs` |
| `pub fn staged_status(&self, key: &[u8]) -> Option<bool>` | Ermittelt atomar den Staging-Status eines Schlüssels über alle uncommitted Transaktionen. Liefert `Some(true)` bei Einfügung, `Some(false)`… | `src/tx_buffer.rs` |
| `pub fn try_register_read( &self, tx: TxId, key: impl Into<Vec<u8>>, snapshot_seq: u64, max_keys: usize, ) -> Result<()>` | Registriert einen gelesenen Schlüssel für Transaktion `tx` mit Obergrenzen-Prüfung (`max_keys`). Verhindert unbegrenztes Speicherwachstum b… | `src/tx_buffer.rs` |
| `pub fn tx_timeout(&self) -> Duration` | Liefert die konfigurierte Transaktions-Timeout-Dauer. | `src/tx_buffer.rs` |
| `pub fn validate_pending_ops(&self, tx: TxId) -> Result<()>` | Prüft, ob für eine registrierte Transaktion tatsächlich anhängige Operationen existieren. | `src/tx_buffer.rs` |
| `pub fn with_clock(mut self, clock: Arc<dyn Clock>) -> Self` | Bindet eine benutzerdefinierte [`Clock`]-Instanz an diesen Puffer an. | `src/tx_buffer.rs` |


---

## `acta-sys`

| Attribut | Wert |
|---|---|
| **Ring** | 0 — Ring 0 — Foundation & Pure Numerics/Types |
| **Status** | Fertig |
| **Zweck** | System-Abstraktionen für mmap, mlock und Win32-ACLs (Unsafe-Insel). |
| **Unsafe-Code** | Ja (deklarierte Unsafe-Insel) |
| **Interne Abhängigkeiten** | — |
| **Externe Abhängigkeiten (Auswahl)** | `libc`, `tracing` |

### Modul- und Re-Export-Oberfläche (lib.rs)

```rust
pub mod durability;
pub mod error;
pub mod fs_probe;
pub mod hardening;
pub mod memlock;
pub mod perm;
pub mod rename;
pub mod sync;
pub use durability::{ durable_flush_dir, durable_flush_file, flush_dir, flush_dir_sync, flush_file, flush_file_sync, rename_durable, rename_durable_sync, write_all_durable, FlushKind, };
pub use error::{SyncFailure, SysError, UnsupportedKind};
pub use fs_probe::{fs_class, fs_class_spec, FsClass, FsKind};
pub use hardening::{ apply_process_hardening, ensure_fd_limit, lock_pages, mark_dont_dump, secure_zero_volatile, set_fd_limit, HardeningPolicy, HardeningReport, };
pub use perm::{set_mode, set_restrictive_file_acl, verify_file_acl_owner_only, verify_owner_only};
```

### Öffentliche Datentypen — Structs (1)

| Struct | Felder / Form | Beschreibung | Datei |
|---|---|---|---|
| `HardeningReport` | `{ /// Ob das Coredump-Limit (`RLIMIT_CORE`) auf 0 gesetzt werden konnte. pub core_limit_zero: bool, /// Ob `PR_SET_DUMPABLE` auf 0 gesetzt wurde (Linux). pub dum… }` | Statusbericht ueber die durchgefuehrten Haertungsmaßnahmen. | `src/hardening.rs` |

### Öffentliche Datentypen — Enums (7)

| Enum | Varianten | Beschreibung | Datei |
|---|---|---|---|
| `FlushKind` | `/// Flush only data without forcing metadata updates to disk (Linux `fdatasync`). Data, /// Flush both data and all filesystem metadata (Linux `fsync`, macOS `F_FULLFSYNC`). All, /// Abwaertskompatib…` | Specification of the flush scope for file handles. | `src/durability.rs` |
| `FsClass` | `/// Local filesystem with supported POSIX durability guarantees. LocalSupported, /// Network filesystem. Network, /// User-space filesystem. Fuse, /// Cloud-synchronized directory. CloudSync, /// Unk…` | Dateisystemklassifikation fuer Durability- und Lock-Garantien (SPEC 8.3 / D-56). | `src/fs_probe.rs` |
| `FsKind` | `/// Linux Ext4 Ext4, /// Linux XFS Xfs, /// Apple APFS Apfs, /// Btrfs Btrfs, /// OpenZFS / ZFS Zfs, /// Nicht unterstuetztes oder nicht auditierbares Dateisystem. Unsupported(UnsupportedKind),` | Bekannte und unterstuetzte Dateisystemtypen. | `src/fs_probe.rs` |
| `HardeningPolicy` | `/// Pflichtmodus: Jede fehlgeschlagene Maßnahme fuehrt zum Fehlerabbruch. Required, /// Standardmodus: Maßnahmen werden nach Moeglichkeit angewendet; Fehler fuehren nicht zum Abbruch. #[default] Best…` | Richtlinie fuer die Prozesshaertung (D-63). | `src/hardening.rs` |
| `SyncFailure` | `/// Das Dateisystem oder die Operation wird auf der Zielplattform nicht unterstuetzt. Unsupported, /// Systemaufruf scheiterte mit Zugriffsfehler oder Dateideskriptor ungueltig. PermissionDenied, ///…` | Fester Fehler bei Synchronisationsoperationen (fsync, fdatasync, F_FULLFSYNC, flush_dir). Kann nicht irrtuemlich in `Ok` umgedeutet werden (D-55). | `src/error.rs` |
| `SysError` | `/// Unzureichende Berechtigungen. PermissionDenied, /// Ziel nicht gefunden. NotFound, /// Nicht unterstuetzte Operation oder Plattform. Unsupported, /// Prozesshaertung unvollstaendig bei Policy `Re…` | Allgemeine Systemfehler fuer Low-Level-Operationen in `acta-sys`. | `src/error.rs` |
| `UnsupportedKind` | `/// Dateisystemtyp ist dem System unbekannt oder nicht auditierbar. Unknown, /// Netzlaufwerk (NFS, SMB/CIFS etc.). Network, /// User-Space-Dateisystem (FUSE). Fuse, /// Virtuelles In-Memory-Dateisys…` | Nicht unterstuetzte Dateisystemtypen fuer `FsKind`. | `src/error.rs` |

### Öffentliche Funktionen (21)

| Signatur | Beschreibung | Datei |
|---|---|---|
| `pub fn durable_flush_dir(path: &Path) -> Result<()>` | Abwaertskompatible Signatur fuer `flush_dir`. | `src/durability.rs` |
| `pub fn durable_flush_file(file: &File, metadata_changed: bool) -> Result<()>` | Abwaertskompatible Signatur fuer `flush_file`. | `src/durability.rs` |
| `pub fn flush_dir(path: &Path) -> Result<()>` | Flasht ein Verzeichnis persistent auf den Datenträger. # Ablauf 1. Öffnet das Verzeichnis schreibgeschützt als [`File`]. 2. Ruft [`flush_fi… | `src/durability.rs` |
| `pub fn flush_dir_sync(path: &Path) -> std::result::Result<(), SyncFailure>` | Spezifikationsgemaeße Variante mit `SyncFailure` als Fehlertyp (D-55). | `src/durability.rs` |
| `pub fn flush_file(file: &File, kind: FlushKind) -> Result<()>` | Flasht Dateiinhalt und optional Metadaten auf den physischen Datenträger gemäß den Garantien der Zielplattform. # Ablauf 1. Extrahiert den… | `src/durability.rs` |
| `pub fn flush_file_sync(file: &File, kind: FlushKind) -> std::result::Result<(), SyncFailure>` | Spezifikationsgemaeße Variante mit `SyncFailure` als Fehlertyp (D-55). | `src/durability.rs` |
| `pub fn rename_durable(from: &Path, to: &Path) -> Result<()>` | Durably renames `from` to `to` and flushes the parent directory. # Ablauf 1. Benennt `from` atomar in `to` um mittels [`std::fs::rename`].… | `src/durability.rs` |
| `pub fn rename_durable_sync(from: &Path, to: &Path) -> std::result::Result<(), SyncFailure>` | Spezifikationsgemaeße Variante mit `SyncFailure` als Fehlertyp (D-55). | `src/durability.rs` |
| `pub fn write_all_durable(file: &mut File, mut buf: &[u8]) -> Result<()>` | Schreibt die gesamten Bytes aus `buf` zuverlässig in `file`, unter robuster Behandlung partieller Schreibvorgänge und Signal-Unterbrechunge… | `src/durability.rs` |
| `pub fn fs_class(path: &Path) -> std::io::Result<FsClass>` | Klassifiziert das Dateisystem eines gegebenen Pfades. | `src/fs_probe.rs` |
| `pub fn fs_class_spec(path: &Path) -> Result<FsClass, SysError>` | Spezifikationsgemaeße Klassifikation gemaess 03-acta-sys.md. | `src/fs_probe.rs` |
| `pub fn apply_process_hardening(policy: HardeningPolicy) -> Result<HardeningReport, SysError>` | Fuehrt die Prozesshaertung gemaess der gewaehlten Richtlinie durch. # Nebenwirkungen Beeinflusst das Coredump-Verhalten des gesamten laufen… | `src/hardening.rs` |
| `pub fn ensure_fd_limit(min_fds: u64) -> bool` | Stellt sicher, dass das weiche Limit fuer offene Dateideskriptoren (`RLIMIT_NOFILE`) mindestens `min_fds` betraegt. | `src/hardening.rs` |
| `pub fn lock_pages(slice: &[u8], policy: HardeningPolicy) -> Result<bool, SysError>` | Sperrt die Speicherseiten von `slice` im RAM gegen Auslagerung (Swap). Sicher, da `slice` eine gueltige Allokation fuer die Dauer des Aufru… | `src/hardening.rs` |
| `pub fn mark_dont_dump(slice: &[u8]) -> bool` | Markiert einen Speicher-Slice mit `madvise(MADV_DONTDUMP)`, damit dieser bei Coredumps ausgelassen wird. Sicher, weil `slice` fuer die Daue… | `src/hardening.rs` |
| `pub fn secure_zero_volatile(slice: &mut [u8])` | Überschreibt einen Speicherbereich (`slice`) mit Nullen mittels flüchtiger Schreiboperationen (`write_volatile`) und sequenzieller Compiler… | `src/hardening.rs` |
| `pub fn set_fd_limit(limit: u64) -> bool` | Setzt das weiche und harte Dateideskriptorlimit (`RLIMIT_NOFILE`) des aktuellen Prozesses exakt auf `limit`. | `src/hardening.rs` |
| `pub fn set_mode(path: &Path, mode: u32) -> Result<(), SysError>` | Setzt die Berechtigungsmaske einer Datei oder eines Verzeichnisses (z. B. `0o700` oder `0o600`). Gemäß SY-09 sind ausschließlich `0o700` (V… | `src/perm.rs` |
| `pub fn set_restrictive_file_acl(path: &Path) -> std::io::Result<()>` | Abwaertskompatibler Wrapper: Setzt restriktive Rechte (0o600 fuer Dateien, 0o700 fuer Verzeichnisse). | `src/perm.rs` |
| `pub fn verify_file_acl_owner_only(path: &Path) -> std::io::Result<()>` | Abwaertskompatibler Wrapper: Verifiziert Owner-Only-Rechte. | `src/perm.rs` |
| `pub fn verify_owner_only(path: &Path) -> Result<(), SysError>` | Prueft, dass die Datei oder das Verzeichnis ausschliesslich dem aktuellen Prozess-Besitzer zugaenglich ist und keine unberechtigten Lese-/S… | `src/perm.rs` |


---

## `acta-simd`

| Attribut | Wert |
|---|---|
| **Ring** | 0 — Ring 0 — Foundation & Pure Numerics/Types |
| **Status** | Fertig |
| **Zweck** | AVX2/AVX-512/NEON SIMD-Distanzberechnungskerne mit Laufzeit-Dispatch (Unsafe-Insel). |
| **Unsafe-Code** | Ja (deklarierte Unsafe-Insel) |
| **Interne Abhängigkeiten** | `acta-types` |
| **Externe Abhängigkeiten (Auswahl)** | `bytemuck`, `thiserror`, `tracing` |

### Architekturbeschreibung

# Crate `acta-simd` Vektor-Distanzfunktionen (L2, Dot, Cosine) mit SIMD-Kernen und Skalar-Referenz (Ring 1).  

### Modul- und Re-Export-Oberfläche (lib.rs)

```rust
pub mod dispatch;
pub mod kernels;
pub use dispatch::*;
pub use kernels::scalar::{ canonical_reduce_16, cosine_distance_f32_bytes_scalar, cosine_distance_scalar, cosine_similarity_parts_u8_scalar, dot_product_f32_bytes_scalar, dot_product_scalar, dot_product_u8_scalar, euclidean_distance_f32_bytes_scalar, euclidean_distance_scalar, euclidean_distance_sq_u8_scalar, normalize_inplace, CosineSimilarityPartsU8, };
```

### Öffentliche Datentypen — Structs (1)

| Struct | Felder / Form | Beschreibung | Datei |
|---|---|---|---|
| `CosineSimilarityPartsU8` | `{ pub dot: u32, pub norm_a_sq: u32, pub norm_b_sq: u32, }` | Komponenten für ganzzahlige Kosinus-Ähnlichkeit. | `src/kernels/scalar.rs` |

### Öffentliche Datentypen — Enums (2)

| Enum | Varianten | Beschreibung | Datei |
|---|---|---|---|
| `SimdPath` | `Avx512, Avx2Fma, Neon, Scalar,` | SIMD execution path selected at runtime based on hardware capabilities. | `src/dispatch.rs` |
| `SimdPathOverride` | `Scalar, Avx2, Avx512, Neon,` | SIMD path override options for testing purposes. | `src/dispatch.rs` |

### Konstanten (1)

| Konstante | Typ | Beschreibung |
|---|---|---|
| `MAX_VECTOR_DIMENSION` | `usize` | — |

### Öffentliche Funktionen (31)

| Signatur | Beschreibung | Datei |
|---|---|---|
| `pub fn active_path() -> SimdPath` | Ermittelt den aktiv gewählten SIMD-Pfad für Diagnose und Berichte (SI-04). Der Pfad wird beim ersten Aufruf bestimmt und gecacht. | `src/dispatch.rs` |
| `pub fn cosine_distance(a: &[f32], b: &[f32]) -> Result<f32, ActaError>` | Computes cosine distance ($1.0 - \text{cosine\_similarity}$) between two `f32` vectors. | `src/dispatch.rs` |
| `pub fn cosine_distance_f32_bytes(a: &[f32], b_bytes: &[u8]) -> Result<f32, ActaError>` | Computes cosine distance between an `f32` vector and an LE raw `f32` byte slice. | `src/dispatch.rs` |
| `pub fn cosine_similarity_parts_u8( a: &[u8], b: &[u8], ) -> Result<scalar::CosineSimilarityPartsU8, ActaError>` | Computes cosine similarity component parts for two `u8` byte slices. | `src/dispatch.rs` |
| `pub fn dot_product_distance(a: &[f32], b: &[f32]) -> Result<f32, ActaError>` | Computes dot product distance ($- \sum a_i b_i$) between two `f32` vectors. | `src/dispatch.rs` |
| `pub fn dot_product_distance_f32_bytes(a: &[f32], b_bytes: &[u8]) -> Result<f32, ActaError>` | Computes dot product distance between an `f32` vector and an LE raw `f32` byte slice. | `src/dispatch.rs` |
| `pub fn dot_product_u8(a: &[u8], b: &[u8]) -> Result<u32, ActaError>` | Computes integer dot product between two `u8` byte slices. | `src/dispatch.rs` |
| `pub fn euclidean_distance(a: &[f32], b: &[f32]) -> Result<f32, ActaError>` | Computes Euclidean distance between two `f32` vectors. | `src/dispatch.rs` |
| `pub fn euclidean_distance_f32_bytes(a: &[f32], b_bytes: &[u8]) -> Result<f32, ActaError>` | Computes Euclidean distance between an `f32` vector and an LE raw `f32` byte slice. | `src/dispatch.rs` |
| `pub fn euclidean_distance_sq_u8(a: &[u8], b: &[u8]) -> Result<u32, ActaError>` | Computes squared Euclidean distance between two `u8` byte slices. | `src/dispatch.rs` |
| `pub fn get_dispatch_override() -> Option<SimdPathOverride>` | Retrieves the current thread-local SIMD dispatch path override. | `src/dispatch.rs` |
| `pub fn set_dispatch_override(path: Option<SimdPathOverride>)` | Sets thread-local SIMD dispatch path override for testing and verification. | `src/dispatch.rs` |
| `pub fn canonical_reduce_16(lanes: &[f32; 16]) -> f32` | Feste paarweise Reduktion von 16 Teilsummen nach SPEC D-122 (5b). | `src/kernels/scalar.rs` |
| `pub fn cosine_distance_f32_bytes_scalar(a: &[f32], b_bytes: &[u8]) -> f32` | Skalare kanonische Kosinus-Distanz zwischen `f32`-Slice und LE `f32`-Byte-Slice. | `src/kernels/scalar.rs` |
| `pub fn cosine_distance_scalar(a: &[f32], b: &[f32]) -> f32` | Skalare kanonische Kosinus-Distanz auf `f32`-Slices. | `src/kernels/scalar.rs` |
| `pub fn cosine_similarity_parts_u8_scalar(a: &[u8], b: &[u8]) -> CosineSimilarityPartsU8` | Skalare Referenz für Kosinus-Komponenten auf u8. | `src/kernels/scalar.rs` |
| `pub fn dot_product_f32_bytes_scalar(a: &[f32], b_bytes: &[u8]) -> f32` | Skalare kanonische Skalarprodukt-Distanz zwischen `f32`-Slice und LE `f32`-Byte-Slice. | `src/kernels/scalar.rs` |
| `pub fn dot_product_f32_u8(a: &[f32], b: &[u8]) -> f32` | Kompatibilitäts-Funktion für gemischte f32/u8 Skalarprodukte. | `src/kernels/scalar.rs` |
| `pub fn dot_product_scalar(a: &[f32], b: &[f32]) -> f32` | Skalare kanonische Skalarprodukt-Distanz (negiertes Dot Product) auf `f32`-Slices. | `src/kernels/scalar.rs` |
| `pub fn dot_product_u8_scalar(a: &[u8], b: &[u8]) -> u32` | Skalare Referenz für ganzzahliges Skalarprodukt. | `src/kernels/scalar.rs` |
| `pub fn euclidean_distance_f32_bytes_scalar(a: &[f32], b_bytes: &[u8]) -> f32` | Skalare kanonische Euklidische Distanz zwischen `f32`-Slice und LE `f32`-Byte-Slice. | `src/kernels/scalar.rs` |
| `pub fn euclidean_distance_scalar(a: &[f32], b: &[f32]) -> f32` | Skalare kanonische Euklidische Distanz auf `f32`-Slices. | `src/kernels/scalar.rs` |
| `pub fn euclidean_distance_sq_f32_u8(a: &[f32], b: &[u8], alphas: &[f32], mins: &[f32]) -> f32` | Kompatibilitäts-Funktion für dequantisierte quadrierte euklidische Distanz. | `src/kernels/scalar.rs` |
| `pub fn euclidean_distance_sq_u8_scalar(a: &[u8], b: &[u8]) -> u32` | Skalare Referenz für quadrierte euklidische Distanz auf u8. | `src/kernels/scalar.rs` |
| `pub fn normalize_inplace(v: &mut [f32])` | Normalisiert einen `f32`-Vektor in-place auf Einheitslänge. | `src/kernels/scalar.rs` |
| `pub fn compute_distance( a: &[f32], b: &[f32], metric: DistanceMetric, ) -> acta_types::Result<f32>` | Berechnet die Distanz zwischen zwei Vektoren unter Verwendung der angegebenen Metrik inklusive Validierung. 1. Vektor `a` auf Nicht-Endlich… | `src/lib.rs` |
| `pub fn compute_distance_f32_bytes_trusted( a: &[f32], b_bytes: &[u8], metric: DistanceMetric, ) -> acta_types::Result<f32>` | Berechnet die Distanz zwischen einem `f32`-Vektor und einem rohen LE-Byte-Puffer (`&[u8]`). 1. Fallunterscheidung nach `DistanceMetric`. 2.… | `src/lib.rs` |
| `pub fn compute_distance_trusted( a: &[f32], b: &[f32], metric: DistanceMetric, ) -> acta_types::Result<f32>` | Berechnet die Distanz zwischen zwei bereits validierten `f32`-Vektoren ohne erneute NaN-Prüfung. 1. Fallunterscheidung anhand der `Distance… | `src/lib.rs` |
| `pub fn distance(metric: DistanceMetric, a: &[f32], b: &[f32]) -> acta_types::Result<f32>` | Berechnet die Distanz zwischen zwei Vektoren inklusive Validierung (SPEC §8.4 / SI-05). | `src/lib.rs` |
| `pub fn distance_prevalidated( metric: DistanceMetric, a: &[f32], b: &[f32], ) -> acta_types::Result<f32>` | Berechnet die Distanz zwischen zwei bereits vorvalidierten Vektoren (SPEC §8.4 / SI-05). | `src/lib.rs` |
| `pub fn validate_vector(vec: &[f32]) -> acta_types::Result<()>` | Prüft, ob ein Vektor gültige endliche Zahlen enthält und eine zulässige Dimension besitzt (1..=4096, D-87). 1. Prüfung der Vektorlänge: Wen… | `src/lib.rs` |


---

## `acta-crypto`

| Attribut | Wert |
|---|---|
| **Ring** | 0 — Ring 0 — Foundation & Pure Numerics/Types |
| **Status** | In Migration |
| **Zweck** | AES-256-GCM-SIV, WAL-HMAC-Integritätsketten, Zeroize und Deletion-Proofs. |
| **Unsafe-Code** | Nein (`#![forbid(unsafe_code)]`) |
| **Interne Abhängigkeiten** | `acta-ports`, `acta-types` |
| **Externe Abhängigkeiten (Auswahl)** | `aes-gcm-siv`, `argon2`, `blake3`, `ed25519-dalek`, `hkdf`, `hmac`, `rand`, `sha2`, `subtle`, `thiserror`, `zeroize` |

### Modul- und Re-Export-Oberfläche (lib.rs)

```rust
pub mod aead;
pub mod chain;
pub mod dek;
pub mod error;
pub mod header;
pub mod hierarchy;
pub mod kdf;
pub mod labels;
pub mod secret;
pub mod sign;
pub mod tag;
pub use aead::{open_record, seal_record, unwrap_master, wrap_master, Block, BlockSealer, SlotAad};
pub use chain::{ chain_next, ledger_mac, verify_chain_complete, verify_wal_chain_completeness, wal_mac, };
pub use dek::{derive_dek, AncestorSalts, Dek, DocTag, Salt};
pub use error::{CryptoError, Result};
pub use header::{ Header, HeaderSlot, HEADER_ALG_IDS, HEADER_FORMAT_VERSION, HEADER_KIND, HEADER_MAGIC, HEADER_SIZE, };
pub use hierarchy::{ CommitKey, EncKey, FileKey, FileKind, HdrKey, IdxKey, KeyHierarchy, LedgerKey, MasterKey, SignSeed, UnitKey, VaultMacKey, WalKey, };
pub use kdf::{ derive_kek, hkdf_expand, KdfParams, MAX_M_KIB, MAX_P, MAX_T, MIN_M_KIB, MIN_P, MIN_PASSPHRASE_LEN, MIN_T, };
pub use labels::{ COLL_TAG, COMMIT, DEK, DEK_SALT, DOC_TAG, ENC, FILE, FKMASK, HDR, IDX, LEDGER, NAME_TAG, REC_AAD, REPORT_PREFIX, SIGN, SLOT_AAD, SUBJECT_TAG, TBIND_AAD, TENANT_TAG, TERM_TAG, UNIT, VAULT_MAC, WAL, };
pub use secret::{Secret, SecretKey};
pub use sign::{sign_report, verify_report, DerivedReportSigner};
pub use tag::{tag, TagPreimage};
```

### Schnittstellen — Public Traits (1)

**`NonceSource`** — *Sealed record envelope according to F.6. Trait for injecting nonces during tests, defaulting to OS CSPRNG.* (`src/aead.rs`)

```rust
pub trait NonceSource {
fn fill_nonce(&mut self, dest: &mut [u8; 12]);
}
```

### Öffentliche Datentypen — Structs (25)

| Struct | Felder / Form | Beschreibung | Datei |
|---|---|---|---|
| `AncestorSalts` | `{ pub t: Salt, pub c: Salt, pub s: Option<Salt>, pub d: Salt, }` | — | `src/dek.rs` |
| `Block` | `{ pub ciphertext: Vec<u8>, pub tag: [u8; 16], }` | Block data representation. | `src/aead.rs` |
| `BlockSealer` | `{ cipher: Aes256GcmSiv, kind: FileKind, file_id: [u8; 16], prev: Option<[u8; 32]>, next_index: u64, }` | Block-AEAD sealer per D-60 and F.1. | `src/aead.rs` |
| `CommitKey` | `(tuple/unit)` | — | `src/hierarchy.rs` |
| `Dek` | `(tuple/unit)` | — | `src/dek.rs` |
| `DerivedReportSigner` | `{ signing_key: SigningKey, pub_fingerprint: String, }` | Standard internal report signer using derived `K_sign` (SPEC §8.2, §11.7, D-80, D-130, F.10). | `src/sign.rs` |
| `EncKey` | `(tuple/unit)` | — | `src/hierarchy.rs` |
| `FileKey` | `(tuple/unit)` | — | `src/hierarchy.rs` |
| `HdrKey` | `(tuple/unit)` | — | `src/hierarchy.rs` |
| `Header` | `{ pub format_version: u16, pub kind: u8, pub alg_ids: [u8; 4], pub store_id: [u8; 16], pub generation: u64, pub slot_count: u8, pub report_pub: [u8; 32], pub slo… }` | Parsed HEADER representation. | `src/header.rs` |
| `HeaderSlot` | `{ pub slot_id: u8, pub anchor_id: u8, pub kdf_id: u8, pub status: u8, // 0 = empty, 1 = active, 2 = removed pub m_kib: u32, pub t: u8, pub p: u8, pub slot_salt:… }` | Slot representation inside HEADER (128 bytes). | `src/header.rs` |
| `IdxKey` | `(tuple/unit)` | — | `src/hierarchy.rs` |
| `KdfParams` | `{ pub m_kib: u32, pub t: u8, pub p: u8, }` | Argon2id parameters. | `src/kdf.rs` |
| `KeyHierarchy` | `{ enc: EncKey, idx: IdxKey, unit: UnitKey, sign_seed: SignSeed, wal: WalKey, ledger: LedgerKey, hdr: HdrKey, master: MasterKey, }` | — | `src/hierarchy.rs` |
| `LedgerKey` | `(tuple/unit)` | — | `src/hierarchy.rs` |
| `MasterKey` | `(tuple/unit)` | Master key wrapper (ZeroizeOnDrop). | `src/hierarchy.rs` |
| `OsCsprng` | `(tuple/unit)` | — | `src/aead.rs` |
| `Secret` | `{ bytes: Vec<u8>, }` | Secret passphrase buffer. | `src/secret.rs` |
| `SecretKey` | `{ bytes: [u8; N], }` | Fixed-size secret key that is wiped on drop and never printed. | `src/secret.rs` |
| `SignSeed` | `(tuple/unit)` | — | `src/hierarchy.rs` |
| `SlotAad` | `{ pub store_id: [u8; 16], pub slot_id: u8, pub anchor_id: u8, pub kdf_id: u8, pub m_kib: u32, pub t: u8, pub p: u8, pub slot_salt: [u8; 32], }` | Keyslot AAD structure according to F.2. | `src/aead.rs` |
| `TagPreimage` | `{ parts: Vec<&'a [u8]>, }` | Tag preimage abstraction ensuring length prefixing per CR-11 / SPEC 9.3. Encapsulates byte components for domain-separated keyed BLAKE3 tags. | `src/tag.rs` |
| `UnitKey` | `(tuple/unit)` | — | `src/hierarchy.rs` |
| `VaultMacKey` | `(tuple/unit)` | — | `src/hierarchy.rs` |
| `WalKey` | `(tuple/unit)` | — | `src/hierarchy.rs` |

### Öffentliche Datentypen — Enums (2)

| Enum | Varianten | Beschreibung | Datei |
|---|---|---|---|
| `CryptoError` | `/// Authentication failure (e.g. wrong passphrase, MAC/tag mismatch) #[error("Auth")] Auth, /// Data corruption or malformed header/payload #[error("Corrupt")] Corrupt, /// Configuration error (e.g.…` | Cryptographic error type with closed enum variants and fixed strings. | `src/error.rs` |
| `FileKind` | `Wal = 1, SSTable = 2, Manifest = 3, TextSegment = 4, VectorSnapshot = 5, Journal = 6, GateLog = 7, Ledger = 8, Scope = 9, Vault = 10,` | — | `src/hierarchy.rs` |

### Typ-Aliase (3)

| Alias | Definition | Beschreibung |
|---|---|---|
| `DocTag` | `type DocTag= [u8` | — |
| `Result` | `type Result<T> = std::result::Result<T, CryptoError>` | — |
| `Salt` | `type Salt= [u8` | — |

### Konstanten (36)

| Konstante | Typ | Beschreibung |
|---|---|---|
| `COLL_TAG` | `&[u8]` | — |
| `COMMIT` | `&[u8]` | Commitment label |
| `DEFAULT` | `Self` | Standard parameters per D-68: m = 128 MiB (131072 KiB), t = 3, p = 4. |
| `DEK` | `&[u8]` | — |
| `DEK_SALT` | `&[u8]` | — |
| `DOC_TAG` | `&[u8]` | — |
| `ENC` | `&[u8]` | Key derivation labels |
| `FILE` | `&[u8]` | — |
| `FKMASK` | `&[u8]` | — |
| `HDR` | `&[u8]` | — |
| `HEADER_ALG_IDS` | `[u8` | — |
| `HEADER_FORMAT_VERSION` | `u16` | — |
| `HEADER_KIND` | `u8` | — |
| `HEADER_MAGIC` | `&[u8` | — |
| `HEADER_SIZE` | `usize` | — |
| `IDX` | `&[u8]` | — |
| `LEDGER` | `&[u8]` | — |
| `MAX_M_KIB` | `u32` | — |
| `MAX_P` | `u8` | — |
| `MAX_T` | `u8` | — |
| `MIN_M_KIB` | `u32` | KDF parameter bounds per D-06, D-68, FM-16. |
| `MIN_P` | `u8` | — |
| `MIN_PASSPHRASE_LEN` | `usize` | Minimum passphrase length in bytes per SPEC §5.7. |
| `MIN_T` | `u8` | — |
| `NAME_TAG` | `&[u8]` | — |
| `REC_AAD` | `&[u8]` | AAD labels |
| `REPORT_PREFIX` | `&[u8]` | — |
| `SIGN` | `&[u8]` | — |
| `SLOT_AAD` | `&[u8]` | — |
| `SUBJECT_TAG` | `&[u8]` | — |
| `TBIND_AAD` | `&[u8]` | — |
| `TENANT_TAG` | `&[u8]` | Tag labels |
| `TERM_TAG` | `&[u8]` | — |
| `UNIT` | `&[u8]` | — |
| `VAULT_MAC` | `&[u8]` | — |
| `WAL` | `&[u8]` | — |

### Öffentliche Funktionen (59)

| Signatur | Beschreibung | Datei |
|---|---|---|
| `pub fn header_tag(&self, header48: &[u8; 48]) -> [u8` | — | `src/aead.rs` |
| `pub fn new(key: FileKey, kind: FileKind, file_id: [u8; 16], prev: Option<[u8; 32]>) -> Self` | — | `src/aead.rs` |
| `pub fn open_record( dek: &crate::dek::Dek, key_bytes: &[u8], env: &[u8], ) -> Result<Zeroizing<Vec<u8>>>` | Opens a sealed record envelope according to F.6. | `src/aead.rs` |
| `pub fn seal(&mut self, plain: &[u8]) -> Result<Block>` | — | `src/aead.rs` |
| `pub fn seal_record( dek: &crate::dek::Dek, doc: [u8; 16], key_bytes: &[u8], plain: &[u8], ) -> Vec<u8>` | — | `src/aead.rs` |
| `pub fn seal_record_with_nonce_source<N: NonceSource>( dek: &crate::dek::Dek, doc: [u8; 16], key_bytes: &[u8], plain: &[u8], nonce_source: &mut N, ) -> Vec<u8>` | — | `src/aead.rs` |
| `pub fn set_counter_for_test(&mut self, val: u64)` | — | `src/aead.rs` |
| `pub fn to_bytes(self) -> Vec<u8>` | — | `src/aead.rs` |
| `pub fn unwrap_master( kek: &SecretKey<32>, aad: &SlotAad, nonce: [u8; 12], w: &[u8; 48], ) -> Result<MasterKey>` | Unwraps the master key from a 48-byte ciphertext + tag. Fails with CryptoError::Auth on mismatch. | `src/aead.rs` |
| `pub fn wrap_master(kek: &SecretKey<32>, aad: &SlotAad, nonce: [u8; 12], m: &MasterKey) -> [u8` | Wraps the master key into a 48-byte ciphertext + tag using AES-256-GCM-SIV. | `src/aead.rs` |
| `pub fn chain_next(prev: &[u8; 32], entry: &[u8]) -> [u8` | Advances a SHA-256 hash chain: SHA-256(prev || entry). # Inputs * `prev`: 32-byte previous hash state. * `entry`: Serialized entry bytes to… | `src/chain.rs` |
| `pub fn ledger_mac(k: &LedgerKey, chain: &[u8; 32]) -> [u8` | Computes Ledger MAC: HMAC-SHA256(K_ledger, chain). # Inputs * `k`: Strongly-typed `LedgerKey`. * `chain`: 32-byte cumulative hash chain sta… | `src/chain.rs` |
| `pub fn verify_chain_complete(expected_tail: &[u8; 32], actual_tail: &[u8; 32]) -> Result<()>` | Verifies completeness of a WAL hash/MAC chain against an expected high-water mark or tail. Evaluates equivalence using constant-time compar… | `src/chain.rs` |
| `pub fn verify_wal_chain_completeness(expected: &[u8; 32], actual: &[u8; 32]) -> Result<()>` | Re-export alias for storage recovery verification per SPEC 10.4. | `src/chain.rs` |
| `pub fn wal_mac( k: &WalKey, prev: &[u8; 32], len: u32, seq: u64, ct: &[u8], tag: &[u8; 16], ) -> [u8` | Computes the WAL frame MAC: HMAC-SHA256(K_wal, prev_mac || len || seq || ct || aead_tag). # Inputs * `k`: Strongly-typed `WalKey` derived f… | `src/chain.rs` |
| `pub fn derive_dek(unit: &UnitKey, a: &AncestorSalts, doc: DocTag) -> Dek` | Derives a DEK using AncestorSalts and DocTag. salt = SHA-256("ctx/dek-salt/v2" || salt_T || salt_C || salt_S || salt_D) with salt_S = 0^32… | `src/dek.rs` |
| `pub fn check16(bytes584: &[u8]) -> [u8` | — | `src/header.rs` |
| `pub fn decode(bytes: &[u8; 128]) -> Self` | — | `src/header.rs` |
| `pub fn decode_checked(raw: &[u8; HEADER_SIZE]) -> Result<Self>` | — | `src/header.rs` |
| `pub fn empty() -> Self` | — | `src/header.rs` |
| `pub fn encode(&self, out: &mut [u8; 128])` | — | `src/header.rs` |
| `pub fn encode(&self, hdr_key: &HdrKey) -> [u8` | — | `src/header.rs` |
| `pub fn mac(hdr_key: &HdrKey, bytes584: &[u8]) -> [u8` | — | `src/header.rs` |
| `pub fn verify_mac(&self, hdr_key: &HdrKey, raw: &[u8; HEADER_SIZE]) -> Result<()>` | — | `src/header.rs` |
| `pub fn as_bytes(&self) -> &[u8` | — | `src/hierarchy.rs` |
| `pub fn as_bytes(&self) -> &[u8` | — | `src/hierarchy.rs` |
| `pub fn commit(&self, epoch: u32) -> CommitKey` | — | `src/hierarchy.rs` |
| `pub fn enc(&self) -> &EncKey` | — | `src/hierarchy.rs` |
| `pub fn file_key(&self, kind: FileKind, id: [u8; 16]) -> FileKey` | — | `src/hierarchy.rs` |
| `pub fn fkmask(&self, id: [u8; 16]) -> [u8` | — | `src/hierarchy.rs` |
| `pub fn from_master(m: MasterKey) -> Self` | — | `src/hierarchy.rs` |
| `pub fn generate_random() -> Self` | Generates a new random master key using OsRng. | `src/hierarchy.rs` |
| `pub fn hdr(&self) -> &HdrKey` | — | `src/hierarchy.rs` |
| `pub fn idx(&self) -> &IdxKey` | — | `src/hierarchy.rs` |
| `pub fn ledger(&self) -> &LedgerKey` | — | `src/hierarchy.rs` |
| `pub fn master(&self) -> &MasterKey` | — | `src/hierarchy.rs` |
| `pub fn new(bytes: [u8; 32]) -> Self` | Wraps existing 32 bytes into MasterKey. | `src/hierarchy.rs` |
| `pub fn new(b: [u8; 32]) -> Self` | — | `src/hierarchy.rs` |
| `pub fn sign_seed(&self) -> &SignSeed` | — | `src/hierarchy.rs` |
| `pub fn unit(&self) -> &UnitKey` | — | `src/hierarchy.rs` |
| `pub fn vault_mac(&self) -> VaultMacKey` | — | `src/hierarchy.rs` |
| `pub fn wal(&self) -> &WalKey` | — | `src/hierarchy.rs` |
| `pub fn derive_kek(pass: &Secret, salt: &[u8; 32], params: &KdfParams) -> Result<SecretKey<32>>` | Derives a Key Encryption Key (KEK) using Argon2id. | `src/kdf.rs` |
| `pub fn from_config(m_kib: u32, t: u8, p: u8) -> Result<Self>` | Validate and construct from config input (fails with Config). | `src/kdf.rs` |
| `pub fn from_header(m_kib: u32, t: u8, p: u8) -> Result<Self>` | Validate and construct from header input (fails with Corrupt). | `src/kdf.rs` |
| `pub fn hkdf_expand(prk: &[u8], info: &[u8], out: &mut [u8]) -> Result<()>` | HKDF-SHA256 expand. | `src/kdf.rs` |
| `pub fn new(m_kib: u32, t: u8, p: u8) -> Result<Self>` | Generic new constructor for compatibility (defaults to Config error). | `src/kdf.rs` |
| `pub fn as_bytes(&self) -> &[u8` | Exposes a reference to the secret bytes. | `src/secret.rs` |
| `pub fn as_bytes(&self) -> &[u8]` | Exposes a slice to the secret bytes. | `src/secret.rs` |
| `pub fn from_slice(slice: &[u8]) -> Self` | Creates a new secret from a byte slice. | `src/secret.rs` |
| `pub fn new(bytes: [u8; N]) -> Self` | Creates a new secret key wrapping raw bytes. | `src/secret.rs` |
| `pub fn new(seed: &SignSeed) -> Self` | Creates a new `DerivedReportSigner` from a strongly-typed `SignSeed`. | `src/sign.rs` |
| `pub fn sign_report(seed: &SignSeed, canon: &[u8]) -> [u8` | Signs a canonical report message with prefix ctx/report/v1. # Inputs * `seed`: Strongly-typed `SignSeed` (32 bytes) derived from the master… | `src/sign.rs` |
| `pub fn verify_report(pubkey: &[u8; 32], canon: &[u8], sig: &[u8; 64]) -> Result<()>` | Verifies a canonical report signature against an Ed25519 public key using verify_strict (RFC 8032). | `src/sign.rs` |
| `pub fn verifying_key(&self) -> VerifyingKey` | Exposes verifying key. | `src/sign.rs` |
| `pub fn new() -> Self` | Constructs a new empty tag preimage. | `src/tag.rs` |
| `pub fn push(&mut self, part: &'a [u8]) -> &mut Self` | Appends a byte part to the preimage. | `src/tag.rs` |
| `pub fn tag(k: &IdxKey, pre: &TagPreimage) -> [u8` | Computes a 16-byte BLAKE3 keyed tag. # Inputs * `k`: Strongly-typed `IdxKey` derived from the master key. * `pre`: Length-prefixed `TagPrei… | `src/tag.rs` |
| `pub fn to_bytes(&self) -> Vec<u8>` | Serializes parts with length prefixes into a canonical byte vector. | `src/tag.rs` |


---

## `acta-vector`

| Attribut | Wert |
|---|---|
| **Ring** | 0 — Ring 0 — Foundation & Pure Numerics/Types |
| **Status** | In Migration |
| **Zweck** | HNSW- und DiskANN-Vektorindex-Implementierungen. |
| **Unsafe-Code** | Nein (`#![forbid(unsafe_code)]`) |
| **Interne Abhängigkeiten** | `acta-crypto`, `acta-ports`, `acta-simd`, `acta-types` |
| **Externe Abhängigkeiten (Auswahl)** | `ahash`, `blake3`, `parking_lot`, `rand`, `roaring`, `serde`, `subtle`, `tracing`, `zeroize` |

### Architekturbeschreibung

Acta Index — HNSW vector index with SIMD distance computation.

### Modul- und Re-Export-Oberfläche (lib.rs)

```rust
pub mod diskann;
pub mod distance;
pub mod hnsw;
pub mod persistence;
pub mod quantize;
pub mod acorn;
pub mod candidate_stream;
pub mod compute_pool;
pub mod partial_rebuild;
pub mod quantize_rabitq;
pub use candidate_stream::{VectorCandidateStream, DEFAULT_VECTOR_STREAM_BATCH_SIZE};
pub use compute_pool::ComputePool;
pub use diskann::{DiskAnnConfig, DiskAnnFallbackPolicy, DiskAnnIndex};
pub use hnsw::{ DeletionStats, GhostFreeVectorIndex, HnswConfig, HnswConfigBuilder, HnswIndex, RebuildStatus, };
pub use persistence::{BufferIndex, HnswHeader};
pub use quantize::ScalarQuantizer;
pub use quantize_rabitq::RaBitQQuantizer;
```

### Feature-Flags

| Flag | Aktiviert |
|---|---|
| `default` | `[]` |
| `docid-128` | `["acta-types/docid-128"]` |
| `experimental-diskann` | `[]` |
| `experimental-rabitq` | `[]` |
| `experimental-predicate-augmented-search` | `[]` |
| `graph` | `[]` |
| `partial-index-rebuild` | `[]` |

### Schnittstellen — Public Traits (2)

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

### Öffentliche Datentypen — Structs (35)

| Struct | Felder / Form | Beschreibung | Datei |
|---|---|---|---|
| `AdaptiveEfPolicy` | `{ /// Initial search depth ef for layer 0. pub min_ef: usize, /// Maximum search depth ef limit. pub max_ef: usize, /// Growth factor for increasing ef in each e… }` | Configuration policy for adaptive ef_search (Ada-ef). | `src/hnsw/adaptive_ef.rs` |
| `AdaptiveEfStateMachine` | `{ policy: AdaptiveEfPolicy, k: usize, current_ef: usize, previous_top_k: Vec<DocId>, stable_rounds: usize, round_count: usize, is_terminated: bool, converged: bo… }` | Deterministic state machine managing Ada-ef expansion rounds. | `src/hnsw/adaptive_ef.rs` |
| `AdaptiveEfStats` | `{ /// Final ef depth reached at termination. pub final_ef: usize, /// Total expansion rounds executed. pub rounds: usize, /// Whether search terminated due to ca… }` | Execution statistics for an adaptive search run. | `src/hnsw/adaptive_ef.rs` |
| `BacklinkTable` | `{ pub map: AHashMap<BacklinkKey, Vec<u32>>, }` | Thread-safe $O(1)$ backlink connection table using `AHashMap`. | `src/hnsw/arena.rs` |
| `BatchContext` | `{ pub running_max_layer: usize, pub has_entry_point: bool, pub has_ram_entry_point: bool, pub backlink_map: BacklinkTable, }` | Batch tracking context for running state across multi-operation transaction commits. | `src/hnsw/batch.rs` |
| `BufferIndex` | `{ pub buffer: std::sync::Arc<[u8]>, pub header: HnswHeader, pub path: Option<std::path::PathBuf>, }` | A reader for buffered HNSW indices. | `src/persistence/buffer.rs` |
| `Candidate` | `{ pub index: usize, pub distance: f32, }` | Search candidate. | `src/hnsw/types.rs` |
| `ComputePool` | `{ inner: Arc<PoolState>, }` | Bounded compute thread pool for background index rebuilds and heavy CPU tasks. Enforces a hard ceiling on concurrent worker threads to prevent thread… | `src/compute_pool.rs` |
| `DeletionStats` | `{ /// Document ID that was deleted. pub doc_id: DocId, /// Number of neighborhood edges repaired (backlinks removed/replaced). pub repaired_edges: usize, /// Num… }` | Statistics returned after a ghost-free node deletion and neighborhood repair. | `src/hnsw/deletion.rs` |
| `DiskAnnConfig` | `{ /// Path to the on-disk index file. pub index_path: PathBuf, /// Vector dimension. pub dimension: usize, /// Maximum graph degree (R). pub max_degree: usize, /… }` | Configuration for DiskANN index. | `src/diskann/config.rs` |
| `DiskAnnIndex` | `{ pub(crate) inner: Arc<DiskAnnIndexInner>, }` | DiskANN out-of-core vector index. This index uses `acta_ports::Vfs` to read vector data and graph edges from disk. | `src/diskann/types.rs` |
| `HnswArena` | `{ pub arena: RwLock<Vec<u32>>, pub offsets: RwLock<Vec<usize>>, pub capacities: RwLock<Vec<usize>>, pub count_offsets: RwLock<Vec<usize>>, pub counts: RwLock<Vec… }` | Contiguous 64-byte aligned arena allocator for HNSW node connections with free-list slot reuse. | `src/hnsw/arena.rs` |
| `HnswColdCore` | `{ pub config: HnswConfig, pub validation_error: Option<String>, pub tx_buffer: LocalTxBuffer, pub quantizer: RwLock<Option<crate::quantize::ScalarQuantizer>>, pu… }` | — | `src/hnsw/types.rs` |
| `HnswConfig` | `{ /// Vector dimensionality. pub dimension: usize, /// Maximum number of elements. pub max_elements: usize, /// Number of connections per element (M parameter).… }` | — | `src/hnsw/config.rs` |
| `HnswConfigBuilder` | `{ config: HnswConfig, }` | Builder for HnswConfig with resource limit enforcements to prevent OOM. | `src/hnsw/config.rs` |
| `HnswHeader` | `{ magic: u32, version: u16, dimension: u32, m: u32, metric: u8, quantized: u8, q_min: f32, // Legacy v1 field q_max: f32, // Legacy v1 field node_count: u64, ent… }` | The header of an HNSW persistent file. | `src/persistence/header.rs` |
| `HnswHotCore` | `{ pub nodes: RwLock<Vec<HnswNode>>, #[cfg(not(feature = "docid-128"))] pub doc_to_node: RwLock<AHashMap<u64, usize>>, #[cfg(feature = "docid-128")] pub doc_to_no… }` | — | `src/hnsw/types.rs` |
| `HnswIndex` | `{ pub inner: std::sync::Arc<HnswIndexCore>, }` | The HNSW (Hierarchical Navigable Small World) vector index. | `src/hnsw/types.rs` |
| `HnswIndexCore` | `{ pub hot: HnswHotCore, pub cold: HnswColdCore, }` | The core implementation of the HNSW index. | `src/hnsw/types.rs` |
| `HnswNode` | `{ pub doc_id: DocId, pub vector: VectorData, pub max_layer: usize, pub committed_tx: u64, }` | A node in the HNSW graph. | `src/hnsw/types.rs` |
| `LocalTxBuffer` | `{ shards: RwLock<AHashMap<TxId, Vec<LocalVectorOp>>>, }` | Indexlokaler Transaktions-Puffer. | `src/hnsw/staging.rs` |
| `NaiveReferenceIndex` | `{ vectors: Vec<(DocId, Vec<f32>)>, metric: DistanceMetric, }` | Naive reference index performing brute-force search over all stored points. **NICHT für Produktion — O(n) Referenzimplementierung** Serves exclusivel… | `src/acorn/naive_reference.rs` |
| `NeighborBacklink` | `{ pub neighbor_ram_idx: usize, pub layer: usize, pub updated_connections: Vec<u32>, }` | Back-link connection update for a neighbor node at a specific layer. | `src/hnsw/batch.rs` |
| `NodeRecord` | `{ #[cfg(not(feature = "docid-128"))] pub doc_id: u64, #[cfg(feature = "docid-128")] pub doc_id: u128, pub max_layer: u8, pub vector_offset: u64, pub connections_… }` | Represents a node's metadata in the flat file. | `src/persistence/node.rs` |
| `PartialRebuildConfig` | `{ /// Kritische Übersättigungs-Schwelle θ_c. Default: 3.0 (3× globale Dichte). pub critical_ratio: f32, /// Ringpuffer-Größe N für Traversal-Statistik. Default:… }` | Konfiguration für den Partial-Rebuild-Trigger. | `src/partial_rebuild.rs` |
| `PreparedInsert` | `{ pub doc_id: DocId, pub vector_data: VectorData, pub new_layer: usize, pub new_idx: usize, pub final_connections: Vec<Vec<u32>>, pub neighbor_backlinks: Vec<Nei… }` | — | `src/hnsw/batch.rs` |
| `RaBitQQuantizer` | `{ dimension: usize, /// Row-major orthogonal rotation matrix R of size dimension x dimension. rotation_matrix: Vec<f32>, }` | RaBitQ Quantizer with random orthogonal rotation and scalar metadata. | `src/quantize_rabitq.rs` |
| `RebuildGuard` | `(tuple/unit)` | — | `src/hnsw/types.rs` |
| `ScalarQuantizer` | `{ pub(crate) mins: Vec<f32>, pub(crate) maxes: Vec<f32>, pub(crate) scales: Vec<f32>, pub(crate) inv_scales: Vec<f32>, pub(crate) dimension: usize, #[serde(skip,… }` | An 8-bit Scalar Quantizer (SQ8) with per-dimension scaling. Quantization reduces the memory footprint of vector storage by 4x. Per-dimension scaling… | `src/quantize/mod.rs` |
| `Sq8Bias` | `{ /// Mean quantization error across distance computations. pub mean_bias: f32, /// Variance of quantization error across distance computations. pub variance_bia… }` | Measured quantization bias and variance statistics for SQ8 distance calculations. | `src/hnsw/sq8_bias.rs` |
| `TaskHandle` | `{ rx: Receiver<T>, }` | Handle to await the outcome of a task submitted to [`ComputePool`]. | `src/compute_pool.rs` |
| `TestVfs` | `{ state: Arc<RwLock<InMemoryFs>>, }` | A test VFS implementation that supports in-memory files and local OS test files. | `src/diskann/test_vfs.rs` |
| `TraversalTracker` | `{ config: PartialRebuildConfig, visited_ring: VecDeque<Vec<u64>>, // Ein Vec<u64> pro Traversierungsaufruf }` | Ringpuffer für besuchte HNSW-Knoten-IDs während ef_search-Traversierungen. | `src/partial_rebuild.rs` |
| `VectorCandidateStream` | `{ index: &'a V, query: Vec<f32>, seq_no: Option<u64>, batch_size: usize, buffer: VecDeque<ScoredDocument>, seen: HashSet<DocId>, fetched_k: usize, yielded: usize… }` | On-demand loading async pull-stream for vector search candidates. # Candidate Ordering & Determinism Vector search scores represent relevance (higher… | `src/candidate_stream.rs` |
| `VectorSnapshotHeader` | `{ pub dimension: u16, pub metric: u8, pub m: u16, pub ef_construction: u16, pub slot_count: u32, pub entry_slot: u32, pub max_level: u8, pub built_through_seq: u… }` | Parsed F.8 Vector Snapshot Header. | `src/persistence/snapshot.rs` |

### Öffentliche Datentypen — Enums (5)

| Enum | Varianten | Beschreibung | Datei |
|---|---|---|---|
| `AcornError` | `/// Dimension mismatch between query and vector database points. DimensionMismatch { /// Expected dimension. expected: usize, /// Provided query dimension. got: usize, }, /// Invalid search parameter…` | Errors occurring during ACORN search or graph traversal operations. | `src/acorn/mod.rs` |
| `DiskAnnFallbackPolicy` | `/// Transparently fall back to in-memory HNSW index on load failure or detected corruption. #[default] UseHnswOnFailure, /// Immediately return a typed error on load failure or index corruption. Fail…` | Fallback behavior policy when DiskANN loading, integrity checks, or reads fail. | `src/diskann/config.rs` |
| `LocalVectorOp` | `/// Einfügen eines Vektors. Insert { doc_id: DocId, data: Vec<f32> }, /// Löschen eines Dokuments. Delete { doc_id: DocId },` | Lokale Vektoroperation im Staging-Puffer. | `src/hnsw/staging.rs` |
| `RebuildStatus` | `/// Kein Rebuild läuft oder geplant. Idle, /// Rebuild läuft gerade im Hintergrund. Running, /// Rebuild wurde getriggert, startet bald. Pending,` | — | `src/hnsw/types.rs` |
| `VectorData` | `/// Standard 32-bit floating point vectors. F32(Vec<f32>), /// 8-bit quantized vectors (SQ8). U8(Vec<u8>),` | — | `src/hnsw/types.rs` |

### Typ-Aliase (2)

| Alias | Definition | Beschreibung |
|---|---|---|
| `BacklinkKey` | `type BacklinkKey= (usize, usize)` | Composite key (neighbor_ram_idx, layer) for backlink lookups. |
| `InMemoryVfs` | `type InMemoryVfs= TestVfs` | — |

### Konstanten (16)

| Konstante | Typ | Beschreibung |
|---|---|---|
| `ARENA_ALIGNMENT_BYTES` | `usize` | Slot alignment in bytes (64-byte alignment = 16 u32 elements for SIMD cache-line compatibility). |
| `ARENA_ALIGNMENT_U32` | `usize` | Alignment in u32 elements (64 bytes / 4 bytes = 16 u32 elements). |
| `DEFAULT_LAYER_SEED` | `u64` | Default-Seed für die deterministische HNSW-Ebenenzuweisung (wie in hnswlib üblich). |
| `DEFAULT_P_HIGH` | `f32` | Default high percentile for quantization scaling (99.5%). |
| `DEFAULT_P_LOW` | `f32` | Default low percentile for quantization scaling (0.5%). |
| `DEFAULT_VECTOR_STREAM_BATCH_SIZE` | `usize` | Default batch size for vector candidate retrieval. |
| `HNSW_MAGIC` | `u32` | Magic number for HNSW files (0x484E5357 = "HNSW"). |
| `HNSW_REBUILD_DELETION_RATIO` | `f64` | Standard-Löschanteil (0.10 = 10 % gelöschte Knoten), ab dem ein Rebuild getriggert wird. |
| `HNSW_VERSION` | `u16` | Current file format version. |
| `SENTINEL_NO_ENTRY_POINT` | `u32` | Sentinel-Wert für ungültigen / nicht gesetzten Entry-Point in HNSW. |
| `SIZE` | `usize` | — |
| `SIZE` | `usize` | — |
| `SIZE` | `usize` | — |
| `SNAPSHOT_FORMAT_VERSION` | `u16` | Format version for v1. |
| `SNAPSHOT_KIND_VECTOR` | `u8` | Format F.8 kind identifier for vector snapshots. |
| `SNAPSHOT_MAGIC` | `&[u8` | Magic constant for Format F.1 / F.8 files. |

### Öffentliche Funktionen (202)

| Signatur | Beschreibung | Datei |
|---|---|---|
| `pub fn compute_gamma_edge_budget(base_degree: usize, predicate_selectivity: f32) -> usize` | Computes the augmented edge budget factor $\gamma$ for graph expansion in ACORN search. Based on Patel et al. 2024 (ACORN: Performant and P… | `src/acorn/gamma_augmentation.rs` |
| `pub fn from_vectors(vectors: Vec<(DocId, Vec<f32>)>) -> Self` | Creates a `NaiveReferenceIndex` pre-populated with vectors using Euclidean distance. | `src/acorn/naive_reference.rs` |
| `pub fn insert(&mut self, id: DocId, vector: Vec<f32>)` | Inserts a document vector into the index. | `src/acorn/naive_reference.rs` |
| `pub fn new(metric: DistanceMetric) -> Self` | Creates a new empty `NaiveReferenceIndex` with the specified distance metric. | `src/acorn/naive_reference.rs` |
| `pub fn fetched_depth(&self) -> usize` | Returns the last $k$ search depth requested from the vector index. | `src/candidate_stream.rs` |
| `pub fn is_exhausted(&self) -> bool` | Returns whether the candidate stream is exhausted. | `src/candidate_stream.rs` |
| `pub fn new(index: &'a V, query: impl Into<Vec<f32>>, seq_no: Option<u64>) -> Self` | Creates a new vector candidate stream for a query and index snapshot. | `src/candidate_stream.rs` |
| `pub async fn next_batch(&mut self) -> Result<Vec<ScoredDocument>>` | Fetches the next batch of candidate documents (up to `batch_size`). Returns an empty `Vec` when exhausted. | `src/candidate_stream.rs` |
| `pub async fn next_doc(&mut self) -> Result<Option<DocId>>` | Fetches the next single candidate document ID. Returns `Ok(None)` when exhausted. | `src/candidate_stream.rs` |
| `pub fn with_batch_size(mut self, n: usize) -> Self` | Sets a custom batch size (minimum 1). | `src/candidate_stream.rs` |
| `pub fn yielded(&self) -> usize` | Returns the total number of items yielded so far. | `src/candidate_stream.rs` |
| `pub fn active_jobs(&self) -> usize` | Returns the number of jobs currently executing in worker threads. | `src/compute_pool.rs` |
| `pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static,` | Submits a fire-and-forget closure for execution on the pool. | `src/compute_pool.rs` |
| `pub fn join(self) -> Result<T, std::sync::mpsc::RecvError>` | Waits for the task to complete and returns its result, or an error if the task failed or was dropped. | `src/compute_pool.rs` |
| `pub fn max_workers(&self) -> usize` | Returns the maximum worker capacity of this pool. | `src/compute_pool.rs` |
| `pub fn new(max_workers: usize) -> Self` | Creates a new [`ComputePool`] with a fixed worker thread ceiling. | `src/compute_pool.rs` |
| `pub fn spawn<F, T>(&self, f: F) -> TaskHandle<T> where F: FnOnce() -> T + Send + 'static, T: Send + 'static,` | Submits a function and returns a [`TaskHandle`] to wait for its return value. | `src/compute_pool.rs` |
| `pub async fn build(&self, vectors: &[Vec<f32>], ids: &[DocId]) -> Result<()>` | Builds the index from a set of vectors. | `src/diskann/build.rs` |
| `pub fn build_sync(&self, vectors: &[Vec<f32>], ids: &[DocId]) -> Result<()>` | — | `src/diskann/build.rs` |
| `pub fn build_to_path_sync( &self, target_path: &std::path::Path, vectors: &[Vec<f32>], ids: &[DocId], ) -> Result<()>` | — | `src/diskann/build.rs` |
| `pub fn recover_pending_delta( &self, ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<usize>> + Send + '_>>` | Liest uncommittete Pending-Vektoren aus `pending.wal` und fügt sie in den Index ein. | `src/diskann/build.rs` |
| `pub fn recover_pending_delta_sync(&self) -> Result<usize>` | — | `src/diskann/build.rs` |
| `pub fn new_with_vfs(index_path: PathBuf, vfs: Arc<dyn Vfs>) -> Self` | Erzeugt eine Konfiguration mit injiziertem VFS. | `src/diskann/config.rs` |
| `pub async fn load(&self) -> Result<()>` | Loads the index from the configured path. | `src/diskann/persistence.rs` |
| `pub fn load_sync(&self) -> Result<()>` | — | `src/diskann/persistence.rs` |
| `pub async fn persist_delta(&self) -> Result<()>` | — | `src/diskann/persistence.rs` |
| `pub fn persist_delta_sync(&self) -> Result<()>` | — | `src/diskann/persistence.rs` |
| `pub fn trigger_background_persist_delta(&self)` | — | `src/diskann/persistence.rs` |
| `pub async fn search_predicate_augmented( &self, query: &[f32], k: usize, filter: &(dyn Fn(DocId) -> bool + Send + Sync), gamma: usize, ) -> Result<Vec<ScoredDocument>>` | Executes a predicate-augmented vector search using ACORN 2-hop neighbor expansion during graph traversal. # Parameters - `query`: Query vec… | `src/diskann/predicate_augmented.rs` |
| `pub async fn search_internal(&self, query: &[f32], k: usize) -> Result<Vec<ScoredDocument>>` | — | `src/diskann/search.rs` |
| `pub fn new() -> Self` | — | `src/diskann/test_vfs.rs` |
| `pub fn current_ef(&self) -> usize` | Returns the active `ef_search` for the current expansion round. | `src/hnsw/adaptive_ef.rs` |
| `pub fn is_terminated(&self) -> bool` | Returns whether the state machine has reached a termination condition. | `src/hnsw/adaptive_ef.rs` |
| `pub fn new( min_ef: usize, max_ef: usize, growth_factor: f32, convergence_window: usize, stability_threshold: f32, ) -> Result<Self>` | Validating constructor for `AdaptiveEfPolicy`. | `src/hnsw/adaptive_ef.rs` |
| `pub fn new(policy: AdaptiveEfPolicy, k: usize) -> Self` | Initializes a new state machine for adaptive search with target top-k. | `src/hnsw/adaptive_ef.rs` |
| `pub fn round_count(&self) -> usize` | Returns the number of expansion rounds completed. | `src/hnsw/adaptive_ef.rs` |
| `pub fn stats(&self) -> AdaptiveEfStats` | Returns the current execution statistics. | `src/hnsw/adaptive_ef.rs` |
| `pub fn step(&mut self, current_top_k: &[DocId]) -> bool` | Evaluates current top-k candidate set, updating state and determining if search should terminate. Returns `true` if search should terminate… | `src/hnsw/adaptive_ef.rs` |
| `pub fn validate(&self) -> Result<()>` | Validates policy invariants. | `src/hnsw/adaptive_ef.rs` |
| `pub fn align_capacity(raw_capacity: usize) -> usize` | Rounds up an allocation size in u32s to the nearest 64-byte (16 x u32) boundary. | `src/hnsw/arena.rs` |
| `pub fn allocate_node( &self, max_layer: usize, m: usize, final_connections: &[Vec<u32>], ) -> Result<usize>` | Allocates or reuses a slot for a node's neighbor lists across all its layers. Ensures 64-byte alignment and validates capacity on slot reus… | `src/hnsw/arena.rs` |
| `pub fn clear(&mut self)` | Clears all stored backlinks. | `src/hnsw/arena.rs` |
| `pub fn free_node(&self, ram_idx: usize)` | Marks a node's slot as free for future slot reuse. | `src/hnsw/arena.rs` |
| `pub fn get(&self, neighbor_ram_idx: usize, layer: usize) -> Option<&Vec<u32>>` | Gets connection list for (neighbor_ram_idx, layer) in O(1) time. | `src/hnsw/arena.rs` |
| `pub fn get_ram_node_connections(&self, ram_idx: usize, layer: usize, m: usize) -> Vec<u32>` | Gets connection list for a node at a specific layer without panicking. | `src/hnsw/arena.rs` |
| `pub fn get_vector_slice(&self, ram_idx: usize, dim: usize) -> Option<Vec<f32>>` | Reads vector payload from a contiguous slot within the arena. | `src/hnsw/arena.rs` |
| `pub fn insert(&mut self, neighbor_ram_idx: usize, layer: usize, connections: Vec<u32>)` | Inserts or updates connection list for (neighbor_ram_idx, layer) in O(1) time. | `src/hnsw/arena.rs` |
| `pub fn layer_offset(node_offset: usize, layer: usize, m: usize) -> usize` | Calculates layer offset within a node's allocated arena block. | `src/hnsw/arena.rs` |
| `pub fn new() -> Self` | Creates a new `BacklinkTable`. | `src/hnsw/arena.rs` |
| `pub fn new() -> Self` | Creates a new `HnswArena`. | `src/hnsw/arena.rs` |
| `pub fn store_vector(&self, ram_idx: usize, dim: usize, vec: &[f32])` | Zeroizes connection memory for a node slot across all layers, clears counts, and then marks the node's slot as free in the free_list. Store… | `src/hnsw/arena.rs` |
| `pub fn try_relink_pruned_neighbors( &self, ram_idx: usize, layer: usize, m: usize, kept_neighbors: &[u32], ) -> bool` | Non-blocking relinking for lazy neighbor pruning during search traversal. | `src/hnsw/arena.rs` |
| `pub fn update_backlink( &self, ram_idx: usize, layer: usize, m: usize, updated_connections: &[u32], ) -> Result<()>` | Updates connection list for a backlink update safely. | `src/hnsw/arena.rs` |
| `pub fn zeroize_and_free_node(&self, ram_idx: usize, max_layer: usize, m: usize)` | — | `src/hnsw/arena.rs` |
| `pub fn zeroize_vector(&self, ram_idx: usize, dim: usize)` | Zeroizes vector payload in the arena for a freed slot. | `src/hnsw/arena.rs` |
| `pub fn new(core: &HnswIndexCore) -> Self` | — | `src/hnsw/batch.rs` |
| `pub fn backoff_alert_threshold(mut self, threshold: u32) -> Self` | Sets the threshold of consecutive rapid rebuilds that trigger warning alerts. | `src/hnsw/config.rs` |
| `pub fn backoff_base_delay(mut self, delay: std::time::Duration) -> Self` | Sets the base delay for rebuild exponential backoff cooldown. | `src/hnsw/config.rs` |
| `pub fn backoff_max_delay(mut self, delay: std::time::Duration) -> Self` | Sets the maximum cap for rebuild exponential backoff cooldown. | `src/hnsw/config.rs` |
| `pub fn build(self) -> Result<HnswConfig>` | Build the configuration after validating bounds. | `src/hnsw/config.rs` |
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
| `pub fn validate(&self) -> Result<()>` | Validates that the configuration parameters are within acceptable bounds. | `src/hnsw/config.rs` |
| `pub fn apply_insert(&self, prepared: PreparedInsert, tx_id: u64)` | — | `src/hnsw/core_insert.rs` |
| `pub fn check_connectivity(&self) -> acta_types::Result<()>` | — | `src/hnsw/core_insert.rs` |
| `pub fn compute_insert(&self, id: DocId, vector: &[f32]) -> Result<PreparedInsert>` | — | `src/hnsw/core_insert.rs` |
| `pub fn compute_insert_with_context( &self, id: DocId, vector: &[f32], offset: usize, prior_prepared: &mut [PreparedInsert], batch_ctx: &mut BatchContext, ) -> Result<PreparedInsert>` | — | `src/hnsw/core_insert.rs` |
| `pub fn connectivity_score(&self) -> f64` | — | `src/hnsw/core_insert.rs` |
| `pub fn deleted_ratio(&self) -> f64` | — | `src/hnsw/core_rebuild.rs` |
| `pub fn deterministic_layer_for_doc(doc_raw: u64, chunk: u32) -> usize` | Assigns a random layer for node insertion using geometric distribution. Governance Rule P28 Compliance: Uhrzeit und Zufall im Produktionsco… | `src/hnsw/core_rebuild.rs` |
| `pub fn is_rebuild_required(&self) -> bool` | — | `src/hnsw/core_rebuild.rs` |
| `pub async fn rebuild(&self) -> Result<()>` | — | `src/hnsw/core_rebuild.rs` |
| `pub fn rebuild_count(&self) -> u64` | — | `src/hnsw/core_rebuild.rs` |
| `pub async fn rebuild_region(&self, region_node_ids: Vec<u64>) -> Result<()>` | — | `src/hnsw/core_rebuild.rs` |
| `pub fn rebuild_region_sync(&self, region_node_ids: Vec<u64>) -> Result<()>` | — | `src/hnsw/core_rebuild.rs` |
| `pub fn rebuild_status(&self) -> RebuildStatus` | — | `src/hnsw/core_rebuild.rs` |
| `pub fn rebuild_sync(&self) -> Result<()>` | — | `src/hnsw/core_rebuild.rs` |
| `pub fn rebuild_sync_with_clock(&self, clock: &dyn acta_ports::Clock) -> Result<()>` | — | `src/hnsw/core_rebuild.rs` |
| `pub async fn rebuild_with_clock(&self, clock: &dyn acta_ports::Clock) -> Result<()>` | — | `src/hnsw/core_rebuild.rs` |
| `pub fn visited_dead_nodes(&self) -> u64` | — | `src/hnsw/core_rebuild.rs` |
| `pub async fn wait_for_rebuild(&self) -> bool` | — | `src/hnsw/core_rebuild.rs` |
| `pub async fn wait_for_rebuild_with_clock(&self, clock: &dyn acta_ports::Clock) -> bool` | — | `src/hnsw/core_rebuild.rs` |
| `pub async fn wait_for_rebuild_with_timeout(&self, timeout: std::time::Duration) -> bool` | — | `src/hnsw/core_rebuild.rs` |
| `pub async fn wait_for_rebuild_with_timeout_and_clock( &self, timeout: std::time::Duration, clock: &dyn acta_ports::Clock, ) -> bool` | — | `src/hnsw/core_rebuild.rs` |
| `pub async fn search_adaptive( &self, query: &[f32], k: usize, policy: &AdaptiveEfPolicy, ) -> Result<Vec<ScoredDocument>>` | Performs an adaptive search (Ada-ef) on the HNSW index, dynamically adjusting search depth `ef`. | `src/hnsw/core_search.rs` |
| `pub async fn search_adaptive_filtered( &self, query: &[f32], k: usize, policy: &AdaptiveEfPolicy, filter: Option<&(dyn Fn(DocId) -> bool + Send + Sync)>, ) -> Result<Vec<ScoredDocument>>` | Performs an adaptive search (Ada-ef) with custom document filter predicate. | `src/hnsw/core_search.rs` |
| `pub async fn search_adaptive_with_stats( &self, query: &[f32], k: usize, policy: &AdaptiveEfPolicy, filter: Option<&(dyn Fn(DocId) -> bool + Send + Sync)>, ) -> Result<(Vec<ScoredDocument>, AdaptiveEfStats)>` | Performs an adaptive search (Ada-ef) returning scored documents along with execution statistics. | `src/hnsw/core_search.rs` |
| `pub fn calibrate( batch: &[&[f32]], quantizer: &crate::quantize::ScalarQuantizer, metric: acta_types::DistanceMetric, ) -> Self` | Calibrates quantization error statistics across a representative batch of vectors. | `src/hnsw/sq8_bias.rs` |
| `pub fn new(mean_bias: f32, variance_bias: f32, sample_count: u64) -> Self` | Creates a new `Sq8Bias` instance with calibrated parameters. | `src/hnsw/sq8_bias.rs` |
| `pub fn discard(&self, tx: TxId)` | Verwirft alle gestagten Operationen einer Transaktion (z. B. bei Rollback). | `src/hnsw/staging.rs` |
| `pub fn drain(&self, tx: TxId) -> Vec<LocalVectorOp>` | Entnimmt alle gestagten Operationen einer Transaktion. | `src/hnsw/staging.rs` |
| `pub fn new() -> Self` | Erzeugt einen neuen leeren Staging-Puffer. | `src/hnsw/staging.rs` |
| `pub fn stage(&self, tx: TxId, op: LocalVectorOp) -> Result<()>` | Nimmt eine Operation für die angegebene Transaktion auf. | `src/hnsw/staging.rs` |
| `pub fn all_doc_ids_and_layers(&self) -> Vec<(DocId, usize)>` | Returns all non-deleted document IDs and their assigned HNSW max layers. | `src/hnsw/types.rs` |
| `pub fn all_doc_ids_from_map(&self) -> Vec<DocId>` | — | `src/hnsw/types.rs` |
| `pub fn check_and_trigger_partial_rebuild(&self)` | — | `src/hnsw/types.rs` |
| `pub fn check_connectivity(&self) -> acta_types::Result<()>` | — | `src/hnsw/types.rs` |
| `pub fn connectivity_score(&self) -> f64` | — | `src/hnsw/types.rs` |
| `pub fn deleted_ratio(&self) -> f64` | — | `src/hnsw/types.rs` |
| `pub fn get_entry_point(&self) -> Option<usize>` | — | `src/hnsw/types.rs` |
| `pub fn get_ram_entry_point(&self) -> Option<usize>` | — | `src/hnsw/types.rs` |
| `pub fn get_ram_node_connections(&self, ram_idx: usize, layer: usize, m: usize) -> Vec<u32>` | — | `src/hnsw/types.rs` |
| `pub fn is_rebuild_required(&self) -> bool` | — | `src/hnsw/types.rs` |
| `pub async fn load_buffer(&self, path: impl AsRef<std::path::Path> + Send) -> Result<()>` | — | `src/hnsw/types.rs` |
| `pub fn metrics_sink(&self) -> std::sync::Arc<dyn acta_ports::MetricsSink>` | Returns the metrics sink. | `src/hnsw/types.rs` |
| `pub fn metrics_sink(&self) -> std::sync::Arc<dyn acta_ports::MetricsSink>` | Returns the metrics sink. | `src/hnsw/types.rs` |
| `pub fn new(config: HnswConfig) -> Self` | — | `src/hnsw/types.rs` |
| `pub fn quantizer(&self) -> Option<crate::quantize::ScalarQuantizer>` | Returns a snapshot clone of the current quantizer if trained. | `src/hnsw/types.rs` |
| `pub async fn rebuild(&self) -> Result<()>` | — | `src/hnsw/types.rs` |
| `pub fn rebuild_count(&self) -> u64` | — | `src/hnsw/types.rs` |
| `pub async fn rebuild_region(&self, region_node_ids: Vec<u64>) -> Result<()>` | — | `src/hnsw/types.rs` |
| `pub fn rebuild_status(&self) -> RebuildStatus` | — | `src/hnsw/types.rs` |
| `pub async fn save(&self, path: impl AsRef<std::path::Path>) -> Result<()>` | — | `src/hnsw/types.rs` |
| `pub fn save_sync(&self, path: impl AsRef<std::path::Path>) -> Result<()>` | — | `src/hnsw/types.rs` |
| `pub fn set_entry_point(&self, ep: Option<usize>)` | — | `src/hnsw/types.rs` |
| `pub fn set_fault_injection_insert_target(&self, target: u64)` | Sets the target insertion count for simulating compute_insert fault injection in tests. | `src/hnsw/types.rs` |
| `pub fn set_layer_seed(&self, seed: u64)` | Sets the random seed used for HNSW layer selection during insertions. | `src/hnsw/types.rs` |
| `pub fn set_layer_seed(&self, seed: u64)` | Sets the random seed used for HNSW layer selection during insertions. | `src/hnsw/types.rs` |
| `pub fn set_metrics_sink(&self, sink: std::sync::Arc<dyn acta_ports::MetricsSink>)` | Sets the metrics sink. | `src/hnsw/types.rs` |
| `pub fn set_metrics_sink(&self, sink: std::sync::Arc<dyn acta_ports::MetricsSink>)` | Sets the metrics sink. | `src/hnsw/types.rs` |
| `pub fn set_ram_entry_point(&self, ep: Option<usize>)` | — | `src/hnsw/types.rs` |
| `pub fn sq8_bias(&self) -> Sq8Bias` | Returns the calibrated SQ8 quantization bias statistics. | `src/hnsw/types.rs` |
| `pub fn trigger_rebuild_async(&self)` | — | `src/hnsw/types.rs` |
| `pub fn try_new(config: HnswConfig) -> Result<Self>` | Creates a new HNSW index, validating configuration upfront. Uses an internal, non-cryptographic, reproducible default RNG seeded with `DEFA… | `src/hnsw/types.rs` |
| `pub fn try_new_with_rng( config: HnswConfig, rng: std::sync::Arc<dyn acta_ports::Rng>, ) -> Result<Self>` | Creates a new HNSW index with an explicit random number generator for deterministic testing or custom seeding. | `src/hnsw/types.rs` |
| `pub fn visited_dead_nodes(&self) -> u64` | — | `src/hnsw/types.rs` |
| `pub async fn wait_for_rebuild(&self) -> bool` | — | `src/hnsw/types.rs` |
| `pub async fn wait_for_rebuild_with_timeout(&self, timeout: std::time::Duration) -> bool` | — | `src/hnsw/types.rs` |
| `pub fn with_metrics_sink(self, sink: std::sync::Arc<dyn acta_ports::MetricsSink>) -> Self` | Builder method to set the metrics sink. | `src/hnsw/types.rs` |
| `pub fn find_oversaturated_regions( &self, tombstone_map: &HashMap<u64, bool>, // node_id → is_tombstoned ) -> Vec<u64>` | Berechnet S_local für alle Hot-Path-Regionen. Gibt Knoten-IDs zurück, für die S_local > θ_c gilt. | `src/partial_rebuild.rs` |
| `pub fn hot_path_nodes(&self) -> impl Iterator<Item = u64> + '_` | Gibt die N zuletzt besuchten eindeutigen Knoten-IDs zurück. | `src/partial_rebuild.rs` |
| `pub fn new(config: PartialRebuildConfig) -> Self` | Erstellt einen neuen TraversalTracker mit der gegebenen Konfiguration. | `src/partial_rebuild.rs` |
| `pub fn record_traversal(&mut self, visited_node_ids: Vec<u64>)` | Trägt die in einem ef_search-Lauf besuchten Node-IDs ein. | `src/partial_rebuild.rs` |
| `pub fn should_trigger_partial_rebuild( tracker: &TraversalTracker, tombstone_map: &HashMap<u64, bool>, global_tombstone_ratio: f32, config: &PartialRebuildConfig, ) -> Option<Vec<u64>>` | Entscheidet ob ein lokaler Partial-Rebuild ausgelöst werden soll. Returns `Some(hot_node_ids)` wenn Partial-Rebuild ausgelöst werden soll,… | `src/partial_rebuild.rs` |
| `pub fn get_connections(&self, record: &NodeRecord, layer: usize) -> Result<Vec<u32>>` | — | `src/persistence/buffer.rs` |
| `pub fn get_node_record(&self, index: usize) -> Result<NodeRecord>` | — | `src/persistence/buffer.rs` |
| `pub fn get_vector(&self, record: &NodeRecord) -> Result<&[u8]>` | — | `src/persistence/buffer.rs` |
| `pub fn open(path: impl AsRef<std::path::Path>) -> Result<Self>` | — | `src/persistence/buffer.rs` |
| `pub async fn open_async(path: impl AsRef<std::path::Path> + Send) -> Result<Self>` | Opens an HNSW file without requiring external runtime blocking task spawns. | `src/persistence/buffer.rs` |
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
| `pub fn version(&self) -> u16` | — | `src/persistence/header.rs` |
| `pub fn from_bytes(bytes: &[u8]) -> Result<Self>` | — | `src/persistence/node.rs` |
| `pub fn to_bytes(&self) -> Vec<u8>` | — | `src/persistence/node.rs` |
| `pub fn load_snapshot_vfs<V: Vfs>(vfs: &V, path: &Path) -> Result<VectorSnapshotHeader>` | Reads and validates snapshot header through VFS port. | `src/persistence/snapshot.rs` |
| `pub fn parse(bytes: &[u8]) -> Result<Self>` | Parses snapshot header from raw bytes verifying fail-closed invariants. | `src/persistence/snapshot.rs` |
| `pub fn save_snapshot_vfs<V: Vfs>( vfs: &V, path: &Path, header: &VectorSnapshotHeader, ) -> Result<()>` | Atomically saves snapshot data through VFS port. | `src/persistence/snapshot.rs` |
| `pub fn to_bytes(&self) -> Vec<u8>` | Serializes header to bytes (19 bytes payload after F.1 frame). | `src/persistence/snapshot.rs` |
| `pub fn asymmetric_dist( &self, query: &[f32], quantized: &[u8], metric: DistanceMetric, ) -> acta_types::Result<f32>` | Computes the asymmetric distance between an exact query and a quantized vector. Optimized for zero allocations via inline dequantization. | `src/quantize/mod.rs` |
| `pub fn calculate_vector_drift(&self, vector: &[f32]) -> f32` | Calculates quantization drift for a single vector as the fraction of dimensions falling outside `[mins[i], maxes[i]]`. This is a **pure cal… | `src/quantize/mod.rs` |
| `pub fn check_drift(&self, vector: &[f32]) -> f32` | Legacy pure drift calculation wrapper for [`calculate_vector_drift`](Self::calculate_vector_drift). # Deprecated Renamed to [`calculate_vec… | `src/quantize/mod.rs` |
| `pub fn dequantize(&self, vector: &[u8]) -> acta_types::Result<Vec<f32>>` | Dequantizes a `u8` vector back to `f32`. | `src/quantize/mod.rs` |
| `pub fn dimension(&self) -> usize` | Returns the target dimension of the quantizer. | `src/quantize/mod.rs` |
| `pub fn drift_ratio(&self) -> f32` | Returns the fraction of quantized queries that contained values outside the trained min/max range. This is a **stateful metric** reflecting… | `src/quantize/mod.rs` |
| `pub fn inv_scales(&self) -> &[f32]` | Returns a reference to per-dimension inverse scale values. | `src/quantize/mod.rs` |
| `pub fn is_rebuild_required(&self, threshold: f32) -> bool` | Checks if recalibration / index rebuild is required based on cumulative quantization drift ratio. Requires at least 20 queries to avoid fal… | `src/quantize/mod.rs` |
| `pub fn maxes(&self) -> &[f32]` | Returns a reference to per-dimension maximum values. | `src/quantize/mod.rs` |
| `pub fn mins(&self) -> &[f32]` | Returns a reference to per-dimension minimum values. | `src/quantize/mod.rs` |
| `pub fn quantize(&self, vector: &[f32]) -> acta_types::Result<Vec<u8>>` | Quantizes an `f32` vector to `u8`. | `src/quantize/mod.rs` |
| `pub fn scales(&self) -> &[f32]` | Returns a reference to per-dimension scale values. | `src/quantize/mod.rs` |
| `pub fn symmetric_dist( &self, q1: &[u8], q2: &[u8], metric: DistanceMetric, ) -> acta_types::Result<f32>` | Computes symmetric (approximate) distance purely in u8. Optimized for zero allocations via inline dequantization. | `src/quantize/mod.rs` |
| `pub fn train(batch: &[&[f32]], dimension: usize) -> Self` | Creates a new ScalarQuantizer trained on a batch of vectors to find per-dimension min/max. For long-lived or growing collections, callers s… | `src/quantize/mod.rs` |
| `pub fn train_with_percentiles( batch: &[&[f32]], dimension: usize, p_low: f32, p_high: f32, ) -> Self` | Creates a new ScalarQuantizer trained on a batch of vectors with custom percentile bounds. | `src/quantize/mod.rs` |
| `pub fn try_train(batch: &[&[f32]], dimension: usize) -> acta_types::Result<Self>` | Creates a new ScalarQuantizer trained on a batch of vectors to find per-dimension min/max. For long-lived or growing collections, callers s… | `src/quantize/mod.rs` |
| `pub fn try_train_with_percentiles( batch: &[&[f32]], dimension: usize, p_low: f32, p_high: f32, ) -> acta_types::Result<Self>` | Safely creates a new ScalarQuantizer trained on a batch of vectors using custom percentile clipping bounds. Range `p_low` and `p_high` must… | `src/quantize/mod.rs` |
| `pub fn asymmetric_distance(&self, query: &[f32], code: &[u8]) -> Result<f32>` | Computes the asymmetric Euclidean distance between an unquantized query vector and a quantized bitcode. # Mathematical Derivation & Bias Co… | `src/quantize_rabitq.rs` |
| `pub fn code_length(&self) -> usize` | Returns the expected byte length of quantized codes (`dimension.div_ceil(8) + 8` bytes). | `src/quantize_rabitq.rs` |
| `pub fn dimension(&self) -> usize` | Returns the vector dimension supported by this quantizer. | `src/quantize_rabitq.rs` |
| `pub fn quantize(&self, vector: &[f32]) -> Result<Vec<u8>>` | Quantizes a single $f32$ vector into a RaBitQ packed bitcode buffer. The returned code consists of: - $\lceil D / 8 \rceil$ bytes of packed… | `src/quantize_rabitq.rs` |
| `pub fn try_train(vectors: &[Vec<f32>], dimension: usize) -> Result<Self>` | Safely trains a `RaBitQQuantizer` for vectors of the specified `dimension`. Generates a deterministic orthogonal matrix $R \in \mathbb{R}^{… | `src/quantize_rabitq.rs` |


---

## `acta-text`

| Attribut | Wert |
|---|---|
| **Ring** | 0 — Ring 0 — Foundation & Pure Numerics/Types |
| **Status** | Fertig |
| **Zweck** | BM25/BM25F-Volltextindexierung und deutsche Kompositazerlegung. |
| **Unsafe-Code** | Nein (`#![forbid(unsafe_code)]`) |
| **Interne Abhängigkeiten** | `acta-crypto`, `acta-ports`, `acta-types` |
| **Externe Abhängigkeiten (Auswahl)** | `aes-gcm-siv`, `bincode`, `blake3`, `bytes`, `futures`, `itoa`, `parking_lot`, `regex`, `serde`, `serde_json`, `thiserror`, `tracing`, `unicode-segmentation` |

### Modul- und Re-Export-Oberfläche (lib.rs)

```rust
pub mod analyzer;
pub mod bm25;
pub mod buffer;
pub mod domain;
pub mod erasure;
pub mod inverted;
pub mod member;
pub mod morphology;
pub mod phrase;
pub mod posting_list;
pub mod postings;
pub mod segment;
pub mod stats;
pub mod stream;
pub mod tokenizer;
pub mod wand;
pub use analyzer::{ compute_term_tag, get_analyzer, Analyzer, GermanAnalyzer, UndAnalyzer, UndeterminedAnalyzer, };
pub use buffer::{open_binding_envelope, seal_binding_envelope, TextBufRecord, TextBuffer};
pub use erasure::TextErasureLayer;
pub use member::{compute_member_hash, MembershipBlock};
pub use phrase::match_phrase_positions;
pub use postings::PostingBlock;
pub use segment::{ DictEntry, DocEntry, SegmentHeader, SegmentReader, SegmentWriter, POSTING_BLOCK_SIZE, };
pub use stats::SegmentStats;
pub use bm25::{score_term_bm25f, FieldId, FieldWeight, BM25, BM25F};
pub use domain::{DomainVocabulary, LegalDomainVocabulary, MedicalDomainVocabulary};
pub use inverted::{ BM25MorphIndex, InvertedIndex, Language, ParsedTextTarget, PurgeReport, PurgeStats, ScanTarget, TextPurger, };
pub use morphology::{normalize_umlauts, GermanCompoundSplitter, MorphologicalTokenizer};
pub use posting_list::{Posting, PostingList, ResidentPostingIndex};
pub use stream::{Bm25CandidateStream, DEFAULT_STREAM_BATCH_SIZE};
pub use tokenizer::{DefaultTokenizer, GermanMorphTokenizer, Tokenizer};
```

### Feature-Flags

| Flag | Aktiviert |
|---|---|
| `default` | `[]` |

### Schnittstellen — Public Traits (4)

**`Analyzer`** — *Trait für Text-Analyzer.* (`src/analyzer.rs`)

```rust
pub trait Analyzer : Send + Sync {
fn id(&self) -> u8;
    fn version(&self) -> u16;
    fn lang(&self) -> [u8; 3];
    fn analyze(&self, text: &str) -> Vec<String>;

    fn term_tag(&self, key: &IdxKey, term: &str) -> [u8; 16] {
        let mut pre = TagPreimage::new();
        pre.push(TERM_TAG);
        let id_bytes = [self.id()];
        pre.push(&id_bytes);
        let lang = self.lang();
        pre.push(&lang);
        pre.push(term.as_bytes());
        tag(key, &pre)
    }
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

**`MorphologicalTokenizer`** — *Trait für morphologische Zerlegung von Wörtern in ihre Grundbestandteile. # Eingabevertrag Eingegebene Wörter MÜSSEN vor dem Aufruf von `decompose` in Kleinbuchstaben vorliegen und idealerweise mit [`normalize_umlauts`] vorbereitet sein. Bei Großbuchstaben od…* (`src/morphology/compound_splitter.rs`)

```rust
pub trait MorphologicalTokenizer : Send + Sync {
/// Zerlegt ein Token in morphologische Komponenten (Teilwörter).
    fn decompose<'a>(&self, token: &'a str) -> Vec<&'a str>;

    /// Liefert den Sprachcode (z. B. "de").
    fn language(&self) -> &str;
}
```

**`Tokenizer`** — *Trait für Text-Tokenizer mit unterschiedlichen Filter- oder Zerlegungsstrategien.* (`src/tokenizer.rs`)

```rust
pub trait Tokenizer : Send + Sync {
/// Zerlegt den übergebenen Text in kleingeschriebene Wort-Token und filtert Stoppwörter heraus.
    fn tokenize(&self, text: &str) -> Vec<String>;
}
```

### Öffentliche Datentypen — Structs (40)

| Struct | Felder / Form | Beschreibung | Datei |
|---|---|---|---|
| `BM25` | `{ pub k1: f32, pub b: f32, }` | Konfigurationsparameter für das BM25-Scoring-Modell. - `k1`: Steuert die Sättigung der Termhäufigkeit (TF). Standardwert: 1.5. - `b`: Steuert den Ein… | `src/bm25.rs` |
| `BM25F` | `{ pub k1: f32, pub field_weights: Vec<FieldWeight>, cached_weights: Vec<(FieldId, f32, f32)>, }` | Konfiguration für feldgewichtetes BM25F-Scoring (§11.3). | `src/bm25.rs` |
| `BM25MorphIndex` | `{ inner: InvertedIndex<S>, tokenizer: Arc<dyn MorphologicalTokenizer>, }` | Invertierter Index mit morphologischer Zerlegung für deutsche Volltextsuche. | `src/inverted/morph_index.rs` |
| `Bm25CandidateStream` | `{ index: &'a InvertedIndex<S>, query: String, max_seq: Option<u64>, batch_size: usize, buffer: VecDeque<(DocId, f32)>, seen: BTreeSet<DocId>, fetched_k: usize, y… }` | Asynchroner Pull-Stream zum bedarfsgerechten Nachladen von BM25-Kandidaten. # Kandidaten-Sortierung & Determinismus Ergebnisse werden strikt determin… | `src/stream.rs` |
| `Bm25Scorer` | `{ index: InvertedIndex<S>, }` | Berechnet Begriffsgewichte und wendet die Standard-BM25-Relevanzbewertung an. | `src/lib.rs` |
| `BoundedTopK` | `{ heap: BinaryHeap<Bm25Candidate>, capacity: usize, }` | Begrenzter Min-Heap zur effizienten Verwaltung der K besten Treffer. | `src/wand.rs` |
| `DefaultTokenizer` | `(tuple/unit)` | Standard-Tokenizer basierend auf Unicode-Wortgrenzen und generischen Stoppwörtern. | `src/tokenizer.rs` |
| `DictEntry` | `{ pub term_tag: [u8; 16], pub df: u32, pub posting_off: u64, pub posting_len: u32, }` | Wörterbucheintrag im Segment. | `src/segment.rs` |
| `DocEntry` | `{ pub doc_ord: u32, pub doclen: u32, pub binding_envelope: Vec<u8>, }` | Dokumenttabelleneintrag (D-69, F.7). | `src/segment.rs` |
| `FieldWeight` | `{ pub field_id: FieldId, pub weight: f32, pub b: f32, }` | Feldgewichtungskonfiguration für ein bestimmtes Feld in BM25F. | `src/bm25.rs` |
| `GermanAnalyzer` | `{ splitter: GermanCompoundSplitter, }` | — | `src/analyzer.rs` |
| `GermanCompoundSplitter` | `{ /// Minimum component length for splitting. min_component_len: usize, /// Trie data structure for fast prefix checking and dictionary matching. trie: Trie, }` | Deutscher Komposita-Zerleger basierend auf Dynamischer Programmierung und Fugenanalyse. # Funktionsweise & Algorithmus 1. **Eingebettetes Wörterbuch*… | `src/morphology/compound_splitter.rs` |
| `GermanMorphTokenizer` | `{ splitter: Arc<crate::morphology::GermanCompoundSplitter>, }` | Deutscher Tokenizer mit morphologischer Zerlegung von Wortzusammensetzungen. | `src/tokenizer.rs` |
| `InvertedIndex` | `{ storage: Arc<S>, prefix: Vec<u8>, tokenizer: Arc<dyn Tokenizer>, pub(crate) total_docs: Arc<AtomicU64>, pub(crate) total_tokens: Arc<AtomicU64>, pub(crate) avg… }` | Persistenter invertierter Index über einer Kollektion im Storage-Backend. # Nebenläufigkeit & Sperrhierarchie: 1. `commit_lock` (`parking_lot::Mutex`… | `src/inverted/index_struct.rs` |
| `LegalDomainVocabulary` | `(tuple/unit)` | Deutsches Juristisches Fachvokabular zur Anbindung an [`DomainVocabulary`]. | `src/domain/legal_de.rs` |
| `MedicalDomainVocabulary` | `(tuple/unit)` | Deutsches Medizinisches Fachvokabular zur Anbindung an [`DomainVocabulary`]. | `src/domain/medical_de.rs` |
| `MembershipBlock` | `{ pub members: Vec<u64>, }` | Mitgliedschaftsblock zur schnellen Segmentauflösung bei Löschungen (D-121). | `src/member.rs` |
| `PassthroughTokenizer` | `{ lang: String, }` | Passthrough-Tokenizer für Sprachen, die keine Zerlegung von Zusammensetzungen benötigen. Gibt eingegebene Token unverändert als Einzelkomponente zurü… | `src/morphology/passthrough.rs` |
| `Posting` | `{ /// Dokument-ID des Eintrags. pub doc_id: DocIdRaw, /// Termfrequenz (Häufigkeit des Terms in diesem Dokument). pub tf: u32, /// Gesamtlänge des Dokuments in T… }` | Kompakte Repräsentation eines einzelnen Vorkommens (Posting) im Index. 8-Byte ausgerichtet (`repr(C, align(8))`) für optimale CPU-Cacheline-Ausnutzun… | `src/posting_list.rs` |
| `PostingBlock` | `{ pub max_doc_ord: u32, pub max_tf_norm: u16, pub doc_ords: Vec<u32>, pub tfs: Vec<u32>, pub positions: Option<Vec<Vec<u32>>>, }` | — | `src/postings.rs` |
| `PostingBlockInfo` | `{ /// Höchste Dokument-ID in diesem Block (ermöglicht schnelles Überspringen). pub max_doc_id: DocIdRaw, /// Maximale Termfrequenz innerhalb dieses Blocks. pub m… }` | Metadaten eines 64er-Posting-Blocks zur Berechnung oberer BM25-Schranken in Block-Max WAND. | `src/posting_list.rs` |
| `PostingList` | `{ postings: Vec<Posting>, blocks: Vec<PostingBlockInfo>, }` | Zusammenhängende, nach DocId sortierte Posting-Liste für einen Begriff. | `src/posting_list.rs` |
| `PurgeReport` | `{ /// Number of units successfully purged. pub units_purged: u64, /// Number of units that failed during purge execution. pub units_failed: u64, /// Number of st… }` | Detailed purge execution report capturing purged units, failed units, bytes freed, and errors. | `src/inverted/purge.rs` |
| `PurgeStats` | `{ /// Number of units physically purged. pub purged_units: u64, /// Number of storage bytes freed. pub bytes_freed: u64, }` | Statistics for purged units and storage bytes freed. | `src/inverted/purge.rs` |
| `ResidentPostingIndex` | `{ index: RwLock<BTreeMap<String, Arc<PostingList>>>, }` | Speicherresidenter Lese-Cache für Posting-Listen mit RCU-Verwaltung. | `src/posting_list.rs` |
| `ScanTarget` | `{ /// Target identifier string (e.g. "doc:123" or collection name). pub target_id: String, /// BLAKE3 digest of target. pub target_hash: [u8; 32], }` | Target descriptor for physical purging operations. | `src/inverted/purge.rs` |
| `SegmentHeader` | `{ pub bucket: u8, pub bucket_count: u8, pub analyzer_id: u8, pub analyzer_version: u16, pub lang: [u8; 3], pub flags: u8, pub doc_count: u32, pub term_count: u32… }` | Big-Endian Segmentheader (SPEC Anhang F.7). | `src/segment.rs` |
| `SegmentReader` | `{ pub header: SegmentHeader, pub dictionary: Vec<DictEntry>, pub docs: Vec<DocEntry>, pub postings: Vec<PostingBlock>, }` | SegmentReader zum Entschlüsseln und Durchsuchen von Textsegmenten. | `src/segment.rs` |
| `SegmentStats` | `{ pub doc_count: u32, pub total_len: u64, pub avg_doc_len: f32, }` | — | `src/stats.rs` |
| `SegmentWriter` | `{ pub header: SegmentHeader, pub dictionary: Vec<DictEntry>, pub postings: Vec<PostingBlock>, pub docs: Vec<DocEntry>, }` | SegmentWriter zum Aufbau versiegelter Segmente nach F.1 und F.7. | `src/segment.rs` |
| `TextBufRecord` | `{ pub cluster_tag: [u8; 16], pub doc_tag: [u8; 16], pub text: String, pub doc_ord: u32, }` | — | `src/buffer.rs` |
| `TextBuffer` | `{ records: Vec<TextBufRecord>, }` | — | `src/buffer.rs` |
| `TextErasureLayer` | `{ layer_id: LayerId, }` | — | `src/erasure.rs` |
| `TextIndexMetadata` | `{ /// Gesamtzahl der aktiven Dokumente im Index. pub total_docs: u64, /// Gesamtzahl aller indizierten Token über alle Dokumente. pub total_tokens: u64, /// Fest… }` | Konsolidierte Metadaten des Textindex zur persistenten Speicherung im Schlüssel `meta:stats`. | `src/inverted/types.rs` |
| `TextPurger` | `{ index: Arc<InvertedIndex<S>>, }` | Physical Purger implementation for `InvertedIndex`. | `src/inverted/purge.rs` |
| `TokenReductionMetrics` | `{ /// Anzahl ursprünglicher Token vor der morphologischen Zerlegung. pub original_tokens: usize, /// Anzahl resultierender Token nach Zerlegung von Komposita. pu… }` | Metriken zur Erfassung der Token-Expansions- und Zerlegungswirkung. | `src/morphology/metrics.rs` |
| `Trie` | `{ root: TrieNode, }` | Präfixbaum (Trie) für schnelle O(m)-Wörterbuchabfragen (m = Wortlänge). | `src/morphology/trie.rs` |
| `TrieNode` | `{ /// Zeigt an, ob an diesem Knoten ein vollständiges Wort aus dem Wörterbuch endet. pub is_terminal: bool, /// Kindknoten, indiziert nach Folgezeichen. pub chil… }` | Knoten im Präfixbaum zur Repräsentation von Buchstabenfolgen deutscher Wörter. | `src/morphology/trie.rs` |
| `UndeterminedAnalyzer` | `(tuple/unit)` | — | `src/analyzer.rs` |
| `WandSearchResult` | `{ /// Gefundene Trefferdokumente mit Relevanzwert. pub results: Vec<(DocId, f32)>, /// Anzahl tatsächlich voll berechneter Dokumente (Pruning-Effektivitätsmetrik… }` | Ergebnis einer Block-Max-WAND-Suche inklusive Top-K-Treffern und Metadaten über evaluierte Dokumente. | `src/wand.rs` |

### Öffentliche Datentypen — Enums (2)

| Enum | Varianten | Beschreibung | Datei |
|---|---|---|---|
| `Language` | `/// Englischer Standard-Tokenizer (Unicode-Wortgrenzen + Stoppwortfilterung). #[default] English, /// Deutscher morphologischer Tokenizer (Komposita-Zerlegung + Umlaut-Normalisierung). German, /// Be…` | Unterstützte Sprachen und Tokenisierungsstrategien für den invertierten Index. Steuert, welcher Tokenizer für die Dokumentenindizierung und Suchabfra… | `src/inverted/types.rs` |
| `ParsedTextTarget` | `/// A single document ID target. Document(DocId), /// A collection namespace target. Collection(String),` | Parsed text target representation. | `src/inverted/purge.rs` |

### Typ-Aliase (3)

| Alias | Definition | Beschreibung |
|---|---|---|
| `DocIdRaw` | `type DocIdRaw= u64` | — |
| `FieldId` | `type FieldId= u32` | Eindeutige Kennung für Dokumentenfelder im feldgewichteten BM25F-Scoring. |
| `UndAnalyzer` | `type UndAnalyzer= UndeterminedAnalyzer` | — |

### Konstanten (14)

| Konstante | Typ | Beschreibung |
|---|---|---|
| `ALG_IDS` | `[u8` | — |
| `BLOCK_SIZE` | `usize` | — |
| `BLOCK_SIZE` | `usize` | Blockgröße für die Block-Max-WAND-Zerlegung (64 Postings pro Block). |
| `BM25_B` | `f32` | Standard-b-Parameter für die Dokumentlängen-Normalisierung in BM25 (0.75). |
| `BM25_K1` | `f32` | Standard-k1-Parameter für die Termfrequenz-Sättigung in BM25 (1.5). |
| `DEFAULT_STREAM_BATCH_SIZE` | `usize` | Standard-Batch-Größe für das Nachladen von Kandidaten (16 Dokumente). |
| `FILE_KIND_TEXT` | `u8` | — |
| `FLAG_POSITIONS` | `u8` | — |
| `FORMAT_VERSION` | `u16` | — |
| `MAGIC` | `&[u8` | — |
| `MAX_STAGED_TRANSACTIONS` | `usize` | Maximale Anzahl gleichzeitig ungecommitteter Transaktionen (10.000). |
| `MAX_TEXT_BYTES` | `usize` | Maximale Textgröße pro Dokument (10 MiB) zur Begrenzung von Speicherverbrauch und DoS-Schutz. |
| `POSTING_BLOCK_SIZE` | `usize` | — |
| `POSTING_LIST_V2_MAGIC` | `&[u8` | Magische Bytes für das kompakte V2-Posting-Listen-Format (`PL\x02\x00`). |

### Öffentliche Funktionen (131)

| Signatur | Beschreibung | Datei |
|---|---|---|
| `pub fn compute_term_tag(key: &IdxKey, analyzer_id: u8, lang: &[u8; 3], term: &str) -> [u8` | — | `src/analyzer.rs` |
| `pub fn get_analyzer(lang: &str, version: u16) -> Option<Box<dyn Analyzer>>` | — | `src/analyzer.rs` |
| `pub fn new() -> Self` | — | `src/analyzer.rs` |
| `pub fn new(k1: f32, b: f32) -> Result<Self>` | Erzeugt eine neue `BM25`-Instanz mit validierten Hyperparametern. # Fehler Liefert [`ActaError::InvalidInput`], falls `k1 < 0.0` bzw. NaN i… | `src/bm25.rs` |
| `pub fn new(field_id: FieldId, weight: f32, b: f32) -> Result<Self>` | Erzeugt eine neue `FieldWeight`-Konfiguration nach Validierung von `weight` und `b`. | `src/bm25.rs` |
| `pub fn new(k1: f32, field_weights: Vec<FieldWeight>) -> Result<Self>` | Erzeugt ein neues BM25F-Modell mit Feldgewichtungen. | `src/bm25.rs` |
| `pub fn score_term(&self, tf: u32, doc_len: u32, avg_doc_len: f32, df: u32, n: u32) -> f32` | Berechnet den BM25-Score für einen Begriff mit den Parametern dieser Instanz. | `src/bm25.rs` |
| `pub fn score_term(tf: u32, doc_len: u32, avg_doc_len: f32, df: u32, n: u32) -> f32` | Berechnet den BM25-Score für einen Begriff mit den Standardparametern (`k1 = 1.5`, `b = 0.75`). # Parameter * `tf` - Termhäufigkeit (Term F… | `src/bm25.rs` |
| `pub fn score_term( &self, field_term_frequencies: &[(FieldId, u32, u32, f32)], df: u32, n: u32, ) -> f32` | Berechnet den BM25F-Score über mehrere Felder hinweg. | `src/bm25.rs` |
| `pub fn score_term_bm25f( field_term_frequencies: &[(FieldId, u32, u32, f32)], field_weights: &[(FieldId, f32, f32)], k1: f32, df: u32, n: u32, ) -> f32` | Berechnet den feldgewichteten BM25F-Score nach Robertson & Zaragoza (2004). Berechnet zunächst die gewichtete Pseudo-Termfrequenz über alle… | `src/bm25.rs` |
| `pub fn score_term_with_params( tf: u32, doc_len: u32, avg_doc_len: f32, df: u32, n: u32, k1: f32, b: f32, ) -> f32` | Berechnet den BM25-Score mit frei konfigurierbaren Parametern `k1` und `b`. # Ablauf 1. Schnellausstieg mit 0.0, wenn $N = 0$, $df = 0$ ode… | `src/bm25.rs` |
| `pub fn drain(&mut self) -> Vec<TextBufRecord>` | — | `src/buffer.rs` |
| `pub fn insert(&mut self, cluster_tag: [u8; 16], doc_tag: [u8; 16], text: String) -> u32` | — | `src/buffer.rs` |
| `pub fn is_empty(&self) -> bool` | — | `src/buffer.rs` |
| `pub fn len(&self) -> usize` | — | `src/buffer.rs` |
| `pub fn new() -> Self` | — | `src/buffer.rs` |
| `pub fn open_binding_envelope( dek: &Dek, file_id: [u8; 16], doc_ord: u32, envelope: &[u8], ) -> Result<([u8` | — | `src/buffer.rs` |
| `pub fn seal_binding_envelope( dek: &Dek, file_id: [u8; 16], doc_ord: u32, cluster_tag: [u8; 16], doc_tag: [u8; 16], ) -> Result<Vec<u8>>` | — | `src/buffer.rs` |
| `pub fn get_legal_compound_stems() -> &'static [&'static str]` | Liefert den statischen Slice aller juristischen Komposita-Wortstämme. | `src/domain/legal_de.rs` |
| `pub fn get_legal_protected_terms() -> &'static [&'static str]` | Liefert den statischen Slice aller juristischen geschützten Begriffe. | `src/domain/legal_de.rs` |
| `pub fn get_medical_compound_stems() -> &'static [&'static str]` | Liefert den statischen Slice aller medizinischen Komposita-Wortstämme. | `src/domain/medical_de.rs` |
| `pub fn get_medical_protected_terms() -> &'static [&'static str]` | Liefert den statischen Slice aller medizinischen geschützten Fachbegriffe. | `src/domain/medical_de.rs` |
| `pub fn new() -> Self` | — | `src/erasure.rs` |
| `pub fn commit_lock(&self) -> &parking_lot::Mutex<()>` | Accessor for single-writer commit lock. | `src/inverted/index_struct.rs` |
| `pub async fn delete_document(&self, tx: TxId, doc_id: DocId) -> Result<()>` | Löscht ein Dokument logisch aus dem Index und markiert alle Terme mit Tombstones. | `src/inverted/index_struct.rs` |
| `pub async fn load_stats(&self) -> Result<()>` | Lädt die persistenten Statistiken (`meta:stats`) aus dem Storage in die atomaren In-Memory-Felder. | `src/inverted/index_struct.rs` |
| `pub fn new(storage: Arc<S>, namespace: &str) -> Self` | Erzeugt einen neuen invertierten Index mit englischem Standard-Tokenizer. | `src/inverted/index_struct.rs` |
| `pub fn new_with_language(storage: Arc<S>, namespace: &str, language: Language) -> Self` | Erzeugt einen neuen invertierten Index mit expliziter Sprachkonfiguration. # Language Selection Use `Language::from_iso("de")` for German,… | `src/inverted/index_struct.rs` |
| `pub fn prefix(&self) -> &[u8]` | Accessor for index storage prefix bytes. | `src/inverted/index_struct.rs` |
| `pub async fn resolve_tombstones(&self, tx: TxId) -> Result<u64>` | Löst aufgelaufene Tombstones (`tbs:{doc_id}:{term}`) auf und bereinigt verwaiste Posting-Listen. | `src/inverted/index_struct.rs` |
| `pub async fn search_bm25( &self, query: &str, k: usize, max_seq: Option<u64>, ) -> Result<Vec<(DocId, f32)>>` | Führt eine Volltextsuche mit BM25 aus. | `src/inverted/index_struct.rs` |
| `pub async fn search_bm25_at( &self, query: &str, k: usize, max_seq: Option<u64>, ) -> Result<Vec<(DocId, f32)>>` | Führt eine Volltextsuche mit BM25 zu einer bestimmten MVCC-Snapshot-Sequenznummer aus. | `src/inverted/index_struct.rs` |
| `pub fn storage(&self) -> &Arc<S>` | Accessor for underlying storage engine. | `src/inverted/index_struct.rs` |
| `pub async fn upsert_document(&self, tx: TxId, doc_id: DocId, text: &str) -> Result<()>` | Fügt ein Dokument in den Index ein oder aktualisiert ein bestehendes. # Ablauf 1. Größenprüfung gegen `MAX_TEXT_BYTES`. 2. Tokenisierung un… | `src/inverted/index_struct.rs` |
| `pub fn with_tokenizer(mut self, tokenizer: Arc<dyn Tokenizer>) -> Self` | Konfiguriert einen benutzerdefinierten Tokenizer. | `src/inverted/index_struct.rs` |
| `pub fn new( storage: Arc<S>, namespace: &str, tokenizer: Arc<dyn MorphologicalTokenizer>, ) -> Self` | Erzeugt einen neuen morphologischen Index mit deutschem Standard-Tokenizer. | `src/inverted/morph_index.rs` |
| `pub fn tokenizer(&self) -> &dyn MorphologicalTokenizer` | Liefert eine Referenz auf den konfigurierten morphologischen Tokenizer. | `src/inverted/morph_index.rs` |
| `pub fn is_success(&self) -> bool` | Returns `true` if no errors occurred during purge. | `src/inverted/purge.rs` |
| `pub fn new() -> Self` | Creates a new empty `PurgeReport`. | `src/inverted/purge.rs` |
| `pub fn new(target_id: impl Into<String>, target_hash: [u8; 32]) -> Self` | Creates a new `ScanTarget`. | `src/inverted/purge.rs` |
| `pub fn new(index: Arc<InvertedIndex<S>>) -> Self` | Creates a new `TextPurger` for the given index. | `src/inverted/purge.rs` |
| `pub fn parse(target_id: &str) -> Self` | Parses a target ID string into a document ID or collection namespace. | `src/inverted/purge.rs` |
| `pub fn purge_collection(&self, tx: TxId, namespace: &str) -> Result<PurgeStats>` | Fast-path purge for an entire collection namespace (`__txt:{collection}:`). | `src/inverted/purge.rs` |
| `pub fn purge_collection_report(&self, tx: TxId, namespace: &str) -> PurgeReport` | Fast-path purge for an entire collection namespace (`__txt:{collection}:`), returning a `PurgeReport`. | `src/inverted/purge.rs` |
| `pub fn purge_document(&self, tx: TxId, doc_id: DocId) -> Result<PurgeStats>` | Purges a single document from the inverted index and recalculates corpus statistics. | `src/inverted/purge.rs` |
| `pub fn purge_document_report(&self, tx: TxId, doc_id: DocId) -> PurgeReport` | Purges a single document from the inverted index and recalculates corpus statistics, returning a detailed `PurgeReport`. | `src/inverted/purge.rs` |
| `pub fn to_purge_stats(&self) -> Result<PurgeStats>` | Converts this report to `PurgeStats`, or returns an error if partial failure occurred. | `src/inverted/purge.rs` |
| `pub fn from_iso(code: &str) -> Self` | Parst einen ISO-639-1 Sprachcode ("de", "de-AT", "en", "en-US"). Bewertet nur das primäre Subtag vor dem ersten Bindestrich. Unbekannte Cod… | `src/inverted/types.rs` |
| `pub fn new(storage: std::sync::Arc<S>, namespace: &str) -> Self` | — | `src/lib.rs` |
| `pub fn add_tag(&mut self, k_idx: &IdxKey, tag_bytes: &[u8; 16])` | Fügt ein Tag (Cluster- oder Dokument-Tag) dem Mitgliedschaftsblock hinzu. | `src/member.rs` |
| `pub fn compute_member_hash(k_idx: &IdxKey, tag_bytes: &[u8; 16]) -> u64` | Berechnet den 64-Bit Mitgliedschafts-Hash m(t) nach SPEC D-121 Abs. 3: `m(t) = u64_be(BLAKE3-keyed(K_idx, "ctx/member/v1" || t)[0..8])` | `src/member.rs` |
| `pub fn contains_cluster(&self, k_idx: &IdxKey, cluster: &ClusterTag) -> bool` | — | `src/member.rs` |
| `pub fn contains_doc(&self, k_idx: &IdxKey, doc: &DocTag) -> bool` | — | `src/member.rs` |
| `pub fn contains_tag(&self, k_idx: &IdxKey, tag_bytes: &[u8; 16]) -> bool` | Prüft, ob ein gegebenes Tag im Mitgliedschaftsblock enthalten ist (SPEC D-121 Abs. 3). Falsch-negativ ist ausgeschlossen! | `src/member.rs` |
| `pub fn decode(bytes: &[u8]) -> Option<(Self, usize)>` | — | `src/member.rs` |
| `pub fn encode(&self) -> Vec<u8>` | — | `src/member.rs` |
| `pub fn finalize(&mut self)` | Sortiert und dedupliziert die Mitgliederliste kanonisch. | `src/member.rs` |
| `pub fn new() -> Self` | — | `src/member.rs` |
| `pub fn extend_vocabulary(&mut self, words: impl IntoIterator<Item = impl AsRef<str>>)` | Erweitert das Wörterbuch des Zerlegers dynamisch um zusätzliche Begriffe. | `src/morphology/compound_splitter.rs` |
| `pub fn min_component_len(&self) -> usize` | Liefert die Mindestlänge für Teilkomponenten. | `src/morphology/compound_splitter.rs` |
| `pub fn new() -> Self` | Erzeugt einen neuen Zerleger mit der Standard-Mindestlänge (3 Zeichen) und Basis-Wortschatz. | `src/morphology/compound_splitter.rs` |
| `pub fn new_with_all_domains() -> Self` | Erzeugt einen Zerleger mit allen eingebauten Fachdomänen (Recht & Medizin). | `src/morphology/compound_splitter.rs` |
| `pub fn with_dictionary( min_len: usize, custom_words: impl IntoIterator<Item = impl AsRef<str>>, ) -> Self` | Erzeugt einen Zerleger mit benutzerdefiniertem Wörterbuch. | `src/morphology/compound_splitter.rs` |
| `pub fn with_domains(min_len: usize, domains: &[&dyn DomainVocabulary]) -> Self` | Erzeugt einen Zerleger erweitert um spezifische Fachvokabulare (z. B. Recht, Medizin). | `src/morphology/compound_splitter.rs` |
| `pub fn with_min_length(min_len: usize) -> Self` | Erzeugt einen Zerleger mit konfigurierbarer Mindest-Segmentlänge. | `src/morphology/compound_splitter.rs` |
| `pub fn expansion_ratio(&self) -> f32` | Berechnet das Expansionsverhältnis der Token-Menge. Ein Wert > 1,0 bedeutet eine Erhöhung der Tokenanzahl durch Komposita-Splitting. # Ziel… | `src/morphology/metrics.rs` |
| `pub fn new(lang: impl Into<String>) -> Self` | Erzeugt einen neuen Durchleitungs-Tokenizer für den angegebenen ISO-Sprachcode (z. B. "en"). | `src/morphology/passthrough.rs` |
| `pub fn get_german_stopwords() -> &'static BTreeSet<String>` | Liefert die statische deutsche Stoppwort-Liste. | `src/morphology/stopwords.rs` |
| `pub fn is_german_stopword(word: &str) -> bool` | — | `src/morphology/stopwords.rs` |
| `pub fn normalize_umlauts(input: &str) -> String` | — | `src/morphology/stopwords.rs` |
| `pub fn contains(&self, word: &str) -> bool` | Prüft, ob ein Wort exakt als terminaler Eintrag im Baum enthalten ist. | `src/morphology/trie.rs` |
| `pub fn insert(&mut self, word: &str)` | Fügt ein Wort zeichenweise in den Baum ein und markiert den Endknoten als terminal. | `src/morphology/trie.rs` |
| `pub fn new() -> Self` | Erzeugt einen leeren Präfixbaum. | `src/morphology/trie.rs` |
| `pub fn starts_with(&self, prefix: &str) -> bool` | Prüft, ob im Baum mindestens ein Wort existiert, das mit dem übergebenen Präfix beginnt. | `src/morphology/trie.rs` |
| `pub fn match_phrase_positions(term_positions: &[&[u32]]) -> bool` | — | `src/phrase.rs` |
| `pub fn as_slice(&self) -> &[Posting]` | Liefert die Postings als Slice. | `src/posting_list.rs` |
| `pub fn blocks(&self) -> &[PostingBlockInfo]` | Liefert die Block-Max-Metadaten. | `src/posting_list.rs` |
| `pub fn clear(&self)` | Leert den gesamten Cache (z. B. bei vollständigem Index-Neuladen). | `src/posting_list.rs` |
| `pub fn decode_compact(bytes: &[u8]) -> Result<Self, ActaError>` | Dekodiert eine Posting-Liste aus dem Delta-Varint-Format mit Fallback auf Legacy-Bincode. | `src/posting_list.rs` |
| `pub fn doc_id(&self) -> DocId` | — | `src/posting_list.rs` |
| `pub fn empty() -> Self` | Erzeugt eine leere Posting-Liste. | `src/posting_list.rs` |
| `pub fn encode_compact(&self) -> Result<Vec<u8>, ActaError>` | Kodiert die Posting-Liste in das kompakte Binärformat mit Delta-Varint-Kompression. | `src/posting_list.rs` |
| `pub fn get(&self, term: &str) -> Option<Arc<PostingList>>` | Liefert eine referenzgezählte Posting-Liste für den Begriff, falls im Cache vorhanden. | `src/posting_list.rs` |
| `pub fn insert_list(&self, term: String, list: PostingList)` | Fügt eine Posting-Liste in den Index ein oder ersetzt sie. | `src/posting_list.rs` |
| `pub fn is_empty(&self) -> bool` | Prüft, ob die Liste leer ist. | `src/posting_list.rs` |
| `pub fn len(&self) -> usize` | Liefert die Anzahl der Einträge in der Liste. | `src/posting_list.rs` |
| `pub fn new(doc_id: DocId, tf: u32, doc_len: u32) -> Self` | — | `src/posting_list.rs` |
| `pub fn new(mut postings: Vec<Posting>) -> Self` | Erzeugt eine sortierte und deduplizierte Posting-Liste und berechnet die Block-Metadaten. | `src/posting_list.rs` |
| `pub fn new() -> Self` | Erzeugt einen leeren residenten Posting-Index. | `src/posting_list.rs` |
| `pub fn remove(&self, doc_id: DocId) -> Self` | Entfernt ein Posting per Copy-On-Write (RCU). | `src/posting_list.rs` |
| `pub fn remove_posting_from_terms(&self, terms: &[String], doc_id: DocId)` | Entfernt eine Dokument-ID aus den Posting-Listen mehrerer Begriffe per RCU. | `src/posting_list.rs` |
| `pub fn remove_terms(&self, terms: &[String])` | Entfernt die angegebenen Begriffe aus dem Cache (z. B. bei Rollback). | `src/posting_list.rs` |
| `pub fn upsert(&self, posting: Posting) -> Self` | Fügt ein Posting per Copy-On-Write (RCU) ein oder aktualisiert einen bestehenden Eintrag. | `src/posting_list.rs` |
| `pub fn upsert_posting(&self, term: &str, posting: Posting)` | Fügt ein einzelnes Posting per RCU (Copy-On-Write) in die Posting-Liste des Begriffs ein. | `src/posting_list.rs` |
| `pub fn decode(bytes: &[u8], has_positions: bool) -> Result<(Self, usize)>` | — | `src/postings.rs` |
| `pub fn decode_vbyte(slice: &[u8]) -> Option<(u64, usize)>` | — | `src/postings.rs` |
| `pub fn encode(&self, has_positions: bool) -> Vec<u8>` | — | `src/postings.rs` |
| `pub fn encode_vbyte(mut val: u64, out: &mut Vec<u8>)` | — | `src/postings.rs` |
| `pub fn new() -> Self` | — | `src/postings.rs` |
| `pub fn add_document(&mut self, doclen: u32, binding_envelope: Vec<u8>) -> u32` | — | `src/segment.rs` |
| `pub fn add_term(&mut self, term_tag: [u8; 16], df: u32, block: PostingBlock)` | — | `src/segment.rs` |
| `pub fn decode(bytes: &[u8; 32]) -> Self` | — | `src/segment.rs` |
| `pub fn doc_count(&self) -> u32` | — | `src/segment.rs` |
| `pub fn encode(&self) -> [u8` | — | `src/segment.rs` |
| `pub fn flags(&self) -> u8` | — | `src/segment.rs` |
| `pub fn get_binding(&self, target_doc_ord: u32) -> Option<(u32, &[u8])>` | — | `src/segment.rs` |
| `pub fn get_postings(&self, target_term: &TermTag) -> Option<&PostingBlock>` | — | `src/segment.rs` |
| `pub fn new( bucket: u8, bucket_count: u8, analyzer_id: u8, analyzer_version: u16, lang: [u8; 3], positions: bool, ) -> Self` | — | `src/segment.rs` |
| `pub fn open_from_bytes(bytes: &[u8], file_id: [u8; 16], file_key: FileKey) -> Result<Self>` | — | `src/segment.rs` |
| `pub fn seal_to_bytes(&mut self, file_id: [u8; 16], file_key: FileKey) -> Result<Vec<u8>>` | — | `src/segment.rs` |
| `pub fn serialize_payload(&mut self) -> Vec<u8>` | — | `src/segment.rs` |
| `pub fn new() -> Self` | — | `src/stats.rs` |
| `pub fn recompute(living_doc_lens: &[u32]) -> Self` | — | `src/stats.rs` |
| `pub fn is_exhausted(&self) -> bool` | Prüft, ob der Stream vollständig erschöpft ist (keine gepufferten oder nachladbaren Elemente mehr). | `src/stream.rs` |
| `pub fn new( index: &'a InvertedIndex<S>, query: impl Into<String>, max_seq: Option<u64>, ) -> Self` | Erzeugt einen neuen Kandidaten-Stream für die übergebene Abfrage und Index-Snapshot. # Parameter - `index`: Referenz auf den invertierten I… | `src/stream.rs` |
| `pub async fn next_batch(&mut self) -> Result<Vec<(DocId, f32)>>` | Holt den nächsten Batch von Kandidaten-Dokumenten (bis zu `batch_size`). Liefert einen leeren Vektor, sobald der Stream erschöpft ist. # Ab… | `src/stream.rs` |
| `pub async fn next_doc(&mut self) -> Result<Option<DocId>>` | Holt die nächste einzelne Dokument-ID aus dem Stream. Liefert `Ok(None)`, wenn keine weiteren Treffer vorliegen. | `src/stream.rs` |
| `pub fn with_batch_size(mut self, n: usize) -> Self` | Konfiguriert eine benutzerdefinierte Batch-Größe (mindestens 1). | `src/stream.rs` |
| `pub fn yielded(&self) -> usize` | Liefert die Gesamtzahl aller bisher an den Aufrufer herausgegebenen Treffer. | `src/stream.rs` |
| `pub fn compile_protected_regex(pattern: &str) -> Result<Regex>` | Kompiliert ein reguläres Muster für geschützte Token mit Fail-Closed-Fehlerbehandlung. # Fehler Gibt [`ActaError::Text`] zurück, falls der… | `src/tokenizer.rs` |
| `pub fn get_protected_regex() -> Result<&'static Regex>` | Liefert den gecachten regulären Standardausdruck für geschützte URLs und E-Mail-Adressen. | `src/tokenizer.rs` |
| `pub fn new() -> Self` | — | `src/tokenizer.rs` |
| `pub fn new() -> Self` | — | `src/tokenizer.rs` |
| `pub fn tokenize(text: &str) -> Vec<String>` | Hilfsfunktion zur Tokenisierung mit dem [`DefaultTokenizer`]. | `src/tokenizer.rs` |
| `pub fn try_new() -> Result<Self>` | Fallible Konstruktion eines `DefaultTokenizer` mit Validierung des regulären Schutzmusters. | `src/tokenizer.rs` |
| `pub fn try_new() -> Result<Self>` | Erzeugt eine neue Instanz des deutschen morphologischen Tokenizers. | `src/tokenizer.rs` |
| `pub async fn block_max_wand_search<S: StorageEngine>( storage: &S, terms: &[String], resident_index: &crate::posting_list::ResidentPostingIndex, prefix_helper: impl Fn(&str) -> Vec<u8>, tombstone_key_helper: impl Fn(DocId, &str) -> Vec<u8>, doc_len_key_helper…` | — | `src/wand.rs` |
| `pub fn into_sorted_vec(self) -> Vec<(DocId, f32)>` | Konvertiert den Heap in einen nach Relevanz absteigend sortierten Ergebnisvektor. | `src/wand.rs` |
| `pub fn min_threshold(&self) -> f32` | Liefert den aktuellen Minimal-Score (Schwelle), den ein Kandidat übertreffen muss. | `src/wand.rs` |
| `pub fn new(capacity: usize) -> Self` | Erzeugt einen neuen `BoundedTopK`-Puffer mit maximaler Kapazität (`MAX_SEARCH_K`). | `src/wand.rs` |
| `pub fn push(&mut self, doc_id: DocId, score: f32)` | Fügt einen neuen Kandidaten ein, wenn er besser als der schlechteste bisherige Treffer ist. | `src/wand.rs` |


---

## `acta-rank`

| Attribut | Wert |
|---|---|
| **Ring** | 0 — Ring 0 — Foundation & Pure Numerics/Types |
| **Status** | In Migration |
| **Zweck** | 4-Signal-Retrieval-Fusion, Score-Kalibrierung und Drift-Messung. |
| **Unsafe-Code** | Nein (`#![forbid(unsafe_code)]`) |
| **Interne Abhängigkeiten** | — |
| **Externe Abhängigkeiten (Auswahl)** | — |

### Architekturbeschreibung

Reciprocal Rank Fusion für die Hybridsuche.  # Acta Rank (`acta-rank`)  **Ring:** Ring 2 (Ranking & Fusion, SPEC §8.11, D-26) **Klasse:** Rand `[R]` **Reifegrad:** Aktiv (Kern v1)  ## Zweck des Crates Dieses Crate implementiert die deterministische Reciprocal Rank Fusion (RRF) für die Hybridsuche im Acta-System: - Kanonische Schnittstelle: [`rrf_fuse`] - Fester Glättungsfaktor: $k = 60$ ([`K_RRF`]) - Reine Festkomma-Arithmetik ($\lfloor 2^{40} / (60 + \text{Rang}) \rfloor$), keine Gleitkommazahlen - Deterministischer Tie-Break (primär Score absteigend, sekundär `doc_tag` aufsteigend, tertiär `cluster_tag` aufsteigend) - Bit-genaue Permutationsinvarianz bezüglich der Listenreihenfolge - Schrankenberechnung für ungesehene Dokumente ([`unseen_bound`]) - Keine Hintergrund-Threads, kein `unsafe` (`#![forbid(unsafe_code)]`).

### Modul- und Re-Export-Oberfläche (lib.rs)

```rust
pub mod bound;
pub mod error;
pub mod rrf;
pub use bound::unseen_bound;
pub use error::RankError;
pub use rrf::{rrf_fuse, Candidate, DocHandle, Fused, RrfScore, K_RRF};
```

### Öffentliche Datentypen — Structs (4)

| Struct | Felder / Form | Beschreibung | Datei |
|---|---|---|---|
| `Candidate` | `{ /// Eindeutiger Dokument-Handle. pub handle: DocHandle, /// Chunk-Index innerhalb des Dokuments (D-124). pub chunk: u32, }` | Ein Kandidat einer bereits gefilterten Teilliste (RK-04, RK-05, D-124). Der Rang entspricht der 1-basierten Position innerhalb der Liste. Die Identit… | `src/rrf.rs` |
| `DocHandle` | `{ /// Betroffenen- / Cluster-Kennung (`ClusterTag`). pub cluster_tag: [u8; 16], /// Gehashte Dokumentkennung (`DocTag`). pub doc_tag: [u8; 16], }` | Opaker Dokument-Handle für Suchtreffer aus Ring 0 (`types`, D-70). | `src/rrf.rs` |
| `Fused` | `{ /// Eindeutiger Dokument-Handle des Treffers. pub handle: DocHandle, /// Chunk-Index innerhalb des Dokuments (D-124). pub chunk: u32, /// Akkumulierter Festkom… }` | Ein fusionierter Treffer mit Rang und berechnetem Festkomma-Score. Die Identität eines Treffers ist `(doc_tag, chunk)` (SPEC D-124, T-HYB-2). | `src/rrf.rs` |
| `RrfScore` | `(tuple/unit)` | Festkomma-Ganzzahl-Repräsentation des fusionierten RRF-Scores (RK-02). Vermeidet jegliche Gleitkomma-Nichtassoziativität und garantiert bit-genaue Pe… | `src/rrf.rs` |

### Öffentliche Datentypen — Enums (1)

| Enum | Varianten | Beschreibung | Datei |
|---|---|---|---|
| `RankError` | `/// Eine Kandidatenliste überschreitet die Obergrenze von 1.000 Einträgen (D-87). #[error("Kandidatenliste überschreitet die Obergrenze von 1000 Einträgen")] TooLarge, /// Ein Dokument kommt mehrfach…` | Fehler bei der Validierung von Eingaben und Berechnung der Reciprocal Rank Fusion. | `src/error.rs` |

### Konstanten (1)

| Konstante | Typ | Beschreibung |
|---|---|---|
| `K_RRF` | `u32` | Glättungsfaktor $k = 60$ für Reciprocal Rank Fusion (SPEC §8.11, Anhang C, RK-01). |

### Öffentliche Funktionen (2)

| Signatur | Beschreibung | Datei |
|---|---|---|
| `pub fn unseen_bound(depths: &[usize]) -> Result<RrfScore, RankError>` | Berechnet eine obere Schranke für den Score eines Dokuments, das in jeder Teilliste erst nach der Tiefe `depths[i]` auftreten kann. Formel:… | `src/bound.rs` |
| `pub fn rrf_fuse(lists: &[&[Candidate]], top: usize) -> Result<Vec<Fused>, RankError>` | Fusioniert beliebig viele Kandidatenlisten deterministisch via Reciprocal Rank Fusion. - Glättungsfaktor: $k = 60$ (fest). - Beitrag je Tei… | `src/rrf.rs` |


---

## `acta-store`

| Attribut | Wert |
|---|---|
| **Ring** | 1 — Ring 1 — Persistence & Cache |
| **Status** | Fertig |
| **Zweck** | LSM-Tree Storage Engine mit WAL-Group-Commit und HMAC-Integritätsprüfung. |
| **Unsafe-Code** | Nein (`#![forbid(unsafe_code)]`) |
| **Interne Abhängigkeiten** | `acta-crypto`, `acta-mvcc`, `acta-ports`, `acta-sys`, `acta-types` |
| **Externe Abhängigkeiten (Auswahl)** | `aes-gcm-siv`, `ahash`, `blake3`, `bytes`, `crc32fast`, `hkdf`, `hmac`, `lru`, `parking_lot`, `quick_cache`, `rand`, `serde`, `sha2`, `thiserror`, `tokio`, `tokio-util`, `tracing`, `uuid`, `zeroize` |

### Architekturbeschreibung

# acta-store (Ring 2)  Zweck: LSM-Speicher: WAL, Memtable, SSTables, Manifest, Kompaktion, Purge-Primitive.  Ring: Ring 2. Erlaubte Abhängigkeiten: `acta-types`, `acta-ports`, `acta-sys`, `acta-crypto`, `acta-mvcc`. Referenz: SPEC Abschnitt 5.1, 5.3, 8.8.  ## Sicherheitsinvariante Die gesamte Crate erzwingt `#![forbid(unsafe_code)]` in Produktion. Es existieren keinerlei `unsafe`-Bloecke im Produktionscode.

### Modul- und Re-Export-Oberfläche (lib.rs)

```rust
pub mod compaction;
pub mod engine;
pub mod envelope;
pub mod fsync;
pub mod lsm;
pub mod manifest;
pub mod memtable;
pub mod sstable;
pub mod system_pressure;
pub mod tenant_codec;
pub mod vfs;
pub mod wal;
pub mod erase;
pub mod health;
pub mod purge;
pub mod tenant_scoped;
pub use compaction::{ adaptive::{ AdaptiveCompactionPlan, AdaptiveCompactionPlanner, CompactionStrategy, CostBasedAdaptivePlanner, WorkloadMetrics, WorkloadMetricsSnapshot, }, CompactionConfig, CompactionEngine, MergeOperator, TtlMetadata, };
pub use lsm::{ CommittedBatch, LsmConfig, LsmStorage, ObserverRegistry, WalEntryRef, WalObserver, WriteOrigin, DEFAULT_MAX_OBSERVER_LATENCY, };
pub use manifest::{Manifest, ManifestEntry};
pub use system_pressure::{PressureLevel, SystemPressure, SystemPressureMonitor};
pub use tenant_codec::{TenantKeyCodec, TenantScopedStorage};
```

### Feature-Flags

| Flag | Aktiviert |
|---|---|
| `default` | `[]` |
| `test-hooks` | `[]` |
| `test-utils` | `["test-hooks"]` |
| `memory-only-storage` | `[]` |
| `fault-injection` | `[]` |
| `block-cache-v2` | `["dep:quick_cache"]` |
| `docid-128` | `["acta-types/docid-128"]` |

### Schnittstellen — Public Traits (7)

**`AdaptiveCompactionPlanner`** — *Trait defining adaptive compaction strategy selection.* (`src/compaction/adaptive.rs`)

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

**`BlockCacheBackend`** — ** (`src/sstable/block_cache.rs`)

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
}
```

**`MergeOperator`** — *Trait for merging two raw byte values (`existing_val` from older tier, `new_val` from newer tier) for the same key across different SSTable tiers.* (`src/compaction/merge_operator.rs`)

```rust
pub trait MergeOperator : Send + Sync {
/// Merges two raw byte values for the same key.
    ///
    /// # Fail-Safe Requirement
    /// Implementations running untrusted guest code MUST
    /// implement fail-safe handling: if execution fails (e.g. fuel exhaustion or trap),
    /// the caller or operator must swallow the error for that single entry, log a `tracing::warn!`,
    /// and retain both original versions in the compacted SSTable without aborting the overall compaction.
    fn merge(&self, existing_val: &[u8], new_val: &[u8]) -> Result<Vec<u8>>;
}
```

**`PurgePrimitives`** — *Purge-Primitive (nur über ErasureLayer erreichbar; Reihenfolge fest, jeder Schritt idempotent).* (`src/purge/backend.rs`)

```rust
pub trait PurgePrimitives : Send + Sync + 'static {
/// Memtable-Einträge im Umfang verwerfen (ST-04).
    fn drain_range<'a>(
        &'a self,
        matcher: &'a Matcher,
    ) -> acta_ports::BoxFuture<'a, Result<DrainStats>>;

    /// Aktives Segment an Sequenzgrenze versiegeln (ST-04).
    fn seal_wal_at<'a>(&'a self) -> acta_ports::BoxFuture<'a, Result<SealPoint>>;

    /// Segmente <= f_seq als Ganzes entfernen (Dateischlüssel vernichten, unlink) (ST-04).
    fn drop_wal_until<'a>(&'a self, f_seq: u64)
        -> acta_ports::BoxFuture<'a, Result<WalFloor>>;

    /// Kopiert Nicht-Ziel-Records als byteidentische Envelopes in eine neue Datei (ST-07).
    fn rewrite_without<'a>(
        &'a self,
        file_id: FileId,
        matcher: &'a Matcher,
        reserve: DiskReserve,
    ) -> acta_ports::BoxFuture<'a, Result<RewriteOutcome>>;

    /// Manifest-Swap dauerhaft -> F-Slot nullen -> unlink und Verzeichnis-fsync (ST-07, ST-08, ST-09).
    fn unlink_verified<'a>(&'a self, file_id: FileId)
        -> acta_ports::BoxFuture<'a, Result<()>>;
}
```

**`ReplayProgressSink`** — ** (`src/wal/replay.rs`)

```rust
pub trait ReplayProgressSink : Send + Sync {
fn on_entry_replayed(&self, seq: WalSeq, entries_total_estimate: Option<u64>);
}
```

**`StorageEngineHandle`** — *Internal trait abstraction enabling `TenantScopedStorage` to wrap both `S: StorageEngine` and `Arc<S>`.* (`src/tenant_codec.rs`)

```rust
pub trait StorageEngineHandle : Send + Sync + 'static {
fn get_handle<'a>(&'a self, key: &'a [u8]) -> BoxFuture<'a, Result<Option<Bytes>>>;
    fn get_at_seq_handle<'a>(
        &'a self,
        key: &'a [u8],
        seq_no: u64,
    ) -> BoxFuture<'a, Result<Option<Bytes>>>;
    fn get_tracked_handle<'a>(
        &'a self,
        tx_id: TxId,
        key: &'a [u8],
    ) -> BoxFuture<'a, Result<Option<Bytes>>>;
    fn get_at_seq_tracked_handle<'a>(
        &'a self,
        tx_id: TxId,
        key: &'a [u8],
        seq: u64,
    ) -> BoxFuture<'a, Result<Option<Bytes>>>;
    #[allow(clippy::type_complexity)]
    fn scan_prefix_tracked_handle<'a>(
        &'a self,
        tx_id: TxId,
        prefix: &'a [u8],
    ) -> BoxFuture<'a, Result<Vec<(Vec<u8>, Vec<u8>)>>>;
    fn supports_ssi_tracking_handle(&self) -> bool;
    fn put_handle<'a>(
        &'a self,
        tx_id: TxId,
        key: &'a [u8],
        value: &'a [u8],
    ) -> BoxFuture<'a, Result<()>>;
    fn put_if_absent_handle<'a>(
        &'a self,
        tx_id: TxId,
        key: &'a [u8],
        value: &'a [u8],
    ) -> BoxFuture<'a, Result<bool>>;
    fn put_batch_handle<'a>(
        &'a self,
        tx_id: TxId,
        entries: &'a [(Vec<u8>, Vec<u8>)],
    ) -> BoxFuture<'a, Result<()>>;
    fn delete_handle<'a>(&'a self, tx_id: TxId, key: &'a [u8]) -> BoxFuture<'a, Result<()>>;
    fn delete_many_handle<'a>(
        &'a self,
        tx_id: TxId,
        keys: Vec<Vec<u8>>,
    ) -> BoxFuture<'a, Result<u64>>;
    fn delete_prefix_handle<'a>(
        &'a self,
        tx_id: TxId,
        prefix: &'a [u8],
    ) -> BoxFuture<'a, Result<u64>>;
    fn commit_handle<'a>(&'a self, tx_id: TxId) -> BoxFuture<'a, Result<()>>;
    fn rollback_handle<'a>(&'a self, tx_id: TxId) -> BoxFuture<'a, Result<()>>;
    fn rollback_to_tx_handle<'a>(&'a self, tx_id: TxId) -> BoxFuture<'a, Result<()>>;
    fn flush_handle<'a>(&'a self) -> BoxFuture<'a, Result<()>>;
    fn stats_handle<'a>(&'a self) -> BoxFuture<'a, Result<StorageStats>>;
    fn last_seq_no_handle<'a>(&'a self) -> BoxFuture<'a, Result<u64>>;
    fn last_tx_id_handle<'a>(&'a self) -> BoxFuture<'a, Result<TxId>>;
    fn pin_checkpoint_handle<'a>(&'a self, seq_no: u64) -> BoxFuture<'a, Result<()>>;
    fn unpin_checkpoint_handle<'a>(&'a self, seq_no: u64) -> BoxFuture<'a, Result<()>>;
    #[allow(clippy::type_complexity)]
    fn scan_prefix_handle<'a>(
        &'a self,
        prefix: &'a [u8],
    ) -> BoxFuture<'a, Result<Vec<(Vec<u8>, Vec<u8>)>>>;
    #[allow(clippy::type_complexity)]
    fn scan_prefix_bounded_handle<'a>(
        &'a self,
        prefix: &'a [u8],
        limit: usize,
        cursor: Option<&'a [u8]>,
    ) -> BoxFuture<'a, Result<(Vec<(Vec<u8>, Vec<u8>)>, Option<Vec<u8>>)>>;
    #[allow(clippy::type_complexity)]
    fn scan_prefix_at_handle<'a>(
        &'a self,
        prefix: &'a [u8],
        seq_no: u64,
    ) -> BoxFuture<'a, Result<Vec<(Vec<u8>, Vec<u8>)>>>;
    #[allow(clippy::type_complexity)]
    fn scan_handle<'a>(
        &'a self,
        start: Bound<&'a [u8]>,
        end: Bound<&'a [u8]>,
        limit: Option<usize>,
    ) -> BoxFuture<'a, Result<Vec<(Vec<u8>, Vec<u8>)>>>;
    #[allow(clippy::type_complexity)]
    fn scan_bounded_handle<'a>(
        &'a self,
        start: Bound<&'a [u8]>,
        end: Bound<&'a [u8]>,
        limit: usize,
        cursor: Option<&'a [u8]>,
    ) -> BoxFuture<'a, Result<(Vec<(Vec<u8>, Vec<u8>)>, Option<Vec<u8>>)>>;
}
```

**`WalObserver`** — *Synchronous, deterministic observer interface for committed WAL batches.* (`src/lsm/observer.rs`)

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

### Öffentliche Datentypen — Structs (56)

| Struct | Felder / Form | Beschreibung | Datei |
|---|---|---|---|
| `AdaptiveCompactionPlan` | `{ pub strategy: CompactionStrategy, pub candidates: Vec<Arc<SstableReader>>, pub is_full_compaction: bool, }` | Resulting plan proposed by adaptive compaction planner. | `src/compaction/adaptive.rs` |
| `AsyncObserverAdapter` | `{ inner: Arc<dyn WalObserver>, sender: tokio::sync::mpsc::Sender<(u64, TxId, WriteOrigin)>, }` | Non-blocking, bounded async adapter wrapper for `WalObserver` implementations. | `src/lsm/observer.rs` |
| `BlockBuilder` | `{ data: BytesMut, offsets: Vec<u32>, pub(super) block_size: usize, key_hashes: Vec<(u64, u64)>, v3_bloom: [u64; 4], format_version: u16, }` | A builder for SSTable data blocks (Format v4 default, supports v3). | `src/sstable/builder.rs` |
| `BlockCache` | `{ first_shard: BlockCacheShard, additional_shards: Vec<BlockCacheShard>, hash_builder: ahash::RandomState, }` | — | `src/sstable/block_cache.rs` |
| `BloomFilter` | `{ pub(super) bits: Vec<u64>, pub(super) num_hashes: usize, pub(super) num_bits: usize, }` | SPECCED: Speichereffizienter Bloom-Filter für SSTable-Pre-Checks. Ziel: False-Positive-Rate p ≤ 0.01. Formel: m = -n * ln(p) / (ln(2)^2) ≈ 9.6 * n Ha… | `src/sstable/bloom.rs` |
| `CommitContext` | `{ pub tx_id: TxId, pub origin: WriteOrigin, pub durable: bool, }` | Context describing a committed transaction passed to observers. | `src/lsm/observer.rs` |
| `CommitGuard` | `{ pub(super) _lock: &'a tokio::sync::MutexGuard<'a, ()>, }` | Proof that `commit_mutex` is currently held by the calling task. Can only be constructed while holding the mutex guard. | `src/lsm/guard.rs` |
| `CommittedBatch` | `{ pub entries: Vec<WalEntryRef<'a>>, pub origin: WriteOrigin, }` | Batch of committed WAL entries passed to observers. | `src/lsm/observer.rs` |
| `CompactionConfig` | `{ /// Minimum number of SSTables in a size tier to trigger compaction. pub min_sstables_per_tier: usize, /// Size ratio between adjacent tiers (e.g., 4.0 means e… }` | Configuration for the compaction engine. | `src/compaction/config.rs` |
| `CompactionEngine` | `{ pub purge_waiting: std::sync::atomic::AtomicBool, config: CompactionConfig, pub(super) snapshot_registry: Arc<SnapshotRegistry>, block_cache: Arc<BlockCache>,… }` | Orchestriert die periodische Verdichtung und Bereinigung von SSTables. | `src/compaction/engine.rs` |
| `CostBasedAdaptivePlanner` | `{ read_ratio_threshold: f64, min_sstables_per_tier: usize, size_ratio: f64, }` | Reference implementation of cost-based adaptive compaction (EcoTune / ArceKV inspired). Adjusts compaction aggressiveness based on read-vs-write work… | `src/compaction/adaptive.rs` |
| `DiskReserve` | `{ pub required_bytes: u64, pub available_bytes: u64, }` | Struktur zur Berechnung und Überwachung der Plattenreserve. | `src/health/reserve.rs` |
| `DocStoreLayer` | `{ purge: Arc<P>, }` | Repräsentiert die Schicht `doc_store` in der Erasure-Pipeline. | `src/erase/layer.rs` |
| `DrainStats` | `{ pub entries_drained: usize, pub bytes_drained: u64, }` | Statistiken über verworfene Memtable-Einträge. | `src/purge/backend.rs` |
| `InMemoryStorageEngine` | `{ data: Arc<RwLock<StoreDataMap>>, staged: Arc<RwLock<StagedTxMap>>, seq: Arc<AtomicU64>, last_tx: Arc<RwLock<TxId>>, }` | An in-memory implementation of [`StorageEngine`] guaranteed to perform no disk I/O. | `src/engine/memory_only.rs` |
| `IntegrityVerifier` | `{ last_hmac: [u8; 32], integrity_key: Vec<u8>, last_seq_no: Option<u64>, }` | Stateful verifier for WAL HMAC-SHA256 hash chains. | `src/wal/verifier.rs` |
| `KeyManager` | `{ key: VolatileKey, cipher_cache: OnceLock<Aes256GcmSiv>, nonce_prefix: [u8; 4], nonce_counter: AtomicU64, }` | Central key manager for symmetric encryption in the store. | `src/wal/key_manager.rs` |
| `LruBlockCacheBackend` | `{ state: RwLock<LruBlockCacheState>, }` | Standard strict-LRU block cache backend using `parking_lot::RwLock<LruCache>` with byte-based capacity. | `src/sstable/block_cache.rs` |
| `LsmConfig` | `{ /// Pfad zum Datenverzeichnis. pub path: PathBuf, /// Maximale MemTable-Größe vor Flush. pub memtable_size_limit: usize, /// Maximale RAM-Nutzung in MB. pub ma… }` | Konfiguration des LSM-Speichers. Verschlüsselung ist Pflicht (D-05): Der `KeyManager` wird separat über `LsmStorage::open_with_key_manager` injiziert… | `src/lsm/config.rs` |
| `LsmState` | `{ pub(crate) memtable: Arc<MemTable>, pub(crate) immutable_memtables: Vec<Arc<MemTable>>, }` | — | `src/lsm/guard.rs` |
| `LsmStorage` | `{ pub config: LsmConfig, pub(super) key_manager: Option<Arc<KeyManager>>, pub(crate) state: RwLock<LsmState>, /// SSTables stored separately for shared access wi… }` | Primaere Schnittstelle fuer den LSM-Baum mit get, put, delete und scan. | `src/lsm/engine.rs` |
| `Manifest` | `{ pub(super) path: PathBuf, pub(super) file: tokio::sync::Mutex<tokio::fs::File>, pub(super) _dir_lock: Option<DirLock>, }` | Verwaltet die VersionSet-Manifestdatei zur dauerhaften Erfassung aller SSTables. | `src/manifest/core.rs` |
| `ManifestIntervalIndex` | `{ root: Option<Box<IntervalNode>>, }` | Schnellprüfungs-Index für Manifest-SSTable-Intervalle in O(log n + k) (SPEC 5.5). | `src/manifest/core.rs` |
| `MemTable` | `{ /// Sharded storage: each shard holds a disjoint subset of keys. /// Shard selection is deterministic via `shard_for()`. shards: [MemTableShard; SHARD_COUNT],… }` | — | `src/memtable.rs` |
| `NoopReplayProgressSink` | `(tuple/unit)` | — | `src/wal/replay.rs` |
| `ObserverRegistry` | `{ observers: parking_lot::RwLock<Vec<Arc<BoundedObserverWorker>>>, historical_drops: Arc<AtomicU64>, max_observer_latency: parking_lot::RwLock<Duration>, clock:… }` | Configuration and handle for managing WAL observers with bounded delivery and circuit breaker. | `src/lsm/observer.rs` |
| `PoisonState` | `{ poisoned: AtomicBool, }` | Atomarer Poison-Status für Speichertriebwerke. | `src/health/poison.rs` |
| `PreparedBatch` | `(tuple/unit)` | A batch of prepared WAL entries bound to a specific HMAC chain. `PreparedBatch` has a private inner field and cannot be constructed outside `wal.rs`.… | `src/wal/encode.rs` |
| `QuickCacheBlockCacheBackend` | `{ cache: quick_cache::sync::Cache<(u64, u64), Bytes, BlockWeighter>, }` | Lock-optimized block cache backend using `quick_cache::sync::Cache` (S3-FIFO / Clock-based eviction) with byte-based capacity. | `src/sstable/block_cache.rs` |
| `RawStorageHandle` | `{ inner: Arc<LsmStorage>, }` | Kapselt den ungeschützten Zugriff ausschließlich für interne Wiederherstellungspfade. | `src/tenant_scoped/raw.rs` |
| `RecordEnvelopeV2` | `{ pub doc_tag: DocTag, pub nonce: [u8; NONCE_LEN], pub ciphertext: Vec<u8>, pub aead_tag: [u8; AEAD_TAG_LEN], }` | Repräsentiert einen entschlüsselbaren oder versiegelten Record-Envelope v2. | `src/envelope.rs` |
| `RestoreProbe` | `{ probed_count: usize, }` | Restore-Guard-Sondierungsprüfer zur Konsistenzsicherung nach Crash-Wiederherstellung. | `src/erase/probe.rs` |
| `RewriteOutcome` | `{ pub old_file: FileId, pub new_file: FileId, pub records_copied: usize, pub records_purged: usize, pub bytes_written: u64, }` | Ergebnis eines SSTable-Rewrites ohne Ziel-Records. | `src/purge/backend.rs` |
| `SealPoint` | `{ pub seq: u64, pub tail_mac: [u8; 32], }` | Versiegelungspunkt im aktiven WAL-Segment. | `src/purge/backend.rs` |
| `SieveCacheBackend` | `{ state: RwLock<SieveCacheState>, }` | Trait abstraction for block cache backend implementations. ## Approximative LRU & Lock-Optimized Read Path SSTable block caches store immutable 4KB d… | `src/sstable/block_cache.rs` |
| `SstableBuilder` | `{ path: PathBuf, file: File, block_builder: BlockBuilder, index: Vec<(Bytes, u64)>, // (last_key, offset) first_key: Option<Bytes>, last_key: Option<Bytes>, prev… }` | Erzeugt deterministisch geordnete SSTable-Dateien mit Datenbloecken und Index. | `src/sstable/builder.rs` |
| `SstableMetadata` | `{ pub first_key: Bytes, pub last_key: Bytes, pub file_size: u64, pub min_tx_id: u64, pub max_tx_id: u64, pub min_seq: u64, pub max_seq: u64, }` | Metadata for an SSTable. | `src/sstable/builder.rs` |
| `SstableReader` | `{ pub(super) file: Arc<dyn acta_ports::vfs::VfsFile>, pub(super) index: Vec<(Bytes, u64)>, pub(super) metadata: SstableMetadata, /// Byte offset where the index… }` | Liest SSTable-Dateien mit Block-Cache und Bloom-Filter-Unterstuetzung. | `src/sstable/reader.rs` |
| `SstableStream` | `{ pub(super) reader: Arc<SstableReader>, pub(super) block_idx: usize, pub(super) entry_idx: usize, pub(super) current_block: Option<Bytes>, pub(super) num_offset… }` | — | `src/sstable/stream.rs` |
| `StorageVfs` | `(tuple/unit)` | Default filesystem wrapper implementing POSIX durable operations through ports. | `src/vfs.rs` |
| `StorePosixFile` | `{ inner: Mutex<std_file_type::File>, }` | In-crate VfsFile implementation backed by OS file handle for positional reads. | `src/vfs.rs` |
| `SystemPressure` | `{ pub wal_queue_depth: usize, pub scheduler_queue_depth: f32, // 0.0–1.0 (Tokio global scheduler queue depth ratio) pub blocking_util: f32, // 0.0–1.0 (Blocking… }` | — | `src/system_pressure.rs` |
| `SystemPressureMonitor` | `{ pressure_tx: tokio::sync::watch::Sender<SystemPressure>, pub pressure_rx: tokio::sync::watch::Receiver<SystemPressure>, sampling_interval: Duration, }` | — | `src/system_pressure.rs` |
| `TenantKeyCodec` | `{ tenant_id: TenantId, /// Vorberechnetes Scan-Präfix ohne Allokation im Hot-Path. scan_prefix_cache: Vec<u8>, }` | Encodes LSM storage keys with tenant isolation prefixes. | `src/tenant_codec.rs` |
| `TenantScopedStorage` | `{ inner: S, codec: TenantKeyCodec, }` | Tenant-aware `StorageEngine` wrapper implementing strict key-level isolation (INV-TENANT-2). | `src/tenant_codec.rs` |
| `TenantScopedStorage` | `{ tenant: TenantTag, inner: Arc<LsmStorage>, }` | Gekapselter Zugriff auf den LSM-Speicher für einen spezifischen Mandanten (AR-6). | `src/tenant_scoped/scoped.rs` |
| `TextCommitInfo` | `{ pub add: Option<FileId>, pub remove: Vec<FileId>, pub floor_seq: u64, }` | Strukturierte Text-Commit-Informationen (F.4, D-121). | `src/manifest/core.rs` |
| `WalEntry` | `{ /// The operation performed. pub op: WalOp, /// Sequence number assigned to the operation. pub seq_no: u64, /// HMAC of the current entry (includes previous HM… }` | A single entry in the Write-Ahead Log. | `src/wal/encode.rs` |
| `WalEntryRef` | `{ pub entry: &'a WalEntry, }` | Lightweight view reference into a `WalEntry`. | `src/lsm/observer.rs` |
| `WalEntrySnapshot` | `{ pub tx_id: u64, pub seq_no: u64, pub op_type: u8, pub key: Vec<u8>, pub value: Vec<u8>, pub checksum: [u8; 32], pub prev_hmac: [u8; 32], }` | Snapshot of a WAL entry used for cryptographic integrity verification. | `src/wal/verifier.rs` |
| `WalFloor` | `{ pub seq: u64, pub tail_mac: [u8; 32], pub removed_file_ids: Vec<FileId>, }` | Strukturierter WAL-Floor für Recovery und Verkettungsprüfung (F.4, ST-04). | `src/manifest/core.rs` |
| `WalFloor` | `{ pub seq: u64, pub tail_mac: [u8; 32], pub removed_file_ids: Vec<FileId>, }` | Manifest-Marker für entfernte WAL-Segmente. | `src/purge/backend.rs` |
| `WalFlusherConfig` | `{ /// Maximum time to wait for additional messages after receiving the first, /// before issuing `sync_all()`. Set to 0 to disable (immediate flush). /// /// Typ… }` | Configuration for the WAL background flusher actor. Controls how aggressively the flusher coalesces concurrent writes into a single `sync_all()` call… | `src/wal/flusher.rs` |
| `WalHmac` | `{ mac: Hmac<Sha256>, }` | Stateful wrapper around HMAC-SHA256 initialized with WAL domain separation. | `src/wal/verifier.rs` |
| `WorkloadMetrics` | `{ read_count: AtomicU64, write_count: AtomicU64, last_seq_no: AtomicU64, }` | Operational workload metrics captured deterministically without wall-clock time (P28 compliance). All trigger decisions are strictly derived from tra… | `src/compaction/adaptive.rs` |
| `WorkloadMetricsSnapshot` | `{ pub read_count: u64, pub write_count: u64, pub last_seq_no: u64, }` | Point-in-time snapshot of operational workload metrics. | `src/compaction/adaptive.rs` |

### Öffentliche Datentypen — Enums (11)

| Enum | Varianten | Beschreibung | Datei |
|---|---|---|---|
| `CompactionStrategy` | `/// Standard Size-Tiered Compaction Strategy optimized for write throughput. WriteOptimizedSTCS, /// Aggressive compaction merging more tiers / full LSM to reduce read amplification. ReadOptimizedAgg…` | Compaction strategy variants selected by adaptive planner. | `src/compaction/adaptive.rs` |
| `DurabilityConfigError` | `#[error("DurabilityMode::{mode:?} is incompatible with feature '{feature}': {reason}")] IncompatibleCombination { mode: DurabilityMode, feature: &'static str, reason: &'static str, },` | — | `src/lsm/config.rs` |
| `DurabilityMode` | `/// Vollständige Persistenz mit HMAC-Kette (einzige zulässige Variante, D-05). #[default] Full, /// Nur für In-Memory-Tests ohne Dateisystem-I/O. MemoryOnly,` | WAL-Dauerhaftigkeitsmodus. Nur `Full` ist zulässig (D-05). `WalNoHmac` und `MemoryOnly` wurden entfernt, da sie die WAL-Integrität abschwächen würden… | `src/lsm/config.rs` |
| `LegacyKeyStatus` | `/// A standard, secure integrity key (e.g. from `.wal_integrity_key` or `KeyManager`) is in use. Standard, /// The legacy obfuscated key is active and requires migration. LegacyActive,` | Status indicating whether a WAL segment or key configuration uses the obfuscated legacy key. | `src/wal/hmac.rs` |
| `ManifestEntry` | `/// A new SSTable file was created and fully fsynced. Add { path: PathBuf, max_tx: u64 }, /// An SSTable file was removed or superseded. Remove { path: PathBuf }, /// A transaction rollback operation…` | An entry in the SSTable manifest log. | `src/manifest/entry.rs` |
| `PressureLevel` | `Normal, Elevated, // Warnung: System unter Last Critical, // Backpressure aktiv: neue Inserts verlangsamen` | — | `src/system_pressure.rs` |
| `StorageHealth` | `Healthy, FlushFailing, Poisoned,` | Health status of the storage engine. | `src/lsm/engine.rs` |
| `WalOp` | `/// Inserts or updates a key-value pair. Put { tx_id: TxId, key: Vec<u8>, value: Vec<u8>, }, /// Deletes a key. Delete { tx_id: TxId, key: Vec<u8> }, /// Transaction outcome marker (committed / abort…` | — | `src/wal/encode.rs` |
| `WalParseError` | `#[error("WAL entry too short for CRC header")] HeaderTooShort, #[error("CRC mismatch: stored={stored:#010x}, computed={computed:#010x}")] CrcMismatch { stored: u32, computed: u32 }, #[error("WAL payl…` | Typed parsing errors encountered when decoding WAL entries from raw bytes. | `src/wal/encode.rs` |
| `WalVersion` | `V1, // Legacy: kein HMAC V2, // Current: HMAC ohne tx_id V3, // New: HMAC mit tx_id` | — | `src/wal/encode.rs` |
| `WriteOrigin` | `/// Regular user/application write transaction. UserWrite,` | Origin tag indicating where a committed batch originated from. | `src/lsm/observer.rs` |

### Typ-Aliase (3)

| Alias | Definition | Beschreibung |
|---|---|---|
| `BlockCacheShard` | `type BlockCacheShard= LruBlockCacheBackend` | — |
| `BlockCacheShard` | `type BlockCacheShard= QuickCacheBlockCacheBackend` | — |
| `WalSeq` | `type WalSeq= u64` | — |

### Konstanten (26)

| Konstante | Typ | Beschreibung |
|---|---|---|
| `AEAD_TAG_LEN` | `usize` | — |
| `BLOCKING_UTIL_CRITICAL` | `f32` | — |
| `BLOCKING_UTIL_ELEVATED` | `f32` | — |
| `BLOCK_CACHE_SHARDS` | `usize` | Sharded block cache: 64 independent shards to reduce contention. Shard selection uses ahash tuple hashing on (file_id, offset). |
| `CURRENT_MANIFEST_VERSION` | `u8` | — |
| `DEFAULT_MAX_OBSERVER_LATENCY` | `Duration` | Default maximum latency allowed for WAL observer callback execution (1 millisecond). |
| `DOC_STORE_LAYER_ID` | `LayerId` | Konstante Schicht-ID für den Dokumentenspeicher (`doc_store`). |
| `ENVELOPE_V2_ALG_ID` | `u8` | — |
| `ENVELOPE_V2_VERSION` | `u8` | — |
| `HEADER_LEN` | `usize` | — |
| `MANIFEST_HEADER_MAGIC` | `&[u8` | — |
| `MIN_DISK_RESERVE_BYTES` | `u64` | Mindestplattenreserve in Bytes (1 GiB laut SPEC D-65 / ST-13). |
| `NONCE_LEN` | `usize` | — |
| `OP_TAG_SST_SWAP` | `u8` | Op-Tag für atomaren SSTable-Tausch (ADR-Vorschlag SL-1 aus P02). SPEC-Änderung SL-1 ausstehend. Intern gehaltener Op-Tag 0x07 für einen unteilbaren T… |
| `OP_TAG_TEXT_COMMIT` | `u8` | Op-Tag für TextCommit (SPEC F.4: 0x06). |
| `OP_TAG_WAL_FLOOR` | `u8` | Op-Tag für WalFloor (SPEC F.4: 0x03). |
| `RESERVED_PREFIXES` | `&[&[u8]]` | List of known reserved system key prefixes. |
| `ROLLBACK_INLINE_THRESHOLD_ENTRIES` | `usize` | Maximum threshold for surviving entries during rollback below which entries are retained in memtable instead of creating a new SSTable (M-4 optimizat… |
| `SCHEDULER_QUEUE_CRITICAL` | `f32` | — |
| `SCHEDULER_QUEUE_ELEVATED` | `f32` | — |
| `SSTABLE_MAGIC_LEGACY` | `u32` | — |
| `SSTABLE_MAGIC_MFSX` | `u32` | Binary search for a key index inside a SSTable data block. Returns `Ok(Ok(index))` if the exact key is found, or `Ok(Err(index))` indicating the inse… |
| `WAL_QUEUE_CRITICAL_THRESHOLD` | `usize` | — |
| `WAL_QUEUE_ELEVATED_THRESHOLD` | `usize` | — |
| `WAL_V2_HEADER` | `[u8` | Magic header for V2 batch-encrypted WAL files (`b"MFW2"`). |
| `WAL_V3_HEADER` | `[u8` | Magic header for V3 WAL files (`b"MFW3"`). |

### Öffentliche Funktionen (299)

| Signatur | Beschreibung | Datei |
|---|---|---|
| `pub fn new() -> Self` | Creates a new WorkloadMetrics counter instance initialized to zero. | `src/compaction/adaptive.rs` |
| `pub fn new(read_ratio_threshold: f64, min_sstables_per_tier: usize, size_ratio: f64) -> Self` | Creates a new CostBasedAdaptivePlanner with configurable read ratio threshold. | `src/compaction/adaptive.rs` |
| `pub fn record_read(&self)` | Records a read operation atomically. | `src/compaction/adaptive.rs` |
| `pub fn record_write(&self, seq_no: u64)` | Records a write operation and updates the latest sequence number atomically. | `src/compaction/adaptive.rs` |
| `pub fn snapshot(&self) -> WorkloadMetricsSnapshot` | Takes a deterministic snapshot of current metrics for compaction strategy evaluation. | `src/compaction/adaptive.rs` |
| `pub fn validate_tombstone_safety( candidates: &[Arc<SstableReader>], min_active_seqno: u64, is_full_compaction: bool, ) -> bool` | Hard invariant verification enforcing INV-COMPACTION-ADAPTIVE-1. Validates that no candidate tombstone with `seq >= min_active_seqno` can e… | `src/compaction/adaptive.rs` |
| `pub fn generate_sst_path(&self, data_path: &std::path::Path) -> Result<PathBuf>` | Generates a unique SSTable file path using a monotonic counter. | `src/compaction/engine.rs` |
| `pub async fn maybe_compact( &self, sstables: &RwLock<Vec<Arc<SstableReader>>>, data_path: &std::path::Path, ) -> Result<bool>` | — | `src/compaction/engine.rs` |
| `pub async fn maybe_compact_with_cancel( &self, sstables: &RwLock<Vec<Arc<SstableReader>>>, data_path: &std::path::Path, cancel_token: Option<&tokio_util::sync::CancellationToken>, ) -> Result<bool>` | Evaluates whether compaction should run and performs it with an optional cancellation token. | `src/compaction/engine.rs` |
| `pub async fn merge_sstables( &self, inputs: &[Arc<SstableReader>], output_path: &std::path::Path, min_snapshot_seq: u64, is_full_compaction: bool, ) -> Result<()>` | Performs a multi-way merge of input SSTables into a single output SSTable. During merge: - Duplicate keys: newest sequence number wins - To… | `src/compaction/engine.rs` |
| `pub async fn merge_sstables_with_cancel( &self, inputs: &[Arc<SstableReader>], output_path: &std::path::Path, min_snapshot_seq: u64, is_full_compaction: bool, cancel_token: Option<&tokio_util::sync::CancellationToken>, ) -> Result<()>` | Performs a multi-way merge with an optional cancellation token. | `src/compaction/engine.rs` |
| `pub fn new( config: CompactionConfig, snapshot_registry: Arc<SnapshotRegistry>, block_cache: Arc<BlockCache>, key_manager: Option<Arc<KeyManager>>, budget: Arc<acta_ports::ResourceTracker>, manifest: Option<Arc<crate::manifest::Manifest>>, ) -> Self` | Creates a new compaction engine. | `src/compaction/engine.rs` |
| `pub fn record_read_op(&self)` | Records a read operation for workload tracking. | `src/compaction/engine.rs` |
| `pub fn record_write_op(&self, seq_no: u64)` | Records a write operation for workload tracking. | `src/compaction/engine.rs` |
| `pub async fn run_loop( self: Arc<Self>, sstables: Arc<RwLock<Vec<Arc<SstableReader>>>>, data_path: PathBuf, shutdown: tokio_util::sync::CancellationToken, )` | Runs the background compaction loop. Periodically checks if compaction is needed and performs it. Designed to be spawned via `tokio::spawn`. | `src/compaction/engine.rs` |
| `pub fn select_compaction_candidates( &self, ssts: &[Arc<SstableReader>], ) -> Option<Vec<Arc<SstableReader>>>` | Selects SSTables to compact using Size-Tiered strategy. Groups by size class and returns the group that meets the threshold, prioritizing k… | `src/compaction/engine.rs` |
| `pub fn with_adaptive_planner(mut self, planner: Arc<dyn AdaptiveCompactionPlanner>) -> Self` | Attaches a custom adaptive compaction planner. | `src/compaction/engine.rs` |
| `pub fn with_clock(mut self, clock: Arc<dyn acta_ports::Clock>) -> Self` | Attaches a custom clock port for deterministic execution timing (P28). | `src/compaction/engine.rs` |
| `pub fn with_merge_operator(mut self, merge_operator: Arc<dyn MergeOperator>) -> Self` | Attaches a custom merge operator for compaction value merging (§4.12). | `src/compaction/engine.rs` |
| `pub fn with_pressure_rx( mut self, rx: tokio::sync::watch::Receiver<crate::system_pressure::SystemPressure>, ) -> Self` | Attaches a system pressure watch receiver to enable pressure-aware compaction backpressure. | `src/compaction/engine.rs` |
| `pub fn retain_key_versions<T, F>(entries: Vec<T>, get_seq: F, floor_seq: u64) -> Vec<T> where F: Fn(&T) -> u64,` | Filters a sequence of entries for a SINGLE key (sorted by raw sequence DESCENDING) according to MVCC retention rules given a `floor` sequen… | `src/compaction/retention.rs` |
| `pub fn new() -> Self` | Creates a new, empty in-memory storage engine. | `src/engine/memory_only.rs` |
| `pub fn construct_aad(key: &KeyBuf) -> Vec<u8>` | Konstruiert AAD für AEAD: `"ctx/rec/v2" ‖ key_bytes`. | `src/envelope.rs` |
| `pub fn decode(bytes: &[u8]) -> Result<Self>` | Dekodiert und validiert einen Envelope aus Rohbytes. Gibt `ActaError::Corrupt` zurück, falls Version != 2 oder zu kurz. | `src/envelope.rs` |
| `pub fn encode(&self) -> Vec<u8>` | Kodiert den Envelope in Bytes gemäß F.6. | `src/envelope.rs` |
| `pub fn new(purge: Arc<P>) -> Self` | — | `src/erase/layer.rs` |
| `pub fn new() -> Self` | — | `src/erase/probe.rs` |
| `pub fn probe_key_consistency( &mut self, key: &[u8], expected_digest: &[u8; 32], ) -> Result<bool>` | Prüft die Konsistenz des wiederhergestellten Schlüsselraums gegen gespeicherte Prüfsummen. | `src/erase/probe.rs` |
| `pub fn probed_count(&self) -> usize` | Anzahl durchgeführter Sondierungen. | `src/erase/probe.rs` |
| `pub async fn fsync_parent_dir(path: &Path) -> Result<()>` | Alias for `sync_parent_dir` for backward compatibility across storage modules. | `src/fsync.rs` |
| `pub async fn sync_file(path: &Path) -> Result<()>` | Performs fsync on a single file at `path`. # Errors Returns `ActaError::Storage` on Unix if opening or syncing fails. Returns `ActaError::I… | `src/fsync.rs` |
| `pub async fn sync_parent_dir(path: &Path) -> Result<()>` | Performs fsync on the parent directory of `path`. Directory fsync is required on POSIX filesystems to guarantee that newly created files or… | `src/fsync.rs` |
| `pub fn check_healthy(&self) -> Result<()>` | Weist mutierende Operationen mit `ActaError::Io` ab, wenn vergiftet (D-55). | `src/health/poison.rs` |
| `pub fn is_poisoned(&self) -> bool` | Prüft, ob der Speicher vergiftet ist. | `src/health/poison.rs` |
| `pub fn new() -> Self` | Erzeugt einen neuen unvergifteten Zustand. | `src/health/poison.rs` |
| `pub fn poison(&self)` | Vergiftet den Speicherzustand dauerhaft nach einem fsync- oder I/O-Fehler (D-55). | `src/health/poison.rs` |
| `pub fn check_free_space(&self, _path: &Path, available_bytes: u64) -> Result<()>` | Prüft den verfügbaren Speicherplatz auf dem Dateisystem von `path`. Weist mutierende Schreibzugriffe mit `Limit::DiskReserve` ab, wenn der… | `src/health/reserve.rs` |
| `pub fn for_candidate(largest_candidate_bytes: u64) -> Self` | Erzeugt eine neue Plattenreserve basierend auf der größten bekannten Kandidatendatei. Formel laut ST-13: `max(1 GiB, 2 × größte Kandidatend… | `src/health/reserve.rs` |
| `pub fn has_sufficient_space(&self) -> bool` | Prüft, ob ausreichend Speicherplatz vorhanden ist. | `src/health/reserve.rs` |
| `pub fn required_bytes(&self) -> u64` | Liefert die geforderten Reserve-Bytes. | `src/health/reserve.rs` |
| `pub fn cleanup_intent_locks_where<F>(&self, mut predicate: F) where F: FnMut(TxId) -> bool,` | Clears intent locks matching a predicate, gracefully recovering from poisoned locks. | `src/lsm/commit.rs` |
| `pub fn clear_intent_locks_above_tx(&self, target_tx: TxId)` | Removes all intent lock registrations associated with transactions strictly newer than target_tx. | `src/lsm/commit.rs` |
| `pub fn clear_intent_locks_for_tx(&self, tx_id: TxId)` | — | `src/lsm/commit.rs` |
| `pub fn block_cache_for_test(&self) -> Arc<BlockCache>` | — | `src/lsm/engine.rs` |
| `pub fn budget_for_test(&self) -> Arc<ResourceTracker>` | — | `src/lsm/engine.rs` |
| `pub fn budget_tracking_drift_bytes(&self) -> u64` | Returns the accumulated total memory budget tracking drift in bytes caused by unbudgeted memtable puts during commit when memory limit was… | `src/lsm/engine.rs` |
| `pub fn clock(&self) -> Arc<dyn acta_ports::Clock>` | Returns the currently configured clock port. | `src/lsm/engine.rs` |
| `pub async fn close(&self) -> Result<()>` | Gracefully closes the storage engine, stopping background tasks and flushing active memtable to disk. | `src/lsm/engine.rs` |
| `pub fn data_path(&self) -> &std::path::Path` | Returns the data directory path. | `src/lsm/engine.rs` |
| `pub fn deregister_observer(&self, observer: &Arc<dyn WalObserver>)` | Deregisters an observer matching the provided `Arc` reference via pointer equality. | `src/lsm/engine.rs` |
| `pub async fn get_at_seq_tracked( &self, tx_id: TxId, key: &[u8], snapshot_seq: u64, ) -> Result<Option<Bytes>>` | Performs a tracked read for transaction `tx_id` at an explicit `snapshot_seq`. Registers the read key in `TxBuffer` at `snapshot_seq` and r… | `src/lsm/engine.rs` |
| `pub async fn get_tracked(&self, tx_id: TxId, key: &[u8]) -> Result<Option<Bytes>>` | Performs a tracked read for transaction `tx_id`. Evaluates `snapshot_seq = next_seq_no - 1` ONCE (K2), registers the read key in `TxBuffer`… | `src/lsm/engine.rs` |
| `pub fn has_observers(&self) -> bool` | Returns true if at least one observer is registered. | `src/lsm/engine.rs` |
| `pub async fn has_pending_legacy_wal_migration(&self) -> Result<bool>` | Checks if the current storage instance's data directory contains any pending legacy WAL files. Dies ist der Migrationsmechanismus vor der e… | `src/lsm/engine.rs` |
| `pub async fn has_pending_legacy_wal_migration_path(path: &std::path::Path) -> Result<bool>` | Checks if a database directory contains any legacy WAL files requiring explicit migration. Dies ist der Migrationsmechanismus vor der endgü… | `src/lsm/engine.rs` |
| `pub fn health(&self) -> StorageHealth` | — | `src/lsm/engine.rs` |
| `pub fn is_poisoned(&self) -> bool` | Returns the current storage health status. Gibt an, ob der Speicher nach einem I/O- oder Fsync-Fehler vergiftet ist (D-55). | `src/lsm/engine.rs` |
| `pub fn last_applied_seq(&self) -> u64` | Returns the highest sequence number applied to the MemTable and published. | `src/lsm/engine.rs` |
| `pub fn manifest_for_test(&self) -> Arc<crate::manifest::Manifest>` | — | `src/lsm/engine.rs` |
| `pub fn max_observer_latency(&self) -> std::time::Duration` | Returns the current maximum latency threshold for observer callbacks. | `src/lsm/engine.rs` |
| `pub async fn maybe_compact_for_test(&self) -> Result<bool>` | — | `src/lsm/engine.rs` |
| `pub fn metrics_sink(&self) -> Arc<dyn acta_ports::MetricsSink>` | Returns the currently configured [`MetricsSink`]. | `src/lsm/engine.rs` |
| `pub async fn migrate_legacy_wal_keys( config: &LsmConfig, clock: Arc<dyn acta_ports::Clock>, ) -> Result<usize>` | Explicitly migrates all legacy WAL files in the specified directory using the provided clock port. Returns the number of migrated WAL files… | `src/lsm/engine.rs` |
| `pub fn next_seq_no_for_test(&self) -> u64` | Returns the current `next_seq_no` value for testing verification. | `src/lsm/engine.rs` |
| `pub async fn open(config: LsmConfig) -> Result<Self>` | Opens or recovers an LSM storage instance with the given configuration. | `src/lsm/engine.rs` |
| `pub async fn open_with_key_manager( config: LsmConfig, key_manager: Option<Arc<KeyManager>>, ) -> Result<Self>` | Opens or recovers an LSM storage instance with the given configuration and a KeyManager. | `src/lsm/engine.rs` |
| `pub async fn open_with_merge_operator( config: LsmConfig, merge_operator: Arc<dyn MergeOperator>, ) -> Result<Self>` | Opens or recovers an LSM storage instance with the given configuration and a `MergeOperator`. | `src/lsm/engine.rs` |
| `pub fn poison(&self)` | Vergiftet den Speicher nach einem schwerwiegenden I/O- oder Fsync-Fehler dauerhaft (D-55). | `src/lsm/engine.rs` |
| `pub fn pressure_receiver( &self, ) -> tokio::sync::watch::Receiver<crate::system_pressure::SystemPressure>` | Returns a watch receiver for monitoring system pressure levels. | `src/lsm/engine.rs` |
| `pub fn register_observer(&self, observer: Arc<dyn WalObserver>)` | Registers a new WAL observer. | `src/lsm/engine.rs` |
| `pub fn request_flush(&self)` | Requests an asynchronous background flush without blocking the caller or holding commit locks. | `src/lsm/engine.rs` |
| `pub async fn restore_wal_file_handle_for_test(&self)` | — | `src/lsm/engine.rs` |
| `pub async fn scan_prefix_tracked( &self, tx_id: TxId, prefix: &[u8], ) -> Result<Vec<(Vec<u8>, Vec<u8>)>>` | Performs a tracked prefix scan for transaction `tx_id`. Evaluates `snapshot_seq = last_applied_seq` ONCE, scans all matching entries, and r… | `src/lsm/engine.rs` |
| `pub async fn seal_wal_at(&self) -> Result<crate::purge::backend::SealPoint>` | Versiegelt das aktive WAL-Segment (ST-04). | `src/lsm/engine.rs` |
| `pub fn set_clock(&self, clock: Arc<dyn acta_ports::Clock>)` | Sets the injected clock port for deterministic execution timing (P28). | `src/lsm/engine.rs` |
| `pub fn set_max_observer_latency(&self, latency: std::time::Duration)` | Sets the maximum latency threshold for observer callbacks. | `src/lsm/engine.rs` |
| `pub fn set_metrics_sink(&self, sink: Arc<dyn acta_ports::MetricsSink>)` | Sets the [`MetricsSink`] for recording operational metrics. | `src/lsm/engine.rs` |
| `pub fn set_ssi_validator( &mut self, validator: Arc<dyn acta_mvcc::SsiValidator + Send + Sync>, )` | Sets or replaces the [`SsiValidator`] port for transaction conflict detection. | `src/lsm/engine.rs` |
| `pub fn shutdown(&self)` | Signals the background compaction engine to stop. | `src/lsm/engine.rs` |
| `pub async fn simulate_wal_append_failure_for_test(&self)` | — | `src/lsm/engine.rs` |
| `pub async fn simulate_wal_append_failure_for_tx_for_test(&self, _tx_id: u64)` | — | `src/lsm/engine.rs` |
| `pub fn spawn_tracked<F>(&self, future: F) where F: std::future::Future<Output = ()> + Send + 'static,` | Spawns a background task tracked by this storage instance. | `src/lsm/engine.rs` |
| `pub fn sstables_for_test(&self) -> Arc<RwLock<Vec<Arc<SstableReader>>>>` | — | `src/lsm/engine.rs` |
| `pub async fn wait_shutdown(&self)` | Waits for all spawned tasks to shut down fully. | `src/lsm/engine.rs` |
| `pub fn with_clock(self, clock: Arc<dyn acta_ports::Clock>) -> Self` | Configures the clock port fluently on `self` and returns `self`. | `src/lsm/engine.rs` |
| `pub fn with_metrics_sink(self, sink: Arc<dyn acta_ports::MetricsSink>) -> Self` | Configures the [`MetricsSink`] fluently on `self` and returns `self`. | `src/lsm/engine.rs` |
| `pub fn with_ssi_validator( mut self, validator: Arc<dyn acta_mvcc::SsiValidator + Send + Sync>, ) -> Self` | Configures the [`SsiValidator`] fluently on `self` and returns `self`. | `src/lsm/engine.rs` |
| `pub async fn force_flush(&self) -> Result<()>` | — | `src/lsm/flush.rs` |
| `pub async fn maybe_compact(&self) -> Result<bool>` | Evaluates whether compaction should run and performs it if needed. Uses the persistent CompactionEngine instance to maintain a stable count… | `src/lsm/flush.rs` |
| `pub fn checksum(&self) -> &'a [u8` | Returns a reference to the entry's checksum. | `src/lsm/observer.rs` |
| `pub fn clear_circuit_breaker(&self, observer: &Arc<dyn WalObserver>)` | Resets the circuit breaker state for a specific observer. | `src/lsm/observer.rs` |
| `pub fn deregister_observer(&self, observer: &Arc<dyn WalObserver>)` | Deregisters an observer matching the provided `Arc` reference via pointer equality. | `src/lsm/observer.rs` |
| `pub fn dropped_count(&self) -> u64` | Returns the total number of dropped events across all observers (including deregistered ones). | `src/lsm/observer.rs` |
| `pub fn dropped_count_for(&self, observer: &Arc<dyn WalObserver>) -> u64` | Returns the number of dropped events for a specific registered observer. | `src/lsm/observer.rs` |
| `pub fn is_any_circuit_breaker_open(&self) -> bool` | Returns whether any registered observer currently has an open circuit breaker. | `src/lsm/observer.rs` |
| `pub fn is_circuit_breaker_open(&self, observer: &Arc<dyn WalObserver>) -> bool` | Returns whether the circuit breaker is currently open for a specific observer. | `src/lsm/observer.rs` |
| `pub fn is_empty(&self) -> bool` | Returns true if no observers are registered. | `src/lsm/observer.rs` |
| `pub fn max_observer_latency(&self) -> Duration` | Returns the current maximum latency threshold. | `src/lsm/observer.rs` |
| `pub fn new(entry: &'a WalEntry) -> Self` | Creates a new `WalEntryRef` from a `WalEntry`. | `src/lsm/observer.rs` |
| `pub fn new( inner: Arc<dyn WalObserver>, capacity: usize, ) -> (Self, tokio::task::JoinHandle<()>)` | — | `src/lsm/observer.rs` |
| `pub fn new() -> Self` | Creates a new, empty `ObserverRegistry`. | `src/lsm/observer.rs` |
| `pub fn notify(&self, entries: &[WalEntry], seq_no: u64, tx_id: TxId, origin: WriteOrigin)` | Synchronously notifies all registered observers of a committed batch. | `src/lsm/observer.rs` |
| `pub fn notify_with_context(&self, entries: &[WalEntry], seq_no: u64, ctx: CommitContext)` | Synchronously notifies all registered observers of a committed batch with explicit CommitContext. When invoked within a multi-threaded Toki… | `src/lsm/observer.rs` |
| `pub fn op(&self) -> &'a WalOp` | Returns a reference to the operation inside the entry. | `src/lsm/observer.rs` |
| `pub fn prev_hmac(&self) -> &'a [u8` | Returns a reference to the entry's previous HMAC checksum. | `src/lsm/observer.rs` |
| `pub fn register_observer(&self, observer: Arc<dyn WalObserver>)` | Registers a new WAL observer. | `src/lsm/observer.rs` |
| `pub fn seq_no(&self) -> u64` | Returns the sequence number of the entry. | `src/lsm/observer.rs` |
| `pub fn set_clock(&self, clock: Arc<dyn Clock>)` | Sets the injected clock port for deterministic execution timing (P28). | `src/lsm/observer.rs` |
| `pub fn set_max_observer_latency(&self, latency: Duration)` | Sets the maximum latency threshold for observer callbacks. | `src/lsm/observer.rs` |
| `pub fn tx_id(&self) -> TxId` | Returns the transaction ID associated with this entry. | `src/lsm/observer.rs` |
| `pub async fn new(config: LsmConfig) -> Result<Self>` | — | `src/lsm/recovery.rs` |
| `pub async fn new_with_key_manager_and_merge_operator( config: LsmConfig, key_manager: Option<Arc<KeyManager>>, merge_operator: Option<Arc<dyn MergeOperator>>, ) -> Result<Self>` | — | `src/lsm/recovery.rs` |
| `pub async fn new_with_merge_operator( config: LsmConfig, merge_operator: Option<Arc<dyn MergeOperator>>, ) -> Result<Self>` | — | `src/lsm/recovery.rs` |
| `pub async fn rollback_to_tx(&self, target_tx: TxId) -> Result<()>` | — | `src/lsm/recovery.rs` |
| `pub async fn point_lookup_metrics(&self, key: &[u8]) -> (usize, usize, usize, usize, bool)` | Evaluates detailed traversal metrics (evaluated_sstables, bloom_passes, range_passes, block_reads, found) for point lookups. | `src/lsm/scan.rs` |
| `pub fn derive_doc_id(key: &[u8]) -> DocId` | — | `src/lsm/validate.rs` |
| `pub fn derive_doc_id(key: &[u8]) -> DocId` | — | `src/lsm/validate.rs` |
| `pub fn validate_key(key: &[u8]) -> Result<()>` | — | `src/lsm/validate.rs` |
| `pub fn validate_value(value: &[u8]) -> Result<()>` | — | `src/lsm/validate.rs` |
| `pub async fn append(&self, entry: &ManifestEntry) -> Result<()>` | Appends a new entry to the manifest and performs flush + fsync. | `src/manifest/core.rs` |
| `pub async fn append_batch(&self, entries: &[ManifestEntry]) -> Result<()>` | Appends multiple entries to the manifest in a single atomic batch with flush + fsync. | `src/manifest/core.rs` |
| `pub async fn append_sst_swap(&self, add: &[PathBuf], remove: &[PathBuf]) -> Result<()>` | Hängt einen atomaren SSTable-Tausch (SstSwap) an (ADR SL-1 ausstehend). | `src/manifest/core.rs` |
| `pub async fn append_text_commit( &self, add: Option<FileId>, remove: Vec<FileId>, floor_seq: u64, ) -> Result<()>` | Hängt einen TextCommit-Eintrag an und führt flush + fsync aus (F.4 / D-121). | `src/manifest/core.rs` |
| `pub async fn append_wal_floor( &self, seq: u64, tail_mac: [u8; 32], removed_file_ids: Vec<FileId>, ) -> Result<()>` | Hängt einen WalFloor-Eintrag an und erzwingt Durability via flush + fsync (F.4). | `src/manifest/core.rs` |
| `pub fn candidates_intersecting( &self, query_lo: impl AsRef<[u8]>, query_hi: impl AsRef<[u8]>, ) -> Vec<PathBuf>` | Ermittelt alle Kandidatendateien, deren Intervall [lo, hi] den Bereich [query_lo, query_hi] schneidet. Komplexität: O(log n + k), wobei k d… | `src/manifest/core.rs` |
| `pub fn current_text_commit(entries: &[ManifestEntry]) -> Option<TextCommitInfo>` | Liefert den aktuellsten (letzten) TextCommit aus den Manifest-Einträgen. | `src/manifest/core.rs` |
| `pub fn current_wal_floor(entries: &[ManifestEntry]) -> Option<WalFloor>` | Liefert den aktuellsten (letzten) WalFloor aus den Manifest-Einträgen. | `src/manifest/core.rs` |
| `pub fn extract_high_water_mark(entries: &[ManifestEntry]) -> Option<[u8` | Extracts the most recent WAL high-water-mark HMAC from a list of manifest entries. | `src/manifest/core.rs` |
| `pub fn extract_text_commits(entries: &[ManifestEntry]) -> Vec<TextCommitInfo>` | Extrahiert alle TextCommit-Einträge in logischer Reihenfolge. | `src/manifest/core.rs` |
| `pub fn insert(&mut self, path: impl Into<PathBuf>, lo: impl AsRef<[u8]>, hi: impl AsRef<[u8]>)` | Fügt ein Dateipfad-Intervall [lo, hi] in den Index ein. | `src/manifest/core.rs` |
| `pub async fn load(path: &Path) -> Result<Vec<ManifestEntry>>` | Loads all valid manifest entries from `path`. If the file does not exist, returns an empty vector. Tail-truncation (an incomplete frame at… | `src/manifest/core.rs` |
| `pub fn new() -> Self` | Erzeugt einen neuen leeren Intervallindex. | `src/manifest/core.rs` |
| `pub async fn open(path: impl AsRef<Path>) -> Result<Self>` | Opens or creates an append-only Manifest file at `path`. | `src/manifest/core.rs` |
| `pub fn path(&self) -> &Path` | — | `src/manifest/core.rs` |
| `pub fn reconstruct_dead_sstables( entries: &[ManifestEntry], ) -> std::collections::BTreeSet<PathBuf>` | Reconstructs the set of SSTable file names (or path components) that are proven dead (explicitly removed via `Remove` or `Replace.removed`)… | `src/manifest/core.rs` |
| `pub fn reconstruct_valid_sstables(entries: &[ManifestEntry]) -> Vec<(PathBuf, u64)>` | Reconstructs the ordered list of currently valid SSTable file names (or path components) and their rank (shadowing order: smaller rank = ol… | `src/manifest/core.rs` |
| `pub fn from_bytes(data: &[u8]) -> Result<Self>` | Deserializes a manifest entry from a slice containing `[crc32: u32 LE] [payload...]`. | `src/manifest/entry.rs` |
| `pub fn to_bytes(&self) -> Result<Vec<u8>>` | Serializes the entry into binary format with a 4-byte total size prefix and a 4-byte CRC32. | `src/manifest/entry.rs` |
| `pub async fn maybe_rollover( &self, live_entries: &[ManifestEntry], threshold_bytes: u64, ) -> Result<bool>` | Evaluates whether the MANIFEST file size exceeds `threshold_bytes` and performs an atomic rollover if needed. Rollover process: (a) Write c… | `src/manifest/rollover.rs` |
| `pub async fn rollover(&self, live_entries: &[ManifestEntry]) -> Result<()>` | Evaluates whether the MANIFEST file size exceeds `threshold_bytes` and performs an atomic rollover if needed. Rollover process: (a) Write c… | `src/manifest/rollover.rs` |
| `pub fn get(&self, key: &[u8]) -> Option<(Bytes, u64)>` | Retrieves the latest value and sequence number by key. | `src/memtable.rs` |
| `pub fn get_at_seq(&self, key: &[u8], seq_no: u64, max_tx: u64) -> Option<(Bytes, u64, u64)>` | Retrieves a value, sequence number, and transaction ID by key at or below a specific sequence number and bounded by maximum transaction ID… | `src/memtable.rs` |
| `pub fn is_empty(&self) -> bool` | Returns true if the memtable is empty. WARNING: This reads all shards sequentially. The result is not an atomic snapshot, but this method i… | `src/memtable.rs` |
| `pub fn is_valid_range_bounds(start: Bound<&[u8]>, end: Bound<&[u8]>) -> bool` | Validates whether start and end key range bounds define a non-empty, valid interval (`start <= end`). | `src/memtable.rs` |
| `pub fn iter(&self) -> Vec<(Bytes, Bytes, u64, u64)>` | Iterates over all entries (all versions) in sorted key order. Returns (Key, Value, SeqNo, TxId). | `src/memtable.rs` |
| `pub fn iter_latest(&self) -> Vec<(Bytes, Bytes, u64, u64)>` | Iterates over only the latest version of each key in sorted key order. Returns (Key, Value, SeqNo, TxId). | `src/memtable.rs` |
| `pub fn new() -> Self` | Creates a new empty MemTable with `SHARD_COUNT` independent shards. | `src/memtable.rs` |
| `pub fn put(&self, key: Bytes, value: Bytes, seq_no: u64, tx_id: u64)` | Inserts a key-value pair with a sequence number and transaction ID. ## Concurrent Sequence Invariant Sequence numbers pre-allocated to conc… | `src/memtable.rs` |
| `pub fn retain<F>(&self, mut f: F) where F: FnMut(&[u8]) -> bool,` | Retains only the elements specified by the predicate. | `src/memtable.rs` |
| `pub fn rollback(&self, tx_id: u64)` | Removes all entries associated with the specified transaction ID from the MemTable. | `src/memtable.rs` |
| `pub fn scan_prefix_into( &self, prefix: &[u8], seq_no: u64, max_tx: TxId, target: &mut BTreeMap<Bytes, (Bytes, u64)>, )` | Scans entries matching a key prefix into `target` according to MVCC visibility rules. Iterates over each shard's range starting at `prefix`… | `src/memtable.rs` |
| `pub fn scan_prefix_into_matching<F>( &self, prefix: &[u8], seq_no: u64, max_tx: TxId, target: &mut BTreeMap<Bytes, (Bytes, u64)>, predicate: F, ) where F: Fn(&[u8], u64, u64) -> bool,` | Scans entries matching a key prefix into `target` with an additional matching predicate. | `src/memtable.rs` |
| `pub fn scan_range_into( &self, start: Bound<&[u8]>, end: Bound<&[u8]>, seq_no: u64, max_tx: TxId, target: &mut BTreeMap<Bytes, (Bytes, u64)>, )` | Scans entries matching a key range into `target` according to MVCC visibility rules. Iterates over each shard's range defined by `(start, e… | `src/memtable.rs` |
| `pub fn scan_range_into_matching<F>( &self, start: Bound<&[u8]>, end: Bound<&[u8]>, seq_no: u64, max_tx: TxId, target: &mut BTreeMap<Bytes, (Bytes, u64)>, predicate: F, ) where F: Fn(&[u8], u64, u64) -> bool,` | Scans entries matching a key range into `target` with an additional matching predicate. | `src/memtable.rs` |
| `pub fn shard_entry_counts(&self) -> [usize` | Returns entry count for each shard (used for testing shard distribution). | `src/memtable.rs` |
| `pub fn shard_for(key: &[u8]) -> usize` | Deterministic shard selector mapping keys to independent shards. Constructs a 16-bit big-endian value from the first two bytes of the key t… | `src/memtable.rs` |
| `pub fn size(&self) -> usize` | Returns the approximate size in bytes. | `src/memtable.rs` |
| `pub fn tx_range(&self) -> (u64, u64)` | Returns the transaction ID range covered by this MemTable. | `src/memtable.rs` |
| `pub fn drain_memtable(memtable: &MemTable, matcher: &Matcher) -> DrainStats` | Discards all entries in `memtable` matching `matcher`. | `src/purge/drain.rs` |
| `pub async fn rewrite_sstable_without( file_id: FileId, _matcher: &Matcher, _reserve: DiskReserve, ) -> Result<RewriteOutcome>` | Rewrites the SSTable identified by `file_id` copying only non-target records byte-identically. | `src/purge/rewrite.rs` |
| `pub async fn unlink_verified_sstable(_file_id: FileId) -> Result<()>` | Performs verified file unlink following permanent manifest swap and F-slot zeroing. | `src/purge/unlink.rs` |
| `pub async fn seal_active_wal(wal: &Wal, current_seq: u64) -> Result<SealPoint>` | Seals the active WAL segment at the current sequence boundary. | `src/purge/wal_seal.rs` |
| `pub fn contains(&self, file_id: u64, offset: u64) -> bool` | — | `src/sstable/block_cache.rs` |
| `pub fn create_block_cache(capacity_mb: usize) -> Arc<BlockCache>` | Creates a new block cache instance with default shard count. Capacity is in MB (assuming 4KB blocks). | `src/sstable/block_cache.rs` |
| `pub fn create_block_cache_with_shards(capacity_mb: usize, num_shards: usize) -> Arc<BlockCache>` | Creates a new block cache instance with configurable shard count. Capacity is in MB. | `src/sstable/block_cache.rs` |
| `pub fn get(&self, file_id: u64, offset: u64) -> Option<Bytes>` | — | `src/sstable/block_cache.rs` |
| `pub fn insert(&self, file_id: u64, offset: u64, data: Bytes)` | — | `src/sstable/block_cache.rs` |
| `pub fn is_empty(&self) -> bool` | — | `src/sstable/block_cache.rs` |
| `pub fn len(&self) -> usize` | — | `src/sstable/block_cache.rs` |
| `pub fn new(capacity_per_shard: usize) -> Self` | — | `src/sstable/block_cache.rs` |
| `pub fn new_with_shards(capacity_per_shard: usize, num_shards: usize) -> Self` | — | `src/sstable/block_cache.rs` |
| `pub fn shard_idx(&self, file_id: u64, offset: u64) -> usize` | — | `src/sstable/block_cache.rs` |
| `pub fn binary_search_entry_in_block( block_data: &[u8], offsets_start: usize, num_offsets: usize, key: &[u8], is_v3: bool, ) -> Result<Option<(usize, usize)>>` | — | `src/sstable/block_search.rs` |
| `pub fn binary_search_first_index_in_block( block_data: &[u8], offsets_start: usize, num_offsets: usize, key: &[u8], is_v3: bool, ) -> Result<Option<usize>>` | — | `src/sstable/block_search.rs` |
| `pub fn binary_search_index_in_block( block_data: &[u8], offsets_start: usize, num_offsets: usize, key: &[u8], is_v3: bool, ) -> Result<std::result::Result<usize, usize>>` | — | `src/sstable/block_search.rs` |
| `pub fn block_binary_search( block_data: &[u8], offsets_start: usize, num_offsets: usize, key: &[u8], is_v3: bool, ) -> Result<Option<usize>>` | — | `src/sstable/block_search.rs` |
| `pub fn get_entry_at_index( block_data: &[u8], offsets_start: usize, idx: usize, is_v3: bool, ) -> Result<(usize, usize)>` | Entry offset accessor supporting both 16-bit (legacy v0..v2) and 32-bit (v3+) block formats. | `src/sstable/block_search.rs` |
| `pub fn get_entry_key_at_index( block_data: &[u8], offsets_start: usize, idx: usize, is_v3: bool, ) -> Result<&[u8]>` | — | `src/sstable/block_search.rs` |
| `pub fn from_bytes(data: &[u8]) -> Result<Self>` | — | `src/sstable/bloom.rs` |
| `pub fn hash_pair(key: &[u8]) -> (u64, u64)` | Erzeugt zwei unabhängige 64-bit Hashes aus dem Blake3-Digest. h1 = erste 8 Bytes, h2 = Bytes 8-15 (oder fallback zu h1 ^ const) | `src/sstable/bloom.rs` |
| `pub fn insert(&mut self, key: &[u8])` | Inserts a key into the filter. | `src/sstable/bloom.rs` |
| `pub fn may_contain(&self, key: &[u8]) -> bool` | Checks if a key might be in the filter. | `src/sstable/bloom.rs` |
| `pub fn new(expected_elements: usize, fpr: f64) -> Self` | Creates a new Bloom filter for the expected number of elements and target false positive rate (fpr). ## FPR Trade-off - `fpr = 0.01` (1%) u… | `src/sstable/bloom.rs` |
| `pub fn to_bytes(&self) -> Vec<u8>` | — | `src/sstable/bloom.rs` |
| `pub fn add(&mut self, key: &[u8], value: &[u8], seq_no: u64, tx_id: u64) -> bool` | — | `src/sstable/builder.rs` |
| `pub async fn add(&mut self, key: &[u8], value: &[u8], seq_no: u64, tx_id: u64) -> Result<()>` | Adds a key-value pair to the SSTable being built. | `src/sstable/builder.rs` |
| `pub fn build(mut self) -> Bytes` | Finalizes the block and returns the bytes. | `src/sstable/builder.rs` |
| `pub async fn create(path: impl AsRef<Path>) -> Result<Self>` | Creates a new SstableBuilder that writes to the given file path. | `src/sstable/builder.rs` |
| `pub async fn create_with_key_manager( path: impl AsRef<Path>, key_manager: Option<Arc<KeyManager>>, ) -> Result<Self>` | — | `src/sstable/builder.rs` |
| `pub fn current_size(&self) -> usize` | — | `src/sstable/builder.rs` |
| `pub async fn finish(mut self) -> Result<SstableMetadata>` | Finalizes the SSTable and returns metadata. | `src/sstable/builder.rs` |
| `pub fn is_empty(&self) -> bool` | — | `src/sstable/builder.rs` |
| `pub fn new(block_size: usize) -> Self` | — | `src/sstable/builder.rs` |
| `pub fn new_with_version(block_size: usize, format_version: u16) -> Self` | — | `src/sstable/builder.rs` |
| `pub fn set_format_version(&mut self, version: u16)` | Explicitly configures the SSTable format version to write (default: 4). | `src/sstable/builder.rs` |
| `pub fn file_path(&self) -> &Path` | — | `src/sstable/reader.rs` |
| `pub fn first_key(&self) -> &Bytes` | — | `src/sstable/reader.rs` |
| `pub async fn get(&self, key: &[u8]) -> Result<Option<(Bytes, u64, u64)>>` | Retrieves a value from the SSTable by key. | `src/sstable/reader.rs` |
| `pub async fn get_at( &self, key: &[u8], max_seq: u64, max_tx: u64, ) -> Result<Option<(Bytes, u64, u64)>>` | Retrieves a value, sequence number, and transaction ID from the SSTable by key bounded by maximum sequence number (`max_seq`) and maximum t… | `src/sstable/reader.rs` |
| `pub async fn iter(&self) -> Result<Vec<(Bytes, Bytes, u64)>>` | Iterates over all entries in sorted key order (allocates memory for all entries). | `src/sstable/reader.rs` |
| `pub fn last_key(&self) -> &Bytes` | — | `src/sstable/reader.rs` |
| `pub async fn lookup_metrics(&self, key: &[u8]) -> (bool, bool, bool, bool)` | Metrics result for point lookup instrumentation. | `src/sstable/reader.rs` |
| `pub fn max_tx_id(&self) -> u64` | — | `src/sstable/reader.rs` |
| `pub fn metadata(&self) -> &SstableMetadata` | — | `src/sstable/reader.rs` |
| `pub fn min_tx_id(&self) -> u64` | — | `src/sstable/reader.rs` |
| `pub async fn open(path: impl AsRef<Path>, block_cache: Arc<BlockCache>) -> Result<Self>` | Opens an existing SSTable file for reading. | `src/sstable/reader.rs` |
| `pub async fn open_with_key_manager( path: impl AsRef<Path>, block_cache: Arc<BlockCache>, key_manager: Option<Arc<KeyManager>>, ) -> Result<Self>` | — | `src/sstable/reader.rs` |
| `pub async fn stream(self: &Arc<Self>) -> Result<SstableStream>` | — | `src/sstable/reader.rs` |
| `pub async fn scan_prefix(&self, prefix: &[u8]) -> Result<Vec<(Bytes, Bytes, u64, u64)>>` | Scans the SSTable for keys starting with the given prefix. | `src/sstable/reader_ext.rs` |
| `pub async fn scan_range( &self, start: std::ops::Bound<&[u8]>, end: std::ops::Bound<&[u8]>, ) -> Result<Vec<(Bytes, Bytes, u64, u64)>>` | Scans all entries within a key range. | `src/sstable/reader_ext.rs` |
| `pub async fn next(&mut self) -> Result<Option<(Bytes, Bytes, u64, u64)>>` | — | `src/sstable/stream.rs` |
| `pub async fn next_entry(&mut self) -> Result<Option<(Bytes, Bytes, u64, u64)>>` | Optimized for compaction: avoids copying if possible. | `src/sstable/stream.rs` |
| `pub fn compute_pressure( &self, wal_depth: usize, scheduler_queue_depth: f32, blocking_util: f32, embedding_queue: usize, max_permits: usize, ) -> SystemPressure` | — | `src/system_pressure.rs` |
| `pub fn new(sampling_interval: Duration) -> Self` | — | `src/system_pressure.rs` |
| `pub async fn run( self, cancellation: CancellationToken, wal_queue_depth_fn: impl Fn() -> usize + Send + 'static, embedding_permits_fn: impl Fn() -> usize + Send + 'static, max_embedding_permits: usize, )` | — | `src/system_pressure.rs` |
| `pub async fn run_with_metrics_sink( self, cancellation: CancellationToken, wal_queue_depth_fn: impl Fn() -> usize + Send + 'static, embedding_permits_fn: impl Fn() -> usize + Send + 'static, max_embedding_permits: usize, metrics_sink: Option<Arc<parking_lot::…` | — | `src/system_pressure.rs` |
| `pub fn collection_prefix(&self, collection: &CollectionId) -> Vec<u8>` | Collection-spezifisches Scan-Präfix: `t:{tenant}:{col}:` | `src/tenant_codec.rs` |
| `pub fn decode_tenant_id(key: &[u8]) -> Option<TenantId>` | Dekodiert TenantId aus encoded Key. None bei ungültigem Format. Kein Unwrap — robuste Nutzung im Recovery-Pfad. | `src/tenant_codec.rs` |
| `pub fn encode_chunk_key(&self, collection: &CollectionId, doc_id: DocId) -> Vec<u8>` | Encodes a document chunk key: `t:{tenant}:{collection}:chunk:{doc}`. | `src/tenant_codec.rs` |
| `pub fn encode_graph_key(&self, collection: &CollectionId, entity_id: u64) -> Vec<u8>` | Encodes a graph entity key: `t:{tenant}:{collection}:graph:{entity}`. | `src/tenant_codec.rs` |
| `pub fn new(tenant_id: TenantId) -> Self` | Creates a new `TenantKeyCodec` for the given `TenantId`. | `src/tenant_codec.rs` |
| `pub fn new(inner: S, tenant_id: TenantId) -> Self` | Creates a new `TenantScopedStorage` wrapping an underlying `StorageEngine` or shared pointer. | `src/tenant_codec.rs` |
| `pub fn scan_prefix(&self) -> &[u8]` | Globales Scan-Präfix für diesen Tenant (alle Collections): `t:{tenant}:` Cached — keine Allokation im Hot-Path. | `src/tenant_codec.rs` |
| `pub fn tenant_id(&self) -> TenantId` | Returns the immutable `TenantId` bound to this storage wrapper. | `src/tenant_codec.rs` |
| `pub fn as_inner(&self) -> &Arc<LsmStorage>` | Liefert eine Referenz auf das innere LSM-Speichertriebwerk. | `src/tenant_scoped/raw.rs` |
| `pub fn into_scoped(self, tenant: TenantTag) -> super::scoped::TenantScopedStorage` | Bindet das Rohe Handle an einen spezifischen Mandanten (AR-6). | `src/tenant_scoped/raw.rs` |
| `pub fn new(inner: Arc<LsmStorage>) -> Self` | Erzeugt ein neues Rohes Handle. | `src/tenant_scoped/raw.rs` |
| `pub fn new(tenant: TenantTag, inner: Arc<LsmStorage>) -> Self` | Erstellt eine neue mandantenisolierte Speicherinstanz. | `src/tenant_scoped/scoped.rs` |
| `pub fn tenant(&self) -> TenantTag` | Liefert den zugeordneten Mandanten. | `src/tenant_scoped/scoped.rs` |
| `pub fn validate_key(&self, key: &KeyBuf) -> Result<()>` | Validiert, dass ein Schlüssel zum Mandanten gehört. | `src/tenant_scoped/scoped.rs` |
| `pub fn create_dir_all(vfs: &dyn Vfs, path: &Path) -> Result<()>` | — | `src/vfs.rs` |
| `pub fn exists(vfs: &dyn Vfs, path: &Path) -> Result<bool>` | — | `src/vfs.rs` |
| `pub fn len(vfs: &dyn Vfs, path: &Path) -> Result<u64>` | — | `src/vfs.rs` |
| `pub fn new(file: std_file_type::File) -> Self` | — | `src/vfs.rs` |
| `pub fn open(vfs: &dyn Vfs, path: &Path, options: &VfsOpenOptions) -> Result<Box<dyn VfsFile>>` | — | `src/vfs.rs` |
| `pub async fn open_vfs_file(path: &Path) -> Result<(Arc<dyn VfsFile>, u64)>` | — | `src/vfs.rs` |
| `pub fn read(vfs: &dyn Vfs, path: &Path) -> Result<Vec<u8>>` | — | `src/vfs.rs` |
| `pub fn read_dir(vfs: &dyn Vfs, path: &Path) -> Result<Vec<PathBuf>>` | — | `src/vfs.rs` |
| `pub fn remove_file(vfs: &dyn Vfs, path: &Path) -> Result<()>` | — | `src/vfs.rs` |
| `pub fn rename(vfs: &dyn Vfs, from: &Path, to: &Path) -> Result<()>` | — | `src/vfs.rs` |
| `pub fn sync_dir(vfs: &dyn Vfs, path: &Path) -> Result<()>` | — | `src/vfs.rs` |
| `pub fn write_atomic(vfs: &dyn Vfs, path: &Path, data: &[u8]) -> Result<()>` | — | `src/vfs.rs` |
| `pub fn verify_wal_chain_completeness( wal_tail_hmac: &[u8; 32], manifest_high_water_mark: &[u8; 32], ) -> Result<()>` | Verifiziert, dass der WAL-Tail nicht abgeschnitten oder manipuliert wurde, indem der letzte berechnete WAL-HMAC mit der im Manifest erwarte… | `src/wal/chain_check.rs` |
| `pub fn compute_checksum( op: &WalOp, seq_no: u64, integrity_key: &[u8], prev_hmac: [u8; 32], ) -> Result<[u8` | — | `src/wal/encode.rs` |
| `pub fn compute_checksum_v2( op: &WalOp, seq_no: u64, integrity_key: &[u8], prev_hmac: [u8; 32], ) -> Result<[u8` | Legacy V2 checksum calculation (without tx_id and length-prefixes in HMAC). | `src/wal/encode.rs` |
| `pub fn compute_checksum_v3( op: &WalOp, seq_no: u64, integrity_key: &[u8], prev_hmac: [u8; 32], ) -> Result<[u8` | Computes V3 checksum (includes tx_id and length prefixes for key/value). | `src/wal/encode.rs` |
| `pub fn entries(&self) -> &[WalEntry]` | Returns a slice of the inner WAL entries. | `src/wal/encode.rs` |
| `pub fn extend(&mut self, other: PreparedBatch)` | Extends this prepared batch with entries from another prepared batch. | `src/wal/encode.rs` |
| `pub fn from_bytes(data: &[u8]) -> Result<Self>` | Deserializes a WAL entry from bytes, verifying CRC32. | `src/wal/encode.rs` |
| `pub fn from_bytes_classified(data: &[u8]) -> std::result::Result<Self, WalParseError>` | Deserializes a WAL entry from bytes, returning a classified `WalParseError` on failure. | `src/wal/encode.rs` |
| `pub fn into_inner(self) -> Vec<WalEntry>` | Consumes the `PreparedBatch` and returns the inner vector of entries. | `src/wal/encode.rs` |
| `pub fn is_empty(&self) -> bool` | Returns `true` if the batch contains no entries. | `src/wal/encode.rs` |
| `pub fn len(&self) -> usize` | Returns the number of entries in the batch. | `src/wal/encode.rs` |
| `pub fn to_bytes(&self) -> Result<Vec<u8>>` | Serializes the entry to bytes. | `src/wal/encode.rs` |
| `pub fn try_new( op: WalOp, seq_no: u64, integrity_key: &[u8], prev_hmac: [u8; 32], ) -> Result<Self>` | Creates a new WAL entry with HMAC-SHA256 checksum and chaining. | `src/wal/encode.rs` |
| `pub fn tx_id(&self) -> TxId` | — | `src/wal/encode.rs` |
| `pub fn tx_id(&self) -> TxId` | — | `src/wal/encode.rs` |
| `pub fn integrity_key_for_test(&self) -> Result<[u8` | Exposes the HMAC integrity key for testing. | `src/wal/hmac.rs` |
| `pub fn legacy_integrity_key_for_test() -> [u8` | — | `src/wal/hmac.rs` |
| `pub fn migrate_legacy_key(legacy: &[u8], new_key_material: &[u8]) -> Result<[u8` | Migrates a legacy integrity key to a new 32-byte key material state. # Chain Compatibility When migrating a legacy WAL segment to a new int… | `src/wal/hmac.rs` |
| `pub async fn prepare_batch(&self, ops: Vec<(WalOp, u64)>) -> Result<(PreparedBatch, [u8` | Prepares a batch of WAL operations with sequential sequence numbers and HMAC hash-chaining. | `src/wal/hmac.rs` |
| `pub async fn rollback_hmac(&self, prev_hmac: [u8; 32])` | Rolls back the in-memory HMAC state to `prev_hmac` asynchronously. | `src/wal/hmac.rs` |
| `pub fn rollback_hmac_sync(&self, prev_hmac: [u8; 32]) -> bool` | Synchronously rolls back the in-memory HMAC state to `prev_hmac` using non-blocking `try_lock`. Suitable for use within `Drop` implementati… | `src/wal/hmac.rs` |
| `pub async fn append_batch(&self, batch: PreparedBatch) -> Result<()>` | — | `src/wal/io.rs` |
| `pub async fn append_batch_locked( &self, batch: PreparedBatch, guard: &tokio::sync::MutexGuard<'_, ()>, ) -> Result<()>` | — | `src/wal/io.rs` |
| `pub fn arm_truncate_failure_for_test(&self)` | Aktiviert eine einmalige simulierte Truncate-I/O-Failure für Tests. Der Flusher gibt beim nächsten `WalCommand::Truncate` einen Fehler zurü… | `src/wal/io.rs` |
| `pub async fn enqueue_append_batch_locked( &self, batch: PreparedBatch, _guard: &tokio::sync::MutexGuard<'_, ()>, ) -> Result<tokio::sync::oneshot::Receiver<Result<()>>>` | — | `src/wal/io.rs` |
| `pub async fn find_tx_offset(&self, target_tx_id: TxId) -> Result<(u64, [u8` | — | `src/wal/io.rs` |
| `pub async fn last_hmac_snapshot(&self) -> [u8` | Returns a snapshot of the last HMAC written to the log. | `src/wal/io.rs` |
| `pub async fn rotate_and_seal(&self) -> Result<PathBuf>` | — | `src/wal/io.rs` |
| `pub async fn scan_entries_with_callback<F>( &self, file_size: u64, mut callback: F, ) -> Result<WalVersion> where F: FnMut(u64, WalEntry, u64) -> bool,` | Scans the WAL entry by entry, executing full HMAC chain validation, CRC checks, and key manager decryption. Invokes `callback(seq_no, entry… | `src/wal/io.rs` |
| `pub async fn truncate(&self, offset: u64, new_last_hmac: [u8; 32]) -> Result<()>` | — | `src/wal/io.rs` |
| `pub async fn try_append_batch(&self, batch: PreparedBatch) -> Result<()>` | Non-blocking attempt to append a prepared batch. Returns `Err(ActaError::Storage("WAL queue full (backpressure)"))` if the flusher channel… | `src/wal/io.rs` |
| `pub async fn try_append_batch_locked( &self, batch: PreparedBatch, guard: &tokio::sync::MutexGuard<'_, ()>, ) -> Result<()>` | — | `src/wal/io.rs` |
| `pub async fn try_enqueue_append_batch_locked( &self, batch: PreparedBatch, _guard: &tokio::sync::MutexGuard<'_, ()>, ) -> Result<tokio::sync::oneshot::Receiver<Result<()>>>` | — | `src/wal/io.rs` |
| `pub fn decrypt_auto_nonce(&self, ciphertext: &[u8], nonce_bytes: &[u8; 12]) -> Result<Vec<u8>>` | Decrypts a block of data using a 12-byte nonce. | `src/wal/key_manager.rs` |
| `pub fn derive_file_key(&self, file_id: &[u8]) -> Result<Self>` | Derives a sub-key for a specific file or logical stream. | `src/wal/key_manager.rs` |
| `pub fn encrypt_auto_nonce(&self, data: &[u8]) -> Result<(Vec<u8>, [u8` | Encrypts a block of data with an auto-incrementing 12-byte nonce. | `src/wal/key_manager.rs` |
| `pub fn integrity_key(&self) -> Result<[u8` | Derives an integrity key for HMAC-SHA256. | `src/wal/key_manager.rs` |
| `pub fn try_new(passphrase: &str, salt: &[u8]) -> Result<Self>` | Creates a new KeyManager by deriving a key from a passphrase and salt using HKDF-SHA256. | `src/wal/key_manager.rs` |
| `pub async fn replay(&self) -> Result<Vec<(u64, WalEntry, u64)>>` | Replays the WAL into memory for entry decoding. Falls back to stream replay if buffer reading or parsing fails. | `src/wal/replay.rs` |
| `pub async fn replay_buffer(&self) -> Result<(Vec<(u64, WalEntry, u64)>, WalVersion)>` | Replays the WAL. Returns all valid entries with sequence numbers, entries, end offsets, and detected WAL version. | `src/wal/replay.rs` |
| `pub async fn replay_buffer_with_sink<S: ReplayProgressSink>( &self, sink: &S, ) -> Result<(Vec<(u64, WalEntry, u64)>, WalVersion)>` | Replays the WAL with progress callback. | `src/wal/replay.rs` |
| `pub async fn replay_from(&self, since_seq_no: u64) -> Result<Vec<WalEntry>>` | Replays the WAL and returns all entries with seq_no > since_seq_no. | `src/wal/replay.rs` |
| `pub async fn replay_stream(&self) -> Result<Vec<(u64, WalEntry, u64)>>` | Replays the WAL using the stream reader (`BufReader`). | `src/wal/replay.rs` |
| `pub async fn replay_with_sink<S: ReplayProgressSink>( &self, sink: &S, ) -> Result<Vec<(u64, WalEntry, u64)>>` | Replays the WAL while notifying the provided `ReplayProgressSink` of progress. | `src/wal/replay.rs` |
| `pub async fn scan_entries_buffer<F>( &self, file_size: u64, mut callback: F, ) -> Result<WalVersion> where F: FnMut(u64, WalEntry, u64) -> bool,` | Scans the WAL using slice decoding. If slice reading or scanning encounters any error, it logs a warning (`tracing::warn!`) and automatical… | `src/wal/replay.rs` |
| `pub fn finalize(self) -> [u8` | Finalizes the HMAC calculation and returns the 32-byte digest. | `src/wal/verifier.rs` |
| `pub fn last_hmac_snapshot(&self) -> [u8` | — | `src/wal/verifier.rs` |
| `pub fn last_seq_no_snapshot(&self) -> Option<u64>` | — | `src/wal/verifier.rs` |
| `pub fn new(integrity_key: &[u8]) -> Result<Self>` | — | `src/wal/verifier.rs` |
| `pub fn new(integrity_key: &[u8]) -> Self` | — | `src/wal/verifier.rs` |
| `pub fn set_last_hmac(&mut self, hmac: [u8; 32])` | — | `src/wal/verifier.rs` |
| `pub fn set_last_seq_no(&mut self, seq_no: Option<u64>)` | — | `src/wal/verifier.rs` |
| `pub fn skip_hmac_verify_legacy(&mut self, entry: &WalEntrySnapshot)` | Updates the chain state for legacy V1 entries without HMAC verification. | `src/wal/verifier.rs` |
| `pub fn update(&mut self, data: &[u8])` | Feeds new data into the ongoing HMAC calculation. | `src/wal/verifier.rs` |
| `pub fn verify_and_update(&mut self, entry: &WalEntrySnapshot, offset: u64) -> Result<()>` | Default verification delegating to V3 verification. | `src/wal/verifier.rs` |
| `pub fn verify_and_update_v2(&mut self, entry: &WalEntrySnapshot, offset: u64) -> Result<()>` | Verifies a V2 entry (legacy without tx_id and without length prefixes) and updates the chain state. | `src/wal/verifier.rs` |
| `pub fn verify_and_update_v3(&mut self, entry: &WalEntrySnapshot, offset: u64) -> Result<()>` | Verifies a V3 entry and updates the chain state. | `src/wal/verifier.rs` |


---

## `acta-engine`

| Attribut | Wert |
|---|---|
| **Ring** | 3 — Ring 3 — Orchestration & Cognition |
| **Status** | In Migration |
| **Zweck** | Collection-LSM-Anbindung, Multi-Index-Pläne und Schreibtransaktionen. |
| **Unsafe-Code** | Nein (`#![forbid(unsafe_code)]`) |
| **Interne Abhängigkeiten** | `acta-crypto`, `acta-erasure`, `acta-mvcc`, `acta-ports`, `acta-rank`, `acta-store`, `acta-sys`, `acta-text`, `acta-types`, `acta-vector` |
| **Externe Abhängigkeiten (Auswahl)** | `ahash`, `bytes`, `parking_lot`, `serde`, `serde_json`, `thiserror`, `tokio`, `tokio-util`, `tracing`, `zeroize` |

### Architekturbeschreibung

Orchestrierung: Mandanten, Collections, Transaktionen, Suche, Hintergrundtasks.  # Acta Engine (`acta-engine`)  **Ring:** Ring 3 (SPEC §8.13, §3.1)  Die Engine orchestriert Mandanten, Collections, Transaktionen (SSI), Suchströme mit Nachladen (D-62), Hintergrund-Worker und die Startsequenz (SPEC §14.1). Sie enthält **keine** eigene Speicher-, Krypto- oder Löschlogik.

### Modul- und Re-Export-Oberfläche (lib.rs)

```rust
pub mod builder;
pub mod collection;
pub mod hardening;
pub mod health;
pub mod instance;
pub mod search;
pub mod startup;
pub mod tenant;
pub mod wiring;
pub mod workers;
pub use builder::EngineBuilder;
pub use collection::{Collection, CollectionOptions, Document, ScanRange};
pub use health::{DegradedReason, EngineState, HealthReport};
pub use instance::Acta;
pub use search::{Hit, HitSource, SearchPage};
pub use tenant::Tenant;
```

### Feature-Flags

| Flag | Aktiviert |
|---|---|
| `default` | `[]` |
| `experimental-diskann` | `["acta-vector/experimental-diskann"]` |

### Öffentliche Datentypen — Structs (11)

| Struct | Felder / Form | Beschreibung | Datei |
|---|---|---|---|
| `Acta` | `{ pub(crate) data_dir: PathBuf, pub(crate) passphrase: Vec<u8>, pub(crate) erase_sla: Duration, pub(crate) backup_retention: Duration, pub(crate) runtime: tokio:… }` | Zentrales Instanz-Handle von Acta. | `src/instance.rs` |
| `BackgroundWorker` | `{ name: &'static str, stop_flag: Arc<AtomicBool>, handle: Option<JoinHandle<()>>, }` | Repräsentiert einen kooperativ stoppbaren Hintergrund-Worker. | `src/workers.rs` |
| `Collection` | `{ name: String, tenant_id: TenantId, options: CollectionOptions, doc_locks: Arc<StripedDocLocks>, }` | Handle auf eine Collection eines Mandanten. | `src/collection.rs` |
| `CollectionOptions` | `{ /// Tokenizer/Sprach-Analyzer (Standard "de/1"). pub analyzer: String, /// Ob Wortpositionen gespeichert werden. pub positions: bool, /// Dimension der Vektore… }` | Optionen beim Anlegen oder Öffnen einer Collection (SPEC §7.1, D-81). | `src/collection.rs` |
| `Document` | `{ /// Textinhalt des Dokuments. pub content: String, /// Metadaten als Schlüssel-Wert-Paare. pub metadata: Vec<(String, String)>, /// Optionales Embedding / Vekt… }` | Dokument mit Textchunks, Metadaten und Vektoren (SPEC §7.2, D-124). | `src/collection.rs` |
| `EngineBuilder` | `{ pub(crate) data_dir: Option<PathBuf>, pub(crate) passphrase: Option<Vec<u8>>, pub(crate) erase_sla: Option<Duration>, pub(crate) backup_retention: Option<Durat… }` | Konfigurations- und Initialisierungs-Builder für Acta nach SPEC §5.7. | `src/builder.rs` |
| `HealthReport` | `{ /// Aktueller Zustand. pub state: EngineState, /// p50 fsync Latenz der Startprobe. pub fsync_p50: Duration, /// Anzahl offener Löschvorgänge. pub open_erasure… }` | Gesundheitsbericht der Engine ohne blockierende Datei-I/O (D-64). | `src/health.rs` |
| `Hit` | `{ /// Opaque Dokumentenreferenz. pub handle: DocHandle, /// Mandantenweiter Dokument-Tag. pub doc_tag: DocTag, /// Chunk-Index innerhalb des Dokuments. pub chunk… }` | Einzelner Treffer in einer Suchseite. | `src/search.rs` |
| `SearchPage` | `{ /// Liste von Treffern. pub hits: Vec<Hit>, /// Gibt an, ob der Kandidatenpool vollständig erschöpft ist. pub exhausted: bool, }` | Seite eines Suchergebnisses mit Erschöpfungsanzeige (D-62). | `src/search.rs` |
| `StripedDocLocks` | `{ locks: Vec<Mutex<()>>, }` | 1024 gestreifte asynchrone Dokumentlocks (SPEC §5.4, D-128, T-LOCK-2). | `src/collection.rs` |
| `Tenant` | `{ pub(crate) engine: Arc<Acta>, pub(crate) ctx: TenantContext, pub(crate) tenant_id: TenantId, }` | Mandantengebundenes Handle auf die Engine. | `src/tenant.rs` |

### Öffentliche Datentypen — Enums (4)

| Enum | Varianten | Beschreibung | Datei |
|---|---|---|---|
| `DegradedReason` | `/// Ledger fehlt (neuer Rechner). LedgerMissing, /// Purge-Fortschritt hinter SLA. PurgeSlow, /// Freier Speicherplatz unter Reserve. FreeSpaceLow, /// Prozesshärtung nur teilweise wirksam. Hardening…` | Grund für den Zustand `Degraded`. | `src/health.rs` |
| `EngineState` | `/// Normalbetrieb. Ok, /// Beeinträchtigter Betrieb. Degraded, /// Fataler E/A- oder Integritätsfehler. Poisoned, /// Wiederanlauf und Derivate-Wiederaufbau. Recovering,` | Grundlegender Systemzustand. | `src/health.rs` |
| `HitSource` | `/// BM25-Volltextindex. Text, /// Vektorindex. Vector, /// RRF-Fusion. Hybrid,` | Quelle eines Suchtreffers. | `src/search.rs` |
| `ScanRange` | `/// Gesamte Collection scannen. All, /// Dokumente ab einem bestimmten Schlüssel scannen. From(DocRef),` | Scan-Bereich für Dokumenteniteration (SPEC §7.2, D-41). | `src/collection.rs` |

### Öffentliche Funktionen (51)

| Signatur | Beschreibung | Datei |
|---|---|---|
| `pub fn backup_retention(mut self, retention: Duration) -> Self` | Setzt die Aufbewahrungsdauer für Backups (Pflicht, 1 Tag bis 30 Jahre). | `src/builder.rs` |
| `pub fn batch_window(mut self, window: Duration) -> Self` | Setzt das Batch-Fenster für Purges (Standard: 60 s, muss ≤ erase_sla / 2 sein, D-39). | `src/builder.rs` |
| `pub fn clock(mut self, clock: Arc<dyn Clock>) -> Self` | Setzt die Systemuhr (Pflicht). | `src/builder.rs` |
| `pub fn data_dir(mut self, path: impl Into<PathBuf>) -> Self` | Setzt das Datenverzeichnis (Pflicht). | `src/builder.rs` |
| `pub fn erase_return_timeout(mut self, timeout: Duration) -> Self` | Setzt das Timeout für Erase-Rückgaben (Standard 60 s, 1 s bis 10 min). | `src/builder.rs` |
| `pub fn erase_sla(mut self, sla: Duration) -> Self` | Setzt das Lösch-SLA (Pflicht, 1 min bis 30 Tage). | `src/builder.rs` |
| `pub fn history_horizon(mut self, horizon: Duration) -> Self` | Setzt den Historien-Horizont für Versionen (Standard 24 h, 0 bis 365 d). | `src/builder.rs` |
| `pub fn new() -> Self` | Erzeugt einen neuen leeren `EngineBuilder`. | `src/builder.rs` |
| `pub async fn open(self) -> Result<Acta>` | Validiert alle Pflichtparameter und öffnet die Datenbankinstanz (SPEC §5.7, T-CFG-3). | `src/builder.rs` |
| `pub fn passphrase(mut self, passphrase: impl AsRef<[u8]>) -> Self` | Setzt die Master-Passphrase (Pflicht, ≥ 12 Bytes). | `src/builder.rs` |
| `pub fn rng(mut self, rng: Arc<dyn Rng>) -> Self` | Setzt den Zufallszahlengenerator (Pflicht). | `src/builder.rs` |
| `pub fn runtime(mut self, handle: tokio::runtime::Handle) -> Self` | Setzt das Tokio-Runtime-Handle (Pflicht, SPEC §5.4). | `src/builder.rs` |
| `pub fn vfs(mut self, vfs: Arc<dyn Vfs>) -> Self` | Setzt das VFS (Pflicht, SPEC D-04). | `src/builder.rs` |
| `pub fn doc_locks(&self) -> &StripedDocLocks` | Liefert die Dokument-Locks (1024 gestreifte Shards). | `src/collection.rs` |
| `pub async fn get(&self, _doc_ref: DocRef, _at: Option<u64>) -> Result<Option<Document>>` | Liest ein Dokument (SPEC §5.5, §7.2). | `src/collection.rs` |
| `pub fn get_lock(&self, doc_tag_bytes: &[u8; 16]) -> &Mutex<()>` | Liefert die Referenz auf das spezifische Lock. | `src/collection.rs` |
| `pub async fn insert(&self, doc_ref: DocRef, doc: Document) -> Result<()>` | Fügt ein Dokument ein (SPEC §5.5, §7.2). Prüft Kapazitätsgrenzen (D-87, D-123) und hält das gestreifte Dokument-Lock. | `src/collection.rs` |
| `pub fn name(&self) -> &str` | Name der Collection. | `src/collection.rs` |
| `pub fn new() -> Self` | Initialisiert 1024 gestreifte Locks mit festem Speicherbedarf O(1). | `src/collection.rs` |
| `pub fn new_collection(name: &str, tenant_id: TenantId, options: CollectionOptions) -> Self` | Erzeugt eine neue Collection-Instanz. | `src/collection.rs` |
| `pub fn options(&self) -> &CollectionOptions` | Options der Collection. | `src/collection.rs` |
| `pub async fn scan(&self, _range: ScanRange) -> Result<Vec<DocHandle>>` | Scan über Dokumentenbereiche. | `src/collection.rs` |
| `pub async fn search_text(&self, _query: &str, k: usize) -> Result<SearchPage>` | Volltextsuche mittels BM25 mit Nachladen (D-62). | `src/collection.rs` |
| `pub async fn search_vector(&self, _vec: Vec<f32>, k: usize) -> Result<SearchPage>` | Vektorsuche mittels HNSW mit Nachladen (D-62). | `src/collection.rs` |
| `pub fn shard_index(&self, doc_tag_bytes: &[u8; 16]) -> usize` | Bestimmt den Lock-Index anhand des `doc_tag`-Hashes. | `src/collection.rs` |
| `pub async fn soft_delete(&self, _doc_ref: DocRef) -> Result<()>` | Setzt einen logischen Tombstone (SPEC §7.2, D-51). | `src/collection.rs` |
| `pub fn tenant_id(&self) -> TenantId` | Mandanten-ID. | `src/collection.rs` |
| `pub async fn transaction<F, R>(&self, f: F) -> Result<R> where F: FnOnce(&Collection) -> Result<R>,` | Führt eine SSI-Transaktion aus (SPEC §5.5, D-124, T-TXN-1). | `src/collection.rs` |
| `pub async fn update(&self, doc_ref: DocRef, doc: Document) -> Result<()>` | Aktualisiert ein Dokument unter Beibehaltung oder Erzeugung einer Version. | `src/collection.rs` |
| `pub fn apply_process_hardening() -> Result<bool>` | Führt Prozesshärtung (Coredump-Verbot, Sperrung sensitiver Speicherseiten) aus. | `src/hardening.rs` |
| `pub fn backup_retention(&self) -> Duration` | Backup-Retention der Instanz. | `src/instance.rs` |
| `pub fn batch_window(&self) -> Duration` | Batch-Fenster der Instanz. | `src/instance.rs` |
| `pub fn clock(&self) -> Arc<dyn Clock>` | Systemuhr der Instanz. | `src/instance.rs` |
| `pub async fn close(&self) -> Result<()>` | Schließt die Instanz geordnet (SPEC §14.3). | `src/instance.rs` |
| `pub async fn collection_for_tenant( &self, name: &str, tenant_id: TenantId, opts: CollectionOptions, ) -> Result<Arc<Collection>>` | Öffnet oder erstellt eine Collection für den angegebenen Mandanten. | `src/instance.rs` |
| `pub fn data_dir(&self) -> &Path` | Datenverzeichnis der Instanz. | `src/instance.rs` |
| `pub fn erase_sla(&self) -> Duration` | Lösch-SLA der Instanz. | `src/instance.rs` |
| `pub fn health(&self) -> HealthReport` | Liefert einen schnellen Gesundheitsbericht ohne blockierende Datei-I/O (D-64). | `src/instance.rs` |
| `pub fn passphrase(&self) -> &[u8]` | Passphrase der Instanz (Bytes). | `src/instance.rs` |
| `pub fn rng(&self) -> Arc<dyn Rng>` | Zufallszahlengenerator der Instanz. | `src/instance.rs` |
| `pub fn runtime_handle(&self) -> &tokio::runtime::Handle` | Runtime-Handle der Instanz. | `src/instance.rs` |
| `pub fn vfs(&self) -> Arc<dyn Vfs>` | VFS der Instanz. | `src/instance.rs` |
| `pub async fn execute_startup_sequence( data_dir: PathBuf, vfs: Arc<dyn Vfs>, _clock: Arc<dyn Clock>, _rng: Arc<dyn Rng>, ) -> Result<()>` | Führt die Startsequenz durch. | `src/startup.rs` |
| `pub async fn collection(&self, name: &str, opts: CollectionOptions) -> Result<Arc<Collection>>` | Öffnet oder erstellt eine Collection für diesen Mandanten. | `src/tenant.rs` |
| `pub fn context(&self) -> &TenantContext` | Liefert den zugehörigen Mandantenkontext. | `src/tenant.rs` |
| `pub fn tenant_id(&self) -> TenantId` | Liefert die Mandanten-ID. | `src/tenant.rs` |
| `pub fn create_vault( _vfs: Arc<dyn Vfs>, _clock: Arc<dyn Clock>, _rng: Arc<dyn Rng>, _vault_dir: std::path::PathBuf, ) -> Result<Arc<FileVault>>` | Erzeugt die FileVault-Instanz innerhalb von acta-engine (D-95). | `src/wiring.rs` |
| `pub async fn join(&mut self)` | Wartet auf die Beendigung des Workers. | `src/workers.rs` |
| `pub fn name(&self) -> &'static str` | Liefert den Namen des Workers. | `src/workers.rs` |
| `pub fn spawn<F, Fut>(name: &'static str, runtime: &tokio::runtime::Handle, task_fn: F) -> Self where F: FnOnce(Arc<AtomicBool>) -> Fut + Send + 'static, Fut: std::future::Future<Output = ()> + Send + 'static,` | Startet einen neuen Hintergrund-Worker. | `src/workers.rs` |
| `pub fn stop(&self)` | Signaliert dem Worker zu stoppen. | `src/workers.rs` |


---

## `acta`

| Attribut | Wert |
|---|---|
| **Ring** | 4 — Ring 4 — Boundary & Composition Roots |
| **Status** | Fertig |
| **Zweck** | Öffentliche Haupt-Fassade und Composition Root für Rust-Anwendungen. |
| **Unsafe-Code** | Nein (`#![forbid(unsafe_code)]`) |
| **Interne Abhängigkeiten** | `acta-durable-fs`, `acta-engine`, `acta-types` |
| **Externe Abhängigkeiten (Auswahl)** | `getrandom`, `thiserror`, `tokio` |

### Architekturbeschreibung

Öffentliche API und Verdrahtung. Enthält keine Logik.  # Architektur-Rolle: Ring 4 (Fassade)  Gemäß SPEC Abschnitt 5.1, 5.2, 7 und 8.14 (D-89, D-91, D-95): - Das Crate `acta` bildet die einzige öffentliche Oberfläche (D-66). - Es enthält **keine eigene Logik** (AR-12, D-89). - Erlaubte Module: `api`, `wiring`, `error`. - Erlaubte Abhängigkeiten: `acta-types`, `acta-durable-fs`, `acta-engine` (D-88). - Pflichtparameter (Passphrase, Runtime-Handle, Backup-Retention, TenantContext) werden zur Compile-Zeit per Typsystem erzwungen (Type-State Builder). - Strikte Trennung zwischen logischem `soft_delete` (Tombstone) und physischer Löschung `erase` (D-51). - Alle öffentlichen Typen sind mit `#[non_exhaustive]` geschützt.

### Modul- und Re-Export-Oberfläche (lib.rs)

```rust
pub mod api;
pub mod error;
pub mod wiring;
pub use api::{ Collection, CollectionOptions, Acta, DocHandle, DocKey, DocRef, Document, EngineBuilder, EraseMode, EraseReceipt, EraseRequest, EraseTarget, ErasureId, ErasureStatus, Filter, HealthReport, Hit, Missing, RestrictedAccess, RestrictedPurpose, RetentionAttestation, RetentionCode, ScanRange, SearchPage, Secret, Set, SignedReport, SnapshotId, Supersede, Tenant, TenantContext, TenantRef, Timing, UpdateOptions, UpdateReceipt, };
pub use error::{ConfigDetails, ConflictDetails, Error, ErrorCode, LimitDetails, Result};
pub use wiring::{system_defaults, OsRng, SystemDefaults};
```

### Feature-Flags

| Flag | Aktiviert |
|---|---|
| `default` | `[]` |

### Öffentliche Datentypen — Structs (19)

| Struct | Felder / Form | Beschreibung | Datei |
|---|---|---|---|
| `Acta` | `{ inner: Arc<acta_engine::Acta>, }` | Zentrales Handle auf eine geöffnete Acta-Instanz (SPEC 7.1). | `src/api.rs` |
| `Collection` | `{ inner: Arc< acta_engine::Collection< acta_store::tenant_codec::TenantScopedStorage<Arc<acta_store::LsmStorage>>, >, >, tenant_id: acta_types::TenantId, options… }` | Dokument-API innerhalb einer Collection (SPEC 7.2). | `src/api.rs` |
| `CollectionOptions` | `{ /// Analyzer-Bezeichnung, z. B. "de/1". pub analyzer: String, /// Ob Wortpositionen gespeichert werden. pub positions: bool, /// Dimension der Vektoren, falls… }` | Optionen für die Anlage einer Collection (SPEC 7.1, D-81). | `src/api.rs` |
| `Document` | `{ /// Textinhalt des Dokuments. pub content: String, /// Metadaten als Schlüssel-Wert-Paare. pub metadata: Vec<(String, String)>, /// Optionales Embedding / Vekt… }` | Dokument mit Textchunks, Metadaten und Vektoren (SPEC 7.2, D-124). | `src/api.rs` |
| `EngineBuilder` | `{ data_dir: PathBuf, passphrase: Option<Secret>, runtime: Option<Handle>, backup_retention: Option<Duration>, dimension: Option<usize>, erase_sla: Duration, syst… }` | Builder zur Konfiguration und Erstellung einer `Acta`-Instanz (SPEC 5.7, D-89). | `src/api.rs` |
| `Filter` | `{ /// Liste von Prädikaten (Feldname, Wert). pub predicates: Vec<(String, String)>, }` | Filter für Suchanfragen (SPEC 7.2, D-62, D-124 Abs. 1). | `src/api.rs` |
| `HealthReport` | `{ /// Statusmeldung. pub status: &'static str, }` | Gesundheitsbericht der Engine ohne I/O (SPEC 7.1, D-64). | `src/api.rs` |
| `Hit` | `{ /// Dokument-Tag des Treffers. pub doc_tag: DocTag, /// Chunk-Index innerhalb des Dokuments. pub chunk: u32, /// Relevanz-Score. pub score: f32, }` | Treffer einer Suche (SPEC 7.2, D-124 Abs. 1). | `src/api.rs` |
| `Missing` | `(tuple/unit)` | Typestate-Marker: Parameter noch nicht gesetzt. | `src/api.rs` |
| `OsRng` | `(tuple/unit)` | OS-basierter Zufallszahlengenerator für Produktions-Defaults (Composition Root). | `src/wiring.rs` |
| `RestrictedAccess` | `{ /// Zweck des autorisierten Zugriffs. pub purpose: RestrictedPurpose, /// Attestierung des autorisierten Betreibers. pub attestation: OperatorAttestation, }` | Zugriffsberechtigung für eingeschränkte Daten nach Art. 18 Abs. 2 DSGVO (SPEC 7.2, D-116). | `src/api.rs` |
| `SearchPage` | `{ /// Trefferliste. pub hits: Vec<Hit>, /// Ob alle Ergebnisse erschöpft sind. pub exhausted: bool, }` | Paginierte Suchergebnisseite (SPEC 7.2, D-62). | `src/api.rs` |
| `Set` | `(tuple/unit)` | Typestate-Marker: Parameter gesetzt. | `src/api.rs` |
| `SignedReport` | `{ /// ID des Löschvorgangs. pub erasure_id: ErasureId, /// Rohbericht als formatierte Bytes oder JSON. pub payload: Vec<u8>, /// Kryptografische Signatur des Ber… }` | Signierter Bericht über eine durchgeführte Löschung (SPEC §7.1, 7.4, D-13, D-69). | `src/api.rs` |
| `SnapshotId` | `(tuple/unit)` | Snapshot-Identifikator für Zeitreisen (SPEC 7.2, D-124 Abs. 1). | `src/api.rs` |
| `SystemDefaults` | `{ /// Virtuelles Dateisystem für dauerhafte Operationen. pub vfs: Arc<dyn Vfs>, /// Systemuhr für Zeitmessung und Timestamps. pub clock: Arc<dyn Clock>, /// OS-C… }` | Standard-Systemverdrahtung für Acta-Instanzen. Bündelt `RealVfs`, Systemuhr und OS-CSPRNG zur Übergabe an die Engine (D-89, D-95). | `src/wiring.rs` |
| `Tenant` | `{ engine: Arc<acta_engine::Acta>, ctx: TenantContext, tenant_id: acta_types::TenantId, }` | Mandantengebundenes Handle (SPEC 7.1, D-07). | `src/api.rs` |
| `UpdateOptions` | `{ /// Bestimmt, wie mit vorigen Versionen verfahren wird. pub supersede: Supersede, }` | Optionen für Aktualisierungsoperationen (SPEC 7.2, D-117). | `src/api.rs` |
| `UpdateReceipt` | `{ /// Neue Versionssequenznummer. pub version: u64, /// Optionaler Löschbeleg, falls ein Purge alter Versionen erfolgte. pub erasure: Option<EraseReceipt>, }` | Quittung für eine Dokumentaktualisierung (SPEC 7.2). | `src/api.rs` |

### Öffentliche Datentypen — Enums (8)

| Enum | Varianten | Beschreibung | Datei |
|---|---|---|---|
| `ConfigDetails` | `/// Vault liegt unerlaubt im Datenverzeichnis (D-112). #[error("Schlüsseltresor darf nicht im Datenverzeichnis liegen. Bitte separates vault_dir konfigurieren.")] VaultPlacement, /// Vault-Verzeichni…` | Detaillierte Konfigurationsfehler gemäß SPEC 7.5 und 5.1.0-Entscheidungen. | `src/error.rs` |
| `ConflictDetails` | `/// Tag-Kollision (D-08). #[error( "Identitätskollision festgestellt. Transaktion wiederholen oder Schlüsselraum prüfen." )] IdCollision, /// Record existiert bereits. #[error("Eintrag existiert bere…` | Detaillierte Konfliktursachen gemäß SPEC 7.5 und D-124. | `src/error.rs` |
| `Error` | `/// Konfigurationsfehler beim Öffnen oder Starten des Systems. #[error("{0}")] Config(#[from] ConfigDetails), /// Pflichtparameter fehlt beim Erstellen der Instanz oder des Handles. #[error("Pflichtp…` | Öffentlicher Fehlertyp der Fassade (SPEC 7.5). | `src/error.rs` |
| `ErrorCode` | `/// Konfigurationsfehler allgemein. Config = 1000, /// Vault-Platzierung ungültig (D-112). ConfigVaultPlacement = 1001, /// Vault-Verzeichnis fehlt (D-112). ConfigVaultMissing = 1002, /// Vault-Param…` | Fester, eingefrorener Fehlercode zur stabilen programmatischen Identifikation (SPEC 7.5, D-110). | `src/error.rs` |
| `LimitDetails` | `/// Speicherkapazität erschöpft. #[error("Speicherkapazität erschöpft. Bereinigung durchführen oder Speicherplatz erweitern.")] Capacity, /// Plattenreserve unterschritten (D-65). #[error( "Mindest-P…` | Detaillierte Limitüberschreitungen gemäß SPEC 7.5 und D-87. | `src/error.rs` |
| `RestrictedPurpose` | `/// Rechtsansprüche geltend machen, ausüben oder verteidigen. LegalClaims, /// Schutz der Rechte einer anderen Person. ProtectionOfRights, /// Wichtiges öffentliches Interesse. PublicInterest, /// Mi…` | Zweck eines eingeschränkten Zugriffs nach Art. 18 Abs. 2 DSGVO (SPEC 7.2, D-116). | `src/api.rs` |
| `ScanRange` | `/// Alle Dokumente der Collection. All, /// Dokumente eines bestimmten Betroffenen (Subject). Subject(SubjectRef), /// Dokumente nach einem bestimmten Dokumenten-Handle. After(DocHandle),` | Bereichsauswahl für Scan-Iterationen (SPEC 7.2, D-41). | `src/api.rs` |
| `Supersede` | `/// Alte Versionen im Zeitreise-Horizont behalten (Standard). Keep, /// Alte Versionen physisch aus Schichten entfernen (Purge). Purge,` | Modus für das Ersetzen alter Versionen (SPEC 7.2, D-117). | `src/api.rs` |

### Typ-Aliase (1)

| Alias | Definition | Beschreibung |
|---|---|---|
| `Result` | `type Result<T> = std::result::Result<T, Error>` | Typalias für Result-Typen der Fassade. |

### Öffentliche Funktionen (40)

| Signatur | Beschreibung | Datei |
|---|---|---|
| `pub async fn acknowledge_ledger_reset(&self, _att: OperatorAttestation) -> Result<()>` | Bestätigt einen Ledger-Reset durch den Betreiber (SPEC 7.1). | `src/api.rs` |
| `pub fn backup_retention( self, duration: Duration, ) -> EngineBuilder<PassState, RtState, Set, DimState>` | Setzt die Pflicht-Aufbewahrungsdauer für Sicherungen (`backup_retention`, SPEC 5.7, D-74). | `src/api.rs` |
| `pub async fn build(self) -> Result<Acta>` | Alias für `open`. | `src/api.rs` |
| `pub fn builder() -> EngineBuilder<Missing, Missing, Missing, Missing>` | Erzeugt einen neuen `EngineBuilder` zur Konfiguration (SPEC 5.7, D-89). | `src/api.rs` |
| `pub async fn change_passphrase(&self, _old: &Secret, _new: &Secret) -> Result<()>` | Ändert die Passphrase des Masterschlüssels (SPEC 7.1). | `src/api.rs` |
| `pub async fn close(self) -> Result<()>` | Schließt die Instanz geordnet (SPEC 7.1). | `src/api.rs` |
| `pub async fn collection(&self, name: &str, opts: CollectionOptions) -> Result<Collection>` | Öffnet oder erstellt eine Collection für diesen Mandanten (SPEC 7.1, D-81). | `src/api.rs` |
| `pub fn context(&self) -> &TenantContext` | Liefert den zugehörigen Mandantenkontext. | `src/api.rs` |
| `pub fn data_dir(mut self, path: impl AsRef<Path>) -> Self` | Setzt das Datenverzeichnis (Standard: `./acta_data`). | `src/api.rs` |
| `pub fn dimension(self, dim: usize) -> EngineBuilder<PassState, RtState, RetState, Set>` | Setzt die Pflicht-Vektordimension (SPEC 5.7, D-89). | `src/api.rs` |
| `pub async fn erase(&self, _req: EraseRequest) -> Result<EraseReceipt>` | Führt einen Löschvorgang nach SPEC §7.3 aus. | `src/api.rs` |
| `pub fn erase_sla(mut self, sla: Duration) -> Self` | Setzt das Lösch-SLA als Dauer (SPEC 5.7). | `src/api.rs` |
| `pub async fn erase_status(&self, _id: ErasureId) -> Result<ErasureStatus>` | Prüft den Status einer laufenden oder abgeschlossenen Löschung (SPEC 7.1). | `src/api.rs` |
| `pub async fn get(&self, doc_ref: DocRef, _at: Option<SnapshotId>) -> Result<Option<Document>>` | Liest ein Dokument, optional zu einem Snapshot-Zeitpunkt (SPEC 7.2). | `src/api.rs` |
| `pub async fn get_restricted( &self, _doc_ref: DocRef, _access: &RestrictedAccess, ) -> Result<Document>` | Lesen nach Art. 18 Abs. 2 DSGVO, nur auf `Restricted`-Dokumenten zulässig (SPEC 7.2, D-116). | `src/api.rs` |
| `pub fn health(&self) -> HealthReport` | Liefert einen schnellen Gesundheitsbericht ohne I/O (SPEC 7.1, D-64). | `src/api.rs` |
| `pub async fn insert(&self, doc_ref: DocRef, doc: Document) -> Result<()>` | Fügt ein Dokument ein (SPEC 7.2). | `src/api.rs` |
| `pub async fn lift_restriction(&self, _id: ErasureId, _att: RetentionAttestation) -> Result<()>` | Hebt eine Einschränkung (Art. 18) wieder auf (SPEC 7.1). | `src/api.rs` |
| `pub async fn load(&self, _handle: DocHandle) -> Result<Document>` | Lädt das Dokument zu einem Suchtreffer-Handle (SPEC 7.2). | `src/api.rs` |
| `pub fn new() -> Self` | Erzeugt einen neuen leeren Filter. | `src/api.rs` |
| `pub fn new() -> Self` | Erzeugt einen neuen Builder mit Standardwerten für optionale Parameter. | `src/api.rs` |
| `pub async fn open(self) -> Result<Acta>` | Öffnet oder initialisiert die Datenbankinstanz asynchron. | `src/api.rs` |
| `pub fn passphrase(self, secret: Secret) -> EngineBuilder<Set, RtState, RetState, DimState>` | Setzt die Pflicht-Passphrase (SPEC 5.7). | `src/api.rs` |
| `pub async fn replace_erasing( &self, _old: DocRef, _new_ref: DocRef, _new_doc: Document, _retention: RetentionAttestation, ) -> Result<EraseReceipt>` | Ersetzung eines Mehr-Personen-Dokuments (SPEC 7.2, D-115). | `src/api.rs` |
| `pub async fn report(&self, _id: ErasureId) -> Result<SignedReport>` | Liefert den signierten Nachweisbericht für eine Löschung (SPEC 7.1, D-69). | `src/api.rs` |
| `pub fn runtime(self, handle: Handle) -> EngineBuilder<PassState, Set, RetState, DimState>` | Setzt das Tokio-Runtime-Handle (Pflicht, SPEC 5.4, 5.7). | `src/api.rs` |
| `pub async fn scan(&self, _range: ScanRange) -> Result<Vec<DocHandle>>` | Iteriert über Dokumente in `(cluster_tag, doc_tag)`-Reihenfolge (SPEC 7.2, D-41). | `src/api.rs` |
| `pub async fn scan_restricted( &self, _range: ScanRange, _access: &RestrictedAccess, ) -> Result<Vec<DocHandle>>` | Iteration über eingeschränkte Dokumente mit Audit-Eintrag (SPEC 7.2, D-116). | `src/api.rs` |
| `pub async fn search_hybrid( &self, query: &str, _vec: Vec<f32>, k: usize, filter: Option<Filter>, ) -> Result<SearchPage>` | Hybride Suche mit RRF-Fusion (SPEC 7.2). | `src/api.rs` |
| `pub async fn search_text( &self, query: &str, k: usize, _filter: Option<Filter>, ) -> Result<SearchPage>` | Volltextsuche mittels BM25 (SPEC 7.2). | `src/api.rs` |
| `pub async fn search_vector( &self, _vec: Vec<f32>, _k: usize, _filter: Option<Filter>, ) -> Result<SearchPage>` | Vektorsuche mittels HNSW (SPEC 7.2). | `src/api.rs` |
| `pub async fn snapshot(&self) -> Result<SnapshotId>` | Erzeugt einen mandantengebundenen MVCC-Snapshot (SPEC 7.2, D-61). | `src/api.rs` |
| `pub async fn soft_delete(&self, doc_ref: DocRef) -> Result<()>` | Für Löschpflichten `erase` verwenden. Setzt einen logischen Tombstone (SPEC 7.2, D-51). `soft_delete` markiert das Dokument lediglich logis… | `src/api.rs` |
| `pub async fn tenant(&self, ctx: TenantContext) -> Result<Tenant>` | Erzeugt ein mandantengebundenes Handle (SPEC 7.1, D-07). | `src/api.rs` |
| `pub async fn update(&self, doc_ref: DocRef, doc: Document) -> Result<()>` | Aktualisiert ein Dokument unter Erzeugung einer neuen Version (SPEC 7.2). | `src/api.rs` |
| `pub async fn update_with( &self, _doc_ref: DocRef, _doc: Document, _opts: UpdateOptions, ) -> Result<UpdateReceipt>` | Berichtigende Aktualisierung mit optionalem Purge alter Versionen (SPEC 7.2, D-117). | `src/api.rs` |
| `pub fn with_eq(mut self, key: impl Into<String>, val: impl Into<String>) -> Self` | Fügt ein Gleichheitsprädikat hinzu. | `src/api.rs` |
| `pub fn code(&self) -> ErrorCode` | Liefert den zugehörigen `ErrorCode` (SPEC 7.5, D-110). | `src/error.rs` |
| `pub fn new() -> Self` | Erzeugt einen neuen `OsRng`. | `src/wiring.rs` |
| `pub fn system_defaults() -> SystemDefaults` | Liefert die Standard-Systemverdrahtung (RealVfs, SystemClock, OsRng). Entspricht SPEC D-89 und D-95. | `src/wiring.rs` |


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
