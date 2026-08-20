// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmergenceError {
    InvalidPattern(String),
    ConsensusFailure(String),
    CriticalPointUnstable(String),
    HierarchyViolation(String),
    MeasurementError(String),
    IoError(String),
}

impl fmt::Display for EmergenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EmergenceError::InvalidPattern(msg) => write!(f, "invalid pattern: {}", msg),
            EmergenceError::ConsensusFailure(msg) => write!(f, "consensus failure: {}", msg),
            EmergenceError::CriticalPointUnstable(msg) => write!(f, "critical point unstable: {}", msg),
            EmergenceError::HierarchyViolation(msg) => write!(f, "hierarchy violation: {}", msg),
            EmergenceError::MeasurementError(msg) => write!(f, "measurement error: {}", msg),
            EmergenceError::IoError(msg) => write!(f, "io error: {}", msg),
        }
    }
}

impl std::error::Error for EmergenceError {}

pub type Result<T> = std::result::Result<T, EmergenceError>;

pub mod spontaneous_patterns;
pub mod self_organization;
pub mod criticality;
pub mod collective_behavior;
pub mod hierarchy;
pub mod distributed_consensus;
pub mod global_states;
pub mod emergence_metrics;

pub use spontaneous_patterns::{Formation, FormationId, Pattern, PatternId};
pub use self_organization::{Organizer, OrganizerId, Principle};
pub use criticality::{CriticalPoint, PowerLaw};
pub use collective_behavior::{Flock, FlockId, Swarm, SwarmId};
pub use hierarchy::{DominanceHierarchy, DominanceRelation, Level, LevelId};
pub use distributed_consensus::{ConsensusRound, ConsensusState, Vote, VotingProtocol};
pub use global_states::{GlobalState, GlobalStateId, MacroState};
pub use emergence_metrics::{Measurement, Metric, MetricId};
