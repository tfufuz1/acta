# acta-rank — Crate-Spezifikation (Fassung 3.0 Final)

*Bezug: Normative Gesamtspezifikation [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) (Teil III, IV, VII)*  
*Ring: R1 (Retrieval / Ranking) · Geltung: Verbindlich für `acta-rank`*

---

## 1. Zweck und Einordnung

`acta-rank` implementiert Re-Ranking-, Score-Fusions- (RRF) und Kalibrierungsverfahren:
- **Hybrid-Fusion:** Reciprocal Rank Fusion über BM25- und Vektor-Ergebnisse.
- **Gate-Filterung:** Striktes Ausblenden gegateter Kandidaten (`ErasedOrds`) vor dem Scoring.
- **Keine PII-Akkumulation:** Ranker arbeiten rein auf IDs und Scores; Klartextinhalte werden nicht gepuffert.

---

## 2. Architektur und Verträge

| Merkmal | Vorgabe (docs/SPEC.md §3.1) |
|---|---|
| **Ring** | **R1 (Retrieval / Ranking)** |
| **Erlaubte interne Abhängigkeiten** | Ring R0 (`acta-ids`, `acta-env`), Ring R2 (`acta-types`, `acta-ports`) |
| **Default-Features** | `["std"]` |
| **Fehlertyp** | `RankError` (Fail-Closed) |

---

## 3. Normative Anforderungen

1. **Gate-Prüfung vor Fusion:**  
   Trefferlisten aus Text- und Vektorindizes werden vor der Score-Berechnung gegen `ReadGate` gefiltert.
2. **Determinismus:**  
   Score-Gleichstände (Ties) werden deterministisch nach aufsteigender Dokument-ID aufgelöst.
