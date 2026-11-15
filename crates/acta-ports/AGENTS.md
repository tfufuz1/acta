# AGENTS.md — `acta-ports`

> Arbeitskontext für Agenten. Gültig gemäß [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) (Fassung 3.0 Final).

## Strikte Regeln für Agenten
1. **Regel R-PORT:** Keine toten Ports definieren. Jeder Trait MUSS eine reale Produktionsimplementierung besitzen.
2. **Dyn-Kompatibilität:** Alle Traits müssen als `Arc<dyn Trait>` oder `Box<dyn Trait>` nutzbar sein.
3. **Ring R2:** Keine konkreten Implementierungen oder Persistenzlogik in dieses Crate aufnehmen.
