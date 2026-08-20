// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum MorphogenesisError {
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
}

impl fmt::Display for MorphogenesisError {
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
        }
    }
}

impl std::error::Error for MorphogenesisError {}

pub mod mutation;
pub mod crossover;
pub mod recombination;
pub mod crystallization;
pub mod stabilization;
pub mod bifurcation;
pub mod collapse;
pub mod regeneration;
pub mod evolutionary_cycles;
