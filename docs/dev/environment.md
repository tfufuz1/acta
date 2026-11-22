# Umgebung (VM) für Claude-Sitzungen

Stand: 2026-11-22, verifiziert durch read-only Inspektion. Werte können sich ändern. Vor Annahmen mit den Befehlen unten prüfen.

## Steckbrief

- OS: Ubuntu 24.04, x86_64, Benutzer `root`, Shell `bash`
- cargo/rustc/clippy/rustfmt: 1.97.0 (rustup `stable`)
- rustup 1.28.2, git 2.43, gh 2.89, python3 3.13, node 22, make 4.3, clang 18, gcc 13, pkg-config 1.8
- Nur die Toolchain `stable` ist installiert. Nightly fehlt. `cargo-deny` und `cargo-fuzz` fehlen.

## Toolchain-Pin

`rust-toolchain.toml` pinnt `1.89.0` (mit rustfmt, clippy). Diese Version kann in der VM **nicht geladen** werden, weil `static.rust-lang.org` nicht erreichbar ist (Tunnel-Fehler). Jeder Cargo-Aufruf im Repo scheitert dann ohne Workaround.

Workaround:

```bash
export RUSTUP_TOOLCHAIN=stable
cargo --version   # erwartet: cargo 1.97.0
```

Folgen: `rust-version = "1.89"` und Edition 2024 sind erfüllt. Die Lints von clippy und rustfmt laufen aber gegen 1.97, nicht gegen 1.89. Ergebnisse dürfen nicht als „mit 1.89 geprüft“ berichtet werden. Die Entscheidung über den Pin ist offen (BL-7), siehe `open-items.md`.

## Netzwerk und Proxy

- Ausgehender Verkehr läuft über einen Agent-Proxy. CA-Bundle: `/root/.ccr/ca-bundle.crt`. Die Variablen `SSL_CERT_FILE`, `CURL_CA_BUNDLE`, `CARGO_HTTP_CAINFO` und `GIT_SSL_CAINFO` sind bereits gesetzt.
- Erreichbar (geprüft): `index.crates.io`, `static.crates.io`, `github.com` (git über HTTPS).
- Nicht erreichbar: `static.rust-lang.org`.
- Regeln aus `/root/.ccr/README.md`: TLS-Prüfung nie abschalten, `HTTPS_PROXY` nie entfernen. Bei 403/407 vom Proxy den Host melden, nicht umgehen. Bei „unsuccessful tunnel“ zuerst die `[agent-proxy]`-Zeilen im Output lesen.
- Ein `git clone` kann über den Proxy mehrere Minuten dauern. Timeout großzügig setzen. Den Befehl nicht wegen scheinbarem Stillstand abbrechen.
- Ein Repo darf nur einmal gleichzeitig geklont werden (Proxy-Limit 2 parallele Smart-HTTP-Operationen). Bei HTTP 429: 5 Sekunden warten, einmal wiederholen.

## Credentials

- `GH_TOKEN` ist gesetzt, wird aber als **ungültig** abgelehnt (`gh auth status`). `gh pr create` und andere schreibende `gh`-Befehle scheitern, bis ein gültiges Token gesetzt ist. Den Wert nie ausgeben.
- Git hat keinen Credential-Helper. Der Push-Zugriff läuft über den angehängten GitHub-Scope (siehe `git-workflow.md`).
- Git-Identität ist gesetzt (`Claude <noreply@anthropic.com>`). Signing-Keys sind konfiguriert.
- Keine `~/.cargo/config.toml`.

## Pfade

- Repo-Klone: `/home/claude/acta` (Session-Klon, shallow) oder `/tmp/acta` (vollständiger Klon, nur Lesen).
- Scratchpad für temporäre Dateien: `/tmp/claude-0/-home-claude/<session-id>/scratchpad` (Modus 700). Nicht im Repo ablegen.
- Speicher: rund 30 GB frei auf `/`.

## Erst-Checkliste

```bash
cd /home/claude/acta
git status --short && git branch --show-current
export RUSTUP_TOOLCHAIN=stable
cargo --version
curl --max-time 10 -sI https://index.crates.io/config.json | head -1
git ls-remote --heads origin | head -3
gh auth status 2>&1 | head -3      # erwartet bis zur Erneuerung: Token ungültig
cargo check --workspace            # Baseline
```

## Fuzzing

`fuzz/` ist ein eigenes Cargo-Projekt (edition 2021, eigenes `[workspace]`). Befehle daher im Verzeichnis `fuzz/` ausführen. `cargo-fuzz` braucht Nightly, und beides fehlt aktuell. Ohne Installation ist Fuzzing in dieser VM nicht möglich.

## Veraltete Skripte

`legacy/contextra-store/fuzz/run_smoke.sh` ist veraltet. Es ruft `cargo check --locked` im Legacy-Crate auf (scheitert) und nennt Targets, die es nicht mehr gibt. Nicht als Referenz verwenden.
