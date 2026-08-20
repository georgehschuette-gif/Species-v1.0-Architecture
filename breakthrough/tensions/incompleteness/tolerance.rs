// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

use crate::TensionsError;

pub struct IncompletenessTolerance {
    pub tolerance: f64,
    pub acceptable_gap: f64,
    pub anxiety: f64,
}

impl IncompletenessTolerance {
    pub fn new(tolerance: f64) -> Result<Self, TensionsError> {
        if !(0.0..=1.0).contains(&tolerance) { return Err(TensionsError::OutOfRange { field: "tolerance".into(), value: tolerance, min: 0.0, max: 1.0 }); }
        Ok(Self { tolerance, acceptable_gap: tolerance, anxiety: 0.0 })
    }

    pub fn assess(&mut self, gap: f64) -> Result<(), TensionsError> {
        if gap > self.acceptable_gap {
            self.anxiety = (self.anxiety + (gap - self.acceptable_gap)).min(1.0);
        } else {
            self.anxiety = (self.anxiety * 0.9).max(0.0);
        }
        Ok(())
    }

    pub fn anxiety_level(&self) -> f64 { self.anxiety }
    pub fn is_tolerable(&self, gap: f64) -> bool { gap <= self.acceptable_gap }
    pub fn set_tolerance(&mut self, tolerance: f64) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&tolerance) { return Err(TensionsError::OutOfRange { field: "tolerance".into(), value: tolerance, min: 0.0, max: 1.0 }); }
        self.tolerance = tolerance;
        self.acceptable_gap = tolerance;
        Ok(())
    }
    pub fn validate(&self) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&self.tolerance) { return Err(TensionsError::OutOfRange { field: "tolerance".into(), value: self.tolerance, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for IncompletenessTolerance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IncompletenessTolerance").field("tolerance", &self.tolerance).field("anxiety", &self.anxiety).finish()
    }
}
