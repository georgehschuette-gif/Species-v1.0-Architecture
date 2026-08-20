// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Reality: The interface between cognitive models and the external world.
//!
//! This module bridges internal cognitive representations with the
//! observed environment through a complete perception-action loop:
//!
//! 1. **Observations** — Raw and processed sensory data from the environment.
//! 2. **Measurements** — Quantified attributes with precision and accuracy.
//! 3. **Predictions** — Forecasts of future states derived from models.
//! 4. **Interventions** — Actions taken to modify the environment.
//! 5. **Consequences** — Outcomes resulting from interventions.
//! 6. **Verification** — Validation of predictions against observations.
//! 7. **Calibration** — Adjustment of model parameters to reduce error.
//! 8. **Feedback** — Learning signals that close the perception-action loop.
//!
//! # Reality Loop
//!
//! The canonical cycle:
//!
//! ```text
//! observe → measure → predict → intervene → observe consequence
//!    ↑                                              |
//!    └───────────── verify + calibrate + feedback ───┘
//! ```
//!
//! Each phase feeds into the next, with verification, calibration,
//! and feedback closing the loop to continuously improve model fidelity.

use std::fmt;

/// Errors that can occur during reality operations.
#[derive(Debug, Clone, PartialEq)]
pub enum RealityError {
    /// An input value is outside the valid range [`min`, `max`].
    OutOfRange {
        field: String,
        value: f64,
        min: f64,
        max: f64,
    },
    /// Required input data is missing or empty.
    MissingInput(String),
    /// An operation cannot be performed due to an invalid internal state.
    InvalidState(String),
    /// A threshold was not met for the requested operation.
    ThresholdNotMet {
        threshold: f64,
        actual: f64,
    },
    /// A capacity limit has been exceeded.
    CapacityExceeded {
        max: usize,
        attempted: usize,
    },
    /// A measurement or rate value is invalid.
    InvalidMeasurement {
        value: f64,
        reason: String,
    },
    /// Two observations or measurements are incompatible for the requested operation.
    IncompatibleObservations {
        reason: String,
    },
    /// An internal consistency check failed during verification.
    ConsistencyError(String),
    /// Calibration failed to converge within the allowed iterations.
    CalibrationFailed {
        iterations: usize,
        residual: f64,
    },
}

impl fmt::Display for RealityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RealityError::OutOfRange { field, value, min, max } => write!(
                f,
                "field '{}' value {} is out of range [{}, {}]",
                field, value, min, max
            ),
            RealityError::MissingInput(detail) => {
                write!(f, "missing required input: {}", detail)
            }
            RealityError::InvalidState(detail) => {
                write!(f, "invalid state: {}", detail)
            }
            RealityError::ThresholdNotMet { threshold, actual } => write!(
                f,
                "threshold {} not met (actual: {})",
                threshold, actual
            ),
            RealityError::CapacityExceeded { max, attempted } => write!(
                f,
                "capacity exceeded: max {} but attempted {}",
                max, attempted
            ),
            RealityError::InvalidMeasurement { value, reason } => {
                write!(f, "invalid measurement {}: {}", value, reason)
            }
            RealityError::IncompatibleObservations { reason } => {
                write!(f, "incompatible observations: {}", reason)
            }
            RealityError::ConsistencyError(detail) => {
                write!(f, "consistency error: {}", detail)
            }
            RealityError::CalibrationFailed { iterations, residual } => write!(
                f,
                "calibration failed after {} iterations with residual {}",
                iterations, residual
            ),
        }
    }
}

impl std::error::Error for RealityError {}

pub mod observations;
pub mod measurements;
pub mod predictions;
pub mod interventions;
pub mod consequences;
pub mod verification;
pub mod calibration;
pub mod feedback;

pub use observations::{
    Observation, ObservationSource, ObservationConfidence, Scene, ObjectDetection,
    SensoryModality,
};
pub use measurements::{
    Measurement, MeasurementType, Precision, Accuracy, Unit, Scale,
};
pub use predictions::{
    Prediction, PredictionModel, Forecast, Uncertainty, Scenario, TrendDirection,
};
pub use interventions::{
    Intervention, InterventionType, Action, Effect, ExecutionPlan,
};
pub use consequences::{
    Consequence, CausalChain, EffectMagnitude, Outcome,
};
pub use verification::{
    Verification, TestResult, ValidationStatus, FalsificationCriterion,
};
pub use calibration::{
    Calibration, CalibrationMethod, FitMetric, ResidualAnalysis, ParameterAdjustment,
};
pub use feedback::{
    Feedback, LearningSignal, AdaptationRate, ErrorCorrection, LearningMode,
};
