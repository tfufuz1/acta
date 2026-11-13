#!/usr/bin/env bash
# =============================================================================
# remove_deleted_stubs.sh
# -----------------------------------------------------------------------------
# Entfernt Stub-Dateien aus dem Repository. Ein Stub ist eine Datei, deren
# gesamter Inhalt exakt "DELETED" lautet (optional mit abschließendem
# Zeilenumbruch oder CRLF). Nur solche Dateien werden gelöscht; alle anderen
# Dateien bleiben unberührt.
#
# Aufruf (aus beliebigem Verzeichnis, Repo-Wurzel wird relativ zum Skript
# bestimmt):
#   bash acta/scripts/remove_deleted_stubs.sh             # interaktiv
#   bash acta/scripts/remove_deleted_stubs.sh --dry-run   # nur anzeigen
#   bash acta/scripts/remove_deleted_stubs.sh --yes       # ohne Rückfrage
#
# Sicherheit:
#   - Läuft nur, wenn <Repo-Wurzel>/Cargo.toml existiert.
#   - Ignoriert .git/ und target/.
#   - Kandidaten müssen <= 16 Byte groß sein, bevor der Inhalt geprüft wird.
#   - Leere Elternverzeichnisse werden nach dem Löschen entfernt, die
#     Repo-Wurzel nie.
# =============================================================================
set -euo pipefail

usage() {
    sed -n '2,22p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'
}

DRY_RUN=0
ASSUME_YES=0
for arg in "$@"; do
    case "$arg" in
        --dry-run) DRY_RUN=1 ;;
        -y|--yes) ASSUME_YES=1 ;;
        -h|--help) usage; exit 0 ;;
        *) echo "Unbekannte Option: $arg" >&2; usage >&2; exit 2 ;;
    esac
done

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

if [[ ! -f "$REPO_ROOT/Cargo.toml" ]]; then
    echo "Abbruch: $REPO_ROOT/Cargo.toml nicht gefunden. Skript liegt nicht im Repository (erwartet: <repo>/scripts/)." >&2
    exit 1
fi

# 1. Kandidaten: Dateien <= 16 Byte, ohne .git und target/
mapfile -d '' CANDIDATES < <(
    find "$REPO_ROOT" \( -name .git -o -name target \) -prune -o \
         -type f -size -17c -print0
)

STUBS=()
for f in "${CANDIDATES[@]}"; do
    # Command-Substitution entfernt abschließende Zeilenumbrüche; \r wird vorab
    # entfernt, damit CRLF-Stubs ebenfalls erkannt werden.
    if [[ "$(tr -d '\r' < "$f")" == "DELETED" ]]; then
        STUBS+=("$f")
    fi
done

if (( ${#STUBS[@]} == 0 )); then
    echo "Keine Stub-Dateien gefunden. Nichts zu tun."
    exit 0
fi

echo "Gefundene Stub-Dateien (${#STUBS[@]}):"
for f in "${STUBS[@]}"; do
    echo "  ${f#"$REPO_ROOT"/}"
done

if (( DRY_RUN )); then
    echo "Dry-Run: nichts gelöscht."
    exit 0
fi

if (( ! ASSUME_YES )); then
    if [[ ! -t 0 ]]; then
        echo "Abbruch: keine interaktive Eingabe möglich. Mit --yes bestätigen." >&2
        exit 1
    fi
    read -r -p "Diese ${#STUBS[@]} Dateien löschen? [j/N] " answer
    case "$answer" in
        j|J|y|Y|ja|yes) ;;
        *) echo "Abgebrochen."; exit 0 ;;
    esac
fi

# 2. Löschen
for f in "${STUBS[@]}"; do
    rm -f -- "$f"
done

# 3. Leere Elternverzeichnisse entfernen (nie die Repo-Wurzel)
for f in "${STUBS[@]}"; do
    d="$(dirname "$f")"
    while [[ "$d" != "$REPO_ROOT" && "$d" == "$REPO_ROOT"/* ]]; do
        rmdir -- "$d" 2>/dev/null || break
        d="$(dirname "$d")"
    done
done

echo "Fertig: ${#STUBS[@]} Stub-Dateien entfernt."
