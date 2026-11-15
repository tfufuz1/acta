# acta-wire — Crate-Spezifikation (Fassung 3.0 Final)

*Bezug: Normative Gesamtspezifikation [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) (Teil III, IV, IX)*  
*Ring: R0 (Fundament) · Geltung: Verbindlich für `acta-wire`*

---

## 1. Zweck und Einordnung

`acta-wire` stellt Zero-Copy-Serialisierung und Frame-Parsing für IPC, Netzwerk und binäre Persistenzformate bereit:
- **Wire-Puffer:** `WireBuffer` mit Typestate-Verifikation (`Unverified` $\rightarrow$ `Verified<T>`).
- **Endianness:** Strikt Big-Endian für alle binären Ganzzahlen (u16, u32, u64).
- **Torn-Tail-Schutz:** Robuste Erkennung partieller Frames am Dateiende.

---

## 2. Architektur und Verträge

| Merkmal | Vorgabe (docs/SPEC.md §3.1) |
|---|---|
| **Ring** | **R0 (Fundament)** |
| **Interne Pflicht-Abhängigkeiten** | **0 (keine)** |
| **Default-Features** | `["std"]` |
| **Fehlertyp** | `WireError` (Fail-Closed) |

---

## 3. Normative Anforderungen

1. **Typestate-Validierung:**  
   Zugriffe auf FlatBuffers-Payloads verlangen den Nachweis einer erfolgreichen Verifikation gegen das Wurzelschema (`verify<T>()`).
2. **Keine PII im Wire-Header:**  
   Header enthalten ausschließlich Längenfelder, Prüfsummen, Magic-Bytes und Typkennungen.
3. **Fail-Closed bei Korruption:**  
   Unerwartete Bytes oder Magic-Fehlschläge brechen die Deserialisierung sofort ab.
