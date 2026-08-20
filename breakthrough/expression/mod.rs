// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

mod language;
mod code;
mod vision;
mod planning;
mod action;
mod explanation;
mod creativity;
mod reflection;

pub use language::{Utterance, Grammar};
pub use code::{Program, CodeExecution};
pub use vision::{Image, SceneInterpretation};
pub use planning::{Plan, Goal};
pub use action::{Action, ActionExecution};
pub use explanation::{Explanation, Justification};
pub use creativity::{Creation, Novelty};
pub use reflection::{Reflection, Introspection};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExpressionError {
    InvalidUtterance(String),
    InvalidGrammar(String),
    InvalidProgram(String),
    InvalidExecution(String),
    InvalidImage(String),
    InvalidScene(String),
    InvalidPlan(String),
    InvalidGoal(String),
    InvalidAction(String),
    InvalidExplanation(String),
    InvalidJustification(String),
    InvalidCreation(String),
    InvalidNovelty(String),
    InvalidReflection(String),
    InvalidIntrospection(String),
}

impl fmt::Display for ExpressionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExpressionError::InvalidUtterance(msg) => write!(f, "Invalid utterance: {}", msg),
            ExpressionError::InvalidGrammar(msg) => write!(f, "Invalid grammar: {}", msg),
            ExpressionError::InvalidProgram(msg) => write!(f, "Invalid program: {}", msg),
            ExpressionError::InvalidExecution(msg) => write!(f, "Invalid execution: {}", msg),
            ExpressionError::InvalidImage(msg) => write!(f, "Invalid image: {}", msg),
            ExpressionError::InvalidScene(msg) => write!(f, "Invalid scene: {}", msg),
            ExpressionError::InvalidPlan(msg) => write!(f, "Invalid plan: {}", msg),
            ExpressionError::InvalidGoal(msg) => write!(f, "Invalid goal: {}", msg),
            ExpressionError::InvalidAction(msg) => write!(f, "Invalid action: {}", msg),
            ExpressionError::InvalidExplanation(msg) => write!(f, "Invalid explanation: {}", msg),
            ExpressionError::InvalidJustification(msg) => write!(f, "Invalid justification: {}", msg),
            ExpressionError::InvalidCreation(msg) => write!(f, "Invalid creation: {}", msg),
            ExpressionError::InvalidNovelty(msg) => write!(f, "Invalid novelty: {}", msg),
            ExpressionError::InvalidReflection(msg) => write!(f, "Invalid reflection: {}", msg),
            ExpressionError::InvalidIntrospection(msg) => write!(f, "Invalid introspection: {}", msg),
        }
    }
}

impl std::error::Error for ExpressionError {}

