// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::oscillations::Oscillator;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DampingType {
    Undamped,
    Underdamped,
    CriticallyDamped,
    Overdamped,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DampedOscillator {
    pub base: Oscillator,
    pub damping_coefficient: f64,
    pub damping_type: DampingType,
}

impl DampedOscillator {
    pub fn new(base: Oscillator, damping_coefficient: f64) -> Result<Self, crate::DynamicsError> {
        if damping_coefficient < 0.0 {
            return Err(crate::DynamicsError::InvalidDamping(damping_coefficient));
        }
        let damping_type = if damping_coefficient == 0.0 {
            DampingType::Undamped
        } else if damping_coefficient < 1.0 {
            DampingType::Underdamped
        } else if damping_coefficient == 1.0 {
            DampingType::CriticallyDamped
        } else {
            DampingType::Overdamped
        };
        Ok(Self { base, damping_coefficient, damping_type })
    }

    pub fn damped_frequency(&self) -> f64 {
        if self.damping_type == DampingType::Underdamped || self.damping_type == DampingType::Undamped {
            self.base.params.frequency * (1.0 - self.damping_coefficient.powi(2)).sqrt()
        } else {
            0.0
        }
    }
}

impl fmt::Display for DampedOscillator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DampedOscillator({}, damping={:.2}, type={:?})", self.base, self.damping_coefficient, self.damping_type)
    }
}

