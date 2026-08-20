// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::collections::HashMap;
use std::fmt;

use super::model::PredictionModel;
use super::prediction::Prediction;
use crate::RealityError;

/// Scenario: A possible future state described by a set of predictions.
///
/// Scenarios bundle multiple predictions into coherent narratives
/// of possible futures, enabling planning under uncertainty.
#[derive(Debug, Clone, PartialEq)]
pub struct Scenario {
    /// Unique identifier for this scenario.
    pub id: String,
    /// Human-readable description.
    pub description: String,
    /// Predictions comprising this scenario.
    pub predictions: HashMap<String, Prediction>,
    /// Prior probability of this scenario occurring.
    pub probability: f64,
    /// Model associated with this scenario.
    pub model: PredictionModel,
    /// Whether this scenario has been evaluated.
    pub evaluated: bool,
}

impl Scenario {
    /// Minimum valid probability.
    pub const MIN_PROBABILITY: f64 = 0.0;
    /// Maximum valid probability.
    pub const MAX_PROBABILITY: f64 = 1.0;

    /// Creates a new scenario.
    ///
    /// # Errors
    /// Returns `RealityError::OutOfRange` if probability is outside [0.0, 1.0].
    pub fn new(
        id: impl Into<String>,
        description: impl Into<String>,
        model: PredictionModel,
        probability: f64,
    ) -> Result<Self, RealityError> {
        if probability < Self::MIN_PROBABILITY || probability > Self::MAX_PROBABILITY {
            return Err(RealityError::OutOfRange {
                field: "probability".to_string(),
                value: probability,
                min: Self::MIN_PROBABILITY,
                max: Self::MAX_PROBABILITY,
            });
        }
        Ok(Self {
            id: id.into(),
            description: description.into(),
            predictions: HashMap::new(),
            probability,
            model,
            evaluated: false,
        })
    }

    /// Adds a prediction to this scenario.
    pub fn add_prediction(&mut self, key: impl Into<String>, prediction: Prediction) {
        self.predictions.insert(key.into(), prediction);
    }

    /// Returns a prediction by key.
    pub fn get_prediction(&self, key: &str) -> Option<&Prediction> {
        self.predictions.get(key)
    }

    /// Returns the number of predictions in this scenario.
    pub fn prediction_count(&self) -> usize {
        self.predictions.len()
    }

    /// Normalizes probabilities across scenarios (typically called on a set of scenarios).
    pub fn normalize_probability(&mut self, total_probability: f64) -> Result<(), RealityError> {
        if total_probability <= 0.0 {
            return Err(RealityError::InvalidMeasurement {
                value: total_probability,
                reason: "total_probability must be positive".to_string(),
            });
        }
        self.probability /= total_probability;
        Ok(())
    }

    /// Returns the expected value of a prediction key, weighted by probability.
    pub fn expected_value(&self, key: &str) -> Option<f64> {
        self.predictions.get(key).map(|p| p.value * self.probability)
    }

    /// Marks the scenario as evaluated.
    pub fn mark_evaluated(&mut self) {
        self.evaluated = true;
    }
}
