// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

/// ConceptForgetting: The gradual loss of unused concept strength.
///
/// Forgetting models the exponential decay of a concept's strength
/// over time when it is not actively used or reinforced. The forgetting
/// rate determines how quickly unused concepts lose their salience.
///
/// # Fields
/// - `forgetting_rate`: Rate at which strength decays per time unit, in [0.0, 1.0].
/// - `usage_threshold`: Minimum activity level below which forgetting accelerates, in [0.0, 1.0].
///
/// # Example
/// ```
/// use breakthrough::cognition::concept_decay::ConceptForgetting;
///
/// let mut forget = ConceptForgetting::new(0.05, 0.2).expect("valid parameters");
/// let remaining = forget.apply_decay(1.0, 0.9).expect("valid call");
/// assert!(remaining < 0.9);
/// ```
pub struct ConceptForgetting {
    /// Rate at which strength decays per time unit, in [0.0, 1.0].
    pub forgetting_rate: f64,
    /// Minimum activity level below which forgetting accelerates, in [0.0, 1.0].
    pub usage_threshold: f64,
}

impl ConceptForgetting {
    /// Creates a new `ConceptForgetting` with the given rate and threshold.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if either parameter is outside [0.0, 1.0].
    pub fn new(forgetting_rate: f64, usage_threshold: f64) -> Result<Self, CognitionError> {
        if !(0.0..=1.0).contains(&forgetting_rate) {
            return Err(CognitionError::OutOfRange {
                field: "forgetting_rate".to_string(),
                value: forgetting_rate,
                min: 0.0,
                max: 1.0,
            });
        }
        if !(0.0..=1.0).contains(&usage_threshold) {
            return Err(CognitionError::OutOfRange {
                field: "usage_threshold".to_string(),
                value: usage_threshold,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(Self {
            forgetting_rate,
            usage_threshold,
        })
    }

    /// Applies decay to a concept's current strength over a time duration.
    ///
    /// Uses the exponential decay model: `strength * exp(-rate * elapsed)`.
    /// If the current activity is below the usage threshold, the decay
    /// rate is doubled (rapid forgetting of unused concepts).
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if `elapsed` is negative.
    /// Returns [`CognitionError::InvalidDecay`] if the forgetting rate is 1.0.
    pub fn apply_decay(&self, elapsed: f64, current_strength: f64) -> Result<f64, CognitionError> {
        if elapsed < 0.0 {
            return Err(CognitionError::OutOfRange {
                field: "elapsed".to_string(),
                value: elapsed,
                min: 0.0,
                max: f64::INFINITY,
            });
        }
        if current_strength < 0.0 || current_strength > 1.0 {
            return Err(CognitionError::OutOfRange {
                field: "current_strength".to_string(),
                value: current_strength,
                min: 0.0,
                max: 1.0,
            });
        }
        if self.forgetting_rate >= 1.0 {
            return Err(CognitionError::InvalidDecay {
                rate: self.forgetting_rate,
            });
        }
        let effective_rate = if current_strength < self.usage_threshold {
            self.forgetting_rate * 2.0
        } else {
            self.forgetting_rate
        };
        Ok(current_strength * (-effective_rate * elapsed).exp())
    }

    /// Determines whether a concept has been effectively forgotten
    /// (strength below a minimal threshold).
    pub fn is_forgotten(&self, current_strength: f64) -> bool {
        current_strength < 0.01 * (1.0 + self.usage_threshold)
    }

    /// Computes the half-life of a concept at the given strength level,
    /// i.e., the time for strength to halve.
    pub fn half_life(&self, current_strength: f64) -> f64 {
        if current_strength <= 0.0 {
            return 0.0;
        }
        let effective_rate = if current_strength < self.usage_threshold {
            self.forgetting_rate * 2.0
        } else {
            self.forgetting_rate
        };
        if effective_rate <= 0.0 {
            return f64::INFINITY;
        }
        (2.0_f64).ln() / effective_rate
    }

    /// Validates all field values are within valid ranges.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] for invalid parameters.
    pub fn validate(&self) -> Result<(), CognitionError> {
        if !(0.0..=1.0).contains(&self.forgetting_rate) {
            return Err(CognitionError::OutOfRange {
                field: "forgetting_rate".to_string(),
                value: self.forgetting_rate,
                min: 0.0,
                max: 1.0,
            });
        }
        if !(0.0..=1.0).contains(&self.usage_threshold) {
            return Err(CognitionError::OutOfRange {
                field: "usage_threshold".to_string(),
                value: self.usage_threshold,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(())
    }
}

impl fmt::Debug for ConceptForgetting {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConceptForgetting")
            .field("forgetting_rate", &self.forgetting_rate)
            .field("usage_threshold", &self.usage_threshold)
            .finish()
    }
}