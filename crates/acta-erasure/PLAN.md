# Implementierungsplan: `acta-erasure`

> **Status:** Normativ fuer die Crate-Entwicklung · **Ziel:** Inkrementelle Ueberfuehrung aus `archive/acta-erasure/` in sauberen Acta-Produktionscode.  
> **Architekturvorgaben:** [docs/SPEC.md](../../docs/SPEC.md) · [docs/spec/08_loeschkern.md](../../docs/spec/08_loeschkern.md) · [CLAUDE.md](CLAUDE.md)

---

## 1. Einordnung & Spezifikation
- **Level:** L4 (`rings.toml`)
- **Code-Budget:** 9000 LOC Produktionscode (G-HUB, strikt einzuhalten)
- **Spezifikationskapitel:** docs/spec/08_loeschkern.md
- **Zweck & Aufgabe:** Loeschkern: 7-Stufen-Loeschung (S1-S7), Saga-Orchestrierung, Gate-Integration, Nachweismessung, Restore-Schutz
- **Archiv-Quelle:** [archive/acta-erasure/MANIFEST.md](../../archive/acta-erasure/MANIFEST.md)

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

1. Saga-Zustandsmaschine fuer S1 (Sperre), S2 (Outbox), S3 (Bereinigung), S4 (Messung), S5 (Vernichtung), S6 (Ledger), S7 (Abschluss)
2. **Mehrbetroffenen-Planung:** Erkennung und Loesung von Konflikten mit Zwischenzustand Restricted(MultiSubjectReview)
3. **Messung und Nachweis (Z11):** Scanner mit Positivkontroll-Tests vor Schluesselvernichtung
4. **Restore-Schutz:** Verhinderung des Wiederauflebens geloeschter Daten bei Backupeinspielung (8.8)
5. **Bereinigung:** bincode, futures-util, parking_lot entfernen

---

## 4. Testkatalog & Freigabekriterien
1. **Zusagen-Konformitaet:** Mindestens ein zugeordneter Test gemaess Zusagenmatrix 16.3 / G-ZUSAGE.
2. **Positivkontrolle (16.4):** Durabilitaets- und Messpruefungen muessen bei defektem Verhalten deterministisch fehlschlagen.
3. **Workspace-Gates:** `cargo xtask gates` muss zu jedem Schritt vollstaendig gruen sein.
4. **Clippy & Lints:** `cargo clippy -p acta-erasure --all-targets -- -D warnings` ohne Befunde.
5. **Archiv-Bereinigung:** Nach vollstaendiger Ueberfuehrung und Validierung wird der Ordner `archive/acta-erasure/` geloescht.
