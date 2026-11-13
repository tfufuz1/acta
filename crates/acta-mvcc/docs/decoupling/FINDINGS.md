# Findings während der Entkopplung von acta-mvcc

Keine internen Fehler in acta-mvcc.
Vorbestehender Fehler auf `main` außerhalb von BESITZ:
- `crates/acta-store/src/sstable/builder.rs:621:3: unclosed delimiter`
Liegt nicht in acta-mvcc und wurde vorschriftsmäßig unberührt gelassen.
