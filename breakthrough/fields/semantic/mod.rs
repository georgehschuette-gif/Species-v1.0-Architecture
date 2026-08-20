// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Semantic Field: Meaning-space distributions over entities and relations.
//! Encodes conceptual similarity, topical coherence, lexical semantics,
//! and vector-space representations of cognitive meaning.
//!
//! The semantic field module provides tools for computing similarity between
//! conceptual embeddings, measuring coherence within semantic clusters, and
//! assessing the structural properties of meaning distributions.

pub mod embedding;
pub mod similarity;
pub mod coherence;

pub use embedding::SemanticEmbedding;
pub use similarity::{SemanticSimilarity, SimilarityMetric};
pub use coherence::SemanticCoherence;

use std::fmt;

/// SemanticField: A container for a collection of semantic embeddings
/// representing a coherent meaning-space region.
#[derive(Debug, Clone, PartialEq)]
pub struct SemanticField {
    pub name: String,
    pub embeddings: Vec<SemanticEmbedding>,
    pub dimensionality: usize,
}

impl SemanticField {
    pub fn new(name: String, dimensionality: usize) -> Self {
        Self { name, embeddings: Vec::new(), dimensionality }
    }

    pub fn add_embedding(&mut self, embedding: SemanticEmbedding) {
        if embedding.dimensions == self.dimensionality {
            self.embeddings.push(embedding);
        }
    }

    pub fn centroid(&self) -> Option<SemanticEmbedding> {
        if self.embeddings.is_empty() {
            return None;
        }
        let dims = self.dimensionality;
        let mut sum = vec![0.0f64; dims];
        for emb in &self.embeddings {
            for (i, &v) in emb.values.iter().enumerate() {
                sum[i] += v;
            }
        }
        let n = self.embeddings.len() as f64;
        let values: Vec<f64> = sum.iter().map(|&s| s / n).collect();
        SemanticEmbedding::new(dims, values).ok()
    }

    pub fn embedding_count(&self) -> usize {
        self.embeddings.len()
    }

    pub fn is_empty(&self) -> bool {
        self.embeddings.is_empty()
    }
}

impl Default for SemanticField {
    fn default() -> Self {
        Self::new(String::from("unnamed"), 3)
    }
}

/// Errors that can occur during semantic field operations.
#[derive(Debug, Clone, PartialEq)]
pub enum SemanticError {
    OutOfRange { field: String, value: f64, min: f64, max: f64 },
    InvalidDimension { dimension: usize },
    DimensionMismatch { expected: usize, actual: usize },
    InvalidThreshold { threshold: f64 },
    InvalidDensity { density: f64 },
    InvalidAlignment { alignment: f64 },
    InvalidNoise { noise: f64 },
    InsufficientData,
    ComputationError(String),
}

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SemanticError::OutOfRange { field, value, min, max } => write!(f, "field '{}' value {} is out of range [{}, {}]", field, value, min, max),
            SemanticError::InvalidDimension { dimension } => write!(f, "invalid dimension: {}", dimension),
            SemanticError::DimensionMismatch { expected, actual } => write!(f, "dimension mismatch: expected {}, got {}", expected, actual),
            SemanticError::InvalidThreshold { threshold } => write!(f, "invalid threshold: {}", threshold),
            SemanticError::InvalidDensity { density } => write!(f, "invalid density: {}", density),
            SemanticError::InvalidAlignment { alignment } => write!(f, "invalid alignment: {}", alignment),
            SemanticError::InvalidNoise { noise } => write!(f, "invalid noise: {}", noise),
            SemanticError::InsufficientData => write!(f, "insufficient data for operation"),
            SemanticError::ComputationError(msg) => write!(f, "computation error: {}", msg),
        }
    }
}

impl std::error::Error for SemanticError {}

/// Result type alias for semantic field operations.
pub type SemanticResult<T> = Result<T, SemanticError>;