// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

/// ConceptNucleation: The initial formation of a concept seed from perception.
///
/// Nucleation is the process by which raw perceptual data coheres into
/// an incipient concept. A seed forms when the input strength exceeds
/// the nucleation threshold, marking the birth of a new cognitive entity.
///
/// # Fields
/// - `seed_strength`: Current strength of the forming concept seed, in [0.0, 1.0].
/// - `nucleation_threshold`: The minimum strength required for nucleation to occur, in [0.0, 1.0].
///
/// # Example
/// ```
/// use breakthrough::cognition::concept_birth::ConceptNucleation;
///
/// let nucleation = ConceptNucleation::new(0.5, 0.4).expect("valid parameters");
/// assert!(nucleation.can_nucleate());
/// ```
pub struct ConceptNucleation {
    /// Current strength of the forming concept seed, in [0.0, 1.0].
    pub seed_strength: f64,
    /// The minimum strength required for nucleation to occur, in [0.0, 1.0].
    pub nucleation_threshold: f64,
}

impl ConceptNucleation {
    /// Creates a new `ConceptNucleation` with the given seed strength and threshold.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if either parameter is outside [0.0, 1.0].
    pub fn new(seed_strength: f64, nucleation_threshold: f64) -> Result<Self, CognitionError> {
        if !(0.0..=1.0).contains(&seed_strength) {
            return Err(CognitionError::OutOfRange {
                field: "seed_strength".to_string(),
                value: seed_strength,
                min: 0.0,
                max: 1.0,
            });
        }
        if !(0.0..=1.0).contains(&nucleation_threshold) {
            return Err(CognitionError::OutOfRange {
                field: "nucleation_threshold".to_string(),
                value: nucleation_threshold,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(Self {
            seed_strength,
            nucleation_threshold,
        })
    }

    /// Determines whether the current seed strength is sufficient to trigger nucleation.
    ///
    /// Returns `true` if `seed_strength >= nucleation_threshold`.
    pub fn can_nucleate(&self) -> bool {
        self.seed_strength >= self.nucleation_threshold
    }

    /// Attempts to form a concept seed from incoming perceptual strength.
    ///
    /// The resulting seed strength is the midpoint of the current strength
    /// and the input, capped at 1.0. If nucleation is not possible, an error
    /// is returned.
    ///
    /// # Errors
    /// Returns [`CognitionError::ThresholdNotMet`] if the input strength is
    /// insufficient to meet the nucleation threshold.
    /// Returns [`CognitionError::InvalidState`] if the seed is already at maximum strength.
    pub fn form_seed(&mut self, input_strength: f64) -> Result<f64, CognitionError> {
        if self.seed_strength >= 1.0 {
            return Err(CognitionError::InvalidState(
                "seed strength already at maximum (1.0)".to_string(),
            ));
        }
        if input_strength < self.nucleation_threshold {
            return Err(CognitionError::ThresholdNotMet {
                threshold: self.nucleation_threshold,
                actual: input_strength,
            });
        }
        self.seed_strength = ((self.seed_strength + input_strength) / 2.0).min(1.0);
        Ok(self.seed_strength)
    }

    /// Boosts the seed strength by a given amount, capped at 1.0.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if the boost amount is negative.
    pub fn boost(&mut self, amount: f64) -> Result<f64, CognitionError> {
        if amount < 0.0 {
            return Err(CognitionError::OutOfRange {
                field: "boost_amount".to_string(),
                value: amount,
                min: 0.0,
                max: f64::INFINITY,
            });
        }
        self.seed_strength = (self.seed_strength + amount).min(1.0);
        Ok(self.seed_strength)
    }

    /// Validates that all fields are within their valid ranges.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] for any field outside [0.0, 1.0].
    pub fn validate(&self) -> Result<(), CognitionError> {
        if !(0.0..=1.0).contains(&self.seed_strength) {
            return Err(CognitionError::OutOfRange {
                field: "seed_strength".to_string(),
                value: self.seed_strength,
                min: 0.0,
                max: 1.0,
            });
        }
        if !(0.0..=1.0).contains(&self.nucleation_threshold) {
            return Err(CognitionError::OutOfRange {
                field: "nucleation_threshold".to_string(),
                value: self.nucleation_threshold,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(())
    }
}

impl fmt::Debug for ConceptNucleation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConceptNucleation")
            .field("seed_strength", &self.seed_strength)
            .field("nucleation_threshold", &self.nucleation_threshold)
            .finish()
    }
}