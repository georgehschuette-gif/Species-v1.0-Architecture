// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.


/// ResonantFrequency: The natural oscillation frequency of an entity.
///
/// Frequency represents how rapidly an entity oscillates in its
/// cognitive state. Higher frequencies indicate faster processing cycles.
///
/// # Invariants
/// - Frequency must be non-negative and finite
/// - Zero frequency represents a static, non-oscillating entity
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ResonantFrequency(pub f64);

impl ResonantFrequency {
    /// The minimum valid frequency (static, non-oscillating).
    pub const MIN_FREQ: f64 = 0.0;
    /// The maximum valid frequency.
    pub const MAX_FREQ: f64 = 1000.0;

    /// Creates a new resonant frequency.
    ///
    /// # Errors
    /// Returns `FrequencyError::InvalidFrequency` if the value is NaN, infinite, or negative.
    pub fn new(freq: f64) -> Result<Self, FrequencyError> {
        if freq.is_nan() || freq.is_infinite() {
            return Err(FrequencyError::InvalidFrequency { freq });
        }
        if freq < Self::MIN_FREQ {
            return Err(FrequencyError::InvalidFrequency { freq });
        }
        Ok(Self(freq.min(Self::MAX_FREQ)))
    }

    /// Creates a frequency without validation.
    ///
    /// # Safety
    /// The caller must ensure the value is non-negative and finite.
    pub const fn new_unchecked(freq: f64) -> Self {
        Self(freq)
    }

    /// Checks whether this frequency matches another within a tolerance.
    pub fn matches(&self, other: &ResonantFrequency, tolerance: f64) -> bool {
        if tolerance < 0.0 || tolerance.is_nan() {
            return false;
        }
        (self.0 - other.0).abs() < tolerance
    }

    /// Computes the harmonic mean of two frequencies.
    pub fn harmonic_mean(&self, other: &ResonantFrequency) -> Result<Self, FrequencyError> {
        if self.is_zero() || other.is_zero() {
            return Ok(Self::new(0.0).unwrap());
        }
        let harmonic = 2.0 * self.0 * other.0 / (self.0 + other.0);
        Self::new(harmonic)
    }

    /// Returns the period corresponding to this frequency.
    ///
    /// Returns infinity for zero frequency.
    pub fn period(&self) -> f64 {
        if self.is_zero() {
            f64::INFINITY
        } else {
            1.0 / self.0
        }
    }

    /// Computes the beat frequency between this and another frequency.
    pub fn beat_frequency(&self, other: &ResonantFrequency) -> ResonantFrequency {
        let beat = (self.0 - other.0).abs();
        Self::new(beat).unwrap_or(Self(0.0))
    }

    /// Check whether this frequency is zero (static entity).
    pub fn is_zero(&self) -> bool {
        self.0 < f64::EPSILON
    }

    /// Checks whether this frequency is at or near maximum.
    pub fn is_maximum(&self) -> bool {
        (self.0 - Self::MAX_FREQ).abs() < f64::EPSILON
    }
}

impl Default for ResonantFrequency {
    fn default() -> Self {
        Self(1.0)
    }
}

/// Error type for frequency validation failures.
#[derive(Debug, Clone, PartialEq)]
pub enum FrequencyError {
    /// The frequency value is NaN, infinite, or negative.
    InvalidFrequency { freq: f64 },
}

impl std::fmt::Display for FrequencyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FrequencyError::InvalidFrequency { freq } => {
                write!(f, "Invalid frequency {}: must be non-negative and finite", freq)
            }
        }
    }
}

impl std::error::Error for FrequencyError {}