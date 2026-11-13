# ERRORS — acta-engine (S-64)

Dokumentation der Fehlerabbildung und Fehler-Durchreichung an der Integrationsschicht `acta-engine`.

## 1. Fehlerdomäne der Engine

Gemäß `docs/SPEC.md` §3.1 verwendet `acta-engine` als Ebene E2 (Integrationsschicht) die zentrale Fehlerdomäne `acta_types::ActaError`.

## 2. Lokale Fehlererzeugung in `acta-engine`

Die folgenden Varianten von `ActaError` werden in `acta-engine` direkt erzeugt:

| Variante | Quelle / Anwendungsfall | Display-Muster |
|---|---|---|
| `ActaError::InvalidInput(msg)` | Validierung von Dimension, Parametern, Query-Limits, Tenant-Scopes | `Ungültige Eingabe: {msg}` |
| `ActaError::PolicyViolation(msg)` | Lizenz-Autorisierung (`FeatureRing`), TenantPolicy-Verletzung | `Richtlinienverletzung: {msg}` |
| `ActaError::Transaction(msg)` | Transaktionskonflikte, Commit-Intent-Serialisierung, Intent-Rollback | `Transaktionsfehler: {msg}` |
| `ActaError::Serialization(msg)` | Import/Export, JSON/Bincode-Deserialisierung | `Serialisierungsfehler: {msg}` |
| `ActaError::LimitExceeded { limit, context }` | Extraktionslimits, Entitäts-/Tripel-Beschränkungen | `Limit überschritten: {limit} ({context})` |
| `ActaError::ParseError(msg)` | Parsing von Modell-Antworten / Extraktionsmustern | `Parsing-Fehler: {msg}` |
| `ActaError::Storage(msg)` | Initialisierungsprüfung, VFS-Repair-Fehler beim Booten | `Speicherfehler: {msg}` |
| `ActaError::Sandbox(msg)` | WASM-Sandbox Validierungs- und Ausführungsfehler (Feature `sandbox`) | `Sandbox-Fehler: {msg}` |
| `ActaError::Internal(msg)` | Unerwartete System-/Thread-Fehler | `Interner Fehler: {msg}` |

## 3. Fehler-Durchreichung aus den Backend-Subsystemen

Backend-Fehler aus den Speicher-, Index- und Port-Modulen werden über den `Result<T>`-Typ (`acta_types::Result<T>`) transparent und unverändert an Aufrufer propagiert:

| Backend | Quell-Fehlertyp | Durchgereichte Varianten | Verhalten an Fassade |
|---|---|---|---|
| `acta-store` | `StoreError` -> `ActaError` | `ActaError::Storage`, `ActaError::Corrupt`, `ActaError::Io` | Bitgleich durchgereicht mit unveränderter Meldung |
| `acta-vector` | `VectorError` -> `ActaError` | `ActaError::Index`, `ActaError::InvalidInput` | 1:1 durchgereicht |
| `acta-text` | `TextIndexError` -> `ActaError` | `ActaError::Index`, `ActaError::InvalidInput` | 1:1 durchgereicht |
| `acta-graph` | `GraphError` -> `ActaError` | `ActaError::Index`, `ActaError::InvalidInput` | 1:1 durchgereicht |
| `acta-rank` | `RankError` -> `ActaError` | `ActaError::InvalidInput`, `ActaError::Index` | 1:1 durchgereicht |
| `acta-checkpoint` | `CheckpointError` -> `ActaError` | `ActaError::Storage`, `ActaError::Corrupt` | 1:1 durchgereicht |
