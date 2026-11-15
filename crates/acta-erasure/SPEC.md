# acta-erasure — Crate-Spezifikation (Fassung 3.0 Final)

*Bezug: Normative Gesamtspezifikation [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) (Teil IV, V, VI, VII, VIII, XI, XII)*  
*Ring: R3 (Löschkern) · Geltung: Verbindlich für `acta-erasure`*

---

## 1. Zweck und Einordnung

`acta-erasure` ist das Herzstück der DSGVO-Löschkette in **Ring R3 (Löschkern)**:
- **Lösch-Saga (E0 bis E9):** Monotoner Zustandsautomat (`Requested` $\rightarrow$ `Gated` $\rightarrow$ `KeyDestroyed` $\rightarrow$ `Purging` $\rightarrow$ `Verifying` $\rightarrow$ `Completed`).
- **ReadGate & GateLog:** Sofortige Lesesperre in Schritt E2 mit $O(1)$-Latenzzusage (< 500 ms) via `ErasedOrds: RoaringBitmap`.
- **Kryptografischer Tresor (Vault):** 16-Shard-Tresor für Ancestor-Salts (U, T, C, S, D, F, E). Grabsteine nullen `tag` und `payload` vollständig (V-11).
- **Epochenrotation (V-04):** Rollierende Maskierung der Slots unter $E_e$ und HSM-Vernichtung alter Epochen zur Abwehr von Backup-Wiederbelebungen.
- **Forensische Messung:** Typestate-Scanner (`Scanner<Unarmed>` $\rightarrow$ `Scanner<Armed>` $\rightarrow$ `LayerWitness`) über MK1, MK2 und MK3.
- **Audit-Bericht & Notar:** Signierter `ErasureReport` v1 nach RFC 8785 (JCS) mit kryptografischer Hash-Kette im Ledger.

---

## 2. Architektur und Workspace-Integration

| Merkmal | Vorgabe (docs/SPEC.md §3.1, §3.2) |
|---|---|
| **Ring** | **R3 (Löschkern)** |
| **Workspace-Mitgliedschaft (V-03)**| Zwingend in `[workspace] members` und `default-members` |
| **Erlaubte interne Abhängigkeiten** | Ring R0 (`acta-ids`, `acta-crypto`, `acta-env`, `acta-sys`, `acta-durable-fs`), Ring R2 (`acta-types`, `acta-ports`) |
| **Default-Features** | `["signing", "vault"]` (`async` ist rein opt-in!) |
| **Fehlertyp** | `ErasureError` (Fail-Closed, ohne Fallback) |

---

## 3. Normative Anforderungen und Defektbehebungen

1. **Behebung V-01 & V-05 (Beseitigung aller Krypto-Fallbacks, S7):**  
   - Bei nicht lesbarem oder gelöschtem Slot MUSS `resolve_dek` zwingend `Err(ErasureError::Erased(SlotType))` zurückgeben.
   - Der Fallback auf `blake3::keyed_hash(&k_unit, key)` ist vollständig entfernt.
   - Sämtliche Dummy-Schlüssel (`[0x55; 32]`, `[0x42; 32]`, Nullschlüssel) sind eliminiert.
2. **Behebung V-02 (Umschlag v3):**  
   - Der veraltete XOR-Sealer ist entfernt. Siegelung erfolgt ausschließlich mit XChaCha20-Poly1305.
3. **Behebung V-04 (Slot-Maskierung & Epochenrotation):**  
   - Slot-Payloads liegen im Tresor niemals im Klartext. Sie sind unter $K_{\text{enc}}$ maskiert.
   - Epochenrotation alle $T_{\text{ROT}} = 24\text{ h}$ oder $10\,000$ Vernichtungen mit physischer Anker-Zerstörung von $E_e$.
4. **Behebung V-11 & V-12 (Tresor-Härtung):**  
   - Grabsteine nullen `tag` und `payload` (alle Bytes 0x00).
   - Zur Entlastung von Sweepern existiert `destroy_many(&[SlotRef])` mit gebündeltem fsync.
5. **Behebung V-15 (Pseudonymitätsfreies Widerrufs-Log):**  
   - Keine Klartext-IDs im Widerrufs-Log. Einträge nutzen keyed Commitments unter $K_{\text{commit}}[e]$.
6. **Löschsaga-Schrittfolge (F-1 & F-2):**  
   - **E2 Gate zuerst:** $O(1)$-Garantie bis zum Rücksprung.
   - **E2a Nadelstichprobe:** Gedeckelt auf maximal 4.096 Klartextfenster, entnommen *nach* dem Gate.
   - **E2b Pflicht-Ack:** E3 (Schlüsselvernichtung) darf erst nach bestätigtem RAM-Zeroing aller Schichten ausgeführt werden.

---

## 4. Tests und Abnahme

- **Pflichttest V-01:** `vault_missing_slot_is_erased` – Nach Slot-Vernichtung liefert `open` hart `Err(Erased)`, niemals Datenmüll.
- **Pflichttest V-04:** `slot_payload_masked_at_rest` – Binärer Scan bestätigt die Abwesenheit von Klartext-Salts im Tresor.
- **Pflichttest V-14:** `proof_crash_resume_matrix` und `proof_fail_closed_and_consistency` kompilieren fehlerfrei und bestehen alle 12 Crash-Punkte.
- **Pflichttest V-15:** `revocation_log_has_no_raw_ids` – Bestätigt Abwesenheit von Personen-Kennungen im Widerrufs-Log.
- Gate `G-ERA-1`: Löschungen dürfen ausschließlich über `ErasureCore::execute` initiiert werden.
