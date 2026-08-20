// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::MetabolismError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecayCurve {
    Linear { rate: u8 },
    Exponential { base: u8 },
    Logarithmic,
}

impl DecayCurve {
    pub fn current_factor(&self) -> f64 {
        match self {
            DecayCurve::Linear { rate } => 100.0 - (*rate as f64),
            DecayCurve::Exponential { base } => (100.0 / (*base as f64)).max(0.0),
            DecayCurve::Logarithmic => 85.0,
        }
    }

    pub fn apply(&self, value: u64, efficiency_ratio: u8) -> Result<u64, MetabolismError> {
        let factor = self.current_factor() / 100.0;
        let efficiency_boost = efficiency_ratio as f64 / 100.0;
        let final_factor = (factor + efficiency_boost).min(1.0);
        Ok((value as f64 * final_factor) as u64)
    }
}

