// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Populations: Groups of entities of the same species.
//! Defines how entities aggregate, reproduce, and compete within populations.

pub mod dynamics;
pub mod growth;
pub mod regulation;

use std::fmt;

/// Unique identifier for a population cluster.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PopulationId(pub u64);

/// Unique identifier for a species.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpeciesId(pub u64);

/// Population: A cluster of entities sharing the same species.
///
/// A population is characterized by its identity, species membership,
/// current size, and the environmental carrying capacity that bounds it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Population {
    pub id: PopulationId,
    pub species_id: SpeciesId,
    pub size: usize,
    pub carrying_capacity: usize,
}

impl Population {
    /// Create a new population with validated parameters.
    pub fn new(id: PopulationId, species_id: SpeciesId, size: usize, carrying_capacity: usize) -> Result<Self, PopulationError> {
        if size == 0 {
            return Err(PopulationError::InvalidSize("Population size must be positive".into()));
        }
        if carrying_capacity == 0 {
            return Err(PopulationError::InvalidSize("Carrying capacity must be positive".into()));
        }
        if size > carrying_capacity {
            return Err(PopulationError::InfeasiblePopulation);
        }

        Ok(Self { id, species_id, size, carrying_capacity })
    }

    /// Create a population at maximal carrying capacity.
    pub fn at_capacity(id: PopulationId, species_id: SpeciesId, carrying_capacity: usize) -> Self {
        assert!(carrying_capacity > 0);
        Self { id, species_id, size: carrying_capacity, carrying_capacity }
    }

    /// Create an empty population.
    pub fn empty(id: PopulationId, species_id: SpeciesId) -> Self {
        Self { id, species_id, size: 0, carrying_capacity: usize::MAX }
    }

    /// Return whether the population is viable (size > 0 with positive capacity).
    pub fn is_viable(&self) -> bool {
        self.size > 0 && self.carrying_capacity > 0
    }

    /// Compute the density of the population relative to its carrying capacity.
    pub fn density(&self) -> f64 {
        if self.carrying_capacity == 0 {
            0.0
        } else {
            self.size as f64 / self.carrying_capacity as f64
        }
    }

    /// Update the population size and capacity in place, returning an error if constraints are violated.
    pub fn resize(&mut self, new_size: usize, new_k: usize) -> Result<(), PopulationError> {
        if new_k == 0 {
            return Err(PopulationError::InvalidSize("Carrying capacity must be positive".into()));
        }
        if new_size > new_k {
            return Err(PopulationError::InfeasiblePopulation);
        }
        self.size = new_size;
        self.carrying_capacity = new_k;
        Ok(())
    }
}

impl Default for Population {
    fn default() -> Self {
        Self::empty(PopulationId(0), SpeciesId(0))
    }
}

/// Errors that can occur during population operations.
#[derive(Debug, Clone, PartialEq)]
pub enum PopulationError {
    /// A size-related parameter is invalid (zero, negative, or overflowing).
    InvalidSize(String),
    /// A growth rate parameter is invalid (NaN, infinite, or out of expected bounds).
    InvalidGrowthRate(String),
    /// A rate parameter is invalid (NaN, infinite, or out of bounds).
    InvalidRate(String),
    /// Arithmetic overflow occurred during population calculation.
    Overflow,
    /// Population size fell to zero or below during regulation.
    Underflow,
    /// Parameters describe an infeasible population (e.g., size exceeds capacity).
    InfeasiblePopulation,
    /// A required population reference is uninitialized (zero ID).
    UninitializedReference,
}

impl fmt::Display for PopulationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PopulationError::InvalidSize(msg) => write!(f, "Invalid population size: {}", msg),
            PopulationError::InvalidGrowthRate(msg) => write!(f, "Invalid growth rate: {}", msg),
            PopulationError::InvalidRate(msg) => write!(f, "Invalid rate: {}", msg),
            PopulationError::Overflow => write!(f, "Population overflow"),
            PopulationError::Underflow => write!(f, "Population underflow"),
            PopulationError::InfeasiblePopulation => write!(f, "Infeasible population parameters"),
            PopulationError::UninitializedReference => write!(f, "Uninitialized population reference"),
        }
    }
}

impl std::error::Error for PopulationError {}

pub use dynamics::PopulationDynamics;
pub use growth::PopulationGrowth;
pub use regulation::PopulationRegulation;
