// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

use crate::TensionsError;

pub struct CuriosityReward {
    pub predicted_value: f64,
    pub actual_value: f64,
    pub learning_signal: f64,
}

impl CuriosityReward {
    pub fn new(predicted: f64, actual: f64) -> Result<Self, TensionsError> {
        if !(0.0..=1.0).contains(&predicted) || !(0.0..=1.0).contains(&actual) {
            return Err(TensionsError::OutOfRange { field: "values".into(), value: predicted.max(actual), min: 0.0, max: 1.0 });
        }
        let diff = (actual - predicted).abs();
        Ok(Self { predicted_value: predicted, actual_value: actual, learning_signal: diff })
    }

    pub fn signal(&self) -> f64 { self.learning_signal }
    pub fn error(&self) -> f64 { (self.actual_value - self.predicted_value).abs() }
    pub fn update(&mut self, actual: f64) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&actual) { return Err(TensionsError::OutOfRange { field: "actual".into(), value: actual, min: 0.0, max: 1.0 }); }
        self.actual_value = actual;
        self.learning_signal = (actual - self.predicted_value).abs();
        Ok(())
    }
    pub fn reinforce(&mut self, amount: f64) -> Result<(), TensionsError> {
        self.predicted_value = (self.predicted_value + amount).clamp(0.0, 1.0);
        Ok(())
    }
    pub fn validate(&self) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&self.learning_signal) { return Err(TensionsError::OutOfRange { field: "learning_signal".into(), value: self.learning_signal, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Debug for CuriosityReward {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CuriosityReward").field("predicted", &self.predicted_value).field("actual", &self.actual_value).field("signal", &self.learning_signal).finish()
    }
}
