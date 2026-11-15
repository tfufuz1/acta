# acta-checkpoint — Crate-Spezifikation (Fassung 3.0 Final)

*Bezug: Normative Gesamtspezifikation [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) (Teil IV, VI, VII)*  
*Ring: R1 (Persistenz) · Geltung: Verbindlich für `acta-checkpoint`*

---

## 1. Zweck und Einordnung

`acta-checkpoint` koordiniert die Erstellung von Snapshots, Sicherungen und Verzweigungen (Forks):
- **CopyRegistry (Behebung V-10):** Lückenlose Erfassung aller physischen Dateikopien und Hardlinks von SSTables.
- **RestoreGuard (Zusage S9, F-7):** Schutz vor Wiederbelebung gelöschter Daten. Verweigert Rücksicherungen von Tresor-Ständen älter als $T_{\text{ROT\_BACKUP}} = 72\text{ h}$ (`RestoreBeyondHorizon`).
- **Checkpoint-Pinning:** `PinGuard` hält Sequenznummern gegen vorzeitige GC-Kompaktierung fest.
- **Physische Bereinigung:** Implementiert die Kopienbereinigung in Phase h des Purge-Fensters.

---

## 2. Architektur und Verträge

| Merkmal | Vorgabe (docs/SPEC.md §3.1) |
|---|---|
| **Ring** | **R1 (Persistenz)** |
| **Erlaubte interne Abhängigkeiten** | Ring R0 (`acta-ids`, `acta-env`, `acta-durable-fs`), Ring R2 (`acta-types`, `acta-ports`) |
| **Fehlertyp** | `CheckpointError` (Fail-Closed) |

---

## 3. Normative Anforderungen

1. **Behebung V-10 (Registrierungspflicht für Hardlinks):**  
   `CheckpointHardlinkCloner` darf niemals ohne Eintrag in die `CopyRegistry` physische SSTable-Hardlinks anlegen. Jede Kopie MUSS dem Löschkern bekannt sein.
2. **RestoreGuard Durchsetzung (S9, F-7):**  
   Beim Einspielen eines Backups prüft `RestoreGuard` das Erstellungsdatum. Liegt der Stand jenseits des Horizonts, bricht der Start hart mit `Err(CheckpointError::RestoreBeyondHorizon)` ab.
3. **Phase h im Purge-Fenster:**  
   Nach der Schlüsselvernichtung werden registrierte Kopien in der `CopyRegistry` entweder physisch getilgt oder als unlesbare Chiffrate im Audit-Bericht ausgewiesen.
