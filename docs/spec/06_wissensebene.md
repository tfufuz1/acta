## 6. Versionskontrolle der Wissensebene („Git für RAG“)

### 6.1 Objektmodell [E]

Alle Objekte sind unveränderlich und inhaltsadressiert: `ObjectId = SHA-256(Domänenpräfix ‖ kanonische Bytes)`.

|Objekt|Inhalt|
|---|---|
|**Blob**|Bytes eines Wissensdokuments oder einer Konfiguration. Kanonische Bytes: `acta/blob/v1 ‖ salt(16) ‖ Inhalt`, `salt = HMAC-SHA256(K_dedup(t), Inhalt)[..16]`, `K_dedup(t) = HKDF(k_idx, "acta/dedup/v1" ‖ TenantId)`. Keine Personendaten (Z5)|
|**Tree**|Sortierte Einträge `{name, kind, id, meta}`; `kind ∈ {Blob 0x01, Tree 0x02, PersonalRef 0x03}`. Bei `PersonalRef` ist `id` das 16-Byte-Handle, **nie** ein Inhaltshash. Präfix `acta/tree/v1`|
|**Commit**|`{tree, parents[], tx_seq, tx_time, valid_from?, valid_to?, author: ActorRef, kind, note_tag, flags}`; `kind ∈ {Ingest, Edit, Merge, Config, Takedown, Release}`; `flags`: `clamped`. Präfix `acta/commit/v1`|
|**Ref**|Benannter Zeiger auf einen Commit: Branch (`main`, `szenario/<id>`, `entwurf/<id>`, `policy`) oder Tag (unveränderlich)|

**Schlüssel-Salz statt reinem Inhaltshash.** Ein reiner Inhaltshash ist für Inhalte mit geringer Entropie wörterbuchprüfbar und bliebe nach einem Takedown als prüfbares Pseudonym zurück. Das Salz ist inhaltsabhängig, aber **geheim abgeleitet**: Gleicher Inhalt ergibt im Mandanten dieselbe `ObjectId` (Idempotenz), ohne `K_dedup` ist der Hash nicht prüfbar. Ein Takedown entfernt Inhalt **und** Salz. Mandantenübergreifende Gleichheit gibt es bewusst nicht.

`ActorRef` ist ein zufälliges Handle auf eine Identität in P. `note_tag` ist eine strukturierte Kennung (`u16`) aus einer Whitelist; freier Text wird nie in Commits gespeichert, sondern als validierter Blob abgelegt. Baumnamen sind NFC-normalisiert, enthalten weder NUL noch `/` und sind höchstens 255 Byte lang.

**Wissensobjekte im Tree (Pfadkonvention):**

|Pfad|Inhalt|
|---|---|
|`collections/<name>/docs/…`|Wissensdokumente (Blobs) mit Metadaten im `meta`-Feld (12.2)|
|`collections/<name>/collection.cfg`|Sammlungskonfiguration (12.2)|
|`config/chunk_cfg`, `config/retrieval_cfg`, `config/models`|Chunking-, Abruf- und Modellbeschreibung (`ModelFingerprint`)|
|`prompts/…`|Prompt-Vorlagen, Systemprompt, Verifier-Prompt|
|`templates/…`|Schreiben-Vorlagen und Textbausteine (10)|
|`checklists/…`, `calendar/…`|Prüflisten, Feiertags- und Fristenkalender (10)|
|`goldsets/…`|Fragensets (9.7)|
|`rules/…`|Schwellenwerte (`support_min`, `K_MIN`) und Regeln|

Die Zugriffsrichtlinie (`policy`) liegt auf eigenem Branch `policy` und wirkt **immer in ihrem aktuellen Stand**, nie zeitreisend: ein früherer Wissensstand erweitert nie die Zugriffsrechte (9.4). **Alles, was eine Entscheidung beeinflusst, ist versioniert.**

### 6.2 Protokolle und Integrität [E]

Je Mandant gibt es **drei** getrennte Ledger desselben Typs `HashChain`: **K** (Wissen), **E** (Nachweise) und **A** (Zugriff), `chain_id = SHA-256(TenantId ‖ "K" | "E" | "A")`.

|Ledger|Inhalt|Änderung durch Löschung in P|
|---|---|---|
|**K-Ledger**|Commit, Ref-Bewegung, Tag, `Redacted`, `Expired`, `Waiver`, `DecisionAppended { id, record_hash }`, `PolicyChanged { hash }`|**nie** (Z6)|
|**E-Ledger**|Löschvorgänge (Commitments, Zustandswechsel, Berichts-Hashes), Rotationen (`RootRotated`), Restore-Ereignisse (`Restored`), Re-Witness, `restricted_read`-Zugriffe (Zweckcode, ohne Inhalte), Konfigurationsänderungen von Aufbewahrung und Holds|ja, aber getrennt von K|
|**A-Ledger**|Zugriffsprotokoll: Zeit, `ActorRef`, Aktion (`read`, `query`, `export`, `review`, `admin`), Handle (`PersonalRef`, `CaseId` oder `ObjectId`), Zweckcode, Ergebnis|Einträge sind **versiegelt** unter `LogKey(m)` (Monat); Ablauf von `access_log_retention` vernichtet den Monatsschlüssel, die Kette bleibt über die Chiffrate prüfbar|

`entry_hash = SHA-256(prev_hash ‖ kanonischer Eintrag)`, Domäne `acta/ledger/v1` und `chain_id`; kanonischer Eintrag `[seq u64][kind u8][len u32][payload]`. Beim A-Ledger bildet der Hash die **versiegelte** Nutzlast. Alle `checkpoint_every` = 1 024 Einträge entsteht ein vom Anker signierter Checkpoint (`notary_sign`, `acta/checkpoint/v1`), sein Kopf wird gemeldet (`record_head`). Damit lässt sich die Geschichte ohne Inhaltszugriff prüfen und nicht unbemerkt umschreiben oder zurückrollen. Ledger enthalten keine Personendaten (Z15). Mandanten sehen nur ihre eigenen Ledger; Lücken in globalen Folgenummern gibt es nicht. Abgelaufene A-Ledger-Monate erhalten den Vermerk `Pruned { block, last_hash }`, die Chiffrate dürfen entfernt werden.

**Commitments (Lösch-Paradoxon).** Für jeden vernichteten Schlüssel enthält das E-Ledger `commit(e, KeyId) = HMAC-SHA256(CommitKey(e), "acta/cmt/v1" ‖ KeyId)[..16]`. `KeyId` ist `SubjectTag`, `VerRef` oder Segment-`file_id`, **niemals** eine externe Kennung. `CommitKey(e)` gilt je Epoche `commit_epoch` = 30 Tage, liegt im Mandantentresor und wird `commit_key_retention` nach Epochenende vernichtet (Standard 90 Tage); danach sind die Commitments anonym, die Kette bleibt verifizierbar. **[⚠]** Solange `CommitKey(e)` existiert, sind Commitments pseudonym und personenbezogen; sie brauchen Grundlage, Zweck und Frist (Art. 5 Abs. 2, Art. 17 Abs. 3 lit. e DSGVO; Verweis auf Art. 11 DSGVO in der Literatur). Das AG Lörrach (3 C 1099/24, erstinstanzlich) verneint einen Anspruch der betroffenen Person auf einen dauerhaften individuellen Löschnachweis; ein Gericht hat die Dauer pseudonymer Lösch-IDs nicht entschieden [V]. Die Frist ist Konfiguration des Verantwortlichen mit Untergrenze (R-HORIZON, 15.4).

**Registersenke [E].** Gate-Log und E-Ledger (einschließlich Checkpoints und Hochwassermarken) werden über `erase_registry_sink` (Pflichtparameter, **getrennter Ausfallbereich**: anderes Medium, anderes System) gespiegelt. Das Spiegeln ist asynchron und blockiert die Sperre nie. Der Bericht weist `registry_mirrored` aus.

```rust
pub trait RegistrySink: Send + Sync {
    fn append(&self, tenant: TenantId, rec: &SinkRecord) -> Result<(), SinkError>;
    fn last_acked(&self, tenant: TenantId) -> Result<SinkPosition, SinkError>;
    fn fetch_since(&self, tenant: TenantId, pos: SinkPosition) -> Result<Vec<SinkRecord>, SinkError>;
}
// SinkRecord = GateFrame | LedgerEntry | Checkpoint | HighWater   (keine Inhalte, keine Kennungen)
```

`DirSink` (im Kern): anderes Medium oder Mount, append-only, `fsync`; der Start prüft `st_dev` verschieden vom Datenmedium (R-SINK). Netzsenken liegen in `acta-sink-net`.

### 6.3 Zeitachsen [E]

|Achse|Bedeutung|Abfrageparameter|
|---|---|---|
|**Gültigkeitszeit** `valid`|Fachliche Geltung einer Vorschrift: `[valid_from, valid_to)`, Kalendertage; „kein Wert“ über Präsenzbit, nicht über `0`|`valid_at: Date`|
|**Transaktionszeit** `tx`|Wann Acta den Stand erfasst hat; Commit-Folge `tx_seq` (je Mandant, dicht) mit `tx_time` (nicht fallend)|`as_of: CommitId \| tx_seq \| Zeitpunkt`|
|**Löschzeit** `erase`|Wann P ein Ziel gesperrt hat; nur im E-Ledger und Gate-Log|–|

`tx_time = max(Clock::now(), tx_time(Vorgänger))`; ein geklemmter Wert setzt das Flag `clamped`. Ein Rücksprung über `clock_skew_max` ist `Poisoned(Node, ClockRegression)`. Ein Zeitpunkt für `as_of` wird auf den letzten Commit mit `tx_time ≤ Zeitpunkt` abgebildet (Gleichstand: höchste `tx_seq`). Beispiel: „Welche Fassung der Satzung galt am 1.3. (`valid_at`) und was wusste das System am 15.3. (`as_of`)?“ Die Antwort ist deterministisch (Z7).

### 6.4 Operationen [E]

|Operation|Bedeutung und Pflichten|
|---|---|
|`commit`|Neuer Stand aus Änderungen. Pflicht: `Validated<Tree>` aus dem N-Validator (7.5), Klassendeklaration. Schreibreihenfolge: Objekte → K-Ledger → Ref (WP-2)|
|`branch`, `tag`|Verzweigung für Szenarien, Entwürfe; unveränderliche Markierung|
|`checkout(as_of, valid_at)`|Lesender Stand (Sicht), keine Kopie|
|`diff(a, b)`|Unterschied zweier Stände auf Dokument-, Chunk- und Metadatenebene|
|`merge(src, dst)`|Drei-Wege-Zusammenführung; Konflikte pro Pfad; `PersonalRef`-Einträge werden nach Handle verglichen. Nach `main`: Vier-Augen-Freigabe (konfigurierbar `merge_main_four_eyes`, Standard an) und, wenn der Mandant es aktiviert, **Regressionsprüfung** (Goldset, 9.7)|
|`retire(branch)`|Branch freigeben; nicht mehr erreichbare Objekte werden nach `branch_retention` (Standard 90 Tage, anpinnbar) bereinigt|

Jeder Branch ist an eine **Index-Generation** gebunden (`retrieval_cfg`, `chunk_cfg`, Embedder); ändert ein Commit diese Konfiguration, entsteht beim `merge` nach `main` ein Indexaufbau im Hintergrund, und `main` zeigt erst nach dessen Abschluss auf die neue Generation (11.4).

### 6.5 Sichtbarkeitsmengen und versionsbewusste Suche [E]

Jeder Wissens-Blob erhält eine `KOrd`. Je Commit gibt es eine **Sichtbarkeitsmenge** `V(commit)` (Roaring-Treemap der `KOrd` im Tree), gespeichert als Basismenge plus Deltas zum Elterncommit (Schnappschuss alle 256 Commits). Die Suche arbeitet auf **globalen, unveränderlichen DK-Indexsegmenten** des Mandanten und filtert die Kandidaten mit `V ∩ A` (9.2). So kostet ein Branch oder Zeitpunkt keinen eigenen Index. Dokumente der Ebene P sind **nicht** Teil von Sichtbarkeitsmengen; für sie gelten Zugriff und Gate (9.4).

### 6.6 Zeitreise über Personendaten (Ebene P) [E]

Ebene P hat **keine** allgemeine Zeitreise. Sie führt je Dokument höchstens kurze `Superseded`-Versionen für den Nachweis der Berichtigung (Art. 16) und für Aufbewahrungsfristen, begrenzt durch `history_horizon` (`Personal`: 30 Tage; `Besonders`: 0). `Superseded`-Versionen sind für **alle** Lesepfade gesperrt (Z1) und nur über `restricted_read` lesbar; danach werden sie bereinigt. Superseded-Versionen aus **Löschvorgängen** (`RedactSplit`, `RestrictShared`) werden sofort bereinigt, nicht erst nach `history_horizon` (8.9). Abfragen mit `as_of` auf P prüfen **zuerst** das Gate; ein gelöschtes Ziel liefert `Erased`, nie Klartext. **Begründung:** Eine unbegrenzte Historie personenbezogener Inhalte widerspricht dem Zweck der Löschung (Art. 5 Abs. 1 lit. d, e); Nachvollziehbarkeit gelingt über die Begründungskette in K (7).

### 6.7 Garbage Collection, Fristlöschung und Takedown in K [E]

**Erreichbarkeit.** Objekte, die von keinem Ref und keinem aufbewahrten Entscheidungsprotokoll erreichbar sind, werden nach `branch_retention` entfernt. Ein Entscheidungsprotokoll hält die Objekte nur bis zum Ende seiner `decision_retention` (10.3 → 12.5); danach ist es selbst Löschkandidat. Pack-Segmente werden ohne unerreichbare Blobs neu geschrieben (neuer `SegmentKey`, alter wird vernichtet). Die Hash-Kette bleibt unberührt, weil sie nur Commits, Refs und Vermerke enthält.

**Fristlöschung (`Expired`).** Nach `decision_retention` entsteht ein Vermerk `Expired { id }` im K-Ledger; der Eintrag wird nach der `ArchiveHook`-Prüfung (8.4) aus dem Entscheidungsspeicher entfernt (Ersatzsegment, `SegmentKey` vernichtet). Kein Vier-Augen-Schritt.

**Takedown.** Wird in K irrtümlich Personenbezug festgestellt, besteht der Wiederholungstest von `Split` nicht, oder ordnet ein Gericht die Entfernung an:

1. Vier-Augen-Freigabe, kryptografisch signiert (Rollen `DataProtection` und `Legal` oder `TenantAdmin`).
2. **Sofort** `RedactK(KOrd)` im Gate-Log (Sperre analog Z1) und Aufnahme in `RedactedK`; alle Lesepfade liefern den Platzhalter `Redacted(ObjectId)`.
3. DK-Segmente mit `KOrd` im `source_set` werden ersetzt (neuer `SegmentKey`, alter vernichtet); Pack-Segment ohne den Blob **inklusive Salz** neu schreiben, `SegmentKey` vernichten, Rotation anfordern (4.3).
4. Im K-Ledger Vermerk `Redacted { object_id, grund_code }`; dessen Commitment im E-Ledger und in der Registersenke.

Die Objekt-ID und die Kette bleiben erhalten, der Inhalt nicht. Prüfer behandeln eine ID mit `Redacted`-Vermerk als undurchsichtig und prüfen nur die Kette. Nach Wiederherstellung wird jeder `Redacted`-Vermerk erneut angewendet (8.8). **Restrisiko [A] R-OBJID:** Wer `K_dedup` und den vermuteten Inhalt besitzt, kann die Objekt-ID dennoch testen.

---

