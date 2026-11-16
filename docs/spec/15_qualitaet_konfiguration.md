## 15. Qualität, Fehler und Konfiguration

### 15.1 Fehlerklassen und Behandlung [E]

|Klasse|Behandlung|
|---|---|
|`Transient`|Wiederholung mit Backoff, Zustand bleibt, Alarm nach Schwelle|
|`Integrity`, `Crypto`|Vergiftung (15.2)|
|`Erased`|normales Ergebnis, nur aus Gate oder Registerbeleg (3.5)|
|`Policy`|Ablehnung (Rolle, Zugriff, Freigabe)|
|`Invalid`|Ablehnung der Eingabe, nichts wird geschrieben|

### 15.2 Vergiftung [E]

Vergiftung schließt den **kleinsten nachweislich isolierenden Bereich**: `Document` (ein Umschlag mit Authentisierungsfehler bei vorhandenem Schlüssel), `Segment` (Kopf-MAC oder Prüfsumme), `Tenant` (Tresor, Manifest, Gate-Log, Ledger-Kette, Senkenwiderspruch, `FsyncFailed` ohne Ersatz), `Node` (Anker, Uhr, Zufall, Ordinalüberlauf). In einem vergifteten Bereich scheitern alle Lese- und Schreibzugriffe mit `Integrity` oder `Crypto`; **Gate-Sperren bleiben wirksam**. Aufhebung nur durch `acta-cli repair` mit Prüfbericht und, bei `Tenant` und `Node`, Vier-Augen-Freigabe; ein Neustart allein hebt eine Integritätsvergiftung nicht auf.

### 15.3 Leistungsziele [Q]

Referenz: 8 Kerne, 64 GiB RAM, NVMe, ein Knoten. Alle Werte sind Startwerte; Freigabe nur nach gemessenem Bericht.

|Messgröße|Zielwert|
|---|---|
|Rückkehr einer Löschung im Zustand `Gated` (Person mit 1 000 Versionen)|p99 unter 50 ms (harte Grenze `erase_return_timeout` 2 s)|
|Gate-Prüfung|unter 1 µs je Prüfung|
|Hybridsuche (1 Mio. Chunks, ein Mandant)|p95 unter 1,5 s bis zu den Top-n Belegen|
|Antwortbeginn (lokale Inferenz)|abhängig vom Modell; Pipeline ohne Modell unter 2 s p95|
|Tresor öffnen (10 Mio. Einträge)|unter 30 s|
|Rotation (10 Mio. Einträge)|unter 120 s|
|Ingest (Text, ohne Embedding)|mindestens 50 MiB/s gesamt|
|Segmentersatz (256 MiB gefiltert)|unter 20 s|
|Klasse B einer Einzelperson mit 1 000 Dokumenten|unter 24 h (`erase_sla_B`), erwartbar Minuten|

### 15.4 Konfiguration und Invarianten [E]

Der Start prüft folgende Invarianten und verweigert bei Verletzung den Betrieb mit `Invalid(Config)`:

|ID|Invariante|
|---|---|
|R-HORIZON|`commit_key_retention ≥ T_BACKUP + backup_epoch`|
|R-BK|`backup_epoch ≤ T_BACKUP`; `data_backup_retention ≤ T_BACKUP`|
|R-ROT|`T_ROT ≤ 24 h` und `rotation_min_gap ≤ T_ROT`|
|R-EXP|`vault_export_interval ≤ 4 h`|
|R-SINK|Registersenke liegt auf einem anderen Gerät als die Daten (`DirSink`: `st_dev` verschieden)|
|R-ANCHOR|`anchor_destroy_max` gesetzt; bei `Besonders` kein `FilePassphrase`|
|R-HIST|`history_horizon(Besonders) = 0`|
|R-RET|`decision_retention` und `payload_retention` je Art gesetzt; `access_log_retention` gesetzt|
|R-RETMAX|`commit_key_retention ≤ 3 Jahre`, darüber nur mit dokumentierter Begründung im Löschkonzept [⚠]|
|R-KMIN|`K_MIN ≥ 5`|
|R-DPBK|bei `Besonders`: `backup_derived_dp = false`|
|R-FS|Dateisystem zulässig oder deklariert (13.1)|
---

