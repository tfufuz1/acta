# Implementierungsplan: `acta-history`

> **Status:** verbindlich für die Crate-Überführung · **Paket:** 3 (L3)
> **Vorgaben:** [docs/spec/06_wissensebene.md](../../docs/spec/06_wissensebene.md) (6.1 bis 6.7) · [docs/spec/08_loeschkern.md](../../docs/spec/08_loeschkern.md) (8.1 Takedown, 8.8) · [docs/spec/03_systemarchitektur.md](../../docs/spec/03_systemarchitektur.md) (3.4, 3.6) · [docs/spec/04_kryptografie.md](../../docs/spec/04_kryptografie.md) (4.2 `LogKey`, `RegistryKey`) · [CLAUDE.md](CLAUDE.md) · [archive/acta-history/MANIFEST.md](../../archive/acta-history/MANIFEST.md)

## 0. Verifikationsstand
- Ist-Stand: **Gerüst** (17 LOC). Nichts ausgeführt.
- Das Archiv liefert **nur Referenzen** (`mvcc`: `tx_seq`, Schnappschuss-Registry; `checkpoint`: Zeitreise-Muster). Das Git-Objektmodell 6.1 ist Neubau.

## 1. Einordnung
- **Level:** 3. **Abhängigkeiten (SPEC 3.3):** `acta-base`, `acta-crypto`, `acta-pii`, `acta-ledger`, `acta-vault`, `acta-seg`, `acta-gate`.
- **Budget:** 6 500 LOC (G-HUB).
- **Externe Abhängigkeiten:** `roaring`, `sha2`, `thiserror`. **Nicht:** `ahash`, `blake3`, `bytes`, `parking_lot`, `tracing`.
- **Rolle:** Versionskontrolle der Ebene K („Git für RAG“): Objekte, Refs, Sichtbarkeitsmengen, Diff, Merge, Takedown.

## 2. Ist-Stand
- Nur `HistoryError`. Archiv-Umfang (`mvcc`, `checkpoint`) wird nur als Referenz gelesen. Alte Datei-Formate und `serde`-Ableitungen werden nicht übernommen (7 Treffer).

## 3. Spec-Bezug
- **Objekte (6.1):** Blob, Tree, Commit, Ref. Inhaltsadressiert mit `ObjectId = SHA-256(Präfix ‖ kanonische Bytes)`.
- **Blob-Salz:** `salt = HMAC-SHA256(K_dedup(t), Inhalt)[..16]`. Gleicher Inhalt im Mandanten ergibt dieselbe `ObjectId`. Ohne `K_dedup` nicht prüfbar. Takedown entfernt Inhalt **und** Salz.
- **Tree:** Einträge `{name, kind, id, meta}`, `kind ∈ {Blob 0x01, Tree 0x02, PersonalRef 0x03}`. Bei `PersonalRef` ist `id` ein 16-Byte-Handle, nie ein Inhaltshash. Namen NFC, ohne NUL und `/`, höchstens 255 Byte.
- **Commit:** `{tree, parents[], tx_seq, tx_time, valid_from?, valid_to?, author: ActorRef, kind, note_tag, flags}`. `kind ∈ {Ingest, Edit, Merge, Config, Takedown, Release}`. `note_tag` ist `u16` aus Whitelist. Freitext nie im Commit.
- **Zeitachsen (6.3):** `valid` (Kalendertage), `tx` (`tx_seq` dicht, `tx_time` nicht fallend), `erase` (nur E-Ledger und Gate). `tx_time = max(Clock::now(), Vorgänger)`, geklemmt mit Flag `clamped`. Rücksprung über `clock_skew_max` ist `Poisoned(Node, ClockRegression)`.
- **Operationen (6.4):** `commit` (Pflicht `Validated<Tree>`, Schreibreihenfolge Objekte → K-Ledger → Ref, WP-2), `branch`, `tag`, `checkout(as_of, valid_at)`, `diff`, `merge` (Vier-Augen für `main`, Regressionsprüfung optional), `retire`.
- **Sichtbarkeitsmengen (6.5):** `V(commit)` als Roaring-Treemap der `KOrd`, Basismenge plus Deltas, Schnappschuss alle 256 Commits. Suche filtert mit `V ∩ A` auf globalen DK-Segmenten.
- **Ebene P:** Keine Zeitreise. Nur kurze `Superseded`-Versionen bis `history_horizon`. Löschungen bereinigen sofort.
- **K-Ledger:** Änderungen durch Löschung in P: **nie** (Z6).
- **Takedown (6.7, 8.1):** Vier-Augen, signiert. Sofort `RedactK(KOrd)` im Gate-Log. DK-Segmente ersetzen. Pack ohne Blob inklusive Salz neu schreiben, `SegmentKey` vernichten. K-Ledger-Vermerk `Redacted`. Commitment im E-Ledger.
- **Zustandsübergänge** `Expired` nach `decision_retention`: Vermerk im K-Ledger, Ersatzsegment, kein Vier-Augen-Schritt.
- **Z5:** K enthält keine Personendaten. `Validated<T>` ist Pflicht vor `commit` (7.5).
- **Z7:** Abfragen zu Stand und Zeitpunkt sind deterministisch. Sortierung und Gleichstand gemäß 9.2.

## 4. Archiv-Entscheidungen

| Archivquelle | Entscheidung | Begründung |
|---|---|---|
| `mvcc/*` (Schnappschuss, `tx_seq`) | Nur Referenz | Mehrversionen-Transaktionen sind laut `NICHT_EXTRAHIERT.md` ausgeschlossen. Ein Schreiber je Mandant (3.6). |
| `checkpoint/*` (`manifest.rs`, `meta.rs`: `legacy`-JSON, Manifest-Pruefsumme `legacy_expected`) | Verwerfen | ALTLASTEN: Acta hat keine Altformate (14.8, Z10) |
| `serde`-Ableitungen (7 Treffer) | Verwerfen, handkodieren | G-NOSERDE-DISK |
| `async` (90 Treffer) | Verwerfen | G-NOTOKIO |
| `ahash`, `blake3`, `bytes`, `parking_lot`, `tracing` | Verwerfen | Whitelist |
| `liste_a` (2 Treffer, verbotene Formulierung) | Entfernen | G-WORD (1.6) |
| `alt_spec` (2 Treffer) | Durch SPEC-Verweise ersetzen | ALTLASTEN |

## 5. Offene Punkte und Spec-Lücken

1. **Git-Objektmodell ist Neubau.** Die Spec legt Objekte, Refs und Commits fest, aber nicht die Speicherstruktur (Pack-Format, Referenzzähler). Vor AP 2 Format als ADR festhalten.
2. **`K_dedup`-Ableitung** (Konflikt aus `acta-crypto`, Punkt 2). Blob-Salz hängt davon ab. **Nicht implementieren, bevor geklärt.**
3. **Sichtbarkeitsmengen-Schnappschüsse** alle 256 Commits: Das Intervall ist Festwert in 6.5, aber kein Anhang-A-Parameter. Als Konstante festhalten oder als Parameter aufnehmen.
4. **`retire` und `branch_retention`.** Erreichbarkeit und GC (6.7) braucht eine Traversierung über Refs und `decision_retention`. Die Abhängigkeit zu `acta-erasure` (Entscheidungsprotokolle) liegt höher (L4). Hier nur die Schnittstelle, die `acta-erasure` aufruft.
5. **`ArchiveHook`** (8.4) liegt im Level darüber. Trait nach HR-2 bei Verbraucher prüfen.

## 6. Arbeitspakete

**AP 1 – Objektkodierung.** Blob, Tree, Commit, Ref mit kanonischen Bytes nach 6.1 (handkodiert). `ObjectId`-Berechnung.
- *Abnahme:* Golden-Vektoren aus unabhängiger Referenz. Gleiche Eingabe ergibt gleiche ID.

**AP 2 – Blob-Salz und Idempotenz.** `salt` aus `K_dedup` (nach Klärung Punkt 2). Gleichen Inhalt zweimal committen ergibt dieselbe `ObjectId`.
- *Abnahme:* Test mit Positivkontrolle (anderer Mandant erzeugt andere ID). Z5-Test.

**AP 3 – Commit und Zeitachsen.** `tx_seq`, `tx_time` mit Klemmung und Flag `clamped`, `valid_from`/`valid_to`. Rücksprung führt zu `Poisoned(Node)`.
- *Abnahme:* Test mit `ManualClock`: Rücksprung über `clock_skew_max` wird gemeldet. Z7-Test `T-TT-*` für `as_of`-Abbildung.

**AP 4 – Refs, Branches, Diff, Merge.** Drei-Wege-Merge, Konflikte pro Pfad, `PersonalRef` nach Handle verglichen. Vier-Augen-Gate für `main`.
- *Abnahme:* Merge-Tests mit Konfliktfällen. Vier-Augen-Pflicht getestet.

**AP 5 – Sichtbarkeitsmengen.** Roaring-Basis plus Deltas, Schnappschuss alle 256 Commits. Kein Index je Branch.
- *Abnahme:* Z7-Test: Gleiche Abfrage liefert identische Ergebnisse über `V`. `T-TT-*` über x86_64 und aarch64 (Plattformtest, A-Lücke).

**AP 6 – Takedown (6.7, 8.1).** `RedactK` im Gate-Log, Ersatz des Packs inklusive Salz, `SegmentKey`-Vernichtung. Crash-Punkt C22.
- *Abnahme:* `T-TT-REDACTED`, `T-CRASH-TAKEDOWN`. Nach Takedown ist der Inhalt nicht mehr lesbar, die Objekt-ID bleibt in der Kette.

**AP 7 – Erreichbarkeit und GC.** Traversierung über Refs und Entscheidungsprotokolle, Bereinigung nach `branch_retention`. Schnittstelle für `acta-erasure`.
- *Abnahme:* Test: unerreichbare Objekte werden entfernt, erreichbare bleiben.

**AP 8 – Archivordner und Gates.** `archive/acta-history/` nach Entfernung der Altformate und Abschluss von AP 1 bis 7. G-NOSERDE-DISK, G-NOTOKIO, G-LINT, G-WORD, G-HUB (≤ 6 500 LOC).

## 7. Tests und Gates

| Test | Zusage / Gate |
|---|---|
| Golden-Objekte (AP 1) | 6.1 |
| Z5-Test (keine Personendaten in K) | Z5, `T-SEP-*` |
| `T-VCS-ERASE-1` (Objekt-IDs, Commit-Hashes unverändert nach P-Löschung) | Z6 |
| `T-TT-*`, `T-TT-REDACTED` | Z7 |
| `T-CRASH-TAKEDOWN` (C22) | Z4 |
| `T-DEC-*` (Begründungskette) | Z8 |
| `T-SCEN-*` (Branch-Isolation) | Z9 |
| G-NOSERDE-DISK, G-NOTOKIO, G-WORD, G-LINT, G-HUB | 16.6 |

## 8. Befehle
```
cargo test -p acta-history --locked
cargo clippy -p acta-history --all-targets --locked -- -D warnings
cargo fmt --all -- --check
cargo xtask gates
```
