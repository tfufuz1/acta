# ACTA – Spezifikation v0.1

**Fundament für Zeitreisen (Versionierung) und Löschung (Crypto-Shredding) in einem lokalen Wissens- und Gedächtnissystem für Unternehmen und LLM-Agenten**

| | |
|---|---|
| Status | Entwurf zur Freigabe (Architekturentscheidung) |
| Zielort im Repo | `acta/docs/SPEC.md` |
| Sprache / Toolchain | Rust, Edition 2021, MSRV 1.89 (Workspace bereits so gepinnt) |
| Quellen | Repos `tfufuz1/contextra`, `tfufuz1/acta`; die vier Löschvarianten aus *Crypto-Shredding_RAG_Rechts-und-Architektursynthese.md* |
| Normative Wörter | **MUSS**, **DARF NICHT**, **SOLL**, **KANN** (im Sinne von RFC 2119) |
| Rechtlicher Vorbehalt | Die DSGVO-Bezüge sind technische Auslegung, keine Rechtsberatung. Vor Produktivbetrieb ist eine juristische Prüfung der Aufbewahrungs- und Nachweislogik nötig (siehe §21). |

---

## Inhaltsverzeichnis

1. Zielbild und Designprinzipien
2. Lessons Learned aus Contextra
3. Synthese der vier Löschvarianten
4. Kernkonzept: drei Ebenen und eine Brücke
5. Verantwortungsgrenze: was Acta löscht und was nicht
6. Architektur: Schichten, Crates, Ports, Verdrahtung
7. Domänenmodell
8. Zeit und Versionierung
9. Kryptografie und Schlüsselverwaltung
10. Speicherlayout und Formate
11. Schreibpfad (Ingest)
12. Lesepfad (ReadGate)
13. Löschung: Geschäftslogik und Saga
14. Messung und Nachweis
15. Backup, Restore, Kopien
16. Schnittstellen und Bereitstellung
17. Konfiguration
18. Invarianten-Katalog
19. Teststrategie und Gates
20. Roadmap
21. Offene Entscheidungen, Risiken, Vorbehalte
- Anhang A: Glossar
- Anhang B: Durchgespielte Szenarien

---

## 1. Zielbild und Designprinzipien

### 1.1 Zielbild

Acta ist eine **einbettbare Rust-Bibliothek** (optional als Server, MCP-Dienst und CLI), die drei Dinge gleichzeitig kann, ohne dass sie sich widersprechen:

1. **Nachvollziehbarkeit (Zeitreisen):** Für Unternehmen ist jederzeit belegbar, *welcher Stand von Normen, Richtlinien, Verträgen, Konfigurationen und Entscheidungsregeln zu welchem Zeitpunkt galt* und *was das System zu welchem Zeitpunkt darüber wusste*. Ein LLM kann auf Basis eines wählbaren Stichtags befragt werden, und die Antwort ist später reproduzierbar belegbar.
2. **Löschung (Crypto-Shredding):** Personenbezogene Daten sind nachweisbar und zeitnah unlesbar und physisch entfernt, inklusive abgeleiteter Strukturen (Vektoren, Volltext, Graph, Caches) und Acta-eigener Kopien (Checkpoints, Backups).
3. **Stabilität:** Jede Komponente ist entkoppelt, einzeln testbar und fail-closed. Es gibt genau einen Löschpfad und genau eine Stelle, an der alles verdrahtet wird.

### 1.2 Der Kernkonflikt und seine Auflösung (Kurzfassung)

> Versionskontrolle will **nichts vergessen**, Löschung will **vergessen können**. Beides auf denselben Daten zu verlangen, ist unlösbar. Acta löst es, indem es **nicht dieselben Daten** versioniert und löscht.

* Was **keine Personendaten** enthält (Normen, Richtlinien, Konfigurationsstände, Modellversionen, Entscheidungsregeln, Nachweise) liegt in der **Chronik** (Ebene N): unveränderlich, bitemporal, hash-verkettet, dauerhaft zeitreisbar.
* Was **personenbezogen** ist, liegt im **Tresor** (Ebene P): pro Einheit verschlüsselt, mit kurzem, rechtsgrundabhängigem Versionshorizont. **Löschung gewinnt immer**, auch gegen Zeitreise.
* Was **abgeleitet** ist (Indizes, Caches), liegt in **Ableitungen** (Ebene D): nie Quelle der Wahrheit, immer neu aufbaubar, mit Lineage zu seinen Eingaben.
* Eine **Brücke** (`SubjectHandle`, keyed Commitments) erlaubt es, Nachweise über Löschungen in der Chronik abzulegen, ohne dass dort Personendaten stehen.

Details: §4.

### 1.3 Designprinzipien

| ID | Prinzip | Konsequenz |
|---|---|---|
| P1 | **Eine Wahrheit pro Datenklasse** | Jede Einheit hat genau eine Klasse und genau eine Ebene. Doppelhaltung ist verboten. |
| P2 | **Löschung ist ein Schlüsselereignis, kein Dateivorgang** | Unlesbarkeit tritt durch Schlüsselvernichtung in Minuten ein. Physisches Aufräumen ist terminierte Hygiene, nicht Voraussetzung. |
| P3 | **Erase gewinnt** | Kein Lesepfad (auch kein historischer) liefert Daten einer eingefrorenen oder gelöschten Einheit. |
| P4 | **Ports gehören dem Konsumenten** | Ein Crate definiert die Traits, die es braucht. Wer sie implementiert, ist ihm unbekannt. |
| P5 | **Genau ein Composition Root** | Nur `acta-node` kennt konkrete Adapter. Kein anderes Crate verdrahtet. |
| P6 | **Fail-closed** | Bei Zweifel, Fehler oder fehlender Verdrahtung wird gesperrt, nicht freigegeben. |
| P7 | **Beweis nur aus Messung** | Nachweise entstehen ausschließlich aus unabhängigen Scans mit Positivkontrolle, nie aus Konstanten oder Selbstauskunft. |
| P8 | **Synchroner Kern, asynchrone Hülle** | Kern-Crates sind runtime-frei. Tokio lebt nur in Server- und Shell-Crates. |
| P9 | **Klein halten** | Jedes Crate hat ein LOC-Budget (§20). Überschreitung braucht eine Begründung im ADR. |
| P10 | **Verdrahtung ist testbar** | Unverdrahtete Komponenten werden zur Compile-, Gate- oder Bootzeit gefunden, nicht im Betrieb (§6.5). |

---

## 2. Lessons Learned aus Contextra

Aus Struktur, `docs/decoupling/REPORT.md` und `FINDINGS.md` in Contextra sowie der Analyse der vier Löschvarianten:

| # | Befund in Contextra | Ursache | Gegenmaßnahme in Acta |
|---|---|---|---|
| L1 | Fünf bis sechs getrennte Löschpfade (`Collection::delete`, `execute_forget_pipeline`, `DpiPurgeDriver`, Inline-Löschung in `collections.rs`, MCP-Handler) mit unterschiedlicher Semantik | Löschung wurde pro Feature nachgerüstet | **Ein** Einstiegspunkt `erase()` (§13). Der rohe Store-Delete ist `pub(crate)`. Lint verbietet andere Löschaufrufe. |
| L2 | `ErasureLedger`, WAL-Retire, `PurgeOrchestrator::resume` ohne Produktionsaufrufer oder als No-Op | Komponenten gebaut, aber nie verdrahtet | **Wiring Contract** (§6.5): Pflicht-Ports als nicht-optionale Felder, Dead-Port-Gate, Boot-Selbsttest mit Kanarien-Löschung |
| L3 | Crypto-Shredding nur für KV-Cache, gewrappter DEK blieb im Payload | Schlüssel und Chiffretext nicht getrennt | **KeyTable** getrennt vom Chiffretext, Ersetzen-Semantik (§9.4) |
| L4 | Hartkodierter Fallback-Schlüssel im MCP-Handler, Konstanten `0` im Beweis | Defaults für Geheimnisse, Beweis ohne Messung | Keine Default-Schlüssel (Start verweigert). Proof-Konstruktoren sind `pub(crate)`, nur aus `ScanOutcome` (§14). |
| L5 | MVCC-Retention behält die jeweils neueste Version `<= floor`. Ein Tombstone ist ein Markierungsbit, kein Löschen. Zeitreise liefert Klartext gelöschter Daten. | MVCC und Löschung teilen sich dieselben Daten | **Drei Ebenen** (§4). Retention ist klassenabhängig. ReadGate sitzt *vor* jeder Zeitreise (§12). |
| L6 | Gemeinsames `types`-Crate, `compat/`-Module, Feature-Default-Flip (`default = ["workspace"]`) | Entkopplung nachträglich per Brückenschicht | Keine `compat`-Schichten. Ports gehören dem Konsumenten. Features sind rein additiv (R6, §6.2). |
| L7 | 64-Bit-`DocId` als Löscheinheit | Kollisionsrisiko, zu grobe Einheit | 128-Bit-`UnitId`, abgeleitet per keyed BLAKE3 (§7) |
| L8 | Vier konkurrierende Löschkonzepte, 15 Rand-Crates, Saga-Phasen 6/7/9/10 | Konzept wuchs ohne Konsolidierung | Diese Spezifikation vereinheitlicht (§3). Ein Zustandsautomat. LOC-Budgets. |
| L9 | Async-Journal, Hintergrund-Threads im WAL-Flusher in den Löschpfad verwoben | Runtime-Kopplung | Synchroner Kern, I/O nur über die bestehende `io_seam`/`StoreIo`-Abstraktion (§6.6) |
| L10 | Backups konnten gelöschte Daten wiederbeleben | Restore kannte die Löschhistorie nicht | `ErasedSet` + Zwei-Spuren-Backup + RestoreGuard (§15) |
| L11 | Viele deaktivierte Workflows (`.github/_disabled/*`), Gates ohne Durchsetzung | Prozess wuchs schneller als Disziplin | Wenige, **erzwungene** Gates (§19), alle lokal per `cargo xtask gates` ausführbar |

---

## 3. Synthese der vier Löschvarianten

| Variante | Übernommen | Verworfen | Grund |
|---|---|---|---|
| **V1** In-Place-Härtung | Dekker-Fence mit RAII-Ticket; Regel *Journal vor Manifest-Tausch* (J1); streamendes SSTable-Rewrite in `O(Block)`; Intent erst nach Scope-Datei löschen; kanonische TLV-Kodierung; `LayerWitness` nur aus `ScanOutcome` | 9-Phasen-Enum, zweistufiger Proof A/B, Verteilung über fünf Bestands-Crates | Zu viele Phasen, Verteilung ist die Ursache von L1/L2 |
| **V2** Autarkes Kern-Crate | Null interne Abhängigkeiten, `forbid(unsafe_code)`, Ports `FenceGate`/`WalSink`/`KeyStore`/`ResidueScanner`, Fail-closed-Vergiftung des Tresor-Handles, Restore-Guard, `can_transition_to` | Reine Ein-Crate-Lösung für alles | Löschkern ja, aber Chronik, Tresor, Crypto brauchen eigene Crates |
| **V3** Zwei Ebenen, drei Spuren | **Fast-Path vor Hygiene**; `KeyTable` mit Ersetzen-Semantik; `Epoch Seal` (Re-Wrap-then-Destroy); Padmé-Padding; Nadelscan; Zwei-Spuren-Backup D/K; `ErasedSetMirror`; Trennung *Verified* / *Validated*; `RecipientLedger`; Mixed-Policy | Custody-Stufen K0–K3 als eigene Hierarchie (ersetzt durch `KeyAnchor`-Stärke); vier verkettete Siegel (ersetzt durch Klassen L/P/C/E/M) | Gleiche Semantik, weniger Begriffe |
| **V4** Verschlankter Neubau | `ErasureManifest` (exhaustives `KeyKind`); 128-Bit `scope_id`; zentraler **`ReadGate`, Erase gewinnt**; `ForensicScanner` mit Positivkontrolle (`CanaryFactory`); `classify` als reine Funktion; LOC-Budget; Entfernen des KV-Cache | Async `Vfs`-Trait (Acta hat bereits ein synchrones `io_seam`); eigene Ed25519-Variante `v4` (Neustart bei Version 1) | Passt zu Acta, aber synchron |

**Bewusste Entscheidung:** Acta liefert in v1 **keinen** KV-/LLM-Kontext-Cache. Jeder künftige Cache MUSS sich als `DerivedParticipant` registrieren (§6.3), sonst wird er beim Commit abgelehnt (BR-9).

---

## 4. Kernkonzept: drei Ebenen und eine Brücke

### 4.1 Die drei Ebenen

```
                      ┌───────────────────────────────────────────────┐
  Ebene N  CHRONIK    │ Normen · Richtlinien · Konfig-/Modellstände   │  keine PII
  (unveränderlich,    │ Entscheidungsregeln · Nachweise · Receipts    │  bitemporal
   bitemporal)        │ Hash-Kette + signierte Checkpoints            │  dauerhaft
                      └──────────────▲────────────────────────────────┘
                                     │ Brücke: nur Commitments/Handles
                      ┌──────────────┴────────────────────────────────┐
  Ebene P  TRESOR     │ Personenbezogene Einheiten                    │  pro Einheit
  (löschbar,          │ DEK je Einheit, KEK je Epoche                 │  verschlüsselt
   Versionshorizont)  │ ErasedSet · ReadGate · Saga                   │  Erase gewinnt
                      └──────────────▲────────────────────────────────┘
                                     │ Lineage (Pflicht)
                      ┌──────────────┴────────────────────────────────┐
  Ebene D  ABLEITUNG  │ Vektorindex · Volltext · Graph · Caches       │  nie Quelle,
  (neu aufbaubar)     │ ArtKey je Artefakt                            │  immer purgebar
                      └───────────────────────────────────────────────┘
```

### 4.2 Datenklassen-Matrix (normativ)

| Klasse | Beispiele | Ebene | Zeitreise | Löschung | Aufbewahrung |
|---|---|---|---|---|---|
| `Norm` | Gesetzestext in Fassung, interne Richtlinie v7, Vertragsvorlage, Tarifregel, Prompt-/Modellversion, Konfigurationsstand | N | **unbegrenzt**, bitemporal (`valid` × `recorded`) | nur **Takedown** (Ausnahme, Vier-Augen, §13.12) | dauerhaft |
| `Personal` | Kundenstammdaten, Vertrag *mit* Namen, E-Mail, Mitarbeiterakte | P | nur **Versionshorizont** (Default 30 Tage, oder bis `retain_until` bei Rechtspflicht) | Crypto-Shred + Purge | gemäß Rechtsgrundlage |
| `Special` | Art. 9/10-Daten (Gesundheit, Strafurteile, Biometrie) | P | Horizont **0** (keine Historie), keine Karenz | wie `Personal`, ohne `grace` | kürzestmöglich |
| `Derived` | Embeddings, Postings, Graph-Kanten, Caches | D | keine; AsOf wird aus N/P neu berechnet oder zeitgefiltert | Lineage-Purge | = Lebensdauer der Quelle |
| `Evidence` | `ErasureProof`, `ContextReceipt`, Audit-Kette | N (Unterkette `audit`) | unbegrenzt | **nie** (enthält per Konstruktion keine PII) | dauerhaft |
| `Unclassified` | Ergebnis fehlgeschlagener Klassifikation | P (Quarantäne) | keine | wie `Personal` | bis Klärung |

**BR-2 (im Zweifel personenbezogen):** `Unclassified` wird wie `Personal` behandelt. Der Weg in die Chronik führt ausschließlich über einen Beweis der Nicht-Personenbezogenheit (§4.5).

### 4.3 Die Brücke

* **`SubjectHandle`** (128 Bit, opak): Von `SubjectDirectory` vergeben. Die Zuordnung *externe Kennung → Handle* ist selbst Personendatum und liegt im Tresor. Sie wird bei Löschung mit vernichtet. Ein Betroffener, der zurückkehrt, erhält ein **neues** Handle. Eine inhaltliche Wiedererkennung ist Sache des Verantwortlichen, nicht von Acta.
* **`SubjectCommitment`** = `BLAKE3-keyed(K_commit[tenant, epoch], canonical(scope))`. Wird im `ErasedSet` und in Nachweisen der Chronik verwendet. Nach Rotation und Vernichtung von `K_commit[epoch]` sind alte Commitments nicht mehr verknüpfbar. Ein Nachweis bleibt gültig, wird aber nach Ablauf der `commitment_linkability`-Frist anonym (§14.6).
* Die Chronik enthält damit nie Namen, Kennungen oder rückrechenbare Hashes. Sie enthält **Handles/Commitments und Messergebnisse**.

### 4.4 Klassifikation und Segmentierung (Ingest)

Ein Eingabedokument wird in **Einheiten (Units)** zerlegt. Jede Einheit hat genau eine Klasse:

1. `Extractor` (Port) liefert strukturierte Abschnitte mit Spans.
2. `Classifier` (Port) liefert je Abschnitt `DataClass` + `SubjectSpans` + Konfidenz.
3. Regelwerk (`ClassificationPolicy`): Konfidenz unter Schwelle ⇒ `Unclassified`. Jede erkannte Person ⇒ `Personal`. Art-9-Detektor ⇒ `Special`.
4. **Gemischte Abschnitte** (Norm-Text mit eingebettetem Namen): Segmentierung auf Satz-/Spanebene. Ist sie nicht trennscharf möglich, entscheidet `MixedPolicy`:
   * `SplitStrict` (Default): Einheit wird in `Personal`-Segment und `Norm`-Rest zerlegt. Der Norm-Rest enthält *keine* Platzhalter mit Rückbezug, nur eine Lücke.
   * `WholeAsPersonal`: gesamte Einheit bleibt im Tresor.
   * `Reject`: Ingest wird abgelehnt und zur Klärung gemeldet.
5. Mehrere Betroffene in einer Einheit: bevorzugt `Split` je Betroffenem. Verbleibende Mehrbetroffenen-Einheiten tragen eine `SubjectSet` und unterliegen bei Löschung der `MultiSubjectPolicy` (`BlockAll`, `RedactAndRekey`, `DeleteAll`, §13.3).

### 4.5 Promotion (Personal → Norm) und Anonymisierung

* Automatische Promotion findet **nicht** statt (BR-3).
* Ein `Redactor` (Port) kann aus einer Personal-Einheit eine **neue** Norm-Einheit erzeugen. Voraussetzung: `PromotionPolicy` erlaubt es, ein `AnonymityVerifier` bestätigt (k-Anonymität/Pseudonym-Freiheit gemäß Policy), und ein `Controller` zeichnet frei. Die Norm-Einheit trägt `ResidualRisk::Reidentification` im Nachweis.
* Die Ursprungseinheit bleibt im Tresor und wird normal gelöscht. Die Norm-Einheit hat **keine Lineage** auf die Ursprungseinheit (sonst wäre sie weiterhin personenbezogen).

---

## 5. Verantwortungsgrenze: was Acta löscht und was nicht

Die Frage „nur Daten in Contextra selbst oder auch Kopien und Dateien“ wird als **Custody Boundary** beantwortet. Es gibt sechs Zonen mit je eigenem Versprechen und eigener Nachweisklasse.

| Zone | Ort | Besitzer | Mechanismus | Versprechen / Nachweis |
|---|---|---|---|---|
| **Z1 Kern** | WAL, SSTables, Manifest, Blob-Segmente, KeyTable, Journale | Acta | Crypto-Shred (Fast-Path) + gefilterter Rewrite (Hygiene) | **Verified**, Klassen L, P, C, M |
| **Z2 Ableitungen** | Vektorindex (fp32, SQ8), Volltext-Postings, Graph, Caches, LLM-Kontextspeicher | Acta (Indexcrates) | Lineage-Purge, `ArtKey`-Wechsel, geplanter Neuaufbau | **Verified**, Klasse E |
| **Z3 Acta-eigene Kopien** | Checkpoints, Hardlink-Forks, Backups, Exporte | Acta | `CopyRegistry` (jede Kopie ist registriert), Zwei-Spuren-Backup, Schlüsselvernichtung | **Verified** (Daten-Spur unlesbar), Schlüssel-Spur terminiert |
| **Z4 Weitergegebene Kopien** | Export an Dritte, Kontext an externen LLM-Anbieter, Empfänger | Dritte | `ReleaseLedger` + `RecipientNotifier` (Art. 19) | **Notified/Acknowledged**, *nicht* Verified |
| **Z5 Quelldateien** | PDF auf Dateiserver, Mail im Postfach, SharePoint | Verantwortlicher | Modus `Reference` (Acta hält nur Hash + URI) oder `Ingest` (verschlüsselte Kopie in Z1). Optional `SourceConnector::request_removal`. `SourceProbe` meldet verbliebene Quellen | **NotOwned**, plus Befund `SourceStillPresent` als Aufgabe |
| **Z6 Umgebung** | RAM, Swap, Core-Dumps, SSD-Wear-Leveling, Page-Cache | Betriebssystem/Hardware | Verschlüsselung dominiert (Reste sind ohne Schlüssel wertlos), `mlock`/`zeroize` für Schlüssel, `NoCoreDump`-Härtung | **Annahmen** `A-FLASH`, `A-RAM` im Nachweis, keine Messung möglich |

**Konsequenz für die Implementierung:**

* Acta **MUSS** Z1 bis Z3 verifiziert löschen können und dafür Messungen liefern.
* Acta **MUSS** Z4 und Z5 *verfolgen* (Ledger, Benachrichtigung, Sondierung), **DARF NICHT** dort Löschung als erfolgt behaupten.
* Der Nachweis **MUSS** je Zone den Status ausweisen und Z6-Annahmen explizit nennen (§14).
* `Ingest`-Modus für Quelldateien ist **opt-in**. Der Default für Dateien ist `Reference`, damit Acta nicht ungefragt Kopien besitzt.

---

## 6. Architektur: Schichten, Crates, Ports, Verdrahtung

### 6.1 Crate-Karte

Bestehende Acta-Crates bleiben unverändert in ihrer Rolle. Neue Crates sind fett.

| Schicht | Crate | Verantwortung | Darf abhängen von | Status | LOC-Budget (Prod.) |
|---|---|---|---|---|---|
| L0 | `acta-ids` | `TenantId`, `CollectionId`, `DocId`, `TxId`, `TOMBSTONE_BIT` | – | vorhanden | – |
| L0 | `acta-env` | `Clock`, `Rng`, `IdGen`, `MetricsSink`, `ManualClock` | `acta-ids` | vorhanden | – |
| L0 | `acta-sys` | mmap, Durability, POSIX/Win32-ACL. **Einziges Crate mit `unsafe`** | – | vorhanden | – |
| L1 | **`acta-model`** | Reine Domänentypen: `UnitId`, `SubjectHandle`, `DataClass`, `Interval`, `AsOf`, `LegalBasis`, `ErasureState`, `ProofClass`, `KeyKind` | `acta-ids` | neu | 700 |
| L1 | **`acta-codec`** | Kanonische TLV-Kodierung, Envelope-Layout, Hash-Kette, Padmé | `acta-model` | neu | 600 |
| L2 | `acta-mvcc` | Sequenzlog, Snapshots, `TxBuffer`, SSI. **Kennt keine Löschung** | `acta-ids`, `acta-env` | vorhanden | – |
| L2 | `acta-store` | LSM (WAL, Memtable, SSTable, Manifest, Compaction), `tenant_codec`, `io_seam` | `acta-ids/env/sys/mvcc` | vorhanden, wird erweitert (§6.6) | – |
| L2 | **`acta-crypto`** | Schlüsselhierarchie, Envelope, `KeyTable`, `EpochSeal`, Signatur | `acta-model`, `acta-codec`, `acta-env` | neu | 1.600 |
| L3 | **`acta-derive`** | Verträge für Ableitungen: `DerivedParticipant`, Lineage, Konformanzsuite | `acta-model` | neu | 500 |
| L3 | **`acta-chronicle`** | Ebene N: bitemporale Chronik, Hash-Kette, Checkpoints | `acta-model`, `acta-codec`, `acta-crypto` | neu | 1.500 |
| L3 | **`acta-vault`** | Ebene P: Units, SubjectDirectory, ErasedSet, ReadGate, Horizont | `acta-model`, `acta-codec`, `acta-crypto`, `acta-derive` | neu | 1.600 |
| L3 | **`acta-erasure`** | Saga, Fence, Plan, Hygiene, Fristen | `acta-model`, `acta-codec`, `acta-crypto`, `acta-derive` | neu | 2.200 |
| L3 | **`acta-proof`** | Messung → Klassifikation → `ErasureProof`, Verifikation | `acta-model`, `acta-codec`, `acta-crypto` | neu | 900 |
| L3 | **`acta-backup`** | `CopyRegistry`, Spuren D/K, RestoreGuard | `acta-model`, `acta-codec`, `acta-crypto` | neu | 900 |
| L4 | **`acta-query`** | AsOf-Planer, Kontextbau für LLM, `ContextReceipt` | `acta-model` + Traits von Chronicle/Vault | neu | 1.100 |
| L5 | **`acta-engine`** | Use-Case-Dienste (Ingest, Query, Erase) gegen Ports | L1–L4 (nur Traits) | neu | 1.200 |
| L5 | **`acta-adapter-store`** | Implementiert Store-nahe Ports mit `acta-store` | `acta-store`, Port-besitzende Crates | neu | 1.000 |
| L5 | **`acta-adapter-*`** | je ein Adapter pro Indexcrate (`-vector`, `-text`, `-graph`) | Indexcrate, `acta-derive` | neu | je 400 |
| L6 | **`acta-node`** | **Composition Root.** Baut `Wiring`, startet, Boot-Selbsttest | alles | neu | 700 |
| L7 | **`acta`** (Facade), `acta-server`, `acta-mcp`, `acta-cli` | Einbettungs-API, HTTP/gRPC, MCP, CLI | `acta-node` | neu | je 500 |
| Test | **`acta-lab`** | `FaultVfs`, `ForensicScanner`, `HeapScanner`, `CanaryFactory`, `ReferenceModel` | nur dev-Abhängigkeit | neu | 1.800 |
| Test | **`acta-conformance`** | Wiederverwendbare Port-Konformanztests | dev | neu | 800 |

Gesamtbudget Produktionscode neue Crates: etwa **16.000 Zeilen**. Der Löschkern (`acta-crypto` + `acta-vault` + `acta-erasure` + `acta-proof`) bleibt unter **6.300 Zeilen**.

### 6.2 Abhängigkeitsregeln (erzwungen durch Gate G1)

| Regel | Inhalt |
|---|---|
| **R1** | Abhängigkeiten zeigen nur nach unten (höhere Schichtnummer → niedrigere). Gleiche Schicht: keine Querabhängigkeit. |
| **R2** | Port-Traits gehören dem **konsumierenden** Crate. Es gibt kein zentrales `ports`-Crate und kein zentrales `types`-Crate über `acta-model` hinaus. |
| **R3** | Nur `acta-adapter-*` dürfen zwei Peer-Crates gleichzeitig kennen. |
| **R4** | Nur `acta-node` nennt konkrete Adaptertypen. |
| **R5** | Öffentliche APIs re-exportieren keine Typen Dritter (kein `pub use blake3::Hash`). Newtypes statt Durchreichen. |
| **R6** | Cargo-Features sind **rein additiv** und verändern nie Verhalten des Löschpfads. Kein Default-Flip. |
| **R7** | Keine `compat`-Module, keine Altschnittstellen. Breaking Change = neue Version. |
| **R8** | Kern-Crates (`L1`–`L4`) enthalten kein `tokio`, kein `std::fs`, kein `std::time::SystemTime`, kein `rand::thread_rng`. Zugriff nur über `Clock`/`Rng`/`StoreIo` (Gate G3). |
| **R9** | `#![forbid(unsafe_code)]` in allen Crates außer `acta-sys`. |
| **R10** | `unwrap`, `expect`, `panic!`, Indexzugriff ohne Prüfung in Produktionscode sind `deny` (Workspace-Lints von `warn` auf `deny` anheben). |

### 6.3 Port-Katalog

Jeder Port wird im besitzenden Crate definiert (R2), ist **synchron**, hat eine Fehlertaxonomie und eine Konformanzsuite in `acta-conformance`.

| Port | Besitzer | Zweck | Produktionsadapter (Pflicht) |
|---|---|---|---|
| `KeyAnchor` | `acta-crypto` | Wurzelschlüssel wrappen/unwrappen, `destroy()`, `strength()` | `HardwareAnchor` (TPM/HSM), `OsKeyringAnchor`, `PassphraseAnchor` (Argon2id), `ExternalKmsAnchor`, `FileOnlyAnchor` (nur Entwicklung, ergibt `C-soft`) |
| `AtomicFileIo` | `acta-crypto` | Datei atomar ersetzen (tmp → fsync → rename → Verzeichnis-fsync), scrubben | über `acta-store::io_seam` |
| `KvPort` | `acta-vault`, `acta-chronicle` | Schlüssel-Wert-Zugriff mit Snapshot (`get`, `get_at`, `scan_*`, `put`, `delete`) | `acta-adapter-store` auf `TenantScopedStorage` |
| `PurgeTarget` | `acta-erasure` | `rewrite_filtered(matcher)`, `flush`, `retire_wal`, `compaction_barrier` | `acta-adapter-store` |
| `WriteAdmission` | `acta-erasure` | Schreibzulassung für Fence (Dekker-Muster) | `acta-adapter-store` |
| `RawScanner` | `acta-proof` | Rohbyte-Scan **ohne** Format-Parser über Dateien, Heap | `acta-lab::ForensicScanner` (auch Produktion), `HeapScanner` |
| `DerivedParticipant` | `acta-derive` | `purge`, `scan`, `rebuild_due`, `kinds()` | je Indexcrate ein Adapter |
| `SagaJournal` | `acta-erasure` | Append-only, fsync, CRC + Hash-Kette, Scope verschlüsselt | `acta-adapter-store` oder eigenes Journal über `StoreIo` |
| `ErasedSetMirror` | `acta-vault` | Spiegel des `ErasedSet` auf zweites Medium/WORM | Datei-Adapter, optional Objektspeicher |
| `Notary` | `acta-proof` | Qualifizierter Zeitstempel (RFC 3161) | `NoopNotary` (Standard), `Rfc3161Notary` |
| `Classifier`, `Extractor`, `Redactor`, `AnonymityVerifier` | `acta-engine` | Ingest-Intelligenz | austauschbar (Regeln, ONNX-Modell) |
| `SourceConnector` | `acta-engine` | Quellen lesen, `request_removal`, `probe` | optional je System |
| `RecipientNotifier` | `acta-erasure` | Art. 19-Benachrichtigung | Webhook/Mail-Adapter |
| `Authz` | `acta-engine` | Rollen, Vier-Augen | `acta-node` konfigurierbar |
| `Clock`, `Rng`, `IdGen`, `MetricsSink` | `acta-env` | Umwelt | vorhanden |

### 6.4 Sync-Kern, async-Hülle (P8)

* Alle Crates L1–L5 sind **synchron**. Nebenläufigkeit entsteht durch Threads im Composition Root oder in der Hülle.
* `acta-server`/`acta-node-shell` rufen den Kern über `spawn_blocking` oder einen festen Worker-Pool auf.
* Kein Sperr-Guard wird über `.await` gehalten (gilt bereits für `acta-mvcc`).
* Die bestehende globale Sperrenordnung aus `acta-mvcc/lib.rs` bleibt gültig. **Erweiterung** um zwei Glieder am Anfang und am Ende:
  1. `ErasureFence` (Scope-Zulassung)
  2. Document/Key-Lock → TxBuffer-Shards → SnapshotRegistry → SequenceLog/SSI (unverändert)
  3. `KeyTable`-Lock (kurz, niemals über I/O gehalten, nie vor 2.)

### 6.5 Composition Root und Wiring Contract (Antwort auf L2)

`acta-node` baut genau eine Struktur. **Jeder Pflicht-Port ist ein nicht-optionales Feld.** Ein vergessener Adapter ist ein Kompilierfehler.

```rust
pub struct Wiring {
    // Schlüssel
    pub anchor:        Arc<dyn KeyAnchor>,
    pub file_io:       Arc<dyn AtomicFileIo>,
    // Speicher
    pub kv:            Arc<dyn KvPort>,
    pub purge:         Arc<dyn PurgeTarget>,
    pub admission:     Arc<dyn WriteAdmission>,
    // Messung
    pub raw_scanner:   Arc<dyn RawScanner>,
    pub notary:        Arc<dyn Notary>,
    // Löschung
    pub journal:       Arc<dyn SagaJournal>,
    pub mirror:        Arc<dyn ErasedSetMirror>,
    pub participants:  NonEmptyVec<Arc<dyn DerivedParticipant>>,
    pub notifier:      Arc<dyn RecipientNotifier>,
    // Umwelt
    pub clock:         Arc<dyn Clock>,
    pub rng:           Arc<dyn Rng>,
    pub authz:         Arc<dyn Authz>,
}
```

Zusätzliche Absicherungen:

1. **Dead-Port-Gate (G2):** Ein Skript prüft, dass jeder `pub trait` mit Marker `#[acta_port]` mindestens (a) eine Produktionsimplementierung außerhalb von Tests und (b) mindestens eine Aufrufstelle im Kern hat.
2. **Registrierungsvollständigkeit:** Jedes `KeyKind` (§7) MUSS von mindestens einem `DerivedParticipant` oder vom Store-Pfad behandelt werden. Test `every_key_kind_has_a_purger` iteriert das exhaustive Enum.
3. **Boot-Selbsttest (Pflicht, `boot_selftest = true`):** Beim Start wird in einem **flüchtigen Test-Tenant** ein Kanarien-Dokument geschrieben, abgeleitet, gelöscht, gemessen und der Nachweis verifiziert. Schlägt dies fehl, startet Acta **nicht** (fail-closed). Dauer-Budget: < 2 s.
4. **Recovery-Pflicht:** `Acta::open` ruft `resume_pending()` auf, **bevor** der erste Lese- oder Schreibzugriff zugelassen wird. Ein Journal mit offener Saga ohne laufbaren Participant ist ein Startfehler, kein No-Op.

### 6.6 Abbildung auf den bestehenden Acta-Code

*Hinweis: Die Zuordnung beruht auf Dateistruktur, Modulköpfen und den gelesenen Dateien (`acta-mvcc/src/{lib,tombstone}.rs`, `acta-store/src/compaction/retention.rs`, `acta-ids`, Workspace-Manifest). Die Meilenstein-0-Inventur (§20) MUSS jede Zeile verifizieren.*

| Vorhandenes Modul | Rolle in Acta | Änderung |
|---|---|---|
| `acta-ids::TOMBSTONE_BIT` (Bit 63) | MVCC-Löschmarkierung | bleibt. **Ein Tombstone ist keine Löschung** im Sinne dieser Spezifikation (BR-1). |
| `acta-mvcc` (`SnapshotRegistry`, `SequenceLog`, `TxBuffer`, SSI) | Nebenläufigkeit, Mindest-Lesesequenz (`floor`) | unverändert, **bekommt keinerlei Löschwissen** |
| `acta-store::tenant_codec` | **Einziger** Zugang der Engine zum Store, Einhängepunkt für ReadGate | wird zur Fassade; `LsmStorage` wird `pub(crate)` |
| `acta-store::compaction::retention::retain_key_versions` | Version-GC nach `floor` | erweitert um `RetentionPolicy`-Hook je `KeyKind` (§8.4) |
| `acta-store::lsm::{commit,recovery,fence,observer}` | Commit, Recovery, vorhandenes `fence.rs`, Observer | `fence.rs` wird Basis des `WriteAdmission`-Adapters. Observer liefert Commit-Ereignisse für Lineage (§11). |
| `acta-store::io_seam::{StoreIo, RealFs}` | Dateizugriff mit Fehlerinjektion | **wird die einzige I/O-Grenze** der Kern-Crates (statt neuem Async-`Vfs`) |
| `acta-store::sstable::{builder,reader,block_cache}` | Streaming-Rewrite, Cache-Invalidierung | neue öffentliche, **löschneutrale** Fähigkeit `rewrite_filtered(Matcher)` |
| `acta-store::wal::*` (HMAC-Kette, Segmente, Flusher) | WAL, Scrub, Segmentretirement | neue Fähigkeit `rotate_and_seal`, `retire_segments_up_to`, `scrub` |
| `acta-store::manifest::*`, `acta-store::compaction::engine::install` | atomarer Dateitausch | wird für `rewrite_filtered` wiederverwendet |
| `acta-env::{Clock,Rng,IdGen,ManualClock}` | Determinismus in Tests | unverändert. Kern-Crates nutzen ausschließlich dies. |
| `acta-store::tests/tenant_scoped_storage.rs` u. a. | vorhandene Tests | bleiben als Regressionsbasis |

**Wichtig:** `acta-store` bekommt **kein** Konzept „Erasure“. Es bietet generische Fähigkeiten (gefilterter Rewrite, Barrier gegen Compaction, WAL-Scrub, Rohscan-Hilfen). Die Löschsemantik lebt außerhalb. Das verhindert die Rückkehr zur Contextra-Verdrahtung.

---

## 7. Domänenmodell

### 7.1 Identifikatoren

```rust
// acta-ids (vorhanden): TenantId, CollectionId, DocId(u64), TxId, TOMBSTONE_BIT
// acta-model (neu):
pub struct UnitId(u128);        // kleinste löschbare Einheit; BLAKE3-keyed(K_scope, tenant‖collection‖user_key)
pub struct SubjectHandle(u128); // opak, von SubjectDirectory vergeben
pub struct ErasureId(u128);     // pro Löschauftrag, aus IdGen
pub struct KeyEpoch(u32);       // KEK-Epoche je Collection
pub struct ArtifactId(u128);    // Index-Artefakt (Segment, Snapshot-Generation)
pub struct ChronicleEntryId(u128);
pub struct Commitment([u8; 32]);// SubjectCommitment / ScopeCommitment
```

`DocId` (64 Bit) bleibt Index-intern im MVCC. **Löschen und Beweisen laufen ausschließlich über `UnitId`** (L7).

### 7.2 Klassen, Rechtsgrundlage, Einheit

```rust
pub enum Plane { Chronicle, Vault, Derived }

pub enum DataClass { Norm, Personal, Special, Derived, Evidence, Unclassified }

pub enum LegalBasis {
    Consent       { given_at: Date, scope: ConsentScope },
    Contract,
    LegalObligation { statute: StatuteRef, retain_until: Date },
    LegitimateInterest { assessment_ref: Commitment },
    PublicTask,
}

pub struct Unit {
    pub id: UnitId,
    pub tenant: TenantId,
    pub collection: CollectionId,
    pub class: DataClass,
    pub subjects: SubjectSet,            // leer nur bei Norm/Evidence/Derived
    pub basis: LegalBasis,               // Pflicht bei Personal/Special
    pub history_horizon: Horizon,        // aus Klasse + Basis abgeleitet
    pub source: SourceRef,               // Reference | Ingested
    pub created_seq: Seq,
}

pub enum Horizon { None, Days(u16), Until(Date) }
```

### 7.3 Schlüsselarten (exhaustiv)

```rust
// Bewusst KEIN #[non_exhaustive]. Jede Ergänzung bricht den Build der Purger. (L2, V4)
pub enum KeyKind {
    User, DocMap, Rel, Intent,        // Kernschlüssel im Store
    Text, Graph, Vector,              // Ableitungen
    ErasureManifest,                  // Eintrag der Manifest-Tabelle selbst
    Chronicle,                        // Ebene N (nie gelöscht)
    Journal, KeyTable,                // Verwaltungsdaten
}
```

### 7.4 Löschmodell

```rust
pub enum EraseTarget {
    Subject(SubjectHandle),                          // Art. 17
    Units(Vec<UnitId>),                              // gezielt
    Collection { tenant: TenantId, collection: CollectionId },
    Tenant(TenantId),                                // Off-Boarding
    Expired { before: Date },                        // Speicherbegrenzung (Sweeper)
    ConsentWithdrawn(SubjectHandle, ConsentScope),   // nur Einheiten mit Basis Consent
}

pub enum RetentionDecision {
    EraseNow,
    RestrictUntil(Date, StatuteRef),   // Art. 17 Abs. 3: Sperrung statt Löschung
    Partial,                           // Policy-Engine teilt nach Einheit auf
}

pub struct ErasureTicket { pub id: ErasureId, pub legal_deadline: Date, pub state: ErasureState }

pub enum ErasureState {
    Requested, Frozen, Planned, LogicallyDeleted, DerivedPurged,
    PlaintextMeasured, KeyDestroyed, CryptoMeasured, Sealed,
    Hygiene { step: HygieneStep },
    Completed,
    Restricted { until: Date },
    Pending { attempt: u32, next_retry_at: Instant },
    Escalated { reason: EscalationReason },   // nie „Failed“: Löschung wird wiederholt bis Erfolg oder Mensch
}
```

`ErasureState::can_transition_to(next)` ist eine **reine Funktion** mit vollständiger Übergangstabelle (§13.4). Proptest prüft: keine unerlaubte Kante, kein toter Zustand.

---

## 8. Zeit und Versionierung

### 8.1 Chronik (Ebene N): bitemporal

Zwei Zeitachsen, beide Pflicht:

| Achse | Bedeutung | Quelle |
|---|---|---|
| `valid: Interval<Date>` | **Geltungszeit.** Wann gilt die Norm, Richtlinie, Konfiguration in der Welt? Halboffen `[from, to)`. | vom Erfasser, geprüft |
| `recorded: Seq + Instant` | **Wissenszeit.** Wann hat Acta es erfahren und festgeschrieben? | von Acta (`Clock`, Sequenz) |

Daraus folgen die vier Fragen, die ein Unternehmen stellt:

| Frage | Abfrage |
|---|---|
| Was gilt jetzt? | `AsOf { valid_at: heute, known_at: jetzt }` |
| Was galt am 01.03.2022? | `AsOf { valid_at: 2022-03-01, known_at: jetzt }` |
| Was *wussten wir* am 15.04.2022 über den 01.03.2022? | `AsOf { valid_at: 2022-03-01, known_at: 2022-04-15 }` |
| Wie hat sich unser Wissen über den 01.03.2022 verändert? | `history(norm_id, valid_at)`: alle Korrekturen in `recorded`-Reihenfolge |

**Korrekturen überschreiben nie.** Sie legen einen neuen Eintrag mit `supersedes: Some(id)` an.

```rust
pub struct ChronicleEntry {
    pub id: ChronicleEntryId,
    pub kind: EntryKind,            // NormVersion | PolicyVersion | ConfigState | ModelVersion
                                    // | DecisionRule | ContextReceipt | ErasureProof | Checkpoint
    pub subject_matter: MatterId,   // z. B. "DSGVO/Art.17", "Policy/Reisekosten", "Model/embedder"
    pub valid: Interval<Date>,
    pub recorded_seq: Seq,
    pub recorded_at: Instant,
    pub content_hash: [u8; 32],     // BLAKE3 über kanonischen Klartext
    pub payload_ref: PayloadRef,    // Verweis auf verschlüsselten Blob (NormKey, §9.6)
    pub supersedes: Option<ChronicleEntryId>,
    pub authority: AuthorityRef,    // Wer hat erfasst (Hash der Authz-Entscheidung, keine Person im Klartext)
    pub prev_hash: [u8; 32],        // Hash-Kette je Chronik
}
```

**Integrität:** Hash-Kette (`prev_hash`) über kanonisch kodierte Einträge. Alle `checkpoint_every` Einträge (Default 1.024) wird ein **Checkpoint** signiert (Ed25519) mit Merkle-Wurzel. Optional ein `Notary`-Zeitstempel (RFC 3161). Damit ist die Chronik offline gegenüber Dritten beweisbar (§14.5).

**Scope der Chronik:** `Global` (geteilter, nur lesbarer Rechtskorpus, z. B. Gesetzestexte) und `Tenant` (interne Richtlinien). Abfragen können beide vereinen. Ein Mandant kann nie in die globale Chronik schreiben.

### 8.2 Tresor (Ebene P): Horizont statt Archiv

* Jede Einheit hat einen `Horizon` aus Klasse und Rechtsgrundlage:
  * `Personal` + `Contract` → `Days(history_default_days)` (Default 30)
  * `Personal` + `LegalObligation{retain_until}` → `Until(retain_until)` (z. B. handelsrechtliche Aufbewahrung)
  * `Special` → `None` (keine Historie)
* MVCC liefert **Konsistenz und Parallelität** (Snapshot-Isolation, SSI) und **kurze** Zeitreise innerhalb des Horizonts.
* Nach Ablauf des Horizonts verwirft die Compaction ältere Versionen (über `RetentionPolicy`, §8.4). Eine gezielte Zeitreise dahinter liefert `HorizonExceeded`.
* Ein **Ablauf-Sweeper** (§13.11) löst bei `retain_until`/Zweckende eine `Expired`-Löschung aus (Speicherbegrenzung, Art. 5 Abs. 1 lit. e).
* **Berichtigung** (Art. 16): `rectify()` erzeugt eine neue Version, markiert die alte als `Superseded`. Die alte bleibt im Horizont lesbar (für Audit), wird aber bei Löschung des Betroffenen mit vernichtet (gleicher `DocDek`).

### 8.3 AsOf-Semantik (normativ)

| Plane / Klasse | `valid_at` | `known_at` | Ergebnis |
|---|---|---|---|
| Chronik | beliebig | beliebig | `Found(entry)` oder `NotFound` |
| Tresor, Einheit lebendig, innerhalb Horizont | n. a. | Sequenz im Horizont | `Found` |
| Tresor, Einheit lebendig, **außerhalb** Horizont | n. a. | zu alt | `HorizonExceeded` |
| Tresor, Einheit **eingefroren/gelöscht** | n. a. | *egal* | `Erased` (**Erase gewinnt**) |
| Tresor, Einheit `Restricted` | n. a. | *egal* | `Restricted` nur für Rolle `LegalHold`, sonst `NotFound` |
| Ableitung | beliebig | beliebig | wird über Quelle zeitgefiltert oder neu berechnet; nie eigenständig als Wahrheit |

**Offenlegung:** Nach außen (Rolle `Reader`) wird `Erased` auf `NotFound` abgebildet. Nur `Auditor` und `Controller` sehen den Unterschied. Die Existenz gelöschter Daten ist selbst Information.

### 8.4 MVCC-Integration

* `acta-mvcc` bleibt unverändert und kennt keine Löschung.
* `acta-store` erhält einen Hook:

```rust
pub trait RetentionPolicy: Send + Sync {
    /// Entscheidet je Version, ob sie die Compaction überlebt.
    fn decide(&self, kind: KeyKind, seq: Seq, floor: Seq, horizon_floor: Seq, erased: bool) -> Keep;
}
// Vault-Policy: erased ⇒ Drop (auch wenn seq > floor); seq < horizon_floor ⇒ Drop (außer neueste ≥ floor-Regel);
// Chronicle-Policy: KeepAll.
```

* Die bestehende Regel von `retain_key_versions` (neueste Version `<= floor` bleibt) gilt weiter für **lebendige** Einheiten, wird aber für gelöschte durch `erased ⇒ Drop` überstimmt.
* **Offene Snapshots:** Ein Lesesnapshot, der vor der Löschung geöffnet wurde, darf den *physischen* Unlink verzögern (`Arc`-Drain). Er liefert nach dem Freeze **keine Daten mehr**, weil der ReadGate die Entscheidung trifft, nicht die Sichtbarkeit der Version.

### 8.5 LLM-Anfragen mit Stichtag

```rust
pub struct ContextRequest {
    pub query: String,
    pub as_of: AsOf,                    // Pflicht, kein Default „jetzt“ implizit im Server
    pub planes: PlaneSelection,         // { chronicle: true, vault: Option<VaultScope> }
    pub budget: ContextBudget,          // max. Items/Tokens
    pub actor: Actor,
}

pub struct ContextBundle {
    pub stand_header: String,           // z. B. "Rechtsstand 2022-03-01, bekannt seit 2022-04-15"
    pub items: Vec<ContextItem>,
    pub receipt_id: ChronicleEntryId,   // ContextReceipt in der Chronik
}

pub struct ContextItem {
    pub text: String,
    pub plane: Plane,
    pub provenance: Provenance,         // entry_id | unit_commitment, valid, recorded, content_hash
    pub confidence: Option<f32>,
}
```

* **`ContextReceipt`** (Evidence, Ebene N) enthält: `AsOf`, Abfrage-Hash, **Menge der verwendeten Chronik-Einträge** (IDs + Hashes), für Tresor-Treffer nur **Commitments**, Parameter des Rankings, Modellversion von Embedder und Ranker. Keine Personendaten.
* **Reproduzierbarkeit:** Dieselbe `ContextRequest` mit demselben `AsOf` liefert für die Chronik **identische** Ergebnisse (Determinismus von Index + Ranker + Seeds über `Rng`). Für Tresor-Treffer liefert die Wiederholung nach Löschung bewusst `Erased`-Lücken; der Receipt belegt die ursprüngliche Verwendung, ohne den Inhalt zu enthalten.
* **Der Stand-Header** wird dem LLM-Prompt vorangestellt. Antworten MÜSSEN Zitate über `content_hash` und `entry_id` liefern können (Nachprüfbarkeit).
* **Egress (Z4):** Jede Weitergabe von Tresor-Inhalten an einen externen Modellanbieter geht über `EgressGuard` (Pseudonymisierung/Platzhalter) und wird im `ReleaseLedger` verzeichnet (§15.4).
* **Invariante `INV-NOTRAIN`:** Acta verwendet Tresor-Inhalte nie zum Training oder Fine-Tuning. Embedder-Modelle werden nur gewichtet verwendet, nie mit Tresor-Daten angepasst.
