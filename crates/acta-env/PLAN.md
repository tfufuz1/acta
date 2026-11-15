# acta-env — Umsetzungsplan (Fassung 3.0 Final)

*Bezug: Roadmap R0–R7 aus [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) §12*

## Meilensteine

| Stufe | Aufgabe | Ziel / Nachweis | Status |
|---|---|---|---|
| **R0** | Durchsetzung Gate G3 (`clippy.toml disallowed_methods`) | Keine direkten OS-Aufrufe in Kern-Crates | In Arbeit |
| **R1** | Prozesshärtungs-Schnittstelle für RAM/Swap (A-RAM) | Härtungstest auf Linux x86_64 | Geplant |

## Definition of Done
- `#![forbid(unsafe_code)]` strikt eingehalten.
- Determinismus-Harness für alle zeit- und zufallsabhängigen Tests einsatzbereit.
