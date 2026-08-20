// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// CognitiveNiche: The functional role and position of a species in the ecosystem.
///
/// Defines how a species interacts with its environment, what resources it
/// requires, and what role it plays in the cognitive ecosystem.
pub struct CognitiveNiche {
    /// Unique identifier for this niche.
    pub niche_id: u64,
    /// The functional role of the species in this niche.
    pub functional_role: NicheRole,
    /// The resource types and amounts required by this niche.
    pub resource_requirements: Vec<f64>,
}

impl CognitiveNiche {
    /// Minimum valid niche ID.
    pub const MIN_ID: u64 = 1;
    /// Maximum resource requirement value.
    pub const MAX_RESOURCE: f64 = 1000.0;

    /// Creates a new CognitiveNiche.
    ///
    /// # Errors
    /// Returns `NicheError::InvalidId` if niche_id is zero.
    /// Returns `NicheError::InvalidResource` if any resource requirement is negative or exceeds MAX_RESOURCE.
    pub fn new(
        niche_id: u64,
        functional_role: NicheRole,
        resource_requirements: Vec<f64>,
    ) -> Result<Self, NicheError> {
        if niche_id < Self::MIN_ID {
            return Err(NicheError::InvalidId { id: niche_id });
        }
        for (i, &req) in resource_requirements.iter().enumerate() {
            if req < 0.0 || req > Self::MAX_RESOURCE {
                return Err(NicheError::InvalidResource {
                    index: i,
                    amount: req,
                });
            }
        }
        Ok(Self {
            niche_id,
            functional_role,
            resource_requirements,
        })
    }

    /// Returns the total resource requirement across all types.
    pub fn total_requirement(&self) -> f64 {
        self.resource_requirements.iter().sum()
    }

    /// Returns whether this niche can be satisfied by the given resource availability.
    pub fn is_satisfied_by(&self, available: &[f64]) -> bool {
        if available.len() < self.resource_requirements.len() {
            return false;
        }
        self.resource_requirements
            .iter()
            .zip(available.iter())
            .all(|(req, avail)| *avail >= *req)
    }

    /// Returns a compatibility score with another niche (0.0 = incompatible, 1.0 = identical).
    pub fn compatibility_with(&self, other: &CognitiveNiche) -> f64 {
        if self.resource_requirements.len() != other.resource_requirements.len() {
            return 0.0;
        }
        let mut total_diff = 0.0;
        for (a, b) in self.resource_requirements.iter().zip(other.resource_requirements.iter()) {
            total_diff += (a - b).abs();
        }
        let max_diff = self.resource_requirements.len() as f64 * Self::MAX_RESOURCE;
        if max_diff == 0.0 {
            1.0
        } else {
            1.0 - (total_diff / max_diff).min(1.0)
        }
    }

    /// Validates the niche state.
    pub fn validate(&self) -> Result<(), NicheError> {
        Self::new(self.niche_id, self.functional_role, self.resource_requirements.clone())?;
        Ok(())
    }
}

/// NicheRole: The functional role of a species in the cognitive ecosystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NicheRole {
    /// Produces new cognitive resources or energy.
    Producer,
    /// Consumes resources produced by others.
    Consumer,
    /// Breaks down and recycles unused cognitive structures.
    Decomposer,
    /// Transforms resources from one form to another.
    Transformer,
}

impl NicheRole {
    /// Returns a string label for the role.
    pub fn label(&self) -> &'static str {
        match self {
            NicheRole::Producer => "Producer",
            NicheRole::Consumer => "Consumer",
            NicheRole::Decomposer => "Decomposer",
            NicheRole::Transformer => "Transformer",
        }
    }

    /// Returns whether this role generates net resources.
    pub fn is_generative(&self) -> bool {
        matches!(self, NicheRole::Producer | NicheRole::Transformer)
    }

    /// Returns whether this role consumes resources.
    pub fn is_consumptive(&self) -> bool {
        matches!(self, NicheRole::Consumer)
    }
}

/// Error type for niche operation failures.
#[derive(Debug, Clone, PartialEq)]
pub enum NicheError {
    InvalidId { id: u64 },
    InvalidResource { index: usize, amount: f64 },
    ComputationError(String),
}

impl std::fmt::Display for NicheError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NicheError::InvalidId { id } => write!(f, "Invalid niche id: {}", id),
            NicheError::InvalidResource { index, amount } => write!(f, "Invalid resource at index {}: {}", index, amount),
            NicheError::ComputationError(msg) => write!(f, "Computation error: {}", msg),
        }
    }
}

impl std::error::Error for NicheError {}
