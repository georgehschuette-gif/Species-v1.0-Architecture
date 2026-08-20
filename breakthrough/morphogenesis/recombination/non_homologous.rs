// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::MorphogenesisError;

use super::*;

pub struct NonHomologousRecombination {
    pub break_sites: Vec<usize>,
    pub joining_sequence: Vec<u8>,
    pub fidelity: f64,
}

impl NonHomologousRecombination {
    pub fn new(break_sites: Vec<usize>, joining_sequence: Vec<u8>) -> Self {
        Self { break_sites, joining_sequence, fidelity: 0.5 }
    }

    pub fn break_sites(&self) -> &[usize] { &self.break_sites }
    pub fn join(&self) -> Vec<u8> { self.joining_sequence.clone() }
    pub fn set_fidelity(&mut self, fidelity: f64) -> Result<(), MorphogenesisError> {
        if !(0.0..=1.0).contains(&fidelity) { return Err(MorphogenesisError::OutOfRange { field: "fidelity".into(), value: fidelity, min: 0.0, max: 1.0 }); }
        self.fidelity = fidelity;
        Ok(())
    }
    pub fn is_error_prone(&self) -> bool { self.fidelity < 0.7 }
    pub fn validate(&self) -> Result<(), MorphogenesisError> {
        if self.break_sites.is_empty() { return Err(MorphogenesisError::MissingInput("break_sites must not be empty".into())); }
        Ok(())
    }
}

impl fmt::Debug for NonHomologousRecombination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NonHomologousRecombination").field("breaks", &self.break_sites.len()).field("fidelity", &self.fidelity).finish()
    }
}
