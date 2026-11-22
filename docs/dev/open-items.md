# Offene Punkte und Entscheidungen

Stand: 2026-11-22. Diese Liste sammelt, was in der Inspektion aufgefallen ist und **nicht eigenständig geändert wurde**. Jede Zeile braucht eine Entscheidung des Nutzers oder einen eigenen Auftrag.

## Entscheidungen des Nutzers

1. **Toolchain-Pin 1.89.0 (BL-7):** Pin auf eine in der VM ladbare Version setzen, oder `static.rust-lang.org` freigeben lassen, oder den Pin dokumentiert lassen? Solange offen, sind clippy-Ergebnisse nicht verbindlich.
2. **`GH_TOKEN` ungültig:** Soll ein neues Token gesetzt werden? Wird Push-Zugriff für PRs gebraucht?
3. **`cargo-fuzz` in der VM:** Soll Nightly und cargo-fuzz bereitgestellt werden?
4. **Leere Datei `RunBuilder`** (0 Byte, Wurzelverzeichnis): ohne Referenz im Build, vermutlich Artefakt. Löschen?
5. **Veralteter Branch `spec/implementation-rev3`:** löschen?
6. **`REPORT.md`:** Die Messwerte (p99 18,9 µs, 100 % Filterwirkung) sind nicht reproduziert, es gibt keinen Benchmark-Code. Als „nicht reproduziert“ kennzeichnen oder entfernen?
7. **Spec-Stand:** SPEC (R2) sagt „nichts kompiliert oder gemessen“, IMPL Rev. 3 beschreibt Kompilierbarkeit. Änderungsanträge (IMPL §10, CR-1 bis CR-12) einarbeiten oder verwerfen?
8. **Kennungs-Kollisionen** (`E-n` für Errata und Erase-Schritte, `M-nn` für Metriken und Migration, `A-n` für Angreifer und Annahmen) bereinigen?
9. **`legacy/`:** Mit README als „nicht Workspace, nicht baubar“ kennzeichnen oder in ein Archiv verschieben?
10. **`default = ["testing"]`** in `contextra-storage` (S-9): entfernen?
11. **`docs/archive/`** wird in der SPEC genannt, existiert aber nicht. Anlegen oder SPEC korrigieren?

## Technische Schulden (mit Plan in IMPL)

- Redo-Pfad nicht verdrahtet (S-1), Recovery/Flush/Compaction nicht deklariert (S-2), keine Facade (S-3), `erase/` leer (S-4).
- Lint-Verstöße: 8 `unwrap` in `engine/committer.rs` und `sync.rs`, `#![allow(clippy::panic)]` mit 7 `panic!` in `sync.rs` (S-8).
- `deny.toml` hat nur `[licenses]`. Fehlend: `[advisories]`, `[bans]`, `[sources]` (IMPL 0.2).
- Fuzz-Projekt nutzt edition 2021, Workspace 2024. Bewusst, aber nicht dokumentiert.
- Keine CI (BL-9). `.github/` fehlt. Gates sind damit nicht erzwungen.

## Dokumente

Angelegt: `README.md`, `ARCHITECTURE.md`, `CONTRIBUTING.md`, `fuzz/README.md`, `crates/contextra-storage/README.md`, `legacy/README.md`. PR-Template, `Makefile`, `scripts/metrics.sh` und `.ignore` stammen vom Eigentümer (PR #2).

Noch offen:

- `CODEOWNERS` (Eigentümer nennen)
- CI unter `.github/workflows/` (BL-9)
- ADRs für SD-01 bis SD-64 (bisher nur Tabelle in SPEC §21)
- `CHANGELOG.md` (erst bei Releases)
