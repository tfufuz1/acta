## 10. Fachfunktionen für Sachbearbeitung und Verwaltung

Alle Fachfunktionen sind Anwendungsfälle in `acta-engine` über die API (18.5). Das Modell ruft **keine** Werkzeuge auf; deterministische Werkzeuge werden von der Anwendung aufgerufen und als `Tool`-Entscheidung protokolliert (7.1). Jede Funktion unterliegt Zugriffskontrolle (9.4), Gate (8.1) und Belegpflicht, soweit sie Modellausgaben erzeugt. **Entscheidungshilfe, keine Entscheidung (Art. 22):** Ergebnisse tragen den Hinweis und sind durch einen Menschen freizugeben (`Review`).

|ID|Funktion|Eingaben|Verarbeitung|Ausgabe und Persistenz|Löschverhalten|
|---|---|---|---|---|---|
|FF1|**Belegte Antwort**|Frage, Sammlungen, optional `as_of`/`valid_at`|Pipeline 9.4 bis 9.6 nur über K|Antwort mit Belegen; `Query`-Entscheidung (Eingabe `Sealed`, falls Personenbezug)|Nutzlast nach `payload_retention`|
|FF2|**Vorgangsassistent**|`case_id`, Frage oder Auftrag (Zusammenfassung, Chronologie, offene Punkte)|Pipeline mit K **und** P im Vorgangskontext; Belege `P` werden nie gecacht (9.5)|Antwort/Zusammenfassung mit Belegen aus Akte und Vorschriften; `Query`, `Sealed` an die Betroffenen der Belege und des Vorgangs|Löschung einer Person macht Anteile unlesbar (Z3); Begründungskette bleibt|
|FF3|**Prüfliste**|Verfahrensart oder Rechtsgrundlage, Stand|Zerlegung der einschlägigen Vorschriften in Prüfschritte mit Beleg je Schritt; Vorlagen aus `checklists/`|Liste mit Fundstellen; `Query`; speicherbar als Entwurf (`entwurf/<id>`)|wie FF1|
|FF4|**Entwurfsassistent**|Vorlage aus `templates/`, `case_id`, Variablen|Der Entwurf entsteht **in P** (Klasse `Personal`): Platzhalter für personenbezogene Felder werden aus P gefüllt, nie in K geschrieben; Modell schlägt Textbausteine und Begründung mit Belegen vor|Entwurf als P-Dokument-Version (`Draft`), Export als Datei; `Release` bei Freigabe durch Menschen (`Review`)|wie FF2; Entwurfsversionen sind Versionen im Sinne 4.2|
|FF5|**Fristen- und Datumsrechner**|Ereignisdatum, Fristart (Tage, Wochen, Monate), Bundesland|Deterministisch (kein Modell): Fristbeginn, Fristende, Verlängerung bei Sonn-/Feiertag/Samstag nach den Regeln des BGB (§§ 187 bis 193) [V], Feiertagskalender je Bundesland aus `calendar/` (versioniert, Gültigkeitszeit); Ergebnis mit Herleitung|`Tool`-Entscheidung; Ergebnis als Beleg (Regeltext, Kalenderstand)|Eingabedaten `Sealed`|
|FF6|**Änderungsradar**|Sammlung, Zeitraum oder zwei Stände|`diff` der Normen-Blobs; Zuordnung betroffener Chunks; frühere `Query`/`Release`-Entscheidungen mit `EvidenceRef::Knowledge` auf geänderte Blobs; Neulauf des Goldsets; **kein Zugriff auf P**|Bericht: geänderte Normen, betroffene frühere Entscheidungen (nur Kennung, Datum, Art), Antwortunterschiede; `Beleg`|keine Personendaten, daher keine Löschwirkung|
|FF7|**Vergleich und Szenario**|zwei Stände oder Szenario-Definition|Kapitel 7.3|Vergleichsbericht; `ScenarioRun`|Ergebnisse mit Personenbezug `Sealed`|
|FF8|**Begründungsbericht**|`decision_id`|`explain` (7.2) als lesbarer, signierter Bericht (Wissensstand, Belege, Modell, Prüfer)|Export (Kopienregister); keine neuen Daten|Erased-Anteile als „nicht mehr verfügbar“|
|FF9|**Wissenspflege-Workflow**|Änderungen an Wissensdokumenten|Entwurf (`entwurf/<id>`), Validator, Regression (9.7), Vier-Augen-`merge` nach `main`; Veröffentlichung löst ggf. Indexaufbau aus|`Release`; K-Ledger|Takedown nach 6.7|

**Qualitätsregel:** Jede Fachfunktion mit Modellausgabe hat ein Goldset im Mandanten (9.7); ohne Goldset ist die Funktion für den Mandanten nicht freigeschaltet (`feature_enabled`).

---

