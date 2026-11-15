# AGENTS.md — `acta-ids`

> Arbeitskontext für Agenten. Gültig gemäß [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) (Fassung 3.0 Final).

## Leitlinien für Agenten
1. **Ring R0:** Keine internen Abhängigkeiten zu anderen Workspace-Crates zulässig.
2. **Keine PII:** Keine Felder für personenbezogene Rohdaten anlegen. Identifikatoren sind typisierte Zufallswerte oder BLAKE3-Tags.
3. **Fail-Closed:** Kein `Default` für IDs implementieren.
4. **Lineage:** `DocOrd` ist ein `u32` Newtype für monotone Dokument-Ordinale.
