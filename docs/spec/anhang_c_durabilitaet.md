## Anhang C – Durabilitäts- und Absturzpunkte

Grundregeln: **Schlüssel zuerst** (WP-1); **Ledger zuerst**, dann Ref-Bewegung (WP-2); **Sichtbarkeit zuletzt** (WP-3); **physische Tatsachen vor Journal** (WP-4); Segmente, Manifeste und Dateien atomar über `rename` und Verzeichnis-`fsync` (WP-5). Jeder Punkt hat einen `T-CRASH-*`-Test.

|ID|Absturzpunkt|Zustand nach Neustart|
|---|---|---|
|C01|Schlüssel durabel, noch kein Umschlag|verwaister Schlüssel; Sweeper vernichtet ihn nach Karenz, wenn kein Bezug entsteht|
|C02|Umschlag im Journal, Segment fehlt|Journalwiederholung, idempotent|
|C03|Segment geschrieben, Manifest fehlt|Segment verwaist, wird bereinigt, nichts sichtbar|
|C04|Manifest durabel, K-Commit fehlt|unsichtbar bis Commit; Ingest mit gleicher `ingest_id` wiederholbar|
|C05|K-Objekte geschrieben, Ref nicht bewegt|unerreichbar; Garbage Collection|
|C06|K-Ledger-Eintrag durabel, Ref nicht bewegt|Start vollzieht die Ref-Bewegung|
|C07|RAM-Sperre gesetzt, Gate-Log-`fsync` fehlt|kein Erfolg gemeldet, Sperre entfällt, Antrag erneut stellen|
|C08|Gate-Log durabel, Rückkehr fehlt|Sperre aktiv, Saga `Gated`; Aufruf mit gleicher `ErasureId` wiederholbar|
|C09|Gate durabel, Senke ohne Eintrag|Senke wird asynchron nachgeholt|
|C10|Ersatzsegment geschrieben, Manifest fehlt|Ersatz verwaist, S3 wiederholt|
|C11|Manifest-Batch durabel, Altdatei nicht entfernt|Altdatei als verwaist mit Scrub entfernt|
|C12|`destroy_many` mitten im Batch|Tresorzustand maßgeblich; Rest wird vernichtet (idempotent)|
|C13|Grabstein geschrieben, Direct-Read fehlt|Verifikation wiederholt|
|C14|Rotation Schritt 2 mitten (Neuschreiben)|neue Datei verworfen, Rotation neu; Tresor g maßgeblich|
|C15|Rotation nach `rename`, Anker nicht vernichtet|Start setzt Schritt 4 fort|
|C16|Anker vernichtet, `RootRotated` fehlt|Start schreibt `RootRotated` nach (Tatsache vor Journal)|
|C17|Tresorexport halb geschrieben|verworfen; letzter vollständiger Export gilt|
|C18|`destroy_backup_epoch` vollzogen, E-Ledger fehlt|Eintrag nachgetragen|
|C19|Checkpoint signiert, `record_head` fehlt|Kopf nachgemeldet|
|C20|Bericht signiert, E-Ledger-Eintrag fehlt|Eintrag nachgeholt; Bericht gilt erst mit Eintrag|
|C21|Restore mitten in der Wiedervernichtung|bleibt `Restoring`, Verfahren neu|
|C22|Takedown: Pack ersetzt, alter `SegmentKey` lebt|fortsetzen; `Redacted`-Vermerk erst nach Vernichtung|
|C23|Gate-Log-Verdichtung mitten|atomar über neue Datei und `rename`; alter Stand gültig, Archivierung wiederholt|
|C24|Journalrotation mitten|alter Journalstand gültig, Rotation wiederholt|
|C25|Journal-Flush: Segmente geschrieben, Altjournal nicht entfernt|Altjournal wird anhand des Manifests als überführt erkannt und mit Scrub entfernt, Journal-`SegmentKey` vernichtet|
|C26|Registersenke: Eintrag geschrieben, `last_acked` fehlt|Eintrag idempotent erneut gesendet|
|C27|Off-Boarding mitten|Zustand `OffBoarding`; Fortsetzung anhand der verbleibenden Dateien; Sperre bleibt|

Zusätzliche Fehlerklassen mit Test: `FsyncFailed` (Datei nicht erneut synchronisieren, neue Datei, sonst Vergiftung), `ENOSPC` (Reserve `min_free_bytes`, Abbruch vor Beginn), zerrissene Schreibvorgänge (Prüfsumme und Kettenmarke).

---

