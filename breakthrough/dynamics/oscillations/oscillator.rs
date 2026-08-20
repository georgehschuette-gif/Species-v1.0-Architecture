// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OscillatorType {
    Simple,
    Forced,
    Parametric,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OscillationParams {
    pub frequency: f64,
    pub amplitude: f64,
    pub phase: f64,
}

impl OscillationParams {
    pub fn new(frequency: f64, amplitude: f64, phase: f64) -> Result<Self, crate::DynamicsError> {
        if frequency <= 0.0 {
            return Err(crate::DynamicsError::InvalidFrequency(frequency));
        }
        if amplitude < 0.0 {
            return Err(crate::DynamicsError::InvalidAmplitude(amplitude));
        }
        Ok(Self { frequency, amplitude, phase })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Oscillator {
    pub params: OscillationParams,
    pub oscillator_type: OscillatorType,
    pub is_active: bool,
}

impl Oscillator {
    pub fn new(params: OscillationParams, oscillator_type: OscillatorType) -> Self {
        Self { params, oscillator_type, is_active: true }
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn reactivate(&mut self) {
        self.is_active = true;
    }

    pub fn period(&self) -> f64 {
        1.0 / self.params.frequency
    }
}

impl fmt::Display for Oscillator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Oscillator(type={:?}, freq={:.2}Hz, amp={:.2})", self.oscillator_type, self.params.frequency, self.params.amplitude)
    }
}

