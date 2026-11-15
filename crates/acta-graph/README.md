# acta-graph

CSR-Wissensgraph, bitemporale Topologie, GraphRAG und Herkunfts-Bereinigung für Acta.

## Rolle und Zweck
`acta-graph` ist das Wissensgraph-Crate in Ring R1:
- **CSR-Topologie:** Speicheroptimierte Graphen für Pfad-Traversierung und Community Detection.
- **Lineage:** Bindung von Knoten und Kanten an monotone `doc_ord`.
- **ErasureLayer:** Maskierung im Gate und physische Bereinigung inzidenter Kanten im Purge-Fenster.
