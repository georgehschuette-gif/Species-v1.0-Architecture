// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

use crate::TensionsError;

pub struct PressureGradient {
    pub vector: Vec<f64>,
    pub magnitude: f64,
}

impl PressureGradient {
    pub fn new(vector: Vec<f64>) -> Result<Self, TensionsError> {
        let mag = vector.iter().map(|v| v * v).sum::<f64>().sqrt();
        if !(0.0..=f64::INFINITY).contains(&mag) { return Err(TensionsError::OutOfRange { field: "magnitude".into(), value: mag, min: 0.0, max: f64::INFINITY }); }
        Ok(Self { vector, magnitude: mag })
    }

    pub fn direction(&self) -> Vec<f64> { self.vector.clone() }
    pub fn strength(&self) -> f64 { self.magnitude }
    pub fn step(&mut self, field: &mut PressureField) -> Result<(), TensionsError> {
        if field.dimensions() != self.vector.len() { return Err(TensionsError::DimensionMismatch { expected: field.dimensions(), actual: self.vector.len() }); }
        for (i, g) in self.vector.iter().enumerate() {
            field.values[i] = (field.values[i] + g * 0.1).clamp(0.0, 1.0);
        }
        Ok(())
    }
    pub fn normalize(&mut self) { let mag = self.magnitude.max(1e-12); for v in &mut self.vector { *v /= mag; } self.magnitude = 1.0; }
    pub fn validate(&self) -> Result<(), TensionsError> {
        for (i, v) in self.vector.iter().enumerate() { if !v.is_finite() { return Err(TensionsError::OutOfRange { field: format!("vector[{}]", i), value: *v, min: -f64::INFINITY, max: f64::INFINITY }); } }
        Ok(())
    }
}

impl fmt::Debug for PressureGradient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PressureGradient").field("magnitude", &self.magnitude).finish()
    }
}
