# Implementierungsplan: `acta-rag`

> **Status:** Normativ fuer die Crate-Entwicklung · **Ziel:** Inkrementelle Ueberfuehrung aus `archive/acta-rag/` in sauberen Acta-Produktionscode.  
> **Architekturvorgaben:** [docs/SPEC.md](../../docs/SPEC.md) · [docs/spec/09_ingest_abruf_antwort.md](../../docs/spec/09_ingest_abruf_antwort.md) · [CLAUDE.md](CLAUDE.md)

---

## 1. Einordnung & Spezifikation
- **Level:** L4 (`rings.toml`)
- **Code-Budget:** 7000 LOC Produktionscode (G-HUB, strikt einzuhalten)
- **Spezifikationskapitel:** docs/spec/09_ingest_abruf_antwort.md
- **Zweck & Aufgabe:** Abrufpipeline: Hybridsuche (Text, Vektor, Graph), Reciprocal Rank Fusion (RRF), Reranking, Kontextaufbau, Belegpruefung
- **Archiv-Quelle:** [archive/acta-rag/MANIFEST.md](../../archive/acta-rag/MANIFEST.md)

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

1. **Hybridsuche & RRF:** Parallele Abfrage von Text, Vektor und Graph; 6-Listen-Fusion gemaess SPEC 9.4
   - **Stand:** RRF in Ganzzahl und Auswahl nach Reranker sind umgesetzt (`fusion`). Die Abfrage selbst wartet auf `acta-text`, `acta-vec` und `acta-graph`.
2. Reranker-Anbindung mit Rundung auf Q32.32 an der Portgrenze
3. **Kontextaufbau:** Budgetierung (ctx_len), Belegklammerung mit Zufallstoken gegen Prompt-Injection (T7)
4. **Belegpruefung (Z17):** Lexikalische und semantische Stuetzungspruefung; Enthaltung wenn nicht belegt
5. **ErasureLayer:** Implementierung fuer Antwort- und Sitzungscaches

**Zusätzlich umgesetzt (SPEC 6.3, 9.7):** Gültigkeitszeit (`validity`), Bewertungskennzahlen (`metrics`) und Regressionsprüfung mit `QualityDrift` (`quality`).

---

## 4. Testkatalog & Freigabekriterien
1. **Zusagen-Konformitaet:** Mindestens ein zugeordneter Test gemaess Zusagenmatrix 16.3 / G-ZUSAGE.
2. **Positivkontrolle (16.4):** Durabilitaets- und Messpruefungen muessen bei defektem Verhalten deterministisch fehlschlagen.
3. **Workspace-Gates:** `cargo xtask gates` muss zu jedem Schritt vollstaendig gruen sein.
4. **Clippy & Lints:** `cargo clippy -p acta-rag --all-targets -- -D warnings` ohne Befunde.
5. **Archiv-Bereinigung:** Nach vollstaendiger Ueberfuehrung und Validierung wird der Ordner `archive/acta-rag/` geloescht.
