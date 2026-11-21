contextra-storage wird das neue und optimierte fundament für das in Archiv.zip System.
Dies darf nur in seltenen Fällen entpackt werden, und wichtig ist das die Schnittstellen von contextra-storage korrekt sind, die Crates/Funktionalität wird inkrementell aus Archiv.zip zurückgeholt um auf diesem maximal sauber konzipierten Storage Fundament aufbauend zu funktionieren.

---

Der Plan, `contextra-storage` (zusammen mit `contextra-ports`) als eigenständiges, abhängigkeitsfreies Fundament neu zu bauen und die übrigen Contextra-Domänen erst danach schrittweise darauf aufzusetzen, ist architektonisch der **richtige Schnitt**. Im Altstand (`crates/contextra-store`, knapp 60 000 Zeilen) waren Speicher-Engine, MVCC/SSI-Transaktionspuffer, Mandanten-Logik, Purger, 420 `async`-Funktionen über blockierender E/A, drei Format-Generationen und sechs globale Test-Schalter untrennbar verflochten. `SPEC.md` löst genau diese Kopplung auf und liefert eine Spezifikation, die in ihrer Stringenz deutlich über das ursprüngliche Mnemos-Konzept hinausgeht.

  

### Was an dem Entwurf (`SPEC.md`) besonders stark ist

1. **Saubere Port-Entkopplung ohne Abhängigkeits-Sog (`contextra-ports`)** Dass `contextra-storage` ausschließlich von `contextra-ports` und `std` abhängt (`forbid(unsafe_code)`), während SIMD-Prüfsummen (`contextra-simd`), AEAD/KDF (`contextra-crypto`), Tokenizer-Gewichte (`Weigher`), Metadatenfilter (`Filter`), Vergessen (`CompactionFilter`), Run-Indizes (`Extension`) und Vektorgraphen (`Segment`) von oben eingehängt werden, verhindert zyklische Abhängigkeiten. Gleichzeitig bleibt der gesamte Speicherkern deterministisch testbar.
    
2. **Gelöster Zielkonflikt zwischen Mnemos-Features und physischem `Erase`** Die Entscheidung gegen einen Prolly-Tree / Merkle-DAG als Hauptstruktur (`NZ-12`, Anhang C) ist richtig: Inhaltsadressierte Deduplizierung widerspricht garantierter physischer Löschung (`Z-3`) und `CompactionFilter`-Kürzungen. Stattdessen übernimmt `SPEC.md` die besten Ideen aus `MNEMOS_CONCEPT.md` in ein beweisbares LSM+Blob-Design:
    
    - **Header-zuerst-Lesen & Wert-Trennung (`separate_body`, `PutRef`, `BlobRef`):** Entspricht der Hot-Head/Cold-Tail-Trennung. Filter (`Filter::keep`) und Gewichte laufen ohne Körperzugriff, und `view.bodies(&entries)` sortiert Nachladungen nach `(blob_no, offset)` und fasst Lücken $\le 16\text{ KiB}$ zu zusammenhängenden Reads zusammen.
        
    - **Token-native Seitenbildung (`weights(k)`, `estimate`, `scan_page`):** Index-Blöcke summieren bis zu 4 `u64`-Gewichte (inklusive `weigher_version`), `estimate` liefert obere Schranken in $O(\text{Index})$, und `scan_page` schneidet exakt am Token-/Byte-Limit mit Fortschrittsgarantie (`oversized`) und `Mac16`-gesichertem `ResumeToken`.
        
    - **Kryptografisches Löschen je Subjekt ohne Ordnungsverlust (`KeyDomain` + `domain_sealed`, `SD-42`):** Da die Verschlüsselung pro Subjekt auf _Körper_-Ebene statt auf Block-Ebene ansetzt, bleibt die sortierte Schlüssel-Lokalität im LSM erhalten, während Körper bei Schlüsselvernichtung sofort in Redo _und_ Blobs unlesbar werden (`INV-ERA-7`). Für physisches `Erase` erlaubt die logische `blob_no` + `generation` zudem einen Offset-stabilen Blob-Sweep (Hole-Records mit Nullen, unverschlüsselt nach `INV-BLOB-3`), ohne Runs neu schreiben zu müssen.
        
3. **Mathematisch und technisch saubere Härtung**
    
    - Die Reduktion auf **sieben streng geordnete Sperrränge** mit `loom`-Modellen, dem Beweis der Lese-Reihenfolge (`visible_stamp` _vor_ `Version`, `INV-VIS-3`) und der monotonen Untergrenze (`INV-RET-1`) eliminiert die Deadlock- und Race-Risiken des Altstands.
        
    - Details wie **Siegel-Frames** (`INV-LOG-6`), **Redo-HWM-Verankerung** gegen Lost Writes (`INV-LOG-8`), **ID-Hochwassermarken vor Nutzung** (`INV-KEY-2`), disjunkte Nonce-Präfixe (`0x01..0x05`, Erratum E-6) und vorberechnete Python-Goldens in Anhang A verhindern ganze Klassen subtiler Absturz- und Krypto-Bugs.
        

### Kritische Reibungspunkte für den späteren Aufbau der übrigen Contextra-Crates

Beim Übergang von `contextra-storage` zu den oberen Schichten (`contextra-db`, `contextra-vector`, `contextra-graph`, `contextra-cognition`, `contextra-agent`, `contextra-privacy`) gibt es sechs Stellen, an denen ihr jetzt schon aufpassen solltet:

  

#### 1. `MergeOperator`-Vakuum (`SD-12`, `B-15`) vs. GraphRAG und Kognition

Im Altstand nutzt `contextra-db` einen `MergeOperator` (WASM-Brücke), und in den oberen Contextra-Domänen fallen regelmäßig In-Place-Akkumulationen an – etwa beim **Edge-Reinforcement** ($w_{uv} \leftarrow w_{uv} + \eta \cdot r$), beim **Memory Decay** ($A(t)$) oder bei Zählern. `SPEC.md` verweist all das auf Read-Modify-Write-Schleifen über `Guard::equals` / `unchanged_since` (`SD-12`).

  

- **Risiko:** Wenn mehrere Agenten-Schritte oder Konsolidierungs-Jobs parallel dieselben Graph-Kanten oder Access-Stats aktualisieren, führt `Guard::equals` unter dem globalen `committer`-Lock zu unnötigen Konflikten (`Error::Conflict`) und Retries.
    
- **Empfehlung:** Dass `kind = Merge (6)` im Tag-Byte reserviert ist, ist goldrichtig. Haltet euch offen, `Merge` nach dem MVP gezielt für assoziative/kommutative Deltas (Zähler, Kantengewichte) zu aktivieren, statt komplexe WASM-Merges erzwingen zu müssen.
    

#### 2. Rückstände gelöschter Daten in Vektor- und Volltext-Segmenten (`O-2`)

`SPEC.md` benennt es unter `O-2` und `§12.5` selbst: Nach `Erase::range` oder `Erase::domain` sind Runs und Blobs bereinigt, aber **Vektor-Segmente (`HNSW`/`DiskANN`) und Volltext-Postings enthalten noch Embeddings und Terme des gelöschten Subjekts**. Ohne `rewrite_without(range)` verwirft `contextra-storage` bei Überlappung das _gesamte_ Segment.

  

- **Risiko:** Bei Millionen Vektoren ist ein kompletter Segment-Neuaufbau bei jeder Einzellöschung unbezahlbar – und bei `Erase::domain` (`Cryptographic`) gibt es gar keinen Range-Scan, sodass gelöschte Vektoren im Segment zunächst auffindbar blieben.
    
- **Empfehlung:** Baut in `contextra-vector` (über euren zweiphasigen _Delete-and-Backfill_-Algorithmus `hnsw_delete_and_backfill`) und `contextra-text` von Beginn an Tombstone-Bitmaps/Delete-Listen als leichte Delta-Segmente ein und koppelt diese an den `Consumer`-Stream (`domain_retired` + `history`).
    

#### 3. Synchrone `Guard`-I/O unter dem globalen `committer`-Lock (`§10.2`)

`apply` hält genau eine globale `committer`-Sperre über den Gruppen-`fsync` – und prüft unter derselben Sperre die `Guard`s gegen den neuesten Stand. Trifft ein `Guard::equals` oder `unchanged_since` auf einen Block, der nicht im `BlockCache` liegt, blockiert dieses synchrone Lesen die gesamte Schreib-Pipeline für alle Spaces.

  

- **Empfehlung:** Die in `§10.2 Schritt 1` erwähnte **Guard-Vorwärmung** (benötigte Blöcke vor dem Erwerb des `committer`-Locks in den `BlockCache` laden) sollte kein optionales „SOLL“, sondern fester Bestandteil des `apply`-Pfads sein. Prüft `H-S2` in Phase F5 explizit mit kaltem Cache und vielen `Guard`s.
    

#### 4. BTreeMap-Memtable vs. Allokationsdruck (`H-S7`, `§17.5`)

Eine `RwLock<BTreeMap<InternalKey, Slot>>` in `std` erzeugt pro Eintrag eine eigene Heap-Allokation (anders als eine Arena-basierte SkipList). Bei vielen kleinen Agenten-Writes und 16 MiB Memtable pro Space entsteht hoher Allokations- und Pointer-Overhead.

  

- **Empfehlung:** Euer Abbruchkriterium in `§20` (Wechsel auf eine isolierte SkipList, falls `H-S7` um $> 2\times$ verfehlt wird) ist genau richtig. Messt diesen Punkt früh in F5, bevor ihr obere Schichten darauf optimiert.
    

#### 5. Krypto- und Speicher-Abgleich mit `contextra-crypto` / `contextra-vault`

Zwischen `SPEC.md` und der Beschreibung der übrigen Contextra-Domänen gibt es zwei kleine Differenzen, die dank `contextra-ports` sauber auflösbar sind, aber bewusst abgestimmt werden sollten:

  

- **AEAD-Verfahren:** `SPEC.md §15` nennt `ChaCha20-Poly1305` oder `AES-256-GCM`, während `contextra-crypto` auf **AES-256-GCM-SIV** (Nonce-Reuse-resistent) setzt. Da `Cipher` in `contextra-ports` lediglich 32-Byte-Keys, 12-Byte-Nonces und 16-Byte-Tags verlangt, passt AES-256-GCM-SIV nahtlos in den Port und macht eure strenge Nonce-Konstruktion (`INV-KEY-1`) zusätzlich gegen Implementierungsfehler immun.
    
- **RAM-Schutz (`Secret32` vs. `mlock`):** `SPEC.md §15.7` verzichtet wegen `forbid(unsafe_code)` auf `mlock` (`Best-Effort zeroize`), während `contextra-sys` / `contextra-vault` `mlock`/`VirtualLock` und `ZeroizeOnDrop` bereitstellen. Haltet Master-Schlüssel (KEK, Domänenschlüssel-Cache) im `mlock`-geschützten `KeyProvider` (`contextra-vault`) und übergebt dem Kern nur die kurzlebigen Arbeitskopien.
    

#### 6. Ephemeral-Spaces und `MixedDurability`-Grenze (`§6.4`)

`SpaceOptions::ephemeral()` ist ideal für das **CLM Scratchpad** (`contextra-agent`). Da `contextra-storage` das Mischen dauerhafter und flüchtiger Spaces in einem `Change` verbietet (`Forbidden{MixedDurability}`), muss die Agenten-Execution-Engine (`checkpoint → execute → commit → audit`) strikt zwischen dauerhaften State-Commits und flüchtigen Scratchpad-Writes trennen.

### Konkrete Empfehlung für euer Vorgehen

1. **Verbindliche Dokumenten-Hygiene ab sofort:** `SPEC.md` hat in Anhang/§22.3 bereits wichtige Fehler aus `REFACTORING_PLAN.md` v2 korrigiert – insbesondere **Erratum E-4** (Gegendruck wartet _vor_ dem Ticket statt unter dem Committer-Lock, um einen Deadlock mit der Flush-Barriere zu verhindern), **Erratum E-5** (`redo_hwm := durable_stamp`) und **Erratum E-6** (`0x05` statt `0x04` für Domänen-Körper in Blobs). Verschiebt `REFACTORING_PLAN.md` v2 ins Archiv, damit niemand versehentlich gegen den überholten Stand implementiert.
    
2. **Gate `G-F` (Format-Freeze) vor den oberen Crates abschließen:** Vervollständigt zunächst die offenen Goldens aus Anhang A.8 (`Authenticated`/`Encrypted` für alle Dateitypen, Run, Blob, Catalog) und prüft anhand der vier Kern-Workflows aus `contextra-db` (`remember`, `recall` mit Filter + Zoom-Knapsack, `erase_subject`, `agent_step`), ob in `FileMeta` oder `SpaceOptions` noch ein Metadatum fehlt.
    
3. **Empfohlene Aufbaureihenfolge nach `contextra-storage`:**
    
    - **Stufe 1 (Ports & Hardware):** `contextra-ports` → `contextra-storage` (inkl. `FaultFs`, `loom`, Python-Goldens) → `contextra-simd` & `contextra-crypto` / `contextra-vault`.
        
    - **Stufe 2 (Retrieval-Indizes):** `contextra-text` (Extension/Segment), `contextra-vector` (HNSW/DiskANN/Acorn als `Segment` + `Consumer` + Tombstone-Tracking) und `contextra-graph` (CSR + Delta-Space).
        
    - **Stufe 3 (Logik & Orchestrierung):** `contextra-db` & `contextra-rank` (SSI/Transaktionen via `Guard`, Overlay-Branches Stufe A, Zoom-Knapsack über `estimate` + `scan_page` + `bodies`) → darauf aufbauend `contextra-agent`, `contextra-cognition` (LeanRAG/Decay via `CompactionFilter`), `contextra-adapt` und `contextra-privacy`.