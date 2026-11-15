# acta-sys — Crate-Spezifikation (Fassung 3.0 Final)

*Bezug: Normative Gesamtspezifikation [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) (Teil III, IV, X)*  
*Ring: R0 (Fundament, unsafe-Insel) · Geltung: Verbindlich für `acta-sys`*

---

## 1. Zweck und Einordnung

`acta-sys` ist die isolierte, systemnahe Plattform-Abstraktionsschicht in **Ring R0 (Fundament)**:
- **Speichersperrung (`mlock`):** Physisches Festhalten sensibler Krypto-Schlüssel im RAM, `madvise(MADV_DONTDUMP)`, Core-Dump-Verhinderung (`RLIMIT_CORE = 0`) zur Stützung von Annahme **A-RAM**.
- **Speicherabbildung (`mmap`):** Ausschließliche Nur-Lese-Mmaps (`mmap_readonly`) zur Eliminierung von Data Races.
- **Dateisystem-Sicherheit:** Restriktive Dateirechte (`0600`) und Windows-DACLs.
- **Plattform-Synchronisation:** Robuste `fsync`- und `fdatasync`-Routinen für Dateien und Verzeichnisse (`sync_dir`).

---

## 2. Architektur und Verträge

| Merkmal | Vorgabe (docs/SPEC.md §3.1) |
|---|---|
| **Ring** | **R0 (Fundament)** |
| **Interne Abhängigkeiten** | **0 (keine)** |
| **Unsafe-Policy** | Zulässig nur mit formalem `// SAFETY:`-Nachweis je Block (Lint `undocumented_unsafe_blocks`) |
| **Poisoning** | Permanente Vergiftung des Handles bei `fsync`-Fehlern (INV-SYS-FSYNC-1) |

---

## 3. Normative Anforderungen

1. **Unsafe-Isolation:**  
   `acta-sys` ist neben `acta-simd` die einzige Crate mit erlaubtem `unsafe`-Code. Jeder Block MUSS die Erfüllung aller Vorbedingungen dokumentieren.
2. **Speicherhygiene vor `munlock`:**  
   Gesperrter Speicher MUSS vor dem Entsperren mit Nullen überschrieben werden (`volatile_zeroize` + Memory-Fence).
3. **Fail-Closed Dateisystem-Synchronisation:**  
   Fehlgeschlagene Synchronisationen markieren das Handle unwiderruflich als vergiftet. Stille Fehlerbehandlung ist untersagt.
