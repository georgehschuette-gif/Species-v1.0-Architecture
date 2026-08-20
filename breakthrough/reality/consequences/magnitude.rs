// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::RealityError;

/// EffectMagnitude: The size of a consequence's effect.
///
/// Magnitude quantifies how large or significant a consequence is,
/// relative to some reference or absolute scale.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct EffectMagnitude(pub f64);

impl EffectMagnitude {
    /// Minimum valid magnitude.
    pub const MIN: f64 = 0.0;
    /// Maximum valid magnitude.
    pub const MAX: f64 = 100.0;

    /// Creates a new effect magnitude.
    ///
    /// # Errors
    /// Returns `RealityError::OutOfRange` if value is outside [0.0, 100.0].
    pub fn new(value: f64) -> Result<Self, RealityError> {
        if value.is_nan() || value.is_infinite() {
            return Err(RealityError::InvalidMeasurement {
                value,
                reason: "magnitude must be finite".to_string(),
            });
        }
        if value < Self::MIN || value > Self::MAX {
            return Err(RealityError::OutOfRange {
                field: "magnitude".to_string(),
                value,
                min: Self::MIN,
                max: Self::MAX,
            });
        }
        Ok(Self(value.clamp(Self::MIN, Self::MAX)))
    }

    /// Returns whether this magnitude is negligible.
    pub fn is_negligible(&self, threshold: f64) -> bool {
        self.0 < threshold
    }

    /// Returns whether this magnitude is significant.
    pub fn is_significant(&self, threshold: f64) -> bool {
        self.0 >= threshold
    }

    /// Returns the normalized magnitude (0.0 to 1.0).
    pub fn normalized(&self) -> f64 {
        self.0 / Self::MAX
    }

    /// Combines two magnitudes (assuming additive effects).
    pub fn combine(&self, other: &Self) -> Self {
        Self((self.0 + other.0).min(Self::MAX))
    }

    /// Scales the magnitude by a factor.
    pub fn scale(&self, factor: f64) -> Result<Self, RealityError> {
        let scaled = self.0 * factor;
        Self::new(scaled)
    }
}

impl Default for EffectMagnitude {
    fn default() -> Self {
        Self(0.0)
    }
}

impl std::fmt::Display for EffectMagnitude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2}", self.0)
    }
}

