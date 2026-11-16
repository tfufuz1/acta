# Implementierungsplan: `acta-gate`

> **Status:** Normativ fuer die Crate-Entwicklung · **Ziel:** Inkrementelle Ueberfuehrung aus `archive/acta-gate/` in sauberen Acta-Produktionscode.  
> **Architekturvorgaben:** [docs/SPEC.md](../../docs/SPEC.md) · [docs/spec/03_systemarchitektur.md](../../docs/spec/03_systemarchitektur.md) · [CLAUDE.md](CLAUDE.md)

---

## 1. Einordnung & Spezifikation
- **Level:** L1 (`rings.toml`)
- **Code-Budget:** 1200 LOC Produktionscode (G-HUB, strikt einzuhalten)
- **Spezifikationskapitel:** docs/spec/03_systemarchitektur.md, 05_speicher.md
- **Zweck & Aufgabe:** Zentrales Gate: ErasedView, GateWriter, unveraenderliche Roaring-Sperrmengen (ErasedSubjects, ErasedVers, RestrictedVers, RedactedK), ErasureLayer-Vertrag
- **Archiv-Quelle:** [archive/acta-gate/MANIFEST.md](../../archive/acta-gate/MANIFEST.md)

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

1. **Gate-Datenstrukturen:** Schnappschuesse ueber Arc mit atomarem Tausch (3.6) fuer O(1)-Pruefungen ohne I/O
2. **GateWriter:** Fuehrung des append-only Gate-Logs mit HMAC-Kette und Gruppen-fsync (5.3)
3. **Sperrvorrang (Z4):** ErasedVers > RestrictedVers > SupersededVers; RedactedK fuer Takedowns
4. **ErasureLayer-Trait (HR-3):** Universeller Vertrag fuer Loeschkerndurchgriff auf alle Speicher und Caches
5. **Bereinigung:** parking_lot eliminieren, synchrone Sperren gemaess Lock-Order verwenden

---

## 4. Testkatalog & Freigabekriterien
1. **Zusagen-Konformitaet:** Mindestens ein zugeordneter Test gemaess Zusagenmatrix 16.3 / G-ZUSAGE.
2. **Positivkontrolle (16.4):** Durabilitaets- und Messpruefungen muessen bei defektem Verhalten deterministisch fehlschlagen.
3. **Workspace-Gates:** `cargo xtask gates` muss zu jedem Schritt vollstaendig gruen sein.
4. **Clippy & Lints:** `cargo clippy -p acta-gate --all-targets -- -D warnings` ohne Befunde.
5. **Archiv-Bereinigung:** Nach vollstaendiger Ueberfuehrung und Validierung wird der Ordner `archive/acta-gate/` geloescht.
