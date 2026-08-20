// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct NegativeFeedback {
    pub damping_factor: f64,
    pub target_value: f64,
    pub current_value: f64,
    pub error: f64,
}

impl NegativeFeedback {
    pub fn new(target_value: f64, damping_factor: f64) -> Self {
        Self { target_value, damping_factor, current_value: target_value, error: 0.0 }
    }

    pub fn update(&mut self, measured: f64) -> Result<(), crate::DynamicsError> {
        self.error = self.target_value - measured;
        self.current_value = measured + self.damping_factor * self.error;
        Ok(())
    }

    pub fn residual_error(&self) -> f64 {
        self.error.abs()
    }
}

impl fmt::Display for NegativeFeedback {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NegativeFeedback(target={:.2}, current={:.2}, error={:.2})", self.target_value, self.current_value, self.error)
    }
}

