// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ImaginationError {
    OutOfRange {
        field: String,
        value: f64,
        min: f64,
        max: f64,
    },
    MissingInput(String),
    InvalidInput(String),
    DimensionMismatch {
        expected: usize,
        actual: usize,
    },
    CapacityExceeded {
        max: usize,
        attempted: usize,
    },
    ParadoxViolation {
        description: String,
    },
    Inconsistency {
        detail: String,
    },
}

impl fmt::Display for ImaginationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfRange { field, value, min, max } => {
                write!(f, "{} out of range: {} not in [{}, {}]", field, value, min, max)
            }
            Self::MissingInput(msg) => write!(f, "missing input: {}", msg),
            Self::InvalidInput(msg) => write!(f, "invalid input: {}", msg),
            Self::DimensionMismatch { expected, actual } => {
                write!(f, "dimension mismatch: expected {}, got {}", expected, actual)
            }
            Self::CapacityExceeded { max, attempted } => {
                write!(f, "capacity exceeded: max {}, attempted {}", max, attempted)
            }
            Self::ParadoxViolation { description } => {
                write!(f, "paradox violation: {}", description)
            }
            Self::Inconsistency { detail } => {
                write!(f, "logical inconsistency: {}", detail)
            }
        }
    }
}

impl std::error::Error for ImaginationError {}

pub mod simulations;
pub mod alternate_worlds;
pub mod counterfactuals;
pub mod impossible_objects;
pub mod future_histories;
pub mod dream_cycles;
pub mod synthetic_memories;
pub mod speculative_reasoning;
