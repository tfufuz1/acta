# acta-ports — Crate-Spezifikation (Fassung 3.0 Final)

*Bezug: Normative Gesamtspezifikation [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) (Teil IV, V, VI, VII, VIII)*  
*Ring: R2 (Systemverträge) · Geltung: Verbindlich für `acta-ports`*

---

## 1. Zweck und Einordnung

`acta-ports` definiert die zentralen Trait-Verträge und Schnittstellen zur Entkopplung aller Subsysteme (Persistenz, Indizes, Löschung, Krypto, Umwelt):
- **Lösch- und Schichtverträge:** `ErasureLayer` mit `resolve()`, `on_gate()`, `purge()`, `witness()`.
- **Lesesperre & MVCC-Sicht:** `ReadGate`, `ErasedView` ($O(1)$ Prüfung via `doc_ord`).
- **Nebenläufigkeits- und Barrier-Ports:** `CommitBarrier`, `PurgeBarrier`, `FenceGate`, `WalSink`.
- **Snapshot-Widerruf:** `SnapshotRevoker` (`revoke_covering`).
- **Hardware-Sicherheit:** `KeyAnchor` Trait zur Verwahrung und Vernichtung von Epochenschlüsseln ($E_e$) im HSM/KMS.
- **Forensik & Notariat:** `ResidueScanner`, `ReportSigner`, `Notary`.

---

## 2. Architektur und Verträge (Regel R-PORT)

| Merkmal | Vorgabe (docs/SPEC.md §3.1, §4.3) |
|---|---|
| **Ring** | **R2 (Systemverträge)** |
| **Erlaubte interne Abhängigkeiten** | `acta-types`, `acta-ids`, `acta-env` |
| **Regel R-PORT** | Jeder Port besitzt genau eine Produktionsimplementierung und eine Aufrufstelle (Gate G2) |
| **Dyn-Kompatibilität** | Alle Ports sind dyn-safe (`Send + Sync + 'static`) |
| **Unsafe-Policy** | `#![forbid(unsafe_code)]` |

---

## 3. Normative Anforderungen

1. **Gate G2 Durchsetzung (Dead-Port-Verbot):**  
   Jeder in `acta-ports` definierte Trait MUSS eine aktive Produktionsimplementierung besitzen und im System aufgerufen werden. Toter Code ist verboten.
2. **ReadGate & ErasedView (F-8):**  
   Der Store erhält zur Bereinigung historischer Versionen ausschließlich Zugriff auf den lesenden Port `&dyn ErasedView`. Er kennt keine Lösch-Interna.
3. **Schicht-Quittung (`on_gate`):**  
   Der `ErasureLayer`-Port verlangt eine synchrone Quittierung von `on_gate`, die bestätigt, dass flüchtige Klartexte im RAM und Caches genullt wurden.
4. **Auflösung doppelter Traits:**  
   Historische Traits aus `acta-erasure::types_local` sind vollständig in `acta-ports` integriert.
