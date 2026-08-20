// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::RealityError;

/// Precision: The repeatability of a measurement.
///
/// Precision quantifies the spread of repeated measurements of the
/// same quantity. High precision means low random error.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Precision {
    /// The standard deviation of repeated measurements.
    pub standard_deviation: f64,
    /// The number of samples used to compute this precision.
    pub sample_size: usize,
}

impl Precision {
    /// The maximum acceptable standard deviation for a precise measurement.
    pub const MAX_STD_DEV: f64 = 1.0;

    /// Creates a new precision measure.
    ///
    /// # Errors
    /// Returns `RealityError::InvalidMeasurement` if std_dev is negative or sample_size is zero.
    pub fn new(standard_deviation: f64, sample_size: usize) -> Result<Self, RealityError> {
        if standard_deviation.is_nan() || standard_deviation.is_infinite() {
            return Err(RealityError::InvalidMeasurement {
                value: standard_deviation,
                reason: "standard deviation must be finite".to_string(),
            });
        }
        if standard_deviation < 0.0 {
            return Err(RealityError::InvalidMeasurement {
                value: standard_deviation,
                reason: "standard deviation cannot be negative".to_string(),
            });
        }
        if sample_size == 0 {
            return Err(RealityError::InvalidMeasurement {
                value: 0.0,
                reason: "sample_size must be greater than zero".to_string(),
            });
        }
        Ok(Self {
            standard_deviation,
            sample_size,
        })
    }

    /// Returns whether this precision is considered high (low std dev).
    pub fn is_high(&self, threshold: f64) -> bool {
        self.standard_deviation <= threshold
    }

    /// Returns the coefficient of variation (std_dev / mean).
    pub fn coefficient_of_variation(&self, mean: f64) -> f64 {
        if mean.abs() < f64::EPSILON {
            f64::INFINITY
        } else {
            self.standard_deviation / mean.abs()
        }
    }

    /// Updates the precision estimate with a new sample standard deviation
    /// using online variance (Welford's method approximation).
    pub fn update(&mut self, new_std_dev: f64) -> Result<(), RealityError> {
        let new_precision = Precision::new(new_std_dev, self.sample_size)?;
        self.standard_deviation = (self.standard_deviation + new_precision.standard_deviation) / 2.0;
        self.sample_size += 1;
        Ok(())
    }

    /// Scales the standard deviation by a factor.
    pub fn scale(&self, factor: f64) -> Result<Self, RealityError> {
        let scaled = self.standard_deviation * factor;
        Self::new(scaled, self.sample_size)
    }

    /// Returns the precision as a variance value.
    pub fn variance(&self) -> f64 {
        self.standard_deviation.powi(2)
    }
}

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

