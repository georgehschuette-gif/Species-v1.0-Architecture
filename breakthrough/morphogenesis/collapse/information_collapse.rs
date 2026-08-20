// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::MorphogenesisError;

use super::*;

pub struct InformationCollapse {
    pub information_content: f64,
    pub entropy: f64,
    pub redundancy: f64,
}

impl InformationCollapse {
    pub fn new(information_content: f64) -> Result<Self, MorphogenesisError> {
        if !(0.0..=1.0).contains(&information_content) { return Err(MorphogenesisError::OutOfRange { field: "information_content".into(), value: information_content, min: 0.0, max: 1.0 }); }
        Ok(Self { information_content, entropy: 1.0 - information_content, redundancy: 0.0 })
    }

    pub fn degrade(&mut self, noise: f64) -> Result<(), MorphogenesisError> {
        self.information_content = (self.information_content - noise * 0.1).max(0.0);
        self.entropy = 1.0 - self.information_content;
        Ok(())
    }

    pub fn is_corrupted(&self, threshold: f64) -> bool { self.information_content < threshold }
    pub fn recover(&mut self, source_information: f64) -> Result<(), MorphogenesisError> {
        self.information_content = (self.information_content + source_information * 0.2).min(1.0);
        self.entropy = 1.0 - self.information_content;
        Ok(())
    }
    pub fn validate(&self) -> Result<(), MorphogenesisError> {
        if !(0.0..=1.0).contains(&self.information_content) { return Err(MorphogenesisError::OutOfRange { field: "information_content".into(), value: self.information_content, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for InformationCollapse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InformationCollapse").field("content", &self.information_content).field("entropy", &self.entropy).finish()
    }
}
