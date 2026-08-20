// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

use crate::TensionsError;

pub struct CuriosityDrive {
    pub intensity: f64,
    pub direction: f64,
    pub persistence: f64,
}

impl CuriosityDrive {
    pub fn new(intensity: f64) -> Result<Self, TensionsError> {
        if !(0.0..=1.0).contains(&intensity) { return Err(TensionsError::OutOfRange { field: "intensity".into(), value: intensity, min: 0.0, max: 1.0 }); }
        Ok(Self { intensity, direction: 0.0, persistence: 1.0 })
    }

    pub fn intensity(&self) -> f64 { self.intensity }
    pub fn orient(&mut self, direction: f64) -> Result<(), TensionsError> {
        if !(-1.0..=1.0).contains(&direction) { return Err(TensionsError::OutOfRange { field: "direction".into(), value: direction, min: -1.0, max: 1.0 }); }
        self.direction = direction;
        Ok(())
    }
    pub fn fatigue(&mut self, amount: f64) -> Result<(), TensionsError> {
        self.persistence = (self.persistence - amount).max(0.0);
        self.intensity = (self.intensity * self.persistence).clamp(0.0, 1.0);
        Ok(())
    }
    pub fn restore(&mut self, amount: f64) -> Result<(), TensionsError> {
        self.persistence = (self.persistence + amount).min(1.0);
        Ok(())
    }
    pub fn validate(&self) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&self.intensity) { return Err(TensionsError::OutOfRange { field: "intensity".into(), value: self.intensity, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for CuriosityDrive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CuriosityDrive").field("intensity", &self.intensity).field("direction", &self.direction).field("persistence", &self.persistence).finish()
    }
}
