// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Predictive Field: Forward-looking cognitive distributions.
//! Encodes expected outcomes, confidence intervals, and scenario likelihoods
//! for planning and anticipation within the cognitive ecosystem.
//!
//! The predictive field module provides tools for modeling anticipated future
//! states, quantifying uncertainty through confidence distributions, and
//! evaluating the plausibility of alternative scenarios.

pub mod expectation;
pub mod confidence;
pub mod scenario;

pub use expectation::{PredictiveExpectation, PredictiveDistribution};
pub use confidence::PredictiveConfidence;
pub use scenario::{ScenarioLikelihood, Scenario};

use std::fmt;

/// PredictiveField: A forward-looking container for anticipated cognitive states.
#[derive(Debug, Clone, PartialEq)]
pub struct PredictiveField {
    pub name: String,
    pub expectations: Vec<PredictiveExpectation>,
    pub confidence: f64,
}

impl PredictiveField {
    pub fn new(name: String) -> Self {
        Self { name, expectations: Vec::new(), confidence: 0.5 }
    }

    pub fn add_expectation(&mut self, expectation: PredictiveExpectation) {
        self.expectations.push(expectation);
    }

    pub fn expectation_count(&self) -> usize {
        self.expectations.len()
    }

    pub fn set_confidence(&mut self, confidence: f64) -> Result<(), PredictiveError> {
        if !(0.0..=1.0).contains(&confidence) {
            return Err(PredictiveError::InvalidConfidence { confidence });
        }
        self.confidence = confidence;
        Ok(())
    }

    pub fn avg_expected_value(&self) -> Option<f64> {
        if self.expectations.is_empty() {
            return None;
        }
        let sum: f64 = self.expectations.iter().map(|e| e.expected_value).sum();
        Some(sum / self.expectations.len() as f64)
    }

    pub fn mean_confidence_width(&self) -> f64 {
        if self.expectations.is_empty() {
            return 0.0;
        }
        let total: f64 = self.expectations.iter().map(|e| e.confidence_width()).sum();
        total / self.expectations.len() as f64
    }
}

impl Default for PredictiveField {
    fn default() -> Self {
        Self::new(String::from("unnamed"))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PredictiveError {
    InvalidHorizon { horizon: f64 },
    InvalidVariance { variance: f64 },
    InvalidConfidence { confidence: f64 },
    InvalidProbability { probability: f64, reason: String },
    InvalidPrediction { prediction_id: u64 },
    InsufficientData,
    ComputationError(String),
}

impl fmt::Display for PredictiveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PredictiveError::InvalidHorizon { horizon } => write!(f, "invalid horizon: {}", horizon),
            PredictiveError::InvalidVariance { variance } => write!(f, "invalid variance: {}", variance),
            PredictiveError::InvalidConfidence { confidence } => write!(f, "invalid confidence: {}", confidence),
            PredictiveError::InvalidProbability { probability, reason } => write!(f, "invalid probability {}: {}", probability, reason),
            PredictiveError::InvalidPrediction { prediction_id } => write!(f, "invalid prediction id: {}", prediction_id),
            PredictiveError::InsufficientData => write!(f, "insufficient data for operation"),
            PredictiveError::ComputationError(msg) => write!(f, "computation error: {}", msg),
        }
    }
}

impl std::error::Error for PredictiveError {}

pub type PredictiveResult<T> = Result<T, PredictiveError>;
