## 11. Inferenz und Modell-Lebenszyklus

### 11.1 Ports [E]

```rust
pub struct ModelFingerprint {
    pub role: ModelRole,              // Embedder | Reranker | Generator | Verifier
    pub provider: String, pub name: String,
    pub digest: [u8; 32],             // SHA-256 der Gewichtsdatei bzw. vom Dienst gemeldeter Digest
    pub tokenizer_digest: [u8; 32], pub params_hash: [u8; 32],
    pub ctx_len: u32, pub dim: Option<u16>,
}
pub trait Embedder: Send + Sync {
    fn fingerprint(&self) -> &ModelFingerprint;
    fn embed(&self, texts: &[&str], kind: EmbedKind) -> Result<Vec<Vec<f32>>, InferError>; // Query | Passage
}
pub trait Reranker: Send + Sync {
    fn fingerprint(&self) -> &ModelFingerprint;
    fn rerank(&self, query: &str, cands: &[&str]) -> Result<Vec<f32>, InferError>;
}
pub trait Generator: Send + Sync {
    fn fingerprint(&self) -> &ModelFingerprint;
    fn count_tokens(&self, text: &str) -> Result<u32, InferError>;
    fn generate(&self, req: &GenRequest, sink: &mut dyn FnMut(Frame) -> Control) -> Result<GenResult, InferError>;
    fn purge(&self, request: &RequestId) -> Result<PurgeAck, InferError>;   // ContextPurge
}
pub trait Verifier: Send + Sync {
    fn fingerprint(&self) -> &ModelFingerprint;
    fn support(&self, statement: &str, evidence: &[&str]) -> Result<f32, InferError>; // 0..1, an der Grenze nach Q32.32 gerundet
}
pub struct GenRequest { pub id: RequestId, pub system: String, pub messages: Vec<Message>,
    pub params: GenParams /* temperature, top_p, seed, max_tokens */, pub schema: Option<JsonSchema>, pub stop: Vec<String> }
```

`acta-infer-http` implementiert die Ports gegen eine OpenAI-kompatible Schnittstelle (Chat-Completions mit `response_format`/Schema, Embeddings, optional Rerank). Zeitüberschreitungen (`infer_timeout`: Embedding 10 s, Rerank 10 s, Generierung 120 s) und Wiederholung (höchstens 1) sind Konfiguration; Ausfall führt zu `Degraded(Inference)` (14.6).

### 11.2 Anforderungen an den Inferenzdienst [E][A]

Separater Prozess auf Loopback oder Unix-Socket; Pflichtkonfiguration: **keine Protokollierung von Inhalten**, **kein persistenter Prompt-/Präfix-Cache** für Anfragen mit Personenbezug, KV-Cache anfragegebunden, Abbruch laufender Anfragen auf `purge`, kein Netzzugang des Dienstes, Gewichte schreibgeschützt. Der Dienst gehört zum Vertrauensbereich (A-INFER). Ein Dienst, der das nicht zusichert, gilt als externer Dienst und braucht einen `EgressGrant` (13.3). Der Betreiber bestätigt die Zusicherungen im Löschkonzept (12.4).

### 11.3 Referenzprofile (Orientierung, keine Zusage) [Q]

|Profil|Hardware|Generator|Embedder und Reranker|Last|
|---|---|---|---|---|
|S|CPU, 64 GB RAM|7 bis 8 Mrd. Parameter, 4 Bit|mehrsprachiges Embedding (384 bis 768 Dimensionen), kleiner Reranker|bis ca. 10 Nutzer|
|M|1 GPU mit 24 GB|14 bis 32 Mrd. Parameter, 4 Bit|mehrsprachiges Embedding (768 bis 1024), Cross-Encoder|bis ca. 50 Nutzer|
|L|2 GPU mit 80 GB|70 Mrd. Parameter|wie M, höherer Durchsatz|ab ca. 200 Nutzer|

Die Modellauswahl ist Betreiberentscheidung; jedes Modell durchläuft vor Aktivierung den Lebenszyklus 11.4 (Goldset-Messung mit Freigabegrenzen). Ausgeliefert werden keine Modelle.

### 11.4 Modell- und Index-Lebenszyklus [E]

|Phase|Ablauf|
|---|---|
|1 Registrieren|Modell im Inferenzdienst bereitstellen; `ModelFingerprint` erfassen; Eintrag in `config/models` auf Branch `entwurf/<id>`|
|2 Bewerten|Goldset-Lauf (9.7) gegen den Entwurfsstand; bei Embedder-, Chunking- oder Normalisierungswechsel Aufbau einer neuen **`IndexGen`** im Hintergrund (DK; für DP als Reindex-Aufgabe unter dem Systemzweck `Reindex`, aggregiert protokolliert)|
|3 Aktivieren|`merge` nach `main` (Vier-Augen, Regressionsprüfung); `main` zeigt auf die neue `IndexGen` erst nach vollständigem Aufbau; bis dahin antworten Abfragen mit der alten Generation|
|4 Betreiben|Metriken je Modell (Latenz, Ausfallrate, Enthaltungsquote), Drift-Überwachung durch periodischen Goldset-Lauf (`goldset_interval` = 7 Tage); Überschreiten einer Regressionsgrenze löst `QualityDrift` aus|
|5 Auslaufen|Vorgängergeneration bleibt für `index_gen_retention` (= `branch_retention`) erhalten, damit Zeitreisen und `replay` aktueller Entscheidungen möglich sind|
|6 Außerdienststellen|Danach wird die `IndexGen` entfernt (Segmente, `SegmentKey`s vernichtet). Abfragen mit `as_of` auf Stände, deren `IndexGen` außer Dienst ist, antworten mit der aktuellen Generation und der Kennzeichnung `regenerated` oder, auf Wunsch, mit `GenerationRetired`; `rebuild_generation` baut sie aus den Blobs mit dem damaligen Modell neu auf, sofern verfügbar (nicht deterministisch, A-INFER)|

DP-Indizes werden bei Modellwechsel vollständig neu aufgebaut; alte DP-Generationen werden sofort nach Umschaltung entfernt (Datenminimierung, keine Aufbewahrung alter personenbezogener Ableitungen).

### 11.5 Quoten, Budget, Fehlerbilder [E]

Je Mandant: `inference_budget` (Tokens je Stunde), `max_parallel_generations`, Warteschlange `infer_queue_max`; Überschreitung: `Transient(Overloaded)` mit `Retry-After`. Verhalten bei Ausfall: Suche läuft weiter (Belegliste ohne Antwort, `Unavailable`), Ingest staut Embedding-Aufträge in einer begrenzten Warteschlange (`embed_queue_max`) und liefert danach `Transient`; kein Teilzustand wird sichtbar (Embedding vor Manifest, WP-3).
---

