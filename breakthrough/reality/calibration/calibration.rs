// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::collections::HashMap;
use std::fmt;

use super::parameter::{CalibrationMethod, FitMetric};
use super::fitting::ParameterAdjustment;
use crate::RealityError;

/// Calibration: The process of adjusting model parameters to match observations.
///
/// Calibration iteratively refines model parameters to minimize the
/// discrepancy between predictions and observations.
#[derive(Debug, Clone, PartialEq)]
pub struct Calibration {
    /// Unique identifier for this calibration.
    pub id: String,
    /// The model being calibrated.
    pub model_id: String,
    /// Calibration method used.
    pub method: CalibrationMethod,
    /// Current parameter values.
    pub parameters: HashMap<String, f64>,
    /// History of parameter adjustments.
    pub adjustment_history: Vec<ParameterAdjustment>,
    /// Current fit metric.
    pub fit_metric: FitMetric,
    /// Whether calibration has converged.
    pub converged: bool,
    /// Number of iterations performed.
    pub iterations: usize,
    /// Maximum allowed iterations.
    pub max_iterations: usize,
    /// Convergence threshold.
    pub convergence_threshold: f64,
}

impl Calibration {
    /// Default maximum iterations for calibration.
    pub const DEFAULT_MAX_ITERATIONS: usize = 1000;
    /// Default convergence threshold.
    pub const DEFAULT_CONVERGENCE_THRESHOLD: f64 = 1e-6;

    /// Creates a new calibration session.
    pub fn new(
        id: impl Into<String>,
        model_id: impl Into<String>,
        method: CalibrationMethod,
        initial_parameters: HashMap<String, f64>,
    ) -> Self {
        Self {
            id: id.into(),
            model_id: model_id.into(),
            method,
            parameters: initial_parameters,
            adjustment_history: Vec::new(),
            fit_metric: FitMetric::new(f64::INFINITY, f64::INFINITY, 0.0),
            converged: false,
            iterations: 0,
            max_iterations: Self::DEFAULT_MAX_ITERATIONS,
            convergence_threshold: Self::DEFAULT_CONVERGENCE_THRESHOLD,
        }
    }

    /// Sets the convergence threshold.
    pub fn with_convergence_threshold(mut self, threshold: f64) -> Self {
        self.convergence_threshold = threshold.max(0.0);
        self
    }

    /// Sets the maximum iterations.
    pub fn with_max_iterations(mut self, max: usize) -> Self {
        self.max_iterations = max.max(1);
        self
    }

    /// Records a parameter adjustment.
    pub fn record_adjustment(&mut self, adjustment: ParameterAdjustment) {
        self.adjustment_history.push(adjustment);
    }

    /// Updates a parameter value.
    pub fn update_parameter(&mut self, name: impl Into<String>, value: f64) {
        let name = name.into();
        let original = self.parameters.get(&name).copied().unwrap_or(0.0);
        self.parameters.insert(name.clone(), value);
        self.adjustment_history.push(ParameterAdjustment::new(
            name,
            original,
            value,
            "calibration update",
            0.0,
        ));
    }

    /// Checks whether calibration has converged.
    pub fn check_convergence(&mut self, current_error: f64) -> bool {
        if current_error <= self.convergence_threshold {
            self.converged = true;
            return true;
        }
        if self.iterations >= self.max_iterations {
            self.converged = true;
            return true;
        }
        false
    }

    /// Returns the number of adjustments made.
    pub fn adjustment_count(&self) -> usize {
        self.adjustment_history.len()
    }
}

