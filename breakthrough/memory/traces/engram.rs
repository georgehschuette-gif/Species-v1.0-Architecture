// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::MemoryError;

use super::*;

/// EngramTrace: A coherent memory representation formed by linked feature patterns.
///
/// Engrams are the units of long-term memory storage, where a distributed
/// pattern of feature activations encodes the content of a memory.
/// They can be consolidated for stability, reactivated for reconsolidation,
/// and compared for similarity with other traces.
#[derive(Debug, Clone, PartialEq)]
pub struct EngramTrace {
    /// Feature vector encoding the memory content.
    pub features: Vec<f64>,
    /// Current memory strength in [0.0, 1.0].
    pub strength: f64,
    /// Base decay rate per time unit for unconsolidated traces.
    pub decay_rate: f64,
    /// Age of the trace since first formation.
    pub age: f64,
    /// Current consolidation state (0.0 = fragile, 1.0 = stable).
    pub consolidation_state: f64,
    /// Time window after activation during which reconsolidation is possible.
    pub reconsolidation_window: f64,
    /// Number of times this trace has been reactivated.
    pub reactivation_count: usize,
}

impl EngramTrace {
    /// Minimum valid feature count.
    pub const MIN_FEATURES: usize = 1;
    /// Maximum valid feature count.
    pub const MAX_FEATURES: usize = MAX_ENGRAM_FEATURES;

    /// Creates a new EngramTrace from a feature slice and decay rate.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::DimensionMismatch`] if feature length is zero or
    /// exceeds [`MAX_ENGRAM_FEATURES`].
    /// Returns [`MemoryError::OutOfRange`] if decay rate is outside [0.0, 1.0].
    pub fn new(features: &[f64], decay_rate: f64) -> Result<Self, MemoryError> {
        let len = features.len();
        if len == 0 || len > Self::MAX_FEATURES {
            return Err(MemoryError::DimensionMismatch {
                expected: Self::MIN_FEATURES,
                actual: len,
            });
        }
        if !(0.0..=1.0).contains(&decay_rate) {
            return Err(MemoryError::OutOfRange {
                field: "decay_rate".into(),
                value: decay_rate,
                min: 0.0,
                max: 1.0,
            });
        }
        let feature_vec = features.to_vec();
        let norm = feature_vec.iter().map(|v| v * v).sum::<f64>().sqrt();
        let normalized = if norm > 0.0 {
            feature_vec.iter().map(|v| v / norm).collect()
        } else {
            feature_vec
        };
        Ok(Self {
            features: normalized,
            strength: 1.0,
            decay_rate,
            age: 0.0,
            consolidation_state: 0.0,
            reconsolidation_window: 5.0,
            reactivation_count: 0,
        })
    }

    /// Ages the trace by delta time, applying decay and reducing strength.
    pub fn decay(&mut self, delta: f64, usage_factor: f64) -> f64 {
        if delta <= 0.0 {
            return self.strength;
        }
        self.age += delta;
        let rate = if self.strength < 0.2 { self.decay_rate * 2.0 } else { self.decay_rate };
        let decay_factor = (1.0 - rate).powf(delta);
        let new_strength = (self.strength * decay_factor + usage_factor * 0.05).clamp(0.0, 1.0);
        self.strength = new_strength;
        new_strength
    }

    /// Attempts to consolidate the trace, advancing its stability.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::ThresholdNotMet`] if the current strength is below
    /// the consolidation threshold.
    pub fn consolidate(&mut self, delta: f64) -> Result<(), MemoryError> {
        if self.strength < CONSOLIDATION_THRESHOLD {
            return Err(MemoryError::ThresholdNotMet {
                threshold: CONSOLIDATION_THRESHOLD,
                actual: self.strength,
            });
        }
        let gain = 0.1 * delta;
        self.consolidation_state = (self.consolidation_state + gain).min(1.0);
        self.strength = (self.strength + gain * 0.5).min(1.0);
        Ok(())
    }

    /// Reactivates the trace, potentially triggering reconsolidation.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::OutOfRange`] if reactivation strength is outside [0.0, 1.0].
    pub fn reactivate(&mut self, strength: f64) -> Result<(), MemoryError> {
        if !(0.0..=1.0).contains(&strength) {
            return Err(MemoryError::OutOfRange {
                field: "reactivation_strength".into(),
                value: strength,
                min: 0.0,
                max: 1.0,
            });
        }
        self.strength = (self.strength + strength * 0.3).min(1.0);
        self.age *= 0.5;
        self.reactivation_count += 1;
        self.consolidation_state = (self.consolidation_state + 0.05).min(1.0);
        Ok(())
    }

    /// Returns whether the trace is fully consolidated and stable.
    pub fn is_consolidated(&self) -> bool {
        self.consolidation_state >= CONSOLIDATION_THRESHOLD && self.strength >= CONSOLIDATION_THRESHOLD
    }

    /// Computes cosine similarity with another engram trace.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::DimensionMismatch`] if feature lengths differ.
    pub fn similarity(&self, other: &EngramTrace) -> Result<f64, MemoryError> {
        if self.features.len() != other.features.len() {
            return Err(MemoryError::DimensionMismatch {
                expected: self.features.len(),
                actual: other.features.len(),
            });
        }
        let dot: f64 = self.features.iter().zip(&other.features).map(|(a, b)| a * b).sum();
        let norm_a = self.features.iter().map(|v| v * v).sum::<f64>().sqrt();
        let norm_b = other.features.iter().map(|v| v * v).sum::<f64>().sqrt();
        let denom = norm_a * norm_b;
        if denom == 0.0 {
            Ok(0.0)
        } else {
            Ok((dot / denom).clamp(0.0, 1.0))
        }
    }

    /// Returns the number of features in this trace.
    pub fn feature_count(&self) -> usize {
        self.features.len()
    }

    /// Validates the engram trace state.
    pub fn validate(&self) -> Result<(), MemoryError> {
        let len = self.features.len();
        if len == 0 || len > Self::MAX_FEATURES {
            return Err(MemoryError::DimensionMismatch {
                expected: Self::MIN_FEATURES,
                actual: len,
            });
        }
        if !(0.0..=1.0).contains(&self.decay_rate) {
            return Err(MemoryError::DecayError { rate: self.decay_rate });
        }
        if !(0.0..=1.0).contains(&self.strength) {
            return Err(MemoryError::OutOfRange {
                field: "strength".into(),
                value: self.strength,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(())
    }
}

