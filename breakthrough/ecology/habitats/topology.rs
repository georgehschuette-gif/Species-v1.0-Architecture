// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// HabitatTopology: The spatial and structural layout of cognitive habitats.
///
/// Describes the dimensionality, connectivity, and fragmentation of a habitat.
/// These properties determine how easily entities can move, interact, and spread.
#[derive(Debug)]
pub struct HabitatTopology {
    /// Number of spatial dimensions in the habitat.
    pub dimensions: usize,
    /// Connectivity score in [0.0, 1.0] (1.0 = fully connected).
    pub connectivity: f64,
    /// Fragmentation score in [0.0, 1.0] (1.0 = completely fragmented).
    pub fragmentation: f64,
}

impl HabitatTopology {
    /// Minimum valid number of dimensions.
    pub const MIN_DIMENSIONS: usize = 1;
    /// Maximum valid number of dimensions.
    pub const MAX_DIMENSIONS: usize = 3;

    /// Creates a new HabitatTopology.
    ///
    /// # Errors
    /// Returns `TopologyError::InvalidDimension` if dimensions is outside [1, 3].
    /// Returns `TopologyError::InvalidScore` if connectivity or fragmentation are outside [0.0, 1.0].
    pub fn new(
        dimensions: usize,
        connectivity: f64,
        fragmentation: f64,
    ) -> Result<Self, TopologyError> {
        if !(Self::MIN_DIMENSIONS..=Self::MAX_DIMENSIONS).contains(&dimensions) {
            return Err(TopologyError::InvalidDimension { dimension: dimensions });
        }
        if !(0.0..=1.0).contains(&connectivity) {
            return Err(TopologyError::InvalidScore {
                parameter: "connectivity".to_string(),
                value: connectivity,
            });
        }
        if !(0.0..=1.0).contains(&fragmentation) {
            return Err(TopologyError::InvalidScore {
                parameter: "fragmentation".to_string(),
                value: fragmentation,
            });
        }
        Ok(Self {
            dimensions,
            connectivity,
            fragmentation,
        })
    }

    /// Returns the effective connectivity after accounting for fragmentation.
    ///
    /// Effective connectivity = connectivity * (1 - fragmentation).
    pub fn effective_connectivity(&self) -> f64 {
        self.connectivity * (1.0 - self.fragmentation)
    }

    /// Returns whether the habitat is well-connected.
    pub fn is_well_connected(&self) -> bool {
        self.effective_connectivity() > 0.5
    }

    /// Returns the estimated path length between random points in the habitat.
    ///
    /// In a well-connected habitat, paths are shorter.
    pub fn estimated_path_length(&self) -> f64 {
        if self.effective_connectivity() < f64::EPSILON {
            f64::INFINITY
        } else {
            self.dimensions as f64 / self.effective_connectivity()
        }
    }

    /// Returns whether the habitat is fragmented.
    pub fn is_fragmented(&self) -> bool {
        self.fragmentation > 0.5
    }

    /// Validates the topology state.
    pub fn validate(&self) -> Result<(), TopologyError> {
        Self::new(self.dimensions, self.connectivity, self.fragmentation)?;
        Ok(())
    }
}

/// Error type for topology operation failures.
#[derive(Debug, Clone, PartialEq)]
pub enum TopologyError {
    InvalidDimension { dimension: usize },
    InvalidScore { parameter: String, value: f64 },
    ComputationError(String),
}

impl std::fmt::Display for TopologyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TopologyError::InvalidDimension { dimension } => write!(f, "Invalid dimension: {}", dimension),
            TopologyError::InvalidScore { parameter, value } => write!(f, "Invalid score for {}: {}", parameter, value),
            TopologyError::ComputationError(msg) => write!(f, "Computation error: {}", msg),
        }
    }
}

impl std::error::Error for TopologyError {}
