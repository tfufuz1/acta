# Git- und PR-Workflow

Stand: 2026-11-22.

## Ist-Zustand

- Remote `origin` = `https://github.com/tfufuz1/acta.git`.
- Branches: `main` (Standard) und `spec/implementation-rev3`. Der zweite ist inhaltlich identisch mit `main` und veraltet. Löschen nur nach Bestätigung durch den Nutzer.
- Die Historie hat rund 76 Commits. Davon heißen ~70 nur „Shell-Commit“. Diese Sammel-Snapshots sind nicht umzuschreiben (sie sind öffentlich). Neue Arbeit folgt dem Muster unten.
- Es gibt keine aktiven Git-Hooks, kein `.github/`, keine CI, kein PR-Template, keine CODEOWNERS.

## Erlaubnis

Dauerhafte Freigabe des Nutzers (2026-11-22): Claude pusht Feature-Branches und öffnet danach immer selbst einen PR gegen `main`, ohne vorher zu fragen. Ist der bisherige PR schon gemergt, kommt ein neuer PR. Weiterhin verboten ohne neue Anweisung: auf `main` pushen, Force-Push, selbst mergen, fremde Branches oder PRs löschen oder schließen. Kurzfassung mit Befehlen: `CLAUDE.md` §5.

## Regeln

1. Nie direkt auf `main` committen oder pushen, außer der Nutzer erlaubt es ausdrücklich.
2. Branch-Namen: `feat/<kurz>`, `fix/<kurz>`, `docs/<kurz>`, `spec/<kurz>`. Kleinbuchstaben, Bindestriche.
3. Commit-Messages im Conventional-Commits-Stil, wie in den guten Commits der Historie: `feat(storage): …`, `fix(...): …`, `docs: …`. Keine Sammel-Messages wie „Shell-Commit“.
4. Kleine, thematische Commits. Keine Dateien mischen, die nichts miteinander zu tun haben.
5. Vor jedem Push: `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` laufen lassen. Ergebnisse im PR-Text nennen. Ist das wegen Umgebung nicht möglich (siehe `environment.md`), dies im PR ausdrücklich vermerken.
6. Keine Credentials, Tokens oder Secrets committen.

## Ablauf

```bash
git switch -c feat/<name>
# Änderungen, kleine Commits
git push -u origin feat/<name>
```

Danach PR gegen `main` erstellen:

- Per REST (funktioniert in Claude-Code-Sitzungen, geprüft): `gh api repos/tfufuz1/acta/pulls -f title='…' -f head='feat/<name>' -f base='main' -f body='…' --jq .html_url` (Entwurf: zusätzlich `-F draft=true`).
- `gh pr create` und andere GraphQL-Befehle scheitern in Sessions mit HTTP 403 („GitHub GraphQL is not available“). Nicht wiederholen, REST nutzen.
- PR-Text endet mit `🤖 Generated with [Claude Code](https://claude.com/claude-code)` und der Session-URL, Commits mit den vorgegebenen `Co-Authored-By`- und `Claude-Session`-Zeilen.
- Fallback ohne `gh`: Compare-URL `https://github.com/tfufuz1/acta/compare/main...feat/<name>` öffnen und dort den PR anlegen.
- Fallback ohne Push-Zugriff: `git format-patch main..feat/<name>` erzeugen und dem Nutzer übergeben.

## GitHub-Zugriff in der Session

- Der Repo-Zugriff wird über `mcp__claude-code-remote__add_repo` angehängt (`owner: tfufuz1`, `repo: acta`, `access: push` für Schreibzugriff). Vorher nicht mit `gh repo view` oder `git ls-remote` vorprüfen, denn unauthentifizierte Anfragen liefern fälschlich 404.
- Nach erfolgreichem Anhängen Klon ausführen (shallow reicht, siehe `environment.md`), dann `register_repo_root` mit dem Klonpfad aufrufen, damit CLAUDE.md geladen wird.
- Bei Ablehnung den genauen Grund aus der Antwort weitergeben. Der Hinweis für den Nutzer: GitHub unter claude.ai Settings → Connectors neu verbinden oder einen Organisations-Owner um Freigabe bitten. Nicht wiederholen.
- Bei shallow Klonen: Push mit `git -c push.negotiate=true push -u origin <branch>`, sonst kann die Anfrage mit HTTP 413 scheitern.

## PR-Zustand vor jedem Push prüfen

`gh api repos/tfufuz1/acta/pulls/<n> --jq '{state,merged}'`. Wurde der PR gemergt, erreichen weitere Commits auf dem Branch `main` nicht. Für den Rest nach Rückfrage einen neuen PR öffnen (Beispiel: #3 wurde gemergt, bevor die späteren Commits kamen, Folge-PR #4).

## Ungeklärt

- Branch-Schutzregeln auf `main` sind von der VM aus nicht prüfbar, weil `gh` nicht authentifiziert ist.
