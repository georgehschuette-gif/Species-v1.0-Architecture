// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Archives: A repository of cognitive phenomena that have been preserved,
//! lost, or abandoned across evolutionary and developmental time.

use std::fmt;

pub mod extinct_species;
pub mod abandoned_hypotheses;
pub mod historical_states;
pub mod fossils;
pub mod timelines;

pub use extinct_species::{ExtinctSpecies, ExtinctSpeciesError};
pub use abandoned_hypotheses::{AbandonedHypothesis, AbandonedHypothesisError};
pub use historical_states::{HistoricalState, HistoricalStateError};
pub use fossils::{FossilRecord, FossilError};
pub use timelines::{Timeline, TimelineError};

#[derive(Debug, Clone, PartialEq)]
pub enum ArchivesError {
    ExtinctSpecies(ExtinctSpeciesError),
    AbandonedHypothesis(AbandonedHypothesisError),
    HistoricalState(HistoricalStateError),
    Fossil(FossilError),
    Timeline(TimelineError),
    CorruptedArchive,
}

impl fmt::Display for ArchivesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExtinctSpecies(e) => write!(f, "extinct species error: {}", e),
            Self::AbandonedHypothesis(e) => write!(f, "abandoned hypothesis error: {}", e),
            Self::HistoricalState(e) => write!(f, "historical state error: {}", e),
            Self::Fossil(e) => write!(f, "fossil error: {}", e),
            Self::Timeline(e) => write!(f, "timeline error: {}", e),
            Self::CorruptedArchive => write!(f, "archive data is corrupted"),
        }
    }
}

impl std::error::Error for ArchivesError {}

pub type ArchivesResult<T> = Result<T, ArchivesError>;
