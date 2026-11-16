# Implementierungsplan: `acta-engine`

> **Status:** Normativ fuer die Crate-Entwicklung · **Ziel:** Inkrementelle Ueberfuehrung aus `archive/acta-engine/` in sauberen Acta-Produktionscode.  
> **Architekturvorgaben:** [docs/SPEC.md](../../docs/SPEC.md) · [docs/spec/10_fachfunktionen.md](../../docs/spec/10_fachfunktionen.md) · [CLAUDE.md](CLAUDE.md)

---

## 1. Einordnung & Spezifikation
- **Level:** L6 (`rings.toml`)
- **Code-Budget:** 12000 LOC Produktionscode (G-HUB, strikt einzuhalten)
- **Spezifikationskapitel:** docs/spec/10_fachfunktionen.md, 19_lebenszyklen.md
- **Zweck & Aufgabe:** Zentrale Geschaeftslogik: Orchestrierung der Lebenszyklen (Kap. 19), Fachfunktionen FF1-FF9, Mandantenverarbeitung
- **Archiv-Quelle:** [archive/acta-engine/MANIFEST.md](../../archive/acta-engine/MANIFEST.md)

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

1. Lebenszyklen gemaess SPEC 19 vollstaendig implementieren (Ingest, Query, Erase, Szenario, Maintenance)
2. Fachfunktionen FF1 bis FF9 gemaess SPEC 10 einbinden
3. **Thread-Pools verwalten:** writer(t), read, rewrite, maint gemaess SPEC 3.6
4. **Bereinigung:** scc, parking_lot, blake3, tokio-util entfernen; synchrone Ausfuehrung garantieren

---

## 4. Testkatalog & Freigabekriterien
1. **Zusagen-Konformitaet:** Mindestens ein zugeordneter Test gemaess Zusagenmatrix 16.3 / G-ZUSAGE.
2. **Positivkontrolle (16.4):** Durabilitaets- und Messpruefungen muessen bei defektem Verhalten deterministisch fehlschlagen.
3. **Workspace-Gates:** `cargo xtask gates` muss zu jedem Schritt vollstaendig gruen sein.
4. **Clippy & Lints:** `cargo clippy -p acta-engine --all-targets -- -D warnings` ohne Befunde.
5. **Archiv-Bereinigung:** Nach vollstaendiger Ueberfuehrung und Validierung wird der Ordner `archive/acta-engine/` geloescht.
