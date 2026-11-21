# Claude
# Engram: eine Storage-Engine für LLM-Agenten (Rust)

## 1. Designthese

RocksDB und Co. optimieren für Key-Value-Zugriffe von Programmen. LLM-Agenten arbeiten anders:

| Programm | LLM-Agent |
|---|---|
| exakte Keys | unscharfe Anfragen (semantisch, assoziativ) |
| beliebig große Ergebnisse | **Token-Budget** pro Abfrage (Kontextfenster) |
| deterministisch | spekulativ: probiert, verwirft, korrigiert |
| vertraut den Daten | Daten können **Prompt-Injection** enthalten |
| Daten ändern sich | Daten werden **abgeleitet** (Summaries, Embeddings) und müssen reproduzierbar bleiben |

Die zentrale Idee: Die Datenbank liefert keine Zeilen, sondern **fertig gepackten Kontext** (`assemble`). Das Layout auf der Platte ist darauf ausgelegt.

## 2. Datenmodell

Alles ist ein **unveränderliches, inhaltsadressiertes Objekt** (BLAKE3-Hash), organisiert in einem Merkle-DAG. Objekttypen:

- **Doc**: Text als Baum (Dokument → Abschnitte → Chunks), pro Knoten Token-Zahl
- **Fact**: Triple oder Statement mit Bi-Temporalität (`valid_from/to`, `tx_time`)
- **Episode**: Agenten-Trace (Gedanke, Tool-Call, Ergebnis, Beobachtung)
- **Vec**: Embedding mit `model_id`, mehrere pro Objekt möglich
- **Blob**: Bilder, Dateien, KV-Cache-Tensoren
- **Derived**: Summary, Extraktion usw. mit Lineage (Modell, Prompt-Hash, Input-Hashes)

Jedes Objekt trägt ein **Trust-Label** (`system | user | tool | web | model`), das über Ableitungen propagiert wird. Eine Summary aus Web-Inhalten bleibt `web`-tainted.

## 3. Physisches Layout

**3.1 Token-natives Speichern.** Text liegt zusätzlich als Token-ID-Strom pro Tokenizer-Familie vor (zstd mit trainiertem Wörterbuch). Dadurch gibt es kein Re-Tokenisieren beim Packen, exakte Budget-Berechnung aus dem Index heraus und stabile Präfixe für Prompt-Caching.

**3.2 LOD-Cluster („Level of Detail“).** Ein Chunk, seine Summaries (L1/L2/L3), Embeddings und Metadaten liegen in **einem zusammenhängenden Extent**. Progressive Offenlegung (Outline → Abschnitt → Volltext) ist dann ein Sequential-Read statt vieler Random-Reads.

**3.3 Affinitäts-Clustering statt Key-Order.** Segmente werden nach **Ko-Retrieval** sortiert, nicht nach Key. Die Engine loggt, welche Objekte gemeinsam in einem `assemble` landen (Co-Access-Graph) und kombiniert das mit Embedding-Clustern (IVF-Zellen). Die Compaction ordnet physisch um. Die Sortierung ist `(space, cluster_id, hilbert(time), hash)`: ANN-Nachbarn und Dinge, die zusammen gebraucht werden, liegen auf denselben Seiten. Das ist der größte Unterschied zu LSM-Level-Compaction, die nur nach Key sortiert.

**3.4 Segmente.** Immutable Dateien mit festem Aufbau:

```
[Header][Column blocks: tokens | text | meta | trust]
[LOD blocks][Vector block (PQ + full)][FST/BM25 postings]
[Graph adjacency][Footer: Bloom, min/max, Merkle root, checksums]
```

Die Indizes (Volltext, Vektor, Graph) werden **pro Segment** gebaut. Weil Segmente unveränderlich sind, sind die Indizes automatisch versioniert und lassen sich beim Mergen einfach zusammenführen.

## 4. Versionskontrolle im Storage

Die Haupt-Indexstruktur ist ein **Prolly-Tree** (wie bei Dolt/Noms): history-independent, mit Structural Sharing.

- **Commit** = neue Wurzel-Hash, Atomizität über einen einzigen CAS auf den Branch-Pointer.
- **Branch** ist O(1). Jeder Agent-Run bekommt einen eigenen Branch. Rollback ist ein `reset`, kein Aufräumen.
- **Diff** kostet O(Änderungen), nicht O(Daten). Dadurch sieht man genau, was ein Agent verändert hat.
- **Merge** in drei Stufen: (1) strukturell automatisch, (2) typisierte CRDTs für Memory-Listen und Zähler, (3) ein Callback `resolve(conflict)`, den ein LLM oder Mensch bedient.
- **Time-Travel**: `search(... , at=commit)` funktioniert auf jedem Stand, weil auch Indizes versioniert sind.
- **Spekulative Forks** machen Tree-of-Thought oder Parallel-Agenten billig: N Branches, der beste wird gemergt.

Transaktionen sind Snapshot-Isolation: Writes gehen in den WAL und dann in ein neues Segment bzw. einen neuen Baum-Pfad. Die Wurzel wird erst nach `fsync` atomar umgehängt.

## 5. LLM-native API

```rust
pub struct ContextQuery {
    pub intent: String,                 // Freitext
    pub budget: TokenBudget,            // z.B. 6000 für Tokenizer "claude"
    pub policy: PackPolicy,             // diversity, recency, LOD-Strategie
    pub min_trust: Trust,               // z.B. Trust::User
    pub at: Option<CommitId>,
}

pub struct PackedContext {
    pub blocks: Vec<Block>,             // Text + Provenance + Handle + LOD
    pub tokens_used: u32,
    pub expandable: Vec<Handle>,        // "mehr davon" per Handle
    pub prefix_id: PrefixHash,          // stabiler Präfix für Prompt-Cache
}
```

Wichtige Eigenschaften:

- **`assemble`** macht Hybrid-Retrieval (BM25 + ANN + Graph-Expansion), Rerank und Greedy-Knapsack-Packing unter Token-Budget und Diversität. Je nach Budget wählt es pro Treffer das passende LOD.
- **Kurze Handles** (z. B. `d7k2`, eindeutig per Präfix des Hashs) statt UUIDs, weil LLMs lange IDs schlecht kopieren.
- **Fehler als handlungsleitender Text**: „Branch `run-42` existiert nicht. Vorhanden: …“.
- Eingebauter **MCP-Server** und ein schmales Tool-Set: `search`, `read(handle, lod)`, `remember`, `link`, `branch`, `diff`, `commit`.
- **Cursor mit Token-Budget** statt `LIMIT/OFFSET`.

## 6. Cache-Bewusstsein

- Die Packer-Ausgabe ist **deterministisch und präfix-stabil**: stabile Blöcke vorn, volatile hinten. Das erhöht die Trefferrate beim Provider-Prompt-Caching.
- Für lokale Modelle: **KV-Cache-Blobs** content-adressiert, als Radix-Baum über Token-Präfixe (wie SGLang) und mit Tiering GPU/RAM/NVMe/S3.
- Bei Modellwechsel: Embeddings sind mit `model_id` versioniert und werden **lazy re-embedded** in einem Hintergrundjob, während alte Vektoren weiter nutzbar bleiben.

## 7. Memory-Semantik

- Tiers: Working (Branch-lokal, flüchtig), Episodic (Traces), Semantic (Fakten), Procedural (Skills, Prompts).
- **Konsolidierung** als Compaction-Job: Episoden werden zu Facts und Summaries verdichtet, dedupliziert und mit Decay gewichtet. Jede Summary behält Pointer auf die Quellen, sodass Verdichtung **reversibel** bleibt.
- **Bi-Temporalität**: „Was wusste der Agent am Dienstag?“ ist von „Was galt am Dienstag?“ unterscheidbar.
- **Lineage** erlaubt: „Welche Summaries sind veraltet, weil sich Quelle X geändert hat?“ wird als Invalidierungsgraph ausgewertet.

## 8. Sicherheit

- **Taint-Tracking**: Web- und Tool-Inhalte werden in `assemble` markiert oder gefiltert, was Prompt-Injection über Memory eindämmt.
- Capabilities pro Space und Branch (read/append/merge). Ein Agent darf nur auf seinem Branch schreiben, `main` nur per Merge-Gate.
- **Verschlüsselung pro Space** mit eigenem Key. Löschen (DSGVO) in einem unveränderlichen Store geschieht per **Crypto-Shredding** plus Tombstone und anschließendem Rewrite in der Compaction.

## 9. Engine-Internals (Rust)

- **Thread-per-core** mit `io_uring` (z. B. `glommio` oder `tokio-uring`), O_DIRECT und eigener Buffer-Pool statt mmap (kontrollierte Latenz, keine Page-Fault-Überraschungen).
- Zero-Copy-Zugriff auf Segmente über `rkyv` oder eigenes Layout, `zstd`-Dictionaries, `blake3`, `fst`, `roaring`.
- WAL mit Group-Commit, danach Memtable → Segment. Die Memtable hält auch einen kleinen HNSW für frische Vektoren, bevor sie in den Segment-Index (DiskANN/Vamana oder IVF-PQ) gehen.
- Cold Tier = S3: unveränderliche Segmente lassen sich ohne Koordination hochladen und per Range-Request lesen.
- Zwei Betriebsarten: **eingebettet** (wie SQLite, eine Bibliothek pro Agent) und **Server** (gRPC + MCP, Multi-Tenant).

Crate-Aufteilung: `engram-core` (Objekte, Hashing), `-seg` (Segmentformat), `-tree` (Prolly/Branches), `-index` (BM25/ANN/Graph), `-pack` (Assemble), `-wal`, `-tier`, `-mcp`, `-server`.

## 10. Vergleich mit RocksDB

| Aspekt | RocksDB | Engram |
|---|---|---|
| Sortierung | Key | Affinität/Cluster |
| Versionierung | Snapshots (flüchtig) | Merkle-Commits, Branches, Merge |
| Indizes | extern | pro Segment, versioniert |
| Abfrageergebnis | Zeilen | token-budgetiertes Kontextpaket |
| Ableitungen | keine | Lineage + Invalidierung |
| Vertrauen | keines | Taint-Labels |
| Cache | Block-Cache | Block-Cache + Prompt-/KV-Prefix-Cache |

## 11. Roadmap

1. **MVP**: Objektmodell, Segmentformat, WAL, Prolly-Tree mit Branch/Commit/Diff (eingebettet).
2. Hybrid-Suche (BM25 + ANN) und `assemble` mit Token-Packing.
3. Affinitäts-Compaction und LOD-Cluster, Benchmarks gegen RocksDB + Qdrant + Postgres/pgvector auf Agenten-Workloads.
4. Merge-Strategien, CRDT-Typen, Taint, Verschlüsselung.
5. Server, MCP, S3-Tiering, KV-Cache-Store.

**Risiken**: Affinitäts-Clustering kann bei nicht stationären Workloads thrashen (Gegenmittel: Hysterese, nur nachts umordnen). Prolly-Trees haben Write-Amplification bei vielen Kleinstwrites (Gegenmittel: Batching in der Memtable). Die Qualität von `assemble` hängt am Reranker, deshalb sollte er pluggable sein.

Wenn du möchtest, schreibe ich als Nächstes das Segmentformat als Rust-Structs aus, skizziere den Prolly-Tree-Chunker oder fasse alles als Dokument zusammen.

---
# Gemini

Klassische Storage-Engines wie RocksDB (LSM-Trees) oder B-Tree-Datenbanken sind für unabhängige, punktförmige Key-Value-Mutationen optimiert – genau das Gegenteil dessen, was LLMs und autonome Agenten benötigen. LLM-Workloads bestehen aus **verzweigten, append-lastigen Kontext-DAGs (Directed Acyclic Graphs)**: Identische Präfixe (System-Prompts, RAG-Kontexte, Tool-Definitionen) werden tausendfach geteilt, spekulativ geforkt (Tree-of-Thoughts, MCTS, Agent-Retries) und müssen für minimale Time-To-First-Token (TTFT) als physisch zusammenhängende Blöcke aus Tokens, Vektoren und KV-Cache-Tensoren direkt in den Speicher gestreamt werden.

  

Das Konzept für **AetherStore** – eine in Rust geschriebene, LLM-native Storage-Engine – ersetzt LSM-Trees durch eine **Copy-on-Write (CoW) Merkle-Radix-Arena-Architektur** mit nativer Git-Semantik und Hardware-naher Zero-Copy-Lokalität.

  

## 1. Paradigmenwechsel: Warum RocksDB für LLMs suboptimal ist

|**Architektur-Dimension**|**RocksDB (LSM-Tree)**|**Dedizierte Vector-DB**|**AetherStore (LLM-Native Engine)**|
|---|---|---|---|
|**Primäre Datenstruktur**|Sortierte SSTables + MemTable|HNSW / IVF Graph + Metadata Store|**Persistent CoW Merkle-Radix-DAG**|
|**Branching / Versionierung**|Teure Snapshots; kein echtes Prefix-Sharing bei Forks|Kaum vorhanden (Mutable Collections)|**$O(1)$ Zero-Copy Branching** (Git-like Commits & Rollbacks)|
|**Physische Lokalität**|Keys lexikografisch sortiert; Werte getrennt/fragmentiert|Vektoren getrennt vom eigentlichen Payload-Text|**Co-Located Context Slabs** (Tokens + Embeddings + KV-Cache benachbart)|
|**Write-Verhalten**|Hohe Write-Amplification durch Background-Compaction|In-Place Graph-Updates mit Lock-Contention|**Append-Only Shadow Paging** (kein WAL für Payloads nötig)|
|**I/O & Serialisierung**|Protobuf/JSON Deserialisierung über CPU-Heap|gRPC/REST + Heap-Allokationen|**4KB-aligned Zero-Copy DMA** (`io_uring` & GPUDirect Storage)|

## 2. Physische Datenlokalität: Co-Located Context Slabs

In herkömmlichen Agenten-Stacks liegen der Konversationstext in Postgres, die Embeddings in Qdrant und die vorberechneten Attention-KV-Caches im RAM oder Redis. Bei jedem Agenten-Step entstehen Serialisierungs- und Netzwerk-Overheads.

  

AetherStore organisiert Daten stattdessen in **Context Slabs** (physische 64-KB- bis 2-MB-Arenen, ausgerichtet an OS-HugePages und 4096-Byte-NVMe-Sektoren). Innerhalb eines Slabs werden zusammengehörige Daten physisch direkt hintereinander abgelegt (**Array-of-Structures-of-Arrays Layout**):

  

1. **Token-Block-Alignment (PagedAttention-kompatibel):** Text wird nicht als beliebiger String gespeichert, sondern direkt beim Schreiben tokenisiert und in feste Seiten (z. B. 64 Tokens pro Page) quantisiert. Das eliminiert wiederholte Tokenisierung beim Laden.
    
2. **Inline-Embeddings:** Direkt neben dem Token-Block liegt das quantisierte Embedding (Int8/Binary für schnelle Grobfilterung, FP16 für Rescoring). Ein einziger NVMe-Read lädt Vektor und Text gleichzeitig in den CPU-L2-Cache.
    
3. **KV-Cache-Tensor-Slots:** Für häufig genutzte Präfixe (System-Prompts, Codebases) speichert der Slab direkt angrenzend die vorberechneten Key-Value-Attention-Tensoren (4KB-aligned). Über `io_uring` und NVIDIA GPUDirect Storage (`cuFileRead`) können diese ohne Umweg über den CPU-RAM direkt in den GPU-VRAM gestreamt werden.
    
4. **Trajectory-Packing:** Da ein LLM beim Inferenz-Start den gesamten Pfad von der Wurzel bis zum aktuellen Blatt (Root-to-Leaf Trajectory) benötigt, schreibt der Arena-Allocator aufeinanderfolgende Agenten-Steps sequenziell auf denselben physischen NVMe-Block. Ein einziger Sequential-Read ersetzt hunderte Random-IOPS.
    

## 3. Native Versionskontrolle & Atomare Transaktionen

Agenten arbeiten nicht linear: Sie verwerfen Halluzinationen, testen parallele Tool-Pfade (Monte-Carlo Tree Search) und benötigen exakte Reproduzierbarkeit für Evals und Debugging.

  

- **Content-Addressable Merkle-DAG:** Jeder unveränderliche Token- oder Memory-Block wird über seinen **BLAKE3-Hash** identifiziert. Identische System-Prompts oder Dokument-Chunks existieren physisch nur ein einziges Mal auf der Disk (automatische Deduplizierung).
    
- **$O(1)$ Branching & Time-Travel:** Ein Konversationsverlauf oder Agenten-Zustand ist lediglich ein Zeiger (`CommitId`) auf die Wurzel eines persistenten Radix-Trees (Branching-Faktor 32). Ein `fork()` klont nur diesen 32-Byte-Hash – völlig ohne Daten zu kopieren.
    
- **Atomare Commits ohne klassisches WAL:** Da alle Datenblöcke immutable (Copy-on-Write) sind, schreibt eine Transaktion zuerst die neuen Slabs per `io_uring` auf freie Sektoren. Erst wenn alle Blöcke persistent sind, aktualisiert ein einzelner **atomarer Compare-And-Swap (CAS)** im Ring-Buffer-Superblock den `BranchHead`. Ein Crash während des Schreibens hinterlässt niemals korrupte Zustände; unreferenzierte Slabs werden beim Neustart einfach ignoriert.
    

## 4. Integrierter Semantischer & Struktureller Index

Statt eines externen Vektor-Index nutzt AetherStore ein **Branch-Aware DiskANN-Overlay**:

  

- Index-Knoten verweisen nicht auf abstrakte UUIDs, sondern direkt auf physische `(ArenaId, ByteOffset, CommitMask)`-Tupel.
    
- Die `CommitMask` (ein komprimiertes Roaring-Bitmap) filtert während des Graph-Traversals in Nanosekunden alle Vektoren heraus, die nicht zum aktuellen Agenten-Branch oder Zeitpunkt gehören. So gibt es keine "Phantom-Ergebnisse" aus verworfenen Agenten-Pfaden.
    

## 5. Rust-Systemarchitektur & Core-Implementierung

Um Lock-Contention bei hochparallelen Agenten-Swarms zu vermeiden, nutzt AetherStore eine **Thread-per-Core-Architektur** (analog zu Glommio/Seastar), kombiniert mit `io_uring` für asynchrones Direct-I/O und Epoch-Based Reclamation (`crossbeam-epoch`) für lock-freie Lesezugriffe.

  

Rust

```
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// 32-Byte BLAKE3 Content-Addressable Hash für Block-Deduplizierung & Merkle-Verification
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(C)]
pub struct ContentHash(pub [u8; 32]);

/// Physische Adresse auf NVMe: Arena-ID + 4KB-aligned Offset + Länge
#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct PhysicalExtent {
    pub arena_id: u32,
    pub page_offset: u32, // Multipliziert mit 4096 ergibt exakten Byte-Offset
    pub page_count: u16,
    pub flags: u16,       // Bitflags: HasKvCache, CompressedZstd, QuantizedInt8
}

/// 4096-Byte-aligned Context-Page für Zero-Copy Direct-I/O & GPUDirect Storage.
/// Enthält physisch benachbart: Metadata, Token-IDs, Quantized Embedding & KV-Cache-Ref.
#[repr(C, align(4096))]
pub struct AlignedContextPage {
    pub magic: u32,
    pub token_count: u16,
    pub embedding_dim: u16,
    pub parent_hash: ContentHash,
    pub content_hash: ContentHash,
    /// Vor-tokenisierte IDs (z. B. 64 Tokens * 4 Bytes = 256 Bytes)
    pub token_ids: [u32; 64],
    /// Quantisiertes Int8-Embedding direkt im selben Cache-Line-Bereich (z. B. 768 Dim)
    pub quantized_embedding: [i8; 768],
    /// Optionaler Verweis auf ausgelagerte Attention-KV-Tensoren im High-Speed-Tier
    pub kv_cache_extent: PhysicalExtent,
    /// Restlicher Platz der 4KB-Page für komprimierte Raw-Bytes / Tool-JSON-Payloads
    pub inline_payload: [u8; 2944],
}

/// Ein atomarer Snapshot eines Agenten-Zustands (Git-Commit-Äquivalent)
#[derive(Clone, Debug)]
pub struct AgentCommit {
    pub commit_id: ContentHash,
    pub parent_commit: Option<ContentHash>,
    /// Wurzel des persistenten CoW-Radix-Trees für alle Kontext-Blöcke dieses Branches
    pub context_radix_root: PhysicalExtent,
    /// Roaring-Bitmap-Root für Branch-isolierte Vektorsuche
    pub vector_visibility_root: PhysicalExtent,
    pub timestamp_ns: u64,
}

/// Lock-freier Branch-Zeiger für O(1) Forks und atomare Updates
pub struct BranchHead {
    pub name: String,
    /// Atomarer Pointer auf die aktuelle Commit-Version (Slot-Index im Superblock)
    current_commit_slot: AtomicU64,
}

pub struct AetherEngine {
    // Thread-lokale io_uring Queues & Arena-Allokatoren würden hier verwaltet
    superblock_slots: Arc<Vec<AtomicU64>>,
}

impl AetherEngine {
    /// Erzeugt einen neuen Branch in O(1) ohne jegliches Kopieren von Kontextdaten.
    pub fn fork_branch(&self, source: &BranchHead, new_name: String) -> BranchHead {
        let active_slot = source.current_commit_slot.load(Ordering::Acquire);
        // Refcount für den zugrundeliegenden Merkle-Root wird atomar erhöht
        self.increment_slab_refcount(active_slot);

        BranchHead {
            name: new_name,
            current_commit_slot: AtomicU64::new(active_slot),
        }
    }

    /// Hängt einen neuen Inferenz-/Tool-Step atomar an einen Branch an (Optimistic Concurrency).
    pub fn atomic_append_step(
        &self,
        branch: &BranchHead,
        expected_slot: u64,
        new_page: &AlignedContextPage,
    ) -> Result<u64, StorageError> {
        // 1. Schreibe AlignedContextPage ohne Locks in die aktuelle thread-lokale Arena (Shadow Paging)
        let extent = self.write_page_direct_io(new_page)?;

        // 2. Erzeuge neuen CoW-Radix-Pfad (nur O(log_32 N) Knoten werden neu geschrieben)
        let new_commit_slot = self.build_cow_commit(expected_slot, extent)?;

        // 3. Atomarer Commit via CAS: Schlägt fehl, falls ein anderer Worker denselben Branch mutiert hat
        branch
            .current_commit_slot
            .compare_exchange(
                expected_slot,
                new_commit_slot,
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .map_err(|_| StorageError::ConcurrentBranchConflict)
    }

    /// Zero-Copy-Zugriff auf eine komplette Root-to-Leaf-Trajektorie als Speicher-Slices
    pub fn read_trajectory_zero_copy(&self, _branch: &BranchHead) -> Vec<&AlignedContextPage> {
        // Mappt die physisch benachbarten Extents direkt via mmap / io_uring Registered Buffers
        // ohne Heap-Allokation oder Deserialisierung in den Adressraum der Inferenz-Engine.
        unimplemented!("Direct-I/O & Registered-Buffer Mapping")
    }

    fn write_page_direct_io(&self, _page: &AlignedContextPage) -> Result<PhysicalExtent, StorageError> {
        Ok(PhysicalExtent { arena_id: 1, page_offset: 0, page_count: 1, flags: 0 })
    }

    fn build_cow_commit(&self, _parent_slot: u64, _extent: PhysicalExtent) -> Result<u64, StorageError> {
        Ok(42)
    }

    fn increment_slab_refcount(&self, _slot: u64) {}
}

#[derive(Debug)]
pub enum StorageError {
    ConcurrentBranchConflict,
    IoFailure,
}
```

## 6. Storage-Tiering & Trajectory-Aware Garbage Collection

Da Agenten bei Tree-Search-Verfahren (z. B. Code-Generierung mit Unit-Test-Feedback) bis zu 90 % aller erkundeten Branches wieder verwerfen, benötigt die Engine eine spezialisierte Bereinigung:

- **Automatisches Multi-Tiering:**
    
    - _Tier 0 (GPU VRAM / Host DRAM):_ Aktive KV-Caches und die obersten Ebenen des Merkle-Radix-Trees.
        
    - _Tier 1 (Lokale NVMe via `io_uring`):_ Vollständige `AlignedContextPages` inklusive Int8-Embeddings und warmen KV-Caches.
        
    - _Tier 2 (Object Storage / S3):_ Abgeschlossene Agenten-Sessions werden asynchron in spaltenorientierte Arrow/Parquet-Archiv-Slabs verdichtet, behalten aber ihre Content-Hashes für spätere Audits.
        
- **Epoch-Based GC & Hot-Path-Defragmentierung:** Wird ein Branch gelöscht (`drop_branch`), dekrementiert die Engine atomar die Referenzzähler entlang des Merkle-Pfads. Sobald ein Slab unter eine Auslastung von 40 % fällt, kopiert ein Hintergrund-Worker die verbliebenen "lebenden" Pfade sequenziell in eine neue Arena (**Hot-Trajectory-Packing**), sodass die physische Leselokalität über Monate hinweg optimal bleibt, ohne jemals die Latenz laufender Inferenz-Requests zu stören.

---

# grok

**Konzept: “AetherStore” – eine LLM-/Agenten-native Storage-Engine in Rust**

### Design-Ziele (explizit gegen RocksDB & Co.)
- **Physische Lokalität zuerst**: Daten, die ein LLM/Agent zusammen braucht (Embedding + Kontext-Chunks + Metadaten + Version-History), liegen auf dem Medium und im RAM so nah beieinander wie möglich.
- **Atomare, versionierte Mutation** ohne klassische B-Tree/LSM-Overheads.
- **Native Unterstützung für die drei Kern-Datenarten von Agents**:
  1. Dense/Sparse Embeddings + quantisierte Varianten
  2. Token-/Text-Chunks mit hierarchischer Kontext-Struktur
  3. Graph-/Relation- und Event-Streams (Trajectories, Tool-Calls, Episodic Memory)
- Schreibpfad extrem write-friendly und lesepfad extrem cache-freundlich (LLM-Inference und Retrieval sind stark leselastig und stark lokal).
- Eingebaute Versionskontrolle (branch/merge/snapshot) auf Storage-Ebene, nicht als Overlay.
- Zero-copy und mmap-first wo sinnvoll, aber mit sicheren Rust-Abstraktionen.

RocksDB (LSM), LMDB, SQLite, und die meisten Vector-DBs sind entweder generisch key-value oder nur für einen Aspekt (Vektoren) optimiert. AetherStore ist von Anfang an um die Zugriffsmuster von RAG + Agent-Memory + Trajectory-Versionierung herum gebaut.

### Kern-Architektur

#### 1. Physisches Layout: “Semantic Slabs” + “Locality Groups”
Statt reiner Key-Value oder reiner LSM:

- **Slab** = festes, page-aligned (meist 64 KiB–2 MiB) physisches Speicherstück.
- Jeder Slab gehört zu einer **Locality Group**. Eine Locality Group fasst alle Daten zusammen, die ein Agent/LLM typischerweise in einem “Arbeitskontext” braucht:
  - Embedding(s) eines Dokuments/Chunks
  - Der eigentliche Text/Token-Stream (komprimiert)
  - Metadaten + Tags
  - Graph-Kanten zu verwandten Chunks
  - Die Version-History dieses Objekts (als delta-chain)

Dadurch wird ein Cache-Miss oder ein Disk-Seek extrem teuer vermieden. Der Storage-Engine-Scheduler hält ganze Locality Groups hot, wenn ein Agent an ihnen arbeitet.

#### 2. Adressierung & Content-Addressability
Jedes logische Objekt hat:
- Einen **logischen Key** (user-defined, z. B. `agent:42/memory/episodic/2026-11-21`)
- Einen **Content-Hash** (BLAKE3) → echte Content-Addressability
- Eine **Version-ID** (monoton steigend oder Merkle-Root)

Das ermöglicht:
- Deduplizierung auf Block-Ebene
- Günstige Snapshots und Branches (nur Root-Zeiger ändern)
- Integritätsprüfung ohne Extra-Layer

#### 3. Versionierung (first-class)
- Jede Mutation erzeugt eine neue Version (Copy-on-Write auf Slab-Ebene oder Delta).
- Es gibt explizite **Branches** (z. B. `main`, `agent-run-7`, `experiment-rlhf`).
- Snapshots sind billig (nur Root-Pointer + Referenzzählung).
- Merge ist möglich (3-way oder operational transform für Text + Graph).
- Garbage Collection läuft asynchron und behält konfigurierbare History-Tiefe pro Locality Group.

Das ist deutlich stärker als “RocksDB + manuelle Versionierung” oder Git-ähnliche Overlays.

#### 4. Daten-Typen (native)
| Typ                  | Speicherung                          | Besonderheit                              |
|----------------------|--------------------------------------|-------------------------------------------|
| Embedding            | quantisiert (int8/fp16/bf16) + optional raw | SIMD-freundliche Layouts, ANN-Index lokal |
| Text/Tokens          | zstd oder spezielles Token-Delta     | Hierarchische Chunks (Sentence → Paragraph → Doc) |
| Graph / Relations    | adjacency lists + edge attributes    | Locality-aware Partitionierung            |
| Event / Trajectory   | append-only log + index              | Zeitreihen + causal ordering              |
| Structured State     | MessagePack / Cap’n Proto            | Schema-evolution-fähig                    |

#### 5. Indexe (multi-modal, co-located)
- **ANN-Index** (HNSW oder IVF + PQ) lebt *innerhalb* der Locality Group oder als Secondary Index mit Pointern zurück in die Slabs.
- **Inverted + Semantic Index** für Keyword + Hybrid-Search.
- **Temporal Index** für Agent-Trajectories und “was wusste der Agent zum Zeitpunkt X”.
- Alle Indexe können versioniert und branched werden.

#### 6. Transaktionen & Atomarität
- Optimistic Concurrency Control + MVCC auf Versionsebene.
- Atomic Multi-Object Updates innerhalb einer Locality Group sind extrem billig (ein Slab-Write + Atomic Root-Update).
- Cross-Group Transaktionen nutzen 2PC oder eine leichte WAL nur für die Commit-Records.

#### 7. Speicherhierarchie
```
L0  – Hot Working Set   (in-memory, pinned Locality Groups)
L1  – Warm              (NVMe, mmap)
L2  – Cold / Archive    (object storage oder HDD, compressed)
```
Der Engine entscheidet automatisch (oder per Hint vom Agenten), welche Groups hot bleiben. LLM-Inference-Pattern (wiederholte Zugriffe auf denselben Kontext) werden erkannt und bevorzugt.

### Rust-Implementierungs-Skizze

**Kern-Crates (vorgeschlagen):**
- `aether-core` – Slab-Allocator, Locality-Group-Manager, Version-Graph
- `aether-index` – HNSW/IVF + inverted indexes (SIMD mit `wide` oder `simsimd`)
- `aether-codec` – Embedding-Quantisierung, Token-Delta, zstd
- `aether-tx` – MVCC + Atomic Commit
- `aether-fs` – mmap + io_uring (Linux) / io_uring-ähnliche Abstraktion
- `aether-api` – high-level Agent-Memory-API (`store_memory`, `retrieve_relevant`, `branch`, `snapshot`, `merge`)

Wichtige Design-Entscheidungen in Rust:
- Arena-Allocator pro Locality Group → fast zero-fragmentation und excellent cache behavior.
- `Arc` + generational indices oder epoch-based reclamation für concurrent readers.
- `io_uring` + `mmap` hybrid: Writes gehen über io_uring, Reads bevorzugt über mmap (zero-copy in den LLM-Tensor-Pfad).
- Kein globaler Lock; Sharding nach Locality-Group-Hash.
- Optional: GPU-offload für ANN-Rebuild und Quantisierung (via `cudarc` oder `wgpu`).

### API-Beispiel (Agent-Perspektive)

```rust
let store = AetherStore::open("agent_memory.aether")?;

// Atomic, versionierter Write einer ganzen Locality Group
store.with_tx(|tx| {
    tx.put_embedding("doc:42", &emb, Quantization::Int8)?;
    tx.put_text("doc:42", chunk_text)?;
    tx.link("doc:42", "doc:17", Relation::Similar)?;
    tx.commit_branch("agent-run-7")
})?;

// Retrieval mit physischer Lokalität
let results = store.retrieve(
    Query::hybrid("was war der Status von Projekt X")
        .with_graph_hops(2)
        .at_version("agent-run-7")
        .limit(12)
)?;
```

### Warum das besser als RocksDB + Vector-DB-Kombination ist
- RocksDB optimiert für generische Key-Value-Writes → schlechte räumliche Lokalität für Embeddings + Text + Graph.
- Separate Vector-DBs (Milvus, Qdrant, …) haben ihren eigenen Speicher und erzwingen Netzwerk oder teure Joins.
- Versionierung ist meist Overlay → teuer und inkonsistent.
- AetherStore macht die Locality Group zur atomaren Einheit und die Version zum first-class Bürger. Dadurch sinken Cache-Misses und I/O dramatisch, und Agenten können sicher experimentieren (Branch → Train/Run → Merge oder Discard).

### Erweiterungen / Roadmap-Ideen
- Native Support für “Memory Consolidation” (LLM-gesteuertes Zusammenfassen alter Versionen).
- Hardware-aware Layout (CXL, persistent memory, GPU-direct storage).
- Built-in “Attention Cache” – der Engine merkt sich, welche Chunks in den letzten Inference-Läufen hot waren.
- Plugin-System für custom Codecs und Similarity-Metriken.

Das Konzept ist bewusst radikal lokalitäts- und versionszentriert. Es opfert etwas generische Key-Value-Flexibilität zugunsten der realen Zugriffsmuster von modernen LLM-Agents. In Rust lässt sich das mit sehr hoher Performance und Speichersicherheit realisieren.
