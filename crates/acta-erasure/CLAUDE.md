# acta-erasure

acta-erasure: Loeschkern: Saga, Gate-Journal, Messung, Restore-Schutz, Mehrbetroffenen-Plan.

> Teil des Acta-Workspace. Verbindliche Vorgaben: Wurzel-`CLAUDE.md` und `docs/SPEC.md`.

## Einordnung
- **Level:** 4 (`rings.toml`)
- **Größenbudget:** 9000 LOC Produktionscode (G-HUB); Überschreitung nur mit ADR
- **Aufgabe laut SPEC 3.3:** Löschkern: Gate-Journal, Zustandsmaschine, Messung, Bericht-Rohdaten, Restore-Schutz, Mehrbetroffenen-Plan
- **Erlaubte interne Abhängigkeiten (SPEC 3.3):** base, crypto, fs, gate, ledger, vault, seg, pstore
- **Ist-Abhängigkeiten (Cargo.toml):** acta-base, acta-crypto, acta-fs, acta-gate, acta-ledger, acta-pstore, acta-seg, acta-vault
- **Externe Abhängigkeiten:** roaring, thiserror
- **Dev-Abhängigkeiten (zählen nicht für G1):** acta-testkit, proptest
- **Referenz im Quelltext:** Level L4 · Budget 9000 LOC · Spezifikation docs/SPEC.md Kap. 8

## Status
- **Stand:** Arbeitspaket 1 abgeschlossen: reine Logik ohne E/A. Tests grün (35), Clippy `-D warnings` sauber, `cargo fmt --check` sauber, `cargo xtask gates` grün (alle sieben Gates).
- **Umgesetzt:**
  - Saga-Automat S0 bis S9 je Weg, Standardpfad und vorgezogener Pfad (`saga.rs`, SPEC 8.3). Wahl des Wegs nach `Besonders` oder SLA ≥ 700 Promille.
  - Lesesperre und `restricted_read`, Zugriffsrecht ohne Existenzhinweis (`read.rs`, SPEC 8.1, Z1, Z16).
  - Messung mit Positivkontrolle: ungültiger Lauf zählt nie als bestanden (`measure.rs`, SPEC 8.5, Z11).
  - Erfüllungsklassen G, B, K, M, kumulativ, nur `reached` oder `not_reached` (`classes.rs`, SPEC 8.7).
  - Trockenlauf für Mitbetroffene mit Modusauswahl; nie automatisches `DeleteAll` nach Fristablauf (`multi.rs`, SPEC 8.9, Z3).
- **Nicht umgesetzt (Ausführung und Speicherung):** Schritte mit Seiteneffekten (Gate-Sperre durabel in S1, Bereinigung S3, Schlüsselvernichtung S5, Rotation S6, Sicherungsfenster S7), Messverfahren selbst (8.5), Restore-Schutz (8.8), Ausgangskorb und Kopienregister (8.6), `ArchiveHook` (8.4), Schwärzung für `RedactSplit`.
- **Blockiert durch Gerüste:** `acta-vault`, `acta-seg`, `acta-ledger` und `acta-pstore` enthalten nur Fehlertypen (je ca. 25 Zeilen). Die Ausführung der Schritte kann erst nach deren Umsetzung folgen.
- **Offene Widersprüche in der Spezifikation:**
  - Anhang B.1 nennt `Requested → Planned` und `Planned → Gated`. Kapitel 8.3 nennt `S1 → Gated` und `S2 → Planned`. Der Automat folgt 8.3, weil Kapitel 8 das Verfahren beschreibt. Entscheidung durch den Nutzer nötig.
  - 8.7 sagt, die Klassen bauen aufeinander auf. Der Crate setzt daher B ⇒ G, K ⇒ B, M ⇒ K voraus. Bestätigung durch den Nutzer nötig.
- **Archiv:** `archive/acta-erasure/` bleibt bestehen, bis die Überführung vollständig ist (siehe Regeln unten).
- **Schnittstelle:** Gate-Zustände und Gründe kommen aus `acta-gate` (`GateState`, `RestrictReason`, `EraseReason`). Eigene Duplikate gibt es nicht.

## Regeln für diesen Crate
- Abhängigkeiten nur nach unten: `level(A) > level(B)`; keine Querkanten (G1).
- Keine Alias-Re-Exports, kein `include!`, kein `#[deprecated]` (HR-6).
- Ein Fehlertyp je Crate, `thiserror`, `#[non_exhaustive]`, kein `Other(String)` (HR-4).
- Workspace-Lints gelten (`[lints] workspace = true`): kein `unwrap`, `expect`, `panic`, Indexierung mit `[]` auf Produktionspfaden.
- `unsafe` ist hier verboten (G-UNSAFE, nur `acta-sys` darf es).
- Keine Zeit- oder Zufallsquellen außerhalb der injizierten Ports `Clock` und `CryptoRng` (G3).
- Verbotene Formulierungen aus SPEC 1.6 (Liste A) nicht verwenden (G-WORD).
- Vor Abschluss: `cargo xtask gates`, `cargo test -p acta-erasure`, `cargo clippy -p acta-erasure --all-targets -- -D warnings`, `cargo fmt --all -- --check`.

## Vorgehen bei Überführung aus dem Archiv
1. Archivdateien lesen, nicht blind kopieren. Namen, Importe und Verweise auf frühere Spezifikationsstände sind auf Acta umzuschreiben (siehe `archive/INDEX.md`).
2. Jede Funktion gegen den Lebenszyklus in SPEC Kap. 19 und die Zusagen in 1.5 und 16.3 prüfen. Funktionen ohne Eintrag dort werden nicht implementiert (SPEC 0.4 Regel 3).
3. Tests mit Positivkontrolle mitführen (SPEC 16.4).
4. Nach vollständiger, grün geprüfter Überführung wird der Archiv-Ordner dieses Crates entfernt.
