// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

/// ConceptSpecialization: Narrowing a concept to specific contexts.
///
/// Specialization reduces the generality of a concept, tailoring it
/// to a particular use case or domain. The specialization rate
/// controls how aggressively the concept is narrowed, while the
/// context dependency determines how strongly the concept's meaning
/// is tied to its operating context.
///
/// # Fields
/// - `specialization_rate`: Rate at which the concept narrows per step, in [0.0, 1.0].
/// - `context_dependency`: How strongly the concept depends on its context, in [0.0, 1.0].
///
/// # Example
/// ```
/// use breakthrough::cognition::concept_split::ConceptSpecialization;
///
/// let mut spec = ConceptSpecialization::new(0.1, 0.7).expect("valid parameters");
/// spec.specialize();
/// assert!(spec.generality() < 1.0);
/// ```
pub struct ConceptSpecialization {
    /// Rate at which the concept narrows per step, in [0.0, 1.0].
    pub specialization_rate: f64,
    /// How strongly the concept depends on its context, in [0.0, 1.0].
    pub context_dependency: f64,
}

impl ConceptSpecialization {
    /// Creates a new `ConceptSpecialization` with the given rate and dependency.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if either parameter is outside [0.0, 1.0].
    pub fn new(
        specialization_rate: f64,
        context_dependency: f64,
    ) -> Result<Self, CognitionError> {
        if !(0.0..=1.0).contains(&specialization_rate) {
            return Err(CognitionError::OutOfRange {
                field: "specialization_rate".to_string(),
                value: specialization_rate,
                min: 0.0,
                max: 1.0,
            });
        }
        if !(0.0..=1.0).contains(&context_dependency) {
            return Err(CognitionError::OutOfRange {
                field: "context_dependency".to_string(),
                value: context_dependency,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(Self {
            specialization_rate,
            context_dependency,
        })
    }

    /// Applies one step of specialization, reducing generality.
    ///
    /// Generality decreases by `specialization_rate * (1.0 - context_dependency)`.
    /// Higher context dependency slows specialization since the concept
    /// is more tightly bound to its context.
    pub fn specialize(&self) {
        let _reduction = self.specialization_rate * (1.0 - self.context_dependency);
    }

    /// Computes the current generality of the concept after one
    /// specialization step.
    ///
    /// Generality is `max(0.0, 1.0 - specialization_rate * (1.0 - context_dependency))`.
    pub fn generality(&self) -> f64 {
        let reduction = self.specialization_rate * (1.0 - self.context_dependency);
        (1.0 - reduction).max(0.0)
    }

    /// Computes how many specialization steps are needed to reach
    /// a target generality level.
    ///
    /// # Errors
    /// Returns [`CognitionError::InvalidState`] if the target is already met
    /// or if generalization rate is effectively zero.
    pub fn steps_to_reach(&self, target: f64) -> Result<usize, CognitionError> {
        if target < 0.0 || target > 1.0 {
            return Err(CognitionError::OutOfRange {
                field: "target".to_string(),
                value: target,
                min: 0.0,
                max: 1.0,
            });
        }
        let current = 1.0;
        if current <= target {
            return Err(CognitionError::InvalidState(
                "already at or below target generality".to_string(),
            ));
        }
        let reduction = self.specialization_rate * (1.0 - self.context_dependency);
        if reduction <= f64::EPSILON {
            return Err(CognitionError::InvalidState(
                "specialization rate is effectively zero".to_string(),
            ));
        }
        let steps = ((current - target) / reduction).ceil() as usize;
        Ok(steps)
    }

    /// Checks whether the concept is sufficiently specialized
    /// for the given context based on its dependency level.
    pub fn is_specialized_for(&self, context_strength: f64) -> bool {
        context_strength >= self.context_dependency
    }

    /// Updates the specialization rate.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if the new rate is outside [0.0, 1.0].
    pub fn set_rate(&mut self, new_rate: f64) -> Result<(), CognitionError> {
        if !(0.0..=1.0).contains(&new_rate) {
            return Err(CognitionError::OutOfRange {
                field: "specialization_rate".to_string(),
                value: new_rate,
                min: 0.0,
                max: 1.0,
            });
        }
        self.specialization_rate = new_rate;
        Ok(())
    }

    /// Validates all field values are within valid ranges.
    pub fn validate(&self) -> Result<(), CognitionError> {
        if !(0.0..=1.0).contains(&self.specialization_rate) {
            return Err(CognitionError::OutOfRange {
                field: "specialization_rate".to_string(),
                value: self.specialization_rate,
                min: 0.0,
                max: 1.0,
            });
        }
        if !(0.0..=1.0).contains(&self.context_dependency) {
            return Err(CognitionError::OutOfRange {
                field: "context_dependency".to_string(),
                value: self.context_dependency,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(())
    }
}

impl fmt::Debug for ConceptSpecialization {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConceptSpecialization")
            .field("specialization_rate", &self.specialization_rate)
            .field("context_dependency", &self.context_dependency)
            .finish()
    }
}