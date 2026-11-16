# Implementierungsplan: `acta-sys`

> **Status:** verbindlich für die Crate-Überführung · **Paket:** 1 (L1)
> **Vorgaben:** [docs/spec/04_kryptografie.md](../../docs/spec/04_kryptografie.md) (4.6) · [docs/spec/13_sicherheit_lieferkette.md](../../docs/spec/13_sicherheit_lieferkette.md) (13.1, 13.2) · [docs/spec/14_betrieb.md](../../docs/spec/14_betrieb.md) (14.1 Selbsttest) · [docs/spec/03_systemarchitektur.md](../../docs/spec/03_systemarchitektur.md) (3.1, 3.3, 3.5, 3.7) · [docs/spec/anhang_a_parameter.md](../../docs/spec/anhang_a_parameter.md) · [CLAUDE.md](CLAUDE.md) · [archive/acta-sys/MANIFEST.md](../../archive/acta-sys/MANIFEST.md)

## 0. Verifikationsstand dieser Planung
- Ist-Stand laut `CLAUDE.md` des Crates und `pub`-Oberfläche. **Build und Tests wurden in dieser Planung nicht ausgeführt** (Toolchain nicht verfügbar).
- Die Entscheidung zu SIMD (Abschnitt 4) betrifft den Umfang des Crates und braucht die Zustimmung des Auftraggebers.

## 1. Einordnung
- **Level:** 1. **Abhängigkeiten:** `acta-base`. Dev: `acta-testkit`, `proptest`.
- **Budget:** 1 600 LOC (G-HUB). Ist laut CLAUDE.md ca. 260 LOC.
- **Einziges Crate mit `unsafe`** (G-UNSAFE). Jeder Block trägt `// SAFETY:`.
- **Externe Abhängigkeiten (Whitelist):** `rustix` (nur Features `mm`, `process`, `rand`), `getrandom`, `landlock`, `seccompiler`, `zeroize`, `thiserror`. **Nicht** `libc`, `memmap2`, `tracing`.

## 2. Ist-Stand (laut CLAUDE.md)
| Modul | Stand |
|---|---|
| `secure_buf` | `SecureBuf`: je Puffer eigene, seitenausgerichtete anonyme Abbildung, `mlock` und `MADV_DONTDUMP` fail-closed, Nullung vor `munlock`/`munmap`. Feste Kapazität. Ohne `Default`/`Clone`/`Debug`/`Serialize`. Konstruktoren `zeroed`, `from_slice`, `random`. |
| `harden` | `harden_process`: `RLIMIT_CORE = 0`, `PR_SET_DUMPABLE = 0`; `memlock_soft_limit` für den Selbsttest |
| `clock` | `SystemClock` (Port `Clock`) |
| `rng` | `OsCryptoRng` (Port `CryptoRng`, Quelle `getrandom`) |
| `error` | `SysError` mit Klassen `Policy`, `Crypto`, `Invalid`; Codes ohne Werte |
| `host` | `check_swap`, `check_core_pattern`, `check_ptrace_scope`, `check_memlock`: reine Auswertung, fail-closed bei unlesbaren Werten |
| Tests | `secure_buf`, `harden`, `mlock_control` (Positivkontrolle über `VmLck`), Einheitstests in `error.rs` |

## 3. Spec-Bezug
- **4.6:** Schlüsselmaterial liegt nur in `SecureBuf`. Entropiefehler liefert `Err`, nie einen Ersatzpuffer (Z10, `T-RNG-*`).
- **13.1:** `mlock`, `MADV_DONTDUMP`, `RLIMIT_CORE = 0`, `PR_SET_DUMPABLE = 0`, `ptrace` nur bei `ptrace_scope ≥ 1`. Der Selbsttest prüft Swap, Core-Pattern, `RLIMIT_MEMLOCK`, `O_DIRECT`, Dateisystemtyp.
- **13.2:** Worker-Sandbox: Landlock ohne Dateizugriff, seccomp-Positivliste, kein Netz, rlimits, `PR_SET_NO_NEW_PRIVS`. **Noch nicht begonnen.**
- **3.5:** `SysError` gehört zu den Fehlerklassen. Keine Werte in Meldungen (Z15).
- **3.7:** `SystemClock` implementiert `Clock`. `now_ms` ist ohne Fehler definiert.

## 4. Archiv-Entscheidungen

| Archivquelle | Entscheidung | Begründung |
|---|---|---|
| `vault/src/vault.rs`, `sys/src/vault.rs` | Nur Konzept (`SecureBuf`) | Bereits umgesetzt, mit eigener Seitenabbildung je Puffer |
| `sys/src/mlock.rs`, Richtlinie `Tolerant` | **Nicht** übernehmen | Warnung statt Fehler verletzt Z10 (kein Fallback). Umgesetzt fail-closed. |
| `sys/src/mmap.rs` (`std::fs`, `unsafe`) | Nur die `mmap`-Logik für `SecureBuf` | `memmap2` nicht auf der Whitelist; umgesetzt über `rustix::mm` |
| `sys/src/posix.rs` (`unsafe`, `std::fs`) | Prüfen, Teile nach `host` | Keine `libc`-Abhängigkeit. Dateizugriffe nur in `acta-fs`. |
| `sys/src/durability.rs` (`fsync`, `std::fs`, 15 Stellen) | **Nicht** hier | Doppelung zu `acta-fs` (`DurableVfs`). Durabilität gehört dorthin (G-FS). |
| `sys/src/acl_win32.rs` | **Verwerfen** | Plattform ist Linux (1.2) |
| `sys/src/contract.rs` | Nur Vertragsform prüfen | Verträge ergeben sich aus 3.7 |
| `sys/tests/error_paths.rs`, `integration_tests.rs` | Übernehmen, `std::fs` nur für `/proc`-Lesen (siehe Abschnitt 5) | Orakel für Fehlerpfade |
| `simd/` (ca. 3 000 Zeilen, `float` in 360 Stellen, `unsafe` in 316 Stellen) | **Nicht übernehmen** (siehe Abschnitt 5, Punkt 1) | Budget, Z7, Default-deny |
| `simd/tests/`, `simd/benches/` | Gleiche Entscheidung wie `simd/` | |

**Begründung der Abhängigkeitsentscheidung:** `libc` und `memmap2` sind nicht auf der Whitelist (3.8) und würden erst per ADR aufgenommen.

## 5. Offene Punkte und Spec-Konflikte

1. **SIMD-Kerne (`archive/acta-sys/simd/`) sind nicht in SPEC 3.3 vorgesehen.** 3.3 nennt für `acta-sys` ausschließlich Speicherhärtung, Sandbox, Hostprüfungen und die Ports. Der Archiv-Ansatz ist mit dem Budget (1 600 LOC) nicht vereinbar: Der SIMD-Code allein umfasst ca. 3 000 Zeilen Produktionscode. Außerdem nutzt er Gleitkomma, was Z7 für Ranking verbietet. **Vorgehen (Default-deny, 0.4 Regel 1):** Kein SIMD in Paket 1. `acta-vec` nutzt die skalare Ganzzahl-Implementierung (`i8`-Skalarprodukt ist exakt und plattformunabhängig). Eine Beschleunigung braucht einen ADR mit Messbericht (16.5) und ein eigenes Crate oder eine Budgeterhöhung.
2. **Dateisystemtyp und `O_DIRECT`-Prüfung (13.1).** Fehlt. Die Prüfung braucht `statfs`, also Dateizugriff. Die Abfrage gehört in `acta-fs` (siehe `acta-fs`/AP 4), `acta-sys` liefert nur die Auswertung.
3. **Sandbox (13.2) für `acta-parse-worker`.** Noch nicht begonnen. Umsetzung in `acta-sys` als Hostbaustein mit `landlock` und `seccompiler`. Diese beiden Crates stehen auf der Whitelist (Sandbox). Die Positivliste der Syscalls ist per Test festzuhalten.
4. **`SystemClock` bildet Zeiten vor 1970 auf 0 ab.** `Clock::now_ms` ist fehlerfrei (3.7). Ein Rücksprung auf 0 würde vom Knoten als Uhrenrücksprung erkannt (A-CLOCK, `clock_skew_max`). Die Abbildung ist dokumentiert. **Prüfen**, ob der Knoten den Rücksprung zuverlässig erkennt, sobald er auf 0 fällt. Eventuell Port-Vertrag ändern (ADR).
5. **Test liest `/proc/self/status` mit `std::fs`.** G-FS erlaubt Dateizugriffe nur in `acta-fs`. Lösung wie in `acta-fs`/Punkt 5 festlegen: Port aus `acta-fs` **oder** ADR-Ausnahme nur für `tests/`.
6. **Gate G3 nicht umgesetzt.** `SystemClock` nutzt `SystemTime::now` als einzige erlaubte Stelle. G3 muss diese eine Ausnahme ausdrücklich zulassen und alle anderen Aufrufe verbieten.
7. **Seitengröße.** 4096 wird als Annahme [A-PAGE] geführt. `rustix::param` bräuchte das Feature `param`, das nicht auf der Whitelist steht. Im Bericht ausweisen.

## 6. Arbeitspakete

**AP 1 – SIMD-Entscheidung festhalten.** ADR anlegen: SIMD wird in Paket 1 nicht übernommen. Begründung: Budget, Z7, 3.3, Default-deny. Auftraggeber bestätigt oder verwirft.
- *Abnahme:* ADR liegt vor. Archiv-Ordner `simd/` bleibt bis zur Entscheidung im Archiv, wird aber nicht überführt.

**AP 2 – Hostprüfungen zum Selbsttest (14.1).** `check_swap`, `check_core_pattern`, `check_ptrace_scope`, `check_memlock` gegen 13.1 prüfen. Die Auswertung ist rein. Das Lesen der Werte läuft im Knoten über `acta-fs` (L7).
- *Abnahme:* Jede Prüfung ist fail-closed bei unlesbarem Wert. Tests mit synthetischen Eingaben (keine echten Systemwerte).

**AP 3 – Sandbox für den Worker (13.2).** Modul `sandbox` mit Landlock-Regeln (kein Dateizugriff), seccomp-Positivliste, `PR_SET_NO_NEW_PRIVS`, rlimits (CPU, Speicher, Dateien). Nur Konstruktion und Anwendung; der Aufruf erfolgt im Worker (L9).
- *Abnahme:* Positivkontrolle: Der Worker kann ohne Erlaubnis keine Datei öffnen und keinen Socket anlegen. Ohne Landlock-Unterstützung scheitert der Start (fail-closed).

**AP 4 – `SecureBuf`-Orakel übernehmen.** `mlock_control` (Positivkontrolle über `VmLck`) und `secure_buf` bleiben. Test-Lesezugriff gemäß Punkt 5 festlegen.
- *Abnahme:* Negativaussage „Puffer ist gesperrt und nullgesetzt“ hat Anwesenheitsnachweis (16.4).

**AP 5 – Fehlerpfade (Z10, Z15).** `SysError`-Codes ohne Werte. Klassen `Policy` (Host- und Speicherverweigerung), `Crypto` (Entropie), `Invalid` (unlesbare Prüfwerte).
- *Abnahme:* `code()` liefert feste Zeichenketten. Entropiefehler erzeugt weder Puffer noch Schlüssel (`T-RNG-*`).

**AP 6 – Gates.** G3 erweitern (eine erlaubte Stelle). G-UNSAFE prüfen: nur `acta-sys`, jeder Block mit `// SAFETY:`.
- *Abnahme:* `cargo xtask gates` grün, G3 verbietet alle Aufrufe außer `SystemClock`.

**AP 7 – Archivordner.** `archive/acta-sys/` erst entfernen, wenn Sandbox (AP 3) und Hostprüfungen (AP 2) überführt sind **und** die SIMD-Entscheidung (AP 1) getroffen ist. Sonst bleibt der Ordner bestehen (CLAUDE.md, Abschnitt zu `acta-sys`).

## 7. Tests und Gates

| Test | Zusage / Gate |
|---|---|
| `secure_buf`, `mlock_control`, `harden` | 4.6, 13.1, Z10 |
| `error`-Einheitstests | Z15 |
| Neu: `host` mit synthetischen Eingaben (AP 2) | 13.1, 14.1 |
| Neu: `sandbox`-Positivkontrolle (AP 3) | 13.2 |
| `T-RNG-*` | Z10 |
| G-UNSAFE, G-FS, G-NOTOKIO, G3, G-LINT, G-HUB (≤ 1 600 LOC) | 16.6 |

## 8. Befehle
```
cargo test -p acta-sys --locked
cargo clippy -p acta-sys --all-targets --locked -- -D warnings
cargo fmt --all -- --check
cargo xtask gates
```
