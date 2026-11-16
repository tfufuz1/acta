## 20. Rechtliche Einordnung

Dieses Kapitel ordnet ein und **stellt nichts rechtlich fest**. Konformität ist ein Betriebszustand der Organisation (1.6). Fundstellen tragen den Prüfstatus aus Anhang E.

### 20.1 Personenbezug und Rollen [R]

- Verschlüsselte Daten bleiben personenbezogen, solange ein Schlüssel existiert (Art. 4 Nr. 5, Erwägungsgrund 26; EDPB-Leitlinien 02/2025 Rn. 51). Nach Vernichtung ist der **relative** Maßstab anzulegen (EuGH C-582/14 Breyer; C-413/23 P EDPS/SRB, 04.09.2025); ob der Personenbezug für jede Stelle entfällt, ist vor Außenverwendung zu prüfen [V][⚠].
- Vektor-Embeddings aus personenbezogenen Quellen behandelt Acta vorsorglich als personenbezogen (DP, versiegelt, löschbar). Die Rechtslage ist offen [⚠]; die DSK-Orientierungshilfe RAG behandelt die Datenbank als personenbezogenen Verarbeitungsgegenstand ohne Invertierbarkeitsprüfung. Die EDPB-Stellungnahme 28/2024 betrifft Modelle, nicht RAG-Datenbanken (Analogie).
- `PersonalRef`, Ordinale und Commitments sind pseudonym, solange Zuordnungen oder `CommitKey(e)` existieren; danach nach dem relativen Maßstab nicht mehr zuordenbar, mit Restrisiko R-HANDLE, R-ORD, R-HORIZON.
- Löschen ist selbst Verarbeitung (Art. 4 Nr. 2) und braucht eine Grundlage: die Erfüllung der Löschpflicht.
- Verantwortlicher ist der Betreiber (Art. 4 Nr. 7). Zum Hersteller siehe 14.7: Bei dem Zuschnitt ohne Zugriff auf Klartext, Chunks, Vektoren und Schlüssel ist der Hersteller in der Regel weder Auftragsverarbeiter noch Verantwortlicher [⚠]. Art. 25 bindet den Verantwortlichen; eine unmittelbare Herstellerhaftung nach DSGVO besteht nicht (die DSK-Entschließung vom 12.12.2025 fordert sie *de lege ferenda*).
- Produktaussagen unterliegen § 5 UWG und dem Sachmangelrecht (§ 434 BGB) [V]; daher 1.6.

### 20.2 Löschbegriff und Methode [R][⚠]

Die DSGVO enthält keinen Legalbegriff der Löschung und keine Methode. Nach dem Stand der Rechercheprüfung (27.10.2026) hat **weder eine Aufsichtsbehörde noch ein Gericht** Schlüsselvernichtung als Löschung nach Art. 17 anerkannt oder abgelehnt; der EDPB nennt im Blockchain-Kontext Bedingungen (Rn. 51 bis 53, 63, 85, 87, 96, 103, 104). NIST SP 800-88 Rev. 2 erkennt Cryptographic Erase technisch unter Voraussetzungen als Purge an [V]. Das SDM (Baustein 60) nennt kryptografisches Löschen nicht als Methode, verlangt aber Regeln zu Schlüsseln und ihrer Löschung.

**Acta stützt sich nie allein auf Schlüsselvernichtung.** Die Löschung besteht aus drei Säulen: (1) sofortige Lesesperre (Klasse G), (2) physische Bereinigung der Klartext-Strukturen und Ableitungen (Klasse B), (3) Schlüsselvernichtung nach Rotation und Sicherungsfenster (Klasse K), jeweils gemessen (Klasse M). Die Bedingungen für die Tragfähigkeit der Schlüsselvernichtung und ihre Umsetzung:

|Bedingung|Umsetzung|
|---|---|
|Schlüsselgranularität je Betroffenem und je Version|4.2 (`SubjectKey`, `DocumentKey`, DEK je `VerRef`)|
|Vernichtung aller Schlüsselkopien|Tresor ohne Schlüsseljournal, Rotation, Exportepochen, Anker; Core-Dumps und Swap unterbunden (13.1)|
|Kein unverschlüsselter Parallelbestand|Ableitungen DP versiegelt, Caches mit Lineage, `ContextPurge`, Tempdateien gemessen|
|Metadaten ohne Re-Identifizierung|Auffüllung (4.5), keine Klartext-Tags, `ClusterId` abgeleitet, Ordinale ohne Zuordnung|
|Stand der Technik, Algorithmenwechsel|4.7|
|Passphrase-Ableitung|Argon2id nach RFC 9106, Entropieprüfung 80 Bit (4.1, 4.4)|
|Echte Vernichtung|Grabstein, Direct-Read, Scrub, Rotation|
|Wartefristen Cloud-KMS|`anchor_destroy_max`, Bericht weist Frist aus|
|Keine Kompromittierung vor Vernichtung|`mlock`, Sandbox, Bedrohungsmodell 2.5|
|Nachweisbarkeit|Löschbericht, Ledger, Positivkontrolle, `acta-verify`|

Anonymisierung als Erfüllung der Löschpflicht ist umstritten (Instanzgerichte bejahen, EDPB-CEF 2026 skeptisch gegenüber Praxis) und wird von Acta **nicht** als Löschmethode verwendet; `Split` setzt sie nur als Aufnahmebedingung für K voraus (2.3). Bei Aufbewahrungspflicht gilt Einschränkung statt Löschung (Art. 17 Abs. 3, Art. 18). Ein nur gesperrter Schlüssel ist Einschränkung. § 35 BDSG trägt Vektordatenbanken und automatisierte Backups nicht (nur nicht automatisierte Verarbeitung; unionsrechtliche Zweifel).

**Backups.** Nach SDM Baustein 60 und EDPB-CEF 2026 müssen Sicherungen nicht bei jedem Antrag punktuell verändert werden, solange nach einer Wiederherstellung unmittelbar erneut gelöscht wird. Eine feste Behördenfrist gibt es nicht; die Fristen in Anhang A sind Auslegungsentscheidungen dieser Spezifikation. Acta ergänzt die Rücksicherungssperre durch ablaufende Epochenschlüssel, damit Kopien nach `T_BACKUP + backup_epoch` unlesbar sind.

### 20.3 Annahmen und Restrisiken [A]

|ID|Annahme oder Restrisiko|Wirkung|
|---|---|---|
|A-FLASH|Überschreiben erreicht nicht verlässlich die physische Zelle (Wear-Leveling, Dateisystem-Journale)|Klasse K hängt an Rotation, Ankervernichtung und Sicherungsfenster|
|A-RAM|Schlüssel- und Klartextreste in RAM, Swap, Dumps, Root-Zugriff sind nicht auszuschließen|ausgewiesen; T8 ohne Zusage|
|A-ANCHOR|Der Anker vernichtet Generationsgeheimnisse innerhalb `anchor_destroy_max`|Frist geht in Klasse K ein|
|A-CRYPTO|XChaCha20-Poly1305, HKDF, HMAC, SHA-2, Ed25519, Argon2id halten|Neubewertung 4.7|
|A-EXT|Kopien außerhalb von Acta (Nutzer, Exporte, externe Dienste, Bildschirmfotos, ausgelieferte Streamrahmen) sind nicht erfasst|ausgewiesen|
|A-INFER|Der Inferenzdienst hält seine Zusicherungen (11.2); Modellausgaben sind nicht deterministisch|`not_measured` im Bericht; Z7 gilt ab Abfragevektor|
|A-CLOCK|Zeit ist nicht vertrauenswürdig|Klemmung, `Poisoned(Node, ClockRegression)`|
|A-DETECT|Automatische Erkennung von Personenbezug ist unvollständig|Standardmodus `Whole`, Vier-Augen, Takedown|
|A-ARCH|Ganzzahlarithmetik verhält sich auf x86_64 und aarch64 gleich|Z7|
|A-FS|Das Dateisystem hält `fsync`-Zusagen; `O_DIRECT`-Rücklesen ist möglich; kein Copy-on-Write ohne Deklaration|Selbsttest 13.1|
|R-HANDLE|Handles verknüpfen K-Stellen und Entscheidungen zu einer Person oder Akte|ausgewiesen; `actor_handle_epoch`|
|R-CLUSTER|`ClusterId` gruppiert Datensätze im Abbild, solange `SubjectKey` lebt|nach Vernichtung bedeutungslos|
|R-ORD|Zeitmuster dichter Ordinale|ausgewiesen|
|R-CASE|Aktenzeichen im Vorgang pseudonym bis zur Vernichtung|ausgewiesen|
|R-OUTBOX|Externe Kennung bleibt bis Korb-Abschluss unter `ScopeKey`|ausgewiesen|
|R-OBJID|Wer `K_dedup` und den vermuteten Inhalt besitzt, kann eine Objekt-ID prüfen|ausgewiesen|
|R-HORIZON|Commitments pseudonym, solange `CommitKey(e)` lebt|Frist mit Untergrenze (15.4)|
|ADR-15|`k_idx` wird nicht rotiert; Besitz von `k_idx` und einer Kennung erlaubt die Prüfung, ob eine Person erfasst war|ausgewiesen|

### 20.4 Freigabepflichten vor dem Produktivbetrieb und vor Produktzusagen [R]

Vor Außenverwendung einer Aussage und vor jeder Freigabe sind mit Fachanwalt oder Datenschutzfunktion zu klären: (1) Qualifikation der Schlüsselvernichtung (Löschung, Anonymisierung, Unzugänglichmachung, Einschränkung) und Haltung der zuständigen Aufsicht am Sitz des Betreibers; (2) relativer Personenbezug nach Vernichtung für jede Stelle; (3) Wirksamkeitsbeweis (Penetrationstest, Audit der Schlüsselvernichtung, KMS, HSM, RAM, Swap, Dumps, Backups); (4) Umfang (Quelle, Chunks, Vektoren, Indizes, Caches, Protokolle, Prompt-Historie, Kontexte externer Dienste); (5) Embeddings; (6) Backups (Frist, Wiedereinspielsperre, Backup-Schlüssel, Haltung der Aufsicht); (7) Nachweis-Metadaten und Dauer (Art. 5 Abs. 2 gegen Rückstandslosigkeit; AG Lörrach; OLG Dresden 4 U 940/24); (8) Rolle des Herstellers (Support, Telemetrie, Updates, Schlüsselwiederherstellung, Inferenzanbindung); (9) Aufbewahrungspflichten und Archivrecht; (10) Wortwahl nach UWG und BGB; (11) KI-Verordnung; (12) NIS2 und CRA; (13) DSFA; (14) Aktualität (EDPB-Anonymisierungsleitlinien, Neufassung SDM-Baustein 60, Digital Omnibus, Rechtsprechung zu Embeddings und Schlüsselvernichtung). Zusätzlich: Kryptobericht (4.7), Messbericht (16.5), unabhängige kryptografische Prüfung, Penetrationstest, Wiederherstellungsübung.

### 20.5 Zuordnung zu Betreiberpflichten [R]

|Norm|Pflicht des Verantwortlichen|Unterstützung durch Acta|
|---|---|---|
|Art. 5 Abs. 1 lit. c, e|Datenminimierung, Speicherbegrenzung|Aufbewahrungsklassen, `history_horizon`, `payload_retention`, Fristenjob, Auswahl der Ingest-Dokumente|
|Art. 5 Abs. 2, 24|Rechenschaft|Löschbericht, Ledger, Entscheidungsprotokoll|
|Art. 12|Fristen, Identitätsprüfung|`due_at`, `RequestDue`; Prüfung beim Betreiber|
|Art. 13, 14|Informationspflichten|Beschreibung der Verarbeitung (18.2); Betreiberakt|
|Art. 15|Auskunft|8.2, Schwärzung Mitbetroffener|
|Art. 16|Berichtigung|Versionen, L06|
|Art. 17|Löschung|Saga 8.3, Klassen 8.7|
|Art. 18|Einschränkung|`Restricted`, L07|
|Art. 19|Mitteilung|Ausgangskorb 8.6|
|Art. 20|Übertragbarkeit|signierter Export|
|Art. 21|Widerspruch|`Restricted(Objection)`|
|Art. 22|automatisierte Einzelentscheidung|Nicht-Ziel; `Review` mit Pflichtreviewer|
|Art. 25|Technikgestaltung, Voreinstellung|Entkopplung K/P, Fail-Closed, Typzwang, `Whole` als Standard|
|Art. 28|Auftragsverarbeitung|kein Herstellerzugriff (14.7), Egress-Prüfung (13.3)|
|Art. 30|Verzeichnis|18.2|
|Art. 32|Sicherheit, Wirksamkeitsprüfung|Kapitel 4, 13, 14, 16|
|Art. 33, 34|Meldung von Verletzungen|Alarme `Poisoned`, Integritätsberichte; Meldung ist Betreiberakt|
|Art. 35|DSFA|18.3|
|Art. 44 ff.|Drittlandtransfer|kein Egress im Standard (13.3)|

### 20.6 KI-Verordnung, NIS2, Cyber Resilience Act [R][V]

- **KI-VO (VO (EU) 2024/1689).** Ein RAG-System ist ein KI-System (Art. 3 Nr. 1). Die Risikoklasse hängt vom Einsatz ab (Anhang III, zum Beispiel Personalwesen, Zugang zu wesentlichen öffentlichen Leistungen); sie ist vom Betreiber vor Einsatz zu prüfen. Bei Hochrisiko: Ereignisprotokolle (Art. 12), menschliche Aufsicht (Art. 14; technisch `Review`), Betreiberpflichten (Art. 26; Aufbewahrung der Protokolle mindestens sechs Monate, soweit Unionsrecht, insbesondere Datenschutzrecht, nichts anderes vorsieht), für bestimmte öffentliche Stellen Grundrechte-Folgenabschätzung (Art. 27). Transparenz (Art. 50) und KI-Kompetenz (Art. 4) liegen beim Betreiber. Ob Art. 10 auf eine RAG-Wissensbasis anwendbar ist, ist offen [⚠]. Nach den Rechercheangaben verschiebt der Digital Omnibus (VO (EU) 2026/1744) Pflichten für eigenständige Hochrisiko-Systeme (Anhang III) auf 02.12.2027 [V]. Das Entscheidungsprotokoll (7.1) ist ein **Ereignisprotokoll**, kein Klartextprotokoll aller Ein- und Ausgaben, und lässt sich mit der Löschpflicht vereinbaren.
- **NIS2 (RL (EU) 2022/2555).** Art. 21 Abs. 2 lit. d, h (Lieferkette, Kryptografie) gilt für wesentliche und wichtige Einrichtungen und gelangt über die Lieferkette zum Hersteller. Krypto-Agilität folgt aus Art. 32 DSGVO, EDPB 02/2025 Rn. 85, 96 und BSI-Empfehlungen, nicht aus dem NIS2-Text. Die nationale Umsetzung ist zu prüfen.
- **CRA (VO (EU) 2024/2847).** Meldepflichten ab 11.09.2026, Hauptpflichten ab 11.12.2027; Anwendbarkeit auf den Hersteller prüfen (13.6).

### 20.7 Verifikation der Fundstellen [V]

Jede mit [V] oder ◐/○ markierte Fundstelle ist vor Außenverwendung am Primärtext zu verifizieren (Volltext, Datum, Aktenzeichen, amtliche Fassung). Anhang E führt Prüfstatus. Der Betreiber ergänzt Prüfdatum und Prüfer. Quartalsweise Aktualisierung der Beobachtungsliste: EDPB-Anonymisierungsleitlinien, Endfassung der EDPB-Leitlinien 01/2025, Neufassung SDM-Baustein 60, Digital Omnibus, Rechtsprechung zu Embeddings und Schlüsselvernichtung, Rechtsmittel gegen AG Lörrach. Rechtsfolgen ohne Fundstelle werden nicht in Berichte übernommen.
---

