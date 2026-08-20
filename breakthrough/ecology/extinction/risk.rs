// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// ExtinctionRisk: Predicts the probability of species extinction based on
/// population dynamics, habitat fragmentation, and environmental stochasticity.
pub struct ExtinctionRisk {
    pub population_threshold: usize,
    pub habitat_fragmentation: f64,
    pub environmental_variance: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RiskAssessment {
    pub probability: f64,
    pub confidence: f64,
    pub dominant_factor: RiskFactor,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RiskFactor {
    Population,
    Habitat,
    Environmental,
}

impl ExtinctionRisk {
    /// Create a new ExtinctionRisk model with validated parameters.
    pub fn new(population_threshold: usize) -> Self {
        Self {
            population_threshold,
            habitat_fragmentation: 0.0,
            environmental_variance: 0.0,
        }
    }

    /// Set habitat fragmentation factor in the range [0.0, 1.0].
    pub fn with_fragmentation(mut self, fragmentation: f64) -> Result<Self, ExtinctionError> {
        self.set_fragmentation(fragmentation)?;
        Ok(self)
    }

    /// Set environmental variance (σ²) parameter.
    pub fn with_variance(mut self, variance: f64) -> Result<Self, ExtinctionError> {
        self.set_variance(variance)?;
        Ok(self)
    }

    /// Update habitat fragmentation, clamped to [0.0, 1.0].
    pub fn set_fragmentation(&mut self, fragmentation: f64) -> Result<(), ExtinctionError> {
        if fragmentation < 0.0 || fragmentation > 1.0 {
            return Err(ExtinctionError::InvalidProbability(fragmentation));
        }
        self.habitat_fragmentation = fragmentation;
        Ok(())
    }

    /// Update environmental variance. Must be non-negative.
    pub fn set_variance(&mut self, variance: f64) -> Result<(), ExtinctionError> {
        if variance < 0.0 {
            return Err(ExtinctionError::InvalidProbability(variance));
        }
        self.environmental_variance = variance;
        Ok(())
    }

    /// Assess extinction probability for a given population size.
    /// Uses a heuristic that combines threshold ratio, fragmentation penalty,
    /// and environmental variance to produce a probability in [0.0, 1.0].
    pub fn assess(&self, population: usize) -> Result<RiskAssessment, ExtinctionError> {
        if self.population_threshold == 0 {
            return Err(ExtinctionError::EmptyPopulation);
        }

        let pop_ratio = population as f64 / self.population_threshold as f64;
        let population_factor = 1.0 / (1.0 + pop_ratio);

        let fragmentation_penalty = self.habitat_fragmentation.powi(2);
        let env_penalty = 1.0 - (-0.5 * self.environmental_variance).exp();

        let probability = population_factor * 0.5
            + fragmentation_penalty * 0.3
            + env_penalty * 0.2;

        let probability = probability.clamp(0.0, 1.0);

        let confidence = 1.0 - fragmentation_penalty;

        let dominant_factor = if population_factor >= fragmentation_penalty && population_factor >= env_penalty {
            RiskFactor::Population
        } else if fragmentation_penalty >= env_penalty {
            RiskFactor::Habitat
        } else {
            RiskFactor::Environmental
        };

        Ok(RiskAssessment {
            probability,
            confidence,
            dominant_factor,
        })
    }

    /// Return true if the model considers the species at high risk (probability > 0.7).
    pub fn is_high_risk(&self, population: usize) -> Result<bool, ExtinctionError> {
        self.assess(population).map(|a| a.probability > 0.7)
    }

    /// Compute the critical population size at which risk exceeds the threshold value.
    pub fn critical_population(&self, risk_threshold: f64) -> Result<usize, ExtinctionError> {
        if !(0.0..=1.0).contains(&risk_threshold) {
            return Err(ExtinctionError::InvalidProbability(risk_threshold));
        }
        if risk_threshold >= 1.0 {
            return Ok(self.population_threshold.saturating_sub(1));
        }

        let adjusted = self.habitat_fragmentation.powi(2) * 0.3 / 0.5
            + (1.0 - (-0.5 * self.environmental_variance).exp()) * 0.2 / 0.5;
        let ratio = (1.0 - risk_threshold) / (1.0 - adjusted);
        let pop = (ratio * self.population_threshold as f64).ceil() as usize;

        Ok(pop)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExtinctionError {
    InvalidProbability(f64),
    EmptyPopulation,
    UnfeasibleRecovery,
    UnrecognizedCause,
}

impl std::fmt::Display for ExtinctionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidProbability(value) => write!(f, "invalid probability value: {}", value),
            Self::EmptyPopulation => write!(f, "population threshold is zero"),
            Self::UnfeasibleRecovery => write!(f, "recovery strategy is not feasible"),
            Self::UnrecognizedCause => write!(f, "extinction cause not recognized"),
        }
    }
}

impl std::error::Error for ExtinctionError {}
