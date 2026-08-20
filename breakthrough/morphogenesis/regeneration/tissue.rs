// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::MorphogenesisError;

use super::*;

pub struct TissueRegeneration {
    pub tissue_type: TissueType,
    pub integrity: f64,
    pub scar_tissue: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TissueType {
    Epithelial,
    Connective,
    Muscle,
    Nervous,
}

impl TissueRegeneration {
    pub fn new(tissue_type: TissueType, integrity: f64) -> Result<Self, MorphogenesisError> {
        if !(0.0..=1.0).contains(&integrity) { return Err(MorphogenesisError::OutOfRange { field: "integrity".into(), value: integrity, min: 0.0, max: 1.0 }); }
        Ok(Self { tissue_type, integrity, scar_tissue: 0.0 })
    }

    pub fn heal(&mut self, rate: f64) -> Result<(), MorphogenesisError> {
        if !(0.0..=1.0).contains(&rate) { return Err(MorphogenesisError::OutOfRange { field: "rate".into(), value: rate, min: 0.0, max: 1.0 }); }
        self.integrity = (self.integrity + rate).min(1.0);
        self.scar_tissue = (self.scar_tissue + rate * 0.1).min(1.0);
        Ok(())
    }

    pub fn is_healed(&self) -> bool { self.integrity >= 0.95 }
    pub fn scar_level(&self) -> f64 { self.scar_tissue }
    pub fn validate(&self) -> Result<(), MorphogenesisError> {
        if !(0.0..=1.0).contains(&self.integrity) { return Err(MorphogenesisError::OutOfRange { field: "integrity".into(), value: self.integrity, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

