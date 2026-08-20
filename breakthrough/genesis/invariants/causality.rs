// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::genesis::{GenesisError, GenesisResult};

/// CausalityInvariant: Validates temporal ordering of cause and effect.
///
/// This invariant ensures that cause always precedes effect in the
/// cognitive timeline. It prevents paradoxes where effects would
/// influence their own causes, which would create logical
/// inconsistencies in the ecosystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CausalityInvariant;

impl CausalityInvariant {
    /// The smallest positive time delta that can be distinguished
    /// in the cognitive timeline.
    pub const MIN_TIME_DELTA: f64 = 1e-15;

    /// Creates a new CausalityInvariant instance.
    pub fn new() -> Self {
        Self
    }

    /// Checks whether the cause precedes or is simultaneous with
    /// the effect.
    ///
    /// Returns `true` if `cause_time <= effect_time`.
    pub fn check(&self, cause_time: f64, effect_time: f64) -> bool {
        cause_time <= effect_time
    }

    /// Validates that the cause properly precedes the effect,
    /// with a minimum temporal separation.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::ValidationFailure`] if the effect
    /// precedes the cause.
    /// Returns [`GenesisError::ComputationError`] if either time
    /// value is NaN.
    pub fn validate_temporal_order(
        &self,
        cause_time: f64,
        effect_time: f64,
    ) -> GenesisResult<()> {
        if cause_time.is_nan() || effect_time.is_nan() {
            return Err(GenesisError::ComputationError(
                "temporal values cannot be NaN".to_string(),
            ));
        }
        if cause_time > effect_time {
            return Err(GenesisError::ValidationFailure(format!(
                "effect time {} precedes cause time {}",
                effect_time, cause_time
            )));
        }
        Ok(())
    }

    /// Validates a sequence of temporal events to ensure they are
    /// properly ordered.
    ///
    /// Each event is given as (timestamp, event_label). The sequence
    /// must be non-decreasing in time.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::MissingInput`] if the sequence is empty.
    /// Returns [`GenesisError::ComputationError`] if any timestamp
    /// is NaN.
    /// Returns [`GenesisError::ValidationFailure`] if the sequence
    /// is not non-decreasing.
    pub fn validate_event_sequence(
        &self,
        events: &[(f64, &str)],
    ) -> GenesisResult<()> {
        if events.is_empty() {
            return Err(GenesisError::MissingInput(
                "event sequence cannot be empty".to_string(),
            ));
        }
        for (i, &(ts, _)) in events.iter().enumerate() {
            if ts.is_nan() {
                return Err(GenesisError::ComputationError(format!(
                    "event {} has NaN timestamp",
                    i
                )));
            }
            if i > 0 && ts < events[i - 1].0 {
                return Err(GenesisError::ValidationFailure(format!(
                    "event {} at time {} precedes event {} at time {}",
                    i, ts, i - 1, events[i - 1].0
                )));
            }
        }
        Ok(())
    }

    /// Computes the temporal distance between cause and effect.
    ///
    /// Returns 0.0 if cause and effect are simultaneous.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::ValidationFailure`] if effect precedes
    /// cause.
    /// Returns [`GenesisError::ComputationError`] if either time is NaN.
    pub fn temporal_distance(
        &self,
        cause_time: f64,
        effect_time: f64,
    ) -> GenesisResult<f64> {
        self.validate_temporal_order(cause_time, effect_time)?;
        Ok(effect_time - cause_time)
    }
}

impl Default for CausalityInvariant {
    fn default() -> Self {
        Self
    }
}