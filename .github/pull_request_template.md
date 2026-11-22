## Ziel

<!-- Was ändert sich, warum? Befunde-IDs aus IMPLEMENTATION_SPEC nennen (S-*, BL-*, WP-*). -->

## Prüfung

- [ ] `make check` grün
- [ ] `make test-target T=<Target>` grün (volles `make test` erst, wenn `tests/engine` kompiliert)
- [ ] `make clippy`: keine neuen Deny-Verstöße (`unwrap`, `expect`, `panic`)
- [ ] `make layering` grün, falls Module oder Abhängigkeiten geändert wurden
- [ ] `make metrics`: Zahlen aktualisiert, falls betroffen

## Spezifikation

- [ ] Keine Änderung an SPEC-Abschnitten 6–9, 13, 14, 18 — sonst Änderungsantrag (CR-*) und zwei Reviewer nach SPEC §1.5
- [ ] Betroffene INV-IDs im Testnamen oder Kommentar zitiert
- [ ] Freeze-Liste (SPEC §9.8) und Referenzvektor (Anhang A) geprüft, falls ein Format betroffen ist

## Status-Änderung

- [ ] Keine Statusänderung in `IMPLEMENTATION_SPEC.md` ohne Testnachweis
