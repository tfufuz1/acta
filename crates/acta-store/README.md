# acta-store

Durabler LSM-Tree-Speicher, WAL mit HMAC-Kette, KeyV2, Partitionen N/P/D und gefilterte Rewrites für Acta.

## Rolle und Zweck
`acta-store` ist die Persistenz-Engine in Ring R1:
- **Klassentrennung:** Physisch getrennte Partitionen **N, P, D** je Mandant.
- **KeyV2 mit Lokalität:** 78-Byte-Schlüssel mit `cluster = subject_tag` (Ermöglicht atomaren Datei-Drop bei Löschungen).
- **Verschlüsselung vor WAL:** Klasse P wird vor dem Schreiben in den WAL mit Umschlag v3 versiegelt.
- **Lineage-SSTables:** Format 5 mit `source_set: RoaringBitmap` im Footer.
- **Compaction mit ErasedView:** Automatisches Verwerfen gelöschter Chiffrate beim Mergen.

## Schnittstellen-Sicherheit
`LsmStorage` ist `pub(crate)`. Jeder öffentliche Lesepfad führt zwingend über den gegateten Reader in der Engine (Gate `G-READ-1`).
