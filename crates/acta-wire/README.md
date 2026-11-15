# acta-wire

Zero-Copy-Wire-Formate, FlatBuffers-Adapter und Frame-Parsing für Acta.

## Rolle und Zweck
`acta-wire` ist ein Ring-0-Fundamentbaustein:
- **`WireBuffer`:** Typgesicherte Pufferverifikation (`Unverified` $\rightarrow$ `Verified`).
- **Deterministische Serialisierung:** Striktes Big-Endian-Encoding.
- **Torn-Tail-Resilienz:** Sichere Erkennung unvollständiger Frame-Enden.
