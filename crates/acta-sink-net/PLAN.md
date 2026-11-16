# Implementierungsplan: `acta-sink-net`

> **Status:** Normativ fuer die Crate-Entwicklung · **Ziel:** Inkrementelle Ueberfuehrung aus `Kein Archiv (Neubau)` in sauberen Acta-Produktionscode.  
> **Architekturvorgaben:** [docs/SPEC.md](../../docs/SPEC.md) · [docs/spec/06_wissensebene.md](../../docs/spec/06_wissensebene.md) · [CLAUDE.md](CLAUDE.md)

---

## 1. Einordnung & Spezifikation
- **Level:** L8 (`rings.toml`)
- **Code-Budget:** 600 LOC Produktionscode (G-HUB, strikt einzuhalten)
- **Spezifikationskapitel:** docs/spec/06_wissensebene.md
- **Zweck & Aufgabe:** Netzwerk-Senke: Asynchrone Uebertragung von Checkpoints und Commitments ueber mTLS (RegistrySink-Port)
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

1. RegistrySink-Port asynchron ueber mTLS implementieren
2. Sichere Uebertragung mit Wiederholungslogik und Backoff
3. Strikte Egress-Pruefung gemaess Z18
4. Nahtstelle zu acta-ledger

---

## 4. Testkatalog & Freigabekriterien
1. **Zusagen-Konformitaet:** Mindestens ein zugeordneter Test gemaess Zusagenmatrix 16.3 / G-ZUSAGE.
2. **Positivkontrolle (16.4):** Durabilitaets- und Messpruefungen muessen bei defektem Verhalten deterministisch fehlschlagen.
3. **Workspace-Gates:** `cargo xtask gates` muss zu jedem Schritt vollstaendig gruen sein.
4. **Clippy & Lints:** `cargo clippy -p acta-sink-net --all-targets -- -D warnings` ohne Befunde.
5. **Archiv-Bereinigung:** Nach vollstaendiger Ueberfuehrung und Validierung wird der Ordner `Kein Archiv (Neubau)` geloescht.
