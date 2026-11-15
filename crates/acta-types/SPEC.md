# acta-types — Crate-Spezifikation (Fassung 3.0 Final)

*Bezug: Normative Gesamtspezifikation [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) (Teil II, III, IV, VII, VIII)*  
*Ring: R2 (Systemverträge) · Geltung: Verbindlich für `acta-types`*

---

## 1. Zweck und Einordnung

`acta-types` definiert die zentralen Domänentypen, Datenklassen, typgesicherten Lineage-Container und Fehlerstrukturen für das Gesamtsystem:
- **Datenklassen (Regel R-CLS):** `DataClass` (`Norm`, `Evidence`, `Personal`, `Special`, `Derived`, `Unclassified`).
- **Typgesicherte Lineage (Zusage S10, Prinzip P3):** `Derived<T>` kapselt Nutzdaten untrennbar mit einer nicht-leeren `Lineage { source: RoaringBitmap }`.
- **Kryptografische Schale (Prinzip P2):** `Sealed<P>` erzwingt, dass Personendaten in Ebene P ausschließlich verschlüsselt transportiert werden.
- **Nachweis- und Messklassen:** `MeasureClass` (MK1, MK2, MK3), `ReachStatus` für die Klassen G, K, P, D, C.
- **Fehlertypen:** Zentraler `ActaError` ohne undurchsichtige String-Oracles.

---

## 2. Architektur und Verträge

| Merkmal | Vorgabe (docs/SPEC.md §3.1) |
|---|---|
| **Ring** | **R2 (Systemverträge)** |
| **Erlaubte interne Abhängigkeiten** | Ausschließlich Ring R0 (`acta-ids`, `acta-env`) |
| **Default-Features** | `[]` (leer) |
| **Wortfilter (Gate G-WORD)** | Striktes Verbot von „DSGVO-konform“, „garantiert gelöscht“ etc. |

---

## 3. Normative Anforderungen

1. **Typzwang für Herkunft (S10):**  
   Ein Ableitungsobjekt (`Derived<T>`) kann ohne Herkunftsinformation (`RoaringBitmap` von `doc_ord`) nicht konstruiert werden.
2. **Datenklassen-Pflicht (R-CLS):**  
   Schreibvorgänge ohne deklarierte Datenklasse scheitern zur Compile-Zeit oder mit `ClassViolation`.
3. **Auflösung der Trait-Doppelung:**  
   Die ehemals in `acta-erasure::types_local` duplizierten Typen werden kanonisch in `acta-types` zusammengeführt.
4. **Fehlermodell:**  
   Fehler im Krypto- und Löschpfad sind typisiert und lecken keine Klartexte oder Schlüsselüberreste.
