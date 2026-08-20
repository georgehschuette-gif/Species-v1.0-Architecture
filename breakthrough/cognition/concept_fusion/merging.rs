// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

/// MergeStrategy: The strategy used to combine concept strengths.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MergeStrategy {
    /// The merged strength is the arithmetic mean of the inputs.
    WeightedAverage,
    /// The merged strength is the maximum of the inputs.
    Maximum,
    /// The merged strength is the sum of all strengths, capped at 1.0.
    Union,
}

/// ConceptMerging: Combining multiple concepts into one.
///
/// Merging unifies related concepts under a single umbrella,
/// using one of several strategies to determine the resulting
/// concept strength. The merge threshold determines how similar
/// two concepts must be before they can be merged.
///
/// # Fields
/// - `merge_threshold`: Minimum similarity required to trigger a merge, in [0.0, 1.0].
/// - `merge_strategy`: The strategy used to combine strengths.
///
/// # Example
/// ```
/// use breakthrough::cognition::concept_fusion::{ConceptMerging, MergeStrategy};
///
/// let merger = ConceptMerging::new(0.6, MergeStrategy::WeightedAverage)
///     .expect("valid parameters");
/// let combined = merger.merge(&[0.7, 0.8]).expect("sufficient similarity");
/// assert_eq!(combined, 0.75);
/// ```
pub struct ConceptMerging {
    /// Minimum similarity required to trigger a merge, in [0.0, 1.0].
    pub merge_threshold: f64,
    /// The strategy used to combine strengths.
    pub merge_strategy: MergeStrategy,
}

impl ConceptMerging {
    /// Creates a new `ConceptMerging` with the given threshold and strategy.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if `merge_threshold` is outside [0.0, 1.0].
    pub fn new(merge_threshold: f64, merge_strategy: MergeStrategy) -> Result<Self, CognitionError> {
        if !(0.0..=1.0).contains(&merge_threshold) {
            return Err(CognitionError::OutOfRange {
                field: "merge_threshold".to_string(),
                value: merge_threshold,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(Self {
            merge_threshold,
            merge_strategy,
        })
    }

    /// Merges the given concept strengths using the configured strategy.
    ///
    /// # Errors
    /// Returns [`CognitionError::MissingInput`] if the strengths slice is empty.
    /// Returns [`CognitionError::ThresholdNotMet`] if the average similarity
    /// is below the merge threshold.
    /// Returns [`CognitionError::IncompatibleConcepts`] if any strength
    /// is outside [0.0, 1.0].
    pub fn merge(&self, strengths: &[f64]) -> Result<f64, CognitionError> {
        if strengths.is_empty() {
            return Err(CognitionError::MissingInput(
                "cannot merge zero concepts".to_string(),
            ));
        }
        for &s in strengths {
            if !(0.0..=1.0).contains(&s) {
                return Err(CognitionError::IncompatibleConcepts {
                    reason: format!(
                        "strength {} is outside valid range [0.0, 1.0]",
                        s
                    ),
                });
            }
        }
        let avg_similarity = strengths.iter().sum::<f64>() / strengths.len() as f64;
        if avg_similarity < self.merge_threshold {
            return Err(CognitionError::ThresholdNotMet {
                threshold: self.merge_threshold,
                actual: avg_similarity,
            });
        }
        let result = match self.merge_strategy {
            MergeStrategy::WeightedAverage => avg_similarity,
            MergeStrategy::Maximum => *strengths
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                .unwrap_or(&0.0),
            MergeStrategy::Union => strengths.iter().sum::<f64>().min(1.0),
        };
        Ok(result)
    }

    /// Merges exactly two concepts, returning the combined strength
    /// and the similarity between them.
    ///
    /// # Errors
    /// Returns [`CognitionError::IncompatibleConcepts`] if either strength
    /// is outside [0.0, 1.0].
    pub fn merge_pair(
        &self,
        strength_a: f64,
        strength_b: f64,
    ) -> Result<(f64, f64), CognitionError> {
        if !(0.0..=1.0).contains(&strength_a) {
            return Err(CognitionError::IncompatibleConcepts {
                reason: format!("strength_a {} out of range", strength_a),
            });
        }
        if !(0.0..=1.0).contains(&strength_b) {
            return Err(CognitionError::IncompatibleConcepts {
                reason: format!("strength_b {} out of range", strength_b),
            });
        }
        let similarity = 1.0 - (strength_a - strength_b).abs();
        let combined = self.merge(&[strength_a, strength_b])?;
        Ok((combined, similarity))
    }

    /// Updates the merge threshold.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if the new threshold is outside [0.0, 1.0].
    pub fn set_threshold(&mut self, new_threshold: f64) -> Result<(), CognitionError> {
        if !(0.0..=1.0).contains(&new_threshold) {
            return Err(CognitionError::OutOfRange {
                field: "merge_threshold".to_string(),
                value: new_threshold,
                min: 0.0,
                max: 1.0,
            });
        }
        self.merge_threshold = new_threshold;
        Ok(())
    }

    /// Validates all field values are within valid ranges.
    pub fn validate(&self) -> Result<(), CognitionError> {
        if !(0.0..=1.0).contains(&self.merge_threshold) {
            return Err(CognitionError::OutOfRange {
                field: "merge_threshold".to_string(),
                value: self.merge_threshold,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(())
    }
}
