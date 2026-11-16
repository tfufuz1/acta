## 16. Test und Freigabe

### 16.1 Prinzipien [E]

1. Eine Zusage (1.5) ohne automatisierten Test gilt als nicht erfüllt (Gate G-ZUSAGE). Jeder Test hat ein **unabhängiges Orakel**: Testvektoren aus RFCs, eine unabhängige Referenzimplementierung im Test, Eigenschaften (Hin- und Rückumwandlung, Invarianten) oder Golden-Dateien. Ein Test, dessen Orakel aus dem getesteten Code abgeleitet ist, zählt nicht.
2. **Positivkontrolle vor Negativaussage:** Jeder Test, der die Abwesenheit von etwas behauptet (kein Klartext, kein Schlüssel, kein Treffer), enthält einen Teil, der die Anwesenheit am Canary nachweist.
3. Testdaten sind synthetisch. Echte personenbezogene Daten sind in Tests verboten.
4. Tests laufen deterministisch (`ManualClock`, `SeededRng`, `FaultVfs`); Nichtdeterminismus (Inferenz) ist hinter Ports durch Doubles ersetzt, die Fingerprints und Ausgaben festlegen.

### 16.2 Testebenen [E]

|Ebene|Werkzeug|Gegenstand|
|---|---|---|
|Einheit|`cargo test`|reine Funktionen, Parser, Kodierer, Ableitungen|
|Eigenschaft|`proptest`|Kodierung Hin und Zurück, `no_shared_dek`, Roaring-Schnitte, Festkomma-Arithmetik, Chunking-Determinismus, Mandantentrennung|
|Nebenläufigkeit|`loom`|Sperrreihenfolge (3.6), Gate-Zeigertausch, Schnappschüsse, Rotation gegen Schlüsseloperationen|
|Fehlerinjektion|`FaultVfs`|Absturz an jedem Durabilitätspunkt (Anhang C), zerrissene Schreibvorgänge, `FsyncFailed`, `ENOSPC`, Lesefehler|
|Fuzzing|`cargo-fuzz`|ein Ziel je Plattenformat (Journal, Segment, Manifest, Tresor, Gate-Log, Ledger), Worker-Protokoll, jeder Parser|
|Compile-Fail|`trybuild`|Z13 (`Derived<T>` ohne Lineage), `Validated<T>`, `Sealed<P>`, Schlüsseltypen ohne `Default`/`Clone`/`Debug`/`Serialize`|
|Golden|Dateien im Repository|jedes Plattenformat je `format_version`, kanonische Objektformate, Berichtsformat, Festkomma-Vektoren `T-TT-FX`|
|Integration|Anwendungsfälle|Lebenszyklen aus Kapitel 19, Ende-zu-Ende mit Doubles für Inferenz|
|Mutation|`cargo-mutants`|Löschkern, Gate, Tresor, Restore-Schutz: überlebende Mutanten sind Defekte|
|Plattform|CI auf x86_64 und aarch64|Z7 byte-identisch über beide Architekturen|
|Last und Messung|eigene Benchmarks|[Q]-Werte aus 15.3, 9.3, 9.6|
|Sicherheit|extern|Penetrationstest (API, Parser, Worker-Sandbox), Krypto-Audit|

### 16.3 Zusagenmatrix [E]

Jede Zusage hat mindestens die folgenden Tests; die vollständige Familienliste steht in Anhang D.

|Zusage|Primäre Tests|
|---|---|
|Z1|`T-GATE-<Lesepfad>` (Suche, Antwort, `as_of`, Szenario, Cache, Export, Vorschau, Sitzung, Streaming, `Superseded`); `T-CONC-GATE` (`loom`: Linearisierung)|
|Z2|`T-CRY-1` (Bytes-Grep über alle Dateien mit bekannten Klartexten und Kennungen), `T-DERIVED-SEALED`, `T-META-*` (keine Klartext-Tags, keine exakten Längen)|
|Z3|`T-MULTI-*`: Trockenlauf, jeder Konfliktmodus, Zwischenzustand `Restricted(MultiSubjectReview)`, Sammelakte, `AddSubject`|
|Z4|`T-CRASH-*` für jeden Punkt C01 bis C27|
|Z5|`T-SEP-1` (K enthält keine Muster aus dem Personenkorpus), `T-ANON-*`, Compile-Fail `Validated`|
|Z6|`T-VCS-ERASE-1`: Objekt-IDs, Commit-Hashes und K-Ledger vor und nach einer P-Löschung identisch|
|Z7|`T-TT-*` (x86_64 und aarch64), `T-TT-FX` (Festkomma-Goldvektoren), `T-TT-REDACTED`, `ppr_reference_validation`, `proptest_csr_invariants`|
|Z8|`T-DEC-*`: Begründungskette nach Löschung vollständig, personenbezogene Nutzlast `Erased`|
|Z9|`T-SCEN-*`: `main`, P und Zugriffsrechte vor und nach dem Szenario identisch|
|Z10|`T-NOFALLBACK-*`, `T-RNG-*` (Entropiefehler erzeugt weder Umschlag noch Schlüssel)|
|Z11|`T-EVID-*`: jedes Messverfahren mit absichtlich verstümmeltem Scanner muss scheitern|
|Z12|`T-REST-*`: mit Senke, nur mit Ankerstand, ohne beides (`Quarantine`), außerhalb Horizont, zurückgerolltes K-Ledger|
|Z13|Compile-Fail `derived_requires_lineage`|
|Z14|Eigenschaft `no_shared_dek` über `VerRef`|
|Z15|`T-LOG-*`: Canary-Kennungen in Journalen, Ledgern, Fehlertexten, Metriken, Supportpaket|
|Z16|`T-ACL-*`, `T-ACL-RANK` (Hinzufügen, Entfernen, Ändern eines unzugänglichen Dokuments ändert Ergebnisse, Zählwerte, Fehlerbilder und Zeitklassen byte-genau nicht), `ppr_snapshot_isolation`|
|Z17|`T-GROUND-*` (Struktur, D); Goldset-Messung (S, 9.6)|
|Z18|`T-EGRESS-*`: Netzblockade im Standard, Grant-Prüfung, Ablauf|
|Z19|`T-VENDOR-*`, `T-VENDOR-BUNDLE`|

### 16.4 Crash-Matrix und Positivkontroll-Tests [E]

Für jeden Durabilitätspunkt `C-nn` läuft ein Szenario: Zustand aufbauen, am Punkt abbrechen (`FaultVfs`), neu starten, Invarianten prüfen:

- **I1** Eine durable Sperre geht nie verloren.
- **I2** Kein Lesepfad liefert Klartext eines gesperrten Ziels.
- **I3** Ein Schlüssel geht nie ohne Absicht verloren (R-KEY-FIRST).
- **I4** Jede Saga ist vollendet oder `Pending` bei aktiver Sperre.
- **I5** Ledger-Ketten, Manifest und Gate-Log sind konsistent.
- **I6** Ordinale werden nie wiederverwendet.
- **I7** Nach `FsyncFailed` wird dieselbe Datei nicht als stabil angenommen (WP-7).

Zusätzlich je Messverfahren (8.5) ein Positivkontroll-Test: Das Verfahren wird deaktiviert oder verstümmelt, der Test MUSS dann scheitern. Ein Messverfahren ohne diesen Test liefert `NotImplemented`.

### 16.5 Messberichte und Freigabekriterien [E]

Alle [Q]-Werte (15.3, 9.3, 9.6, 4.3) sind Startwerte. Freigegeben wird eine Version, wenn:

1. alle Zusagen-Tests grün sind und G-ZUSAGE besteht;
2. die vollständige Crash-Matrix besteht und kein Mutant im Löschkern überlebt;
3. ein **Messbericht** (Referenzhardware, Korpus, Messdatum, Konfigurationsstand als Commit) jeden [Q]-Wert erfüllt oder die Abweichung mit Begründung und ADR ausweist;
4. `support_min` und die Regressionsgrenzen (9.7) aus dem Messbericht festgelegt sind (Pflichtparameter);
5. der Kryptobericht (4.7) und ein Penetrationstest (API, Parser, Sandbox) vorliegen;
6. die Freigabepflichten aus 20.4 erledigt sind;
7. ein Restore-Test (14.4) auf Produktionsumgebung mit Testmandant im laufenden Quartal bestanden ist.

Der Messbericht der Speicher-Engine-Prüfung (5.1, Entscheidungsregel Eigenbau) liegt vor Beginn der Speicherimplementierung vor.

### 16.6 Architektur-Gates (`cargo xtask gates`) [E]

|Gate|Prüfung|
|---|---|
|G1|Level-Graph gegen `rings.toml` und `cargo metadata` (negative Fixtures)|
|G2|Jeder Port hat im Betrieb genau eine Verdrahtung (HR-7)|
|G3|keine nicht injizierte Zeit- oder Zufallsquelle (`SystemTime::now`, `thread_rng`)|
|G-CRY|keine `Default`/`Clone`/`Debug`/`Serialize` auf Geheimnissen; kein `unwrap_or_default()` und kein `let _ =` auf `Result` in `acta-crypto`, `acta-vault`, `acta-seg`, `acta-erasure`, `acta-history`, `acta-pstore`|
|G-FS|Dateizugriffe nur in `acta-fs`|
|G-ERA|Löschungen nur über den Löschkern; Segmente, Indizes, Caches nur über `ErasureLayer`|
|G-READ|öffentliche Lesepfade nur über den gegateten Leser (`ErasedView`)|
|G-UNSAFE|`unsafe` nur in `acta-sys`, jeder Block mit `// SAFETY:`|
|G-NOTOKIO|kein `tokio` in Level 0 bis 7|
|G-NOSERDE-DISK|kein `serde`/`bincode` in Plattenformaten|
|G-HUB|Größenbudgets (3.3), HR-1 bis HR-9|
|G-WORD|Wortfilter (1.6) über Quelltext, Texte, Vorlagen und diese Spezifikation; Regel B über Vorlagen und Oberflächentexte|
|G-SUPPLY|`cargo-deny` (Lizenz, Advisories, Quellen), Audit kryptografischer Abhängigkeiten, SBOM, `cargo geiger`-Bericht|
|G-DEF|alle Crates in `default-members`|
|G-ZUSAGE|jede Zusage Z1 bis Z19 hat mindestens einen grünen Test|
|G-LINT|Produktionscode: `deny(clippy::unwrap_used, expect_used, panic, indexing_slicing, let_underscore_must_use)`|

---

