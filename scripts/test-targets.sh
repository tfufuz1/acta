#!/usr/bin/env bash
# Führt jedes Test-Target des Pakets einzeln aus. Ersatz für `make test`, das am
# Compilerfehler in tests/engine abbricht (CLAUDE.md §3). Kein Target wird ausgelassen.
# Am Ende steht eine Übersicht; mindestens ein rotes Target ergibt Exit 1.
#
# Aufruf: scripts/test-targets.sh   (Umgebung: PAKET, Standard contextra-storage)
# Die Toolchain kommt aus rust-toolchain.toml bzw. aus RUSTUP_TOOLCHAIN.

set -u

PAKET="${PAKET:-contextra-storage}"
cd "$(dirname "$0")/.." || exit 2

# Targets aus cargo metadata: Lib, Integrationstests, danach Doctests.
mapfile -t TARGETS < <(
    cargo metadata --no-deps --format-version 1 |
        python3 -I -c '
import json, sys
paket = sys.argv[1]
meta = json.load(sys.stdin)
for p in meta["packages"]:
    if p["name"] != paket:
        continue
    for t in p["targets"]:
        if "lib" in t["kind"]:
            print("lib")
        elif "test" in t["kind"]:
            print("test:" + t["name"])
print("doc")
' "$PAKET"
)

if [ "${#TARGETS[@]}" -eq 0 ]; then
    echo "Keine Targets für Paket '$PAKET' gefunden." >&2
    exit 2
fi

ERGEBNISSE=()
ROT=0

for ziel in "${TARGETS[@]}"; do
    case "$ziel" in
        lib) args=(--lib) ;;
        doc) args=(--doc) ;;
        test:*) args=(--test "${ziel#test:}") ;;
    esac

    echo "::group::cargo test -p $PAKET ${args[*]}"
    log="$(mktemp)"
    cargo test -p "$PAKET" "${args[@]}" >"$log" 2>&1
    rc=$?
    cat "$log"
    echo "::endgroup::"

    zusammenfassung="$(grep -E '^test result:' "$log" | tail -1)"
    rm -f "$log"

    if [ "$rc" -eq 0 ]; then
        status="GRÜN"
    else
        status="ROT"
        ROT=$((ROT + 1))
    fi
    ERGEBNISSE+=("$status|$ziel|${zusammenfassung:-kein Testergebnis (Compilerfehler oder Abbruch)}")
done

# Übersicht in der Konsole und, falls vorhanden, in der Job-Zusammenfassung.
uebersicht="| Status | Target | Ergebnis |
|---|---|---|"
for zeile in "${ERGEBNISSE[@]}"; do
    IFS='|' read -r status ziel text <<<"$zeile"
    uebersicht+=$'\n'"| $status | \`$ziel\` | $text |"
done

echo "$uebersicht"
if [ -n "${GITHUB_STEP_SUMMARY:-}" ]; then
    printf '%s\n' "$uebersicht" >>"$GITHUB_STEP_SUMMARY"
fi

if [ "$ROT" -gt 0 ]; then
    echo "$ROT von ${#TARGETS[@]} Targets sind rot." >&2
    exit 1
fi
echo "Alle ${#TARGETS[@]} Targets grün."
