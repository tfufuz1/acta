# AGENTS.md — `acta-erasure`

> Arbeitskontext für Agenten. Gültig gemäß [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) (Fassung 3.0 Final).

## Strikte Regeln für Agenten
1. **Keine Fallbacks (Zusage S7):** Bei gelöschten oder fehlenden Slots MUSS `Err(ErasureError::Erased)` gemeldet werden. Ein Fallback auf BLAKE3 oder Dummy-Schlüssel ist absolut verboten.
2. **Reihenfolge einhalten (F-1, F-2):** Gate E2 IMMER vor E3. E3 DARF ERST nach bestätigtem `on_gate`-Ack (E2b) ausgeführt werden.
3. **Grabstein-Hygiene (V-11):** Grabsteine nullen `tag` und `payload` vollständig aus.
4. **Keine Klartext-IDs in Logs (V-15):** Widerrufs-Log und Ledger dürfen ausschließlich keyed Commitments führen.
5. **Einziger Löschpfad (Gate G-ERA-1):** Löschungen ausschließlich über `ErasureCore::execute` ausführen.
