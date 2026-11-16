## 7. Entscheidungsprotokoll, Szenarien, Akteure, Validator

### 7.1 Entscheidungsprotokoll [E]

Zu jeder Antwort, jedem Szenariolauf, jeder Prüfung und jedem Werkzeuglauf entsteht ein Eintrag der Klasse `Beleg`. Er liegt im **Entscheidungsspeicher** (K-Partition, `SegmentSealed`), **nicht** im Commit-DAG; das K-Ledger erhält nur `DecisionAppended { id, record_hash }`.

```rust
pub struct DecisionRecord {
    pub id: [u8; 16],                // zufällig
    pub kind: DecisionKind,          // Query | ScenarioRun | Review | Release | Tool
    pub commit: ObjectId,            // Wissensstand
    pub valid_at: Option<Date>,
    pub index_gen: IndexGen,
    pub prompt: ObjectId,            // Prompt-Version (Blob in K)
    pub retrieval_cfg: ObjectId,
    pub chunk_cfg: ObjectId,         // macht Chunk-Nummern eindeutig
    pub models: ModelSet,            // ModelFingerprint je Rolle: Embedder, Reranker, Generator, Verifier (inkl. Parameter, Seed)
    pub view_class: [u8; 16],        // Hash der Sichtklasse (Replay-Determinismus)
    pub input: PayloadRef,           // die Frage
    pub query_rewrite: Option<PayloadRef>,
    pub evidence: Vec<EvidenceRef>,
    pub answer: PayloadRef,
    pub abstained: bool,
    pub reviewer: Option<ActorRef>,  // Pflicht bei kind = Review
    pub actor: ActorRef,             // Handle, löschbar
    pub scenario: Option<[u8; 16]>,
    pub linked: Option<[u8; 16]>,    // Review → zugehörige Query
    pub retention: RetentionRef,     // decision_retention, payload_retention
    pub prev_hash: [u8; 32],         // K-Ledger-Verkettung
}
pub enum EvidenceRef { Knowledge { blob: ObjectId, chunk: u32 }, Personal(PersonalRef) }
pub enum PayloadRef  { Open(ObjectId), Sealed(PersonalRef) }
```

- **Standard ist `Sealed`.** Fragen enthalten oft Personenbezug (Akten, Namen), den ein Mustervalidator übersehen kann. `Open` ist nur zulässig, wenn die Richtlinie `open_decisions` des Mandanten aktiv ist, **alle** Belege `Knowledge` sind und Eingabe wie Antwort den N-Validator bestehen.
- **Betroffene einer `Sealed`-Nutzlast** (bestimmen ihren DEK, 4.2) sind: Betroffene der Belege ∪ Betroffene des Vorgangs (API-Parameter `subject_refs`/`case_id`) ∪ in der Eingabe erkannte und über das Verzeichnis auflösbare Personen. Wird Personenbezug erkannt, der sich nicht auflösen lässt, gilt `payload_retention` als harte Obergrenze; `payload_retention` ist je Entscheidungsart **Pflichtparameter ohne Standardwert**, unabhängig von `decision_retention`. Nach Ablauf wird der Schlüssel der Nutzlast vernichtet; die Begründungskette (Stand, Prompt, Modell, Belege) bleibt.
- **Der Akteur ist nicht Teil der Schlüsselableitung:** Das Ausscheiden einer Mitarbeiterin löscht ihre Identität (`Actor`, 8.1), nicht die Begründungskette fremder Vorgänge (Art. 17 Abs. 3 lit. b, e DSGVO).
- Nach einer Löschung bleibt die Begründungskette **vollständig sichtbar**: Wissensstand, Prompt, Modell, Wissensbelege. Eingaben und Antworten mit Personenbezug lösen zu `Erased` auf.
- Bei `kind = Review` ist `reviewer` Pflicht: Ohne menschliche Freigabe gilt eine Antwort nicht als Entscheidungsgrundlage (Art. 22 DSGVO; Art. 14 KI-VO bei Hochrisiko-Einsatz, 20.6) [R][V]. `Release` protokolliert die Freigabe eines Entwurfs oder Merge nach `main`. `Tool` protokolliert deterministische Werkzeuge (10): Werkzeugkennung, Version, Eingabe (`Sealed` bei Personenbezug), Ergebnis.
- **Ereignisprotokoll, kein Klartextprotokoll aller Ein- und Ausgaben:** Der Datensatz ist auf die genannten Felder begrenzt.

### 7.2 Nachvollziehen einer alten Entscheidung [E]

`explain(decision_id)` liefert: Stand der Wissensebene (Commit, `valid_at`), Prompt, Modell und Parameter, jeden Beleg (soweit nicht gelöscht und nicht durch Zugriffsrechte verborgen), die gespeicherte Antwort. Optional `replay(decision_id)`: erneuter Lauf mit denselben Parametern, derselben `IndexGen` und derselben `view_class`. Ist die `IndexGen` außer Dienst (11.4), antwortet `replay` mit `GenerationRetired`, `explain` bleibt vollständig. **Grenze (A-INFER):** Byte-identische Wiederholung ist nur möglich, wenn das Inferenzsystem deterministische Dekodierung unterstützt; die **gespeicherte Antwort** ist der Beleg, der Wiederholungslauf ein Vergleich. Der Bericht weist aus, ob der Wiederholungslauf identisch war. Der Zugriff auf `explain` unterliegt den Rechten des fragenden Nutzers zum Zeitpunkt der Abfrage (nicht denen des damaligen Nutzers).

### 7.3 Szenarien („Wenn-Dann“) [E]

Ein **Szenario** besteht aus: Basis-Commit, Szenario-Branch mit Änderungen (geänderte Vorschrift, anderer Schwellenwert, anderer Prompt, anderes Modell, andere Abrufkonfiguration), einem **Fragenset** (Blob in K, N-Validator-geprüft, ohne Personenbezug) und den Ergebnissen. Ablauf: (1) Branch aus Basis-Commit, (2) Änderungen als Commits, (3) Fragenset gegen Basis und Szenario laufen lassen (`ScenarioRun`), (4) Vergleich: Unterschiede in Belegen, Antworten, Enthaltungen, Metriken, (5) bei Bedarf `merge` oder `retire`. Z9 gilt: Ein Szenario schreibt nie in `main` und nie in Ebene P, verändert keine Zugriffsrechte. Personendaten dürfen in einem Szenario nur gelesen werden, wenn Zugriffsrecht, Vorgangsbindung und Gate es erlauben; Ergebnisse mit Personenbezug werden `Sealed`. `ScenarioRun`-Protokolle folgen der Aufbewahrung des Szenario-Branches. Das Szenario nutzt Index-Generationen des Basis-Standes; ändert es Chunking oder Embedder, baut es einen **temporären** Index (`szenario/<id>`), der mit `retire` entfällt.

### 7.4 Akteure und Mitarbeitende [E]

Nutzer sind selbst Betroffene (`subject_kind = Staff`). Anmeldung: `oidc:<iss>|<sub>` → `SubjectTag` → `SubjectKey` → Verzeichnis → aktuelles `ActorRef`-Handle der Epoche (`actor_handle_epoch`, Standard 12 Monate; neue Commits und Protokolle erhalten das neue Handle, die Zuordnung liegt in P). Eine Löschung oder Auskunft über Mitarbeitende folgt denselben Mechanismen wie jede Betroffenenanfrage; die Begründungskette fremder Vorgänge bleibt. **[A]** Personalvertretung und interne Regeln zur Auswertung von Nutzungsdaten (A-Ledger) liegen beim Betreiber.

### 7.5 N-Validator (Schutz der Ebene K) [E]

Vor jedem Schreiben in K prüft der Validator: (1) **Schema-Whitelist** für strukturierte Felder, (2) **Mustererkennung** (Namen, Anschriften, Kennnummern, E-Mail, Telefon, Kontonummern, Aktenzeichen mit Personenbezug) über `acta-pii`, (3) **Entropie- und Handle-Prüfung** für Kennungen, (4) bei Aggregaten **k-Anonymität** mit `K_MIN` ≥ 5 (vom Mandanten erhöhbar). Ein Treffer über der Schwelle blockiert das Schreiben (Fail-Closed); der Fehler nennt nur Befundcodes und Zeichenbereiche, nie Inhalte. **Typzwang:** `acta-history` nimmt nur `Validated<Tree>` entgegen; `Validated` ist ausschließlich von `acta-pii` konstruierbar. **Falschpositive:** Ein Befund DARF nur durch einen dokumentierten `Waiver` übergangen werden (zwei verschiedene Personen, davon `DataProtection`; Erklärung „kein Personenbezug“; Eintrag `Waiver` im K-Ledger); echte Treffer können nicht freigegeben werden und gehören in `Whole`. **Grenze [A] A-DETECT:** Automatische Erkennung ist nicht vollständig; deshalb Standardmodus `Whole`, Vier-Augen-Freigabe für `Open` und `Split` und Takedown (6.7).
---

