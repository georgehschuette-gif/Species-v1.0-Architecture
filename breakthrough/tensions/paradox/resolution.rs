// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

use crate::TensionsError;

pub struct ParadoxResolution {
    pub resolution_type: ParadoxResolutionType,
    pub coherence_gain: f64,
    pub energy_cost: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParadoxResolutionType {
    Reframe,
    Accept,
    Transcend,
    Dissolve,
}

impl ParadoxResolution {
    pub fn new(resolution_type: ParadoxResolutionType) -> Self {
        Self { resolution_type, coherence_gain: 0.0, energy_cost: 0.0 }
    }

    pub fn apply(&mut self, paradox: &ParadoxContainment) -> Result<f64, TensionsError> {
        self.coherence_gain = match self.resolution_type {
            ParadoxResolutionType::Reframe => 0.4,
            ParadoxResolutionType::Accept => 0.6,
            ParadoxResolutionType::Transcend => 0.9,
            ParadoxResolutionType::Dissolve => 1.0,
        };
        self.energy_cost = paradox.intensity * 0.5;
        Ok(self.coherence_gain)
    }

    pub fn coherence(&self) -> f64 { self.coherence_gain }
    pub fn cost(&self) -> f64 { self.energy_cost }
    pub fn set_type(&mut self, resolution_type: ParadoxResolutionType) { self.resolution_type = resolution_type; }
    pub fn validate(&self) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&self.coherence_gain) { return Err(TensionsError::OutOfRange { field: "coherence_gain".into(), value: self.coherence_gain, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

