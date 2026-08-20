// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

use crate::TensionsError;

pub struct CuriositySaturation {
    pub current_level: f64,
    pub threshold: f64,
    pub saturation_curve: f64,
}

impl CuriositySaturation {
    pub fn new(threshold: f64) -> Result<Self, TensionsError> {
        if !(0.0..=1.0).contains(&threshold) { return Err(TensionsError::OutOfRange { field: "threshold".into(), value: threshold, min: 0.0, max: 1.0 }); }
        Ok(Self { current_level: 0.0, threshold, saturation_curve: 2.0 })
    }

    pub fn increase(&mut self, amount: f64) -> Result<bool, TensionsError> {
        self.current_level = (self.current_level + amount).min(1.0);
        Ok(self.current_level >= self.threshold)
    }

    pub fn is_saturated(&self) -> bool { self.current_level >= self.threshold }
    pub fn relief(&mut self, amount: f64) -> Result<(), TensionsError> {
        self.current_level = (self.current_level - amount).max(0.0);
        Ok(())
    }
    pub fn set_curve(&mut self, curve: f64) -> Result<(), TensionsError> {
        if curve <= 0.0 { return Err(TensionsError::OutOfRange { field: "saturation_curve".into(), value: curve, min: 0.0, max: f64::INFINITY }); }
        self.saturation_curve = curve;
        Ok(())
    }
    pub fn validate(&self) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&self.current_level) { return Err(TensionsError::OutOfRange { field: "current_level".into(), value: self.current_level, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for CuriositySaturation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CuriositySaturation").field("level", &self.current_level).field("threshold", &self.threshold).finish()
    }
}
