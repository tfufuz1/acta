# AGENTS.md — `acta-durable-fs`

> Arbeitskontext für Agenten. Gültig gemäß [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) (Fassung 3.0 Final).

## Strikte Regeln für Agenten
1. **Gate G-FS-1:** Niemals `std::fs::remove_file` direkt verwenden. Für sensible Daten MUSS immer `scrub_if_safe` aufgerufen werden.
2. **Durabilität:** Bei Dateierstellungen oder atomaren Renames MUSS zwingend auch das übergeordnete Verzeichnis gefsynct werden (`sync_dir`).
3. **Ring R0:** Keine internen Abhängigkeiten.
