# Implementierungsplan: `acta-pstore`

> **Status:** verbindlich für die Crate-Überführung · **Paket:** 3 (L3)
> **Vorgaben:** [docs/spec/05_speicher.md](../../docs/spec/05_speicher.md) (5.4, 5.5) · [docs/spec/04_kryptografie.md](../../docs/spec/04_kryptografie.md) (4.2, 4.5) · [docs/spec/02_grundmodell.md](../../docs/spec/02_grundmodell.md) (2.2 bis 2.4) · [docs/spec/08_loeschkern.md](../../docs/spec/08_loeschkern.md) (8.2, 8.3 S3, 8.9) · [docs/spec/09_ingest_abruf_antwort.md](../../docs/spec/09_ingest_abruf_antwort.md) (9.1 Stufen 5/8, 9.4 `A_P`) · [CLAUDE.md](CLAUDE.md)

## 0. Verifikationsstand
- Ist-Stand: **Gerüst** (17 LOC). **Kein Archivbestand** (CLAUDE.md: „Archiv-Quelle: keine“). Der gesamte Crate ist Neubau nach Spec.
- Nichts ausgeführt (Toolchain nicht verfügbar).

## 1. Einordnung
- **Level:** 3. **Abhängigkeiten (SPEC 3.3):** `acta-base`, `acta-crypto`, `acta-fs`, `acta-gate`, `acta-ledger`, `acta-vault`, `acta-seg`.
- **Budget:** 5 000 LOC (G-HUB).
- **Externe Abhängigkeiten:** `roaring`, `thiserror`.
- **Rolle:** Ebene P: Dokumentköpfe, Versionen, Betroffenenverzeichnis, Handle-Register, Vorgänge, Fallindex (DP), Auflösung von Schlüsseln und DEK.

## 2. Ist-Stand
- Nur `PstoreError`. Keine Logik.

## 3. Spec-Bezug
- **Dokument-Version (5.4):** (a) Dokumentkopf unter `HK(v) = HKDF(DocumentKey(v), salt = VerRef, "acta/hdr/v1")`, (b) Inhaltsrecords unter `DEK(v)`, (c) Ableitungen in DP. `DocRef` ist stabile Identität, die aktuelle Version steht im Handle-Register.
- **Betroffenenverzeichnis:** je Betroffenem ein Datensatz unter `cluster_id`, versiegelt unter `DirKey(s)`. Inhalt: `SubjOrd`, `subject_kind` (`External`, `Staff`, `Synthetic`), Aliase, `(DocRef, VerRef, VerOrd, Rolle main|co, Zustand)`, `ActorRef`-Handles, `DisclosureLog`, Anfragen mit `identity_verified` und `due_at`.
- **Auffinden (5.4):** externe Kennung → `SubjectTag` → Tresoreintrag → `SubjectKey` (bei Alias: Primär-Tag) → `ClusterId` → Verzeichnis.
- **Handle-Register:** `PersonalRef → (DocRef, VerRef, VerOrd, ClusterId)`, versiegelt unter `RegistryKey(t) = HKDF(VK(t,g), "acta/gen/v1")`. Mit jeder Rotation neu versiegelt. Bei S3 entfernt.
- **Vorgang (`Case`):** `CaseId`, `external_ref`, Mitglieds-`VerRef`s, zugewiesene `ActorRef`s, Status. DEK mit `n = 0`. Fallindex als Roaring-Bitmap je `CaseId` (DP-Segment mit Lineage).
- **Idempotenztabelle:** `ingest_id → (VerRef, Ergebnis)` je Mandant, versiegelt (Keyspace `0x0070`), TTL `ingest_idem_ttl` = 7 Tage.
- **Ingest-Reihenfolge (9.1 Stufe 8):** Schlüssel durabel (WP-1) → P-Umschläge → DP-Segmente → **zuletzt** Manifest (Sichtbarkeit, WP-3).
- **Z2:** Kein `SubjectTag` im Klartext. Verzeichnis und Register nur verschlüsselt.
- **Z14:** Zwei Versionen teilen nie denselben DEK.
- **Z16:** `A_P` wird über Fallindex-Bitmaps berechnet, nur für offene, zugewiesene Vorgänge und nur für die in der Anfrage genannten `case_id`s (≤ `max_cases_per_query` = 5). Ohne `case_id` keine P-Inhalte.
- **R-CASE:** Das Aktenzeichen im Vorgang ist pseudonym bis zur Vernichtung. Restrisiko ausweisen.

## 4. Archiv-Entscheidungen
- **Keine Vorlage im Archiv.** Konzepte aus `archive/acta-erasure/` (Löschkern) sind Verbraucherseite und gehören nicht hierher. Das Handle-Register und das Betroffenenverzeichnis wurden im Archiv nicht als eigenes Modul geführt.
- **Verboten:** Die Altformate von Verzeichnis und Register dürfen nicht übernommen werden, auch nicht Teile von `serde`-Darstellungen.

## 5. Offene Punkte und Spec-Lücken

1. **Neubau-Umfang.** 5 000 LOC Budget für einen vollständig neuen Crate. Nach dem Entwurf Schätzung nachholen; bei Überschreitung ADR.
2. **Vorgang-Schließen und Vernichtung.** 5.4 sagt: Vorgang endet bei Schließen oder wenn alle Mitglieder erledigt sind. Die Zustandsübergänge in Tabellenform festlegen (fehlt in der Spec).
3. **Fallindex-Pflege.** Neue Versionen erweitern den Fallindex (L04). Ob das synchron im Ingest geschieht, ist offen. WP-3 verlangt Sichtbarkeit zuletzt.
4. **`ActorRef`-Epoche (`actor_handle_epoch`).** Startwert 12 Monate ist in Monaten angegeben. Die Umrechnung auf Millisekunden und die Epochengrenzen festlegen (Lücke in A.5).

## 6. Arbeitspakete

**AP 1 – Typen und Kennungen.** `DocRef`, `VerRef`, `VerOrd`, `SubjOrd`, `CaseId`, `PersonalRef`, `ActorRef` als Newtypes. Nutzung der Typen aus `acta-base`, Zuordnung nach HR-2.
- *Abnahme:* Keine Typen-Dubletten zu `acta-base`. Kein Typ mit `Clone`/`Debug` auf Schlüsselmaterial.

**AP 2 – Dokumentkopf und Versionen.** Kopf unter `HK(v)`, Inhaltsrecords unter `DEK(v)`. `VerOrd` vergeben, Ordinale monoton.
- *Abnahme:* Z14-Eigenschaftstest `no_shared_dek`. I6: Ordinale nie wiederverwendet.

**AP 3 – Betroffenenverzeichnis und Auffinden.** Verzeichnis unter `DirKey(s)`. Auffinden-Pfad aus 5.4.
- *Abnahme:* Z2-Test: keine Kennung im Klartext in Verzeichnisdateien. Alias-Auflösung zu Primär-Tag getestet.

**AP 4 – Handle-Register und Rotation.** Versiegelung unter `RegistryKey(t)`, Neuversiegelung bei Rotation (4.3).
- *Abnahme:* Nach Rotation sind alte Register-Umschläge nicht mehr lesbar (Schlüssel vernichtet), neue lesbar.

**AP 5 – Vorgänge und Fallindex.** Zustandsübergänge aus Punkt 5.2. Fallindex als DP-Segment mit Lineage (`Derived<T>`, Z13).
- *Abnahme:* Compile-Fail-Test `derived_requires_lineage`. `A_P`-Berechnung nur über zugewiesene offene Vorgänge (`T-ACL-CASE`).

**AP 6 – Idempotenz.** Tabelle `ingest_id`, TTL 7 Tage.
- *Abnahme:* Wiederholung mit gleicher `ingest_id` liefert dasselbe Ergebnis (L01).

**AP 7 – Crash-Punkte.** C01, C02, C03, C04 für den Ingest-Pfad.
- *Abnahme:* `T-CRASH-*` mit Positivkontrolle.

**AP 8 – Gates und Archivordner.** G-CRY, G-LINT, G-HUB (≤ 5 000 LOC). Kein Archivordner zu entfernen.

## 7. Tests und Gates

| Test | Zusage / Gate |
|---|---|
| `no_shared_dek` | Z14 |
| `T-CRY-1`, `T-META-*` | Z2 |
| `T-ACL-CASE`, `T-CASE-*` | Z16 |
| `derived_requires_lineage` (trybuild) | Z13 |
| `T-CRASH-*` für C01 bis C04 | Z4 |
| `T-RET-*` (Idempotenzfristen) | 15.4 |
| G-CRY, G-LINT, G-HUB | 16.6 |

## 8. Befehle
```
cargo test -p acta-pstore --locked
cargo clippy -p acta-pstore --all-targets --locked -- -D warnings
cargo fmt --all -- --check
cargo xtask gates
```
