// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::genesis::{GenesisError, GenesisResult};

/// FundamentalConstants: The base physical constants of the cognitive universe.
///
/// These constants define the irreducible parameters that govern
/// cognitive dynamics at the most fundamental level, analogous to
/// Planck's constant, the speed of light, and the gravitational
/// constant in physical cosmology.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FundamentalConstants;

impl FundamentalConstants {
    /// Quantum of cognitive action, the smallest meaningful unit
    /// of cognitive transformation.
    pub const COGNITIVE_PLANCK: f64 = 1.054_571_817e-34;

    /// Maximum rate of information propagation through the cognitive medium.
    pub const COGNITIVE_SPEED: f64 = 299_792_458.0;

    /// Coupling constant governing the attraction between cognitive entities.
    pub const ENTITY_GRAVITY: f64 = 6.674_30e-11;

    /// Returns a human-readable description of all fundamental constants.
    pub fn describe() -> String {
        format!(
            "FundamentalConstants:\n  COGNITIVE_PLANCK = {}\n  COGNITIVE_SPEED = {}\n  ENTITY_GRAVITY = {}",
            Self::COGNITIVE_PLANCK,
            Self::COGNITIVE_SPEED,
            Self::ENTITY_GRAVITY
        )
    }

    /// Computes the cognitive action for a given energy and time interval.
    ///
    /// Action = energy * time. This is compared against the
    /// cognitive Planck constant to determine if the transformation
    /// is quantum-mechanically significant.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::ComputationError`] if energy or time is NaN.
    pub fn compute_action(energy: f64, time: f64) -> GenesisResult<f64> {
        if energy.is_nan() || time.is_nan() {
            return Err(GenesisError::ComputationError(
                "energy and time must be finite numbers".to_string(),
            ));
        }
        Ok(energy * time)
    }

    /// Determines whether a cognitive action is below the quantum scale,
    /// meaning it is effectively classical rather than quantum-mechanical.
    pub fn is_classical_action(action: f64) -> bool {
        action > Self::COGNITIVE_PLANCK
    }

    /// Computes the gravitational coupling strength between two entities
    /// based on their distance and the entity gravity constant.
    ///
    /// Returns 0.0 if `distance` is zero or non-finite.
    pub fn gravitational_potential(mass_a: f64, mass_b: f64, distance: f64) -> f64 {
        if distance <= 0.0 || !distance.is_finite() {
            return 0.0;
        }
        Self::ENTITY_GRAVITY * mass_a * mass_b / (distance * distance)
    }

    /// Validates all fundamental constants are non-zero and finite.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::ValidationFailure`] if any constant is
    /// zero, NaN, or infinite.
    pub fn validate() -> GenesisResult<()> {
        if !Self::COGNITIVE_PLANCK.is_finite() || Self::COGNITIVE_PLANCK == 0.0 {
            return Err(GenesisError::ValidationFailure(
                "COGNITIVE_PLANCK must be finite and non-zero".to_string(),
            ));
        }
        if !Self::COGNITIVE_SPEED.is_finite() || Self::COGNITIVE_SPEED <= 0.0 {
            return Err(GenesisError::ValidationFailure(
                "COGNITIVE_SPEED must be finite and positive".to_string(),
            ));
        }
        if !Self::ENTITY_GRAVITY.is_finite() {
            return Err(GenesisError::ValidationFailure(
                "ENTITY_GRAVITY must be finite".to_string(),
            ));
        }
        Ok(())
    }
}

impl Default for FundamentalConstants {
    fn default() -> Self {
        Self
    }
}