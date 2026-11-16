# acta-gate

acta-gate: ErasedView, GateWriter, Roaring-Mengen, ErasureLayer, LayerAck.

> Teil des Acta-Workspace. Verbindliche Vorgaben: Wurzel-`CLAUDE.md` und `docs/SPEC.md`.

## Einordnung
- **Level:** 1 (`rings.toml`)
- **Größenbudget:** 1200 LOC Produktionscode (G-HUB); Überschreitung nur mit ADR
- **Aufgabe laut SPEC 3.3:** `ErasedView`, `GateWriter`, Roaring-Mengen, Epoche, Traits `ErasureLayer`, `LayerAck`
- **Erlaubte interne Abhängigkeiten (SPEC 3.3):** base
- **Ist-Abhängigkeiten (Cargo.toml):** acta-base
- **Externe Abhängigkeiten:** roaring, thiserror, hmac, sha2, zeroize (alle auf der Whitelist, SPEC 3.8)
- **Dev-Abhängigkeiten (zählen nicht für G1):** acta-testkit, proptest
- **Referenz im Quelltext:** Level L1 · Budget 1200 LOC · Spezifikation docs/SPEC.md Kap. 3.7, 5.3

## Status
- **Stand:** Kern umgesetzt (Zustand, Log, Schreiber, Verträge); Verdichtung und Persistenz offen
- **Produktionscode (ohne Tests/Leerzeilen/Kommentare):** 836 LOC (Budget 1200)
- **Tests:** `tests/gate_state.rs` (11), `tests/gate_writer.rs` (10), `tests/gate_model.rs` (2, Proptest gegen Referenzmodell). Alle grün; Clippy `-D warnings` und `fmt` sauber.
- **Umgesetzt:** `GateState`, `LineageHit` (Rangfolge Erased > Redacted > Restricted > Superseded), `Lineage` (Z13), `TargetSet`, `GateSnapshot` mit Roaring-Mengen und Epoche, `ErasedView` (fail-closed bei nicht lesbarer Sperre), Gate-Log mit HMAC-Kette `[seq][op][len][payload][tag]` für die Operationen 0x01 bis 0x07, `GateWriter` mit Z4-Reihenfolge (RAM zuerst, dann Anhängen und `fsync`, Fehler vergiftet), `ErasureLayer`-Vertrag, `LayerAck`, Gründe und Codes (8.1, 8.2).
- **Abweichungen (begründet):**
  - HMAC direkt über `hmac`/`sha2`, nicht über `acta-crypto`: beide liegen auf Level 1, G1 verbietet die Kante. Der Schlüssel wird vom Aufrufer aus `HKDF(k_idx, "acta/gate/v1" ‖ TenantId)` abgeleitet und als `GateKey` übergeben.
  - Kein E/A im Crate: Das Log läuft über den Port `GateSink` (`append`, `sync`). Die Datei-Implementierung liegt außerhalb.
  - `LayerError` ist `GateError` (HR-4: ein Fehlertyp je Crate). `LayerAck` ist ein Wertetyp, kein Trait (SPEC 3.7 verwendet ihn als Rückgabe).
  - Herkunft (`Lineage`) trägt `VerOrd` und `KOrd`; `SubjOrd` läuft über die Versionen.
  - Die Sperre einer nicht lesbaren Sperre meldet fail-closed (`Erased`, `Epoche = u64::MAX`).
  - Ein abgeschnittener letzter Rahmen wird verworfen. Restrisiko: Eine beschädigte Längenangabe im letzten Rahmen ist vom abgeschnittenen Schreibvorgang nicht zu unterscheiden. Beschädigung in der Mitte wird über die Kette erkannt.
- **Offen:**
  - Verdichtung (5.3, `0x08 Snapshot`, C23) nicht umgesetzt; Log wächst bis dahin.
  - Linearisierungstest `T-CONC-GATE` (loom) und Crash-Punkte C-Liste (`T-CRASH`) fehlen.
  - Keine Erasure-Schicht implementiert; Verträge stehen nur. Messung von Rückständen mit Positivkontrolle (Z11) gehört in den Crate, der sie ausführt.
  - Datei-Sink (`GateSink` auf `DurableVfs`) fehlt; wird mit `acta-node` verdrahtet.
  - Aus dem Archiv noch nicht überführt: `erasure/src/gate/batch.rs`, `fence.rs`, `erased_set.rs`, `tests/loom_gate.rs`, `gate_linearization_test.rs` (async und parking_lot, sind nach SPEC neu zu bewerten).
- **Archiv-Quelle:** `archive/acta-gate/` (siehe `archive/acta-gate/MANIFEST.md`). Der Ordner bleibt bestehen, bis die Überführung vollständig und grün ist (Regel aus Wurzel-CLAUDE.md).

## Regeln für diesen Crate
- Abhängigkeiten nur nach unten: `level(A) > level(B)`; keine Querkanten (G1).
- Keine Alias-Re-Exports, kein `include!`, kein `#[deprecated]` (HR-6).
- Ein Fehlertyp je Crate, `thiserror`, `#[non_exhaustive]`, kein `Other(String)` (HR-4).
- Workspace-Lints gelten (`[lints] workspace = true`): kein `unwrap`, `expect`, `panic`, Indexierung mit `[]` auf Produktionspfaden.
- `unsafe` ist hier verboten (G-UNSAFE, nur `acta-sys` darf es).
- Keine Zeit- oder Zufallsquellen außerhalb der injizierten Ports `Clock` und `CryptoRng` (G3).
- Verbotene Formulierungen aus SPEC 1.6 (Liste A) nicht verwenden (G-WORD).
- Vor Abschluss: `cargo xtask gates`, `cargo test -p acta-gate`, `cargo clippy -p acta-gate --all-targets -- -D warnings`, `cargo fmt --all -- --check`.

## Vorgehen bei Überführung aus dem Archiv
1. Archivdateien lesen, nicht blind kopieren. Namen, Importe und Verweise auf frühere Spezifikationsstände sind auf Acta umzuschreiben (siehe `archive/INDEX.md`).
2. Jede Funktion gegen den Lebenszyklus in SPEC Kap. 19 und die Zusagen in 1.5 und 16.3 prüfen. Funktionen ohne Eintrag dort werden nicht implementiert (SPEC 0.4 Regel 3).
3. Tests mit Positivkontrolle mitführen (SPEC 16.4).
4. Nach vollständiger, grün geprüfter Überführung wird der Archiv-Ordner dieses Crates entfernt.
