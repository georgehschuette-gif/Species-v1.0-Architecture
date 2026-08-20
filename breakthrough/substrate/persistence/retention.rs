// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::PersistenceError;

/// MemoryRetention: The persistence of cognitive state over time.
///
/// Tracks how strongly a cognitive state is maintained as it ages.
/// Retention strength decays over time according to configurable
/// decay parameters, modeling the natural fading of memory traces.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MemoryRetention {
    /// Current retention strength in [0.0, 1.0].
    pub strength: f64,
    /// Age of this memory trace in time units.
    pub age: f64,
    /// Base decay rate per time unit.
    pub decay_rate: f64,
    /// Rehearsal boost applied each time the memory is accessed.
    pub rehearsal_boost: f64,
}

impl MemoryRetention {
    /// Minimum valid retention strength.
    pub const MIN_STRENGTH: f64 = 0.0;
    /// Maximum valid retention strength.
    pub const MAX_STRENGTH: f64 = 1.0;
    /// Minimum valid decay rate.
    pub const MIN_DECAY: f64 = 0.0;
    /// Maximum valid decay rate.
    pub const MAX_DECAY: f64 = 1.0;

    /// Creates a new MemoryRetention with the given initial strength.
    ///
    /// # Errors
    /// Returns `PersistenceError::InvalidStrength` if strength is outside [0.0, 1.0].
    /// Returns `PersistenceError::InvalidDecay` if decay_rate is outside [0.0, 1.0].
    pub fn new(strength: f64, decay_rate: f64) -> Result<Self, PersistenceError> {
        if !(Self::MIN_STRENGTH..=Self::MAX_STRENGTH).contains(&strength) {
            return Err(PersistenceError::InvalidStrength { strength });
        }
        if !(Self::MIN_DECAY..=Self::MAX_DECAY).contains(&decay_rate) {
            return Err(PersistenceError::InvalidDecay { decay_rate });
        }
        Ok(Self {
            strength,
            age: 0.0,
            decay_rate,
            rehearsal_boost: 0.1,
        })
    }

    /// Ages the memory by the given delta, applying exponential decay.
    ///
    /// strength = strength * (1 - decay_rate)^delta
    pub fn age(&mut self, delta: f64) {
        if delta < 0.0 {
            return;
        }
        self.age += delta;
        let decay_factor = (1.0 - self.decay_rate).powf(delta);
        self.strength *= decay_factor;
        self.strength = self.strength.clamp(Self::MIN_STRENGTH, Self::MAX_STRENGTH);
    }

    /// Rehearses the memory, boosting its strength and resetting some age.
    pub fn rehearse(&mut self) {
        self.strength = (self.strength + self.rehearsal_boost).min(Self::MAX_STRENGTH);
        self.age *= 0.5;
    }

    /// Returns the effective retention after accounting for decay and age.
    pub fn effective_retention(&self) -> f64 {
        let decay_factor = (1.0 - self.decay_rate).powf(self.age);
        (self.strength * decay_factor).clamp(Self::MIN_STRENGTH, Self::MAX_STRENGTH)
    }

    /// Returns whether the memory has decayed below the dissolution threshold.
    pub fn is_dissolved(&self) -> bool {
        self.effective_retention() < crate::genesis::constants::ThresholdConstants::DISSOLUTION_THRESHOLD
    }

    /// Returns the half-life of this memory in time units.
    pub fn half_life(&self) -> f64 {
        if self.decay_rate <= 0.0 {
            f64::INFINITY
        } else {
            (0.5f64).ln() / (1.0 - self.decay_rate).ln()
        }
    }

    /// Validates the memory retention state.
    pub fn validate(&self) -> Result<(), PersistenceError> {
        Self::new(self.strength, self.decay_rate)?;
        if self.age < 0.0 {
            return Err(PersistenceError::InvalidAge { age: self.age });
        }
        Ok(())
    }
}

impl Default for MemoryRetention {
    fn default() -> Self {
        Self {
            strength: 1.0,
            age: 0.0,
            decay_rate: 0.01,
            rehearsal_boost: 0.1,
        }
    }
}
