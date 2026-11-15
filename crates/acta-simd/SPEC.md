# acta-simd — Crate-Spezifikation (Fassung 3.0 Final)

*Bezug: Normative Gesamtspezifikation [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) (Teil III, IV, VII)*  
*Ring: R0 (Fundament, unsafe-Insel) · Geltung: Verbindlich für `acta-simd`*

---

## 1. Zweck und Einordnung

`acta-simd` stellt hardwarenahe Vektorberechnungen und Distanzkerne für den Vektorindex (`acta-vector`) bereit:
- **Distanzmaße:** Cosine Distance, Dot Product, L2 Squared.
- **Hardware-Ziele:** AVX2, AVX-512, ARM NEON mit deterministischem skalarem Referenzpfad.
- **Validierung:** Fail-Closed Zurückweisung nicht-finiter Werte (`NaN`, `±Inf`).

---

## 2. Architektur und Verträge

| Merkmal | Vorgabe (docs/SPEC.md §3.1) |
|---|---|
| **Ring** | **R0 (Fundament, unsafe-Insel)** |
| **Interne Pflicht-Abhängigkeiten** | **0 (keine)** |
| **Default-Features** | `["std"]` |
| **Fehlertyp** | `SimdError` (Fail-Closed) |
| **Unsafe-Policy** | Zulässig ausschließlich mit lückenlosem `// SAFETY:`-Nachweis |

---

## 3. Normative Anforderungen

1. **Determinismus (Invariante INV-SIMD-DETERMINISM-1):**  
   Float-Akkumulationen folgen einer unveränderlichen Summationsreihenfolge. Der Skalarpfad dient als formales Orakel für Differenztests.
2. **Endlichkeit (Invariante INV-SIMD-FINITE-1):**  
   Vektoren mit `NaN` oder `±Inf` werden strikt mit `SimdError::InvalidInput` abgelehnt.
3. **Keine PII-Kenntnis:**  
   `acta-simd` operiert rein auf rohen Float-Arrays; es besitzt keinerlei Kenntnis von Metadaten oder IDs.
