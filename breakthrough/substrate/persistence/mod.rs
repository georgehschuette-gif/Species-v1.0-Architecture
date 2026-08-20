// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Persistence: The maintenance of cognitive state over time.
//! Governs memory retention, durability, and resistance to decay.

pub mod retention;
pub mod durability;
pub mod decay;

pub use retention::MemoryRetention;
pub use durability::StateDurability;
pub use decay::{DecayResistance, ProtectionMechanism};

use crate::genesis::GenesisError;

#[derive(Debug, Clone, PartialEq)]
pub enum PersistenceError {
    InvalidStrength { strength: f64 },
    InvalidDecay { decay_rate: f64 },
    InvalidAge { age: f64 },
    InvalidDurability { durability: f64 },
    InvalidThreshold { threshold: f64 },
    InvalidRecovery { recovery_rate: f64 },
    InvalidElasticity { elasticity: f64 },
    InvalidResistance { resistance: f64 },
    InvalidDecayRate { decay_rate: f64 },
    InvalidStability { stability: f64 },
    ZeroDecayRate,
    Genesis(GenesisError),
}

impl std::fmt::Display for PersistenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PersistenceError::InvalidStrength { strength } => {
                write!(f, "Invalid retention strength {}: must be in [0.0, 1.0]", strength)
            }
            PersistenceError::InvalidDecay { decay_rate } => {
                write!(f, "Invalid decay rate {}: must be in [0.0, 1.0]", decay_rate)
            }
            PersistenceError::InvalidAge { age } => {
                write!(f, "Invalid age {}: must be non-negative", age)
            }
            PersistenceError::InvalidDurability { durability } => {
                write!(f, "Invalid durability {}: must be in [0.0, 1.0]", durability)
            }
            PersistenceError::InvalidThreshold { threshold } => {
                write!(f, "Invalid perturbation threshold {}: must be non-negative", threshold)
            }
            PersistenceError::InvalidRecovery { recovery_rate } => {
                write!(f, "Invalid recovery rate {}: must be in [0.0, 1.0]", recovery_rate)
            }
            PersistenceError::InvalidElasticity { elasticity } => {
                write!(f, "Invalid elasticity {}: must be in [0.0, 1.0]", elasticity)
            }
            PersistenceError::InvalidResistance { resistance } => {
                write!(f, "Invalid resistance {}: must be in [0.0, 1.0]", resistance)
            }
            PersistenceError::InvalidDecayRate { decay_rate } => {
                write!(f, "Invalid decay rate {}: must be in [0.0, 1.0]", decay_rate)
            }
            PersistenceError::InvalidStability { stability } => {
                write!(f, "Invalid stability {}: must be in [0.0, 1.0]", stability)
            }
            PersistenceError::ZeroDecayRate => {
                write!(f, "Decay rate cannot be zero for this operation")
            }
            PersistenceError::Genesis(err) => {
                write!(f, "Genesis error: {}", err)
            }
        }
    }
}

impl std::error::Error for PersistenceError {}
