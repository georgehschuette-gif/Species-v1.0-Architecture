// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::MorphogenesisError;

use super::*;

pub struct EnergyCollapse {
    pub energy_level: f64,
    pub dissipation_rate: f64,
    pub critical_threshold: f64,
}

impl EnergyCollapse {
    pub fn new(energy_level: f64) -> Result<Self, MorphogenesisError> {
        if !(0.0..=1.0).contains(&energy_level) { return Err(MorphogenesisError::OutOfRange { field: "energy_level".into(), value: energy_level, min: 0.0, max: 1.0 }); }
        Ok(Self { energy_level, dissipation_rate: 0.1, critical_threshold: 0.2 })
    }

    pub fn dissipate(&mut self, dt: f64) -> Result<(), MorphogenesisError> {
        self.energy_level = (self.energy_level - self.dissipation_rate * dt).max(0.0);
        Ok(())
    }

    pub fn is_depleted(&self) -> bool { self.energy_level <= self.critical_threshold }
    pub fn inject(&mut self, amount: f64) -> Result<(), MorphogenesisError> {
        self.energy_level = (self.energy_level + amount).min(1.0);
        Ok(())
    }
    pub fn set_dissipation_rate(&mut self, rate: f64) -> Result<(), MorphogenesisError> {
        if !(0.0..=1.0).contains(&rate) { return Err(MorphogenesisError::OutOfRange { field: "dissipation_rate".into(), value: rate, min: 0.0, max: 1.0 }); }
        self.dissipation_rate = rate;
        Ok(())
    }
    pub fn validate(&self) -> Result<(), MorphogenesisError> {
        if !(0.0..=1.0).contains(&self.energy_level) { return Err(MorphogenesisError::OutOfRange { field: "energy_level".into(), value: self.energy_level, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for EnergyCollapse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EnergyCollapse").field("level", &self.energy_level).field("rate", &self.dissipation_rate).finish()
    }
}
