# acta-vector

Approximativer HNSW-Vektorindex, SIMD-Distanzberechnung, Ghost-Vector-Beseitigung und Rebuild-Saga für Acta.

## Rolle und Zweck
`acta-vector` ist das Vektorindex-Crate in Ring R1:
- **HNSW mit SIMD:** Skalierbare Ähnlichkeitssuche für hochdimensionale Embeddings.
- **Inversionsschutz:** Vektoren werden als personenbezogene Daten behandelt.
- **Ghost-Vector-Bereinigung:** Maskierung im Gate + Slot-Nullung + Graph-Rebuild.
- **Rebuild-Modus:** Verhindert Verfügbarkeitsausfälle durch segmentweites Schlüssel-Shredding (Behebung F-5).
- **ErasureLayer:** Bestätigung von `on_gate` und Zeugnisgenerierung (`LayerWitness`).
