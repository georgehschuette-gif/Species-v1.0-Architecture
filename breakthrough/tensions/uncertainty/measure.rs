// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

use crate::TensionsError;

pub struct UncertaintyMeasure {
    pub value: f64,
    pub variance: f64,
}

impl UncertaintyMeasure {
    pub fn new(value: f64) -> Result<Self, TensionsError> {
        if !(0.0..=1.0).contains(&value) { return Err(TensionsError::OutOfRange { field: "value".into(), value, min: 0.0, max: 1.0 }); }
        Ok(Self { value, variance: value * (1.0 - value) })
    }

    pub fn value(&self) -> f64 { self.value }
    pub fn variance(&self) -> f64 { self.variance }
    pub fn confidence(&self) -> f64 { 1.0 - self.value }
    pub fn update(&mut self, observation: f64, weight: f64) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&weight) { return Err(TensionsError::OutOfRange { field: "weight".into(), value: weight, min: 0.0, max: 1.0 }); }
        self.value = (self.value * (1.0 - weight) + observation * weight).clamp(0.0, 1.0);
        self.variance = self.value * (1.0 - self.value);
        Ok(())
    }
    pub fn decay(&mut self, rate: f64) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&rate) { return Err(TensionsError::OutOfRange { field: "rate".into(), value: rate, min: 0.0, max: 1.0 }); }
        self.value = (self.value * (1.0 - rate)).max(0.0);
        self.variance = self.value * (1.0 - self.value);
        Ok(())
    }
    pub fn validate(&self) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&self.value) { return Err(TensionsError::OutOfRange { field: "value".into(), value: self.value, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for UncertaintyMeasure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UncertaintyMeasure").field("value", &self.value).field("variance", &self.variance).finish()
    }
}
