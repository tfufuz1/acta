# Prüfbericht SPEC R3: Blinde Flecken, Logikfehler, Änderungsanträge

Stand: 2026-11-22. Geprüft: `SPEC.md` (R2), `IMPLEMENTATION_SPEC.md`, `ARCHITECTURE.md`, `CLAUDE.md`, Anhänge A bis D.

## 1. Vorgehen und Geltung

Die Prüfung sucht Widersprüche zwischen Abschnitten, Lücken in Protokollen und Invarianten, die nichts absichern, sowie Mechanismen, die doppelt definiert oder doppelt ausgeführt werden. Befunde sind nach Schwere sortiert.

Die Änderungen zerfallen in zwei Gruppen:

- **Direkt korrigiert** (Abschnitte ohne Änderungsschutz nach `CLAUDE.md` §4 und `CONTRIBUTING.md`): §1 bis §5 (ohne Schutz), §7, §10, §11, §12, §15, §16, §17, §20, §22, Anhang B. Die Korrekturen stehen in `SPEC.md`, §22.5 führt sie mit Status.
- **Änderungsantrag** (geschützt nach `SPEC.md` §1.5: §6–9, §13, §14, §18): Diese Befunde sind hier mit Formulierungsvorschlag dokumentiert und **nicht** eingearbeitet. Sie brauchen zwei Reviewer (§1.5), aktualisierte Goldens (Anhang A) und, wo ein Format betroffen ist, eine Prüfung gegen die Freeze-Liste (§9.8).

## 2. Kritische Befunde

### CR-R3-01: Stamps werden nach Absturz neu vergeben (R3-14, R3-13)

**Stelle:** §13.1 Schritt 7 (`next_stamp = max(Redo-Stamp, flushed, Run) + 1`), §16 E-2, §7.2 (`Consumer::advance`), §9.7 (`reserve`).

**Problem:** Bei `Durability::Interval` oder `Never` werden Changes sichtbar, bevor sie dauerhaft sind (E-2). Stirbt der Prozess, fehlen diese Frames nach dem Neustart. `next_stamp` beginnt beim höchsten *wiederhergestellten* Stamp, also wird die Nummer erneut vergeben. Ein Consumer, der über den verlorenen Stamp hinaus vorgerückt ist (über `view`-Stände, §7.1), überspringt dann den neuen Change mit derselben Nummer. Außerdem kollidieren Erase-IDs `S` (§14.3 E1, `erase_pending_add`).

**Anders als bei IDs** (INV-KEY-2, Hochwassermarken im Catalog vor Nutzung) gibt es für Stamps keine persistente Hochwassermarke.

**Vorschlag:**
1. Feld `stamp_hwm u64` im Catalog-Feld `0x0007 reserve`. Die Wartung reserviert Stamps in Blöcken (wie `id_reserve_block`), bevor sie vergeben werden. Nach Absturz beginnt die Vergabe bei `stamp_hwm`. Ungenutzte Stamps verfallen; das ist für Stamps zulässig, weil `INV-STAMP-1` nur über angenommene Changes gilt.
2. `next_stamp` beim Öffnen = `max(stamp_hwm, letzter Redo-Stamp, …) + 1`.
3. Neue Invariante **INV-STAMP-3**: Ein Stamp, der einmal einem Change zugeordnet wurde, wird nach einem Absturz nie einem anderen Change zugeordnet.
4. Schritt 7 in §10.2 (R3-06) bleibt: Bei `Busy` wird nur der Zähler zurückgesetzt, nicht die Hochwassermarke.

**Test:** FaultFs mit `Interval`: Change mit Stamp n sichtbar, Absturz vor Sync, Neustart, neuer Change. Erwartung: neuer Stamp > n, Consumer sieht beide Stände eindeutig.

### CR-R3-02: Physische Löschung lässt Altdaten in Redo-Segmenten (R3-15)

**Stelle:** §14.2 (Stufe `Physical`: „Redo-Segmente entfernen“), §14.3 E2 (Redo-Segmente mit `max_stamp ≤ S` ins Graveyard), §18 `INV-ERA-2`.

**Problem:** Ein Redo-Segment, das Frames vor und nach dem Löschstamp `S` enthält, hat `max_stamp > S` und bleibt bestehen. Seine Frames mit Stamp ≤ S enthalten die gelöschten Werte im Klartext (bei `Plain`) oder unter dem Space-DEK (bei `Encrypted`). `INV-ERA-2` verlangt aber: nach `erase_done(S)` kein lebendes Redo-Segment mit einer Version im Bereich mit Stamp ≤ S. Der beschriebene Ablauf verletzt die Invariante.

**Vorschlag:**
- E1 erzwingt einen **Segmentschnitt** direkt vor dem Erase-Frame: Das laufende Segment wird gesiegelt, der Erase-Frame liegt im neuen Segment.
- Für `Physical` gilt: Vor E5 wird jedes Segment mit Frames ≤ S und Frames > S ohne die Frames ≤ S neu geschrieben (Redo-Rewrite über die Stamp-Grenze, gleiche Segment-ID-Familie über Neustart-Regel), oder der Erase-Frame-Schnitt ist die einzige Garantie und `Physical` schließt Redo ausdrücklich aus (Tabelle §14.2, Spalte „Schützt nicht gegen“ erweitern).
- `INV-ERA-2` präzisieren: „... kein lebendes Redo-Segment mit Frames ≤ S **außerhalb** der dokumentierten Redo-Ausnahme“.

**Empfehlung:** Erster Weg (Segment-Rewrite), weil `Physical` sonst für Redo keine Aussage hat.

**Test:** ErasureOracle (§14.7) mit Muster in einem Change vor S und Muster nach S im selben Segment. Nach `erase_done` darf das Muster in keiner Datei stehen.

### CR-R3-03: Torn-Tail-Heilung verwechselt Bitfäule mit Torn-Write (R3-16)

**Stelle:** §13.1 Schritt 6 („letztes ungesiegeltes Segment: auf letztes gültiges Frame abschneiden“), F-2, F-7, `INV-LOG-2`.

**Problem:** Ein defekter Frame mitten im letzten Segment führt zum Abschneiden **aller** folgenden Frames. Sind diese mit `Always` quittiert worden und liegen nach dem letzten Catalog-Edit, hält `INV-LOG-8` nicht, weil `redo_hwm` diese Frames noch nicht kennt. Die Folge ist stiller Verlust quittierter Daten. Das widerspricht F-1 („jede Quittung mit `durable=true` bleibt erhalten“).

**Vorschlag (wie in RocksDB/LevelDB):** Torn-Tail nur heilen, wenn **nach** dem ersten ungültigen Frame kein weiterer Frame prüfbar gültig ist (Kette, CRC bzw. MAC, Stamp-Monotonie). Ist ein gültiger Frame danach vorhanden, ist es `Corrupt{RedoMidLog}`.
Neue Invariante **INV-LOG-9**. Die reine Nullbytes-Prüfung scheidet aus, weil ein echter Torn-Write Teilbytes eines Frames hinterlässt, die nicht null sind.

**Test:** Bitflip in Frame k eines Always-Segments mit Frames k+1…n gültig: Erwartung `Corrupt`, nicht stillschweigende Kürzung. Torn-Write im letzten Frame: Erwartung Heilung.

### CR-R3-04: `ViewPolicy::Wait` meldet Fehler nach dem logischen Löschpunkt (R3-17)

**Stelle:** §14.5 („Nach Timeout `Err(Blocked{views})`; der Vorgang bleibt in E6 offen“), §14.1 (logischer Punkt E1), §14.3 (`erase(...)?`).

**Problem:** Nach E1 gilt die Löschung logisch. Ein `Err` lässt den Aufrufer aber annehmen, es sei nichts geschehen. Wiederholt er die Operation, entsteht ein zweiter Auftrag mit anderem `S`.

**Vorschlag:** `Blocked` ist kein Fehler mehr, wenn E1 erfolgt ist. `erase()` liefert `Ok(Pending{id = S})`, `erase_status(id)` meldet `InProgress{phase: AwaitingViews}`. Vor E1 bleibt `Err(Blocked)` (dann ist nichts geschrieben, passend zu `HoldBlocked`).

### CR-R3-05: Blob-Schlüssel uneinheitlich (R3-18)

**Stelle:** §9.6 (`Kdf(space_dek, "ctxs blob v1" ‖ blob_no)`) gegen §15.2 (`Kdf(DEK, "ctxs blob v1" ‖ key_epoch u32 ‖ blob_no u64)`), §9.6 Domänen-Nonce `0x04` gegen E-6 (`0x05`).

**Vorschlag:** §9.6 an §15.2 angleichen. Mit CR-R3-11 entfällt der Domänen-Nonce ohnehin (siehe dort). Für den Text: „Blob-Schlüssel = Kdf(DEK, `ctxs blob v1` ‖ key_epoch ‖ blob_no); Nonce `0x04 ‖ blob_no u56 ‖ offset u32`.“ Zusätzlich E-6 als Kommentar in §9.6 zurückziehen.

### CR-R3-06: Authentisierung von FORMAT widerspricht „nur CRC32C“ (R3-19)

**Stelle:** §9.1 („Header und Footer tragen in allen Modi nur CRC32C“) gegen §9.2 (`chk` = `Mac16` für `Authenticated/Encrypted`).

**Vorschlag:** §9.1 auf „Header und Footer tragen CRC32C; Ausnahme `FORMAT`, dort `Mac16` in `Authenticated/Encrypted` (§9.2)“ ändern. Die Sicherheitsaussage von §9.1 („Authentizität entsteht über AAD-Bindung“) bleibt richtig, weil Blöcke und Records gebunden sind; nur die Ausnahme fehlt.

## 3. Mittlere und niedrige Befunde

### CR-R3-07: Redo-Frame-Grenze (R3-20)

§6.7: „Redo-Frame ≤ 64 MiB“. §9.4: „Ein Frame (≤ 32 MiB)“. **Vorschlag:** 32 MiB (= `Change`-Grenze, `redo_segment_bytes / 2`). §6.7 ändern.

### CR-R3-08: Prüfreihenfolge beim Öffnen (R3-21)

§9.2: „bevor irgendeine andere Datei gelesen wird“. §13.1 Schritt 2 liest `KEYS` vor der `FORMAT.chk`-Prüfung (`Mac16` braucht `root_key`). **Vorschlag:** §9.2 präzisieren: „Der Modus wird vor jeder anderen Datei geprüft; die Prüfung von `FORMAT.chk` (Mac16) folgt nach dem Laden von `KEYS`, vor Catalog und Daten.“

### CR-R3-09: Gesamtdeckel Memtable (R3-11, O-10)

§6.6 ergänzen: `memtable_total_bytes` (Standard 256 MiB), harte Obergrenze über alle Spaces einer Instanz. `memtable_bytes` je Space bleibt 16 MiB. §17.2 nennt dann beide Größen.

### CR-R3-10: Typbreiten (R3-22, Nonce-Breite)

- §9.4 Header: `segment_id u64`; §15.2 Nonce: `segment_id u32`. Einheitlich: `segment_id < 2^32` ist Pflicht, Header bleibt `u64` für Dateinamen-Kompatibilität; §9.4 nennt die Grenze ausdrücklich.
- §9.5 Extension-Blockart `0x1000 + id`: `Extension::id()` darf nur `0x0000…0xEFFF` sein; Konstruktor prüft das.

### CR-R3-11: Körper genau einmal versiegeln (R3-23, „nichts doppelt“)

**Problem:** Ein domänenversiegelter Körper wird zweimal AEAD-verschlüsselt: einmal im Redo (Nonce `0x03`, §10.2 Schritt 5), einmal beim Flush für den Blob (Nonce `0x05`, §12.1). Das ist doppelte Arbeit, doppelte Nonce-Pflicht und zwei Pfade für `domain_retired`. §12.3 verlangt außerdem beim GC ein Neuversiegeln, was ohne Domänenschlüssel nicht geht.

**Vorschlag:** Der Körper wird **einmal** im Committer versiegelt (`0x03`). Der Flush übernimmt die versiegelten Bytes unverändert in den Blob-Record. Der äußere Blob-Schutz (Blob-Schlüssel, Nonce `0x04`, AAD mit `blob_no` und `offset`) bindet die Position. Dann entfallen: Nonce `0x05`, das Neuversiegeln beim Flush, das Neuversiegeln bei GC (§12.3 Punkt 2 für domänenversiegelte Körper) und Erratum E-6.

**Sicherheit:** Dasselbe Ciphertext erscheint in zwei Dateien, aber nicht unter einem neuen (Schlüssel, Nonce)-Paar; es gibt keine Nonce-Wiederverwendung. `INV-KEY-1` bleibt erfüllt. Die Vernichtung des Domänenschlüssels wirkt auf beide Kopien gleich (INV-ERA-7).

**Test:** Property „Blob-Körper == Redo-Körper-Bytes“ für domänenversiegelte Ops; Absturzmatrix für Flush und GC.

### CR-R3-12: Schlüssel des Hole-Record-MAC (R3-24)

§9.6 nennt `Mac16(blob_no ‖ offset ‖ rlen ‖ 0x02)` ohne Schlüssel. **Vorschlag:** Schlüssel = Blob-Schlüssel (CR-R3-05), AAD = `space ‖ blob_no ‖ offset ‖ rlen ‖ 0x02`.

## 4. Neue Invarianten und Tests (R3-25)

| Neu | Aussage | Netz |
|---|---|---|
| INV-STAMP-3 | Keine Stamp-Wiederverwendung über Absturz (CR-R3-01) | Absturzmatrix mit `Interval` |
| INV-LOG-9 | Torn-Tail nur ohne nachfolgenden gültigen Frame (CR-R3-03) | Bitflip, Absturzmatrix |
| INV-ERA-8 | Nach `erase_done(S)` kein lebendes Redo-Segment mit Frames ≤ S (CR-R3-02); ersetzt die Formulierung von INV-ERA-2 | Orakel (Redo-Bytes) |
| INV-EPOCH-2 | Klon nutzt eigenen `root_key`; keine gemeinsamen `(Schlüssel, Nonce)`-Paare zwischen Original und Klon (CR-R3-08 in §15.2) | Property (Klon-Divergenz) |

Hinweis: Die Namen INV-STAMP-3, INV-LOG-9, INV-ERA-8 und INV-EPOCH-2 sind neu und in §18 noch nicht eingetragen.

## 5. Reihenfolge

1. CR-R3-01 (Stamp-Hochwassermarke) und CR-R3-03 (Torn-Tail): Korrektheit der Recovery, vor allen weiteren Phasen.
2. CR-R3-02 und CR-R3-04 (Erase): vor F7.
3. CR-R3-11 (einmal versiegeln) und CR-R3-05, CR-R3-12 (Blob-Format): vor G-F, weil Goldens.
4. CR-R3-06 bis CR-R3-10: Textkorrekturen und Konstanten, zusammen mit einem Format-Review.
5. O-8 bis O-10 (offene Entscheidungen, §22.2) vor F5.

Jeder Änderungsantrag braucht zwei Reviewer, aktualisierte Goldens (Anhang A) und eine Prüfung gegen die Freeze-Liste (§9.8).

## 6. Methodische Anmerkungen

- Die Leistungshypothesen (§20) sind unverändert Hypothesen. H-S3 ist präzisiert, weil der Zielwert ohne Herleitung nicht prüfbar war.
- Die Erratum-Nummern E-1…E-15 sind mehrfach belegt: Errata (§22.3) und Erase-Schritte E0 bis E7 (§14.3). Das bleibt bestehen, bis die Änderungsanträge eingearbeitet sind; CLAUDE.md §8 Punkt 17 nennt das.
- §7.7 (`hint()`): Strukturierte Fehlerfelder dürfen Bereichs- und Kennungswerte tragen (z. B. `Erased{range}`). `hint()` tut das nicht. Diese Trennung ist in der SPEC noch nicht ausdrücklich, sollte aber mit dem nächsten Format-Review aufgenommen werden.

## 7. Abgleich mit `legacy/archive/` (nur intern, nicht Teil der SPEC)

Das Archiv enthält frühere Fassungen von Bausteinen, die die neue SPEC bereits definiert. Die Gefahr ist nicht Code aus dem Archiv, sondern eine **zweite Implementierung** desselben Konzepts, sobald später weitere Teile des Repositories folgen. Das verletzt die Vorgabe, nichts doppelt zu implementieren.

| Archiv | Gegenstück in der neuen SPEC | Befund |
|---|---|---|
| `contextra-durable-fs/src/vfs.rs` | `Fs`-Port (§4), `RealFs` (§3.2), `MemFs` | Zweiter Dateisystem-Port mit eigenem Fehlermodell. Neue Teile nutzen nur `Fs`. |
| `contextra-durable-fs/src/fault_injector.rs` | `FaultFs` (§19.2) | Zweiter Fehlerinjektor. Die Absturzmatrix ist nur über `FaultFs` zulässig (Anforderung §19.2). |
| `contextra-durable-fs/src/journal.rs`, `scrub.rs` | Redo-Log (§9.4), `verify` (§12.8) | Eigene Journal- und Prüflogik. Nicht übernehmen; Bezeichnung „Journal“ in der neuen SPEC meidet das bewusst. |
| `contextra-checkpoint/src/hardlink_cloner.rs` | `checkpoint(dest, Copy \| Hardlink)` (§12.8) | Gleiche Funktion. Hardlink-Verhalten gegen `INV-ERA-2` ist in §12.8 dokumentiert. |
| `contextra-ports/` (Archiv) | `contextra-storage-ports` (§3.1) | **Gleicher Crate-Name, anderer Inhalt.** Bei einer Migration darf kein Crate dieses Namens doppelt existieren. Der Konflikt ist mit dem bekannten Namensdrift (CR-1, CLAUDE.md §2) zu lösen. |
| `contextra-mvcc/` | SSI und Transaktionspuffer sind Nicht-Ziele (NZ-3, NZ-13) | Konsistent. Nicht in den Kern übernehmen. |

Empfehlung: Vor der Migration eine Zuordnungstabelle in `IMPLEMENTATION_SPEC.md` führen, die jeden Archivbaustein als „übernehmen“, „ersetzen durch“ oder „verwerfen“ kennzeichnet. Die Tabelle gehört nicht in die SPEC, weil sie Herkunft beschreibt.
