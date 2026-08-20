// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::MemoryError;

use super::*;

/// SemanticConcept: A named cognitive concept with feature vectors and associations.
///
/// Concepts are the nodes of the semantic knowledge graph. Each concept carries
/// a feature vector describing its meaning, an activation level governing its
/// accessibility, and associative links to related concepts.
#[derive(Debug, Clone, PartialEq)]
pub struct SemanticConcept {
    /// Name or label of the concept.
    pub name: String,
    /// Feature vector encoding the concept's meaning.
    pub features: Vec<f64>,
    /// Current activation level in [0.0, 1.0].
    pub activation: f64,
    /// Number of associative links to other concepts.
    pub link_count: usize,
    /// Associative strength weighted sum in [0.0, 1.0].
    pub associative_strength: f64,
    /// Number of times this concept has been activated.
    pub access_count: usize,
}

impl SemanticConcept {
    /// Minimum valid feature count.
    pub const MIN_FEATURES: usize = 0;
    /// Maximum valid feature count.
    pub const MAX_FEATURES: usize = MAX_CONCEPT_FEATURES;

    /// Creates a new SemanticConcept with the given name.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::MissingInput`] if the name is empty.
    pub fn new(name: impl Into<String>) -> Result<Self, MemoryError> {
        let n = name.into();
        if n.is_empty() {
            return Err(MemoryError::MissingInput("concept name must not be empty".into()));
        }
        Ok(Self {
            name: n,
            features: Vec::new(),
            activation: 0.0,
            link_count: 0,
            associative_strength: 0.0,
            access_count: 0,
        })
    }

    /// Activates the concept, increasing its accessibility.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::OutOfRange`] if amount is outside [0.0, 1.0].
    pub fn activate(&mut self, amount: f64) -> Result<(), MemoryError> {
        if !(0.0..=1.0).contains(&amount) {
            return Err(MemoryError::OutOfRange {
                field: "amount".into(),
                value: amount,
                min: 0.0,
                max: 1.0,
            });
        }
        self.activation = (self.activation + amount).min(1.0);
        self.access_count += 1;
        Ok(())
    }

    /// Links a feature to this concept, expanding its feature vector.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::DimensionMismatch`] if feature count exceeds
    /// [`MAX_CONCEPT_FEATURES`].
    pub fn link_feature(&mut self, feature: f64) -> Result<(), MemoryError> {
        if self.features.len() >= Self::MAX_FEATURES {
            return Err(MemoryError::DimensionMismatch {
                expected: Self::MAX_FEATURES,
                actual: self.features.len() + 1,
            });
        }
        self.features.push(feature);
        let norm = self.features.iter().map(|v| v * v).sum::<f64>().sqrt();
        if norm > 0.0 {
            self.associative_strength = self
                .features
                .iter()
                .map(|v| (v / norm).abs())
                .sum::<f64>()
                / self.features.len() as f64;
        }
        Ok(())
    }

    /// Links this concept to another concept, increasing associative density.
    pub fn link_concept(&mut self, other: &SemanticConcept) -> Result<(), MemoryError> {
        self.link_count += 1;
        self.associative_strength = (self.associative_strength + 0.05).min(1.0);
        let _ = other;
        Ok(())
    }

    /// Computes semantic distance to another concept via feature cosine similarity.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::DimensionMismatch`] if feature lengths differ.
    pub fn semantic_distance(&self, other: &SemanticConcept) -> Result<f64, MemoryError> {
        if self.features.len() != other.features.len() {
            return Err(MemoryError::DimensionMismatch {
                expected: self.features.len(),
                actual: other.features.len(),
            });
        }
        if self.features.is_empty() {
            return Ok(1.0);
        }
        let dot: f64 = self
            .features
            .iter()
            .zip(&other.features)
            .map(|(a, b)| a * b)
            .sum();
        let norm_a = self.features.iter().map(|v| v * v).sum::<f64>().sqrt();
        let norm_b = other.features.iter().map(|v| v * v).sum::<f64>().sqrt();
        let denom = norm_a * norm_b;
        if denom == 0.0 {
            Ok(0.0)
        } else {
            Ok((dot / denom).clamp(0.0, 1.0))
        }
    }

    /// Returns the associative density (links per feature).
    pub fn associative_density(&self) -> f64 {
        if self.features.is_empty() {
            return 0.0;
        }
        self.link_count as f64 / self.features.len() as f64
    }

    /// Decays activation passively over time.
    pub fn decay(&mut self, rate: f64) {
        self.activation = (self.activation * (1.0 - rate)).clamp(0.0, 1.0);
    }

    /// Validates the concept state.
    pub fn validate(&self) -> Result<(), MemoryError> {
        if self.name.is_empty() {
            return Err(MemoryError::MissingInput(
                "concept name must not be empty".into(),
            ));
        }
        if self.features.len() > Self::MAX_FEATURES {
            return Err(MemoryError::DimensionMismatch {
                expected: Self::MAX_FEATURES,
                actual: self.features.len(),
            });
        }
        if !(0.0..=1.0).contains(&self.activation) {
            return Err(MemoryError::OutOfRange {
                field: "activation".into(),
                value: self.activation,
                min: 0.0,
                max: 1.0,
            });
        }
        if !(0.0..=1.0).contains(&self.associative_strength) {
            return Err(MemoryError::OutOfRange {
                field: "associative_strength".into(),
                value: self.associative_strength,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(())
    }
}
