// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::genesis::{GenesisError, GenesisResult};

/// ThresholdConstants: Activation and decay thresholds for cognitive
/// processes.
///
/// These constants define the critical values that govern state
/// transitions within the cognitive ecosystem: when entities
/// activate, resonate, decay, or dissolve.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThresholdConstants;

impl ThresholdConstants {
    /// The minimum energy level required for an entity to transition
    /// from Dormant to Active.
    pub const ACTIVATION_THRESHOLD: f64 = 0.5;

    /// The rate at which entity energy decays per time step when no
    /// external input is received.
    pub const DECAY_RATE: f64 = 0.01;

    /// The minimum energy level required for an entity to reach the
    /// Resonant state.
    pub const RESONANCE_THRESHOLD: f64 = 0.8;

    /// The maximum energy level below which an entity is considered
    /// for dissolution.
    pub const DISSOLUTION_THRESHOLD: f64 = 0.1;

    /// Returns whether the given energy meets the activation threshold
    /// for transitioning to the Active state.
    pub fn is_active(&self, energy: f64) -> bool {
        energy >= Self::ACTIVATION_THRESHOLD
    }

    /// Returns whether the given energy meets the resonance threshold
    /// for transitioning to the Resonant state.
    pub fn is_resonant(&self, energy: f64) -> bool {
        energy >= Self::RESONANCE_THRESHOLD
    }

    /// Returns whether the given energy falls below the dissolution
    /// threshold, indicating the entity should be dissolved.
    pub fn should_dissolve(&self, energy: f64) -> bool {
        energy < Self::DISSOLUTION_THRESHOLD
    }

    /// Computes the decayed energy after one time step.
    ///
    /// Applies the decay rate multiplicatively to the current energy.
    /// Ensures the result is non-negative.
    pub fn decay_energy(&self, energy: f64) -> f64 {
        let decayed = energy * (1.0 - Self::DECAY_RATE);
        decayed.max(0.0)
    }

    /// Computes the number of time steps required for energy to decay
    /// below the dissolution threshold.
    ///
    /// Returns 0 if energy is already below the threshold.
    pub fn steps_to_dissolution(&self, energy: f64) -> u64 {
        if energy < Self::DISSOLUTION_THRESHOLD {
            return 0;
        }
        if energy <= 0.0 {
            return 0;
        }
        let ratio = Self::DISSOLUTION_THRESHOLD / energy;
        if ratio <= 0.0 {
            return u64::MAX;
        }
        (-ratio.log(Self::DECAY_RATE).ceil()).max(0.0) as u64
    }

    /// Validates all threshold constants are within valid bounds
    /// (0.0 to 1.0, and monotonically ordered where required).
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::OutOfRange`] if any threshold is outside
    /// [0.0, 1.0].
    /// Returns [`GenesisError::InvalidState`] if thresholds are not
    /// monotonically ordered.
    pub fn validate(&self) -> GenesisResult<()> {
        if !(0.0..=1.0).contains(&Self::ACTIVATION_THRESHOLD) {
            return Err(GenesisError::OutOfRange {
                field: "ACTIVATION_THRESHOLD".to_string(),
                value: Self::ACTIVATION_THRESHOLD,
                min: 0.0,
                max: 1.0,
            });
        }
        if !(0.0..=1.0).contains(&Self::RESONANCE_THRESHOLD) {
            return Err(GenesisError::OutOfRange {
                field: "RESONANCE_THRESHOLD".to_string(),
                value: Self::RESONANCE_THRESHOLD,
                min: 0.0,
                max: 1.0,
            });
        }
        if !(0.0..=1.0).contains(&Self::DISSOLUTION_THRESHOLD) {
            return Err(GenesisError::OutOfRange {
                field: "DISSOLUTION_THRESHOLD".to_string(),
                value: Self::DISSOLUTION_THRESHOLD,
                min: 0.0,
                max: 1.0,
            });
        }
        if Self::DISSOLUTION_THRESHOLD >= Self::ACTIVATION_THRESHOLD {
            return Err(GenesisError::InvalidState(format!(
                "DISSOLUTION_THRESHOLD ({}) must be less than ACTIVATION_THRESHOLD ({})",
                Self::DISSOLUTION_THRESHOLD, Self::ACTIVATION_THRESHOLD
            )));
        }
        if Self::ACTIVATION_THRESHOLD >= Self::RESONANCE_THRESHOLD {
            return Err(GenesisError::InvalidState(format!(
                "ACTIVATION_THRESHOLD ({}) must be less than RESONANCE_THRESHOLD ({})",
                Self::ACTIVATION_THRESHOLD, Self::RESONANCE_THRESHOLD
            )));
        }
        if Self::DECAY_RATE < 0.0 || Self::DECAY_RATE > 1.0 {
            return Err(GenesisError::OutOfRange {
                field: "DECAY_RATE".to_string(),
                value: Self::DECAY_RATE,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(())
    }
}

impl Default for ThresholdConstants {
    fn default() -> Self {
        Self
    }
}