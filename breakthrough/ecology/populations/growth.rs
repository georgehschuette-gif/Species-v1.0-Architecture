// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// PopulationGrowth: Models the increase of population size through reproduction.
///
/// Encapsulates birth and death rates along with generation time.
/// Provides methods to compute net growth, doubling time, and population projection.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PopulationGrowth {
    pub birth_rate: f64,
    pub death_rate: f64,
    pub generation_time: f64,
}

impl PopulationGrowth {
    /// Create a new PopulationGrowth with validated parameters.
    ///
    /// # Errors
    /// Returns `InvalidRate` if any rate is negative, NaN, or infinite.
    /// Returns `InvalidRate` if `generation_time` is non-positive, NaN, or infinite.
    pub fn new(birth_rate: f64, death_rate: f64, generation_time: f64) -> Result<Self, PopulationError> {
        if birth_rate.is_nan() || birth_rate.is_infinite() || birth_rate < 0.0 {
            return Err(PopulationError::InvalidRate(
                "Birth rate must be a non-negative finite number".into(),
            ));
        }
        if death_rate.is_nan() || death_rate.is_infinite() || death_rate < 0.0 {
            return Err(PopulationError::InvalidRate(
                "Death rate must be a non-negative finite number".into(),
            ));
        }
        if generation_time.is_nan() || generation_time.is_infinite() || generation_time <= 0.0 {
            return Err(PopulationError::InvalidRate(
                "Generation time must be a positive finite number".into(),
            ));
        }

        Ok(Self { birth_rate, death_rate, generation_time })
    }

    /// Net growth rate per time step (per capita).
    pub fn net_growth_rate(&self) -> f64 {
        self.birth_rate - self.death_rate
    }

    /// Intrinsic rate of increase r (per unit time).
    ///
    /// Computed as ln((1 + b) / (1 + d)) where b is birth rate and d is death rate.
    pub fn intrinsic_rate(&self) -> f64 {
        if (1.0 + self.birth_rate) <= 0.0 || (1.0 + self.death_rate) <= 0.0 {
            return f64::NAN;
        }
        (1.0 + self.birth_rate).ln() - (1.0 + self.death_rate).ln()
    }

    /// Finite rate of increase lambda.
    ///
    /// The multiplicative factor by which the population multiplies each time step.
    pub fn finite_rate(&self) -> f64 {
        if 1.0 + self.death_rate == 0.0 {
            return f64::INFINITY;
        }
        (1.0 + self.birth_rate) / (1.0 + self.death_rate)
    }

    /// Doubling time in generations.
    ///
    /// Returns `infinity` if the net growth rate is non-positive.
    pub fn doubling_time(&self) -> f64 {
        let r = self.net_growth_rate();
        if r <= 0.0 {
            f64::INFINITY
        } else {
            std::f64::consts::LN_2 / r
        }
    }

    /// Number of births produced at population size `n`.
    pub fn births_at(&self, n: usize) -> f64 {
        self.birth_rate * n as f64
    }

    /// Number of deaths occurring at population size `n`.
    pub fn deaths_at(&self, n: usize) -> f64 {
        self.death_rate * n as f64
    }

    /// Project population size after `g` generations starting from `n0`.
    ///
    /// Uses discrete exponential growth: `N_g = N_0 * (1 + r)^g`.
    pub fn project(&self, n0: usize, g: usize) -> usize {
        let r = self.net_growth_rate();
        let λ = 1.0 + r;
        let projected = if λ > 0.0 {
            (n0 as f64) * λ.powi(g as i32)
        } else {
            0.0
        };
        projected.round().max(0.0) as usize
    }

    /// Compute the generation time adjusted growth rate r'.
    pub fn generation_adjusted_rate(&self) -> f64 {
        if self.generation_time == 0.0 {
            return f64::NAN;
        }
        self.net_growth_rate() / self.generation_time
    }
}

impl Default for PopulationGrowth {
    fn default() -> Self {
        Self { birth_rate: 0.0, death_rate: 0.0, generation_time: 1.0 }
    }
}
