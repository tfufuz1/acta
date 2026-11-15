# acta-types — Umsetzungsplan (Fassung 3.0 Final)

*Bezug: Roadmap R0–R7 aus [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) §12*

## Meilensteine

| Stufe | Aufgabe | Ziel / Nachweis | Status |
|---|---|---|---|
| **R0** | Wortfilter-Audit (Gate G-WORD) | Keine verbotenen Marketingbegriffe | In Arbeit |
| **R2** | Definition von `Derived<T>` mit Roaring-Lineage | Compile-Fail Tests für leere Herkunft | In Arbeit |
| **R3** | Auflösung der Duplikate aus `acta-erasure::types_local` | Konsolidierte Typdefinitionen | Geplant |

## Definition of Done
- Gate `G-WORD` und Gate `G1` bestanden.
- Typzwang für Herkunft kompilierbar.
