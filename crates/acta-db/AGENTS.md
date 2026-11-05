# AGENTS.md — acta-db
> Ring 3 · stable · Quelle: capabilities.toml · Spec: K.9, III.19

## 1. Zweck

Dient als interne Re-Export-Fassade und Strangler Shell für Rückwärtskompatibilität über `acta-engine` und `acta-cognition`. Bietet zusätzlich den `AdaptiveDecayController`, `RerankPidController` (Homeostat), die `MultiStepEngine` für LLM-gestützte iterative Abfragen sowie den flüchtigen Arbeitsspeicher-Tresor (`VolatileContextVault`). Die primäre öffentliche Fassade für Anwendungsentwickler ist das Crate `acta`.

## 2. Modul-Karte
| Datei / Verzeichnis | Verantwortung |
|---|---|
| `src/lib.rs` | `#![forbid(unsafe_code)]`, Re-Exports von `engine`, `cognition` & Modul-Aliase |
| `src/homeostat.rs` | `RerankDeadline` und `RerankPidController` für adaptive Rerank-Latenzsteuerung |
| `src/multistep.rs` | `MultiStepEngine`, `QueryRewriter` und `MultiStepConfig` für iterative Suche |
| `src/volatile_vault.rs` | `VolatileContextVault` für ephemeren In-Memory-Speicher [Feature: `volatile-vault`] |

## 3. Invarianten

- **AGT-DB-001 / P28 TxId-Inkrement:** `TxId` **MUSS IMMER** über `collection.allocate_tx()` bezogen werden (deterministischer Zähler, keine direkte `SystemTime`).
- **INV-REEXPORT-COMPAT:** `acta-db` re-exportiert `acta-engine` und `acta-cognition` Typen ohne direkte eigene Storage-Mutationen.
- **INV-VAULT-1/2/3:** `VolatileContextVault` speichert Daten ausschließlich im flüchtigen RAM mit Nullisierung/mlock über `acta-sys`.

## 4. Verboten / Anti-Patterns

- **Keine direkte SystemTime für Transaktionen:** Kausalitätsbruch bei Graph & LSM.
- **Kein Halten von Guards über `.await`:** `consolidate_via_llm` oder `MultiStepEngine::search` dürfen niemals unter aktiven Locks ausgeführt werden.
- **Keine Verwechslung mit `acta` Facade:** `acta-db` ist eine interne L3-Fassade; externe Nutzer binden `acta` ein.

## 5. Nebenläufigkeit, Async- und Lock-Regeln

Vererbt die Lock-Hierarchie von `acta-engine`: `collections` (RwLock) → `kv_locks` (KvKeyLocks) → `embedder` (RwLock). MultiStep-LLM-Aufrufe erfolgen ohne aktive Locks, um Thread-Aushungerung zu verhindern.

## 6. Verifikation

```bash
cargo test -p acta-db
cargo test -p acta-db --all-features
cargo xtask check-agents-integrity
cargo xtask doctrine-scan --crate acta-db
```

## 7. Bekannte Lücken / SOLL

- `acta-db` ist eine Strangler-Shell im Übergang; neue High-Level-APIs werden in `acta-engine` bzw. `acta` entwickelt.

