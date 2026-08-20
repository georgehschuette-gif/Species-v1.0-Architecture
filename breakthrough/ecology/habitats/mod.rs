// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Habitats: The environments and niches where cognitive entities exist.
//! Defines spatial, temporal, and resource characteristics of cognitive environments.

pub mod niche;
pub mod resource;
pub mod topology;

pub use niche::{CognitiveNiche, NicheRole, NicheError};
pub use resource::{ResourceMap, Resource, ResourceType, ResourceError};
pub use topology::{HabitatTopology, TopologyError};

use std::collections::HashMap;
use std::fmt;

/// Habitat: A cognitive environment with spatial, resource, and niche characteristics.
pub struct Habitat {
    pub id: u64,
    pub name: String,
    pub topology: HabitatTopology,
    pub resources: ResourceMap,
}

impl Habitat {
    /// Creates a new Habitat with validated parameters.
    ///
    /// # Errors
    /// Returns `HabitatError::InvalidNicheId` if `id` is zero.
    /// Returns `HabitatError::ComputationError` if the name is empty.
    pub fn new(
        id: u64,
        name: String,
        topology: HabitatTopology,
        resources: ResourceMap,
    ) -> Result<Self, HabitatError> {
        if id == 0 {
            return Err(HabitatError::InvalidNicheId { id });
        }
        if name.trim().is_empty() {
            return Err(HabitatError::ComputationError(
                "Habitat name must not be empty".to_string(),
            ));
        }
        Ok(Self {
            id,
            name,
            topology,
            resources,
        })
    }

    /// Returns the habitat's unique identifier.
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Returns the habitat's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Validates the habitat state.
    pub fn validate(&self) -> Result<(), HabitatError> {
        if self.id == 0 {
            return Err(HabitatError::InvalidNicheId { id: self.id });
        }
        if self.name.trim().is_empty() {
            return Err(HabitatError::ComputationError(
                "Habitat name must not be empty".to_string(),
            ));
        }
        self.topology.validate()?;
        self.resources.validate()?;
        Ok(())
    }
}

impl std::fmt::Display for Habitat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Habitat(id={}, name={}, topology={:?})",
            self.id, self.name, self.topology
        )
    }
}

/// HabitatMap: A collection of habitats indexed by their unique ID.
pub type HabitatMap = HashMap<u64, Habitat>;

/// Errors that can occur during habitat operations.
#[derive(Debug, Clone, PartialEq)]
pub enum HabitatError {
    InvalidNicheId { id: u64 },
    InvalidResourceAmount { amount: f64 },
    InvalidTopologyDimension { dimension: usize },
    InsufficientResources { required: f64, available: f64 },
    ComputationError(String),
}

impl fmt::Display for HabitatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HabitatError::InvalidNicheId { id } => write!(f, "Invalid niche id: {}", id),
            HabitatError::InvalidResourceAmount { amount } => write!(f, "Invalid resource amount: {}", amount),
            HabitatError::InvalidTopologyDimension { dimension } => write!(f, "Invalid topology dimension: {}", dimension),
            HabitatError::InsufficientResources { required, available } => write!(f, "Insufficient resources: required {}, available {}", required, available),
            HabitatError::ComputationError(msg) => write!(f, "Computation error: {}", msg),
        }
    }
}

impl std::error::Error for HabitatError {}

impl From<TopologyError> for HabitatError {
    fn from(err: TopologyError) -> Self {
        match err {
            TopologyError::InvalidDimension { dimension } => HabitatError::InvalidTopologyDimension { dimension },
            TopologyError::InvalidScore { parameter, value } => HabitatError::ComputationError(format!("invalid topology score {}: {}", parameter, value)),
            TopologyError::ComputationError(msg) => HabitatError::ComputationError(msg),
        }
    }
}

impl From<ResourceError> for HabitatError {
    fn from(err: ResourceError) -> Self {
        match err {
            ResourceError::InvalidCapacity { capacity } => HabitatError::InvalidResourceAmount { amount: capacity },
            ResourceError::InvalidRegeneration { rate } => HabitatError::InvalidResourceAmount { amount: rate },
            ResourceError::InvalidAmount { amount } => HabitatError::InvalidResourceAmount { amount },
            ResourceError::InvalidQuality { quality } => HabitatError::InvalidResourceAmount { amount: quality },
            ResourceError::ComputationError(msg) => HabitatError::ComputationError(msg),
        }
    }
}

/// Result type alias for habitat operations.
pub type HabitatResult<T> = Result<T, HabitatError>;