## Anhang E – Fundstellenregister

Prüfstatus aus den Recherchen mit Stand 27.10.2026: ✔ am Primärtext oder Volltext geprüft, ◐ nur über Sekundärquelle oder Leitsatz belegt, ○ allgemeiner Rechtskenntnisstand, nicht verifiziert. **Jede Verwendung außerhalb dieser Spezifikation erfordert Verifikation am Primärtext (20.7).** Prüfdatum und Prüfer trägt der Betreiber ein. Dieser Anhang ist durch `g-word:ignore` vom Wortfilter ausgenommen, soweit er verbotene Formulierungen als Fehlzuordnungen zitiert.

### E.1 Normen und Behördenpapiere

|Fundstelle|Verwendung|Status|
|---|---|---|
|DSGVO Art. 4 Nr. 2, 3, 5, 7, 8; Art. 5, 11, 12, 15 bis 22, 24, 25, 28, 30, 32 bis 35, 44 ff.; Erwägungsgrund 26|Normtext|✔/○|
|EDPB-Leitlinien 02/2025 (Blockchain; v1 04/2025, v2 07.07.2026), Rn. 50, 51 bis 53, 63, 85, 87, 96, 103, 104|Bedingungen bei Schlüsselvernichtung, Notfallplan, Neubewertung (Blockchain-Kontext)|✔|
|EDPB-Leitlinien 01/2025 (Pseudonymisierung)|Pseudonymisierte Daten bleiben personenbezogen; Endfassung nicht verifiziert|◐|
|EDPB-Stellungnahme 28/2024 (17.12.2024)|KI-Modelle, Anonymität|✔|
|EDPB-CEF-Bericht Recht auf Löschung (10.02.2026), Abschn. 4.2.6, 4.2.7|Pseudonymisierung genügt nicht als Löschung; Backups; Verifikation|✔/◐|
|DSK-Orientierungshilfe RAG (V1.0, 10/2025)|On-Premise-Annahme, Embeddings, Adressierbarkeit|✔|
|DSK-Orientierungshilfe KI und Datenschutz (V1.0, 05/2024)|Betroffenenrechte bei KI|◐|
|DSK-Entschließung 12.12.2025|Herstellerverantwortung *de lege ferenda*|✔|
|HmbBfDI-Diskussionspapier LLM (15.07.2024)|Gegenposition für modellinterne Embeddings|✔|
|SDM Baustein 60 (V1.0a, seit 02.09.2020; M60.P01 bis P14, M60.S04 bis S10, M60.D01, D02)|Löschkonzept, Protokolle, Rücksicherung, Schlüssel, Kopien|✔; Neufassung in SDM V3.x prüfen|
|DIN 66398; DIN 66399; ISO/IEC 27555; ISO/IEC 27040; ISO/IEC 27001:2022 A.8.10, A.8.24; BSI IT-Grundschutz CON.6; BSI TR-02102|Löschkonzept, Vernichtung, Kryptoempfehlungen (TR-02102 regelt Verfahren, nicht Löschen)|○|
|NIST SP 800-88 Rev. 2 (26.09.2025)|Cryptographic Erase als Purge unter Voraussetzungen|◐|
|RFC 2119, 5869, 8032, 8785, 9106|Schlüsselwörter, HKDF, Ed25519, JCS, Argon2|Primärtext prüfen|
|§ 35 BDSG|nur nicht automatisierte Verarbeitung|✔|
|§ 147 AO, § 257 HGB, § 630f BGB, BEG IV (BGBl. 2024 I Nr. 323)|Aufbewahrungsfristen|◐|
|§ 5 UWG, § 434 BGB, § 203 StGB|Irreführung, Sachmangel; Schweigepflicht (keine Aufbewahrungsnorm)|○|
|KI-VO (VO (EU) 2024/1689) Art. 3, 4, 10, 12, 14, 19, 26, 27, 50; Digital Omnibus VO (EU) 2026/1744|KI-Einordnung, Protokolle, Aufsicht, Fristen|○/◐|
|NIS2 (RL (EU) 2022/2555) Art. 21; CRA (VO (EU) 2024/2847)|Lieferkette, Kryptografie, Herstellerpflichten|○|
|Bundes- und Landesarchivrecht|Anbietungspflicht (8.4)|nicht recherchiert [V]|
|Herstellerangaben Cloud-KMS (Wartefenster)|`anchor_destroy_max`|nur Indiz, Herstellerdokumentation prüfen|

### E.2 Rechtsprechung

|Entscheidung|Aussage und Relevanz|Status|
|---|---|---|
|EuGH C-582/14 Breyer (19.10.2016); C-413/23 P EDPS/SRB (04.09.2025)|relativer Personenbezug|○|
|EuGH C-131/12 Google Spain (13.05.2014)|Auslistung; nicht zu technischen Löschverfahren|○|
|EuGH C-129/21 Proximus (27.10.2022)|Art. 17 Abs. 2: andere Verantwortliche informieren|○|
|EuGH C-340/21 (14.12.2023)|Nachweis der Angemessenheit von Sicherheitsmaßnahmen|○|
|EuGH C-300/21 (04.05.2023)|Art. 82: keine Erheblichkeitsschwelle|○|
|EuGH C-40/17 Fashion ID (29.07.2019)|gemeinsame Verantwortlichkeit|○|
|EuGH C-460/20 (08.12.2022)|Auslistung bei offensichtlicher Unrichtigkeit|○|
|BGH VI ZR 10/24 (18.11.2024)|Art. 82, Kontrollverlust (Scraping), nicht Löschanspruch|○|
|BGH I ZR 98/23 (27.06.2024)|Aufklärung in der Werbung (Analogie)|○|
|OLG Dresden, 03.02.2026, 4 U 292/25|Anonymisierung als „Minus zur Löschung“ (Leitsatz)|◐|
|LG München II, 14.03.2025, 14 O 1461/24|Anonymisierung als Unterfall der Löschung (Leitsatz)|◐|
|OLG Dresden, 14.12.2021, 4 U 1278/21|Inkasso, Aufbewahrungspflichten je Datum, Zugriffsunterbindung, Schwärzung|◐|
|OLG München, 22.03.2022, 18 U 1697/21 Pre|Löschbegriff autonom; Nicht-Wiederverarbeitung|◐|
|AG Lörrach, 03.03.2025, 3 C 1099/24|kein Anspruch auf individuellen Löschnachweis (erstinstanzlich)|◐|
|OLG Dresden, 4 U 940/24 (Datum offen)|Nachweis der Löschung durch Auftragsverarbeiter|◐|
|OLG Nürnberg, 11.06.2025, 3 U 383/25|Darlegungs- und Beweislast (nur Titel)|◐|

Es ist keine Entscheidung bekannt, die Crypto-Shredding billigt oder ein Löschverfahren wegen technischer Unzulänglichkeit beanstandet (Nicht-Fund, kein Beleg der Nichtexistenz).

### E.3 Häufige Fehlzuordnungen (nicht übernehmen)

|Verbreitete Aussage|Richtig|
|---|---|
|OLG Dresden 4 U 1278/21 begründe „Anonymisierung ist Minus zur Löschung“|Die Formel stammt aus 4 U 292/25; 4 U 1278/21 betrifft Inkasso und Aufbewahrungspflichten. Keine der Entscheidungen äußert sich zu Crypto-Shredding|
|Gerichte sähen Chiffretext als hinreichend anonymisiert|nicht belegt|
|EDPB 01/2025 (Pseudonymisierung) enthalte Aussagen zu Quantencomputing bei Schlüsselvernichtung|diese stehen in 02/2025 (Blockchain)|
|ICO und CNIL erkennen Crypto-Shredding als legitim an|nicht belegt; ICO: „beyond use“ für Backups|
|BSI TR-02102 empfehle kryptografisches Löschen; ISO/IEC 27040 akzeptiere es ausdrücklich|TR-02102 regelt Algorithmen und Längen; Akzeptanz nicht belegt|
|NIST SP 800-88 ohne Revisionsangabe|maßgeblich Rev. 2|
|Landesbehörden tolerierten 30/60/90 Tage Backup-Rotation; „§ 35 BDSG analog“|keine Primärquelle; § 35 BDSG nur nicht automatisiert|
|EDPB 28/2024 stelle Embeddings gesichert als personenbezogen dar|betrifft Modelle; für RAG-Datenbanken offen; HmbBfDI vertritt für modellinterne Embeddings die Gegenposition|
|AG Lörrach: Löschkonzept plus eidesstattliche Versicherung genüge|nicht in der geprüften Berichterstattung enthalten|
|SDM-Baustein 60 „V2.0“ mit fünf Mindestinhalten|Baustein 60 ist V1.0a; die Fünferliste ist Praxis|
|EuGH „C-649/17 (Fashion ID)“|Fashion ID ist C-40/17|
|HGB 6 Jahre, AO 10 Jahre; § 203 StGB als Aufbewahrungsgrundlage|seit 2025 Buchungsbelege 8 Jahre; § 203 StGB ist Schweigepflicht|
|AI Act Art. 12 verlange Klartextprotokollierung aller Ein- und Ausgaben|nur Hochrisiko; Ereignisprotokolle; Art. 26 Abs. 6 verweist auf Datenschutzrecht|
|NIS2 verlange Krypto-Agilität|Begriff nicht im Text|
|DSK-OH RAG nenne Crypto-Shredding oder verlange physische Entfernung|Crypto-Shredding nicht erwähnt|

---

