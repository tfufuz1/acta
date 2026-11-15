# acta-simd

SIMD-beschleunigte Distanzkerne (AVX2, AVX-512, ARM NEON) mit skalarem Orakel für den Vektorindex in Acta.

## Rolle und Zweck
`acta-simd` ist eine gekapselte `unsafe`-Insel in Ring R0:
- **Distanzfunktionen:** Cosine, L2-Squared, Dot-Product.
- **Deterministische Reduktion:** Feste Summationsbäume zur Vermeidung numerischer Nicht-Determinismen.
- **Fail-Closed:** Strikte Zurückweisung von `NaN`/`Inf`.
