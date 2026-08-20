// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

/// ConceptReconfiguration: Restructuring concept internal representations.
///
/// Reconfiguration reorganizes the internal architecture of a concept
/// without changing its external semantics. This is essential for
/// maintaining cognitive efficiency as the concept network evolves.
/// The reconfiguration rate controls how much restructuring occurs
/// per operation, while structural plasticity determines the flexibility
/// of the internal representation.
///
/// # Fields
/// - `reconfiguration_rate`: Fraction of structure to reorganize per step, in [0.0, 1.0].
/// - `structural_plasticity`: Maximum structural change per reconfiguration, in [0.0, 1.0].
pub struct ConceptReconfiguration {
    /// Fraction of structure to reorganize per step, in [0.0, 1.0].
    pub reconfiguration_rate: f64,
    /// Maximum structural change per reconfiguration, in [0.0, 1.0].
    pub structural_plasticity: f64,
    /// Current structural complexity of the representation.
    structural_complexity: f64,
}

impl ConceptReconfiguration {
    /// Creates a new `ConceptReconfiguration` with the given rate and plasticity.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if either parameter is outside [0.0, 1.0].
    pub fn new(
        reconfiguration_rate: f64,
        structural_plasticity: f64,
    ) -> Result<Self, CognitionError> {
        if !(0.0..=1.0).contains(&reconfiguration_rate) {
            return Err(CognitionError::OutOfRange {
                field: "reconfiguration_rate".to_string(),
                value: reconfiguration_rate,
                min: 0.0,
                max: 1.0,
            });
        }
        if !(0.0..=1.0).contains(&structural_plasticity) {
            return Err(CognitionError::OutOfRange {
                field: "structural_plasticity".to_string(),
                value: structural_plasticity,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(Self {
            reconfiguration_rate,
            structural_plasticity,
            structural_complexity: 1.0,
        })
    }

    /// Applies one reconfiguration step, restructuring the internal representation.
    ///
    /// The complexity is adjusted by a delta bounded by structural_plasticity
    /// and scaled by reconfiguration_rate. Complexity cannot go below 0.1 or above 1.0.
    pub fn reconfigure(&mut self) -> f64 {
        let delta = self.structural_plasticity * self.reconfiguration_rate;
        // Oscillate complexity between 0.1 and 1.0 based on current state
        let direction = if self.structural_complexity > 0.5 { -1.0 } else { 1.0 };
        self.structural_complexity =
            (self.structural_complexity + direction * delta).clamp(0.1, 1.0);
        self.structural_complexity
    }

    /// Applies multiple reconfiguration steps.
    ///
    /// # Errors
    /// Returns [`CognitionError::InvalidState`] if `steps` is zero.
    pub fn reconfigure_steps(&mut self, steps: usize) -> Result<f64, CognitionError> {
        if steps == 0 {
            return Err(CognitionError::InvalidState(
                "steps must be greater than zero".to_string(),
            ));
        }
        let mut complexity = self.structural_complexity;
        for _ in 0..steps {
            complexity = self.reconfigure();
        }
        Ok(complexity)
    }

    /// Returns the current structural complexity.
    pub fn complexity(&self) -> f64 {
        self.structural_complexity
    }

    /// Simplifies the representation, reducing complexity by a fixed amount.
    ///
    /// # Errors
    /// Returns [`CognitionError::InvalidState`] if the representation is already at minimum complexity.
    pub fn simplify(&mut self) -> Result<f64, CognitionError> {
        let new_complexity = self.structural_complexity - self.structural_plasticity;
        if new_complexity < 0.1 {
            return Err(CognitionError::InvalidState(
                "representation is already at minimum complexity (0.1)".to_string(),
            ));
        }
        self.structural_complexity = new_complexity;
        Ok(self.structural_complexity)
    }

    /// Complexifies the representation, increasing complexity.
    ///
    /// # Errors
    /// Returns [`CognitionError::InvalidState`] if the representation is already at maximum complexity.
    pub fn complexify(&mut self) -> Result<f64, CognitionError> {
        let new_complexity = self.structural_complexity + self.structural_plasticity;
        if new_complexity > 1.0 {
            return Err(CognitionError::InvalidState(
                "representation is already at maximum complexity (1.0)".to_string(),
            ));
        }
        self.structural_complexity = new_complexity;
        Ok(self.structural_complexity)
    }

    /// Updates the reconfiguration rate.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if the new rate is outside [0.0, 1.0].
    pub fn set_rate(&mut self, new_rate: f64) -> Result<(), CognitionError> {
        if !(0.0..=1.0).contains(&new_rate) {
            return Err(CognitionError::OutOfRange {
                field: "reconfiguration_rate".to_string(),
                value: new_rate,
                min: 0.0,
                max: 1.0,
            });
        }
        self.reconfiguration_rate = new_rate;
        Ok(())
    }

    /// Validates all field values are within valid ranges.
    pub fn validate(&self) -> Result<(), CognitionError> {
        if !(0.0..=1.0).contains(&self.reconfiguration_rate) {
            return Err(CognitionError::OutOfRange {
                field: "reconfiguration_rate".to_string(),
                value: self.reconfiguration_rate,
                min: 0.0,
                max: 1.0,
            });
        }
        if !(0.0..=1.0).contains(&self.structural_plasticity) {
            return Err(CognitionError::OutOfRange {
                field: "structural_plasticity".to_string(),
                value: self.structural_plasticity,
                min: 0.0,
                max: 1.0,
            });
        }
        if self.structural_complexity < 0.1 || self.structural_complexity > 1.0 {
            return Err(CognitionError::InvalidState(format!(
                "structural_complexity {} out of range [0.1, 1.0]",
                self.structural_complexity
            )));
        }
        Ok(())
    }
}

impl fmt::Debug for ConceptReconfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConceptReconfiguration")
            .field("reconfiguration_rate", &self.reconfiguration_rate)
            .field("structural_plasticity", &self.structural_plasticity)
            .field("structural_complexity", &self.structural_complexity)
            .finish()
    }
}