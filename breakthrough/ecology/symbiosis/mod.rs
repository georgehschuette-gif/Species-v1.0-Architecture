// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Symbiosis: Close, long-term interactions between different species.
//! Defines mutualistic, commensal, and parasitic relationships between entities.

pub mod mutualism;
pub mod commensalism;
pub mod parasitism;

pub use mutualism::MutualisticBond;
pub use commensalism::CommensalRelationship;
pub use parasitism::ParasiticInteraction;

/// SymbiosisType: The category of a symbiotic interaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbiosisType {
    Mutualism,
    Commensalism,
    Parasitism,
}

/// SymbioticRelationship: A typed wrapper over the three symbiotic interaction variants.
pub enum SymbioticRelationship {
    Mutualism(MutualisticBond),
    Commensalism(CommensalRelationship),
    Parasitism(ParasiticInteraction),
}

/// SymbiosisError: Error types for symbiosis operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymbiosisError {
    InvalidBenefit(String),
    InvalidStability(String),
    InvalidExploitation(String),
    InvalidVirulence(String),
    SpeciesNotFound(String),
    InvalidRelationship(String),
}

impl std::fmt::Display for SymbiosisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SymbiosisError::InvalidBenefit(msg) => {
                write!(f, "Invalid benefit value: {}", msg)
            }
            SymbiosisError::InvalidStability(msg) => {
                write!(f, "Invalid stability value: {}", msg)
            }
            SymbiosisError::InvalidExploitation(msg) => {
                write!(f, "Invalid exploitation rate: {}", msg)
            }
            SymbiosisError::InvalidVirulence(msg) => {
                write!(f, "Invalid virulence value: {}", msg)
            }
            SymbiosisError::SpeciesNotFound(msg) => {
                write!(f, "Species not found: {}", msg)
            }
            SymbiosisError::InvalidRelationship(msg) => {
                write!(f, "Invalid relationship: {}", msg)
            }
        }
    }
}

impl std::error::Error for SymbiosisError {}