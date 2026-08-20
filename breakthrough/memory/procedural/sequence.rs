// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::collections::HashMap;
use std::fmt;

use crate::MemoryError;

use super::*;

/// ActionSequence: An ordered chain of actions with transition probabilities.
///
/// Procedural sequences model the temporal structure of skilled behavior.
/// Each action follows from the previous one with some probability, and the
/// overall sequence has an expected duration and error recovery strategies.
#[derive(Debug, Clone, PartialEq)]
pub struct ActionSequence {
    /// Ordered list of action identifiers.
    pub actions: Vec<String>,
    /// Transition probabilities: (from, to) -> probability.
    pub transition_matrix: HashMap<String, HashMap<String, f64>>,
    /// Expected total duration of the sequence in abstract units.
    pub expected_duration: f64,
    /// Maximum number of error recovery attempts allowed.
    pub max_retries: usize,
    /// Count of times the sequence has been executed.
    pub execution_count: usize,
}

impl ActionSequence {
    /// Minimum number of actions in a valid sequence.
    pub const MIN_ACTIONS: usize = 1;
    /// Maximum number of actions in a valid sequence.
    pub const MAX_ACTIONS: usize = 10_000;

    /// Creates a new ActionSequence from a slice of action names.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::MissingInput`] if any action name is empty.
    /// Returns [`MemoryError::DimensionMismatch`] if the number of actions
    /// is outside [`MIN_ACTIONS`..`MAX_ACTIONS`].
    pub fn new(actions: &[impl AsRef<str>]) -> Result<Self, MemoryError> {
        let count = actions.len();
        if count == 0 || count > Self::MAX_ACTIONS {
            return Err(MemoryError::DimensionMismatch {
                expected: Self::MIN_ACTIONS,
                actual: count,
            });
        }
        let act_vec: Vec<String> = actions.iter().map(|a| a.as_ref().to_string()).collect();
        for a in &act_vec {
            if a.is_empty() {
                return Err(MemoryError::MissingInput(
                    "action names must not be empty".into(),
                ));
            }
        }
        let mut matrix = HashMap::new();
        for i in 0..act_vec.len() {
            let mut inner = HashMap::new();
            if i + 1 < act_vec.len() {
                inner.insert(act_vec[i + 1].clone(), 1.0);
            }
            matrix.insert(act_vec[i].clone(), inner);
        }
        Ok(Self {
            actions: act_vec,
            transition_matrix: matrix,
            expected_duration: count as f64,
            max_retries: 3,
            execution_count: 0,
        })
    }

    /// Adds an action to the end of the sequence.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::CapacityExceeded`] if the sequence is at capacity.
    /// Returns [`MemoryError::MissingInput`] if the action name is empty.
    pub fn add_action(&mut self, action: impl AsRef<str>) -> Result<(), MemoryError> {
        let name = action.as_ref().to_string();
        if name.is_empty() {
            return Err(MemoryError::MissingInput(
                "action name must not be empty".into(),
            ));
        }
        if self.actions.len() >= Self::MAX_ACTIONS {
            return Err(MemoryError::CapacityExceeded {
                max: Self::MAX_ACTIONS,
                attempted: self.actions.len() + 1,
            });
        }
        let prev = self.actions.last().cloned();
        self.actions.push(name.clone());
        self.expected_duration += 1.0;
        if let Some(prev_name) = prev {
            self.transition_matrix
                .entry(prev_name)
                .or_default()
                .insert(name.clone(), 1.0);
        }
        Ok(())
    }

    /// Predicts the next action after the given current action.
    ///
    /// Returns the most probable successor, or `None` if no transitions exist.
    pub fn predict_next(&self, current: impl AsRef<str>) -> Option<&String> {
        let key = current.as_ref();
        let inner = self.transition_matrix.get(key)?;
        let mut best: Option<(&String, f64)> = None;
        for (next, prob) in inner {
            if best.map_or(true, |(_, p)| *prob > p) {
                best = Some((next, *prob));
            }
        }
        best.map(|(s, _)| s)
    }

    /// Returns the transition probability between two actions.
    pub fn transition_probability(&self, from: impl AsRef<str>, to: impl AsRef<str>) -> f64 {
        self.transition_matrix
            .get(from.as_ref())
            .and_then(|inner| inner.get(to.as_ref()))
            .copied()
            .unwrap_or(0.0)
    }

    /// Sets a transition probability between two actions.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::OutOfRange`] if probability is outside [0.0, 1.0].
    pub fn set_transition(
        &mut self,
        from: impl AsRef<str>,
        to: impl AsRef<str>,
        probability: f64,
    ) -> Result<(), MemoryError> {
        if !(0.0..=1.0).contains(&probability) {
            return Err(MemoryError::OutOfRange {
                field: "probability".into(),
                value: probability,
                min: 0.0,
                max: 1.0,
            });
        }
        let from_key = from.as_ref().to_string();
        let to_key = to.as_ref().to_string();
        if !self.actions.contains(&from_key) || !self.actions.contains(&to_key) {
            return Err(MemoryError::MissingInput(
                "both from and to actions must exist in sequence".into(),
            ));
        }
        self.transition_matrix
            .entry(from_key)
            .or_default()
            .insert(to_key, probability);
        Ok(())
    }

    /// Validates the sequence state.
    pub fn validate(&self) -> Result<(), MemoryError> {
        Self::new(&self.actions)?;
        if self.expected_duration < 0.0 {
            return Err(MemoryError::OutOfRange {
                field: "expected_duration".into(),
                value: self.expected_duration,
                min: 0.0,
                max: f64::INFINITY,
            });
        }
        Ok(())
    }
}
