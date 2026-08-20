// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// PopulationRegulation: Mechanisms that keep population size within bounds.
///
/// Encapsulates density-dependent regulation, resource limits, and regulatory
/// strength. Provides methods to constrain growth and compute pressure on the population.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PopulationRegulation {
    pub density_dependence: f64,
    pub resource_limit: f64,
    pub regulatory_strength: f64,
}

impl PopulationRegulation {
    /// Create a new PopulationRegulation with validated parameters.
    ///
    /// # Errors
    /// Returns `InvalidRate` if `density_dependence` is outside [0.0, 1.0].
    /// Returns `InvalidRate` if `resource_limit` is negative, NaN, or infinite.
    /// Returns `InvalidRate` if `regulatory_strength` is negative, NaN, or infinite.
    pub fn new(density_dependence: f64, resource_limit: f64, regulatory_strength: f64) -> Result<Self, PopulationError> {
        if density_dependence.is_nan()
            || density_dependence.is_infinite()
            || !(0.0..=1.0).contains(&density_dependence)
        {
            return Err(PopulationError::InvalidRate(
                "Density dependence must be in the range [0.0, 1.0]".into(),
            ));
        }
        if resource_limit.is_nan() || resource_limit.is_infinite() || resource_limit < 0.0 {
            return Err(PopulationError::InvalidRate(
                "Resource limit must be a non-negative finite number".into(),
            ));
        }
        if regulatory_strength.is_nan() || regulatory_strength.is_infinite() || regulatory_strength < 0.0 {
            return Err(PopulationError::InvalidRate(
                "Regulatory strength must be a non-negative finite number".into(),
            ));
        }

        Ok(Self { density_dependence, resource_limit, regulatory_strength })
    }

    /// Effective population growth rate after density-dependent regulation.
    ///
    /// Applies a logistic correction: `r_eff = r * (1 - alpha * N/K)`.
    pub fn effective_growth_rate(&self, base_rate: f64, current_size: usize, carrying_capacity: usize) -> f64 {
        if carrying_capacity == 0 {
            return 0.0;
        }
        let density = current_size as f64 / carrying_capacity as f64;
        let density_effect = 1.0 - self.density_dependence * density;
        base_rate * density_effect * self.regulatory_strength
    }

    /// Limit a population size based on resource constraints.
    ///
    /// Returns `size` clamped to the effective resource limit: `K * resource_limit`.
    pub fn limit_size(&self, size: usize, carrying_capacity: usize) -> usize {
        let effective_limit = (carrying_capacity as f64 * self.resource_limit.min(1.0)).round() as usize;
        let hard_floor = 1usize;
        size.min(effective_limit.max(hard_floor))
    }

    /// Regulatory pressure exerted on the population at a given size and capacity.
    ///
    /// Returns a value in [0, 1] representing how strongly regulation suppresses growth.
    pub fn regulatory_pressure(&self, current_size: usize, carrying_capacity: usize) -> f64 {
        if carrying_capacity == 0 {
            return 0.0;
        }
        let density = (current_size as f64 / carrying_capacity as f64).min(1.0);
        self.regulatory_strength * density * self.density_dependence
    }

    /// Fraction of the carrying capacity consumed by the current population.
    pub fn resource_utilization(&self, current_size: usize, carrying_capacity: usize) -> f64 {
        if carrying_capacity == 0 {
            return 0.0;
        }
        (current_size as f64 / carrying_capacity as f64) * self.resource_limit.min(1.0)
    }

    /// Return whether regulation is strong enough to prevent overpopulation.
    pub fn prevents_overpopulation(&self, current_size: usize, carrying_capacity: usize) -> bool {
        if carrying_capacity == 0 {
            return true;
        }
        self.limit_size(current_size, carrying_capacity) <= current_size
            && self.limit_size(current_size * 2, carrying_capacity) < current_size * 2
    }
}

impl Default for PopulationRegulation {
    fn default() -> Self {
        Self { density_dependence: 1.0, resource_limit: 1.0, regulatory_strength: 1.0 }
    }
}
