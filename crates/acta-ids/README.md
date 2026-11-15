# acta-ids

Kanonische Identifikatoren, typisierte Schlüssel, monotone Ordinale und kryptografische Tags für die Acta-Architektur.

## Rolle und Zweck
`acta-ids` ist ein autarker Ring-0-Fundamentbaustein ohne interne Abhängigkeiten. Es stellt typensichere Identifikatoren bereit:
- **Typisierte IDs:** `TenantId`, `CollectionId`, `DocId`, `SubjectId`, `EntityId`, `TxId`, `EpochId`.
- **Dichte Ordinale:** `DocOrd` (`u32`) zur Adressierung von Herkunftsmengen (`RoaringBitmap`) in $O(1)$.
- **KeyV2-Tags:** `TenantTag`, `CollectionTag`, `ClusterTag`, `DocTag` (16 Byte BLAKE3-keyed Hashes).

## Normative Vorgaben
- Verbot von Klartext-Personenbezug in IDs.
- `ClusterTag` entspricht dem `SubjectTag` (Physische Lokalität im LSM-Store).
- Kein `Default` für sicherheitsrelevante IDs (Fail-Closed).
