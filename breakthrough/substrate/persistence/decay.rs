// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::PersistenceError;

/// DecayResistance: The ability to resist cognitive decay over time.
///
/// Encapsulates mechanisms that slow the natural degradation of
/// cognitive structures. Higher resistance means slower decay rates
/// and longer-lasting cognitive traces.
#[derive(Debug, Clone, PartialEq)]
pub struct DecayResistance {
    /// Resistance factor in [0.0, 1.0] (1.0 = complete immunity).
    pub resistance: f64,
    /// Base decay rate per time unit without resistance.
    pub decay_rate: f64,
    /// Protective mechanisms active for this state.
    pub protections: Vec<ProtectionMechanism>,
    /// Current stability multiplier.
    pub stability: f64,
}

/// ProtectionMechanism: A method by which decay is resisted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtectionMechanism {
    /// Rehearsal periodically strengthens the memory trace.
    Rehearsal,
    /// Redundancy stores multiple copies across different substrates.
    Redundancy,
    /// Association links the trace to other stable structures.
    Association,
    /// Consolidation transfers the trace to more stable storage.
    Consolidation,
    /// Reminiscence actively recalls and refreshes the trace.
    Reminiscence,
}

impl DecayResistance {
    /// Minimum valid resistance.
    pub const MIN_RESISTANCE: f64 = 0.0;
    /// Maximum valid resistance.
    pub const MAX_RESISTANCE: f64 = 1.0;
    /// Minimum valid decay rate.
    pub const MIN_DECAY: f64 = 0.0;
    /// Maximum valid decay rate.
    pub const MAX_DECAY: f64 = 1.0;

    /// Creates a new DecayResistance with the given parameters.
    ///
    /// # Errors
    /// Returns `PersistenceError::InvalidResistance` if resistance is outside [0.0, 1.0].
    /// Returns `PersistenceError::InvalidDecayRate` if decay_rate is outside [0.0, 1.0].
    pub fn new(resistance: f64, decay_rate: f64) -> Result<Self, PersistenceError> {
        if !(Self::MIN_RESISTANCE..=Self::MAX_RESISTANCE).contains(&resistance) {
            return Err(PersistenceError::InvalidResistance { resistance });
        }
        if !(Self::MIN_DECAY..=Self::MAX_DECAY).contains(&decay_rate) {
            return Err(PersistenceError::InvalidDecayRate { decay_rate });
        }
        Ok(Self {
            resistance,
            decay_rate,
            protections: vec![ProtectionMechanism::Rehearsal],
            stability: 1.0,
        })
    }

    /// Adds a protection mechanism to this resistance profile.
    pub fn add_protection(&mut self, mechanism: ProtectionMechanism) {
        if !self.protections.contains(&mechanism) {
            self.protections.push(mechanism);
        }
    }

    /// Removes a protection mechanism.
    pub fn remove_protection(&mut self, mechanism: ProtectionMechanism) {
        self.protections.retain(|&m| m != mechanism);
    }

    /// Computes the effective decay rate after applying all protections.
    ///
    /// Each protection mechanism reduces the decay rate by a factor.
    pub fn effective_decay_rate(&self) -> f64 {
        let mut effective = self.decay_rate;
        for mechanism in &self.protections {
            effective = match mechanism {
                ProtectionMechanism::Rehearsal => effective * 0.8,
                ProtectionMechanism::Redundancy => effective * 0.6,
                ProtectionMechanism::Association => effective * 0.7,
                ProtectionMechanism::Consolidation => effective * 0.5,
                ProtectionMechanism::Reminiscence => effective * 0.75,
            };
        }
        effective * (1.0 - self.resistance)
    }

    /// Applies decay over the given time delta.
    ///
    /// Returns the remaining strength after decay.
    pub fn apply_decay(&mut self, initial_strength: f64, delta: f64) -> f64 {
        if delta < 0.0 {
            return initial_strength;
        }
        let effective_rate = self.effective_decay_rate();
        let decayed = initial_strength * (1.0 - effective_rate).powf(delta);
        decayed.max(0.0)
    }

    /// Returns the half-life of a cognitive trace with this resistance.
    pub fn half_life(&self) -> f64 {
        let effective_rate = self.effective_decay_rate();
        if effective_rate <= 0.0 {
            f64::INFINITY
        } else {
            (0.5f64).ln() / (1.0 - effective_rate).ln()
        }
    }

    /// Returns whether this resistance profile provides significant protection.
    pub fn is_significant(&self) -> bool {
        self.resistance > 0.5 || self.protections.len() >= 3
    }

    /// Updates stability based on recent decay performance.
    pub fn update_stability(&mut self, expected_decay: f64, actual_decay: f64) {
        let ratio = if expected_decay > 0.0 {
            actual_decay / expected_decay
        } else {
            1.0
        };
        self.stability = (self.stability * 0.9 + ratio * 0.1).clamp(0.0, 1.0);
    }

    /// Validates the decay resistance state.
    pub fn validate(&self) -> Result<(), PersistenceError> {
        Self::new(self.resistance, self.decay_rate)?;
        if self.stability < 0.0 || self.stability > 1.0 {
            return Err(PersistenceError::InvalidStability { stability: self.stability });
        }
        Ok(())
    }
}

impl Default for DecayResistance {
    fn default() -> Self {
        Self {
            resistance: 0.5,
            decay_rate: 0.01,
            protections: vec![ProtectionMechanism::Rehearsal],
            stability: 1.0,
        }
    }
}
