# acta-checkpoint — Umsetzungsplan (Fassung 3.0 Final)

*Bezug: Roadmap R0–R7 aus [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) §12*

## Meilensteine

| Stufe | Aufgabe | Ziel / Nachweis | Status |
|---|---|---|---|
| **R1** | `RestoreGuard` mit `T_ROT_BACKUP` (72 h) Horizont | Test `T-REST-1` | In Arbeit |
| **R5** | Einführung der `CopyRegistry` für `CheckpointHardlinkCloner` (V-10) | Test `all_hardlinks_registered` | Geplant |
| **R5** | Einbindung in Purge-Phase h | Test `copy_purged_in_ledger` | Geplant |

## Definition of Done
- Keine unregistrierten SSTable-Kopien (V-10 behoben).
- Zusage S9 durch automatisierte Restore-Tests nachgewiesen.
