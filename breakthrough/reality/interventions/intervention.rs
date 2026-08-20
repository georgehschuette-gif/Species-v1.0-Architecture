// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::collections::HashMap;
use std::fmt;

use super::{Effect, EffectDirection, InterventionType};
use crate::RealityError;

/// Intervention: An action taken to modify the environment or system state.
///
/// Interventions are the active component of the reality loop. They
/// represent the cognitive system's attempts to shape reality according
/// to its goals and predictions.
#[derive(Debug, Clone, PartialEq)]
pub struct Intervention {
    /// Unique identifier for this intervention.
    pub id: String,
    /// Human-readable description.
    pub description: String,
    /// Type of intervention.
    pub intervention_type: InterventionType,
    /// Expected effect of this intervention.
    pub expected_effect: Effect,
    /// Actual observed effect (filled after execution).
    pub actual_effect: Option<Effect>,
    /// Timestamp when the intervention was initiated.
    pub initiated_at: f64,
    /// Timestamp when the intervention was completed.
    pub completed_at: Option<f64>,
    /// Whether the intervention succeeded.
    pub succeeded: Option<bool>,
    /// Contextual metadata.
    pub metadata: HashMap<String, String>,
}

impl Intervention {
    /// Minimum valid timestamp (Unix epoch).
    pub const MIN_TIMESTAMP: f64 = 0.0;
    /// Maximum valid timestamp (year 3000 in seconds).
    pub const MAX_TIMESTAMP: f64 = 32503680000.0;

    /// Creates a new intervention.
    ///
    /// # Errors
    /// Returns `RealityError::InvalidMeasurement` if expected_effect is invalid.
    /// Returns `RealityError::OutOfRange` if timestamps are invalid.
    pub fn new(
        id: impl Into<String>,
        description: impl Into<String>,
        intervention_type: InterventionType,
        expected_effect: Effect,
        initiated_at: f64,
    ) -> Result<Self, RealityError> {
        if initiated_at < Self::MIN_TIMESTAMP || initiated_at > Self::MAX_TIMESTAMP {
            return Err(RealityError::OutOfRange {
                field: "initiated_at".to_string(),
                value: initiated_at,
                min: Self::MIN_TIMESTAMP,
                max: Self::MAX_TIMESTAMP,
            });
        }
        Ok(Self {
            id: id.into(),
            description: description.into(),
            intervention_type,
            expected_effect,
            actual_effect: None,
            initiated_at,
            completed_at: None,
            succeeded: None,
            metadata: HashMap::new(),
        })
    }

    /// Records the completion of this intervention with an observed effect.
    pub fn complete(&mut self, completed_at: f64, actual_effect: Effect) -> Result<(), RealityError> {
        if completed_at < self.initiated_at {
            return Err(RealityError::InvalidState(
                "completed_at cannot be before initiated_at".to_string(),
            ));
        }
        self.completed_at = Some(completed_at);
        self.actual_effect = Some(actual_effect);
        self.succeeded = Some(self.assess_success());
        Ok(())
    }

    /// Marks the intervention as failed.
    pub fn fail(&mut self, completed_at: f64) -> Result<(), RealityError> {
        if completed_at < self.initiated_at {
            return Err(RealityError::InvalidState(
                "completed_at cannot be before initiated_at".to_string(),
            ));
        }
        self.completed_at = Some(completed_at);
        self.succeeded = Some(false);
        Ok(())
    }

    /// Returns the duration of the intervention if completed.
    pub fn duration(&self) -> Option<f64> {
        self.completed_at.map(|end| end - self.initiated_at)
    }

    /// Returns whether the intervention has been completed.
    pub fn is_completed(&self) -> bool {
        self.completed_at.is_some()
    }

    /// Returns whether the intervention is still pending.
    pub fn is_pending(&self) -> bool {
        self.completed_at.is_none()
    }

    /// Assesses whether the intervention achieved its expected effect.
    pub fn assess_success(&self) -> bool {
        match (&self.actual_effect, &self.expected_effect) {
            (Some(actual), expected) => {
                let magnitude_match = actual.magnitude >= expected.magnitude * 0.5;
                let direction_match = actual.direction == expected.direction || expected.direction == EffectDirection::Neutral;
                magnitude_match && direction_match
            }
            _ => false,
        }
    }

    /// Adds metadata to the intervention.
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

/// ExecutionPlan: A sequence of interventions to achieve a goal.
#[derive(Debug, Clone, PartialEq)]
pub struct ExecutionPlan {
    /// Unique identifier for this plan.
    pub id: String,
    /// Ordered sequence of interventions.
    pub interventions: Vec<Intervention>,
    /// Current execution index.
    pub current_index: usize,
    /// Whether the plan has been completed.
    pub completed: bool,
}

impl ExecutionPlan {
    /// Creates a new execution plan.
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            interventions: Vec::new(),
            current_index: 0,
            completed: false,
        }
    }

    /// Adds an intervention to the plan.
    pub fn add_intervention(&mut self, intervention: Intervention) {
        self.interventions.push(intervention);
    }

    /// Returns the current intervention if any.
    pub fn current(&self) -> Option<&Intervention> {
        self.interventions.get(self.current_index)
    }

    /// Advances to the next intervention.
    pub fn advance(&mut self) -> Option<&Intervention> {
        if self.current_index + 1 < self.interventions.len() {
            self.current_index += 1;
            self.current()
        } else {
            self.completed = true;
            None
        }
    }

    /// Returns the number of remaining interventions.
    pub fn remaining(&self) -> usize {
        self.interventions.len().saturating_sub(self.current_index)
    }

    /// Returns the number of completed interventions.
    pub fn completed_count(&self) -> usize {
        self.current_index
    }
}
