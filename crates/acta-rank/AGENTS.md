# AGENTS.md — `acta-rank`

> Arbeitskontext für Agenten. Gültig gemäß [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) (Fassung 3.0 Final).

## Strikte Regeln für Agenten
1. **Gate vor Ranking:** Niemals ungeprüfte Kandidaten ranken.
2. **Determinismus:** Keine unkontrollierten Hash-Maps bei Tie-Breaks.
3. **Ring R1:** Abhängigkeiten nur nach unten.
