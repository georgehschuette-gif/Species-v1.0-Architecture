// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::MorphogenesisError;

use super::*;

pub struct StabilizationEnergy {
    pub barrier: f64,
    pub current_energy: f64,
    pub thermal_noise: f64,
}

impl StabilizationEnergy {
    pub fn new(barrier: f64) -> Result<Self, MorphogenesisError> {
        if !(0.0..=1.0).contains(&barrier) { return Err(MorphogenesisError::OutOfRange { field: "barrier".into(), value: barrier, min: 0.0, max: 1.0 }); }
        Ok(Self { barrier, current_energy: 0.0, thermal_noise: 0.1 })
    }

    pub fn barrier(&self) -> f64 { self.barrier }
    pub fn add_energy(&mut self, amount: f64) -> Result<(), MorphogenesisError> {
        self.current_energy = (self.current_energy + amount).min(1.0);
        Ok(())
    }
    pub fn dissipate(&mut self, rate: f64) -> Result<(), MorphogenesisError> {
        if !(0.0..=1.0).contains(&rate) { return Err(MorphogenesisError::OutOfRange { field: "rate".into(), value: rate, min: 0.0, max: 1.0 }); }
        self.current_energy = (self.current_energy * (1.0 - rate)).max(0.0);
        Ok(())
    }
    pub fn is_stable(&self) -> bool { self.current_energy < self.barrier }
    pub fn set_thermal_noise(&mut self, noise: f64) -> Result<(), MorphogenesisError> {
        if !(0.0..=1.0).contains(&noise) { return Err(MorphogenesisError::OutOfRange { field: "thermal_noise".into(), value: noise, min: 0.0, max: 1.0 }); }
        self.thermal_noise = noise;
        Ok(())
    }
    pub fn validate(&self) -> Result<(), MorphogenesisError> {
        if !(0.0..=1.0).contains(&self.current_energy) { return Err(MorphogenesisError::OutOfRange { field: "current_energy".into(), value: self.current_energy, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for StabilizationEnergy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StabilizationEnergy").field("barrier", &self.barrier).field("current", &self.current_energy).finish()
    }
}
