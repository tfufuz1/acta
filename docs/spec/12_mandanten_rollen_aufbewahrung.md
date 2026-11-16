## 12. Mandanten, Rollen, Aufbewahrung

### 12.1 Mandantentrennung [E]

Je Mandant getrennt: `TenantId`/`TenantOrd`, Tresor (4.3), alle Ordinalräume, drei Ledger (K, E, A), Indexsegmente (K, P, DK, DP, nie gemischt), Gate-Log, Konfiguration, Quoten, Betroffenenverzeichnisse, Schlüsselableitungen (Salz `TenantId`, `K_dedup(t)`, `K_tag(t)`, Journal-Schlüssel). Mandantenübergreifende Abfragen und Deduplikation gibt es nicht. Quoten: Speicher, Anfragen je Minute, Inferenzbudget, Parallelität. Test: `T-TENANT-*` (Eigenschaftstest mit zufälligen Mehrmandantenlasten: kein Lesen, Zählen oder Zeitkanal über Grenzen; getrennte Prozess-Hintergrundlast beeinflusst Latenzklassen nicht über die Quote hinaus).

### 12.2 Sammlungen, Metadaten, Aufbewahrungsklassen [E][R]

**Sammlung:** Name, Standard-Datenklasse, Zugriffszellen-Regel (9.4), Standard-Aufbewahrungsklasse, Sprache, `chunk_cfg`, Quellsystem (`source_system`), `feature_enabled` je Fachfunktion.

**Metadatenschema** (versioniert in K, Whitelist, N-Validator): `doc_id`, `title`, `source_of_record`, `valid_from`/`valid_to`, `owner_unit`, `confidentiality` (`offen`, `intern`, `vertraulich`, `streng`), `acl_labels`, `retention_class`, `language`, `tags`, `norm_refs`. Freitext in Metadaten ist verboten.

**Aufbewahrungsklasse** = Löschregel nach DIN 66398 [V]: Datenart, Frist, Startzeitpunkt, Löschart. `retain_until = Start + Frist` in Kalendertagen (UTC), je Datum berechnet. Beispiele (der Mandant wählt; Fristen vor Verwendung am Primärtext zu prüfen [V]):

|Klasse|Datenart|Frist|Rechtsgrund|
|---|---|---|---|
|`ak-wissen`|Wissen ohne Personenbezug|unbegrenzt|Betriebszweck|
|`ak-buch-10`|Bücher, Jahresabschlüsse|10 Jahre|§ 147 Abs. 1 Nr. 1 AO, § 257 HGB|
|`ak-beleg-8`|Buchungsbelege|8 Jahre seit 01.01.2025 (Banken, Versicherungen weiter 10)|§ 147 Abs. 3 AO, § 257 Abs. 4 HGB|
|`ak-gesch-6`|Handels- und Geschäftsbriefe, auch E-Mails|6 Jahre|§ 147 Abs. 1 Nr. 2, 3 AO, § 257 HGB|
|`ak-patient-10`|ärztliche Dokumentation|10 Jahre nach Behandlungsabschluss|§ 630f Abs. 3 BGB|
|`ak-behoerde`|Verwaltungsakten|nach Aktenplan, Bundes- oder Landesrecht|Landesrecht [V]|
|`ak-sitzung`|Sitzungsdaten|`session_ttl`|Betriebszweck|

Änderungen: Verlängerung frei, Verkürzung nur mit Review und Vermerk im E-Ledger. Ein täglicher Fristenjob löst Sagas fällig gewordener Ziele gebündelt aus (Batch-Vernichtung, 4.3). Jede Sammlung MUSS eine Aufbewahrungsklasse tragen.

### 12.3 Vorgänge (Cases) [E]

Ein **Vorgang** ist die Zweckbindung für P-Inhalte (9.4). Lebenszyklus:

1. **Registrieren** (`case.register`): `external_ref`, Betroffene (`subject_refs`), `owner_unit`, Aufbewahrungsklasse; erzeugt `CaseId`, Vorgangsobjekt in P (5.4).
2. **Zuweisen** (`case.grant`): Nutzer (`ActorRef`) erhalten Zugriff; Quelle ist das Fachverfahren oder `TenantAdmin`; jede Zuweisung steht im A-Ledger.
3. **Befüllen:** Ingest mit `case_id`; neue Versionen erweitern den Fallindex.
4. **Betreiben:** FF2 bis FF5 im Vorgangskontext; Zugriff nur für zugewiesene Nutzer.
5. **Schließen** (`case.close`): Zuweisungen enden; Dokumente bleiben nach Aufbewahrungsklasse (`Restricted(Retention)` nach Antrag, sonst `Live` bis `retain_until`); der Fallindex wird bereinigt (kein aktives Retrieval nach Schließung außer Wiedereröffnung).
6. **Beenden:** Fristablauf löst die Saga für alle Dokumente aus; der Vorgang wird mit dem letzten Dokument vernichtet.

### 12.4 Mandanten-Löschkonzept [E]

Pflichtartefakt je Mandant; `acta-reports` erzeugt den Entwurf. Inhalt (SDM-Baustein 60, DIN 66398 [V]): Datenkategorien und Verarbeitungstätigkeiten; Fristen und Auslöser; Rollen (anordnen, prüfen, ausführen); technisches Verfahren je Speicherort (Acta liefert die Tabelle Ebene × Speicher × Verfahren × Erfüllungsklasse); Backup-Sperrung und Rücksicherungsschutz (`T_BACKUP`, 8.8); Löschprotokoll; Stichprobenkontrolle; Kopienregister; Auftragsverarbeiter; Archivierung; Zusicherungen des Inferenzdienstes (11.2); Notfallplan Algorithmenwechsel (4.7). Der Start verweigert den Betrieb eines Mandanten, dessen Sammlungen keine Aufbewahrungsklasse oder dessen Pflichtparameter keinen Wert haben.

### 12.5 Rollen und Vier-Augen-Aktionen [E]

|Rolle|Rechte|
|---|---|
|`SystemAdmin`|Betrieb, Konfiguration; **kein** Datenzugriff (T3)|
|`TenantAdmin`|Benutzer, Sammlungen, Sichtklassen, Fallzuweisung des Mandanten|
|`Sachbearbeitung`, `Fachabteilung`|Abfragen, Vorgangsarbeit und Wissenspflege im Rahmen der Sichtklasse|
|`DataProtection`|`restricted_read`, Reviews, Betroffenenanfragen, `subjects.discover`|
|`Legal`|`legal_hold`, Aufbewahrungsentscheidungen|
|`Revision`|nur lesend auf Ledger und Berichte|

Das Prinzip „anordnen ≠ ausführen ≠ prüfen“ wird technisch erzwungen: Anordnende und Freigebende sind verschiedene Personen. **Vier-Augen** (zwei verschiedene Personen, signiert, Zeitfenster 72 h) für: `Open`- und `Split`-Freigabe, Takedown, `legal_hold`, Verkürzung der Aufbewahrung, `merge` nach `main`, Review-Entscheidungen nach 8.9, `Waiver` (7.5), `EgressGrant` (13.3), `accept_rollback` (8.8), Aufhebung einer Vergiftung bei `Tenant`/`Node` (15.2).

### 12.6 Mandanten-Lebenszyklus [E]

- **Anlage** (`rotation_lock`): `TenantId`/`TenantOrd`, Tresor, Ledger (`chain_id`, Anker-Köpfe), Standard-Sichtklassen, Löschkonzept-Gerüst, Canary-Betroffener, Pflichtparameter-Abfrage; bis dahin Zustand `Quarantine`.
- **Betrieb:** Kapitel 8 bis 11.
- **Off-Boarding:** (1) Sperre aller Lesepfade (alle Versionen `Restrict(Offboarding)`), (2) Übergabe oder Export nach Betreiberentscheidung (signiert, Kopienregister), (3) Saga `Expired` für alle P-Ziele, (4) Entfernen von K-, DK-, DP-, P-Segmenten (Scrub), Tresor und Exportdateien, (5) Ledger nach Aufbewahrungsvorgabe archivieren oder vernichten, (6) Bericht mit `offboarding_exposure_until` = Ende der letzten Sicherungsepoche, die Schlüssel des Mandanten lebend enthielt, + `T_BACKUP`; danach sind alle Kopien Dritter unlesbar (C27).

### 12.7 Aufbewahrung von Entscheidungen [E]

`decision_retention` und `payload_retention` sind je `DecisionKind` Pflichtparameter **ohne Standardwert**. Empfehlungen, keine Vorgaben: `Query` 90 Tage (Nutzlast 30 Tage), `ScenarioRun` gemäß `branch_retention`, `Review` nach Fachrecht des Vorgangs, `Release` bis zur Änderung der Richtlinie (Nutzlast nach Fachrecht), `Tool` 90 Tage. Nach `decision_retention` entsteht ein `Expired`-Vermerk im K-Ledger, der Eintrag wird nach 8.4 entfernt; nach `payload_retention` wird der Nutzlastschlüssel vernichtet (Art. 5 Abs. 1 lit. e).

---

