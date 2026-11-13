# acta-text

> Ring 0 / Klasse A · Reifegrad R3 · Ebene E1 · `#![forbid(unsafe_code)]`

`acta-text` ist ein autarker Klasse-A-Baustein des Acta-Ökosystems. Er implementiert eine performante Inverted-Index-Engine mit BM25-Scoring, Copy-on-Write Snapshot-Isolation, Block-Max WAND dynamischem Pruning und konfigurierbarer Textanalyse inklusive deutscher Morphologie (Komposita-Zerlegung, Umlaut-Normalisierung, Stoppwort-Filterung).

## Zweck & Abgrenzung
- **Kernaufgabe:** Autarker Volltextindex, BM25 / BM25F Scoring, Block-Max WAND Pruning, Tokenizer und morphologische Zerlegung.
- **Abgrenzung:** Enthält keine I/O- oder Storage-Bindung im Kern; Speicherzugriffe erfolgen über das lokale Minimal-Trait `PostingStore` (`contract.rs` / `storage.rs`). Referenzimplementierung im RAM ist `MemoryTextIndex`.

## Features
| Feature | Beschreibung |
|---|---|
| `std` | Standardbibliothek (Default). |
| `workspace` | Transitional-Feature für Ports-/Types-Kompatibilität (`src/compat/`). |
| `types-compat` | Re-Export von Typenkonvertierungen zu `acta-types`. |
| `ports-compat` | Implementierung von `StorageEngine -> PostingStore` und `TextIndex`. |
| `bm25f` | Feldgewichtetes BM25F Scoring. |
| `docid-128` | 128-Bit Dokument-IDs via `acta-ids`. |

## Tatsächliche interne Abhängigkeiten (autarker Kern)
Erhoben via `cargo tree -p acta-text -e normal --no-default-features`:
- `acta-ids`
- `acta-env`

## Schnellstart

```rust
use acta_text::contract::DocKey;
use acta_text::memory_index::MemoryTextIndex;
use acta_text::traits::Index;

fn main() {
    let mut index = MemoryTextIndex::new();
    let doc1 = DocKey::from(1u64);
    let doc2 = DocKey::from(2u64);

    Index::<DocKey, str>::insert(&mut index, doc1, b"Donaudampfschifffahrt und Volltextsuche").unwrap();
    Index::<DocKey, str>::insert(&mut index, doc2, b"Volltextsuche in Rust mit BM25").unwrap();

    let hits = Index::<DocKey, str>::search(&index, "Volltextsuche", 5).unwrap();
    assert_eq!(hits.len(), 2);
}
```

## Autarkes Bauen & Testen
```bash
cargo build -p acta-text --no-default-features --locked
cargo test  -p acta-text --no-default-features --locked
```
