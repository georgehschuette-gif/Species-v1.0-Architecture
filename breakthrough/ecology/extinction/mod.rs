// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Extinction: The disappearance of species or populations.
//! Defines how cognitive entities and structures die out and are lost.

pub mod risk;
pub mod event;
pub mod recovery;

pub use risk::ExtinctionRisk;
pub use event::ExtinctionEvent;
pub use recovery::DeExtinction;

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ExtinctionError {
    InvalidProbability(f64),
    EmptyPopulation,
    UnfeasibleRecovery,
    UnrecognizedCause,
}

impl fmt::Display for ExtinctionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidProbability(value) => write!(f, "invalid probability value: {}", value),
            Self::EmptyPopulation => write!(f, "population threshold is zero"),
            Self::UnfeasibleRecovery => write!(f, "recovery strategy is not feasible"),
            Self::UnrecognizedCause => write!(f, "extinction cause not recognized"),
        }
    }
}

impl std::error::Error for ExtinctionError {}
