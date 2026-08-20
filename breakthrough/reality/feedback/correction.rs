// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::RealityError;

/// ErrorCorrection: A correction applied to a model based on observed error.
///
/// Error corrections quantify how much a model's parameters or
/// predictions need to be adjusted to account for observed discrepancies.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ErrorCorrection {
    /// The observed error.
    pub error: f64,
    /// The correction applied.
    pub correction: f64,
    /// The learning rate used for this correction.
    pub learning_rate: f64,
    /// Whether the correction was accepted.
    pub accepted: bool,
}

impl ErrorCorrection {
    /// Creates a new error correction.
    pub fn new(error: f64, learning_rate: f64) -> Self {
        let correction = error * learning_rate;
        Self {
            error,
            correction,
            learning_rate,
            accepted: true,
        }
    }

    /// Returns the corrected value for a given predicted value.
    pub fn corrected_value(&self, predicted: f64) -> f64 {
        predicted - self.correction
    }

    /// Returns the residual error after correction.
    pub fn residual(&self) -> f64 {
        self.error - self.correction
    }

    /// Returns whether the correction is significant.
    pub fn is_significant(&self, threshold: f64) -> bool {
        self.correction.abs() > threshold
    }

    /// Rejects this correction.
    pub fn reject(&mut self) {
        self.accepted = false;
        self.correction = 0.0;
    }
}

