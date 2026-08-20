// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

use crate::TensionsError;

pub struct ParadoxSuspension {
    pub suspended: bool,
    pub suspension_strength: f64,
    pub time_remaining: f64,
}

impl ParadoxSuspension {
    pub fn new() -> Self {
        Self { suspended: false, suspension_strength: 0.0, time_remaining: 0.0 }
    }

    pub fn suspend(&mut self, strength: f64, duration: f64) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&strength) { return Err(TensionsError::OutOfRange { field: "strength".into(), value: strength, min: 0.0, max: 1.0 }); }
        if duration <= 0.0 { return Err(TensionsError::OutOfRange { field: "duration".into(), value: duration, min: 0.0, max: f64::INFINITY }); }
        self.suspended = true;
        self.suspension_strength = strength;
        self.time_remaining = duration;
        Ok(())
    }

    pub fn tick(&mut self, dt: f64) -> Result<bool, TensionsError> {
        if !self.suspended { return Ok(false); }
        self.time_remaining -= dt;
        if self.time_remaining <= 0.0 {
            self.suspended = false;
            self.suspension_strength = 0.0;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn is_suspended(&self) -> bool { self.suspended }
    pub fn release(&mut self) { self.suspended = false; self.suspension_strength = 0.0; self.time_remaining = 0.0; }
    pub fn validate(&self) -> Result<(), TensionsError> {
        if self.suspended && !(0.0..=1.0).contains(&self.suspension_strength) { return Err(TensionsError::OutOfRange { field: "suspension_strength".into(), value: self.suspension_strength, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for ParadoxSuspension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ParadoxSuspension").field("suspended", &self.suspended).field("strength", &self.suspension_strength).field("time", &self.time_remaining).finish()
    }
}
