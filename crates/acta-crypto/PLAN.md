# Implementierungsplan: `acta-crypto`

> **Status:** verbindlich für die Crate-Überführung · **Paket:** 1 (L1)
> **Vorgaben:** [docs/spec/04_kryptografie.md](../../docs/spec/04_kryptografie.md) (4.1 bis 4.7) · [docs/spec/03_systemarchitektur.md](../../docs/spec/03_systemarchitektur.md) (3.7) · [docs/spec/anhang_a_parameter.md](../../docs/spec/anhang_a_parameter.md) · [CLAUDE.md](CLAUDE.md) · [archive/acta-crypto/MANIFEST.md](../../archive/acta-crypto/MANIFEST.md)

## Fortschritt (Nachtrag)
| AP | Stand |
|---|---|
| 1 | Entscheidungen offen, Vorschlag in `docs/ADR-VORSCHLAG-crypto.md` (keine Änderung an Ableitungen) |
| 2 | erledigt: `tests/fail_closed.rs` (Codes, Klassen, `T-RNG`, `T-NOFALLBACK`, kein `Erased`) |
| 3 | teilweise: übernommen sind die RFC-Vektoren (RFC 4231 Fälle 3 bis 7, RFC 5869 Fall 3) und die Verhaltensinvarianten; die übrigen Archivtests setzen die alte Passphrasen-API voraus und sind ohne Orakelwert für Umschlag v1 |
| 4 | ausgeführt (114 Mutanten, Randfälle geschlossen) |
| 5 | erledigt: `z14_no_shared_dek.rs`, `z2_no_plaintext.rs` (Positivkontrollen) |
| 6 | Merkmalsprüfung erweitert; `trybuild`-Compile-Fail offen |
| 7 | nicht ausgeführt, siehe ADR-Vorschlag (Archivteile für `acta-vault`/`acta-ledger`/`acta-reports` bleiben) |
| neu | `ScopeKey`, VKb, Tresor-Rahmen (4.3), Keyspaces (4.5) |

## 0. Verifikationsstand dieser Planung
- Ist-Stand laut `CLAUDE.md` des Crates und `pub`-Oberfläche. **Build und Tests wurden in dieser Planung nicht ausgeführt** (Toolchain 1.89.0 nicht verfügbar).
- Die Abweichungen in Abschnitt 5 sind aus CLAUDE.md und Spec-Text abgeleitet und vor Abschluss am Quelltext zu bestätigen.

## 1. Einordnung
- **Level:** 1. **Abhängigkeiten:** nur `acta-base` (Level 0). Keine Querkanten.
- **Budget:** 4 200 LOC Produktionscode (G-HUB). Ist: „per G-HUB geprüft“ laut CLAUDE.md.
- **Externe Abhängigkeiten (Whitelist):** `chacha20poly1305`, `hkdf`, `hmac`, `sha2`, `ed25519-dalek`, `zeroize`, `subtle`, `unicode-normalization`, `thiserror`.
- **Nicht hier:** `argon2` (Argon2id nur für `FileAnchor`, also `acta-vault`, L2). `rand` (nur `CryptoRng`-Port aus `acta-base`).

## 2. Ist-Stand (laut CLAUDE.md)
| Modul | Stand |
|---|---|
| `aead` | Umschlag v1 (XChaCha20-Poly1305), Nonce aus `CryptoRng`, Ebenen `Personal`/`Knowledge`/`Derived` als Typen, AAD mit Ebene, Mandant, Keyspace, Datensatz, Version. `SealKey` ist versiegelt. |
| `key` | `DataKey`, `SubjectKey`, `DocumentKey`, `SegmentKey`, `MasterKey`, `IndexSecret`, `LogKey`, `CommitKey`, `DedupKey`. Ohne `Default`/`Clone`/`Debug`/`Serialize`, `ZeroizeOnDrop`. |
| `kdf` | HKDF-SHA256; `DEK(v)` mit Sortierung und Dublettenprüfung; `HK(v)`, `DirKey`, `ClusterId`, `VK(t,g)`, `K_tag(t)`, `SubjectTag` (NFC), `blob_salt`, `commitment` |
| `pad` | Größenklassen nach 4.5 (Proptests) |
| `mac` | HMAC-SHA256, Konstantzeit-Vergleich |
| `hash` | SHA-256, `HashChain` (mehrere Instanzen) |
| `sign` | Ed25519 mit Domänenpräfix, `verify_strict` |
| `domain` | Alle 23 Präfixe aus 4.1, auf Eindeutigkeit geprüft |
| Tests | `rfc_vectors`, `envelope` (PyNaCl-Festwert), `padding`, `subject_tag`, `ledger_keys`, `sign`, `secret_types` (Compile-Zeit-Merkmale) |
| G-CRY | im `xtask` textbasiert umgesetzt (Grenze: Textmuster, keine Typprüfung) |

## 3. Spec-Bezug
- **Z2:** Jeder persistierte Klartext personenbezogener Daten liegt in einem Umschlag. Keine dauerhaften deterministischen Pseudonyme (`T-CRY-1`, `T-DERIVED-SEALED`, `T-META-*`).
- **Z10:** Fail-Closed. Kein Fallback-Schlüssel, kein Ersatzwert. Entropiefehler erzeugt weder Umschlag noch Schlüssel (`T-RNG-*`, `T-NOFALLBACK-*`).
- **Z14:** Zwei Versionen teilen niemals denselben DEK (Property-Test `no_shared_dek`).
- **Z15:** `HashChain`, Commitments und Fehler enthalten keine Roh-Kennungen.
- **4.5:** Die Nonce ist **kein** API-Parameter. `open` liefert `Zeroizing<Vec<u8>>` mit reservierter Kapazität. Jeder Fehler ohne unterscheidbares Orakel.
- **4.6:** Kein `SystemTime::now`, kein `thread_rng` im Kern.

## 4. Archiv-Entscheidungen

| Archivquelle | Entscheidung | Begründung |
|---|---|---|
| `crypto/src/aead.rs` | Nur Konzept | Umstieg auf XChaCha20-Poly1305. `aes-gcm-siv` ist nicht auf der Whitelist. |
| `crypto/src/crypto.rs` (1 041 Zeilen, 60 Panik-Stellen, `async`, `unsafe`) | Auseinanderlegen, nicht übernehmen | Teile landen in `aead`, `kdf`, `hash`, `sign`. `async` verboten (G-NOTOKIO), `unsafe` nur in `acta-sys`. |
| `crypto/src/dek.rs` | Logik nach `kdf` (bereits umgesetzt) | DEK-Formel ist in 4.2 festgelegt |
| `crypto/src/kdf.rs` | Verifizieren gegen 4.2, sonst übernommen | Argon2-Teil **nicht** übernehmen → `acta-vault` |
| `crypto/src/secret.rs` (`serde`-Ableitungen) | Nur Typen, ohne `serde`, `Default`, `Clone`, `Debug` | Z10, G-CRY |
| `crypto/src/anti_tamper.rs` (`unsafe`) | **Nicht** übernehmen | `unsafe` verboten außerhalb `acta-sys`. Schutzmaßnahmen gehören in `acta-sys`. |
| `crypto/src/audit_chain.rs` (`serde`) | Kettenlogik → `acta-ledger`; `HashChain` ist bereits hier | Ledger ist L2 und nutzt den Primitivtyp |
| `crypto/src/contract.rs` | Nur Vertragsform prüfen | Vertrag ist durch die Spec-Signaturen (3.7) bestimmt |
| `erasure/src/ed25519_proof.rs` | Nach `acta-reports` bzw. `acta-ledger` (Signatur der Berichte und Checkpoints) | `NotarySigner` ist hier schon vorhanden |
| `erasure/src/crypto_local.rs` | **Nicht** übernehmen | Lokale Kopie. Ersetzt durch `acta-crypto`. |
| `rand`, `blake3`, `bincode` | **Nicht** verwenden | Nicht auf der Whitelist (3.8) |

**Tests (Oracle):**
- `rfc_vectors` (RFC 5869, RFC 4231, RFC 8032): bereits übernommen (CLAUDE.md).
- `crypto_vectors_extended`, `proptests`, `crypto_property_and_invariants`, `key_separation_and_edge_cases`, `namespace_isolation`, `nonce_reuse`: prüfen, ob Negativaussagen eine Positivkontrolle haben (16.4). Mit `unwrap` gebaute Tests sind in Ordnung. Produktionscode nicht.
- `anti_tamper_matrix`, `audit_chain_tests`: Kettenlogik gehört zu `acta-ledger`. Dort neu aufsetzen.
- `kdf_argon2id`: nach `acta-vault`.
- `nonce_stress`: enthält `async` und Float. Nicht übernehmen. Die Nonce-Eindeutigkeit wird durch Eigenschaften und `CryptoRng` mit `FailingRng` geprüft.
- `golden_corpus`: Golden-Dateien (Magic-Bytes, `serde`, `std::fs`) **neu erzeugen**, nicht übernehmen (ALTLASTEN).

## 5. Spec-Konflikte und offene Entscheidungen (ADR-Kandidaten)

1. **Betroffene je Version: 255 statt 256.** Anhang A nennt für `max_subjects_per_doc` den Bereich 2 bis 256. Die DEK-Formel (4.2) zählt Betroffene als `n u8`, also höchstens 255. Der Code begrenzt auf `MAX_SUBJECTS_PER_VERSION = 255`. **Empfehlung:** Bereich in Anhang A auf 2 bis 255 setzen (Auftraggeber-Entscheidung, ADR).
2. **`K_dedup`-Ableitung (2.4).** Die Spec schreibt `HKDF(k_idx, "acta/dedup/v1" ‖ TenantId)` ohne Salz-/Info-Trennung. Umgesetzt ist das Muster von `K_tag(t)` (Salz = TenantId, Info = Präfix). Eine Änderung bricht die Blob-Salze und damit die Hash-Identität. **Klärung vor Freigabe nötig.**
3. **`seal`/`open` nehmen `K: SealKey` statt `&DataKey`** (Abweichung von 3.7). Begründung: `SegmentKey` und andere Schlüsselarten bräuchten sonst einen Umweg. Die Grenze bleibt: kein Schlüsselmaterial verlässt den Typ. **ADR festhalten.**
4. **Domänenpräfix der Signatur als `&[u8]`.** RFC-8032-Test 1 braucht die leere Domäne. Ein Aufzählungstyp wäre strenger, bräuchte aber eine Ausnahme für den Testvektor. Entscheidung: `&[u8]` beibehalten, Präfixe nur über die Konstanten in `domain` verwenden.
5. **`HashChain`-Eingabereihenfolge.** Die Spec legt sie nicht vollständig fest. Umgesetzt: Genesis-Vorgänger = Nullwert, Eintrag = SHA-256(LEDGER ‖ chain_id ‖ prev ‖ canonical). **Festlegung in 4.1 nachtragen** und Golden-Vektor dazu erzeugen.
6. **`Erased` vs. `DecryptionFailed`.** 4.5 verlangt `Erased`, wenn der Schlüssel nachweislich vernichtet ist. Diese Entscheidung kann `acta-crypto` nicht treffen, weil ihr Gate und Register fehlen. **Vorgabe:** `open` liefert nur `DecryptionFailed`. Der Aufrufer (`acta-gate`/`acta-erasure`) wandelt in `Erased` um, wenn ein Beleg vorliegt. Sonst ist es `Integrity` (Z10). Dies ist im Abschnitt der Fehlerklassen zu dokumentieren.
7. **Hybride Signaturen (4.7 Nr. 3).** Noch nicht vorgesehen. Kein Arbeitspaket, bis ein standardisiertes Verfahren festgelegt ist.

## 6. Arbeitspakete

**AP 1 – Spec-Konflikte entscheiden (Abschnitt 5, Punkte 1, 2, 5).** Ergebnisse als ADR oder Spec-Änderung festhalten. Keine Codeänderung an Ableitungen vor Entscheidung (Hash-Identität hängt daran).
- *Abnahme:* Jeder Punkt hat eine dokumentierte Entscheidung. `K_dedup` ist dokumentiert.

**AP 2 – Fehlerpfade und Fehlerklassen prüfen (Z10, 4.5).** `CryptoError` auf `ClassifiedError` abgleichen. Klassen: `Crypto` bei Schlüssel-/Zufallsfehlern, `Integrity` bei Fehlern ohne Schlüsselbeleg. Keine Varianten mit Nutzdaten (Z15).
- *Abnahme:* `code()` liefert feste Zeichenketten ohne Werte. Test `T-NOFALLBACK-*` mit Positivkontrolle (fehlender Schlüssel führt zu Fehler, nicht zu Ersatz).

**AP 3 – Archiv-Orakel übernehmen (ohne Verletzung der Crate-Grenze).** `rfc_vectors`, `crypto_vectors_extended`, `proptests`, `key_separation_and_edge_cases`, `namespace_isolation`, `nonce_reuse` nach `tests/` übernehmen. `nonce_stress` ohne `async` neu schreiben. Golden-Dateien neu erzeugen.
- *Abnahme:* Jeder Test hat ein unabhängiges Orakel (RFC, PyNaCl, Python-Referenz, Eigenschaft). Keine `unwrap`-Abhängigkeit im Produktionspfad.

**AP 4 – Mutationsprüfung.** `cargo-mutants` auf `aead`, `kdf`, `pad`. Überlebende Mutanten in Schlüsselableitung und AAD sind Defekte (16.2).
- *Abnahme:* Entfernen der Klassenbytes aus der AAD oder der Sortierung der Betroffenen lässt Tests scheitern (bereits belegt, im Ergebnis wiederholen).

**AP 5 – Z14 und Z2 belegen.** Property-Test `no_shared_dek` über `VerRef`. Test `T-CRY-1` (Bytes-Grep über erzeugte Dateien auf Klartexte und Kennungen).
- *Abnahme:* Beide Tests grün, mit Positivkontrolle (absichtlich eingebetteter Klartext wird gefunden).

**AP 6 – G-CRY-Grenze schließen.** Textmuster ergänzen durch Compile-Zeit-Prüfung (`tests/secret_types.rs` ist vorhanden). Ziel: keine `Default`/`Clone`/`Debug`/`Serialize`-Implementierung auf Geheimnistypen, geprüft per Trait-Assertion.
- *Abnahme:* Test scheitert, wenn ein Geheimnistyp `Clone` erhält.

**AP 7 – Archivordner entfernen.** Erst nach AP 1 bis 6 und grünen Gates: `archive/acta-crypto/` löschen. Argon2-, Anti-Tamper- und Ledger-Teile bleiben bis zur Überführung der Zielcrates (`acta-vault`, `acta-ledger`) erhalten; **der Ordner wird daher nicht vollständig entfernt**, sondern nach Zielcrate aufgeteilt. Entscheidung dazu im ADR.

## 7. Tests und Gates

| Test | Zusage / Gate |
|---|---|
| `rfc_vectors`, `envelope`, `sign`, `subject_tag`, `ledger_keys`, `padding` | 4.1, 4.5, 4.6 |
| `secret_types` | G-CRY, Z10 |
| `no_shared_dek` (neu) | Z14 |
| `T-CRY-1`, `T-DERIVED-SEALED`, `T-META-*` (neu) | Z2 |
| `T-NOFALLBACK-*`, `T-RNG-*` | Z10 |
| G-CRY, G-UNSAFE, G-LINT, G-HUB (≤ 4 200 LOC), G-WORD | 16.6 |

## 8. Befehle
```
cargo test -p acta-crypto --locked
cargo clippy -p acta-crypto --all-targets --locked -- -D warnings
cargo fmt --all -- --check
cargo xtask gates
```
