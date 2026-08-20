// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Adaptation: The evolutionary modification of entities and populations.
//! Defines how cognitive structures change to better fit their environment.

pub mod selection;
pub mod mutation;
pub mod drift;

pub use selection::{NaturalSelection, FitnessFunction, SelectionError};
pub use mutation::{AdaptiveMutation, MutationError};
pub use drift::{GeneticDrift, DriftError};

use std::fmt;

/// AdaptationMechanism: The type of evolutionary adaptation mechanism.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdaptationMechanism {
    NaturalSelection,
    Mutation,
    GeneticDrift,
}

/// Errors that can occur during adaptation operations.
#[derive(Debug, Clone, PartialEq)]
pub enum AdaptationError {
    InvalidSelectionPressure { pressure: f64 },
    InvalidMutationRate { rate: f64 },
    InvalidPopulationSize { size: usize },
    InvalidFitnessParameter { detail: String },
    ComputationError(String),
}

impl fmt::Display for AdaptationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AdaptationError::InvalidSelectionPressure { pressure } => write!(f, "Invalid selection pressure: {}", pressure),
            AdaptationError::InvalidMutationRate { rate } => write!(f, "Invalid mutation rate: {}", rate),
            AdaptationError::InvalidPopulationSize { size } => write!(f, "Invalid population size: {}", size),
            AdaptationError::InvalidFitnessParameter { detail } => write!(f, "Invalid fitness parameter: {}", detail),
            AdaptationError::ComputationError(msg) => write!(f, "Computation error: {}", msg),
        }
    }
}

impl std::error::Error for AdaptationError {}

/// Result type alias for adaptation operations.
pub type AdaptationResult<T> = Result<T, AdaptationError>;