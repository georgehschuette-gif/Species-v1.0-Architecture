// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Uncertainty Mapping: Representing ambiguity and confidence in perception.
//! Tracks reliability, noise, and probabilistic interpretations.

pub mod confidence;
pub mod noise_model;
pub mod probabilistic_map;

pub use confidence::ConfidenceMap;
pub use noise_model::NoiseModel;
pub use probabilistic_map::ProbabilisticMap;

pub use super::PerceptionResult;
pub use super::PerceptionError;

/// Probability distribution family for noise modeling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoiseDistribution {
    Gaussian,
    Uniform,
    Poisson,
}

impl NoiseDistribution {
    /// Sample a noise value according to this distribution.
    pub fn sample(&self, variance: f64) -> f64 {
        match self {
            Self::Gaussian => {
                let u1: f64 = rand::random();
                let u2: f64 = rand::random();
                let z = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
                z * variance.sqrt()
            }
            Self::Uniform => {
                let u = rand::random::<f64>();
                (u - 0.5) * 2.0 * variance.sqrt()
            }
            Self::Poisson => {
                let lambda = variance.max(0.0);
                let l = (-lambda).exp();
                let mut k = 0.0_f64;
                let mut p = 1.0_f64;
                loop {
                    k += 1.0;
                    p *= rand::random::<f64>();
                    if p <= l {
                        break k - 1.0;
                    }
                }
            }
        }
    }

    /// Probability density function at x.
    pub fn pdf(&self, x: f64, variance: f64) -> f64 {
        match self {
            Self::Gaussian => {
                let c = (2.0 * std::f64::consts::PI * variance).sqrt();
                let e = (-(x * x) / (2.0 * variance)).exp();
                e / c
            }
            Self::Uniform => {
                let half_range = variance.sqrt();
                if x.abs() <= half_range {
                    1.0 / (2.0 * half_range)
                } else {
                    0.0
                }
            }
            Self::Poisson => {
                let lambda = variance.max(0.0);
                let k = x.round();
                (lambda.ln() * k - lambda - (1..=k as u64).into_iter().map(|i| (i as f64).ln()).sum::<f64>()).exp()
            }
        }
    }
}

/// Uncertainty-specific error.
#[derive(Debug)]
pub enum UncertaintyError {
    InsufficientSamples,
    InvalidVariance,
    Underflow,
    InvalidProbability,
}

impl std::fmt::Display for UncertaintyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InsufficientSamples => write!(f, "insufficient samples"),
            Self::InvalidVariance => write!(f, "variance must be non-negative"),
            Self::Underflow => write!(f, "probability underflow"),
            Self::InvalidProbability => write!(f, "probability outside [0, 1]"),
        }
    }
}

impl std::error::Error for UncertaintyError {}

impl From<UncertaintyError> for PerceptionError {
    fn from(err: UncertaintyError) -> Self {
        PerceptionError::ComputationError(err.to_string())
    }
}
