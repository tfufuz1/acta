# acta-types

Zentrale Systemtypen, Datenklassen (`DataClass`), typisierte Lineage-Container (`Derived<T>`) und Fehlerstrukturen für Acta.

## Rolle und Zweck
`acta-types` ist ein Vertragscrate in Ring R2:
- **Datenklassen:** `Norm`, `Evidence`, `Personal`, `Special`, `Derived`, `Unclassified`.
- **Lineage-Typzwang:** `Derived<T>` mit `RoaringBitmap` zur lückenlosen Herkunftsverfolgung.
- **Nachweisklassen:** Klassen G, K, P, D, C für Audit-Berichte.
- **Fehlerbaum:** Strukturierter `ActaError` ohne Klartext-Lecks.
