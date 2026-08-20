// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::collections::HashMap;
use std::fmt;

use crate::RealityError;

/// ParameterAdjustment: An adjustment made to a model parameter during calibration.
#[derive(Debug, Clone, PartialEq)]
pub struct ParameterAdjustment {
    /// Name of the parameter being adjusted.
    pub parameter: String,
    /// Original value before adjustment.
    pub original_value: f64,
    /// New value after adjustment.
    pub new_value: f64,
    /// Reason for the adjustment.
    pub reason: String,
    /// Improvement in model fit (positive = improved).
    pub improvement: f64,
}

impl ParameterAdjustment {
    /// Creates a new parameter adjustment.
    pub fn new(
        parameter: impl Into<String>,
        original_value: f64,
        new_value: f64,
        reason: impl Into<String>,
        improvement: f64,
    ) -> Self {
        Self {
            parameter: parameter.into(),
            original_value,
            new_value,
            reason: reason.into(),
            improvement,
        }
    }

    /// Returns the magnitude of the adjustment.
    pub fn magnitude(&self) -> f64 {
        (self.new_value - self.original_value).abs()
    }

    /// Returns whether this adjustment improved the model.
    pub fn is_improvement(&self) -> bool {
        self.improvement > 0.0
    }
}

