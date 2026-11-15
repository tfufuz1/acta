# acta-durable-fs

Dauerhafte Dateisystem-Operationen, atomare Umbenennungen, sicheres Überschreiben (`scrub_if_safe`) und Crash-Injektion (`FaultVfs`) für Acta.

## Rolle und Zweck
`acta-durable-fs` ist ein Ring-0-Fundamentbaustein:
- **`DurableVfs`:** Virtuelles Dateisystem mit strikter Durchsetzung von Datei- und Verzeichnis-Fsyncs.
- **Sicheres Löschen (`scrub_if_safe`):** Überschreiben von Datenblöcken vor der Dateifreigabe (Gate `G-FS-1`).
- **`FaultVfs`:** Deterministischer Test-Treiber zur Simulation der Crash-Matrix C1 bis C12.
