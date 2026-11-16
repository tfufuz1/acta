# Implementierungsplan: `acta-vec`

> **Status:** Normativ fuer die Crate-Entwicklung · **Ziel:** Inkrementelle Ueberfuehrung aus `archive/acta-vec/` in sauberen Acta-Produktionscode.  
> **Architekturvorgaben:** [docs/SPEC.md](../../docs/SPEC.md) · [docs/spec/09_ingest_abruf_antwort.md](../../docs/spec/09_ingest_abruf_antwort.md) · [CLAUDE.md](CLAUDE.md)

---

## 1. Einordnung & Spezifikation
- **Level:** L3 (`rings.toml`)
- **Code-Budget:** 4000 LOC Produktionscode (G-HUB, strikt einzuhalten)
- **Spezifikationskapitel:** docs/spec/09_ingest_abruf_antwort.md
- **Zweck & Aufgabe:** Segmentierter Vektorindex: Exakte Flat-Suche, i8-Symmetrische Quantisierung, Q32.32 Skalarprodukt (Z7)
- **Archiv-Quelle:** [archive/acta-vec/MANIFEST.md](../../archive/acta-vec/MANIFEST.md)

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

1. **Quantisierung:** i8-Vektoren mit Skalierungsfaktor s und norm2 als u32 vorhalten
2. **Aehnlichkeit:** Deterministisches Q32.32 dot-product mit ganzzahligem isqrt gemaess SPEC 9.3
3. **Exakte Flat-Suche ueber Kandidatenmenge (K:** V ∩ A_K, P: A_P ∩ Live)
4. **Segmentorganisation:** Paralleler Scan ueber Segmente mit fester Gleichstandsordnung
5. **Bereinigung:** ahash, blake3, parking_lot entfernen

---

## 4. Testkatalog & Freigabekriterien
1. **Zusagen-Konformitaet:** Mindestens ein zugeordneter Test gemaess Zusagenmatrix 16.3 / G-ZUSAGE.
2. **Positivkontrolle (16.4):** Durabilitaets- und Messpruefungen muessen bei defektem Verhalten deterministisch fehlschlagen.
3. **Workspace-Gates:** `cargo xtask gates` muss zu jedem Schritt vollstaendig gruen sein.
4. **Clippy & Lints:** `cargo clippy -p acta-vec --all-targets -- -D warnings` ohne Befunde.
5. **Archiv-Bereinigung:** Nach vollstaendiger Ueberfuehrung und Validierung wird der Ordner `archive/acta-vec/` geloescht.
