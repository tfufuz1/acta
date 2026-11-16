## 9. Ingest, Abruf und Antwort

### 9.1 Ingest [E]

|Stufe|Inhalt|
|---|---|
|1 Annahme|API prüft Pflichtfelder: Mandant, Sammlung, **`source_of_record`** (Quellsystem plus Referenz; ohne diese Angabe verweigert die API den Ingest), Datenklassendeklaration (R-CLS), bei `Personal`/`Besonders` die Betroffenenzuordnung (`subject_refs`, 2.3), Aufbewahrungsklasse, Sichtbarkeitsmetadaten, `ingest_id` (Zufallswert des Aufrufers für Idempotenz). Fehlt etwas: `ClassViolation` oder `Invalid`, nichts wird geschrieben|
|2 Parsing|Nur im Prozess `acta-parse-worker`: keine Schlüssel, kein Netz, kein Dateisystem (13.2). Protokoll: längenpräfixierte Rahmen (u32 big-endian, höchstens 64 MiB). Ein Worker verarbeitet genau ein Dokument und endet|
|3 Normalisierung|Unicode NFC, Entfernen von Steuerzeichen, Spracherkennung (DE, EN, sonst `und`)|
|4 Erkennung|`acta-pii` liefert Hinweise und Klassenvorschlag; Standardmodus `Whole` (2.3); bei Personenbezug ohne Zuordnung: `Unklassifiziert`|
|5 Zuordnung|`subject_refs` → `SubjectTag` → Tresor; unbekannte Betroffene werden angelegt (Schlüssel zuerst, WP-1)|
|6 Chunking|deterministisch aus Inhalt und versionierter `chunk_cfg` (9.1.1); Chunk-Nummern sind stabil|
|7 Embedding|Port `Embedder` (Art `Passage`); `ModelFingerprint` wird mit der Ableitung gespeichert; Vektoren `i8`-quantisiert (9.3)|
|8 Schreiben|Reihenfolge: Schlüssel durabel (WP-1) → P-Umschläge → DP-/DK-Segmente mit `Derived<T>` → **zuletzt** der K-Commit bzw. das Manifest, das die Sichtbarkeit herstellt (WP-3)|

**Ressourcenlimits (Startwerte, Anhang A):** Eingabe 512 MiB, CPU-Zeit 60 s, Speicher 2 GiB, Expansionsfaktor 100:1, Verschachtelungstiefe 8, 10 000 Seiten. Verletzung ergibt `Invalid(ResourceLimit)`; der Worker-Absturz bleibt auf den Aufruf beschränkt (T6).

**Atomarität und Idempotenz.** Der Ingest ist ein Journal-Batch. Ein Absturz vor dem Sichtbarkeitsschritt hinterlässt nur unsichtbare Segmente, die der Start verwirft (verwaiste Dateien werden bereinigt, die zugehörigen Schlüssel vernichtet). Wiederholung mit gleicher `ingest_id` liefert dasselbe Ergebnis (Idempotenztabelle 5.4); in K ist der Ingest zusätzlich durch `ObjectId` idempotent (6.1).

**Aktualisierung.** Wissensdokumente (K): neuer Commit mit neuem Blob, alter Blob bleibt versioniert. P-Dokumente: neue Version (Berichtigung, 8.2), Ableitungen der alten Version werden bereinigt (5.2 Nr. 6). **Entfernung eines Quelldokuments** durch das Quellsystem: `document.retire` setzt K-Blobs im neuen Commit außer Kraft (bleibt abfragbar per Zeitreise) bzw. startet die Saga für P-Versionen mit Grund `SourceRemoved`.

#### 9.1.1 Chunking [E]

Ein Chunk ist eine zusammenhängende, strukturbewusste Textspanne. Einheit sind Wörter nach UAX #29. `chunk_cfg` (versionierter Blob) enthält und legt für Version 1 fest:

|Parameter|Wert|
|---|---|
|Zielgröße / Maximum / Minimum|300 / 450 / 60 Wörter|
|Überlappung (nur bei unstrukturiertem Text)|40 Wörter|
|Grenzen (Priorität)|Überschrift > Paragraph/§/Absatz einer Norm > Listenelement-Gruppe > Satzgrenze > Wortgrenze|
|Tabellen|zeilenweise Gruppen bis Maximum, Kopfzeile wird in jede Gruppe wiederholt|
|`heading_path`|Pfad der Überschriften (höchstens 6 Ebenen) als eigenes Feld; wird dem Chunk für Embedding und BM25 **vorangestellt**, aber nicht im Belegtext doppelt angezeigt|
|`norm_ref`|erkannte Normenkennung (z. B. „§ 35 Abs. 2“), als Feld für Zitate|
|`section_id`|Kennung des Abschnitts für Nachbar-Erweiterung (9.4)|

Chunking ist eine reine Funktion `(Bytes, chunk_cfg) → Vec<Chunk>`; gleiche Eingabe liefert gleiche Chunks (Golden-Tests). Die Chunk-Kennung ist `(ObjectId bzw. VerRef, chunk u32)`. Eine Änderung von `chunk_cfg` erzeugt eine neue `IndexGen` (11.4).

### 9.2 Textsuche [E] (Z7, Z16)

- **Normalisierung:** Unicode-Casefolding, Wortgrenzen nach UAX #29, Stoppwörter und leichter Stemmer für DE und EN; die Variante samt **Hash der Unicode-Tabellen und Stemmer-Version** ist Teil der `retrieval_cfg` (versioniert); ein Bibliotheks-Update, das die Tabellen ändert, erzeugt eine neue `IndexGen`.
- **Ranking:** BM25 mit `k1 = 1,2`, `b = 0,75` in **Festkomma Q32.32** (Rundung half-even). `idf(t) = ln(1 + (N − df + 0,5)/(df + 0,5))` über `fx_ln`: Zerlegung `x = m·2^e` mit `m ∈ [1,2)`, `ln x = e·LN2 + 2·atanh((m−1)/(m+1))` als Ganzzahlreihe mit 12 Termen; Golden-Vektoren sind Teil der Tests (`T-TT-FX`). Kein `f32`/`f64` in Ranking-Pfaden.
- **Statistik:** `N = |V ∩ A|`, `df(t) = |Postings(t) ∩ V ∩ A|`, `avgdl = Σ dl / N` mit `V` = Sichtbarkeitsmenge des Standes (6.5) und `A` = **effektive Zugriffsmenge der Sichtklasse** des Nutzers (9.4). Die Längensumme je (Commit, Sichtklasse) liegt in einem DK-Cache und wird bei Änderung der Zellen-Bitmaps der Sichtklasse neu berechnet. Damit hängt keine Rangfolge von Dokumenten ab, auf die der Nutzer keinen Zugriff hat (Z16).
- **Gleichstand:** (Score absteigend, `KOrd` aufsteigend, Chunk aufsteigend).
- **Ebene P:** eigene DP-Indizes; Statistik: `idf` aus der K-Statistik der Sichtklasse (Terme ohne K-Eintrag: `df = 0`), `tf` und Länge aus dem Chunk. Die Fusion mit K erfolgt über Ränge (9.4), nie über Scores.

### 9.3 Vektorsuche [E]

- **Quantisierung (Ingest und Anfrage):** Der Embedder liefert `f32`. Je Vektor wird mit `s = 127 / max|xᵢ|` skaliert und `qᵢ = round_half_even(xᵢ·s)` als `i8` gespeichert; zusätzlich `norm2 = Σ qᵢ²` als `u32`. Gespeichert werden die Bytes; Z7 gilt ab diesen Bytes.
- **Ähnlichkeit:** `dot = Σ qᵢ·dᵢ` (`i32`), `sim = (dot << 32) / isqrt(norm2_q · norm2_d)` als Q32.32 (`isqrt` ganzzahlig). Exakte Flat-Suche je Segment, Segmente parallel, Zusammenführung in fester Reihenfolge; Top-k mit festem Gleichstand wie 9.2.
- **Kandidatenmenge** vor dem Scan: K: `V ∩ A`; P: `A_P ∩ Live` (nicht im Gate, 9.4). **[Q]** 1 Mio. Chunks × 768 Dimensionen, p95 unter 150 ms je Anfrage auf 8 Kernen; Recall@10 der Quantisierung ≥ 0,99 gegenüber `f32`.
- Ein ANN-Backend ist erst nach Messbericht zulässig, **nur** als Kandidatenerzeuger mit exaktem Nachrang, und wird als nicht deterministisch ausgewiesen (Z7 gilt dann nicht).

### 9.4 Zugriffskontrolle und Abrufpipeline [E] (Z16)

**Zugriffsmodell (ABAC).** Die Richtlinie (`policy`-Branch, 6.1) besteht aus geordneten Regeln `allow`/`deny` (deny überschreibt) über Nutzerattribute (Rolle, Organisationseinheit, Freigabestufe, Gruppen) und Ressourcenattribute (Sammlung, `confidentiality`, `owner_unit`, `acl_labels`). **Zugriffszellen** sind `(collection, confidentiality, owner_unit)` und `label:<name>`; je Zelle hält `acta-access` eine Roaring-Bitmap der `KOrd`, gepflegt bei Commit und Ingest. Die **Sichtklasse** eines Nutzers ist die sortierte Menge seiner erlaubten Zellen (Hash als `view_class`); `A_K` ist die Vereinigung der Zell-Bitmaps. Nutzer mit gleicher Zellenmenge teilen Statistiken und Caches. **Für P** gilt `A_P` = Vereinigung der Fallindex-Bitmaps (5.4) der dem Nutzer zugewiesenen, offenen Vorgänge, eingeschränkt auf die in der Anfrage genannten `case_id`s (höchstens `max_cases_per_query` = 5). Ohne `case_id` werden **keine** P-Inhalte abgerufen (Zweckbindung, Datenminimierung); ein freier Korpus über P existiert nicht. Die Richtlinie wirkt immer im aktuellen Stand.

**Reihenfolge:** Authentifizierung → Mandant → Sichtklasse → Stand (`as_of`, `valid_at`) → Kandidatenmengen (K: `V ∩ A_K`; P: `A_P`, `Live`, nicht im Gate) → **Anfrageaufbereitung** → BM25, Vektorsuche und Graph-Traversierung (PPR) parallel → **Reciprocal Rank Fusion** → optional Reranker → Kontextaufbau → Prompt → Generierung (9.5) → Belegprüfung (9.6).

**Parameter (in `retrieval_cfg`, Startwerte):**

|Parameter|Wert|
|---|---|
|`k_text`, `k_vec`, `k_graph` (K)|100, 100, 50|
|`k_text_p`, `k_vec_p`, `k_graph_p` (P)|50, 50, 25|
|`rrf_k`|60 (Ganzzahl: `floor(2³² / (rrf_k + rang))`, Summe über Listen)|
|`fuse_keep`|40|
|`rerank_keep`|12|
|`max_chunks_per_doc`|3|
|`neighbor_expand`|1 (benachbarte Chunks desselben `section_id`)|
|`overlap_min` (lexikalischer Stützungstest)|0,30|

**Anfrageaufbereitung.** (1) Spracherkennung; (2) bei Folgefragen in einer Sitzung **Umformulierung** zu einer eigenständigen Frage durch den `Generator` (Ergebnis wird als `query_rewrite` im Protokoll abgelegt, `Sealed`); (3) Anfragevektor über den `Embedder` (Art `Query`) und Quantisierung; (4) Erkennung von Start-Entitäten für den Graphindex (`acta-graph`). Die Umformulierung ist modellabhängig (A-INFER); Z7 gilt ab dem Anfragevektor und den Start-Entitäten.

**Graphindex und Traversierung (`acta-graph`).** Graph liefert zusätzliche Kandidatenlisten (Graph-K und Graph-P getrennt) über CSR-Traversierung, Personalized PageRank (PPR) und Pfad-RAG. Gemäß Z7 erfolgen PPR, Kantengewichte und Pfadbewertungen deterministisch in Festkomma Q32.32 mit ganzzahligem Gleichstand (Ordinal aufsteigend). Traversierung erfolgt strikt innerhalb der zulässigen Kandidatenmenge (`K: V ∩ A_K`; `P: A_P ∩ Live`).

**Fusion und Reranking.** Sechs Listen (BM25-K, Vektor-K, Graph-K, BM25-P, Vektor-P, Graph-P) werden über RRF zu einer Liste vereinigt (`fuse_keep`). Der Reranker (Port, A-INFER) bewertet die Kandidaten; seine Gleitkommawerte werden an der Portgrenze mit half-even auf Q32.32 gerundet; Gleichstand: (Reranker-Score absteigend, RRF-Rang aufsteigend, `KOrd`/`VerOrd` aufsteigend, Chunk aufsteigend). Ohne Reranker gilt der RRF-Rang. Danach: höchstens `max_chunks_per_doc` je Dokument, Top `rerank_keep`.

**Kontextaufbau.** Das Kontextbudget ist `ctx_len(Generator) − Systemprompt − Frage − Verlauf − answer_reserve` (Tokenzahlen über `Generator::count_tokens`). Belege werden in Rangfolge aufgenommen, jeweils mit Nachbar-Chunks, bis das Budget erschöpft ist; überlappende Spannen werden zusammengeführt; jeder Beleg erhält eine anfragelokale Kennung `E1…En` und einen Kopf (Titel, `heading_path`, `norm_ref`, Gültigkeitszeitraum `valid_from/valid_to`, Herkunft `K|P`, Stand: Commit-Kurzform bzw. Dokumentdatum). Blöcke sind durch ein **zufälliges Anfrage-Token** begrenzt (`<<E3:7f3a…>>`), das nicht im Beleginhalt vorkommen darf (bei Treffer neu würfeln); Markup und Steuerzeichen werden aus Belegen entfernt.

**Prompt-Injection (T7).** Belege sind **Daten**: Der Systemprompt weist das Modell an, Anweisungen in Belegen zu ignorieren; das Modell hat keine Werkzeuge, kein Netz und kein Gedächtnis außerhalb der Anfrage. Die Antwort ist ein strukturiertes Objekt (9.6), das als Text mit Belegverweisen ausgegeben wird, **ohne aktive Inhalte** (keine Links, Bilder, Markup, keine Beleg-URLs aus dem Modell), damit kein Datenabfluss über Darstellung möglich ist.

**Last und Seitenkanäle.** Die Arbeit ist proportional zur **zugänglichen** Menge; Trefferzahlen, Fehlerbilder, Antwortzeiten-Klassen und Quellennamen unterscheiden zugängliche und unzugängliche Dokumente nicht. Ratenbegrenzung je Nutzer (`rate_limit_per_min`) verhindert Abtasten.

### 9.5 Generierung, Gate-Prüfung, `ContextPurge` [E] (Z1)

- **Epochenprüfung.** Eine Anfrage merkt die Gate-Epoche. Vor jedem Ausgaberahmen und vor Abschluss wird die Epoche verglichen; ändert sie sich, werden die Beleg-Ordinale erneut geprüft. Ist ein Beleg gesperrt, bricht die Anfrage ab, der Puffer wird verworfen, der Aufrufer erhält `Erased` ohne Inhalt. Bis zur Sperre bereits ausgelieferte Rahmen liegen außerhalb der Zusage (A-EXT); Grenze `gate_propagation_max` = 1 Rahmen.
- **`ContextPurge`.** Der Port `Generator` bietet `purge(request_id)`. Persistente Prompt- oder Präfix-Caches des Inferenzdienstes sind für Anfragen mit Personenbezug **abgeschaltet** (Pflichtkonfiguration, T10); KV-Caches sind anfragegebunden und werden mit der Anfrage freigegeben.
- **Antwort-Cache.** Antworten mit P-Belegen werden nicht gecacht (`cache_personal = false`). Antworten aus K sind je (Stand, `view_class`, `retrieval_cfg`, normalisierte Frage) cachebar (`answer_cache_ttl` = 24 h), **nur** wenn die Frage den N-Validator besteht; der Cache ist DK-Lineage-gebunden und verwirft bei Takedown.
- **Sitzungen.** Gesprächskontext liegt nur im RAM (`session_ttl` = 8 h). Jeder Gesprächsschritt trägt eine Lineage (`source_set` der Belege); ein Gate-Eintrag verwirft betroffene Schritte (und alle Folgeschritte, die darauf aufbauen). Sitzungen überleben keinen Neustart.
- **Export und Vorschau** nutzen dieselbe Lesepfadregel (8.1).

### 9.6 Belegpflicht und Stützungsprüfung [E] (Z17)

**Antwortobjekt:** `{ abstain: bool, statements: [{ text, evidence: [E-Kennungen] }], notes: [...] }`. Aussagen mit leerer `evidence` oder mit Kennungen außerhalb der Abrufmenge werden entfernt; Kennungen außerhalb der Abrufmenge sind zusätzlich ein harter Fehler (`GroundingViolation`, Alarm). Bleibt keine Aussage, entsteht die Enthaltung „nicht belegt“ (`abstain = true`). Wird die Generierung nicht strukturkonform geliefert, folgt höchstens ein Wiederholungsversuch mit strengerem Schema, danach Enthaltung.

**Stützung (S).** Der Port `Verifier` bewertet je Aussage, ob die zitierten Belege sie stützen: lexikalischer Überlapptest (Anteil der Inhaltswörter der Aussage in den Belegen ≥ `overlap_min`) **und** Modellurteil; Aussagen unter `support_min` werden entfernt (`support_min` ist Pflichtparameter aus dem Messbericht). Gemessen werden auf einem Goldset (mindestens 300 Fragen, DE und EN, davon 20 % unbeantwortbar): **Zitiergenauigkeit**, **Zitiervollständigkeit**, **Enthaltungsgenauigkeit**, **Halluzinationsrate**. **[Q]** Zitiergenauigkeit ≥ 0,95, Enthaltungsgenauigkeit ≥ 0,90. Die Stützungsprüfung ist ein statistisches Verfahren und wird so ausgewiesen.

**Ausgabe:** Text mit Belegmarken `[1]`, `[2]` …, dazu die Belegliste (Titel, Fundstelle `norm_ref`/Abschnitt, Gültigkeit, Stand, Herkunft K/P). Antworten tragen den Hinweis „Entscheidungshilfe, keine Entscheidung“ (Art. 22).

### 9.7 Bewertungswerkzeug [E]

`acta-cli eval` führt Goldsets (Fragen, erwartete Belege, erwartete Enthaltung; Blob in K, N-Validator-geprüft, ohne Personenbezug) gegen einen Wissensstand und vergleicht zwei Stände. Metriken: Recall@k, MRR, nDCG, Zitiergenauigkeit, Zitiervollständigkeit, Enthaltungsgenauigkeit, Latenzverteilung (p50, p95, p99). Ergebnisse sind `Beleg`-Einträge. **Regressionsgrenzen** (Anhang A: Recall@10 −0,02, Zitiergenauigkeit −0,02, Enthaltungsgenauigkeit −0,03, Latenz p95 +20 %) sind Freigabekriterium für `merge` nach `main` und für Modell-/Konfigurationswechsel, wenn der Mandant es aktiviert (Standard: aktiviert, `merge_main_eval_gate`).

---

