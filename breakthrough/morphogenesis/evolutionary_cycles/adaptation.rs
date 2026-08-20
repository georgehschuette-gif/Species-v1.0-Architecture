// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::MorphogenesisError;

use super::*;

pub struct EvolutionaryAdaptation {
    pub fitness: f64,
    pub adaptation_rate: f64,
    pub environmental_pressure: f64,
    pub generations: usize,
}

impl EvolutionaryAdaptation {
    pub fn new(fitness: f64, adaptation_rate: f64) -> Result<Self, MorphogenesisError> {
        if !(0.0..=1.0).contains(&fitness) { return Err(MorphogenesisError::OutOfRange { field: "fitness".into(), value: fitness, min: 0.0, max: 1.0 }); }
        if !(0.0..=1.0).contains(&adaptation_rate) { return Err(MorphogenesisError::OutOfRange { field: "adaptation_rate".into(), value: adaptation_rate, min: 0.0, max: 1.0 }); }
        Ok(Self { fitness, adaptation_rate, environmental_pressure: 0.5, generations: 0 })
    }

    pub fn adapt(&mut self) -> Result<(), MorphogenesisError> {
        self.fitness = (self.fitness + self.adaptation_rate * self.environmental_pressure * 0.1).min(1.0);
        self.generations += 1;
        Ok(())
    }

    pub fn set_pressure(&mut self, pressure: f64) -> Result<(), MorphogenesisError> {
        if !(0.0..=1.0).contains(&pressure) { return Err(MorphogenesisError::OutOfRange { field: "pressure".into(), value: pressure, min: 0.0, max: 1.0 }); }
        self.environmental_pressure = pressure;
        Ok(())
    }
    pub fn is_adapted(&self, threshold: f64) -> bool { self.fitness >= threshold }
    pub fn validate(&self) -> Result<(), MorphogenesisError> {
        if !(0.0..=1.0).contains(&self.fitness) { return Err(MorphogenesisError::OutOfRange { field: "fitness".into(), value: self.fitness, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for EvolutionaryAdaptation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EvolutionaryAdaptation").field("fitness", &self.fitness).field("generations", &self.generations).finish()
    }
}
