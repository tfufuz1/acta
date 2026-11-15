# acta-crypto — Crate-Spezifikation (Fassung 3.0 Final)

*Bezug: Normative Gesamtspezifikation [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) (Teil IV, V)*  
*Ring: R0 (Fundament) · Geltung: Verbindlich für `acta-crypto`*

---

## 1. Zweck und Einordnung

`acta-crypto` stellt die kryptografischen Kernprimitive, Schlüsselableitungsfunktionen und den authentifizierten Datensatz-Umschlag bereit:
- **Datensatz-AEAD:** XChaCha20-Poly1305 mit 192-Bit-Zufallsnonce aus `Rng` (Invariante I-15).
- **Hierarchische DEK-Ableitung (H1–H5):** HKDF-SHA256 über Ancestor-Salts (Mandant, Collection, Subject, Dokument).
- **Datensatz-Umschlag v3:** 
  $$\text{[ver: 0x03][alg: 0x01][doc\_tag: 16B][nonce: 24B][ciphertext][tag: 16B]}$$
  mit $\text{AAD} = \text{"acta/rec/v3"} \parallel \text{tenant\_tag} \parallel \text{class\_u8} \parallel \text{keyspace\_u16} \parallel \text{user\_key} \parallel \text{seq\_u64be}$.
- **Integrität & Commitments:** HMAC-SHA256 für WAL-Ketten und pseudonymitätsfreie Commitments ($K_{\text{commit}}[e]$).
- **Passwortbasierte KDF:** Argon2id ($m \ge 19\,456\text{ KiB}, t \ge 2, p \ge 1$).
- **Signatur:** Ed25519 für Prüfberichte mit Domänentrennung `ctx/report/v1`.
- **Sicherheitscontainer:** Zeroize-on-Drop für alle geheimen Schlüssel; Verbot von `Debug`-Leaks.

---

## 2. Architektur und Verträge

| Merkmal | Vorgabe (docs/SPEC.md §3.1) |
|---|---|
| **Ring** | **R0 (Fundament)** |
| **Erlaubte interne Abhängigkeiten** | `acta-ids`, `acta-env` (beide Ring R0) |
| **Default-Features** | `[]` (leer) |
| **Fehlertyp** | `CryptoError` (thiserror, Fail-Closed) |
| **Zero-Panic & Lints** | `#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::let_underscore_must_use)]` |

---

## 3. Normative Anforderungen und Defektbehebungen

1. **Beseitigung von V-01 und V-05 (Fail-Closed Doktrin, Zusage S7):**
   - Striktes Verbot von Fallback-Schlüsseln: Schlägt `derive_dek` fehl, MUSS ein Fehler zurückgegeben werden. Der fehlerhafte Rückfall auf `blake3::keyed_hash(&k_unit, key)` ist vollständig entfernt.
   - Entfernung sämtlicher Hardcoded-Schlüssel (`[0x55; 32]`, `[0x42; 32]`, Nullschlüssel `[0u8; 32]`).
   - `SecretKey` und `Dek` implementieren **kein** `Default`.
2. **Beseitigung von V-02 (AEAD-Umschlag v3):**
   - Datensatz-Verschlüsselung erfolgt ausschließlich über XChaCha20-Poly1305.
   - Nonce-Generierung erfolgt intern über den `Rng`-Port. Die öffentliche API bietet keinen Parameter für externe Nonces mehr (`seal_record(ctx, key, plain)`).
3. **Invariante I-17 (Container-Isolation):**
   - Jeder Datensatz in Ebene P besitzt einen eigenen Umschlag unter seinem individuellen Dokument-DEK.
   - Persistierte Blöcke oder WAL-Frames teilen niemals einen Schlüssel über verschiedene Dokumente hinweg.
4. **Schlüsselhierarchie (Regeln H1 bis H5):**
   - Slots $T, C, S, D$ sind physisch getrennt. Fehlt $S$ bei Dokumenten mit Betroffenenbezug, bricht der Pfad mit einem Fehler ab.

---

## 4. Tests und Abnahme

- **Pflichttest V-02:** Manipulation jedes einzelnen Bytes im Chiffretext führt zu `DecryptionFailed`. Zwei Chiffrate unter gleichem DEK teilen keinen XOR-Strom.
- **Pflichttest V-05:** Keine Verwendung von `unwrap_or_default()` im gesamten Kryptopfad.
- Testvektoren für Umschlag v3 und HKDF-Hierarchie.
- Gate `G-CRY-1`: Lint-basiertes Verbot von Krypto-Fallbacks.
