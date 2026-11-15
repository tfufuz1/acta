# acta-ids — Crate-Spezifikation (Fassung 3.0 Final)

*Bezug: Normative Gesamtspezifikation [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) (Teil III, IV, VII)*  
*Ring: R0 (Fundament) · Geltung: Verbindlich für `acta-ids`*

---

## 1. Zweck und Einordnung

`acta-ids` ist ein autarker, I/O-freier Basis-Baustein in **Ring R0 (Fundament)**. Er definiert sämtliche typisierten Bezeichner, Mandanten-Tokens, kryptografischen Tags und monotonen Ordinale für die gesamte Acta-Architektur:
- **Typisierte IDs:** `TenantId`, `CollectionId`, `DocId`, `SubjectId`, `EntityId`, `TxId`, `EpochId`.
- **Dichte Ordinale (F-3):** `DocOrd` (`u32`), monoton aufsteigend je Mandant, Grundlage für die $O(1)$-Lineage via `RoaringBitmap`.
- **Kryptografische Tags (KeyV2):** `TenantTag`, `CollectionTag`, `ClusterTag` (definiert als `SubjectTag`, Fallback `DocTag`), `DocTag` (16 Byte BLAKE3-keyed Hashes).
- **Mandanten-Isolation:** `TenantScoped<T>`, `TenantScopeViolation`.

---

## 2. Architektur und Verträge

| Merkmal | Vorgabe (docs/SPEC.md §3.1) |
|---|---|
| **Ring** | **R0 (Fundament)** |
| **Interne Abhängigkeiten** | **0 (keine)** |
| **Externe Abhängigkeiten** | `blake3`, `serde`, `byteorder`, `thiserror` |
| **Default-Features** | `["std"]` |
| **Fehlertyp** | `IdError` |
| **Unsafe-Policy** | `#![forbid(unsafe_code)]` |
| **Zero-Panic** | `#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]` |

---

## 3. Normative Anforderungen

1. **Keine PII in Identifikatoren:**  
   Keine Struktur in `acta-ids` darf Klarnamen, E-Mail-Adressen oder sonstige personenbezogene Daten (PII) im Klartext aufnehmen. Rohidentifikatoren werden über epochenweises $K_{\text{scope}}$ gehasht:
   $$\text{Tag} = \text{BLAKE3-keyed}(K_{\text{scope}}, \text{label} \parallel \text{id})[0..16]$$
2. **Cluster-Definition (F-4):**  
   `ClusterTag` repräsentiert das `cluster`-Feld (16 Byte) im KeyV2-Präfix. Es MUSS zwingend dem `SubjectTag` des Betroffenen entsprechen (oder `DocTag` bei Dokumenten ohne Personenbezug), um physische Lokalität im LSM-Tree sicherzustellen.
3. **Monotone Ordinale (`DocOrd`):**  
   `DocOrd` ist ein `u32`-Newtype. Es wird niemals im laufenden System wiederverwendet und verschwindet nach Löschung mit dem D-Slot aus Ebene P. Es erscheint niemals in Berichten oder der Chronik (Ebene N).
4. **Keine Fallbacks / Kein Default (S7):**  
   Sicherheitsrelevante IDs (`TenantId`, `SubjectId`, `DocId`, `EpochId`) implementieren **kein** `Default`.

---

## 4. Tests und Abnahme

- Property-Tests: Roundtrip-Serialisierung und Kollisionsfreiheit für Tags.
- `cargo check --no-default-features` und `cargo test`.
- Architektur-Gate `G1`: Keine internen Workspace-Abhängigkeiten.
