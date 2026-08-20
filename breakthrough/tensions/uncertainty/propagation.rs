// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

use crate::TensionsError;

pub struct UncertaintyPropagation {
    pub propagated_value: f64,
    pub accumulated_variance: f64,
    pub depth: usize,
}

impl UncertaintyPropagation {
    pub fn from_source(source: &UncertaintyMeasure) -> Self {
        Self { propagated_value: source.value, accumulated_variance: source.variance, depth: 1 }
    }

    pub fn through(&mut self, noise: f64) -> Result<(), TensionsError> {
        self.accumulated_variance += noise;
        self.propagated_value = (self.propagated_value + noise * 0.1).clamp(0.0, 1.0);
        self.depth += 1;
        Ok(())
    }

    pub fn propagated(&self) -> f64 { self.propagated_value }
    pub fn total_variance(&self) -> f64 { self.accumulated_variance }
    pub fn depth_level(&self) -> usize { self.depth }
    pub fn is_bounded(&self, threshold: f64) -> bool { self.accumulated_variance < threshold }
    pub fn validate(&self) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&self.propagated_value) { return Err(TensionsError::OutOfRange { field: "propagated_value".into(), value: self.propagated_value, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for UncertaintyPropagation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UncertaintyPropagation").field("value", &self.propagated_value).field("variance", &self.accumulated_variance).field("depth", &self.depth).finish()
    }
}
