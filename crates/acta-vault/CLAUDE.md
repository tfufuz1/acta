# acta-vault

acta-vault: Mandantentresore, KeyAnchor, FileAnchor, Wurzelrotation, Sicherungsexport.

> Teil des Acta-Workspace. Verbindliche Vorgaben: Wurzel-`CLAUDE.md` und `docs/SPEC.md`.

## Einordnung
- **Level:** 2 (`rings.toml`)
- **Größenbudget:** 5500 LOC Produktionscode (G-HUB); Überschreitung nur mit ADR
- **Aufgabe laut SPEC 3.3:** Mandantentresore, `KeyAnchor`-Trait, `FileAnchor`, Wurzelrotation, Sicherungsexport, Vernichtung, Schlüsselableitungen
- **Erlaubte interne Abhängigkeiten (SPEC 3.3):** base, crypto, fs
- **Ist-Abhängigkeiten (Cargo.toml):** acta-base, acta-crypto, acta-fs
- **Externe Abhängigkeiten:** hkdf, sha2, thiserror, zeroize
- **Dev-Abhängigkeiten (zählen nicht für G1):** acta-testkit, proptest
- **Referenz im Quelltext:** Level L2 · Budget 5500 LOC · Spezifikation docs/SPEC.md Kap. 4.3, 4.4

## Status
- **Stand:** Arbeitspaket 2 (Teil): Klartextlayout des Eintrags, Rahmen und Positions-AAD (`entry.rs`, SPEC 4.3). 8 Tests grün, Clippy `-D warnings` sauber, `cargo xtask gates` grün.
- **Umgesetzt:** Art (`Kind`, Codes 0x01 bis 0x07) und Status (`Status`, aktiv und Grabstein), Eintrag mit Vernichtung (Schlüssel wird genullt), Klartext `[kind][status][id][key]` mit strikter Längen- und Wertprüfung, Rahmen `[len u16][nonce 24][ct 50][tag 16]` = 92 Byte, Positions-AAD `acta/vault/v1 ‖ TenantId ‖ generation u32 ‖ slot u64` (Big-Endian).
- **Nicht umgesetzt:** Versiegelung unter dem Tresorschlüssel, Tresordatei, Speicherindex, `vault_freeze`, Wurzelrotation (4.3, Schritte 1 bis 4), Sicherungsexport (4.3), `KeyAnchor`/`FileAnchor`-Traits (4.4), Schlüsselableitungen.
- **Offene Entscheidung (blockiert die Versiegelung):** `acta-crypto` stellt den Umschlag v1 bereit. Er hat Größenklassen-Padding und ein Klassenbyte, sein Layout ist `ver · alg · class · nonce · ct · tag`. Kapitel 4.3 verlangt dagegen einen festen 92-Byte-Rahmen ohne Padding. Entweder erhält `acta-crypto` eine Funktion für den festen Rahmen, oder Kapitel 4.3 wird präzisiert. Außerdem sind die Abhängigkeitsvorgaben (3.8) für die AEAD-Crate im Vault zu klären (ADR nötig).
- **Hinweis:** `Entry` trägt keine `Debug`- und `Clone`-Ableitung (Schlüsseltyp). `zeroize` ist als Abhängigkeit gelistet, wird aber noch nicht genutzt; die Nullung erfolgt manuell.


## Regeln für diesen Crate
- Abhängigkeiten nur nach unten: `level(A) > level(B)`; keine Querkanten (G1).
- Keine Alias-Re-Exports, kein `include!`, kein `#[deprecated]` (HR-6).
- Ein Fehlertyp je Crate, `thiserror`, `#[non_exhaustive]`, kein `Other(String)` (HR-4).
- Workspace-Lints gelten (`[lints] workspace = true`): kein `unwrap`, `expect`, `panic`, Indexierung mit `[]` auf Produktionspfaden.
- `unsafe` ist hier verboten (G-UNSAFE, nur `acta-sys` darf es).
- Keine Zeit- oder Zufallsquellen außerhalb der injizierten Ports `Clock` und `CryptoRng` (G3).
- Verbotene Formulierungen aus SPEC 1.6 (Liste A) nicht verwenden (G-WORD).
- Vor Abschluss: `cargo xtask gates`, `cargo test -p acta-vault`, `cargo clippy -p acta-vault --all-targets -- -D warnings`, `cargo fmt --all -- --check`.

## Vorgehen bei Überführung aus dem Archiv
1. Archivdateien lesen, nicht blind kopieren. Namen, Importe und Verweise auf frühere Spezifikationsstände sind auf Acta umzuschreiben (siehe `archive/INDEX.md`).
2. Jede Funktion gegen den Lebenszyklus in SPEC Kap. 19 und die Zusagen in 1.5 und 16.3 prüfen. Funktionen ohne Eintrag dort werden nicht implementiert (SPEC 0.4 Regel 3).
3. Tests mit Positivkontrolle mitführen (SPEC 16.4).
4. Nach vollständiger, grün geprüfter Überführung wird der Archiv-Ordner dieses Crates entfernt.
