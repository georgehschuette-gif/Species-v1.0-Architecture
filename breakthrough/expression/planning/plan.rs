// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Plan {
    pub id: String,
    pub description: String,
    pub steps: Vec<PlanStep>,
    pub status: PlanStatus,
    pub priority: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlanStep {
    pub id: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub estimated_duration_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlanStatus {
    Draft,
    Active,
    Completed,
    Blocked,
    Aborted,
}

impl Plan {
    pub fn new<S: Into<String>>(id: S, description: S) -> Self {
        Self {
            id: id.into(),
            description: description.into(),
            steps: Vec::new(),
            status: PlanStatus::Draft,
            priority: 5,
        }
    }

    pub fn with_status(mut self, status: PlanStatus) -> Self {
        self.status = status;
        self
    }

    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority.min(10);
        self
    }

    pub fn add_step(&mut self, step: PlanStep) {
        self.steps.push(step);
    }

    pub fn step_count(&self) -> usize {
        self.steps.len()
    }

    pub fn is_completed(&self) -> bool {
        matches!(self.status, PlanStatus::Completed)
    }
}

impl PlanStep {
    pub fn new<S: Into<String>>(id: S, description: S) -> Self {
        Self {
            id: id.into(),
            description: description.into(),
            dependencies: Vec::new(),
            estimated_duration_ms: 0,
        }
    }

    pub fn with_dependencies(mut self, deps: Vec<String>) -> Self {
        self.dependencies = deps;
        self
    }

    pub fn with_duration(mut self, ms: u64) -> Self {
        self.estimated_duration_ms = ms;
        self
    }
}

impl fmt::Display for Plan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Plan {} ({} steps, {:?})", self.id, self.step_count(), self.status)
    }
}

impl fmt::Display for PlanStep {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Step {}: {}", self.id, self.description)
    }
}

impl fmt::Display for PlanStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PlanStatus::Draft => write!(f, "draft"),
            PlanStatus::Active => write!(f, "active"),
            PlanStatus::Completed => write!(f, "completed"),
            PlanStatus::Blocked => write!(f, "blocked"),
            PlanStatus::Aborted => write!(f, "aborted"),
        }
    }
}

