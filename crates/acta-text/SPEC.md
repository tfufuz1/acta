# acta-text — Crate-Spezifikation (Fassung 3.0 Final)

*Bezug: Normative Gesamtspezifikation [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) (Teil III, IV, VI, VII, VIII, XII)*  
*Ring: R1 (Index / Text) · Geltung: Verbindlich für `acta-text`*

---

## 1. Zweck und Einordnung

`acta-text` implementiert den invertierten BM25-Volltextindex mit WAND-Scoring und morphologischer Tokenisierung:
- **Herkunfts-Modell (F-3):** Posting-Segmente führen ein `source_set: RoaringBitmap` aller enthaltenen `doc_ord`.
- **Walking Skeleton (Stufe R2):** `acta-text` dient als erster Index für den vertikalen Durchstich der Löschkette (Ingest $\rightarrow$ WAL $\rightarrow$ TextIndex $\rightarrow$ `ErasureCore` $\rightarrow$ Messung $\rightarrow$ Bericht).
- **Purge-Fenster (Phase b):** Invertierte Listen betroffener Terme werden ohne Ziel-Ordinale neu geschrieben.
- **ErasureLayer-Implementierung:** `on_gate` invalidiert In-Memory-Postings; `purge` rewrite; `witness` prüft Term-Abwesenheit.

---

## 2. Architektur und Verträge

| Merkmal | Vorgabe (docs/SPEC.md §3.1) |
|---|---|
| **Ring** | **R1 (Index)** |
| **Erlaubte interne Abhängigkeiten** | Ring R0 (`acta-ids`, `acta-env`), Ring R2 (`acta-types`, `acta-ports`) |
| **Default-Features** | `["std"]` |
| **Fehlertyp** | `TextError` (Fail-Closed) |

---

## 3. Normative Anforderungen

1. **Walking Skeleton Priorität (R2):**  
   `acta-text` MUSS den minimalen Durchstich als erste voll integrierte `ErasureLayer`-Schicht demonstrieren (Test `T-DEL-1`).
2. **Roaring-Bitmap-Schnitt (F-3):**  
   Die Identifikation zu bereinigender Posting-Segmente erfolgt über:
   $$\text{source\_set} \cap \text{ErasedOrds} \neq \emptyset$$
3. **Phase b Rewrite:**  
   Postings gelöschter Betroffener werden in Phase b physisch getilgt. Terme, die keine verbleibenden Referenzen mehr besitzen, werden aus dem Vokabular entfernt.
