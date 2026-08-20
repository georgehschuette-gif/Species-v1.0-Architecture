// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// SimilarityMetric: The distance or similarity measure used for comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimilarityMetric {
    Cosine,
    Euclidean,
    DotProduct,
    Manhattan,
}

impl SimilarityMetric {
    pub fn as_str(&self) -> &'static str {
        match self {
            SimilarityMetric::Cosine => "cosine",
            SimilarityMetric::Euclidean => "euclidean",
            SimilarityMetric::DotProduct => "dot_product",
            SimilarityMetric::Manhattan => "manhattan",
        }
    }

    pub fn supports_dimensionality_check(&self) -> bool {
        matches!(self, SimilarityMetric::Cosine | SimilarityMetric::Euclidean | SimilarityMetric::DotProduct | SimilarityMetric::Manhattan)
    }
}

/// SemanticSimilarity: Measure of conceptual overlap between embeddings.
pub struct SemanticSimilarity {
    pub metric: SimilarityMetric,
    pub threshold: f64,
}

impl SemanticSimilarity {
    pub fn new(metric: SimilarityMetric, threshold: f64) -> Result<Self, SemanticError> {
        if !(0.0..=1.0).contains(&threshold) {
            return Err(SemanticError::InvalidThreshold { threshold });
        }
        Ok(Self { metric, threshold })
    }

    pub fn compute(&self, a: &SemanticEmbedding, b: &SemanticEmbedding) -> Result<f64, SemanticError> {
        match self.metric {
            SimilarityMetric::Cosine => a.similarity_to(b),
            SimilarityMetric::Euclidean => {
                if a.dimensions != b.dimensions {
                    return Err(SemanticError::DimensionMismatch {
                        expected: a.dimensions,
                        actual: b.dimensions,
                    });
                }
                let dist: f64 = a.values.iter().zip(&b.values).map(|(x, y)| (x - y).powi(2)).sum::<f64>().sqrt();
                Ok(1.0 / (1.0 + dist))
            }
            SimilarityMetric::DotProduct => {
                if a.dimensions != b.dimensions {
                    return Err(SemanticError::DimensionMismatch {
                        expected: a.dimensions,
                        actual: b.dimensions,
                    });
                }
                Ok(a.values.iter().zip(&b.values).map(|(x, y)| x * y).sum())
            }
            SimilarityMetric::Manhattan => {
                if a.dimensions != b.dimensions {
                    return Err(SemanticError::DimensionMismatch {
                        expected: a.dimensions,
                        actual: b.dimensions,
                    });
                }
                let dist: f64 = a.values.iter().zip(&b.values).map(|(x, y)| (x - y).abs()).sum();
                Ok(1.0 / (1.0 + dist))
            }
        }
    }

    pub fn is_similar(&self, a: &SemanticEmbedding, b: &SemanticEmbedding) -> Result<bool, SemanticError> {
        let sim = self.compute(a, b)?;
        Ok(sim >= self.threshold)
    }

    pub fn threshold_for(&self, metric: SimilarityMetric) -> f64 {
        match metric {
            SimilarityMetric::Cosine => 0.85,
            SimilarityMetric::Euclidean => 0.75,
            SimilarityMetric::DotProduct => 0.6,
            SimilarityMetric::Manhattan => 0.7,
        }
    }
}
