# acta-ports — Umsetzungsplan (Fassung 3.0 Final)

*Bezug: Roadmap R0–R7 aus [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) §12*

## Meilensteine

| Stufe | Aufgabe | Ziel / Nachweis | Status |
|---|---|---|---|
| **R0** | Dead-Port-Prüfung vorbereiten (Gate G2) | Test `every_port_wired` | In Arbeit |
| **R2** | Definition von `ErasureLayer` mit Pflicht-Ack für `on_gate` | Walking-Skeleton-Verdrahtung | In Arbeit |
| **R3** | Bereitstellung von `ErasedView` für Store-Compaction | Integration mit `acta-store` | Geplant |

## Definition of Done
- Gate `G2` bestanden (alle Ports aktiv verdrahtet).
- Dyn-Safety für alle Austausch-Traits nachgewiesen.
