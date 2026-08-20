// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::genesis::{GenesisError, GenesisResult};

/// ConservationInvariant: Validates that total cognitive energy
/// remains constant across transformations.
///
/// This invariant ensures that the ecosystem conserves cognitive
/// energy through all state transitions. It is the primary guardrail
/// against energy creation or destruction anomalies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConservationInvariant;

impl ConservationInvariant {
    /// The default tolerance for floating-point comparison in
    /// conservation checks.
    pub const DEFAULT_TOLERANCE: f64 = 1e-9;

    /// The minimum allowed total energy before a conservation
    /// violation is considered critical.
    pub const MIN_ENERGY: f64 = 0.0;

    /// Creates a new ConservationInvariant instance.
    pub fn new() -> Self {
        Self
    }

    /// Checks whether the total energy matches the expected value
    /// within the default tolerance.
    ///
    /// Returns `true` if the energies match within tolerance.
    pub fn check(&self, total_energy: f64, expected: f64) -> bool {
        let deviation = (total_energy - expected).abs();
        deviation < Self::DEFAULT_TOLERANCE
    }

    /// Checks whether the total energy matches the expected value
    /// within a custom tolerance.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::OutOfRange`] if either energy is negative.
    /// Returns [`GenesisError::InvariantViolation`] if the deviation
    /// exceeds the provided tolerance.
    pub fn check_with_tolerance(
        &self,
        total_energy: f64,
        expected: f64,
        tolerance: f64,
    ) -> GenesisResult<()> {
        if total_energy < Self::MIN_ENERGY {
            return Err(GenesisError::OutOfRange {
                field: "total_energy".to_string(),
                value: total_energy,
                min: Self::MIN_ENERGY,
                max: f64::MAX,
            });
        }
        if expected < Self::MIN_ENERGY {
            return Err(GenesisError::OutOfRange {
                field: "expected".to_string(),
                value: expected,
                min: Self::MIN_ENERGY,
                max: f64::MAX,
            });
        }
        if tolerance < 0.0 {
            return Err(GenesisError::OutOfRange {
                field: "tolerance".to_string(),
                value: tolerance,
                min: 0.0,
                max: f64::MAX,
            });
        }
        let deviation = (total_energy - expected).abs();
        if deviation > tolerance {
            return Err(GenesisError::InvariantViolation(format!(
                "energy conservation violated: total={}, expected={}, deviation={}, tolerance={}",
                total_energy, expected, deviation, tolerance
            )));
        }
        Ok(())
    }

    /// Validates a set of entity energies, ensuring their sum
    /// matches the expected total.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::OutOfRange`] if any individual energy
    /// is negative.
    /// Returns [`GenesisError::InvariantViolation`] if the total
    /// does not match the expected value.
    pub fn validate_energies(
        &self,
        energies: &[f64],
        expected_total: f64,
    ) -> GenesisResult<()> {
        let total: f64 = energies.iter().sum();
        if energies.iter().any(|&e| e < 0.0) {
            return Err(GenesisError::OutOfRange {
                field: "individual_energy".to_string(),
                value: f64::NEG_INFINITY,
                min: 0.0,
                max: f64::MAX,
            });
        }
        self.check_with_tolerance(total, expected_total, Self::DEFAULT_TOLERANCE)?;
        Ok(())
    }
}

impl Default for ConservationInvariant {
    fn default() -> Self {
        Self
    }
}