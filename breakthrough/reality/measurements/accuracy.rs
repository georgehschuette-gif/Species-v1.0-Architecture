// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::RealityError;

/// Accuracy: The closeness of a measurement to the true value.
///
/// Accuracy quantifies systematic error. High accuracy means low bias.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Accuracy {
    /// The bias (systematic error) of the measurement.
    pub bias: f64,
    /// The confidence interval half-width.
    pub confidence_interval: f64,
    /// The confidence level (e.g., 0.95 for 95% CI).
    pub confidence_level: f64,
}

impl Accuracy {
    /// The maximum acceptable bias for an accurate measurement.
    pub const MAX_BIAS: f64 = 0.1;

    /// Creates a new accuracy measure.
    ///
    /// # Errors
    /// Returns `RealityError::InvalidMeasurement` if parameters are invalid.
    pub fn new(bias: f64, confidence_interval: f64, confidence_level: f64) -> Result<Self, RealityError> {
        if bias.is_nan() || bias.is_infinite() {
            return Err(RealityError::InvalidMeasurement {
                value: bias,
                reason: "bias must be finite".to_string(),
            });
        }
        if confidence_interval.is_nan() || confidence_interval.is_infinite() || confidence_interval < 0.0 {
            return Err(RealityError::InvalidMeasurement {
                value: confidence_interval,
                reason: "confidence_interval must be non-negative and finite".to_string(),
            });
        }
        if confidence_level <= 0.0 || confidence_level >= 1.0 {
            return Err(RealityError::InvalidMeasurement {
                value: confidence_level,
                reason: "confidence_level must be in (0.0, 1.0)".to_string(),
            });
        }
        Ok(Self {
            bias,
            confidence_interval,
            confidence_level,
        })
    }

    /// Returns whether this accuracy is considered high (low bias).
    pub fn is_high(&self, threshold: f64) -> bool {
        self.bias.abs() <= threshold
    }

    /// Returns the relative bias (bias / true value).
    pub fn relative_bias(&self, true_value: f64) -> f64 {
        if true_value.abs() < f64::EPSILON {
            f64::INFINITY
        } else {
            self.bias / true_value.abs()
        }
    }

    /// Returns whether the true value falls within the confidence interval.
    pub fn contains_true_value(&self, measured: f64, true_value: f64) -> bool {
        let lower = measured - self.confidence_interval;
        let upper = measured + self.confidence_interval;
        true_value >= lower && true_value <= upper
    }

    /// Updates the bias estimate.
    pub fn update_bias(&mut self, new_bias: f64) -> Result<(), RealityError> {
        if new_bias.is_nan() || new_bias.is_infinite() {
            return Err(RealityError::InvalidMeasurement {
                value: new_bias,
                reason: "bias must be finite".to_string(),
            });
        }
        self.bias = (self.bias + new_bias) / 2.0;
        Ok(())
    }
}

