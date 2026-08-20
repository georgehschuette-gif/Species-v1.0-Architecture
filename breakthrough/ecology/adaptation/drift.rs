// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// GeneticDrift: Random changes in trait frequencies due to sampling effects.
///
/// Models the stochastic fluctuation of trait values in finite populations.
/// Drift is strongest in small populations and can lead to fixation or loss.
pub struct GeneticDrift {
    /// Current effective population size.
    pub population_size: usize,
    /// Intensity of drift effects, in [0.0, 1.0].
    pub drift_intensity: f64,
    /// Probability of eventual fixation for a neutral allele, in [0.0, 1.0].
    pub fixation_probability: f64,
}

impl GeneticDrift {
    /// Minimum valid population size.
    pub const MIN_POPULATION: usize = 1;
    /// Maximum valid drift intensity.
    pub const MAX_INTENSITY: f64 = 1.0;
    /// Minimum valid fixation probability.
    pub const MIN_FIXATION: f64 = 0.0;
    /// Maximum valid fixation probability.
    pub const MAX_FIXATION: f64 = 1.0;

    /// Creates a new GeneticDrift instance.
    ///
    /// # Errors
    /// Returns `DriftError::InvalidPopulation` if population_size is zero.
    /// Returns `DriftError::InvalidIntensity` if drift_intensity is outside [0.0, 1.0].
    /// Returns `DriftError::InvalidProbability` if fixation_probability is outside [0.0, 1.0].
    pub fn new(
        population_size: usize,
        drift_intensity: f64,
        fixation_probability: f64,
    ) -> Result<Self, DriftError> {
        if population_size < Self::MIN_POPULATION {
            return Err(DriftError::InvalidPopulation { size: population_size });
        }
        if !(0.0..=Self::MAX_INTENSITY).contains(&drift_intensity) {
            return Err(DriftError::InvalidIntensity { intensity: drift_intensity });
        }
        if !(Self::MIN_FIXATION..=Self::MAX_FIXATION).contains(&fixation_probability) {
            return Err(DriftError::InvalidProbability {
                probability: fixation_probability,
            });
        }
        Ok(Self {
            population_size,
            drift_intensity,
            fixation_probability,
        })
    }

    /// Returns the variance in allele frequency due to drift per generation.
    ///
    /// Formula: p * (1 - p) / (2 * N_e)
    pub fn drift_variance(&self, allele_frequency: f64) -> Result<f64, DriftError> {
        if !(0.0..=1.0).contains(&allele_frequency) {
            return Err(DriftError::InvalidProbability {
                probability: allele_frequency,
            });
        }
        if self.population_size == 0 {
            return Err(DriftError::InvalidPopulation { size: 0 });
        }
        Ok(allele_frequency * (1.0 - allele_frequency) / (2.0 * self.population_size as f64))
    }

    /// Simulates drift for a single generation on an allele frequency.
    ///
    /// Uses a simple binomial sampling model.
    pub fn simulate_step(&self, allele_frequency: f64) -> Result<f64, DriftError> {
        if !(0.0..=1.0).contains(&allele_frequency) {
            return Err(DriftError::InvalidProbability {
                probability: allele_frequency,
            });
        }
        let mut rng_state = 98765u64;
        let mut next_random = || {
            rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
            ((rng_state >> 16) & 0x7FFF) as f64 / 32767.0
        };

        let copies = (allele_frequency * 2.0 * self.population_size as f64).round() as usize;
        let mut new_copies = 0;
        for _ in 0..(2 * self.population_size) {
            if next_random() < allele_frequency {
                new_copies += 1;
            }
        }
        let new_freq = new_copies as f64 / (2.0 * self.population_size as f64);
        Ok(new_freq.clamp(0.0, 1.0))
    }

    /// Returns the expected time to fixation (in generations) for a neutral allele.
    pub fn time_to_fixation(&self) -> f64 {
        4.0 * self.population_size as f64
    }

    /// Returns the probability that a neutral allele with the given frequency will fix.
    pub fn neutral_fixation_probability(&self, allele_frequency: f64) -> f64 {
        allele_frequency
    }

    /// Validates drift parameters.
    pub fn validate(&self) -> Result<(), DriftError> {
        Self::new(self.population_size, self.drift_intensity, self.fixation_probability)?;
        Ok(())
    }
}

/// Error type for drift operation failures.
#[derive(Debug, Clone, PartialEq)]
pub enum DriftError {
    InvalidPopulation { size: usize },
    InvalidIntensity { intensity: f64 },
    InvalidProbability { probability: f64 },
    ComputationError(String),
}

impl std::fmt::Display for DriftError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DriftError::InvalidPopulation { size } => write!(f, "Invalid population size: {}", size),
            DriftError::InvalidIntensity { intensity } => write!(f, "Invalid drift intensity: {}", intensity),
            DriftError::InvalidProbability { probability } => write!(f, "Invalid probability: {}", probability),
            DriftError::ComputationError(msg) => write!(f, "Computation error: {}", msg),
        }
    }
}

impl std::error::Error for DriftError {}
