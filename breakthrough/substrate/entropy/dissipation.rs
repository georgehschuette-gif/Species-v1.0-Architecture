// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::{EntropyError, EntropyMeasure};

/// DissipationMechanism: The method by which entropy is reduced in the cognitive system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DissipationMechanism {
    /// Organization through structural formation.
    Structuration,
    /// Reduction of redundancy through compression.
    Compression,
    /// Emergence of ordered crystalline patterns.
    Crystallization,
}

impl DissipationMechanism {
    /// Returns the string label of this dissipation mechanism.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Structuration => "structuration",
            Self::Compression => "compression",
            Self::Crystallization => "crystallization",
        }
    }

    /// Attempts to parse a dissipation mechanism from a string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "structuration" => Some(Self::Structuration),
            "compression" => Some(Self::Compression),
            "crystallization" => Some(Self::Crystallization),
            _ => None,
        }
    }

    /// Returns whether this mechanism is computationally intensive.
    pub fn is_intensive(&self) -> bool {
        matches!(self, Self::Crystallization)
    }

    /// Returns the base dissipation factor for this mechanism.
    pub fn base_factor(&self) -> f64 {
        match self {
            Self::Structuration => 0.4,
            Self::Compression => 0.6,
            Self::Crystallization => 0.8,
        }
    }
}

impl std::fmt::Display for DissipationMechanism {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for DissipationMechanism {
    fn default() -> Self {
        Self::Structuration
    }
}

/// EntropyDissipation: The reduction of disorder through organization.
#[derive(Debug, Clone, PartialEq)]
pub struct EntropyDissipation {
    pub rate: f64,
    pub mechanism: DissipationMechanism,
}

impl EntropyDissipation {
    /// The minimum valid dissipation rate.
    pub const MIN_RATE: f64 = 0.0;
    /// The maximum valid dissipation rate.
    pub const MAX_RATE: f64 = 100.0;

    /// Creates a new entropy dissipation tracker.
    ///
    /// # Errors
    /// Returns `EntropyError::InvalidRate` if rate is NaN, infinite, or negative.
    pub fn new(rate: f64, mechanism: DissipationMechanism) -> Result<Self, EntropyError> {
        if rate.is_nan() || rate.is_infinite() {
            return Err(EntropyError::InvalidRate { rate });
        }
        if rate < Self::MIN_RATE {
            return Err(EntropyError::InvalidRate { rate });
        }
        Ok(Self { rate, mechanism })
    }

    /// Returns the estimated entropy dissipated over a given interval.
    pub fn dissipated_in(&self, interval: super::EntropyMeasure) -> EntropyMeasure {
        let entropy = self.rate * interval.0 * self.mechanism.base_factor();
        EntropyMeasure::new(entropy)
            .unwrap_or(EntropyMeasure(Self::MAX_RATE))
    }

    /// Computes the combined dissipation from two mechanisms.
    pub fn combine(&self, other: &EntropyDissipation) -> EntropyDissipation {
        let total_rate = self.rate + other.rate;
        let dominant = if self.rate >= other.rate {
            self.mechanism
        } else {
            other.mechanism
        };
        EntropyDissipation {
            rate: total_rate.min(Self::MAX_RATE),
            mechanism: dominant,
        }
    }

    /// Scales the dissipation rate by a multiplier.
    ///
    /// # Errors
    /// Returns `EntropyError::InvalidRate` if the scaled rate exceeds bounds.
    pub fn scale_rate(&self, multiplier: f64) -> Result<EntropyDissipation, EntropyError> {
        let scaled = self.rate * multiplier;
        Self::new(scaled, self.mechanism)
    }

    /// Checks whether this dissipation rate exceeds the specified threshold.
    pub fn exceeds_threshold(&self, threshold: f64) -> bool {
        self.rate > threshold && threshold.is_finite()
    }

    /// Returns the effectiveness of this dissipation relative to a given entropy measure.
    pub fn effectiveness(&self, entropy: &EntropyMeasure) -> f64 {
        if entropy.0 == 0.0 {
            return 1.0;
        }
        let potential = self.rate * self.mechanism.base_factor();
        potential / (potential + entropy.0)
    }
}

impl Default for EntropyDissipation {
    fn default() -> Self {
        Self {
            rate: 0.0,
            mechanism: DissipationMechanism::Structuration,
        }
    }
}