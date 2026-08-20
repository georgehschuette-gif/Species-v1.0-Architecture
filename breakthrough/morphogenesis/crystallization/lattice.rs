// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::MorphogenesisError;

use super::*;

pub struct CrystalLattice {
    pub lattice_type: LatticeType,
    pub unit_cell: Vec<f64>,
    pub dimensions: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LatticeType {
    Cubic,
    Hexagonal,
    Tetragonal,
    Orthorhombic,
}

impl CrystalLattice {
    pub fn new(lattice_type: LatticeType, dimensions: usize) -> Self {
        Self { lattice_type, unit_cell: vec![1.0; dimensions], dimensions }
    }

    pub fn set_parameter(&mut self, index: usize, value: f64) -> Result<(), MorphogenesisError> {
        if index >= self.dimensions { return Err(MorphogenesisError::OutOfRange { field: "index".into(), value: index as f64, min: 0.0, max: (self.dimensions - 1) as f64 }); }
        if value <= 0.0 { return Err(MorphogenesisError::OutOfRange { field: "parameter".into(), value, min: 0.0, max: f64::INFINITY }); }
        self.unit_cell[index] = value;
        Ok(())
    }

    pub fn volume(&self) -> f64 { self.unit_cell.iter().product() }
    pub fn parameter(&self, index: usize) -> Result<f64, MorphogenesisError> {
        if index >= self.dimensions { return Err(MorphogenesisError::OutOfRange { field: "index".into(), value: index as f64, min: 0.0, max: (self.dimensions - 1) as f64 }); }
        Ok(self.unit_cell[index])
    }
    pub fn validate(&self) -> Result<(), MorphogenesisError> {
        for (i, v) in self.unit_cell.iter().enumerate() { if *v <= 0.0 { return Err(MorphogenesisError::OutOfRange { field: format!("parameter[{}]", i), value: *v, min: 0.0, max: f64::INFINITY }); } }
        Ok(())
    }
}

