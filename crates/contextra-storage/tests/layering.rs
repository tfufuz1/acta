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

fn extract_crate_imports(content: &str) -> Vec<(usize, String)> {
    let mut results = Vec::new();
    let mut in_use = false;
    let mut current_stmt = String::new();
    let mut start_line = 1;

    for (idx, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("//") {
            continue;
        }

        if !in_use {
            if let Some(pos) = line.find("use crate::") {
                in_use = true;
                start_line = idx + 1;
                current_stmt = line[pos..].to_string();
            } else if let Some(pos) = line.find("use crate {") {
                in_use = true;
                start_line = idx + 1;
                current_stmt = line[pos..].to_string();
            }
        } else {
            current_stmt.push(' ');
            current_stmt.push_str(trimmed);
        }

        if in_use && current_stmt.contains(';') {
            in_use = false;
            let stmt = current_stmt.split(';').next().unwrap_or("").trim();
            if let Some(after) = stmt.strip_prefix("use crate::") {
                parse_import_body(after.trim(), start_line, &mut results);
            } else if let Some(after) = stmt.strip_prefix("use crate::{") {
                if let Some(inner) = after.strip_suffix('}') {
                    for item in inner.split(',') {
                        let item = item.trim();
                        if !item.is_empty() {
                            let mod_name = item.split("::").next().unwrap_or("").trim();
                            if !mod_name.is_empty() {
                                results.push((start_line, mod_name.to_string()));
                            }
                        }
                    }
                }
            }
            current_stmt.clear();
        }
    }
    results
}

fn parse_import_body(body: &str, line_no: usize, results: &mut Vec<(usize, String)>) {
    if body.starts_with('{') {
        let inner = body.trim_start_matches('{').trim_end_matches('}');
        for item in inner.split(',') {
            let item = item.trim();
            if !item.is_empty() {
                let mod_name = item.split("::").next().unwrap_or("").trim();
                if !mod_name.is_empty() {
                    results.push((line_no, mod_name.to_string()));
                }
            }
        }
    } else {
        let mod_name = body.split("::").next().unwrap_or("").trim();
        if !mod_name.is_empty() {
            results.push((line_no, mod_name.to_string()));
        }
    }
}

fn check_file(path: &Path, rel_module: &str, violations: &mut Vec<String>) {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return,
    };

    let imports = extract_crate_imports(&content);
    for (line_no, target_module) in imports {
        // Eigenes Modul darf sich selbst referenzieren
        if target_module == rel_module {
            continue;
        }

        // Prüfe Schichtregeln:
        match rel_module {
            "keys" => {
                // keys darf nur codec, env, sync nutzen
                if !matches!(target_module.as_str(), "codec" | "env" | "sync") {
                    violations.push(format!(
                        "Schichtverletzung in {}:{}: keys darf nicht von '{}' abhängen!",
                        path.display(),
                        line_no,
                        target_module
                    ));
                }
            }
            "redo" | "run" | "blob" | "catalog" => {
                // Diese Module dürfen nur keys, codec, env, sync nutzen
                if !matches!(target_module.as_str(), "keys" | "codec" | "env" | "sync") {
                    violations.push(format!(
                        "Schichtverletzung in {}:{}: '{}' darf nicht von '{}' abhängen!",
                        path.display(),
                        line_no,
                        rel_module,
                        target_module
                    ));
                }
            }
            "engine" => {
                // engine darf redo, run, blob, catalog, keys, codec, env, sync nutzen
                if !matches!(
                    target_module.as_str(),
                    "redo" | "run" | "blob" | "catalog" | "keys" | "codec" | "env" | "sync"
                ) {
                    violations.push(format!(
                        "Schichtverletzung in {}:{}: engine darf nicht von '{}' abhängen!",
                        path.display(),
                        line_no,
                        target_module
                    ));
                }
            }
            "erase" => {
                // erase darf engine, keys, codec, env, sync nutzen
                if !matches!(
                    target_module.as_str(),
                    "engine" | "keys" | "codec" | "env" | "sync" | "blob" | "catalog" | "run" | "redo"
                ) {
                    violations.push(format!(
                        "Schichtverletzung in {}:{}: erase darf nicht von '{}' abhängen!",
                        path.display(),
                        line_no,
                        target_module
                    ));
                }
            }
            _ => {}
        }

        // Kein Modul außerhalb testing darf von testing abhängen
        if rel_module != "testing" && target_module == "testing" {
            violations.push(format!(
                "Schichtverletzung in {}:{}: Produktionscode darf nicht von testing abhängen!",
                path.display(),
                line_no
            ));
        }
    }
}

fn walk_dir(dir: &Path, rel_module: &str, violations: &mut Vec<String>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
                let mod_name = if rel_module.is_empty() { name } else { rel_module };
                walk_dir(&p, mod_name, violations);
            } else if p.extension().map_or(false, |ext| ext == "rs") {
                check_file(&p, rel_module, violations);
            }
        }
    }
}

#[test]
fn test_layering_rules() {
    let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut violations = Vec::new();
    walk_dir(&src_dir, "", &mut violations);
    assert!(
        violations.is_empty(),
        "Schichtverletzungen gefunden ({} Treffer):\n{}",
        violations.len(),
        violations.join("\n")
    );
}
