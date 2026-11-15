# acta-mvcc — Umsetzungsplan (Fassung 3.0 Final)

*Bezug: Roadmap R0–R7 aus [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) §12*

## Meilensteine

| Stufe | Aufgabe | Ziel / Nachweis | Status |
|---|---|---|---|
| **R2** | Implementierung von `revoke_covering` für Lösch-Gate E2 | Test `SnapshotRevoked` | In Arbeit |
| **R3** | Transaktionsabbruch bei gegateten Zielen in SSI | Property-Tests `ssi_abort_on_erase` | In Arbeit |
| **R4** | Zeitreise-Szenarien T-TT-1 bis T-TT-8 | Zeitreise-Testsuite grün | Geplant |

## Definition of Done
- `as_of` liefert für gelöschte Ziele immer `Erased`.
- `#![forbid(unsafe_code)]` eingehalten.
