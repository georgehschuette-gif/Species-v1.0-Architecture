// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::PersistenceError;

/// StateDurability: The resistance of cognitive state to perturbation.
///
/// Measures how well a cognitive state withstands external disruptions
/// such as noise, interference, or conflicting inputs. Higher durability
/// means the state is more robust and returns to equilibrium faster.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StateDurability {
    /// Durability score in [0.0, 1.0] (1.0 = completely robust).
    pub durability: f64,
    /// Perturbation magnitude threshold above which the state is disrupted.
    pub perturbation_threshold: f64,
    /// Recovery rate per time unit after disruption.
    pub recovery_rate: f64,
    /// Elasticity: how much the state can stretch before breaking.
    pub elasticity: f64,
}

impl StateDurability {
    /// Minimum valid durability.
    pub const MIN_DURABILITY: f64 = 0.0;
    /// Maximum valid durability.
    pub const MAX_DURABILITY: f64 = 1.0;
    /// Minimum valid perturbation threshold.
    pub const MIN_THRESHOLD: f64 = 0.0;
    /// Minimum valid recovery rate.
    pub const MIN_RECOVERY: f64 = 0.0;
    /// Maximum valid recovery rate.
    pub const MAX_RECOVERY: f64 = 1.0;

    /// Creates a new StateDurability.
    ///
    /// # Errors
    /// Returns `PersistenceError::InvalidDurability` if durability is outside [0.0, 1.0].
    /// Returns `PersistenceError::InvalidThreshold` if perturbation_threshold is negative.
    /// Returns `PersistenceError::InvalidRecovery` if recovery_rate is outside [0.0, 1.0].
    pub fn new(
        durability: f64,
        perturbation_threshold: f64,
        recovery_rate: f64,
    ) -> Result<Self, PersistenceError> {
        if !(Self::MIN_DURABILITY..=Self::MAX_DURABILITY).contains(&durability) {
            return Err(PersistenceError::InvalidDurability { durability });
        }
        if perturbation_threshold < Self::MIN_THRESHOLD {
            return Err(PersistenceError::InvalidThreshold {
                threshold: perturbation_threshold,
            });
        }
        if !(Self::MIN_RECOVERY..=Self::MAX_RECOVERY).contains(&recovery_rate) {
            return Err(PersistenceError::InvalidRecovery { recovery_rate });
        }
        Ok(Self {
            durability,
            perturbation_threshold,
            recovery_rate,
            elasticity: 0.5,
        })
    }

    /// Returns whether the given perturbation exceeds the threshold.
    pub fn is_disrupted_by(&self, perturbation: f64) -> bool {
        perturbation > self.perturbation_threshold
    }

    /// Computes the effective durability after applying a perturbation.
    ///
    /// If the perturbation exceeds the threshold, durability is reduced.
    /// Otherwise, it may be slightly enhanced through anti-fragility.
    pub fn apply_perturbation(&mut self, perturbation: f64) {
        if self.is_disrupted_by(perturbation) {
            let excess = perturbation - self.perturbation_threshold;
            let damage = (excess / (self.perturbation_threshold + f64::EPSILON)).min(1.0);
            self.durability = (self.durability - damage * self.elasticity).max(0.0);
        } else {
            self.durability = (self.durability + 0.01).min(1.0);
        }
    }

    /// Recovers durability over time at the configured recovery rate.
    pub fn recover(&mut self, delta: f64) {
        if delta < 0.0 {
            return;
        }
        let recovery = self.recovery_rate * delta;
        self.durability = (self.durability + recovery).min(Self::MAX_DURABILITY);
    }

    /// Returns the time required to fully recover from zero durability.
    pub fn full_recovery_time(&self) -> f64 {
        if self.recovery_rate <= 0.0 {
            f64::INFINITY
        } else {
            (1.0 - self.durability) / self.recovery_rate
        }
    }

    /// Returns whether the state is considered robust.
    pub fn is_robust(&self) -> bool {
        self.durability >= 0.7
    }

    /// Returns whether the state is considered fragile.
    pub fn is_fragile(&self) -> bool {
        self.durability < 0.3
    }

    /// Validates the durability state.
    pub fn validate(&self) -> Result<(), PersistenceError> {
        Self::new(self.durability, self.perturbation_threshold, self.recovery_rate)?;
        if self.elasticity < 0.0 || self.elasticity > 1.0 {
            return Err(PersistenceError::InvalidElasticity { elasticity: self.elasticity });
        }
        Ok(())
    }
}

impl Default for StateDurability {
    fn default() -> Self {
        Self {
            durability: 0.8,
            perturbation_threshold: 0.5,
            recovery_rate: 0.1,
            elasticity: 0.5,
        }
    }
}
