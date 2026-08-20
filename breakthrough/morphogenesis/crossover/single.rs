// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::MorphogenesisError;

use super::*;

pub struct SingleCrossover {
    pub point: usize,
    pub offspring_a: Vec<u8>,
    pub offspring_b: Vec<u8>,
}

impl SingleCrossover {
    pub fn new(point: usize) -> Self {
        Self { point, offspring_a: Vec::new(), offspring_b: Vec::new() }
    }

    pub fn point(&self) -> usize { self.point }
    pub fn recombine(&mut self, parent_a: &[u8], parent_b: &[u8]) -> Result<(), MorphogenesisError> {
        if self.point > parent_a.len().min(parent_b.len()) {
            return Err(MorphogenesisError::OutOfRange { field: "point".into(), value: self.point as f64, min: 0.0, max: parent_a.len().min(parent_b.len()) as f64 });
        }
        self.offspring_a = parent_a[..self.point].iter().chain(parent_b[self.point..].iter()).cloned().collect();
        self.offspring_b = parent_b[..self.point].iter().chain(parent_a[self.point..].iter()).cloned().collect();
        Ok(())
    }
    pub fn offspring_a(&self) -> &[u8] { &self.offspring_a }
    pub fn offspring_b(&self) -> &[u8] { &self.offspring_b }
    pub fn set_point(&mut self, point: usize) -> Result<(), MorphogenesisError> {
        if self.offspring_a.len() > 0 && point > self.offspring_a.len() {
            return Err(MorphogenesisError::OutOfRange { field: "point".into(), value: point as f64, min: 0.0, max: self.offspring_a.len() as f64 });
        }
        self.point = point;
        Ok(())
    }
    pub fn validate(&self) -> Result<(), MorphogenesisError> {
        if self.offspring_a.len() != self.offspring_b.len() { return Err(MorphogenesisError::DimensionMismatch { expected: self.offspring_b.len(), actual: self.offspring_a.len() }); }
        Ok(())
    }
}

impl fmt::Debug for SingleCrossover {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SingleCrossover").field("point", &self.point).field("offspring_a_len", &self.offspring_a.len()).field("offspring_b_len", &self.offspring_b.len()).finish()
    }
}
