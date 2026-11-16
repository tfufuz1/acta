# Implementierungsplan: `acta-vault`

> **Status:** Normativ fuer die Crate-Entwicklung · **Ziel:** Inkrementelle Ueberfuehrung aus `archive/acta-vault/` in sauberen Acta-Produktionscode.  
> **Architekturvorgaben:** [docs/SPEC.md](../../docs/SPEC.md) · [docs/spec/04_kryptografie.md](../../docs/spec/04_kryptografie.md) · [CLAUDE.md](CLAUDE.md)

---

## 1. Einordnung & Spezifikation
- **Level:** L2 (`rings.toml`)
- **Code-Budget:** 5500 LOC Produktionscode (G-HUB, strikt einzuhalten)
- **Spezifikationskapitel:** docs/spec/04_kryptografie.md
- **Zweck & Aufgabe:** Mandantentresore: KeyAnchor, FileAnchor mit Argon2id, Wurzelrotation mit Generationen g, Sicherungsexport, Vernichtung
- **Archiv-Quelle:** [archive/acta-vault/MANIFEST.md](../../archive/acta-vault/MANIFEST.md)

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

1. **Tresorformat:** AAD-gebundene Speicherung (slot, Generation g), Index rein im RAM (ADR-26)
2. **Wurzelrotation:** Generationen-Modell VK(t, g) mit Freeze-Phase (3.6) und Bereinigung alter Schluessel (ADR-07)
3. **Anker:** FileAnchor mit Argon2id nach SPEC 4.1; KeyAnchor-Abstraktion fuer HSM/KMS
4. **R-KEY-FIRST:** Schluessel durabel vor Datenverwendung schreiben; physische Vernichtung bei Loeschung
5. **Bereinigung:** ahash, bincode, blake3, parking_lot eliminieren

---

## 4. Testkatalog & Freigabekriterien
1. **Zusagen-Konformitaet:** Mindestens ein zugeordneter Test gemaess Zusagenmatrix 16.3 / G-ZUSAGE.
2. **Positivkontrolle (16.4):** Durabilitaets- und Messpruefungen muessen bei defektem Verhalten deterministisch fehlschlagen.
3. **Workspace-Gates:** `cargo xtask gates` muss zu jedem Schritt vollstaendig gruen sein.
4. **Clippy & Lints:** `cargo clippy -p acta-vault --all-targets -- -D warnings` ohne Befunde.
5. **Archiv-Bereinigung:** Nach vollstaendiger Ueberfuehrung und Validierung wird der Ordner `archive/acta-vault/` geloescht.
