# Acta v2 – Spezifikation

**Fassung 0.2 – vollständiger Entwurf mit Zielwerten** · Stand 2026-11-06
Ersetzt Entwurf 0.1 und jede frühere Spezifikation (5.1.0, 6.0.0, generierte Schnittstellenspezifikation).

**Inhalt**

- 0. Status, Geltung, Beweisstand
- 1. Produkt
- 2. Bedrohungsmodell und Vertrauensdomänen
- 3. Globale Invarianten und Regeln
- 4. Kryptografie
- 5. Custodian (Key-Dienst)
- 6. Speicher
- 7. Datenpfad
- 8. Such- und RAG-Kern (nur RAM)
- 9. Löschprotokoll
- 10. Nebenläufigkeit
- 11. Öffentliche Schnittstelle
- 12. Konfiguration, Fehler, Beobachtbarkeit
- 13. Betrieb
- 14. Nachweis und Prüfung
- 15. Architektur und Crates
- 16. Tests
- 17. Gates, Plan, Zielwerte
- 18. Restrisiken und Grenzen
- 19. Befunde, Zuordnung, Änderungen
- 20. Offene Entscheidungen
- Anhang A – Konstanten (normativ)
- Anhang B – Testvektoren und Referenzprogramm (normativ)
- Anhang C – Zulässiger Wortlaut für Außenaussagen

---

## 0. Status, Geltung, Beweisstand

### 0.1 Was dieses Dokument ist

Die normative Spezifikation eines neu gedachten Produkts: ein verschlüsselter Wissensspeicher für personenbezogene RAG-Daten, bei dem das Löschen eines Betroffenen die Vernichtung seines Schlüssels bei einer unabhängigen Stelle ist und ein Dritter das mit eigenem Code nachprüfen kann.

Das Dokument baut nicht auf dem Altrepo `tfufuz1/acta` auf. Aus dem Altrepo und den neun eingereichten Analysen sind nur **Befunde** übernommen, keine Entscheidungen. Kapitel 19 ordnet jeden Befund einer Regel und einem Test zu.

### 0.2 Beweisstand (ehrlich)

| Aussage | Stand |
| --- | --- |
| Das Produkt ist gebaut | **nein** |
| Zahlen (Latenz, Größe, Zeiten) sind gemessen | **nein**, sie sind Zielwerte (gekennzeichnet **[Ziel]**) |
| Das Envelope-Format E1 und die Schlüsselableitung sind mit einer unabhängigen Implementierung (Python/libsodium) erzeugt und entschlüsselt worden | **ja**, Anhang B (ausgeführt am 2026-11-06) |
| Befunde am Bestandscode (Commit `34f00a2`) | teils von mir am Quelltext gelesen (**[V]**), teils aus den Analysen übernommen (**[A]**), siehe 19.1 |
| Kryptografie ist extern geprüft | **nein** (Gate G4) |
| Rechtswirkung der Schlüsselvernichtung ist geklärt | **nein** (Gate G5) |

**Keine Aussage dieses Dokuments ist ein Beleg für Fehlerfreiheit.** Fehlerfreiheit ist nicht beweisbar. Erreichbar ist ein Modell, dessen Verletzung einen Test rot macht (Kapitel 16), und eine externe Prüfung des kleinen Kerns (G4). Eine Spezifikation mit Messwerten heißt „Fassung 1.0“; bis dahin bleibt sie „Entwurf mit Zielwerten“.

### 0.3 Schreibweisen

- **MUSS / MUSS NICHT / SOLL / DARF** im Sinne von RFC 2119.
- IDs: `Z` Zusage, `I` Invariante, `C` Kryptoregel, `R` Restrisiko, `T` Test, `G` Gate, `CP` Crash-Punkt, `B` Bestandsbefund, `O` offene Entscheidung.
- Zahlen mit **[Ziel]** sind keine Zusagen. Zahlen mit **[fix]** sind Formatkonstanten und stehen genau einmal in Anhang A.
- Alle mehrbyteigen Zahlen in persistierten Formaten sind big-endian, Ausnahme: Gleitkommawerte im Klartext P1 (little-endian, 4.6).

### 0.4 Pflege der Spezifikation

1. Diese Datei liegt als `docs/SPEC.md` im Repository und wird **nicht gelöscht**. Test **T-DOC-1** schlägt fehl, wenn sie fehlt oder die Versionszeile nicht mit `Cargo.toml` übereinstimmt.
2. Jede Konstante aus Anhang A steht im Code genau einmal. Test **T-DOC-2** liest Anhang A und vergleicht jede Konstante mit dem Code.
3. Eine Änderung der Spezifikation ist ein Commit, der die Versionszeile und den Änderungsabschnitt (19.3) ändert. Formatänderungen erhöhen `version` im Format, nie still.
4. Abgeleitete Dokumente (API-Doku, Testlisten) sind als „abgeleitet“ gekennzeichnet und nie Quelle der Wahrheit.
5. **Stoppregel:** Kein neues Crate, keine neue Entscheidungs-ID, kein neues Dokument, bevor Gate G1 grün ist.

---

## 1. Produkt

### 1.1 Mission

**Acta ist ein verschlüsselter Wissensspeicher für personenbezogene RAG-Daten. Das Löschen eines Betroffenen besteht darin, dessen Schlüssel bei einer unabhängigen Stelle zu vernichten. Ein Dritter kann das mit eigenem Code nachprüfen.**

Suche (BM25, Vektor, Hybrid) ist Handelsware und macht den Speicher nutzbar. Das Alleinstellungsmerkmal ist ausschließlich: **Schlüsselhierarchie, Gate, extern prüfbare Vernichtung.**

### 1.2 Leitgedanke zum Nachweis

Ein Bericht, den ein System über die eigene Löschung ausstellt, ist Selbstauskunft und beweist nichts, auch nicht mit Signatur oder einem Prüfwerkzeug aus demselben Repository. Acta stellt deshalb **keinen Bericht als Beweis** aus. Der Nachweis besteht aus drei Dingen, die nicht vom Acta-Code abhängen:

1. **Vernichtungsbeleg des Key-Dienstes (Custodian)**, ausgestellt von einer anderen Partei als der Bibliothek.
2. **Ein Datenformat, das ein Auditor in rund 60 Zeilen eigenem Code lesen kann** (Standard-AEAD, Format auf einer Seite, Testvektoren, Anhang B).
3. **Ein vom Auditor selbst geführter Kanarien-Test** auf der echten Konfiguration (14.3).

### 1.3 Zusagen

Jede Zusage ist durch mindestens einen Test belegt (Kapitel 16). Ohne Test ist sie keine Zusage.

| ID | Zusage | Test |
| --- | --- | --- |
| **Z1** | Ab dem Zeitpunkt, an dem `erase()` zurückkehrt, liefert kein Lesepfad mehr Klartext des Ziels, und es wird nichts mehr in das Ziel geschrieben. Das gilt ab Zustand `Gated` in jedem Konfigurationsmodus. Leser, deren zweite Gate-Prüfung (10.3) vor dem Gate-Setzen lag, dürfen noch Daten erhalten. | T-DEL-1, T-CONC-1 |
| **Z2** | Alle persistierten Klartextdaten eines Betroffenen liegen in Envelopes, die unter einem Schlüssel versiegelt sind, der **nur beim Custodian** liegt. Ist er dort vernichtet, ist kein Envelope des Betroffenen mit den verbleibenden Schlüsseln entschlüsselbar. Diese Zusage gilt erst ab Zustand `Destroyed` (9.2); `erase()` meldet den erreichten Zustand wahrheitsgemäß. | T-DEL-1, T-CANARY-1 |
| **Z3** | Ein Dokument mit mehreren Betroffenen wird unlesbar, sobald **einer** von ihnen gelöscht wird. | T-MULTI-1 |
| **Z4** | Eine Unterbrechung (Prozess-Kill) an jedem nummerierten Punkt (16.4) führt nach Neustart zur Vollendung oder zu sichtbarem Status `Gated` mit `Pending`-Hinweis. Eine bestätigte Sperre geht nie verloren. | T-CRASH-n |
| **Z5** | Jeder Nachweis nennt seine **Attestierungsklasse** (14.4) und seine Grenzen (Kapitel 18). | T-EVID-1 |
| **Z6** | Kein Rückgabewert behauptet etwas, das nicht gemessen wurde. Nicht implementierte Funktionen liefern `Err(NotImplemented)`. | T-STUB-1 |
| **Z7** | Kein Fehler im Schlüssel-, Zufalls-, Ableitungs- oder Siegelpfad wird zu einem gültigen Wert (kein Nullschlüssel, keine leere Hülle, kein Ersatzschlüssel). | T-NOFALLBACK-1, T-RNG-1, T-SEAL-* |

### 1.4 Zusagen, die Acta nicht macht

„DSGVO-konform“, „garantiert gelöscht“, „unwiederbringlich gelöscht“. Löschung im RAM, Swap, in Core-Dumps, Hibernation, in Backups (Custodian und Datenverzeichnis), bei externen Verarbeitern (LLM-Anbieter, Embedding-Dienst), in Prompts, Antworten, Logs und Caches des Aufrufers. Prüfung von Aufbewahrungspflichten (Art. 17 Abs. 3): Das ist Aufgabe des Aufrufers vor `erase()`.

**Zulässiger Wortlaut für Außenaussagen** (Anhang C): *„Acta vernichtet den Schlüssel des Betroffenen beim Key-Dienst und macht dessen gespeicherte Daten dadurch unlesbar. Ob das im Einzelfall eine Löschung nach Art. 17 DSGVO darstellt, beurteilt der Verantwortliche.“* Jede Außenaussage MUSS die Attestierungsklasse (14.4) nennen und DARF NICHT stärker sein als sie.

### 1.5 Nicht-Ziele Version 1

Server, Netzwerkprotokoll im Kern, Replikation, eigener Speicher-Engine-Bau (LSM, WAL, MVCC/SSI, Zeitreisen), persistierter Suchindex, HNSW, Embedding-Erzeugung, LLM-Aufrufe, Dateiformat-Parsing, Windows, Schlüssel-Escrow durch den Hersteller, Telemetrie, Prozesshärtung durch die Bibliothek (Core-Dump-Verbot, `mlock`), Einschränkung der Verarbeitung nach Art. 18 (`Restrict`/`Lift`, vorgesehen für 1.1), Block-Verschlüsselung, Berichtigung per Schlüsselrotation, automatische Personenerkennung.

---

## 2. Bedrohungsmodell und Vertrauensdomänen

```
  Domäne A: Anwendung + Acta-Prozess     (Klartext nur im RAM)
  Domäne B: Datenträger / Datenverzeichnis    (nur Ciphertext + Metadaten)
  Domäne C: Custodian (Key-Dienst)            (hält Schlüssel, vernichtet, führt eigenes Audit-Log)
  Domäne D: Auditor                           (eigener Code, eigener Zugang zu B und C)
```

| ID | Angreifer / Prüfer | Fähigkeit | Acta MUSS |
| --- | --- | --- | --- |
| W1 | Kopie von Domäne B zu beliebigem Zeitpunkt, auch vor dem Löschen | liest alle Bytes von B | nach Vernichtung der Schlüssel unlesbar halten (Ausnahme R7) |
| W2 | Kopie von B nach dem Löschen plus **alle dann existierenden Schlüssel** in C | liest B und alle verbleibenden Schlüssel | Daten des gelöschten Betroffenen unlesbar halten |
| W3 | Betreiber, der Löschung behauptet, ohne zu löschen | kontrolliert A und B | durch Custodian-Beleg und Auditor-Test aufdeckbar machen |
| W4 | Absturz oder I/O-Fehler an beliebiger Stelle | Prozess stirbt, `fsync` schlägt fehl | fail-closed, Wiederaufnahme, keine verlorene bestätigte Sperre |
| W5 | Manipulation von B (Zeilen ändern, Backup älterer Stand einspielen) | schreibt in B | nie still falsche Klartexte liefern (AEAD), Manipulation der Löschhistorie erkennbar machen, Wiedereinspielen älterer Stände per Abgleich mit dem Custodian und dem Löschregister (13.4) auffangen |
| W6 | Zwei Mandanten oder Betroffene im selben Prozess | gegenseitige Einflussnahme | Isolation (I-12), kein gemeinsamer veränderlicher Zustand (I-13) |
| P | Auditor ohne Vertrauen in Acta-Code | prüft mit eigenem Code | Format, Beleg und Testverfahren so liefern, dass ohne unseren Code geprüft werden kann |

**Nicht im Modell** (jeweils als Restrisiko in Kapitel 18 geführt): Angreifer mit Zugriff auf den laufenden Prozessspeicher (R1), Angreifer mit Zugriff auf Backups des Custodians (R2), kompromittierter Custodian vor der Vernichtung (R7), kryptografische Brechung (R3), Seitenkanäle auf Hardwareebene.

---

## 3. Globale Invarianten und Regeln

### 3.1 Invarianten

Jede Invariante ist testbar. Die Tests stehen in Kapitel 16 und tragen die Invariantennummer.

| ID | Invariante |
| --- | --- |
| **I-1** | **Gate-Lesegarantie.** Jede Lesefunktion prüft den Gate-Zustand aller beteiligten Betroffenen und des Mandanten **vor und nach** der Entschlüsselung. Liefert eine der Prüfungen „nicht aktiv“, wird kein Klartext zurückgegeben. |
| **I-2** | **Fail-closed.** Jeder Fehler im Schlüssel-, Gate- oder Siegelpfad führt zu Verweigerung, nie zu einem Ersatzpfad. |
| **I-3** | **Keine Schlüssel im Datenverzeichnis.** Weder `TK_t` noch `DK_s` noch ein davon ableitbares Material wird in Domäne B geschrieben, auch nicht temporär, nicht in Logs, Fehlertexten, SQLite-Temp-Dateien oder dem erase_log. |
| **I-4** | **Kein ableitbares Geheimnis.** Jedes Geheimnis stammt aus dem CSPRNG. Kein Schlüssel wird aus öffentlichen oder konstanten Daten abgeleitet. Es gibt keine Konstanten, die als Schlüssel oder Schlüsselmaterial dienen. |
| **I-5** | **Kein Fehler wird zum gültigen Wert.** Funktionen, die scheitern können, liefern `Result`. Kein `unwrap_or_default`, kein `Vec::new()` als Fehlerwert, kein Nullschlüssel, kein Zufall aus Nullen. |
| **I-6** | **Gate vor Destroy.** `custodian.destroy` wird nur für ein Ziel aufgerufen, dessen Gate-Zustand durabel gesetzt ist. |
| **I-7** | **Destroyed nur nach Gegenprobe.** Der Zustand `Destroyed` entsteht nur, wenn `custodian.status` `Destroyed` oder `Disabled` meldet **und** ein Probeöffnen eines Envelopes des Ziels mit `KeyGone` scheitert. |
| **I-8** | **Completed nur nach Destroyed.** `Completed` entsteht nur aus `Destroyed` nach vollständigem Purge und Log-Eintrag. |
| **I-9** | **Bestätigte Sperre ist durabel.** Eine Sperre ist erst sichtbar (RAM) und bestätigt (Rückgabe), nachdem ihr Commit per `fsync` durabel ist. |
| **I-10** | **Monotone Zustände.** Ein Ziel kehrt nie von `Gated`, `Destroyed` oder `Completed` nach `Active` zurück. Ein gelöschter `subject_ref` wird nie wiederverwendet. |
| **I-11** | **Lesbarkeitsformel.** Ein Dokument ist lesbar genau dann, wenn der Mandant `Active` ist, **alle** seine Betroffenen `Active` sind und alle zugehörigen Schlüssel beim Custodian vorhanden sind. |
| **I-12** | **Isolation.** Eine Operation auf einem Ziel verändert die Bytes von Envelopes nicht beteiligter Betroffener und Mandanten nicht. |
| **I-13** | **Kein veränderlicher globaler Zustand.** Keine `static`-Variablen mit Innenveränderlichkeit, kein `lazy_static` mit Zustand, kein globaler Cache. Zwei `Acta`-Instanzen im selben Prozess beeinflussen sich nicht. |
| **I-14** | **Keine stillen Korrekturen.** Überschrittene Limits, ungültige Konfiguration, ungültige Eingaben liefern einen benannten Fehler. Keine stille Kürzung, kein Aliasing von Zahlenbereichen, kein stilles Auffüllen. |
| **I-15** | **Wahrhaftige Rückgaben.** Jede Statusangabe (`health`, `state`, Zähler) ist aus gemessenem Zustand berechnet, nie konstant. |
| **I-16** | **Keine Personendaten in Beobachtbarem.** Logs, Fehlertexte, Metriken und Observer-Ereignisse enthalten keine Klartexte, Schlüssel, `subject_ref`, `tenant_id` oder Pfade mit Personenbezug. Zulässig: `erase_id`, Zähler, Zustandsnamen. |
| **I-17** | **Ein Prozess je Datenverzeichnis**, erzwungen durch Dateisperre. |
| **I-18** | **Identifikatoren sind Bytes.** Keine verlustbehaftete Umwandlung (`from_utf8_lossy`, Kürzen, Auffüllen) von Identifikatoren oder Geheimnissen. |

### 3.2 Verbotene Muster (Bestandsfehler, als Lint oder Architekturtest durchgesetzt)

| ID | Verbot | Durchsetzung |
| --- | --- | --- |
| V-1 | `unwrap`, `expect`, `panic!`, `unreachable!`, `todo!`, `unimplemented!`, Indexzugriff `a[i]` in Produktionscode | Clippy `deny` |
| V-2 | `let _ = ` auf einem `Result` | Clippy `let_underscore_must_use` = `deny` |
| V-3 | `String::from_utf8_lossy`, `unwrap_or_default` auf Fehlerpfaden, `as`-Casts, die abschneiden können | `clippy.toml` `disallowed-methods`, `cast_possible_truncation = deny` |
| V-4 | Öffentliche Konstruktoren für Typen, die eine Wahrheitsaussage tragen (`Destroyed`-Beweis, `Completed`) | `trybuild`-Compile-Fail-Tests (T-TYPE-1) |
| V-5 | `impl Default`, `Clone`, `Copy`, `Display`, `serde::Serialize` für Schlüsseltypen | T-TYPE-2 |
| V-6 | Typen gleichen Namens in zwei Crates | Architekturtest T-ARCH-3 |
| V-7 | `static` mit `Mutex`, `RwLock`, `Atomic*`, `OnceCell`, `static mut` | Architekturtest T-ARCH-4 (syn-Scan) |
| V-8 | Cargo-Features, die Format, Durabilität, Verschlüsselung oder Löschverhalten ändern | T-ARCH-5 (Feature-Liste ist Whitelist) |
| V-9 | Pfadabhängigkeiten außerhalb des Workspace, `[patch]`, Git-Abhängigkeiten | `cargo deny` |
| V-10 | Rückwärtskompatibilitäts- und Legacy-Pfade ohne Nutzer | Review-Regel, `grep legacy` im CI |
| V-11 | Mock oder Attrappe im Datenpfad eines Integrationstests der Gates G1 und G2 | Testregel 16.1 |

---

## 4. Kryptografie

### 4.1 Primitive und Bibliotheken

| Zweck | Verfahren | Bibliothek (Vorschlag, Version wird gepinnt) |
| --- | --- | --- |
| Authentisierte Verschlüsselung | **XChaCha20-Poly1305** (192-Bit-Nonce, 256-Bit-Schlüssel, 128-Bit-Tag) | RustCrypto `chacha20poly1305` |
| Schlüsselableitung | **HKDF-SHA256** (RFC 5869) | RustCrypto `hkdf`, `sha2` |
| Hash (Log-Kette, Manifest) | **SHA-256** | `sha2` |
| Zufall | OS-CSPRNG | `getrandom` |
| Löschen aus dem RAM | `Zeroize` | `zeroize` |

**Verpflichtung vor Gate G4:** Prüfen, welchen Prüfstatus die gewählten Bibliotheksversionen haben und welche Advisories vorliegen (`cargo audit`, `cargo deny`). Diese Prüfung ist **nicht** Teil dieses Dokuments und von mir nicht ausgeführt. Die Chiffrenwahl (O2) ist unter dem Vorbehalt der Prüfung festgelegt (Kapitel 20).

**Warum XChaCha20-Poly1305:** 192-Bit-Zufallsnonces machen Nonce-Kollisionen bei zufälliger Erzeugung praktisch unmöglich (4.10); die Chiffre ist in libsodium und gängigen Bibliotheken vorhanden, sodass ein Auditor mit eigener Implementierung prüfen kann; sie ist in Software ohne Datenabhängigkeiten im Zeitverhalten implementierbar.

### 4.2 Geheimnisse

| Name | Länge | Erzeugung | Speicherort | Lebensdauer |
| --- | --- | --- | --- | --- |
| `TK_t` (Mandantenschlüssel) | 32 Byte | CSPRNG | **nur im Custodian** | bis Mandantenlöschung |
| `DK_s` (Betroffenenschlüssel) | 32 Byte | CSPRNG | **nur im Custodian** | bis Betroffenenlöschung |
| `DEK` (Dokumentschlüssel) | 32 Byte | HKDF (4.4), nie gespeichert | nur RAM, innerhalb **eines** Aufrufs | je Siegeln/Öffnen |
| `MK` (Master des `LocalCustodian`) | 32 Byte | vom Aufrufer geliefert | nur RAM, vom Aufrufer verwaltet | Prozesslaufzeit |

Acta schreibt **keinen** `TK_t`, `DK_s` oder `DEK` in das Datenverzeichnis (I-3). Der DEK wird **nicht zwischengespeichert**: Er wird pro Operation abgeleitet und sofort genullt. Damit gibt es keinen Cache, der nach einer Löschung veralten kann.

### 4.3 Lebenszyklus der Schlüssel

1. **Erzeugung:** `Rng::fill` (fallibel, 4.7) → 32 Byte → `custodian.put`. Scheitert eine der Stufen, entsteht kein Schlüssel und der Aufruf scheitert mit Fehler (kein Ersatzwert).
2. **Laden:** Beim Öffnen werden `TK_t` je Mandant und `DK_s` je aktivem Betroffenen per `get_many` geholt und in `Secret`-Typen im RAM gehalten (`ZeroizeOnDrop`).
3. **Nutzen:** Ableitung eines DEK je Operation (4.4).
4. **Vernichten:** Gate setzen → RAM-Kopie nullen → `custodian.destroy` → Gegenprobe (Kapitel 9).
5. Kein Schlüssel wird rotiert, umgeschlüsselt oder exportiert. Schlüsselrotation ist Nicht-Ziel v1.

### 4.4 Dokumentschlüssel (normativ)

```
IKM  = TK_t ‖ DK_s1 ‖ DK_s2 ‖ … ‖ DK_sn          (n = 0..16; s_i aufsteigend nach subject_ref, bytewise)
PRK  = HKDF-Extract( salt = doc_id (16 B), ikm = IKM )     = HMAC-SHA256(key = doc_id, msg = IKM)
info = "ctx/v2/dek" ‖ 0x00 ‖ tenant_id (16) ‖ collection_id (16) ‖ n (1) ‖ s_1 ‖ … ‖ s_n (je 16) ‖ generation (4, BE)
DEK  = HKDF-Expand( PRK, info, L = 32 )
```

Eigenschaften und Begründung:

- **Mehrpersonen-Regel strukturell:** Fehlt ein `DK_si`, ist der DEK nicht berechenbar. Ein Dokument ohne Betroffene (`NonPersonal`) hängt nur an `TK_t`.
- **Mandantenlöschung:** Wird `TK_t` vernichtet, ist kein DEK des Mandanten mehr berechenbar, unabhängig von der Dokumentzahl.
- **Eindeutigkeit der Kodierung:** `TK_t` und alle `DK` haben feste Länge 32; `n` steht in `info`; alle Bezeichner haben feste Länge 16. Es gibt keine zwei verschiedenen Eingaben mit gleichem `IKM ‖ info`.
- **Bindung:** `info` bindet den Schlüssel an Mandant, Collection, die Menge der Betroffenen und die Generation. Ein Envelope, der in ein anderes Dokument oder einen anderen Mandanten verschoben wird, ist nicht entschlüsselbar (zusätzlich AAD, 4.5).
- **Wirksamkeit des Salzes:** Der Salt `doc_id` ist öffentlich. HKDF-Extract mit öffentlichem Salt ist zulässig; die Sicherheit trägt die Entropie des `IKM` (≥ 256 Bit aus `TK_t`).
- **Label** `ctx/v2/dek` ist das einzige Label für diese Ableitung. Weitere Labels: Anhang A.
- Die Ableitung ist in Anhang B mit einer unabhängigen Python-Implementierung gegengeprüft.

### 4.5 Envelope E1 (normativ, auditor-lesbar)

Alle Zahlen big-endian.

| Offset | Länge | Feld | Regel |
| --- | --- | --- | --- |
| 0 | 4 | `magic` | `"CTX2"` |
| 4 | 1 | `version` | `0x01` |
| 5 | 1 | `alg` | `0x01` = XChaCha20-Poly1305. Anderer Wert → `Corrupt(UnknownAlg)` |
| 6 | 1 | `n_subjects` | 0..16 |
| 7 | 1 | `reserved` | MUSS `0x00` sein, sonst `Corrupt` |
| 8 | 16 | `tenant_id` | |
| 24 | 16 | `collection_id` | |
| 40 | 16 | `doc_id` | |
| 56 | 4 | `generation` | beginnt bei 1, je `update` +1 |
| 60 | 16 × n | `subject_refs` | **streng aufsteigend** (bytewise), keine Duplikate |
| 60 + 16n | 24 | `nonce` | 24 Byte aus dem CSPRNG, je Siegeln neu |
| 84 + 16n | 4 | `ct_len` | Länge des Klartexts in Byte |
| 88 + 16n | `ct_len` + 16 | `ciphertext ‖ tag` | |

**AAD** = alle Bytes von Offset 0 bis einschließlich `ct_len` (also alles vor dem Chiffrat). Jede Veränderung des Headers lässt die Authentisierung scheitern.

**Parserregeln** (jede Verletzung ergibt `Corrupt`, nie einen Panic, nie stilles Kürzen):

1. Gesamtlänge ≥ 88 + 16n + 16, und genau `88 + 16n + ct_len + 16`.
2. `ct_len` ≤ `max_doc_bytes` + P1-Overhead, geprüft **vor** jeder Allokation.
3. `n_subjects` ≤ 16. `reserved` = 0. `version` und `alg` bekannt.
4. `subject_refs` streng aufsteigend.
5. Die Header-IDs (`tenant_id`, `collection_id`, `doc_id`) MÜSSEN mit dem Zeilenschlüssel in der Datenbank übereinstimmen, sonst `Corrupt(RowMismatch)`.
6. Ein fehlender Schlüssel (Custodian meldet `NotFound`/`Disabled`) ergibt `KeyGone`, **nicht** `Corrupt`.
7. Ein AEAD-Fehler bei vorhandenen Schlüsseln ergibt `Corrupt(Auth)`. Es wird nie ein Klartext zurückgegeben.
8. Der Parser hat ein Fuzz-Ziel (T-FUZZ-1).

**Maximale Größe des Containers:** 88 + 16·16 + 16 + `max_doc_bytes` + P1-Overhead.

### 4.6 Klartext P1 (innerhalb des Envelopes)

Deterministisch kodierte CBOR-Map (RFC 8949 §4.2), Schlüssel als Textstrings:

| Schlüssel | Typ | Pflicht | Inhalt |
| --- | --- | --- | --- |
| `"v"` | uint | ja | `1` |
| `"text"` | tstr | nein | Inhalt, UTF-8, ≤ `max_text_bytes` |
| `"meta"` | map | nein | flach: Schlüssel tstr ≤ 64 B, Wert tstr ≤ 1 KiB, int oder bool; ≤ 32 Einträge |
| `"emb"` | bstr | nein | `dim × 4` Byte, jede Zahl **IEEE-754 binary32, little-endian**; Länge MUSS `4 × dim` der Collection entsprechen |

Regeln: mindestens `"text"` oder `"emb"` vorhanden. Unbekannte Schlüssel → `Corrupt`. `NaN` und `±Inf` sind beim Schreiben **nicht zulässig** (`InvalidInput`). Bei der Metrik `Cosine` ist der Nullvektor nicht zulässig. Der Termvektor und andere abgeleitete Daten stehen **nicht** im Envelope; sie werden beim Öffnen aus `"text"` berechnet (Kapitel 8).

### 4.7 Zufall

```rust
pub trait Rng: Send + Sync {
    fn fill(&self, dest: &mut [u8]) -> Result<(), RngError>;   // niemals Nullen bei Fehler
}
```

- Die Produktionsimplementierung ruft `getrandom::fill` auf. Ein Fehler wird als `RngError` zurückgegeben; der aufrufende Pfad bricht ab (I-5, Z7). Es gibt **keinen** Wiederholungs-, Ersatz- oder Nullpfad.
- `Acta::open` hat **keinen** konfigurierbaren `Rng`. Ein austauschbarer `Rng` existiert nur in Testcode (`#[cfg(test)]` bzw. Testkit), nie in der öffentlichen Konfiguration. Damit kann ein deterministischer Generator nie in Produktion gelangen (Bestandsbefund B-09).
- Known-Answer-Tests nutzen eine Testfunktion mit festem Nonce (`#[cfg(test)]`), die nicht Teil der öffentlichen API ist.

### 4.8 Umgang mit Geheimnissen im RAM

- Alle Geheimnisse liegen in Typen `Secret<N>` mit `Zeroize + ZeroizeOnDrop`.
- `Secret` hat **kein** `Clone`, `Copy`, `Default`, `Display`, `Serialize`, kein `PartialEq` (Vergleiche per `ct_eq` nur in Tests) und ein `Debug`, das `[REDACTED]` ausgibt (T-TYPE-2).
- Zwischenpuffer mit Geheimnissen (HKDF-PRK, IKM, Klartext) werden vor Verlassen des Gültigkeitsbereichs genullt.
- Klartext-Dokumente verlassen den Prozess nur über die API-Rückgabe an den Aufrufer (R6).

### 4.9 Fehlerabbildung im Schlüsselpfad

| Situation | Ergebnis | Nie |
| --- | --- | --- |
| Custodian meldet `NotFound`/`Disabled` für einen benötigten Schlüssel | `KeyGone` | Ersatzschlüssel, Cache, Wiederholung |
| Custodian nicht erreichbar, Timeout, Protokollfehler | `CustodianUnavailable` | Interpretation als `KeyGone` oder als Erfolg |
| `Rng` scheitert | `Rng` | Nullen, Zeit-Seed |
| HKDF-Ausgabe nicht berechenbar (kann bei `L = 32` nicht auftreten) | `Crypto` (Fehler) | Nullschlüssel (Bestandsbefund B-05) |
| AEAD-Siegeln scheitert | `Crypto` (Fehler) | leerer Envelope (B-06) |
| AEAD-Öffnen scheitert | `Corrupt(Auth)` | Klartext, Teilklartext |
| Gate sagt „nicht aktiv“ | `Erased` | Klartext |

### 4.10 Nonce-Sicherheit (Rechnung)

Nonce: 192 Bit aus dem CSPRNG. Die Kollisionswahrscheinlichkeit nach `q` Siegelvorgängen unter **demselben** DEK beträgt ≈ `q² / 2¹⁹³`. Ein DEK gehört zu `(doc_id, Mandant, Collection, Betroffene, generation)`; je Generation wird genau einmal gesiegelt, ein Wiederholen desselben Siegelvorgangs (Retry nach Fehler) erzeugt eine **neue** Nonce. Praktisch gilt `q ≤ 2` je DEK. Selbst bei `q = 2³²` läge die Wahrscheinlichkeit bei ≈ `2⁻¹²⁹`. Ein Nonce-Zähler existiert nicht; damit entfällt die Gefahr der Zählerwiederverwendung nach Abbruch (Prüfbericht MODELL-9).

### 4.11 Kryptoregeln (Zusammenfassung)

| ID | Regel |
| --- | --- |
| **C-1** | Nur XChaCha20-Poly1305 siegelt Daten. Kein XOR, kein selbstgebauter Strom, keine Eigenentwicklung eines Algorithmus. |
| **C-2** | Jeder Siegelvorgang nutzt eine frische 192-Bit-Zufallsnonce und authentisiert den gesamten Header als AAD. |
| **C-3** | Jedes Geheimnis stammt aus dem CSPRNG; es gibt keine Konstanten als Schlüssel, keine Ableitung aus Tags, Pfaden oder Zeit. |
| **C-4** | Es gibt **keinen** Fallbackpfad bei Schlüsselfehlern. |
| **C-5** | Jede Funktion im Krypto-Pfad liefert `Result`; kein Fehler wird zu einem Wert. |
| **C-6** | Domänentrennung: jedes Label (Anhang A) wird für genau einen Zweck verwendet. |
| **C-7** | Der DEK wird nicht zwischengespeichert und nach der Operation genullt. |
| **C-8** | Das Format ist in Anhang B mit unabhängigem Code reproduzierbar; ein Test vergleicht beide (T-SEAL-1). |

### 4.12 Prüfliste für die externe Kryptoprüfung (Gate G4)

Der Prüfer SOLL mindestens beantworten:

1. Ist die Kodierung von `IKM` und `info` eindeutig (4.4)? Gibt es eine Kollision zwischen verschiedenen Betroffenenmengen?
2. Reicht der öffentliche Salt `doc_id` aus, und ist die Bindung von Mandant, Collection, Betroffenen und Generation in `info` plus AAD vollständig?
3. Ist die Zufallsnonce je Siegelvorgang unter jeder denkbaren Betriebsfolge (Retry, Absturz, Parallelität) frisch?
4. Ist eine Schlüsselbindung (Key Commitment) nötig? Poly1305-AEADs sind nicht schlüsselbindend; hier wählt ein Angreifer die Schlüssel nicht (4.2), der Prüfer bewertet das.
5. Verbleiben Geheimnisse (PRK, IKM, Klartext) in Puffern oder Stack-Kopien nach der Operation?
6. Ist die Fehlerabbildung (4.9) vollständig, also kein Pfad, der bei Fehler einen Wert statt eines Fehlers liefert?
7. Verhalten bei Fehler des `Rng`, bei Abbruch des Aufrufers (Cancel), bei teilweise geschriebenem Envelope.
8. Sind die Zeitverhalten der Vergleiche und der AEAD-Aufrufe unabhängig von Geheimnissen (Bibliotheksaussage, nicht Eigenbau)?
9. Stimmen Spezifikation, Referenzimplementierung (Anhang B) und Rust-Code auf allen Testvektoren überein?
10. Algorithmenlaufzeit: Aussage zur Langzeitvertraulichkeit (R3).

### 4.13 Grenzen der Kryptografie

Verborgen werden Klartext, Metadaten innerhalb von P1 und Embeddings. **Nicht** verborgen: `tenant_id`, `collection_id`, `doc_id`, `subject_refs`, `generation`, Größen, Zeitstempel der Zeilen, Zugriffsmuster (R4). Embeddings und Terme existieren im RAM im Klartext (R1). Ciphertext bleibt dauerhaft auf Medien; Vertraulichkeit hängt an der Stärke von XChaCha20-Poly1305 und der Vernichtung der Schlüssel (R3).

---

## 5. Custodian (Key-Dienst)

### 5.1 Aufgabe

Der Custodian speichert geheime 32-Byte-Datensätze unter einer `KeyId` und kann sie **vernichten**. Er ist die Partei, die den Beleg der Vernichtung ausstellt. Acta selbst hält keine dauerhafte Kopie (I-3).

### 5.2 Schnittstelle (normativ)

```rust
pub struct KeyId(String);   // Zeichen [a-z0-9/], Länge ≤ 160; nur über Konstruktoren dieses Crates bildbar

pub enum KeyLookup { Present(Secret<32>), Gone }          // Gone = NotFound oder Disabled
pub enum KeyStatus { Present, Disabled, Destroyed, Unknown }

pub enum CustodianError {
    NotFound, Disabled,            // nur von get()
    Conflict,                      // put() auf bestehende KeyId
    Unavailable,                   // Netz, Timeout, Dienst down
    Denied,                        // Berechtigung
    Protocol(u16),                 // unerwartete Antwort, Entschlüsselungs-/Authentisierungsfehler des Adapters
}

pub struct DestroyReceipt {
    pub custodian_ref: String,         // Kennung des Eintrags beim Custodian
    pub custodian_time_ms: Option<u64>,// Zeit laut Custodian, falls verfügbar
    pub scheduled_final_ms: Option<u64>,// bei Karenzfrist: geplanter endgültiger Löschzeitpunkt
    pub raw: Vec<u8>,                  // Originalquittung (z. B. signiert), unverändert
}

pub trait Custodian: Send + Sync {
    fn put(&self, id: &KeyId, secret: &Secret<32>) -> Result<(), CustodianError>;      // nur anlegen
    fn get(&self, id: &KeyId) -> Result<Secret<32>, CustodianError>;
    fn get_many(&self, ids: &[KeyId]) -> Result<Vec<KeyLookup>, CustodianError>;      // Reihenfolge wie ids
    fn destroy(&self, id: &KeyId) -> Result<DestroyReceipt, CustodianError>;           // idempotent
    fn status(&self, id: &KeyId) -> Result<KeyStatus, CustodianError>;
    fn attestation_class(&self) -> AttestationClass;                                   // A0 | A1 | A2
}
```

### 5.3 Vertragsregeln

| Nr. | Regel |
| --- | --- |
| K-1 | `put` legt nur an. Existiert die `KeyId`, liefert `put` `Conflict`, nie ein stilles Überschreiben. |
| K-2 | `get` liefert `NotFound` oder `Disabled`, wenn der Schlüssel nicht lesbar ist. **Jeder andere Fehler** (Netz, Timeout, Auth, Protokoll) ist `Unavailable`, `Denied` oder `Protocol` und wird von Acta **nie** als „Schlüssel weg“ gedeutet. |
| K-3 | `get_many` liefert für jede `KeyId` genau einen Eintrag in derselben Reihenfolge. Scheitert der Transport, scheitert der ganze Aufruf (`Err`), es gibt keine Teilergebnisse mit Lücken. |
| K-4 | `destroy` ist idempotent: Ein zweiter Aufruf auf einen vernichteten Eintrag liefert `Ok`. `destroy` auf eine **unbekannte** `KeyId` liefert `Err(NotFound)`, nie einen Beleg. |
| K-5 | Nach erfolgreichem `destroy` liefert `get` `NotFound` oder `Disabled`, und `status` liefert `Destroyed` oder `Disabled`. |
| K-6 | Dienste mit Karenzfrist beim endgültigen Löschen (KMS-ähnlich) wenden `Disabled` sofort an und planen die endgültige Löschung. Der Beleg weist beide Zeitpunkte aus (`custodian_time_ms`, `scheduled_final_ms`). Bis zum Termin gilt `Disabled`, nicht `Destroyed`. Acta akzeptiert beide Status für den Zustand `Destroyed` (I-7), weist die Karenz aber im Nachweis aus. |
| K-7 | Ein Adapter gibt `Secret` nie in Logs, Fehlertexten oder Debug-Ausgaben aus. |
| K-8 | Jeder Adapter hat eigene Zeitlimits und blockiert nie unbegrenzt. Acta übergibt keine Abbruchsignale an den Adapter. |

### 5.4 Namensraum der Schlüssel-IDs

```
Mandant:     ctx2/<store_id hex32>/t/<tenant_id hex32>
Betroffener: ctx2/<store_id hex32>/s/<tenant_id hex32>/<subject_ref hex32>
```

`store_id` (16 Byte Zufall) entsteht beim Anlegen des Datenverzeichnisses und steht in der Tabelle `meta` (6.3). Der Präfix verhindert Kollisionen, wenn mehrere Stores denselben Custodian nutzen.

### 5.5 Anlegen und Adoption (Absturzsicherheit von `register_subject`, `create_tenant`)

Erzeugt Acta einen Schlüssel, läuft die Reihenfolge so:

1. `custodian.status(id)`. Ist der Status `Destroyed` oder `Disabled`, wird die Registrierung mit `Erased` abgelehnt (kein Wiederverwenden eines gelöschten Bezeichners, I-10). Auch dann, wenn die Datenbank diesen Eintrag nicht kennt (Datenbankverlust, Restore).
2. Ist der Status `Present` und die Datenbank kennt den Eintrag nicht, ist es ein **Waisenschlüssel eines abgebrochenen Registrierversuchs** (CP-R2). Acta **adoptiert** ihn: `get`, dann Datenbankzeile anlegen. Es wird kein neuer Schlüssel erzeugt.
3. Ist der Status `Unknown`: `Rng::fill` → `custodian.put` → Datenbank-Commit.

Ein Waisenschlüssel ohne Datenbankzeile ist unschädlich (R13).

### 5.6 `LocalCustodian` (Klasse A0, nur Entwicklung und Einzelplatz)

- Crate `acta-custodian-local`. Der Aufrufer übergibt einen 32-Byte-Hauptschlüssel `MK` (aus OS-Keyring, Hardware oder Passphrase-Ableitung **in seinem eigenen Code**). Acta implementiert keine Passphrase-Ableitung und keinen Keyring-Zugriff.
- Verzeichnis `local_custodian_dir`, das **nicht** im Datenverzeichnis liegen darf (kanonische Pfade werden geprüft, sonst `Config`-Fehler).
- Eintrag `<sha256(key_id) hex>.k`: `version(1) ‖ nonce(24) ‖ XChaCha20-Poly1305(MK, nonce, aad = "ctx2/local/" ‖ key_id, secret)`.
- `destroy`: Datei mit Zufallsbytes gleicher Länge überschreiben, `fsync`, `unlink`, Verzeichnis-`fsync`, danach Marker `<hash>.gone` anlegen und `fsync`. `status` = `Destroyed` genau dann, wenn der Marker existiert und die `.k`-Datei fehlt.
- Ein falscher `MK` liefert `Protocol(AuthFail)`, nie `NotFound`.
- **Klasse A0: Selbstauskunft. Der Nachweis enthält keine Löschaussage, nur einen Funktionstest.** Auf Flash und Copy-on-Write ist das Überschreiben nicht belastbar.

### 5.7 Externe Adapter (nicht im Kern)

Produktive Wege liegen in getrennten Crates (`acta-custodian-<name>`), damit der Kern netzwerkfrei bleibt (Nicht-Ziel 1.5). Erster produktiver Adapter: Key-Dienst der Art Vault-KV/KMS. Jeder Adapter MUSS die Konformitätssuite T-CUST-1 bestehen. Acta enthält nur die Vorlage und die Suite, keine Dienstbibliothek.

### 5.8 Konformitätssuite T-CUST-1 (für jede Implementierung)

1. `put` → `get` liefert identische Bytes. 2. Zweites `put` → `Conflict`. 3. `destroy` → `get` = `NotFound`/`Disabled`, `status` ∈ {`Destroyed`, `Disabled`}. 4. Zweites `destroy` → `Ok`. 5. `destroy` auf unbekannte ID → `NotFound`. 6. `get_many` mit gemischten IDs liefert Reihenfolge und Zuordnung korrekt. 7. Ausfall des Transports liefert `Unavailable`, nie `NotFound`. 8. Falsche Berechtigung liefert `Denied`. 9. Der Beleg enthält `custodian_ref`. 10. Nach Prozessneustart bleibt ein vernichteter Eintrag vernichtet. **Mutantenregel:** Ein Adapter, der `destroy` nur zusagt, aber nichts tut, MUSS Test 3 oder 10 rot machen.

---

## 6. Speicher

### 6.1 Prinzip

Der Speicher sieht nur Envelopes und Zeilen mit Metadaten. Er muss nichts beweisen. Deshalb baut Acta **keinen eigenen Speicher**. Standard ist **SQLite**. Die Löschsemantik ist von der Engine unabhängig, weil nur Ciphertext in ihr liegt.

```rust
pub trait BlobStore: Send {
    fn begin_write(&self) -> Result<WriteTx, StoreError>;        // genau ein Schreiber (Schreib-Mutex, Kapitel 10)
    fn read(&self) -> Result<ReadTx, StoreError>;
}
// WriteTx: put_doc, delete_docs_of_subject_chunk, set_state(target, state, erase_id), append_log(entry),
//          insert_manifest_rows, … ; commit(self) -> Result<(), StoreError>   // kehrt erst nach durablem fsync zurück
```

Die Schnittstelle ist klein und synchron. Die Konformitätssuite **T-STORE-1** prüft jede Implementierung: ein Commit überlebt `kill -9`; ein nicht festgeschriebener Commit ist nach Neustart nicht sichtbar; `commit` meldet einen Fehler, wenn `fsync` scheitert.

### 6.2 SQLite-Konfiguration (verbindlich)

| Pragma | Wert | Zweck |
| --- | --- | --- |
| `journal_mode` | `WAL` | atomare Commits |
| `synchronous` | `FULL` | Commit erst nach `fsync` des WAL (D1) |
| `fullfsync`, `checkpoint_fullfsync` | `ON` | `F_FULLFSYNC` auf macOS |
| `secure_delete` | `ON` | freigegebene Seiten werden genullt (Hygiene, keine Zusage) |
| `locking_mode` | `EXCLUSIVE` | kein zweiter Prozess |
| `mmap_size` | `0` | kein `mmap` (D3) |
| `foreign_keys` | `ON` | Referenzen |
| `trusted_schema` | `OFF` | keine fremden Funktionen |
| `cell_size_check` | `ON` | Beschädigung früh erkennen |
| `application_id` | `0x43545832` (`CTX2`) | Dateikennung |
| `user_version` | `2` | Schemaversion; abweichend → `Corrupt(Schema)` |

Die Bibliothek wird als `bundled` mit festem Versionspin eingebunden, Mindestversion SQLite 3.40 (für `STRICT`). Die gebündelte C-Bibliothek ist eine Abhängigkeit mit `unsafe` im Fremdcode (R15); `#![forbid(unsafe_code)]` gilt für alle eigenen Crates.

### 6.3 Schema (normativ)

```sql
CREATE TABLE meta (
  key   TEXT PRIMARY KEY,
  value BLOB NOT NULL
) STRICT;                         -- 'store_id' (16 B), 'created_ms', 'schema' = 2

CREATE TABLE tenants (
  tenant_id BLOB PRIMARY KEY CHECK (length(tenant_id) = 16),
  state     INTEGER NOT NULL CHECK (state IN (0,1,2,3)),   -- 0 Active, 1 Gated, 2 Destroyed, 3 Completed
  erase_id  BLOB CHECK (erase_id IS NULL OR length(erase_id) = 16),
  receipt   BLOB,                                           -- DestroyReceipt.raw
  pending   INTEGER                                         -- NULL oder PendingReason
) STRICT, WITHOUT ROWID;

CREATE TABLE subjects (
  tenant_id   BLOB NOT NULL CHECK (length(tenant_id) = 16),
  subject_ref BLOB NOT NULL CHECK (length(subject_ref) = 16),
  state       INTEGER NOT NULL CHECK (state IN (0,1,2,3)),
  erase_id    BLOB CHECK (erase_id IS NULL OR length(erase_id) = 16),
  receipt     BLOB,
  pending     INTEGER,
  PRIMARY KEY (tenant_id, subject_ref)
) STRICT, WITHOUT ROWID;

CREATE TABLE collections (
  tenant_id     BLOB NOT NULL CHECK (length(tenant_id) = 16),
  collection_id BLOB NOT NULL CHECK (length(collection_id) = 16),
  name          TEXT NOT NULL,
  dim           INTEGER NOT NULL CHECK (dim BETWEEN 0 AND 4096),   -- 0 = nur Text
  metric        INTEGER NOT NULL CHECK (metric IN (0,1)),          -- 0 Cosine, 1 Dot
  analyzer      TEXT NOT NULL,                                     -- z. B. 'a1', 'a1-de'
  filterable    BLOB NOT NULL,                                     -- CBOR-Array der Meta-Schlüssel
  PRIMARY KEY (tenant_id, collection_id),
  UNIQUE (tenant_id, name)
) STRICT, WITHOUT ROWID;

CREATE TABLE docs (
  tenant_id     BLOB NOT NULL CHECK (length(tenant_id) = 16),
  collection_id BLOB NOT NULL CHECK (length(collection_id) = 16),
  doc_id        BLOB NOT NULL CHECK (length(doc_id) = 16),
  generation    INTEGER NOT NULL CHECK (generation >= 1),
  tombstone     INTEGER NOT NULL DEFAULT 0 CHECK (tombstone IN (0,1)),
  created_ms    INTEGER NOT NULL,
  updated_ms    INTEGER NOT NULL,
  envelope      BLOB NOT NULL,
  PRIMARY KEY (tenant_id, collection_id, doc_id)
) STRICT, WITHOUT ROWID;

CREATE TABLE doc_subjects (
  tenant_id     BLOB NOT NULL,
  collection_id BLOB NOT NULL,
  doc_id        BLOB NOT NULL,
  subject_ref   BLOB NOT NULL,
  PRIMARY KEY (tenant_id, collection_id, doc_id, subject_ref)
) STRICT, WITHOUT ROWID;
CREATE INDEX doc_subjects_by_subject ON doc_subjects (tenant_id, subject_ref, collection_id, doc_id);

CREATE TABLE erase_log (
  seq          INTEGER PRIMARY KEY,                -- lückenlos aufsteigend ab 1
  kind         INTEGER NOT NULL CHECK (kind IN (1,2,3,4)),   -- 1 GATED, 2 DESTROYED, 3 COMPLETED, 4 RECONCILED
  erase_id     BLOB NOT NULL CHECK (length(erase_id) = 16),
  target_kind  INTEGER NOT NULL CHECK (target_kind IN (1,2)), -- 1 Subject, 2 Tenant
  tenant_id    BLOB NOT NULL CHECK (length(tenant_id) = 16),
  subject_ref  BLOB NOT NULL CHECK (length(subject_ref) = 16),-- bei Tenant: 16 × 0x00
  ts_ms        INTEGER NOT NULL,
  reason       INTEGER NOT NULL,
  aux          BLOB NOT NULL CHECK (length(aux) = 32),
  prev_hash    BLOB NOT NULL CHECK (length(prev_hash) = 32),
  entry_hash   BLOB NOT NULL CHECK (length(entry_hash) = 32)
) STRICT;
CREATE TRIGGER erase_log_no_update BEFORE UPDATE ON erase_log BEGIN SELECT RAISE(ABORT, 'append-only'); END;
CREATE TRIGGER erase_log_no_delete BEFORE DELETE ON erase_log BEGIN SELECT RAISE(ABORT, 'append-only'); END;

CREATE TABLE erase_manifest (
  erase_id        BLOB NOT NULL CHECK (length(erase_id) = 16),
  doc_id          BLOB NOT NULL CHECK (length(doc_id) = 16),
  collection_id   BLOB NOT NULL CHECK (length(collection_id) = 16),
  envelope_sha256 BLOB NOT NULL CHECK (length(envelope_sha256) = 32),
  PRIMARY KEY (erase_id, collection_id, doc_id)
) STRICT, WITHOUT ROWID;
```

Hinweis: `subject_ref` und `tenant_id` sind Pseudonyme (R4). Es gibt keine Spalte mit Klartext, Schlüsseln oder Personendaten. Die Trigger schützen gegen **Programmierfehler**, nicht gegen einen Angreifer mit Schreibzugriff (6.6).

### 6.4 Durabilitätsregeln

| Nr. | Regel |
| --- | --- |
| **D1** | Eine Mutation gilt erst nach erfolgreichem `COMMIT` mit `synchronous=FULL`. Erst danach wird sie im RAM sichtbar und bestätigt. |
| **D2** | Ein `fsync`- oder `SQLITE_IOERR`-Fehler macht das Handle `Poisoned`. Alle Mutationen liefern `Io(Poisoned)`; es wird nie auf demselben Handle wiederholt (Lektion „fsyncgate“: ein erneuter `fsync` kann Erfolg melden, obwohl Seiten verloren sind). Recovery geschieht beim nächsten Öffnen. Lesen bleibt für Verweigerungen und bereits geladene Daten möglich (Gate bleibt wirksam). `erase()` auf einem `Poisoned`-Handle setzt das RAM-Gate best-effort (fail-closed lokal), liefert aber `Err(Poisoned)` und **keine** Bestätigung (T-POISON-1). |
| **D3** | Kein `mmap` im Acta-Code und in der SQLite-Konfiguration. |
| **D4** | Plattformen: Linux (ext4, XFS), macOS (APFS). Netzwerk-, FUSE- und Overlay-Dateisysteme werden nicht unterstützt und per Startprobe erkannt (13.1). |
| **D5** | Stromausfall: Die Garantie ist die von SQLite und dem Betriebssystem. Acta testet Prozessabbrüche (`kill -9` an nummerierten Punkten), nicht Hardwareverhalten (R14, Test T-POWER-1 als zusätzlicher Nachtlauf). |
| **D6** | Nach dem Purge einer Löschung: `wal_checkpoint(TRUNCATE)` als Hygiene (kein Zustandsübergang hängt davon ab). |
| **D7** | Acta nutzt weder `VACUUM INTO` noch die Backup-API selbst. Sicherungen sind Betreiberpflicht (13.4). |

### 6.5 Zeitstempel

`ts_ms` stammt aus dem `Clock`-Port (Unix-Millisekunden, Wanduhr). Acta setzt keine Annahme über Monotonie. Der Abgleich mit der Custodian-Zeit (14.3, A-5) verwendet eine konfigurierbare Toleranz (`clock_skew_tolerance`).

### 6.6 Löschlog (`erase_log`)

```
genesis   = SHA-256( "ctx/v2/log-genesis" ‖ 0x00 ‖ store_id )
entry_hash = SHA-256( "ctx/v2/log" ‖ 0x00 ‖ seq (8) ‖ kind (1) ‖ erase_id (16) ‖ target_kind (1)
                      ‖ tenant_id (16) ‖ subject_ref (16) ‖ ts_ms (8) ‖ reason (1) ‖ aux (32) ‖ prev_hash (32) )
prev_hash  = entry_hash des Vorgängers (seq − 1); beim ersten Eintrag: genesis
```

`aux` ist bei `GATED` und `RECONCILED` 32 × `0x00`, bei `DESTROYED` `SHA-256(receipt.raw)`, bei `COMPLETED` der `manifest_root` (9.4 E8). Jeder Eintrag entsteht in **derselben Transaktion** wie der zugehörige Zustandswechsel.

**Ehrliche Grenze:** Die Hash-Kette ist **unverschlüsselt und ungeschlüsselt**. Sie macht Teilmanipulationen und Programmierfehler erkennbar, verhindert aber nicht, dass ein Angreifer mit Schreibzugriff die ganze Kette neu berechnet (W5). Belastbare Verankerung liefern nur Dinge außerhalb von Domäne B: der Custodian-Beleg und das Löschregister (13.4).

**Prüfung beim Öffnen:** Die Kette wird vollständig von `genesis` an verifiziert (O(Einträge)). Ein Fehler führt zu `Corrupt(LogChain)`; der Store öffnet nicht, außer der Aufrufer wählt ausdrücklich `OpenMode::ReadOnlyForensic`. Der Zustand eines Ziels in `subjects`/`tenants` wird mit dem letzten Log-Eintrag des Ziels abgeglichen; bei Abweichung gilt der **fortgeschrittenere** Zustand (I-10), und es entsteht ein `RECONCILED`-Eintrag.

### 6.7 Größen und Limits

Ein Envelope ist höchstens `88 + 16·16 + 16 + max_doc_bytes + P1-Overhead` Byte groß (Standard `max_doc_bytes` = 4 MiB). Es gibt keine stille Kürzung (I-14). Die Datenbankgröße ist nicht begrenzt; der RAM-Index ist begrenzt (8.9).

---

## 7. Datenpfad

### 7.1 Anlegen von Mandant, Collection und Betroffenem

- **`create_tenant(tenant_id)`:** Adoption/Anlage von `TK_t` (5.5) → Zeile `tenants(Active)` → Commit → Schlüssel in den RAM. `tenant_id` sind 16 Byte, **ohne** reservierte Werte (kein `0`-Sentinel, kein „Default-Mandant“); der Typ `TenantRef` hat kein `Default`. Ein `tenant_id` aus 16 Nullbytes wird abgelehnt (`InvalidInput`).
- **`create_collection(tenant, name, spec)`:** `collection_id` = 16 Byte Zufall. `spec` legt `dim`, `metric`, `analyzer` und `filterable` fest und ist **unveränderlich**.
- **`register_subject(tenant, subject_ref)`:** Schlüssel anlegen oder adoptieren (5.5), Zeile `subjects(Active)`, Commit, Schlüssel in den RAM. Ein bereits gelöschter oder in Löschung befindlicher `subject_ref` wird mit `Erased` abgelehnt (I-10). `subject_ref` sind 16 Byte Zufall, vom Aufrufer vergeben oder von `SubjectRef::random()` erzeugt, **nicht** aus Personendaten abgeleitet. Die Zuordnung Person → `subject_ref` ist Sache des Aufrufers (R10).

### 7.2 Schreiben (`put`, `update`)

Eingabe: `DocInput { doc_id, subjects: Subjects, text, meta, embedding }` mit `Subjects::Personal(&[SubjectRef])` (1..=16 verschiedene) **oder** `Subjects::NonPersonal` (ausdrückliche Erklärung, kein Personenbezug). Eine leere Liste ist nicht zulässig; ein Aufrufer muss sich entscheiden.

Ablauf (unter dem Schreib-Mutex `W`, Kapitel 10):

| Schritt | Aktion | Fehlerverhalten |
| --- | --- | --- |
| P0 | Validierung: Limits (`max_doc_bytes`, `max_text_bytes`, Meta, Dimension), `NaN`/`Inf`, Nullvektor bei `Cosine`, verschiedene Betroffene | `InvalidInput`, `Limit(..)` |
| P1 | Prüfen: Mandant `Active`; **jeder** Betroffene `Active` und im Schlüssel-Cache | `Erased`, `UnknownSubject`, `UnknownTenant` |
| P2 | Siegeln: `generation` (1 bei `put`, +1 bei `update`), DEK ableiten (4.4), Nonce aus `Rng`, Envelope E1, DEK nullen | `Rng`, `Crypto` (kein Wert, kein Teilzustand) |
| P3 | Transaktion: `docs` einfügen/ersetzen, `doc_subjects` ersetzen; **COMMIT durabel** | `Io(Poisoned)` bei Fehler, kein Teilzustand |
| P4 | `S.write`: RAM-Index aktualisieren (Katalog, Vektor, Text, Filter); Klartextpuffer nullen | — |
| P5 | Rückgabe `DocVersion { generation }` | — |

- `put` auf eine bestehende `doc_id` → `Exists`. `update` auf eine nicht vorhandene → `NotFound`.
- **Klartext verlässt den Schritt nie in den Speicher:** Nur der Envelope wird geschrieben (I-3, Z2).
- Bei `update` bleibt die alte Version unter denselben Schlüsseln lesbar, **bis** die Betroffenen gelöscht werden. Reste älterer Versionen in freigegebenen SQLite-Seiten oder im WAL sind Ciphertext unter denselben Schlüsseln (R12). Es gibt keine Berichtigung per Schlüsselrotation (Nicht-Ziel 1.5).

### 7.3 Lesen (`get`)

1. `S.read`: Katalogeintrag suchen; nicht gefunden oder `tombstone` → `NotFound`.
2. **Gate-Prüfung 1** (I-1): Mandant und alle Betroffenen `Active`, sonst `Erased`.
3. Zeile aus der Datenbank lesen (kein Lock über den Lesezugriff gehalten, 10.2).
4. Header-IDs gegen den Zeilenschlüssel prüfen (4.5, Regel 5), Schlüssel aus dem Cache holen (fehlt einer → `KeyGone`), DEK ableiten, öffnen. AEAD-Fehler → `Corrupt(Auth)` und Dokument in die Quarantäneliste (health `Degraded`), **nie** Klartext.
5. **Gate-Prüfung 2** (I-1): Gate-Zustände erneut lesen. Nicht aktiv → Klartext verwerfen (genullt), `Erased`.
6. Rückgabe des Klartexts.

### 7.4 Tombstone

`tombstone(doc)` setzt `docs.tombstone = 1` und entfernt das Dokument aus dem RAM-Index. **Es ist keine Löschaussage**: Der Envelope bleibt lesbar entschlüsselbar. Die Funktion heißt bewusst nicht `delete`. Es gibt kein Rückgängigmachen in v1. Dokumentweises kryptografisches Löschen erreicht der Aufrufer, indem er für das Dokument einen eigenen `subject_ref` registriert und dessen Löschung ausführt (kostet einen Custodian-Eintrag je Dokument).

### 7.5 Berichtigung und Nachschwärzen mehrpersoniger Dokumente

Kryptografie kann einen Text nicht teilweise schwärzen. Der unterstützte Weg ist `replace_and_erase`:

1. Der Aufrufer zerlegt den Inhalt beim Einfügen so, dass ein Chunk möglichst **einen** Betroffenen hat.
2. Soll ein Betroffener B aus einem Chunk mit A und B verschwinden: `replace_and_erase(redacted_docs, Erase::Subject(B))`. Zuerst werden die geschwärzten Ersatzdokumente (ohne B) per `put` geschrieben (jeweils durabel), danach läuft `erase(B)`. Der alte Chunk wird mit B unlesbar (Z3).
3. Ein Absturz zwischen den Schritten hinterlässt die Ersatzdokumente und den noch nicht gelöschten Chunk. Der Aufrufer wiederholt den Aufruf mit derselben `erase_id`; `put` auf vorhandene Ersatz-IDs wird als idempotent erkannt, wenn der Inhalt (Hash des Klartexts) gleich ist, sonst `Exists`.

---

## 8. Such- und RAG-Kern (nur RAM)

### 8.1 Prinzip

Es gibt **keinen persistierten Index**. Beim Öffnen werden alle Envelopes nicht gesperrter Ziele entschlüsselt und die Indizes im RAM gebaut (9.6, Schritt O8). Damit existiert keine abgeleitete Struktur, die nach einer Löschung repariert werden müsste. Es gibt keine HNSW-Kantenreparatur, keine Surrogate, keinen Segment-Rewrite. Der Klartext der Dokumente wird **nicht** im RAM gehalten; nur abgeleitete Strukturen (Terme, Vektoren, Filterwerte).

### 8.2 Datenstrukturen

| Struktur | Inhalt |
| --- | --- |
| **Katalog** | je Dokument: `ordinal (u32)`, `collection`, `doc_id`, `subjects` (Liste), `tombstone`. Kein Text. |
| **Vektormatrix** je Collection | zusammenhängender `Vec<f32>` (`dim` Spalten), Zeile je Ordinal; freie Zeilen in einer Freiliste. Bei `Cosine` sind die Zeilen L2-normalisiert abgelegt. |
| **Textindex** je Collection | Wörterbuch `Term → Postingliste`; Postings: sortierte `(ordinal u32, tf u32)`; je Ordinal `dl (u32)`; Summe der `dl` für `avgdl`. |
| **Filterindex** je Collection | je `(filterable-Schlüssel, Wert)` eine Bitmap (`Vec<u64>`) über Ordinale. Höchstens 1024 verschiedene Werte je Schlüssel, sonst `Limit(FilterCardinality)`. |

Alle Strukturen liegen in einer `State`-Struktur hinter dem `RwLock` `S` (Kapitel 10). Es gibt keine statischen oder globalen Strukturen (I-13).

### 8.3 Analyzer (Version `a1`)

1. Unicode-Normalisierung NFKC.
2. Wortgrenzen nach UAX #29 (Unicode Word Segmentation).
3. Kleinschreibung (`to_lowercase`).
4. Token mit mehr als 64 Byte werden **nicht indexiert**; die Zahl wird im Collection-Report (`skipped_tokens`) ausgewiesen (dokumentiertes Verhalten, kein stilles Verschwinden).
5. Variante `a1-de`: zusätzlich Snowball-Stemmer Deutsch. Keine Stoppwortliste, keine Präfix-/Regex-Suche, keine Dekomposition.

Der Analyzer ist Teil der Collection-Spezifikation und der Indexversion; da der Index beim Öffnen neu entsteht, gibt es keine Indexmigration.

### 8.4 BM25 (normativ, damit Tests exakt vergleichen)

```
N        = Anzahl aktiver Dokumente der Collection mit Text
n_t      = Anzahl aktiver Dokumente, die Term t enthalten
idf(t)   = ln( 1 + (N − n_t + 0.5) / (n_t + 0.5) )
score(d) = Σ_{t ∈ Q} idf(t) · tf(t,d) · (k1 + 1) / ( tf(t,d) + k1 · (1 − b + b · dl(d) / avgdl) )
k1 = 1.2,  b = 0.75      (Konstanten, Anhang A)
```

`Q` ist die **deduplizierte** Menge der Anfrageterme, **lexikografisch sortiert**; die Summe wird in dieser Reihenfolge in `f64` gebildet. Ergebnisreihenfolge: Score absteigend (`f64::total_cmp`), bei Gleichheit `doc_id` aufsteigend (bytewise).

### 8.5 Vektorsuche (exakt)

- Metriken: `Cosine` (Zeilen und Anfragevektor L2-normalisiert, Normberechnung in `f64`, dann Rundung auf `f32`) und `Dot`.
- Skalarprodukt mit **8 festen Akkumulatoren**: `acc[i mod 8] += a[i] · b[i]` für `i = 0 … dim−1` in aufsteigender Reihenfolge; Reduktion `((acc0+acc1)+(acc2+acc3)) + ((acc4+acc5)+(acc6+acc7))`. Das ist plattformunabhängig bestimmt und für den Compiler vektorisierbar; es ist kein `unsafe` nötig.
- Top-k exakt über alle Kandidaten (nach Filter und Gate), geordnet nach Score absteigend (`total_cmp`), dann `doc_id` aufsteigend. Es gibt keine Näherung und keine Recall-Aussage, die gemessen werden müsste.

### 8.6 Hybridsuche (RRF)

Je Retriever (Text, Vektor) die Top-`rrf_depth` Liste (Standard 200, Bereich `[k, 1000]`). `rrf(d) = Σ_r 1 / (k_rrf + rank_r(d))` mit `rank` ab 1 und `k_rrf = 60` (Anhang A), Summation in `f64` in fester Reihenfolge (Text, Vektor). Ergebnisreihenfolge: Score absteigend, bei Gleichheit `doc_id` aufsteigend. Fehlt eine Anfrageart, besteht das Ergebnis aus der anderen Liste.

### 8.7 Filter

`Filter::Eq(key, value)`, `Filter::In(key, values)`, `Filter::And(Vec<Filter>)`. Nur Schlüssel aus `filterable` der Collection sind erlaubt (sonst `InvalidInput`). Der Filter wirkt als **Vorfilter** (Bitmap-Schnitt) vor dem Scoring; es entsteht kein Recallverlust durch Nachfiltern.

### 8.8 Gate im Suchpfad

1. Die Kandidatenmenge schließt vor dem Scoring alle Dokumente aus, deren Mandant oder einer der Betroffenen nicht `Active` ist, sowie Tombstones.
2. **Vor der Rückgabe** wird für jeden Treffer der Gate-Zustand erneut geprüft (I-1). Treffer eines inzwischen gesperrten Ziels werden entfernt. Das Ergebnis darf deshalb **weniger als `k`** Treffer enthalten; das geschieht nur in dieser Rennbedingung und wird nicht aufgefüllt.
3. `Hit { doc_id, collection, score }`. Treffer enthalten keinen Klartext. Den Inhalt liefert `get` (mit eigenen Gate-Prüfungen). `search_with_content` ist `search` + `get` und liefert pro Treffer `Ok(Doc)` oder `Err(Erased)`.
4. Die Antwort enthält **kein** Feld `exhausted` o. Ä. Die exakte Suche betrachtet alle Kandidaten; ein konstantes Statusfeld wäre nicht wahrhaftig (I-15, Bestandsbefund B-10, Prüfbericht WID-7).

### 8.9 Limits und Speicherbudget

| Limit | Standard | Bereich | Verhalten bei Überschreitung |
| --- | --- | --- | --- |
| `k` | — | ≤ 1000 | `Limit(K)` |
| Anfragetext | — | ≤ 4 KiB | `Limit(Query)` |
| `dim` | je Collection | 0 … 4096 | `InvalidInput` beim Anlegen |
| `max_index_bytes` | 4 GiB | 64 MiB … Hauptspeicher | `Limit(IndexBytes)` bei `put` und beim Öffnen; **niemals** Out-of-Memory |

Schätzung je Dokument: `4·dim + 32 + Σ(Postings 8 Byte) + Katalog 64`. Beim Öffnen wird die Summe vorab aus den Zeilengrößen und `dim` berechnet; überschreitet sie `max_index_bytes`, öffnet der Store nicht (`Limit(IndexBytes)`).

### 8.10 Entfernen und Nullen bei Löschung

Beim Löschen (9.4, E2) werden die Dokumente des Ziels aus dem Katalog entfernt, ihre Vektorzeilen **sofort genullt** und in die Freiliste gestellt, ihre Postings entfernt, Wörterbucheinträge ohne Postings entfernt und die Termstrings genullt. Übersteigt der Anteil freier Zeilen 25 %, wird die Matrix neu gebaut; der alte Speicher wird vor der Freigabe genullt. Klartext im RAM kann dennoch in Swap, Dumps oder Hibernation überleben (R1).

### 8.11 Skalierung (Ziel, ungemessen)

Der Aufbau ist O(N). Eine **Obergrenze je Prozess** wird nach Messung (G3) dokumentiert. Größenordnung der Annahme: bis etwa 10⁵ bis 10⁶ Chunks je Prozess. Rechenbeispiel (kein Beleg): 10⁵ × 768 × 4 B ≈ 307 MB Vektoren. Ein Index für größere Bestände (Snapshots unter Epochenschlüssel) ist Version 1.1, nur wenn gemessen nötig (O4).

---

## 9. Löschprotokoll

### 9.1 Ziele und Zustände

Löschziele: `Subject(tenant, subject_ref)` und `Tenant(tenant)`. Jedes Ziel durchläuft genau diese Zustände:

```
Active ──► Gated ──► Destroyed ──► Completed        (Terminal: Completed; es gibt keinen Weg zurück, I-10)
```

Zusätzlich gibt es für `Gated` und `Destroyed` eine **Zusatzanzeige** `pending: Option<PendingReason>` mit `PendingReason ∈ { CustodianUnavailable, CustodianInconsistent, Io, PurgeFailed }`. `pending` ist kein Zustand, sondern die Ursache, warum der nächste Übergang noch nicht erfolgt ist. Ein Zustand, den ein Test nicht erreichen kann, existiert nicht.

| Übergang | Bedingung | Wirkung | Persistenz (eine Transaktion) | Log-Kind |
| --- | --- | --- | --- | --- |
| Active → Gated | Ziel existiert und ist `Active` | Lesen und Schreiben des Ziels verweigert (Z1) | `state=1`, `erase_id`, Log `GATED`, **COMMIT durabel** | 1 |
| Gated → Destroyed | `destroy` erfolgreich, `status` ∈ {`Destroyed`,`Disabled`}, Probe `KeyGone` (I-7) | Z2 gilt | `state=2`, `receipt`, Log `DESTROYED` (aux = SHA-256 des Belegs) | 2 |
| Destroyed → Completed | Purge vollständig, Manifest vollständig, Wurzel berechnet (I-8) | Hygiene abgeschlossen | `state=3`, Log `COMPLETED` (aux = `manifest_root`) | 3 |
| (beliebig) → fortgeschrittener Zustand | Abgleich beim Öffnen (6.6, 9.6) | Anpassung an Log/Custodian | Log `RECONCILED` | 4 |

### 9.2 Bedeutung der Zustände für Zusagen

| Zustand | Z1 (kein Klartext) | Z2 (Schlüssel vernichtet) | Nachweis |
| --- | --- | --- | --- |
| `Active` | nein | nein | — |
| `Gated` | **ja** | nein (Schlüssel existiert noch beim Custodian) | Log `GATED` |
| `Destroyed` | **ja** | **ja** (bei Karenz: `Disabled`, ausgewiesen) | Beleg + Log `DESTROYED` |
| `Completed` | ja | ja | Beleg + Manifest + Log `COMPLETED` |

### 9.3 Rückgabeverhalten von `erase()`

Konfiguration `erase_return` (Standard **`Destroyed`**):

- **`Destroyed` (Standard):** `erase()` kehrt zurück, sobald der Zustand `Destroyed` erreicht ist, **oder** nach `erase_return_timeout`. Im Timeout-Fall liefert es `Ok(EraseHandle)` mit dem tatsächlich erreichten Zustand (`Gated` + `pending`). Z1 gilt in jedem Fall. Z2 gilt nur, wenn `handle.state() ≥ Destroyed`.
- **`Gated`:** `erase()` kehrt nach dem Gate-Commit zurück (Latenzmodus). Dann gilt Z1, nicht Z2. Im Fenster bis `Destroyed` kann ein Restore der Datenbank aus einem älteren Backup die Sperre verlieren (R11); das Löschregister (13.4) mildert das.

Die Rückgabe enthält **immer** den wahren Zustand. Eine Statusangabe, die nicht gemessen ist, existiert nicht (I-15, Z6).

### 9.4 Ablauf `erase(Subject)` (normativ)

| Schritt | Aktion | Locks | Persistenz / Wirkung | CP |
| --- | --- | --- | --- | --- |
| **E0** | Validierung: Ziel existiert; `erase_id` (16 Byte; vom Aufrufer als Idempotenzschlüssel oder `Rng`); bereits `Gated`/`Destroyed`/`Completed` → Handle des bestehenden Vorgangs (idempotent); gleiche `erase_id`, anderes Ziel → `InvalidInput` | — | — | CP-E1 danach |
| **E1** | **GATE:** Transaktion `state=Gated`, `erase_id`, Log `GATED`; **COMMIT durabel** | `W` | I-9 | CP-E2 vor COMMIT, CP-E3 nach COMMIT |
| **E2a** | RAM-Gate-Flag des Ziels atomar setzen (`Release`) | `W` | ab hier: Z1 | CP-E4 danach |
| **E2b** | `S.write`: Dokumente des Ziels (alle, auch mehrpersonige) aus dem Index entfernen und nullen (8.10); `DK_s` im RAM-Schlüsselcache nullen und entfernen | `W` → `S` | RAM | CP-E5 danach |
| *(Modus `Gated`: `erase()` kehrt hier zurück)* | | | | |
| **E3** | `custodian.destroy(id)` → `DestroyReceipt`. Fehler → `pending=CustodianUnavailable` (bzw. `Protocol`/`Denied` → `CustodianInconsistent`), Wiederholung durch den Worker mit Backoff (9.7). **Kein Lock gehalten.** | — | — | CP-E6 danach |
| **E4** | **Verifikation:** `custodian.status(id)` ∈ {`Destroyed`,`Disabled`}; **Probe:** `custodian.get(id)` MUSS `NotFound`/`Disabled` liefern. Liefert `status` `Unknown` oder die Probe `Present` → `pending=CustodianInconsistent`, Zustand bleibt `Gated` (I-7) | — | — | CP-E7 danach |
| **E5** | Transaktion `state=Destroyed`, `receipt`, Log `DESTROYED`; COMMIT durabel | `W` | Z2 | CP-E8 vor COMMIT, CP-E9 nach COMMIT |
| *(Modus `Destroyed`: `erase()` kehrt hier zurück)* | | | | |
| **E6** | **Purge in Chunks** (≤ `purge_chunk_docs`, Standard 1000) über `doc_subjects_by_subject`: je Chunk eine Transaktion: `erase_manifest` einfügen (`INSERT OR IGNORE`, `envelope_sha256`), `docs`- und `doc_subjects`-Zeilen der Dokumente löschen (bei mehrpersonigen Dokumenten **alle** Zeilen des Dokuments); COMMIT durabel | `W` je Chunk (kurz) | Hygiene | CP-E10 nach jedem Chunk |
| **E7** | Hygiene: `wal_checkpoint(TRUNCATE)`; ein Fehler ist nicht fatal und wird als Gesundheitshinweis vermerkt | — | — | CP-E11 danach |
| **E8** | `manifest_root = SHA-256("ctx/v2/manifest" ‖ 0x00 ‖ ⨁ (doc_id ‖ envelope_sha256))` über alle `erase_manifest`-Zeilen der `erase_id`, sortiert nach `(collection_id, doc_id)`. Transaktion `state=Completed`, Log `COMPLETED` (aux = Wurzel); COMMIT durabel | `W` | — | CP-E12 vor/nach COMMIT |

**Reihenfolge-Invarianten:** Gate vor Destroy (I-6); Destroyed erst nach Gegenprobe (I-7); Completed erst nach Destroyed und vollständigem Purge (I-8); Beleg gespeichert vor `DESTROYED`-Eintrag (gleiche Transaktion).

**Idempotenz:** E3 ist idempotent (K-4). E4 ist rein lesend. E6 ist wiederholbar (`INSERT OR IGNORE`, Löschen ist idempotent). Jeder Schritt kann nach Absturz erneut ausgeführt werden.

**Manifest:** Die Einträge werden **vor** dem Löschen der Zeilen eines Chunks geschrieben und mit demselben Commit festgeschrieben. So ist die Menge der gelöschten Envelope-Hashes vollständig und auch nach Abbruch fortsetzbar. Das Manifest wird nicht beim Gate-Zeitpunkt berechnet, weil das O(Dokumente) wäre und das Gate sofort wirken muss; nach dem Gate sind Schreibzugriffe auf das Ziel gesperrt (Z1), die Menge ist also stabil.

### 9.5 Ablauf `erase(Tenant)`

Wie 9.4 mit Ziel `tenants` und folgenden Unterschieden:

- **E2b:** Alle Index-Strukturen des Mandanten werden entfernt und genullt, alle `DK_s` des Mandanten und `TK_t` im RAM genullt.
- **E3/E4:** `destroy`/`status`/Probe für `TK_t`. Ist `TK_t` vernichtet, ist **kein** DEK des Mandanten mehr berechenbar (4.4); `Destroyed` gilt damit für alle Dokumente, unabhängig von ihrer Zahl.
- **E6:** zusätzlich zum Purge aller Dokumente des Mandanten: **Kaskade** `custodian.destroy(DK_s)` für jeden noch aktiven Betroffenen des Mandanten (Hygiene und Vollständigkeit; für die Unlesbarkeit nicht erforderlich). Fortschritt je Betroffenem durabel (`subjects.state` der Betroffenen → `Gated`/`Completed`); Wiederholung nach Absturz setzt dort fort (CP-T1 nach jedem Kaskaden-`destroy`).
- `Completed` erst, wenn Purge und Kaskade vollständig sind.
- Ein parallel laufendes `erase(Subject)` innerhalb des Mandanten läuft unabhängig weiter; beide Vorgänge sind idempotent.

### 9.6 Öffnen und Wiederaufnahme (normativ)

| Schritt | Aktion | Fehlerverhalten | CP |
| --- | --- | --- | --- |
| **O1** | Datei-Sperre auf dem Datenverzeichnis (13.1); Plattformprobe | `Locked`, `Unsupported` | CP-O1 danach |
| **O2** | SQLite öffnen mit Pragmas (6.2); `application_id`, `user_version` prüfen; `quick_check` | `Corrupt(Schema/Db)` | — |
| **O3** | `erase_log`-Kette verifizieren (6.6); Zustände mit Log abgleichen (fortgeschrittener gewinnt) | `Corrupt(LogChain)`, sonst `RECONCILED` | CP-O2 danach |
| **O4** | Alle Mandanten- und Betroffenenzeilen laden; **RAM-Gate-Flags** für jedes Ziel mit `state ≠ Active` setzen | — | — |
| **O5** | **Custodian-Abgleich:** `get_many` für `TK_t` aller `Active`-Mandanten und `DK_s` aller `Active`-Betroffenen. `Present` → in den RAM-Cache. `Gone` → das Ziel ist faktisch gesperrt (Schlüssel fehlt): Zustand auf `Gated` mit Log `RECONCILED`; Vorgang fortsetzen (Destroy ist idempotent). Scheitert der Transport → `CustodianUnavailable`, der Store **öffnet nicht** (kein halboffener Betrieb ohne Schlüssel) | `CustodianUnavailable` | CP-O3 vor/nach der Abgleich-Transaktion |
| **O6** | Löschregister abgleichen (13.4), falls konfiguriert: jeder Eintrag, dessen Ziel noch `Active` oder `Gated` ist, wird gesperrt und zur Vernichtung eingereiht | wie O5 | — |
| **O7** | Offene Vorgänge fortsetzen: `Gated` → E3 ff.; `Destroyed` → E6 ff.; Mandantenkaskaden fortsetzen. Dies läuft im Worker (10.4) | — | — |
| **O8** | **Indexaufbau:** für jedes Dokument, dessen Mandant und alle Betroffenen `Active` sind: Envelope lesen, öffnen, P1 parsen, in den Index einfügen, Klartext nullen. AEAD-Fehler → Dokument in die Quarantäneliste, health `Degraded`; bei `strict_open = true` (Standard) bricht der Start mit `Corrupt(Auth)` ab | `Corrupt`, `Limit(IndexBytes)` | CP-O4 beliebig oft |
| **O9** | Betriebsbereit; `health()` aus gemessenem Zustand | — | — |

Ein Absturz in beliebigen Schritten von O1 bis O9 ändert nichts an der Korrektheit des nächsten Öffnens; alle Schritte sind idempotent.

### 9.7 Worker, Wiederholung und Frist

- **Ein** Worker-Thread je `Acta`-Instanz führt E3–E8 sowie die Kaskaden aus. Er hält zwischen Schritten **keine** Sperre über einen Custodian-Aufruf oder `fsync`.
- Backoff: Anfangsverzögerung 1 s, Verdopplung bis höchstens 15 min, vollständiges Jitter (Zufall aus `Rng`).
- `erase_sla` ist die **Frist bis `Completed`** ab dem Gate-Zeitpunkt (nicht die Latenz bis `Gated`). Wird sie überschritten, meldet `health()` `Degraded(ErasePastSla)` und der Observer ein Ereignis; die Wiederholung läuft weiter. Es gibt keine stille Aufgabe.

### 9.8 Absturzpunkte und Erwartung nach Neustart (normative Tabelle)

Alle Punkte sind benannte Haken (`Hooks::at(CrashPoint)`, 16.4). Erwartung gilt nach `kill -9` an dem Punkt **und** anschließendem Öffnen.

| Punkt | Lage | Erwartung nach Neustart |
| --- | --- | --- |
| CP-R1 | `register_subject`: nach `Rng`, vor `custodian.put` | Betroffener unbekannt; Wiederholung erzeugt neu |
| CP-R2 | nach `custodian.put`, vor DB-Commit | Waisenschlüssel; Wiederholung **adoptiert** (5.5) |
| CP-R3 | nach DB-Commit, vor Schlüsselablage im RAM | Betroffener `Active`; Schlüssel wird beim Öffnen geladen |
| CP-P1 | `put`: nach Validierung, vor Transaktion | Dokument fehlt |
| CP-P2 | in der Transaktion, vor COMMIT | Dokument fehlt, keine Teilzeilen |
| CP-P3 | nach COMMIT, vor Index-Update | Dokument vorhanden und indexiert (Neuaufbau) |
| CP-P4 | nach Index-Update, vor Rückgabe | Dokument vorhanden; Wiederholung → `Exists` |
| CP-E1 | `erase`: nach Validierung | `Active` |
| CP-E2 | in der Gate-Transaktion, vor COMMIT | `Active`; die Sperre war nicht bestätigt |
| CP-E3 | nach Gate-COMMIT, vor RAM-Gate | `Gated` (Z4); Worker setzt fort |
| CP-E4 | nach Flag, vor Indexentfernung | `Gated`; Dokumente kommen nie in den Index |
| CP-E5 | nach Indexentfernung, vor `destroy` | `Gated`; Worker ruft `destroy` |
| CP-E6 | nach `destroy`, vor Verifikation | `Gated`; Worker wiederholt `destroy` (idempotent), verifiziert |
| CP-E7 | nach Verifikation, vor Destroyed-Transaktion | wie CP-E6 |
| CP-E8 | in der Destroyed-Transaktion, vor COMMIT | `Gated`; wie CP-E6 |
| CP-E9 | nach Destroyed-COMMIT, vor Purge | `Destroyed`; Worker purged |
| CP-E10 | nach einem Purge-Chunk-COMMIT (je Chunk) | `Destroyed`; Purge setzt fort; Manifest der gelöschten Chunks vollständig |
| CP-E11 | nach Purge, vor Completed-Transaktion | `Destroyed`; Worker hygienisiert und schließt ab |
| CP-E12 | in/nach der Completed-Transaktion | `Gated`-/`Destroyed`-Rest bzw. `Completed` |
| CP-T1 | Mandantenkaskade: nach jedem `destroy(DK_s)` | Kaskade setzt fort; kein Betroffener doppelt belegt |
| CP-O1 … CP-O4 | Öffnen: nach Sperre / nach Log-Prüfung / um den Abgleich / im Indexaufbau | nächstes Öffnen identisch und korrekt |

**Eine bestätigte Sperre (nach CP-E3) geht in keinem Fall verloren** (Z4).

### 9.9 Was `erase` nicht tut

`erase` vernichtet Schlüssel beim Custodian und entfernt Daten des Ziels aus dem Speicher und aus dem RAM-Index. Es löscht nicht: Backups des Custodians (R2) oder des Datenverzeichnisses (R14), Klartext, den der Aufrufer bereits erhalten hat (R6), Daten bei externen Verarbeitern (R5), Swap, Core-Dumps, Hibernation (R1). Es prüft keine Aufbewahrungspflichten nach Art. 17 Abs. 3: Die Verantwortung liegt beim Aufrufer.

---

## 10. Nebenläufigkeit

Acta ist synchron und nutzt keine Async-Laufzeit (Nicht-Ziel 1.5: kein Server). Ein `Acta`-Handle ist `Send + Sync`. Aufrufer-Threads lesen und schreiben; **ein** Worker-Thread führt Löschvorgänge fort (10.4).

### 10.1 Sperren und Ordnung

| Name | Typ | Schützt | Wer hält sie |
| --- | --- | --- | --- |
| `W` | `Mutex<()>` | Serialisiert jede **Mutation**: `create_*`, `register_subject`, `put`, `update`, `tombstone`, Gate- und Zustandsübergänge, jeder Purge-Chunk | Schreiber, Worker (je Schritt) |
| `S` | `RwLock<State>` | RAM-Index (Katalog, Vektormatrix, Textindex, Filter), Schlüsselcache (`TK_t`, `DK_s`), Gate-Tabelle (Karte `Ziel → Arc<TargetGate>`) | Leser kurz (`read`), Schreiber unter `W` (`write`) |
| `C` | `Mutex<Connection>` | die **eine** SQLite-Verbindung (`locking_mode = EXCLUSIVE` erlaubt nur eine, 6.2) | Leser für **einen** `SELECT`, Schreiber für die Transaktion bis nach `fsync` |
| `Q` | `Mutex<VecDeque<Job>>` + `Condvar` | Warteschlange des Workers | kurz, nie zusammen mit anderen Sperren |

`TargetGate` enthält nur ein `AtomicU8` mit den Zustandscodes aus Anhang A (`0` Active … `3` Completed). Ein Eintrag wird nie entfernt und nie auf einen kleineren Wert gesetzt (I-10).

**Regeln (jede ist ein Test, 10.7):**

1. **Ordnung:** `W` vor `S`, `W` vor `C`. Kein Pfad nimmt `W`, während er `S` oder `C` hält.
2. `S`, `C` und `Q` sind **Blattsperren**: Keine Operation hält zwei davon gleichzeitig. (`put` macht erst `C` für den Commit, gibt ihn frei und nimmt dann `S` für das Index-Update.)
3. **Kein Aufruf nach außen unter einer Sperre.** Weder `Observer`, `Clock`, `Hooks` noch `Custodian` noch `ErasureRegistry` werden mit gehaltenem `S`, `C` oder `Q` aufgerufen. Der Custodian wird auch nicht unter `W` aufgerufen (Ausnahme: `put` beim Anlegen eines Schlüssels in `register_subject`/`create_tenant`, 5.5; diese Aufrufe sind kurz und durch K-8 zeitbegrenzt).
4. **Klarstellung zu 9.7:** „keine Sperre über einen Custodian-Aufruf oder `fsync`“ gilt für die **Worker-Schritte** E3, E4 und die Wartezeiten dazwischen. Die Transaktionen E1, E5, E6 (je Chunk) und E8 halten `W` und `C` bis nach dem `fsync` (D1); das ist beabsichtigt und der Grund für kleine Chunks (`purge_chunk_docs`).
5. Eine vergiftete Sperre (`PoisonError`) ist fatal: Das Handle wird `Poisoned` (D2). Es wird nie mit `into_inner` weitergearbeitet.

### 10.2 Lesepfad ohne Sperre über den Datenbankzugriff

Ergänzt 7.3 um die Sperren:

| Schritt (7.3) | Sperre | Dauer |
| --- | --- | --- |
| 1 Katalog suchen; **Gate-Prüfung 1**; `Arc<TargetGate>` des Mandanten und aller Betroffenen merken; Ordinal und Betroffenenliste kopieren | `S.read` | Mikrosekunden |
| 3 Zeile lesen (`SELECT` auf `docs`) | `C` | ein Zeilenzugriff; **kein** `S`, kein `W` |
| 4 Header gegen Zeilenschlüssel prüfen; DEK ableiten (die Schlüssel liegen in `S` und haben kein `Clone`) | `S.read` | eine HKDF-Berechnung |
| 4 AEAD öffnen | keine | — |
| 5 **Gate-Prüfung 2** über die gemerkten `Arc<TargetGate>` | keine (nur `load(Acquire)`) | — |

Während eines Commits wartet ein Leser auf `C`. Das ist eine bekannte Folge von `EXCLUSIVE` (Entscheidung O5, Kapitel 20).

### 10.3 Gate-Protokoll und Z1

- **Setzen (E1 bis E2b) in einem Abschnitt unter `W`:** E1 (Commit, durabel), E2a (`store(Gated, Release)`), E2b (Index entfernen, Schlüssel nullen). `W` wird zwischen E1 und E2b **nicht** freigegeben. Damit kann sich kein `put` zwischen den Commit und das Flag schieben (P1 prüft das Flag unter `W`).
- **Lesen:** `load(Acquire)` vor der Entschlüsselung (Prüfung 1) und **nach** der Entschlüsselung, unmittelbar vor der Rückgabe (Prüfung 2). Prüfung 2 ist der Linearisierungspunkt des Lesevorgangs.
- **Aussage zu Z1:** Beginnt Prüfung 2 eines Lesevorgangs, nachdem `erase()` zurückgekehrt ist (E2a liegt dann im happens-before), liefert sie „nicht aktiv“ und der Lesevorgang endet mit `Erased` oder `NotFound`, nie mit Klartext. Ein Lesevorgang, dessen Prüfung 2 **vor** E2a lag, darf noch Klartext liefern; dieser Klartext liegt beim Aufrufer (R6).
- **Suche:** Die Gate-Prüfung vor dem Scoring (8.8 Punkt 1) liest dieselben Flags; die erneute Prüfung je Treffer (8.8 Punkt 2) liegt vor der Rückgabe.
- **Rückgabe nach dem Entfernen:** Nach E2b ist das Dokument nicht mehr im Katalog. `get` liefert dann `NotFound` (7.3 Schritt 1), nur im Fenster zwischen E2a und E2b `Erased`. Beides ist mit Z1 vereinbar. Wer den Zustand eines Ziels sicher abfragen will, nutzt `subject_state` oder `tenant_state` (11.3). Siehe 19.3, Punkt 4.

### 10.4 Worker

- **Ein** Thread `ctx-worker` je Instanz. Er wird beim Öffnen nach O7 gestartet und beim Schließen beendet.
- **Aufgaben (`Job`):** Fortsetzen eines Löschvorgangs ab seinem Zustand (`Gated` → E3, `Destroyed` → E6), Mandantenkaskade, Wiederholung nach `pending`, `wal_checkpoint(TRUNCATE)` (E7).
- **Takt:** Der Worker schläft auf `Q` und wacht bei neuen Jobs, bei Ablauf des nächsten Backoff-Termins oder beim Schließen auf. Backoff nach 9.7 (Anhang A).
- **Rückkehr von `erase()`:** Der aufrufende Thread führt E0 bis E2b selbst aus (unter `W`), reiht den Job ein und wartet dann auf eine `Condvar` des Vorgangs, bis der Zustand `≥ Destroyed` ist, bis `erase_return_timeout` abläuft oder (Modus `Gated`) sofort. Der Worker führt E3 ff. aus.
- **Chunks:** Der Worker gibt `W` zwischen den Purge-Chunks frei. Ein Schreiber wartet höchstens auf einen Chunk-Commit.
- **Beobachter:** Ereignisse werden aus einer Liste nach dem Freigeben aller Sperren zugestellt (Regel 3).
- **Schließen (`close`):** Setzt ein Stoppflag, weckt den Worker und wartet auf ihn. Ein laufender `custodian`-Aufruf ist nicht abbrechbar (K-8); `close` kann deshalb bis zum Zeitlimit des Adapters dauern. Offene Löschvorgänge bleiben in der Datenbank und werden beim nächsten Öffnen fortgesetzt (O7).

### 10.5 Panics, Abbruch, Gift

- Release-Profil: `panic = "abort"`, `overflow-checks = true` (15.4). Arithmetik im Produktionscode verwendet `checked_*`/`saturating_*` und ist durch `clippy::arithmetic_side_effects = deny` abgesichert; der Abbruch ist die letzte Verteidigung und wegen Z4 ein zulässiger Absturzpunkt.
- Es gibt keine Abbruchsignale und kein `Drop`, das Zustandsübergänge ausführt. Ein weggeworfenes `EraseHandle` beendet keinen Vorgang.

### 10.6 Handles teilen Zustand

`Tenant` und `Collection` sind dünne Verweise (`Arc<Inner>` plus Bezeichner). Sie halten **keine** eigenen Sperren. Zwei Handles auf dieselbe Collection nutzen dieselben `W`, `S`, `C` (Bestandsbefund B-36). Es gibt keine gestreiften Dokumentsperren: `W` ist die einzige Schreibsperre. Die Skalierung der Schreiber ist durch `fsync` begrenzt und dort Zielwert (17.4), nicht Zusage.

### 10.7 Tests

| ID | Prüft | Verfahren |
| --- | --- | --- |
| **T-CONC-1** | Gate-Protokoll (10.3), Z1 | `loom`-Modell mit Leser, Löscher und Schreiber; Eigenschaft: kein Klartext, wenn Prüfung 2 das Flag sieht; ein `put` nach E1 sieht das Flag |
| **T-CONC-2** | Handles teilen Zustand (10.6, B-36) | zwei Handles, parallele `put`; `loom` und Stresslauf |
| **T-CONC-3** | Stresslauf | N Leser, M Schreiber, ein `erase`; Invarianten I-1, I-11, I-12 nach jedem Lauf |
| **T-CONC-4** | Sperrordnung (10.1) | Testbuild mit Sperr-Wrapper, der Verletzungen der Regeln 1 bis 3 meldet (nur `cfg(test)`) |
| **T-CONC-5** | Worker-Schließen | `close` während `destroy` hängt höchstens bis zum Adapter-Zeitlimit; Neustart setzt fort |

---

## 11. Öffentliche Schnittstelle

Normativ ist die **Form** (Typen, Fehler, Pfade). Rust-Syntax ist Schreibweise; Feinheiten (Lebensdauern, `impl Into`) sind nicht normativ. **Jede** Funktion hat entweder die beschriebene Wirkung oder liefert `Err(NotImplemented)`; nie `Ok` ohne Wirkung (Z6, T-STUB-1).

### 11.1 Typen

```rust
// Bezeichner: 16 Byte, kein Default, keine Verlustumwandlung (I-18). Debug gibt nur "[ID]" aus (I-16, T-TYPE-3).
pub struct TenantRef([u8; 16]);     // 16 Nullbytes → Err(InvalidInput)
pub struct SubjectRef([u8; 16]);    // random() liefert Result (Rng-Fehler)
pub struct DocId([u8; 16]);
pub struct CollectionId([u8; 16]);
pub struct EraseId([u8; 16]);

pub enum Subjects<'a> { Personal(&'a [SubjectRef]), NonPersonal }   // 1..=16 verschiedene, 7.2

pub struct DocInput<'a> { pub doc_id: DocId, pub subjects: Subjects<'a>, pub text: Option<&'a str>,
                          pub meta: &'a [(&'a str, MetaValue<'a>)], pub embedding: Option<&'a [f32]> }
pub enum MetaValue<'a> { Str(&'a str), Int(i64), Bool(bool) }
pub struct Doc { pub doc_id: DocId, pub generation: u32, pub text: Option<String>, pub meta: Vec<(String, MetaOwned)>,
                 pub embedding: Option<Vec<f32>> }     // Klartext: R6
pub struct DocVersion { pub generation: u32 }

pub struct CollectionSpec { pub dim: u16, pub metric: Metric, pub analyzer: Analyzer, pub filterable: Vec<String> }
pub enum Metric { Cosine, Dot }       // Codes in Anhang A
pub enum Analyzer { A1, A1De }

pub enum Filter { Eq(String, MetaOwned), In(String, Vec<MetaOwned>), And(Vec<Filter>) }
pub struct SearchQuery<'a> { pub text: Option<&'a str>, pub vector: Option<&'a [f32]>, pub filter: Option<Filter>,
                             pub k: u16, pub rrf_depth: Option<u16> }
pub struct Hit { pub doc_id: DocId, pub collection: CollectionId, pub score: f64 }   // kein Klartext, kein `exhausted`

pub enum Erase { Subject(TenantRef, SubjectRef), Tenant(TenantRef) }
pub struct EraseOptions { pub erase_id: Option<EraseId> }       // Idempotenzschlüssel (E0)

pub enum State { Active, Gated, Destroyed, Completed }
pub enum PendingReason { CustodianUnavailable, CustodianInconsistent, Io, PurgeFailed }

pub struct EraseHandle { /* erase_id(), target(), state(), pending(), receipt() -> Option<&DestroyReceipt>,
                            wait(timeout) -> Result<State, Error> */ }
```

### 11.2 Funktionen

| Funktion | Wirkung | Kapitel |
| --- | --- | --- |
| `Acta::open(Config) -> Result<Acta, Error>` | Öffnen nach O1 bis O9 | 9.6 |
| `Acta::close(self) -> Result<(), Error>` | Worker beenden, Sperre lösen | 10.4 |
| `health(&self) -> Health` | gemessener Zustand | 12.4 |
| `create_tenant(&self, TenantRef) -> Result<Tenant, Error>` | `TK_t` anlegen oder adoptieren | 7.1 |
| `tenant(&self, TenantRef) -> Result<Tenant, Error>` | Handle auf vorhandenen Mandanten (`UnknownTenant`) | — |
| `Tenant::create_collection(name, CollectionSpec) -> Result<Collection, Error>` | Spezifikation unveränderlich | 7.1 |
| `Tenant::collection(name) -> Result<Collection, Error>` | Handle (`NotFound`) | 10.6 |
| `Tenant::register_subject(SubjectRef) -> Result<(), Error>` | `DK_s` anlegen oder adoptieren | 7.1, 5.5 |
| `Collection::put(DocInput) -> Result<DocVersion, Error>` | Schreiben, `Exists` bei vorhandener `doc_id` | 7.2 |
| `Collection::update(DocInput) -> Result<DocVersion, Error>` | neue Generation, `NotFound` | 7.2 |
| `Collection::get(DocId) -> Result<Doc, Error>` | Lesen mit zwei Gate-Prüfungen | 7.3 |
| `Collection::tombstone(DocId) -> Result<(), Error>` | **keine** Löschaussage | 7.4 |
| `Collection::search(SearchQuery) -> Result<Vec<Hit>, Error>` | BM25, Vektor, Hybrid | 8.4 bis 8.8 |
| `Collection::search_with_content(SearchQuery) -> Result<Vec<(Hit, Result<Doc, Error>)>, Error>` | `search` + `get` je Treffer | 8.8 |
| `Collection::report() -> CollectionReport` | Zähler aus dem Zustand (`docs`, `skipped_tokens`, `index_bytes`) | 8.3 |
| `erase(&self, Erase, EraseOptions) -> Result<EraseHandle, Error>` | Löschvorgang E0 ff.; Rückgabe nach `erase_return` | 9.3, 9.4 |
| `erase_status(&self, EraseId) -> Result<EraseStatus, Error>` | Zustand, `pending`, Zeitpunkte aus `erase_log` | 9.1 |
| `subject_state(&self, TenantRef, SubjectRef) -> Result<State, Error>`, `tenant_state(&self, TenantRef) -> Result<State, Error>` | gemessener Zustand | 10.3 |
| `replace_and_erase(&self, TenantRef, collection: &str, Vec<DocInput>, Erase, EraseOptions) -> Result<EraseHandle, Error>` | Ersatzdokumente `put`, dann `erase` | 7.5 |
| `evidence(&self, EraseId) -> Result<EvidencePackage, Error>` | Materialsammlung, **kein** Bericht | 14.1 |

Nicht vorhanden in v1: `delete`, `restrict`, `lift_restriction`, `change_passphrase`, `report` im Sinne eines Löschberichts, `acknowledge_ledger_reset`, Backup-Funktionen (Nicht-Ziele 1.5, D7). Es gibt keine Stubs für sie.

### 11.3 Verhalten bei Wiederholung

- `update` auf ein Dokument mit `generation = 4 294 967 295` → `Limit(Generation)`; die Generation wird nie überlaufen oder zurückgesetzt (I-14).
- `put` auf vorhandene `doc_id` → `Exists`; in `replace_and_erase` idempotent bei gleichem Klartext-Hash (7.5).
- `erase` mit bereits laufendem oder abgeschlossenem Ziel → Handle des bestehenden Vorgangs (E0). Gleiche `erase_id` auf anderem Ziel → `InvalidInput`.
- `register_subject` auf bekannten, aktiven Betroffenen → `Ok(())` ohne neuen Schlüssel (idempotent). Auf gelöschten oder in Löschung befindlichen → `Erased`.
- `create_tenant` verhält sich entsprechend (`Exists`-frei, idempotent; gelöscht → `Erased`).

---

## 12. Konfiguration, Fehler, Beobachtbarkeit

### 12.1 Konfiguration

`Config` hat **kein** `Default` (der Custodian ist Pflicht). `Config::new(data_dir, custodian)` setzt die Standardwerte; `Acta::open` ruft `validate()` und liefert `Err(Config(..))` mit benanntem Grund (I-14). Es gibt keine stille Korrektur.

| Parameter | Standard | Bereich | Bedeutung |
| --- | --- | --- | --- |
| `data_dir` | — | existierendes, beschreibbares Verzeichnis | Datenverzeichnis (13.1) |
| `custodian` | — | `Arc<dyn Custodian>` | Pflicht |
| `registry` | keines | `Option<Arc<dyn ErasureRegistry>>` | Löschregister (13.4) |
| `registry_policy` | `Required` | `Required`, `BestEffort` | nur wirksam, wenn `registry` gesetzt |
| `observer` | keiner | `Option<Arc<dyn Observer>>` | 12.3 |
| `clock` | Systemuhr | `Arc<dyn Clock>` | Zeitquelle, 6.5 |
| `open_mode` | `ReadWrite` | `ReadWrite`, `ReadOnlyForensic` | 6.6 |
| `strict_open` | `true` | bool | 9.6 O8 |
| `erase_return` | `Destroyed` | `Destroyed`, `Gated` | 9.3 |
| `erase_return_timeout` | 60 s | 1 s … 10 min | 9.3 |
| `erase_sla` | 24 h | 1 min … 30 d | **Frist bis `Completed`** ab Gate (9.7), nicht Latenz |
| `purge_chunk_docs` | 1000 | 1 … 10 000 | 9.4 E6 |
| `clock_skew_tolerance` | 5 min | 1 s … 1 h | 14.3 A-5 |
| `max_doc_bytes` | 4 MiB | 1 MiB … 64 MiB | Nutzdaten je Dokument (Text, Meta, Embedding) |
| `max_text_bytes` | 1 MiB | 1 KiB … 4 MiB | Text je Dokument |
| `max_index_bytes` | `min(4 GiB, ½ physischer Speicher)` | 64 MiB … physischer Speicher | 8.9; der wirksame Wert steht in `health()` |
| `rrf_depth` | 200 | `k` … 1000 | 8.6; überschreibbar je Anfrage |

**Konsistenzregeln (jede Verletzung ist ein benannter `Config`-Fehler):**

1. `erase_return_timeout ≤ erase_sla`.
2. `max_text_bytes ≤ max_doc_bytes`.
3. `local_custodian_dir` liegt nicht im Datenverzeichnis (5.6); das Löschregister liegt nicht im Datenverzeichnis (13.4).
4. Es gibt keinen Parameter für `Rng`, VFS, Chiffre, Format oder Durabilität (4.7, V-8).

**T-CONFIG-1:** Jeder Standardwert ist mit **jedem** zulässigen Wert jedes anderen Parameters verträglich (Bestandsbefund B-28: Standard-`batch_window` gegen `erase_sla`); ein Test durchläuft alle Grenzwertkombinationen. Es gibt in v2 kein `batch_window` und keine Stapelung von Löschvorgängen. `max_index_bytes` hängt vom Rechner ab und ist von der Kombinationsprüfung ausgenommen; sein wirksamer Wert wird ausgewiesen (I-15).

### 12.2 Fehler

```rust
pub enum Error {
    InvalidInput(InvalidInput), Limit(LimitKind), Config(ConfigError),
    Erased, UnknownTenant, UnknownSubject, Exists, NotFound,
    KeyGone, CustodianUnavailable, Custodian(CustodianKind),   // Denied | Protocol
    Rng, Crypto, Clock,
    Corrupt(CorruptKind),             // Schema, Db, LogChain, Auth, UnknownAlg, RowMismatch, Format
    Io(IoKind),                       // Poisoned, Other
    Locked, Unsupported, Registry(RegistryKind), NotImplemented, Closed,
}
pub enum LimitKind { K, Query, IndexBytes, FilterCardinality, DocBytes, TextBytes, Meta, Dim, Subjects, Generation }
```

Regeln: Jeder Fehler ist ein benannter Wert (I-14). `Display` und `Debug` enthalten keine Schlüssel, Klartexte, `subject_ref`, `tenant_id` oder Pfade (I-16, T-OBS-1). Die Abbildung der Schlüsselpfadfehler steht in 4.9. `Custodian(Denied/Protocol)` wird nie als `KeyGone` gedeutet (K-2).

### 12.3 Beobachter

```rust
pub trait Observer: Send + Sync { fn event(&self, e: &Event); }
pub enum Event {
    OpenDone { docs_indexed: u64, quarantined: u32, duration_ms: u64 },
    EraseGated { erase_id: EraseId }, EraseDestroyed { erase_id: EraseId, karenz: bool },
    EraseCompleted { erase_id: EraseId }, ErasePending { erase_id: EraseId, reason: PendingReason },
    ErasePastSla { erase_id: EraseId }, Reconciled { count: u32 }, Quarantined { count: u32 },
    Poisoned, CheckpointFailed, RegistryMissing, ClockSkew { erase_id: EraseId },
}
```

Ereignisse enthalten nur `erase_id`, Zähler und Zustandsnamen (I-16). Der Beobachter wird ohne gehaltene Sperre aufgerufen (10.1 Regel 3); sein Panic oder seine Langsamkeit verändert keinen Zustand. Es gibt keine Telemetrie nach außen (Nicht-Ziel 1.5).

### 12.4 `health()`

```rust
pub struct Health { pub status: Status, pub degraded: Vec<DegradedReason>, pub open_erases: u32,
                    pub oldest_open_erase_ms: Option<u64>, pub quarantined: u32, pub docs_indexed: u64,
                    pub index_bytes: u64, pub max_index_bytes: u64 }
pub enum Status { Ok, Degraded, Poisoned, Closed }
pub enum DegradedReason { Quarantine, ErasePastSla, CheckpointFailed, RegistryMissing, ClockSkew }
```

Jedes Feld wird aus dem Zustand berechnet (I-15). **T-HEALTH-1:** Für jedes Feld wird der Zustand verändert (Quarantäne, überfälliger Vorgang, fehlgeschlagener Checkpoint, vergiftetes Handle) und die Änderung muss sichtbar werden; eine Implementierung mit konstantem `Ok` macht den Test rot (Bestandsbefund B-14).

### 12.5 Limits und Eingabeprüfung

Alle Limits stehen in Anhang A; jede Überschreitung liefert einen `Limit`-Fehler **vor** jeder Allokation (4.5 Regel 2). `put` prüft: `text` ≤ `max_text_bytes`, Summe aus Text, Metawerten und Embedding ≤ `max_doc_bytes`, Meta nach 4.6, `embedding.len() == dim` der Collection, kein `NaN`/`Inf`, bei `Cosine` kein Nullvektor, `Subjects::Personal` mit 1 bis 16 verschiedenen Betroffenen. **T-LIMIT-1** prüft jede Grenze auf beiden Seiten.

---

## 13. Betrieb

### 13.1 Start, Sperre, Plattformprobe

Verzeichnislayout des Datenverzeichnisses:

```
<data_dir>/LOCK          Sperrdatei (leer)
<data_dir>/ctx.sqlite3   Datenbank
<data_dir>/ctx.sqlite3-wal, -shm   von SQLite verwaltet
```

**O1 im Einzelnen:**

1. Verzeichnis prüfen (existiert, gehört dem Prozessnutzer, Modus `0700`; sonst `Config`-Fehler). Acta legt keine Verzeichnisse mit weiterem Zugriff an.
2. Exklusive Dateisperre auf `LOCK` über `std::fs::File::try_lock` (kein `unsafe`, keine Zusatzbibliothek). Scheitert sie → `Locked` (I-17). Die Sperre hält bis `close`.
3. **Plattformprobe:** (a) Dateisystemtyp über eine sichere `statfs`-Schnittstelle (Bibliothek wird beim Bau festgelegt und in der Abhängigkeitsliste geführt, 15.3); NFS, SMB, FUSE und Overlay → `Unsupported`; (b) **Verhaltensprobe**: temporäre Datei im Datenverzeichnis schreiben, `fsync`, umbenennen, Verzeichnis-`fsync`, erneut öffnen, Inhalt vergleichen, dann mit `secure_delete`-ähnlicher Überschreibung entfernen; jeder Fehler → `Unsupported`. Die Typliste allein ist keine Zusage, die Verhaltensprobe ist der Test.
4. Danach O2.

Unterstützt: Linux (ext4, XFS), macOS (APFS) (D4). Windows ist Nicht-Ziel.

### 13.2 Verzeichnistrennung

Der `LocalCustodian` und das Löschregister liegen **außerhalb** des Datenverzeichnisses, idealerweise auf einem anderen Gerät mit eigener Sicherungsregel. `Config::validate` und die Adapter prüfen kanonische Pfade (5.6). Ein Betreiber, der Schlüssel und Daten gemeinsam sichert, hebt die Wirkung der Vernichtung für diese Sicherung auf (R2, R14).

### 13.3 Schließen

`close` beendet den Worker (10.4), führt `PRAGMA wal_checkpoint(TRUNCATE)` aus (Hygiene, Fehler nicht fatal), schließt die Verbindung und gibt die Sperre frei. `Drop` ohne `close` verhält sich wie ein Absturz: Die nächste Öffnung stellt den Zustand her (Z4).

### 13.4 Löschregister und Sicherungen

**Zweck:** W5 und R11. Ein Restore eines älteren Datenverzeichnisses löscht die Sperre eines kürzlich beauftragten Vorgangs. Das Löschregister liegt **außerhalb** von Domäne B und wird beim Öffnen abgeglichen (O6).

```rust
pub struct RegistryEntry { pub store_id: [u8; 16], pub erase_id: EraseId, pub target_kind: u8,   // 1 Subject, 2 Tenant
                           pub tenant_id: TenantRef, pub subject_ref: [u8; 16], pub ts_ms: u64 } // 73 Byte, Anhang A
pub trait ErasureRegistry: Send + Sync {
    fn record(&self, e: &RegistryEntry) -> Result<(), RegistryError>;   // append-only, durabel vor Rückkehr, idempotent je (store_id, erase_id)
    fn entries(&self, store_id: &[u8; 16]) -> Result<Vec<RegistryEntry>, RegistryError>;
}
```

**Ablauf in `erase` (zwischen E0 und E1):** `registry.record(entry)`. Erst danach beginnt E1.

- `registry_policy = Required` (Standard): Scheitert `record`, liefert `erase` `Err(Registry(..))` **ohne** Zustandsänderung; der Aufrufer wiederholt.
- `registry_policy = BestEffort`: Scheitert `record`, läuft `erase` weiter, `health()` meldet `Degraded(RegistryMissing)` und der Beobachter `RegistryMissing`.
- **Zusätzlicher Absturzpunkt CP-E1R** (nur mit konfiguriertem Register): nach `record`, vor dem Gate-COMMIT. Erwartung nach Neustart: Das Register kennt das Ziel, die Datenbank nicht als gesperrt; O6 sperrt das Ziel (`RECONCILED`, Grund „Register“) und reiht die Vernichtung ein. Der Aufrufer hatte keine Bestätigung, das Ergebnis ist die beauftragte Löschung. Das ist die sichere Richtung.
- Das Register enthält Pseudonyme (R4), keine Schlüssel und keine Klartexte. Es speichert **nie** Belege oder Personendaten.
- Referenzimplementierung `acta-registry-file` (A0-nah): Datei mit angehängten Einträgen, `fsync` je Eintrag, Pfad außerhalb des Datenverzeichnisses. Sie prüft den Pfad gegen das Datenverzeichnis.

**Sicherungen sind Betreiberpflicht (D7).** Empfehlung (kein Zwang):

1. Sicherung des Datenverzeichnisses nur im geschlossenen Zustand oder per Dateisystem-Snapshot, der SQLite-WAL-konsistent ist. Acta nutzt weder `VACUUM INTO` noch die Backup-API (D7).
2. Aufbewahrung der Sicherungen kürzer als die Frist, nach der gelöschte Personen aus Sicherungen verschwinden müssen; Sicherungen des Custodians gleich lang oder kürzer (R2).
3. Nach einem Restore: Öffnen führt O3, O5, O6 aus. Gilt der Custodian als wahr für Schlüssel, so sind vernichtete Ziele weiterhin unlesbar; Ziele, die nur in der Datenbank `Gated` waren und nicht im Register stehen, gehen verloren (R11). Der Betreiber **MUSS** nach einem Restore die offenen Löschaufträge seiner eigenen Aufzeichnungen erneut einreichen (idempotent über `erase_id`).

### 13.5 Verlust von Hauptschlüssel oder Custodian

Es gibt kein Schlüssel-Escrow durch den Hersteller (Nicht-Ziel 1.5). Verlust des `MK` des `LocalCustodian` oder des Custodians insgesamt macht alle Daten unlesbar (R9). Das ist die beabsichtigte Eigenschaft der Architektur und kein Fehler; sie MUSS in jeder Betreiberdokumentation stehen.

### 13.6 Formatversionen

`user_version = 2` und `application_id` (6.2) kennzeichnen die Datei. Eine Datei mit anderer Version öffnet nicht (`Corrupt(Schema)`). Es gibt kein Altformat und keine Migration (V-10). Eine Formatänderung erhöht `version` des Envelopes oder `user_version` und die Spezifikationsversion (0.4 Punkt 3).

### 13.7 Plattform und Toolchain

Linux (ext4, XFS) und macOS (APFS) auf aktuellen Hardwarevarianten; Rust-Toolchain exakt in `rust-toolchain.toml` gepinnt (15.4). Referenzplattform für Messungen (G3) wird bei G3 festgelegt und im Benchmarkprotokoll genannt; bis dahin ist jede Zahl **[Ziel]**.

---

## 14. Nachweis und Prüfung

### 14.1 Nachweispaket (`evidence`)

Acta stellt **keinen Bericht** aus (1.2). `evidence(erase_id)` liefert eine **Materialsammlung** aus Tatsachenfeldern, die ein Dritter mit eigenem Code nachprüft. Format: UTF-8-JSON, Schlüssel alphabetisch, Binärwerte als Kleinbuchstaben-Hex, keine Gleitkommazahlen.

| Feld | Inhalt |
| --- | --- |
| `spec_version` | Fassung dieser Spezifikation, gegen die gebaut wurde |
| `store_id` | 16 Byte hex |
| `erase_id`, `target_kind`, `tenant_id`, `subject_ref` | Pseudonyme (R4); der Auditor leitet daraus die `KeyId` nach 5.4 ab |
| `state`, `pending` | gemessener Zustand zum Zeitpunkt der Abfrage |
| `attestation_class` | `A0`, `A1` oder `A2` (14.4), unverändert aus dem Adapter |
| `log` | alle `erase_log`-Einträge dieser `erase_id`: `seq`, `kind`, `ts_ms`, `reason`, `aux`, `prev_hash`, `entry_hash`; dazu `genesis` und der `entry_hash` des letzten Gesamteintrags |
| `custodian` | `custodian_ref`, `custodian_time_ms`, `scheduled_final_ms`, `karenz` (bool), `receipt_raw` (hex, unverändert), `receipt_sha256` |
| `manifest` | Zeilen `(collection_id, doc_id, envelope_sha256)` und `manifest_root` (nur ab `Completed`) |
| `clock_skew` | bool (A-5) |
| `limits_text` | feste Grenzaussage (A-6) |

Das Paket enthält **keine** Felder `verified`, `compliant`, `deleted`, `erased` o. Ä. (A-4). Es enthält keine Schlüssel und keine Klartexte (I-3, I-16 gelten; die Pseudonyme sind ausdrücklich zugelassen, weil der Auditor sie zur Abfrage braucht). Das Paket ist als personenbezogen-nahes Material zu behandeln (R4).

### 14.2 Prüfverfahren für einen Auditor (Domäne D)

Der Auditor braucht: eine Kopie von Domäne B (idealerweise **vor** und **nach** der Löschung), eigenen Zugang zum Custodian (Lesen und Audit-Log), das Nachweispaket und eigenen Code. Für Format und Schlüsselableitung genügt das Referenzprogramm in Anhang B (35 Zeilen, nicht Teil der Acta-Bibliothek).

| Schritt | Prüfung | Belegt |
| --- | --- | --- |
| V1 | `erase_log` der Kopie aus der Datenbank lesen (`sqlite3`, eigener Code), Kette ab `genesis` neu berechnen (6.6), Abgleich mit dem Paket | keine Teilmanipulation; Grenze: kein Schutz gegen komplette Neuberechnung (W5) |
| V2 | `KeyId` nach 5.4 bilden; **beim Custodian direkt** `status` und `get` mit eigenen Zugangsdaten abfragen: `Destroyed`/`Disabled` bzw. `NotFound`/`Disabled` | Z2, unabhängig von Acta |
| V3 | Beleg: `receipt_sha256` gleich SHA-256 von `receipt_raw`; `aux` des `DESTROYED`-Eintrags gleich `receipt_sha256`; `custodian_ref` im **Audit-Log des Custodians** wiederfinden | Beleg stammt vom Custodian (A1/A2) |
| V4 | Manifest: `manifest_root` nach 9.4 E8 neu berechnen; Vergleich mit `aux` des `COMPLETED`-Eintrags | Manifest vollständig und unverändert |
| V5 | In Kopien **vor** dem Purge: SHA-256 jedes Envelopes der Manifest-Dokumente bilden und mit `envelope_sha256` vergleichen | Manifest beschreibt reale Objekte |
| V6 | **Unlesbarkeit:** alle Envelopes des Betroffenen aus einer Vorher-Kopie mit **allen derzeit beim Custodian vorhandenen Schlüsseln** (Referenzprogramm) zu öffnen versuchen; jeder Versuch MUSS scheitern (`KeyError` oder Authentisierungsfehler) | Z2, W2 |
| V7 | Zeiten: `ts_ms` des `DESTROYED`-Eintrags gegen `custodian_time_ms` (A-5) | Plausibilität |
| V8 | Klasse und Grenzen: `attestation_class` und `limits_text` lesen; Außenaussage nicht stärker als die Klasse (14.4) | Z5 |

V6 setzt eine **Positivkontrolle** voraus: Dieselbe Methode MUSS auf einem Envelope eines **nicht** gelöschten Betroffenen erfolgreich öffnen. Sonst beweist ein Misserfolg nichts.

### 14.3 Kanarien-Test (Auditor, echte Konfiguration) und Abgleichregeln

Der Auditor führt den Test auf der **echten** Konfiguration aus (derselbe Adapter, dieselben Zugangsdaten und Einstellungen des Betreibers), nicht auf einer Testattrappe.

| Schritt | Aktion |
| --- | --- |
| K1 | Der Auditor erzeugt zwei Zufallswerte: `marker` (32 Byte, hex) und `subject_ref`. |
| K2 | `register_subject(canary)` und `put` eines Dokuments, dessen Text `marker` enthält (Betroffener = Kanarienbetroffener). |
| K3 | Kopie von Domäne B einschließlich WAL (`kopie_vor`). |
| K4 | **Positivkontrolle:** mit eigenem Code (Anhang B) und den Schlüsseln, die der Auditor selbst beim Custodian liest, den Envelope aus `kopie_vor` öffnen; `marker` MUSS erscheinen. |
| K5 | `erase(canary)` bis mindestens `Destroyed`. |
| K6 | Beim Custodian direkt: `get(DK)` liefert `NotFound`/`Disabled`; `status` ∈ {`Destroyed`, `Disabled`}. |
| K7 | **Negativkontrolle:** K4 auf `kopie_vor` mit allen jetzt vorhandenen Schlüsseln wiederholen; Öffnen MUSS scheitern. |
| K8 | `marker` (hex und Rohbytes) in allen Dateien von Domäne B (aktuell, `kopie_vor`, WAL) suchen; **kein Treffer** (Hygiene, I-3). |
| K9 | `custodian_ref` des Nachweises im Audit-Log des Custodians wiederfinden. |

**T-CANARY-1** automatisiert K1 bis K8 in der CI gegen den `LocalCustodian` (Klasse A0, ein Funktionstest). Für jeden externen Adapter führt der Auditor den Test **einmal vor der Inbetriebnahme** auf dem echten Dienst aus; das Ergebnis liegt außerhalb von Acta.

**Abgleichregeln:**

| Nr. | Regel |
| --- | --- |
| **A-1** | Das Nachweispaket nennt die Klasse unverändert aus `attestation_class()` des Adapters. |
| **A-2** | `receipt.raw` wird unverändert gespeichert und gehasht. Acta interpretiert den Beleg nicht. Bei Klasse `A1` und `A2` ist ein leeres `raw` ein Protokollfehler (`CustodianInconsistent`, Zustand bleibt `Gated`). |
| **A-3** | Eine Karenzfrist (K-6) wird ausgewiesen (`karenz = true`, `scheduled_final_ms`). Der Zustand `Destroyed` bedeutet dann „Schlüssel gesperrt, endgültige Löschung geplant“. |
| **A-4** | Das Paket enthält nur Tatsachenfelder (14.1), keine Wertung. |
| **A-5** | **Zeitabgleich:** Sind `ts_ms` des `DESTROYED`-Eintrags und `custodian_time_ms` beide vorhanden und weicht der Betrag um mehr als `clock_skew_tolerance` ab, setzt das Paket `clock_skew = true`; der Beobachter erhält `ClockSkew`, `health()` meldet `Degraded(ClockSkew)`. Das ist ein Hinweis, **kein** Fehler und kein Zustandswechsel. |
| **A-6** | `limits_text` ist eine feste Konstante (Anhang A), nennt die Restrisiken R1, R2, R5, R6, R7, R8, R14 und R17 mit Kapitelverweis und ist nicht abschaltbar. |

### 14.4 Attestierungsklassen

```rust
pub enum AttestationClass { A0, A1, A2 }
```

| Klasse | Bedeutung | Zulässige Außenaussage (höchstens) |
| --- | --- | --- |
| **A0** | **Selbstauskunft.** Der Schlüsseleintrag liegt in einem lokalen Verzeichnis (`LocalCustodian`, 5.6) oder der Adapter stellt keinen unabhängigen Beleg aus. Der Nachweis enthält keine Löschaussage, nur einen Funktionstest. | „Acta hat den lokalen Schlüsseleintrag überschrieben und entfernt (Funktionstest, Klasse A0). Das ist keine Löschbestätigung.“ |
| **A1** | **Dienstbeleg.** Ein externer Key-Dienst stellt einen Beleg mit eigener Kennung aus (`custodian_ref`, `raw`) und führt die Vernichtung aus; Karenz wird ausgewiesen. | Wortlaut aus Anhang C plus: „Der Key-Dienst hat die Vernichtung quittiert (Klasse A1).“ |
| **A2** | **A1 plus unabhängige Prüfbarkeit.** (a) `raw` ist vom Dienst **signiert** und ohne Acta prüfbar; (b) der Dienst führt ein eigenes Audit-Log, das der Betreiber der Anwendung nicht verändern kann; (c) der Adapter besteht **T-CUST-2**. | wie A1 plus: „Der Beleg ist vom Dienst signiert und im Audit-Log des Dienstes nachvollziehbar (Klasse A2).“ |

**T-CUST-2 (Zusatzsuite für A2):** 1. `raw` ist signiert und die Signatur prüft mit dem separat bezogenen öffentlichen Schlüssel des Dienstes. 2. Ein verändertes `raw` besteht die Prüfung nicht. 3. Die Abfrage des Audit-Logs liefert das Vernichtungsereignis mit demselben `custodian_ref`. 4. Die Zugangsdaten der Anwendung können keine Audit-Log-Einträge löschen oder ändern (Negativtest durch den Auditor).

Der `LocalCustodian` liefert fest `A0` (T-EVID-1). Kein Adapter darf eine höhere Klasse melden, als seine bestandenen Suiten stützen. **Jede Außenaussage DARF NICHT stärker sein als die Klasse** (1.4). „DSGVO-konform“ ist in **keiner** Klasse zulässig.

### 14.5 Grenzen des Nachweises

Ein Nachweis belegt **nicht**: Löschung aus Sicherungen des Custodians oder des Datenverzeichnisses (R2, R14), aus RAM, Swap, Dumps (R1), bei externen Verarbeitern (R5), aus bereits ausgelieferten Antworten (R6), und nicht die rechtliche Einordnung (R17). Bei Karenz gilt die endgültige Löschung erst nach `scheduled_final_ms` (R8). Vor einer Vernichtung kompromittierte Schlüsselkopien sind nicht erfasst (R7).

---

## 15. Architektur und Crates

### 15.1 Crates

Der Kern ist klein genug, dass ein Dritter ihn lesen kann. Zeilenbudgets sind **[Ziel]** und werden nur gesenkt (Ratchet). Zum Vergleich: Das Altrepo hatte ein Budget von 64 400 Zeilen für 14 Produktionscrates; die Summe unten ist 12 400 Zeilen vor G2 und 12 900 mit `acta-registry-file`.

| Crate | Aufgabe | Budget [Ziel] | ab Gate |
| --- | --- | --- | --- |
| `acta-seal` | `Secret<N>`, Envelope E1 (Bauen, Parsen), P1 (CBOR-Teilmenge), DEK-Ableitung, `Rng`-Schnittstelle (getrandom), AEAD-Aufruf. Keine E/A, kein Netz, kein Speicher | 1500 | vor G1 |
| `acta-custodian` | `Custodian`, `KeyId`, `KeyLookup`, `KeyStatus`, `CustodianError`, `DestroyReceipt`, `AttestationClass`, Konformitätssuiten T-CUST-1/-2 (als Bibliothek zum Testen von Adaptern) | 600 | vor G1 |
| `acta-custodian-local` | `LocalCustodian` (5.6, Klasse A0) | 800 | vor G1 |
| `acta-store` | `BlobStore`, SQLite-Umsetzung, Schema, Pragmas, Log-Kette, T-STORE-1 | 2500 | vor G1 |
| `acta-index` | RAM-Index: Analyzer, BM25, Vektor, RRF, Filter (Kapitel 8) | 3000 | vor G1 |
| `acta` | Kern: Gate, Löschprotokoll, Worker, Öffnen, öffentliche API, `Observer`, `Clock`, `ErasureRegistry` (Schnittstelle) | 4000 | vor G1 |
| `acta-testkit` | Nur Tests: feste Uhr, Absturz-Haken (`cfg`), Mutanten-Custodians, Hilfen. Keine Abhängigkeit eines Produktionscrates | — | vor G1 |
| `acta-registry-file` | Referenz-Löschregister (13.4) | 500 | G2 |

Es gibt **keine Fassade** und keine zweite Definition öffentlicher Namen (V-6). `acta` ist die öffentliche Schnittstelle. Kein Crate heißt wie ein früheres Crate des Altrepos, damit nichts verwechselt wird (V-10).

### 15.2 Abhängigkeitsmatrix (erlaubte Kanten)

| von \ auf | seal | custodian | custodian-local | store | index | acta | testkit | registry-file |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `seal` | — | | | | | | | |
| `custodian` | ✔ | — | | | | | | |
| `custodian-local` | ✔ | ✔ | — | | | | | |
| `store` | ✔ | | | — | | | | |
| `index` | ✔ | | | | — | | | |
| `acta` | ✔ | ✔ | | ✔ | ✔ | — | | |
| `testkit` | ✔ | ✔ | ✔ | ✔ | ✔ | ✔ | — | ✔ |
| `registry-file` | ✔ | | | | | ✔ | | — |

(`acta` kennt `custodian-local` nur über Dev-Abhängigkeiten.) **T-ARCH-1** vergleicht die Matrix mit allen `Cargo.toml`. `acta-seal` hängt von **keinem** anderen Crate des Workspaces ab. Es gibt keine Kante auf `acta` von einem Produktionscrate außer `registry-file`.

### 15.3 Externe Abhängigkeiten (Whitelist, Versionen werden gepinnt)

| Zweck | Bibliothek (Vorschlag) | Crate |
| --- | --- | --- |
| AEAD | `chacha20poly1305` | seal |
| KDF, Hash | `hkdf`, `sha2` | seal, store, acta |
| Zufall | `getrandom` | seal |
| Nullen | `zeroize` | seal und alle mit Geheimnissen |
| SQLite | `rusqlite` mit Feature `bundled` (Pin) | store |
| Unicode | `unicode-normalization`, `unicode-segmentation` | index |
| Stemmer | ein Snowball-Stemmer für Deutsch (Auswahl bei G1, Prüfung wie 4.1) | index |
| Dateisystemtyp | eine sichere `statfs`-Schnittstelle (Auswahl bei G1) | acta |
| nur Test | `loom`, `proptest`, `trybuild`, `libfuzzer-sys`, `syn` | testkit, Tests |

Keine Netzwerkbibliothek, keine Async-Laufzeit, kein `serde` im Produktionspfad, kein `bincode`, kein `regex`, kein `rand`, kein `memmap2`, kein `lru`. `cargo deny` erzwingt die Liste (V-9, T-ARCH-2). Jede Version ist vor G4 auf Advisories und Prüfstatus zu prüfen (4.1); das ist von mir nicht ausgeführt.

### 15.4 Bau- und Lintregeln

- `#![forbid(unsafe_code)]` in allen eigenen Crates (T-ARCH-6). Fremdcode mit `unsafe` (SQLite, Krypto-Bibliotheken) ist Restrisiko R15.
- Clippy: `unwrap_used`, `expect_used`, `panic`, `unreachable`, `todo`, `unimplemented`, `indexing_slicing`, `let_underscore_must_use`, `cast_possible_truncation`, `arithmetic_side_effects` = `deny` (V-1 bis V-3). `-D warnings`.
- `clippy.toml` mit `disallowed-methods`: `String::from_utf8_lossy`, `Option::unwrap_or_default`, `Result::unwrap_or_default`, `std::mem::forget` auf Geheimnissen.
- Release-Profil: `panic = "abort"`, `overflow-checks = true`, `lto = "fat"`, `codegen-units = 1`. (Eine Festlegung für Benchmarks bei G3.)
- Cargo-Features: **Whitelist** (T-ARCH-5); in v2 gibt es **keine** Features, die Format, Durabilität, Verschlüsselung oder Löschverhalten ändern. Absturz-Haken sind **`cfg`**-Schalter über `RUSTFLAGS` (`--cfg ctx_crash_hooks`), kein Cargo-Feature; ein Release-Build enthält sie nicht (T-ARCH-5 prüft Symbole).
- `Cargo.lock` im Repository; alle Aufrufe mit `--locked`. `rust-toolchain.toml` pinnt die Toolchain exakt.
- CI-Aufträge: `cargo fmt --check`, `clippy`, `test --workspace --all-targets --locked`, `deny`, `audit`, Architekturtests, Anhang-B-Prüfung (Python), Mutationsläufe (16.3), Fuzz-Kurzläufe; nachts: lange Fuzzläufe, T-POWER-1, Stresslauf.

### 15.5 Repository-Layout (Neubau)

```
Cargo.toml  Cargo.lock  rust-toolchain.toml  deny.toml  clippy.toml
docs/SPEC.md                     diese Datei (nicht löschen, T-DOC-1)
crates/<crate>/                  Kapitel 15.1
tests/                           Walking Skeleton, Absturz-Harness, Kanarientest (Workspace-Integrationstests)
fuzz/                            Fuzz-Ziele
audit/                           Referenz-Öffner (Python, Anhang B), Vektoren, Auditor-Anleitung
```

Das Altrepo bleibt als Archiv (eigener Tag), **nicht** im Baum. Im Quellbaum liegen keine generierten Dokumente, keine Kopien, keine Berichte (T-ARCH-8). Das Altrepo wird nicht umgebaut; Befunde sind in Kapitel 19 geführt, Entscheidungen nicht übernommen (0.1).

### 15.6 Typen für Wahrheitsaussagen

Zustandsübergänge, die eine Zusage tragen, erzeugen **Beweiswerte**, deren Konstruktor privat ist (V-4):

- `GateProof` entsteht nur nach dem durablen Commit von E1 und wird von `custodian.destroy` verlangt (I-6).
- `DestroyProof { receipt, status, probe }` entsteht nur in der Funktion, die E3 und E4 ausführt, und nur wenn `status ∈ {Destroyed, Disabled}` **und** die Probe `NotFound`/`Disabled` lieferte (I-7).
- `PurgeProof { manifest_root }` entsteht nur nach vollständigem E6 (I-8); der Übergang nach `Completed` verlangt `&DestroyProof` und `PurgeProof` als Argumente.
- Es gibt keinen Konstruktor, keine `Default`-Umsetzung und keine öffentlichen Felder für diese Typen; **T-TYPE-1** (`trybuild`, Übersetzung MUSS scheitern) schützt das.

Jedes Konzept hat genau **einen** Typ im Workspace (V-6, T-ARCH-3). Bezeichnerbreiten stehen in Anhang A; es gibt keine `cfg`-abhängigen Typbreiten (Bestandsbefund B-35).

---

## 16. Tests

### 16.1 Regeln

| Nr. | Regel |
| --- | --- |
| **TR-1** | **Keine Attrappe im Datenpfad** der Integrationstests der Gates G1 und G2 (V-11): echte SQLite-Datei, echter `LocalCustodian`, echtes Dateisystem. Mutanten-Custodians (`acta-testkit`) sind erlaubt, **weil** sie absichtlich lügen und rot machen müssen. |
| **TR-2** | **Mutationsregel (16.3):** Für jeden Schutzmechanismus existiert ein Mutant, der mindestens einen Test rot macht. Ein grüner Test, der auch mit der Attrappe besteht, ist ein Befund (Bestandsbefund B-08). |
| **TR-3** | **Neustartregel:** Jeder Löschtest endet mit einem Neustart in einem **neuen Prozess** und einer Schlüsselprobe beim Custodian. |
| **TR-4** | **Assertionsregel:** Jeder Test enthält mindestens eine Assertion (Skript in der CI). `#[ignore]` ist verboten; Nachtläufe sind eigene, benannte Aufträge. |
| **TR-5** | **Determinismus:** feste Seeds, feste Uhr (`cfg(test)`), keine echten Wartezeiten außer in benannten Zeitlimit-Tests. |
| **TR-6** | **Rot zuerst:** T-WALK-1 wird **rot** eingecheckt, bevor Implementierung entsteht, und bleibt bis zum Release unverändert. Änderungen daran sind Spezifikationsänderungen (0.4). |
| **TR-7** | **Rückführbarkeit:** Jeder Test trägt die IDs der Zusagen, Invarianten oder Befunde, die er belegt (Katalog 16.2). T-DOC-3 prüft, dass jede Zusage Z1 bis Z7 und jede Invariante I-1 bis I-18 mindestens einen Test im Katalog hat. |

### 16.2 Katalog

| ID | Prüft | Belegt | Gate |
| --- | --- | --- | --- |
| **T-WALK-1** | Walking Skeleton (16.2.1) | Z1, Z2, I-1, I-2, I-9, I-11 | G1 |
| **T-SEAL-1** | Known-Answer-Test: Envelope und DEK aus Anhang B byteweise reproduzieren; zusätzlich Gegenprüfung mit dem Python-Referenzprogramm | C-8 | G1 |
| **T-SEAL-2** | Parserregeln 4.5 (jede Regel ein Fall: Länge, `reserved`, `n_subjects`, Reihenfolge, Duplikate, `ct_len`, unbekannte `alg`/`version`) | I-14, 4.5 | G1 |
| **T-SEAL-3** | Bindung: Änderung je eines Bestandteils (Mandant, Collection, `doc_id`, Betroffenenmenge, Generation, ein `DK_s`, `TK_t`) ergibt anderen DEK; Verschieben eines Envelopes in andere Zeile scheitert | 4.4, Z3 | G1 |
| **T-SEAL-4** | Jedes Bit des Headers und des Chiffrats manipulieren: Öffnen scheitert immer (`Corrupt(Auth)`) | C-2, W5 | G1 |
| **T-NOFALLBACK-1** | Custodian liefert `NotFound`, `Disabled`, `Unavailable`, `Denied`, `Protocol`: Ergebnis je `KeyGone`/`CustodianUnavailable`/`Custodian(..)`, nie Klartext, nie Ersatzschlüssel | Z7, I-2, I-4, C-4, K-2 | G1 |
| **T-RNG-1** | `Rng`-Fehler (Test-Rng nur in `cfg(test)`) bricht `put` und `register_subject` mit `Rng` ab; es entsteht weder Schlüssel noch Zeile | Z7, I-5, B-07, B-08 | G1 |
| **T-DEL-1** | `erase(Subject)`: danach `get`, `search`, `search_with_content` liefern keinen Klartext; Schlüsselprobe; nach Neustart gleich | Z1, Z2, I-7 | G1 |
| **T-DEL-2** | `erase(Tenant)`: alle Dokumente unlesbar, Kaskade über Betroffene, `Completed` nur nach Purge und Kaskade | 9.5, I-8 | G2 |
| **T-MULTI-1** | Dokument mit A und B: Löschen von B macht es unlesbar; ein Dokument nur mit A bleibt lesbar | Z3, I-11, I-12 | G1 |
| **T-ISO-1** | Isolation: Bytes der Envelopes nicht beteiligter Betroffener und Mandanten bleiben bei `erase` und `put` anderer unverändert | I-12, W6 | G2 |
| **T-GATE-1** | Suchpfad: gesperrte Ziele fehlen vor dem Scoring; Treffer eines während der Suche gesperrten Ziels werden vor der Rückgabe entfernt; Ergebnis darf kürzer als `k` sein | 8.8, I-1 | G2 |
| **T-CONC-1 … 5** | Kapitel 10.7 | Z1, I-1, I-13 | G2 |
| **T-CRASH-`<CP>`** | je Absturzpunkt aus 9.8 plus CP-E1R: `kill -9` am Punkt, Neustart, Erwartung der Tabelle (16.4) | Z4, I-6, I-8, I-9, I-10 | G2 |
| **T-OPEN-1** | O1 bis O9: Sperre, Pragmas, Kettenprüfung, Abgleich, Indexaufbau, `strict_open` | 9.6, I-17 | G2 |
| **T-RESTORE-1** | Älteres Datenverzeichnis einspielen: vernichtete Ziele (Custodian `Gone`) werden `Gated` mit `RECONCILED`; mit Register auch Ziele, die nur dort stehen; ohne Register bleibt `Gated`-Verlust sichtbar dokumentiert (R11) | W5, 9.6 O5/O6 | G2 |
| **T-REG-1** | Löschregister: `Required` und `BestEffort`, CP-E1R, Idempotenz | 13.4 | G2 |
| **T-LOG-1** | Hash-Kette: Vektor aus Anhang B; jede Manipulation (Bit, Löschen, Umordnen, Einfügen) wird erkannt; Trigger verhindern `UPDATE`/`DELETE` | 6.6, W5 | G2 |
| **T-STORE-1** | Commit überlebt `kill -9`; nicht festgeschriebener Commit unsichtbar; `fsync`-Fehler → Fehler | D1, D2 | G2 |
| **T-STORE-2** | Schema: alle `CHECK`, `STRICT`, Trigger, Pragmas (Abfrage nach dem Öffnen), `application_id`, `user_version` | 6.2, 6.3 | G1 |
| **T-POISON-1** | Nach simuliertem `fsync`-Fehler: alle Mutationen `Io(Poisoned)`; kein Wiederholen auf demselben Handle; `erase` setzt RAM-Gate, liefert `Err` ohne Bestätigung. (Der Fehler wird an der `BlobStore`-Grenze eingespeist; echte E/A-Fehler prüft T-POWER-1.) | D2 | G2 |
| **T-CUST-1** | Konformitätssuite 5.8 gegen `LocalCustodian` und jeden Adapter; Mutantenregel | K-1 bis K-8 | G1 |
| **T-CUST-2** | Zusatzsuite A2 (14.4) | A2 | je Adapter |
| **T-CANARY-1** | 14.3 K1 bis K8 automatisiert gegen `LocalCustodian` (A0) | Z2, W2 | G2 |
| **T-EVID-1** | Paket nach 14.1: Klasse, `limits_text`, keine Wertungsfelder; `LocalCustodian` = `A0`; leeres `raw` bei A1/A2 → `CustodianInconsistent` | Z5, A-1 bis A-6 | G2 |
| **T-STUB-1** | Jede öffentliche Funktion hat Wirkung oder liefert `NotImplemented`; Sammeltest: nach `Ok` ist die Wirkung beobachtbar (Neustart) | Z6, B-12, B-13 | G1 |
| **T-HEALTH-1** | 12.4: jedes Feld folgt dem Zustand | I-15, B-14, B-15 | G2 |
| **T-CONFIG-1** | 12.1: Standardwerte verträglich mit allen zulässigen Werten; jede ungültige Kombination liefert benannten Fehler | I-14, B-28 | G1 |
| **T-LIMIT-1** | jedes Limit beidseitig (inkl. `generation`); `tenant_id` aus 16 Nullbytes → `InvalidInput`; keine Allokation vor der Prüfung | I-14, 4.5 Regel 2, 7.1, B-30 | G1 |
| **T-OBS-1** | Fehlertexte, `Debug`, Beobachterereignisse und Logs enthalten keine Schlüssel, Klartexte, `subject_ref`, `tenant_id` (Suche nach bekannten Testwerten) | I-16 | G2 |
| **T-LEAK-1** | Nach einem Lauf alle Bytes des Datenverzeichnisses (inkl. WAL und freier Seiten) nach den bekannten Schlüsseln, DEKs und dem Kanarienmarker durchsuchen: kein Treffer | I-3, B-37 | G1 |
| **T-TYPE-1** | `trybuild`: Beweiswerte (15.6) nicht konstruierbar | V-4 | G1 |
| **T-TYPE-2** | `Secret`: kein `Clone`, `Copy`, `Default`, `Display`, `Serialize`, `PartialEq`; `Debug` = `[REDACTED]` | V-5, 4.8 | G1 |
| **T-TYPE-3** | Debug der Bezeichner gibt keine Bytes aus; Bezeichner haben keine verlustbehaftete Umwandlung (kein `from_utf8_lossy`, kein Kürzen oder Auffüllen; Konstruktoren verlangen genau 16 Byte) | I-16, I-18, B-26 | G1 |
| **T-IDX-1** | BM25 gegen unabhängige Referenzrechnung (Python) auf festem Korpus, exakt (`f64`-Reihenfolge nach 8.4) | 8.4 | G2 |
| **T-IDX-2** | Exakte Vektorsuche: Reihenfolge der 8 Akkumulatoren, Gleichstand nach `doc_id`, `Cosine` und `Dot` gegen Referenz | 8.5 | G2 |
| **T-IDX-3** | RRF gegen Referenz; fehlende Anfrageart | 8.6 | G2 |
| **T-IDX-4** | Vorfilter: gleiche Ergebnismenge wie vollständige Suche mit Nachfilter; Kardinalitätslimit | 8.7 | G2 |
| **T-IDX-5** | Entfernen: nach `erase` enthalten Katalog, Matrix (Zeilen genullt, Freiliste), Postings, Wörterbuch nichts vom Ziel; Neubau bei 25 % | 8.10 | G2 |
| **T-IDX-6** | `max_index_bytes`: Überschreitung bei `put` und beim Öffnen → `Limit(IndexBytes)`, nie Speichererschöpfung | 8.9 | G2 |
| **T-WORKER-1** | Backoff, Wiederholung nach `CustodianUnavailable`, `Degraded(ErasePastSla)`, kein stilles Aufgeben | 9.7 | G2 |
| **T-REPLACE-1** | `replace_and_erase`: Absturz zwischen den Schritten, Wiederholung idempotent | 7.5, Z3 | G2 |
| **T-IDEM-1** | `erase` wiederholt (gleiche und neue `erase_id`), parallele Vorgänge auf demselben Ziel | 9.4 E0 | G2 |
| **T-LOCK-1** | Zweiter Prozess auf demselben Verzeichnis → `Locked`; Plattformprobe: `Unsupported` auf nicht unterstützten Dateisystemen (Testumgebung) | I-17, D4 | G2 |
| **T-FUZZ-1** | Fuzz des Envelope-Parsers (nie Panic, nie unbegrenzte Allokation) | 4.5 Regel 8 | G2 |
| **T-FUZZ-2** | Fuzz des P1-Dekoders (CBOR-Teilmenge, kanonische Form erzwungen) | 4.6 | G2 |
| **T-FUZZ-3** | Fuzz des Analyzers und der Filter-/Anfragekodierung | 8.3, 8.7 | G2 |
| **T-ARCH-1** | Abhängigkeitsmatrix 15.2 | AR | G1 |
| **T-ARCH-2** | `cargo deny`: Whitelist 15.3, kein Netzwerk im Baum | V-9 | G1 |
| **T-ARCH-3** | Keine Typen gleichen Namens in zwei Crates | V-6 | G1 |
| **T-ARCH-4** | Kein `static` mit Innenveränderlichkeit (`syn`-Scan) | V-7, I-13 | G1 |
| **T-ARCH-5** | Feature-Whitelist; Release-Binär ohne Absturz-Haken (Symbolprüfung) | V-8 | G1 |
| **T-ARCH-6** | `forbid(unsafe_code)` in jedem eigenen Crate | 15.4 | G1 |
| **T-ARCH-7** | Keine `#[ignore]`, jeder Test mit Assertion, Budget-Ratchet | TR-4 | G1 |
| **T-ARCH-8** | Keine generierten Dokumente, Kopien, Berichte im Quellbaum; kein `legacy` | V-10 | G1 |
| **T-DOC-1** | `docs/SPEC.md` vorhanden, Versionszeile gleich `Cargo.toml` | 0.4 | G1 |
| **T-DOC-2** | Anhang A gegen Code, jede Konstante genau einmal | 0.4 | G1 |
| **T-DOC-3** | Jede Zusage und Invariante hat Tests im Katalog | TR-7 | G1 |
| **T-POWER-1** | Nachtlauf: Abbrüche durch Gerätefehler (`dm-error`/Loopback), Abschneiden von Dateien; nur Prozessverhalten, **kein** Hardwarebeweis | D5, R14 | G3 |

#### 16.2.1 T-WALK-1 (Walking Skeleton, verbindlich)

1. Temporäres Datenverzeichnis `D`, Custodian-Verzeichnis `K` **außerhalb** von `D`, echter `LocalCustodian` mit zufälligem `MK`, echte SQLite-Datei. Ein Lauf mit und einer ohne `ErasureRegistry` (`FileRegistry`).
2. `create_tenant(T)`, `create_collection`, `register_subject(S)`, `put` eines Dokuments mit zufälligem `marker`, `Subjects::Personal([S])`; `get` liefert den Text; `search` nach dem `marker` findet das Dokument.
3. Prozess beenden und in **neuem Prozess** öffnen: `get` und `search` liefern dasselbe (Persistenz, I-1 der Modellprüfung).
4. `erase(Subject(T, S))` mit Standardmodus: Handle mit `state ≥ Destroyed`.
5. `get` liefert `Err(Erased)` oder `Err(NotFound)`; `search` findet nichts; `subject_state(T, S) ∈ {Destroyed, Completed}`; `register_subject(T, S)` liefert `Erased`.
6. Prozess beenden und in neuem Prozess öffnen: Punkt 5 wiederholt; `health().status == Ok`.
7. **Schlüsselprobe:** `custodian.get(key_id(S))` liefert `NotFound`/`Disabled`; im Verzeichnis `K` fehlt die `.k`-Datei und der Marker `.gone` ist vorhanden.
8. **Unabhängige Prüfung:** Das Python-Referenzprogramm öffnet den (vor dem `erase` kopierten) Envelope mit allen jetzt vorhandenen Schlüsseln **nicht**; auf einem Kontrolldokument eines anderen Betroffenen öffnet es.
9. T-LEAK-1 auf `D`.

### 16.3 Mutationstests

Jeder Mutant wird per Quelltextpatch oder `cargo-mutants` in den Build eingeführt; die CI MUSS **mindestens einen** der genannten Tests rot sehen. Ein Mutant, der alle Tests grün lässt, ist ein Fehler der Tests (TR-2). Zielwert für `cargo-mutants` auf `seal`, `custodian-local` und Löschprotokoll in `acta`: nicht überlebende Mutanten ≥ 90 % **[Ziel]**; die handgeschriebenen Mutanten unten sind **verbindlich** (alle MÜSSEN sterben).

| ID | Mutant | Rot in |
| --- | --- | --- |
| **M-1** | Siegeln per XOR statt AEAD (Bestandsbefund B-01) | T-SEAL-1, T-SEAL-4 |
| **M-2** | Bei Schlüsselfehler Ersatzschlüssel ableiten (B-02, B-03) | T-NOFALLBACK-1, T-DEL-1, T-WALK-1 |
| **M-3** | DEK wird zwischengespeichert und übersteht `erase` (B-37) | T-DEL-1, T-LEAK-1 |
| **M-4** | Nonce konstant oder aus einem Zähler | T-SEAL-1, T-SEAL-3 |
| **M-5** | AAD ohne Header oder mit gekürztem Header | T-SEAL-4 |
| **M-6** | DEK ohne einen `DK_s` abgeleitet (Mehrpersonen-Regel aufgehoben) | T-MULTI-1, T-SEAL-3 |
| **M-7** | `Rng` liefert bei Fehler Nullen (B-07) | T-RNG-1 |
| **M-8** | `derive`/`seal` liefern bei Fehler Nullschlüssel oder leeren Envelope (B-05, B-06) | T-RNG-1, T-SEAL-2, T-NOFALLBACK-1 |
| **M-9** | `destroy` des Custodians meldet Erfolg, tut nichts | T-CUST-1 (Test 3, 10), T-WALK-1 |
| **M-10** | `LocalCustodian.destroy` entfernt die Datei, überschreibt nicht oder legt den Marker nicht an | T-CUST-1, T-WALK-1 |
| **M-11** | Zweite Gate-Prüfung entfernt | T-CONC-1 |
| **M-12** | Erste Gate-Prüfung entfernt (nur zweite) | T-GATE-1, T-DEL-1 |
| **M-13** | Gate wird vor dem durablen Commit sichtbar oder bestätigt | T-CRASH-CP-E2, T-CRASH-CP-E3 |
| **M-14** | `destroy` vor dem Gate (I-6) | T-CRASH-CP-E3, Typtest T-TYPE-1 |
| **M-15** | E4 ohne Probe: `Destroyed` ohne Gegenprobe (I-7) | T-DEL-1 mit Mutanten-Custodian, der `destroy` quittiert und `get` weiter liefert |
| **M-16** | `Completed` ohne vollständigen Purge oder ohne Manifest (I-8) | T-CRASH-CP-E10, T-LOG-1 |
| **M-17** | Zustand kehrt nach `Active` zurück oder Wiederverwendung eines gelöschten `subject_ref` (I-10) | T-OPEN-1, T-IDEM-1 |
| **M-18** | Hash-Kette wird beim Öffnen nicht geprüft oder Trigger fehlen | T-LOG-1, T-STORE-2 |
| **M-19** | `health()` konstant `Ok` (B-14) | T-HEALTH-1 |
| **M-20** | Funktion meldet `Ok`, speichert nichts (B-12) | T-STUB-1, T-WALK-1 |
| **M-21** | Purge löscht bei mehrpersonigen Dokumenten nur Zeilen des Ziels, nicht alle | T-DEL-2, T-MULTI-1 |
| **M-22** | Custodian-Abgleich beim Öffnen unterbleibt oder deutet `Unavailable` als `Gone` | T-RESTORE-1, T-NOFALLBACK-1 |
| **M-23** | Index entfernt Ziel nicht oder nullt Vektorzeilen nicht | T-IDX-5, T-DEL-1 |
| **M-24** | Registry-Eintrag nach statt vor dem Gate | T-REG-1 |
| **M-25** | Mutanten-Custodian `put` überschreibt statt `Conflict` | T-CUST-1 (Test 2) |

### 16.4 Absturzpunkte und Haken

```rust
pub enum CrashPoint { R1, R2, R3, P1, P2, P3, P4, E1, E1R, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, T1, O1, O2, O3, O4 }
```

Die Haken heißen in der Tabelle 9.8 `CP-<Name>` (plus `CP-E1R`, 13.4). Der Aufruf `Hooks::at(CrashPoint)` existiert nur mit `--cfg ctx_crash_hooks`. **Harness:** Ein Kindprozess führt die Operation aus und meldet am Haken `CP <Name>` über eine Pipe, danach **hält er an** (Schlafschleife); der Elternprozess beendet ihn mit `SIGKILL` (`Child::kill`), öffnet das Verzeichnis in einem neuen Prozess und prüft die Erwartung aus 9.8, die Invarianten I-1, I-6 bis I-11 und die Schlüsselprobe. Die Tests sind pro Punkt ein eigener Fall (T-CRASH-`<Name>`), bei `P*` und `E*` zusätzlich mit einem zweiten Betroffenen und einem mehrpersonigen Dokument.

Haken ohne Test gelten als nicht vorhanden; ein Test ohne Haken im Code scheitert (T-ARCH-5 vergleicht die Menge der Haken im Code mit der Tabelle 9.8).

### 16.5 Eigenschaften und Fuzzing

- `proptest`: Round-Trip Envelope (Bauen, Parsen, Öffnen) für zufällige Betroffenenmengen; Index-Eigenschaften (Entfernen und Neuaufbau sind gleich, Reihenfolge bei Gleichstand stabil).
- Fuzzing: T-FUZZ-1 bis -3. Kurzläufe in jeder CI, lange Läufe nachts; Korpus im Repository.
- Referenzimplementierungen außerhalb von Rust: BM25, Vektor, RRF und Envelope in Python (`audit/`). Die Vektoren stehen in Anhang B.

---

## 17. Gates, Plan, Zielwerte

### 17.1 Gates

| Gate | Inhalt | Bedingung (alle MÜSSEN grün sein) |
| --- | --- | --- |
| **G1** Walking Skeleton | Kern vorhanden, kein Stub | T-WALK-1 auf Linux **und** macOS; T-SEAL-1 bis -4; T-NOFALLBACK-1; T-RNG-1; T-DEL-1; T-MULTI-1; T-CUST-1 (`LocalCustodian`); T-STORE-2; T-CONFIG-1; T-LIMIT-1; T-LEAK-1; T-STUB-1; T-TYPE-1 bis -3; T-ARCH-1 bis -8; T-DOC-1 bis -3; Mutanten M-1 bis M-10, M-12, M-14, M-20 tot; TR-1 eingehalten |
| **G2** Protokoll | Löschprotokoll, Wiederaufnahme, Nebenläufigkeit | alle Tests des Katalogs mit Gate G2; T-CRASH-`<CP>` für **jeden** Punkt; alle Mutanten M-1 bis M-25 tot; Fuzzläufe ohne Fund; keine `NotImplemented`-Pfade in Kernfunktionen |
| **G3** Messung | Zahlen werden gemessen | Benchmarkprotokoll auf festgelegter Referenzplattform; jeder **[Ziel]**-Wert ist durch einen Messwert ersetzt oder gestrichen; T-POWER-1 bestanden; Obergrenze je Prozess (8.11) dokumentiert |
| **G4** Krypto-Prüfung | externe Prüfung des Kerns `acta-seal` | Prüfliste 4.12 vollständig beantwortet; Funde behoben oder schriftlich akzeptiert; `cargo audit`/`deny` ohne offene Advisories; Bibliotheksversionen geprüft (4.1) |
| **G5** Recht | Einordnung der Schlüsselvernichtung | schriftliche Einschätzung einer fachkundigen Stelle zum vorgesehenen Einsatz (Art. 17 DSGVO, Klassen A1 und A2); erst danach darf Anhang C erweitert werden |

**Release 1.0** verlangt G1 bis G5, die Spezifikation als „Fassung 1.0“ mit Messwerten (0.2), mindestens einen produktiven Custodian-Adapter der Klasse A1 oder höher mit bestandener T-CUST-1 (Entscheidung O3) und einen Kanarientest auf der echten Konfiguration (14.3).

### 17.2 Reihenfolge

Stoppregel (0.4 Punkt 5): Kein neues Crate, keine neue Entscheidungs-ID, kein neues Dokument, bevor G1 grün ist.

| Paket | Inhalt | Ende, wenn |
| --- | --- | --- |
| **AP0** | Repository anlegen (15.5), CI, Lints, `docs/SPEC.md`, T-DOC-1/-2/-3 und T-ARCH-* rot oder grün | T-ARCH-*, T-DOC-* grün; **T-WALK-1 rot eingecheckt** |
| **AP1** | `acta-seal`: Secret, Envelope, P1, DEK, Rng; Anhang-B-Vektoren | T-SEAL-1 bis -4, T-TYPE-2, M-1, M-4 bis M-8 tot |
| **AP2** | `acta-custodian` und `-local` | T-CUST-1, M-9, M-10, M-25 |
| **AP3** | `acta-store`: Schema, Pragmas, Log-Kette, `BlobStore` | T-STORE-1, T-STORE-2, T-LOG-1 |
| **AP4** | `acta-index` | T-IDX-1 bis -6 |
| **AP5** | Kern: Anlegen, `put`/`get`, Gate, `erase` bis `Destroyed` (ohne Worker-Wiederholung) | **G1** |
| **AP6** | Worker, Wiederaufnahme O1 bis O9, Purge, `Completed`, Mandantenkaskade, Registry, Nachweispaket, Crash-Matrix | **G2** |
| **AP7** | Messungen, Optimierung nach Messung | **G3** |
| **AP8** | Externe Prüfung (G4), Rechtsgutachten (G5), erster A1-Adapter (O3) | Release 1.0 |

Spätere Versionen (nicht v1): `Restrict`/`Lift` (Art. 18, 1.1), Index-Snapshots unter Epochenschlüssel nur bei gemessenem Bedarf (O4).

### 17.3 Entscheidungsregel bei Konflikt

Ein Testfehler wird nie durch Abschwächen des Tests behoben, wenn der Test eine Zusage oder Invariante belegt. Eine Zusage ändert sich nur durch Spezifikationsänderung (0.4). Wer eine Zusage nicht halten kann, streicht sie in der Spezifikation, bevor er sie bricht.

### 17.4 Zielwerte (ungemessen, **[Ziel]**)

Referenzplattform wird bei G3 festgelegt. Die Zahlen sind Rechenannahmen und **keine Zusagen**.

| Größe | Ziel | Annahme |
| --- | --- | --- |
| `erase` bis `Gated` (ohne Register) | p50 ≤ 10 ms **[Ziel]** | ein Commit mit `fsync` plus RAM-Arbeit; mit Register zwei `fsync` |
| `put`, Dokument ≤ 4 KiB | p50 ≤ 15 ms **[Ziel]** | ein Commit |
| `get`, Dokument ≤ 4 KiB | p50 ≤ 1 ms **[Ziel]** | Zeilenlesen, HKDF, AEAD |
| BM25, 10⁵ Dokumente, 3 Terme | p50 ≤ 10 ms **[Ziel]** | |
| exakte Vektorsuche, 10⁵ × 768, k = 10 | p50 ≤ 100 ms **[Ziel]** | 7,7 · 10⁷ Multiplikationen/Additionen, skalar mit 8 Akkumulatoren |
| Öffnen (O8), 10⁵ Dokumente ≤ 4 KiB | ≤ 60 s **[Ziel]** | Entschlüsselung und Indexaufbau, O(N) |
| Purge, 10⁴ Dokumente | ≤ 30 s **[Ziel]** | 10 Chunks zu 1000, je ein `fsync` |
| RAM-Index, 10⁵ × 768 | ≈ 307 MB Vektoren plus Terme **[Ziel]** | Rechenbeispiel aus 8.11 |
| Zeit bis `Destroyed` | **nicht** von Acta bestimmt | hängt vom Custodian ab (K-6, K-8) |

---

## 18. Restrisiken und Grenzen

### 18.1 Restrisiken

Ein Restrisiko ist ein Zustand, den Acta weder verhindert noch aufdeckt. Jedes ist benannt, damit keine Außenaussage es verschweigt (A-6, Anhang C).

| ID | Restrisiko | Wirkung | Zuständig / Gegenmaßnahme außerhalb des Kerns |
| --- | --- | --- | --- |
| **R1** | Klartext im Prozessspeicher: Terme, Vektoren, kurzzeitig Dokumenttext, Schlüssel im RAM; Auslagerung (Swap), Core-Dumps, Hibernation, Speicherabzug durch Dritte | Löschung im RAM ist nur best-effort (Nullen, 8.10); Kopien in Dumps und Swap überleben | Betreiber: verschlüsselter Swap, Dump-Verbot, `mlock` (Prozesshärtung ist Nicht-Ziel 1.5) |
| **R2** | Sicherungen des Custodians | Vernichtung erreicht Sicherungen des Dienstes nicht | Betreiber des Dienstes: Aufbewahrung der Sicherungen höchstens so lang wie die Löschfrist; A2-Dienst |
| **R3** | Kryptografie: Brechung der Primitive, Fehler in Bibliotheken, Quantenrechner; Ciphertext liegt dauerhaft in Sicherungen | Langzeitvertraulichkeit nicht garantiert; keine Schlüsselrotation in v1 | Standard-Primitive (C-1), externe Prüfung (G4), Beobachtung von Advisories |
| **R4** | Metadaten im Klartext der Datenbank: `tenant_id`, `collection_id`, `doc_id`, `subject_refs`, `generation`, Größen, Zeitstempel, Zugriffsmuster | Wer die Zuordnung Person → `subject_ref` kennt, erkennt Existenz und Größe von Daten | Aufrufer: zufällige Bezeichner, keine Ableitung aus Personendaten (7.1, R10) |
| **R5** | Externe Verarbeiter: Embedding-Dienste, LLM-Anbieter, Prompts, Antworten | `erase` erreicht sie nicht | Aufrufer: Verträge, eigene Löschung |
| **R6** | Bereits ausgelieferter Klartext: Antworten, Caches, Protokolle, Kopien beim Aufrufer; Leser, deren zweite Gate-Prüfung vor dem Gate lag (10.3) | Klartext bleibt dort, wo er hingeliefert wurde | Aufrufer |
| **R7** | Kompromittierter Custodian oder Kopie von Schlüsseln **vor** der Vernichtung (W1-Ausnahme) | Wer Ciphertext und Schlüssel vor `destroy` kopierte, kann später lesen | Betreiber: Zugriffskontrolle am Dienst, Audit |
| **R8** | Karenzfrist des Dienstes (K-6): Schlüssel nur `Disabled`, endgültige Löschung erst zum Termin | Bis `scheduled_final_ms` kann der Betreiber des Dienstes den Schlüssel wiederherstellen | im Nachweis ausgewiesen (A-3); Dienst ohne Karenz oder Termin abwarten |
| **R9** | Verlust von `MK` des `LocalCustodian` oder Verlust des Custodians | **alle** Daten dauerhaft unlesbar; kein Hersteller-Escrow | Betreiber: eigene Sicherung der Hauptschlüssel (nicht in der Datenbanksicherung, R2/R14) |
| **R10** | Zuordnung Person → `subject_ref` liegt beim Aufrufer | falsche Zuordnung löscht die falsche Person oder lässt Daten einer Person ungelöscht; zwei Personen auf einem `subject_ref` werden gemeinsam gelöscht; falsch zerlegte Mehrpersonen-Chunks | Aufrufer (7.1, 7.5) |
| **R11** | Wiedereinspielen eines **älteren** Datenverzeichnisses im Fenster `Gated` bis `Destroyed` | Die Sperre geht verloren, wenn weder Custodian (noch nicht vernichtet) noch Löschregister sie kennen | Löschregister (13.4); Betreiber reicht offene Aufträge nach Restore erneut ein |
| **R12** | Ältere Versionen nach `update` in freigegebenen SQLite-Seiten oder im WAL | Ciphertext unter denselben Schlüsseln; bis zur Vernichtung lesbar; `secure_delete` ist Hygiene, keine Zusage; keine Berichtigung per Schlüsselrotation | Aufrufer: `replace_and_erase` (7.5) |
| **R13** | Waisenschlüssel nach abgebrochenem Registrieren (CP-R2) | unschädlich; belegt einen Custodian-Eintrag | Adoption (5.5) |
| **R14** | Sicherungen des Datenverzeichnisses; Hardwareverhalten: Stromausfall, Flash-Wear-Leveling, Firmware, die `fsync` falsch meldet | Garantie ist die von SQLite und Betriebssystem (D5); Reste auf dem Medium nicht auszuschließen | Betreiber: Medienwahl, Sicherungsregel |
| **R15** | Fremdcode mit `unsafe`: gebündeltes SQLite (C), Krypto-Bibliotheken, Standardbibliothek, Betriebssystem | Fehler dort sind nicht von `forbid(unsafe_code)` erfasst | Pins, `cargo audit`/`deny`, G4 |
| **R16** | Seitenkanäle: Zeitverhalten der Suche und der Antwort, Cache- und Hardwareeffekte, Zugriffsmuster im Dateisystem | Hinweise auf Inhalte und Existenz | nicht im Modell (2); Betreiber |
| **R17** | Rechtliche Einordnung: Ob Schlüsselvernichtung eine Löschung nach Art. 17 DSGVO ist; Aufbewahrungspflichten (Art. 17 Abs. 3) | offen (G5) | Verantwortlicher; vor `erase()` zu prüfen |

### 18.2 Zuständigkeiten

| Wer | Pflichten |
| --- | --- |
| **Acta (Bibliothek)** | Zusagen Z1 bis Z7, Invarianten, Format, Tests, ehrliche Rückgaben |
| **Aufrufer (Anwendung)** | Zuordnung Person → `subject_ref` (R10), Zerlegung in Chunks, Prüfung von Aufbewahrungspflichten, eigene Kopien und Caches (R6), externe Verarbeiter (R5) |
| **Betreiber** | Custodian-Betrieb und -Sicherung (R2, R9), Sicherung der Datenverzeichnisse (R14), Prozesshärtung (R1), Löschregister und Wiedereinreichung nach Restore (R11) |
| **Auditor** | Kanarientest und Prüfverfahren auf der echten Konfiguration (14.2, 14.3) |

### 18.3 Aussagen, die nicht zulässig sind

In **keiner** Attestierungsklasse zulässig: „DSGVO-konform“, „garantiert gelöscht“, „unwiederbringlich gelöscht“, „zertifiziert“, „fehlerfrei“, „bewiesen sicher“, „Löschung aus allen Sicherungen“. Zulässig ist der Wortlaut aus Anhang C, höchstens in der Stärke der Klasse (14.4).

---

## 19. Befunde, Zuordnung, Änderungen

### 19.1 Bestandsbefunde (Altrepo `34f00a268`)

Das Altrepo wird nicht umgebaut (0.1). Jeder Befund ist einer Regel und einem Test zugeordnet, sodass er im Neubau nicht wiederkehren kann.

**Stufen:** **[V]** am Quelltext von `34f00a268` in **dieser Sitzung** gelesen (Datei und Zeile genannt) · **[A]** aus den beiden Analysen (Prüfbericht und Modellprüfung) übernommen, hier nicht erneut geprüft.

| ID | Befund | Stufe | Fundstelle (Altrepo) | v2-Regel | Test |
| --- | --- | --- | --- | --- | --- |
| **B-01** | `seal`/`open` sind ein XOR mit einem 32-Byte-Muster (`stream_key` per `blake3::keyed_hash`, `mask[i % 32]`), ohne Nonce, Authentisierung, AAD | [V] | `acta-erasure/src/vault/sealer.rs:170-194` | C-1, C-2 | M-1, T-SEAL-1, T-SEAL-4 |
| **B-02** | `resolve_dek` fällt bei jedem Fehler von `derive_dek_from_vault` auf `blake3::keyed_hash(k_unit, key)` zurück; nach `erase` bleibt der Record lesbar | [V] | `sealer.rs:117-148` (Fallback Zeile 143) | I-2, C-4 | M-2, T-NOFALLBACK-1 |
| **B-03** | `K_unit = [0x55; 32]` konstant im `VaultSealer::new()`; Dateischlüssel-Basis `k_enc = [0x42; 32]` in `VaultKeyring::create` | [V] | `sealer.rs:45`, `keyring.rs:73` | I-4, C-3 | T-NOFALLBACK-1, T-LEAK-1 |
| **B-04** | Hierarchie-Tags (Mandant, Collection, Dokument) werden im Fallback alle aus den ersten 16 Bytes des Record-Keys gebildet | [V] | `sealer.rs:127-137` | 4.4 | T-SEAL-3 |
| **B-05** | `derive_dek` füllt bei HKDF-Fehler den Schlüssel mit Nullen und liefert ihn | [V] | `acta-crypto/src/dek.rs:52-53` | I-5, 4.9 | M-8 |
| **B-06** | Verschlüsselungsfehler liefert `Vec::new()` als Ergebnis | [V] | `acta-crypto/src/aead.rs:224` | I-5, 4.9 | M-8, T-SEAL-2 |
| **B-07** | `OsRng` der Fassade liefert bei `getrandom`-Fehler Nullen | [V] | `acta/src/wiring.rs:23,30` | I-5, 4.7 | M-7, T-RNG-1 |
| **B-08** | `t_vault_2_and_4_sealer_seal_open_evict` ist nur ein Round-Trip; es gibt keinen Test „Slot vernichten, dann `open` scheitert“ | [A] | `acta-erasure` | TR-2 | M-1 bis M-3 |
| **B-09** | `Rng` ist als austauschbarer Port in die Verdrahtung injizierbar (`create_vault(_rng: Arc<dyn Rng>)`); die Schnittstelle (`next_u64`, `fill_bytes`) kann nicht scheitern | [V] | `acta-engine/src/wiring.rs:8-13` | 4.7 | T-RNG-1, T-ARCH-5 |
| **B-10** | `search_text`/`search_vector`/`scan` setzen `exhausted: true` konstant | [V] | `acta-engine/src/collection.rs:198,212` | I-15, 8.8 Punkt 4 | T-STUB-1, T-HEALTH-1 |
| **B-11** | `create_vault` ignoriert VFS, Uhr, Rng und Pfad und baut `FileVault::new_in_memory(1 MiB)`; der Tresor ist nicht persistent. (`vault_id` und MAC-Schlüssel mit Nullwerten: [A]) | [V] | `wiring.rs:15` | I-4, 5.1 | T-WALK-1 (Neustart) |
| **B-12** | `Collection::insert` prüft Grenzen und gibt `Ok(())` zurück, ohne zu speichern; `soft_delete` nimmt nur ein Lock | [V] (soft_delete), [A] (insert-Rest) | `collection.rs:142-181` | Z6 | M-20, T-STUB-1 |
| **B-13** | `Collection::get` liefert immer `Ok(None)` | [V] | `collection.rs:171-173` | Z6 | T-STUB-1, T-WALK-1 |
| **B-14** | `Acta::health` meldet hartkodiert `status: "OK"` | [V] | `acta/src/api.rs:406` | I-15 | M-19, T-HEALTH-1 |
| **B-15** | `apply_process_hardening` gibt `Ok(true)` ohne Wirkung zurück | [V] | `acta-engine/src/hardening.rs:9` | I-15, Nicht-Ziel 1.5 | T-STUB-1 |
| **B-16** | `execute_startup_sequence` berechnet den Sperrpfad `LOCK` und nimmt die Sperre nie (`_lock_path`) | [V] | `acta-engine/src/startup.rs:17` | I-17 | T-LOCK-1 |
| **B-17** | Zwei inkompatible Typuniversen in `acta-types` und `acta-ports` (`KeyBuf` 78 gegen 16 Byte, `FileId` `[u8;16]` gegen `u64` u. a.) | [A] | `types`, `ports` | V-6 | T-ARCH-3 |
| **B-18** | `LayerWitness::empty()` und `from_hash` sind `pub const fn`; jeder kann Zeugen fälschen | [V] | `acta-ports/src/erasure.rs:190,197` | V-4, 15.6 | T-TYPE-1 |
| **B-19** | `GLOBAL_GATE_SET` ist ein prozessglobaler Zustand (`static` mit `RwLock`) | [V] | `acta-erasure/src/gate/batch.rs:197` | I-13, V-7 | T-ARCH-4 |
| **B-20** | `DocStoreLayer`: `purge` liefert `PurgeOutcome::new(0, 0)`, `witness` liefert `empty()`, `key_coverage()` meldet immer `Full` | [A] | `acta-store/src/erase/layer.rs` | I-15, 15.6 | T-DEL-2 |
| **B-21** | Fehler der Saga nach dem Gate werden nicht journalisiert; Gate liegt, kein `Pending` | [A] | `saga/core.rs` | Z4, 9.1 `pending` | T-CRASH-n, T-WORKER-1 |
| **B-22** | Ledger-Eintrag `Completed` vor Journal-`Completed` geschrieben | [A] | `saga/core.rs` | I-8, 9.4 (eine Transaktion) | T-CRASH-CP-E12 |
| **B-23** | Workspace lädt nicht: `Cargo.toml` listet `testkit`, `testmacros`, `ctx-verify`, die Verzeichnisse fehlen (im Klon: 15 Crate-Verzeichnisse) | [V] | `Cargo.toml` (members) | T-ARCH-1, 15.2 | T-ARCH-1 |
| **B-24** | Keine normative Spezifikation im Baum; `docs/` enthält nur zwei generierte Schnittstellenspezifikationen und `evidence/` | [V] | `docs/` | 0.4 | T-DOC-1 |
| **B-25** | Die Fassade ignoriert `DocRef.subject` beim Insert; ohne Zuordnung kein Löschen pro Betroffenem | [A] | `acta/src/api.rs` | 7.2 `Subjects` Pflicht | T-DEL-1, T-MULTI-1 |
| **B-26** | `search_text` bildet `DocTag` aus den ersten 16 Bytes der `doc_id` (gekürzt, aufgefüllt) | [A] | `acta/src/api.rs` | I-18 | T-TYPE-3 |
| **B-27** | Fest einkompilierter Legacy-Integritätsschlüssel `LEGACY_INTEGRITY_KEY_DEOBFUSCATED` für die WAL-HMAC-Kette | [V] | `acta-store/src/wal/hmac.rs:13,57` | I-4, V-10 | T-ARCH-8 |
| **B-28** | Konfigurationsarithmetik: Standard-`batch_window` 60 s gegen `erase_sla ≥ 1 min` und `batch_window ≤ erase_sla/2` | [A] | `acta-engine/src/builder.rs` | 12.1 | T-CONFIG-1 |
| **B-29** | `vault_dir` wird angenommen und verworfen (`_vault_dir`); keine Trennung von Tresor und Daten | [V] | `wiring.rs:12` | 13.2, 5.6 | T-WALK-1, T-CUST-1 |
| **B-30** | `TenantId`: `SYSTEM`, `DEFAULT`, `INVALID` alle `0` | [A] | `acta-types/.../ids.rs` | 7.1 | T-LIMIT-1 |
| **B-31** | `TOMBSTONE_BIT = 1 << 63` maskiert Sequenzen stillschweigend (Aliasing) | [A] | `acta-mvcc` | I-14 | T-LIMIT-1 |
| **B-32** | Verbotene Features und Abhängigkeiten: `bincode`, `memmap2` und Tokio-Feature `fs` im Workspace; `docid-128`, `memory-only-storage`, `fault-injection` u. a. in Crates | [V] (bincode, memmap2, `fs`, `docid-128`), [A] (übrige) | `Cargo.toml:56,60,64`, `*/Cargo.toml` | V-8, V-9, 15.3 | T-ARCH-2, T-ARCH-5 |
| **B-33** | DEK wird aus langlebigem `K_unit` plus Salt abgeleitet (Löschen = Salt entfernen); Löschaussage hängt an Medieneigenschaften | [A] | `acta-crypto/src/dek.rs` | I-4, 4.4 (zufällige `DK_s` beim Custodian) | T-DEL-1, T-CANARY-1 |
| **B-34** | Passphrase als `Vec<u8>` ohne Nullen, `Drop` setzt nur ein Flag | [A] | `acta-engine` | 4.8, 5.6 | T-TYPE-2 |
| **B-35** | Feature `docid-128` schaltet die Breite eines persistenten Identifikators per `cfg` | [V] | `acta-types/Cargo.toml:18` | 15.6, V-8 | T-ARCH-5 |
| **B-36** | `Tenant::collection` → `collection_for_tenant` erzeugt bei **jedem** Aufruf eine neue `Collection` mit neuen Sperren; alle Operationen sperren `dummy_tag = [0;16]` | [V] | `acta-engine/src/instance.rs:99-107`, `collection.rs:158` | 10.6 | T-CONC-2 |
| **B-37** | `VaultSealer` hält einen `dek_cache`, der abgeleitete DEKs dauerhaft speichert; nach `erase` entschlüsselt der Cache weiter | [V] | `sealer.rs:117-123,145-147` | C-7, 4.2 | M-3, T-DEL-1, T-LEAK-1 |

### 19.2 Zuordnung der Prüfberichte

Der **Prüfbericht** bezog sich auf die Altspezifikation 5.1.0 (Stand `34f00a268~1`), die **Modellprüfung** auf den Code. Spalte „Behandlung“: *entfällt* = Gegenstand existiert in v2 nicht (Architektur), *Regel* = durch eine v2-Regel gelöst, *bleibt* = als Restrisiko geführt.

| Befund | Gegenstand | Behandlung |
| --- | --- | --- |
| REPO-1, B-23 | Workspace nicht ladbar | Regel: Neubau, T-ARCH-1; Matrix 15.2 |
| REPO-2, B-24 | Normative Dokumente gelöscht | Regel: 0.4, T-DOC-1 bis -3 |
| REPO-3 | generierte Schnittstellenspezifikation beschreibt ein anderes Produkt | Regel: 0.4 Punkt 4, T-ARCH-8 |
| REPO-4 | verbotene CI-Workflows | entfällt; CI-Aufträge 15.4 |
| REPO-5 | Ratchet-Zahlen | Regel: Budgets 15.1, T-ARCH-7 |
| MODELL-1 | AAD bindet `commit_seq`, Siegeln vor Commit | entfällt (kein MVCC, kein `commit_seq`); AAD ist der Header (4.5) |
| MODELL-2 | Lock-Reihenfolge im Purge | entfällt als Modell; Regel: 10.1, T-CONC-4 |
| MODELL-3 | Restore-Guard im Fenster `Gated` | Regel: 13.4, O5/O6; bleibt als R11 |
| MODELL-4 | Tresor SPOF, nicht sektorsicherer Slot | entfällt (kein Tresor); bleibt als R9 |
| MODELL-5 | Zwangs-Rehash blockiert `erase` | entfällt (kein Tresor) |
| MODELL-6 | `D`-Slot-Aufzählung nach LSM-Purge | entfällt; Regel: Manifest je Chunk vor dem Löschen (9.4 E6), Kaskadenfortschritt je Betroffenem durabel (9.5) |
| MODELL-7 | WAL-Torn-Write-Regel | entfällt (SQLite); T-STORE-1, T-POWER-1 |
| MODELL-8, B-28 | `erase_sla` mit drei Bedeutungen | Regel: 12.1 (kein `batch_window`; `erase_sla` = Abschlussfrist; `erase_return_timeout`), T-CONFIG-1 |
| MODELL-9 | Nonce-Zähler nach Abschneiden | entfällt; 192-Bit-Zufallsnonce (4.10) |
| MODELL-10 | Tresor-HEADER, KDF-Parameter, zirkuläre Startreihenfolge | entfällt; O1 braucht nur `data_dir`, `store_id` wird erst in O2 gelesen und erst in O5 für `KeyId` gebraucht |
| WID-1, WID-2, WID-3, WID-4, WID-6, WID-9 | Widersprüche der Altspezifikation | entfallen mit ihr; Matrix 15.2 enthält `acta` als Quelle und Ziel |
| WID-5 | `Erased` gegen `NotFound` nach `Completed` | Regel: 10.3 (Rückgabe nach E2b), `subject_state`; siehe 19.3 Punkt 4 |
| WID-7, B-10 | Kandidatentiefe, `exhausted` | Regel: exakte Suche, kein Statusfeld (8.5, 8.8) |
| WID-8 | `lto` | Regel: 15.4 (`fat`, bei G3 festgelegt) |
| CODE-1, B-30 | `TenantId` 0 | Regel: 7.1; T-LIMIT-1 |
| CODE-2, B-31 | Tombstone-Bit, Aliasing | entfällt; Tombstone ist Spalte (6.3); `generation` überläuft nie (11.3) |
| CODE-3 | Legacy-Typen im Kern | entfällt; V-6, V-10 |
| CODE-4, B-32 | verbotene Features und Abhängigkeiten | Regel: 15.3, 15.4, T-ARCH-2, T-ARCH-5 |
| CODE-5 | Release-Profil gegen Zero-Panic | Regel: 10.5, 15.4 |
| Modellprüfung 2 (Spezifikationswidersprüche) | | entfällt mit der Altspezifikation |
| Modellprüfung 3.1 bis 3.4 | Sealer, Tresor, Silent Success, Typuniversen | B-01 bis B-20, B-36, B-37 |
| Modellprüfung 3.5 | Ableitung statt Wrapping | Regel: I-4; zufällige `DK_s`, nur beim Custodian (B-33) |
| Modellprüfung 3.6 | Fehlerpfade der Saga | Regel: `pending` in der Datenbank (6.3, 9.1), T-CRASH, T-WORKER-1 (B-21, B-22) |
| Modellprüfung 3.7 | Altlasten | Regel: V-10, 15.3 (B-27, B-32) |
| Modellprüfung 4.1 | ein Betroffener je Dokument | Regel: Z3, 7.2, 7.5; bleibt als R10 |
| Modellprüfung 4.2 | Index-Klartext im RAM | Regel: 8.10; bleibt als R1 |
| Modellprüfung 4.3 | selbst ausgestellter Löschbeweis | Regel: 1.2, Kapitel 14 |
| Modellprüfung 4.4 | Backups, Escrow | bleibt als R2, R9, R14 |
| Modellprüfung 4.5 | Rechtswirkung | bleibt als R17, G5 |
| Modellprüfung 5 (Invarianten I-1 bis I-10, Walking Skeleton) | | übernommen und erweitert zu I-1 bis I-18, T-WALK-1 |

### 19.3 Änderungsabschnitt

**Versionsverlauf**

| Fassung | Stand | Inhalt |
| --- | --- | --- |
| 0.1 | — | erster Entwurf (ersetzt; Datum liegt mir nicht vor) |
| **0.2** | 2026-11-06 | vollständiger Entwurf mit Zielwerten: Teil 1 (Kapitel 0 bis 4), Teil 2 (Kapitel 5 bis 9), Teil 3 (Kapitel 10 bis 20, Anhang A bis C). Anhang B in dieser Sitzung mit unabhängigem Code neu ausgeführt und um das Referenzprogramm ergänzt |

**Beim Zusammenführen festgestellte Unstimmigkeiten.** Teil 1 und Teil 2 sind unverändert übernommen. Folgendes ist dort **nicht** still korrigiert, sondern hier geführt und, wo möglich, in Teil 3 entschieden:

1. **9.7 gegen 9.4.** 9.7 sagt, der Worker halte keine Sperre über einen `fsync`; die Transaktionen E1, E5, E6, E8 halten `W` während des Commits. *Geklärt in 10.1 Regel 4:* Die Regel gilt für E3, E4 und die Wartezeiten, nicht für die Commits.
2. **I-7 gegen E4.** I-7 verlangt ein Probeöffnen eines Envelopes des Ziels; E4 beschreibt als Probe `custodian.get`. *Festlegung:* E4 führt die `get`-Probe immer aus und zusätzlich ein Probeöffnen, sofern das Ziel Dokumente hat; T-DEL-1 prüft beides. Ein Ziel ohne Dokumente braucht nur die `get`-Probe.
3. **`foreign_keys = ON` ohne `REFERENCES`.** Das Schema in 6.3 enthält keine Fremdschlüssel; das Pragma ist wirkungslos. *Offen als O9.*
4. **`get` nach der Sperre.** 7.3 Schritt 1 liefert `NotFound` bei fehlendem Katalogeintrag; 8.10 entfernt die Dokumente des Ziels aus dem Katalog. Damit liefert `get` nach E2b `NotFound`, nicht `Erased` (`Erased` nur im Fenster E2a bis E2b). *Festlegung in 10.3:* beides zulässig; `subject_state` und `tenant_state` liefern den gemessenen Zustand; T-WALK-1 akzeptiert `Erased` oder `NotFound`.
5. **SQLite-Mindestversion.** 6.2 begründet 3.40 mit `STRICT`. Nach meiner Kenntnis führt SQLite `STRICT`-Tabellen seit 3.37 ein; 3.40 ist hinreichend, die Begründung ist ungenau. Das ist **nicht gegen die SQLite-Dokumentation geprüft** und vor G1 zu prüfen.
6. **`manifest_root` ohne `collection_id`.** Die Wurzel in 9.4 E8 nutzt `doc_id ‖ envelope_sha256`, sortiert nach `(collection_id, doc_id)`; `collection_id` selbst geht nicht ein. Die Bindung an die Collection entsteht über den Hash des Envelopes (sein Header enthält `collection_id`). Das ist ausreichend, aber nicht offensichtlich; **keine Änderung**, der Auditor-Algorithmus (V4, Anhang B) folgt 9.4 wörtlich.
7. **`Protocol(u16)` gegen `Protocol(AuthFail)`.** 5.2 hat einen `u16`-Code, 5.6 nennt `AuthFail`. *Festlegung:* `PROTOCOL_AUTHFAIL = 1` (Anhang A).
8. **Kollision der Kennung `O`.** 0.3 führt `O` für offene Entscheidungen; 9.6 nummeriert die Öffnungsschritte ebenfalls `O1` bis `O9` (und 8.1, 13.4 verweisen auf sie). *Festlegung für diese Fassung:* Ohne Zusatz bezeichnet `O<n>` in den Kapiteln 8, 9, 13 den **Öffnungsschritt**; offene Entscheidungen werden als „Entscheidung O<n>“ zitiert und stehen in Kapitel 20 (Verweise in 4.1 und 8.11 sind Entscheidungsverweise). Eine Umbenennung der Entscheidungen (etwa `E-n`) ist der saubere Weg und gehört in die nächste Fassung; sie ändert Teil 1 und Teil 2 und ist deshalb hier nicht vorgenommen.

**In Teil 1 und Teil 2 genannt, aber nicht definiert; in Teil 3 festgelegt:** Zustands-, Log-, Ziel-, Pending- und Grundcodes (Anhang A); `P1-Overhead` (65 536 Byte, Anhang A); `max_doc_bytes` und `max_text_bytes` und ihr Verhältnis (12.1); Überlauf von `generation` (11.3); `Hooks`, `CrashPoint` (16.4); `Clock`, `Observer`, `Health`, `Event` (12.3, 12.4); `ErasureRegistry` (13.4); `AttestationClass` und Klassendefinitionen (14.4); Regel A-5 (14.3); Rückgabe von `get` nach der Sperre (10.3); zusätzlicher Absturzpunkt CP-E1R (13.4).

**Weitere Änderungen von Teil 3 gegenüber den Teilen 1 und 2:** keine; Teil 3 ändert **keine** Zusage, Invariante oder Regel aus Teil 1 und Teil 2.

---

## 20. Offene Entscheidungen

**Zitierweise:** Die Kennungen `O1` bis `O11` bezeichnen hier **Entscheidungen**; in Kapitel 9.6 bezeichnen `O1` bis `O9` die **Öffnungsschritte** (19.3 Punkt 8). Eine Entscheidung ist offen, bis ihr **Status** `entschieden` ist. Vorläufig Entschiedenes nennt seine Revisionsbedingung.

| ID | Frage | Vorschlag | Revisionsbedingung / Kriterium | Status |
| --- | --- | --- | --- | --- |
| **O1** | Speicher | SQLite, kein Eigenbau (6.1, 1.5) | G3 zeigt SQLite als untauglich für die Zielwerte | vorläufig entschieden |
| **O2** | Chiffre und KDF | XChaCha20-Poly1305, HKDF-SHA256 (4.1). Alternative: AES-256-GCM-SIV (nonce-missbrauchsresistent, Hardwarebeschleunigung) | Prüfung der Bibliotheken (4.1) und externer Prüfer (G4) | vorläufig entschieden unter Vorbehalt G4 |
| **O3** | Erster produktiver Custodian-Adapter | Key-Dienst der Art Vault-KV oder Cloud-KMS (5.7) | signierter Beleg (A2), eigenes Audit-Log, Verhalten bei Karenz (viele KMS-Dienste erzwingen eine Wartezeit, R8) | **offen** |
| **O4** | Index für mehr als etwa 10⁶ Chunks je Prozess | Version 1.1, Snapshots unter Epochenschlüssel (8.11) | nur wenn G3 den Bedarf misst | zurückgestellt |
| **O5** | Lesezugriff und `locking_mode = EXCLUSIVE` | v1: eine Verbindung, Leser warten auf Commits (10.2). Alternative: WAL ohne `EXCLUSIVE` mit mehreren Verbindungen; die WAL-Indexdatei (`-shm`) wird dann per Speicherabbildung geteilt, was D3 (kein `mmap`) widerspricht | G3 misst die Wartezeit der Leser; wird sie untragbar, ist D3 zu lockern **oder** eine Lesekopie zu bauen | offen |
| **O6** | `strict_open` Standard `true` | `true`: ein beschädigtes Dokument verhindert den Start (Integrität vor Verfügbarkeit); Quarantäne ist abschaltbar vorhanden | Betriebserfahrung | vorläufig entschieden |
| **O7** | Passphrase- und Schlüsselverwaltung für `MK` | bleibt beim Aufrufer (5.6); optionales Hilfscrate außerhalb des Kerns | Bedarf von Anwendern | offen |
| **O8** | Mehrere Prozesse (Leser-Replikate) | Nicht-Ziel v1 (I-17) | Bedarf nach 1.0 | zurückgestellt |
| **O9** | `foreign_keys` und `REFERENCES` (19.3 Punkt 3) | Pragma behalten und Referenz `doc_subjects → docs` ergänzen (verhindert verwaiste Zeilen); `erase_manifest` und `erase_log` ohne Referenzen | Entscheidung vor G1, weil das Schema Format ist | **offen** |
| **O10** | Zeitquelle | Wanduhr ohne Monotonie; Abweichungen zeigt nur A-5 an | Vorfälle | vorläufig entschieden |
| **O11** | Quarantäneliste nur im RAM oder persistiert | RAM; Neuaufbau beim Öffnen findet dieselben Dokumente erneut | Betriebserfahrung | vorläufig entschieden |

---

## Anhang A – Konstanten (normativ)

Jede Konstante steht im Code genau einmal (0.4 Punkt 2, T-DOC-2). Das Testprogramm liest diesen Anhang: Jede Zeile der Form `` `NAME` | Wert `` wird gegen die gleichnamige Konstante im Code verglichen. Mehrbyteige Zahlen sind big-endian (0.3).

### A.1 Format und Kodierung

| Name | Wert | Verweis |
| --- | --- | --- |
| `ENV_MAGIC` | `"CTX2"` (`43 54 58 32`) | 4.5 |
| `ENV_VERSION` | `0x01` | 4.5 |
| `ENV_ALG_XCHACHA20POLY1305` | `0x01` | 4.5 |
| `ENV_HEADER_FIXED_LEN` | 88 (Offset des Chiffrats bei n = 0) | 4.5 |
| `ENV_SUBJECT_REF_LEN` | 16 | 4.5 |
| `ENV_NONCE_LEN` | 24 | 4.5 |
| `ENV_TAG_LEN` | 16 | 4.5 |
| `ENV_MAX_SUBJECTS` | 16 | 4.5 |
| `P1_OVERHEAD` | 65536 Byte (Obergrenze der CBOR-Kodierung über die Nutzdaten hinaus: Map-Köpfe, Schlüsselnamen, Metastrukturen) | 4.5, 4.6 |
| `P1_VERSION` | 1 | 4.6 |
| `P1_EMB_ELEM_LEN` | 4 (IEEE-754 binary32, little-endian) | 4.6 |
| `ID_LEN` | 16 (Mandant, Collection, Dokument, Betroffener, `erase_id`) | 11.1 |
| `SECRET_LEN` | 32 | 4.2 |
| `KEY_ID_MAX_LEN` | 160 | 5.2 |
| `STORE_ID_LEN` | 16 | 5.4 |
| `LOCAL_CUSTODIAN_FILE_VERSION` | `0x01` | 5.6 |

### A.2 Labels (Domänentrennung, C-6)

Jedes Label wird für **genau einen** Zweck verwendet. Zeichen `0x00` steht, wo „‖ 0x00 ‖“ angegeben ist.

| Name | Wert | Zweck |
| --- | --- | --- |
| `LABEL_DEK` | `"ctx/v2/dek"` | `info` der DEK-Ableitung (4.4) |
| `LABEL_LOG_GENESIS` | `"ctx/v2/log-genesis"` | Genesis der Löschlog-Kette (6.6) |
| `LABEL_LOG` | `"ctx/v2/log"` | Eintragshash der Löschlog-Kette (6.6) |
| `LABEL_MANIFEST` | `"ctx/v2/manifest"` | Manifestwurzel (9.4 E8) |
| `LABEL_LOCAL_AAD` | `"ctx2/local/"` | AAD der Einträge des `LocalCustodian` (5.6) |
| `KEY_ID_PREFIX` | `"ctx2/"` | Namensraum der `KeyId` (5.4) |

### A.3 Zustands- und Ursachencodes (persistiert)

| Name | Wert | Bedeutung |
| --- | --- | --- |
| `STATE_ACTIVE` | 0 | `Active` |
| `STATE_GATED` | 1 | `Gated` |
| `STATE_DESTROYED` | 2 | `Destroyed` |
| `STATE_COMPLETED` | 3 | `Completed` |
| `PENDING_CUSTODIAN_UNAVAILABLE` | 1 | `PendingReason::CustodianUnavailable` |
| `PENDING_CUSTODIAN_INCONSISTENT` | 2 | `PendingReason::CustodianInconsistent` |
| `PENDING_IO` | 3 | `PendingReason::Io` |
| `PENDING_PURGE_FAILED` | 4 | `PendingReason::PurgeFailed` |
| `LOG_KIND_GATED` | 1 | Log-Eintrag `GATED` |
| `LOG_KIND_DESTROYED` | 2 | Log-Eintrag `DESTROYED` |
| `LOG_KIND_COMPLETED` | 3 | Log-Eintrag `COMPLETED` |
| `LOG_KIND_RECONCILED` | 4 | Log-Eintrag `RECONCILED` |
| `TARGET_SUBJECT` | 1 | Löschziel Betroffener |
| `TARGET_TENANT` | 2 | Löschziel Mandant (`subject_ref` = 16 × `0x00`) |
| `LOG_REASON_REQUEST` | 0 | Aufruf von `erase` (und `GATED`, `DESTROYED`, `COMPLETED` allgemein) |
| `LOG_REASON_STATE_MISMATCH` | 1 | Abgleich Zeilenzustand gegen Log (O3) |
| `LOG_REASON_KEY_GONE` | 2 | Abgleich: Schlüssel beim Custodian fehlt (O5) |
| `LOG_REASON_REGISTRY` | 3 | Abgleich: Eintrag im Löschregister (O6) |
| `METRIC_COSINE` | 0 | `Metric::Cosine` |
| `METRIC_DOT` | 1 | `Metric::Dot` |
| `PROTOCOL_AUTHFAIL` | 1 | `CustodianError::Protocol(1)`: falscher Hauptschlüssel, Authentisierungsfehler des Adapters (5.6) |

### A.4 Datenbank

| Name | Wert | Verweis |
| --- | --- | --- |
| `SQLITE_APPLICATION_ID` | `0x43545832` (`CTX2`) | 6.2 |
| `SQLITE_USER_VERSION` | 2 | 6.2 |
| `SQLITE_MIN_VERSION` | 3.40 (Prüfung offen, 19.3 Punkt 5) | 6.2 |
| `META_KEY_STORE_ID` | `"store_id"` | 6.3 |
| `META_KEY_CREATED_MS` | `"created_ms"` | 6.3 |
| `META_KEY_SCHEMA` | `"schema"` (Wert 2) | 6.3 |
| `LOCK_FILE_NAME` | `"LOCK"` | 13.1 |
| `DB_FILE_NAME` | `"ctx.sqlite3"` | 13.1 |

### A.5 Limits und Zahlenwerte

| Name | Wert | Verweis |
| --- | --- | --- |
| `LIMIT_K_MAX` | 1000 | 8.9 |
| `LIMIT_QUERY_BYTES` | 4096 | 8.9 |
| `LIMIT_DIM_MAX` | 4096 | 6.3, 8.9 |
| `LIMIT_FILTER_VALUES_PER_KEY` | 1024 | 8.2 |
| `LIMIT_TOKEN_BYTES` | 64 | 8.3 |
| `LIMIT_META_KEY_BYTES` | 64 | 4.6 |
| `LIMIT_META_VALUE_BYTES` | 1024 | 4.6 |
| `LIMIT_META_ENTRIES` | 32 | 4.6 |
| `LIMIT_SUBJECTS_PER_DOC` | 16 | 7.2 |
| `DEFAULT_MAX_DOC_BYTES` | 4194304 (4 MiB) | 6.7, 12.1 |
| `DEFAULT_MAX_TEXT_BYTES` | 1048576 (1 MiB) | 12.1 |
| `DEFAULT_PURGE_CHUNK_DOCS` | 1000 | 9.4 E6 |
| `DEFAULT_RRF_DEPTH` | 200 | 8.6 |
| `RRF_DEPTH_MAX` | 1000 | 8.6 |
| `BM25_K1` | 1.2 | 8.4 |
| `BM25_B` | 0.75 | 8.4 |
| `RRF_K` | 60 | 8.6 |
| `VECTOR_ACCUMULATORS` | 8 | 8.5 |
| `INDEX_REBUILD_FREE_RATIO_PERCENT` | 25 | 8.10 |
| `INDEX_ESTIMATE_PER_DOC_FIXED` | 32 (plus `4·dim`, plus 8 je Posting, plus `INDEX_ESTIMATE_CATALOG` = 64) | 8.9 |
| `DEFAULT_ERASE_RETURN_TIMEOUT_S` | 60 | 12.1 |
| `DEFAULT_ERASE_SLA_S` | 86400 (24 h) | 12.1 |
| `DEFAULT_CLOCK_SKEW_TOLERANCE_S` | 300 | 12.1 |
| `BACKOFF_INITIAL_MS` | 1000 | 9.7 |
| `BACKOFF_MAX_MS` | 900000 (15 min) | 9.7 |
| `REGISTRY_ENTRY_LEN` | 73 (`store_id` 16, `erase_id` 16, `target_kind` 1, `tenant_id` 16, `subject_ref` 16, `ts_ms` 8) | 13.4 |
| `DATA_DIR_MODE` | `0700` | 13.1 |

### A.6 Feste Aussagetexte

`LIMITS_TEXT` (14.3 A-6, nicht abschaltbar, wörtlich im Nachweispaket):

> Dieser Nachweis erfasst nicht: Arbeitsspeicher, Auslagerung, Speicherabzüge und Ruhezustand (R1); Sicherungen des Key-Dienstes (R2) und des Datenverzeichnisses (R14); externe Verarbeiter (R5); bereits ausgelieferter Klartext (R6); Schlüsselkopien vor der Vernichtung (R7); die Karenzfrist des Dienstes (R8); die rechtliche Einordnung (R17). Der Nachweis nennt seine Attestierungsklasse; er ist keine Bestätigung der Rechtmäßigkeit.

`WORDING_STANDARD` (Anhang C.1) steht dort wörtlich und wird von T-DOC-2 gegen den Code verglichen.

---

## Anhang B – Testvektoren und Referenzprogramm (normativ)

### B.0 Herkunft und Stand

Die Vektoren wurden am **2026-11-06** mit zwei voneinander unabhängigen Wegen erzeugt und stimmen byteweise überein:

1. **libsodium** über PyNaCl 1.6.2 (`crypto_aead_xchacha20poly1305_ietf_*`).
2. **Eigener Code** für HChaCha20 (Teilschlüssel aus den ersten 16 Nonce-Bytes) plus `ChaCha20Poly1305` der Bibliothek `cryptography` 50.0.1 mit der Nonce `00 00 00 00 ‖ Nonce[16..24]`. Die Konstruktion folgt XChaCha20 nach `draft-irtf-cfrg-xchacha`; der eigene HChaCha20 liefert für den Prüfvektor des Entwurfs (Schlüssel `00 … 1f`, Nonce `00000009 0000004a 00000000 31415927`) den Teilschlüssel `82413b4227b27bfed30e42508a877d73a0f9e4d58a74a853c12ec41326d3ecdc`.

HKDF wurde mit eigenem Code (HMAC-SHA256, RFC 5869) berechnet und gegen `HKDFExpand` der Bibliothek `cryptography` gegengeprüft. P1 wurde von Hand kodiert und mit `cbor2` 6.2.0.post2 gegengeprüft (Dekodierung und kanonische Kodierung ergeben dieselben Bytes). Python 3.13. Die libsodium-Version hinter PyNaCl wurde nicht ausgelesen.

**Was das belegt und was nicht:** Es belegt, dass die in 4.4, 4.5 und 6.6 beschriebenen Formate mit zwei unabhängigen Implementierungen **reproduzierbar** sind, dass das Referenzprogramm (B.4) sie öffnet und dass keine Ein-Bit-Änderung der Envelopes `A` und `B` (jedes Bit jedes Bytes, 1248 und 992 Versuche) angenommen wird. Es belegt **nicht**, dass der spätere Rust-Code richtig ist (das tut T-SEAL-1 gegen diese Vektoren) und nicht, dass die Kryptografie sicher ist (G4).

Alle Schlüssel in diesem Anhang sind **öffentliche Testwerte**. Sie dürfen nie als Geheimnisse verwendet werden.

### B.1 Eingaben

| Größe | Wert (hex) |
| --- | --- |
| `TK_t` | `00 01 02 … 1f` (32 Byte, aufsteigend ab `00`) |
| `DK_s1` | `20 21 22 … 3f` |
| `DK_s2` | `40 41 42 … 5f` |
| `tenant_id` | `a1` × 16 |
| `collection_id` | `b2` × 16 |
| `doc_id` | `c3` × 16 |
| `subject_ref` s1 | `01` × 16 |
| `subject_ref` s2 | `02` × 16 |
| `generation` | 1 |
| `nonce` (nur Testvektor; produktiv ist sie zufällig, 4.7) | `80 81 82 … 97` (24 Byte) |
| Klartext P1 | `a261760164746578746a48616c6c6f2057656c74` = `{"v": 1, "text": "Hallo Welt"}` (20 Byte, kanonisches CBOR) |

### B.2 Vektor A: zwei Betroffene (n = 2)

```
IKM  = TK_t ‖ DK_s1 ‖ DK_s2                       (96 Byte)
PRK  = aea6f612ed72ec34b3c56ea5c8a2c698e96e45eb1db6af603aa5ba6c4beebfc6
info = 6374782f76322f64656b00a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b202010101010101010101010101010101010202020202020202020202020202020200000001
DEK  = eab2a4389683dda6abb64a468d7ad329caa4e8cb1b55761013cebc996cc60252

Header (AAD, 88 + 16·2 = 120 Byte):
4354583201010200a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3000000010101010101010101010101010101010102020202020202020202020202020202808182838485868788898a8b8c8d8e8f909192939495969700000014

Chiffrat ‖ Tag (20 + 16 = 36 Byte):
d7a80b75f667e99e46605928042ddd91bc0bf34e98a194969c9a4f1b081cb2ed94b66c1d

Envelope (156 Byte):
4354583201010200a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3000000010101010101010101010101010101010102020202020202020202020202020202808182838485868788898a8b8c8d8e8f909192939495969700000014d7a80b75f667e99e46605928042ddd91bc0bf34e98a194969c9a4f1b081cb2ed94b66c1d

SHA-256(Envelope) = 4e9a5861fb493f89520cca8e8ea97c4d1e6a56619d1a7fe475b8f6c43f40917b
```

### B.3 Vektor B: kein Betroffener (`NonPersonal`, n = 0)

```
IKM  = TK_t                                        (32 Byte)
PRK  = d7affa1d75cc2db8804002faeb8c7fe477350400dd0d25b52909068d996cc5f3
info = 6374782f76322f64656b00a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b20000000001
DEK  = 2b194306dde7da704b5a2150f09c860e4cb3740dd3af2937441f481821430c42

Header (AAD, 88 Byte):
4354583201010000a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c300000001808182838485868788898a8b8c8d8e8f909192939495969700000014

Chiffrat ‖ Tag (36 Byte):
ad88e82f7ff7e92f2a2edd72a679a08121650389be13e2c37c88bb4668f7f3c6fcd6d5d3

Envelope (124 Byte):
4354583201010000a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c300000001808182838485868788898a8b8c8d8e8f909192939495969700000014ad88e82f7ff7e92f2a2edd72a679a08121650389be13e2c37c88bb4668f7f3c6fcd6d5d3

SHA-256(Envelope) = 8f16e6cafc487aad2c4782bc980869ad8005015ac1b4a4b8563ea5c00c83f644
```

### B.4 Referenzprogramm für Auditoren (`audit_open.py`)

Eigenständiger Code (Python 3, Paket PyNaCl), **keine** Acta-Bibliothek. 35 nichtleere Zeilen. Er parst den Envelope nach 4.5, bildet den DEK nach 4.4 und öffnet. Ein fehlender Betroffenenschlüssel ergibt `KeyError`, ein falscher Schlüssel oder ein veränderter Byte ergibt `nacl.exceptions.CryptoError`; beides ist „nicht entschlüsselbar“ im Sinne von Z2 und V6 (14.2).

```python
#!/usr/bin/env python3
"""Referenz-Öffner für Acta-Envelope E1 (Spezifikation 0.2, Anhang B). Eigener Code, keine Acta-Bibliothek."""
import hashlib, hmac, struct, sys
import nacl.bindings as sodium, nacl.exceptions

def parse(env):
    if env[:4] != b"CTX2" or env[4] != 1 or env[5] != 1 or env[7] != 0: raise ValueError("Kopf")
    n = env[6]
    if n > 16: raise ValueError("n_subjects")
    tenant, coll, doc = env[8:24], env[24:40], env[40:56]
    gen = struct.unpack(">I", env[56:60])[0]
    subs = [env[60 + 16 * i: 76 + 16 * i] for i in range(n)]
    if subs != sorted(set(subs)): raise ValueError("subject_refs nicht streng aufsteigend")
    o = 60 + 16 * n
    nonce, ct_len = env[o:o + 24], struct.unpack(">I", env[o + 24:o + 28])[0]
    if len(env) != o + 28 + ct_len + 16: raise ValueError("Länge")
    return dict(tenant=tenant, coll=coll, doc=doc, gen=gen, subs=subs, nonce=nonce,
                aad=env[:o + 28], ct=env[o + 28:])

def hkdf_sha256(salt, ikm, info, length=32):
    prk = hmac.new(salt, ikm, hashlib.sha256).digest()
    out, t, i = b"", b"", 1
    while len(out) < length:
        t = hmac.new(prk, t + info + bytes([i]), hashlib.sha256).digest(); out += t; i += 1
    return out[:length]

def open_envelope(env, tk, dk_by_subject):
    """tk: 32 Byte Mandantenschluessel; dk_by_subject: {subject_ref: 32 Byte}. Fehlt ein Schluessel -> KeyError."""
    e = parse(env)
    ikm = tk + b"".join(dk_by_subject[s] for s in e["subs"])
    info = (b"ctx/v2/dek\x00" + e["tenant"] + e["coll"] + bytes([len(e["subs"])])
            + b"".join(e["subs"]) + struct.pack(">I", e["gen"]))
    dek = hkdf_sha256(e["doc"], ikm, info)
    return sodium.crypto_aead_xchacha20poly1305_ietf_decrypt(e["ct"], e["aad"], e["nonce"], dek)

if __name__ == "__main__":
    env, tk, *dks = (bytes.fromhex(a) for a in sys.argv[1:])
    subs = parse(env)["subs"]
    print(open_envelope(env, tk, dict(zip(subs, dks))).hex())
```

Aufruf: `python3 audit_open.py <envelope-hex> <TK-hex> <DK_s1-hex> [<DK_s2-hex> …]` (Schlüssel in der Reihenfolge der aufsteigenden `subject_refs`). Die Datei ist Teil des Repositorys unter `audit/` (15.5) und wird **nicht** aus Acta-Code erzeugt.

**Ergebnisse des Programms auf den Vektoren (ausgeführt 2026-11-06):**

| Versuch | Ergebnis |
| --- | --- |
| Vektor A mit `TK_t`, `DK_s1`, `DK_s2` | öffnet, liefert P1 aus B.1 |
| Vektor B mit `TK_t` | öffnet, liefert P1 aus B.1 |
| Vektor A ohne `DK_s2` | `KeyError`, kein Klartext (Z3) |
| Vektor A mit `DK_s2` = 32 × `00` | `CryptoError`, kein Klartext |
| Vektor A, jedes der 1248 Bits einzeln gekippt | alle 1248 Versuche abgelehnt (`CryptoError` oder Parserfehler), keiner akzeptiert |
| Vektor B, jedes der 992 Bits einzeln gekippt | alle 992 Versuche abgelehnt, keiner akzeptiert |

### B.5 Löschlog-Kette (6.6)

```
store_id   = f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff
genesis    = SHA-256("ctx/v2/log-genesis" ‖ 0x00 ‖ store_id)
           = 9acb038bd7fb9b9c08d7782509af5a0c9db8b7192918e4b3ef4928ea08944a5f

Eintrag 1:  seq = 1, kind = 1 (GATED), erase_id = e1 × 16, target_kind = 1 (Subject),
            tenant_id = a1 × 16, subject_ref = 01 × 16, ts_ms = 1762400000000,
            reason = 0, aux = 32 × 00, prev_hash = genesis
entry_hash = SHA-256("ctx/v2/log" ‖ 0x00 ‖ seq (8) ‖ kind (1) ‖ erase_id (16) ‖ target_kind (1)
                     ‖ tenant_id (16) ‖ subject_ref (16) ‖ ts_ms (8) ‖ reason (1) ‖ aux (32) ‖ prev_hash (32))
           = 6a01149b3ab2c080619b23641d24a5394cde4394d1dd463e550140883b8d4120
```

### B.6 Schlüsselbezeichner (5.4)

```
Mandant:     ctx2/f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff/t/a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1
Betroffener: ctx2/f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff/s/a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1/01010101010101010101010101010101
```

### B.7 Manifestwurzel (9.4 E8)

Die Eingabe je Zeile ist `doc_id ‖ envelope_sha256` (die `collection_id` dient nur der Sortierung, 19.3 Punkt 6). Die Zeilen sind nach `(collection_id, doc_id)` bytewise aufsteigend sortiert.

```
Eine Zeile:  (collection_id = b2 × 16, doc_id = c3 × 16, envelope_sha256 = SHA-256(Vektor B))
  manifest_root = SHA-256("ctx/v2/manifest" ‖ 0x00 ‖ c3×16 ‖ 8f16e6cafc487aad2c4782bc980869ad8005015ac1b4a4b8563ea5c00c83f644)
                = a6b20f3d179b5dc50d4eaa93bf116eda7ae48d1464ae1c7f1f6f15eccebc09c2

Zwei Zeilen: (collection_id = 01 × 16, doc_id = c4 × 16, envelope_sha256 = SHA-256(Vektor B)),
             (collection_id = 02 × 16, doc_id = c3 × 16, envelope_sha256 = SHA-256(Vektor A))
  manifest_root = e1667059eb692c86d72f2c7da2c3454950e7115f29149d83df4d4fe671715f30
```

### B.8 Erzeugungsprogramm

Dieses Programm erzeugt B.1 bis B.7 (73 nichtleere Zeilen; zusätzlich zum Referenzprogramm B.4, das es **nicht** benutzt). Es ist unter `audit/gen_vectors.py` abgelegt. T-SEAL-1 vergleicht die Ausgabe mit den Vektoren der Rust-Tests.

```python
#!/usr/bin/env python3
"""Erzeugt die Testvektoren aus Anhang B (Acta-Spezifikation 0.2).
Zwei unabhängige Wege für das Siegeln: libsodium (PyNaCl) und HChaCha20 (eigener Code) + ChaCha20-Poly1305 (cryptography).
HKDF: eigener Code, gegengeprüft mit `cryptography`. P1: von Hand kodiert, gegengeprüft mit cbor2."""
import hashlib, hmac, json, struct
import cbor2
import nacl.bindings as sodium
from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.primitives.ciphers.aead import ChaCha20Poly1305
from cryptography.hazmat.primitives.kdf.hkdf import HKDFExpand

def hkdf_extract(salt, ikm): return hmac.new(salt, ikm, hashlib.sha256).digest()
def hkdf_expand(prk, info, length):
    out, t, i = b"", b"", 1
    while len(out) < length:
        t = hmac.new(prk, t + info + bytes([i]), hashlib.sha256).digest(); out += t; i += 1
    return out[:length]

def rotl(v, c): return ((v << c) & 0xffffffff) | (v >> (32 - c))
def quarter(s, a, b, c, d):
    s[a] = (s[a] + s[b]) & 0xffffffff; s[d] = rotl(s[d] ^ s[a], 16)
    s[c] = (s[c] + s[d]) & 0xffffffff; s[b] = rotl(s[b] ^ s[c], 12)
    s[a] = (s[a] + s[b]) & 0xffffffff; s[d] = rotl(s[d] ^ s[a], 8)
    s[c] = (s[c] + s[d]) & 0xffffffff; s[b] = rotl(s[b] ^ s[c], 7)
def hchacha20(key, nonce16):
    s = list(struct.unpack("<4I", b"expand 32-byte k")) + list(struct.unpack("<8I", key)) + list(struct.unpack("<4I", nonce16))
    for _ in range(10):
        quarter(s, 0, 4, 8, 12); quarter(s, 1, 5, 9, 13); quarter(s, 2, 6, 10, 14); quarter(s, 3, 7, 11, 15)
        quarter(s, 0, 5, 10, 15); quarter(s, 1, 6, 11, 12); quarter(s, 2, 7, 8, 13); quarter(s, 3, 4, 9, 14)
    return struct.pack("<8I", *(s[0:4] + s[12:16]))
def xchacha_seal_alt(key, nonce24, aad, pt):
    return ChaCha20Poly1305(hchacha20(key, nonce24[:16])).encrypt(b"\0\0\0\0" + nonce24[16:], pt, aad)

# --- feste Testeingaben (nur für Testvektoren, keine Geheimnisse) ---
TK, DK1, DK2 = bytes(range(0x00, 0x20)), bytes(range(0x20, 0x40)), bytes(range(0x40, 0x60))
TENANT, COLL, DOC = bytes([0xA1]) * 16, bytes([0xB2]) * 16, bytes([0xC3]) * 16
S1, S2 = bytes([0x01]) * 16, bytes([0x02]) * 16
NONCE = bytes(range(0x80, 0x98))
GEN = 1

def make_envelope(tk, dks, subs, pt):
    assert subs == sorted(subs) and len(set(subs)) == len(subs)
    prk = hkdf_extract(DOC, tk + b"".join(dks))
    info = (b"ctx/v2/dek\x00" + TENANT + COLL + bytes([len(subs)]) + b"".join(subs) + struct.pack(">I", GEN))
    dek = hkdf_expand(prk, info, 32)
    assert dek == HKDFExpand(hashes.SHA256(), 32, info).derive(prk)          # Gegenprobe HKDF
    hdr = (b"CTX2" + bytes([1, 1, len(subs), 0]) + TENANT + COLL + DOC + struct.pack(">I", GEN)
           + b"".join(subs) + NONCE + struct.pack(">I", len(pt)))
    ct = sodium.crypto_aead_xchacha20poly1305_ietf_encrypt(pt, hdr, NONCE, dek)
    assert ct == xchacha_seal_alt(dek, NONCE, hdr, pt)                        # zwei Wege für das Siegeln
    assert sodium.crypto_aead_xchacha20poly1305_ietf_decrypt(ct, hdr, NONCE, dek) == pt
    return dict(prk=prk.hex(), info=info.hex(), dek=dek.hex(), header=hdr.hex(), ct_tag=ct.hex(),
                envelope=(hdr + ct).hex(), length=len(hdr + ct), sha256=hashlib.sha256(hdr + ct).hexdigest())

text = b"Hallo Welt"
P1 = bytes([0xa2, 0x61, 0x76, 0x01, 0x64]) + b"text" + bytes([0x60 + len(text)]) + text   # {"v":1,"text":"Hallo Welt"}
assert cbor2.loads(P1) == {"v": 1, "text": "Hallo Welt"}
assert cbor2.dumps({"v": 1, "text": "Hallo Welt"}, canonical=True) == P1                 # kanonische Form

out = {"P1": P1.hex()}
out["A"] = make_envelope(TK, [DK1, DK2], [S1, S2], P1)
out["B"] = make_envelope(TK, [], [], P1)

store_id = bytes(range(0xF0, 0x100))
genesis = hashlib.sha256(b"ctx/v2/log-genesis\x00" + store_id).digest()
erase_id = bytes([0xE1]) * 16
entry1 = hashlib.sha256(b"ctx/v2/log\x00" + struct.pack(">Q", 1) + bytes([1]) + erase_id + bytes([1]) + TENANT + S1
                        + struct.pack(">Q", 1762400000000) + bytes([0]) + b"\0" * 32 + genesis).digest()
out["log"] = dict(store_id=store_id.hex(), erase_id=erase_id.hex(), ts_ms=1762400000000, genesis=genesis.hex(), entry1=entry1.hex())
out["keyid_t"] = f"ctx2/{store_id.hex()}/t/{TENANT.hex()}"
out["keyid_s"] = f"ctx2/{store_id.hex()}/s/{TENANT.hex()}/{S1.hex()}"

sha_a, sha_b = bytes.fromhex(out["A"]["sha256"]), bytes.fromhex(out["B"]["sha256"])
def root(rows):   # rows: (collection_id, doc_id, envelope_sha256); sortiert nach (collection_id, doc_id); Eingabe doc_id ‖ sha256
    body = b"".join(d + h for c, d, h in sorted(rows, key=lambda r: (r[0], r[1])))
    return hashlib.sha256(b"ctx/v2/manifest\x00" + body).hexdigest()
out["manifest_one"] = root([(COLL, DOC, sha_b)])
rows2 = [(bytes([0x02]) * 16, DOC, sha_a), (bytes([0x01]) * 16, bytes([0xC4]) * 16, sha_b)]
out["manifest_two"] = root(rows2)
out["hchacha20_rfc"] = hchacha20(bytes(range(32)), bytes.fromhex("000000090000004a0000000031415927")).hex()
print(json.dumps(out, indent=1))
```

---

## Anhang C – Zulässiger Wortlaut für Außenaussagen

Jede Außenaussage MUSS die Attestierungsklasse (14.4) nennen und DARF NICHT stärker sein als sie (1.4). Wortlaut, der hier nicht steht, ist nicht freigegeben; eine Erweiterung verlangt G5 (17.1).

### C.1 Standardaussage (`WORDING_STANDARD`)

> *„Acta vernichtet den Schlüssel des Betroffenen beim Key-Dienst und macht dessen gespeicherte Daten dadurch unlesbar. Ob das im Einzelfall eine Löschung nach Art. 17 DSGVO darstellt, beurteilt der Verantwortliche.“*

### C.2 Zusätze je Klasse

| Klasse | Pflichtzusatz |
| --- | --- |
| **A0** | „Der Schlüsseleintrag wurde lokal überschrieben und entfernt (Funktionstest, Klasse A0). Das ist keine Löschbestätigung.“ Die Standardaussage C.1 darf bei A0 **nicht** verwendet werden; es gilt nur der A0-Satz. |
| **A1** | C.1 plus: „Der Key-Dienst hat die Vernichtung quittiert (Klasse A1).“ Bei Karenz: „Die endgültige Löschung ist für den `<Zeitpunkt aus scheduled_final_ms>` geplant; bis dahin ist der Schlüssel gesperrt, nicht gelöscht.“ |
| **A2** | C.1 plus: „Der Beleg ist vom Dienst signiert und im Audit-Log des Dienstes nachvollziehbar (Klasse A2).“ Bei Karenz wie A1. |

### C.3 Pflichthinweis auf Grenzen

Jede Außenaussage, die über einen einzelnen Satz hinausgeht (Webseite, Datenblatt, Vertrag, Auditbericht), MUSS `LIMITS_TEXT` (A.6) vollständig enthalten oder auf ihn verweisen.

### C.4 Nicht zulässig

Siehe 18.3. Insbesondere in keiner Klasse: „DSGVO-konform“, „garantiert gelöscht“, „unwiederbringlich gelöscht“, „zertifiziert“, „fehlerfrei“, „bewiesen sicher“, „Löschung aus allen Sicherungen“.

### C.5 Beispiele

| Kontext | Zulässig | Nicht zulässig |
| --- | --- | --- |
| Produktseite (A1) | „Acta vernichtet den Schlüssel des Betroffenen beim Key-Dienst und macht dessen gespeicherte Daten dadurch unlesbar. … Der Key-Dienst hat die Vernichtung quittiert (Klasse A1).“ | „Löschung nach DSGVO in Sekunden.“ |
| Entwicklung (A0) | „Der Schlüsseleintrag wurde lokal überschrieben und entfernt (Funktionstest, Klasse A0). Das ist keine Löschbestätigung.“ | „Daten sicher gelöscht.“ |
| Auditbericht | Tatsachen aus dem Nachweispaket und dem Prüfverfahren 14.2 mit Klasse und `LIMITS_TEXT` | „Löschung bestätigt“, „konform“ |

---

*Ende der Spezifikation Fassung 0.2 (Entwurf mit Zielwerten).*
