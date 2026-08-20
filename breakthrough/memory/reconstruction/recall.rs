// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::MemoryError;

use super::*;

/// MemoryRecall: The retrieval pipeline for accessing stored memories.
///
/// Transforms a query vector into a retrieved memory by matching against
/// candidate traces and reconstructing the best-fitting candidate within
/// the reconstruction window.
#[derive(Clone, PartialEq)]
pub struct MemoryRecall {
    /// Query vector used for retrieval.
    pub query: Vec<f64>,
    /// Similarity threshold for a match to be considered valid.
    pub retrieval_threshold: f64,
    /// Half-width of the reconstruction window.
    pub reconstruction_window: f64,
    /// Number of retrieval attempts performed.
    pub attempt_count: usize,
    /// Best similarity score found so far.
    pub best_score: f64,
}

impl MemoryRecall {
    /// Minimum valid query length.
    pub const MIN_QUERY_LEN: usize = 1;
    /// Maximum valid query length.
    pub const MAX_QUERY_LEN: usize = 4096;

    /// Creates a new MemoryRecall with the given query and thresholds.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::DimensionMismatch`] if query is empty or exceeds
    /// max length.
    /// Returns [`MemoryError::OutOfRange`] if threshold or window is outside [0.0, 1.0].
    pub fn new(
        query: Vec<f64>,
        retrieval_threshold: f64,
        reconstruction_window: f64,
    ) -> Result<Self, MemoryError> {
        let len = query.len();
        if len == 0 || len > Self::MAX_QUERY_LEN {
            return Err(MemoryError::DimensionMismatch {
                expected: Self::MIN_QUERY_LEN,
                actual: len,
            });
        }
        if !(0.0..=1.0).contains(&retrieval_threshold) {
            return Err(MemoryError::OutOfRange {
                field: "retrieval_threshold".into(),
                value: retrieval_threshold,
                min: 0.0,
                max: 1.0,
            });
        }
        if !(0.0..=1.0).contains(&reconstruction_window) {
            return Err(MemoryError::OutOfRange {
                field: "reconstruction_window".into(),
                value: reconstruction_window,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(Self {
            query,
            retrieval_threshold,
            reconstruction_window,
            attempt_count: 0,
            best_score: 0.0,
        })
    }

    /// Retrieves the best-matching memory candidate from a store.
    ///
    /// Candidates are (trace_vector, strength) tuples. Returns the best
    /// match whose similarity exceeds the retrieval threshold.
    pub fn retrieve(&mut self, store: &[(Vec<f64>, f64)]) -> Option<(Vec<f64>, f64)> {
        self.attempt_count += 1;
        let query_norm = self.query.iter().map(|v| v * v).sum::<f64>().sqrt();
        if query_norm == 0.0 {
            return None;
        }
        let mut best: Option<(Vec<f64>, f64, f64)> = None;
        for (trace, strength) in store {
            let sim = self.cosine_similarity(trace, query_norm);
            if sim > self.best_score {
                self.best_score = sim;
            }
            if sim >= self.retrieval_threshold {
                let fitness = sim * strength;
                let should_replace = match &best {
                    None => true,
                    Some((_, _, f)) => fitness > *f,
                };
                if should_replace {
                    best = Some((trace.clone(), *strength, fitness));
                }
            }
        }
        best.map(|(t, s, _)| (t, s))
    }

    /// Reconstructs a partial memory from the best candidate and query.
    ///
    /// Blends the query and retrieved trace weighted by the reconstruction window.
    pub fn reconstruct(&self, trace: &[f64]) -> Vec<f64> {
        if trace.len() != self.query.len() {
            return self.query.clone();
        }
        self.query
            .iter()
            .zip(trace)
            .map(|(&q, &t)| {
                q * (1.0 - self.reconstruction_window) + t * self.reconstruction_window
            })
            .collect()
    }

    /// Returns the retrieval quality as a function of best score and threshold.
    pub fn retrieval_quality(&self) -> f64 {
        if self.retrieval_threshold == 0.0 {
            return 1.0;
        }
        (self.best_score / self.retrieval_threshold).clamp(0.0, 1.0)
    }

    /// Checks whether the query partially matches any candidate.
    pub fn partial_match(&self, store: &[(Vec<f64>, f64)]) -> bool {
        let query_norm = self.query.iter().map(|v| v * v).sum::<f64>().sqrt();
        if query_norm == 0.0 {
            return false;
        }
        store.iter().any(|(trace, _)| {
            self.cosine_similarity(trace, query_norm) >= self.retrieval_threshold * 0.5
        })
    }

    /// Sets a new query vector for retrieval.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::DimensionMismatch`] if query length differs from
    /// the current query.
    pub fn set_query(&mut self, query: Vec<f64>) -> Result<(), MemoryError> {
        if query.len() != self.query.len() {
            return Err(MemoryError::DimensionMismatch {
                expected: self.query.len(),
                actual: query.len(),
            });
        }
        self.query = query;
        self.best_score = 0.0;
        Ok(())
    }

    /// Computes cosine similarity between trace and query.
    fn cosine_similarity(&self, trace: &[f64], query_norm: f64) -> f64 {
        if trace.len() != self.query.len() {
            return 0.0;
        }
        let dot: f64 = trace.iter().zip(&self.query).map(|(t, q)| t * q).sum();
        let trace_norm = trace.iter().map(|v| v * v).sum::<f64>().sqrt();
        let denom = trace_norm * query_norm;
        if denom == 0.0 {
            0.0
        } else {
            (dot / denom).clamp(0.0, 1.0)
        }
    }

    /// Validates the recall state.
    pub fn validate(&self) -> Result<(), MemoryError> {
        Self::new(
            self.query.clone(),
            self.retrieval_threshold,
            self.reconstruction_window,
        )?;
        Ok(())
    }
}

impl fmt::Debug for MemoryRecall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MemoryRecall")
            .field("query_len", &self.query.len())
            .field("retrieval_threshold", &self.retrieval_threshold)
            .field("reconstruction_window", &self.reconstruction_window)
            .field("attempt_count", &self.attempt_count)
            .field("best_score", &self.best_score)
            .finish()
    }
}
