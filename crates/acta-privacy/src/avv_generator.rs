//! `avv_generator` — Re-Export-Schicht auf `acta-avv-generator`.
//!
//! # Abkündigung & Konsolidierung (AV-1)
//! Dieses Modul diente historisch der Generierung von Entwürfen für Auftragsverarbeitungsverträge
//! (AVV) gemäß Art. 28 DSGVO (Datenschutz-Grundverordnung). Die Logik wurde in das dedizierte Crate
//! `acta-avv-generator` ausgelagert.
//!
//! # Ring-Zugehörigkeit
//! Ring 3 (Domain / Compliance).
//!
//! # Warnung
//! Veraltet (deprecated). Neue Implementierungen sollten direkt das Crate `acta-avv-generator` verwenden.

#![deprecated(note = "use acta-avv-generator crate directly")]

// ANCHOR[acta-privacy:AVV-001]
/// Re-Export des Kontexts für die AVV-Generierung aus `acta-avv-generator`.
#[deprecated(note = "use acta_avv_generator::AvvContext")]
pub use acta_avv_generator::AvvContext;

// ANCHOR[acta-privacy:AVV-002]
/// Generiert einen AVV-Entwurf im Markdown-Format anhand des übergebenen Kontexts.
///
/// # Ablauf
/// 1. Übergibt die Kontextstruktur an `acta_avv_generator::render_avv_markdown`.
/// 2. Im Erfolgsfall wird der generierte Markdown-Text zurückgegeben.
/// 3. Tritt bei der Formatierung ein Fehler auf, wird ein Fehlerhinweis als Markdown formatiert.
///
/// # Parameter
/// * `ctx`: Referenz auf den AVV-Kontext mit Angaben zu Verantwortlichem und Auftragsverarbeiter.
///
/// # Rückgabe
/// Markdown-formatiertes Dokument als `String`.
#[deprecated(note = "use acta_avv_generator::render_avv_markdown")]
pub fn generate_avv_draft(ctx: &acta_avv_generator::AvvContext) -> String {
    acta_avv_generator::render_avv_markdown(ctx).unwrap_or_else(|e| format!("# Fehler: {e}"))
}

#[cfg(test)]
#[allow(deprecated, clippy::field_reassign_with_default)]
mod tests {
    use super::*;

    // ANCHOR[acta-privacy:TEST-AVV-001]
    // Prüft die Äquivalenz der AVV-Generierung über den Re-Export mit den Standardfeldern.
    #[test]
    fn test_generate_avv_draft_equivalence() {
        let mut ctx = AvvContext::default();
        ctx.controller_name = "Acme Corp".to_string();
        ctx.processor_name = "Acta AG".to_string();
        let md = generate_avv_draft(&ctx);
        assert!(md.contains("Acme Corp") && md.contains("Art. 28 DSGVO"));
    }
}
