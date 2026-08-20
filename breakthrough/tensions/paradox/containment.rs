// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

use crate::TensionsError;

pub struct ParadoxContainment {
    pub intensity: f64,
    pub boundary_strength: f64,
    pub containment_zone: f64,
}

impl ParadoxContainment {
    pub fn new(intensity: f64) -> Result<Self, TensionsError> {
        if !(0.0..=1.0).contains(&intensity) { return Err(TensionsError::OutOfRange { field: "intensity".into(), value: intensity, min: 0.0, max: 1.0 }); }
        Ok(Self { intensity, boundary_strength: 0.5, containment_zone: 0.0 })
    }

    pub fn intensity(&self) -> f64 { self.intensity }
    pub fn strengthen_boundary(&mut self, amount: f64) -> Result<(), TensionsError> {
        self.boundary_strength = (self.boundary_strength + amount).clamp(0.0, 1.0);
        self.containment_zone = self.intensity * (1.0 - self.boundary_strength);
        Ok(())
    }
    pub fn is_contained(&self, threshold: f64) -> bool { self.containment_zone < threshold }
    pub fn validate(&self) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&self.intensity) { return Err(TensionsError::OutOfRange { field: "intensity".into(), value: self.intensity, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for ParadoxContainment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ParadoxContainment").field("intensity", &self.intensity).field("boundary", &self.boundary_strength).finish()
    }
}
