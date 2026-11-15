# AGENTS.md — `acta-checkpoint`

> Arbeitskontext für Agenten. Gültig gemäß [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) (Fassung 3.0 Final).

## Strikte Regeln für Agenten
1. **Registrierungspflicht:** Niemals `fs::hard_link` oder Kopien von Datendateien ohne vorherige Eintragung in der `CopyRegistry` anlegen.
2. **RestoreGuard einhalten:** Backups älter als 72 Stunden dürfen nicht stillschweigend geladen werden.
3. **Ring R1:** Abhängigkeiten nur nach unten.
