// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// NoiseModel: Characterizes uncertainty sources in perception.
pub struct NoiseModel {
    pub variance: f64,
    pub distribution: NoiseDistribution,
}

impl NoiseModel {
    /// Create a new noise model with the given variance and distribution.
    pub fn new(variance: f64, distribution: NoiseDistribution) -> PerceptionResult<Self> {
        if variance < 0.0 {
            return Err(PerceptionError::InvalidConfiguration(
                "variance must be non-negative".into(),
            ));
        }
        Ok(Self {
            variance,
            distribution,
        })
    }

    /// Sample a noise value from the configured distribution.
    pub fn sample(&self) -> f64 {
        self.distribution.sample(self.variance)
    }

    /// Evaluate the probability density at a given value.
    pub fn pdf(&self, x: f64) -> f64 {
        self.distribution.pdf(x, self.variance)
    }

    /// Compute the log-likelihood of an observed value.
    pub fn log_likelihood(&self, x: f64) -> f64 {
        self.distribution.pdf(x, self.variance).ln()
    }

    /// Update the variance, clamped to a minimum of 0.0.
    pub fn set_variance(&mut self, variance: f64) -> PerceptionResult<()> {
        if variance < 0.0 {
            return Err(PerceptionError::InvalidConfiguration(
                "variance must be non-negative".into(),
            ));
        }
        self.variance = variance;
        Ok(())
    }

    /// Validate that the noise model parameters are within valid ranges.
    pub fn validate(&self) -> PerceptionResult<()> {
        if self.variance < 0.0 {
            return Err(PerceptionError::InvalidConfiguration(
                "variance must be non-negative".into(),
            ));
        }
        Ok(())
    }
}