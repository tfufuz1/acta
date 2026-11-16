## 14. Betrieb

### 14.1 Start und Selbsttest [E]

Der Knoten öffnet die Schnittstelle erst nach dieser Reihenfolge; ein Fehler führt zu `Quarantine` oder `Poisoned` (15.2), nie zu einem Teilbetrieb ohne Hinweis:

1. Konfiguration validieren (Pflichtparameter, Invarianten 15.4).
2. Hostprüfungen (13.1).
3. Anker öffnen: `assurance`, `current_generation`, `min_generation`; Vergleich mit dem Tresor (Rotation fortsetzen, falls `anchor.current_generation() > vault.generation`).
4. Tresore öffnen (Scan, RAM-Index); Schwanz der Rotation anwenden (R-KEY-FIRST).
5. Ledger prüfen (Kette, Checkpoints) und Kopf gegen `last_head` des Ankers (`RestoreBehindAnchor`).
6. Gate-Log laden; **Abgleich physischer Tatsachen** (Tresorzustände, Manifest) mit dem Journal; offene Sagas wieder aufnehmen (Z4).
7. Manifest prüfen; verwaiste Dateien bereinigen (9.1); verwaiste Schlüssel (C01) nach Karenz vernichten.
8. Registersenke abgleichen (`fetch_since`); fehlt sie, `Degraded(Sink)` (nur zulässig, wenn der Ankerstand vorhanden ist).
9. **Positivkontroll-Selbsttest:** Ein synthetischer Betroffener wird angelegt, gesperrt, sein Schlüssel vernichtet und die Messung gegen den Canary durchlaufen. Scheitert das, ist der Löschpfad defekt und der Knoten geht in `Quarantine`.

### 14.2 Hintergrundaufgaben

|Aufgabe|Takt|Bemerkung|
|---|---|---|
|Wurzelrotation|`T_ROT` 12 h, `N_SHRED`, auf Anforderung|4.3|
|Tresorexport|`vault_export_interval` 1 h|4.3, 14.4|
|Datensicherung|`data_backup_interval` 24 h|14.4|
|Fristenjob|täglich|12.2|
|Quarantäne-Sweeper|stündlich|2.2|
|Epochenverwaltung|laufend|`CommitKey`, `M_bk(e)`, `LogKey`|
|Sicherungsepochen vernichten|zeitgesteuert|S7 in 8.3|
|Senkenspiegelung|laufend, asynchron|6.2|
|Checkpoints|alle 1 024 Einträge|6.2|
|`Rewrap`|nach Bedarf|4.7|
|Ausgangskorb|laufend|8.6|
|Garbage Collection|täglich|6.7|
|Gate-Log-Verdichtung|wöchentlich|5.3|
|A-Ledger-Pruning|monatlich|6.2|
|Wiederholungstest `Split`|`split_retest_interval`|2.3|
|Kryptobericht|`crypto_review_interval`|4.7|
|Canary-Selbsttest|täglich|14.1 Nr. 9|
|Goldset-Drift|`goldset_interval` 7 Tage|11.4|
|Indexaufbau, Reindex|bei Bedarf|11.4|

### 14.3 Metriken und Alarme [E]

Metriken ohne Kennungen (13.5): Gate-Latenz, Alter der `Pending`-Sagas, Verbrauch von `erase_sla_B`, Rotationsverzug, Exportalter, Senken-Rückstand, Vergiftungszustand, Messfehler (`Invalid`), Ausgangskorb-Alter, Speicher, Quoten, Suchlatenz, Enthaltungsquote, Modelllatenz. Alarme (Mindestumfang): `SlaBreach`, `RotationOverdue`, `ExportLate` (Export älter als `2 × vault_export_interval`), `BackupOverdue` (**kein** gültiger Export innerhalb `T_BACKUP`), `SinkLag`, `MeasureInvalid`, `SagaStuck`, `ReviewOverdue`, `Poisoned`, `RequestDue` (70 % der Frist), `QualityDrift`, `GroundingViolation`.

### 14.4 Sicherung und Wiederherstellung [E]

Drei Teile, die zusammen die Wiederherstellung tragen: (1) **Datensicherung** der unveränderlichen Segmente (K, DK, P), des Manifests, Gate-Log-Snapshots, Ledger und Verzeichnisse (nur Chiffrate; Konsistenzpunkt durch Manifest-Schnappschuss und Journalrotation; **ohne** DP-Segmente, ohne Caches, ohne Tempdateien, ohne Tresordateien); (2) **Tresorexport** nach 4.3 (Epochenschlüssel, begrenzte Gültigkeit); (3) **Ankersicherung** (14.5). Die Registersenke (6.2) ist **nicht** Teil der Sicherung, sondern ein eigener Ausfallbereich. Ohne gültigen Export ist eine Datensicherung unlesbar; die Aufbewahrung von Datensicherungen darf `T_BACKUP` nicht überschreiten (R-BK). **Restore-Tests** sind quartalsweise verpflichtend (`T-REST` auf Produktionsumgebung mit Testmandant). Die Wiederherstellung folgt 8.8; Ableitungen DP werden danach neu aufgebaut (Dauer proportional zum P-Bestand, `RTO`-relevant).

### 14.5 Ankersicherung [E]

`k_idx` und die Anker-Köpfe sind Teil der Ankersicherung; sie werden nicht rotiert und getrennt von den Datenschlüsseln gesichert. Verlust von `k_idx`: Betroffene sind nicht mehr auffindbar (Auskunft und Löschung über `SubjectTag` unmöglich). Verlust von `M_g`: die Daten in Ebene P sind unlesbar (so gewollt, daher Hochverfügbarkeit des Ankers). Beim `FileAnchor` MUSS die Passphrase auf mindestens zwei Personen verteilt sein (Wissensaufteilung) und DARF nirgends gespeichert werden; bei HSM und KMS gelten die Sicherungsverfahren des Herstellers des Geräts bzw. Dienstes. **Kein Wiederherstellungsweg über den Acta-Hersteller** (Z19).

### 14.6 Verfügbarkeit [E]

Ein Knoten je Installation; Wiederherstellung ist **kalt**. **[Q]** Startwerte: RTO 4 Stunden (ohne DP-Neuaufbau), RPO für Schlüssel höchstens `vault_export_interval` (1 Stunde), RPO für Daten höchstens `data_backup_interval` (24 Stunden; Segmente und Journal enthalten nur Chiffrate und dürfen laufend auf ein zweites Medium gespiegelt werden, was den RPO senkt). Ein Dokument, dessen Schlüssel nach dem letzten Export entstand, ist nach einem Tresorverlust unlesbar.

|Betriebszustand|Auslöser|Wirkung|
|---|---|---|
|`Normal`|–|alles aktiv|
|`Degraded(Anchor)`|Anker nicht erreichbar|Lesen und Schreiben laufen (Schlüssel im RAM); keine Rotation, kein Export; Klasse K bleibt `not_reached`, Sagas `Pending`|
|`Degraded(Sink)`|Senke nicht erreichbar|Sperren wirken unverändert (Spiegelung blockiert nie); `registry_mirrored = false`, Klasse M `not_reached`, Alarm `SinkLag`|
|`Degraded(Inference)`|Inferenzdienst nicht erreichbar|Suche läuft, Antworten enthalten sich (`Unavailable`)|
|`Restoring`|Wiederherstellung läuft|alle Lesepfade geschlossen (8.8)|
|`Quarantine`|Registerbeleg fehlt, Selbsttest fehlgeschlagen, Ordinalkonflikt, `RestoreBehindAnchor`|keine Daten|
|`Poisoned(Scope)`|Integritäts- oder Kryptofehler|Bereich geschlossen (15.2)|

### 14.7 Anbieterneutralität [E] (Z19)

Keine Telemetrie, kein Fernzugriff, keine Lizenzprüfung mit Nutzerdaten, kein Escrow. **Lizenz:** offline prüfbare signierte Datei ohne Nutzerdaten. **Updates:** signierte Pakete, offline einspielbar. **Support:** `acta-cli support-bundle` erzeugt lokal ein Paket aus einer Positivliste (Version, Konfiguration ohne Geheimnisse, Aggregatmetriken, Fehlercodes, Hostprüfungen, Größenklassen). Test `T-VENDOR-BUNDLE`: ein Canary mit synthetischen Inhalten und Kennungen darf im Paket nicht auftauchen. Der Betreiber gibt das Paket frei und trägt es selbst weiter (Kopienregister). **Einordnung [⚠]:** Bei diesem Zuschnitt ist der Hersteller für Daten der Betreiber in der Regel weder Auftragsverarbeiter noch Verantwortlicher; das ist eine Einschätzung, keine Feststellung (20.1). Sie entfällt bei Fernwartung, Telemetrie, Hosting, Escrow, Lizenzdiensten mit Nutzerdaten oder Weiterleitung an externe Dienste.

### 14.8 Upgrade und Formatmigration [E]

Plattenformate tragen `format_version`; Migrationen laufen nur vorwärts, nach Sicherung und mit Golden-Tests je Version; ein Downgrade wird nicht unterstützt. Algorithmenwechsel nutzen `Rewrap` (4.7). Ein Upgrade ist ein kurzes Wartungsfenster (ein Knoten); der Server fährt geordnet herunter (Anfragen abschließen, Journal flushen, Gate-Log synchronisieren, Schlüsselspeicher freigeben). Nach dem Upgrade läuft der Selbsttest (14.1); ein Fehler stellt den Knoten auf `Quarantine` und verlangt die Wiederherstellung der Vorversion aus der Sicherung.

---

