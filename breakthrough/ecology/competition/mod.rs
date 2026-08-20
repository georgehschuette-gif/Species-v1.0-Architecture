// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Competition: The struggle for limited resources between entities.
//! Defines how entities contest resources, territory, and attention.

pub mod interference;
pub mod exploitation;
pub mod contest;

pub use interference::InterferenceCompetition;
pub use exploitation::ExploitationCompetition;
pub use contest::ResourceContest;

/// CompetitionArena: A structured environment where competitive interactions occur.
pub struct CompetitionArena {
    pub participants: Vec<u64>,
    pub resource_id: u64,
    pub rounds: usize,
}

/// CompetitionResult: The outcome of a competitive interaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompetitionResult {
    Win(u64),
    Loss(u64),
    Draw(u64, u64),
}

/// CompetitionError: Error types for competition operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompetitionError {
    InvalidAggression(String),
    InvalidTerritoriality(String),
    InvalidConsumption(String),
    NoContestants,
    ResourceExhausted(String),
    InvalidParticipant(String),
}

impl std::fmt::Display for CompetitionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompetitionError::InvalidAggression(msg) => {
                write!(f, "Invalid aggression value: {}", msg)
            }
            CompetitionError::InvalidTerritoriality(msg) => {
                write!(f, "Invalid territoriality value: {}", msg)
            }
            CompetitionError::InvalidConsumption(msg) => {
                write!(f, "Invalid consumption rate: {}", msg)
            }
            CompetitionError::NoContestants => {
                write!(f, "No contestants in the competition")
            }
            CompetitionError::ResourceExhausted(msg) => {
                write!(f, "Resource exhausted: {}", msg)
            }
            CompetitionError::InvalidParticipant(msg) => {
                write!(f, "Invalid participant: {}", msg)
            }
        }
    }
}

impl std::error::Error for CompetitionError {}