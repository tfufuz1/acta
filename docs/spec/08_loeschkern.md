## 8. Löschkern

### 8.1 Zustände und Lesesperre [E]

|Zustand|Bedeutung|Lesen|Schreiben|Schlüsselvernichtung|
|---|---|---|---|---|
|`Live`|regulär|ja (Z16)|ja|nein|
|`Quarantine`|Klasse `Unklassifiziert`, höchstens `quarantine_max`|nein|nein|nur durch Ablauf|
|`Restricted(grund)`|eingeschränkt; Gründe: `Retention`, `LegalHold`, `Objection`, `Contested`, `MultiSubjectReview`, `Precautionary`|nur `restricted_read`|nein|**nein**|
|`Gated`|Löschvorgang läuft, Sperre durabel|**nein**|nein|ja (8.3)|
|`Superseded`|ältere Version eines Dokuments in P|nur `restricted_read`|nein|nach `history_horizon` bzw. sofort bei Löschgrund|
|`Pending`|Sperre aktiv, Fortsetzung wartet (Anker, Senke, Review, Archiv, Sicherungsfenster)|wie Ausgangszustand|nein|wartet|
|`Expired`|Frist abgelaufen, Löschvorgang ausgelöst|wie `Gated`|nein|ja|
|`Erased`|Endzustand nach Schlüsselvernichtung; Gate bleibt dauerhaft|**nein**|nein|erledigt|

**Lesepfadregel.** Jeder Lesepfad (Suche, Antwort, Zeitreise, Szenario, Cache-Treffer, Export, Vorschau, Sitzung, Streaming) ruft `check_ver(ord)` **vor** jeder Entschlüsselung und Ausgabe auf (O(1), keine E/A). Ein Nutzer **mit** Zugriffsrecht erhält `Erased`; ein Nutzer **ohne** Zugriffsrecht erhält dasselbe Fehlerbild wie bei einem nicht vorhandenen Ziel (Z16).

**`restricted_read`.** Die Rolle `DataProtection` darf `Restricted`- und `Superseded`-Inhalte für festgelegte Zwecke lesen: `Auskunft`, `Rechtsanspruch`, `Berichtigung`, `Review`, `Aufbewahrung`, `Aufsicht`. Jeder Zugriff ist zweckgebunden, wird mit `ActorRef` und Zweckcode im E-Ledger protokolliert (ohne Inhalte, Z15) und erreicht nie die RAG-Pipeline, nie Caches und nie Sitzungen. Erledigte Ziele (`Gated`, `Erased`) sind auch für `restricted_read` gesperrt.

**Mitarbeitende (`Actor`).** Das Ausscheiden löscht die Identität in Ebene P (Zuordnung `ActorRef` → Person durch Saga auf dem `Staff`-Betroffenen), nicht die Begründungskette fremder Vorgänge (7.1).

**Mandanten-Off-Boarding.** Tresor und Exportdateien des Mandanten werden entfernt (Scrub), seine Segmente bereinigt, seine Ledger nach Aufbewahrungsvorgabe archiviert oder vernichtet. Kopien Dritter werden mit ihrer Sicherungsepoche unlesbar; der Bericht weist `offboarding_exposure_until` aus (12.6).

### 8.2 Auslöser und Betroffenenanfragen [E][R]

**Auslöser:** Antrag (Art. 17 Abs. 1), Widerruf oder Widerspruch (Art. 21), Fristablauf der Aufbewahrungsklasse, Zweckfortfall, Takedown (6.7), Off-Boarding, Ausscheiden von Mitarbeitenden, Ablauf von `quarantine_purge_after`, `payload_retention`, `history_horizon`, gerichtliche Anordnung.

**Ablauf einer Anfrage.**

1. Eingang über API oder CLI durch eine berechtigte Rolle des Betreibers (`request.create`) mit einer oder mehreren externen Kennungen derselben Person.
2. **Identitätsprüfung** (Art. 12 Abs. 6) liegt beim Betreiber; Acta speichert nur das Ergebnis (`identity_verified`), keine Ausweisdaten. Ohne `identity_verified` laufen weder Auskunft noch Löschung; Einschränkung (`Restricted(Contested)`) ist vorab zulässig.
3. Auffinden: externe Kennung → `SubjectTag` → Tresor → `SubjectKey` → `ClusterId` → Verzeichnis (5.4). Mehrere Kennungen werden über Aliase vereinigt; ist eine Person nachträglich als zwei Betroffene geführt, wird die Anfrage mit allen Kennungen gestellt (Zusammenführung bestehender Betroffener ist nicht vorgesehen).
4. Plan und Trockenlauf (8.9).
5. Entscheidung durch Rolle gemäß 12.5.
6. Ausführung.
7. Antwort an die betroffene Person durch den Betreiber.

Acta führt je Anfrage `due_at` = Eingang + 1 Monat (Art. 12 Abs. 3; Verlängerung um bis zu 2 Monate mit Begründung ist ein Betreiberakt) und warnt ab 70 % der Frist (`RequestDue`).

**Auffinden unerkannter Datensätze.** `subjects.discover` (nur `DataProtection`, `restricted_read`) sucht in P und in K nach Name- und Kennungsmustern der Person und liefert Kandidaten (Handle, Fundstelle ohne Volltext). Die Entscheidung über `AddSubject` (P) bzw. Takedown (K) trifft der Review; es gibt keine automatische Ausweitung.

|Recht|Acta-Funktion|
|---|---|
|Auskunft, Art. 15|`restricted_read` auf Verzeichnis und Dokumente; Ausgabe: Datenkopie, Zwecke, Empfänger (`DisclosureLog`), Aufbewahrungsklasse, Herkunft; Ableitungen (Embeddings) werden nach Art und Existenz genannt, nicht als Vektoren; **Daten Mitbetroffener werden vor dem Schreiben des Exports serverseitig geschwärzt (Art. 15 Abs. 4)**; ist das nicht sicher möglich, Review|
|Berichtigung, Art. 16|neue Version (neue `VerRef`), alte `Superseded` (Berichtigungsnachweis bis `history_horizon`); Mitteilung nach Art. 19|
|Löschung, Art. 17|Saga 8.3|
|Einschränkung, Art. 18|`Restricted(Contested)` oder `Restricted(Objection)`; Ableitungen werden bereinigt|
|Mitteilung, Art. 19|Ausgangskorb 8.6|
|Datenübertragbarkeit, Art. 20|signierter JSON-Export der von der Person bereitgestellten Daten (nur Ebene P), Eintrag im Kopienregister|
|Widerspruch, Art. 21|`Restricted(Objection)` bis zur Prüfung; danach Saga oder Aufhebung (`Release`)|

Die Zuordnung zu den Pflichten des Betreibers steht in 20.5.

### 8.3 Löschsaga [E]

Die Saga ist ein dauerhafter, idempotenter Zustandsautomat je **Löschvorgang** (`ErasureId`) mit dem Umfang `Subject` (Betroffener), `Document` (Dokument mit allen Versionen), `Version`, `Expired` (Stapel fällig gewordener Ziele) oder `Takedown` (K, 6.7). Ein Löschvorgang enthält **Dokument-Aufgaben** (`DocTask`) je Version mit eigenem Modus (8.9). Jeder Schritt ist wiederholbar; **physische Tatsachen** (Schlüssel vernichtet, Segment ersetzt) haben beim Neustart Vorrang vor dem Journalstand (Z4).

|Schritt|Zustand nach Abschluss|Inhalt|Durabilitätspunkt|
|---|---|---|---|
|S0|`Requested`|Anfrage im E-Ledger (Commitment auf `SubjectTag` bzw. `VerRef`), `ErasureId` (UUIDv7)|Ledger-`fsync`|
|S1|`Gated`|**Trockenlauf** (8.9) über alle Versionen der Zielmenge einschließlich `Superseded`; Konfliktfreie Versionen: `EraseVer`; Konfliktversionen: `Restrict(MultiSubjectReview)`; wird der Trockenlauf nicht innerhalb `erase_return_timeout` fertig, werden **alle** Versionen der Zielmenge vorläufig `Restrict(Precautionary)` und der Lauf wiederholt; Sperre im RAM, Gate-Log mit Gruppen-`fsync`, Epoche erhöhen, Registersenke **asynchron**; danach Rückkehr an den Aufrufer|Gate-Log-`fsync`|
|S2|`Planned`|Messmuster und Canary-Bindung erzeugen (8.5), Ausgangskorb füllen (8.6), Kopienregister erfassen, Segment- und Record-Listen der Zielmenge ermitteln. Alle Arbeitsdaten unter `ScopeKey`|Arbeitsdatei-`fsync`|
|S3|`Purged`|Caches, Sitzungen, Antwortcache, DEK-Cache, Chunk-Cache, `ContextPurge`; DP-Ersatzsegmente schreiben (neuer `SegmentKey`, Manifest-Batch `AddSegment`/`RemoveSegment` atomar) oder Datei-Drop; P-Segmente ohne Zielumschläge neu schreiben oder ganze Segmente entfernen; Handle-Register-Einträge, Fallindex-Mitgliedschaften und Vorgänge entfernen; Journal flushen, rotieren, Altdatei mit Scrub entfernen; Tempdateien; Arbeitslisten stammen aus S2 (kein `SubjectKey` erforderlich)|Manifest-`fsync` je Batch|
|S4|`PlaintextMeasured`|Messung Klasse B (8.5)|Messprotokoll|
|S5|`KeyTombstoned`|**S5a:** `destroy_many` für `DocumentKey(v)` aller Zielversionen sowie `SubjectKey(s)` und `SubjectAlias`-Einträge, **sobald keine Version mehr an `s` gebunden und lesbar bleiben muss** (4.2). **S5b:** nach S3 `destroy_many` der ersetzten `SegmentKey`s. Grabstein, Direct-Read-Verifikation, **Canary-Probe**|Tresor-`fsync`|
|S6|`Rotated`|Rotation auf Anforderung (4.3), `DestroyProof`, `RootRotated`|Ledger-`fsync`|
|S7|`BackupsExpired`|wartet bis `backup_exposure_until`, dann `destroy_backup_epoch` je Epoche (zeitgesteuert)|Anker-Nachweis|
|S8|`Measured`|Messung Klasse K und M|Messprotokoll|
|S9|`Reported`|Bericht (18.1), E-Ledger, Senke bestätigt; `ScopeKey` vernichtet (bei offenem Ausgangskorb erst nach dessen Abschluss)|Ledger-`fsync`|

**Reihenfolge der Schlüsselvernichtung [E].**
- *Standardpfad:* S1, S2, S3, S4, S5, … Solange der Zielschlüssel lebt, lässt sich eine unvollständige Bereinigung nachholen (Wiederableitung von Messmustern, Reparatur von Ersatzsegmenten), und eine Fehlbedienung ist bis S5 durch den Review rücknehmbar (nur vor S1 durch Abbruch; nach S1 ist die Sperre dauerhaft, außer `Release` aus `Restricted`).
- *Vorgezogener Pfad* (Klasse `Besonders` oder wenn `erase_sla_B` zu 70 % verbraucht ist): S1, S2, **S5a**, S3, S4, S5b. S3 arbeitet mit den Listen aus S2 und braucht keine Zielschlüssel. Die `SegmentKey`s gemischter Segmente bleiben bis zu ihrem Ersatz am Leben (S5b), sonst gingen Daten Dritter verloren. Die Messmuster (S2) wurden vor der Vernichtung abgeleitet; Messung B (S4) läuft nach S5a mit diesen Mustern.

**Teilläufe (Tranchen).** Ein Löschvorgang mit Konfliktversionen läuft in Tranchen: Tranche 1 = konfliktfreie Versionen (S1 bis S5 sofort), Tranche 2 = Versionen nach Review-Entscheidung (S1 bis S5 je Entscheidung). `SubjectKey(s)` wird mit der letzten Tranche vernichtet. Bis dahin steht der Löschvorgang als `Pending(Review)` mit aktiver Sperre.

**Nebenläufigkeit.** Ein Ausführer je Mandant (Einzelschreiber, 3.6); Leser laufen parallel gegen die Gate-Epoche. Sagas mit überlappenden Zielen werden nach `ErasureId` serialisiert; der zweite Plan wird nach dem ersten neu berechnet. S6 fängt Anforderungen mehrerer Sagas in einer Rotation (Mindestabstand `rotation_min_gap`). Rotation (4.3) und S5 stimmen sich über `vault_freeze` ab.

**Fehlerverhalten.** Transiente Fehler: Wiederholung mit Backoff (1 s bis 1 h, `saga_retry_max` = 8), Zustand bleibt. Nicht behebbar: `Pending` mit Alarm (`SagaStuck`). `Integrity`/`Crypto`: Vergiftung des kleinsten nachweislich isolierenden Bereichs (15.2). Ein Fehler in S5 nach teilweiser Vernichtung wird beim Start anhand des Tresorzustands zu Ende geführt (Vernichtung ist idempotent).

### 8.4 Aufbewahrung, Einschränkung, Legal Hold, Archiv-Hook [E][R]

- **Aufbewahrungsklasse** (12.2) liefert `retain_until` je Datum (nicht je Person; OLG Dresden, 14.12.2021, 4 U 1278/21 [V]). Ein Löschantrag während der Frist führt zu `Restricted(Retention)`: Ableitungen (Embeddings, Indexeinträge) werden bereinigt (Datenminimierung; bei Aufhebung Neuaufbau), der Datensatz bleibt unter seinem Schlüssel. Zum Fristende läuft die Saga automatisch (`Besonders`: Karenz 0 Tage).
- **`legal_hold`** ist ein Kennzeichen je Dokument, Betroffenem oder Mandant (Art. 17 Abs. 3 lit. b, e), gesetzt durch die Rolle `Legal` mit Vier-Augen-Freigabe, Grundcode und Wiedervorlagefrist `hold_review_interval`. Es blockiert S5.
- **Original und Repräsentation.** Das Original liegt im Quellsystem (`source_of_record`, 9.1). Die Aufbewahrungspflicht für das Original rechtfertigt nicht automatisch die Weiternutzung im aktiven Retrieval **[⚠]**: Acta isoliert die Repräsentation (Gate, Ableitungen bereinigt) und lässt das Original unberührt.
- **`ArchiveHook`** (Kantenport): Vor Frist- und Takedown-Löschung eines Objekts, für das Acta selbst Quelle ist (Entscheidungsprotokolle, `source_of_record = acta` nur mit schriftlicher Betreiberentscheidung und DSFA), wird `offer` aufgerufen. Fehlt eine Entscheidung nach `archive_wait_max` (30 Tage) oder schlägt der Hook fehl, bleibt das Objekt `Restricted(Retention)`. Anbietungspflichten gegenüber Archiven folgen aus Bundes- oder Landesarchivrecht [R][V] und sind im Löschkonzept (12.4) abzubilden.

```rust
pub trait ArchiveHook: Send + Sync {
    fn offer(&self, tenant: TenantId, class: ObjectClass, due: Date) -> Result<ArchiveOutcome, HookError>;
}
pub enum ArchiveOutcome { Accepted, Declined, NotRequired }
```

### 8.5 Messung und Positivkontrolle [E] (Z11)

Jede Messung läuft zunächst gegen einen **Canary**: einen synthetischen Betroffenen (`subject_kind = Synthetic`) mit künstlichem Dokument im selben Mandanten, der durch dieselben Pfade (Ingest, Index, Cache, Cluster-Segment) erzeugt wurde. Canaries sind für alle Lesepfade und Berichte unsichtbar (nur Zähler `synthetic_subjects`). Trifft das Messverfahren den Canary nicht, ist der Lauf `Invalid`; nach `measure_retries` = 3 Fehlschlägen entsteht `Pending` mit Alarm `MeasureInvalid`. Messmuster (bis zu 256 Fenster-Prüfwerte `HMAC(ScopeKey, 32-Byte-Fenster)` aus dem Ziel) entstehen in S2 und leben nur unter `ScopeKey`.

|Schicht|Verfahren|Art|
|---|---|---|
|Gate (alle Lesepfade)|Sondierungslesung je Pfad liefert `Erased`; Positivkontrolle: gleiche Sondierung am Canary vor dessen Sperre trifft|vollständig|
|DP-Segmente, Indizes|Manifest-Aufzählung: kein Segment mit `source_set` ∩ Ziel ≠ ∅|vollständig|
|Caches, Sitzungen, Antwort-, DEK-, Chunk-Cache|Aufzählung und Messmuster-Scan|vollständig|
|P-Segmente, Journal|Suche nach `cluster_id` und `ver_ref` der Zielmenge in allen Dateien (Aufzählung der Schlüsselräume)|vollständig (Klasse M)|
|Tempdateien, Arbeitsverzeichnisse|Messmuster-Scan|Stichprobe, ausgewiesen|
|Tresor|Grabstein, Direct-Read; **Canary-Probe:** vor S5 wird ein Umschlag unter dem echten Zielschlüssel erzeugt, nach S5 muss `open` mit `Erased` oder `DecryptionFailed` scheitern|vollständig|
|Anker, Sicherungsepochen|`DestroyProof`, `destroy_backup_epoch`|Beleg|
|Inferenzdienst|`ContextPurge`-Bestätigung des Dienstes|`not_measured` (A-INFER)|
|RAM, Swap|nicht messbar|deklariert (A-RAM)|
|Flash-Zellen|nicht messbar|deklariert (A-FLASH)|
|Externe Kopien|nicht messbar|deklariert (A-EXT)|

Nicht implementierte Messungen liefern `NotImplemented`, nie `reached`.

### 8.6 Nachlauf: Art. 19, Kopienregister [E][R]

- **Ausgangskorb (`Outbox`).** In S2 liest Acta das `DisclosureLog` und legt je Empfänger eine Mitteilung ab (Empfängerhandle, Art der Mitteilung `Erasure|Rectification|Restriction`, externe Kennung der Person für das Fachverfahren des Betreibers; versiegelt unter `ScopeKey`, keyspace `0x0060`). Der Betreiber stellt über den Kantenport `Notifier` zu und bestätigt. Abgeschlossen ist ein Eintrag mit `Delivered`, `Impossible` (Begründung) oder `Expired` nach `outbox_ttl` = 90 Tage; erst dann wird `ScopeKey` vernichtet. Acta veröffentlicht keine Daten und löst Art. 17 Abs. 2 nur als Hinweis an das Fachverfahren aus. Restrisiko R-OUTBOX: Die externe Kennung bleibt bis zum Abschluss des Korbs unter `ScopeKey` erhalten.
- **Kopienregister.** Je Mandant führt Acta ein Register aller Kopien außerhalb der Primärspeicher: Tresorexporte, Datensicherungen, Art.-15/20-Exporte, Berichte, Supportpakete, externe Inferenzfreigaben. Eintrag: Art, Anzahl, Ort (Bezeichnung des Betreibers), Ablauf. Das Register enthält keine personenbezogenen Inhalte.

```rust
pub trait Notifier: Send + Sync {
    fn deliver(&self, tenant: TenantId, m: &OutboxMessage) -> Result<DeliveryAck, NotifyError>;
}
```

### 8.7 Erfüllungsklassen [E]

Klassen bauen aufeinander auf und werden über die **Zielmenge** des Löschvorgangs (alle Versionen mit Modus `DeleteAll` oder Löschgrund) bewertet. Der Bericht nennt je Klasse `reached` oder `not_reached` mit Zeitpunkt und Annahmen (18.1). Das Wort „gelöscht“ steht nur mit der erreichten Klasse (1.6).

|Klasse|Name|`reached`, wenn|
|---|---|---|
|**G**|gesperrt|Gate durabel, alle Lesepfad-Sondierungen liefern `Erased`, Positivkontrolle bestanden|
|**B**|bereinigt|S3 vollzogen; Messung Klasse B (Caches, Sitzungen, DP-Segmente, Tempdaten) mit Positivkontrolle|
|**K**|Schlüssel vernichtet|S5 und S6 vollzogen, Canary-Probe wie erwartet gescheitert, `DestroyProof` des Ankers **und alle betroffenen Sicherungsepochen vernichtet** (`backup_exposure_until` erreicht)|
|**M**|vollständig gemessen|Klasse K, P-Segment- und Journalmessung ohne Zielschlüsselräume, Registersenke bestätigt (`registry_mirrored`), alle Schichten gemessen oder deklariert|

Zwischenstände: `key_tombstoned` (Grabstein, Rotation offen) und `K_pending_backups` (Rotation erfolgt, Sicherungsfenster offen). `Restricted` wird als „eingeschränkt, nicht gelöscht“ ausgewiesen; der Bericht trennt Zielmenge und zurückgestellte Versionen (`scope_result ∈ {complete, partial_restricted, pending}`). Klasse M ist eine technische Aussage innerhalb der Annahmen, **keine** rechtliche Feststellung der Löschung nach Art. 17 (20.2).

**Erwartungswerte.** Klasse G sofort, Klasse B innerhalb `erase_sla_B` (24 h), Klasse K nach `max(T_ROT, anchor_destroy_max)` und dem Sicherungsfenster (`backup_exposure_until` ≤ Tombstone + `T_BACKUP` + `backup_epoch`, höchstens 36 Tage). Dass Sicherungen nicht bei jedem Antrag punktuell verändert werden müssen, solange nach einer Wiederherstellung erneut gelöscht wird, entspricht der Auffassung der Aufsicht (SDM-Baustein 60, EDPB-CEF 2026; Fristen ungeregelt) [V].

### 8.8 Restore-Schutz [E] (Z12)

Eine Wiederherstellung durchläuft den Modus `Restoring` (alle Lesepfade geschlossen):

1. **Voraussetzung:** Registersenke erreichbar **oder** Ankerstand (`last_head`) für Gate-Log und beide Ledger (K, E) vorhanden. Fehlt beides: `Quarantine`.
2. **Tresor** nur aus einem gültigen Export innerhalb des Restore-Horizonts (`T_BACKUP + backup_epoch`); ältere Sicherungen werden mit `RestoreBeyondHorizon` verweigert. Datensicherungen (Segmente, Journal) ohne gültigen Export sind unlesbar. Dokumente ohne Schlüssel im Export (nach dem letzten Export entstanden) werden gezählt (`unrecoverable_docs`) und im Wiederherstellungsbericht ausgewiesen.
3. **Ordinale:** alle Zähler auf `max(Stand der Sicherung, Hochwassermarke der Senke) + 1` (2.4).
4. **Senke lesen:** Gate-Log und E-Ledger holen; Kette, signierte Checkpoints, Kopf ≥ Ankerkopf prüfen, sonst `RestoreBehindAnchor`. **Gate-Log und E-Ledger werden aus der Senke vorwärts ergänzt.** Ein zurückgerolltes **K-Ledger** bedeutet verlorene Commits: der Knoten bleibt für diesen Mandanten in `Quarantine`, bis `DataProtection` und `TenantAdmin` den Verlust mit Vier-Augen bestätigen (`accept_rollback`, Eintrag im E-Ledger).
5. **Gate-Mengen** = Vereinigung aus Sicherungsstand und Senke.
6. **Wiedervernichtung:** Für jeden wiederhergestellten `SubjectKey` und `DocumentKey` wird geprüft, ob (a) die `SubjOrd`/`VerOrd` im Gate steht (Verzeichnis oder Dokumentkopf lesbar, solange der Export den Schlüssel noch lebend enthält) oder (b) `commit(e, KeyId)` für eine Epoche im Horizont im E-Ledger vorkommt (alle `CommitKey(e)` im Horizont sind im Export enthalten). Treffer: Grabstein, Saga ab S3 wiederholen. **Ferner:** jedes wiederhergestellte Segment mit `source_set ∩ Gate ≠ ∅` wird ersetzt oder entfernt (betrifft DP-Segmente bei `backup_derived_dp = true` und P-Segmente).
7. **Takedown:** jeder `Redacted`-Vermerk wird erneut angewendet (6.7).
8. **Offene Sagas** der Senke (Status ≠ `Reported`) laufen weiter.
9. **Ableitungen:** DP-Indizes werden nach Wiederherstellung neu aufgebaut (nur für Versionen `Live` und nicht im Gate); DK folgt der Sicherung.
10. Erst danach `Live`; Ereignis `Restored` im E-Ledger (Re-Witness).

Die Zusage ist **bedingt** (Z12): Sie hängt an der Verfügbarkeit der Senke oder des Ankerstands und an R-HORIZON.

### 8.9 Mehrbetroffene Datensätze [E] (Z3)

**Trockenlauf.** Für jede Version `d` der Zielperson `s` berechnet Acta die Mitbetroffenen `C = subjects(d) \ {s}` (Dokumentkopf) und prüft je `c ∈ C`: (1) Aufbewahrungsklasse mit `retain_until` in der Zukunft, (2) `legal_hold`, (3) registrierter Widerspruch oder offener Streit, (4) offener Vorgang (Status `open` mit Zuweisung im Fachverfahren). Kein Befund: Modus `DeleteAll`. Mindestens ein Befund oder Sammelakte: Review.

|Modus|Wirkung|Wann|
|---|---|---|
|`DeleteAll` (Standard)|`SubjectKey(s)` und `DocumentKey(v)` werden vernichtet; die Version ist für **alle** unlesbar|kein Konflikt, Mitbetroffene vertretbar|
|`RedactSplit`|Neue Version (neue `VerRef`, neue Betroffenenliste ohne `s`, Inhalt mit geschwärzten Spannen von `s`); die alte Version wird sofort als Löschgrund-`Superseded` behandelt und durchläuft S1 bis S5|Dokument lässt sich sinnvoll schwärzen; **Pflicht für Sammelakten**|
|`RestrictShared`|Neue Version **ohne Bindung an `s`** (DEK aus `DocumentKey` und den Schlüsseln der Mitbetroffenen), Zustand `Restricted(grund)`, Ableitungen nicht aufgebaut; die alte Version durchläuft S1 bis S5. Die Daten von `s` bleiben nur im Rahmen der Aufbewahrungspflicht oder des Holds eines Mitbetroffenen erhalten; Bewertung wiederholt sich bei Wegfall des Konflikts (dann `DeleteAll` oder `RedactSplit`)|Aufbewahrungspflicht oder Hold eines Mitbetroffenen|

**Review** (`DataProtection` plus Fachbereich, Vier-Augen) entscheidet innerhalb `multi_review_max` = 10 Werktage; bis dahin `Restricted(MultiSubjectReview)`. Es gibt kein automatisches `DeleteAll` nach Fristablauf; bei Überschreitung eskaliert `ReviewOverdue` an die Datenschutzfunktion, das Dokument bleibt eingeschränkt. Die Schwärzung für `RedactSplit` erfolgt im Review-Werkzeug (`review.redact`): `DataProtection` markiert die Spannen von `s`, `acta-pii` schlägt Spannen vor, die Prüfung auf Restvorkommen läuft vor dem Schreiben der neuen Version; scheitert sie, bleibt das Dokument `Restricted`. Der Bericht nennt die **Anzahl** der Mitbetroffenen je Dokument, nie deren Kennungen. Gleichzeitige Anträge mehrerer Betroffener desselben Dokuments werden nach `ErasureId` serialisiert; der Plan wird je Lauf neu berechnet. Ein nachträglich erkannter Mitbetroffener löst `AddSubject` aus (2.3); bis dahin gilt Z3 für ihn nicht und der Bericht weist das aus.
---

