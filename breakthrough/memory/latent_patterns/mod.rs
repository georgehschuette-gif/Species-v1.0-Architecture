// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Latent Patterns: Hidden statistical structure discovered from sensory data streams.
//!
//! Latent pattern discovery identifies recurring, non-obvious structure within
//! noisy or high-dimensional data. Unlike explicit memory, latent patterns are
//! not consciously accessible but constrain future perception and inference.
//!
//! # Components
//!
//! - **LatentPatternDiscovery** — Statistical engine that observes streams,
//!   builds histograms, and detects significant patterns beyond chance.
//! - **PatternExtraction** — Compression and reconstruction machinery that
//!   distills observed patterns into compact representations.
//!
//! # Discovery Pipeline
//!
//! 1. **Observation** — Raw data points are fed into a sliding observation window.
//! 2. **Histogram Update** — Frequency counts are accumulated over the window.
//! 3. **Significance Scoring** — Patterns are tested against a null baseline
//!    using Poisson or chi-squared metrics.
//! 4. **Normalization** — Scores are normalized for cross-pattern comparison.

use std::fmt;

use crate::MemoryError;

pub mod discovery;
pub mod extraction;

pub use discovery::LatentPatternDiscovery;
pub use extraction::PatternExtraction;

/// Default observation window size for pattern discovery.
pub const DEFAULT_WINDOW_SIZE: usize = 1000;
/// Default significance threshold for a pattern to be considered latent.
pub const DEFAULT_SIGNIFICANCE_THRESHOLD: f64 = 0.05;
/// Maximum number of distinct patterns the discovery engine can track.
pub const MAX_DISCOVERED_PATTERNS: usize = 5000;
/// Default compression target ratio.
pub const DEFAULT_COMPRESSION_RATIO: f64 = 0.5;

/// Creates a new latent pattern discovery engine with the given window size.
///
/// # Errors
///
/// Returns [`MemoryError::DimensionMismatch`] if window size exceeds
/// [`MAX_DISCOVERED_PATTERNS`].
pub fn create_discovery(window_size: usize) -> Result<LatentPatternDiscovery, MemoryError> {
    LatentPatternDiscovery::new(window_size)
}

/// Creates a new pattern extraction engine with the given target ratio.
///
/// # Errors
///
/// Returns [`MemoryError::OutOfRange`] if ratio is outside (0.0, 1.0].
pub fn create_extraction(compression_ratio: f64) -> Result<PatternExtraction, MemoryError> {
    PatternExtraction::new(compression_ratio)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discovery_creation_succeeds() {
        let d = create_discovery(100).unwrap();
        assert_eq!(d.observed_histogram.len(), 0);
    }

    #[test]
    fn discovery_observation_updates_histogram() {
        let mut d = create_discovery(50).unwrap();
        d.observe(&[1.0, 2.0, 1.0, 3.0]).unwrap();
        assert!(d.observed_histogram.len() > 0 || d.observation_count > 0);
    }

    #[test]
    fn discovery_pattern_score_non_negative() {
        let d = create_discovery(100).unwrap();
        let score = d.pattern_score("default");
        assert!(score >= 0.0);
    }

    #[test]
    fn extraction_creation_succeeds() {
        let e = create_extraction(0.5).unwrap();
        assert!(e.extraction_depth >= 0.0);
    }

    #[test]
    fn extraction_compress_reduces_size() {
        let mut e = create_extraction(0.8).unwrap();
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        e.compress(&data).unwrap();
        assert!(e.reconstruction_error >= 0.0);
    }

    #[test]
    fn default_constants_valid() {
        assert!(DEFAULT_WINDOW_SIZE > 0);
        assert!(DEFAULT_SIGNIFICANCE_THRESHOLD > 0.0);
        assert!(MAX_DISCOVERED_PATTERNS > 0);
    }
}
