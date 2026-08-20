// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

/// ConceptCrystallization: The solidification of a concept into stable form.
///
/// Crystallization transforms an unstable concept seed into a durable
/// cognitive structure. The process applies a crystallization rate over
/// time, increasing the concept's stability until it reaches a stable
/// equilibrium.
///
/// # Fields
/// - `stability`: Current stability of the crystallized concept, in [0.0, 1.0].
/// - `crystallization_rate`: Rate at which stability increases per time step, in (0.0, 1.0].
///
/// # Example
/// ```
/// use breakthrough::cognition::concept_birth::ConceptCrystallization;
///
/// let mut crystal = ConceptCrystallization::new(0.3, 0.1).expect("valid parameters");
/// crystal.crystallize();
/// assert!(crystal.stability > 0.3);
/// ```
pub struct ConceptCrystallization {
    /// Current stability of the crystallized concept, in [0.0, 1.0].
    pub stability: f64,
    /// Rate at which stability increases per time step, in (0.0, 1.0].
    pub crystallization_rate: f64,
}

impl ConceptCrystallization {
    /// Creates a new `ConceptCrystallization` with the given stability and rate.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if `stability` is outside [0.0, 1.0]
    /// or `crystallization_rate` is not in (0.0, 1.0].
    pub fn new(stability: f64, crystallization_rate: f64) -> Result<Self, CognitionError> {
        if !(0.0..=1.0).contains(&stability) {
            return Err(CognitionError::OutOfRange {
                field: "stability".to_string(),
                value: stability,
                min: 0.0,
                max: 1.0,
            });
        }
        if crystallization_rate <= 0.0 || crystallization_rate > 1.0 {
            return Err(CognitionError::OutOfRange {
                field: "crystallization_rate".to_string(),
                value: crystallization_rate,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(Self {
            stability,
            crystallization_rate,
        })
    }

    /// Applies one step of crystallization, increasing stability by the rate factor.
    ///
    /// The new stability is computed as `stability + crystallization_rate * (1.0 - stability)`,
    /// which asymptotically approaches 1.0 without ever exceeding it.
    pub fn crystallize(&mut self) {
        self.stability += self.crystallization_rate * (1.0 - self.stability);
    }

    /// Applies crystallization for a given number of time steps.
    ///
    /// # Errors
    /// Returns [`CognitionError::InvalidState`] if `steps` is zero.
    pub fn crystallize_steps(&mut self, steps: usize) -> Result<(), CognitionError> {
        if steps == 0 {
            return Err(CognitionError::InvalidState(
                "steps must be greater than zero".to_string(),
            ));
        }
        for _ in 0..steps {
            self.crystallize();
        }
        Ok(())
    }

    /// Checks whether the concept has reached a stable equilibrium
    /// (stability remaining unchanged after one crystallization step).
    pub fn is_stable(&self) -> bool {
        let next = self.stability + self.crystallization_rate * (1.0 - self.stability);
        (next - self.stability).abs() < f64::EPSILON
    }

    /// Returns the remaining instability, i.e., `1.0 - stability`.
    pub fn remaining_instability(&self) -> f64 {
        1.0 - self.stability
    }

    /// Validates that all fields are within their valid ranges.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] for stability outside [0.0, 1.0]
    /// or crystallization_rate outside (0.0, 1.0].
    pub fn validate(&self) -> Result<(), CognitionError> {
        if !(0.0..=1.0).contains(&self.stability) {
            return Err(CognitionError::OutOfRange {
                field: "stability".to_string(),
                value: self.stability,
                min: 0.0,
                max: 1.0,
            });
        }
        if self.crystallization_rate <= 0.0 || self.crystallization_rate > 1.0 {
            return Err(CognitionError::OutOfRange {
                field: "crystallization_rate".to_string(),
                value: self.crystallization_rate,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(())
    }
}

impl fmt::Debug for ConceptCrystallization {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConceptCrystallization")
            .field("stability", &self.stability)
            .field("crystallization_rate", &self.crystallization_rate)
            .finish()
    }
}