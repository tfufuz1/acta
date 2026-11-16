# Implementierungsplan: `acta-seg`

> **Status:** Normativ fuer die Crate-Entwicklung · **Ziel:** Inkrementelle Ueberfuehrung aus `archive/acta-seg/` in sauberen Acta-Produktionscode.  
> **Architekturvorgaben:** [docs/SPEC.md](../../docs/SPEC.md) · [docs/spec/05_speicher.md](../../docs/spec/05_speicher.md) · [CLAUDE.md](CLAUDE.md)

---

## 1. Einordnung & Spezifikation
- **Level:** L2 (`rings.toml`)
- **Code-Budget:** 6500 LOC Produktionscode (G-HUB, strikt einzuhalten)
- **Spezifikationskapitel:** docs/spec/05_speicher.md
- **Zweck & Aufgabe:** Versiegelter, schluesselfreier Segmentspeicher: WAL/Journal, Segmente (K, P, DK, DP), Manifest, Ersatzsegmente
- **Archiv-Quelle:** [archive/acta-seg/MANIFEST.md](../../archive/acta-seg/MANIFEST.md)

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

1. **HR-9 strikt einhalten:** acta-seg ist voellig schluesselfrei; Schluessel &SegmentKey kommen ausschliesslich als Parameter hinein
2. **Kanonische Formate:** 64 KiB Bloecke, versiegelt mit Umschlag v1; handkodiert ohne serde (G-NOSERDE-DISK)
3. **Segment-Partitionierung (ADR-33):** K (global), P (Cluster), DK (Lineage), DP (ClusterId)
4. **Altlasten tilgen:** Legacy-WAL-Schluesselpfade und Alt-Magics entfernen (siehe ALTLASTEN.md)
5. **ErasureLayer implementieren:** Gezielter Segmentersatz und Drop fuer Loeschstufe S3

---

## 4. Testkatalog & Freigabekriterien
1. **Zusagen-Konformitaet:** Mindestens ein zugeordneter Test gemaess Zusagenmatrix 16.3 / G-ZUSAGE.
2. **Positivkontrolle (16.4):** Durabilitaets- und Messpruefungen muessen bei defektem Verhalten deterministisch fehlschlagen.
3. **Workspace-Gates:** `cargo xtask gates` muss zu jedem Schritt vollstaendig gruen sein.
4. **Clippy & Lints:** `cargo clippy -p acta-seg --all-targets -- -D warnings` ohne Befunde.
5. **Archiv-Bereinigung:** Nach vollstaendiger Ueberfuehrung und Validierung wird der Ordner `archive/acta-seg/` geloescht.
