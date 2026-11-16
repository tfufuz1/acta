## 18. Berichte und Schnittstellen

### 18.1 Löschbericht [E]

Kanonisches JSON (RFC 8785), signiert mit Ed25519 (`acta/report/v1`, `sig_alg`, `key_id`). **Keine** Kennungen von Personen, keine Inhalte. Alle Felder Pflicht, sofern nicht vermerkt.

|Feld|Inhalt|
|---|---|
|`report_version`, `tenant`, `erasure_id`|Kennzeichnung (`TenantId`, `ErasureId`)|
|`trigger`, `scope_kind`|Auslöser (8.2) und Ziel (`Subject`, `Document`, `Version`, `Expired`, `Takedown`)|
|`ledger_ref`|Eintragshash im E-Ledger|
|`times`|`requested`, `planned`, `gated`, `purged`, `tombstoned`, `rotated`, `backups_expired`, `measured`|
|`classes`|je G, B, K, M: `reached` / `not_reached` / `not_applicable` mit Zeitpunkt und Belegverweis; Zwischenstände `key_tombstoned`, `K_pending_backups`|
|`scope_result`|`complete` / `partial_restricted` / `pending`; getrennte Zählung von Zielmenge und zurückgestellten Versionen|
|`restricted`|Grundcode (`Retention`, `LegalHold`, `Objection`, `Contested`, `MultiSubjectReview`, `Precautionary`); Aussage „eingeschränkt, nicht gelöscht“|
|`measurements`|je Schicht: Verfahren, Art (`complete`, `sample`, `declared`, `not_measured`, `not_implemented`), Positivkontrolle (`passed`/`failed`), Ergebnis, Protokollhash|
|`co_subjects`|Anzahl und Modus je Dokument (8.9), nie Kennungen|
|`registry_mirrored`, `backup_exposure_until`|Spiegelstand der Senke; Ende des Sicherungsfensters|
|`anchor`|`assurance`, Vernichtungsfenster, Hash des `DestroyProof`|
|`synthetic_subjects`|Zähler der Canaries|
|`assumptions`, `residual_risks`|A-* und R-* aus 20.3|
|`crypto`|`alg`, `sig_alg`, `key_id`|
|`generated_at`, `signature`|Zeit (Port `Clock`), Signatur über `acta/report/v1 ‖ kanonische Bytes ohne signature`|

Textformulierungen folgen 1.6, zum Beispiel „Klasse K erreicht am …; Klasse M nicht erreicht: Senke nicht bestätigt“. Ein Bericht entsteht erst mit Eintrag im E-Ledger (C20).

### 18.2 Verzeichnis von Verarbeitungstätigkeiten, Art. 30 (Entwurf) [E]

Je Mandant erzeugt `acta-reports` einen Entwurf: Zwecke (aus Sammlungen), Kategorien betroffener Personen und Daten, Empfänger (`DisclosureLog`, Egress-Freigaben), Fristen (Aufbewahrungsklassen), technische und organisatorische Maßnahmen (Verschlüsselung, Zugriffskontrolle, Löschkonzept, Protokollierung). Der Betreiber ergänzt und verantwortet das Verzeichnis.

### 18.3 DSFA-Entwurf, Art. 35 [E]

Gliederung: Beschreibung der Verarbeitung; Notwendigkeit und Verhältnismäßigkeit; Risiken (aus 2.5); Abhilfemaßnahmen; Restrisiken (R-*, A-*); KI-Besonderheiten (Halluzination, Prompt-Injection, Embeddings, Membership Inference); Mehrbetroffene; Betroffenenrechte. Der Entwurf ersetzt die Bewertung durch den Verantwortlichen nicht.

### 18.4 Weitere Berichte [E]

|Bericht|Inhalt|
|---|---|
|Fristen und Aufbewahrung|fällige Löschungen, `Restricted`, `legal_hold`, offene Anfragen, `RequestDue`|
|Betrieb|Zustände (14.6), Metriken (14.3), Exportintervall, `offboarding_exposure_until`, Hostprüfungen, Deklarationen (A-FS, A-RAM)|
|Messbericht|[Q]-Werte, Goldset-Messungen, Zusagen-Tests (16.5)|
|Kryptobericht|4.7|
|Kopienregister|8.6|
|Wiederherstellung|`unrecoverable_docs`, Horizont, Wiedervernichtungen (8.8)|
|Löschkonzept-Entwurf|12.4|

Alle Berichte nennen ihre Annahmen, tragen keine verbotenen Formulierungen und enthalten keine Kennungen.

### 18.5 Schnittstelle [E]

HTTP/JSON über TLS, Authentifizierung OIDC oder mTLS (13.4), Streaming über Server-Sent Events, Versionskennung in der URL (`/v1/…`). Jede Operation prüft Rolle, Sichtklasse und Mandant vor jeder Verarbeitung.

|Bereich|Operationen|
|---|---|
|Wissen|`ingest` (K und P), `commit`, `branch`, `tag`, `diff`, `merge`, `retire`, `checkout`, `document.retire`|
|Abfrage|`query` (mit `as_of`, `valid_at`, `case_id`), `explain`, `replay`, `session.*`|
|Fachfunktionen|`ff.answer`, `ff.case_assist`, `ff.checklist`, `ff.draft`, `ff.deadline`, `ff.radar`, `ff.compare`, `ff.rationale`, `ff.knowledge_flow` (FF1 bis FF9, 10)|
|Szenarien|`scenario.create`, `scenario.run`, `scenario.compare`|
|Vorgänge|`case.register`, `case.grant`, `case.close`|
|Betroffene|`request.create`, `request.status`, `subjects.discover`, `export` (Art. 15, 20), `erasure.create`, `erasure.status`, `review.redact`, `review.decide`|
|Evaluation|`eval.run`, `eval.compare`|
|Administration|`tenant.*`, `collection.*`, `retention.set`, `hold.set`, `egress.grant`, `backup.*`, `restore`, `repair`, `report.*`, `support-bundle`, `accept_rollback`|

**Fehlerabbildung:** `Policy` → 403; `Invalid` → 400 oder 422; `Erased` → 410 (nur mit Zugriffsrecht; ohne Recht 404, Z16); `Integrity`/`Crypto` → 503 (Vergiftung); `Transient` → 503 mit `Retry-After`. Fehlerkörper enthalten nur `code()` und eine Korrelations-ID ohne Personenbezug. Ressourcennamen und Antwortzeitklassen unterscheiden unzugängliche und nicht vorhandene Ziele nicht.

---

