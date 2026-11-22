# fuzz

Eigenständiges Cargo-Projekt (`acta-fuzz`, edition 2021, eigenes `[workspace]`, **nicht** Mitglied des Root-Workspace). Edition 2021 ist bewusst wegen cargo-fuzz, der Workspace nutzt 2024. Befehle immer im Verzeichnis `fuzz/` ausführen.

## Targets

| Target | Gegenstand |
|---|---|
| `run_block` | Run-Datenblock (`DataBlockReader`) |
| `run_footer` | Run-Footer |
| `run_directory` | Run-Directory |

Soll laut IMPL: mindestens 8 Ziele (Metrik M-15). Aktuell 3.

## Ausführen

```bash
cargo install cargo-fuzz          # einmalig
cd fuzz
cargo +nightly fuzz run run_block -- -max_total_time=60   # Smoke, 60 s
```

Voraussetzungen: Nightly-Toolchain und `cargo-fuzz`. **In der Claude-VM sind beide nicht installiert**, und `static.rust-lang.org` ist gesperrt (`docs/dev/environment.md`). Fuzzing ist dort bis zu einer Freigabe nicht möglich. Das in Berichten ausdrücklich vermerken, nicht als „keine Funde“ melden.

Corpus und Crashes (`fuzz/corpus`, `fuzz/artifacts`) sind per `.gitignore` ausgeschlossen. Fund reproduzieren, minimieren und als Regressionstest unter `crates/contextra-storage/tests/` ablegen.

## Hinweis

`legacy/contextra-store/fuzz/run_smoke.sh` ist veraltet und nicht zu verwenden.
