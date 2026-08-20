// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// SemanticEmbedding: Vector representation of conceptual meaning.
#[derive(Debug, Clone, PartialEq)]
pub struct SemanticEmbedding {
    pub dimensions: usize,
    pub values: Vec<f64>,
    pub norm: f64,
}

impl SemanticEmbedding {
    pub const MIN_DIMENSIONS: usize = 1;
    pub const MAX_DIMENSIONS: usize = 4096;

    pub fn new(dimensions: usize, values: Vec<f64>) -> Result<Self, SemanticError> {
        if dimensions < Self::MIN_DIMENSIONS || dimensions > Self::MAX_DIMENSIONS {
            return Err(SemanticError::InvalidDimension { dimension: dimensions });
        }
        if values.len() != dimensions {
            return Err(SemanticError::DimensionMismatch {
                expected: dimensions,
                actual: values.len(),
            });
        }
        let norm = values.iter().map(|v| v * v).sum::<f64>().sqrt();
        Ok(Self { dimensions, values, norm })
    }

    pub fn similarity_to(&self, other: &SemanticEmbedding) -> Result<f64, SemanticError> {
        if self.dimensions != other.dimensions {
            return Err(SemanticError::DimensionMismatch {
                expected: self.dimensions,
                actual: other.dimensions,
            });
        }
        if self.norm == 0.0 || other.norm == 0.0 {
            return Ok(0.0);
        }
        let dot: f64 = self.values.iter().zip(&other.values).map(|(a, b)| a * b).sum();
        Ok(dot / (self.norm * other.norm))
    }

    pub fn normalize(&mut self) {
        if self.norm > 0.0 {
            for v in &mut self.values {
                *v /= self.norm;
            }
            self.norm = 1.0;
        }
    }

    pub fn validate(&self) -> Result<(), SemanticError> {
        Self::new(self.dimensions, self.values.clone())?;
        Ok(())
    }
}

impl Default for SemanticEmbedding {
    fn default() -> Self {
        Self { dimensions: 0, values: Vec::new(), norm: 0.0 }
    }
}
