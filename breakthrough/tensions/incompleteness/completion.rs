// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

use crate::TensionsError;

pub struct IncompletenessCompletion {
    pub progress: f64,
    pub estimated_remaining: f64,
    pub steps: usize,
}

impl IncompletenessCompletion {
    pub fn new() -> Self {
        Self { progress: 0.0, estimated_remaining: 1.0, steps: 0 }
    }

    pub fn advance(&mut self, amount: f64) -> Result<(), TensionsError> {
        self.progress = (self.progress + amount).min(1.0);
        self.estimated_remaining = 1.0 - self.progress;
        self.steps += 1;
        Ok(())
    }

    pub fn progress(&self) -> f64 { self.progress }
    pub fn is_complete(&self) -> bool { self.progress >= 1.0 }
    pub fn estimate_remaining(&self) -> f64 { self.estimated_remaining }
    pub fn validate(&self) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&self.progress) { return Err(TensionsError::OutOfRange { field: "progress".into(), value: self.progress, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for IncompletenessCompletion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IncompletenessCompletion").field("progress", &self.progress).field("remaining", &self.estimated_remaining).finish()
    }
}
