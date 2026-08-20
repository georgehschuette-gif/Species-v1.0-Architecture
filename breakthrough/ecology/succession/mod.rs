// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Succession: The directional change in community composition over time.
//! Defines how cognitive ecosystems evolve through stages of development.

pub mod primary;
pub mod secondary;
pub mod climax;

pub use primary::PrimarySuccession;
pub use secondary::SecondarySuccession;
pub use climax::ClimaxCommunity;

/// SuccessionStage: The current stage of ecological succession.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SuccessionStage {
    Primary,
    Secondary,
    Climax,
}

/// EcologicalSuccession: Tracks the progress of ecological succession over time.
pub struct EcologicalSuccession {
    pub stage: SuccessionStage,
    pub current_community: Vec<u64>,
    pub progress: f64,
    pub time_step: f64,
}

/// SuccessionError: Error types for succession operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SuccessionError {
    InvalidStage(String),
    InvalidProgress(String),
    InvalidTime(String),
    NoPioneerSpecies(String),
    CommunityNotStable(String),
    InvalidTrust(String),
    InvalidStability(String),
    InvalidParticipant(String),
}

impl std::fmt::Display for SuccessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SuccessionError::InvalidStage(msg) => {
                write!(f, "Invalid succession stage: {}", msg)
            }
            SuccessionError::InvalidProgress(msg) => {
                write!(f, "Invalid progress value: {}", msg)
            }
            SuccessionError::InvalidTime(msg) => {
                write!(f, "Invalid time value: {}", msg)
            }
            SuccessionError::NoPioneerSpecies(msg) => {
                write!(f, "No pioneer species: {}", msg)
            }
            SuccessionError::CommunityNotStable(msg) => {
                write!(f, "Community not stable: {}", msg)
            }
            SuccessionError::InvalidTrust(msg) => {
                write!(f, "Invalid trust value: {}", msg)
            }
            SuccessionError::InvalidStability(msg) => {
                write!(f, "Invalid stability value: {}", msg)
            }
            SuccessionError::InvalidParticipant(msg) => {
                write!(f, "Invalid participant: {}", msg)
            }
        }
    }
}

impl std::error::Error for SuccessionError {}