// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::collections::HashMap;
use std::fmt;

use crate::RealityError;

/// CalibrationMethod: The method used for model calibration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CalibrationMethod {
    /// Gradient-based optimization.
    GradientDescent,
    /// Least squares fitting.
    LeastSquares,
    /// Maximum likelihood estimation.
    MaximumLikelihood,
    /// Bayesian calibration.
    Bayesian,
    /// Manual parameter adjustment.
    Manual,
    /// Evolutionary/genetic algorithm.
    Evolutionary,
}

impl CalibrationMethod {
    /// Returns the string label of this calibration method.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::GradientDescent => "gradient_descent",
            Self::LeastSquares => "least_squares",
            Self::MaximumLikelihood => "maximum_likelihood",
            Self::Bayesian => "bayesian",
            Self::Manual => "manual",
            Self::Evolutionary => "evolutionary",
        }
    }

    /// Returns whether this method requires gradient computation.
    pub fn requires_gradient(&self) -> bool {
        matches!(self, Self::GradientDescent | Self::MaximumLikelihood)
    }

    /// Returns whether this method is iterative.
    pub fn is_iterative(&self) -> bool {
        matches!(
            self,
            Self::GradientDescent | Self::MaximumLikelihood | Self::Bayesian | Self::Evolutionary
        )
    }
}

impl std::fmt::Display for CalibrationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for CalibrationMethod {
    fn default() -> Self {
        Self::LeastSquares
    }
}

/// FitMetric: A metric used to evaluate calibration quality.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FitMetric {
    /// Mean squared error.
    pub mse: f64,
    /// Mean absolute error.
    pub mae: f64,
    /// R-squared value.
    pub r_squared: f64,
    /// Log-likelihood (if applicable).
    pub log_likelihood: Option<f64>,
}

impl FitMetric {
    /// Creates a new fit metric.
    pub fn new(mse: f64, mae: f64, r_squared: f64) -> Self {
        Self {
            mse,
            mae,
            r_squared,
            log_likelihood: None,
        }
    }

    /// Creates a fit metric with log-likelihood.
    pub fn with_log_likelihood(mut self, ll: f64) -> Self {
        self.log_likelihood = Some(ll);
        self
    }

    /// Returns whether this fit is good (R-squared > threshold).
    pub fn is_good_fit(&self, r_squared_threshold: f64) -> bool {
        self.r_squared >= r_squared_threshold
    }

    /// Returns a composite score for this fit.
    pub fn composite_score(&self) -> f64 {
        let mse_component = 1.0 / (1.0 + self.mse);
        let mae_component = 1.0 / (1.0 + self.mae);
        let r2_component = self.r_squared.max(0.0);
        (mse_component + mae_component + r2_component) / 3.0
    }
}

