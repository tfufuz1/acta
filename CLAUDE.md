# CLAUDE.md – Einstieg für Claude-Sitzungen im Repo `acta`

Lies zuerst diese Datei. Details stehen in `docs/dev/`. Lade die großen Spec-Dateien nicht vollständig, sondern gezielt nach Abschnitt.

## Projekt in 3 Sätzen

`acta` ist das Repo des Neubaus von `contextra-storage`: ein eingebetteter, synchroner, crash-sicherer Key-Value-Store mit atomaren Multi-Space-Änderungen, MVCC-Views, Holds, Consumern, kontrollierter Löschung und Wertetrennung (Header im LSM, Körper in Blob-Dateien). Er ist Fundament für RAG- und Agenten-Gedächtnisschichten, enthält aber bewusst keinen Tokenizer, keine Embeddings und keine Mandanten. Durability (Redo-Log, Catalog, Gruppen-Fsync), Nebenläufigkeit und Löschung sind die Kernthemen.

## Workspace

- `crates/contextra-storage-ports` – stabile Ports und Werttypen, keine Laufzeit-Abhängigkeiten
- `crates/contextra-storage` – Kern (Engine, Run, Redo, Catalog, Blob, Erase, Keys, Codec), `#![forbid(unsafe_code)]`
- `crates/contextra-crypto-ref` – Referenzimplementierung Cipher/Mac/Kdf
- `fuzz/` – eigenständiges Cargo-Projekt (nicht Workspace-Mitglied), Targets `run_block`, `run_footer`, `run_directory`
- `tools/reference/` – Python-Referenzcodierer `generator.py` und Golden-Vektoren `golden/*.hex`
- `legacy/` – Altbestand, nicht im Workspace, nicht baubar. Nicht ändern, außer die Aufgabe verlangt es ausdrücklich.

## Dokumentenvorrang

1. `SPEC.md` (R2) ist normativ und maßgeblich.
2. `IMPLEMENTATION_SPEC.md` (Rev. 3) ist operativ: Ist-Stand, Befunde, Arbeitspakete, Gates.
3. `REPORT.md` ist **keine verlässliche Messquelle** (siehe `docs/dev/open-items.md`).
4. Änderungen an der SPEC laufen als Anträge (IMPL §10), nicht still.

Gezielte Lesepfade: SPEC §1, §3, §6.1 (Vokabular), §7.1 (API), §9.4 (Redo), §10 (Schreibpfad), §13 (Recovery), §16.1 (Sperrränge), §18 (Invarianten); IMPL §0, §4, §5 des betroffenen Arbeitspakets.

## Befehle

Voraussetzung und Umgebung: siehe `docs/dev/environment.md`.

```bash
export RUSTUP_TOOLCHAIN=stable   # nur nötig, wenn der Pin 1.89.0 nicht ladbar ist
cargo check --workspace --all-targets
cargo test --workspace --no-fail-fast
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
cargo doc -p contextra-storage --no-deps
python3 -I tools/reference/generator.py   # vor Änderungen an Golden-Dateien
```

Ergebnisse mit `RUSTUP_TOOLCHAIN=stable` sind **nicht** gegen 1.89.0 verifiziert. Das ist in den Ergebnissen zu vermerken.

## Harte Regeln

- Kein `unsafe`. `unwrap`, `expect`, `panic`, `todo`, `unreachable`, `indexing_slicing` sind per Workspace-Lint verboten.
- Kein `std::fs` außerhalb von `env/real_fs.rs`.
- Keine Laufzeit-Abhängigkeiten im Kern-Crate ohne Entscheidung.
- Längen aus Dateien erst nach Plausibilitätsprüfung allozieren (P-4).
- Jeder Invarianten-Test zitiert seine `INV-*`-ID.
- Auf MemFs-Tests allein darf kein Durabilitätsnachweis gestützt werden.
- Nie Secrets, Tokens oder Credentials in Dateien, Commits oder Ausgaben schreiben.

## Git und PRs

Nie direkt auf `main` arbeiten. Feature-Branch, kleine thematische Commits, PR gegen `main`. PR per `gh api` (REST), nicht `gh pr create` (GraphQL gesperrt). Details: `docs/dev/git-workflow.md`.

## Bekannte Blocker (Stand 2026-11-22)

- Redo-Pfad nicht verdrahtet (IMPL S-1): Der Committer schreibt nur einen Stamp. Vor Arbeiten an API oder WP-4 zuerst BL-8 klären.
- Recovery, Flush und Compaction nicht im Modulbaum deklariert (S-2).
- Keine öffentliche Facade (S-3).
- Lint-Verstöße in Produktionscode laut Plan: 8 `unwrap`, `#![allow(clippy::panic)]` in `sync.rs`.

Vollständige Liste und offene Entscheidungen: `docs/dev/open-items.md`.
