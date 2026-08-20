// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// NaturalSelection: The process by which advantageous traits become more common.
///
/// Models selection pressure applied to a population over discrete generations.
/// Fitness is evaluated according to the configured `FitnessFunction`.
pub struct NaturalSelection {
    /// Strength of selection pressure, in [0.0, 1.0].
    pub selection_pressure: f64,
    /// The fitness evaluation function.
    pub fitness_function: FitnessFunction,
    /// Number of generations per selection event.
    pub generation_span: usize,
}

impl NaturalSelection {
    /// Minimum valid selection pressure.
    pub const MIN_PRESSURE: f64 = 0.0;
    /// Maximum valid selection pressure.
    pub const MAX_PRESSURE: f64 = 1.0;
    /// Minimum generation span.
    pub const MIN_GENERATIONS: usize = 1;

    /// Creates a new NaturalSelection instance.
    ///
    /// # Errors
    /// Returns `AdaptationError::InvalidSelectionPressure` if pressure is outside [0.0, 1.0].
    /// Returns `AdaptationError::InvalidFitnessParameter` if generation_span is zero.
    pub fn new(
        selection_pressure: f64,
        fitness_function: FitnessFunction,
        generation_span: usize,
    ) -> AdaptationResult<Self> {
        if !(Self::MIN_PRESSURE..=Self::MAX_PRESSURE).contains(&selection_pressure) {
            return Err(AdaptationError::InvalidSelectionPressure {
                pressure: selection_pressure,
            });
        }
        if generation_span < Self::MIN_GENERATIONS {
            return Err(AdaptationError::InvalidFitnessParameter {
                detail: format!("generation_span must be >= {}", Self::MIN_GENERATIONS),
            });
        }
        Ok(Self {
            selection_pressure,
            fitness_function,
            generation_span,
        })
    }

    /// Evaluates the fitness of a trait value according to the configured function.
    pub fn evaluate_fitness(&self, trait_value: f64) -> AdaptationResult<f64> {
        if trait_value.is_nan() || trait_value.is_infinite() {
            return Err(AdaptationError::InvalidFitnessParameter {
                detail: "trait value must be finite".to_string(),
            });
        }
        let fitness = match self.fitness_function {
            FitnessFunction::Linear => trait_value.clamp(0.0, 1.0),
            FitnessFunction::Exponential => {
                let t = trait_value.clamp(0.0, 1.0);
                t.powi(2)
            }
            FitnessFunction::Threshold(threshold) => {
                if trait_value >= threshold { 1.0 } else { 0.0 }
            }
            FitnessFunction::Gaussian { mean, std_dev } => {
                if std_dev <= 0.0 {
                    return Err(AdaptationError::InvalidFitnessParameter {
                        detail: "std_dev must be positive for Gaussian fitness".to_string(),
                    });
                }
                let diff = trait_value - mean;
                (-diff * diff / (2.0 * std_dev * std_dev)).exp()
            }
        };
        Ok(fitness.clamp(0.0, 1.0))
    }

    /// Computes the selection differential for a population.
    ///
    /// Returns the difference between the mean trait value and the mean fitness-weighted trait value.
    pub fn selection_differential(&self, trait_values: &[f64]) -> AdaptationResult<f64> {
        if trait_values.is_empty() {
            return Ok(0.0);
        }
        let mean_trait: f64 = trait_values.iter().sum::<f64>() / trait_values.len() as f64;
        let mut fitness_weighted_sum = 0.0;
        for &tv in trait_values {
            let w = self.evaluate_fitness(tv)?;
            fitness_weighted_sum += w * tv;
        }
        let mean_fitness_trait = fitness_weighted_sum / trait_values.len() as f64;
        Ok(self.selection_pressure * (mean_fitness_trait - mean_trait))
    }

    /// Applies selection pressure to a population, returning the new mean trait value.
    pub fn apply_selection(&self, trait_values: &[f64]) -> AdaptationResult<f64> {
        if trait_values.is_empty() {
            return Ok(0.0);
        }
        let differential = self.selection_differential(trait_values)?;
        let current_mean: f64 = trait_values.iter().sum::<f64>() / trait_values.len() as f64;
        Ok(current_mean + differential)
    }

    /// Returns the expected response to selection per generation.
    pub fn response_to_selection(&self, trait_values: &[f64]) -> AdaptationResult<f64> {
        if trait_values.is_empty() {
            return Ok(0.0);
        }
        let differential = self.selection_differential(trait_values)?;
        let variance: f64 = trait_values
            .iter()
            .map(|tv| {
                let mean = trait_values.iter().sum::<f64>() / trait_values.len() as f64;
                (tv - mean).powi(2)
            })
            .sum::<f64>()
            / trait_values.len() as f64;
        let heritability = 0.5;
        Ok(self.selection_pressure * heritability * differential)
    }
}

/// FitnessFunction: The mathematical function used to evaluate trait fitness.
#[derive(Debug, Clone, Copy)]
pub enum FitnessFunction {
    /// Linear fitness: trait value maps directly to fitness.
    Linear,
    /// Exponential fitness: trait value is squared.
    Exponential,
    /// Threshold fitness: fitness is 1.0 above threshold, 0.0 below.
    Threshold(f64),
    /// Gaussian fitness: bell curve centered at mean with standard deviation.
    Gaussian { mean: f64, std_dev: f64 },
}

impl FitnessFunction {
    /// Returns the minimum valid threshold for Threshold fitness.
    pub const MIN_THRESHOLD: f64 = 0.0;
    /// Returns the maximum valid threshold for Threshold fitness.
    pub const MAX_THRESHOLD: f64 = 1.0;

    /// Validates the fitness function parameters.
    pub fn validate(&self) -> AdaptationResult<()> {
        match self {
            FitnessFunction::Threshold(t) => {
                if !(Self::MIN_THRESHOLD..=Self::MAX_THRESHOLD).contains(t) {
                    return Err(AdaptationError::InvalidFitnessParameter {
                        detail: format!("threshold {} out of range", t),
                    });
                }
            }
            FitnessFunction::Gaussian { mean, std_dev } => {
                if !mean.is_finite() {
                    return Err(AdaptationError::InvalidFitnessParameter {
                        detail: "Gaussian mean must be finite".to_string(),
                    });
                }
                if *std_dev <= 0.0 {
                    return Err(AdaptationError::InvalidFitnessParameter {
                        detail: "Gaussian std_dev must be positive".to_string(),
                    });
                }
            }
            _ => {}
        }
        Ok(())
    }
}

/// Error type for selection operation failures.
#[derive(Debug, Clone, PartialEq)]
pub enum SelectionError {
    InvalidSelectionPressure { pressure: f64 },
    InvalidFitnessParameter { detail: String },
    ComputationError(String),
}

impl std::fmt::Display for SelectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SelectionError::InvalidSelectionPressure { pressure } => write!(f, "Invalid selection pressure: {}", pressure),
            SelectionError::InvalidFitnessParameter { detail } => write!(f, "Invalid fitness parameter: {}", detail),
            SelectionError::ComputationError(msg) => write!(f, "Computation error: {}", msg),
        }
    }
}

impl std::error::Error for SelectionError {}
