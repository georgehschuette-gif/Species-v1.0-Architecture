// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

pub mod sleep_cycles;
pub mod consolidation;
pub mod spontaneous_replay;
pub mod imagination_growth;
pub mod pruning;
pub mod latent_restructuring;

pub use sleep_cycles::{SleepCycle, SleepCycleType, SleepStage, StageMetrics};
pub use consolidation::{MemoryConsolidation, ConsolidationType, MemoryReplay, ReplayType};
pub use spontaneous_replay::{SpontaneousReplay, DreamReplayType, ReplaySequence, SequenceStep};
pub use imagination_growth::{ImaginationGrowth, GrowthPhase, ElaborationProcess, ElaborationStep};
pub use pruning::{SynapticPruning, PruningTarget, ExperienceSelection, SelectionCriterion};
pub use latent_restructuring::{LatentRestructuring, RestructuringMode, KnowledgeIntegration, IntegrationMethod};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DreamsError {
    InvalidCycle(String),
    InvalidStage(String),
    InvalidConsolidation(String),
    InvalidReplay(String),
    InvalidGrowth(String),
    InvalidPruning(String),
    InvalidRestructuring(String),
    ConfigurationError(String),
}

impl std::fmt::Display for DreamsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DreamsError::InvalidCycle(msg) => write!(f, "Invalid sleep cycle: {}", msg),
            DreamsError::InvalidStage(msg) => write!(f, "Invalid sleep stage: {}", msg),
            DreamsError::InvalidConsolidation(msg) => write!(f, "Invalid consolidation: {}", msg),
            DreamsError::InvalidReplay(msg) => write!(f, "Invalid replay: {}", msg),
            DreamsError::InvalidGrowth(msg) => write!(f, "Invalid growth: {}", msg),
            DreamsError::InvalidPruning(msg) => write!(f, "Invalid pruning: {}", msg),
            DreamsError::InvalidRestructuring(msg) => write!(f, "Invalid restructuring: {}", msg),
            DreamsError::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
        }
    }
}

impl std::error::Error for DreamsError {}

pub type DreamsResult<T> = Result<T, DreamsError>;