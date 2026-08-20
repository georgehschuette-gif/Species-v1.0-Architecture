// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// MutualisticBond: A relationship where both species benefit.
pub struct MutualisticBond {
    pub species_a: u64,
    pub species_b: u64,
    pub benefit_a: f64,
    pub benefit_b: f64,
    pub stability: f64,
}

impl MutualisticBond {
    /// Constructs a new MutualisticBond with validated parameters.
    ///
    /// # Errors
    /// Returns `InvalidBenefit` if either benefit is negative, NaN, or infinite.
    /// Returns `InvalidStability` if stability is outside [0.0, 1.0].
    /// Returns `SpeciesNotFound` if either species ID is zero.
    pub fn new(
        species_a: u64,
        species_b: u64,
        benefit_a: f64,
        benefit_b: f64,
        stability: f64,
    ) -> Result<Self, SymbiosisError> {
        if species_a == 0 {
            return Err(SymbiosisError::SpeciesNotFound(
                "Species A ID must be non-zero".to_string(),
            ));
        }
        if species_b == 0 {
            return Err(SymbiosisError::SpeciesNotFound(
                "Species B ID must be non-zero".to_string(),
            ));
        }
        if benefit_a.is_nan() || benefit_a.is_infinite() || benefit_a < 0.0 {
            return Err(SymbiosisError::InvalidBenefit(
                "Benefit A must be a non-negative finite number".to_string(),
            ));
        }
        if benefit_b.is_nan() || benefit_b.is_infinite() || benefit_b < 0.0 {
            return Err(SymbiosisError::InvalidBenefit(
                "Benefit B must be a non-negative finite number".to_string(),
            ));
        }
        if stability.is_nan() || stability.is_infinite() || !(0.0..=1.0).contains(&stability) {
            return Err(SymbiosisError::InvalidStability(
                "Stability must be in the range [0.0, 1.0]".to_string(),
            ));
        }
        Ok(Self {
            species_a,
            species_b,
            benefit_a,
            benefit_b,
            stability,
        })
    }

    /// Validates the internal consistency of this mutualistic bond.
    pub fn validate(&self) -> Result<(), SymbiosisError> {
        if self.species_a == 0 {
            return Err(SymbiosisError::SpeciesNotFound(
                "Species A ID must be non-zero".to_string(),
            ));
        }
        if self.species_b == 0 {
            return Err(SymbiosisError::SpeciesNotFound(
                "Species B ID must be non-zero".to_string(),
            ));
        }
        if self.benefit_a.is_nan() || self.benefit_a.is_infinite() || self.benefit_a < 0.0 {
            return Err(SymbiosisError::InvalidBenefit(
                "Benefit A must be a non-negative finite number".to_string(),
            ));
        }
        if self.benefit_b.is_nan() || self.benefit_b.is_infinite() || self.benefit_b < 0.0 {
            return Err(SymbiosisError::InvalidBenefit(
                "Benefit B must be a non-negative finite number".to_string(),
            ));
        }
        if self.stability.is_nan() || self.stability.is_infinite() || !(0.0..=1.0).contains(&self.stability) {
            return Err(SymbiosisError::InvalidStability(
                "Stability must be in the range [0.0, 1.0]".to_string(),
            ));
        }
        Ok(())
    }

    /// Returns the ratio of benefit A to benefit B.
    ///
    /// Returns `f64::INFINITY` if benefit B is zero.
    pub fn benefit_ratio(&self) -> f64 {
        if self.benefit_b == 0.0 {
            f64::INFINITY
        } else {
            self.benefit_a / self.benefit_b
        }
    }

    /// Returns true if the bond is considered stable (stability > 0.5).
    pub fn is_stable(&self) -> bool {
        self.stability > 0.5
    }

    /// Returns the average benefit across both species.
    pub fn strength(&self) -> f64 {
        (self.benefit_a + self.benefit_b) / 2.0
    }

    /// Returns the symbiosis type for this bond.
    pub fn interaction_type(&self) -> SymbiosisType {
        SymbiosisType::Mutualism
    }

    /// Computes the combined fitness gain for both species.
    pub fn combined_fitness_gain(&self) -> f64 {
        self.benefit_a + self.benefit_b
    }

    /// Returns the species with the higher benefit.
    pub fn dominant_beneficiary(&self) -> u64 {
        if self.benefit_a >= self.benefit_b {
            self.species_a
        } else {
            self.species_b
        }
    }
}

impl std::fmt::Display for MutualisticBond {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MutualisticBond(species_a={}, species_b={}, benefit_a={:.4}, benefit_b={:.4}, stability={:.4})",
            self.species_a, self.species_b, self.benefit_a, self.benefit_b, self.stability
        )
    }
}