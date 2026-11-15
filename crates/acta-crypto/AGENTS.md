# AGENTS.md — `acta-crypto`

> Arbeitskontext für Agenten. Gültig gemäß [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) (Fassung 3.0 Final).

## Strikte Regeln für Agenten
1. **Keine Fallbacks (Zusage S7):** Es gibt keinen Fallback-Schlüssel, keinen Default-Schlüssel und keinen XOR-Strom. Jeder Fehler bricht hart ab (`CryptoError`).
2. **Keine Hardcoded Keys:** `[0x55; 32]`, `[0x42; 32]` oder Nullschlüssel sind verboten.
3. **AEAD v3:** Ausschließlich XChaCha20-Poly1305 verwenden. Nonces kommen nur aus `Rng`.
4. **Ring R0:** Keine Abhängigkeiten zu höheren Ringen (R1, R2, R3).
