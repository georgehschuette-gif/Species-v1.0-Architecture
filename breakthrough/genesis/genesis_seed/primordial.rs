// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::genesis::{GenesisError, GenesisResult};

/// PrimordialField: The pre-cognitive substrate from
/// which all entities emerge.
///
/// The primordial field represents the raw potential
/// from which the first entities crystallize. It has
/// a base potential energy and a fluctuation amplitude
/// that determines how much its potential varies
/// across time steps. Unlike classical random fluctuations,
/// the primordial field uses a deterministic pseudo-random
/// mechanism suitable for reproducible initialization.
#[derive(Debug, Clone, PartialEq)]
pub struct PrimordialField {
    /// The base potential energy of the field.
    pub potential: f64,
    /// The maximum amplitude of fluctuations in the
    /// field potential.
    pub fluctuation_amplitude: f64,
}

impl PrimordialField {
    /// The minimum allowed potential value.
    pub const MIN_POTENTIAL: f64 = -1e308;

    /// The maximum allowed potential value.
    pub const MAX_POTENTIAL: f64 = 1e308;

    /// The minimum allowed fluctuation amplitude.
    pub const MIN_FLUCTUATION: f64 = 0.0;

    /// The maximum allowed fluctuation amplitude.
    pub const MAX_FLUCTUATION: f64 = 1e308;

    /// Creates a new primordial field with the specified
    /// potential and fluctuation amplitude.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::OutOfRange`] if `potential`
    /// or `fluctuation_amplitude` is not finite, or if
    /// `fluctuation_amplitude` is negative.
    pub fn new(
        potential: f64,
        fluctuation_amplitude: f64,
    ) -> GenesisResult<Self> {
        if potential.is_nan() || potential.is_infinite() {
            return Err(GenesisError::OutOfRange {
                field: "potential".to_string(),
                value: potential,
                min: Self::MIN_POTENTIAL,
                max: Self::MAX_POTENTIAL,
            });
        }
        if fluctuation_amplitude.is_nan()
            || fluctuation_amplitude.is_infinite()
        {
            return Err(GenesisError::OutOfRange {
                field: "fluctuation_amplitude".to_string(),
                value: fluctuation_amplitude,
                min: Self::MIN_FLUCTUATION,
                max: Self::MAX_FLUCTUATION,
            });
        }
        if fluctuation_amplitude < Self::MIN_FLUCTUATION {
            return Err(GenesisError::OutOfRange {
                field: "fluctuation_amplitude".to_string(),
                value: fluctuation_amplitude,
                min: Self::MIN_FLUCTUATION,
                max: Self::MAX_FLUCTUATION,
            });
        }
        Ok(Self {
            potential,
            fluctuation_amplitude,
        })
    }

    /// Generates a deterministic pseudo-random fluctuation
    /// value based on a linear congruential scheme seeded
    /// by the current potential. Unlike `rand::random`, this
    /// produces reproducible results and has no external
    /// dependency.
    ///
    /// The fluctuation is in the range
    /// `[-fluctuation_amplitude, +fluctuation_amplitude]`.
    pub fn fluctuate(&mut self) -> f64 {
        let seed = self.potential.to_bits();
        let mix = seed ^ (seed >> 32);
        let pseudo_random = (mix as f64) / (u64::MAX as f64);
        let normalized = (pseudo_random - 0.5) * 2.0;
        let fluctuation = normalized * self.fluctuation_amplitude;
        self.potential += fluctuation;
        self.potential = self.potential
            .clamp(Self::MIN_POTENTIAL, Self::MAX_POTENTIAL);
        fluctuation
    }

    /// Applies a stabilization pass that reduces the
    /// fluctuation amplitude toward zero, simulating
    /// the cooling of the primordial field.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::OutOfRange`] if the
    /// resulting amplitude would be negative.
    pub fn cool(&mut self, factor: f64) -> GenesisResult<()> {
        if factor.is_nan() || factor < 0.0 || factor > 1.0 {
            return Err(GenesisError::OutOfRange {
                field: "cooling_factor".to_string(),
                value: factor,
                min: 0.0,
                max: 1.0,
            });
        }
        self.fluctuation_amplitude *= 1.0 - factor;
        Ok(())
    }

    /// Returns the current energy density of the field,
    /// computed as `potential.abs() * fluctuation_amplitude`.
    pub fn energy_density(&self) -> f64 {
        self.potential.abs() * self.fluctuation_amplitude
    }

    /// Validates the primordial field, ensuring potential
    /// and fluctuation amplitude are finite and within
    /// valid bounds.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::OutOfRange`] if either value
    /// is invalid.
    pub fn validate(&self) -> GenesisResult<()> {
        if self.potential.is_nan() || self.potential.is_infinite() {
            return Err(GenesisError::OutOfRange {
                field: "potential".to_string(),
                value: self.potential,
                min: Self::MIN_POTENTIAL,
                max: Self::MAX_POTENTIAL,
            });
        }
        if self.fluctuation_amplitude.is_nan()
            || self.fluctuation_amplitude.is_infinite()
        {
            return Err(GenesisError::OutOfRange {
                field: "fluctuation_amplitude".to_string(),
                value: self.fluctuation_amplitude,
                min: Self::MIN_FLUCTUATION,
                max: Self::MAX_FLUCTUATION,
            });
        }
        if self.fluctuation_amplitude < Self::MIN_FLUCTUATION {
            return Err(GenesisError::OutOfRange {
                field: "fluctuation_amplitude".to_string(),
                value: self.fluctuation_amplitude,
                min: Self::MIN_FLUCTUATION,
                max: Self::MAX_FLUCTUATION,
            });
        }
        Ok(())
    }
}

impl Default for PrimordialField {
    fn default() -> Self {
        Self {
            potential: 0.0,
            fluctuation_amplitude: 1.0,
        }
    }
}
