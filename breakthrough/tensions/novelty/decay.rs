// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

use crate::TensionsError;

pub struct NoveltyDecay {
    pub decay_constant: f64,
    pub current_novelty: f64,
}

impl NoveltyDecay {
    pub fn new(decay_constant: f64) -> Result<Self, TensionsError> {
        if !(0.0..=1.0).contains(&decay_constant) { return Err(TensionsError::OutOfRange { field: "decay_constant".into(), value: decay_constant, min: 0.0, max: 1.0 }); }
        Ok(Self { decay_constant, current_novelty: 1.0 })
    }

    pub fn step(&mut self) -> f64 {
        self.current_novelty *= (1.0 - self.decay_constant);
        self.current_novelty.max(0.0)
    }

    pub fn inject(&mut self, amount: f64) -> Result<(), TensionsError> {
        self.current_novelty = (self.current_novelty + amount).min(1.0);
        Ok(())
    }
    pub fn level(&self) -> f64 { self.current_novelty }
    pub fn is_familiar(&self, threshold: f64) -> bool { self.current_novelty < threshold }
    pub fn validate(&self) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&self.current_novelty) { return Err(TensionsError::OutOfRange { field: "current_novelty".into(), value: self.current_novelty, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for NoveltyDecay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NoveltyDecay").field("decay_constant", &self.decay_constant).field("current_novelty", &self.current_novelty).finish()
    }
}
