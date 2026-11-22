# Beitragsregeln

Gilt für Menschen und Agenten. Kurzfassung für Agenten: `CLAUDE.md`.

## Vor dem Start

1. `CLAUDE.md` und `ARCHITECTURE.md` lesen. Betroffenes Arbeitspaket in `IMPLEMENTATION_SPEC.md` §5 und Befunde in §4 prüfen.
2. Blocker beachten (`CLAUDE.md` §7): Vor Arbeiten an API oder Engine zuerst S-1/BL-8 klären.
3. Umgebung prüfen: `docs/dev/environment.md`.

## Branches und Commits

- Nie direkt auf `main`.
- Branch: `feat/<kurz>`, `fix/<kurz>`, `docs/<kurz>`, `spec/<kurz>`.
- Conventional Commits wie `feat(storage): …`, `fix(redo): …`, `docs: …`. Befund-IDs nennen (`S-1`, `WP-4`). Keine Sammel-Commits.
- Ein Commit, ein Thema. Keine Format-Änderungen quer durch fremden Code.
- Vor jedem Push `git fetch origin <branch>`, bei Abweichung rebase, nie Force-Push auf geteilte Branches.

## Definition of Done

- [ ] `make check` grün
- [ ] `make test-target T=<betroffen>` grün (volles `make test` erst, wenn `tests/engine` kompiliert)
- [ ] `make clippy`: keine neuen Verstöße gegen `unwrap`, `expect`, `panic`
- [ ] `make fmt-check` grün
- [ ] `make layering` grün, falls Module oder Abhängigkeiten geändert wurden
- [ ] `make loom` grün, falls Sperren oder Nebenläufigkeit betroffen sind
- [ ] `make metrics` aktualisiert, falls betroffen
- [ ] Betroffene `INV-*` im Testnamen oder Kommentar zitiert
- [ ] Format betroffen: Freeze-Liste (SPEC §9.8) und Referenzvektor (Anhang A) geprüft, Goldens per `tools/reference/generator.py`
- [ ] Statusänderungen in `IMPLEMENTATION_SPEC.md` nur mit Testnachweis
- [ ] Verwendete Toolchain im PR genannt (`stable` statt Pin 1.89.0 ist nicht verifiziert)

## Spezifikation ändern

SPEC-Abschnitte 6–9, 13, 14, 18 nur per Änderungsantrag (`CR-*`, IMPL §10) mit zwei Reviewern (SPEC §1.5). Keine stillen Änderungen. Entscheidungen als `SD-nn` in SPEC §21 festhalten.

## Pull Requests

PR gegen `main` mit dem Template in `.github/pull_request_template.md`. Kleine PRs. Keine Credentials, Tokens oder Secrets in Dateien, Commits oder PR-Texten. PR-Erstellung per Sitzung: `docs/dev/git-workflow.md`.

## Code-Regeln

Siehe `CLAUDE.md` §4. Kein `unsafe`, kein `std::fs` außerhalb `env/real_fs.rs`, Längen aus Dateien erst prüfen, dann allozieren, Sperren nur in Rangfolge, neue Abhängigkeiten im Kern nur per Entscheidung und mit Lizenz aus `deny.toml`.
