// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::MemoryError;

use super::*;

/// MemorySuppression: Active inhibitory force reducing memory strength.
///
/// Models the conscious or unconscious suppression of memories, where an
/// external inhibitory force pushes the memory strength below its natural
/// level. Suppressed memories retain recovery potential and may resurface
/// if the suppression force weakens.
#[derive(Debug, Clone, PartialEq)]
pub struct MemorySuppression {
    /// Current suppression force in [0.0, 1.0].
    pub suppression_force: f64,
    /// Innate resistance of the memory to suppression in [0.0, 1.0].
    pub resistance: f64,
    /// Potential for recovery once suppression is removed.
    pub recovery_potential: f64,
    /// Current net memory strength after suppression.
    pub net_strength: f64,
    /// Duration for which suppression has been active.
    pub suppression_duration: f64,
}

impl MemorySuppression {
    /// Creates a new MemorySuppression with the given force and resistance.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::OutOfRange`] if force or resistance is outside [0.0, 1.0].
    pub fn new(force: f64, resistance: f64) -> Result<Self, MemoryError> {
        if !(0.0..=1.0).contains(&force) {
            return Err(MemoryError::OutOfRange {
                field: "suppression_force".into(),
                value: force,
                min: 0.0,
                max: 1.0,
            });
        }
        if !(0.0..=1.0).contains(&resistance) {
            return Err(MemoryError::OutOfRange {
                field: "resistance".into(),
                value: resistance,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(Self {
            suppression_force: force,
            resistance,
            recovery_potential: 1.0 - force,
            net_strength: 1.0,
            suppression_duration: 0.0,
        })
    }

    /// Increases the suppression force.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::OutOfRange`] if force exceeds maximum.
    pub fn suppress(&mut self, additional_force: f64) -> Result<(), MemoryError> {
        let new_force = self.suppression_force + additional_force;
        if new_force > 1.0 {
            return Err(MemoryError::CapacityExceeded {
                max: 1,
                attempted: (new_force * 100.0) as usize,
            });
        }
        self.suppression_force = new_force;
        self.recovery_potential = (1.0 - new_force).max(0.0);
        self.net_strength = (self.net_strength * (1.0 - additional_force)).max(0.0);
        Ok(())
    }

    /// Releases suppression, allowing the memory to recover.
    pub fn release(&mut self, amount: f64) {
        self.suppression_force = (self.suppression_force - amount).max(0.0);
        self.recovery_potential = (self.recovery_potential + amount).min(1.0);
    }

    /// Computes net memory strength accounting for resistance.
    pub fn net_strength(&self, base_strength: f64) -> f64 {
        let effective_force = self.suppression_force * (1.0 - self.resistance);
        (base_strength * (1.0 - effective_force)).clamp(0.0, 1.0)
    }

    /// Returns the depth of suppression (how far below baseline).
    pub fn suppression_depth(&self, base_strength: f64) -> f64 {
        (base_strength - self.net_strength(base_strength)).clamp(0.0, 1.0)
    }

    /// Returns whether suppression is currently active.
    pub fn is_active(&self) -> bool {
        self.suppression_force > 0.01
    }

    /// Advances suppression duration, affecting recovery dynamics.
    pub fn age(&mut self, delta: f64) {
        if delta <= 0.0 {
            return;
        }
        self.suppression_duration += delta;
        self.recovery_potential = (self.recovery_potential + delta * 0.01).min(1.0);
    }

    /// Validates the suppression state.
    pub fn validate(&self) -> Result<(), MemoryError> {
        Self::new(self.suppression_force, self.resistance)?;
        if !(0.0..=1.0).contains(&self.recovery_potential) {
            return Err(MemoryError::OutOfRange {
                field: "recovery_potential".into(),
                value: self.recovery_potential,
                min: 0.0,
                max: 1.0,
            });
        }
        if self.suppression_duration < 0.0 {
            return Err(MemoryError::OutOfRange {
                field: "suppression_duration".into(),
                value: self.suppression_duration,
                min: 0.0,
                max: f64::INFINITY,
            });
        }
        Ok(())
    }
}

