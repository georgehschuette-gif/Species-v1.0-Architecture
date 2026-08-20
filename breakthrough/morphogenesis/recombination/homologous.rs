// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::MorphogenesisError;

use super::*;

pub struct HomologousRecombination {
    pub homology_length: usize,
    pub strand_invasion: bool,
    pub crossover_points: Vec<usize>,
}

impl HomologousRecombination {
    pub fn new(homology_length: usize) -> Self {
        Self { homology_length, strand_invasion: false, crossover_points: Vec::new() }
    }

    pub fn homology_length(&self) -> usize { self.homology_length }
    pub fn invade(&mut self) { self.strand_invasion = true; }
    pub fn add_crossover(&mut self, point: usize) -> Result<(), MorphogenesisError> {
        if self.crossover_points.len() >= 1000 { return Err(MorphogenesisError::CapacityExceeded { max: 1000, attempted: self.crossover_points.len() + 1 }); }
        self.crossover_points.push(point);
        Ok(())
    }
    pub fn crossover_count(&self) -> usize { self.crossover_points.len() }
    pub fn is_active(&self) -> bool { self.strand_invasion }
    pub fn validate(&self) -> Result<(), MorphogenesisError> {
        if self.homology_length == 0 { return Err(MorphogenesisError::MissingInput("homology_length must be > 0".into())); }
        Ok(())
    }
}

impl fmt::Debug for HomologousRecombination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HomologousRecombination").field("homology", &self.homology_length).field("crossover_count", &self.crossover_points.len()).finish()
    }
}
