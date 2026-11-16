## 2. Grundmodell

### 2.1 Vier Ebenen

```
┌───────────────────────────────────────────────────────────────────────────┐
│ Ebene K – Wissen (versioniert, unveränderlich, ohne Personenbezug)        │
│   Blobs, Trees, Commits, Branches, Tags; Entscheidungsprotokolle (Beleg)  │
│   Verweise auf Personendaten nur als zufällige PersonalRef-Handles        │
├───────────────────────────────────────────────────────────────────────────┤
│ Ebene P – Personendaten (verschlüsselt, löschbar, nicht Teil der Hashes)  │
│   Dokument-Versionen unter Versions-, Dokument- und Betroffenenschlüsseln,│
│   Betroffenenverzeichnis, Handle-Register, Vorgänge                       │
├───────────────────────────────────────────────────────────────────────────┤
│ Ebene D – Ableitungen (Indizes, Caches, Embeddings, Sitzungen)            │
│   Tragen Herkunft (Lineage). DP: aus P, versiegelt, löschbar, je Cluster. │
│   DK: aus K, versiegelt, global je Mandant, ohne Löschlogik               │
├───────────────────────────────────────────────────────────────────────────┤
│ Ebene E – Nachweise (E-Ledger, Gate-Log, Löschregister, Berichte,         │
│   Tresore, Anker-Anbindung, Registersenke)                                │
└───────────────────────────────────────────────────────────────────────────┘
```

**Kerngedanke [E]:** Versionskontrolle und Löschung sind **konstruktiv entkoppelt**. Die Versionsgeschichte (K) enthält nie Inhalte, die auf Personenanfrage gelöscht werden müssen. Sie enthält an deren Stelle ein **PersonalRef**, ein zufälliges 128-Bit-Handle ohne Ableitung aus dem Inhalt. Der Hash eines Trees oder Commits wird über das Handle gebildet, nie über Personendaten. Die Löschung entfernt die Verschlüsselung der Daten in Ebene P und die Ableitungen; das Handle bleibt gültig und löst nach der Löschung zu `Erased` auf.

**Restrisiko des Handles [A] (R-HANDLE):** Ein Handle verbindet K-Stellen und Entscheidungen zu einer Akte oder Person, auch nach der Löschung (Muster aus Zeitpunkt und Häufigkeit). Es ist ohne das entfernte Handle-Register nicht auf eine Person abbildbar. Die Restwirkung wird im Bericht ausgewiesen.

### 2.2 Datenklassen (Regel R-CLS) [E]

Jeder Schreibvorgang MUSS eine Datenklasse deklarieren. Fehlt sie, bricht die Operation mit `ClassViolation` ab.

|Klasse|Ebene|Beschreibung|Aufbewahrung|Löschmechanismus|
|---|---|---|---|---|
|`Wissen`|K|Sachinhalte ohne Personenbezug: Vorschriften, Richtlinien, Handbücher, Vorlagen, Konfigurationen, Prompts|unbegrenzt (versioniert)|Takedown (6.7)|
|`Beleg`|K|Entscheidungsprotokolle ohne Personenbezug|`decision_retention` je Entscheidungsart, **Pflichtparameter ohne Standardwert**|Frist- und Takedown-Löschung (6.7)|
|`Personal`|P|Personenbezogene Daten|nach Aufbewahrungsklasse (12.2)|Saga (8.3)|
|`Besonders`|P|Daten nach Art. 9 und 10 DSGVO|Karenz nach Fristablauf: 0 Tage|Saga mit vorgezogener Schlüsselvernichtung und Rotation auf Anforderung|
|`Abgeleitet`|D|Indizes, Caches, Embeddings, Sitzungen|keine eigene Frist|herkunftsbasierte Bereinigung, Rebuild|
|`Unklassifiziert`|P|Eingang ohne Entscheidung oder ohne Betroffenenzuordnung|Zustand `Quarantine` höchstens `quarantine_max` (24 h), danach `Restricted(Precautionary)`, nach `quarantine_purge_after` (7 Tage) automatisch Saga `Expired`|wie `Personal`|

### 2.3 Dokumente mit gemischtem Inhalt [E]

Der Ingest wählt je Dokument genau einen Modus:

|Modus|Wirkung|Wann|
|---|---|---|
|`Whole` (Standard bei erkanntem oder möglichem Personenbezug)|Das gesamte Dokument liegt in Ebene P. In K entsteht höchstens ein PersonalRef|Akten, Fallbearbeitung, Schreiben an Personen|
|`Split`|Personenbezogene Spannen werden entfernt und in P abgelegt; K enthält den bereinigten Text mit Platzhaltern `⟦P:<handle>⟧`. **Pflicht vor dem Commit:** bestandener N-Validator (7.5), **bestandener Anonymitätstest** (Kontext erlaubt keine Identifizierung; Verfahren und Ergebnis dokumentiert), **Vier-Augen-Freigabe**|nur bei ausdrücklicher Richtlinie `split_allowed` des Mandanten|
|`Open`|Das Dokument ist vollständig `Wissen`|Dokument besitzt bestätigt keinen Personenbezug (Vorschrift, Handbuch); Vier-Augen-Freigabe|

**Entscheidung:** Ein `Split`-Dokument ohne bestandenen Anonymitätstest ist **nicht committbar**; es bleibt in `Whole`. Es gibt keinen Zwischenzustand „pseudonymisiert in K“. `Split`-Blobs tragen `origin=Split`, werden alle `split_retest_interval` (12 Monate) neu getestet und im Takedown vorrangig behandelt. Der Bericht spricht von „unter dokumentierter Methode als anonym bewertet, Restrisiko [⚠]“. Besteht ein Wiederholungstest nicht, löst das Takedown aus (6.7). Maßstab ist der relative Personenbezug (20.1).

**Betroffenenzuordnung [E].** Für jede Klasse `Personal` und `Besonders` MUSS der Ingest die **Betroffenen des Dokuments** kennen: Liste externer Kennungen aus dem Fachverfahren (z. B. Aktenzeichen → Person), geliefert von der integrierenden Schnittstelle oder durch Handentscheidung. `acta-pii` liefert nur Hinweise, nie die Zuordnung. Fehlt die Zuordnung, ist das Dokument `Unklassifiziert`. Eine nachträglich erkannte weitere Person löst `AddSubject` aus (neue Version, neuer Schlüssel, 4.2); bis dahin gilt Z3 für sie nicht und der Bericht weist das aus. Die Datenschutzfunktion kann über `subjects.discover` (18.5) Kandidaten für `AddSubject` finden.

### 2.4 Identifikatoren [E]

|Typ|Größe|Bildung|Sichtbarkeit|
|---|---|---|---|
|`TenantId`|16 B|zufällig bei Anlage|Mandant; nicht geheim; Teil der AAD und Dateinamen|
|`TenantOrd`|u32|dicht, nie wiederverwendet; 0 ist reserviert für Systeminterna|intern|
|`SubjectTag`|16 B|`HMAC-SHA256(K_tag(t), "acta/tag/v1" ‖ kind u8 ‖ NFC(ext_id))[..16]`, `K_tag(t) = HKDF(k_idx, salt = TenantId, info = "acta/tag-key/v1")`; `ext_id` trägt einen Namensraum (z. B. `az:`, `pnr:`, `oidc:<iss>\|<sub>`)|**nur** als versiegelte Tresor-ID (4.3) und im Betroffenenverzeichnis. Nie im Klartext in Segmenten, Journalen, Ledgern, Berichten, Dokumentköpfen|
|`SubjOrd`|u32|dicht je Mandant, nie wiederverwendet|Gate, Sperrmengen, intern|
|`ClusterId`|16 B|`HKDF(SubjectKey, "acta/cluster/v1")[..16]`; **nicht gespeichert**, nur mit `SubjectKey` berechenbar. Bei Dokumenten ohne Betroffenen: `DocRef`|Lokalität in Segmenten (5.1)|
|`DocRef`|16 B|zufällig|stabile Identität eines P-Dokuments über Versionen|
|`VerRef`|16 B|zufällig je Version|Tresor-ID des `DocumentKey` einer Version; Salz der Ableitungen|
|`VerOrd`|u32|dicht je Mandant, nie wiederverwendet; Überlauf → `Poisoned(Node)`|Lineage, Gate, intern|
|`PersonalRef`|16 B|zufällig, getrennt von `DocRef`|Verweis aus K; Auflösung nur über das Handle-Register|
|`CaseId`|16 B|zufällig|Vorgang (12.3)|
|`ActorRef`|16 B|zufällig je Person, Mandant und Epoche (`actor_handle_epoch`)|Handle auf Identität in P|
|`KOrd`|u64|dicht, monoton **je Mandant**, nie wiederverwendet|Sichtbarkeitsmengen, DK-Lineage, intern|
|`ObjectId`|32 B|SHA-256 über kanonische Bytes mit Domänenpräfix (6.1)|Objekte der Ebene K|
|`ErasureId`|16 B|UUIDv7|Löschvorgang|
|`IndexGen`|16 B|`SHA-256(chunk_cfg ‖ embedder_fingerprint ‖ norm_variant)[..16]`|Index-Generation (9.2)|

`k_idx` ist ein 256-Bit-Geheimnis **ohne Standardwert**, geliefert vom Schlüsselanker (4.4). Ein Start ohne `k_idx` wird verweigert. **Verlust von `k_idx` macht Betroffene nicht mehr auffindbar** (Auskunft, Löschung); `k_idx` ist daher Teil der Ankersicherung (14.5). `k_idx` wird in Version 1 nicht rotiert; wer `k_idx` und eine externe Kennung besitzt, kann prüfen, ob eine Person erfasst war (Restrisiko ADR-15, ausgewiesen).

**Ordinal-Hochwassermarken.** Nach einer Wiederherstellung werden alle Ordinalzähler (`SubjOrd`, `VerOrd`, `KOrd`, `TenantOrd`) auf `max(Stand der Sicherung, Hochwassermarke der Registersenke) + 1` gesetzt (8.8), damit nie ein bereits gesperrtes Ordinal erneut vergeben wird.

### 2.5 Bedrohungsmodell [E]

|ID|Angreifer oder Ereignis|Zusicherung von Acta|Außerhalb der Zusage|
|---|---|---|---|
|T1|Netzangreifer ohne Konto|Authentifizierung, Ratenbegrenzung, kein Egress|Schwachstellen der TLS-Umgebung des Betreibers|
|T2|Nutzer mit Konto|Zugriffskontrolle vor der Suche (Z16), Mandantentrennung, Vorgangsbindung von P|Weitergabe ausgegebener Texte durch den Nutzer (A-EXT)|
|T3|Administrator **ohne** Ankerzugriff|kein Klartext in P über die API; Dateien nur als Chiffrat|Metadaten (Größenklassen, Zeitpunkte, Ordinale), `ClusterId`-Gruppierung solange `SubjectKey` lebt (R-CLUSTER)|
|T4|Besitzer eines Datenträgerabbilds oder einer Sicherung|alles Persistierte ist AEAD-versiegelt; Tresorabbilder werden mit der Rotation unlesbar; Exporte mit Ablauf der Sicherungsepoche|Abbild **zusammen mit** Anker (T5)|
|T5|Besitzer eines alten Abbilds **und** späterer Ankerzugriff|vernichtete Schlüssel sind nach Rotation der Generation und Ablauf der betroffenen Sicherungsepochen unlesbar (4.3)|Schlüssel, die vor Vernichtung und Rotation kopiert wurden|
|T6|Bösartiges Dokument (Parser-Angriff)|Parser laufen in einem Prozess ohne Schlüssel, Netz und Dateisystem (13.2)|Fehler im Betriebssystemkern|
|T7|Bösartiger Inhalt (Prompt-Injection)|Belege sind Daten; Ausgabeprüfung; keine Werkzeuge; keine aktiven Inhalte (9.4)|Fehlverhalten des Modells (A-INFER)|
|T8|Administrator **mit** Root auf dem Host|Erschwerung (`mlock`, kein Core-Dump, kein `ptrace`), **keine Zusicherung**|Root-Zugriff auf Prozessspeicher (A-RAM); Gegenmaßnahme organisatorisch oder durch Vertrauensumgebungen|
|T9|Langfristige Kryptoschwäche|Algorithmus-Byte, Neubewertung, Migration (4.7)|Bruch vor Migration (A-CRYPTO)|
|T10|Inferenzdienst|Gehört zum Vertrauensbereich (A-INFER); Pflichtkonfiguration „keine Protokollierung von Inhalten, kein persistenter Prompt-Cache“; `ContextPurge` (9.5); sonst als extern behandelt (13.3)|Verhalten des Dienstes|

---

