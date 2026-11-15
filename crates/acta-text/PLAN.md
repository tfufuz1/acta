# acta-text — Umsetzungsplan (Fassung 3.0 Final)

*Bezug: Roadmap R0–R7 aus [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) §12*

## Meilensteine

| Stufe | Aufgabe | Ziel / Nachweis | Status |
|---|---|---|---|
| **R2** | Walking Skeleton Durchstich: `TextLayer` Integration | Test `T-DEL-1` Ende-zu-Ende | In Arbeit |
| **R3** | Migration auf monotone `doc_ord` Postings | Test `postings_use_doc_ord` | Geplant |
| **R5** | Purge-Phase b Rewrite und `LayerWitness` | Test `T-TEXT` | Geplant |

## Definition of Done
- `ErasureLayer` synchron im Walking Skeleton lauffähig.
- 0 verbleibende Term-Postings nach Purge.
