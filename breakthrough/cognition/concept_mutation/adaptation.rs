// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

/// ConceptAdaptation: Adjusting concepts to new contexts or data.
///
/// Adaptation enables concepts to evolve in response to changing
/// environmental conditions or new input data. The adaptation
/// rate controls how quickly concepts adjust, while plasticity
/// determines the range of acceptable modifications.
///
/// # Fields
/// - `adaptation_rate`: Speed of adjustment per time step, in [0.0, 1.0].
/// - `plasticity`: Maximum magnitude of a single adaptation step, in [0.0, 1.0].
///
/// # Example
/// ```
/// use breakthrough::cognition::concept_mutation::ConceptAdaptation;
///
/// let mut adapt = ConceptAdaptation::new(0.15, 0.3).expect("valid parameters");
/// adapt.adjust(0.5);
/// assert!(adapt.current_deviation() < 0.3);
/// ```
pub struct ConceptAdaptation {
    /// Speed of adjustment per time step, in [0.0, 1.0].
    pub adaptation_rate: f64,
    /// Maximum magnitude of a single adaptation step, in [0.0, 1.0].
    pub plasticity: f64,
    /// Current deviation from the original concept state.
    current_deviation: f64,
}

impl ConceptAdaptation {
    /// Creates a new `ConceptAdaptation` with the given rate and plasticity.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if either parameter is outside [0.0, 1.0].
    pub fn new(adaptation_rate: f64, plasticity: f64) -> Result<Self, CognitionError> {
        if !(0.0..=1.0).contains(&adaptation_rate) {
            return Err(CognitionError::OutOfRange {
                field: "adaptation_rate".to_string(),
                value: adaptation_rate,
                min: 0.0,
                max: 1.0,
            });
        }
        if !(0.0..=1.0).contains(&plasticity) {
            return Err(CognitionError::OutOfRange {
                field: "plasticity".to_string(),
                value: plasticity,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(Self {
            adaptation_rate,
            plasticity,
            current_deviation: 0.0,
        })
    }

    /// Adjusts the concept toward a target value based on the current state.
    ///
    /// The adjustment magnitude is bounded by `plasticity` and scaled by
    /// `adaptation_rate`. If the target is within plasticity distance,
    /// the concept snaps to the target.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if `target` is outside [0.0, 1.0].
    pub fn adjust(&mut self, target: f64) -> Result<f64, CognitionError> {
        if !(0.0..=1.0).contains(&target) {
            return Err(CognitionError::OutOfRange {
                field: "target".to_string(),
                value: target,
                min: 0.0,
                max: 1.0,
            });
        }
        let raw_adjustment = target - self.current_deviation;
        let capped_adjustment = raw_adjustment.clamp(-self.plasticity, self.plasticity);
        let effective = capped_adjustment * self.adaptation_rate;
        self.current_deviation = (self.current_deviation + effective).clamp(0.0, 1.0);
        Ok(self.current_deviation)
    }

    /// Returns the current deviation from the original state.
    pub fn current_deviation(&self) -> f64 {
        self.current_deviation
    }

    /// Resets the deviation to zero, restoring the concept to its original state.
    pub fn reset(&mut self) {
        self.current_deviation = 0.0;
    }

    /// Returns `true` if the concept has stabilized (deviation below a minimal threshold).
    pub fn is_stabilized(&self) -> bool {
        self.current_deviation < 0.01
    }

    /// Computes the distance remaining to fully adapt to the target value.
    pub fn distance_to_target(&self, target: f64) -> Result<f64, CognitionError> {
        if !(0.0..=1.0).contains(&target) {
            return Err(CognitionError::OutOfRange {
                field: "target".to_string(),
                value: target,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok((target - self.current_deviation).abs())
    }

    /// Updates the plasticity parameter.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if the new plasticity is outside [0.0, 1.0].
    pub fn set_plasticity(&mut self, new_plasticity: f64) -> Result<(), CognitionError> {
        if !(0.0..=1.0).contains(&new_plasticity) {
            return Err(CognitionError::OutOfRange {
                field: "plasticity".to_string(),
                value: new_plasticity,
                min: 0.0,
                max: 1.0,
            });
        }
        self.plasticity = new_plasticity;
        Ok(())
    }

    /// Validates all field values are within valid ranges.
    pub fn validate(&self) -> Result<(), CognitionError> {
        if !(0.0..=1.0).contains(&self.adaptation_rate) {
            return Err(CognitionError::OutOfRange {
                field: "adaptation_rate".to_string(),
                value: self.adaptation_rate,
                min: 0.0,
                max: 1.0,
            });
        }
        if !(0.0..=1.0).contains(&self.plasticity) {
            return Err(CognitionError::OutOfRange {
                field: "plasticity".to_string(),
                value: self.plasticity,
                min: 0.0,
                max: 1.0,
            });
        }
        if self.current_deviation < 0.0 || self.current_deviation > 1.0 {
            return Err(CognitionError::InvalidState(format!(
                "current_deviation {} out of range [0.0, 1.0]",
                self.current_deviation
            )));
        }
        Ok(())
    }
}

impl fmt::Debug for ConceptAdaptation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConceptAdaptation")
            .field("adaptation_rate", &self.adaptation_rate)
            .field("plasticity", &self.plasticity)
            .field("current_deviation", &self.current_deviation)
            .finish()
    }
}