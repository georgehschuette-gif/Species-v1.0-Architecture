// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

use crate::TensionsError;

pub struct PressureRelief {
    pub relief_capacity: f64,
    pub current_load: f64,
    pub efficiency: f64,
}

impl PressureRelief {
    pub fn new(capacity: f64) -> Result<Self, TensionsError> {
        if !(0.0..=1.0).contains(&capacity) { return Err(TensionsError::OutOfRange { field: "capacity".into(), value: capacity, min: 0.0, max: 1.0 }); }
        Ok(Self { relief_capacity: capacity, current_load: 0.0, efficiency: 1.0 })
    }

    pub fn load(&mut self, amount: f64) -> Result<(), TensionsError> {
        self.current_load = (self.current_load + amount).min(1.0);
        Ok(())
    }

    pub fn relieve(&mut self, amount: f64) -> Result<f64, TensionsError> {
        let relieved = (amount * self.efficiency).min(self.current_load);
        self.current_load -= relieved;
        Ok(relieved)
    }

    pub fn overloaded(&self, threshold: f64) -> bool { self.current_load > threshold }
    pub fn efficiency(&self) -> f64 { self.efficiency }
    pub fn set_efficiency(&mut self, eff: f64) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&eff) { return Err(TensionsError::OutOfRange { field: "efficiency".into(), value: eff, min: 0.0, max: 1.0 }); }
        self.efficiency = eff;
        Ok(())
    }
    pub fn validate(&self) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&self.current_load) { return Err(TensionsError::OutOfRange { field: "current_load".into(), value: self.current_load, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for PressureRelief {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PressureRelief").field("capacity", &self.relief_capacity).field("load", &self.current_load).field("efficiency", &self.efficiency).finish()
    }
}
