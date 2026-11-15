# acta-store — Crate-Spezifikation (Fassung 3.0 Final)

*Bezug: Normative Gesamtspezifikation [docs/SPEC.md](file:///home/freddy/Projekte/acta/docs/SPEC.md) (Teil IV, V, VI, VII, VIII, X)*  
*Ring: R1 (Persistenz) · Geltung: Verbindlich für `acta-store`*

---

## 1. Zweck und Einordnung

`acta-store` ist die primäre LSM-Tree-Persistenz-Engine in **Ring R1 (Persistenz)**:
- **Speicherorganisation:** Getrennte Partitionen **N, P, D** je Mandant (Behebung V-07).
- **Schlüsselraum KeyV2 (Behebung V-06 & F-4):** 78 Byte Binärschlüssel mit fester Belegung `cluster = subject_tag` zur Sicherstellung physischer Datenlokalität.
- **SSTable Format 5:** Footer führt `source_set: RoaringBitmap` aller enthaltenen `doc_ord` (F-3).
- **Write-Ahead-Log (WAL v2):** HMAC-SHA256-verkettete Frames. Nutzlasten der Klasse P liegen vor dem WAL-Eintrag zwingend als Umschlag v3 verschlüsselt vor (B-03, Invariante I-17).
- **Compaction & Datei-Drop:** Atomarer Batch `AddSst + RemoveSst` im Manifest v3. Direkter Datei-Drop bei Betroffenen-Isolation ohne I/O-Rewrite.
- **Lösch-Barriere (Behebung V-08):** Asymmetrisches `FlushPurgeFence`, voll verdrahtet über `enter_purge`.
- **Retention mit ErasedView (F-8):** `may_drop_version(&self, v: VersionRef, gate: &dyn ErasedView)` verwirft Chiffrate gelöschter Betroffener bei der nächsten Compaction.

---

## 2. Architektur und Verträge

| Merkmal | Vorgabe (docs/SPEC.md §3.1) |
|---|---|
| **Ring** | **R1 (Persistenz)** |
| **Erlaubte interne Abhängigkeiten** | Ring R0 (`acta-ids`, `acta-crypto`, `acta-env`, `acta-sys`, `acta-durable-fs`, `acta-simd`, `acta-wire`), Ring R2 (`acta-types`, `acta-ports`) |
| **Sichtbarkeit** | `LsmStorage` ist `pub(crate)`. Öffentliche Lesezugriffe nur über gegatete Leser (Gate G-READ-1) |
| **Fehlertyp** | `StoreError` (Fail-Closed, Handle-Vergiftung bei Sync-Fehlern) |

---

## 3. Normative Anforderungen und Defektbehebungen

1. **Behebung V-06 (KeyV2 78 Byte):**  
   Vollständige Eliminierung des ASCII-Codecs (`t:{tenant}:{collection}:chunk:{doc}`). Das Schlüsselpräfix (65 Byte) lautet:
   $$\text{[kind u8][tenant\_tag 16][collection\_tag 16][cluster 16][doc\_tag 16]}$$
   gefolgt vom Suffix (13 Byte):
   $$\text{[!seq u64][flags u8][reserved 4]}$$
2. **Behebung V-07 (Partitionen N, P, D):**  
   `DEFAULT_PARTITION` ist entfernt. Compaction mischt niemals Daten unterschiedlicher Klassen (Invariante I-7).
3. **Behebung V-08 (FlushPurgeFence Verdrahtung):**  
   Purge-Läufe (Phase c und d) fordern zwingend `enter_purge()` an. Flushes melden `Busy`, solange ein Purge aktiv ist.
4. **Behebung V-17 (Compaction-Deadlock):**  
   Der Hintergrund-GC und Compaction-Worker dürfen auf Single-Core-Systemen nicht blockieren.
5. **Invariante I-17 (Keine Blockschlüssel):**  
   Kein SST-Block enthält Nutzdaten zweier Dokumente unter demselben Schlüssel.

---

## 4. Tests und Abnahme

- **Pflichttest V-06:** `no_ascii_keys` – Binärer Scan bestätigt die Abwesenheit von ASCII-Präfixen in SST-Dateien.
- **Pflichttest B-03:** `T-WAL-ENC` – P-Nutzdaten im WAL sind ausnahmslos Umschlag v3 Chiffrate.
- **Crash-Tests:** Matrix C7 und C8 (Abstürze während gefiltertem Rewrite und Manifest-Batch).
- Loom-Modelle für `FlushPurgeFence` und Memtable-Drain.
