// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Probabilistic Field: Uncertainty and belief distributions.
//! Encodes Bayesian beliefs, entropy measures, and mixture weights
//! for reasoning under uncertainty within the cognitive ecosystem.
//!
//! The probabilistic field module provides tools for representing
//! uncertain knowledge as probability distributions, measuring the
//! information content of beliefs, and combining multiple belief sources.

pub mod belief;
pub mod entropy;
pub mod mixture;

pub use belief::BayesianBelief;
pub use entropy::ProbabilisticEntropy;
pub use mixture::BeliefMixture;

use std::fmt;

/// ProbabilisticField: A container for uncertain beliefs and distributions.
#[derive(Debug, Clone, PartialEq)]
pub struct ProbabilisticField {
    pub name: String,
    pub beliefs: Vec<BayesianBelief>,
    pub entropy: f64,
}

impl ProbabilisticField {
    pub fn new(name: String) -> Self {
        Self { name, beliefs: Vec::new(), entropy: 0.0 }
    }

    pub fn add_belief(&mut self, belief: BayesianBelief) {
        self.beliefs.push(belief);
    }

    pub fn belief_count(&self) -> usize {
        self.beliefs.len()
    }

    pub fn set_entropy(&mut self, entropy: f64) {
        if entropy.is_finite() && entropy >= 0.0 {
            self.entropy = entropy;
        }
    }

    pub fn avg_posterior(&self) -> Option<f64> {
        if self.beliefs.is_empty() {
            return None;
        }
        let sum: f64 = self.beliefs.iter().map(|b| b.posterior).sum();
        Some(sum / self.beliefs.len() as f64)
    }

    pub fn is_certain(&self) -> bool {
        self.avg_posterior().map_or(false, |p| p > 0.9 || p < 0.1)
    }

    pub fn max_posterior(&self) -> Option<f64> {
        Some(self.beliefs.iter().map(|b| b.posterior).fold(f64::NEG_INFINITY, f64::max).max(0.0))
    }
}

impl Default for ProbabilisticField {
    fn default() -> Self {
        Self::new(String::from("unnamed"))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProbabilisticError {
    InvalidProbability { probability: f64 },
    InvalidDistribution,
    InvalidEntropy { entropy: f64 },
    InsufficientData,
    ComputationError(String),
}

impl fmt::Display for ProbabilisticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProbabilisticError::InvalidProbability { probability } => write!(f, "invalid probability: {}", probability),
            ProbabilisticError::InvalidDistribution => write!(f, "invalid probability distribution"),
            ProbabilisticError::InvalidEntropy { entropy } => write!(f, "invalid entropy: {}", entropy),
            ProbabilisticError::InsufficientData => write!(f, "insufficient data for operation"),
            ProbabilisticError::ComputationError(msg) => write!(f, "computation error: {}", msg),
        }
    }
}

impl std::error::Error for ProbabilisticError {}

pub type ProbabilisticResult<T> = Result<T, ProbabilisticError>;
