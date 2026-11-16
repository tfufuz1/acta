# Implementierungsplan: `acta-base`

> **Status:** verbindlich für die Crate-Überführung · **Paket:** 1 (L0)
> **Vorgaben:** [docs/SPEC.md](../../docs/SPEC.md) · [docs/spec/03_systemarchitektur.md](../../docs/spec/03_systemarchitektur.md) (3.3, 3.5, 3.7) · [docs/spec/04_kryptografie.md](../../docs/spec/04_kryptografie.md) (4.6) · [docs/spec/anhang_a_parameter.md](../../docs/spec/anhang_a_parameter.md) · [CLAUDE.md](CLAUDE.md) · [archive/acta-base/MANIFEST.md](../../archive/acta-base/MANIFEST.md)

## 0. Verifikationsstand dieser Planung
- Der Ist-Stand stammt aus `CLAUDE.md` des Crates und aus dem Quelltext (`pub`-Oberfläche). **Build und Tests wurden für diese Überarbeitung nicht ausgeführt**: Die Rust-Toolchain 1.89.0 war in der Planungsumgebung nicht verfügbar. Vor jeder Abnahme sind `cargo test -p acta-base` und die Gates selbst auszuführen.
- Die Spezifikationsabschnitte 2.4 (Newtypes) und Kap. 19 wurden für dieses Crate nur auszugsweise gegengelesen. Vor Abschluss von AP 1 ist 2.4 vollständig gegen `ids.rs` zu prüfen.

## 1. Einordnung
- **Level:** 0 (`rings.toml`). Keine internen Abhängigkeiten. Alle anderen Crates hängen von `acta-base` ab.
- **Budget:** 2 500 LOC Produktionscode (G-HUB, harte Grenze). Ist laut CLAUDE.md ca. 603 LOC.
- **Externe Abhängigkeiten:** `thiserror` (Whitelist). Dev: `proptest`.
- **Aufgabe (SPEC 3.3):** Newtypes (Ids, Ordinale), `DataClass`, `ErrorClass`, `ClassifiedError`, Ports `Clock` und `CryptoRng`, kanonische Kodierung, Festkomma `fx`, Grenzwerte.

## 2. Ist-Stand (laut CLAUDE.md, nicht neu verifiziert)
| Modul | Stand |
|---|---|
| `ids` | Ids, Ordinale, `ErasureId` (UUIDv7), `SubjectTag`, `ClusterId`, `IndexGen`, `ObjectId` |
| `class` | `DataClass` mit Ebenenzuordnung |
| `error` | `BaseError`, `ClassifiedError` |
| `ports` | `Clock` (`now_ms`), `CryptoRng` (`try_fill`, fallibel) |
| `canon` | `Reader`/`Writer`, geprüfte Längen vor Allokation, atomarer Schreiber |
| `fx` | Q32.32, `fx_ln`, `isqrt`, Sättigung |
| `limits` | Teil von Anhang A (A.1 bis A.4) |
| Tests | `tests/fx_golden.rs` (18 Goldvektoren, gegen `tests/golden/fx_ln_oracle.py`), `tests/proptest_base.rs` (12 Eigenschaften) |

## 3. Spec-Bezug
- **Z7 (Zeitreise-Determinismus):** Festkomma ist die einzige Zahlenarithmetik im Ranking-Pfad. Die Goldvektoren müssen auf x86_64 **und** aarch64 identisch sein. Der aarch64-Lauf ist **offen**.
- **Z15:** Fehlertexte und `code()` enthalten keine Nutzdaten und keine Kennungen. Das gilt für `BaseError` und jeden `ClassifiedError`.
- **HR-4:** Ein Fehlertyp je Crate, `#[non_exhaustive]`, kein `Other(String)`.
- **3.7:** `Clock::now_ms` und `CryptoRng::try_fill` sind die einzigen Zeit- und Zufallsquellen für den Kern (G3).

## 4. Archiv-Entscheidungen

| Archivquelle | Entscheidung | Begründung |
|---|---|---|
| `ids/src/lib.rs` (720 Zeilen, `serde`) | Nur Konzepte übernehmen, Typen neu schreiben | Alte Id-Typen (u64-`TenantId`, `DocId`) passen nicht zu SPEC 2.4. `serde` ist für Plattenformate verboten. |
| `ids/tests/golden_tombstone_bit.rs` | Prüfen, dann übernehmen | Testet Grabstein-Bit. Golden-Daten neu erzeugen (ALTLASTEN). |
| `ids/tests/ids_tests.rs` (5 Panik-Stellen) | Übernehmen, `unwrap` entfernen | G-LINT |
| `ids/tests/differential_ids.rs` | Entfernt im Archiv | Nicht vorhanden, nichts zu tun |
| `types/src/error.rs` (1 177 Zeilen) | **Nicht** übernehmen | Sammel-Fehlertyp verletzt HR-1/HR-4. Fehlertypen gehören je Crate. |
| `types/.../ids_tests.rs`, `types/tests/newtype_invariants_suite.rs` | Invarianten als Oracle prüfen, Umfang reduzieren | Nur Newtype-Invarianten, die SPEC 2.4 fordert |
| `env/src/clock.rs`, `SystemClock` | **Nach `acta-sys`** verschieben | SPEC 3.3 ordnet `SystemClock` `acta-sys` zu. L0 kennt nur den Port. |
| `env/src/rng.rs` (`SeededRng`, `f64`) | **Nicht** übernehmen | Testdouble liegt in `acta-testkit`. `f64` ist im Kern verboten (Z7). |
| `env/src/id_gen.rs`, `metrics` | **Nicht** übernehmen | Kein Eintrag in SPEC Kap. 19 (Default-deny, 0.4 Regel 3) |
| `blake3` (Altquelle) | **Nicht** verwenden | Nicht auf der Whitelist (3.8). Hashes sind SHA-256 in `acta-crypto`. |

## 5. Offene Punkte und Spec-Lücken

1. **aarch64-Goldvektoren (Z7):** Kein aarch64-Target und kein qemu in der Planungsumgebung. Abnahme nur mit echtem Lauf oder CI-Matrix (16.2 „Plattform“). Bis dahin im Bericht als **nicht geprüft** ausweisen.
2. **Anhang A unvollständig:** Nicht aufgenommen sind A.5 bis A.9, `erase_registry_sink`, `access_log_retention`, `bridge_threads` (kernzahlabhängig) und die Monatsparameter (`split_retest_interval`, `actor_handle_epoch`). **Vorgehen:** Konstanten erst dort anlegen, wo ein Verbraucher-Crate sie braucht. Pflichtparameter ohne Startwert (`T_BACKUP`, `anchor_destroy_max`, `support_min`, `inference_budget`, `erase_registry_sink`, `access_log_retention`) bekommen **keine** Konstante (0.4 Regel 1).
3. **`ErrorClass`-Varianten:** Der Code muss exakt `Transient, Integrity, Crypto, Erased, Policy, Invalid` (3.5) haben und `#[non_exhaustive]` sein. Vor AP 1 prüfen.
4. **Verbotene Formulierungen (G-WORD):** Fehlertexte und Doc-Kommentare gegen Liste A prüfen (1.6).

## 6. Arbeitspakete

**AP 1 – Newtypes gegen SPEC 2.4 abgleichen.** Jeden `ids`-Typ gegen 2.4 lesen: 16-Byte-Handles, streng monotone Ordinale, Größen. Abweichungen korrigieren oder als ADR festhalten.
- *Abnahme:* Jeder Typ hat einen Verweis auf seine SPEC-Stelle im Doc-Kommentar. `T-` Tests zu Monotonie und Eindeutigkeit bestehen.

**AP 2 – Archiv-Orakel übernehmen.** `golden_tombstone_bit` und `ids_tests` als Tests nach `crates/acta-base/tests/` übernehmen. Golden-Daten neu erzeugen. Panik-Stellen entfernen.
- *Abnahme:* Keine `unwrap`/`expect`/`panic`-Treffer im Produktionscode (G-LINT). Golden-Dateien stammen aus einer unabhängigen Quelle, nicht aus dem getesteten Code.

**AP 3 – Positivkontrolle und Eigenschaften ergänzen.** Für jeden Golden-Test eine Positivkontrolle nach 16.4 (absichtlich falscher Wert muss scheitern; `golden_check_detects_a_wrong_value` ist bereits aktiv und ist das Muster).
- *Abnahme:* Jede Negativaussage hat einen Anwesenheitsnachweis.

**AP 4 – Anhang A ergänzen (bedarfsgetrieben).** Konstanten erst mit dem Verbraucher-Crate einführen, der sie braucht. Bounds-Prüfung über `Bounds` beibehalten.
- *Abnahme:* Jeder Parameter hat Startwert, Bereich und Abschnitt (Anhang A). Keine stillen Standardwerte für Pflichtparameter.

**AP 5 – aarch64-Abnahme (Z7).** Goldvektoren im CI-Lauf auf aarch64 ausführen.
- *Abnahme:* Ergebnis im Messbericht (16.5) mit Commit-Stand. Ohne Lauf bleibt Z7 für `acta-base` **offen**.

**AP 6 – Archivordner entfernen.** Erst nach AP 1 bis 3 und grünen Gates: `archive/acta-base/` löschen (Regel 7 in `archive/INDEX.md`). Das gilt erst, wenn die Ids-Orakel übernommen sind.

## 7. Tests und Gates

| Test | Zusage / Gate |
|---|---|
| `fx_golden` (18 Goldvektoren) | Z7, `T-TT-FX` |
| `proptest_base` (12 Eigenschaften) | Hin-/Rückumwandlung, Sättigung, Algebra, Monotonie |
| Neu: Ids-Orakel (AP 2) | Ids-Monotonie, Eindeutigkeit (I6 aus 16.4) |
| Neu: Positivkontrollen (AP 3) | 16.4 |
| G-LINT, G-HUB (≤ 2 500 LOC), G-NOTOKIO, G-UNSAFE (kein `unsafe`), G-WORD | 16.6 |
| G3 (keine `SystemTime::now`/`thread_rng`) | noch nicht in `xtask` umgesetzt |

## 8. Befehle
```
cargo test -p acta-base --locked
cargo clippy -p acta-base --all-targets --locked -- -D warnings
cargo fmt --all -- --check
cargo xtask gates
```
