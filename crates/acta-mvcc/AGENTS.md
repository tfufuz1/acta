# AGENTS.md — `acta-mvcc`

> Arbeitskontext für Agenten. Gültig gemäß [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) (Fassung 3.0 Final).

## Strikte Regeln für Agenten
1. **Löschung triumphiert:** Tombstones sind keine DSGVO-Löschung. Das Lösch-Gate überstimmt die MVCC-Sichtbarkeit bedingungslos.
2. **Keine Wiederverwendung:** Sequenznummern werden niemals wiederverwendet (Invariante I-6).
3. **Ring R1:** Abhängigkeiten nur nach unten (R0, R2).
