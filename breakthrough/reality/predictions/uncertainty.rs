// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::collections::VecDeque;
use std::fmt;

use crate::RealityError;

/// TrendDirection: The direction of a predicted trend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TrendDirection {
    /// Increasing trend.
    Increasing,
    /// Decreasing trend.
    Decreasing,
    /// Stable/no trend.
    Stable,
    /// Oscillating trend.
    Oscillating,
    /// Unknown or unpredictable trend.
    Unknown,
}

impl TrendDirection {
    /// Returns the string label of this trend direction.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Increasing => "increasing",
            Self::Decreasing => "decreasing",
            Self::Stable => "stable",
            Self::Oscillating => "oscillating",
            Self::Unknown => "unknown",
        }
    }

    /// Returns the opposite of this trend direction.
    pub fn opposite(&self) -> Self {
        match self {
            Self::Increasing => Self::Decreasing,
            Self::Decreasing => Self::Increasing,
            Self::Stable => Self::Stable,
            Self::Oscillating => Self::Oscillating,
            Self::Unknown => Self::Unknown,
        }
    }

    /// Returns whether this trend is monotonic.
    pub fn is_monotonic(&self) -> bool {
        matches!(self, Self::Increasing | Self::Decreasing)
    }
}

impl std::fmt::Display for TrendDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for TrendDirection {
    fn default() -> Self {
        Self::Unknown
    }
}

/// Uncertainty: Quantified uncertainty in a prediction or measurement.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Uncertainty {
    /// Standard deviation of the uncertainty distribution.
    pub standard_deviation: f64,
    /// The lower bound of a confidence interval.
    pub lower_bound: f64,
    /// The upper bound of a confidence interval.
    pub upper_bound: f64,
    /// Confidence level (e.g., 0.95 for 95% CI).
    pub confidence_level: f64,
}

impl Uncertainty {
    /// Creates a new uncertainty measure.
    ///
    /// # Errors
    /// Returns `RealityError::InvalidMeasurement` if parameters are invalid.
    pub fn new(
        standard_deviation: f64,
        lower_bound: f64,
        upper_bound: f64,
        confidence_level: f64,
    ) -> Result<Self, RealityError> {
        if standard_deviation.is_nan() || standard_deviation.is_infinite() || standard_deviation < 0.0 {
            return Err(RealityError::InvalidMeasurement {
                value: standard_deviation,
                reason: "standard_deviation must be non-negative and finite".to_string(),
            });
        }
        if lower_bound >= upper_bound {
            return Err(RealityError::InvalidMeasurement {
                value: 0.0,
                reason: "lower_bound must be less than upper_bound".to_string(),
            });
        }
        if confidence_level <= 0.0 || confidence_level >= 1.0 {
            return Err(RealityError::InvalidMeasurement {
                value: confidence_level,
                reason: "confidence_level must be in (0.0, 1.0)".to_string(),
            });
        }
        Ok(Self {
            standard_deviation,
            lower_bound,
            upper_bound,
            confidence_level,
        })
    }

    /// Creates a symmetric uncertainty from a standard deviation.
    pub fn symmetric(standard_deviation: f64, confidence_level: f64) -> Result<Self, RealityError> {
        Self::new(standard_deviation, -standard_deviation, standard_deviation, confidence_level)
    }

    /// Returns the width of the confidence interval.
    pub fn width(&self) -> f64 {
        self.upper_bound - self.lower_bound
    }

    /// Returns whether a value falls within the confidence interval.
    pub fn contains(&self, value: f64) -> bool {
        value >= self.lower_bound && value <= self.upper_bound
    }

    /// Returns the relative width of the confidence interval.
    pub fn relative_width(&self, center: f64) -> f64 {
        if center.abs() < f64::EPSILON {
            f64::INFINITY
        } else {
            self.width() / (2.0 * center.abs())
        }
    }

    /// Combines two uncertainties (assuming independence).
    pub fn combine(&self, other: &Uncertainty) -> Result<Self, RealityError> {
        let combined_std = (self.standard_deviation.powi(2) + other.standard_deviation.powi(2)).sqrt();
        let combined_lower = self.lower_bound.min(other.lower_bound);
        let combined_upper = self.upper_bound.max(other.upper_bound);
        let combined_confidence = self.confidence_level.min(other.confidence_level);
        Self::new(combined_std, combined_lower, combined_upper, combined_confidence)
    }

    /// Returns a score representing how tight this uncertainty is (higher = tighter).
    pub fn tightness(&self) -> f64 {
        if self.width() == 0.0 {
            return 1.0;
        }
        1.0 / (1.0 + self.width())
    }
}

