// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::MemoryError;

use super::*;

/// PatternExtraction: Compression and reconstruction of observed latent patterns.
///
/// Distills high-dimensional or high-volume observations into compact
/// representations that preserve the essential structure. Measures fidelity
/// through reconstruction error and information gain metrics.
#[derive(Debug, Clone, PartialEq)]
pub struct PatternExtraction {
    /// Compressed representation of observed patterns.
    pub compressed_representation: Vec<f64>,
    /// Mean squared reconstruction error after compression.
    pub reconstruction_error: f64,
    /// Target compression ratio (0.0 = no compression, 1.0 = maximal).
    pub extraction_depth: f64,
    /// Number of original data points processed.
    pub original_count: usize,
    /// Number of extracted latent features.
    pub extracted_features: usize,
    /// Target compression ratio.
    pub target_ratio: f64,
}

impl PatternExtraction {
    /// Creates a new PatternExtraction with the given compression ratio.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::OutOfRange`] if ratio is not in (0.0, 1.0].
    pub fn new(compression_ratio: f64) -> Result<Self, MemoryError> {
        if !(0.0..=1.0).contains(&compression_ratio) || compression_ratio == 0.0 {
            return Err(MemoryError::OutOfRange {
                field: "compression_ratio".into(),
                value: compression_ratio,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(Self {
            compressed_representation: Vec::new(),
            reconstruction_error: 0.0,
            extraction_depth: 0.0,
            original_count: 0,
            extracted_features: 0,
            target_ratio: compression_ratio,
        })
    }

    /// Compresses a slice of data into a compact representation.
    ///
    /// Uses dimensionality reduction by retaining a subset of features based
    /// on the target ratio and computing a centroid representation.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::MissingInput`] if data is empty.
    pub fn compress(&mut self, data: &[f64]) -> Result<(), MemoryError> {
        if data.is_empty() {
            return Err(MemoryError::MissingInput(
                "compression data must not be empty".into(),
            ));
        }
        self.original_count += data.len();
        let retain_count = (data.len() as f64 * self.target_ratio).ceil() as usize;
        let retain_count = retain_count.max(1).min(data.len());
        let mut sorted: Vec<(usize, f64)> = data.iter().copied().enumerate().collect();
        sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        self.compressed_representation = sorted.into_iter().take(retain_count).map(|(_, v)| v).collect();
        self.extracted_features = retain_count;
        self.extraction_depth = retain_count as f64 / data.len() as f64;
        self.reconstruction_error = self.estimate_reconstruction_error(data);
        Ok(())
    }

    /// Reconstructs data from the compressed representation, returning the
    /// reconstructed vector of the original length.
    pub fn reconstruct(&self, original_length: usize) -> Vec<f64> {
        if original_length == 0 || self.compressed_representation.is_empty() {
            return vec![0.0; original_length];
        }
        let mut reconstructed = vec![0.0; original_length];
        let src = &self.compressed_representation;
        for i in 0..original_length {
            let src_idx = i % src.len();
            reconstructed[i] = src[src_idx];
        }
        reconstructed
    }

    /// Returns the information gain relative to the original data volume.
    pub fn information_gain(&self) -> f64 {
        if self.original_count == 0 {
            return 0.0;
        }
        let compressed_size = self.compressed_representation.len();
        let ratio = compressed_size as f64 / self.original_count as f64;
        (1.0 - ratio).max(0.0)
    }

    /// Returns the fidelity score (1.0 - normalized reconstruction error).
    pub fn fidelity_score(&self) -> f64 {
        (1.0 - self.reconstruction_error).clamp(0.0, 1.0)
    }

    /// Estimates reconstruction error by comparing centroids.
    fn estimate_reconstruction_error(&self, data: &[f64]) -> f64 {
        if data.is_empty() || self.compressed_representation.is_empty() {
            return 0.0;
        }
        let original_mean = data.iter().sum::<f64>() / data.len() as f64;
        let compressed_mean: f64 = self.compressed_representation.iter().sum::<f64>()
            / self.compressed_representation.len() as f64;
        let diff = (original_mean - compressed_mean).abs();
        diff.clamp(0.0, 1.0)
    }

    /// Validates the extraction state.
    pub fn validate(&self) -> Result<(), MemoryError> {
        Self::new(self.target_ratio)?;
        if self.reconstruction_error < 0.0 {
            return Err(MemoryError::OutOfRange {
                field: "reconstruction_error".into(),
                value: self.reconstruction_error,
                min: 0.0,
                max: f64::INFINITY,
            });
        }
        if self.extracted_features > MAX_DISCOVERED_PATTERNS {
            return Err(MemoryError::CapacityExceeded {
                max: MAX_DISCOVERED_PATTERNS,
                attempted: self.extracted_features,
            });
        }
        Ok(())
    }
}
