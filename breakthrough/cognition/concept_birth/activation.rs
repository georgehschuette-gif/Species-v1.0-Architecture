// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

/// ConceptActivation: The process of bringing a concept into active use.
///
/// Activation represents the dynamic engagement of a concept within
/// working cognition. An activated concept decays over time at a
/// configurable rate and must be periodically refreshed to remain active.
///
/// # Fields
/// - `activation_level`: Current activation level, in [0.0, 1.0].
/// - `decay_rate`: Rate at which activation decays per time step, in [0.0, 1.0].
///
/// # Example
/// ```
/// use breakthrough::cognition::concept_birth::ConceptActivation;
///
/// let mut activation = ConceptActivation::new(0.8, 0.05).expect("valid parameters");
/// assert!(activation.is_active());
/// activation.decay(1.0);
/// assert!(activation.activation_level < 0.8);
/// ```
pub struct ConceptActivation {
    /// Current activation level, in [0.0, 1.0].
    pub activation_level: f64,
    /// Rate at which activation decays per time step, in [0.0, 1.0].
    pub decay_rate: f64,
}

impl ConceptActivation {
    /// Creates a new `ConceptActivation` with the given level and decay rate.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if `activation_level` is outside [0.0, 1.0]
    /// or `decay_rate` is outside [0.0, 1.0].
    pub fn new(activation_level: f64, decay_rate: f64) -> Result<Self, CognitionError> {
        if !(0.0..=1.0).contains(&activation_level) {
            return Err(CognitionError::OutOfRange {
                field: "activation_level".to_string(),
                value: activation_level,
                min: 0.0,
                max: 1.0,
            });
        }
        if !(0.0..=1.0).contains(&decay_rate) {
            return Err(CognitionError::OutOfRange {
                field: "decay_rate".to_string(),
                value: decay_rate,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(Self {
            activation_level,
            decay_rate,
        })
    }

    /// Increases the activation level by the given delta, capped at 1.0.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if `delta` is negative.
    pub fn activate(&mut self, delta: f64) -> Result<(), CognitionError> {
        if delta < 0.0 {
            return Err(CognitionError::OutOfRange {
                field: "delta".to_string(),
                value: delta,
                min: 0.0,
                max: f64::INFINITY,
            });
        }
        self.activation_level = (self.activation_level + delta).min(1.0);
        Ok(())
    }

    /// Applies decay to the activation level over a given time duration.
    ///
    /// The decay follows the exponential model:
    /// `activation_level *= (1.0 - decay_rate).powf(elapsed)`.
    ///
    /// # Errors
    /// Returns [`CognitionError::InvalidDecay`] if the decay rate is 1.0 (instant decay).
    /// Returns [`CognitionError::OutOfRange`] if `elapsed` is negative.
    pub fn decay(&mut self, elapsed: f64) -> Result<(), CognitionError> {
        if elapsed < 0.0 {
            return Err(CognitionError::OutOfRange {
                field: "elapsed".to_string(),
                value: elapsed,
                min: 0.0,
                max: f64::INFINITY,
            });
        }
        if self.decay_rate >= 1.0 {
            return Err(CognitionError::InvalidDecay {
                rate: self.decay_rate,
            });
        }
        self.activation_level *= (1.0 - self.decay_rate).powf(elapsed);
        Ok(())
    }

    /// Returns `true` if the concept is currently active (activation > 0.0).
    pub fn is_active(&self) -> bool {
        self.activation_level > 0.0
    }

    /// Returns `true` if the concept is fully activated (activation >= 1.0).
    pub fn is_fully_activated(&self) -> bool {
        self.activation_level >= 1.0
    }

    /// Forces the activation level to zero, deactivating the concept.
    pub fn deactivate(&mut self) {
        self.activation_level = 0.0;
    }

    /// Validates that all fields are within their valid ranges.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] for any field outside [0.0, 1.0].
    pub fn validate(&self) -> Result<(), CognitionError> {
        if !(0.0..=1.0).contains(&self.activation_level) {
            return Err(CognitionError::OutOfRange {
                field: "activation_level".to_string(),
                value: self.activation_level,
                min: 0.0,
                max: 1.0,
            });
        }
        if !(0.0..=1.0).contains(&self.decay_rate) {
            return Err(CognitionError::OutOfRange {
                field: "decay_rate".to_string(),
                value: self.decay_rate,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(())
    }
}

impl fmt::Debug for ConceptActivation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConceptActivation")
            .field("activation_level", &self.activation_level)
            .field("decay_rate", &self.decay_rate)
            .finish()
    }
}