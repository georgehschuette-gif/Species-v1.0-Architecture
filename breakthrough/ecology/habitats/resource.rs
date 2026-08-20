// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// ResourceMap: Tracks the distribution and availability of resources in a habitat.
///
/// Maintains a collection of resources with amounts and quality ratings.
/// Supports regeneration queries and capacity management.
pub struct ResourceMap {
    /// The individual resources tracked in this map.
    pub resources: Vec<Resource>,
    /// Total capacity of the habitat for all resources combined.
    pub total_capacity: f64,
    /// Rate at which resources regenerate per time unit.
    pub regeneration_rate: f64,
}

impl ResourceMap {
    /// Minimum valid total capacity.
    pub const MIN_CAPACITY: f64 = 0.0;
    /// Minimum valid regeneration rate.
    pub const MIN_REGEN: f64 = 0.0;
    /// Maximum valid resource quality.
    pub const MAX_QUALITY: f64 = 1.0;
    /// Minimum valid resource amount.
    pub const MIN_AMOUNT: f64 = 0.0;

    /// Creates a new ResourceMap.
    ///
    /// # Errors
    /// Returns `ResourceError::InvalidCapacity` if total_capacity is negative.
    /// Returns `ResourceError::InvalidRegeneration` if regeneration_rate is negative.
    pub fn new(
        resources: Vec<Resource>,
        total_capacity: f64,
        regeneration_rate: f64,
    ) -> Result<Self, ResourceError> {
        if total_capacity < Self::MIN_CAPACITY {
            return Err(ResourceError::InvalidCapacity { capacity: total_capacity });
        }
        if regeneration_rate < Self::MIN_REGEN {
            return Err(ResourceError::InvalidRegeneration { rate: regeneration_rate });
        }
        Ok(Self {
            resources,
            total_capacity,
            regeneration_rate,
        })
    }

    /// Returns the total amount of all resources currently available.
    pub fn total_available(&self) -> f64 {
        self.resources.iter().map(|r| r.amount).sum()
    }

    /// Returns the average quality across all resources.
    pub fn average_quality(&self) -> f64 {
        if self.resources.is_empty() {
            0.0
        } else {
            self.resources.iter().map(|r| r.quality).sum::<f64>() / self.resources.len() as f64
        }
    }

    /// Consumes a specified amount from a resource type.
    ///
    /// Returns the actual amount consumed (may be less than requested).
    pub fn consume(&mut self, resource_type: ResourceType, amount: f64) -> Result<f64, ResourceError> {
        if amount < 0.0 {
            return Err(ResourceError::InvalidAmount { amount });
        }
        for resource in &mut self.resources {
            if resource.resource_type == resource_type {
                let consumed = resource.amount.min(amount);
                resource.amount -= consumed;
                return Ok(consumed);
            }
        }
        Ok(0.0)
    }

    /// Replenishes a resource type by the given amount, capped at total_capacity.
    pub fn replenish(&mut self, resource_type: ResourceType, amount: f64) -> Result<(), ResourceError> {
        if amount < 0.0 {
            return Err(ResourceError::InvalidAmount { amount });
        }
        for resource in &mut self.resources {
            if resource.resource_type == resource_type {
                resource.amount = (resource.amount + amount).min(self.total_capacity);
                return Ok(());
            }
        }
        Ok(())
    }

    /// Applies regeneration to all resources.
    pub fn regenerate(&mut self) {
        for resource in &mut self.resources {
            resource.amount = (resource.amount + self.regeneration_rate).min(self.total_capacity);
        }
    }

    /// Returns whether the habitat is at capacity.
    pub fn is_at_capacity(&self) -> bool {
        self.total_available() >= self.total_capacity
    }

    /// Returns the utilization ratio (available / capacity).
    pub fn utilization(&self) -> f64 {
        if self.total_capacity == 0.0 {
            0.0
        } else {
            self.total_available() / self.total_capacity
        }
    }

    /// Validates the resource map state.
    pub fn validate(&self) -> Result<(), ResourceError> {
        Self::new(self.resources.clone(), self.total_capacity, self.regeneration_rate)?;
        for resource in &self.resources {
            if resource.amount < 0.0 || resource.amount > self.total_capacity {
                return Err(ResourceError::InvalidAmount { amount: resource.amount });
            }
            if resource.quality < 0.0 || resource.quality > Self::MAX_QUALITY {
                return Err(ResourceError::InvalidQuality { quality: resource.quality });
            }
        }
        Ok(())
    }
}

/// Resource: A single tracked resource in the habitat.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Resource {
    /// The type of resource.
    pub resource_type: ResourceType,
    /// Current amount available.
    pub amount: f64,
    /// Quality rating in [0.0, 1.0].
    pub quality: f64,
}

impl Resource {
    /// Creates a new Resource.
    ///
    /// # Errors
    /// Returns `ResourceError::InvalidAmount` if amount is negative.
    /// Returns `ResourceError::InvalidQuality` if quality is outside [0.0, 1.0].
    pub fn new(resource_type: ResourceType, amount: f64, quality: f64) -> Result<Self, ResourceError> {
        if amount < 0.0 {
            return Err(ResourceError::InvalidAmount { amount });
        }
        if !(0.0..=ResourceMap::MAX_QUALITY).contains(&quality) {
            return Err(ResourceError::InvalidQuality { quality });
        }
        Ok(Self {
            resource_type,
            amount,
            quality,
        })
    }

    /// Returns the effective amount (amount * quality).
    pub fn effective_amount(&self) -> f64 {
        self.amount * self.quality
    }
}

/// ResourceType: The kind of cognitive resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceType {
    Energy,
    Information,
    Connection,
    Computation,
}

impl ResourceType {
    /// Returns a string label for the resource type.
    pub fn label(&self) -> &'static str {
        match self {
            ResourceType::Energy => "Energy",
            ResourceType::Information => "Information",
            ResourceType::Connection => "Connection",
            ResourceType::Computation => "Computation",
        }
    }
}

/// Error type for resource operation failures.
#[derive(Debug, Clone, PartialEq)]
pub enum ResourceError {
    InvalidCapacity { capacity: f64 },
    InvalidRegeneration { rate: f64 },
    InvalidAmount { amount: f64 },
    InvalidQuality { quality: f64 },
    ComputationError(String),
}

impl std::fmt::Display for ResourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceError::InvalidCapacity { capacity } => write!(f, "Invalid capacity: {}", capacity),
            ResourceError::InvalidRegeneration { rate } => write!(f, "Invalid regeneration rate: {}", rate),
            ResourceError::InvalidAmount { amount } => write!(f, "Invalid amount: {}", amount),
            ResourceError::InvalidQuality { quality } => write!(f, "Invalid quality: {}", quality),
            ResourceError::ComputationError(msg) => write!(f, "Computation error: {}", msg),
        }
    }
}

impl std::error::Error for ResourceError {}
