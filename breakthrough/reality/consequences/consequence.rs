// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::collections::HashMap;
use std::fmt;

use super::chain::Outcome;
use crate::RealityError;

/// Consequence: The outcome resulting from an intervention.
///
/// Consequences are the observable results of interventions. They
/// feed back into the reality loop to inform future predictions,
/// calibrations, and interventions.
#[derive(Debug, Clone, PartialEq)]
pub struct Consequence {
    /// Unique identifier for this consequence.
    pub id: String,
    /// The intervention that caused this consequence.
    pub intervention_id: String,
    /// Observed outcome.
    pub outcome: Outcome,
    /// Time delay between intervention and consequence.
    pub delay: f64,
    /// Whether this consequence was anticipated.
    pub anticipated: bool,
    /// Contextual metadata.
    pub metadata: HashMap<String, String>,
}

impl Consequence {
    /// Minimum valid delay.
    pub const MIN_DELAY: f64 = 0.0;

    /// Creates a new consequence.
    pub fn new(
        id: impl Into<String>,
        intervention_id: impl Into<String>,
        outcome: Outcome,
        delay: f64,
        anticipated: bool,
    ) -> Result<Self, RealityError> {
        if delay < Self::MIN_DELAY {
            return Err(RealityError::InvalidMeasurement {
                value: delay,
                reason: "delay cannot be negative".to_string(),
            });
        }
        Ok(Self {
            id: id.into(),
            intervention_id: intervention_id.into(),
            outcome,
            delay,
            anticipated,
            metadata: HashMap::new(),
        })
    }

    /// Adds metadata to the consequence.
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Returns whether the consequence matched expectations.
    pub fn matches_expectation(&self) -> bool {
        self.anticipated == self.outcome.expected
    }

    /// Returns the severity of the consequence.
    pub fn severity(&self) -> f64 {
        self.outcome.magnitude.0
    }
}

