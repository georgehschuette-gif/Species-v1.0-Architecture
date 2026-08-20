// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::genesis::{GenesisError, GenesisResult};

/// CoherenceInvariant: Ensures the ecosystem maintains internal
/// consistency.
///
/// Coherence measures how well-structured and self-consistent
/// the cognitive ecosystem is. A coherent ecosystem has
/// no contradictions between its entities, relations, and fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CoherenceInvariant;

impl CoherenceInvariant {
    /// The minimum acceptable coherence score. Values below
    /// this indicate an incoherent system.
    pub const MIN_COHERENCE: f64 = 0.0;

    /// The maximum acceptable coherence score.
    pub const MAX_COHERENCE: f64 = 1.0;

    /// The threshold below which the system is considered
    /// critically incoherent.
    pub const CRITICAL_LOW_THRESHOLD: f64 = 0.3;

    /// Creates a new CoherenceInvariant instance.
    pub fn new() -> Self {
        Self
    }

    /// Checks whether the given consistency score falls within
    /// the valid range [0.0, 1.0].
    ///
    /// Returns `true` if the score is valid, `false` otherwise.
    pub fn check(&self, consistency_score: f64) -> bool {
        consistency_score >= Self::MIN_COHERENCE
            && consistency_score <= Self::MAX_COHERENCE
    }

    /// Validates the coherence score and classifies the system's
    /// coherence level.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::OutOfRange`] if the score is outside
    /// the valid range [0.0, 1.0].
    pub fn validate(&self, consistency_score: f64) -> GenesisResult<()> {
        if consistency_score < Self::MIN_COHERENCE || consistency_score > Self::MAX_COHERENCE {
            return Err(GenesisError::OutOfRange {
                field: "consistency_score".to_string(),
                value: consistency_score,
                min: Self::MIN_COHERENCE,
                max: Self::MAX_COHERENCE,
            });
        }
        Ok(())
    }

    /// Returns the coherence classification for the given score.
    ///
    /// - "critical" if score < CRITICAL_LOW_THRESHOLD
    /// - "marginal" if score < 0.6
    /// - "healthy" if score < 0.8
    /// - "optimal" otherwise
    pub fn classify(&self, consistency_score: f64) -> GenesisResult<&'static str> {
        self.validate(consistency_score)?;
        if consistency_score < Self::CRITICAL_LOW_THRESHOLD {
            Ok("critical")
        } else if consistency_score < 0.6 {
            Ok("marginal")
        } else if consistency_score < 0.8 {
            Ok("healthy")
        } else {
            Ok("optimal")
        }
    }

    /// Checks whether the system's coherence is declining based
    /// on a historical sequence of scores.
    ///
    /// Returns `true` if the most recent score is lower than the
    /// average of all previous scores.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::MissingInput`] if fewer than 2
    /// scores are provided.
    pub fn is_declining(
        &self,
        history: &[f64],
    ) -> GenesisResult<bool> {
        if history.len() < 2 {
            return Err(GenesisError::MissingInput(
                "at least 2 scores required to detect decline".to_string(),
            ));
        }
        for &score in history {
            self.validate(score)?;
        }
        let recent = history.last().copied().unwrap_or(0.0);
        let sum: f64 = history.iter().take(history.len() - 1).sum();
        let previous_avg = sum / (history.len() - 1) as f64;
        Ok(recent < previous_avg)
    }
}

impl Default for CoherenceInvariant {
    fn default() -> Self {
        Self
    }
}