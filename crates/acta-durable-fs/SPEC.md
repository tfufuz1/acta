# acta-durable-fs — Crate-Spezifikation (Fassung 3.0 Final)

*Bezug: Normative Gesamtspezifikation [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) (Teil IV, V, VIII, XI)*  
*Ring: R0 (Fundament) · Geltung: Verbindlich für `acta-durable-fs`*

---

## 1. Zweck und Einordnung

`acta-durable-fs` stellt die dateisystembezogenen Persistenz- und Durabilitätsabstraktionen bereit:
- **`Vfs` & `DurableVfs`:** Robuste Dateizugriffe mit atomarem Rename, `fsync`-Garantien und Verzeichnis-Syncs (`sync_dir`).
- **Sicheres Überschreiben (`scrub_if_safe`):** Physisches Null- oder Zufallsüberschreiben von Datenträgerblöcken vor der Dateifreigabe (Gate `G-FS-1`).
- **Fehlerinjektions-VFS (`FaultVfs`):** Deterministische Simulation von I/O-Fehlern, `ENOSPC` und Abstürzen zur Verifikation der Crash-Matrix C1 bis C12.

---

## 2. Architektur und Verträge

| Merkmal | Vorgabe (docs/SPEC.md §3.1) |
|---|---|
| **Ring** | **R0 (Fundament)** |
| **Interne Abhängigkeiten** | **0 (keine)** |
| **Fehlertyp** | `DurableFsError` (Fail-Closed) |
| **Gate G-FS-1** | Kein ungeschütztes `std::fs::remove_file` für P-Dateien und Tresor |

---

## 3. Normative Anforderungen

1. **Gate G-FS-1 Durchsetzung:**  
   Dateien aus Ebene P und dem Tresor dürfen niemals ohne vorheriges `scrub_if_safe` (Überschreiben mit Nullen oder Zufallsmuster) gelöscht werden, sofern das Dateisystem und Betriebssystem dies unterstützen.
2. **Crash-Matrix C1 bis C12:**  
   `FaultVfs` MUSS an jedem beliebigen Sync- oder Write-Punkt reproduzierbare Prozessabbrüche und E/A-Fehler (`EIO`, `ENOSPC`, `EMFILE`) injizieren können.
3. **Atomare Ersetzung:**  
   Metadaten, Manifeste und Tresordateien werden über temporäre Dateien geschrieben, gefsynct und per atomarem `rename` mit nachfolgendem `fsync(parent_dir)` aktiviert.
