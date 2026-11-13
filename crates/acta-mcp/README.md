# acta-mcp

Model Context Protocol (MCP) Server über stdio/JSON-RPC 2.0 für Acta.

## Rolle & Zweck
`acta-mcp` stellt einen gehärteten, zustandslosen Model Context Protocol (MCP) Server bereit, der über `stdio` mittels JSON-RPC 2.0 mit KI-Hostumgebungen (Claude Desktop, AI-Agenten, IDE-Erweiterungen) kommuniziert.

- **Sicherheits-Invarianten:** `#![forbid(unsafe_code)]` strikt erzwungen; keine unwrap/expect/panic im Produktionspfad; Fail-Closed-Sicherheitsgates für Prompt-Injection und Cloud-Egress.
- **Deterministische Puffergrenzen:** Bounded RPC Message Size (`MAX_RPC_BYTES`).

## Features
- `candle`: Optionales Inferenz-Backend via `acta-infer-candle`
- `ollama`: Optionales Loopback-Backend via `acta-infer-ollama`
- `onnx`: Optionales ONNX-Runtime-Backend via `acta-infer-onnx`
- `test-utils`: Hilfsmittel für Tests

## Verfügbare Tools
- `acta_search`: Hybride semantische Suche (Vektor + Text + Graph)
- `acta_get`: Dokument anhand der ID abrufen
- `acta_collections`: Kollektionen auflisten
- `acta_insert`: Neues Dokument indizieren
- `acta_upsert`: Dokument idempotent einfügen oder aktualisieren
- `acta_delete`: Einzelnes Dokument löschen
- `acta_forget`: Alias für `acta_delete` (Löschen ohne Nachweisobjekt)
- `acta_create_collection`: Neue Kollektion anlegen
- `acta_drop_collection`: Kollektion löschen
- `acta_explain`: Erklärung und Provenienz für Suchergebnisse ausgeben

## Konfiguration & Umgebungsvariablen
- `ACTA_MCP_ALLOW_WRITE`: Steuert Schreibberechtigungen (`1`, `true`, `yes`). Standardmäßig `false` (Nur-Lese-Modus).
- `ACTA_MCP_IDLE_TIMEOUT_SECS`: Inaktivitäts-Timeout in Sekunden für stdio (Standard: 30s).
- `ACTA_INJECTION_POLICY`: Sicherheitsrichtlinie für Prompt-Injection (`quarantine`, `warn`, `disabled`).

## Bekannte Grenzen
- [V] Ein Nur-Lese-Betrieb ist per vorhandener Konfiguration bereits vollständig vorgesehen und aktiv:
  - Über die Umgebungsvariable `ACTA_MCP_ALLOW_WRITE` (Standardwert: `false`) wird `allow_db_writes` in der `SandboxPolicy` gesteuert.
  - Ohne explizites Setzen von `ACTA_MCP_ALLOW_WRITE=true` sind alle mutierenden Schreibwerkzeuge (`acta_insert`, `acta_upsert`, `acta_delete`, `acta_forget`, `acta_create_collection`, `acta_drop_collection`) fail-closed durch die `McpSandbox` gesperrt.
- [H] Weitere feingranulare Filterung (z. B. lesende Beschränkung auf bestimmte Kollektionen) erfordert derzeit anwendungsspezifische Adapter oder Proxy-Schichten.

## Einbettungsbeispiel
Ein eigenständiges, autarkes Beispiel befindet sich unter `examples/mcp_standalone.rs` und kann ausgeführt werden:

```bash
cargo run --example mcp_standalone -p acta-mcp
```

```rust
use acta_mcp::protocol::{JsonRpcRequest, JsonRpcResponse};
use acta_mcp::sandbox::{McpSandbox, SandboxPolicy};
use serde_json::json;

// 1. Sandbox konfigurieren
let policy = SandboxPolicy {
    allow_db_reads: true,
    allow_db_writes: false,
    allow_code_execution: false,
    allow_cloud_egress: false,
    max_execution_ms: 1000,
};
let sandbox = McpSandbox::new(policy).expect("sandbox");

// 2. JSON-RPC Request verarbeiten
let ping_req = JsonRpcRequest {
    jsonrpc: "2.0".to_string(),
    id: Some(json!(1)),
    method: "ping".to_string(),
    params: json!({}),
};
let resp = JsonRpcResponse::ok(ping_req.id, json!({}));
```
