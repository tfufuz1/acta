# acta-erasure — Umsetzungsplan (Fassung 3.0 Final)

*Bezug: Roadmap R0–R7 aus [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) §12*

## Meilensteine

| Stufe | Aufgabe | Ziel / Nachweis | Status |
|---|---|---|---|
| **R0** | Workspace-Integration & Beseitigung aller Krypto-Fallbacks (V-01, V-03, V-05) | Pflichttests V-01, V-05 grün; `members` vollständig | In Arbeit |
| **R0** | Reparatur der `proof_*`-Integrations-Tests (V-14) | Tests kompilieren und bestehen | In Arbeit |
| **R1** | Tresor-Maskierung unter $K_{\text{enc}}$ & Epochenrotation (V-04) | Test `slot_payload_masked_at_rest`, Crash C11 | In Arbeit |
| **R1** | Batch-Vernichtung `destroy_many` (V-12) | Sweeper-Benchmark | In Arbeit |
| **R2** | Walking Skeleton: Durchstich E2 $\rightarrow$ E2a $\rightarrow$ E2b $\rightarrow$ E3 $\rightarrow$ Messung | Test `T-DEL-1` und Kanarien Ende-zu-Ende | In Arbeit |
| **R3** | Pseudonymitätsfreies Widerrufs-Log mit Commitments (V-15) | Test `revocation_log_has_no_raw_ids` | Geplant |
| **R7** | Standalone-Prüftool `acta-verify` und Notar-Audit | Unabhängige Verifikation ohne Acta-Code | Geplant |

## Definition of Done
- Sämtliche Pflichttests aus §11.2 von `docs/SPEC.md` bestehen.
- Gate `G-ERA-1` und Gate `G1` bestanden.
