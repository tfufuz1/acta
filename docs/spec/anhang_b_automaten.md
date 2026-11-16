## Anhang B – Zustandsautomaten

### B.1 Löschsaga

|Von|Ereignis|Nach|Bedingung|
|---|---|---|---|
|(neu)|Anfrage|`Requested`|Ledger-`fsync`|
|`Requested`|Trockenlauf und Plan fertig|`Planned`|8.3 S1|
|`Requested`|Trockenlauf nicht rechtzeitig|`Planned` mit allen Versionen `Restricted(Precautionary)`|`erase_return_timeout`|
|`Planned`|Konflikt, Aufbewahrungspflicht, Hold|`Restricted(grund)` für betroffene Versionen; übrige weiter|8.4, 8.9|
|`Planned`|Sperre durabel|`Gated`|Gate-Log-`fsync`|
|`Gated`|Bereinigung vollzogen|`Purged`|S3|
|`Purged`|Messung B gültig|`PlaintextMeasured`|Positivkontrolle bestanden|
|`PlaintextMeasured`|Schlüssel vernichtet|`KeyTombstoned`|S5|
|`Gated`|`Besonders` oder SLA ≥ 70 %|S5a vor S3 (vorgezogener Pfad)|8.3|
|`KeyTombstoned`|Rotation und `DestroyProof`|`Rotated`|S6|
|`Rotated`|Sicherungsfenster abgelaufen|`BackupsExpired`|S7|
|`BackupsExpired`|Messung K und M|`Measured`|S8|
|`Measured`|Bericht und Ledger|`Reported`|S9|
|jeder|Voraussetzung fehlt|`Pending(grund)`|Anker, Senke, Review, Archiv, Sicherungsfenster|
|`Pending`|Voraussetzung erfüllt|vorheriger Zustand|–|
|`Restricted`|Frist, Hold weg, Review `DeleteAll`/`RedactSplit`|`Planned`|8.4, 8.9|
|jeder|Integritäts- oder Kryptofehler|`Poisoned(Scope)`|15.2|

Es gibt keinen Zustand `Failed`. Eine gesetzte Sperre wird nur durch `Release` aus `Restricted` aufgehoben, nie aus `Gated`, `Expired` oder `Erased`.

### B.2 Version (P)

|Von|Ereignis|Nach|
|---|---|---|
|(neu)|Ingest ohne Betroffenenzuordnung|`Quarantine`|
|`Quarantine`|Zuordnung und Klassifikation|`Live`|
|`Quarantine`|`quarantine_max`|`Restricted(Precautionary)`|
|`Quarantine`/`Restricted(Precautionary)`|`quarantine_purge_after`|`Expired` (Saga)|
|(neu)|Ingest mit Zuordnung|`Live`|
|`Live`|Berichtigung, `AddSubject`, `RedactSplit`, `RestrictShared`|neue Version `Live`/`Restricted`; alte `Superseded`|
|`Superseded`|`history_horizon` oder Löschgrund|Saga auf die Version|
|`Live`|Aufbewahrungspflicht beim Antrag|`Restricted(Retention)`|
|`Live`|Widerspruch, bestrittene Richtigkeit|`Restricted(Objection|Contested)`|
|`Live`|Mehrbetroffenen-Konflikt|`Restricted(MultiSubjectReview)`|
|`Restricted`|Löschentscheidung, Fristende|`Gated` (über `Planned`)|
|`Live`|Antrag, Fristablauf|`Gated` (über `Planned`) oder `Expired`|
|`Gated`/`Expired`|Schlüsselvernichtung abgeschlossen|`Erased`|

### B.3 Objekt in K

`Live` → `Redacted` (Takedown, Gate `RedactedK`, nie zurück) · `Live` → `Expired` (Fristlöschung eines Entscheidungsprotokolls nach `ArchiveHook`) · unerreichbar → entfernt (GC nach `branch_retention`).

### B.4 Betriebszustand

`Normal` ↔ `Degraded(Anchor|Sink|Inference)` · jeder → `Restoring` (Wiederherstellung) → `Normal` oder `Quarantine` · jeder → `Quarantine` (Registerbeleg fehlt, Selbsttest fehlgeschlagen, Ordinalkonflikt, `RestoreBehindAnchor`, `RestoreBeyondHorizon`) · jeder → `Poisoned(Scope)`. Aus `Quarantine` und `Poisoned` führt nur ein dokumentierter Vorgang mit Prüfbericht (`acta-cli repair`, `accept_rollback`) heraus.

### B.5 Tresor-Eintrag

`aktiv (0x01)` → `Grabstein (0x02)` → bei Rotation nicht übernommen → mit `M_g` unlesbar.

### B.6 Mandant

`Quarantine` (Anlage) → `Normal` → `OffBoarding` (alle Versionen `Restrict(Offboarding)`) → entfernt.

---

