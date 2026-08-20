// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// PopulationDynamics: Models the size and composition changes of populations over time.
///
/// Tracks current population size, intrinsic growth rate, and carrying capacity.
/// Provides step-wise simulation methods that combine explicit growth and regulation.
#[derive(Debug, Clone, PartialEq)]
pub struct PopulationDynamics {
    pub size: usize,
    pub growth_rate: f64,
    pub carrying_capacity: usize,
}

impl PopulationDynamics {
    /// Create a new PopulationDynamics with validated parameters.
    ///
    /// # Errors
    /// Returns `InvalidSize` if `size` or `carrying_capacity` is zero.
    /// Returns `InvalidGrowthRate` if `growth_rate` is NaN or infinite.
    /// Returns `InfeasiblePopulation` if `size` exceeds `carrying_capacity`.
    pub fn new(size: usize, growth_rate: f64, carrying_capacity: usize) -> Result<Self, PopulationError> {
        if size == 0 {
            return Err(PopulationError::InvalidSize("Population size must be positive".into()));
        }
        if carrying_capacity == 0 {
            return Err(PopulationError::InvalidSize("Carrying capacity must be positive".into()));
        }
        if growth_rate.is_nan() || growth_rate.is_infinite() {
            return Err(PopulationError::InvalidGrowthRate("Growth rate must be a finite number".into()));
        }
        if size > carrying_capacity {
            return Err(PopulationError::InfeasiblePopulation);
        }

        Ok(Self { size, growth_rate, carrying_capacity })
    }

    /// Simulate one discrete time step using explicit births, deaths, and regulation.
    ///
    /// Births and deaths are computed from `growth`, then the resulting size is
    /// clamped by `regulation`. Updates `self.size` and `self.growth_rate` in place.
    ///
    /// # Errors
    /// Returns `Underflow` if regulation reduces the population to zero.
    pub fn step(&mut self, growth: &PopulationGrowth, regulation: &PopulationRegulation) -> Result<(), PopulationError> {
        let births = growth.births_at(self.size);
        let deaths = growth.deaths_at(self.size);
        let current = (self.size as f64) + births - deaths;
        let current_size = current.round() as usize;

        let regulated = regulation.limit_size(current_size, self.carrying_capacity);

        if regulated == 0 {
            return Err(PopulationError::Underflow);
        }

        self.growth_rate = if self.carrying_capacity > 0 {
            (regulated as f64 / self.carrying_capacity as f64).max(0.0).min(1.0)
        } else {
            0.0
        };

        self.size = regulated;
        Ok(())
    }

    /// Apply logistic growth for one discrete time step.
    ///
    /// Uses the standard logistic equation: dN/dt = rN(1 - N/K).
    /// Updates `self.size` in place, clamped to `[1, carrying_capacity]`.
    pub fn logistic_step(&mut self, growth: &PopulationGrowth) {
        let r = growth.net_growth_rate();
        let n = self.size as f64;
        let k = self.carrying_capacity as f64;

        if k > 0.0 {
            let delta = r * n * (1.0 - n / k);
            let new_size = (n + delta).round() as usize;
            self.size = new_size.max(1).min(self.carrying_capacity);
        }
    }

    /// Project the population size after `t` time steps assuming unrestricted exponential growth.
    ///
    /// The result is clamped to `self.carrying_capacity`.
    pub fn project(&self, t: usize, growth: &PopulationGrowth) -> usize {
        let r = growth.net_growth_rate();
        let n0 = self.size as f64;
        let k = self.carrying_capacity as f64;

        let λ = 1.0 + r;
        let projected = if λ > 0.0 {
            n0 * λ.powi(t as i32)
        } else {
            // If λ is negative or zero, population collapses to zero
            0.0
        };

        if k > 0.0 {
            projected.min(k).round() as usize
        } else {
            projected.round() as usize
        }
    }

    /// Check if the population is near its carrying capacity.
    ///
    /// Returns true if `size / carrying_capacity >= threshold`.
    pub fn is_at_capacity(&self, threshold: f64) -> bool {
        if self.carrying_capacity == 0 {
            return false;
        }
        (self.size as f64 / self.carrying_capacity as f64) >= threshold
    }

    /// Update the carrying capacity, validating consistency with current size.
    ///
    /// # Errors
    /// Returns `InvalidSize` if `new_k` is zero.
    /// Returns `InfeasiblePopulation` if current size exceeds `new_k`.
    pub fn update_carrying_capacity(&mut self, new_k: usize) -> Result<(), PopulationError> {
        if new_k == 0 {
            return Err(PopulationError::InvalidSize(
                "Carrying capacity must be positive".into(),
            ));
        }
        if self.size > new_k {
            return Err(PopulationError::InfeasiblePopulation);
        }
        self.carrying_capacity = new_k;
        Ok(())
    }

    /// Approximate maximum sustainable yield under logistic growth assumptions.
    ///
    /// Returns K/4, the theoretical maximum harvest rate at half the carrying capacity.
    pub fn maximum_sustainable_yield(&self) -> f64 {
        let k = self.carrying_capacity as f64;
        if k > 0.0 { k / 4.0 } else { 0.0 }
    }

    /// Determine whether the current growth trajectory is sustainable given the carrying capacity.
    ///
    /// A population is unsustainable if its projected size after 10 steps exceeds
    /// the carrying capacity with positive growth, or if it is already extinct.
    pub fn is_sustainable(&self, growth: &PopulationGrowth) -> bool {
        let r = growth.net_growth_rate();
        if r <= 0.0 {
            return self.size <= self.carrying_capacity && self.size > 0;
        }
        let projected = self.project(10, growth);
        projected <= self.carrying_capacity && self.size > 0
    }

    /// Return the current relative utilization of carrying capacity.
    pub fn utilization(&self) -> f64 {
        if self.carrying_capacity == 0 {
            0.0
        } else {
            self.size as f64 / self.carrying_capacity as f64
        }
    }
}
