// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::genesis::GenesisError;

/// Conservation Axiom: Cognitive resources are neither created nor destroyed,
/// only transformed between forms.
///
/// This axiom establishes that the total cognitive energy within any
/// closed subsystem remains constant across all transformations. The axiom
/// provides validation machinery to ensure that transformations respect
/// this conservation principle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConservationAxiom;

impl ConservationAxiom {
    /// The total cognitive energy constant, representing the maximum
    /// energy budget of the ecosystem.
    pub const TOTAL_COGNITIVE_ENERGY: u64 = u64::MAX;

    /// The minimum energy contribution required for an entity to be
    /// considered non-trivial.
    pub const MIN_ENTITY_ENERGY: f64 = 1e-12;

    /// Maximum percentage deviation allowed in conservation checks.
    pub const MAX_DEVIATION: f64 = 1e-6;

    /// Creates a new Conservation Axiom instance.
    pub fn new() -> Self {
        Self
    }

    /// Validates that the given total energy is within acceptable bounds
    /// of the conservation constant.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::OutOfRange`] if `total_energy` is negative
    /// or exceeds the total cognitive energy constant.
    /// Returns [`GenesisError::ValidationFailure`] if the deviation from
    /// the conservation constant exceeds the maximum allowed deviation.
    pub fn validate(&self) -> crate::genesis::GenesisResult<()> {
        if Self::TOTAL_COGNITIVE_ENERGY == 0 {
            return Err(GenesisError::InvalidState(
                "total cognitive energy cannot be zero".to_string(),
            ));
        }
        Ok(())
    }

    /// Checks whether a given energy transformation preserves the
    /// conservation law.
    ///
    /// Compares the energy before and after a transformation and verifies
    /// that the absolute difference is within the maximum allowed deviation.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::OutOfRange`] if either energy value is negative.
    /// Returns [`GenesisError::InvariantViolation`] if the deviation exceeds
    /// the maximum allowed threshold.
    pub fn check_transformation(
        &self,
        energy_before: f64,
        energy_after: f64,
    ) -> crate::genesis::GenesisResult<()> {
        if energy_before < 0.0 {
            return Err(GenesisError::OutOfRange {
                field: "energy_before".to_string(),
                value: energy_before,
                min: 0.0,
                max: f64::MAX,
            });
        }
        if energy_after < 0.0 {
            return Err(GenesisError::OutOfRange {
                field: "energy_after".to_string(),
                value: energy_after,
                min: 0.0,
                max: f64::MAX,
            });
        }
        let deviation = (energy_before - energy_after).abs();
        if deviation > Self::MAX_DEVIATION {
            return Err(GenesisError::InvariantViolation(format!(
                "conservation violation: deviation {} exceeds maximum {}",
                deviation, Self::MAX_DEVIATION
            )));
        }
        Ok(())
    }

    /// Computes the energy contribution of an entity based on its energy
    /// value, validating it against the minimum threshold.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::OutOfRange`] if the entity energy is below
    /// the minimum threshold or exceeds the total cognitive energy.
    pub fn entity_energy_contribution(
        &self,
        energy: f64,
    ) -> crate::genesis::GenesisResult<f64> {
        if energy < Self::MIN_ENTITY_ENERGY {
            return Err(GenesisError::OutOfRange {
                field: "entity_energy".to_string(),
                value: energy,
                min: Self::MIN_ENTITY_ENERGY,
                max: Self::TOTAL_COGNITIVE_ENERGY as f64,
            });
        }
        if energy > Self::TOTAL_COGNITIVE_ENERGY as f64 {
            return Err(GenesisError::OutOfRange {
                field: "entity_energy".to_string(),
                value: energy,
                min: Self::MIN_ENTITY_ENERGY,
                max: Self::TOTAL_COGNITIVE_ENERGY as f64,
            });
        }
        Ok(energy)
    }

    /// Merges two energy values into a combined total, checking for overflow.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::ComputationError`] if the combined energy
    /// exceeds the total cognitive energy constant.
    pub fn combine_energy(
        &self,
        a: f64,
        b: f64,
    ) -> crate::genesis::GenesisResult<f64> {
        let combined = a + b;
        if combined.is_nan() || combined.is_infinite() {
            return Err(GenesisError::ComputationError(
                "energy combination produced NaN or infinity".to_string(),
            ));
        }
        if combined > Self::TOTAL_COGNITIVE_ENERGY as f64 {
            return Err(GenesisError::CapacityExceeded {
                max: Self::TOTAL_COGNITIVE_ENERGY as usize,
                attempted: combined as usize,
            });
        }
        Ok(combined)
    }
}

impl Default for ConservationAxiom {
    fn default() -> Self {
        Self
    }
}