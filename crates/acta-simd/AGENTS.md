# AGENTS.md — `acta-simd`

> Arbeitskontext für Agenten. Gültig gemäß [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) (Fassung 3.0 Final).

## Strikte Regeln für Agenten
1. **Unsafe-Nachweis:** Jeder Intrinsics-Aufruf verlangt `// SAFETY:` mit Ausrichtungs- und Längenprüfungen.
2. **Determinismus:** Keine plattformabhängigen Rundungen ohne Skalar-Angleichung.
3. **Ring R0:** Keine Abhängigkeiten zu höheren Ringen.
