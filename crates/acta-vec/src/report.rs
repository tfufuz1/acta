//! Messung der Quantisierungsgüte gegen `f32` (SPEC 9.3 [Q]: Recall@10 ≥ 0,99).
//!
//! Dies ist ein Messwerkzeug. Gleitkomma ist hier zulässig; der Such- und Rangpfad bleibt
//! ganzzahlig.

use std::collections::BTreeSet;

use acta_base::fx::Fx;

use crate::distance::{f32_cosine_oracle, sim_q32_32};
use crate::error::VecError;
use crate::quantize::quantize_f32;

/// Ergebnis der Qualitätsmessung.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QualityReport {
    /// Mittlerer Recall@k gegenüber der `f32`-Referenz, Wert zwischen 0 und 1.
    pub recall_at_k: f64,
    /// Mittlerer absoluter Fehler zwischen `f32`-Kosinus und quantisierter Ähnlichkeit.
    pub mean_abs_error: f64,
    /// Anzahl gemessener Paare aus Anfrage und Dokument.
    pub pairs: u64,
}

/// Misst Recall@k und mittleren Fehler der Quantisierung für Dokumente und Anfragen.
///
/// Die Referenz ist der `f32`-Kosinus. Der Vergleich nutzt dieselbe Rangfolge wie die Suche.
///
/// # Errors
/// `InvalidConfig` bei `k == 0`, `EmptyVector` bei leeren Mengen, `NonFiniteFloat`,
/// `DimensionMismatch` bei inkonsistenten Dimensionen.
pub fn evaluate_quantization_quality(
    docs: &[Vec<f32>],
    queries: &[Vec<f32>],
    k: usize,
) -> Result<QualityReport, VecError> {
    if k == 0 {
        return Err(VecError::InvalidConfig);
    }
    if docs.is_empty() || queries.is_empty() {
        return Err(VecError::EmptyVector);
    }
    let quantized_docs = docs
        .iter()
        .map(|d| quantize_f32(d))
        .collect::<Result<Vec<_>, _>>()?;
    let quantized_queries = queries
        .iter()
        .map(|q| quantize_f32(q))
        .collect::<Result<Vec<_>, _>>()?;

    let top_k = k.min(docs.len());
    let mut recall_sum = 0.0_f64;
    let mut error_sum = 0.0_f64;
    let mut pairs = 0_u64;

    for (raw_query, quant_query) in queries.iter().zip(&quantized_queries) {
        let mut reference: Vec<(f64, usize)> = Vec::with_capacity(docs.len());
        let mut approx: Vec<(Fx, usize)> = Vec::with_capacity(docs.len());
        for (idx, (raw_doc, quant_doc)) in docs.iter().zip(&quantized_docs).enumerate() {
            let exact = f32_cosine_oracle(raw_query, raw_doc)?;
            let sim = sim_q32_32(quant_query, quant_doc)?;
            error_sum += (exact - fx_to_f64(sim)).abs();
            pairs += 1;
            reference.push((exact, idx));
            approx.push((sim, idx));
        }
        reference.sort_by(|a, b| b.0.total_cmp(&a.0).then(a.1.cmp(&b.1)));
        approx.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));

        let truth: BTreeSet<usize> = reference.iter().take(top_k).map(|(_, i)| *i).collect();
        let hits = approx
            .iter()
            .take(top_k)
            .filter(|(_, i)| truth.contains(i))
            .count();
        recall_sum += hits as f64 / top_k as f64;
    }

    Ok(QualityReport {
        recall_at_k: recall_sum / queries.len() as f64,
        mean_abs_error: error_sum / pairs as f64,
        pairs,
    })
}

/// Umrechnung Q32.32 → `f64`, nur für die Messung.
fn fx_to_f64(value: Fx) -> f64 {
    value.0 as f64 / Fx::ONE.0 as f64
}
