# Implementierungsplan: `acta-vec`

> **Status:** verbindlich für die Crate-Überführung · **Paket:** 3 (L3)
> **Vorgaben:** [docs/spec/09_ingest_abruf_antwort.md](../../docs/spec/09_ingest_abruf_antwort.md) (9.3, 9.4) · [docs/spec/16_test_freigabe.md](../../docs/spec/16_test_freigabe.md) (16.3 Z7) · [docs/spec/anhang_a_parameter.md](../../docs/spec/anhang_a_parameter.md) (A.6) · [docs/spec/11_inferenz.md](../../docs/spec/11_inferenz.md) (`Embedder`, `ModelFingerprint`) · [CLAUDE.md](CLAUDE.md) · [archive/acta-vec/MANIFEST.md](../../archive/acta-vec/MANIFEST.md)

## 0. Verifikationsstand
- Ist-Stand: **Gerüst** (17 LOC). Nichts ausgeführt.
- Archivbewertung: 390 Gleitkomma-Treffer, 10 `unsafe` (der SIMD-Pfad). Beides ist für Z7 und G-UNSAFE nicht übernehmbar.

## 1. Einordnung
- **Level:** 3. **Abhängigkeiten (SPEC 3.3):** `acta-base`, `acta-crypto`, `acta-gate`, `acta-vault`, `acta-seg`.
- **Budget:** 4 000 LOC (G-HUB).
- **Externe Abhängigkeiten:** `roaring`, `thiserror`.
- **Rolle:** Segmentierter Vektorindex, exakte Flat-Suche auf `i8`-Vektoren, Lineage.

## 2. Ist-Stand
- Nur `VecError`. Archivbestand: `vector` (6 413 Zeilen, 39 Dateien).

## 3. Spec-Bezug
- **Quantisierung (9.3):** Der Embedder liefert `f32`. Skalierung `s = 127 / max|xᵢ|`, `qᵢ = round_half_even(xᵢ·s)` als `i8`. `norm2 = Σ qᵢ²` als `u32`. **Gespeichert werden die Bytes.** Z7 gilt ab diesen Bytes.
- **Ähnlichkeit:** `dot = Σ qᵢ·dᵢ` (`i32`), `sim = (dot << 32) / isqrt(norm2_q · norm2_d)` als Q32.32. `isqrt` aus `acta-base`.
- **Suche:** Exakte Flat-Suche je Segment. Segmente parallel, Zusammenführung in fester Reihenfolge. Top-k mit Gleichstand wie 9.2.
- **Kandidatenmenge:** K: `V ∩ A`. P: `A_P ∩ Live` (nicht im Gate).
- **Zielwerte [Q]:** 1 Mio. Chunks, 768 Dimensionen, p95 unter 150 ms je Anfrage auf 8 Kernen. Recall@10 der Quantisierung ≥ 0,99 gegenüber `f32`.
- **ANN:** Nur nach Messbericht, nur als Kandidatenerzeuger mit exaktem Nachrang, und dann ist Z7 für diesen Pfad **ausgesetzt** (als nicht deterministisch ausweisen).
- **Z7 und Z16:** Gleiche Eingabe, gleicher Abfragevektor, gleiche Sichtklasse liefert gleiche Ergebnisse. Unzugängliche Dokumente beeinflussen weder Ergebnis noch Zählwerte.
- **Z1:** `ErasedView` vor jeder Ausgabe. Vektoren aus gelöschten Quellen werden nicht gelesen.

## 4. Archiv-Entscheidungen

| Archivquelle | Entscheidung | Begründung |
|---|---|---|
| Gleitkommasuche (`float`: 390 Treffer) | Durch `i8`-Ganzzahl ersetzen | Z7, 9.3 |
| SIMD-Kerne (`unsafe`: 10 Treffer, `avx2`, `avx512`, `neon` in `acta-sys/simd/`) | **Nicht hier.** Siehe `acta-sys`, Punkt 1: nicht in Paket 1 | Budget und Default-deny. Eine Beschleunigung braucht einen ADR. |
| `serde` (13 Treffer) | Verwerfen, handkodieren | G-NOSERDE-DISK |
| `async` (18 Treffer) | Verwerfen | G-NOTOKIO |
| `unsafe` (10 Treffer) | Verwerfen | G-UNSAFE |
| `ahash`, `blake3`, `parking_lot`, `tracing` | Verwerfen | Whitelist |
| `hr6` (5 Treffer) | Entfernen | HR-6 |
| `unwrap`/`expect` (158 Treffer) | Entfernen im Produktionscode | G-LINT |

## 5. Offene Punkte und Spec-Lücken

1. **Skalarprodukt ohne SIMD.** Mit `i8` ist das Ergebnis exakt und plattformunabhängig. Ob die skalare Schleife die 150-ms-Zielwerte erreicht, ist Messfrage (16.5). Bei Verfehlung: ADR, nicht stillschweigend SIMD.
2. **Float an der Quantisierungsgrenze.** `round_half_even` auf `f32`-Produkten ist deterministisch auf IEEE-754, aber die Zielplattform muss das bestätigen. Test auf x86_64 und aarch64.
3. **Modellwechsel und `IndexGen`.** Neuer Embedder ändert die `IndexGen` (11.4). Verhalten bei Mischbestand während des Neuaufbaus ist in 11.4 nicht ausgeführt. Festlegen.
4. **`norm2`-Überlauf.** 768 Dimensionen mit `i8` ergeben höchstens `768 · 127² ≈ 1,2 · 10⁷`, passt in `u32`. Bei anderer Dimension Prüfung einbauen.

## 6. Arbeitspakete

**AP 1 – Quantisierung.** `s`, `round_half_even`, `i8`-Bytes, `norm2` als `u32`. Eingabe nur `f32`-Slice, Ausgabe nur Bytes.
- *Abnahme:* Golden-Vektoren aus unabhängiger Referenz. Recall-Test ≥ 0,99 gegen `f32` auf synthetischem Korpus (keine echten Daten, 16.1).

**AP 2 – Exakte Ähnlichkeit.** `dot`, `sim` als Q32.32, `isqrt` aus `acta-base`.
- *Abnahme:* Eigenschaftstests: Symmetrie, Skalierungsinvarianz des Winkels, Gleichstand-Regel.

**AP 3 – Segmentierter Index.** Vektor-Segmente über `acta-seg`. Strikte Partitionstrennung (K, DK, DP).
- *Abnahme:* Ersatzsegment bei Lineage-Treffer.

**AP 4 – Suche und Sichtklasse.** Kandidatenmengen `V ∩ A` und `A_P ∩ Live`. Top-k mit fester Zusammenführung.
- *Abnahme:* `T-ACL-RANK`-Äquivalent für Vektoren: unzugängliche Dokumente verändern weder Ranking noch Zählwerte.

**AP 5 – Messung.** p95 150 ms, 1 Mio. Chunks, 768 Dimensionen, 8 Kerne als Messbericht.
- *Abnahme:* Bericht mit Commit-Stand. Abweichungen als ADR.

**AP 6 – Gates und Archivordner.** G-UNSAFE, G-NOSERDE-DISK, G-NOTOKIO, G-LINT, G-HUB (≤ 4 000 LOC). `archive/acta-vec/` nach Abschluss von AP 1 bis 5 entfernen. Der SIMD-Teil bleibt im Archiv von `acta-sys` bis zur ADR-Entscheidung.

## 7. Tests und Gates

| Test | Zusage / Gate |
|---|---|
| Golden-Quantisierung (AP 1) | 9.3 |
| Recall ≥ 0,99 (AP 1) | 9.3 [Q] |
| `T-TT-*` x86_64 und aarch64 | Z7 |
| `T-ACL-RANK` (Vektor-Variante) | Z16 |
| `T-GATE-*` | Z1 |
| G-UNSAFE, G-NOSERDE-DISK, G-NOTOKIO, G-LINT, G-HUB | 16.6 |

## 8. Befehle
```
cargo test -p acta-vec --locked
cargo clippy -p acta-vec --all-targets --locked -- -D warnings
cargo fmt --all -- --check
cargo xtask gates
```
