# acta-graph — Crate-Spezifikation (Fassung 3.0 Final)

*Bezug: Normative Gesamtspezifikation [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) (Teil III, IV, VI, VII, VIII)*  
*Ring: R1 (Index / Graph) · Geltung: Verbindlich für `acta-graph`*

---

## 1. Zweck und Einordnung

`acta-graph` implementiert den CSR-Wissensgraphen (Compressed Sparse Row) für GraphRAG, Pfad-Traversierung und Community-Detection:
- **Graph-Topologie:** Bitemporale Kanten und Knoten (`born_seq` / `died_seq`).
- **Lineage-Zwang (F-3, S10):** Jeder Knoten und jede Kante ist an ein `doc_ord: u32` gebunden (`source_set: RoaringBitmap`).
- **Lösch-Bereinigung:** Tilgung von Kanten und Entitäten betroffener Personen im Purge-Fenster.
- **ErasureLayer-Implementierung:** `on_gate` invalidiert flüchtige Traversierungs-Caches; `purge` entfernt Knoten und repariert Nachbarschaften; `witness` prüft Abwesenheit (`LayerWitness`).

---

## 2. Architektur und Verträge

| Merkmal | Vorgabe (docs/SPEC.md §3.1) |
|---|---|
| **Ring** | **R1 (Index)** |
| **Erlaubte interne Abhängigkeiten** | Ring R0 (`acta-ids`, `acta-env`), Ring R2 (`acta-types`, `acta-ports`) |
| **Default-Features** | `[]` (leer) |
| **Unsafe-Policy** | `#![forbid(unsafe_code)]` |
| **Fehlertyp** | `GraphError` (Fail-Closed) |

---

## 3. Normative Anforderungen

1. **Keine verwaisten Kanten nach Löschung:**  
   Wird eine Entität gelöscht, müssen alle inzidenten Kanten im Purge-Fenster physisch entfernt werden.
2. **Gate-Maskierung:**  
   Graph-Traversierungen filtern Knoten gegen `ErasedOrds` in $O(1)$ vor der Expansion.
3. **Lineage-Bindung:**  
   Graph-Mutationen akzeptieren ausschließlich Entitäten und Kanten mit gültigem Herkunftsnachweis (`Derived<T>`).
