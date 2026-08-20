// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

/// ConceptPruning: The removal of weak or irrelevant concepts.
///
/// Pruning cleans up the cognitive landscape by eliminating concepts
/// that fall below a minimum strength threshold or that exceed a
/// manageable count. This prevents cognitive overload and maintains
/// system efficiency.
///
/// # Fields
/// - `pruning_threshold`: Minimum strength a concept must have to survive, in [0.0, 1.0].
/// - `max_concepts`: Maximum number of concepts allowed before pruning triggers.
///
/// # Example
/// ```
/// use breakthrough::cognition::concept_decay::ConceptPruning;
///
/// let mut pruner = ConceptPruning::new(0.1, 100).expect("valid parameters");
/// assert!(pruner.should_prune(0.05, 150));
/// assert!(!pruner.should_prune(0.5, 50));
/// ```
pub struct ConceptPruning {
    /// Minimum strength a concept must have to survive, in [0.0, 1.0].
    pub pruning_threshold: f64,
    /// Maximum number of concepts allowed before pruning triggers.
    pub max_concepts: usize,
}

impl ConceptPruning {
    /// Creates a new `ConceptPruning` with the given threshold and max concepts.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if `pruning_threshold` is outside [0.0, 1.0]
    /// or `max_concepts` is zero.
    pub fn new(pruning_threshold: f64, max_concepts: usize) -> Result<Self, CognitionError> {
        if !(0.0..=1.0).contains(&pruning_threshold) {
            return Err(CognitionError::OutOfRange {
                field: "pruning_threshold".to_string(),
                value: pruning_threshold,
                min: 0.0,
                max: 1.0,
            });
        }
        if max_concepts == 0 {
            return Err(CognitionError::OutOfRange {
                field: "max_concepts".to_string(),
                value: 0.0,
                min: 1.0,
                max: f64::MAX,
            });
        }
        Ok(Self {
            pruning_threshold,
            max_concepts,
        })
    }

    /// Determines whether a concept should be pruned based on its
    /// strength and the current total concept count.
    ///
    /// A concept is pruned if its strength is below the pruning threshold
    /// OR if the total count exceeds the maximum and the concept is
    /// below the median strength.
    pub fn should_prune(&self, concept_strength: f64, total_concepts: usize) -> bool {
        if concept_strength < self.pruning_threshold {
            return true;
        }
        if total_concepts > self.max_concepts {
            true
        } else {
            false
        }
    }

    /// Filters a list of concept strengths, returning only the indices
    /// of concepts that should be pruned.
    ///
    /// # Errors
    /// Returns [`CognitionError::MissingInput`] if the strengths slice is empty.
    pub fn prune_indices(
        &self,
        strengths: &[f64],
        total_concepts: usize,
    ) -> Result<Vec<usize>, CognitionError> {
        if strengths.is_empty() {
            return Err(CognitionError::MissingInput(
                "strengths slice must not be empty".to_string(),
            ));
        }
        let mut indices: Vec<usize> = strengths
            .iter()
            .enumerate()
            .filter(|(_, &s)| self.should_prune(s, total_concepts))
            .map(|(i, _)| i)
            .collect();
        indices.sort();
        Ok(indices)
    }

    /// Computes how many concepts would be pruned from the given list.
    pub fn count_prunable(&self, strengths: &[f64], total_concepts: usize) -> usize {
        strengths
            .iter()
            .filter(|&&s| self.should_prune(s, total_concepts))
            .count()
    }

    /// Adjusts the pruning threshold.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if the new threshold is outside [0.0, 1.0].
    pub fn set_threshold(&mut self, new_threshold: f64) -> Result<(), CognitionError> {
        if !(0.0..=1.0).contains(&new_threshold) {
            return Err(CognitionError::OutOfRange {
                field: "pruning_threshold".to_string(),
                value: new_threshold,
                min: 0.0,
                max: 1.0,
            });
        }
        self.pruning_threshold = new_threshold;
        Ok(())
    }

    /// Validates all field values are within valid ranges.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] for invalid values.
    pub fn validate(&self) -> Result<(), CognitionError> {
        if !(0.0..=1.0).contains(&self.pruning_threshold) {
            return Err(CognitionError::OutOfRange {
                field: "pruning_threshold".to_string(),
                value: self.pruning_threshold,
                min: 0.0,
                max: 1.0,
            });
        }
        if self.max_concepts == 0 {
            return Err(CognitionError::InvalidState(
                "max_concepts must be greater than zero".to_string(),
            ));
        }
        Ok(())
    }
}

impl fmt::Debug for ConceptPruning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConceptPruning")
            .field("pruning_threshold", &self.pruning_threshold)
            .field("max_concepts", &self.max_concepts)
            .finish()
    }
}