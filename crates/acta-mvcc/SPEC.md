# acta-mvcc — Crate-Spezifikation (Fassung 3.0 Final)

*Bezug: Normative Gesamtspezifikation [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) (Teil IV, VI, VII)*  
*Ring: R1 (Persistenz / Versionierung) · Geltung: Verbindlich für `acta-mvcc`*

---

## 1. Zweck und Einordnung

`acta-mvcc` stellt die bitemporale Multi-Version Concurrency Control (MVCC) und Serializable Snapshot Isolation (SSI) bereit:
- **Snapshot-Registry & Isolation:** RAII-verwaltete Schnappschüsse (`SnapshotGuard`) mit lock-freiem Watermark-Tracking.
- **Snapshot-Revocation (`revoke_covering`):** Physischer Widerruf aktiver Snapshots, die gegatete Einheiten sehen könnten (Phase a des Purge-Fensters).
- **SequenceLog:** Monotone Sequenznummernverwaltung; Sequenznummern werden niemals wiederverwendet (Invariante I-6).
- **SSI-Validierung:** Write-Skew-Erkennung und Konflikterkennung. Transaktionen auf gegatete Einheiten brechen mit `Erased` ab.

---

## 2. Architektur und Verträge

| Merkmal | Vorgabe (docs/SPEC.md §3.1) |
|---|---|
| **Ring** | **R1 (Persistenz / Versionierung)** |
| **Erlaubte interne Abhängigkeiten** | Ring R0 (`acta-ids`, `acta-env`), Ring R2 (`acta-types`, `acta-ports`) |
| **Unsafe-Policy** | `#![forbid(unsafe_code)]` |
| **Fehlertyp** | `MvccError` (Fail-Closed) |

---

## 3. Normative Anforderungen

1. **Löschung triumphiert über Zeitreise (Zusage S1, S6):**  
   MVCC-Tombstones (`TOMBSTONE_BIT`) steuern rein die fachliche Versionssichtbarkeit. Sie genügen nicht den Anforderungen an eine Löschung. Das Lösch-Gate (`ErasedOrds`) triumphiert ausnahmslos über historische `as_of`-Abfragen.
2. **Snapshot-Widerruf (`SnapshotRevoker`):**  
   Implementiert den `SnapshotRevoker`-Port aus `acta-ports`. `revoke_covering(gated_seq, target)` entwertet betroffene offene Snapshots unverzüglich.
3. **SSI-Transaktionsabbruch:**  
   Wird ein Datensatz während einer offenen Transaktion im Gate gesperrt, scheitert der nachfolgende Commit hart mit `Err(MvccError::Erased)`.
