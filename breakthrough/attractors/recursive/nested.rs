// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! # Nested Attractor
//!
//! A [`NestedAttractor`] encodes hierarchical attractor structure, where
//! an outer attractor contains inner attractors at different scales. Nested
//! structures appear in fractal geometry, renormalization group flows,
//! multi-scale control systems, and recursive ecosystem models.
//!
//! ## Embedding
//!
//! Each inner attractor is embedded within the outer attractor via a
//! scale-ratio and coordinate transformation. The embedding preserves
//! topological and dynamical properties under suitable conditions.
//!
//! ## Self-Similarity
//!
//! When the inner and outer attractors are dynamically similar up to
//! rescaling, the structure exhibits exact or approximate self-similarity,
//! a hallmark of fractal and critical systems.

use std::cmp::PartialEq;
use std::fmt::{self, Display, Formatter};

/// A nested attractor with inner and outer hierarchical structure.
///
/// # Fields
///
/// * `inner` - Coordinates or type identifier of the inner attractor.
/// * `outer` - Coordinates or type identifier of the outer attractor.
/// * `embedding` - Coordinate transformation matrix (flattened).
/// * `scale_ratio` - Ratio of outer to inner characteristic scale.
/// * `nesting_depth` - Number of nesting levels.
/// * `is_self_similar` - Whether inner and outer are dynamically similar.
/// * `overlap_fraction` - Fraction of inner volume contained within
///   the outer volume.
#[derive(Debug, Clone, PartialEq)]
pub struct NestedAttractor {
    pub inner: Vec<f64>,
    pub outer: Vec<f64>,
    pub embedding: Vec<f64>,
    pub scale_ratio: f64,
    pub nesting_depth: usize,
    pub is_self_similar: bool,
    pub overlap_fraction: f64,
}

impl NestedAttractor {
    /// Creates a new nested attractor with hierarchical embedding.
    ///
    /// # Arguments
    ///
    /// * `inner` - Inner attractor state vector.
    /// * `outer` - Outer attractor state vector.
    /// * `embedding` - Flattened linear transformation matrix.
    /// * `scale_ratio` - Outer-to-inner scale ratio.
    /// * `nesting_depth` - Number of nesting levels (must be >= 1).
    ///
    /// # Panics
    ///
    /// Panics if vectors are empty, embedding dimensions mismatch, or
    /// `scale_ratio` is non-positive.
    pub fn new(
        inner: Vec<f64>,
        outer: Vec<f64>,
        embedding: Vec<f64>,
        scale_ratio: f64,
        nesting_depth: usize,
    ) -> Self {
        assert!(!inner.is_empty(), "inner must be non-empty");
        assert!(!outer.is_empty(), "outer must be non-empty");
        assert_eq!(inner.len(), outer.len(), "inner and outer dimensions must match");
        assert!(scale_ratio > 0.0, "scale_ratio must be positive");
        assert!(nesting_depth >= 1, "nesting_depth must be at least 1");
        let dim = inner.len();
        assert_eq!(embedding.len(), dim * dim, "embedding must be square");
        Self {
            inner,
            outer,
            embedding,
            scale_ratio,
            nesting_depth,
            is_self_similar: false,
            overlap_fraction: 1.0,
        }
    }

    /// Returns the phase-space dimension.
    pub fn dimension(&self) -> usize {
        self.inner.len()
    }

    /// Applies the embedding transformation to the inner attractor state.
    ///
    /// This computes `outer = E · inner` where `E` is the embedding
    /// matrix.
    pub fn embed(&self) -> Vec<f64> {
        let dim = self.dimension();
        let mut result = vec![0.0; dim];
        for i in 0..dim {
            for j in 0..dim {
                result[i] += self.embedding[i * dim + j] * self.inner[j];
            }
        }
        result
    }

    /// Extracts the inner attractor from the outer state via the inverse
    /// embedding (approximate).
    ///
    /// # Arguments
    ///
    /// * `outer_state` - Outer attractor coordinates.
    ///
    /// # Returns
    ///
    /// Approximate inner coordinates.
    pub fn extract(&self, outer_state: &[f64]) -> Vec<f64> {
        assert_eq!(outer_state.len(), self.dimension(), "dimension mismatch");
        let dim = self.dimension();
        let mut result = vec![0.0; dim];
        for i in 0..dim {
            for j in 0..dim {
                result[i] += self.embedding[j * dim + i] * outer_state[j];
            }
        }
        result
    }

    /// Checks whether the nested attractor is approximately self-similar.
    ///
    /// Self-similarity is assessed by comparing the rescaled inner state
    /// to the outer state after embedding.
    pub fn is_self_similar(&mut self, tolerance: f64) -> bool {
        let embedded = self.embed();
        let diff: f64 = embedded.iter().zip(&self.outer).map(|(e, o)| (e - o).abs()).sum();
        let norm: f64 = self.outer.iter().map(|o| o.abs()).sum();
        let similarity = if norm > 1e-12 { 1.0 - diff / norm } else { 0.0 };
        self.is_self_similar = similarity >= 1.0 - tolerance;
        self.is_self_similar
    }

    /// Computes the characteristic scale of the inner attractor.
    pub fn inner_scale(&self) -> f64 {
        self.inner.iter().map(|v| v.abs()).fold(0.0_f64, |a, b| a + b) / self.inner.len() as f64
    }

    /// Computes the characteristic scale of the outer attractor.
    pub fn outer_scale(&self) -> f64 {
        self.outer.iter().map(|v| v.abs()).fold(0.0_f64, |a, b| a + b) / self.outer.len() as f64
    }

    /// Validates that the scale ratio matches the embedding magnitude.
    pub fn scale_consistency(&self) -> f64 {
        let inner_norm: f64 = self.inner.iter().map(|v| v.powi(2)).sum::<f64>().sqrt();
        let outer_norm: f64 = self.outer.iter().map(|v| v.powi(2)).sum::<f64>().sqrt();
        if inner_norm < 1e-12 { 0.0 } else { (outer_norm / inner_norm) / self.scale_ratio }
    }

    /// Returns a summary string.
    pub fn summary(&self) -> String {
        format!(
            "NestedAttractor(depth={}, scale_ratio={:.4}, self_similar={})",
            self.nesting_depth, self.scale_ratio, self.is_self_similar
        )
    }
}

impl Display for NestedAttractor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "NestedAttractor(depth={}, scale_ratio={:.6}, overlap={:.4})",
            self.nesting_depth, self.scale_ratio, self.overlap_fraction
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embed_identity() {
        let na = NestedAttractor::new(
            vec![1.0, 2.0],
            vec![1.0, 2.0],
            vec![1.0, 0.0, 0.0, 1.0],
            1.0,
            1,
        );
        let embedded = na.embed();
        assert!((embedded[0] - 1.0).abs() < 1e-10);
        assert!((embedded[1] - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_self_similarity_detection() {
        let mut na = NestedAttractor::new(
            vec![1.0, 0.0],
            vec![2.0, 0.0],
            vec![2.0, 0.0, 0.0, 2.0],
            2.0,
            1,
        );
        assert!(na.is_self_similar(1e-6));
    }

    #[test]
    fn test_scale_consistency() {
        let na = NestedAttractor::new(
            vec![1.0, 0.0],
            vec![3.0, 0.0],
            vec![3.0, 0.0, 0.0, 3.0],
            3.0,
            1,
        );
        assert!((na.scale_consistency() - 1.0).abs() < 1e-10);
    }
}
