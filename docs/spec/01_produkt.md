## 1. Produkt

### 1.1 Zweck

Acta ist eine eigenständige Software für **Behörden und mittelständische Unternehmen**. Sie bringt lokale Sprachmodelle an den Arbeitsplatz, speist sie mit belegtem Wissen und liefert Sachbearbeitung, Fachabteilungen, Verwaltung und Organisation Entscheidungshilfen und Werkzeuge. Fünf Eigenschaften machen Acta aus:

1. **Belegtes Antworten (RAG).** Jede inhaltliche Aussage trägt einen Quellenverweis auf einen tatsächlich abgerufenen Beleg. Fehlt der Beleg, enthält sich das System („nicht belegt“).
2. **Versionskontrolle für Wissen („Git für RAG“).** Der gesamte nicht personenbezogene Wissensbestand samt Richtlinien, Prompts, Chunking-, Abruf- und Modellkonfiguration, Schwellenwerten und Zugriffsrichtlinie ist unveränderlich versioniert, verzweigbar und zu jedem früheren Stand abfragbar. Alte Entscheidungen sind nachvollziehbar, „Wenn-Dann“-Szenarien laufen auf Verzweigungen des Wissens.
3. **Entkoppeltes, atomares und feingranulares Löschen personenbezogener Daten.** Personenbezogene Daten liegen getrennt vom versionierten Wissen, verschlüsselt, einzeln pro Dokument-Version und Betroffenem adressierbar. Löschung geschieht durch sofortige Lesesperre, physische Bereinigung aller Klartext-Strukturen und Ableitungen, Schlüsselvernichtung mit Rotation und Sicherungsfenster, jeweils gemessen. Die Versionsgeschichte des Wissens bleibt dabei unverändert. Acta stützt sich **nie allein** auf Schlüsselvernichtung (20.2).
4. **Rechtskonforme Datenversorgung des Modells.** Das Modell sieht ausschließlich Belege, auf die der Nutzer im konkreten Vorgang zugreifen darf. Personenbezogene Inhalte sind nur im ausdrücklichen Vorgangskontext abrufbar (Zweckbindung, Datenminimierung), nie im Freitext-Korpus.
5. **Entscheidungsnachvollziehbarkeit.** Zu jeder Antwort existiert ein Entscheidungsprotokoll (Wissensstand, Prompt, Modell, Parameter, Eingabe, Belege). Die Begründungskette überdauert die Löschung von Personen.

### 1.2 Einsatzrahmen [E]

|Merkmal|Festlegung|
|---|---|
|Betreiber|Behörden, Kommunen, Mittelstand; On-Premise oder vom Kunden kontrollierte EU-Umgebung|
|Mandanten|Mehrere Organisationseinheiten je Installation, strikt getrennt bis hinunter zu Schlüsseln, Ordinalräumen, Ledgern, Tresoren und Indizes (12.1)|
|Nutzer|Sachbearbeitung, Fachabteilungen, Datenschutzfunktion, Recht, Administration, Revision|
|Inferenz|Lokal oder in vom Kunden kontrollierter Infrastruktur als **separater Prozess**; keine Inferenz im Kernprozess (11); externe Dienste nur durch ausdrückliche Freigabe (13.3)|
|Plattform|Linux x86_64 und aarch64; **ein Knoten je Installation**; Wiederherstellung kalt (14.6)|
|Netz|Betrieb ohne ausgehende Verbindungen ist Standard und voll funktionsfähig|
|Rechtsrahmen|DSGVO; für öffentliche Stellen des Bundes das BDSG, der Länder zusätzlich Landesdatenschutz- und Archivrecht. Dessen Abweichungen (Löschung, Anbietungspflicht an Archive) sind im Löschkonzept des Mandanten abzubilden (12.4) [R]|
|Sprachen|Deutsch und Englisch voll unterstützt (Normalisierung, Stemming, Goldsets); weitere Sprachen über `retrieval_cfg` ohne Zusage zur Güte|

### 1.3 Mission und Umsetzung

|Säule|Umsetzung|Kapitel|
|---|---|---|
|Souveränes Unternehmenswissen|On-Premise, Schlüssel beim Kunden, kein Egress im Standard, lokale Inferenz, kein Herstellerzugriff|4, 11, 13, 14|
|Transparenz und Nachvollziehbarkeit|Versionierung, Entscheidungsprotokoll, Hash-Ketten je Mandant, unabhängiger Prüfer|6, 7, 17|
|Präzision und Kontrolle|Belegpflicht, Hybridsuche mit Reranking, Zugriffskontrolle vor der Suche, Messbarkeit|9, 16|
|Effizienz und Ressourcenschonung|Segmentierte Indizes, Datei-Drop statt Umschreiben, Batch-Vernichtung, kleine Binärdatei, Festkomma-Ranking|3, 5, 9|
|Schutz personenbezogener Daten|Ebene P, Schlüsselhierarchie, Lesesperre, versiegelte Ableitungen, Mehrbetroffenen-Prüfung|2, 4, 8|
|Verwaltung und Organisation|Mandanten, Sammlungen, Metadatenschema, Aufbewahrungsklassen, Löschkonzept, Vorgänge|12|
|Entscheidungs- und Verwaltungshilfe|Vorgangsassistent, Prüflisten, Entwürfe, Fristen, Änderungsradar, Szenarien, menschliche Freigabe|7, 10|
|Europäischer Datenschutz|Abläufe zu Art. 5, 12 bis 19, 25, 30, 32 DSGVO; Unterstützung für Art. 28, 33, 35; Berichte mit Annahmen|8, 18, 20|

### 1.4 Nicht-Ziele (Version 1) [E]

- Kein Dokumentenmanagementsystem für Originale und keine Langzeitarchivierung. Acta hält Kopien und Repräsentationen; das Original bleibt im Quellsystem (`source_of_record`, 9.1).
- Keine Mehrknoten-Replikation und kein Warm-Standby des Tresors (14.6).
- Keine autonomen Agenten und keine Werkzeugausführung durch das Sprachmodell. Deterministische Werkzeuge (Kapitel 10) werden von der Anwendung aufgerufen, nie vom Modell.
- **Keine automatisierte Einzelentscheidung** im Sinne des Art. 22 DSGVO. Antworten sind Entscheidungshilfe; die Entscheidung trifft ein Mensch (Entscheidungsart `Review`, 7.1).
- Keine grafische Oberfläche als Teil dieser Spezifikation; Referenzoberflächen sind eigene Projekte über die API.
- Keine Zertifizierungsaussage (1.6) und keine Rechtsfeststellung (20).
- Kein Zusammenführen zweier Betroffener nachträglich (8.2): jede Kennung wird einzeln gelöscht.

### 1.5 Normative Zusagen

Jede Zusage MUSS durch automatisierte Tests belegt sein; eine Zusage ohne Test gilt als nicht erfüllt (Gate G-ZUSAGE). **Deterministisch (D)**: der Test entscheidet vollständig. **Statistisch (S)**: ein Gütewert wird gemessen (nur Z17 in Teilen).

|ID|Zusage|Nachweis|
|---|---|---|
|**Z1**|**Sofortige Lesesperre (D).** Nachdem eine Löschung im Zustand `Gated` an den Aufrufer zurückgekehrt ist, liefert kein Standard-Lesepfad Klartext des Ziels: nicht Suche, Antwort, Zeitreise, Szenario, Cache, Export, Vorschau oder Sitzung, auch nicht ältere Versionen (`Superseded`). Schreibzugriffe auf das Ziel werden verweigert. Für `Restricted` gilt dasselbe mit der einzigen Ausnahme `restricted_read` (8.1). Für Takedown in Ebene K gilt die Sperre analog (`RedactedK`, 5.3).|`T-GATE-*`|
|**Z2**|**Kryptografische Isolation (D).** Jeder persistierte Klartext personenbezogener Daten und jede daraus abgeleitete Struktur liegt vor dem Schreiben in einem AEAD-Umschlag unter Schlüsseln, die nach der Vernichtung nicht mehr existieren. Persistierte Metadaten tragen **keine dauerhaften deterministischen Pseudonyme** einer Person (2.4) und keine exakten Längen (4.5).|`T-CRY-*`, `T-DERIVED-SEALED`, `T-META-*`|
|**Z3**|**Mehrbetroffenen-Behandlung (D).** Betrifft ein Dokument mehrere Betroffene, wird es im Standardmodus `DeleteAll` für alle unlesbar, sobald einer von ihnen gelöscht wird. Ein automatischer **Trockenlauf** innerhalb `erase_return_timeout` prüft die Mitbetroffenen auf Aufbewahrungspflicht, `legal_hold`, Einwand und offenen Vorgang. Konfliktdokumente sind **sofort** `Restricted(MultiSubjectReview)`; es gibt nie ein automatisches `DeleteAll` nach Fristablauf. Ein Review entscheidet zwischen `DeleteAll`, `RedactSplit` und `RestrictShared` (8.9). Anzahl und Modus der Mitbetroffenen stehen im Bericht, nie ihre Kennungen.|`T-MULTI-*`|
|**Z4**|**Crash-Invarianz (D).** Ein Absturz an jedem Durabilitätspunkt (Anhang C) führt nach dem Neustart entweder zur Vollendung der Löschung oder zu `Pending` bei dauerhaft aktiver Sperre. Eine durable Sperre geht nie verloren; ein Schlüssel geht nie ohne Absicht verloren. Physische Tatsachen (Schlüssel vernichtet, Segment ersetzt) haben Vorrang vor dem Journalstand.|`T-CRASH-*`|
|**Z5**|**Trennung von Wissen und Personendaten (D).** Die versionierte Wissensebene K enthält keine personenbezogenen Daten. Verweise auf Personendaten sind zufällige Handles ohne Inhaltsbezug. Ein Schreibversuch mit erkanntem Personenbezug scheitert (Fail-Closed); die Prüfung ist typerzwungen (`Validated<T>`, 7.5). Inhalte im Modus `Split` sind nur mit bestandenem Anonymitätstest, Validator-Prüfung und Vier-Augen-Freigabe committbar; ohne bestandenen Test bleibt das Dokument in `Whole` (2.3).|`T-SEP-*`, `T-ANON-*`|
|**Z6**|**Löschung in P berührt die Versionskontrolle nicht (D).** Eine Löschung in Ebene P ändert weder Objekt-IDs noch Commit-Hashes noch bestehende Einträge des K-Ledgers. Löschnachweise liegen im E-Ledger (6.2). (Takedown und Fristlöschung **in K** sind eigene Operationen mit Vermerk im K-Ledger, 6.7.)|`T-VCS-ERASE-*`|
|**Z7**|**Zeitreise-Determinismus (D).** Abfragen der Wissensebene zu einem Stand (Inhalte, Kandidatenmenge, Rangfolge der Textsuche und exakten Vektorsuche) liefern bei gleichen Parametern, gleichem Abfragevektor, gleicher Sichtklasse und gleicher Index-Generation byte-identische Ergebnisse, auch zwischen x86_64 und aarch64. Statistiken sind commit- und sichtklassenrelativ (9.2). Objekte mit Vermerk `Redacted` oder `Expired` liefern einen Platzhalter. Modellausgaben sind ausgenommen (A-INFER).|`T-TT-*`|
|**Z8**|**Entscheidungsnachvollziehbarkeit (D).** Zu jeder Antwort existiert ein Entscheidungsprotokoll mit Wissensstand, Prompt-Version, Modellstand, Parametern, Eingabe und Belegen. Nach Löschung einer Person bleibt die Begründungskette erhalten; personenbezogene Eingaben und Antworten liefern `Erased`.|`T-DEC-*`|
|**Z9**|**Szenario-Isolation (D).** Ein Szenario verändert weder die Hauptlinie des Wissens noch Ebene P noch Zugriffsrechte.|`T-SCEN-*`|
|**Z10**|**Fail-Closed-Kryptografie (D).** Kein Fehler im Schlüssel-, Zufalls-, Ableitungs- oder Siegelpfad führt zu Ersatzwert, Fallback-Schlüssel oder Klartext. Es gibt keine Standardschlüssel. Ein **fehlender** Schlüssel ohne Gate-Eintrag oder Registerbeleg ist ein Integritätsfehler (Vergiftung), nicht `Erased`.|`T-NOFALLBACK-*`, `T-RNG-*`|
|**Z11**|**Nachweis nur mit Positivkontrolle (D).** Ein Löschnachweis entsteht ausschließlich aus einer Messung, deren Messverfahren seine Funktionsfähigkeit an einem synthetischen Treffer bewiesen hat. Nicht implementierte Prüfungen liefern `NotImplemented`, nie `reached`. Stichproben werden als solche ausgewiesen.|`T-EVID-*`|
|**Z12**|**Restore-Schutz (D, bedingt).** Das Einspielen einer alten Sicherung kann gelöschte Daten nicht wiederbeleben, **sofern** das Löschregister unabhängig von der Sicherung verfügbar ist (Registersenke oder Ankerstand, 8.8) und `commit_key_retention` den Restore-Horizont abdeckt. Andernfalls startet der Knoten in `Quarantine` und liefert keine Daten.|`T-REST-*`|
|**Z13**|**Herkunftszwang (D).** Jede abgeleitete Struktur trägt ihre Herkunft (Lineage); Ableitungen ohne Herkunft sind nicht konstruierbar.|Compile-Fail-Tests|
|**Z14**|**Schlüsselisolation je Dokument-Version (D).** Zwei Dokument-Versionen der Ebene P teilen niemals denselben Datenschlüssel.|Property-Test|
|**Z15**|**Kennungsarme Protokolle (D).** Protokolle, Ledger, Metriken und Fehlertexte enthalten weder Schlüsselmaterial noch Roh-Kennungen von Personen noch dauerhafte deterministische Pseudonyme. Zulässig sind ausschließlich epochengeschlüsselte Commitments (6.2), die nach Ablauf von `commit_key_retention` anonymisiert werden, sowie zufällige Handles.|`T-LOG-*`|
|**Z16**|**Zugriffskontrolle vor der Suche (D).** Ein Nutzer erhält nie Treffer, Belege, Texte, Zählwerte, Quellennamen, Rangverschiebungen oder unterscheidbare Fehlerbilder zu Dokumenten, auf die er keinen Zugriff hat; die Prüfung wirkt auf die Kandidatenmenge und die Statistik, nicht erst auf die Ausgabe.|`T-ACL-*`|
|**Z17**|**Belegpflicht (D/S).** Jede inhaltliche Aussage einer Antwort trägt einen Verweis auf einen in dieser Anfrage abgerufenen Beleg, oder die Antwort besteht aus der ausdrücklichen Enthaltung „nicht belegt“ (strukturell, D). Die Güte der Stützungsprüfung wird gemessen (S, 9.6).|`T-GROUND-*`|
|**Z18**|**Kein Egress im Standard (D).** Ohne ausdrückliche, protokollierte, vier-Augen-freigegebene Freigabe verlässt kein Inhalt die Installation.|`T-EGRESS-*`|
|**Z19**|**Anbieter ohne Zugriff (D).** Die Software enthält keine Telemetrie, keinen Fernzugriff, keine Lizenzprüfung mit Nutzerdaten, keine Schlüsselhinterlegung (Escrow) und keinen Wiederherstellungsweg über den Hersteller. Supportpakete werden lokal erzeugt, enthalten nachweislich keine Inhalte und werden vom Betreiber freigegeben (14.7).|`T-VENDOR-*`|

### 1.6 Sprachregeln und Berichtsgrenzen [R]

**Verbotene Formulierungen** in Berichten, Oberflächen, Dokumentation, Werbematerial, Quelltext und dieser Spezifikation (Liste A, exakte Wortformen, Groß-/Kleinschreibung ignoriert): „DSGVO-konform“, „DSGVO-zertifiziert“, „erfüllt Art. 17 automatisch“, „100 % gelöscht“, „garantiert gelöscht“, „garantiert vernichtet“, „unwiederbringlich“, „vergisst Daten“, „revisionssicher“, „fehlerfrei“, „löscht aus allen Backups“, „perfekte Antworten“, sowie jede Behauptung einer Zertifizierung oder behördlichen Billigung.
**Regel B** (nur für Berichtsvorlagen, Oberflächentexte, Exportvorlagen): Das Wort „gelöscht“ steht nur zusammen mit der erreichten Erfüllungsklasse („Klasse K erreicht“).

Konformität ist ein Betriebszustand der Organisation, keine Eigenschaft einer Software. Acta berichtet ausschließlich **messbare Erfüllungsklassen** (8.7) mit `reached` oder `not_reached` und weist die **Annahmen** A-FLASH, A-RAM, A-ANCHOR, A-CRYPTO, A-EXT, A-INFER, A-CLOCK, A-DETECT, A-ARCH, A-FS (20.3) aus. Das Gate G-WORD prüft Liste A über Quelltext, Texte, Vorlagen und diese Spezifikation; Regel B über Vorlagen und Oberflächentexte. Der Abschnitt 1.6 selbst ist durch `g-word:ignore` ausgenommen. Rechtsgrundlage: § 5 UWG (Irreführung), Sachmangelrecht (§ 434 BGB) [R][V].

### 1.7 Begriffe

|Begriff|Bedeutung|
|---|---|
|Ebene K / P / D / E|Wissen (versioniert) / Personendaten (löschbar) / Ableitungen (Indizes, Caches, Embeddings) / Nachweise (Ledger, Gate-Log, Berichte, Tresore)|
|Betroffener (`Subject`)|Natürliche Person, deren Daten verarbeitet werden, einschließlich Mitarbeitenden (Nutzer sind Betroffene)|
|Version (`Ver`)|Unveränderlicher Stand eines P-Dokuments; kleinste Einheit für Schlüssel, Gate und Ordinal (`VerRef`, `VerOrd`)|
|Vorgang (`Case`)|Fachliche Klammer (Akte) in P, die Dokumente und Zugriffsrechte bündelt; Voraussetzung für Abruf von P-Inhalten (9.4)|
|Gate|O(1)-Sperrstruktur, die jeder Lesepfad vor jeder Entschlüsselung befragt (5.3)|
|Saga|Dauerhafter, idempotenter Löschablauf S0 bis S9 (8.3)|
|Klassen G, B, K, M|Erfüllungsklassen: gesperrt, bereinigt, Schlüssel vernichtet, vollständig gemessen (8.7)|
|Handle|Zufälliger 128-Bit-Verweis ohne Inhaltsbezug (`PersonalRef`, `ActorRef`)|
|Ordinal|Dichte interne Nummer (`SubjOrd`, `VerOrd`, `KOrd`), nie wiederverwendet; nur intern sichtbar|
|Lineage / `source_set`|Menge der Ordinale, aus denen eine Ableitung entstanden ist (5.2)|
|Sichtklasse|Menge von Nutzern mit identischer effektiver Zugriffsmenge; teilt Statistiken und Caches (9.2)|
|Anker (`KeyAnchor`)|HSM, KMS oder Datei-Anker, der Generationsgeheimnisse hält und vernichtet (4.4)|
|Senke (`RegistrySink`)|Unabhängiger Ausfallbereich, in den Gate-Log und E-Ledger gespiegelt werden (6.2)|
|Canary|Synthetischer Betroffener zur Positivkontrolle von Messungen (8.5)|
|Index-Generation (`IndexGen`)|Tripel aus Chunking-Konfiguration, Embedder-Stand und Normalisierungsvariante; Grundlage eines Indexsatzes (9.2, 11.4)|
---

