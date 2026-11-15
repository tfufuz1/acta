# acta-erasure

Löschkern, monolithische Saga (E0–E9), ReadGate, Tresor mit Epochenrotation, forensische Typestate-Messung und Audit-Bericht für Acta.

## Rolle und Zweck
`acta-erasure` ist das zentrale Lösch-Crate in Ring R3:
- **Lösch-Saga:** Atomare, monotone Zustandsmaschine für DSGVO-Löschungen.
- **ReadGate:** $O(1)$-Lesesperre über `ErasedOrds: RoaringBitmap`.
- **Kryptografischer Tresor:** Maskierte Shard-Slots (U, T, C, S, D, F, E) mit Epochenrotation und Tag-Nullung bei Grabsteinen.
- **Audit-Zertifikat:** Deterministischer, signierter `ErasureReport` v1 nach RFC 8785 (JCS).

## Workspace-Mitgliedschaft (Behebung V-03)
`acta-erasure` ist vollwertiges Mitglied im Root-Workspace (`members` und `default-members`). Default-Features sind `["signing", "vault"]`.
