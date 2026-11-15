# acta-text

Invertierter BM25-Volltextindex mit WAND-Scoring, morphologischer Analyse und Roaring-Lineage für Acta.

## Rolle und Zweck
`acta-text` ist das Textindex-Crate in Ring R1:
- **BM25-Suche:** Performante Volltextsuche mit WAND-Optimierung.
- **Walking Skeleton:** Erste Indexschicht im minimalen Lösch-Durchstich (Stufe R2).
- **Lineage:** Posting-Listen mit `RoaringBitmap` zur $O(1)$-Purge-Entscheidung.
- **ErasureLayer:** Bereinigung in Phase b des Purge-Fensters.
