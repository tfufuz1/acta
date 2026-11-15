# acta-simd — Umsetzungsplan (Fassung 3.0 Final)

*Bezug: Roadmap R0–R7 aus [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) §12*

## Meilensteine

| Stufe | Aufgabe | Ziel / Nachweis | Status |
|---|---|---|---|
| **R0** | Lückenloser `undocumented_unsafe_blocks` Audit | Alle SIMD-Instrinsics mit `// SAFETY:` | Abgeschlossen |
| **R1** | Differenztests gegen skalares Orakel mit Proptest | IEEE-754 Konformitätstests | In Arbeit |

## Definition of Done
- 0 unkommentierte `unsafe`-Stellen.
- 0 interne Pflicht-Abhängigkeiten.
