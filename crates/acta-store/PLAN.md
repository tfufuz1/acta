# acta-store — Umsetzungsplan (Fassung 3.0 Final)

*Bezug: Roadmap R0–R7 aus [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) §12*

## Meilensteine

| Stufe | Aufgabe | Ziel / Nachweis | Status |
|---|---|---|---|
| **R0** | Behebung V-17 (Compaction-Deadlock) | Test `test_compaction_stress_and_gc` läuft durch | In Arbeit |
| **R3** | KeyV2-Migration & Beseitigung von ASCII-Keys (V-06) | Test `no_ascii_keys` | In Arbeit |
| **R3** | Partitionen N, P, D im Manifest v3 (V-07) | Invariante I-7 Property-Test | In Arbeit |
| **R3** | Wert-Verschlüsselung vor WAL (B-03) | Test `T-WAL-ENC` | In Arbeit |
| **R4** | Verdrahtung von `FlushPurgeFence::enter_purge` (V-08) | Loom-Modell Flush/Purge | Geplant |
| **R4** | Gefilterter Rewrite & SSTable Format 5 mit Roaring-Footer | Crash-Tests C7, C8 | Geplant |

## Definition of Done
- Keine Klartextdaten der Klasse P im WAL oder SST.
- Gate `G-READ-1` und Gate `G1` bestanden.
