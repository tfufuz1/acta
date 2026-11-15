# acta-checkpoint

Snapshot-Koordination, `CopyRegistry` für SSTable-Hardlinks und `RestoreGuard` für Acta.

## Rolle und Zweck
`acta-checkpoint` steuert Sicherungen und Snapshots in Ring R1:
- **`CopyRegistry`:** Verhindert unkontrollierte Hardlinks (Behebung V-10).
- **`RestoreGuard`:** Blockiert die Wiederherstellung veralteter Schlüsselbackups (Zusage S9).
- **Pinning:** Schützt aktive Snapshots während der Verarbeitung vor voreiligem GC.
