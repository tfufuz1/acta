# acta-ids — Umsetzungsplan (Fassung 3.0 Final)

*Bezug: Roadmap R0–R7 aus [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) §12*

## Meilensteine

| Stufe | Aufgabe | Ziel / Nachweis | Status |
|---|---|---|---|
| **R0** | `DocOrd: u32` einführen, kein Default für sicherheitsrelevante IDs | Compile-Checks, `zero_panic` | In Arbeit |
| **R0** | `ClusterTag` als `SubjectTag` definieren (F-4) | KeyV2-Kompatibilität | In Arbeit |
| **R3** | Verknüpfung mit Store-Partitionen und Roaring-Lineage | Test mit `roaring = "0.10"` | Geplant |

## Definition of Done
- `#![forbid(unsafe_code)]` und `#![deny(clippy::unwrap_used)]` aktiv.
- 0 interne Abhängigkeiten (Gate G1).
