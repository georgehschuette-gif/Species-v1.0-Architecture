// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::MorphogenesisError;

use super::*;

pub struct UniformCrossover {
    pub mask: Vec<bool>,
    pub offspring_a: Vec<u8>,
    pub offspring_b: Vec<u8>,
}

impl UniformCrossover {
    pub fn new(mask: Vec<bool>) -> Result<Self, MorphogenesisError> {
        if mask.is_empty() { return Err(MorphogenesisError::MissingInput("mask must not be empty".into())); }
        Ok(Self { mask, offspring_a: Vec::new(), offspring_b: Vec::new() })
    }

    pub fn recombine(&mut self, parent_a: &[u8], parent_b: &[u8]) -> Result<(), MorphogenesisError> {
        if self.mask.len() != parent_a.len().max(parent_b.len()) {
            return Err(MorphogenesisError::DimensionMismatch { expected: parent_a.len().max(parent_b.len()), actual: self.mask.len() });
        }
        let len = parent_a.len().min(parent_b.len());
        self.offspring_a = (0..len).map(|i| if self.mask[i] { parent_a[i] } else { parent_b[i] }).collect();
        self.offspring_b = (0..len).map(|i| if self.mask[i] { parent_b[i] } else { parent_a[i] }).collect();
        Ok(())
    }

    pub fn mask_density(&self) -> f64 {
        if self.mask.is_empty() { return 0.0; }
        self.mask.iter().filter(|&&m| m).count() as f64 / self.mask.len() as f64
    }
    pub fn validate(&self) -> Result<(), MorphogenesisError> {
        if self.offspring_a.len() != self.offspring_b.len() { return Err(MorphogenesisError::DimensionMismatch { expected: self.offspring_b.len(), actual: self.offspring_a.len() }); }
        Ok(())
    }
}

impl fmt::Debug for UniformCrossover {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UniformCrossover").field("mask_density", &self.mask_density()).finish()
    }
}
