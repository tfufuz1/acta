# legacy

Eingefrorener Altbestand. Nur Herkunft, **keine Quelle für Anforderungen** (SPEC §1.3).

- `contextra-store/` – alter LSM-Engine-Crate. Nicht Workspace-Mitglied, **nicht baubar** (`workspace = true`-Abhängigkeiten wie `contextra-ids`, `contextra-env` sind nicht definiert). Eigene Tests, Benches, Fuzz-Skripte.
- `archive/` – rund 620 Dateien aus 17+ alten Crates (z. B. `contextra-engine`, `-db`, `-checkpoint`, `-vector`). Kein Build-Pfad. Einige enthalten eigene `AGENTS.md`/`CLAUDE.md`/`README.md`, die **für dieses Repo nicht gelten**.

Hinweise:

- Nicht ändern ohne ausdrücklichen Auftrag.
- `.ignore` im Wurzelverzeichnis blendet `legacy/` aus ripgrep aus. Suche mit `rg --no-ignore`.
- `contextra-store/fuzz/run_smoke.sh` ist veraltet.
- Migration aus Legacy: IMPL WP-M, Blocker BL-1, BL-3, BL-5.
