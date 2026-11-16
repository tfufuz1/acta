# Implementierungsplan: `acta-history`

> **Status:** Normativ fuer die Crate-Entwicklung · **Ziel:** Inkrementelle Ueberfuehrung aus `archive/acta-history/` in sauberen Acta-Produktionscode.  
> **Architekturvorgaben:** [docs/SPEC.md](../../docs/SPEC.md) · [docs/spec/06_wissensebene.md](../../docs/spec/06_wissensebene.md) · [CLAUDE.md](CLAUDE.md)

---

## 1. Einordnung & Spezifikation
- **Level:** L3 (`rings.toml`)
- **Code-Budget:** 6500 LOC Produktionscode (G-HUB, strikt einzuhalten)
- **Spezifikationskapitel:** docs/spec/06_wissensebene.md
- **Zweck & Aufgabe:** Git-aehnliche Versionskontrolle der Wissensebene K: Immutable Blobs, Trees, Commits, Refs, Sichtbarkeitsmengen, Takedown
- **Archiv-Quelle:** [archive/acta-history/MANIFEST.md](../../archive/acta-history/MANIFEST.md)

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

1. **Objektmodell:** Blobs, Trees, Commits mit SHA-256 Hash-Identitaet und blob_salt (2.4)
2. **Sichtbarkeitsmengen:** Roaring-Bitmap V je Commit fuer O(1) Zeitreisen ohne Reindexierung (ADR-04)
3. **Branching und Merging:** Fast-Forward, 3-Wege-Merge, Vier-Augen-Freigabe fuer main
4. **Takedown-Prozess:** RedactedK Platzhalter im K-Bestand verankern ohne History-Hashbruch (6.7)
5. **Bereinigung:** ahash, bincode, parking_lot eliminieren

---

## 4. Testkatalog & Freigabekriterien
1. **Zusagen-Konformitaet:** Mindestens ein zugeordneter Test gemaess Zusagenmatrix 16.3 / G-ZUSAGE.
2. **Positivkontrolle (16.4):** Durabilitaets- und Messpruefungen muessen bei defektem Verhalten deterministisch fehlschlagen.
3. **Workspace-Gates:** `cargo xtask gates` muss zu jedem Schritt vollstaendig gruen sein.
4. **Clippy & Lints:** `cargo clippy -p acta-history --all-targets -- -D warnings` ohne Befunde.
5. **Archiv-Bereinigung:** Nach vollstaendiger Ueberfuehrung und Validierung wird der Ordner `archive/acta-history/` geloescht.
