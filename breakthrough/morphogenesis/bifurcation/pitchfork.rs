// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::MorphogenesisError;

use super::*;

pub struct PitchforkBifurcation {
    pub parameter: f64,
    pub branches: Vec<f64>,
    pub symmetry: f64,
}

impl PitchforkBifurcation {
    pub fn new(parameter: f64) -> Self {
        Self { parameter, branches: vec![0.0], symmetry: 1.0 }
    }

    pub fn parameter(&self) -> f64 { self.parameter }
    pub fn update_parameter(&mut self, parameter: f64) { self.parameter = parameter; }
    pub fn split(&mut self) -> Result<(), MorphogenesisError> {
        if self.branches.len() >= 16 { return Err(MorphogenesisError::CapacityExceeded { max: 16, attempted: self.branches.len() + 1 }); }
        let last = *self.branches.last().unwrap();
        self.branches.push(last + 0.1);
        self.branches.insert(0, last - 0.1);
        self.symmetry = 0.5;
        Ok(())
    }
    pub fn branch_count(&self) -> usize { self.branches.len() }
    pub fn is_symmetric(&self) -> bool { self.symmetry > 0.9 }
    pub fn validate(&self) -> Result<(), MorphogenesisError> {
        if self.branches.is_empty() { return Err(MorphogenesisError::MissingInput("branches must not be empty".into())); }
        Ok(())
    }
}

impl fmt::Debug for PitchforkBifurcation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PitchforkBifurcation").field("parameter", &self.parameter).field("branches", &self.branches.len()).finish()
    }
}
