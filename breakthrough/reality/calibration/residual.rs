// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::RealityError;

/// ResidualAnalysis: Analysis of prediction residuals.
///
/// Residual analysis examines the differences between predicted and
/// observed values to identify model bias, heteroscedasticity, and
/// other statistical issues.
#[derive(Debug, Clone, PartialEq)]
pub struct ResidualAnalysis {
    /// Array of residuals (predicted - observed).
    pub residuals: Vec<f64>,
    /// Mean of residuals (bias indicator).
    pub mean: f64,
    /// Standard deviation of residuals.
    pub std_dev: f64,
    /// Minimum residual.
    pub min: f64,
    /// Maximum residual.
    pub max: f64,
    /// Whether residuals appear normally distributed (placeholder).
    pub is_normal: bool,
}

impl ResidualAnalysis {
    /// Minimum number of residuals required for meaningful analysis.
    pub const MIN_SAMPLES: usize = 2;

    /// Creates a new residual analysis.
    ///
    /// # Errors
    /// Returns `RealityError::CapacityExceeded` if residuals slice is too small.
    pub fn new(residuals: Vec<f64>) -> Result<Self, RealityError> {
        if residuals.len() < Self::MIN_SAMPLES {
            return Err(RealityError::CapacityExceeded {
                max: Self::MIN_SAMPLES,
                attempted: residuals.len(),
            });
        }
        let n = residuals.len() as f64;
        let mean = residuals.iter().sum::<f64>() / n;
        let variance = residuals.iter().map(|r| (r - mean).powi(2)).sum::<f64>() / n;
        let std_dev = variance.sqrt();
        let min = residuals.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = residuals.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        Ok(Self {
            residuals,
            mean,
            std_dev,
            min,
            max,
            is_normal: false,
        })
    }

    /// Returns the sum of squared residuals.
    pub fn ssr(&self) -> f64 {
        self.residuals.iter().map(|r| r.powi(2)).sum()
    }

    /// Returns the mean squared error.
    pub fn mse(&self) -> f64 {
        if self.residuals.is_empty() {
            return 0.0;
        }
        self.ssr() / self.residuals.len() as f64
    }

    /// Returns the root mean squared error.
    pub fn rmse(&self) -> f64 {
        self.mse().sqrt()
    }

    /// Returns the mean absolute error.
    pub fn mae(&self) -> f64 {
        if self.residuals.is_empty() {
            return 0.0;
        }
        self.residuals.iter().map(|r| r.abs()).sum::<f64>() / self.residuals.len() as f64
    }

    /// Returns whether the residuals show significant bias.
    pub fn has_bias(&self, threshold: f64) -> bool {
        self.mean.abs() > threshold
    }

    /// Returns the range of residuals.
    pub fn range(&self) -> f64 {
        self.max - self.min
    }
}

