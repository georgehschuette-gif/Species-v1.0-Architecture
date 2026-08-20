// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

use crate::TensionsError;

pub struct AmbiguityResolution {
    pub clarity: f64,
    pub residual_ambiguity: f64,
    pub iterations: usize,
}

impl AmbiguityResolution {
    pub fn new() -> Self {
        Self { clarity: 0.0, residual_ambiguity: 1.0, iterations: 0 }
    }

    pub fn iterate(&mut self, gradient: &AmbiguityGradient) -> Result<bool, TensionsError> {
        self.clarity = (self.clarity + gradient.steepness() * 0.1).min(1.0);
        self.residual_ambiguity = 1.0 - self.clarity;
        self.iterations += 1;
        Ok(self.residual_ambiguity < 0.1)
    }

    pub fn clarity(&self) -> f64 { self.clarity }
    pub fn is_resolved(&self) -> bool { self.residual_ambiguity < 0.1 }
    pub fn reset(&mut self) { self.clarity = 0.0; self.residual_ambiguity = 1.0; self.iterations = 0; }
    pub fn validate(&self) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&self.clarity) { return Err(TensionsError::OutOfRange { field: "clarity".into(), value: self.clarity, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for AmbiguityResolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AmbiguityResolution").field("clarity", &self.clarity).field("residual", &self.residual_ambiguity).field("iterations", &self.iterations).finish()
    }
}
