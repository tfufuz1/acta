# AGENTS.md — `acta-wire`

> Arbeitskontext für Agenten. Gültig gemäß [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) (Fassung 3.0 Final).

## Strikte Regeln für Agenten
1. **Zero-Copy mit Typestate:** Niemals unüberprüfte Rohpuffer parsen.
2. **Ring R0:** Keine Abhängigkeiten zu höheren Ringen.
3. **Endianness:** Alle numerischen Felder strikt Big-Endian kodieren.
