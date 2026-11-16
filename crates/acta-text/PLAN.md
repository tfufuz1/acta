# Implementierungsplan: `acta-text`

> **Status:** Normativ fuer die Crate-Entwicklung · **Ziel:** Inkrementelle Ueberfuehrung aus `archive/acta-text/` in sauberen Acta-Produktionscode.  
> **Architekturvorgaben:** [docs/SPEC.md](../../docs/SPEC.md) · [docs/spec/09_ingest_abruf_antwort.md](../../docs/spec/09_ingest_abruf_antwort.md) · [CLAUDE.md](CLAUDE.md)

---

## 1. Einordnung & Spezifikation
- **Level:** L3 (`rings.toml`)
- **Code-Budget:** 5000 LOC Produktionscode (G-HUB, strikt einzuhalten)
- **Spezifikationskapitel:** docs/spec/09_ingest_abruf_antwort.md
- **Zweck & Aufgabe:** Segmentierter Volltextindex: BM25 in Festkomma Q32.32, DE/EN-Stemmer, sichtklassenrelative Statistik (Z7, Z16)
- **Archiv-Quelle:** [archive/acta-text/MANIFEST.md](../../archive/acta-text/MANIFEST.md)

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

1. **BM25 in Q32.32:** Berechnung ueber fx_ln mit festem k1=1.2, b=0.75 ohne Gleitkomma (Z7)
2. **Sichtklassen-Statistik:** N und df relativ zu V ∩ A; Vermeidung von Seitenkanaelen gemaess Z16
3. **Segmentierte Posting-Listen:** Handkodiertes Plattenformat mit KOrd-Lineage
4. ErasureLayer-Integration fuer DP-Textsegmente und Cache-Invalidierung
5. **Bereinigung:** bincode, futures, parking_lot entfernen

---

## 4. Testkatalog & Freigabekriterien
1. **Zusagen-Konformitaet:** Mindestens ein zugeordneter Test gemaess Zusagenmatrix 16.3 / G-ZUSAGE.
2. **Positivkontrolle (16.4):** Durabilitaets- und Messpruefungen muessen bei defektem Verhalten deterministisch fehlschlagen.
3. **Workspace-Gates:** `cargo xtask gates` muss zu jedem Schritt vollstaendig gruen sein.
4. **Clippy & Lints:** `cargo clippy -p acta-text --all-targets -- -D warnings` ohne Befunde.
5. **Archiv-Bereinigung:** Nach vollstaendiger Ueberfuehrung und Validierung wird der Ordner `archive/acta-text/` geloescht.
