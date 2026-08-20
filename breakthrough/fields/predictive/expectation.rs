// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// PredictiveDistribution: The probability distribution family used for prediction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PredictiveDistribution {
    Gaussian,
    Uniform,
    Beta,
    Multimodal,
}

impl PredictiveDistribution {
    pub fn as_str(&self) -> &'static str {
        match self {
            PredictiveDistribution::Gaussian => "gaussian",
            PredictiveDistribution::Uniform => "uniform",
            PredictiveDistribution::Beta => "beta",
            PredictiveDistribution::Multimodal => "multimodal",
        }
    }

    pub fn is_bounded(&self) -> bool {
        matches!(self, PredictiveDistribution::Beta | PredictiveDistribution::Uniform)
    }

    pub fn is_unimodal(&self) -> bool {
        !matches!(self, PredictiveDistribution::Multimodal)
    }
}

/// PredictiveExpectation: Anticipated future cognitive states.
#[derive(Debug, Clone, PartialEq)]
pub struct PredictiveExpectation {
    pub target_time: f64,
    pub expected_value: f64,
    pub variance: f64,
    pub distribution: PredictiveDistribution,
}

impl PredictiveExpectation {
    pub fn new(target_time: f64, expected_value: f64, variance: f64, distribution: PredictiveDistribution) -> Result<Self, PredictiveError> {
        if !target_time.is_finite() || target_time <= 0.0 {
            return Err(PredictiveError::InvalidHorizon { horizon: target_time });
        }
        if !variance.is_finite() || variance < 0.0 {
            return Err(PredictiveError::InvalidVariance { variance });
        }
        Ok(Self { target_time, expected_value, variance, distribution })
    }

    pub fn confidence_interval(&self, z_score: f64) -> (f64, f64) {
        let margin = z_score * self.variance.sqrt();
        (self.expected_value - margin, self.expected_value + margin)
    }

    pub fn is_confident(&self) -> bool {
        self.variance < 0.25
    }

    pub fn distribution_name(&self) -> &'static str {
        self.distribution.as_str()
    }

    pub fn standard_deviation(&self) -> f64 {
        self.variance.sqrt()
    }

    pub fn confidence_width(&self) -> f64 {
        2.0 * self.standard_deviation()
    }
}
