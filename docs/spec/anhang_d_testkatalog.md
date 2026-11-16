## Anhang D – Testkatalog

|Familie|Gegenstand|Zusage|
|---|---|---|
|`T-GATE-*`|alle Lesepfade nach Sperre; `Superseded`; `Restricted`; Cache; Export; Sitzung; Streaming|Z1|
|`T-CONC-*`|`loom`-Modelle: Sperrreihenfolge, Gate, Rotation gegen Schlüsseloperationen|Z1, Z4|
|`T-CRY-*`, `T-SEAL-*`|Umschläge, AAD-Bindung, Auffüllung, Bytes-Grep|Z2|
|`T-DERIVED-SEALED`|Terme und Embeddings nicht im Klartext auf Platte|Z2|
|`T-META-*`|keine Klartext-Tags, keine exakten Längen|Z2|
|`T-MULTI-*`|Trockenlauf, Modi, Sammelakte, `AddSubject`|Z3|
|`T-CRASH-*`|jeder Punkt C01 bis C27, `T-CRASH-VAULT-*`, `T-CRASH-TAKEDOWN`|Z4|
|`T-SEP-*`, `T-ANON-*`|K ohne Personenbezug, Anonymitätstest, `Validated`|Z5|
|`T-VCS-ERASE-*`|Hashes und K-Ledger unverändert|Z6|
|`T-TT-*`, `T-TT-FX`, `T-TT-REDACTED`|Determinismus, Festkomma, Platzhalter|Z7|
|`T-DEC-*`|Begründungskette, `payload_retention`|Z8|
|`T-SCEN-*`|Szenario-Isolation|Z9|
|`T-NOFALLBACK-*`, `T-RNG-*`|Fail-Closed|Z10|
|`T-EVID-*`|Positivkontrolle, `NotImplemented`|Z11|
|`T-REST-*`|Restore-Schutz, Horizont, Senke, Ankerstand|Z12|
|Compile-Fail|`derived_requires_lineage`, `validated_only_from_pii`, `secret_no_clone`|Z13, Z5, Z10|
|`no_shared_dek`|Eigenschaftstest|Z14|
|`T-LOG-*`|Kennungen in Logs, Ledgern, Fehlern, Metriken|Z15|
|`T-ACL-*`, `T-ACL-RANK`, `T-ACL-CASE`|Zugriff vor Suche, Rangverschiebung, Vorgangsbindung|Z16|
|`T-GROUND-*`|Belegstruktur; Goldset-Messung|Z17|
|`T-EGRESS-*`|kein Egress im Standard, Grant|Z18|
|`T-VENDOR-*`, `T-VENDOR-BUNDLE`|kein Herstellerzugriff, Supportpaket ohne Inhalte|Z19|
|`T-TENANT-*`, `T-OFFBOARD-*`|Mandantentrennung, Off-Boarding|–|
|`T-VAULT-*`, `T-ROT-*`, `T-BACKUP-*`|Tresor, Rotation, Export|–|
|`T-SINK-*`|Senke, `st_dev`, Rückstand|–|
|`T-HOST-*`|Hostprüfungen, Dateisystem|–|
|`T-FF-*`, `T-GEN-*`, `T-RET-*`, `T-HOLD-*`, `T-CASE-*`, `T-ACTOR-*`, `T-ALIAS-*`, `T-POISON-*`|Fachfunktionen, Indexgenerationen, Aufbewahrung, Holds, Vorgänge, Akteure, Aliase, Vergiftung|–|

---

