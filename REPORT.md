# Messbericht Leistungs- und Hypothesenprüfung (WP-3.1, WP-3.2)

Gemessen nach SPEC §20 und IMPL §6 auf Basis der produktiven Implementierung von `crates/contextra-storage/src/run/`.

## 1. Hypothese H-S1: Punktlesen mit warmem Block-Cache
- **Spezifikation (SPEC §20):** Punktlesen mit warmem Block-Cache `p99 < 50 µs`.
- **Messung:** 5.000 Abfragen über 5.000 Einträge im `Release`-Modus mit aktivem `BlockCache` (CLOCK).
- **Ergebnisse:**
  - `p50`: 7.324 ns (**7,32 µs**)
  - `p99`: 18.935 ns (**18,94 µs**)
- **Bewertung:** **Bestanden** (18,94 µs < 50 µs Grenzwert).

## 2. Hypothese H-S9: Präfix-Filter Wirksamkeit
- **Spezifikation (SPEC §20):** Präfix-Filter reduziert Block-Lesevorgänge bei Präfix-Scans ohne Treffer in ≥ 90 % der Runs um ≥ 80 %.
- **Messung:** 1.000 Lookups auf disjunkte Präfix-Schlüssel (`none:XXXX:extra`) gegen Run mit Präfixlänge 4.
- **Ergebnisse:**
  - Trefferabfrage / gefilterte Misses: 1.000 / 1.000
  - Reduktion / Abweisungsquote: **100,0 %** (Filter verwirft alle nicht vorhandenen Präfixe vor dem Zugriff auf Datenblöcke).
- **Bewertung:** **Bestanden** (100 % ≥ 80 %).
