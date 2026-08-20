// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::MorphogenesisError;

use super::*;

pub struct StructuralCollapse {
    pub strain: f64,
    pub integrity: f64,
    pub collapse_type: CollapseType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CollapseType {
    Brittle,
    Ductile,
    Catastrophic,
    Partial,
}

impl StructuralCollapse {
    pub fn new(strain: f64) -> Result<Self, MorphogenesisError> {
        if !(0.0..=1.0).contains(&strain) { return Err(MorphogenesisError::OutOfRange { field: "strain".into(), value: strain, min: 0.0, max: 1.0 }); }
        let collapse_type = match strain {
            s if s > 0.9 => CollapseType::Catastrophic,
            s if s > 0.7 => CollapseType::Brittle,
            s if s > 0.4 => CollapseType::Ductile,
            _ => CollapseType::Partial,
        };
        Ok(Self { strain, integrity: 1.0 - strain, collapse_type })
    }

    pub fn strain(&self) -> f64 { self.strain }
    pub fn propagate(&mut self, additional_strain: f64) -> Result<(), MorphogenesisError> {
        self.strain = (self.strain + additional_strain).min(1.0);
        self.integrity = 1.0 - self.strain;
        Ok(())
    }
    pub fn is_collapsed(&self) -> bool { self.strain >= 0.8 }
    pub fn collapse_type(&self) -> CollapseType { self.collapse_type }
    pub fn validate(&self) -> Result<(), MorphogenesisError> {
        if !(0.0..=1.0).contains(&self.strain) { return Err(MorphogenesisError::OutOfRange { field: "strain".into(), value: self.strain, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

