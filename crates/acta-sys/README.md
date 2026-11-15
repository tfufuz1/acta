# acta-sys

Systemnahe Plattform-Primitive, Speichersperrung (`mlock`), sichere Speicherabbildung und Dateisystem-Synchronisation für Acta.

## Rolle und Zweck
`acta-sys` bildet eine gekapselte `unsafe`-Insel in Ring R0 zur Interaktion mit dem Betriebssystemkernel:
- **`mlock` & `MADV_DONTDUMP`:** Schutz sensibler Krypto-Schlüssel vor Auslagerung (Swap) und Core-Dumps.
- **`mmap_readonly`:** Unveränderliche Nur-Lese-Speicherabbildungen für SSTables.
- **Dauerhafte Synchronisation:** Platten-Flushs für Dateien und übergeordnete Verzeichnisse (`durable_flush_dir`).
- **Handle-Poisoning:** Permanente Vergiftung bei I/O-Synchronisationsfehlern.
