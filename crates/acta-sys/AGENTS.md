# AGENTS.md — `acta-sys`

> Arbeitskontext für Agenten. Gültig gemäß [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) (Fassung 3.0 Final).

## Strikte Regeln für Agenten
1. **Unsafe-Disziplin:** Jeder `unsafe`-Block verlangt zwingend einen formalen `// SAFETY:`-Kommentar mit Beweisführung der Invarianten.
2. **Keine Datenrassen:** Niemals schreibbare `mmap`-Strukturen anlegen.
3. **Fail-Closed Poisoning:** Jeder I/O-Fehler beim Datenträgerabgleich vergiftet das Handle dauerhaft.
