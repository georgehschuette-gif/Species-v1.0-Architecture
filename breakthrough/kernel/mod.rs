// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum KernelError {
    OutOfRange { field: String, value: f64, min: f64, max: f64 },
    MissingInput(String),
    InvalidState(String),
    ThresholdNotMet { threshold: u64, actual: u64 },
    CapacityExceeded { max: usize, attempted: usize },
    ValidationFailure(String),
    InvariantViolation(String),
    CycleDetected(String),
    ComputationError(String),
    IncompatibleEntities { reason: String },
}

impl fmt::Display for KernelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KernelError::OutOfRange { field, value, min, max } => {
                write!(f, "field '{}' value {} is out of range [{}, {}]", field, value, min, max)
            }
            KernelError::MissingInput(detail) => write!(f, "missing required input: {}", detail),
            KernelError::InvalidState(detail) => write!(f, "invalid state: {}", detail),
            KernelError::ThresholdNotMet { threshold, actual } => {
                write!(f, "threshold {} not met (actual: {})", threshold, actual)
            }
            KernelError::CapacityExceeded { max, attempted } => {
                write!(f, "capacity exceeded: max {} but attempted {}", max, attempted)
            }
            KernelError::ValidationFailure(reason) => write!(f, "validation failed: {}", reason),
            KernelError::InvariantViolation(reason) => write!(f, "invariant violation: {}", reason),
            KernelError::CycleDetected(reason) => write!(f, "cycle detected: {}", reason),
            KernelError::ComputationError(reason) => write!(f, "computation error: {}", reason),
            KernelError::IncompatibleEntities { reason } => {
                write!(f, "incompatible entities: {}", reason)
            }
        }
    }
}

impl std::error::Error for KernelError {}

pub type KernelResult<T> = Result<T, KernelError>;

pub mod topology_engine;
pub mod coherence_engine;
pub mod evolution_engine;
pub mod resonance_engine;
pub mod event_field;
pub mod scheduler;
pub mod ecosystem_runtime;
pub mod emergence_engine;
pub mod bootstrap;

pub use topology_engine::TopologyEngine;
pub use coherence_engine::CoherenceEngine;
pub use evolution_engine::EvolutionEngine;
pub use resonance_engine::ResonanceEngine;
pub use event_field::EventField;
pub use scheduler::Scheduler;
pub use ecosystem_runtime::EcosystemRuntime;
pub use emergence_engine::EmergenceEngine;
pub use bootstrap::Bootstrap;
