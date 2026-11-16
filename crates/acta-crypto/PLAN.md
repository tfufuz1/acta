# Implementierungsplan: `acta-crypto`

> **Status:** Normativ fuer die Crate-Entwicklung · **Ziel:** Inkrementelle Ueberfuehrung aus `archive/acta-crypto/` in sauberen Acta-Produktionscode.  
> **Architekturvorgaben:** [docs/SPEC.md](../../docs/SPEC.md) · [docs/spec/04_kryptografie.md](../../docs/spec/04_kryptografie.md) · [CLAUDE.md](CLAUDE.md)

---

## 1. Einordnung & Spezifikation
- **Level:** L1 (`rings.toml`)
- **Code-Budget:** 4200 LOC Produktionscode (G-HUB, strikt einzuhalten)
- **Spezifikationskapitel:** docs/spec/04_kryptografie.md
- **Zweck & Aufgabe:** Kryptografische Primitive: AEAD (XChaCha20-Poly1305), HKDF, HMAC, Ed25519, Sealed<P>, Zeroizing-Schluesseltypen, HashChain, Padding
- **Archiv-Quelle:** [archive/acta-crypto/MANIFEST.md](../../archive/acta-crypto/MANIFEST.md)

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

1. Umschlag v1 gemaess SPEC 4.1 implementieren (XChaCha20-Poly1305, Klassen Personal/Knowledge/Derived, AAD-Bindung)
2. **Schluesselableitungshierarchie:** HKDF-SHA256 fuer DEK(v), HK(v), DirKey, ClusterId, VK(t,g), K_tag(t) NFC-normalisiert
3. **Zeroizing & G-CRY:** Alle Schluesseltypen kapseln rohes Material hermetisch ab (kein Clone/Debug/Serialize/Default auf Secrets)
4. **Ed25519-Signaturpruefung:** Strikte Pruefung via verify_strict mit Domaenenpraefixen (acta/report/v1, acta/checkpoint/v1)
5. RFC- und PyNaCl-Testvektoren aus archive/acta-crypto/ vollstaendig als Regressionstests integrieren

---

## 4. Testkatalog & Freigabekriterien
1. **Zusagen-Konformitaet:** Mindestens ein zugeordneter Test gemaess Zusagenmatrix 16.3 / G-ZUSAGE.
2. **Positivkontrolle (16.4):** Durabilitaets- und Messpruefungen muessen bei defektem Verhalten deterministisch fehlschlagen.
3. **Workspace-Gates:** `cargo xtask gates` muss zu jedem Schritt vollstaendig gruen sein.
4. **Clippy & Lints:** `cargo clippy -p acta-crypto --all-targets -- -D warnings` ohne Befunde.
5. **Archiv-Bereinigung:** Nach vollstaendiger Ueberfuehrung und Validierung wird der Ordner `archive/acta-crypto/` geloescht.
