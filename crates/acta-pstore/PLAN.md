# Implementierungsplan: `acta-pstore`

> **Status:** Normativ fuer die Crate-Entwicklung · **Ziel:** Inkrementelle Ueberfuehrung aus `Kein Archiv (Neubau)` in sauberen Acta-Produktionscode.  
> **Architekturvorgaben:** [docs/SPEC.md](../../docs/SPEC.md) · [docs/spec/05_speicher.md](../../docs/spec/05_speicher.md) · [CLAUDE.md](CLAUDE.md)

---

## 1. Einordnung & Spezifikation
- **Level:** L3 (`rings.toml`)
- **Code-Budget:** 5000 LOC Produktionscode (G-HUB, strikt einzuhalten)
- **Spezifikationskapitel:** docs/spec/05_speicher.md
- **Zweck & Aufgabe:** Personenbezogener Speicher (P-Speicher): Dokumentkopf HK(v), Versionen unter DEK(v), Betroffenenverzeichnis, Handle-Register, Vorgaenge (Case)
- **Archiv-Quelle:** -

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

1. **Dokument- und Versionsverwaltung:** Speicherung von Inhaltsrecords unter HKDF-abgeleitetem DEK(v)
2. **Betroffenenverzeichnis:** Datensatz je ClusterId unter DirKey(s) mit SubjOrd und Alias-Listen
3. **Handle-Register:** PersonalRef -> (DocRef, VerRef, VerOrd, ClusterId) versiegelt unter RegistryKey(t)
4. **Vorgangsverwaltung (Case):** Fallindex (Roaring-Bitmap) als DP-Segment zur Speisung von A_P
5. **Schnittstelle fuer ErasureLayer:** Atomare Bereinigung bei Betroffenen- und Versionsloeschung

---

## 4. Testkatalog & Freigabekriterien
1. **Zusagen-Konformitaet:** Mindestens ein zugeordneter Test gemaess Zusagenmatrix 16.3 / G-ZUSAGE.
2. **Positivkontrolle (16.4):** Durabilitaets- und Messpruefungen muessen bei defektem Verhalten deterministisch fehlschlagen.
3. **Workspace-Gates:** `cargo xtask gates` muss zu jedem Schritt vollstaendig gruen sein.
4. **Clippy & Lints:** `cargo clippy -p acta-pstore --all-targets -- -D warnings` ohne Befunde.
5. **Archiv-Bereinigung:** Nach vollstaendiger Ueberfuehrung und Validierung wird der Ordner `Kein Archiv (Neubau)` geloescht.
