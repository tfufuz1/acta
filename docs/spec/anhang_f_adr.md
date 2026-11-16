## Anhang F – Entscheidungsverzeichnis (ADR)

|ID|Entscheidung|Begründung|Preis|Revisionstrigger|
|---|---|---|---|---|
|ADR-01|Vier Ebenen K, P, D, E; `PersonalRef` als Zufallshandle|Versionierung und Löschung entkoppeln (Z5, Z6)|P-Dokumente nur über Vorgang abrufbar|–|
|ADR-02|Standardmodus `Whole`; `Split` nur nach bestandenem Anonymitätstest|Erkennung unvollständig; K ohne Personenbezug|weniger Wissen in K|Rechtsauffassung der Aufsicht|
|ADR-03|Zeitreise nur über K; kurze `Superseded`-Kette in P|Nachvollziehbarkeit ohne Personenhistorie|keine historische Rekonstruktion von P|–|
|ADR-04|Sichtbarkeitsmengen je Commit statt Index je Stand|Branches ohne Indexkopien|Filterkosten|Messung|
|ADR-05|XChaCha20-Poly1305, SHA-2, Ed25519; Algorithmus-Byte|kleiner Primitivsatz, kleiner Prüfer|kein FIPS-Algorithmus|Anforderung|
|ADR-06|DEK = HKDF über Dokument- und Betroffenenschlüssel|Mehrbetroffenen-Löschung ohne Wrapping|92 Byte je Schlüssel|–|
|ADR-07|Wurzelrotation mit Generationen|ein Mechanismus, alte Tresorabbilder unlesbar|Ankerfrist|Anker-Messung|
|ADR-08|Append-only Segmentspeicher|Lastprofil, gezielte Löschung|Eigenbau 6 500 LOC|Messbericht Fremdengine|
|ADR-09|DP versiegelt, kein Klartext-`mmap`|Z2|Speicherbedarf|Ladezeit|
|ADR-10|Kern synchron|deterministische Fehlerinjektion|Thread-Pool-Brücke|`erase_return_timeout` unter Last|
|ADR-11|`unsafe` nur in `acta-sys`|kleine Prüffläche|Spitzenleistung|Messung|
|ADR-12|Exakte Flat-Vektorsuche, ANN nur nach Messung|Z7, Löschung per Segmentersatz|Rechenzeit|Latenzziel verfehlt|
|ADR-13|Level-Modell mit `rings.toml`|prüfbare Schichtung|Konfigurationsdatei|–|
|ADR-14|Keine Sammel-Crates|keine Hubs|`From`-Code|–|
|ADR-15|Deterministische Tags aus `k_idx`; Zufall für Handles|Auffinden ohne Zuordnungstabelle|`k_idx`-Besitz erlaubt Existenztest|Rotationsverfahren|
|ADR-16|Handkodierte Plattenformate|Stabilität|mehr Code|–|
|ADR-17|Egress gesperrt, Freigabe vier-Augen|Souveränität|Einrichtungsaufwand|–|
|ADR-18|Nutzer sind Betroffene; `ActorRef`|Rechte für Mitarbeitende|Auflösung über P|–|
|ADR-19|Belegpflicht mit Enthaltung|Präzision|mehr Enthaltungen|Messbericht|
|ADR-20|Schlüssel zuerst im Tresor g, Rotation übernimmt den Schwanz (R-KEY-FIRST)|kein Schlüsselverlust bei Absturz|Rotationsdauer|Messung|
|ADR-21|Klasse K schließt das Sicherungsfenster ein|Exporte enthalten lebende Schlüssel bis Epochenende|spätere `reached`|Aufsichtsaussage zu Fristen|
|ADR-22|Suchstatistik über `V ∩ A` je Sichtklasse|Z16 ohne Seitenkanal|Cache je Sichtklasse|Kosten über Budget|
|ADR-23|Festkomma-BM25, `i8`-Vektoren|Z7 über Plattformen|Qualitätsverlust gemessen|Recall@10 < 0,99|
|ADR-24|`VerRef` und DEK je Version|Einzelvernichtung alter Versionen|Register|–|
|ADR-25|Zwischensperre `Restricted(MultiSubjectReview)` statt Sperre nach Review|Z1 und Z3 vereinbar|vorsorgliche Sperre [⚠]|Rechtsauffassung|
|ADR-26|Tresor-AAD mit `slot`; Index nur im RAM|keine Klartext-Kennung auf Platte|Öffnungszeit|10 Mio. Einträge über 30 s|
|ADR-27|Zielschlüssel nach Bereinigung (Standard), vorab bei `Besonders` oder SLA ≥ 70 %|Nachholbarkeit gegen Dringlichkeit|Fenster bis S5|Fehlbedienungsrate|
|ADR-28|`tx_time` klemmen statt abweisen|keine Lähmung bei kleiner Uhrkorrektur|Flag `clamped`|–|
|ADR-29|`RegistrySink` als Port; `DirSink` im Kern, Netz am Rand|Kern ohne Netz|zweiter Crate|–|
|ADR-30|Betroffene der Nutzlast aus Belegen, Vorgang, Eingabe; harte `payload_retention`|Löschung erreicht Eingaben|Auflösung nötig|Quote nicht auflösbarer Eingaben|
|ADR-31|Journal-Schlüssel für Journal-Nutzlasten K und D als `SegmentKey` des Journals; Chain-Schlüssel aus `k_idx`|stabil über Tresorrotation|zweiter Schlüsselpfad|–|
|ADR-32|`ClusterId` aus `SubjectKey` abgeleitet, nicht gespeichert|kein dauerhaftes Pseudonym im Abbild|Verzeichnis nötig|–|
|ADR-33|Segmentierung nach Partition (K, P, DK, DP) und Lineage je Quellart|Löschlogik nur für DP|vier Segmentarten|–|
|ADR-34|Graphindex `acta-graph` (L3, CSR, Festkomma-PPR Q32.32, Ableitung D) als RAG-Baustein|Erweiterung der Abrufpipeline um Entitäts- und Pfadtraversierung; RRF-Fusion|31. Produktions-Crate (+4 000 LOC), zusätzliche Ableitungsfläche|Messbericht zeigt keinen Qualitätsgewinn (Recall/Belegquote) oder Z7/Z16 nicht nachweisbar|

**Entscheidungen, die nicht in dieser Spezifikation liegen:** Archiv- und Aufbewahrungsrecht des Mandanten, Wahl der Aufbewahrungsklassen und Fristen, Einstufung nach KI-VO, Zulässigkeit und Dauer pseudonymer Commitments im konkreten Fall, Haltung der zuständigen Aufsicht zur Schlüsselvernichtung, Freigabe externer Dienste. Sie sind Pflichtparameter oder Freigabepflichten (20.4), nicht offene Fragen der Architektur.
