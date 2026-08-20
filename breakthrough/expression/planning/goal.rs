// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GoalType {
    Achievement,
    Maintenance,
    Avoidance,
    Exploration,
}

impl fmt::Display for GoalType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GoalType::Achievement => write!(f, "achievement"),
            GoalType::Maintenance => write!(f, "maintenance"),
            GoalType::Avoidance => write!(f, "avoidance"),
            GoalType::Exploration => write!(f, "exploration"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Goal {
    pub id: String,
    pub description: String,
    pub goal_type: GoalType,
    pub success_criteria: Vec<String>,
    pub constraints: Vec<String>,
    pub priority: u8,
}

impl Goal {
    pub fn new<S: Into<String>>(id: S, description: S, goal_type: GoalType) -> Self {
        Self {
            id: id.into(),
            description: description.into(),
            goal_type,
            success_criteria: Vec::new(),
            constraints: Vec::new(),
            priority: 5,
        }
    }

    pub fn add_success_criterion<S: Into<String>>(&mut self, criterion: S) {
        self.success_criteria.push(criterion.into());
    }

    pub fn add_constraint<S: Into<String>>(&mut self, constraint: S) {
        self.constraints.push(constraint.into());
    }

    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority.min(10);
        self
    }

    pub fn criterion_count(&self) -> usize {
        self.success_criteria.len()
    }
}

impl fmt::Display for Goal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Goal {}: {} ({}, priority {})", self.id, self.description, self.goal_type, self.priority)
    }
}

