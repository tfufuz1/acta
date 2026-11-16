# Implementierungsplan: `acta-infer`

> **Status:** Normativ fuer die Crate-Entwicklung · **Ziel:** Inkrementelle Ueberfuehrung aus `archive/acta-infer/` in sauberen Acta-Produktionscode.  
> **Architekturvorgaben:** [docs/SPEC.md](../../docs/SPEC.md) · [docs/spec/11_inferenz.md](../../docs/spec/11_inferenz.md) · [CLAUDE.md](CLAUDE.md)

---

## 1. Einordnung & Spezifikation
- **Level:** L1 (`rings.toml`)
- **Code-Budget:** 700 LOC Produktionscode (G-HUB, strikt einzuhalten)
- **Spezifikationskapitel:** docs/spec/11_inferenz.md
- **Zweck & Aufgabe:** Inferenz-Vertraege: Embedder, Reranker, Generator, Verifier, ModelFingerprint, ContextPurge
- **Archiv-Quelle:** [archive/acta-infer/MANIFEST.md](../../archive/acta-infer/MANIFEST.md)

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

1. Port-Definitionen gemaess SPEC 11.1 synchron definieren (Embedder, Reranker, Generator, Verifier)
2. **ModelFingerprint:** Eindeutige Kennung fuer Modellfamilie, Versions-Hash, Quantisierung und Prompt-Template
3. **ContextPurge-Port (Z1):** purge(request_id)-Schnittstelle zur Bereinigung ephemerer Modellzustaende
4. Token-Counting und Budgetierungs-Vertraege nach SPEC 11.5

---

## 4. Testkatalog & Freigabekriterien
1. **Zusagen-Konformitaet:** Mindestens ein zugeordneter Test gemaess Zusagenmatrix 16.3 / G-ZUSAGE.
2. **Positivkontrolle (16.4):** Durabilitaets- und Messpruefungen muessen bei defektem Verhalten deterministisch fehlschlagen.
3. **Workspace-Gates:** `cargo xtask gates` muss zu jedem Schritt vollstaendig gruen sein.
4. **Clippy & Lints:** `cargo clippy -p acta-infer --all-targets -- -D warnings` ohne Befunde.
5. **Archiv-Bereinigung:** Nach vollstaendiger Ueberfuehrung und Validierung wird der Ordner `archive/acta-infer/` geloescht.
