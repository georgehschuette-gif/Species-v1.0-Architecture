// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

use crate::TensionsError;

pub struct PressureField {
    pub dimensions: usize,
    pub values: Vec<f64>,
    pub max_pressure: f64,
}

impl PressureField {
    pub fn new(dimensions: usize) -> Self {
        let values = vec![0.0; dimensions];
        Self { dimensions, values, max_pressure: 0.0 }
    }

    pub fn dimensions(&self) -> usize { self.dimensions }
    pub fn set(&mut self, index: usize, value: f64) -> Result<(), TensionsError> {
        if index >= self.dimensions { return Err(TensionsError::OutOfRange { field: "index".into(), value: index as f64, min: 0.0, max: (self.dimensions - 1) as f64 }); }
        if !(0.0..=1.0).contains(&value) { return Err(TensionsError::OutOfRange { field: "value".into(), value, min: 0.0, max: 1.0 }); }
        self.values[index] = value;
        self.max_pressure = self.values.iter().cloned().fold(0.0, f64::max);
        Ok(())
    }

    pub fn get(&self, index: usize) -> Result<f64, TensionsError> {
        if index >= self.dimensions { return Err(TensionsError::OutOfRange { field: "index".into(), value: index as f64, min: 0.0, max: (self.dimensions - 1) as f64 }); }
        Ok(self.values[index])
    }

    pub fn is_critical(&self, threshold: f64) -> bool { self.max_pressure > threshold }
    pub fn normalize(&mut self) { let sum: f64 = self.values.iter().sum(); if sum > 0.0 { for v in &mut self.values { *v /= sum; } } }
    pub fn validate(&self) -> Result<(), TensionsError> {
        for (i, v) in self.values.iter().enumerate() { if !(0.0..=1.0).contains(v) { return Err(TensionsError::OutOfRange { field: format!("values[{}]", i), value: *v, min: 0.0, max: 1.0 }); } }
        Ok(())
    }
}

impl fmt::Debug for PressureField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PressureField").field("dimensions", &self.dimensions).field("max", &self.max_pressure).finish()
    }
}
