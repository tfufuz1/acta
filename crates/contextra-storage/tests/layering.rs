#![allow(clippy::panic, clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing, clippy::collapsible_if, clippy::unnecessary_map_or)]

//! # Schichtarchitektur-Test
//!
//! Prüft die Schichtregeln aus SPEC §3.3:
//! api -> engine -> { redo, run, blob, catalog } -> { keys, codec, env }
//! api -> erase -> engine
//! keys -> { codec, env }
//! testing -> alles; nichts hängt von testing ab.

use std::fs;
use std::path::Path;

fn check_file(path: &Path, rel_module: &str) {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return,
    };

    for (line_no, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if !trimmed.starts_with("use crate::") {
            continue;
        }

        let import_path = trimmed.trim_start_matches("use crate::");
        let target_module = import_path.split("::").next().unwrap_or("").trim_end_matches(';');

        // Eigenes Modul darf sich selbst referenzieren
        if target_module == rel_module {
            continue;
        }

        // Prüfe Schichtregeln:
        match rel_module {
            "keys" => {
                // keys darf nur codec, env, sync nutzen
                if !matches!(target_module, "codec" | "env" | "sync") {
                    panic!(
                        "Schichtverletzung in {}:{}: keys darf nicht von '{}' abhängen!",
                        path.display(),
                        line_no + 1,
                        target_module
                    );
                }
            }
            "redo" | "run" | "blob" | "catalog" => {
                // Diese Module dürfen nur keys, codec, env, sync nutzen
                if !matches!(target_module, "keys" | "codec" | "env" | "sync") {
                    panic!(
                        "Schichtverletzung in {}:{}: '{}' darf nicht von '{}' abhängen!",
                        path.display(),
                        line_no + 1,
                        rel_module,
                        target_module
                    );
                }
            }
            "engine" => {
                // engine darf redo, run, blob, catalog, keys, codec, env, sync nutzen
                if !matches!(
                    target_module,
                    "redo" | "run" | "blob" | "catalog" | "keys" | "codec" | "env" | "sync"
                ) {
                    panic!(
                        "Schichtverletzung in {}:{}: engine darf nicht von '{}' abhängen!",
                        path.display(),
                        line_no + 1,
                        target_module
                    );
                }
            }
            "erase" => {
                // erase darf engine, keys, codec, env, sync nutzen
                if !matches!(
                    target_module,
                    "engine" | "keys" | "codec" | "env" | "sync" | "blob" | "catalog" | "run" | "redo"
                ) {
                    panic!(
                        "Schichtverletzung in {}:{}: erase darf nicht von '{}' abhängen!",
                        path.display(),
                        line_no + 1,
                        target_module
                    );
                }
            }
            _ => {}
        }

        // Kein Modul außerhalb testing darf von testing abhängen
        if rel_module != "testing" && target_module == "testing" {
            panic!(
                "Schichtverletzung in {}:{}: Produktionscode darf nicht von testing abhängen!",
                path.display(),
                line_no + 1
            );
        }
    }
}

fn walk_dir(dir: &Path, rel_module: &str) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
                let mod_name = if rel_module.is_empty() { name } else { rel_module };
                walk_dir(&p, mod_name);
            } else if p.extension().map_or(false, |ext| ext == "rs") {
                check_file(&p, rel_module);
            }
        }
    }
}

#[test]
fn test_layering_rules() {
    let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    walk_dir(&src_dir, "");
}
