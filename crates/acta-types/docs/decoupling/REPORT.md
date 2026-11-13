# Abschlussbericht: Entkopplung & Modernisierung `acta-types`

**Datum:** 2026-11-10
**Bearbeiter:** Principal Rust Engineer (Jules)
**Crate:** `acta-types` (`crates/acta-types`)
**Status:** Erfolgreich entkoppelt, validiert und abgenommen

---

## 1. Gate-Matrix

| Gate / Prüfung | Ausgeführter Befehl | Ergebnis |
|---|---|---|
| **Kompilierung (No-Default)** | `cargo check -p acta-types --no-default-features --locked` | **GRÜN** (0 Fehler, 0 Warnungen) |
| **Kompilierung (`docid-128`)** | `cargo check -p acta-types --features docid-128 --locked` | **GRÜN** (0 Fehler, 0 Warnungen) |
| **Kompilierung (`auto-extraction-opt-out`)** | `cargo check -p acta-types --features auto-extraction-opt-out --locked` | **GRÜN** (0 Fehler, 0 Warnungen) |
| **Unit- & Suite-Tests (Default)** | `cargo test -p acta-types --all-targets --locked` | **GRÜN** (120 Unit-Tests + 13 Suites bestanden, 0 fehlgeschlagen) |
| **Unit- & Suite-Tests (`docid-128`)** | `cargo test -p acta-types --features docid-128 --locked` | **GRÜN** (120 Unit-Tests + 13 Suites bestanden, 0 fehlgeschlagen) |
| **Lints & Clippy** | `cargo clippy -p acta-types --all-targets --locked -- -D warnings` | **GRÜN** (0 Warnungen) |
| **G-L1 (Layering Gate)** | `python3 tools/gate_layering.py --crate acta-types` | **GRÜN** (Kern- und Feature-Closures vollständig konform) |
| **G-L2 (Null-Prüfung)** | `grep -rEn 'acta_(ports\|testkit)' crates/acta-types/src --include=*.rs` | **GRÜN** (Exakt 0 Treffer, keine Rückwärtskopplung) |
| **G-U1 (Unsafe Boundary)** | `python3 tools/unsafe_boundary_auditor.py --crate acta-types` | **GRÜN** (`#![forbid(unsafe_code)]`, 0 Unsafe-Stellen) |
| **INV-DETERMINISM-1** | `python3 tools/determinism_guard.py --crate acta-types` | **GRÜN** (0 Nicht-deterministische Stellen) |
| **Zero-Panic-Audit** | `python3 tools/zero_panic_analyzer.py --crate acta-types` | **GRÜN** (0 Produktions-Panics) |
| **Wire-Format (INV-FORMAT-1)** | `cargo test -p acta-types --test golden_bytes_suite --locked` | **GRÜN** (Bitgleiche bincode-Serialisierung) |
| **ID Parity** | `cargo test -p acta-types --test golden_ids_parity --locked` | **GRÜN** (Prüfsummen identisch zu Golden Manifest) |

---

## 2. Abweichungen von Spezifikation / Prompt mit Begründung

- Keine technischen Abweichungen von den Verträgen in `docs/SPEC.md`.
- `tools/isolation_proof.py`: Scheitert im Workspace-Generierungsschritt für alle Pakete mit `{ workspace = true }`-Abhängigkeiten, da das Root-Manifest in der Sandbox ohne `[workspace.dependencies]`-Inhalt erzeugt wird. Die funktionale Isolation wurde daher über die exakte Closure-Analyse und separate Standalone-Kompilierung nachgewiesen.

---

## 3. Getroffene Entscheidungen

1. **B06 — Verwenderliste:** Statische Liste von 30 Crates in `src/lib.rs` (die das nicht existente `acta-agreement-generator` enthielt) wurde durch einen generischen Architekturverweis auf `capabilities.toml` (Ebene 2, `contracts`) ersetzt.
2. **Begriffs-Bereinigung:** Alle veralteten Begriffe (Ring 0, Ring 1, Klasse A/B, Welle 6) wurden in `src/`, `README.md`, `AGENTS.md` und `capabilities.fragment.toml` durch standardisierte Baukasten-Terminologie ersetzt.
3. **Migrations-Annotationen (R3):** Für veraltete Typen (`erasure.rs`, `witness.rs`, `ids.rs`), die in Folgeschritten nach `acta-erasure` übergehen, wurden verbindliche `// MIGRATION(S-70): ...`-Kommentare gesetzt, ohne bestehende Pfade für Konsumenten zu brechen (INV-2).
4. **Schema v3:** `capabilities.fragment.toml` wurde konsistent mit dem Root-Schema auf v3 (`group = "contracts"`, `level = 2`, `tier = "stable"`, `allowed_internal = ["acta-ids"]`) migriert.

---

## 4. Offene Punkte / Risiken

- **Entfernung veralteter Typen (S-70):** Die Löschtypen in `src/erasure.rs` und `src/witness.rs` bleiben bis Schritt S-70 als `#[deprecated]` erhalten, um Abwärtskompatibilität für alle Workspace-Konsumenten zu garantieren.
- **`ActaError`-Varianten:** Wie in `docs/SPEC.md` §6.3 festgelegt, verbleiben historische Varianten (`Sandbox*`, `Cluster`, etc.) in `ActaError`, bis alle lokalen Fehlerbäume der Konsumenten vollständig entkoppelt sind.

---

## 5. Übergaben an Z (Handoffs)

1. **`tools/isolation_proof.py`:**
   - **Problem:** Das Skript erzeugt bei Zeile 108 ein temporäres `Cargo.toml` mit leerem `[workspace.dependencies]`.
   - **Empfehlung für Z:** Kopieren der Root-Tabelle `[workspace.dependencies]` in die temporäre Workspace-Konfiguration, damit Crates mit vererbten Abhängigkeiten offline validiert werden können.
2. **`acta-store/src/scanner/mod.rs:91`:**
   - Ein Aufruf von `acta_types::RequestContext::new_root()` existiert in fremdem Code (`acta-store`), obwohl `RequestContext` bewusst keinen Default-Mandanten anbietet (VETO-F10). Dies muss im Rahmen der Entkopplung von `acta-store` behoben werden.
3. **Konsolidierung `PprConfig` / `TlHfdParams`:**
   - Wandern in Schritt S-34/S-51 nach `acta-graph`, verbleiben bis dahin abwärtskompatibel in `acta-types`.

---

## 6. Messwerte

- **LOC (Produktion):** 4.540 Zeilen in 22 Quelldateien
- **Interne Abhängigkeiten:** 1 (`acta-ids`)
- **Modulkopplung:** 0 Verstöße gegen Layering-Gates (G-L1 GRÜN)
- **Zero-Panic-Score:** 100% (0 Produktionsverstöße)
- **Determinismus:** 100% (0 Zeit-/Zufallsquellen im Rechenkern)
- **Test-Erfolgsquote:** 100% (133 Tests / Test-Suites bestanden)
