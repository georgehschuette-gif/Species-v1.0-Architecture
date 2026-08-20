// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::MorphogenesisError;

use super::*;

pub struct EvolutionaryDrift {
    pub allele_frequency: f64,
    pub drift_rate: f64,
    pub population_size: usize,
    pub generation: usize,
}

impl EvolutionaryDrift {
    pub fn new(allele_frequency: f64, population_size: usize) -> Result<Self, MorphogenesisError> {
        if !(0.0..=1.0).contains(&allele_frequency) { return Err(MorphogenesisError::OutOfRange { field: "allele_frequency".into(), value: allele_frequency, min: 0.0, max: 1.0 }); }
        if population_size == 0 { return Err(MorphogenesisError::MissingInput("population_size must be > 0".into())); }
        Ok(Self { allele_frequency, drift_rate: 1.0 / (2.0 * population_size as f64), population_size, generation: 0 })
    }

    pub fn step(&mut self) -> Result<(), MorphogenesisError> {
        let noise = (rand::random::<f64>() - 0.5) * 2.0 * self.drift_rate.sqrt();
        self.allele_frequency = (self.allele_frequency + noise).clamp(0.0, 1.0);
        self.generation += 1;
        Ok(())
    }

    pub fn frequency(&self) -> f64 { self.allele_frequency }
    pub fn generation(&self) -> usize { self.generation }
    pub fn is_fixed(&self) -> bool { self.allele_frequency == 0.0 || self.allele_frequency == 1.0 }
    pub fn validate(&self) -> Result<(), MorphogenesisError> {
        if !(0.0..=1.0).contains(&self.allele_frequency) { return Err(MorphogenesisError::OutOfRange { field: "allele_frequency".into(), value: self.allele_frequency, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for EvolutionaryDrift {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EvolutionaryDrift").field("frequency", &self.allele_frequency).field("generation", &self.generation).finish()
    }
}
