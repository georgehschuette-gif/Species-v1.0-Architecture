// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExecutionOutcome {
    Success,
    PartialSuccess(String),
    Failure(String),
    Timeout,
}

impl fmt::Display for ExecutionOutcome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExecutionOutcome::Success => write!(f, "success"),
            ExecutionOutcome::PartialSuccess(reason) => write!(f, "partial success: {}", reason),
            ExecutionOutcome::Failure(reason) => write!(f, "failure: {}", reason),
            ExecutionOutcome::Timeout => write!(f, "timeout"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ActionExecution {
    pub action_id: String,
    pub outcome: ExecutionOutcome,
    pub result_data: Vec<(String, String)>,
    pub actual_duration_ms: u64,
    pub attempts: u32,
}

impl ActionExecution {
    pub fn new<S: Into<String>>(action_id: S) -> Self {
        Self {
            action_id: action_id.into(),
            outcome: ExecutionOutcome::Success,
            result_data: Vec::new(),
            actual_duration_ms: 0,
            attempts: 1,
        }
    }

    pub fn with_outcome(mut self, outcome: ExecutionOutcome) -> Self {
        self.outcome = outcome;
        self
    }

    pub fn add_result<K: Into<String>, V: Into<String>>(&mut self, key: K, value: V) {
        self.result_data.push((key.into(), value.into()));
    }

    pub fn with_duration(mut self, duration_ms: u64) -> Self {
        self.actual_duration_ms = duration_ms;
        self
    }

    pub fn with_attempts(mut self, attempts: u32) -> Self {
        self.attempts = attempts;
        self
    }

    pub fn is_successful(&self) -> bool {
        matches!(self.outcome, ExecutionOutcome::Success)
    }
}

impl fmt::Display for ActionExecution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ActionExecution {} -> {}", self.action_id, self.outcome)
    }
}

