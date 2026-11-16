# Implementierungsplan: `acta-fs`

> **Status:** Normativ fuer die Crate-Entwicklung · **Ziel:** Inkrementelle Ueberfuehrung aus `archive/acta-fs/` in sauberen Acta-Produktionscode.  
> **Architekturvorgaben:** [docs/SPEC.md](../../docs/SPEC.md) · [docs/spec/03_systemarchitektur.md](../../docs/spec/03_systemarchitektur.md) · [CLAUDE.md](CLAUDE.md)

---

## 1. Einordnung & Spezifikation
- **Level:** L1 (`rings.toml`)
- **Code-Budget:** 2500 LOC Produktionscode (G-HUB, strikt einzuhalten)
- **Spezifikationskapitel:** docs/spec/03_systemarchitektur.md, 05_speicher.md, 16_test_freigabe.md
- **Zweck & Aufgabe:** Dateisystemabstraktion: Vfs-Port, DurableVfs, FaultVfs, atomare Operationen (rename_atomic), Scrubbing (remove_scrub)
- **Archiv-Quelle:** [archive/acta-fs/MANIFEST.md](../../archive/acta-fs/MANIFEST.md)

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

1. **Vfs-Portvertrag:** Synchroner I/O-Kern gemaess SPEC 3.7 und 5.6 (WP-1 bis WP-7); kein async/tokio
2. **DurableVfs:** Implementierung von rename_atomic mit Verzeichnis-fsync und remove_scrub (Nullen ueberschreiben + fsync)
3. **WP-7:** FsyncFailed-Zustand verharrt irreversibel und verbietet Retries auf derselben Datei
4. **FaultVfs:** Fehlersimulation fuer Crash-Matrix C01-C27 aus archive/acta-fs/testkit/src/fault_vfs.rs ueberfuehren
5. **Bereinigung:** std::fs nur innerhalb von acta-fs kapseln (G-FS), parking_lot und crc32fast durch std/crc32c ersetzen

---

## 4. Testkatalog & Freigabekriterien
1. **Zusagen-Konformitaet:** Mindestens ein zugeordneter Test gemaess Zusagenmatrix 16.3 / G-ZUSAGE.
2. **Positivkontrolle (16.4):** Durabilitaets- und Messpruefungen muessen bei defektem Verhalten deterministisch fehlschlagen.
3. **Workspace-Gates:** `cargo xtask gates` muss zu jedem Schritt vollstaendig gruen sein.
4. **Clippy & Lints:** `cargo clippy -p acta-fs --all-targets -- -D warnings` ohne Befunde.
5. **Archiv-Bereinigung:** Nach vollstaendiger Ueberfuehrung und Validierung wird der Ordner `archive/acta-fs/` geloescht.
