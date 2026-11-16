# Implementierungsplan: `acta-seg`

> **Status:** verbindlich für die Crate-Überführung · **Paket:** 3 (L2)
> **Vorgaben:** [docs/spec/05_speicher.md](../../docs/spec/05_speicher.md) (5.1, 5.2, 5.5, 5.6) · [docs/spec/anhang_c_durabilitaet.md](../../docs/spec/anhang_c_durabilitaet.md) (C02 bis C05, C10, C11, C22, C24, C25) · [docs/spec/anhang_a_parameter.md](../../docs/spec/anhang_a_parameter.md) (A.3) · [docs/spec/03_systemarchitektur.md](../../docs/spec/03_systemarchitektur.md) (HR-9) · [CLAUDE.md](CLAUDE.md) · [archive/acta-seg/MANIFEST.md](../../archive/acta-seg/MANIFEST.md) · [archive/ALTLASTEN.md](../../archive/ALTLASTEN.md)

## 0. Verifikationsstand
- Ist-Stand: **Gerüst** (17 LOC) laut CLAUDE.md. Nichts ist ausgeführt (Toolchain nicht verfügbar).
- **Vorbedingung aus 5.1:** Vor Beginn der Speicherimplementierung muss der **Messbericht der Entscheidungsregel Eigenbau** vorliegen. Ohne ihn gilt die Spec unverändert: Eigenbau, Budget 6 500 LOC. Diese Entscheidung ist vor AP 1 zu dokumentieren.

## 1. Einordnung
- **Level:** 2. **Abhängigkeiten (SPEC 3.3):** `acta-base`, `acta-crypto`, `acta-fs`, `acta-gate`.
- **Budget:** 6 500 LOC (G-HUB).
- **Externe Abhängigkeiten (Whitelist):** `crc32c`, `roaring`, `sha2`, `hmac`, `thiserror`. **Nicht:** `ahash`, `blake3`, `bytes`, `crc32fast`, `lru`, `parking_lot`, `quick_cache`, `rand`, `tracing`.
- **Rolle:** Append-only Segmentspeicher für alle Partitionen K, P, DK, DP. **Schlüsselfrei** (HR-9): Schlüssel werden als Parameter `&SegmentKey` übergeben, nie erzeugt oder verwaltet.

## 2. Ist-Stand
- `SegError` als einziger Typ. Der Gesamtumfang ist Neubau gegen `archive/acta-seg/` (29 896 Zeilen, 144 Dateien). Das Archiv ist ein LSM-artiger Speicher mit `async`, Compaction und Blockformat. Das widerspricht 5.1 und ist **nicht** Vorlage, sondern nur Referenz für Einzelprüfungen.

## 3. Spec-Bezug
- **Siegelmodi (5.1):** `RecordSealed` (DEK je Dokument-Version, für P) und `SegmentSealed` (`SegmentKey`, für K, DK, DP und das Journal).
- **Partitionen (I-PART):** K, P, DK, DP werden **nie** in derselben Datei gemischt. Ein Indexsegment hat genau eine Quellart.
- **Journal-Rahmen:** `[magic u8][flags u8][len u32][hdr_crc32c u32][payload][chain_tag 32]`. `chain_tag` ist HMAC-SHA256 über alle Rahmen mit `HKDF(k_idx, "acta/wal/v1" ‖ TenantId ‖ journal_id)`. Das Journal enthält nie Klartext.
- **Flush (5.1):** Überführung in Segmente bei `wal_max_bytes` (64 MiB), `wal_max_age` (30 s) und bei jedem Saga-Schritt S3.
- **Segmentformat `format_version` 1:** Kopf 256 B, Magic `ACTASEG\0`, Datenblöcke à 64 KiB, Blockindex, Fuß mit `source_set`, Statistik, SHA-256, Fuß-MAC, End-Magic.
- **Manifest:** atomare Batches `AddSegment`, `RemoveSegment`, `SeqHwm`, `Partition`, HMAC-gekettet. Sichtbar ist ein Segment **erst** durch das Manifest (WP-3, C03, C04). Fehlgeschlagener Manifest-Schreibvorgang vergiftet den Speicher (`Poisoned(Tenant)`).
- **Handkodierte Formate:** kein `serde`, kein `bincode`. Geprüfte Feldzugriffe (G-NOSERDE-DISK). Jedes Format hat Golden-Tests und ein Fuzz-Target.
- **Ordinale (5.2):** Jede abgeleitete Struktur trägt `source_set` (typisiert: `KOrd` oder `VerOrd`). Ersatzsegment bei Betroffenheit (`source_set ∩ Ziel ≠ ∅`).
- **Schlüsselordnung P:** `[kind u8][tenant_ord u32][partition u8][cluster_id 16][ver_ref 16][!seq u64][flags u8]`. `cluster_id` ist nie im Klartext gespeichert, nur über `SubjectKey` berechenbar.
- **Z2:** `SubjectTag` steht nie im Klartext in Segmenten, Journalen oder Köpfen.
- **WP-6 (Scrub):** entfernte Dateien werden überschrieben und gelöscht (`remove_scrub` aus `acta-fs`). Flash-Reste bleiben A-FLASH.

## 4. Archiv-Entscheidungen

| Archivquelle | Entscheidung | Begründung |
|---|---|---|
| `store/*` (Blockformat, Frames) | Nur Konzept, Format neu nach 5.1 | Archiv-Format weicht ab (Blockgröße, Magic) |
| `store/src/io_seam/*`, `fsync.rs` (`async`, `std::fs`) | **Verwerfen** | `async` verboten; Dateizugriff nur über `acta-fs` |
| LSM, Memtable, Compaction (aus `store`) | **Verwerfen** (siehe `NICHT_EXTRAHIERT.md`) | 5.1: kein LSM-Baum |
| `crypto`-Journal-HMAC-Kette | Neu nach 5.1 über `acta-crypto` (`hmac`) | Schlüssel als Parameter, HR-9 |
| Legacy-WAL-Schlüsselpfad (`allow_legacy_integrity_key_fallback`, `legacy_integrity_key`, `rekey_from_legacy`, `migrate_legacy_wal`, `.wal_integrity_key`, `load_or_create_integrity_key`) | **Ersatzlos streichen** (ALTLASTEN) | Z10: keine Fallbacks, kein Schlüssel-Erzeugen in `acta-seg` (HR-9) |
| Alt-Magics (`MFxx`, `AKDF`, `AMNF`, `ASGX`, `ASGT`, `AWL2` bis `AWL4`) | Durch `ACTASEG\0` und Journal-Magic nach 5.1 ersetzen | ALTLASTEN |
| `TODO(acta-hr3)`-Importe (103 Treffer) | Typen dem Crate zuordnen, das sie braucht | HR-2/HR-3 |
| `lru`, `quick_cache`, `ahash`, `bytes`, `tracing`, `parking_lot` | **Verwerfen** | Nicht auf der Whitelist. Caches gehören zu `acta-pstore`/`acta-text`, nicht zum Speicher. |
| `rand` | Verwerfen, Zufall nur über `CryptoRng` | G3 |
| Tests mit `TODO(acta-altlast)` (8 Dateien unter `wal/`, `tests/`) | Entfernen oder neu schreiben | ALTLASTEN |

## 5. Offene Punkte und Spec-Lücken

1. **Messbericht Eigenbau (5.1, 16.5).** Vorbedingung. Eine bestehende eingebettete Engine würde nur gewählt, wenn sie die vier Kriterien erfüllt (gezieltes Entfernen, `FaultVfs`-Prüfbarkeit, Umschlag-Annahme, keine FFI). **Entscheidung dokumentieren**, bevor Code entsteht.
2. **Journal-Schlüssel.** `SegmentKey` des Journals wird als Parameter übergeben. Wer ihn erzeugt und vernichtet (`acta-vault`), ist per HR-9 geregelt. Schnittstelle im Plan festlegen.
3. **Größenbudget bei Fuß-Statistik.** Die Spec nennt die Statistik nur allgemein. Feldliste festlegen und dokumentieren.
4. **Flash-Restrisiko.** Scrub ist Annahme A-FLASH. Im Bericht ausweisen, nicht als Löschnachweis darstellen.
5. **Zeitquelle.** Flush nach `wal_max_age` braucht `Clock` (G3). Nur über den Port.

## 6. Arbeitspakete

**AP 0 – Messbericht und ADR (Vorbedingung).** Eigenbau-Entscheidung dokumentieren (5.1, Punkt 5.1).
- *Abnahme:* ADR vorhanden, Entscheidung festgehalten.

**AP 1 – Segmentformat `format_version` 1.** Kopf, Blöcke, Blockindex, Fuß. Handkodiert, geprüfte Felder. Golden-Vektor aus unabhängiger Referenz.
- *Abnahme:* Fuzz-Target für Segment (Ziel aus 16.2). Negativtests für Kopf-MAC, Prüfsumme, End-Magic.

**AP 2 – Journal.** Rahmen, HMAC-Kette mit übergebenem Schlüssel, Flush-Politik, Rotation, Scrub der Altdatei (C24, C25).
- *Abnahme:* Crash-Tests C02, C24, C25 mit Positivkontrolle. Journal enthält keinen Klartext (`T-CRY-1`).

**AP 3 – Manifest.** Atomare Batches, HMAC-Kette, Vergiftung bei Schreibfehler. Sichtbarkeit nur über Manifest (WP-3).
- *Abnahme:* C03, C04, C10, C11 bestehen. Verwaiste Dateien bleiben unsichtbar.

**AP 4 – Siegelmodi und Ersatzsegmente.** `RecordSealed` und `SegmentSealed`. Ersatz bei Betroffenheit über `source_set`. Datei-Drop bei vollständig betroffenen Segmenten.
- *Abnahme:* Test: Ersatz ohne Zielumschläge; Altdatei wird mit Scrub entfernt. `T-CRY-1` über alle Segmentdateien.

**AP 5 – Partitionen (I-PART).** Strikte Trennung K, P, DK, DP in Dateien. Typisiertes `source_set`.
- *Abnahme:* Compile-Fail oder Laufzeitprüfung verhindert gemischte Partition.

**AP 6 – Archiv-Tests und Gates.** Nur Tests übernehmen, die Format-Invarianten prüfen. Alt-Magics und Legacy-Pfade entfernen. `TODO(acta-altlast)` abarbeiten.
- *Abnahme:* `scripts/check-no-legacy.sh` meldet keine Altspur mehr. G-FS, G-NOTOKIO, G-LINT, G-HUB (≤ 6 500 LOC), G-NOSERDE-DISK grün.

**AP 7 – Archivordner.** `archive/acta-seg/` entfernen, wenn AP 0 bis 6 grün sind. Der Segment-Anteil des Journals war in `archive/acta-fs/durable-fs/` (Paket 1) als Zielcrate `acta-seg` markiert und ist dort mit zu übernehmen.

## 7. Tests und Gates

| Test | Zusage / Gate |
|---|---|
| Format-Golden und Fuzz (AP 1) | 5.1, 16.2 |
| C02, C03, C04, C10, C11, C24, C25 | Z4, `T-CRASH-*` |
| `T-CRY-1`, `T-META-*` | Z2 |
| `T-VCS-ERASE-1` (Partitionen unverändert) | Z6 |
| `T-LOG-*` (keine Kennungen in Segmentdateien) | Z15 |
| G-FS, G-NOSERDE-DISK, G-NOTOKIO, G-LINT, G-HUB | 16.6 |

## 8. Befehle
```
cargo test -p acta-seg --locked
cargo clippy -p acta-seg --all-targets --locked -- -D warnings
cargo fmt --all -- --check
cargo xtask gates
```
