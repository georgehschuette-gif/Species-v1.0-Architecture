// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;
use crate::TensionsError;

pub struct ContradictionBalance {
    pub positive_tension: f64,
    pub negative_tension: f64,
    pub equilibrium: f64,
}

impl ContradictionBalance {
    pub fn new(positive: f64, negative: f64) -> Result<Self, TensionsError> {
        if !(0.0..=1.0).contains(&positive) || !(0.0..=1.0).contains(&negative) {
            return Err(TensionsError::OutOfRange { field: "tensions".into(), value: positive.max(negative), min: 0.0, max: 1.0 });
        }
        Ok(Self { positive_tension: positive, negative_tension: negative, equilibrium: 0.5 })
    }

    pub fn shift(&mut self, delta: f64) -> Result<(), TensionsError> {
        self.positive_tension = (self.positive_tension + delta).clamp(0.0, 1.0);
        self.negative_tension = (self.negative_tension - delta).clamp(0.0, 1.0);
        self.equilibrium = (self.positive_tension + self.negative_tension) / 2.0;
        Ok(())
    }

    pub fn is_balanced(&self, tolerance: f64) -> bool {
        (self.positive_tension - self.negative_tension).abs() <= tolerance
    }

    pub fn net_tension(&self) -> f64 { (self.positive_tension - self.negative_tension).abs() }
    pub fn set_equilibrium(&mut self, eq: f64) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&eq) { return Err(TensionsError::OutOfRange { field: "equilibrium".into(), value: eq, min: 0.0, max: 1.0 }); }
        self.equilibrium = eq;
        Ok(())
    }
    pub fn validate(&self) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&self.positive_tension) { return Err(TensionsError::OutOfRange { field: "positive_tension".into(), value: self.positive_tension, min: 0.0, max: 1.0 }); }
        if !(0.0..=1.0).contains(&self.negative_tension) { return Err(TensionsError::OutOfRange { field: "negative_tension".into(), value: self.negative_tension, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for ContradictionBalance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ContradictionBalance").field("positive", &self.positive_tension).field("negative", &self.negative_tension).field("equilibrium", &self.equilibrium).finish()
    }
}
