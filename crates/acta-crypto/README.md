# acta-crypto

Kryptografische Primitive, AEAD-Datensatzumschlag (XChaCha20-Poly1305), hierarchische DEK-Ableitung und Zeroize-Schlüsselcontainer für Acta.

## Rolle und Zweck
`acta-crypto` ist ein autarker Ring-0-Fundamentbaustein (Ebene 1):
- **Datensatz-AEAD (Umschlag v3):** XChaCha20-Poly1305 mit intern generierter 192-Bit-Zufallsnonce und vollständiger AAD-Bindung.
- **Hierarchische DEK-Ableitung:** HKDF-SHA256 über Ancestor-Salts (Mandant, Collection, Subject, Dokument).
- **WAL- und Ledger-Integrität:** HMAC-SHA256 für Hash-Ketten und pseudonymitätsfreie Commitments ($K_{\text{commit}}[e]$).
- **Speicherhygiene:** `ZeroizeOnDrop` für alle Schlüsselstrukturen; keine Leaks in `Debug` oder `Display`.

## Abgrenzung (Nicht-Ziele)
- **Keine I/O- oder Netzwerkoperationen:** Reine In-Memory-Kryptografie.
- **Keine Rechtsaussagen:** Das Crate implementiert mathematische Algorithmen; es enthält keine Werbeaussagen (Gate `G-WORD`).

## Interne Abhängigkeiten
- `acta-ids`: Typisierte Bezeichner (`TenantId`, `DocId`, `DocTag`, `DocOrd`)
- `acta-env`: Zufalls- und Zeit-Ports (`Rng`, `Clock`)

## Schnellstart

```rust
use acta_crypto::aead::{seal_record, open_record, SealCtx};
use acta_crypto::dek::Dek;

fn example(ctx: &SealCtx, dek: &Dek, plaintext: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let envelope = seal_record(ctx, dek, plaintext)?;
    let opened = open_record(ctx, dek, &envelope)?;
    assert_eq!(opened.as_slice(), plaintext);
    Ok(())
}
```
