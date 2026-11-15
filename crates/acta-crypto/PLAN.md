# acta-crypto — Umsetzungsplan (Fassung 3.0 Final)

*Bezug: Roadmap R0–R7 aus [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) §12*

## Meilensteine

| Stufe | Aufgabe | Ziel / Nachweis | Status |
|---|---|---|---|
| **R0** | Beseitigung aller Krypto-Fallbacks & Konstanten (V-01, V-05) | Pflichttests V-01, V-05 grün; `clippy.toml` aktiv | In Arbeit |
| **R1** | Implementierung Umschlag v3 (XChaCha20-Poly1305) | Test `envelope_is_aead`, Testvektoren | In Arbeit |
| **R1** | Hierarchische DEK-Ableitung H1–H5 ohne Duplikate | Unit-Tests mit Mehrbetroffenen | In Arbeit |
| **R1** | Commitments via $K_{\text{commit}}[e]$ für Ledger und Widerrufs-Log | Test `revocation_log_has_no_raw_ids` | In Arbeit |

## Definition of Done
- Kein `Default` für Schlüsselcontainer.
- Nonce-Generierung ausschließlich über `Rng`.
- Gate `G-CRY-1` und Gate `G1` bestanden.
