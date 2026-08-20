// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

pub mod discovery;
pub mod expansion;
pub mod exploration;
pub mod frontier_detection;
pub mod opportunity_fields;
pub mod unknown_regions;

pub use discovery::{Discovery, DiscoveryError};
pub use expansion::{Expansion, ExpansionError};
pub use exploration::{Exploration, ExplorationError};
pub use frontier_detection::{FrontierDetection, FrontierError};
pub use opportunity_fields::{FieldsError, OpportunityFields};
pub use unknown_regions::UnknownRegions;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HorizonsError {
    InvalidExploration(String),
    InvalidFrontier(String),
    InvalidRegion(String),
    InvalidField(String),
    InvalidDiscovery(String),
    InvalidExpansion(String),
    ConfigurationError(String),
}

impl std::fmt::Display for HorizonsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HorizonsError::InvalidExploration(msg) => write!(f, "Invalid exploration: {}", msg),
            HorizonsError::InvalidFrontier(msg) => write!(f, "Invalid frontier: {}", msg),
            HorizonsError::InvalidRegion(msg) => write!(f, "Invalid region: {}", msg),
            HorizonsError::InvalidField(msg) => write!(f, "Invalid field: {}", msg),
            HorizonsError::InvalidDiscovery(msg) => write!(f, "Invalid discovery: {}", msg),
            HorizonsError::InvalidExpansion(msg) => write!(f, "Invalid expansion: {}", msg),
            HorizonsError::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
        }
    }
}

impl std::error::Error for HorizonsError {}

pub type HorizonsResult<T> = Result<T, HorizonsError>;
