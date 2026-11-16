# Implementierungsplan: `acta-graph`

> **Status:** Normativ fuer die Crate-Entwicklung · **Ziel:** Inkrementelle Ueberfuehrung aus `archive/acta-graph/` in sauberen Acta-Produktionscode.  
> **Architekturvorgaben:** [docs/SPEC.md](../../docs/SPEC.md) · [docs/spec/05_speicher.md](../../docs/spec/05_speicher.md) · [CLAUDE.md](CLAUDE.md)

---

## 1. Einordnung & Spezifikation
- **Level:** L3 (`rings.toml`)
- **Code-Budget:** 4000 LOC Produktionscode (G-HUB, strikt einzuhalten)
- **Spezifikationskapitel:** docs/spec/05_speicher.md, 09_ingest_abruf_antwort.md, anhang_f_adr.md
- **Zweck & Aufgabe:** Segmentierter Graphindex: CSR-Struktur, Festkomma-PPR Q32.32, Pfad-RAG, Entitaetsextraktion, Lineage (ADR-34)
- **Archiv-Quelle:** [archive/acta-graph/MANIFEST.md](../../archive/acta-graph/MANIFEST.md)

---

## 2. Abgrenzung & Architektur-Leitplanken
- **Keine Sammel-Crates (HR-1 bis HR-3):** Typen gehoeren dorthin, wo sie gebraucht werden. Keine Typen-Hubs.
- **Fehlerbehandlung (HR-4):** Genau ein eigener Fehlertyp (`thiserror`, `#[non_exhaustive]`, kein `Other(String)`), implementiert `ClassifiedError`.
- **Synchroner Kern (G-NOTOKIO):** Level 0 bis 7 sind strikt synchron. Kein `tokio`, kein `async`.
- **Speicher- und Krypto-Hygiene (G-CRY, G-UNSAFE, G-NOSERDE-DISK):**
  - Plattenformate sind handkodiert (Big-Endian, Umschlag v1), kein `serde`/`bincode`.
  - `unsafe` ist ausschliesslich in `acta-sys` erlaubt (mit `// SAFETY:`-Dokumentation).
  - Keine verbotenen Traits (`Clone`, `Debug`, `Serialize`, `Default`) auf Schluesseltypen.
- **Wortfilter (G-WORD):** Keine verbotenen Begriffe aus Liste A (siehe SPEC 1.6).

---

## 3. Umbau- und Implementierungsschritte

1. CSR-Struktur (Compressed Sparse Row) synchron und versiegelt implementieren
2. PPR (Personalized PageRank) deterministisch in Festkomma Q32.32 gemaess Z7 umsetzen (kein f32/f64)
3. Pfad-RAG und Entitaetsextraktion fuer K- und P-Dokumente (P-Graph nur in DP-Segmenten, 5.5)
4. ErasureLayer fuer Graph-Segmente und Takedown-Unterstuetzung integrieren
5. **Bereinigung:** arc-swap, scc, ahash, parking_lot eliminieren; compat/ aufloesen

---

## 4. Testkatalog & Freigabekriterien
1. **Zusagen-Konformitaet:** Mindestens ein zugeordneter Test gemaess Zusagenmatrix 16.3 / G-ZUSAGE.
2. **Positivkontrolle (16.4):** Durabilitaets- und Messpruefungen muessen bei defektem Verhalten deterministisch fehlschlagen.
3. **Workspace-Gates:** `cargo xtask gates` muss zu jedem Schritt vollstaendig gruen sein.
4. **Clippy & Lints:** `cargo clippy -p acta-graph --all-targets -- -D warnings` ohne Befunde.
5. **Archiv-Bereinigung:** Nach vollstaendiger Ueberfuehrung und Validierung wird der Ordner `archive/acta-graph/` geloescht.
