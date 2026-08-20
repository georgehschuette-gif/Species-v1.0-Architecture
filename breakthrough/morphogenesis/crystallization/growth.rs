// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::MorphogenesisError;

use super::*;

pub struct CrystalGrowth {
    pub size: usize,
    pub growth_rate: f64,
    pub morphology: CrystalMorphology,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CrystalMorphology {
    Isometric,
    Tabular,
    Acicular,
    Bladed,
}

impl CrystalGrowth {
    pub fn new(size: usize, growth_rate: f64) -> Result<Self, MorphogenesisError> {
        if !(0.0..=1.0).contains(&growth_rate) { return Err(MorphogenesisError::OutOfRange { field: "growth_rate".into(), value: growth_rate, min: 0.0, max: 1.0 }); }
        Ok(Self { size, growth_rate, morphology: CrystalMorphology::Isometric })
    }

    pub fn grow(&mut self, dt: f64) -> Result<(), MorphogenesisError> {
        let increment = (self.growth_rate * dt) as usize;
        self.size = (self.size + increment).min(10000);
        Ok(())
    }

    pub fn set_morphology(&mut self, morphology: CrystalMorphology) { self.morphology = morphology; }
    pub fn size(&self) -> usize { self.size }
    pub fn is_finished(&self) -> bool { self.size >= 10000 }
    pub fn validate(&self) -> Result<(), MorphogenesisError> {
        if !(0.0..=1.0).contains(&self.growth_rate) { return Err(MorphogenesisError::OutOfRange { field: "growth_rate".into(), value: self.growth_rate, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

