// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Fields: Cognitive field types that overlay the substrate topology.
//!
//! Fields are continuous or discrete value distributions that encode
//! different dimensions of cognitive meaning across the ecosystem.
//! They modulate entity behavior, interaction strengths, and
//! emergent dynamics.

pub mod semantic;
pub mod emotional;
pub mod causal;
pub mod temporal;
pub mod social;
pub mod ethical;
pub mod predictive;
pub mod hypothetical;
pub mod imaginative;
pub mod probabilistic;

pub use semantic::{SemanticField, SemanticError};
pub use emotional::{EmotionalField, EmotionalError};
pub use causal::{CausalField, CausalError};
pub use temporal::{TemporalField, TemporalError};
pub use social::{SocialField, SocialError};
pub use ethical::{EthicalField, EthicalError};
pub use predictive::{PredictiveField, PredictiveError};
pub use hypothetical::{HypotheticalField, HypotheticalError};
pub use imaginative::{ImaginativeField, ImaginativeError};
pub use probabilistic::{ProbabilisticField, ProbabilisticError};

use std::fmt;

/// Errors that can occur during field operations.
#[derive(Debug, Clone, PartialEq)]
pub enum FieldError {
    OutOfRange { field: String, value: f64, min: f64, max: f64 },
    InvalidDimension { dimension: usize },
    InsufficientData,
    ComputationError(String),
    IncompatibleFields { reason: String },
}

impl fmt::Display for FieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FieldError::OutOfRange { field, value, min, max } => write!(f, "field '{}' value {} is out of range [{}, {}]", field, value, min, max),
            FieldError::InvalidDimension { dimension } => write!(f, "invalid dimension: {}", dimension),
            FieldError::InsufficientData => write!(f, "insufficient data for operation"),
            FieldError::ComputationError(msg) => write!(f, "computation error: {}", msg),
            FieldError::IncompatibleFields { reason } => write!(f, "incompatible fields: {}", reason),
        }
    }
}

impl std::error::Error for FieldError {}

/// Result type alias for field operations.
pub type FieldResult<T> = Result<T, FieldError>;
