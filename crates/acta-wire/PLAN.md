# acta-wire — Umsetzungsplan (Fassung 3.0 Final)

*Bezug: Roadmap R0–R7 aus [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) §12*

## Meilensteine

| Stufe | Aufgabe | Ziel / Nachweis | Status |
|---|---|---|---|
| **R0** | Kapselung aller FlatBuffers `unsafe`-Generierungen mit `// SAFETY:` | Lint `undocumented_unsafe_blocks` | In Arbeit |
| **R1** | Fuzzing der Wire-Frame-Validierung gegen Torn-Tails | Fuzz-Target `fuzz_wire_buffer` | Geplant |

## Definition of Done
- 0 interne Abhängigkeiten.
- Zero-Panic in Validierungs- und Parsing-Routinen.
