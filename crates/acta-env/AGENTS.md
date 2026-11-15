# AGENTS.md — `acta-env`

> Arbeitskontext für Agenten. Gültig gemäß [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) (Fassung 3.0 Final).

## Regeln für Agenten
1. **Ring R0:** 0 interne Abhängigkeiten im Workspace.
2. **Determinismus:** Alle externen Zeit- und Zufallseinflüsse müssen über Traits (`Clock`, `Rng`) abstrahiert werden.
3. **Fail-Closed:** Keine Fallbacks bei Systementropie-Mangel.
