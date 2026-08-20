// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

use crate::TensionsError;

pub struct NoveltyWeighting {
    pub weight: f64,
    pub familiarity: f64,
    pub recency: f64,
}

impl NoveltyWeighting {
    pub fn new(weight: f64) -> Result<Self, TensionsError> {
        if !(0.0..=1.0).contains(&weight) { return Err(TensionsError::OutOfRange { field: "weight".into(), value: weight, min: 0.0, max: 1.0 }); }
        Ok(Self { weight, familiarity: 1.0 - weight, recency: 1.0 })
    }

    pub fn compute(&self) -> f64 { self.weight * self.recency }
    pub fn update_familiarity(&mut self, delta: f64) -> Result<(), TensionsError> {
        self.familiarity = (self.familiarity + delta).clamp(0.0, 1.0);
        self.weight = 1.0 - self.familiarity;
        Ok(())
    }
    pub fn decay_recency(&mut self, rate: f64) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&rate) { return Err(TensionsError::OutOfRange { field: "rate".into(), value: rate, min: 0.0, max: 1.0 }); }
        self.recency = (self.recency * (1.0 - rate)).max(0.0);
        Ok(())
    }
    pub fn validate(&self) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&self.weight) { return Err(TensionsError::OutOfRange { field: "weight".into(), value: self.weight, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for NoveltyWeighting {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NoveltyWeighting").field("weight", &self.weight).field("familiarity", &self.familiarity).field("recency", &self.recency).finish()
    }
}
