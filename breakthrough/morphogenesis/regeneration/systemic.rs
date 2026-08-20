// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::MorphogenesisError;

use super::*;

pub struct SystemicRegeneration {
    pub resources: f64,
    pub mobilization_rate: f64,
    pub recovery_index: f64,
}

impl SystemicRegeneration {
    pub fn new(resources: f64) -> Result<Self, MorphogenesisError> {
        if !(0.0..=1.0).contains(&resources) { return Err(MorphogenesisError::OutOfRange { field: "resources".into(), value: resources, min: 0.0, max: 1.0 }); }
        Ok(Self { resources, mobilization_rate: 0.1, recovery_index: 0.0 })
    }

    pub fn mobilize(&mut self, amount: f64) -> Result<(), MorphogenesisError> {
        if !(0.0..=1.0).contains(&amount) { return Err(MorphogenesisError::OutOfRange { field: "amount".into(), value: amount, min: 0.0, max: 1.0 }); }
        self.resources = (self.resources - amount).max(0.0);
        self.recovery_index = (self.recovery_index + amount * self.mobilization_rate).min(1.0);
        Ok(())
    }

    pub fn recover(&mut self, dt: f64) -> Result<(), MorphogenesisError> {
        self.resources = (self.resources + dt * 0.05).min(1.0);
        Ok(())
    }
    pub fn recovery(&self) -> f64 { self.recovery_index }
    pub fn is_recovered(&self, threshold: f64) -> bool { self.recovery_index >= threshold }
    pub fn validate(&self) -> Result<(), MorphogenesisError> {
        if !(0.0..=1.0).contains(&self.resources) { return Err(MorphogenesisError::OutOfRange { field: "resources".into(), value: self.resources, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for SystemicRegeneration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SystemicRegeneration").field("resources", &self.resources).field("recovery", &self.recovery_index).finish()
    }
}
