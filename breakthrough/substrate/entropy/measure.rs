// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.


/// EntropyMeasure: The quantified disorder of a cognitive state.
///
/// Values range from 0.0 (perfectly ordered) to 1.0 (maximum disorder).
/// Intermediate values represent varying degrees of partial order.
///
/// # Invariants
/// - The internal value is always in the range [0.0, 1.0]
/// - NaN values are clamped to the nearest boundary
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct EntropyMeasure(pub f64);

impl EntropyMeasure {
    /// Minimum possible entropy value (perfect order).
    pub const MIN: f64 = 0.0;
    /// Maximum possible entropy value (maximum disorder).
    pub const MAX: f64 = 1.0;
    /// Threshold below which entropy is considered low.
    pub const LOW_THRESHOLD: f64 = 0.3;
    /// Threshold above which entropy is considered high.
    pub const HIGH_THRESHOLD: f64 = 0.7;

    /// Creates a new entropy measure from the given value.
    ///
    /// Values below 0.0 are clamped to 0.0; values above 1.0 are clamped to 1.0.
    /// NaN values are treated as 0.0.
    ///
    /// # Errors
    /// Returns `EntropyError::InvalidValue` if the value is NaN or infinite.
    pub fn new(value: f64) -> Result<Self, EntropyError> {
        if value.is_nan() {
            return Err(EntropyError::InvalidValue { value });
        }
        if value.is_infinite() {
            return Err(EntropyError::InvalidValue { value });
        }
        Ok(Self(value.clamp(Self::MIN, Self::MAX)))
    }

    /// Returns whether the entropy is below the low threshold.
    pub fn is_low(&self) -> bool {
        self.0 < Self::LOW_THRESHOLD
    }

    /// Returns whether the entropy is above the high threshold.
    pub fn is_high(&self) -> bool {
        self.0 > Self::HIGH_THRESHOLD
    }

    /// Returns whether the entropy is in a moderate range.
    pub fn is_moderate(&self) -> bool {
        !self.is_low() && !self.is_high()
    }

    /// Returns the inverse of this entropy measure (order = 1 - entropy).
    pub fn order(&self) -> Self {
        Self(1.0 - self.0)
    }

    /// Computes a weighted combination of two entropy measures.
    pub fn blend(&self, other: &EntropyMeasure, weight: f64) -> Result<Self, EntropyError> {
        let w = weight.clamp(0.0, 1.0);
        let blended = self.0 * w + other.0 * (1.0 - w);
        Ok(Self(blended))
    }

    /// Returns the entropy difference between two measures.
    pub fn difference(&self, other: &EntropyMeasure) -> f64 {
        (self.0 - other.0).abs()
    }

    /// Normalizes the entropy value to a [0, 1] range relative to a maximum possible entropy.
    pub fn normalize(&self, max_entropy: EntropyMeasure) -> Result<EntropyMeasure, EntropyError> {
        if max_entropy.0 <= f64::EPSILON {
            return Err(EntropyError::ZeroDivisor);
        }
        Ok(Self(self.0 / max_entropy.0))
    }
}

impl Default for EntropyMeasure {
    fn default() -> Self {
        Self(0.0)
    }
}

/// Error type for entropy measure failures.
#[derive(Debug, Clone, PartialEq)]
pub enum EntropyError {
    /// The input value is NaN or infinite.
    InvalidValue { value: f64 },
    /// The rate value is NaN, infinite, or negative.
    InvalidRate { rate: f64 },
    /// Division by zero in normalization.
    ZeroDivisor,
}

impl std::fmt::Display for EntropyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntropyError::InvalidValue { value } => {
                write!(f, "Invalid entropy value {}: must be finite", value)
            }
            EntropyError::InvalidRate { rate } => {
                write!(f, "Invalid entropy rate {}: must be non-negative and finite", rate)
            }
            EntropyError::ZeroDivisor => {
                write!(f, "Cannot normalize entropy against zero maximum")
            }
        }
    }
}

impl std::error::Error for EntropyError {}