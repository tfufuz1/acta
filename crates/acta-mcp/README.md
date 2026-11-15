# acta-mcp

Model Context Protocol (MCP) Server zur LLM-Integration mit bitemporalem Zeitreise-Retrieval und Lösch-Schnittstellen für Acta.

## Rolle und Zweck
`acta-mcp` ist das Schnittstellen-Crate in Ring R7:
- **RAG-Tools:** Semantische Suche mit Pflicht-Parameter `as_of`.
- **Lösch-Integration:** Auslösung von Löschanträgen (`erasure_request`) direkt aus Agenten-Workflows.
- **Transparenz:** Rückmeldung von `Erased` bei getilgtem Schlüsselmaterial.
