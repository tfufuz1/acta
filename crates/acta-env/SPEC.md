# acta-env — Crate-Spezifikation (Fassung 3.0 Final)

*Bezug: Normative Gesamtspezifikation [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) (Teil III, IV, VIII)*  
*Ring: R0 (Fundament) · Geltung: Verbindlich für `acta-env`*

---

## 1. Zweck und Einordnung

`acta-env` stellt deterministische und kontrollierbare Umwelt-Abstraktionen für die gesamte Architektur bereit:
- **Zeit-Port:** `Clock` Trait mit `now_unix_nanos()` und `monotonic_nanos()`.
- **Zufalls-Port:** `Rng` Trait für kryptografische Zufallszahlen (Einzige Quelle für AEAD-Nonces, Invariante I-15).
- **ID-Generator:** `IdGen` Trait für transaktionale und temporäre Sequenzen.
- **Prozesshärtung:** Schnittstellen für Speicherschutz (`mlock`, `MADV_DONTDUMP`, Deaktivierung von Core-Dumps für Annahme A-RAM).

---

## 2. Architektur und Verträge

| Merkmal | Vorgabe (docs/SPEC.md §3.1) |
|---|---|
| **Ring** | **R0 (Fundament)** |
| **Interne Abhängigkeiten** | **0 (keine)** |
| **Default-Features** | `["std"]` |
| **Optionale Features** | `["test-utils"]` (`ManualClock`, `DeterministicRng`) |
| **Unsafe-Policy** | `#![forbid(unsafe_code)]` |
| **Gate G3** | Verhindert unkontrollierte Systemzeit/Zufall in Produktionscrates |

---

## 3. Normative Anforderungen

1. **Gate G3 Durchsetzung:**  
   Kein Produktions-Crate in Ring R1, R2, R3, R4 oder R5 darf direkt `std::time::SystemTime::now()` oder `rand::thread_rng()` aufrufen. Alle Zeit- und Zufallsoperationen müssen über die Traits aus `acta-env` injiziert werden.
2. **Kryptografischer Zufall:**  
   `CryptoRng` schlägt bei Betriebssystemfehlern hart fehl (Fail-Closed). Es gibt keinen Default-Zufallswert.
3. **Prozesshärtung (A-RAM):**  
   Bereitstellung von Routinen zur Nullung sensibler Puffer und Unterdrückung von Swap/Dumps.

---

## 4. Tests und Abnahme

- Determinismus-Tests mit `ManualClock` und `DeterministicRng`.
- Gate `G3`: Clippy-Prüfung `disallowed_methods` auf Nicht-Verwendung von Standardzeit/-zufall.
