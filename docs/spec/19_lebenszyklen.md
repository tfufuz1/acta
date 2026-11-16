## 19. Lebenszyklen

Jede Funktion ist vollständig beschrieben: Auslöser, Vorbedingungen, Schritte, Nachbedingungen, Fehlerpfade, Absturzverhalten, Nachweis. `C-nn` verweist auf Anhang C. Eine Funktion ohne Eintrag hier ist nicht spezifiziert und DARF NICHT implementiert werden (0.4).

### L01 Ingest in Ebene P [E]
- **Auslöser:** `ingest` mit Klasse `Personal` oder `Besonders`.
- **Vorbedingungen:** Rolle erlaubt; Mandant `Live`; Pflichtfelder (9.1 Stufe 1); `subject_refs` vorhanden, sonst `Unklassifiziert` (L01-Q).
- **Schritte:** 9.1 Stufen 1 bis 8. Parsing im Worker; Normalisierung; Erkennung; Zuordnung (Schlüssel zuerst, WP-1); Chunking; Embedding; `DocumentKey(v)` und DEK; P-Umschläge; Dokumentkopf; DP-Segmente mit `Derived<T>`; Betroffenenverzeichnis, Handle-Register, Fallindex-Mitgliedschaft; Idempotenzeintrag; **zuletzt** Manifest-Batch (Sichtbarkeit).
- **Nachbedingungen:** Version `Live`; Ableitungen in DP mit Lineage; `VerOrd` vergeben; Ergebnis unter `ingest_id` abgelegt.
- **Fehler:** `Invalid(ResourceLimit)`, `ClassViolation`, Worker-Absturz (Aufruf scheitert, nichts geschrieben), Entropiefehler (kein Umschlag), `Transient(Overloaded)`.
- **Absturz:** C01 bis C04; Wiederholung mit gleicher `ingest_id` liefert dasselbe Ergebnis.
- **Nachweis:** `T-CRY-1`, `T-DERIVED-SEALED`, `T-CRASH-*`.
- **L01-Q Quarantäne:** Ohne Zuordnung: `Quarantine`, nicht lesbar; Sweeper stündlich; nach `quarantine_max` `Restricted(Precautionary)`; nach `quarantine_purge_after` Saga `Expired` (nur die Acta-Kopie, 8.4); Zuordnung innerhalb der Frist macht die Version `Live`.

### L02 Wissenspflege in Ebene K [E]
- **Auslöser:** `ingest` Klasse `Wissen`, `commit`, `merge`, `document.retire`, FF9.
- **Vorbedingungen:** `Validated<Tree>` (N-Validator bestanden); bei `Open` und `Split` Vier-Augen-Freigabe, bei `Split` Anonymitätstest; `source_of_record`; Branch-Regeln.
- **Schritte:** Blobs mit Schlüssel-Salz, Trees, Commit; K-Ledger-Eintrag; Ref-Bewegung (WP-2); Sichtbarkeitsmenge `V(commit)` (Delta); DK-Indexsegmente für neue Blobs; bei Konfigurationsänderung neue `IndexGen` im Hintergrund (L13). Nach `main`: Vier-Augen und, wenn aktiv, Regressionsprüfung (9.7).
- **Nachbedingungen:** Commit unveränderlich; `main` zeigt erst nach vollständigem Indexaufbau auf neue `IndexGen`.
- **Fehler:** Validator-Treffer (`Policy`, Befundcodes ohne Inhalte), Merge-Konflikt, Regression über Grenze, `Waiver` fehlt.
- **Absturz:** C05, C06; unerreichbare Objekte werden durch GC entfernt.
- **Nachweis:** `T-SEP-*`, `T-TT-*`, `T-VCS-ERASE-1`.

### L03 Abfrage und Antwort [E]
- **Auslöser:** `query`, FF1 bis FF4.
- **Vorbedingungen:** Authentifizierung; Mandant; Ratenbegrenzung; bei P-Zugriff `case_id` (höchstens `max_cases_per_query`) und Zuweisung; Generator verfügbar sonst `Degraded(Inference)`.
- **Schritte:** Sichtklasse → Stand → Kandidatenmengen (K: `V ∩ A_K`; P: `A_P`, `Live`, nicht im Gate) → Anfrageaufbereitung → BM25 und Vektorsuche → RRF → Reranker → Kontextaufbau mit Beleg-Kennungen und Anfrage-Token → Generierung mit Epochenprüfung (9.5) → Belegprüfung (9.6) → Antwort oder Enthaltung → `Query`-Entscheidung (Eingabe `Sealed` bei Personenbezug).
- **Nachbedingungen:** Entscheidungsprotokoll im K-Ledger (`DecisionAppended`); keine P-Antwort im Cache; Sitzungsschritt mit Lineage.
- **Fehler:** `Erased` bei Gate-Treffer während der Anfrage, `Transient(Overloaded)`, `Unavailable`, `GroundingViolation`, Strukturfehler (ein Wiederholungsversuch, dann Enthaltung).
- **Absturz:** keine Persistenz außer Protokoll; Sitzungen überleben keinen Neustart; Protokolleintrag nach Ledger-Regel (WP-2).
- **Nachweis:** `T-ACL-*`, `T-ACL-RANK`, `T-GROUND-*`, `T-GATE-*`, `T-DEC-*`.

### L04 Vorgang (Case) [E]
- **Auslöser:** `case.register`, `case.grant`, `case.close`, Fristablauf, Fachverfahren.
- **Vorbedingungen:** `external_ref`, Betroffene, `owner_unit`, Aufbewahrungsklasse; Zuweisung nur durch Fachverfahren oder `TenantAdmin`.
- **Schritte:** Registrieren (Vorgangsobjekt, DEK `n = 0`, Fallindex); Zuweisen (A-Ledger); Befüllen (neue Versionen erweitern den Fallindex); Betreiben (FF2 bis FF5); Schließen (Zuweisungen enden, Fallindex bereinigt, Dokumente nach Aufbewahrungsklasse); Beenden (Saga für alle Dokumente, Vorgang mit dem letzten Dokument vernichtet).
- **Fehler:** unbekannte Betroffene (werden angelegt, WP-1), Zuweisung ohne Recht (`Policy`).
- **Absturz:** Fallindex ist Ableitung und wird aus P neu gebaut.
- **Nachweis:** `T-ACL-CASE`, `T-CASE-*`.

### L05 Betroffenenanfrage und Löschsaga [E]
- **Auslöser:** `request.create`, `erasure.create`, Fristjob, Takedown, Off-Boarding, Ausscheiden Mitarbeitender.
- **Vorbedingungen:** `identity_verified` (außer vorsorgliche Einschränkung); Rolle; bei `Besonders` Standardpfad vorgezogen.
- **Schritte:** 8.2 (Anfrage), 8.3 (S0 bis S9), 8.9 (Mehrbetroffene), 8.5 (Messung), 8.6 (Art. 19), 8.7 (Klassen), 18.1 (Bericht).
- **Nachbedingungen:** Zielmenge je Klasse G, B, K, M `reached` oder `not_reached`; zurückgestellte Versionen `Restricted`; Bericht und E-Ledger-Eintrag; `ScopeKey` vernichtet; Canary unberührt.
- **Fehler:** `Pending(Anchor|Sink|Review|Archive|Backup)`, `SagaStuck`, `MeasureInvalid`, `Poisoned`; Fehlschlag des Gate-`fsync`: Aufruf scheitert, RAM-Sperre bleibt, Mandant vergiftet.
- **Absturz:** C07 bis C13, C18, C20, C21; physische Tatsachen haben Vorrang.
- **Nachweis:** `T-CRASH-*`, `T-GATE-*`, `T-EVID-*`, `T-MULTI-*`, `T-REST-*`, `acta-verify` V1.

### L06 Berichtigung (Art. 16) [E]
- **Auslöser:** `request.create` Art Berichtigung, `AddSubject`, `RedactSplit`.
- **Schritte:** neue Version (neue `VerRef`, neuer `DocumentKey`, neuer DEK), alte Version `Superseded` (nur `restricted_read`), Ableitungen der alten Version bereinigt (5.2 Nr. 6), neue Ableitungen; Mitteilung nach Art. 19 über Ausgangskorb; nach `history_horizon` (bei Löschgrund sofort) Saga auf die alte Version.
- **Fehler:** `Besonders`: `history_horizon = 0`, alte Version sofort Saga.
- **Absturz:** C03, C04, C10, C11.
- **Nachweis:** `T-DEC-*`, `T-GATE-SUPERSEDED`.

### L07 Einschränkung, Widerspruch, Legal Hold [E]
- **Auslöser:** Art. 18, Art. 21, Aufbewahrungspflicht, `legal_hold`, vorsorglich (Trockenlauf-Timeout, Quarantäne).
- **Schritte:** `Restrict(grund)` im Gate-Log (RAM zuerst, `fsync`, Epoche); Ableitungen bereinigt; Daten bleiben unter ihrem Schlüssel; `Release` nur aus `Restricted`, nie aus `Erased`; `legal_hold` nur Rolle `Legal` mit Vier-Augen, Wiedervorlage `hold_review_interval`; Hold blockiert S5.
- **Nachbedingungen:** `restricted_read` zweckgebunden im E-Ledger; bei `Release` Neuaufbau der Ableitungen.
- **Absturz:** C07 bis C09.
- **Nachweis:** `T-GATE-RESTRICTED`, `T-HOLD-*`.

### L08 Takedown und Fristlöschung in K [E]
- **Auslöser:** irrtümlicher Personenbezug, fehlgeschlagener `Split`-Wiederholungstest, gerichtliche Anordnung, `decision_retention` abgelaufen.
- **Vorbedingungen:** Takedown: Vier-Augen (`DataProtection` und `Legal` oder `TenantAdmin`), signiert; Fristlöschung: keine Freigabe, `ArchiveHook.offer`.
- **Schritte:** 6.7: `RedactK` im Gate-Log sofort; DK-Segmente ersetzen; Pack ohne Blob inklusive Salz neu schreiben, `SegmentKey` vernichten, Rotation anfordern; K-Ledger `Redacted` bzw. `Expired`; Commitment im E-Ledger und in der Senke.
- **Absturz:** C22; `Redacted`-Vermerk erst nach Vernichtung des alten `SegmentKey`.
- **Nachweis:** `T-TT-REDACTED`, `T-VCS-ERASE-1`, `T-CRASH-TAKEDOWN`.

### L09 Fristenjob [E]
- **Auslöser:** täglich; `retain_until` erreicht; `payload_retention` und `decision_retention` abgelaufen; `quarantine_purge_after`; `history_horizon`; `access_log_retention`.
- **Schritte:** fällige Ziele bündeln; `Expired`-Saga je Stapel (Batch-Vernichtung); Nutzlastschlüssel vernichten; A-Ledger-Monat mit `Pruned`-Vermerk; Entscheidungen nach L08.
- **Nachweis:** `T-RET-*`, `T-CRASH-*`.

### L10 Wurzelrotation [E]
- **Auslöser:** `T_ROT`, `N_SHRED`, Anforderung (Besonders, 70 % `erase_sla_B`).
- **Schritte:** 4.3 Schritte 1 bis 4 mit `rotation_lock`, Schnappschuss, Neuschreiben ohne Sperre, `vault_freeze` mit Schwanz, atomarer Austausch, `anchor.destroy(g)`, `RootRotated`.
- **Fehler:** Mandant fällt aus: `M_g` bleibt, `RotationOverdue`; Anker nicht erreichbar: `Degraded(Anchor)`.
- **Absturz:** C14, C15, C16.
- **Nachweis:** `T-ROT-*`, `T-CRASH-VAULT-*`, Messung 120 s.

### L11 Tresorexport und Datensicherung [E]
- **Auslöser:** `vault_export_interval`, `data_backup_interval`, `acta-cli backup`.
- **Schritte:** Export unter `VKb(t,e)` ohne Grabsteine, serialisiert über `vault_freeze`; Epochenverwaltung (`M_bk(e)` erzeugen, nach `end(e) + T_BACKUP` vernichten); Datensicherung der Segmente, des Manifests, Gate-Log-Snapshots, Ledger, Verzeichnisse (ohne DP, Caches, Temp, Tresordateien); Exportdateien nach Ablauf mit Scrub entfernen; Kopienregister aktualisieren.
- **Fehler:** `ExportLate`, `BackupOverdue`.
- **Absturz:** C17, C18.
- **Nachweis:** `T-BACKUP-*`, `T-REST-*`.

### L12 Wiederherstellung [E]
- **Auslöser:** `restore` nach Verlust oder Wiederherstellungsübung.
- **Schritte:** 8.8 (Modus `Restoring`, Schritte 1 bis 10); anschließender Neuaufbau der DP-Indizes.
- **Fehler:** `RestoreBeyondHorizon`, `RestoreBehindAnchor`, `Quarantine` (kein Registerbeleg), zurückgerolltes K-Ledger (`accept_rollback`).
- **Absturz:** C21.
- **Nachweis:** `T-REST-*`, Quartals-Restore-Test.

### L13 Modell-, Chunking- und Indexwechsel [E]
- **Auslöser:** neues Modell, neue `chunk_cfg`, neue Normalisierungsvariante.
- **Schritte:** 11.4 Phasen 1 bis 6; DK-Neuaufbau als Hintergrund; DP-Reindex unter Systemzweck `Reindex` (aggregiert protokolliert); alte DP-Generationen sofort entfernt; alte DK-Generationen nach `index_gen_retention`.
- **Fehler:** `QualityDrift`, Regression über Grenze verhindert Aktivierung.
- **Absturz:** Neuaufbau ist verwerfbar; `main` zeigt auf die alte Generation bis zum Abschluss.
- **Nachweis:** `T-TT-*`, `T-GEN-*`.

### L14 Szenario [E]
- **Auslöser:** `scenario.create`, `scenario.run`.
- **Schritte:** 7.3; Szenario-Branch, Änderungen, Fragenset (N-Validator-geprüft), Lauf gegen Basis und Szenario, Vergleich, `merge` oder `retire`; temporärer Index bei geändertem Chunking oder Embedder.
- **Nachbedingungen:** `main` und P unverändert (Z9); Ergebnisse mit Personenbezug `Sealed`.
- **Nachweis:** `T-SCEN-*`.

### L15 Mandant: Anlage und Off-Boarding [E]
- **Anlage:** 12.6 (unter `rotation_lock`); bis Abschluss `Quarantine`; Pflichtparameter erfragt; Canary angelegt.
- **Off-Boarding:** 12.6 Schritte 1 bis 6; Sperre aller Versionen, Export nach Betreiberentscheid, Saga `Expired`, Entfernen aller Segmente und des Tresors mit Scrub, Ledger archivieren oder vernichten, Bericht mit `offboarding_exposure_until`.
- **Absturz:** C27; Fortsetzung anhand der verbleibenden Dateien.
- **Nachweis:** `T-TENANT-*`, `T-OFFBOARD-*`.

### L16 Start, Betrieb, Herunterfahren [E]
- **Start:** 14.1 Nr. 1 bis 9; Fehler führt zu `Quarantine` oder `Poisoned`.
- **Betrieb:** Hintergrundaufgaben 14.2; Zustandsmaschine 14.6.
- **Herunterfahren:** Anfragen abschließen, Journal flushen, Gate-Log synchronisieren, Schlüsselspeicher freigeben (14.8).
- **Absturz:** C24, C25.
- **Nachweis:** Selbsttest, `T-HOST-*`.

### L17 Fachfunktionen FF1 bis FF9 [E]
Eingaben, Verarbeitung, Persistenz und Löschverhalten stehen in Tabelle 10. Gemeinsame Vor- und Nachbedingungen: Zugriffskontrolle (9.4), Gate (8.1), Belegpflicht bei Modellausgaben, Entscheidungsart `Query`, `Tool`, `Review` oder `Release`, menschliche Freigabe für Entscheidungsgrundlagen; `feature_enabled` nur mit Goldset. FF4: Entwurf entsteht als P-Dokument-Version, `Release` nach Freigabe. FF5: deterministisch, `Tool`-Entscheidung. FF6: kein Zugriff auf P. FF9: Kette L02.
- **Fehler:** `Unavailable`, Enthaltung, `GroundingViolation`, Kalender fehlt (FF5: `Invalid`).
- **Nachweis:** `T-FF-*`, Goldsets.

### L18 Mitarbeitende: Ausscheiden und Handles [E]
- **Auslöser:** Ausscheiden, Anfrage eines Mitarbeitenden, `actor_handle_epoch` Ablauf.
- **Schritte:** Saga auf dem `Staff`-Betroffenen; Zuordnung `ActorRef` → Person in P entfernt; Handles in K lösen zu `Erased` auf; Begründungskette fremder Vorgänge bleibt; neue Epoche vergibt neues Handle für neue Commits.
- **Nachweis:** `T-ACTOR-*`.

### L19 Egress-Freigabe und externe Inferenz [E]
- **Auslöser:** `egress.grant`.
- **Vorbedingungen:** Vier-Augen; Dienst, Zweck, Datenklassen, Ablauf, DSFA-Referenz, Auftragsverarbeitungsvertrag, Drittlandprüfung.
- **Schritte:** Eintrag im E-Ledger; Positivliste der Ziele; jeder Aufruf protokolliert (Dienst, Größe, Klasse, Grant-ID); Kopienregister; Ablauf sperrt.
- **Fehler:** P-Inhalt ohne ausdrückliche Freigabe: `Policy`.
- **Nachweis:** `T-EGRESS-*`.

### L20 Supportpaket und Upgrade [E]
- **Support:** `acta-cli support-bundle` erzeugt lokal aus Positivliste; Betreiber gibt frei; Kopienregister; `T-VENDOR-BUNDLE`.
- **Upgrade:** 14.8; Sicherung vorab, Wartungsfenster, Migration vorwärts, Selbsttest; Fehler: `Quarantine` und Wiederherstellung der Vorversion.
- **Absturz:** Migration atomar je Datei (WP-5).

### L21 Vergiftung und Reparatur [E]
- **Auslöser:** `Integrity`/`Crypto` (15.2).
- **Schritte:** kleinster isolierender Bereich geschlossen; Gate-Sperren bleiben; `acta-cli repair` mit Prüfbericht; bei `Tenant`/`Node` Vier-Augen; Neustart allein hebt nicht auf.
- **Nachweis:** `T-POISON-*`.

### L22 Betroffenenverzeichnis und Alias [E]
- **Auslöser:** neuer Betroffener, weitere externe Kennung, `subjects.discover`.
- **Schritte:** Tresoreintrag `SubjectKey`; `SubjectAlias` (Tag → Primär-Tag); Verzeichnissatz unter `cluster_id`; `discover` nur Rolle `DataProtection`, Ergebnis Kandidaten ohne Volltext; `AddSubject` nur nach Review.
- **Nachweis:** `T-ALIAS-*`, `T-MULTI-ADD`.

---

