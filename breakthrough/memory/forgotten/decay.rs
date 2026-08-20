// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::MemoryError;

use super::*;

/// DecayType: The mathematical form of forgetting decay.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DecayType {
    /// Exponential decay: strength(t) = strength(0) * exp(-rate * t).
    Exponential,
    /// Power law decay: strength(t) = strength(0) / (1 + rate * t)^alpha.
    PowerLaw,
    /// Step decay: strength drops sharply after a retention interval.
    Step,
}

/// MemoryDecay: The forgetting model governing memory dissolution over time.
///
/// Implements multiple decay curves that reduce memory strength as a function
/// of elapsed time and rehearsal count. Dissolution occurs when strength falls
/// below the existence threshold.
#[derive(Debug, Clone, PartialEq)]
pub struct MemoryDecay {
    /// Base decay rate per time unit.
    pub decay_rate: f64,
    /// Type of decay curve applied.
    pub decay_type: DecayType,
    /// Initial memory strength at time zero.
    pub initial_strength: f64,
    /// Number of rehearsals applied to this memory.
    pub rehearsal_count: usize,
    /// elapsed time since first encoding.
    pub elapsed_time: f64,
}

impl MemoryDecay {
    /// Creates a new MemoryDecay model with the given rate and type.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::OutOfRange`] if rate is outside [0.0, 1.0].
    pub fn new(rate: f64, decay_type: DecayType) -> Result<Self, MemoryError> {
        if !(0.0..=1.0).contains(&rate) {
            return Err(MemoryError::DecayError { rate });
        }
        Ok(Self {
            decay_rate: rate,
            decay_type,
            initial_strength: 1.0,
            rehearsal_count: 0,
            elapsed_time: 0.0,
        })
    }

    /// Applies decay over the given time, returning remaining strength.
    ///
    /// Rehearsals partially counteract decay by boosting initial strength.
    pub fn apply_decay(&self, elapsed: f64, current_strength: f64) -> f64 {
        if elapsed <= 0.0 {
            return current_strength;
        }
        let effective_rate = self.decay_rate * (1.0 - self.rehearsal_count as f64 * 0.01);
        let effective_rate = effective_rate.max(0.0);
        let remaining = match self.decay_type {
            DecayType::Exponential => {
                let factor = (1.0 - effective_rate).powf(elapsed);
                current_strength * factor
            }
            DecayType::PowerLaw => {
                let alpha = 1.5;
                current_strength / (1.0 + effective_rate * elapsed).powf(alpha)
            }
            DecayType::Step => {
                let retention = 10.0;
                if elapsed > retention {
                    current_strength * (1.0 - effective_rate)
                } else {
                    current_strength
                }
            }
        };
        remaining.clamp(0.0, 1.0)
    }

    /// Rehearses the memory, partially restoring strength.
    pub fn rehearse(&mut self, strength: f64) -> f64 {
        self.rehearsal_count += 1;
        (strength + MAX_REHEARSAL_BOOST.min(0.1 * self.rehearsal_count as f64)).min(1.0)
    }

    /// Returns the half-life of this memory in time units.
    pub fn half_life(&self) -> f64 {
        match self.decay_type {
            DecayType::Exponential => {
                if self.decay_rate >= 1.0 {
                    return 0.0;
                }
                (0.5f64).ln() / (1.0 - self.decay_rate).ln()
            }
            DecayType::PowerLaw => {
                let alpha = 1.5;
                (2.0f64.powf(1.0 / alpha) - 1.0) / self.decay_rate
            }
            DecayType::Step => 10.0,
        }
    }

    /// Returns whether the memory has dissolved below the existence threshold.
    pub fn is_dissolved(&self, strength: f64) -> bool {
        strength < EXISTENCE_THRESHOLD
    }

    /// Estimates remaining strength at a future time without applying decay.
    pub fn estimate_remaining(&self, future_delta: f64, strength: f64) -> f64 {
        self.apply_decay(future_delta, strength)
    }

    /// Validates the decay state.
    pub fn validate(&self) -> Result<(), MemoryError> {
        Self::new(self.decay_rate, self.decay_type)?;
        if !(0.0..=1.0).contains(&self.initial_strength) {
            return Err(MemoryError::OutOfRange {
                field: "initial_strength".into(),
                value: self.initial_strength,
                min: 0.0,
                max: 1.0,
            });
        }
        if self.elapsed_time < 0.0 {
            return Err(MemoryError::OutOfRange {
                field: "elapsed_time".into(),
                value: self.elapsed_time,
                min: 0.0,
                max: f64::INFINITY,
            });
        }
        Ok(())
    }
}

