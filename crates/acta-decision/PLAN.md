# Implementierungsplan: `acta-decision`

> **Status:** Normativ fuer die Crate-Entwicklung · **Ziel:** Inkrementelle Ueberfuehrung aus `archive/acta-decision/` in sauberen Acta-Produktionscode.  
> **Architekturvorgaben:** [docs/SPEC.md](../../docs/SPEC.md) · [docs/spec/07_entscheidungsprotokoll.md](../../docs/spec/07_entscheidungsprotokoll.md) · [CLAUDE.md](CLAUDE.md)

---

## 1. Einordnung & Spezifikation
- **Level:** L4 (`rings.toml`)
- **Code-Budget:** 4500 LOC Produktionscode (G-HUB, strikt einzuhalten)
- **Spezifikationskapitel:** docs/spec/07_entscheidungsprotokoll.md
- **Zweck & Aufgabe:** Entscheidungsprotokoll: Unveraenderliches Protokollieren, Szenarienanalyse, Akteursmodelle, Begruendungsketten
- **Archiv-Quelle:** [archive/acta-decision/MANIFEST.md](../../archive/acta-decision/MANIFEST.md)

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

1. **Entscheidungseintrag:** Begruendungskette (Wissensstand, Prompt, Modell, Belege, Eingabe)
2. **Entkopplung von Personendaten (Z8):** Nutzlast liegt unter getrenntem DEK; Begruendung uebersteht P-Loeschung
3. **Szenarien:** Isoliertes Verzweigen und Simulieren auf Wissensstaenden ohne Produktivwirkung (Z9)
4. **Akteure und ActorRef:** Mitarbeiter-Handles ueber Epochen hinweg konsistent halten
5. **Bereinigung:** ahash entfernen, direkte Anbindung an acta-ledger

---

## 4. Testkatalog & Freigabekriterien
1. **Zusagen-Konformitaet:** Mindestens ein zugeordneter Test gemaess Zusagenmatrix 16.3 / G-ZUSAGE.
2. **Positivkontrolle (16.4):** Durabilitaets- und Messpruefungen muessen bei defektem Verhalten deterministisch fehlschlagen.
3. **Workspace-Gates:** `cargo xtask gates` muss zu jedem Schritt vollstaendig gruen sein.
4. **Clippy & Lints:** `cargo clippy -p acta-decision --all-targets -- -D warnings` ohne Befunde.
5. **Archiv-Bereinigung:** Nach vollstaendiger Ueberfuehrung und Validierung wird der Ordner `archive/acta-decision/` geloescht.
