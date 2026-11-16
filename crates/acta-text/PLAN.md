# Implementierungsplan: `acta-text`

> **Status:** verbindlich für die Crate-Überführung · **Paket:** 3 (L3)
> **Vorgaben:** [docs/spec/09_ingest_abruf_antwort.md](../../docs/spec/09_ingest_abruf_antwort.md) (9.1.1, 9.2, 9.4, 9.7) · [docs/spec/16_test_freigabe.md](../../docs/spec/16_test_freigabe.md) (16.2, 16.3 Z7) · [docs/spec/anhang_a_parameter.md](../../docs/spec/anhang_a_parameter.md) (A.5, A.6: BM25, `k_text`, `rrf_k`) · [docs/spec/03_systemarchitektur.md](../../docs/spec/03_systemarchitektur.md) (3.7 Lineage, 3.8) · [CLAUDE.md](CLAUDE.md) · [archive/acta-text/MANIFEST.md](../../archive/acta-text/MANIFEST.md)

## 0. Verifikationsstand
- Ist-Stand: **Gerüst** (17 LOC). Nichts ausgeführt.
- Archivbewertung über Manifest: 131 Gleitkomma-Treffer. Das ist der größte Befund für Z7.

## 1. Einordnung
- **Level:** 3. **Abhängigkeiten (SPEC 3.3):** `acta-base`, `acta-crypto`, `acta-gate`, `acta-vault`, `acta-seg`.
- **Budget:** 5 000 LOC (G-HUB).
- **Externe Abhängigkeiten:** `roaring`, `unicode-normalization`, `unicode-segmentation`, `thiserror`. Alle auf der Whitelist (3.8).
- **Rolle:** Segmentierter Volltextindex. BM25 in Festkomma Q32.32, sichtklassenrelative Statistik, Lineage, DE/EN-Normalisierung.

## 2. Ist-Stand
- Nur `TextError`. Archivbestand: `text` (14 313 Zeilen, 60 Dateien).

## 3. Spec-Bezug
- **BM25 (9.2):** `k1 = 1,2`, `b = 0,75`. Festkomma Q32.32, Rundung half-even. `idf(t) = ln(1 + (N − df + 0,5)/(df + 0,5))` über `fx_ln` aus `acta-base`. **Kein `f32`/`f64` in Ranking-Pfaden.**
- **Statistik:** `N = |V ∩ A|`, `df(t) = |Postings(t) ∩ V ∩ A|`, `avgdl = Σ dl / N`. `A` ist die effektive Zugriffsmenge der Sichtklasse. Die Längensumme je (Commit, Sichtklasse) liegt in einem DK-Cache.
- **Gleichstand:** (Score absteigend, `KOrd` aufsteigend, Chunk aufsteigend).
- **Ebene P:** eigene DP-Indizes. `idf` aus der K-Statistik der Sichtklasse, Terme ohne K-Eintrag: `df = 0`. Fusion erfolgt über Ränge, nie über Scores.
- **Normalisierung:** Casefolding, UAX #29, Stoppwörter und leichter Stemmer für DE und EN. Variante samt Hash der Unicode-Tabellen und Stemmer-Version ist Teil der `retrieval_cfg`. Änderung erzeugt neue `IndexGen`.
- **Chunks (9.1.1):** Chunk-Kennung `(ObjectId bzw. VerRef, chunk u32)`. Chunking selbst liegt in `acta-ingest`, nicht hier.
- **Z7:** Byte-identische Ergebnisse x86_64 und aarch64 (`T-TT-*`, `T-TT-FX`).
- **Z16:** Rangfolge, Statistik und Zählwerte dürfen nicht von unzugänglichen Dokumenten abhängen (`T-ACL-RANK`).
- **Z1:** Jeder Lesepfad ruft `ErasedView` vor Ausgabe auf. Lineage-Treffer werden verworfen.
- **Z13:** Ableitungen tragen Lineage.

## 4. Archiv-Entscheidungen

| Archivquelle | Entscheidung | Begründung |
|---|---|---|
| `text` (BM25, `f32`/`f64`) | Gleitkomma durch Q32.32 ersetzen | Z7. Der Archiv-Code ist als Vorlage nur für Struktur verwendbar. |
| `serde` (36 Treffer) | Verwerfen, handkodieren | G-NOSERDE-DISK |
| `bincode`, `posting_list.rs` `bincode`-Fallback, `legacy`-Pfade | Verwerfen | ALTLASTEN (Z10) |
| `async` (49 Treffer), `futures`, `itoa` | Verwerfen | G-NOTOKIO, Whitelist |
| `unsafe` (6 Treffer) | Verwerfen | G-UNSAFE, nur `acta-sys` |
| `parking_lot`, `bytes`, `tracing` | Verwerfen | Whitelist |
| `hr6` (8 Treffer: `include!`, `#[deprecated]`) | Entfernen | HR-6 |
| `std_fs` (1 Treffer) | über `acta-fs` | G-FS |
| `liste_a` (1 Treffer) | Entfernen | G-WORD |
| `unwrap`/`expect` (106 Treffer) | Entfernen im Produktionscode | G-LINT |

## 5. Offene Punkte und Spec-Lücken

1. **Stemmer-Bibliothek.** 9.2 verlangt einen leichten Stemmer für DE und EN. Keine Stemmer-Bibliothek ist auf der Whitelist (3.8). **Entscheidung nötig:** eigener Stemmer (Regeltabelle, eigene Verantwortung) oder ADR für Aufnahme. Die Stemmer-Version ist Teil der `retrieval_cfg`, also muss die Entscheidung vor dem ersten Index stehen.
2. **Unicode-Tabellen-Hash.** Variante samt Hash muss festgelegt werden. `unicode-normalization` und `unicode-segmentation` haben eigene Tabellenversionen. Die Abhängigkeitsversion ist als Konstante festzuhalten.
3. **`idf` über `fx_ln`.** Die Spec verlangt 12 Terme der atanh-Reihe. Die Implementierung liegt in `acta-base::fx` (laut CLAUDE.md mit Goldvektoren gegen Orakel). Abnahme aus `acta-base`, AP 5, übernehmen.
4. **Sichtklassen-Statistik-Cache.** Die Längensumme wird bei Änderung der Zellen-Bitmaps neu berechnet. Auslöser und Invalidierung nicht festgelegt (Lücke in 9.2, nur „bei Änderung“).

## 6. Arbeitspakete

**AP 1 – Festkomma-BM25.** `idf`, `tf`-Normierung, Score in Q32.32 mit half-even. Keine Gleitkomma-Werte.
- *Abnahme:* Goldvektoren `T-TT-FX` (BM25-Wert bestätigt laut `acta-base`). Plattform-Test x86_64/aarch64 als Pflicht, bis dahin ausgewiesen.

**AP 2 – Normalisierung und Stemmer.** Entscheidung aus Punkt 5.1 umsetzen. Casefolding, UAX #29, Stoppwörter.
- *Abnahme:* Golden-Set DE und EN. Stemmer-Version in `retrieval_cfg`.

**AP 3 – Segmentierter Index.** Postings-Segmente über `acta-seg` (`SegmentSealed` für DK, `RecordSealed` für DP). Partitionen strikt getrennt (I-PART).
- *Abnahme:* Kein Mischen K und P in einer Datei. Segmentersatz bei Lineage-Treffer.

**AP 4 – Sichtklassen-Statistik.** `N`, `df`, `avgdl` über `V ∩ A`. DK-Cache der Längensumme.
- *Abnahme:* `T-ACL-RANK`: Hinzufügen oder Entfernen eines unzugänglichen Dokuments ändert Ergebnisse, Zählwerte und Fehlerbilder nicht.

**AP 5 – Gate und Löschung.** `ErasedView` vor Ausgabe, Ersatzsegmente bei Lineage-Treffer. Tombstone-Verhalten (Archiv-Tests `purge`, `tombstone`).
- *Abnahme:* `T-GATE-<Lesepfad>` für Suche. Nebenläufigkeitstest `T-CONC-GATE` (loom) für Suche gegen Löschung.

**AP 6 – Archiv-Tests.** Purge-, Tombstone- und Nebenläufigkeitstests aus `archive/acta-text/` als Oracle. Ohne `unwrap` im Produktionspfad.
- *Abnahme:* Alle Tests grün mit Positivkontrolle.

**AP 7 – Gates und Archivordner.** G-FS, G-NOSERDE-DISK, G-NOTOKIO, G-UNSAFE, G-LINT, G-HUB (≤ 5 000 LOC). `archive/acta-text/` entfernen nach AP 1 bis 6.

## 7. Tests und Gates

| Test | Zusage / Gate |
|---|---|
| `T-TT-FX` (BM25-Goldvektoren) | Z7 |
| `T-TT-*` x86_64 und aarch64 | Z7 (aarch64 offen, siehe `acta-base`) |
| `T-ACL-RANK` | Z16 |
| `T-GATE-*` (Suche), `T-CONC-GATE` | Z1 |
| `T-GROUND-*` (Lexikalischer Stützungstest, Struktur) | Z17 (Struktur, D) |
| G-FS, G-NOSERDE-DISK, G-NOTOKIO, G-LINT, G-HUB | 16.6 |

## 8. Befehle
```
cargo test -p acta-text --locked
cargo clippy -p acta-text --all-targets --locked -- -D warnings
cargo fmt --all -- --check
cargo xtask gates
```
