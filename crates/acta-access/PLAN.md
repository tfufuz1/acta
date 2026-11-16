# Implementierungsplan: `acta-access`

> **Status:** Normativ fuer die Crate-Entwicklung · **Ziel:** Inkrementelle Ueberfuehrung aus `archive/acta-access/` in sauberen Acta-Produktionscode.  
> **Architekturvorgaben:** [docs/SPEC.md](../../docs/SPEC.md) · [docs/spec/09_ingest_abruf_antwort.md](../../docs/spec/09_ingest_abruf_antwort.md) · [CLAUDE.md](CLAUDE.md)

---

## 1. Einordnung & Spezifikation
- **Level:** L1 (`rings.toml`)
- **Code-Budget:** 3000 LOC Produktionscode (G-HUB, strikt einzuhalten)
- **Spezifikationskapitel:** docs/spec/09_ingest_abruf_antwort.md, 12_mandanten_rollen_aufbewahrung.md
- **Zweck & Aufgabe:** Mandanten, Rollen, ABAC-Attribute, Richtlinienauswertung, Sichtklassen, Zugriffsmengen A_K und A_P (Z16)
- **Archiv-Quelle:** [archive/acta-access/MANIFEST.md](../../archive/acta-access/MANIFEST.md)

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

1. ABAC-Modell gemaess SPEC 9.4/12.1 umsetzen (allow/deny-Regeln ueber Subjekt- und Ressourcenattribute)
2. **Zugriffszellen und Roaring-Bitmaps:** Verwaltung der Zell-Bitmaps fuer KOrd; Berechnung der effektiven Sichtklasse view_class und Zugriffsmenge A_K
3. **Sichtklassen-Isolation (Z16):** Garantieren, dass unzugaengliche Dokumente weder Treffer noch Ranking noch Fehlermeldungen beeinflussen
4. Filter-Logik aus archive/acta-access/types/src/types/filter.rs refaktorisieren und serde-frei einbinden
5. Orakel-Tests aus types/tests/tenant_scope_isolation.rs als Z16-Garanten uebernehmen

---

## 4. Testkatalog & Freigabekriterien
1. **Zusagen-Konformitaet:** Mindestens ein zugeordneter Test gemaess Zusagenmatrix 16.3 / G-ZUSAGE.
2. **Positivkontrolle (16.4):** Durabilitaets- und Messpruefungen muessen bei defektem Verhalten deterministisch fehlschlagen.
3. **Workspace-Gates:** `cargo xtask gates` muss zu jedem Schritt vollstaendig gruen sein.
4. **Clippy & Lints:** `cargo clippy -p acta-access --all-targets -- -D warnings` ohne Befunde.
5. **Archiv-Bereinigung:** Nach vollstaendiger Ueberfuehrung und Validierung wird der Ordner `archive/acta-access/` geloescht.
