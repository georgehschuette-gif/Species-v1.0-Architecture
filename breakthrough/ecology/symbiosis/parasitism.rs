// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// ParasiticInteraction: A relationship where one species benefits at the other's expense.
pub struct ParasiticInteraction {
    pub parasite: u64,
    pub host: u64,
    pub exploitation_rate: f64,
    pub virulence: f64,
    pub resistance: f64,
}

impl ParasiticInteraction {
    /// Constructs a new ParasiticInteraction with validated parameters.
    ///
    /// # Errors
    /// Returns `InvalidExploitation` if exploitation_rate is negative, NaN, or infinite.
    /// Returns `InvalidVirulence` if virulence is outside [0.0, 1.0].
    /// Returns `InvalidStability` if resistance is outside [0.0, 1.0].
    /// Returns `SpeciesNotFound` if either species ID is zero.
    pub fn new(
        parasite: u64,
        host: u64,
        exploitation_rate: f64,
        virulence: f64,
        resistance: f64,
    ) -> Result<Self, SymbiosisError> {
        if parasite == 0 {
            return Err(SymbiosisError::SpeciesNotFound(
                "Parasite ID must be non-zero".to_string(),
            ));
        }
        if host == 0 {
            return Err(SymbiosisError::SpeciesNotFound(
                "Host ID must be non-zero".to_string(),
            ));
        }
        if exploitation_rate.is_nan() || exploitation_rate.is_infinite() || exploitation_rate < 0.0 {
            return Err(SymbiosisError::InvalidExploitation(
                "Exploitation rate must be a non-negative finite number".to_string(),
            ));
        }
        if virulence.is_nan() || virulence.is_infinite() || !(0.0..=1.0).contains(&virulence) {
            return Err(SymbiosisError::InvalidVirulence(
                "Virulence must be in the range [0.0, 1.0]".to_string(),
            ));
        }
        if resistance.is_nan() || resistance.is_infinite() || !(0.0..=1.0).contains(&resistance) {
            return Err(SymbiosisError::InvalidStability(
                "Resistance must be in the range [0.0, 1.0]".to_string(),
            ));
        }
        Ok(Self {
            parasite,
            host,
            exploitation_rate,
            virulence,
            resistance,
        })
    }

    /// Validates the internal consistency of this parasitic interaction.
    pub fn validate(&self) -> Result<(), SymbiosisError> {
        if self.parasite == 0 {
            return Err(SymbiosisError::SpeciesNotFound(
                "Parasite ID must be non-zero".to_string(),
            ));
        }
        if self.host == 0 {
            return Err(SymbiosisError::SpeciesNotFound(
                "Host ID must be non-zero".to_string(),
            ));
        }
        if self.exploitation_rate.is_nan()
            || self.exploitation_rate.is_infinite()
            || self.exploitation_rate < 0.0
        {
            return Err(SymbiosisError::InvalidExploitation(
                "Exploitation rate must be a non-negative finite number".to_string(),
            ));
        }
        if self.virulence.is_nan()
            || self.virulence.is_infinite()
            || !(0.0..=1.0).contains(&self.virulence)
        {
            return Err(SymbiosisError::InvalidVirulence(
                "Virulence must be in the range [0.0, 1.0]".to_string(),
            ));
        }
        if self.resistance.is_nan()
            || self.resistance.is_infinite()
            || !(0.0..=1.0).contains(&self.resistance)
        {
            return Err(SymbiosisError::InvalidStability(
                "Resistance must be in the range [0.0, 1.0]".to_string(),
            ));
        }
        Ok(())
    }

    /// Returns the fitness gain of the parasite.
    pub fn parasite_fitness(&self) -> f64 {
        self.exploitation_rate * self.virulence
    }

    /// Returns the fitness loss of the host.
    pub fn host_fitness(&self) -> f64 {
        -self.exploitation_rate * self.virulence * (1.0 - self.resistance)
    }

    /// Returns true if the parasite is considered highly virulent.
    pub fn is_virulent(&self) -> bool {
        self.virulence > 0.7
    }

    /// Returns true if the host has significant resistance to the parasite.
    pub fn is_resistant(&self) -> bool {
        self.resistance > 0.5
    }

    /// Returns the symbiosis type for this interaction.
    pub fn interaction_type(&self) -> SymbiosisType {
        SymbiosisType::Parasitism
    }

    /// Computes the net effect on the host (always negative or zero).
    pub fn net_host_effect(&self) -> f64 {
        self.host_fitness()
    }

    /// Computes the net effect on the parasite (always positive or zero).
    pub fn net_parasite_effect(&self) -> f64 {
        self.parasite_fitness()
    }

    /// Returns the parasite species ID.
    pub fn parasite_id(&self) -> u64 {
        self.parasite
    }

    /// Returns the host species ID.
    pub fn host_id(&self) -> u64 {
        self.host
    }
}

impl std::fmt::Display for ParasiticInteraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ParasiticInteraction(parasite={}, host={}, exploitation_rate={:.4}, virulence={:.4}, resistance={:.4})",
            self.parasite, self.host, self.exploitation_rate, self.virulence, self.resistance
        )
    }
}