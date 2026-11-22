# acta

Neubau des Storage-Kerns `contextra-storage`: ein eingebetteter, synchroner, crash-sicherer Key-Value-Store in Rust (kein `unsafe`, nur `std` im Kern). Gedacht als Fundament für RAG- und Agenten-Gedächtnisschichten.

## Status

Entwurf im Umbau. Formate und Bausteine (Codec, Run, Redo-Format, Catalog, Blob, Keys) existieren und sind getestet. Die Verdrahtung zu einem durablen Gesamtsystem fehlt: Der Redo-Pfad schreibt noch nur einen Stamp, Recovery, Flush und Compaction sind nicht im Modulbaum, eine öffentliche Facade gibt es nicht. Ist-Stand und Befunde: `IMPLEMENTATION_SPEC.md` §1 und §4. **Nicht für den produktiven Einsatz.**

## Eigenschaften (Ziel laut SPEC)

- Atomare Änderungen über mehrere Räume, Compare-and-Swap über `Guard`
- MVCC-Views, Holds, Consumer
- Kontrollierte physische Löschung (`Erase`)
- Wertetrennung: Header im LSM, Körper in Blob-Dateien
- Budgetierte Ergebnislieferung (Gewichte, Token-Budget), aber ohne Tokenizer und Embeddings

## Schnellstart

```bash
make check                  # kompilieren
make test-target T=redo     # ein Test-Target
make clippy
make metrics
```

`make` setzt `RUSTUP_TOOLCHAIN=stable`. Hintergrund: `docs/dev/environment.md`. Alle Ziele: `CLAUDE.md` §3.

## Dokumente

| Datei | Inhalt |
|---|---|
| `SPEC.md` | normative Spezifikation (R2) |
| `IMPLEMENTATION_SPEC.md` | Umsetzungsplan, Ist-Stand, Gates (Rev. 3) |
| `ARCHITECTURE.md` | Crates, Module mit Status, Pfade, Sperrränge, Kennungen |
| `CLAUDE.md` | Einstieg für Claude-Sitzungen und Stolperfallen |
| `CONTRIBUTING.md` | Beitragsregeln, Branches, Definition of Done |
| `docs/dev/` | Umgebung, Git-Workflow, offene Punkte |
| `REPORT.md` | Messbericht, **nicht reproduziert** |
| `legacy/` | eingefrorener Altbestand, siehe `legacy/README.md` |

## Layout

```
crates/contextra-storage-ports   Ports und Werttypen
crates/contextra-storage         Kern
crates/contextra-crypto-ref      Referenz-Krypto
fuzz/                            libFuzzer-Targets (eigenes Cargo-Projekt)
tools/reference/                 Python-Referenzcodierer, Golden-Vektoren
scripts/                         Messskripte
legacy/                          Altbestand, nicht im Workspace
```

## Lizenz

Siehe `Cargo.toml` (`license`). Erlaubte Abhängigkeitslizenzen: `deny.toml`.
