# acta-ports

Zentrale Schnittstellen-, Trait- und Port-Definitionen für die entkoppelte Acta-Architektur.

## Rolle und Zweck
`acta-ports` ist das Vertragscrate in Ring R2:
- **`ErasureLayer`:** Trait für Store-, Vektor-, Text-, Graph- und Cache-Schichten.
- **`ReadGate` & `ErasedView`:** $O(1)$-Lesesperren für gegatete Leser und Compaction.
- **`KeyAnchor`:** HSM-/KMS-Schnittstelle zur Schlüsselvernichtung.
- **`SnapshotRevoker`:** Schnittstelle zum Widerruf aktiver MVCC-Leser.

## Regel R-PORT
Jeder Port besitzt genau eine Produktionsimplementierung und eine Aufrufstelle (geprüft über Gate `G2`).
