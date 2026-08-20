// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::*;

/// ConceptDifferentiation: Identifying distinguishing features between concepts.
///
/// Differentiation separates overlapping or conflated concepts by
/// discovering and amplifying their distinguishing features. This
/// process reduces ambiguity and sharpens conceptual boundaries.
///
/// # Fields
/// - `differentiation_threshold`: Minimum feature difference required to split, in [0.0, 1.0].
/// - `feature_weight`: Weighting factor applied to distinguishing features, in [0.0, 1.0].
///
/// # Example
/// ```
/// use breakthrough::cognition::concept_split::ConceptDifferentiation;
///
/// let mut diff = ConceptDifferentiation::new(0.3, 0.8).expect("valid parameters");
/// let score = diff.compute_distance(&[0.9, 0.1], &[0.2, 0.8]);
/// assert!(score > 0.0);
/// ```
pub struct ConceptDifferentiation {
    /// Minimum feature difference required to split, in [0.0, 1.0].
    pub differentiation_threshold: f64,
    /// Weighting factor applied to distinguishing features, in [0.0, 1.0].
    pub feature_weight: f64,
}

impl ConceptDifferentiation {
    /// Creates a new `ConceptDifferentiation` with the given threshold and weight.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if either parameter is outside [0.0, 1.0].
    pub fn new(
        differentiation_threshold: f64,
        feature_weight: f64,
    ) -> Result<Self, CognitionError> {
        if !(0.0..=1.0).contains(&differentiation_threshold) {
            return Err(CognitionError::OutOfRange {
                field: "differentiation_threshold".to_string(),
                value: differentiation_threshold,
                min: 0.0,
                max: 1.0,
            });
        }
        if !(0.0..=1.0).contains(&feature_weight) {
            return Err(CognitionError::OutOfRange {
                field: "feature_weight".to_string(),
                value: feature_weight,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(Self {
            differentiation_threshold,
            feature_weight,
        })
    }

    /// Computes the Euclidean distance between two feature vectors,
    /// weighted by the feature weight parameter.
    pub fn compute_distance(
        &self,
        features_a: &[f64],
        features_b: &[f64],
    ) -> f64 {
        assert!(
            !features_a.is_empty() && !features_b.is_empty(),
            "feature vectors must not be empty"
        );
        assert_eq!(
            features_a.len(),
            features_b.len(),
            "feature vectors have different lengths: {} vs {}",
            features_a.len(),
            features_b.len()
        );
        let squared_sum: f64 = features_a
            .iter()
            .zip(features_b.iter())
            .map(|(&a, &b)| {
                let diff = a - b;
                self.feature_weight * diff * diff
            })
            .sum();
        squared_sum.sqrt()
    }

    /// Determines whether two sets of features are sufficiently
    /// distinct to warrant differentiation (splitting).
    ///
    /// Returns `true` if the weighted distance exceeds the threshold.
    pub fn should_differentiate(
        &self,
        features_a: &[f64],
        features_b: &[f64],
    ) -> bool {
        let distance = self.compute_distance(features_a, features_b);
        distance >= self.differentiation_threshold
    }

    /// Identifies the indices of the most distinguishing features
    /// between two vectors.
    pub fn distinguishing_features(
        &self,
        features_a: &[f64],
        features_b: &[f64],
    ) -> Vec<usize> {
        let distance = self.compute_distance(features_a, features_b);
        if distance < self.differentiation_threshold {
            return Vec::new();
        }
        let indices: Vec<usize> = features_a
            .iter()
            .zip(features_b.iter())
            .enumerate()
            .filter(|(_, (&a, &b))| (a - b).abs() > self.differentiation_threshold)
            .map(|(i, _)| i)
            .collect();
        indices
    }

    /// Adjusts the differentiation threshold.
    ///
    /// # Errors
    /// Returns [`CognitionError::OutOfRange`] if the new threshold is outside [0.0, 1.0].
    pub fn set_threshold(&mut self, new_threshold: f64) -> Result<(), CognitionError> {
        if !(0.0..=1.0).contains(&new_threshold) {
            return Err(CognitionError::OutOfRange {
                field: "differentiation_threshold".to_string(),
                value: new_threshold,
                min: 0.0,
                max: 1.0,
            });
        }
        self.differentiation_threshold = new_threshold;
        Ok(())
    }

    /// Validates all field values are within valid ranges.
    pub fn validate(&self) -> Result<(), CognitionError> {
        if !(0.0..=1.0).contains(&self.differentiation_threshold) {
            return Err(CognitionError::OutOfRange {
                field: "differentiation_threshold".to_string(),
                value: self.differentiation_threshold,
                min: 0.0,
                max: 1.0,
            });
        }
        if !(0.0..=1.0).contains(&self.feature_weight) {
            return Err(CognitionError::OutOfRange {
                field: "feature_weight".to_string(),
                value: self.feature_weight,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(())
    }
}

impl fmt::Debug for ConceptDifferentiation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConceptDifferentiation")
            .field("differentiation_threshold", &self.differentiation_threshold)
            .field("feature_weight", &self.feature_weight)
            .finish()
    }
}