// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Reconstruction: Memory retrieval, partial recall, and constructive hallucination.
//!
//! Memory is not a perfect playback but a constructive process. Retrieval
//! rebuilds a memory from fragmented traces, and this reconstruction can
//! introduce errors, embellishments, or entirely hallucinated content when
//! the original trace is degraded or absent.
//!
//! # Components
//!
//! - **MemoryRecall** — The retrieval pipeline that matches a query against
//!   stored memory candidates and reconstructs the best-fit memory.
//! - **MemoryHallucination** — Tracks the generation of content that departs
//!   from verified traces, measuring confidence and grounding.
//!
//! # Retrieval Dynamics
//!
//! Retrieval quality depends on cue strength, trace stability, and similarity
//! to stored memories. Partial matches yield lower fidelity, and highly
//! ambiguous cues may trigger hallucination.
//!
//! # Examples
//!
//! ```
//! use breakthrough::memory::reconstruction::{MemoryRecall, MemoryHallucination};
//!
//! let mut recall = MemoryRecall::new(vec![0.8, 0.2], 0.65, 0.5).expect("valid");
//! let result = recall.retrieve(&vec![(vec![0.7, 0.3], 0.9)]);
//! ```

use std::fmt;

use crate::MemoryError;

pub mod recall;
pub mod hallucination;

pub use recall::MemoryRecall;
pub use hallucination::MemoryHallucination;

/// Default retrieval similarity threshold.
pub const DEFAULT_RETRIEVAL_THRESHOLD: f64 = 0.65;
/// Default width of the reconstruction window.
pub const DEFAULT_RECONSTRUCTION_WINDOW: f64 = 0.5;
/// Minimum grounding score for a recall to be considered reliable.
pub const MIN_GROUNDING_SCORE: f64 = 0.4;
/// Maximum hallucination confidence that can be assigned.
pub const MAX_HALLUCINATION_CONFIDENCE: f64 = 0.3;

/// Creates a new memory recall engine with the given query and thresholds.
///
/// # Errors
///
/// Returns [`MemoryError::DimensionMismatch`] if the query is empty.
/// Returns [`MemoryError::OutOfRange`] if threshold or window is outside [0.0, 1.0].
pub fn create_recall(
    query: Vec<f64>,
    threshold: f64,
    window: f64,
) -> Result<MemoryRecall, MemoryError> {
    MemoryRecall::new(query, threshold, window)
}

/// Creates a new memory hallucination tracker.
///
/// # Errors
///
/// Returns [`MemoryError::DimensionMismatch`] if the original trace is empty.
pub fn create_hallucination(
    original_trace: Vec<f64>,
    hallucination_vector: Vec<f64>,
    grounding_score: f64,
) -> Result<MemoryHallucination, MemoryError> {
    MemoryHallucination::new(original_trace, hallucination_vector, grounding_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recall_creation_succeeds() {
        let r = create_recall(vec![0.8, 0.2], DEFAULT_RETRIEVAL_THRESHOLD, 0.5).unwrap();
        assert_eq!(r.query.len(), 2);
    }

    #[test]
    fn recall_retrieval_with_perfect_match() {
        let mut r = create_recall(vec![1.0, 0.0], 0.5, 0.5).unwrap();
        let traces = vec![(vec![1.0, 0.0], 0.9)];
        let result = r.retrieve(&traces);
        assert!(result.is_some());
        assert!(result.unwrap().1 >= 0.5);
    }

    #[test]
    fn recall_no_match_below_threshold() {
        let mut r = create_recall(vec![1.0, 0.0], 0.9, 0.5).unwrap();
        let traces = vec![(vec![0.0, 1.0], 0.5)];
        let result = r.retrieve(&traces);
        assert!(result.is_none());
    }

    #[test]
    fn hallucination_confidence_non_negative() {
        let h = create_hallucination(vec![1.0, 0.0], vec![0.2, 0.8], 0.6).unwrap();
        assert!(h.confidence() >= 0.0);
        assert!(h.confidence() <= 1.0);
    }

    #[test]
    fn hallucination_score_in_range() {
        let h = create_hallucination(vec![1.0, 0.0], vec![0.2, 0.8], 0.5).unwrap();
        let score = h.hallucination_score();
        assert!(score >= 0.0 && score <= 1.0);
    }

    #[test]
    fn default_constants_valid() {
        assert!(DEFAULT_RETRIEVAL_THRESHOLD > 0.0);
        assert!(DEFAULT_RECONSTRUCTION_WINDOW > 0.0);
    }
}
