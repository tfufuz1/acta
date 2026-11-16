## 0. Lesehinweise

### 0.1 Normative Schlüsselwörter

**MUSS**, **DARF NICHT**, **SOLL**, **KANN** haben die Bedeutung nach RFC 2119.

### 0.2 Kennzeichnung der Aussagen

|Marke|Bedeutung|
|---|---|
|**[E]**|Verbindliche Architekturentscheidung dieser Spezifikation|
|**[R]**|Rechtliche Anforderung oder Einordnung, aus der eine technische Eigenschaft folgt|
|**[Q]**|Messbares Qualitätsziel. Die Zahl ist ein Startwert; freigegeben wird nur nach gemessenem Bericht (16.5)|
|**[A]**|Annahme über die Umgebung; im Betrieb zu bestätigen und im Bericht auszuweisen (20.3)|
|**[⚠]**|Rechtlich offen oder umstritten. Die Spezifikation trifft eine vorsichtige technische Wahl und behauptet keine Rechtsfolge|
|**[V]**|Fundstelle oder Rechtstatsache ist vor jeder Außenverwendung am Primärtext zu verifizieren (20.7)|

### 0.3 Konventionen

- Alle Plattenformate und Protokollrahmen: Big-Endian, Längenfelder geprüft, Zeiten in UTC. `tx_time` in Millisekunden seit Unix-Epoche (u64), Kalendertage als `i32` Tage seit 1970-01-01. Größenangaben in KiB/MiB/GiB (Zweierpotenzen).
- Rust-Signaturen in dieser Spezifikation sind **normativ in Form und Semantik**, nicht in Nebensächlichkeiten wie Lebensdauer-Annotationen.
- `<…>` in Dateinamen und Domänenpräfixen sind Platzhalter. Alle Domänenpräfixe haben die Form `acta/<zweck>/v<N>` (Tabelle in 4.1).
- „Mandant“ ist die Isolationseinheit (Amt, Gesellschaft). „Installation“ ist ein Knoten mit einem oder mehreren Mandanten.

### 0.4 Vollständigkeits- und Vorrangregel [E]

1. **Default-deny:** Was diese Spezifikation nicht erlaubt, ist verboten. Es gibt keine stillen Standardwerte für Pflichtparameter (Anhang A) und keine Fallbacks im Sicherheitspfad (Z10).
2. **Vorrang bei scheinbarem Widerspruch:** Zusagen (1.5) vor Invarianten (15.4) vor Verfahren (Kap. 4 bis 14) vor Parametern (Anhang A). Wo ein Verfahren eine Zusage nicht erfüllen kann, ist das Verfahren falsch und die Zusage maßgeblich; der Fehler ist als Defekt zu behandeln, nicht zu „umgehen“.
3. **Jeder Lebenszyklus ist vollständig:** Kapitel 19 listet für jede Funktion Auslöser, Vorbedingungen, Schritte, Nachbedingungen, Fehlerpfade, Absturzverhalten und Nachweis. Eine Funktion ohne Eintrag dort gilt als nicht spezifiziert und DARF NICHT implementiert werden.

### 0.5 Aufbau

1 Produkt · 2 Grundmodell · 3 Architektur · 4 Kryptografie · 5 Speicher · 6 Wissensebene · 7 Entscheidungsprotokoll · 8 Löschkern · 9 Ingest, Abruf, Antwort · 10 Fachfunktionen · 11 Inferenz · 12 Mandanten, Rollen, Aufbewahrung · 13 Sicherheit · 14 Betrieb · 15 Qualität und Konfiguration · 16 Test und Freigabe · 17 Unabhängiger Prüfer · 18 Berichte und Schnittstellen · 19 Lebenszyklen · 20 Rechtliche Einordnung · Anhänge A bis F.

---

