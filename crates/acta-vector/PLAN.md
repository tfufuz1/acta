# acta-vector — Umsetzungsplan (Fassung 3.0 Final)

*Bezug: Roadmap R0–R7 aus [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) §12*

## Meilensteine

| Stufe | Aufgabe | Ziel / Nachweis | Status |
|---|---|---|---|
| **R2** | Maskierung im Suchpfad via `ErasedOrds` | Test `T-VEC` im Gate | In Arbeit |
| **R5** | Implementierung des Rebuild-Modus ohne ArtKey-Kollateralschaden (F-5) | Test `hnsw_no_residue_after_erase` | Geplant |
| **R5** | Vollständige `ErasureLayer`-Anbindung mit `on_gate`-Ack | Test `vector_layer_witness` | Geplant |

## Definition of Done
- Ghost Vectors werden physisch genullt und der Graph repariert.
- Zusage S10 (Lineage) für alle Vektoren erfüllt.
