# AGENTS.md — `acta-mcp`

> Arbeitskontext für Agenten. Gültig gemäß [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) (Fassung 3.0 Final).

## Strikte Regeln für Agenten
1. **as_of erzwingen:** Jede Abfrage MUSS einen expliziten Zeitstempel für bitemporale Nachvollziehbarkeit führen.
2. **Keine PII in Logs:** Prompts und Dokumenteninhalte niemals in Standard-Traces ausgeben.
3. **Gate G-WORD:** Keine unzulässigen Garantieversprechen in Tool-Dokumentationen.
