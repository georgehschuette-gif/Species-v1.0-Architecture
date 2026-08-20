// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::collections::VecDeque;
use std::fmt;

use crate::RealityError;

/// PredictionModel: A model that generates predictions.
///
/// Models encode the cognitive system's understanding of how the
/// world works. They range from simple heuristics to complex
/// learned functions.
#[derive(Debug, Clone, PartialEq)]
pub struct PredictionModel {
    /// Unique identifier for this model.
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// Model parameters (key-value pairs).
    pub parameters: HashMap<String, f64>,
    /// Recent prediction errors for model evaluation.
    pub recent_errors: VecDeque<f64>,
    /// Maximum number of recent errors to retain.
    pub max_recent_errors: usize,
    /// Current model accuracy metric (mean squared error).
    pub accuracy: f64,
    /// Whether the model is currently calibrated.
    pub is_calibrated: bool,
}

impl PredictionModel {
    /// Default maximum number of recent errors to retain.
    pub const DEFAULT_MAX_ERRORS: usize = 100;

    /// Creates a new prediction model.
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            parameters: HashMap::new(),
            recent_errors: VecDeque::with_capacity(Self::DEFAULT_MAX_ERRORS),
            max_recent_errors: Self::DEFAULT_MAX_ERRORS,
            accuracy: 1.0,
            is_calibrated: false,
        }
    }

    /// Sets a model parameter.
    pub fn set_parameter(&mut self, key: impl Into<String>, value: f64) {
        self.parameters.insert(key.into(), value);
        self.is_calibrated = false;
    }

    /// Gets a model parameter.
    pub fn get_parameter(&self, key: &str) -> Option<f64> {
        self.parameters.get(key).copied()
    }

    /// Records a prediction error.
    pub fn record_error(&mut self, error: f64) {
        if self.recent_errors.len() >= self.max_recent_errors {
            self.recent_errors.pop_front();
        }
        self.recent_errors.push_back(error);
        self.update_accuracy();
    }

    /// Updates the model's accuracy metric based on recent errors.
    pub fn update_accuracy(&mut self) {
        if self.recent_errors.is_empty() {
            self.accuracy = 1.0;
            return;
        }
        let sum_sq: f64 = self.recent_errors.iter().map(|e| e * e).sum();
        let mse = sum_sq / self.recent_errors.len() as f64;
        self.accuracy = 1.0 / (1.0 + mse);
    }

    /// Returns the mean squared error of recent predictions.
    pub fn mean_squared_error(&self) -> f64 {
        if self.recent_errors.is_empty() {
            return 0.0;
        }
        let sum_sq: f64 = self.recent_errors.iter().map(|e| e * e).sum();
        sum_sq / self.recent_errors.len() as f64
    }

    /// Returns the mean absolute error of recent predictions.
    pub fn mean_absolute_error(&self) -> f64 {
        if self.recent_errors.is_empty() {
            return 0.0;
        }
        let sum_abs: f64 = self.recent_errors.iter().map(|e| e.abs()).sum();
        sum_abs / self.recent_errors.len() as f64
    }

    /// Marks the model as calibrated.
    pub fn mark_calibrated(&mut self) {
        self.is_calibrated = true;
    }

    /// Resets the model's error history.
    pub fn reset_errors(&mut self) {
        self.recent_errors.clear();
        self.accuracy = 1.0;
    }

    /// Returns the number of recorded errors.
    pub fn error_count(&self) -> usize {
        self.recent_errors.len()
    }
}

use std::collections::HashMap;

