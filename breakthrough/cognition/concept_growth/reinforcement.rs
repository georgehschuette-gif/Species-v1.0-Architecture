// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

/// ConceptReinforcement: Strengthening of concept connections through use.
///
/// Reinforcement increases the strength of a concept's associations
/// each time it is employed. The strength is bounded by a configurable
/// maximum and grows at a rate that diminishes as the strength approaches the cap.
///
/// # Fields
/// - `reinforcement_rate`: Rate of strength increase per reinforcement event, in (0.0, 1.0].
/// - `max_strength`: Upper bound for concept strength, in (0.0, 1.0].
///
/// # Example
/// ```
/// use breakthrough::cognition::concept_growth::ConceptReinforcement;
///
/// let mut reinforce = ConceptReinforcement::new(0.1, 1.0).expect("valid parameters");
/// reinforce.apply(0.5).unwrap();
/// assert_eq!(reinforce.strength(), 0.05);
/// ```
pub struct ConceptReinforcement {
    /// Rate of strength increase per reinforcement event, in (0.0, 1.0].
    pub reinforcement_rate: f64,
    /// Upper bound for concept strength, in (0.0, 1.0].
    pub max_strength: f64,
    /// Current accumulated strength.
    strength: f64,
}

impl ConceptReinforcement {
    /// Creates a new `ConceptReinforcement` with the given rate and maximum strength.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if `reinforcement_rate` or `max_strength`
    /// are not in (0.0, 1.0].
    pub fn new(reinforcement_rate: f64, max_strength: f64) -> Result<Self, CognitionError> {
        if reinforcement_rate <= 0.0 || reinforcement_rate > 1.0 {
            return Err(CognitionError::OutOfRange {
                field: "reinforcement_rate".to_string(),
                value: reinforcement_rate,
                min: 0.0,
                max: 1.0,
            });
        }
        if max_strength <= 0.0 || max_strength > 1.0 {
            return Err(CognitionError::OutOfRange {
                field: "max_strength".to_string(),
                value: max_strength,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(Self {
            reinforcement_rate,
            max_strength,
            strength: 0.0,
        })
    }

    /// Applies reinforcement, increasing the internal strength by the rate
    /// scaled by the current headroom (`1.0 - strength / max_strength`).
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if `boost_factor` is negative.
    pub fn apply(&mut self, boost_factor: f64) -> Result<f64, CognitionError> {
        if boost_factor < 0.0 {
            return Err(CognitionError::OutOfRange {
                field: "boost_factor".to_string(),
                value: boost_factor,
                min: 0.0,
                max: f64::INFINITY,
            });
        }
        let headroom = 1.0 - self.strength / self.max_strength;
        let increment = self.reinforcement_rate * headroom * boost_factor;
        self.strength = (self.strength + increment).min(self.max_strength);
        Ok(self.strength)
    }

    /// Returns the current strength of the concept, normalized to [0.0, 1.0].
    pub fn strength(&self) -> f64 {
        self.strength
    }

    /// Returns the remaining capacity before reaching the maximum strength.
    pub fn headroom(&self) -> f64 {
        self.max_strength - self.strength
    }

    /// Returns `true` if the concept has reached its maximum strength.
    pub fn is_saturated(&self) -> bool {
        self.strength >= self.max_strength
    }

    /// Resets the internal strength to zero.
    pub fn reset(&mut self) {
        self.strength = 0.0;
    }

    /// Validates all field values are within valid ranges.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] for invalid rate or strength values.
    pub fn validate(&self) -> Result<(), CognitionError> {
        if self.reinforcement_rate <= 0.0 || self.reinforcement_rate > 1.0 {
            return Err(CognitionError::OutOfRange {
                field: "reinforcement_rate".to_string(),
                value: self.reinforcement_rate,
                min: 0.0,
                max: 1.0,
            });
        }
        if self.max_strength <= 0.0 || self.max_strength > 1.0 {
            return Err(CognitionError::OutOfRange {
                field: "max_strength".to_string(),
                value: self.max_strength,
                min: 0.0,
                max: 1.0,
            });
        }
        if self.strength < 0.0 || self.strength > self.max_strength {
            return Err(CognitionError::InvalidState(format!(
                "strength {} exceeds max_strength {}",
                self.strength, self.max_strength
            )));
        }
        Ok(())
    }
}

impl fmt::Debug for ConceptReinforcement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConceptReinforcement")
            .field("reinforcement_rate", &self.reinforcement_rate)
            .field("max_strength", &self.max_strength)
            .field("strength", &self.strength)
            .finish()
    }
}