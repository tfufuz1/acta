# acta-sys — Umsetzungsplan (Fassung 3.0 Final)

*Bezug: Roadmap R0–R7 aus [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) §12*

## Meilensteine

| Stufe | Aufgabe | Ziel / Nachweis | Status |
|---|---|---|---|
| **R0** | Strikter `undocumented_unsafe_blocks` Audit | Alle `unsafe`-Blöcke tragen `// SAFETY:` | In Arbeit |
| **R0** | Handle-Poisoning bei fehlgeschlagenen `durable_flush_dir` Aufrufen | Test `poison_on_sync_failure` | In Arbeit |
| **R1** | Zeroizing + Memory-Fence vor `munlock` auditieren | Valgrind / Speicheranalyse | Geplant |

## Definition of Done
- 0 unkommentierte `unsafe`-Stellen.
- 0 interne Abhängigkeiten.
