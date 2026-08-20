// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::MemoryError;

use super::*;

/// SynapticTrace: Localized weight change at a single cognitive connection.
///
/// Models the micro-level plasticity of individual synapses, governed by
/// Hebbian potentiation, depression, and homeostatic scaling. Synaptic
/// traces are the building blocks of engram formation.
#[derive(Debug, Clone, PartialEq)]
pub struct SynapticTrace {
    /// Current synaptic weight in [0.0, 1.0].
    pub weight: f64,
    /// Number of times this synapse has been activated.
    pub activation_count: usize,
    /// Abstract time of the last activation event.
    pub last_activation: f64,
    /// Hebbian potentiation rate per activation.
    pub hebbian_rate: f64,
    /// Homeostatic target weight this synapse tends toward.
    pub homeostatic_target: f64,
    /// Decay rate for passive weight reduction.
    pub decay_rate: f64,
}

impl SynapticTrace {
    pub const MIN_WEIGHT: f64 = 0.0;
    pub const MAX_WEIGHT: f64 = 1.0;

    /// Creates a new SynapticTrace with the given initial weight and rates.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::OutOfRange`] if weight, hebbian rate, or
    /// homeostatic target is outside [0.0, 1.0].
    pub fn new(
        weight: f64,
        hebbian_rate: f64,
        homeostatic_target: f64,
    ) -> Result<Self, MemoryError> {
        if !(Self::MIN_WEIGHT..=Self::MAX_WEIGHT).contains(&weight) {
            return Err(MemoryError::OutOfRange {
                field: "weight".into(),
                value: weight,
                min: Self::MIN_WEIGHT,
                max: Self::MAX_WEIGHT,
            });
        }
        if !(0.0..=1.0).contains(&hebbian_rate) {
            return Err(MemoryError::OutOfRange {
                field: "hebbian_rate".into(),
                value: hebbian_rate,
                min: 0.0,
                max: 1.0,
            });
        }
        if !(0.0..=1.0).contains(&homeostatic_target) {
            return Err(MemoryError::OutOfRange {
                field: "homeostatic_target".into(),
                value: homeostatic_target,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(Self {
            weight,
            activation_count: 0,
            last_activation: 0.0,
            hebbian_rate,
            homeostatic_target,
            decay_rate: 0.005,
        })
    }

    /// Potentiates the synapse, increasing weight via Hebbian learning.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::OutOfRange`] if factor is outside [0.0, 1.0].
    pub fn potentiate(&mut self, factor: f64) -> Result<(), MemoryError> {
        if !(0.0..=1.0).contains(&factor) {
            return Err(MemoryError::OutOfRange {
                field: "factor".into(),
                value: factor,
                min: 0.0,
                max: 1.0,
            });
        }
        self.activation_count += 1;
        self.weight = (self.weight + factor * self.hebbian_rate).min(Self::MAX_WEIGHT);
        Ok(())
    }

    /// Depresses the synapse, decreasing weight due to anti-correlation.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::OutOfRange`] if factor is outside [0.0, 1.0].
    pub fn depress(&mut self, factor: f64) -> Result<(), MemoryError> {
        if !(0.0..=1.0).contains(&factor) {
            return Err(MemoryError::OutOfRange {
                field: "factor".into(),
                value: factor,
                min: 0.0,
                max: 1.0,
            });
        }
        self.weight = (self.weight - factor * self.hebbian_rate).max(Self::MIN_WEIGHT);
        Ok(())
    }

    /// Prunes the synapse if its weight falls below the survival threshold.
    ///
    /// Returns `true` if the synapse should be removed.
    pub fn prune(&self, threshold: f64) -> bool {
        self.weight < threshold && self.activation_count < 2
    }

    /// Applies homeostatic scaling, moving the weight toward the target.
    pub fn homeostatic_scaling(&mut self) {
        let drift = self.homeostatic_target - self.weight;
        self.weight = (self.weight + drift * 0.01).clamp(Self::MIN_WEIGHT, Self::MAX_WEIGHT);
    }

    /// Applies passive decay over the given duration.
    pub fn decay(&mut self, delta: f64) {
        if delta <= 0.0 {
            return;
        }
        let decay_factor = (1.0 - self.decay_rate).powf(delta);
        self.weight *= decay_factor;
        self.weight = self.weight.clamp(Self::MIN_WEIGHT, Self::MAX_WEIGHT);
    }

    /// Returns the ratio of activations to elapsed time.
    pub fn activation_ratio(&self, current_time: f64) -> f64 {
        let elapsed = current_time - self.last_activation;
        if elapsed <= 0.0 {
            if self.activation_count == 0 {
                return 0.0;
            }
            return f64::INFINITY;
        }
        self.activation_count as f64 / elapsed
    }

    /// Records an activation event at the given time.
    pub fn record_activation(&mut self, time: f64) {
        self.activation_count += 1;
        self.last_activation = time;
    }

    /// Validates the synaptic trace state.
    pub fn validate(&self) -> Result<(), MemoryError> {
        Self::new(self.weight, self.hebbian_rate, self.homeostatic_target)?;
        if self.decay_rate < 0.0 || self.decay_rate > 1.0 {
            return Err(MemoryError::DecayError { rate: self.decay_rate });
        }
        Ok(())
    }
}

