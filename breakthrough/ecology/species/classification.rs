// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// SpeciesClassifier: Categorizes entities into distinct species types.
pub struct SpeciesClassifier {
    pub classification_depth: usize,
    pub feature_weights: Vec<f64>,
}

impl SpeciesClassifier {
    /// Constructs a new SpeciesClassifier with the given depth and feature weights.
    ///
    /// Validates that feature weights are non-empty and classification depth is non-zero.
    pub fn new(depth: usize, feature_weights: Vec<f64>) -> Result<Self, SpeciesError> {
        let classifier = Self {
            classification_depth: depth,
            feature_weights,
        };
        classifier.validate()?;
        Ok(classifier)
    }

    /// Constructs a SpeciesClassifier with a default single feature weight of 1.0.
    pub fn default() -> Self {
        Self {
            classification_depth: 1,
            feature_weights: vec![1.0],
        }
    }

    /// Validates the classifier's internal state.
    ///
    /// Returns an error if the feature weights are empty or contain NaN/Inf values,
    /// or if the classification depth is zero.
    pub fn validate(&self) -> Result<(), SpeciesError> {
        if self.classification_depth == 0 {
            return Err(SpeciesError::InvalidClassification(
                "Classification depth must be non-zero".to_string(),
            ));
        }
        if self.feature_weights.is_empty() {
            return Err(SpeciesError::InvalidClassification(
                "Feature weights must not be empty".to_string(),
            ));
        }
        for (i, w) in self.feature_weights.iter().enumerate() {
            if w.is_nan() {
                return Err(SpeciesError::InvalidClassification(format!(
                    "Feature weight at index {} is NaN",
                    i
                )));
            }
            if w.is_infinite() {
                return Err(SpeciesError::InvalidClassification(format!(
                    "Feature weight at index {} is infinite",
                    i
                )));
            }
        }
        Ok(())
    }

    /// Classifies a feature vector and returns the corresponding SpeciesId.
    ///
    /// The feature vector must have the same length as the classifier's feature weights.
    /// Returns an error if the feature vector is empty, has mismatched length,
    /// or contains NaN/Inf values.
    pub fn classify(&self, features: &[f64]) -> Result<SpeciesId, SpeciesError> {
        if features.is_empty() {
            return Err(SpeciesError::InvalidClassification(
                "Feature vector must not be empty".to_string(),
            ));
        }
        if features.len() != self.feature_weights.len() {
            return Err(SpeciesError::InvalidClassification(format!(
                "Feature vector length {} does not match weight count {}",
                features.len(),
                self.feature_weights.len()
            )));
        }
        for (i, v) in features.iter().enumerate() {
            if v.is_nan() {
                return Err(SpeciesError::InvalidClassification(format!(
                    "Feature value at index {} is NaN",
                    i
                )));
            }
            if v.is_infinite() {
                return Err(SpeciesError::InvalidClassification(format!(
                    "Feature value at index {} is infinite",
                    i
                )));
            }
        }
        let id = self.compute_classification_id(features);
        Ok(SpeciesId(id))
    }

    /// Computes a similarity score between a feature vector and the classifier's weights.
    ///
    /// The score is a weighted cosine similarity normalized to [0.0, 1.0].
    pub fn compute_similarity(&self, features: &[f64]) -> f64 {
        if features.is_empty() || self.feature_weights.is_empty() {
            return 0.0;
        }
        let min_len = features.len().min(self.feature_weights.len());
        let mut dot_product = 0.0;
        let mut norm_features = 0.0;
        let mut norm_weights = 0.0;
        for i in 0..min_len {
            dot_product += features[i] * self.feature_weights[i];
            norm_features += features[i] * features[i];
            norm_weights += self.feature_weights[i] * self.feature_weights[i];
        }
        let denom = (norm_features * norm_weights).sqrt();
        if denom == 0.0 {
            0.0
        } else {
            dot_product / denom
        }
    }

    /// Adds a feature weight to the classifier.
    ///
    /// Returns an error if the weight is NaN or infinite.
    pub fn add_feature_weight(&mut self, weight: f64) -> Result<(), SpeciesError> {
        if weight.is_nan() {
            return Err(SpeciesError::InvalidClassification(
                "Feature weight cannot be NaN".to_string(),
            ));
        }
        if weight.is_infinite() {
            return Err(SpeciesError::InvalidClassification(
                "Feature weight cannot be infinite".to_string(),
            ));
        }
        self.feature_weights.push(weight);
        Ok(())
    }

    /// Sets the classification depth with validation.
    ///
    /// Returns an error if depth is zero.
    pub fn set_classification_depth(&mut self, depth: usize) -> Result<(), SpeciesError> {
        if depth == 0 {
            return Err(SpeciesError::InvalidClassification(
                "Classification depth must be non-zero".to_string(),
            ));
        }
        self.classification_depth = depth;
        Ok(())
    }

    /// Returns the number of feature dimensions.
    pub fn feature_count(&self) -> usize {
        self.feature_weights.len()
    }

    /// Returns the classification confidence based on weight uniformity.
    ///
    /// A confidence of 1.0 indicates uniform weights; lower values indicate
    /// skewed weight distributions.
    pub fn classification_confidence(&self) -> f64 {
        if self.feature_weights.is_empty() {
            return 0.0;
        }
        let n = self.feature_weights.len() as f64;
        let sum: f64 = self.feature_weights.iter().sum();
        let mean = sum / n;
        if mean == 0.0 {
            return 0.0;
        }
        let variance: f64 = self
            .feature_weights
            .iter()
            .map(|w| (w - mean).powi(2))
            .sum::<f64>()
            / n;
        let cv = (variance.sqrt() / mean).min(1.0);
        1.0 - cv
    }

    fn compute_classification_id(&self, features: &[f64]) -> u64 {
        let mut hash: u64 = self.classification_depth as u64;
        for (i, feature) in features.iter().enumerate() {
            let bits = feature.to_bits();
            hash = hash.wrapping_mul(31).wrapping_add(bits);
            hash = hash.wrapping_mul(31).wrapping_add(i as u64);
        }
        hash
    }
}