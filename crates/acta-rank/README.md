# acta-rank

Re-Ranking, Reciprocal Rank Fusion (RRF) und Score-Kalibrierung für Acta.

## Rolle und Zweck
`acta-rank` ist das Ranking-Crate in Ring R1:
- **Hybrid-Fusion:** Vereinigung von Volltext- und Vektorergebnissen.
- **Gate-Filterung:** Ausschluss gelöschter Einheiten vor der Auslieferung an die Engine.
- **Deterministische Tie-Breaks:** Reproduzierbare Sortierung bei identischen Scores.
