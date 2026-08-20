// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

use crate::TensionsError;

pub struct UncertaintyReduction {
    pub reduction_rate: f64,
    pub current_uncertainty: f64,
    pub steps: usize,
}

impl UncertaintyReduction {
    pub fn new(rate: f64) -> Result<Self, TensionsError> {
        if !(0.0..=1.0).contains(&rate) { return Err(TensionsError::OutOfRange { field: "reduction_rate".into(), value: rate, min: 0.0, max: 1.0 }); }
        Ok(Self { reduction_rate: rate, current_uncertainty: 1.0, steps: 0 })
    }

    pub fn step(&mut self) -> Result<f64, TensionsError> {
        self.current_uncertainty = (self.current_uncertainty * (1.0 - self.reduction_rate)).max(0.0);
        self.steps += 1;
        Ok(self.current_uncertainty)
    }

    pub fn uncertainty(&self) -> f64 { self.current_uncertainty }
    pub fn is_certain(&self, threshold: f64) -> bool { self.current_uncertainty < threshold }
    pub fn reset(&mut self) { self.current_uncertainty = 1.0; self.steps = 0; }
    pub fn set_rate(&mut self, rate: f64) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&rate) { return Err(TensionsError::OutOfRange { field: "reduction_rate".into(), value: rate, min: 0.0, max: 1.0 }); }
        self.reduction_rate = rate;
        Ok(())
    }
    pub fn validate(&self) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&self.current_uncertainty) { return Err(TensionsError::OutOfRange { field: "current_uncertainty".into(), value: self.current_uncertainty, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for UncertaintyReduction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UncertaintyReduction").field("rate", &self.reduction_rate).field("uncertainty", &self.current_uncertainty).field("steps", &self.steps).finish()
    }
}
