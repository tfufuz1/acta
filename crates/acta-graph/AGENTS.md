# AGENTS.md — `acta-graph`

> Arbeitskontext für Agenten. Gültig gemäß [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) (Fassung 3.0 Final).

## Strikte Regeln für Agenten
1. **Keine verwaisten Kanten:** Bei Löschungen müssen Kanten beidseitig bereinigt werden.
2. **Lineage beachten:** Alle Graphelemente zwingend an `doc_ord` koppeln.
3. **Ring R1:** Abhängigkeiten nur nach unten.
