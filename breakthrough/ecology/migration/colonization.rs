// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// ColonizationEvent: The successful establishment of a species in a new habitat.
pub struct ColonizationEvent {
    pub species_id: u64,
    pub target_habitat: u64,
    pub founding_population: usize,
    pub success_probability: f64,
}

impl ColonizationEvent {
    /// Construct a new [ColonizationEvent].
    ///
    /// # Errors
    /// Returns [MigrationError::UninitializedReference] if `species_id` or
    /// `target_habitat` is zero.
    /// Returns [MigrationError::FoundingPopulationTooSmall] if
    /// `founding_population` is zero.
    /// Returns [MigrationError::InvalidColonizationProbability] if
    /// `success_probability` is outside [0.0, 1.0].
    pub fn new(
        species_id: u64,
        target_habitat: u64,
        founding_population: usize,
        success_probability: f64,
    ) -> Result<Self, MigrationError> {
        if species_id == 0 {
            return Err(MigrationError::UninitializedReference);
        }
        if target_habitat == 0 {
            return Err(MigrationError::InvalidTargetHabitat);
        }
        if founding_population == 0 {
            return Err(MigrationError::FoundingPopulationTooSmall);
        }
        if success_probability.is_nan()
            || success_probability < 0.0
            || success_probability > 1.0
        {
            return Err(MigrationError::InvalidColonizationProbability);
        }
        Ok(Self {
            species_id,
            target_habitat,
            founding_population,
            success_probability,
        })
    }

    /// Validate the colonization event's parameters are consistent.
    pub fn validate(&self) -> Result<(), MigrationError> {
        if self.species_id == 0 {
            return Err(MigrationError::UninitializedReference);
        }
        if self.target_habitat == 0 {
            return Err(MigrationError::InvalidTargetHabitat);
        }
        if self.founding_population == 0 {
            return Err(MigrationError::FoundingPopulationTooSmall);
        }
        if self.success_probability.is_nan()
            || self.success_probability < 0.0
            || self.success_probability > 1.0
        {
            return Err(MigrationError::InvalidColonizationProbability);
        }
        Ok(())
    }

    /// Simulate the colonization event, returning whether it succeeds.
    ///
    /// Uses a deterministic pseudo-random draw against the success probability.
    pub fn simulate(&self) -> bool {
        if self.success_probability >= 1.0 {
            return true;
        }
        if self.success_probability <= 0.0 {
            return false;
        }
        let rand_val = Self::pseudo_random(self.species_id, self.target_habitat);
        rand_val < self.success_probability
    }

    /// Compute the expected post-colonization population size,
    /// accounting for the founding population and initial growth rate.
    ///
    /// The initial growth rate is assumed to be 1.5x the founding population
    /// per time step for the first step, then stabilizing.
    pub fn expected_initial_growth(&self) -> usize {
        let growth = (self.founding_population as f64) * 1.5;
        growth.min((self.founding_population * 10) as f64) as usize
    }

    /// Return the risk of colonization failure as a probability.
    pub fn failure_risk(&self) -> f64 {
        1.0 - self.success_probability
    }

    /// Assess whether this colonization event carries too much risk
    /// given the engine's extinction threshold.
    ///
    /// Returns `true` if the failure risk is within acceptable bounds
    /// (i.e., the colonization is viable).
    pub fn is_viable(&self, extinction_threshold: f64) -> Result<bool, MigrationError> {
        if extinction_threshold.is_nan() || extinction_threshold < 0.0 || extinction_threshold > 1.0 {
            return Err(MigrationError::InvalidColonizationProbability);
        }
        Ok(self.failure_risk() <= extinction_threshold)
    }

    /// Compute a colonization score representing the expected value
    /// of this colonization attempt. Higher scores indicate more
    /// desirable colonization targets.
    pub fn colonization_score(&self) -> f64 {
        self.success_probability * (self.founding_population as f64).sqrt()
    }

    /// Return a summary of the colonization event.
    pub fn summary(&self) -> String {
        format!(
            "ColonizationEvent(species={}, habitat={}, founders={}, success={:.2}, risk={:.2})",
            self.species_id,
            self.target_habitat,
            self.founding_population,
            self.success_probability,
            self.failure_risk(),
        )
    }

    fn pseudo_random(species_id: u64, habitat_id: u64) -> f64 {
        let mut x = (species_id as u32).wrapping_mul(0x9E3779B9u32);
        x ^= x >> 16;
        x = x.wrapping_mul(0x85EBCA6Bu32);
        x ^= (habitat_id as u32).wrapping_add(0xC2B2AE35u32);
        x ^= x >> 13;
        let result = (x as f64) / u32::MAX as f64;
        result
    }
}

impl Default for ColonizationEvent {
    fn default() -> Self {
        Self {
            species_id: 1,
            target_habitat: 1,
            founding_population: 10,
            success_probability: 0.5,
        }
    }
}