// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::MorphogenesisError;

use super::*;

pub struct Nucleation {
    pub seed_size: usize,
    pub supersaturation: f64,
    pub critical_size: usize,
}

impl Nucleation {
    pub fn new(seed_size: usize) -> Self {
        Self { seed_size, supersaturation: 1.0, critical_size: seed_size.max(1) }
    }

    pub fn seed_size(&self) -> usize { self.seed_size }
    pub fn set_supersaturation(&mut self, value: f64) -> Result<(), MorphogenesisError> {
        if !(0.0..=1.0).contains(&value) { return Err(MorphogenesisError::OutOfRange { field: "supersaturation".into(), value, min: 0.0, max: 1.0 }); }
        self.supersaturation = value;
        Ok(())
    }
    pub fn is_critical(&self) -> bool { self.seed_size >= self.critical_size }
    pub fn grow(&mut self, amount: usize) -> Result<(), MorphogenesisError> {
        if self.seed_size + amount > 10000 { return Err(MorphogenesisError::CapacityExceeded { max: 10000, attempted: self.seed_size + amount }); }
        self.seed_size += amount;
        Ok(())
    }
    pub fn validate(&self) -> Result<(), MorphogenesisError> {
        if self.seed_size > 10000 { return Err(MorphogenesisError::CapacityExceeded { max: 10000, attempted: self.seed_size }); }
        Ok(())
    }
}

impl fmt::Debug for Nucleation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Nucleation").field("seed_size", &self.seed_size).field("supersaturation", &self.supersaturation).finish()
    }
}
