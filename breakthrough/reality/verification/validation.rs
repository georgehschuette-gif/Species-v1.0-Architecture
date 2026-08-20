// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::collections::HashMap;
use std::fmt;

use crate::RealityError;

/// ValidationStatus: The current validation state of a model or prediction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValidationStatus {
    /// Validation is pending.
    Pending,
    /// Validation passed.
    Passed,
    /// Validation failed.
    Failed,
    /// Validation is in progress.
    InProgress,
    /// Validation was skipped.
    Skipped,
}

impl ValidationStatus {
    /// Returns the string label of this status.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::InProgress => "in_progress",
            Self::Skipped => "skipped",
        }
    }

    /// Returns whether this status indicates success.
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Passed)
    }

    /// Returns whether this status indicates failure.
    pub fn is_failure(&self) -> bool {
        matches!(self, Self::Failed)
    }

    /// Returns whether this status is terminal.
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Passed | Self::Failed | Self::Skipped)
    }
}

impl std::fmt::Display for ValidationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for ValidationStatus {
    fn default() -> Self {
        Self::Pending
    }
}

/// ValidationCriterion: A criterion used to validate predictions or models.
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationCriterion {
    /// Name of this criterion.
    pub name: String,
    /// The acceptable error threshold.
    pub threshold: f64,
    /// Metric being measured (e.g., MSE, MAE).
    pub metric: String,
    /// Weight of this criterion in overall validation.
    pub weight: f64,
}

impl ValidationCriterion {
    /// Creates a new validation criterion.
    pub fn new(
        name: impl Into<String>,
        threshold: f64,
        metric: impl Into<String>,
        weight: f64,
    ) -> Self {
        Self {
            name: name.into(),
            threshold,
            metric: metric.into(),
            weight: weight.clamp(0.0, 1.0),
        }
    }

    /// Evaluates whether a measured value passes this criterion.
    pub fn passes(&self, measured: f64) -> bool {
        measured <= self.threshold
    }
}

