// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::MemoryError;

use super::*;

/// MemoryHallucination: Tracks generated content that departs from verified traces.
///
/// When memory retrieval fails to find a close match, the system may construct
/// a plausible but unverified memory. Hallucination tracking measures how far
/// the reconstructed content deviates from any known trace and assigns a
/// confidence and grounding score.
#[derive(Debug, Clone, PartialEq)]
pub struct MemoryHallucination {
    /// Original trace that was closest to the query.
    pub original_trace: Vec<f64>,
    /// Generated hallucination vector.
    pub hallucination_vector: Vec<f64>,
    /// Grounding score indicating how much the output is anchored to a real trace.
    pub grounding_score: f64,
    /// Confidence in the hallucination itself (typically low).
    pub confidence: f64,
    /// Divergence from the original trace.
    pub divergence: f64,
}

impl MemoryHallucination {
    /// Minimum valid grounding score.
    pub const MIN_GROUNDING: f64 = 0.0;
    /// Maximum valid grounding score.
    pub const MAX_GROUNDING: f64 = 1.0;
    /// Maximum hallucination confidence.
    pub const MAX_CONFIDENCE: f64 = MAX_HALLUCINATION_CONFIDENCE;

    /// Creates a new MemoryHallucination from the original trace and generated content.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::DimensionMismatch`] if either vector is empty.
    /// Returns [`MemoryError::OutOfRange`] if grounding_score is outside [0.0, 1.0].
    pub fn new(
        original_trace: Vec<f64>,
        hallucination_vector: Vec<f64>,
        grounding_score: f64,
    ) -> Result<Self, MemoryError> {
        if original_trace.is_empty() || hallucination_vector.is_empty() {
            return Err(MemoryError::DimensionMismatch {
                expected: 1,
                actual: 0,
            });
        }
        if !(Self::MIN_GROUNDING..=Self::MAX_GROUNDING).contains(&grounding_score) {
            return Err(MemoryError::OutOfRange {
                field: "grounding_score".into(),
                value: grounding_score,
                min: Self::MIN_GROUNDING,
                max: Self::MAX_GROUNDING,
            });
        }
        let divergence = compute_divergence(&original_trace, &hallucination_vector);
        let confidence = (grounding_score * 0.5).min(Self::MAX_CONFIDENCE);
        Ok(Self {
            original_trace,
            hallucination_vector,
            grounding_score,
            confidence,
            divergence,
        })
    }

    /// Returns the hallucination confidence.
    pub fn confidence(&self) -> f64 {
        self.confidence
    }

    /// Returns the hallucination score = divergence * (1 - grounding).
    pub fn hallucination_score(&self) -> f64 {
        (self.divergence * (1.0 - self.grounding_score)).clamp(0.0, 1.0)
    }

    /// Updates the grounding score and recomputes dependent metrics.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::OutOfRange`] if new_grounding is outside [0.0, 1.0].
    pub fn update_grounding(&mut self, new_grounding: f64) -> Result<(), MemoryError> {
        if !(Self::MIN_GROUNDING..=Self::MAX_GROUNDING).contains(&new_grounding) {
            return Err(MemoryError::OutOfRange {
                field: "grounding_score".into(),
                value: new_grounding,
                min: Self::MIN_GROUNDING,
                max: Self::MAX_GROUNDING,
            });
        }
        self.grounding_score = new_grounding;
        self.confidence = (new_grounding * 0.5).min(Self::MAX_CONFIDENCE);
        Ok(())
    }

    /// Returns whether this hallucination is sufficiently grounded to be trusted.
    pub fn is_grounded(&self) -> bool {
        self.grounding_score >= MIN_GROUNDING_SCORE
    }

    /// Validates the hallucination state.
    pub fn validate(&self) -> Result<(), MemoryError> {
        Self::new(
            self.original_trace.clone(),
            self.hallucination_vector.clone(),
            self.grounding_score,
        )?;
        if self.confidence < 0.0 || self.confidence > Self::MAX_CONFIDENCE {
            return Err(MemoryError::OutOfRange {
                field: "confidence".into(),
                value: self.confidence,
                min: 0.0,
                max: Self::MAX_CONFIDENCE,
            });
        }
        Ok(())
    }
}

impl fmt::Display for MemoryHallucination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MemoryHallucination(grounding={:.2}, confidence={:.2}, divergence={:.2})",
            self.grounding_score, self.confidence, self.divergence
        )
    }
}

/// Computes normalized divergence between two vectors.
fn compute_divergence(a: &[f64], b: &[f64]) -> f64 {
    if a.len() != b.len() || a.is_empty() {
        return 1.0;
    }
    let max_len = a.len();
    let mut sum_sq = 0.0;
    for i in 0..max_len {
        let diff = a[i] - b[i];
        sum_sq += diff * diff;
    }
    let rmse = (sum_sq / max_len as f64).sqrt();
    (rmse / 1.0).clamp(0.0, 1.0)
}