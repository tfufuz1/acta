## 17. Unabhängiger Prüfer (`acta-verify`)

### 17.1 Zweck und Abgrenzung [E]

`acta-verify` ist ein eigenständiges Programm **ohne interne Abhängigkeit** (3.3). Es nutzt nur `sha2`, `ed25519-dalek`, `serde_json` und eine RFC-8785-Bibliothek und kann deshalb von Revision, Datenschutzfunktion oder Dritten gebaut und geprüft werden. Es prüft **Struktur und Echtheit** von Nachweisen, nicht die Wahrheit einer Messung (die ist Gegenstand von 16.4).

### 17.2 Prüfstufen [E]

|Stufe|Eingaben|Prüfung|
|---|---|---|
|**V1**|Löschbericht, Ledger-Ausschnitt bis zum nächsten Checkpoint, öffentlicher Notar-Schlüssel (`key_id`, `sig_alg`)|(1) JCS-Kanonisierung und Hash des Berichts; (2) Signatur mit Domänenpräfix `acta/report/v1`; (3) Berichtshash steht als Eintrag im E-Ledger; (4) Kette ab Vorgängerhash nachrechnen; (5) Checkpoint-Signatur (`acta/checkpoint/v1`); (6) Konsistenz: jede Klasse mit `reached` trägt Belegfelder, `NotImplemented` und `not_measured` sind nie `reached`, Positivkontrolle `passed` bei jeder vollständigen Messung, Klasse K nur mit `backup_exposure_until` in der Vergangenheit, Klasse M nur mit `registry_mirrored = true`; (7) Wortfilter (1.6) über Textfelder; (8) Annahmenliste vollständig|
|**V2**|zusätzlich Tresorabbild und Generationsschlüssel unter Aufsicht des Betreibers|Grabstein-Struktur am im Bericht genannten Slot, Direct-Read-Nachweis, Generation gegen `DestroyProof`|
|**V3**|zusätzlich `CommitKey(e)` (solange vorhanden)|Nachrechnen eines Commitments für eine dem Betreiber bekannte Kennung|

Ausgabe: `verified` oder `failed` mit stabilen Codes; nie Inhalte oder Kennungen. Ein Objekt mit `Redacted`-Vermerk ist undurchsichtig; geprüft wird nur die Kette.

### 17.3 Vertrauensgrenzen [E]

Der Notar-Schlüssel liegt im Anker; ein kompromittierter Anker kann Nachweise fälschen (A-ANCHOR). Gegenmittel: Anker-Köpfe der Ketten, Registersenke in getrenntem Ausfallbereich, Re-Witness (4.7), mehrere unabhängige Prüfer. Der Prüfer ersetzt keine Prüfung der Messverfahren.

---

