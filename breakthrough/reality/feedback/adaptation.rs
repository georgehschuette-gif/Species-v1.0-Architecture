// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::RealityError;

/// AdaptationRate: The rate at which a system adapts to feedback.
///
/// Adaptation rates control how quickly the cognitive system
/// updates its models in response to new evidence.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct AdaptationRate(pub f64);

impl AdaptationRate {
    /// Minimum valid adaptation rate.
    pub const MIN: f64 = 0.0;
    /// Maximum valid adaptation rate.
    pub const MAX: f64 = 1.0;
    /// Default adaptation rate.
    pub const DEFAULT: f64 = 0.1;

    /// Creates a new adaptation rate.
    ///
    /// # Errors
    /// Returns `RealityError::OutOfRange` if value is outside [0.0, 1.0].
    pub fn new(value: f64) -> Result<Self, RealityError> {
        if value.is_nan() || value.is_infinite() {
            return Err(RealityError::InvalidMeasurement {
                value,
                reason: "adaptation rate must be finite".to_string(),
            });
        }
        if value < Self::MIN || value > Self::MAX {
            return Err(RealityError::OutOfRange {
                field: "adaptation_rate".to_string(),
                value,
                min: Self::MIN,
                max: Self::MAX,
            });
        }
        Ok(Self(value.clamp(Self::MIN, Self::MAX)))
    }

    /// Creates a default adaptation rate.
    pub fn default_rate() -> Self {
        Self(Self::DEFAULT)
    }

    /// Returns whether this is a fast adaptation rate.
    pub fn is_fast(&self, threshold: f64) -> bool {
        self.0 >= threshold
    }

    /// Returns whether this is a slow adaptation rate.
    pub fn is_slow(&self, threshold: f64) -> bool {
        self.0 <= threshold
    }

    /// Scales this adaptation rate by a multiplier.
    pub fn scale(&self, multiplier: f64) -> Result<Self, RealityError> {
        let scaled = self.0 * multiplier;
        Self::new(scaled)
    }

    /// Returns the decay factor for this rate over a time step.
    pub fn decay_factor(&self, time_step: f64) -> f64 {
        (1.0 - self.0).powf(time_step)
    }
}

impl Default for AdaptationRate {
    fn default() -> Self {
        Self::default_rate()
    }
}

impl std::fmt::Display for AdaptationRate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.4}", self.0)
    }
}

