// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// EmotionalValence: Positive-negative affective axis.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EmotionalValence(pub f64);

impl EmotionalValence {
    pub const MIN: f64 = -1.0;
    pub const MAX: f64 = 1.0;

    pub fn new(value: f64) -> Result<Self, EmotionalError> {
        if !value.is_finite() {
            return Err(EmotionalError::InvalidValue { value });
        }
        Ok(Self(value.clamp(Self::MIN, Self::MAX)))
    }

    pub fn is_positive(&self) -> bool { self.0 > 0.0 }
    pub fn is_negative(&self) -> bool { self.0 < 0.0 }
    pub fn magnitude(&self) -> f64 { self.0.abs() }
    pub fn blend(&self, other: Self, weight: f64) -> Self {
        let w = weight.clamp(0.0, 1.0);
        Self(self.0 * w + other.0 * (1.0 - w))
    }
}
