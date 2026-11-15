# AGENTS.md — `acta-store`

> Arbeitskontext für Agenten. Gültig gemäß [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) (Fassung 3.0 Final).

## Strikte Regeln für Agenten
1. **Kein Klartext in P:** Nutzdaten der Klasse P MÜSSEN vor dem Schreiben in Memtable oder WAL als `Sealed<P>` vorliegen.
2. **KeyV2 verwenden:** Niemals ASCII-Schlüsselformat verwenden. `cluster` ist zwingend `subject_tag`.
3. **Partitionen trennen (I-7):** Niemals Partitionen N, P und D in dieselbe SSTable mergen.
4. **Lesepfade:** Keine ungegateten Lesemethoden nach außen exportieren (`pub(crate)` für `LsmStorage`).
