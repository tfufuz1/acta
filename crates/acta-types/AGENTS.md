# AGENTS.md — `acta-types`

> Arbeitskontext für Agenten. Gültig gemäß [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) (Fassung 3.0 Final).

## Strikte Regeln für Agenten
1. **Gate G-WORD einhalten:** Begriffe wie „DSGVO-konform“ oder „garantiert gelöscht“ sind im Code und in Kommentaren streng verboten.
2. **Typzwang (S10):** Ableitungen immer als `Derived<T>` mit gültiger Lineage modellieren.
3. **Ring R2:** Nur Verträge, keine Persistenz- oder Krypto-Logik.
