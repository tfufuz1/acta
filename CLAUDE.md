# CLAUDE.md — Einstieg für Agenten im Repo `acta`

Lies zuerst diese Datei. Sie ersetzt keine Spezifikation, sie zeigt dir, wo was steht und welche Fallen es gibt.

## 1. Dokumenthierarchie

1. `SPEC.md`: normativ (Formate, Protokolle, Invarianten INV-*, Fehlerklassen F-*, Entscheidungen SD-*). Bei Widerspruch gilt sie.
2. `IMPLEMENTATION_SPEC.md`: operativ (Status je Arbeitspaket, Blocker BL-*, kritische Befunde S-*, Gates). Der Abschnitt „Kritische Befunde“ (§4) und „Restumfang“ (§5) sind die Arbeitsliste.
3. `REPORT.md`: **keine Messquelle**. Die Werte sind nicht reproduziert (IMPL S-10).
4. `legacy/`: **keine Quelle für Anforderungen** (SPEC §1.3). Nicht im Workspace-Build. Nicht durchsuchen, außer du arbeitest explizit am Altstand. `.ignore` schließt es für die Suche aus.

Änderungen an SPEC-Abschnitten 6–9, 13, 14, 18 brauchen zwei Reviewer, einen Referenzvektor und eine Prüfung der Freeze-Liste (SPEC §1.5). Ändere die SPEC nicht still; lege einen Änderungsantrag (CR-*) an.

## 2. Workspace-Karte

| Crate | Zweck | Stand |
|---|---|---|
| `crates/contextra-storage-ports` | Traits und Wertetypen, keine Logik | weitgehend konform |
| `crates/contextra-storage` | Kern, nur `std` plus Port-Crate | im Aufbau, siehe unten |
| `crates/contextra-crypto-ref` | Dev-Referenz für Goldens | grün |

Modulstand im Kern (nur was `engine/mod.rs` tatsächlich deklariert):

- Übersetzt: `api` (nur Listener), `codec`, `keys`, `env`, `redo`, `run`, `blob`, `catalog`, `engine` (`committer`, `memtable`, `registry`, `retention`, `version`, `views`), `testing` (Feature), `sync`.
- **Nicht übersetzt, aber im Baum:** `engine/recovery.rs`, `engine/flush.rs`, `engine/compaction/*`. Sie werden nicht kompiliert. Behandle sie als Entwurf.
- `erase/`: nur Doku, kein Code.
- Es gibt **keine öffentliche Storage-Facade** (`Storage::open`, `apply`, `view`).

## 3. Befehle

Alle Befehle laufen über das Makefile. Es setzt `RUSTUP_TOOLCHAIN=stable`, weil der Pin `1.89.0` in der Umgebung nicht lädt (IMPL BL-7).

```
make check            # cargo check, alle Targets
make clippy           # clippy, Kurzausgabe (zeigt Deny-Fehler, siehe Befunde)
make test             # alle Tests, Zusammenfassung; Compilerfehler bleiben sichtbar
make test-target T=redo   # ein Integrationstest-Target
make layering         # Schichtregel-Test
make loom             # Loom-Modelle mit eigenem Target-Verzeichnis
make doc              # rustdoc mit -D warnings
make metrics          # Messwerte aus IMPL Anhang A (Skript scripts/metrics.sh)
```

Ausgaben sind lang. Nutze `make test-target T=…` statt des vollen Laufs, wenn du nur ein Gebiet prüfst.

## 4. Sofortregeln für Produktionscode

Diese Regeln stehen in SPEC §3.5. Hier die Kurzfassung, damit du sie beim Schreiben im Kopf hast.

- **Keine** `unwrap`, `expect`, `panic!`, `todo!`, `unimplemented!`, `unreachable!` in Produktion (P-13). Neue `#![allow(clippy::…)]` in Produktion sind verboten, auch um einen Fehler zu umgehen.
- **Keine** Ganzzahl-`as`-Casts zwischen Breiten; `try_from`, `checked_*` (P-14).
- Längen aus Dateien **vor** jeder Allokation gegen die Restlänge und SPEC §6.7 prüfen (P-4).
- Keine globalen veränderlichen Variablen; Testhaken über `Env::hooks()` (P-5).
- Datei maximal 400 Zeilen, Funktion typisch maximal 60 Zeilen (P-6). Aktuell verletzt: `engine/committer.rs` (1 103 Zeilen).
- Rustdoc auf Deutsch, Bezeichner auf Englisch (P-7).
- Kein `unsafe` (`forbid(unsafe_code)`).
- Keine Stubs und leeren Dateien im Baum. Wenn ein Modul nicht fertig ist, nicht deklarieren und in `IMPLEMENTATION_SPEC.md` vermerken.

## 5. Tests

- Jeder Test, der eine Invariante prüft, zitiert die ID **im Namen oder Kommentar** (`// Zitiert INV-LOG-6`). Ein Doc-Kommentar zählt nicht als Test.
- Testdateien: `tests/<bereich>/…`. Testhilfen in `src/testing/` (Feature `testing`).
- Fehlerinjektion über `FaultFs`. Fehlerschalter nach jedem Szenario zurücksetzen, sonst ist der Folgetest unbrauchbar (IMPL S-6).
- Ein grüner `MemFs`-Test beweist keine Durabilität. Persistenz nur mit Wiederöffnung nach Neustart prüfen (IMPL R-14).

## 6. Bekannte Fallen (Stand `main`)

- `tests/engine` kompiliert nicht (21 Fehler). Deshalb liefert `cargo test --workspace` keinen vollständigen Lauf.
- `cargo clippy` bricht bei den Deny-Lints ab, bis S-8 behoben ist.
- `redo`: 4 Tests rot. Drei sind Testfehler (Offset `48+31`, Fehlerschalter zurücksetzen), einer ist ein Implementierungsfehler (INV-LOG-6).
- Loom mit `RUSTFLAGS="--cfg loom"` baut alles neu. Dafür das eigene Target-Verzeichnis nutzen (`make loom`).
- `default = ["testing"]` in `Cargo.toml` zieht Testcode in jeden Build (IMPL S-9).
- Der Repo-Eintrag `RunBuilder` im Wurzelverzeichnis ist eine leere, nicht zugehörige Datei.

## 7. Commits und Pull Requests

- Betreff kurz, Typ-Präfix (`docs:`, `fix:`, `feat:`, `test:`, `chore:`). Der Rumpf erklärt das Warum.
- Befunde-IDs aus IMPL (S-*, BL-*) im Rumpf nennen, wenn der Commit sie behebt.
- Vor dem Commit: `make check` und `make test-target` für das berührte Gebiet müssen grün sein. Statusänderungen in `IMPLEMENTATION_SPEC.md` nur mit Testnachweis.
- Pull Requests: Vorlage in `.github/pull_request_template.md` ausfüllen.
- Offen (Entscheidung des Projekts): Sprache der Betreffzeilen. Bisher gemischt.

## 8. Was du nicht tust

- Keine Lint-Ausnahmen, um Clippy grün zu bekommen.
- Keine Änderung am Toolchain-Pin ohne Entscheidung zu BL-7.
- Keine Änderung an `legacy/`.
- Keine Dateien ohne Inhalt committen.
