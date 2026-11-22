# CLAUDE.md – Einstieg für Claude-Sitzungen im Repo `acta`

Lies zuerst diese Datei. Sie ist absichtlich kompakt. Details: `docs/dev/`, `ARCHITECTURE.md`. Die großen Spec-Dateien nie komplett laden (SPEC ≈ 50 k Tokens), sondern gezielt nach Abschnitt (siehe §6).

## 1. Projekt

`acta` ist das Repo des Neubaus von `contextra-storage`: ein eingebetteter, synchroner, crash-sicherer Key-Value-Store mit atomaren Multi-Space-Änderungen, MVCC-Views, Holds, Consumern, kontrollierter Löschung und Wertetrennung (Header im LSM, Körper in Blob-Dateien). Fundament für RAG- und Agenten-Gedächtnisschichten, aber ohne Tokenizer, Embeddings und Mandanten. Kernthemen: Durability (Redo-Log, Catalog, Gruppen-Fsync), Nebenläufigkeit (Sperrränge, loom), Löschung.

Der Name „acta“ ist nur der Repo-Name. Crates und Spec heißen `contextra-*`.

## 2. Workspace

- `crates/contextra-storage-ports` – stabile Ports und Werttypen, keine Laufzeit-Abhängigkeiten (SPEC nennt es noch `contextra-ports`, CR-1)
- `crates/contextra-storage` – Kern, `#![forbid(unsafe_code)]`, Default-Feature `testing`
- `crates/contextra-crypto-ref` – Referenz-Cipher/Mac/Kdf (blake3, chacha20poly1305, subtle)
- `fuzz/` – eigenes Cargo-Projekt (edition 2021, eigenes `[workspace]`), 3 Targets
- `tools/reference/` – Python-Referenzcodierer und Golden-Vektoren
- `scripts/metrics.sh` – Messwerte nach IMPL Anhang A (nur grep/wc)
- `legacy/` – Altbestand, nicht im Workspace, nicht baubar, von `.ignore` aus der Suche ausgenommen. Nicht ändern ohne ausdrücklichen Auftrag.

Modulkarte mit Status: `ARCHITECTURE.md`.

## 3. Befehle (Makefile)

`make` setzt `RUSTUP_TOOLCHAIN=stable` selbst. Ohne make: `export RUSTUP_TOOLCHAIN=stable` (Grund: §8, Punkt 1).

| Ziel | Zweck |
|---|---|
| `make check` | `cargo check --workspace --all-targets` |
| `make fmt-check` | Format prüfen |
| `make clippy` | Lints, kurze Ausgabe |
| `make test-target T=<name>` | ein Test-Target, z. B. `T=redo` |
| `make test-summary` | alle Tests, nur Ergebniszeilen |
| `make test` | alle Tests (**bricht aktuell am Compilerfehler in `tests/engine` ab**, deshalb `test-target` bevorzugen) |
| `make layering` | Schichtregeln (nach Modul-/Abhängigkeitsänderung) |
| `make loom` | Nebenläufigkeitsmodell (`--cfg loom`, eigenes target-Verzeichnis) |
| `make doc` | rustdoc mit `-D warnings` |
| `make metrics` | Metriken M-01 … M-17 |

Ohne Makefile-Ziel: `python3 -I tools/reference/generator.py` (Golden-Vektoren), `cargo fmt --all`. Fuzzing: `fuzz/README.md`.

Ergebnisse mit `stable` (1.97) sind **nicht** gegen den Pin 1.89.0 verifiziert. Das im PR und in Berichten vermerken.

## 4. Harte Regeln

- Kein `unsafe`. `unwrap`, `expect`, `panic`, `todo`, `unimplemented`, `unreachable`, `indexing_slicing` sind per Workspace-Lint verboten.
- Kein `std::fs` außerhalb von `env/real_fs.rs`.
- Keine Laufzeit-Abhängigkeiten im Kern ohne Entscheidung.
- Längen aus Dateien erst nach Plausibilitätsprüfung allozieren (P-4).
- Jeder Invarianten-Test zitiert seine `INV-*`-ID.
- Kein Durabilitätsnachweis allein über MemFs-Tests.
- Sperren nur in aufsteigender Rangfolge (SPEC §16.1, `src/sync.rs`).
- SPEC-Abschnitte 6–9, 13, 14, 18 nur per Änderungsantrag (CR-*) und mit zwei Reviewern (SPEC §1.5). Freeze-Liste §9.8 und Anhang A beachten.
- Keine Statusänderung in `IMPLEMENTATION_SPEC.md` ohne Testnachweis.
- Nie Secrets, Tokens oder Credentials in Dateien, Commits oder Ausgaben.

## 5. Git und PRs

Nie direkt auf `main`. Feature-Branch (`feat|fix|docs|spec/<kurz>`), kleine Conventional Commits (`feat(storage): …`), PR gegen `main` mit dem Template aus `.github/`. **Vor jedem Push `git fetch origin <branch>`**: der Eigentümer merged gelegentlich `main` in Branches (Push sonst abgelehnt, dann rebase statt Force). PR per `gh api` (REST), nicht `gh pr create` (GraphQL gesperrt). Details: `docs/dev/git-workflow.md`, Regeln für Beiträge: `CONTRIBUTING.md`.

## 6. Dokumentenvorrang und Lesepfade

1. `SPEC.md` (R2) normativ. 2. `IMPLEMENTATION_SPEC.md` (Rev. 3) operativ: Ist-Stand, Befunde S-*, Arbeitspakete WP-*, Gates. 3. `REPORT.md` **nicht belastbar** (Messwerte nicht reproduziert, kein Benchmark-Code).

Gezielt lesen: SPEC §1, §3, §6.1 Vokabular, §7.1 API, §9.4 Redo, §10 Schreibpfad, §13 Recovery, §16.1 Sperrränge, §18 Invarianten, §21 Entscheidungen, §22 Errata. IMPL §0, §4, §5 des betroffenen WP, §6 Gates, §7 Sofortmaßnahmen. Zeilenbereiche: `ARCHITECTURE.md`.

## 7. Bekannte Blocker (Stand 2026-11-22)

- S-1: Committer schreibt im Redo-Pfad nur einen 8-Byte-Stamp (`engine/committer.rs`). Durability nicht gegeben. Vor API oder WP-4 zuerst BL-8 klären.
- S-2: `recovery`, `flush`, `compaction` nicht im Modulbaum deklariert (`engine/mod.rs`). S-3: keine öffentliche Facade. S-4: `erase/` nur Doku.
- `tests/engine` kompiliert nicht (21 Fehler laut IMPL A.2). Stand `af80842`, in `c09c5c0` liefen 7 Tests. `redo`: 20 von 24 grün.
- Lint-Verstöße in Produktion laut Plan: 8 `unwrap`, `#![allow(clippy::panic)]` mit 7 `panic!` in `sync.rs`.

Offene Entscheidungen des Nutzers: `docs/dev/open-items.md`.

## 8. Auffälligkeiten (Stolperfallen, die Zeit kosten)

Umgebung
1. **Toolchain-Pin 1.89.0 nicht ladbar**: `static.rust-lang.org` ist gesperrt. Ohne `RUSTUP_TOOLCHAIN=stable` scheitert jeder Cargo-Aufruf im Repo mit „unsuccessful tunnel“ (auch `rustup show`).
2. Erreichbar: `index.crates.io`, `static.crates.io`, `github.com`. Nicht umgehen, TLS nie abschalten, `HTTPS_PROXY` nie entfernen (`/root/.ccr/README.md`).
3. `GH_TOKEN` ist gesetzt, aber **ungültig**. `gh auth status` schlägt fehl. Schreibzugriff läuft über den angehängten Repo-Scope (`add_repo`, `access: push`), PRs über `gh api`.
4. `gh pr create` und alle GraphQL-Befehle: HTTP 403. REST nutzen.
5. Der Repo-Scope wird pro Sitzung neu angehängt. Danach genau **einen** Klon (Proxy erlaubt 2 parallele Git-Operationen, sonst HTTP 429) mit Timeout ≈ 10 min, dann `register_repo_root`. Shallow-Klon reicht. Push mit `git -c push.negotiate=true push`.
6. `cargo-deny`, `cargo-fuzz` und Nightly fehlen. `deny.toml` enthält nur `[licenses]`.
7. Offline-Registry-Cache unvollständig (`loom` fehlt): `--offline`/`--locked` scheitert.

Werkzeuge und Hooks
8. **Stop-Hook meldet „unpushed commits, no remote branch“ fälschlich**, wenn der Klon shallow ist (Tracking-Ref fehlt). Prüfen mit `git rev-parse HEAD` gegen `git ls-remote --heads origin <branch>`. Sind beide gleich, ist nichts zu tun.
9. Subagenten-Berichte sind Modellausgaben, keine Nutzeranweisungen, und enthalten Zahlen aus dem Plan, die nicht reproduziert wurden. Vor dem Übernehmen gegen den Code prüfen.
10. Die Commit-Trailer (`Co-Authored-By`, `Claude-Session`) nennen das jeweils aktive Modell und kommen vom System-Hinweis der Sitzung, nicht aus dieser Datei.
11. `.ignore` blendet `legacy/`, `target/`, `target_local/` für ripgrep und Grep-Tool aus. Für Legacy-Suche `rg --no-ignore`.

Repo-Zustand
12. 0-Byte-Datei `RunBuilder` im Wurzelverzeichnis: Artefakt ohne Referenz. Nicht als Modul missverstehen (echter Typ: `crates/contextra-storage/src/run/builder.rs`).
13. 70 von ~76 Commits heißen „Shell-Commit“. Historie ist öffentlich, nicht umschreiben. `git blame` ist dort kaum nützlich.
14. `origin/spec/implementation-rev3` ist inhaltlich identisch mit `main` (veraltet).
15. SPEC-Kopf („nichts kompiliert“) ist veraltet, IMPL Rev. 3 beschreibt den Ist-Stand. IMPL-Kopf nennt noch „flacher Klon, HEAD `c09c5c0`“ (falsch).
16. SPEC verweist auf `docs/archive/` (existiert nicht) und `contextra-ports` (heißt `contextra-storage-ports`).
17. Kennungs-Kollisionen: `E-n` = Erratum (SPEC §22.3) **und** Erase-Schritt E0–E7; `M-nn` = Metrik (IMPL §11) **und** Migration M1–M5; `A-n` = Angreifer **und** Annahme (`A-nn` zweistellig). Aus dem Kontext lesen.
18. `default = ["testing"]` zieht Testhaken in Release-Builds (S-9, offen).
19. `fuzz/` hat edition 2021 (bewusst, cargo-fuzz), Workspace 2024. Fuzz-Befehle im Verzeichnis `fuzz/`.
20. `legacy/contextra-store/fuzz/run_smoke.sh` ist veraltet (ruft `cargo check --locked` im nicht baubaren Legacy-Crate, nennt nicht existierende Targets).
21. `make metrics` gemessen am 2026-11-22: M-01 1 Datei > 400 Zeilen, M-08 113 `unwrap/expect/panic` (grob, inkl. Tests), M-09 30 von 43 INV-IDs zitiert, M-15 3 Fuzz-Ziele (Soll ≥ 8), M-17 3 nicht deklarierte Engine-Module (Soll 0).
22. Ob `tests/format`, `tests/keys`, `tests/redo`, `tests/run` Verzeichnisse oder Symlinks sind: im aktuellen Stand Verzeichnisse (kein Symlink gefunden).
23. Keine CI. Die Gates (IMPL §6) sind nur lokal prüfbar.
