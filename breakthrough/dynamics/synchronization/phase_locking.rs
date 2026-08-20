// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct PhaseLocking {
    pub oscillators: Vec<f64>,
    pub locked_phase: f64,
    pub coupling_strength: f64,
}

impl PhaseLocking {
    pub fn new(oscillators: Vec<f64>, coupling_strength: f64) -> Result<Self, crate::DynamicsError> {
        if oscillators.is_empty() {
            return Err(crate::DynamicsError::SyncError("No oscillators provided".to_string()));
        }
        if coupling_strength < 0.0 || coupling_strength > 1.0 {
            return Err(crate::DynamicsError::SyncError("Coupling strength must be in [0, 1]".to_string()));
        }
        let locked_phase = oscillators.iter().sum::<f64>() / oscillators.len() as f64;
        Ok(Self { oscillators, locked_phase, coupling_strength })
    }

    pub fn order_parameter(&self) -> f64 {
        let n = self.oscillators.len() as f64;
        let sum_sin = self.oscillators.iter().map(|p| p.sin()).sum::<f64>();
        let sum_cos = self.oscillators.iter().map(|p| p.cos()).sum::<f64>();
        ((sum_sin.powi(2) + sum_cos.powi(2)).sqrt()) / n
    }
}

impl fmt::Display for PhaseLocking {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PhaseLocking(phase={:.2}, r={:.2})", self.locked_phase, self.order_parameter())
    }
}

