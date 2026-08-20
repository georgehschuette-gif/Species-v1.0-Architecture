// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::RealityError;

/// Action: A discrete executable unit within an intervention.
///
/// Actions are the smallest executable steps within an intervention.
/// They encapsulate the "how" of achieving an intervention's goal.
#[derive(Debug, Clone, PartialEq)]
pub struct Action {
    /// Unique identifier for this action.
    pub id: String,
    /// Human-readable description.
    pub description: String,
    /// Parameters required for execution.
    pub parameters: HashMap<String, f64>,
    /// Preconditions that must be satisfied before execution.
    pub preconditions: Vec<String>,
    /// Expected effects on the environment.
    pub expected_effects: Vec<String>,
    /// Estimated execution cost (resource units).
    pub estimated_cost: f64,
    /// Priority level (higher = more important).
    pub priority: u32,
    /// Whether this action has been executed.
    pub executed: bool,
    /// Execution result if executed.
    pub result: Option<ActionResult>,
}

/// ActionResult: The outcome of executing an action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActionResult {
    /// Action completed successfully.
    Success,
    /// Action failed.
    Failure,
    /// Action partially succeeded.
    Partial,
    /// Action was skipped.
    Skipped,
    /// Action timed out.
    Timeout,
}

impl ActionResult {
    /// Returns the string label of this result.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Success => "success",
            Self::Failure => "failure",
            Self::Partial => "partial",
            Self::Skipped => "skipped",
            Self::Timeout => "timeout",
        }
    }

    /// Returns whether this result indicates success.
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success | Self::Partial)
    }

    /// Returns whether this result indicates failure.
    pub fn is_failure(&self) -> bool {
        matches!(self, Self::Failure | Self::Timeout)
    }
}

impl std::fmt::Display for ActionResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Action {
    /// Creates a new action.
    pub fn new(
        id: impl Into<String>,
        description: impl Into<String>,
        estimated_cost: f64,
        priority: u32,
    ) -> Self {
        Self {
            id: id.into(),
            description: description.into(),
            parameters: HashMap::new(),
            preconditions: Vec::new(),
            expected_effects: Vec::new(),
            estimated_cost,
            priority,
            executed: false,
            result: None,
        }
    }

    /// Adds a parameter to the action.
    pub fn with_parameter(mut self, key: impl Into<String>, value: f64) -> Self {
        self.parameters.insert(key.into(), value);
        self
    }

    /// Adds a precondition to the action.
    pub fn with_precondition(mut self, precondition: impl Into<String>) -> Self {
        self.preconditions.push(precondition.into());
        self
    }

    /// Adds an expected effect to the action.
    pub fn with_expected_effect(mut self, effect: impl Into<String>) -> Self {
        self.expected_effects.push(effect.into());
        self
    }

    /// Sets the execution result.
    pub fn set_result(&mut self, result: ActionResult) {
        self.executed = true;
        self.result = Some(result);
    }

    /// Returns whether this action is ready to execute.
    pub fn is_ready(&self) -> bool {
        !self.executed && !self.preconditions.is_empty()
    }

    /// Returns whether all preconditions are met (placeholder logic).
    pub fn check_preconditions(&self, satisfied: &[String]) -> bool {
        self.preconditions.iter().all(|p| satisfied.contains(p))
    }
}

use std::collections::HashMap;

