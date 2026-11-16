## 5. Speicher

### 5.1 Segmentspeicher (`acta-seg`) [E]

Der Speicher ist ein **append-only Segmentspeicher**, kein allgemeiner LSM-Baum. **Begründung:** Die Last ist „einmal schreiben, häufig lesen, gezielt nach Betroffenem entfernen“; Aktualisierungen in der Fläche kommen nicht vor. Segmente lassen sich als Ganzes versiegeln, gezielt ersetzen oder entfernen. `acta-seg` ist **schlüsselfrei** (HR-9): Schlüssel werden als Parameter übergeben.

**Siegelmodi:**

|Modus|Schlüssel|Einsatz|Löschwirkung|
|---|---|---|---|
|`RecordSealed`|DEK je Dokument-Version (4.2)|Ebene P: Datensätze, Köpfe|Schlüsselvernichtung macht Datensätze unlesbar|
|`SegmentSealed`|`SegmentKey` je Segment|Ebene K (Packs, Entscheidungsprotokolle), D (Indexsegmente), Journal|Ersatzsegment schreiben, alten `SegmentKey` vernichten, Altdatei mit Scrub entfernen|

**Partitionen (Invariante I-PART):** `K`, `P`, `DK` (Ableitungen aus K) und `DP` (Ableitungen aus P) werden **niemals** in derselben Datei gemischt. Ein Indexsegment hat genau **eine** Quellart; `source_set` ist typisiert (`KOrd` oder `VerOrd`).

**Dateien je Mandant:** Journal (`wal-<journal_id>`), Segmente (`seg-<file_id>.acta`), Manifest (`MANIFEST`, `manifest-<n>`), Tresor, Gate-Log, Ledger, Betriebsdaten. Alle Dateizugriffe laufen ausschließlich über `acta-fs`.

**Journal-Rahmen:** `[magic u8][flags u8][len u32][hdr_crc32c u32][payload][chain_tag 32]`. `chain_tag` ist eine HMAC-SHA256-Kette über alle Rahmen mit Schlüssel `HKDF(k_idx, "acta/wal/v1" ‖ TenantId ‖ journal_id)`. Nutzlasten der Klasse P liegen ausnahmslos als Umschlag unter dem Versions-DEK vor; Nutzlasten der Klassen K und D sind unter dem `SegmentKey` des Journals (keyspace `0x0030`) versiegelt. Das Journal enthält nie Klartext. **Flush-Politik:** Das Journal enthält nur noch nicht in Segmente überführte Daten; es wird bei `wal_max_bytes` (64 MiB), `wal_max_age` (30 s) und bei jedem Saga-Schritt S3 in Segmente überführt, rotiert, und die Altdatei wird mit Scrub entfernt (Journal-`SegmentKey` vernichtet).

**Segmentformat (`format_version` 1):**

|Abschnitt|Inhalt|
|---|---|
|Kopf (256 B)|Magic `ACTASEG\0`, `format_version u16`, `partition u8`, `seal_mode u8`, `tenant_ord u32`, `file_id 16`, `created_seq u64`, `block_size u32` (64 KiB Klartext), `block_count u32`, Kopf-MAC (HMAC mit `HKDF(k_idx, "acta/seg/v1" ‖ file_id)`)|
|Datenblöcke|`SegmentSealed`: je Block ein Umschlag (AAD: `file_id`, Blockindex). `RecordSealed`: Rahmen `[rec_key][len u32][Umschlag]`|
|Blockindex|Folge `(erster_Schlüssel, Offset, Länge, crc32c)`; bei `SegmentSealed` selbst versiegelt|
|Fuß|`source_set` (Roaring, serialisiert; nur bei `P`, `DK`, `DP`), Statistik (Anzahl, Schlüsselbereich), SHA-256 über alle vorangehenden Bytes, Fuß-MAC, `footer_len u32`, End-Magic|

**Manifest:** atomare Batches `AddSegment`, `RemoveSegment`, `SeqHwm`, `Partition`, HMAC-gekettet. Ein fehlgeschlagener Manifest-Schreibvorgang vergiftet den Speicher sofort (Fail-Closed, `Poisoned(Tenant)`). Sichtbar ist ein Segment erst durch das Manifest (C03, C04).

**Größen:** `segment_max_bytes` = 256 MiB begrenzt die Kosten eines gefilterten Rewrite.

**Schlüsselordnung der Datensätze (P):** `[kind u8][tenant_ord u32][partition u8][cluster_id 16][ver_ref 16][!seq u64][flags u8]`. `cluster_id` ist die `ClusterId` des Hauptbetroffenen (bei Dokumenten ohne Betroffenen die `DocRef`). **Wirkung:** Alle Daten einer Person liegen zusammen; bei Löschung genügt häufig das Entfernen ganzer Segmente (Datei-Drop). Die Ingest-Planung bündelt nach `ClusterId`: Cluster mit mehr als `cluster_seg_min` (4 MiB) erhalten eigene Segmente, kleinere werden nach Schlüsselordnung in gemeinsame Segmente gepackt (dann Rewrite ohne Zielrecords, begrenzt durch `segment_max_bytes`).

**`ClusterId`:** nicht gespeichert, nur mit `SubjectKey` berechenbar (4.2). Nach dessen Vernichtung ist jede in Sicherungen verbliebene `cluster_id` ein bedeutungsloser Zufallswert; ein dauerhaftes, mit `k_idx` prüfbares Pseudonym entsteht nicht (Z2). **`SubjectTag` steht nie im Klartext in Segmenten, Journalen oder Köpfen.** Die Gruppierung gleicher `cluster_id` im Abbild bleibt, solange der Schlüssel lebt, als Restrisiko R-CLUSTER ausgewiesen.

**Handkodierte Formate:** Alle Plattenformate werden von Hand mit geprüften Feldzugriffen kodiert (`from_be_bytes`, `get(..)`); `serde` und `bincode` sind für Plattenformate verboten. **Begründung:** Ein Plattenformat ist eine über Jahre stabile Schnittstelle und darf nicht aus Strukturlayouts folgen. Jedes Format hat Golden-Tests und ein Fuzz-Target.

**Entscheidungsregel Eigenbau [E]:** Eine bestehende eingebettete Speicher-Engine wird nur verwendet, wenn sie (1) gezieltes Entfernen und Überschreiben der Dateien eines Betroffenen ohne globale Verdichtung erlaubt, (2) deterministisch unter `FaultVfs` prüfbar ist, (3) Werte vor dem Schreiben im Journal als Umschlag entgegennimmt und (4) keine FFI-Fläche mitbringt. Das Ergebnis wird vor Beginn der Speicherimplementierung als Messbericht dokumentiert; fällt die Prüfung negativ aus, gilt diese Spezifikation unverändert (Eigenbau, Budget 6 500 LOC).

### 5.2 Ordinale, Herkunft (Lineage) und Sperrmengen [E]

1. Jede **Version** eines P-Dokuments erhält beim Ingest eine `VerOrd`, jeder Betroffene eine `SubjOrd`, jedes K-Objekt eine `KOrd` (alle monoton je Mandant, nie wiederverwendet). Sie sind rein intern, erscheinen nie in Ebene K, Berichten, Ledgern oder Metriken und stehen nur im versiegelten Dokumentkopf und Betroffenenverzeichnis, in `DP`-/`DK`-Fußabschnitten und im **Gate-Log**.
2. Jede abgeleitete Struktur (Indexsegment, Cache-Eintrag, Embedding-Block, Sitzungsschritt, Fallindex) trägt `source_set` der Ordinale, aus denen sie entstanden ist.
3. Ein Segment oder Cache-Eintrag ist **betroffen** genau dann, wenn `source_set ∩ (ErasedVers ∪ RestrictedVers ∪ SupersededVers) ≠ ∅` (DP) bzw. `source_set ∩ RedactedK ≠ ∅` (DK).
4. **Typzwang:** `Derived<T>` kapselt Nutzdaten untrennbar mit ihrer `Lineage` (nicht leer). Indizes nehmen Daten nur über `put_derived(Derived<T>)` an. Eine Ableitung ohne Herkunft ist nicht konstruierbar (Z13).
5. Ordinale ohne Zuordnung sind keine Pseudonyme, sobald Verzeichnis und Handle-Register entfernt sind; `ErasedVers` darf daher dauerhaft bestehen (Verdichtung des Logs, 5.3). Restrisiko R-ORD (Zeitmuster dichter Ordinale) wird ausgewiesen.
6. **Versionswechsel.** Ersetzt eine neue Version eine alte, werden alle Ableitungen der alten Version (Indexeinträge, Embeddings, Caches) bereinigt; nur die neue Version wird indiziert.

### 5.3 Gate (`acta-gate`) [E]

`ReadGate`-Zustand (Roaring-Mengen, Epoche, alles im Arbeitsspeicher, Änderungen durch Zeigertausch):

|Menge|Elemente|Bedeutung|
|---|---|---|
|`ErasedSubjects`|`SubjOrd`|Betroffene mit laufender oder abgeschlossener Löschung; verhindert neue Zuordnung|
|`ErasedVers`|`VerOrd`|Versionen im Zustand `Gated`, `Expired` oder erledigt: `check_ver → Erased`|
|`RestrictedVers`|`VerOrd` + Grundcode|`Restricted(grund)`: nur `restricted_read`|
|`SupersededVers`|`VerOrd`|ältere Version: nur `restricted_read`|
|`RedactedK`|`KOrd`|Takedown in K: Objekt gesperrt, Platzhalter `Redacted`|

`ErasedVers` hat Vorrang vor `RestrictedVers` und `SupersededVers`. `check_ver` und `is_redacted_k` sind O(1) ohne E/A. **Sondierung:** Jeder Lesepfad (Suche, Antwort, Zeitreise, Szenario, Cache-Treffer, Export, Vorschau, Sitzung, Streaming) ruft das Gate **vor jeder Entschlüsselung und vor jeder Ausgabe** auf; Cache-Einträge sind betroffen, wenn ihre Lineage das Gate trifft (Miss). Eine Anfrage merkt die Epoche und prüft sie vor jedem Ausgaberahmen (9.5).

**Gate-Log.** Rahmen `[seq u64][op u8][len u16][payload][chain_tag 32]`, HMAC-Kette mit `HKDF(k_idx, "acta/gate/v1" ‖ TenantId)`. Operationen: `0x01 EraseSubject(SubjOrd)`, `0x02 EraseVer(VerOrd, reason)`, `0x03 Restrict(VerOrd, grund)`, `0x04 Release(VerOrd)` (nur aus `Restricted`, nie aus `Erased`), `0x05 Supersede(VerOrd)`, `0x06 RedactK(KOrd)`, `0x07 HighWater(kind, value)`, `0x08 Snapshot(seq)`. Die Epoche ist die `seq` des letzten Rahmens. **Gruppen-`fsync`** (3.6). **Verdichtung** wöchentlich: neuer Snapshot-Stand (Roaring-Mengen, MAC) in neuer Datei, atomarer Austausch, alter Log wird archiviert (C23); `ErasedVers`, `ErasedSubjects`, `RedactedK` werden nie verkleinert.

**Reihenfolge einer Sperre (Z4).** Zuerst im RAM (Epoche erhöht), Erfolg wird erst nach `fsync` gemeldet. Schlägt der `fsync` fehl, bleibt die RAM-Sperre bestehen, der Aufruf scheitert mit `Integrity`, der Mandant wird vergiftet (`Poisoned(Tenant)`, 15.2); nach dem Neustart gilt der durable Stand, fehlt der Eintrag, ist der Antrag erneut zu stellen (der Aufruf hat nie Erfolg gemeldet).

### 5.4 P-Speicher (`acta-pstore`) [E]

**Dokument-Version.** Eine Version besteht aus (a) Dokumentkopf unter `HK(v)` (4.2), (b) Inhaltsrecords (Blöcke des Quelltexts, Chunk-Tabelle, Metadaten) unter `DEK(v)`, (c) Ableitungen in DP. `DocRef` ist die stabile Dokumentidentität; die aktuelle Version steht im Handle-Register.

**Betroffenenverzeichnis.** Je Betroffenem ein Datensatz unter `cluster_id`, versiegelt unter `DirKey(s)`. Inhalt: `SubjOrd`, `subject_kind` (`External`, `Staff`, `Synthetic`), Aliasliste, Liste `(DocRef, VerRef, VerOrd, Rolle main|co, Zustand)`, `ActorRef`-Handles (bei `Staff`, je Epoche), Empfängerliste für Art. 19 (`DisclosureLog`), Anfragen (`request_id`, `due_at`, `identity_verified`), Fristen. Auffinden einer Person: externe Kennung → `SubjectTag` → Tresoreintrag → `SubjectKey` (bei Alias: Primär-Tag) → `ClusterId` → Verzeichnis.

**Handle-Register.** Abbildung `PersonalRef → (DocRef, VerRef, VerOrd, ClusterId)` im P-Teil, versiegelt unter dem Tresorschlüssel `RegistryKey(t)` (abgeleitet `HKDF(VK(t,g), "acta/gen/v1")` und mit jeder Rotation neu versiegelt). Wird bei der physischen Bereinigung (S3) entfernt. Verbleibende Kopien in Sicherungen enthalten nur Zufallswerte ohne Zuordnung (R-HANDLE).

**Vorgang (`Case`).** P-Objekt der Klasse `Personal`: `CaseId`, `external_ref` (Aktenzeichen des Fachverfahrens), Mitglieds-`VerRef`s, zugewiesene Nutzer (`ActorRef`), Status (`open`, `closed`), `closed_at`. DEK mit `n = 0`. Der **Fallindex** (Roaring-Bitmap `VerOrd` je `CaseId`) ist ein DP-Segment mit Lineage; er speist die Zugriffsmenge `A_P` (9.4). Ein Vorgang endet, wenn die Fachanwendung ihn schließt oder alle Mitglieder erledigt sind; bei Löschung des letzten Mitglieds wird der Vorgang vernichtet (S3). Restrisiko R-CASE: Das Aktenzeichen im Vorgang ist pseudonym, bis der Vorgang vernichtet ist.

**Idempotenztabelle.** `ingest_id → (VerRef, Ergebnis)` je Mandant, versiegelt (keyspace `0x0070`), Lebensdauer `ingest_idem_ttl` = 7 Tage.

### 5.5 Ableitungen D [E]

- **DK-Segmente** sind global je Mandant und `IndexGen`, `SegmentSealed`, ohne Löschlogik; sie tragen `KOrd`-Lineage; Takedown erzeugt Ersatzsegmente (6.7). Gilt gleichermaßen für Text-, Vektor- und Graph-Indizes (`acta-graph`, DK-Segmente mit CSR-Struktur und K-Entitäten/Relationen).
- **DP-Segmente** sind **je `ClusterId`** des Hauptbetroffenen organisiert: Cluster mit mehr als `dp_cluster_min_bytes` (1 MiB Index) erhalten eigene Segmente (Verschmelzung nur innerhalb des Clusters), kleinere werden in gemeinsamen Bucket-Segmenten nach Schlüsselbereich gepackt. Erledigt eine Löschung den Hauptbetroffenen, genügt Datei-Drop; bei Nebenbetroffenen oder Bucket-Segmenten Ersatzsegment mit neuem `SegmentKey` ohne die Zielordinale. Entitätsextraktion aus P-Dokumenten (`acta-graph`) erzeugt abgeleitete Personendaten, die ausschließlich in DP-Segmenten abgelegt werden; kein Graph aus P in DK.
- **Datensicherung der Ableitungen:** DK-Segmente sind Teil der Datensicherung (Z7 nach Wiederherstellung); DP-Segmente **nicht** (`backup_derived_dp = false`, Standard und Pflichtwert bei `Besonders`); sie werden nach Wiederherstellung neu aufgebaut (14.4). Ein Betreiber DARF `backup_derived_dp = true` setzen; dann gilt für wiederhergestellte DP-Segmente der Wiederholungsschritt S3 (8.8).
- **Caches** (Antwort-, Sitzungs-, Chunk-, DEK-Cache, Graph-Schnappschüsse) sind Ebene D im Arbeitsspeicher, tragen Lineage und sind `ErasureLayer`.

### 5.6 Schreibpfad-Regeln [E]

|ID|Regel|
|---|---|
|WP-1|**Schlüssel zuerst:** Schlüssel durabel im Tresor, dann Umschlag (R-KEY-FIRST)|
|WP-2|**Ledger zuerst:** Ledger-Eintrag durabel, dann Ref-Bewegung bzw. Sichtbarkeit|
|WP-3|**Sichtbarkeit zuletzt:** Manifest-Batch bzw. K-Commit macht Daten sichtbar; Absturz davor hinterlässt nur unsichtbare, verwaiste Dateien (C03 bis C05)|
|WP-4|**Physische Tatsachen vor Journal:** beim Neustart gelten Tresorzustand, Manifest und vorhandene Dateien vor dem Journalstand|
|WP-5|**Atomar ersetzen:** neue Datei schreiben, `fsync`, `rename`, `fsync` des Verzeichnisses (`rename_atomic`)|
|WP-6|**Scrub:** entfernte Dateien werden überschrieben (Nullen), synchronisiert und gelöscht (`remove_scrub`); Flash-Reste bleiben Annahme (A-FLASH)|
|WP-7|**Kein Retry nach `FsyncFailed`** (3.7)|
---

