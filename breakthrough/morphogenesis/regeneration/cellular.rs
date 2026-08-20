// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::MorphogenesisError;

use super::*;

pub struct CellularRegeneration {
    pub capacity: f64,
    pub proliferation_rate: f64,
    pub differentiation: f64,
}

impl CellularRegeneration {
    pub fn new(capacity: f64) -> Result<Self, MorphogenesisError> {
        if !(0.0..=1.0).contains(&capacity) { return Err(MorphogenesisError::OutOfRange { field: "capacity".into(), value: capacity, min: 0.0, max: 1.0 }); }
        Ok(Self { capacity, proliferation_rate: 0.1, differentiation: 0.0 })
    }

    pub fn capacity(&self) -> f64 { self.capacity }
    pub fn proliferate(&mut self, dt: f64) -> Result<(), MorphogenesisError> {
        self.differentiation = (self.differentiation + self.proliferation_rate * dt).min(1.0);
        Ok(())
    }
    pub fn differentiate(&mut self, lineage: CellLineage) {
        match lineage {
            CellLineage::Stem => self.differentiation *= 0.5,
            CellLineage::Progenitor => self.differentiation *= 0.8,
            CellLineage::Terminal => self.differentiation = 1.0,
        }
    }
    pub fn is_differentiated(&self) -> bool { self.differentiation >= 0.9 }
    pub fn validate(&self) -> Result<(), MorphogenesisError> {
        if !(0.0..=1.0).contains(&self.capacity) { return Err(MorphogenesisError::OutOfRange { field: "capacity".into(), value: self.capacity, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CellLineage {
    Stem,
    Progenitor,
    Terminal,
}

