# acta-mvcc

Bitemporale Snapshot-Isolation, SequenceLog, Serializable Snapshot Isolation (SSI) und Snapshot-Revocation für Acta.

## Rolle und Zweck
`acta-mvcc` ist das Versionierungs-Crate in Ring R1:
- **Snapshot-Isolation:** Zeitreiseabfragen (`as_of`) über bitemporale Stände.
- **Snapshot-Revocation:** `revoke_covering` sperrt aktive Snapshots beim Setzen des Lösch-Gates.
- **Entkopplung:** MVCC steuert Versionen; das Lösch-Gate überstimmt jede historische Sicht.
