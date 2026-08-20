// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::MorphogenesisError;

use super::*;

pub struct StructuralStabilization {
    pub rigidity: f64,
    pub connectivity: f64,
    pub redundancy: usize,
}

impl StructuralStabilization {
    pub fn new(rigidity: f64, connectivity: f64) -> Result<Self, MorphogenesisError> {
        if !(0.0..=1.0).contains(&rigidity) || !(0.0..=1.0).contains(&connectivity) {
            return Err(MorphogenesisError::OutOfRange { field: "parameters".into(), value: rigidity.max(connectivity), min: 0.0, max: 1.0 });
        }
        Ok(Self { rigidity, connectivity, redundancy: 1 })
    }

    pub fn reinforce(&mut self, amount: f64) -> Result<(), MorphogenesisError> {
        self.rigidity = (self.rigidity + amount).clamp(0.0, 1.0);
        self.connectivity = (self.connectivity + amount * 0.5).clamp(0.0, 1.0);
        Ok(())
    }

    pub fn add_redundancy(&mut self) -> Result<(), MorphogenesisError> {
        if self.redundancy >= 100 { return Err(MorphogenesisError::CapacityExceeded { max: 100, attempted: self.redundancy + 1 }); }
        self.redundancy += 1;
        Ok(())
    }
    pub fn stability_score(&self) -> f64 { (self.rigidity + self.connectivity) / 2.0 }
    pub fn is_robust(&self, threshold: f64) -> bool { self.stability_score() > threshold }
    pub fn validate(&self) -> Result<(), MorphogenesisError> {
        if !(0.0..=1.0).contains(&self.rigidity) { return Err(MorphogenesisError::OutOfRange { field: "rigidity".into(), value: self.rigidity, min: 0.0, max: 1.0 }); }
        if !(0.0..=1.0).contains(&self.connectivity) { return Err(MorphogenesisError::OutOfRange { field: "connectivity".into(), value: self.connectivity, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for StructuralStabilization {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StructuralStabilization").field("rigidity", &self.rigidity).field("connectivity", &self.connectivity).field("redundancy", &self.redundancy).finish()
    }
}
