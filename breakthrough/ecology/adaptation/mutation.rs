// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// AdaptiveMutation: Random changes that can improve fitness.
///
/// Models the mutation process that introduces variation into populations.
/// Mutations can be beneficial, neutral, or deleterious.
pub struct AdaptiveMutation {
    /// Probability of mutation per entity per generation, in [0.0, 1.0].
    pub mutation_rate: f64,
    /// Scale of mutation effect on trait values.
    pub mutation_scale: f64,
    /// Probability that a mutation is beneficial, in [0.0, 1.0].
    pub beneficial_probability: f64,
}

impl AdaptiveMutation {
    /// Minimum valid mutation rate.
    pub const MIN_RATE: f64 = 0.0;
    /// Maximum valid mutation rate.
    pub const MAX_RATE: f64 = 1.0;
    /// Minimum valid mutation scale (must be non-negative).
    pub const MIN_SCALE: f64 = 0.0;
    /// Minimum valid beneficial probability.
    pub const MIN_BENEFICIAL: f64 = 0.0;
    /// Maximum valid beneficial probability.
    pub const MAX_BENEFICIAL: f64 = 1.0;

    /// Creates a new AdaptiveMutation instance.
    ///
    /// # Errors
    /// Returns `MutationError::InvalidRate` if mutation_rate is outside [0.0, 1.0].
    /// Returns `MutationError::InvalidScale` if mutation_scale is negative.
    /// Returns `MutationError::InvalidProbability` if beneficial_probability is outside [0.0, 1.0].
    pub fn new(
        mutation_rate: f64,
        mutation_scale: f64,
        beneficial_probability: f64,
    ) -> Result<Self, MutationError> {
        if !(Self::MIN_RATE..=Self::MAX_RATE).contains(&mutation_rate) {
            return Err(MutationError::InvalidRate { rate: mutation_rate });
        }
        if mutation_scale < Self::MIN_SCALE {
            return Err(MutationError::InvalidScale { scale: mutation_scale });
        }
        if !(Self::MIN_BENEFICIAL..=Self::MAX_BENEFICIAL).contains(&beneficial_probability) {
            return Err(MutationError::InvalidProbability {
                probability: beneficial_probability,
            });
        }
        Ok(Self {
            mutation_rate,
            mutation_scale,
            beneficial_probability,
        })
    }

    /// Applies mutations to a population of trait values.
    ///
    /// Returns the mutated population and a list of mutation magnitudes applied.
    pub fn apply(&self, population: &[f64]) -> (Vec<f64>, Vec<f64>) {
        let mut rng_state = 12345u64;
        let mut next_random = || {
            rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
            ((rng_state >> 16) & 0x7FFF) as f64 / 32767.0
        };

        let mut result = Vec::with_capacity(population.len());
        let mut magnitudes = Vec::with_capacity(population.len());
        for &trait_val in population {
            let mut new_val = trait_val;
            let mut magnitude = 0.0;
            if next_random() < self.mutation_rate {
                let direction = if next_random() < self.beneficial_probability {
                    1.0
                } else {
                    -1.0
                };
                magnitude = direction * self.mutation_scale * next_random();
                new_val = (trait_val + magnitude).clamp(0.0, 1.0);
            }
            result.push(new_val);
            magnitudes.push(magnitude);
        }
        (result, magnitudes)
    }

    /// Returns the expected number of mutations per generation.
    pub fn expected_mutations(&self, population_size: usize) -> f64 {
        population_size as f64 * self.mutation_rate
    }

    /// Returns the expected beneficial mutation count.
    pub fn expected_beneficial(&self, population_size: usize) -> f64 {
        self.expected_mutations(population_size) * self.beneficial_probability
    }

    /// Validates the mutation parameters.
    pub fn validate(&self) -> Result<(), MutationError> {
        Self::new(self.mutation_rate, self.mutation_scale, self.beneficial_probability)?;
        Ok(())
    }
}

/// Error type for mutation operation failures.
#[derive(Debug, Clone, PartialEq)]
pub enum MutationError {
    InvalidRate { rate: f64 },
    InvalidScale { scale: f64 },
    InvalidProbability { probability: f64 },
}

impl std::fmt::Display for MutationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MutationError::InvalidRate { rate } => write!(f, "Invalid mutation rate: {}", rate),
            MutationError::InvalidScale { scale } => write!(f, "Invalid mutation scale: {}", scale),
            MutationError::InvalidProbability { probability } => write!(f, "Invalid beneficial probability: {}", probability),
        }
    }
}

impl std::error::Error for MutationError {}