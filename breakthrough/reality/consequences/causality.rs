// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::RealityError;

use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct CausalLink {
    pub cause_id: usize,
    pub effect_id: usize,
    pub strength: f64,
    pub mechanism: String,
    pub time_lag: usize,
}

impl CausalLink {
    pub fn new(cause_id: usize, effect_id: usize, strength: f64, mechanism: impl Into<String>, time_lag: usize) -> Self {
        Self {
            cause_id,
            effect_id,
            strength,
            mechanism: mechanism.into(),
            time_lag,
        }
    }

    pub fn is_strong(&self, threshold: f64) -> bool {
        self.strength >= threshold
    }

    pub fn validate(&self) -> Result<(), RealityError> {
        if !(0.0..=1.0).contains(&self.strength) {
            return Err(RealityError::OutOfRange {
                field: "strength".into(),
                value: self.strength,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(())
    }
}

impl fmt::Display for CausalLink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CausalLink({} -> {}, strength={:.2}, mechanism='{}', lag={})",
            self.cause_id, self.effect_id, self.strength, self.mechanism, self.time_lag
        )
    }
}