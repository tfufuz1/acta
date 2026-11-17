//! Ganzzahlige Ähnlichkeit (SPEC 9.3) und Gleitkomma-Testorakel.
//!
//! `dot = Σ qᵢ·dᵢ` als `i32`, `sim = (dot << 32) / isqrt(norm2_q · norm2_d)` als Q32.32.
//! Der Ranking-Pfad enthält kein Gleitkomma (Z7). `f32_cosine_oracle` dient nur der Messung.

use acta_base::fx::{isqrt, Fx};

use crate::error::VecError;
use crate::quantize::QuantizedVector;

/// Skalarprodukt zweier quantisierter Vektoren.
///
/// # Errors
/// `DimensionMismatch` bei abweichender Dimension, `CapacityExceeded` bei Überlauf.
pub fn dot_i32(a: &QuantizedVector, b: &QuantizedVector) -> Result<i32, VecError> {
    dot_raw(a.as_bytes(), b.as_bytes())
}

/// Exakte Ähnlichkeit zweier quantisierter Vektoren als Q32.32 (SPEC 9.3).
///
/// Der Nullvektor ergibt `0`. Für identische Vektoren ergibt sich exakt `1,0`.
///
/// # Errors
/// `DimensionMismatch` bei abweichender Dimension, `CapacityExceeded` bei Überlauf.
pub fn sim_q32_32(a: &QuantizedVector, b: &QuantizedVector) -> Result<Fx, VecError> {
    sim_parts(a.as_bytes(), a.norm2(), b.as_bytes(), b.norm2())
}

/// Ähnlichkeit auf rohen Teilen; Grundlage für den Scan ohne Zwischenobjekte.
pub(crate) fn sim_parts(a: &[i8], a_norm: u32, b: &[i8], b_norm: u32) -> Result<Fx, VecError> {
    let dot = dot_raw(a, b)?;
    // Produkt zweier u32 passt in u64; die Wurzel liegt damit höchstens bei 2^32.
    let root = isqrt(u64::from(a_norm) * u64::from(b_norm));
    if root == 0 {
        return Ok(Fx::ZERO);
    }
    let root = i64::try_from(root).map_err(|_| VecError::CapacityExceeded)?;
    let numerator = i64::from(dot)
        .checked_mul(Fx::ONE.0)
        .ok_or(VecError::CapacityExceeded)?;
    Ok(Fx(numerator / root))
}

fn dot_raw(a: &[i8], b: &[i8]) -> Result<i32, VecError> {
    if a.len() != b.len() {
        return Err(VecError::DimensionMismatch);
    }
    a.iter().zip(b).try_fold(0_i32, |acc, (&x, &y)| {
        acc.checked_add(i32::from(x) * i32::from(y))
            .ok_or(VecError::CapacityExceeded)
    })
}

/// Kosinus zweier `f32`-Vektoren in `f64` als Messorakel für Recall und Fehler.
///
/// Nicht für die Suche bestimmt. Ein Nullvektor ergibt `0,0`.
///
/// # Errors
/// `EmptyVector`, `DimensionMismatch`, `NonFiniteFloat`.
pub fn f32_cosine_oracle(a: &[f32], b: &[f32]) -> Result<f64, VecError> {
    if a.is_empty() {
        return Err(VecError::EmptyVector);
    }
    if a.len() != b.len() {
        return Err(VecError::DimensionMismatch);
    }
    let (mut dot, mut norm_a, mut norm_b) = (0.0_f64, 0.0_f64, 0.0_f64);
    for (&x, &y) in a.iter().zip(b) {
        if !x.is_finite() || !y.is_finite() {
            return Err(VecError::NonFiniteFloat);
        }
        let (x, y) = (f64::from(x), f64::from(y));
        dot += x * y;
        norm_a += x * x;
        norm_b += y * y;
    }
    let denom = norm_a.sqrt() * norm_b.sqrt();
    Ok(if denom > 0.0 { dot / denom } else { 0.0 })
}
