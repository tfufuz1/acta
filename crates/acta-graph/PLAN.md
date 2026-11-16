# Implementierungsplan: `acta-graph`

> **Status:** **Blockiert bis ADR.** Nicht in SPEC 3.3, nicht im Workspace-Budget. · **Paket:** 3, Zusatz (L3, nur nach ADR)
> **Vorgaben:** [docs/ADR-VORSCHLAG-graph.md](../../docs/ADR-VORSCHLAG-graph.md) · [docs/spec/03_systemarchitektur.md](../../docs/spec/03_systemarchitektur.md) (3.3, 3.8) · [docs/spec/05_speicher.md](../../docs/spec/05_speicher.md) (5.5 DK/DP) · [docs/spec/09_ingest_abruf_antwort.md](../../docs/spec/09_ingest_abruf_antwort.md) (9.4 Graph-Listen) · [docs/spec/16_test_freigabe.md](../../docs/spec/16_test_freigabe.md) (Z7: `ppr_reference_validation`, `proptest_csr_invariants`) · [CLAUDE.md](CLAUDE.md) · [archive/acta-graph/MANIFEST.md](../../archive/acta-graph/MANIFEST.md) · [archive/_reserve/graph-ext/MANIFEST.md](../../archive/_reserve/graph-ext/MANIFEST.md)

## 0. Verifikationsstand und Status
- Ist-Stand: **Gerüst** (17 LOC), **kein** Teil der Spec 3.3. `CLAUDE.md` und `NICHT_EXTRAHIERT.md` sagen: Aufnahme nur nach ADR.
- Die Spec selbst erwähnt Graph in 5.5 und 9.4 als **Erweiterung** (Fassung 2.1, ADR-34 Graphindex). Die Abschnitte sind damit normativ genug für Ableitungen (D), aber die Level-Zuordnung in 3.3 fehlt. Das ist der Blocker.
- Nichts ausgeführt.

## 1. Einordnung (vorgeschlagen, nicht bindend)
- **Level:** 3 (wie `rings.toml` vorsieht). **Abhängigkeiten:** `acta-base`, `acta-crypto`, `acta-gate`, `acta-vault`, `acta-seg`.
- **Budget:** 4 000 LOC (Vorschlag, Teil des ADR).
- **Externe Abhängigkeiten (Whitelist):** `roaring`, `thiserror`. **Nicht auf der Whitelist:** `ahash`, `arc-swap`, `bincode`, `blake3`, `bytes`, `parking_lot`, `scc`, `tracing`. Jede Aufnahme braucht ADR (3.8).

## 2. Ist-Stand
- Nur `GraphError`. Archivbestand: `graph` (19 801 Zeilen, 89 Dateien) plus `_reserve/graph-ext` (13 188 Zeilen, 52 Dateien). Beides nicht im Workspace.

## 3. Spec-Bezug (Erweiterung 5.5, 9.4)
- **DK-Graphsegmente:** CSR-Struktur, Entitäten und Relationen aus K. Lineage `KOrd`. Takedown erzeugt Ersatzsegmente (6.7).
- **DP-Graphsegmente:** Entitätsextraktion aus P-Dokumenten erzeugt personenbezogene Daten. **Nur in DP-Segmenten**, nie in DK (5.5, „kein Graph aus P in DK“).
- **Traversierung (9.4):** strikt innerhalb `K: V ∩ A_K` oder `P: A_P ∩ Live`.
- **PPR und Pfad-RAG:** deterministisch in Festkomma Q32.32, ganzzahliger Gleichstand (Ordinal aufsteigend). Gilt nach Z7.
- **Fusion:** Graph-K und Graph-P als zwei zusätzliche Listen in RRF (9.4: sechs Listen).
- **Z1, Z13, Z16:** Gate vor Ausgabe, Lineage Pflicht, Zugriffskontrolle vor Traversierung.

## 4. Archiv-Entscheidungen (nur nach ADR)

| Archivquelle | Entscheidung | Begründung |
|---|---|---|
| `csr`, `ppr`, `path_rag` | Neu auf Festkomma, Gleitkomma entfernen | Z7. Archiv: 302 Float-Treffer. |
| `provider`, `provenance`, `purger`, `entity_extraction`, `storage`, `types` | Nur nach ADR. Umbau nach Spec | `purger` muss über `ErasureLayer` laufen (G-ERA) |
| `compat/` (Brückenmodul) | **Verwerfen** | HR-6 verbietet Brücken-Module |
| `arc-swap`, `scc`, `ahash`, `bytes`, `tracing`, `parking_lot` | Verwerfen oder ADR | Whitelist (3.8) |
| `bincode`, `serde` (45 Treffer) | Verwerfen, handkodieren | G-NOSERDE-DISK |
| `async` (423 Treffer) | Verwerfen | G-NOTOKIO |
| `unwrap`/`expect` (801 Treffer) | Entfernen im Produktionscode | G-LINT |
| `_reserve/graph-ext` | Nur nach ADR, separat | Erweiterungen sind nicht Teil der Spec-Basisfassung |

## 5. Offene Punkte (vor ADR zu klären)

1. **Level-Zuordnung.** 3.3 nennt kein Graph-Crate. Entweder Spec-Änderung (Auftraggeber) oder ADR mit Begründung, dass Graph als Erweiterung der DK/DP-Ableitungen gilt. Ohne diese Entscheidung ist der Crate im Workspace unzulässig (0.4 Regel 1, Default-deny).
2. **Gleitkomma in PPR und Gewichten.** Z7 verlangt Festkomma. Die Archiv-Version nutzt Float. Mit der Umstellung ändern sich die Ergebnisse. Referenzwerte (`ppr_reference_validation`) neu erzeugen.
3. **Whitelist-fremde Abhängigkeiten.** `arc-swap` und `scc` dienen dem lesefreien Schnappschuss (3.6). Stattdessen `Arc` mit atomarem Zeigertausch über `std::sync` prüfen. Falls nicht ausreichend, ADR.
4. **Entitätsextraktion.** Erzeugt personenbezogene Daten aus P. Braucht Prüfung durch `acta-pii` (L1) und Trennung nach Z5. Ohne Klärung nicht implementieren.
5. **Speicherung.** Graph-Segmente über `acta-seg`. Partitionsregel: DK und DP nie gemischt (I-PART).

## 6. Arbeitspakete

**AP 0 – ADR.** Level-Zuordnung, Budget, Abhängigkeiten, Umgang mit Erweiterungen. Ohne ADR kein weiterer Schritt.
- *Abnahme:* ADR angenommen, `rings.toml` und `docs/spec/03_systemarchitektur.md` konsistent.

**AP 1 – CSR-Struktur.** Lesefreie Snapshots über `Arc` und atomaren Zeigertausch. Keine Gleitkomma-Werte.
- *Abnahme:* Proptest `proptest_csr_invariants`. Keine Zeigerzugriffe ohne Epoche.

**AP 2 – PPR in Festkomma.** Iteration mit fester Anzahl, Rundung half-even, Gleichstand nach Ordinal.
- *Abnahme:* `ppr_reference_validation` gegen unabhängige Referenz (Python-Skript). `ppr_snapshot_isolation` (Z16).

**AP 3 – Pfad-RAG.** Pfadbewertung in Q32.32, Traversierung innerhalb der Zugriffsmenge.
- *Abnahme:* Test: Unzugängliche Knoten erscheinen in keinem Pfad, auch nicht als Zwischenknoten (Z16).

**AP 4 – Entitätsextraktion und DP-Trennung.** Nur nach Klärung Punkt 5.4. Extraktion aus P nur in DP-Segmenten.
- *Abnahme:* Test: Kein Graphsegment mit P-Ursprung in DK.

**AP 5 – Löschung über `ErasureLayer`.** Ersatzsegmente bei Lineage-Treffer (G-ERA). Kein direkter Löschpfad.
- *Abnahme:* `T-GATE-*` für Graph-Lesepfade. Positivkontrolle der Löschung.

**AP 6 – Archivordner.** Erst nach ADR und AP 1 bis 5. Vorher bleibt `archive/acta-graph/` unangetastet.

## 7. Tests und Gates

| Test | Zusage / Gate |
|---|---|
| `ppr_reference_validation` | Z7 |
| `proptest_csr_invariants` | Z7 |
| `ppr_snapshot_isolation` | Z16 |
| `T-GATE-*` (Graph-Pfade) | Z1 |
| G-ERA, G-NOTOKIO, G-LINT, G-HUB, G-NOSERDE-DISK | 16.6 |

## 8. Befehle
```
cargo test -p acta-graph --locked
cargo clippy -p acta-graph --all-targets --locked -- -D warnings
cargo fmt --all -- --check
cargo xtask gates
```
