# Implementierungsplan: `acta-base`

> **Status:** Normativ fuer die Crate-Entwicklung · **Ziel:** Inkrementelle Ueberfuehrung aus `archive/acta-base/` in sauberen Acta-Produktionscode.  
> **Architekturvorgaben:** [docs/SPEC.md](../../docs/SPEC.md) · [docs/spec/02_grundmodell.md](../../docs/spec/02_grundmodell.md) · [CLAUDE.md](CLAUDE.md)

---

## 1. Einordnung & Spezifikation
- **Level:** L0 (`rings.toml`)
- **Code-Budget:** 2500 LOC Produktionscode (G-HUB, strikt einzuhalten)
- **Spezifikationskapitel:** docs/spec/02_grundmodell.md, 03_systemarchitektur.md, 04_kryptografie.md, 05_speicher.md
- **Zweck & Aufgabe:** Basis-Newtypes (Ids, Ordinale), DataClass, ErrorClass, ClassifiedError, Ports Clock/CryptoRng, kanonische Kodierung, Festkomma fx
- **Archiv-Quelle:** [archive/acta-base/MANIFEST.md](../../archive/acta-base/MANIFEST.md)

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

1. **Typ- und Ordinal-Migration:** Vollstaendige Migration der Newtypes nach SPEC 2.4 (16-Byte-Handles, streng monotone Ordinale, SubjectTag, ClusterId, IndexGen, ObjectId)
2. **Festkomma-Arithmetik:** fx Q32.32 (Addition, Subtraktion mit Saettigung, Multiplikation, Division, fx_ln, isqrt) gemaess Z7 mit Goldvektoren verifizieren
3. **Ports Clock und CryptoRng:** Strikte Abstraktion; CryptoRng fallibel via try_fill() (G3: keine direkten Systemzeit-/Zufallsaufrufe)
4. **Kanonische Kodierung:** canon::Reader / canon::Writer fuer Big-Endian-Serialisierung ohne serde auf Plattenformaten (G-NOSERDE-DISK)
5. **Orakel-Tests:** Uebernahme der invarianten Testsuiten aus archive/acta-base/ids/tests/

---

## 4. Testkatalog & Freigabekriterien
1. **Zusagen-Konformitaet:** Mindestens ein zugeordneter Test gemaess Zusagenmatrix 16.3 / G-ZUSAGE.
2. **Positivkontrolle (16.4):** Durabilitaets- und Messpruefungen muessen bei defektem Verhalten deterministisch fehlschlagen.
3. **Workspace-Gates:** `cargo xtask gates` muss zu jedem Schritt vollstaendig gruen sein.
4. **Clippy & Lints:** `cargo clippy -p acta-base --all-targets -- -D warnings` ohne Befunde.
5. **Archiv-Bereinigung:** Nach vollstaendiger Ueberfuehrung und Validierung wird der Ordner `archive/acta-base/` geloescht.
