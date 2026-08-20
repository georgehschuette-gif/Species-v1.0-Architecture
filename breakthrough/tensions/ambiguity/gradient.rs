// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::TensionsError;

use super::*;

pub struct AmbiguityGradient {
    pub entropy: f64,
    pub resolution_potential: f64,
}

impl AmbiguityGradient {
    pub fn new(entropy: f64) -> Result<Self, TensionsError> {
        if !(0.0..=1.0).contains(&entropy) { return Err(TensionsError::OutOfRange { field: "entropy".into(), value: entropy, min: 0.0, max: 1.0 }); }
        Ok(Self { entropy, resolution_potential: 1.0 - entropy })
    }

    pub fn entropy(&self) -> f64 { self.entropy }
    pub fn steepness(&self) -> f64 { self.resolution_potential }
    pub fn resolve(&mut self, amount: f64) -> Result<(), TensionsError> {
        self.entropy = (self.entropy - amount).max(0.0);
        self.resolution_potential = 1.0 - self.entropy;
        Ok(())
    }
    pub fn validate(&self) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&self.entropy) { return Err(TensionsError::OutOfRange { field: "entropy".into(), value: self.entropy, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for AmbiguityGradient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AmbiguityGradient").field("entropy", &self.entropy).field("resolution_potential", &self.resolution_potential).finish()
    }
}
