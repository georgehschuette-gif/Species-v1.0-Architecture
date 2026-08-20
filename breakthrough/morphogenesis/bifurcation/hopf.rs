// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::MorphogenesisError;

use super::*;

pub struct HopfBifurcation {
    pub parameter: f64,
    pub frequency: f64,
    pub amplitude: f64,
    pub limit_cycle: bool,
}

impl HopfBifurcation {
    pub fn new(parameter: f64) -> Self {
        Self { parameter, frequency: 1.0, amplitude: 0.0, limit_cycle: false }
    }

    pub fn parameter(&self) -> f64 { self.parameter }
    pub fn oscillate(&mut self, amplitude: f64) -> Result<(), MorphogenesisError> {
        if !(0.0..=1.0).contains(&amplitude) { return Err(MorphogenesisError::OutOfRange { field: "amplitude".into(), value: amplitude, min: 0.0, max: 1.0 }); }
        self.amplitude = amplitude;
        self.limit_cycle = amplitude > 0.1;
        Ok(())
    }
    pub fn frequency(&self) -> f64 { self.frequency }
    pub fn set_frequency(&mut self, freq: f64) -> Result<(), MorphogenesisError> {
        if freq <= 0.0 { return Err(MorphogenesisError::OutOfRange { field: "frequency".into(), value: freq, min: 0.0, max: f64::INFINITY }); }
        self.frequency = freq;
        Ok(())
    }
    pub fn has_limit_cycle(&self) -> bool { self.limit_cycle }
    pub fn validate(&self) -> Result<(), MorphogenesisError> {
        if !(0.0..=1.0).contains(&self.amplitude) { return Err(MorphogenesisError::OutOfRange { field: "amplitude".into(), value: self.amplitude, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for HopfBifurcation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HopfBifurcation").field("parameter", &self.parameter).field("amplitude", &self.amplitude).field("limit_cycle", &self.limit_cycle).finish()
    }
}
