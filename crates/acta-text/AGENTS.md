# AGENTS.md — `acta-text`

> Arbeitskontext für Agenten. Gültig gemäß [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) (Fassung 3.0 Final).

## Strikte Regeln für Agenten
1. **Walking Skeleton:** `acta-text` bildet den Kern von Stufe R2. Alle Änderungen müssen den minimalen Ende-zu-Ende Löschtest unterstützen.
2. **Lineage mit doc_ord:** Postings ausschließlich über `doc_ord: u32` referenzieren.
3. **Ring R1:** Abhängigkeiten nur nach unten.
