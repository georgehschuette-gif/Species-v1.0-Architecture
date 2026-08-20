// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::{EntropyError, EntropyMeasure};

/// EntropyGeneration: The production of disorder through cognitive operations.
///
/// Tracks the rate at which entropy is produced and identifies
/// the source mechanism responsible.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntropySource {
    /// Disorder produced by computational operations.
    Computation,
    /// Disorder produced by communication between entities.
    Communication,
    /// Disorder produced by natural decay of state.
    Decay,
    /// Disorder produced by external noise.
    Noise,
}

impl EntropySource {
    /// Returns the string label of this entropy source.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Computation => "computation",
            Self::Communication => "communication",
            Self::Decay => "decay",
            Self::Noise => "noise",
        }
    }

    /// Attempts to parse an entropy source from a string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "computation" => Some(Self::Computation),
            "communication" => Some(Self::Communication),
            "decay" => Some(Self::Decay),
            "noise" => Some(Self::Noise),
            _ => None,
        }
    }

    /// Returns whether this source represents an internal cognitive process.
    pub fn is_internal(&self) -> bool {
        matches!(self, Self::Computation | Self::Decay)
    }

    /// Returns whether this source represents an external influence.
    pub fn is_external(&self) -> bool {
        matches!(self, Self::Communication | Self::Noise)
    }

    /// Returns the base generation factor for this source type.
    pub fn base_rate(&self) -> f64 {
        match self {
            Self::Computation => 0.3,
            Self::Communication => 0.2,
            Self::Decay => 0.15,
            Self::Noise => 0.5,
        }
    }
}

impl std::fmt::Display for EntropySource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for EntropySource {
    fn default() -> Self {
        Self::Computation
    }
}

/// EntropyGeneration: Tracks the rate and source of entropy production.
#[derive(Debug, Clone, PartialEq)]
pub struct EntropyGeneration {
    pub rate: f64,
    pub source: EntropySource,
}

impl EntropyGeneration {
    /// The minimum valid generation rate.
    pub const MIN_RATE: f64 = 0.0;
    /// The maximum valid generation rate.
    pub const MAX_RATE: f64 = 100.0;

    /// Creates a new entropy generation tracker.
    ///
    /// # Errors
    /// Returns `EntropyError::InvalidRate` if rate is NaN, infinite, or negative.
    pub fn new(rate: f64, source: EntropySource) -> Result<Self, EntropyError> {
        if rate.is_nan() || rate.is_infinite() {
            return Err(EntropyError::InvalidRate { rate });
        }
        if rate < Self::MIN_RATE {
            return Err(EntropyError::InvalidRate { rate });
        }
        Ok(Self { rate, source })
    }

    /// Returns the estimated entropy produced over a given time interval.
    pub fn produced_in(&self, interval: super::EntropyMeasure) -> EntropyMeasure {
        let entropy = self.rate * interval.0;
        EntropyMeasure::new(entropy)
            .unwrap_or(EntropyMeasure(Self::MAX_RATE))
    }

    /// Computes the combined generation from two sources.
    pub fn combine(&self, other: &EntropyGeneration) -> EntropyGeneration {
        let total_rate = self.rate + other.rate;
        let dominant = if self.rate >= other.rate {
            self.source
        } else {
            other.source
        };
        EntropyGeneration {
            rate: total_rate.min(Self::MAX_RATE),
            source: dominant,
        }
    }

    /// Scales the generation rate by a multiplier.
    ///
    /// # Errors
    /// Returns `EntropyError::InvalidRate` if the scaled rate exceeds bounds.
    pub fn scale_rate(&self, multiplier: f64) -> Result<EntropyGeneration, EntropyError> {
        let scaled = self.rate * multiplier;
        Self::new(scaled, self.source)
    }

    /// Checks whether this generation rate exceeds the specified threshold.
    pub fn exceeds_threshold(&self, threshold: f64) -> bool {
        self.rate > threshold && threshold.is_finite()
    }
}

impl Default for EntropyGeneration {
    fn default() -> Self {
        Self {
            rate: 0.0,
            source: EntropySource::Computation,
        }
    }
}