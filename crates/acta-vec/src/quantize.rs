//! Quantisierung `f32` → `i8` (SPEC 9.3).
//!
//! `s = 127 / max|xᵢ|`, `qᵢ = round_half_even(xᵢ·s)`, `norm2 = Σ qᵢ²` als `u32`.
//! Gespeichert werden die Bytes; alle späteren Berechnungen gehen von ihnen aus (Z7).

use crate::error::VecError;

/// Größter Betrag einer quantisierten Komponente.
pub const QMAX: f32 = 127.0;

/// Quantisierter Vektor: `i8`-Komponenten und vorberechnete Quadratsumme `norm2`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuantizedVector {
    data: Vec<i8>,
    norm2: u32,
}

impl QuantizedVector {
    /// Baut einen Vektor aus gespeicherten Bytes und berechnet `norm2` neu.
    ///
    /// # Errors
    /// `EmptyVector` bei leeren Bytes, `CapacityExceeded` bei Überlauf von `norm2`.
    pub fn from_bytes(data: Vec<i8>) -> Result<Self, VecError> {
        if data.is_empty() {
            return Err(VecError::EmptyVector);
        }
        let norm2 = norm2_of(&data)?;
        Ok(Self { data, norm2 })
    }

    /// Dimension des Vektors.
    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Ob der Vektor keine Komponenten hat (von den Konstruktoren nie erzeugt).
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Gespeicherte Komponenten.
    #[must_use]
    pub fn as_bytes(&self) -> &[i8] {
        &self.data
    }

    /// Quadratsumme der Komponenten.
    #[must_use]
    pub const fn norm2(&self) -> u32 {
        self.norm2
    }
}

/// Quantisiert einen `f32`-Vektor nach SPEC 9.3.
///
/// Der Nullvektor wird auf Nullen abgebildet. Die Skalierung geschieht mit dem größten Betrag,
/// daher liegt jede Komponente nach dem Runden in `[-127, 127]`.
///
/// # Errors
/// `EmptyVector` bei leerer Eingabe, `NonFiniteFloat` bei NaN oder Unendlich,
/// `CapacityExceeded` bei Überlauf von `norm2`.
pub fn quantize_f32(raw: &[f32]) -> Result<QuantizedVector, VecError> {
    if raw.is_empty() {
        return Err(VecError::EmptyVector);
    }
    let mut max_abs = 0.0_f32;
    for &x in raw {
        if !x.is_finite() {
            return Err(VecError::NonFiniteFloat);
        }
        max_abs = max_abs.max(x.abs());
    }
    let scale = if max_abs > 0.0 { QMAX / max_abs } else { 0.0 };
    let data: Vec<i8> = raw.iter().map(|&x| quantize_component(x * scale)).collect();
    let norm2 = norm2_of(&data)?;
    Ok(QuantizedVector { data, norm2 })
}

/// Rundet half-even und begrenzt auf den Wertebereich der Komponenten.
fn quantize_component(scaled: f32) -> i8 {
    scaled.round_ties_even().clamp(-QMAX, QMAX) as i8
}

/// Quadratsumme mit Überlaufprüfung.
fn norm2_of(data: &[i8]) -> Result<u32, VecError> {
    data.iter().try_fold(0_u32, |acc, &q| {
        let square =
            u32::try_from(i32::from(q) * i32::from(q)).map_err(|_| VecError::CapacityExceeded)?;
        acc.checked_add(square).ok_or(VecError::CapacityExceeded)
    })
}

/// Rückführung in Gleitkomma für Messungen: `qᵢ / 127`.
///
/// Der Faktor ist skalenfrei; die Richtung und damit der Kosinus bleiben erhalten.
#[must_use]
pub fn dequantize(q: &QuantizedVector) -> Vec<f32> {
    q.data.iter().map(|&v| f32::from(v) / QMAX).collect()
}
