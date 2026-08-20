// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::RealityError;

use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Forecast {
    pub target_time: u64,
    pub predicted_value: f64,
    pub confidence_interval: (f64, f64),
    pub horizon: usize,
}

impl Forecast {
    pub fn new(target_time: u64, predicted_value: f64, confidence_interval: (f64, f64), horizon: usize) -> Self {
        Self {
            target_time,
            predicted_value,
            confidence_interval,
            horizon,
        }
    }

    pub fn confidence_width(&self) -> f64 {
        self.confidence_interval.1 - self.confidence_interval.0
    }

    pub fn is_precise(&self, threshold: f64) -> bool {
        self.confidence_width() <= threshold
    }

    pub fn validate(&self) -> Result<(), RealityError> {
        if self.confidence_interval.0 > self.confidence_interval.1 {
            return Err(RealityError::InvalidState(
                "confidence interval lower bound exceeds upper bound".into(),
            ));
        }
        Ok(())
    }
}

impl fmt::Display for Forecast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Forecast(target={}, value={:.2}, interval=[{:.2}, {:.2}], horizon={})",
            self.target_time,
            self.predicted_value,
            self.confidence_interval.0,
            self.confidence_interval.1,
            self.horizon
        )
    }
}