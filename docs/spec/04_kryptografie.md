## 4. Kryptografie und Schlüsselverwahrung

### 4.1 Primitive und Domänenpräfixe [E]

|Zweck|Verfahren|Bibliothek|
|---|---|---|
|Datensatz-Verschlüsselung|XChaCha20-Poly1305, 256-Bit-Schlüssel, 192-Bit-Zufallsnonce aus `CryptoRng` (kein API-Parameter)|`chacha20poly1305`|
|Schlüsselableitung|HKDF-SHA256 (RFC 5869); Fehler werden propagiert|`hkdf`, `sha2`|
|Passwortbasierte Ableitung (nur `FileAnchor`)|Argon2id nach RFC 9106, **m ≥ 65 536 KiB, t ≥ 3, p ≥ 4**; Parameter und Salt je Generation im Kopf der Wurzeldatei|`argon2`|
|Integrität, Commitments, Tags|HMAC-SHA256|`hmac`|
|Objekt- und Kettenhashes|SHA-256|`sha2`|
|Signatur (Berichte, Checkpoints)|Ed25519 (RFC 8032) mit `sig_alg`; Domänenpräfix `acta/report/v1` bzw. `acta/checkpoint/v1`|`ed25519-dalek`|
|Speicherhygiene|`Zeroizing`, `SecureBuf`, konstante Zeit beim Vergleich|`zeroize`, `subtle`|

**Begründung:** Ein einziger Primitivsatz (SHA-2-Familie, ChaCha, Ed25519) hält den unabhängigen Prüfer klein und das Krypto-Audit überschaubar. Die 192-Bit-Zufallsnonce macht zufällige Wahl unproblematisch. Die Argon2id-Parameter folgen der zweiten Empfehlung des RFC 9106, weil ein Angreifer mit gestohlenem Tresorabbild offline rechnen kann [V]. **Preis:** XChaCha20-Poly1305 ist kein FIPS-140-Algorithmus; das Umschlagformat trägt ein Algorithmus-Byte (`alg`), `0x02` ist für AES-256-GCM reserviert (4.7). Eigene Primitive sind verboten.

**Domänenpräfixe (vollständig):** `acta/tag/v1`, `acta/tag-key/v1`, `acta/vault/v1`, `acta/vaultbk/v1`, `acta/hdr/v1`, `acta/dek/v1`, `acta/cluster/v1`, `acta/dir/v1`, `acta/wal/v1`, `acta/seg/v1`, `acta/gate/v1`, `acta/dedup/v1`, `acta/blob/v1`, `acta/tree/v1`, `acta/commit/v1`, `acta/cmt/v1` (Commitments), `acta/ledger/v1`, `acta/checkpoint/v1`, `acta/report/v1`, `acta/rec/v1`, `acta/log/v1`, `acta/scope/v1`, `acta/gen/v1`. Neue Zwecke verlangen ein neues Präfix; Wiederverwendung ist verboten.

### 4.2 Schlüsselhierarchie [E]

```
          KeyAnchor (HSM, KMS oder FileAnchor)
   liefert: Generationsgeheimnis M_g · Sicherungs-Epochenschlüssel M_bk(e)
            Notar-Signaturschlüssel · k_idx · Anker-Köpfe der Ketten
                        │
          je Mandant t: Tresor  VK(t,g) = HKDF(M_g, salt = TenantId, info = "acta/vault/v1" ‖ g u32)
                        │   Einträge einzeln versiegelt unter VK(t,g)
   ┌─────────┬──────────┼───────────┬───────────┬────────────┬────────────┐
 SubjectKey DocumentKey SegmentKey ScopeKey   CommitKey(e)  LogKey(m)
 je Betrof- je Dokument- je versie- je Lösch-  je Epoche    je Monat des
 fenem      Version     geltem      vorgang    für Commit-  Zugriffs-
 (+Alias)   (id=VerRef) Segment     (kurz-     ments        protokolls
                        (K, DK, DP) lebig)

 Dokumentkopf-Schlüssel  HK(v)  = HKDF(DocumentKey(v), salt = VerRef(v), info = "acta/hdr/v1")
 Datenschlüssel          DEK(v) = HKDF( IKM  = DocumentKey(v) ‖ SubjectKey(s₁) ‖ … ‖ SubjectKey(sₙ),
                                        salt = VerRef(v),
                                        info = "acta/dek/v1" ‖ n u8 )     s₁ … sₙ aufsteigend nach SubjectTag
 Verzeichnisschlüssel    DirKey(s) = HKDF(SubjectKey(s), "acta/dir/v1")
 Clusterkennung          ClusterId(s) = HKDF(SubjectKey(s), "acta/cluster/v1")[..16]
```

**Dokumentkopf [E].** Die Liste der Betroffenen einer Version (`SubjectTag`s, `n`, Modus, `DocRef`, `VerOrd`, `case_ids`, Klasse, `retain_until`, `supersede_reason`) steht im **Dokumentkopf**, einem kleinen Umschlag unter `HK(v)`. Er ist allein mit dem `DocumentKey` lesbar; daraus werden `SubjectKey`s nachgeschlagen und der `DEK` abgeleitet. Kein Klartext-Tag steht in Segmenten oder Journalen.

**Wirkung:**

- Wird `SubjectKey(sᵢ)` vernichtet, sind **alle** noch an `sᵢ` gebundenen Dokument-Versionen unlesbar (Modus `DeleteAll`, 8.9; Z3).
- Wird `DocumentKey(v)` vernichtet, ist genau diese Version unlesbar (Einzellöschung, Versionsende).
- Zwei Versionen haben nie denselben DEK (Z14); jede Berichtigung, `AddSubject`, `RedactSplit` oder `RestrictShared` erzeugt eine **neue Version** mit neuer `VerRef`, neuem `DocumentKey` und neuem DEK. Die alte Version unterliegt `Superseded` (8.1).
- **Bindungsregel (Beschluss):** `SubjectKey(s)` wird **erst** vernichtet, wenn keine Version mehr an `s` gebunden und lesbar bleiben muss. Dokumente, die wegen Aufbewahrungspflicht oder Hold eines Mitbetroffenen erhalten bleiben (`RestrictShared`), werden zuvor als neue Version **ohne Bindung an `s`** abgelegt (die Daten bleiben, der Schlüssel von `s` ist nicht mehr beteiligt). So blockiert eine zurückgestellte Version nie die Löschung der übrigen Daten von `s` (8.9).
- Höchstens `max_subjects_per_doc` = 64 gebundene Betroffene je Version. Darüber ist das Dokument eine **Sammelakte**: DEK mit `n = 0` (nur `DocumentKey`), die Mitgliederliste steht im Dokumentkopf, jedes Mitglied führt die Version in seinem Betroffenenverzeichnis. Die Löschung eines Mitglieds einer Sammelakte löst **immer** den Review aus und endet in `RedactSplit` (8.9); nie in `DeleteAll`.
- Dokumente ohne Betroffenen (`Unklassifiziert`, Entscheidungsnutzlasten ohne Personenbezug) nutzen ebenfalls `n = 0`.
- **Aliase.** Eine Person kann mehrere externe Kennungen haben. Jede weitere Kennung erhält einen Tresoreintrag `SubjectAlias` (Tag der Kennung → Primär-Tag). Beim Löschen werden alle Aliase aus dem Betroffenenverzeichnis vernichtet.

Fehlt ein erforderlicher Schlüssel und Gate oder Löschregister belegt die Löschung, ist das Ergebnis `Err(Erased)`. Fehlt er ohne Beleg oder ist der Zustand nicht eindeutig (Tresor nicht lesbar), ist das Ergebnis `Err(Integrity)` bzw. `Err(Crypto)` mit Vergiftung. Ein Fallback ist ausgeschlossen (Z10). Lesepfade prüfen das Gate **vor** der Schlüsselsuche (3.6, 8.1); die Reihenfolge verhindert, dass ein bereits entfernter Tresoreintrag eine gesperrte Version als Integritätsfehler erscheinen lässt.

### 4.3 Tresor, Rotation, Sicherungsexport [E]

**Eintrag (Klartext):** `[kind u8][status u8][id 16][key 32]` = 50 Byte; versiegelt zu `[len u16][nonce 24][ciphertext 50][tag 16]` = 92 Byte. AAD = `acta/vault/v1 ‖ TenantId ‖ generation u32 ‖ slot u64` (**Position im Tresor**; die `id` steht nur im Chiffrat). `kind`: `0x01` SubjectKey, `0x02` DocumentKey (id = VerRef), `0x03` SegmentKey (id = `file_id`), `0x04` ScopeKey (id = ErasureId), `0x05` CommitKey (id = Epoche), `0x06` SubjectAlias (id = Alias-Tag, key = Primär-Tag ‖ 16 Nullbyte), `0x07` LogKey (id = Monatsindex). `status`: `0x01` aktiv, `0x02` vernichtet.

**Ein Tresor je Mandant** (`vault-<TenantOrd>.<generation>`). Der **Speicherindex** (`KeyId → slot`) existiert **nur im Arbeitsspeicher** und wird beim Öffnen durch Scan aufgebaut; es gibt keinen persistenten Index mit Klartext-IDs. **[Q]** Öffnen eines Tresors mit 10 Mio. Einträgen unter 30 s.

**R-KEY-FIRST [E] (Schlüssel zuerst).** Jeder Schlüssel wird im aktuellen Tresor (Generation g) angefügt und mit `fsync` bestätigt (Gruppen-Commit, 3.6), **bevor** ein Umschlag unter ihm geschrieben oder referenziert wird. Es gibt kein Schlüsseljournal außerhalb des Tresors und keine Schlüsselhaltung nur im Arbeitsspeicher.

**Vernichtung eines Schlüssels:** Der Eintrag wird durch einen Grabstein ersetzt (`status = 0x02`, `id` und `key` genullt, gleiche Länge, gleiche Versiegelung unter gleichem Slot), die Datei wird synchronisiert, und der Zustand wird mit `O_DIRECT` zurückgelesen (Direct-Read-Verifikation). Ein Grabstein trägt keine Kennung. **Batch-Vernichtung:** `destroy_many(&[KeyId])` bündelt Schreibvorgänge und `fsync`.

**Rest-Fenster auf Flash [E].** Das Überschreiben an Ort und Stelle erreicht auf Flash und in Dateisystem-Journalen nicht zuverlässig die physische Zelle (A-FLASH). Der alte Chiffreeintrag unter `VK(t,g)` bleibt bis zur Rotation lesbar für jeden, der Abbild **und** `M_g` besitzt (T5). Ein Grabstein allein erreicht daher nur den Zwischenstand `key_tombstoned` (8.7).

**Wurzelrotation (orchestriert über alle Mandanten).** Auslöser: alle `T_ROT` = 12 h, nach `N_SHRED` = 10 000 Vernichtungen, oder **auf Anforderung** (Klasse `Besonders`, Löschvorgänge mit 70 % verbrauchtem `erase_sla_B`; frühestens `rotation_min_gap` = 15 min nach der letzten Rotation, Anforderungen werden gebündelt). Ablauf:

1. `rotation_lock` nehmen (Mandantenanlage wartet). `anchor.rotate()` liefert `g+1`; die Mandantenliste wird festgehalten.
2. Je Mandant (bis `rotation_parallel` = 4 gleichzeitig): Schnappschuss `N0` aller **lebenden** Einträge; Neuschreiben unter `VK(t,g+1)` in `vault-<ord>.<g+1>.tmp` **ohne Sperre** für Schlüsselerzeugung und -vernichtung. Tresor `g` arbeitet unverändert weiter (Anfügen, Grabsteine). Grabsteine werden **nicht** übernommen (Verdichtung).
3. `vault_freeze(t)` (kurz): der **Schwanz** (alle Anfügungen und Grabsteine nach `N0`) wird in die neue Datei übernommen; `fsync` der Datei, atomares `rename`, `fsync` des Verzeichnisses; Speicherindex wird getauscht; `vault_freeze` endet.
4. Wenn **alle** Mandanten umgestellt sind: `anchor.destroy(g)` liefert `DestroyProof`; Journal-Eintrag `RootRotated { generation }` im E-Ledger jedes Mandanten (Reihenfolge: Tatsache vor Journal, C16).

Fällt ein Mandant aus, bleibt `M_g` erhalten, die Rotation wird wiederholt (`RotationOverdue`, Klasse K verzögert sich). Eine Tresordatei der Generation ≤ g ist nach Schritt 4 nicht mehr lesbar. Absturz zwischen Schritt 3 und 4: Der Start setzt Schritt 4 fort, sobald `anchor.current_generation() > vault.generation` für alle Mandanten gilt, und beendet unvollständige Mandanten zuerst.

**Sicherungsexport [E].** Eine Sicherung des Tresors ist **kein** Dateikopie-Abbild. `acta-cli backup vault` und die Hintergrundaufgabe schreiben einen **Export**, in dem alle lebenden Einträge unter `VKb(t,e) = HKDF(M_bk(e), salt = TenantId, info = "acta/vaultbk/v1")` neu versiegelt sind (keine Grabsteine, einschließlich `CommitKey`s und `LogKey`s). Der Anker erzeugt `M_bk(e)` je Sicherungsepoche `backup_epoch` = 24 h und vernichtet ihn zum Zeitpunkt `end(e) + T_BACKUP` (`destroy_backup_epoch`). Ein Export ist damit höchstens `T_BACKUP + backup_epoch` gültig; danach ist er unlesbar, unabhängig davon, wohin er kopiert wurde. Exportdateien werden nach Epochenende + `T_BACKUP` mit Scrub entfernt. Export und Schlüsseländerungen sind über `vault_freeze` serialisiert (konsistenter Stand).

**Wiederherstellungspunkt für Schlüssel (RPO) [E].** Schlüssel, die nach dem letzten gültigen Export entstanden, gehen bei Tresorverlust verloren; die zugehörigen Dokumente sind dann unlesbar. Daher gilt `vault_export_interval` = 1 h; der Betriebsbericht weist das tatsächliche Intervall aus; fehlt ein gültiger Export innerhalb `T_BACKUP`, löst `BackupOverdue` aus (14.3). Ein vom Tresor getrenntes Schlüsseljournal ist ausgeschlossen (es wäre eine nicht rotierte Schlüsselkopie).

**Tresorgröße.** 92 Byte je Eintrag; bei 50 Mio. Dokument-/Personenschlüsseln ca. 4,6 GB. **[Q]** Rotation von 10 Mio. Einträgen unter 120 s.

**Off-Boarding eines Mandanten:** Tresordatei und Exportdateien des Mandanten werden entfernt (Scrub). Weil `M_bk(e)` global ist, endet die Lesbarkeit kopierter Exporte erst mit Ablauf der Sicherungsepochen; der Bericht weist `offboarding_exposure_until` aus (12.6).

### 4.4 KeyAnchor [E]

```rust
pub trait KeyAnchor: Send + Sync {
    fn current_generation(&self) -> Result<u32, AnchorError>;
    fn min_generation(&self) -> Result<u32, AnchorError>;                       // niedrigste noch gültige Generation
    fn master(&self, generation: u32) -> Result<MasterKey, AnchorError>;       // Zeroizing, kein Clone/Default
    fn rotate(&self) -> Result<u32, AnchorError>;
    fn destroy(&self, generation: u32) -> Result<DestroyProof, AnchorError>;
    fn backup_master(&self, epoch: u32) -> Result<MasterKey, AnchorError>;
    fn destroy_backup_epoch(&self, epoch: u32) -> Result<DestroyProof, AnchorError>;
    fn index_secret(&self) -> Result<IndexSecret, AnchorError>;                 // k_idx
    fn notary_sign(&self, domain: &str, msg: &[u8]) -> Result<Signature, AnchorError>;
    fn notary_public(&self) -> Result<PublicKeyInfo, AnchorError>;              // inkl. sig_alg, key_id
    fn record_head(&self, chain: ChainId, seq: u64, hash: [u8; 32]) -> Result<(), AnchorError>;
    fn last_head(&self, chain: ChainId) -> Result<Option<(u64, [u8; 32])>, AnchorError>;
    fn assurance(&self) -> AnchorAssurance;           // Hardware | Kms { destroy_window } | FilePassphrase
}
```

Implementierungen: `FileAnchor` (Entwicklung, kleine Installationen ohne `Besonders`), `Pkcs11Anchor` und `KmsAnchor` (Produktion; getrennte Features, additiv).

- **[A] A-ANCHOR:** Der Anker kann ein Generationsgeheimnis innerhalb einer deklarierten Frist `anchor_destroy_max` so vernichten, dass es nicht rekonstruierbar ist. Besitzt ein KMS eine Pflicht-Löschfrist (z. B. Wartefenster bei Cloud-Diensten, Herstellerangaben [V]), MUSS `anchor_destroy_max` sie enthalten; Klasse K ist dann frühestens nach dieser Frist erreicht, und der Bericht weist die tatsächliche Frist aus.
- **`FileAnchor`:** Die Generationsdatei liegt auf demselben Medium wie die Daten; `destroy` kann Flash-Reste nicht ausschließen. Klasse K wird mit `assurance = FilePassphrase` ausgewiesen; die Unlesbarkeit beruht dann allein auf Passphrase und Argon2id. Die Passphrase MUSS mindestens 80 Bit geschätzte Entropie haben (Stärkeprüfung beim Anlegen), DARF nirgends gespeichert werden, ist auf mindestens zwei Personen aufzuteilen und bei Verdacht zu wechseln. `FileAnchor` ist für Produktion mit `Besonders` **nicht zulässig** (Invariante R-ANCHOR).
- **Anker-Köpfe.** Ledger und Gate-Log melden ihren Kopf (`seq`, `hash`) beim Checkpoint an den Anker (`record_head`). Nach einer Wiederherstellung erkennt der Knoten damit ein zurückgerolltes Ledger (8.8).
- Schlüsselhinterlegung beim Hersteller und Wiederherstellungsdienste des Herstellers sind ausgeschlossen (Z19).

### 4.5 Umschlag v1 [E]

```
┌─────────┬─────────┬──────────────┬────────────┬──────────────────────────────┬──────────┐
│ ver u8  │ alg u8  │ key_class u8 │ nonce (24) │ ciphertext (gepolstert)      │ tag (16) │
│  0x01   │  0x01   │ 0x01 P·0x02 K│            │ [len u32][Daten][Nullfüllung]│ Poly1305 │
│         │         │ ·0x03 D      │            │                              │          │
└─────────┴─────────┴──────────────┴────────────┴──────────────────────────────┴──────────┘
AAD = "acta/rec/v1" ‖ tenant_id ‖ class_u8 ‖ keyspace_u16 ‖ record_ref(16) ‖ version_u64be
```

**Keyspaces (`keyspace_u16`):** `0x0001` P-Datensatz, `0x0002` P-Dokumentkopf, `0x0003` Betroffenenverzeichnis, `0x0004` Handle-Register, `0x0005` Vorgang, `0x0006` Entscheidungsnutzlast, `0x0010` K-Pack-Block, `0x0011` Entscheidungsprotokoll-Block, `0x0020` DK-Block, `0x0021` DP-Block, `0x0030` Journalnutzlast, `0x0040` Arbeitsdaten (ScopeKey), `0x0050` Zugriffsprotokoll-Eintrag, `0x0060` Ausgangskorb, `0x0070` Idempotenztabelle.

**Auffüllung (Padding).** Der Klartext wird vor dem Verschlüsseln auf Größenklassen aufgefüllt: bis 4 096 Byte auf ein Vielfaches von 256, darüber auf das nächste Vielfache von `2^(⌊log2 n⌋−3)` (höchstens 12,5 % Mehrbedarf). Damit verraten exakte Längen weder Dokument noch Betroffene; die Größenklasse bleibt als Restmerkmal und wird ausgewiesen.

`open` liefert `Zeroizing<Vec<u8>>` mit vorab reservierter Kapazität (kein Wachsen, damit keine Restkopien entstehen). Jeder Fehler (Länge, MAC, Schlüssel) führt ohne unterscheidbares Orakel zu `DecryptionFailed`; ist der Schlüssel nachweislich vernichtet (Gate oder Register), zu `Erased`. Die Nonce ist **kein** Parameter der öffentlichen Schnittstelle.

### 4.6 Zufall, Zeit, Geheimnisse [E]

`CryptoRng::try_fill` ist fehlbar; ein Entropiefehler erzeugt nie einen Umschlag oder Schlüssel (`T-RNG-*`). `Clock` ist ein Port; `SystemTime::now` und `thread_rng` sind im Kern verboten. **Zeit ist nicht vertrauenswürdig (A-CLOCK):** `tx_time` fällt nie (6.3); ein Rücksprung der Uhr über `clock_skew_max` = 5 min setzt den Knoten in `Poisoned(Node, ClockRegression)`. Geheimnisse (`Dek`, `MasterKey`, `IndexSecret`, `SubjectKey`, `DocumentKey`, `SegmentKey`, `ScopeKey`, `LogKey`) besitzen weder `Default`, `Clone`, `Debug` noch `Serialize`. Schlüsselmaterial liegt ausschließlich in `SecureBuf` (`mlock`, `MADV_DONTDUMP`, feste Kapazität).

### 4.7 Kryptoagilität und Neubewertung [E]

Die Unlesbarkeit vernichteter Daten gilt nur, solange die Primitive halten (EDPB-Leitlinien 02/2025 zur Blockchain, Rn. 51, 85, 96: Notfallplan für den Algorithmenwechsel, Risikobewertung unter Einbeziehung von Obsoleszenz und Quantencomputern, periodische Neubewertung [V]). Acta setzt um:

1. **Neubewertung:** Mindestens alle `crypto_review_interval` = 12 Monate erzeugt `acta-reports` einen Kryptobericht (verwendete `alg`/`sig_alg`, Schlüssellängen, Abgleich gegen die jeweils aktuelle BSI TR-02102 [V] und Herstellerhinweise). Er ist Teil der Freigabepflichten (20.4).
2. **Migration ohne Datenverlust (`Rewrap`):** liest Umschläge mit altem `alg`, schreibt sie mit neuem `alg` unter demselben Schlüssel neu (Hintergrundaufgabe, Segment für Segment, atomarer Austausch) und weist den Anteil migrierter Segmente aus. Zuerst werden noch **nicht vernichtete** Daten umgeschrieben; Chiffrate vernichteter Daten sind ohne Schlüssel und werden mit der physischen Bereinigung entfernt.
3. **Signaturen:** Berichte und Checkpoints tragen `sig_alg`. Ed25519 ist nicht quantenresistent; Berichte DÜRFEN zusätzlich mit einem zweiten Verfahren signiert werden (hybrid), sobald ein standardisiertes Verfahren verfügbar ist. Alte Checkpoints werden vor Ablauf der Vertrauensdauer des alten Verfahrens neu bezeugt (Re-Witness, Eintrag im E-Ledger).
4. **Symmetrische Schlüssel** mit 256 Bit gelten nach heutigem Kenntnisstand als ausreichend; das ist eine Annahme (A-CRYPTO), keine Zusage.
5. **Notfallplan Algorithmenwechsel:** Auslöser (Herstellerhinweis, BSI-Empfehlung, Kryptoanalyse), Verantwortliche, Reihenfolge (zuerst `Rewrap` lebender Daten, dann Signaturen, dann Tresor und Anker), Zielzeiten, Entscheidung über die vorzeitige Beendigung des alten Verfahrens. Der Plan ist Teil des Löschkonzepts (12.4).
6. **Algorithmus-Byte `0x02` (AES-256-GCM)** bleibt reserviert; ein Wechsel erfordert `Rewrap` und einen ADR-Eintrag.
---

