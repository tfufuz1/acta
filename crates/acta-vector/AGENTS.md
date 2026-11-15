# AGENTS.md — `acta-vector`

> Arbeitskontext für Agenten. Gültig gemäß [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) (Fassung 3.0 Final).

## Strikte Regeln für Agenten
1. **Ghost Vectors beseitigen:** Reines Soft-Delete ist verboten. Vektorslots müssen genullt und der HNSW-Graph repariert werden.
2. **Rebuild-Modus beachten (F-5):** Segmente nicht über globale ArtKeys unlesbar machen, wenn fremde Vektoren enthalten sind.
3. **On-Gate Pflicht:** Bei `on_gate` müssen flüchtige Vektoren im RAM sofort genullt werden.
