# AGENTS.md — Acta Richtlinie für Agenten (Fassung 3.0 Final)

> **Verbindliche Arbeitsanweisung:** Dieses Dokument fasst die normativen Regeln für alle Agenten zusammen.
> **Alleinige normative Quelle:** [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) (Fassung 3.0 Final). Sämtliche älteren Notizen, Entwürfe und informelle Zusicherungen sind ungültig.

---

## 1. Mission und Grundsatzentscheidung

Das Ziel von Acta ist die Vereinbarung von atomarer, physischer und kryptografischer **Löschbarkeit von Personenbezug** mit einer bitemporalen **Zeitreise (MVCC)** über nicht-personenbezogene Sachverhalte:

1. **Konstruktive Löschbarkeit von Geburt an:**  
   Personenbezogene Daten werden von Anfang an in **Ebene P** unter individuellen Dokument-Schlüsseln (Umschlag v3, XChaCha20-Poly1305) versiegelt. Löschung erfolgt atomar durch Schlüsselvernichtung im Tresor ($O(1)$) gefolgt von physischem Purge.
2. **Bitemporale Zeitreise ohne Personendaten:**  
   Die Chronik (**Ebene N**) speichert Normen, Richtlinien, Prompt- und Modellstände zur lückenlosen Nachvollziehbarkeit von LLM-Entscheidungen. Ebene N ist strikt frei von personenbezogenen Daten.
3. **Löschung triumphiert über Zeitreise (Zusage S1, S6):**  
   Historische Zeitreiseabfragen (`as_of`) liefern für gelöschte Einheiten niemals Klartext, sondern belegen den Status `Erased`.

---

## 2. Verbotene Begriffe und Nicht-Zusagen (Gate G-WORD)

Agenten dürfen in Code, Kommentaren, Tests und Dokumentationen folgende Begriffe **niemals** als Software-Zusicherung verwenden:
- „DSGVO-konform“ oder „DSGVO-zertifiziert“ (UWG-Risiko; Konformität ist ein Gesamtbetriebszustand).
- „100 % gelöscht“, „garantiert vernichtet“, „unwiederbringlich“, „revisionssicher“, „fehlerfrei“.
- „Löscht aus allen Backups“.

**Zulässig ist ausschließlich:**  
Das Berichten der objektiv gemessenen technischen Klassen **G, K, P, D, C** mit dem Status `reached` oder `not_reached` unter Ausweisung der deklarierten Annahmen (**A-FLASH, A-RAM, A-ANCHOR, A-CRYPTO, A-EXT**).

---

## 3. Unverrückbare Invarianten für Entwicklungsarbeiten

- **Fail-Closed Doktrin (S7):** Kein Krypto-Fallback, kein Default-Schlüssel, keine `unwrap_or_default()`-Aufrufe im Schlüsselpfad.
- **Invariante I-17:** Strikt ein Umschlag je Datensatz in Ebene P; niemals Blockverschlüsselung über Dokumentgrenzen hinweg.
- **Saga-Reihenfolge (F-1, F-2):** Gate E2 IMMER vor E3. E3 (Schlüsselvernichtung) verlangt zwingend die Bestätigung von E2b (`on_gate` aller Schichten bestätigt).
- **Lineage (S10, F-3):** Jede Ableitung (`Derived<T>`) ist typbedingt an monotone `doc_ord: u32` gebunden (`RoaringBitmap`).
- **Cluster-Lokalität (F-4):** KeyV2 belegt `cluster = subject_tag`, um physische Lokalität im LSM-Tree zu garantieren.
- **Ghost Vectors beseitigen:** Im Vektorindex genügen Soft-Deletes nicht. Vektoren müssen genullt und der HNSW-Graph repariert/neu synthetisiert werden.
- **Ringarchitektur (R-DEP):** Abhängigkeiten zeigen streng von höheren zu niedrigeren Ringen (R7 $\rightarrow$ R6 $\rightarrow$ R5 $\rightarrow$ R4 $\rightarrow$ R3/R1 $\rightarrow$ R2 $\rightarrow$ R0).
