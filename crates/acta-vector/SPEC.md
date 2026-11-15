# acta-vector — Crate-Spezifikation (Fassung 3.0 Final)

*Bezug: Normative Gesamtspezifikation [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) (Teil III, IV, V, VI, VII, VIII)*  
*Ring: R1 (Index / Vektor) · Geltung: Verbindlich für `acta-vector`*

---

## 1. Zweck und Einordnung

`acta-vector` implementiert den hochperformanten HNSW-Vektorindex mit SIMD-Beschleunigung und SQ8-Quantisierung:
- **Rechtlicher Status:** Vektor-Embeddings stellen personenbezogene Daten dar (Inversionsschutz gegen Vec2Text). Die Quelle liegt verschlüsselt in P, der Index ist Ebene D.
- **Ghost-Vector-Beseitigung (DSK OH RAG):** Reines Soft-Delete via Bitmaske genügt nicht. Acta erzwingt:
  1. Sofortige Maskierung im Suchpfad via `ErasedOrds: RoaringBitmap`.
  2. Physisches Nullen der Vektorslots im Speicher.
  3. Rebuild der Graph-Topologie ohne gelöschte Knoten.
- **Segment-Modi (Behebung F-5):**
  - **`Rebuild` (Standard):** Segmente teilen keinen Vernichtungsschlüssel; fremde Betroffene bleiben uneingeschränkt verfügbar.
  - **`Sealed`:** Segment unter `ArtKey` (Slot E); begrenzt auf $\le 50\,000$ Vektoren.
- **ErasureLayer-Implementierung:** `on_gate` nullt flüchtige Vektoren (Pflicht-Ack E2b); `purge` regeneriert Segmente in Phase e; `witness` liefert `LayerWitness`.

---

## 2. Architektur und Verträge

| Merkmal | Vorgabe (docs/SPEC.md §3.1) |
|---|---|
| **Ring** | **R1 (Index)** |
| **Erlaubte interne Abhängigkeiten** | Ring R0 (`acta-ids`, `acta-simd`, `acta-env`, `acta-sys`), Ring R2 (`acta-types`, `acta-ports`) |
| **Default-Features** | `["std"]` (kein `ports-compat` im Default) |
| **Fehlertyp** | `VectorError` (Fail-Closed) |

---

## 3. Normative Anforderungen

1. **Behebung F-5 (Kein Verfügbarkeitskollaps durch ArtKey):**  
   Standardmäßig arbeiten Vektorsegmente im Modus `Rebuild`. Die Vernichtung einer Person sperrt deren Vektoren im Gate und stösst einen asynchronen Rebuild an, ohne zehntausende Vektoren anderer Betroffener unlesbar zu machen.
2. **Pflicht-Ack für `on_gate` (E2b):**  
   Bei Eingang von `on_gate` MÜSSEN flüchtige Vektor-Caches und In-Memory-Graphknoten der Zielmenge genullt werden, bevor Schritt E3 ausgeführt werden darf.
3. **Lineage-Zwang (Zusage S10):**  
   Jeder Vektorknoten ist an sein monotones `doc_ord` gebunden. Segmente führen ein `source_set: RoaringBitmap`.
