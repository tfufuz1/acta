# Acta Spezifikation v3.0: Löscharchitektur

**Status:** Zielzustand v1.0, überarbeitet gegenüber Fassung 2.2.1
**Geltungsbereich:** Löschung, Schlüsselverwaltung, Nachweis und Wiederherstellung. Ring-Architektur, Crate-Budgets und Gate-Katalog aus 2.2.1 bleiben gültig, soweit dieses Dokument nichts Anderes festlegt.

> **Rechtshinweis.** Dieses Dokument ist eine technische Spezifikation und keine Rechtsberatung. Die rechtliche Bewertung jeder Löschwirkung trägt der Verantwortliche. Die mit „P-xx“ markierten Punkte müssen vor einer Außenkommunikation von einer Fachanwältin oder einem Fachanwalt für Datenschutzrecht geprüft werden.

---

## 0. Änderungsübersicht gegenüber 2.2.1

| Nr. | Thema | 2.2.1 | 3.0 | Grund |
|---|---|---|---|---|
| N1 | Schlüsselgranularität | Scope-DEK je (Tenant, Collection) | **DocDek je Dokument**, wrapped unter Collection-KEK | Eine Collection-DEK würde beim Löschen eines Dokuments alle anderen Betroffenen mitlöschen oder die Löschung unmöglich machen. |
| N2 | Rolle der Schlüsselvernichtung | „keine Löschmaßnahme“ (E27) | **Zwei Ebenen:** Klartext-Strukturen werden physisch bereinigt. Persistente Reste werden durch Vernichtung des DocDek unlesbar gemacht. | Schlüsselvernichtung ersetzt die Bereinigung von Indizes nicht. Sie ist aber für Flash-Reste, Checkpoint-Kopien und Snapshots die einzige praktikable Methode. |
| N3 | Löschwirkungsklassen | „Erased“ nur nach physischer Messung, kein Krypto-Claim bei K0/K1 | Vier gestufte Klassen L, P, C, E mit getrennten Voraussetzungen (Abschnitt 4) | Der Bericht soll nur Aussagen treffen, die die Messung trägt. |
| N4 | Reihenfolge der Saga | Keine verbindliche Reihenfolge von Bereinigung und Schlüsselvernichtung | Erst Klartext bereinigen und messen, dann DocDek vernichten, dann Schlüsselabwesenheit messen | Vernichtung vor Bereinigung würde andere Dokumente in gemeinsamen Blöcken unlesbar machen. |
| N5 | Schlüsselregister | Ereignisse werden protokolliert | Registereinträge enthalten keine Schlüsselmaterialien. Das Register wird kompaktiert, damit keine alten Wrap-Blobs verbleiben | Ein append-only Log mit gewrappten Schlüsseln wäre eine Kopie des Schlüssels. |
| N6 | Positivkontrolle | Nur für Klartext-Scans | Zusätzlich Kanarien-Chiffrat für die Entschlüsselungsprobe | Ohne Kontrolle wäre „nicht entschlüsselbar“ ein stilles Ergebnis einer kaputten Prüfung. |
| N7 | Antwort-Caches | nicht ausdrücklich geregelt | Semantische Antwort-Caches sind eigene Löschschicht | Gecachte Antworten können Inhalte gelöschter Dokumente wiedergeben. |
| N8 | Trainingsdaten | nicht geregelt | Ausdrücklich ausgeschlossen (Abschnitt 9) | Maschinelles Verlernen ist nicht Gegenstand dieser Spezifikation. |

---

## 1. Rechtlicher Rahmen (Arbeitsgrundlage)

Die Anforderungen dieser Spezifikation leiten sich aus folgenden Normen ab. Ihre Auslegung ist mit P-Punkten markiert.

- **Art. 17 Abs. 1 DSGVO:** Der Verantwortliche muss personenbezogene Daten ohne unangemessene Verzögerung löschen, wenn ein Löschgrund vorliegt.
- **Art. 17 Abs. 2 und Art. 19 DSGVO:** Empfänger sind zu informieren. Das System muss deshalb Empfängerlisten führen können (P-12).
- **Art. 17 Abs. 3 DSGVO:** Ausnahmen, insbesondere gesetzliche Aufbewahrungspflichten. Sie führen zur Einschränkung (`Restricted`), nicht zur Löschung.
- **Art. 18 DSGVO:** Einschränkung der Verarbeitung als Zwischenzustand.
- **Art. 5 Abs. 1 lit. c und e DSGVO:** Datenminimierung und Speicherbegrenzung. Das gilt auch für Sicherungskopien und Schlüsselregister.
- **Art. 25 und Art. 32 DSGVO:** Datenschutz durch Technikgestaltung und angemessene technische Maßnahmen.
- **Erwägungsgrund 26 DSGVO:** Pseudonymisierte Daten bleiben personenbezogen, solange die Identifizierung mit angemessenem Aufwand möglich ist. Ein Schlüssel, der an anderer Stelle noch existiert, macht Reste daher nicht anonym.
- **EDPB Leitlinien 02/2025 (Blockchain):** Verschlüsselte Daten behalten ihren Personenbezug, solange die Verknüpfung nicht dauerhaft aufgehoben ist. Ob und unter welchen Bedingungen Schlüsselvernichtung als Löschung gilt, ist offen (P-01, P-02).
- **EDPB Leitlinien 01/2025 (Pseudonymisierung):** Relevant für Commitments und Surrogat-Vaults.
- **Anonymisierung:** Die Vorarbeiten nennen Leitlinien des EDPB zur Anonymisierung, die im Juli 2026 angenommen worden sein sollen. Der Wortlaut ist nicht geprüft. Dieser Punkt muss vor jeder Berufung darauf geprüft werden (P-14).
- **NIST SP 800-88 (Rev. 1 und Rev. 2):** Begriffe Clear, Purge und Destroy. Kryptografische Löschung (Cryptographic Erase) gilt dort für Speichermedien mit verschlüsselten Daten als anerkannte Methode, und zwar für das Medium und nicht für Anwendungsindizes.
- **DIN 66399:** Begriffe und Sicherheitsstufen für die Vernichtung von Datenträgern. Dient als Referenz für Schichten auf Medienebene.

**Konsequenz:** Die Software kann Art. 17 nicht rechtsverbindlich erfüllen. Sie kann nur belegen, welche technischen Zustände nach einer Löschung nachweislich eingetreten sind. Die Rechtsfolge entscheidet der Verantwortliche.

---

## 2. Begriffe und Löschwirkungsklassen

Jede Löschung wird anhand von vier Klassen bewertet. Die Klassen bauen aufeinander auf. Ein Bericht darf nur die höchste Klasse ausweisen, deren Voraussetzungen belegt sind.

| Klasse | Name | Aussage | Voraussetzungen |
|---|---|---|---|
| **L** | Logisch gesperrt | Der Scope ist für alle Lesepfade gesperrt. | `ErasedSet`-Eintrag durabel (fsync); Lesepfade geprüft. |
| **P** | Klartext bereinigt | Keine Klartextkopie des Scopes ist in einer Datenstruktur auffindbar, die Acta beschreibt. | Alle Klartextschichten (Abschnitt 6, Ebene A) gemessen; Positivkontrolle bestanden. |
| **C** | Persistente Reste unlesbar | Alle verbleibenden Kopien des Scopes liegen nur als Chiffrat unter einem DocDek vor, das nachweislich vernichtet ist. | Ebene B vollständig; Schlüsselabwesenheit gemessen; Entschlüsselungsprobe mit Kanarien-Chiffrat bestanden. |
| **E** | Technisch vollständig | P und C sind beide erfüllt, und es gibt keine Ausschlüsse außer den im Bericht aufgeführten. | P und C. Der Bericht listet explizite Ausschlüsse (Abschnitt 9). |

**Verbotene Aussagen** (Gate `deletion-honesty`): „100 % gelöscht“, „unwiederbringlich“, „DSGVO-konform“, „Art.-17-konform“, „garantiert gelöscht“. Außentexte werden aus dem Bericht erzeugt (INV-CLAIMS-1). Die Klasse E ist die stärkste zulässige Aussage und sagt nichts über die rechtliche Wirkung aus.

**Gültigkeit für Schlüsselstufen:**

- **K0** (unverschlüsselt): Klassen L und P möglich. Klasse C ist nicht erreichbar, weil keine Chiffrate existieren. Klasse E ist nicht erreichbar.
- **K1** (lokaler Keystore, Argon2id-KEK): Alle Klassen erreichbar.
- Die Verwahrungsstufen K2 und K3 aus 1.3 bleiben entfernt (E27 bleibt für Verwahrer gültig).

---

## 3. Kernprinzip: Zwei-Ebenen-Löschung

Ein Löschvorgang muss zwei Arten von Daten erfassen. Keine der beiden Ebenen genügt allein.

**Ebene A: Klartext-Strukturen.** Hierzu gehören MemTable, Index-Arena des HNSW (fp32 und SQ8), Postings und Term-Wörterbuch im Speicher, Block-Cache, Embedding-Cache und Antwort-Cache. Diese Strukturen müssen über die Lebensdauer des Prozesses hinweg physisch bereinigt werden. Eine Verschlüsselung hilft hier nicht, weil die Suche Klartext braucht. Die Bereinigung wird per Messung belegt.

**Ebene B: Persistente Reste.** Hierzu gehören SSTables, WAL-Segmente, Blob-Segmente, Snapshots, Checkpoints, Flash-Reste und Kopien auf Medien, die Acta nicht mehr erreicht. Jeder persistierte Nutzwert liegt als AEAD-Chiffrat unter dem DocDek des Dokuments vor (REQ-KEY-005). Die Vernichtung des DocDek macht alle Reste gleichzeitig unlesbar, ohne dass jede Kopie einzeln überschrieben werden muss.

**Warum beides nötig ist:**

- Ohne Ebene A bleibt Klartext im Speicher und im Index. Das betrifft auch den Vektorindex, der für die Suche unverschlüsselte Vektoren braucht. Vektoren sind pseudonyme personenbezogene Daten und lassen sich rekonstruieren (Inversionsangriffe).
- Ohne Ebene B bleiben Flash-Reste und Checkpoint-Kopien lesbar. Auf SSD ist das Überschreiben einzelner Blöcke nicht zuverlässig, weil das Wear-Leveling alte Zellen verschiebt. Für Ebene B ist deshalb Chiffrierung mit anschließender Schlüsselvernichtung die verlässlichere Methode.

Die Ebenen ergänzen sich. Ebene B deckt, was Ebene A nicht erreichen kann. Ebene A deckt, was ohne Schlüssel lesbar wäre, nämlich Klartext.

---

## 4. Schlüsselhierarchie (korrigiert)

```
KEK_root        Argon2id aus Passphrase (INV-KDF-1), lokal im Keystore (K1)
 └─ KEK_coll    je Collection, gewrappt unter KEK_root
     └─ DocDek  je Dokument (Betroffenen-Einheit), gewrappt unter KEK_coll
K_eng           Engine-Schlüssel für Struktur und Integrität (keine Nutzdaten)
K_commit        je (Tenant, Epoche), für Commitments (Abschnitt 8)
K_sc/*          Zweckschlüssel für Sidecar-Ketten (HMAC)
```

**DocDek-Regeln:**

1. Jedes Dokument (bzw. jede Betroffenen-Einheit, siehe P-05 und O-14) erhält genau einen DocDek. Er wird bei `remember` erzeugt.
2. Jeder Record in WAL, SSTable und Blob-Segment wird mit dem DocDek seines Dokuments verschlüsselt. Ein Block mit Records mehrerer Dokumente enthält daher mehrere DocDeks. Die Blockverschlüsselung mit K_eng bleibt als äußere Schicht erhalten.
3. Der gewrappte DocDek wird in der **KeyTable** gespeichert, nicht in einem Append-Log. Die KeyTable ist ein eigener Speicherbereich mit Ersetzen-Semantik (Abschnitt 5.3).
4. Vernichtung: Der gewrappte Eintrag wird mit Nullen überschrieben, fsync, danach gelöscht. Der Speicherpuffer des entwrappten DocDek wird zeroized (REQ-KEY-008).

**Kosten:** Ein Schlüssellookup je Record. Bei Lesezugriffen wird der entwrappte DocDek im Cache gehalten, der bei der Löschung zeroized wird. Die Messung dieser Kosten gehört in `benchmarks/budgets.toml` (O-24).

**Warum nicht Collection-DEK:** Eine Collection-DEK würde beim Löschen eines Dokuments alle anderen Dokumente der Collection unlesbar machen. Da Collections keine Betroffenen-Einheiten sind, ist das für Art. 17 nicht zulässig. DocDek ist die kleinste Einheit, die ohne Kollateralschaden vernichtet werden kann.

---

## 5. Löschsaga (verbindliche Reihenfolge)

### 5.1 Zustände

`Requested` → `Authorized` → `Frozen` → `LogicallyDeleted` → `PlaintextPurged` → `PlaintextMeasured` → `KeyDestroyed` → `Measured` → `Reported` / `Completed`, sowie `Partial{outstanding}` und `Restricted`.

### 5.2 Reihenfolge und Begründung

| Schritt | Aktion | Durabilität | Begründung |
|---|---|---|---|
| 1 | `Frozen`: `ErasedSet`-Eintrag (Commitment) | fsync vor jeder Änderung | L-Klasse; Lesesperre bereits aktiv (REQ-FGT-3). |
| 2 | `LogicallyDeleted`: S0-Tombstone im WAL-Batch | WAL-Commit | Crash-sicher; Saga kann fortgesetzt werden. |
| 3 | `PlaintextPurged`: Ebene A bereinigen (Index-Slots nullen, Caches zeroizen, Postings neu schreiben, Term-Wörterbuch bereinigen, Antwort-Cache invalidieren) und Ebene-B-Blöcke rekonstruieren | je Schritt Journal-Eintrag vor Ausführung | Blöcke mit fremden Dokumenten müssen neu geschrieben werden, solange deren DocDeks noch existieren. |
| 4 | `PlaintextMeasured`: Scanner prüft Ebene A | Bericht mit Positivkontrolle | P-Klasse. Ohne Messung kein Übergang. |
| 5 | `KeyDestroyed`: DocDek vernichten | Journal-Eintrag mit Intent vor der Vernichtung | Ab jetzt sind alle Reste unlesbar. Bis Schritt 4 wäre die Bereinigung der Blöcke nicht möglich gewesen. |
| 6 | `Measured`: Ebene B prüfen (Schlüsselabwesenheit, Kanarien-Probe, Rohdatei-Scan auf Chiffrat-Identität) | Bericht mit Positivkontrolle | C-Klasse. |
| 7 | `Reported`: Bericht erzeugen und in AuditChain eintragen | fsync | E-Klasse, falls alle Voraussetzungen erfüllt sind. |

**Crash-Verhalten:**

- Absturz nach Schritt 2 und vor Schritt 5: Recovery setzt bei `PlaintextPurged` fort. Der `ErasedSet`-Eintrag ist bereits durabel, also bleibt der Scope gesperrt.
- Absturz nach Schritt 5 und vor Schritt 6: Die Vernichtung ist vollzogen. Recovery führt nur die Messung aus. Der Bericht trägt nie Klasse C, bevor die Messung abgeschlossen ist.
- Ein Intent ohne Ausführungsnachweis wird beim Start erneut ausgeführt. Vernichtung ist idempotent.

### 5.3 KeyTable und Registerkompaktierung

Die KeyTable darf keine Kopien vernichteter Schlüssel halten. Ein append-only Log mit gewrappten Schlüsseln würde den Schlüssel dauerhaft aufbewahren.

- Das `KeyRegistry`-Sidecar (`MFKR`) führt nur **Ereignisse** ohne Schlüsselmaterial, zum Beispiel `created(doc_id_commitment, version)` und `destroyed(doc_id_commitment, version)`.
- Die Schlüsseldaten selbst liegen in der KeyTable mit Ersetzen-Semantik.
- Nach jeder Vernichtung wird die Segmentdatei der KeyTable durch Umschreiben ohne den vernichteten Eintrag ersetzt (`REQ-FMT-5`). Das Umschreiben folgt dem atomaren Ersetzmuster: temporäre Datei, fsync, rename, fsync des Verzeichnisses.
- Schlüsselabwesenheit wird im Rohdatei-Scan geprüft (Abschnitt 7.2).

### 5.4 Gleichzeitige Schreibvorgänge

Die Vernichtung eines DocDek hält den Key-Lock des Dokuments über die gesamte Saga (REQ-DEL-012). Ein konkurrierendes `remember` für dieselbe Betroffenen-Einheit wird abgewiesen. Dies muss mit loom-Tests abgesichert werden.

---

## 6. Schichten

Die Schichten aus 2.2.1 bleiben bestehen. Jede Schicht wird jetzt einer Ebene zugeordnet.

| Schicht | Crate | Ebene | Maßnahme | Messung |
|---|---|---|---|---|
| S1 LSM und WAL | `acta-store` | A und B | Blockrekonstruktion ohne Scope-Records; `compact_range(scope)`; WAL-Scrub; Records unter DocDek | `LsmScanner` (Klartext und Chiffrat) |
| S1b Blob-Segmente | `acta-store` | B | Segment-Rewrite oder Vernichtung über DocDek | `LsmScanner` |
| S2 Vektorindex | `acta-vector` | A und B | Slot-Payload (fp32 und SQ8) nullen; Snapshot neu schreiben, Slot-Payloads unter DocDek; Rebuild bei Tombstone-Anteil > 0,2 oder spätestens nach Frist `purge_rebuild_max_days` (Startwert 30, P-11) | `VectorScanner` |
| S3 Textindex | `acta-text` | A und B | Postings neu schreiben; Term-Wörterbuch: Terme, die nur im Scope vorkommen, entfernen; persistierte Segmente unter DocDek | `TextScanner` |
| S4 Caches | `acta-store`, `acta-engine`, `acta-privacy` | A | Block-, Embedding-, RRF- und Antwort-Caches für den Scope invalidieren und zeroizen; Surrogat-Vault bereinigen | `CacheScanner` |
| S5 Lineage und Ableitungen | `acta-engine` | A | Kaskade über Lineage (INV-DERIVED-1) | `LineageScanner` |
| S6 Checkpoints | `acta-checkpoint` | B | Policy `Rewrite` (Standard) oder `ExpireNow`; `ExpireAfterHold` nur unter Legal Hold. Chiffrierte Checkpoint-Kopien werden durch DocDek-Vernichtung unlesbar, gelten aber erst nach Rewrite als bereinigt. | `CheckpointScanner` |
| S7 Sidecars | `acta-crypto` | nur Commitments | Keine Nutzinhalte; Commitments nach Abschnitt 8 | `SidecarScanner` |
| S8 Externe Sicherungen | außerhalb | – | Ausweisen als `ExternalBackups` (REQ-RST-5) | keine Messung möglich |

**Zu S2 (Vektorindex):** Die Knoten bleiben als Routing-Knoten im Graph, ohne Nutzdaten. Die Graph-Kanten enthalten nur interne Slot-Nummern und keine Inhalte. Eine Wiederverwendung von Slot-Nummern ist nach Löschung nicht zulässig, bevor der Rebuild abgeschlossen ist (INV-HNSW-SLOT-1). Die verbleibende Restinformation (Nachbarschaftsbeziehungen zwischen Routing-Knoten) bleibt im Bericht als Ausschluss, bis der Rebuild ihn beseitigt hat (P-16).

**Zu S6 (Checkpoints):** Checkpoints sind Hardlinks auf SSTables oder Kopien. Da ihre Blöcke unter DocDek verschlüsselt sind, werden sie durch die Schlüsselvernichtung unlesbar. Für Klasse E genügt das allein nicht, denn Reste sollen nach Möglichkeit physisch entfernt werden. Der Standard ist deshalb `Rewrite`.

---

## 7. Messung und Nachweis

### 7.1 Ebene A: Klartext-Scanner

Jeder `LayerScanner` führt zwei Prüfungen durch:

1. **Rohbyte-Scan** aller Dateien des Datenverzeichnisses nach kanonischen Termen, Roh-IDs und Chunk-Prüfsummen des Scopes.
2. **Dekodierte Sicht** über die normalen Lesepfade mit allen aktiven Schlüsseln. Diese Sicht prüft, ob Records des Scopes noch über die Engine lesbar sind.

**Positivkontrolle (AC-1):** Im Scan-Lauf wird ein Kontrolldatensatz eingeschleust, der vom Scan finden muss. Ohne Treffer ist der Lauf ungültig.

### 7.2 Ebene B: Chiffrat-Scanner und Entschlüsselungsprobe

Für Klasse C werden drei Nachweise geführt.

1. **Schlüsselabwesenheit:** Über alle Dateien des Datenverzeichnisses, einschließlich KeyTable, Keystore, Sidecars, Checkpoints, Temp-Dateien und `.bak`-Dateien, wird nach dem gewrappten Eintrag des vernichteten DocDek gesucht. Ergebnis: null Treffer. Die Suche verwendet den Hash des gewrappten Blobs, der beim Erzeugen berechnet und im Journal in gehashter Form gehalten wird.
2. **Kanarien-Probe (Positivkontrolle für C, Neuerung N6):** Vor dem Test wird ein Kanarien-Datensatz mit eigenem DocDek erzeugt und verschlüsselt. Der Scanner muss ihn vor der Vernichtung entschlüsseln können. Danach gilt: Der Kanarien-DocDek ist vernichtet, und die Entschlüsselung schlägt fehl. Erst wenn beides gilt, ist die Probe gültig. Die Kanarien-Probe wird beim Scan nur mit einem eigenen Kanarien-Schlüssel durchgeführt, nie mit dem Schlüssel des Scopes.
3. **Chiffrat-Identität:** Alle Chiffratblöcke im Datenverzeichnis, die zum Scope gehörten, werden gezählt. Sie müssen sämtlich als Chiffrat unter dem vernichteten DocDek klassifiziert sein. Blöcke ohne Schlüsselzuordnung sind ein Fehler (fail-closed).

**Grenze:** Der Scanner kann nicht prüfen, ob der Schlüssel noch in RAM, Swap oder Prozessdumps steht. Diese Fälle gehören zu den Ausschlüssen (Abschnitt 9, P-03).

### 7.3 Bericht (`DeletionProof` V5)

Der Bericht enthält je Schicht Ebene, Methode, Zahlen, Digest, Positivkontrolle und Ergebnis. Zusätzlich enthält er Klasse, die Liste der Ausschlüsse und einen Zeitstempel. Er ist mit Ed25519 signiert, mit dem Schlüssel aus dem `KmsProvider` (INV-PROOF-KEY-1).

**Zeitanker:** Der Bericht wird über einen qualifizierten Zeitstempel (RFC 3161 bzw. eIDAS-Zeitstempel, Art. 42 eIDAS-VO) mit dem Digest des Berichts verankert. Der Zeitstempel belegt, dass der Bericht zu diesem Zeitpunkt existierte. Die Anbieterwahl ist offen und muss den Offline-Betrieb berücksichtigen (O-34).

**Selbstprüfung:** Der Bericht enthält eine Prüfsumme über alle Felder. Ein Verifikationswerkzeug (`doctor verify-report`) prüft Signatur, Prüfsumme und Klasse. Ein Bericht, der die Klasse E ausweist, aber eine Schicht ohne Positivkontrolle enthält, ist ungültig.

---

## 8. Commitments, Epochen und Sidecars

**Zweck:** Berichte und Sperrlisten müssen auf Betroffene verweisen, ohne sie zu identifizieren.

**Regeln:**

1. `subject_commitment = HMAC-SHA256(K_commit[tenant, epoch], kanonische Betroffenen-Referenz)`.
2. Die kanonische Referenz muss eine Mindestentropie haben. E-Mail-Adressen oder Namen dürfen nicht direkt verwendet werden, da sie per Wörterbuchangriff zurückgerechnet werden könnten, solange der Schlüssel existiert. Der Eingabewert wird durch eine zufällige Betroffenen-ID ersetzt, die nur im Dokumentdatensatz selbst steht (REQ-DEL-015).
3. Nach Ablauf von `commit_epoch_len + erased_set_retention` wird `K_commit` vernichtet. Danach sind die Commitments dieser Epoche nicht mehr zuordenbar. Dies gilt nur, wenn `K_commit` nicht in Backups enthalten ist (REQ-DEL-033).

**Sidecars:** Die Sidecar-Liste aus 2.2.1 (Abschnitt 7.9) bleibt gültig. Der `ErasedSet` enthält nur Commitments. Sidecars enthalten keine Nutzinhalte und keine DocDeks.

---

## 9. Ausschlüsse (verpflichtend im Bericht)

Jeder Bericht listet mindestens die folgenden Ausschlüsse, die nicht gemessen werden:

- **Externe Sicherungen** außerhalb des Datenverzeichnisses (S8). Die Rücksicherungsregel trägt der Verantwortliche.
- **Prozessspeicher und Swap** für den Zeitraum vor der Vernichtung. Ein Speicherfreigabe-Nachweis ist nicht möglich (P-03).
- **Hardwareinterne Reste** (Wear-Leveling-Zustände, Remapping, Controller-Cache). Die Chiffrierung ist hier die Gegenmaßnahme, aber die Abwesenheit von Restklartext kann nicht gemessen werden.
- **Trainings- und Fine-Tuning-Daten** externer Modelle (Abschnitt 10).
- **Kopien beim Auftragsverarbeiter** oder bei Dritten, insbesondere Modellanbieter-Logs und Prompt-Caches (P-08, P-12).
- **Ableitungen außerhalb des Lineage-Graphen**, zum Beispiel Exporte, die der Betreiber außerhalb von Acta angelegt hat.

Der Ausschluss-Text darf nicht gekürzt werden, wenn der Bericht an Dritte weitergegeben wird (INV-CLAIMS-1).

---

## 10. Nicht Gegenstand dieser Spezifikation

- **Maschinelles Verlernen** in Modellgewichten. Acta speichert Daten, es trainiert keine Modelle mit ihnen. Wenn der Betreiber eigene Modelle mit Acta-Daten trainiert, ist das ein eigener Verarbeitungsvorgang mit eigener Bewertung.
- **Löschung beim LLM-Anbieter** nach dem Senden eines Prompts. Dies betrifft den Vertrag mit dem Anbieter und die Auftragsverarbeitung.
- **Löschung in Backup-Medien außerhalb des Systems.** Der Restore-Guard (Abschnitt 11) verhindert nur, dass gelöschte Daten nach einem Restore wieder aktiv werden.

---

## 11. Sicherung und Wiederherstellung

**Restore-Guard:** Vor dem ersten Lesezugriff nach `restore_checkpoint` führt das System aus:

1. Lesen des `ErasedSet` des Zielsystems (nicht des Backups).
2. Für jeden Eintrag: Wiederanwendung der Löschsaga ab Schritt 3 (Ebene A) und Vernichtung eines eventuell wiederhergestellten DocDek (Schritt 5).
3. Messung wie in Abschnitt 7. Das Ergebnis wird als `restore_reapplied` im Journal protokolliert.

**Restore-Horizont:** Checkpoints, deren Erzeugungszeit älter als `erased_set_retention` ist, werden verweigert (`RestoreBeyondHorizon`). Die Startprüfung erzwingt `backup_max_age ≤ erased_set_retention`.

**Konsequenz für den Betrieb:** Ein Backup, das älter als die Horizontfrist ist, lässt sich nicht einspielen. Der Betreiber muss deshalb Backup-Aufbewahrung und Horizont aufeinander abstimmen (P-06). Eine Alternative ist ein zentrales Löschregister außerhalb von Acta, mit dem der Betreiber gelöschte Betroffene nach einem alten Restore manuell nachzieht. Diese Alternative ist als O-35 offen.

---

## 12. Anforderungen (Ergänzung und Ersatz)

| ID | Klasse | Anforderung | Mechanismus |
|---|---|---|---|
| REQ-DEL-N1 | [K] | Jedes persistierte Nutzbyte MUSS als AEAD-Chiffrat unter dem DocDek seines Dokuments vorliegen. Ausnahme: K0, dann Klasse C ausgeschlossen. | Typestate + Rohdatei-Scan |
| REQ-DEL-N2 | [K] | Die Löschsaga MUSS die Reihenfolge aus Abschnitt 5.2 einhalten. DocDek-Vernichtung ist erst nach gemessener Ebene A zulässig. | Zustandsautomat + Crash-Matrix |
| REQ-DEL-N3 | [K] | Die Klasse C DARF nur ausgewiesen werden, wenn Schlüsselabwesenheit und Kanarien-Probe bestanden sind. | Gate + Bericht-Typ |
| REQ-DEL-N4 | [K] | Die KeyTable DARF nach Vernichtung keinen Eintrag des DocDek mehr enthalten. Die Segmentdatei MUSS ersetzt worden sein. | Rohdatei-Scan + Ersetzen-Test |
| REQ-DEL-N5 | [K] | Das KeyRegistry-Log DARF kein Schlüsselmaterial und keinen gewrappten Schlüssel enthalten. | Literal-Scan + Schema-Test |
| REQ-DEL-N6 | [K] | Jeder Scan-Lauf MUSS die Positivkontrolle bestehen (AC-1 für Ebene A, Kanarien-Probe für Ebene B). | Test-Harness |
| REQ-DEL-N7 | [K] | Antwort-Caches MÜSSEN als Löschschicht S4 erfasst und vor `PlaintextMeasured` invalidiert sein. | Scanner + Test |
| REQ-DEL-N8 | [K] | Der Bericht MUSS Klasse, Schichtzustände und mindestens die Pflicht-Ausschlüsse aus Abschnitt 9 enthalten. | Typestate |
| REQ-DEL-N9 | [K] | Die Verifikation des Berichts MUSS ohne Netzwerk möglich sein und Signatur, Zeitanker und Prüfsumme prüfen. | `doctor verify-report` |
| REQ-DEL-N10 | [K] | `K_commit` MUSS nach Ablauf der Epoche vernichtet und aus Backups ausgeschlossen werden. | Journal + Backup-Test |
| REQ-DEL-N11 | [K] | Der Restore-Guard MUSS wiederhergestellte DocDeks gelöschter Dokumente erneut vernichten. | Restore-Test mit Backup vor Löschung |
| REQ-DEL-N12 | [K] | Außentexte MÜSSEN aus der erreichten Klasse erzeugt werden. Verbotene Begriffe aus Abschnitt 2 führen zum Build-Fehler. | Gate `deletion-honesty` |

**Ersetzt:** REQ-DEL-002 (Schlüsselvernichtung als Stufe), REQ-DEL-025 und REQ-DEL-026 (2.2.1-Epochen und gewrappte Schlüssel im Payload). Die Anforderungen bleiben als Hinweis erhalten, sind aber nicht mehr gültig.

---

## 13. Tests

- **Crash-Matrix:** Abbruch an jedem Zustandsübergang der Saga, mit anschließendem Neustart und Prüfung von Lesesperre, Berichtszustand und Schlüsselstatus.
- **Rohdatei-Forensik:** Nach Abschluss einer Löschung werden alle Dateien des Datenverzeichnisses sowie eine Speicherkopie des Prozesses auf Kanarien-Klartexte und gewrappte Schlüsselblobs durchsucht. Die Speicherkopie dient als Positivkontrolle für Ebene A, der Prozessspeicher selbst ist ein Ausschluss.
- **Backup-Restore:** Backup vor der Löschung, Löschung, Restore. Prüfung, dass der Scope gesperrt bleibt und keine lesbaren Reste entstehen.
- **Gleichzeitigkeit:** loom-Tests für `remember` und `forget` auf derselben Betroffenen-Einheit.
- **Lasttest:** Messung der Leseleistung mit DocDek-Lookup, Ziel laut `budgets.toml`.
- **Statistische Residualtests:** Auf Restdaten, die nach Löschung noch Chiffrat sind, werden Entropietests ausgeführt. Ein unerwartet niedriger Entropiewert zeigt einen Fehler bei der Chiffrierung an.

---

## 14. Entscheidungsregister (Ergänzung)

| ID | Festlegung | Status |
|---|---|---|
| N-E27 | Schlüsselvernichtung ist **Löschmaßnahme für persistente Reste** (Klasse C), **keine** Maßnahme für Klartext-Strukturen. Ersetzt E27 aus 2.2.1 in diesem Punkt. | ADR-pflichtig (§18.6) |
| N-E40 | DocDek je Dokument, Record-Verschlüsselung, KeyTable mit Ersetzen-Semantik | ADR-pflichtig |
| N-E41 | Löschwirkungsklassen L, P, C, E als einzige Außensprache | ADR-pflichtig |
| N-E42 | Reihenfolge der Saga nach Abschnitt 5.2 | verbindlich |
| N-E43 | Zeitanker für Berichte über qualifizierten Zeitstempel | Anbieterwahl offen (O-34) |

---

## 15. Offene Punkte

| ID | Frage | Hinweis |
|---|---|---|
| P-01 | Gilt Schlüsselvernichtung als Löschung nach Art. 17, als Anonymisierung oder nur als Einschränkung? | Entscheidend für die Wirkung von Klasse C. |
| P-02 | Entfällt der Personenbezug von Chiffraten nach Schlüsselvernichtung für jede Stelle? | Relativer Personenbezug. |
| P-03 | Wie lässt sich die Abwesenheit von Schlüsselmaterial in RAM, Swap und Dumps nachweisen? | Derzeit nicht vollständig möglich. |
| P-05 | Behandlung von Embeddings als personenbezogen | Vorsorglich ja; zu bestätigen. |
| P-06 | Zulässige Backup-Frist und Wiedereinspielsperre | Bestimmt `erased_set_retention`. |
| P-07 | Personenbezug der Nachweis-Metadaten | Commitments nach Abschnitt 8. |
| P-08 | Rolle des Anbieters | Kein Datenzugriff, keine Telemetrie. |
| P-09 | Aufbewahrungspflichten und Einschränkung statt Löschung | `Restricted`. |
| P-11 | Löschfrist „unverzüglich“ und `purge_rebuild_max_days` | Startwert 30 Tage, zu prüfen. |
| P-12 | Empfängerinformation nach Art. 17 Abs. 2 und Art. 19 | Empfängerliste ist noch nicht spezifiziert. |
| P-14 | Stand und Wortlaut der EDPB-Leitlinien zur Anonymisierung | Vor Berufung prüfen. |
| P-16 | Restinformation im HNSW-Graph vor dem Rebuild | Frist festlegen. |
| O-34 | Zeitstempel-Anbieter mit Offline-Verifikation | Technische Auswahl. |
| O-35 | Zentrales Löschregister außerhalb von Acta als Alternative zur Horizontfrist | Architekturentscheidung. |

---

## 16. Fazit

Die Änderung gegenüber 2.2.1 besteht im Kern darin, Schlüsselvernichtung nicht mehr als Ersatz für Bereinigung zu verstehen, sondern als Ergänzung für genau die Reste, die sich physisch nicht erreichen lassen. Dazu braucht es eine feinere Schlüsselgranularität (DocDek statt Scope-DEK), eine feste Reihenfolge der Saga, gestufte Löschwirkungsklassen und Positivkontrollen für beide Ebenen.

Das Alleinstellungsmerkmal liegt darin, dass Acta für jede Löschung ausweist, welche Klasse erreicht wurde, und dabei offenlegt, was nicht gemessen werden kann. Ob ein konkreter Vorgang rechtlich als Löschung gilt, entscheidet der Verantwortliche auf Grundlage dieses Berichts und der offenen P-Punkte.
