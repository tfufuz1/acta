# acta-mcp — Crate-Spezifikation (Fassung 3.0 Final)

*Bezug: Normative Gesamtspezifikation [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) (Teil I, IV, VII, VIII)*  
*Ring: R7 (Schnittstellen / Anwendungen) · Geltung: Verbindlich für `acta-mcp`*

---

## 1. Zweck und Einordnung

`acta-mcp` stellt den Model Context Protocol (MCP) Server für Large Language Models bereit:
- **Kontext-Tools:** Vektor- und Hybridsuche mit Pflicht-Parameter `as_of` für deterministische Zeitreiseabfragen.
- **Lösch-Tools:** `erasure_request` zur Auslösung von DSGVO-Löschungen über die Saga mit Quittungs-Rückgabe.
- **Audit & Nachweis:** Abfrage signierter `ErasureReport`-Dokumente.
- **Egress-Schutz:** Strenges Verbot von PII in Server-Logs und Traces.

---

## 2. Architektur und Verträge

| Merkmal | Vorgabe (docs/SPEC.md §3.1) |
|---|---|
| **Ring** | **R7 (Schnittstellen)** |
| **Abhängigkeiten** | Greift über `acta` (Fassade) und R2-Ports auf das System zu |
| **Default-Features** | `[]` (leer) |
| **Fehlertyp** | `McpError` (Fail-Closed) |
| **Gate G-WORD** | Striktes Verbot irreführender Produktzusagen in Tool-Schemas |

---

## 3. Normative Anforderungen

1. **Pflichtparameter `as_of`:**  
   Jedes Retrieval-Tool verlangt zwingend einen `as_of`-Zeitstempel. Dadurch wird garantiert, dass LLM-Antworten auf einen präzisen historischen Stand fixiert werden können.
2. **Signal `Erased`:**  
   Wird ein Dokument referenziert, dessen Schlüssel vernichtet wurde, meldet das Tool transparent den Status `Erased`, anstatt fälschlich leere Ergebnisse oder Fehler zu simulieren.
3. **Observability ohne Personenbezug:**  
   MCP-Server-Logs und OpenTelemetry-Spans dürfen ausschließlich IDs, Handles und Systemmetriken enthalten; Prompts und Chunks dürfen niemals in ungesicherten Logs landen.
