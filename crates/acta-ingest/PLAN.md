# Implementierungsplan: `acta-ingest`

> **Status:** Normativ fuer die Crate-Entwicklung · **Ziel:** Inkrementelle Ueberfuehrung aus `archive/acta-ingest/` in sauberen Acta-Produktionscode.  
> **Architekturvorgaben:** [docs/SPEC.md](../../docs/SPEC.md) · [docs/spec/09_ingest_abruf_antwort.md](../../docs/spec/09_ingest_abruf_antwort.md) · [CLAUDE.md](CLAUDE.md)

---

## 1. Einordnung & Spezifikation
- **Level:** L1 (`rings.toml`)
- **Code-Budget:** 3000 LOC Produktionscode (G-HUB, strikt einzuhalten)
- **Spezifikationskapitel:** docs/spec/09_ingest_abruf_antwort.md
- **Zweck & Aufgabe:** Dokumentaufbereitung vor Speicherung: Deterministisches Chunking (chunk_cfg v1), UAX #29 Normalisierung, Worker-Protokollrahmen
- **Archiv-Quelle:** [archive/acta-ingest/MANIFEST.md](../../archive/acta-ingest/MANIFEST.md)

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

1. **Reine Chunking-Funktion:** (Bytes, chunk_cfg) -> Vec<Chunk> deterministisch implementieren (SPEC 9.1.1)
2. **Heading-Hierarchie & Normzitate:** Erkennung und Pflege von heading_path und norm_ref Feldern
3. **Prozessgrenze zum Parser-Worker:** Big-Endian Rahmenprotokoll fuer Sandbox-Worker (max 64 MiB)
4. **Bereinigung:** blake3 entfernen, SHA-256 via acta-crypto nutzen

---

## 4. Testkatalog & Freigabekriterien
1. **Zusagen-Konformitaet:** Mindestens ein zugeordneter Test gemaess Zusagenmatrix 16.3 / G-ZUSAGE.
2. **Positivkontrolle (16.4):** Durabilitaets- und Messpruefungen muessen bei defektem Verhalten deterministisch fehlschlagen.
3. **Workspace-Gates:** `cargo xtask gates` muss zu jedem Schritt vollstaendig gruen sein.
4. **Clippy & Lints:** `cargo clippy -p acta-ingest --all-targets -- -D warnings` ohne Befunde.
5. **Archiv-Bereinigung:** Nach vollstaendiger Ueberfuehrung und Validierung wird der Ordner `archive/acta-ingest/` geloescht.
