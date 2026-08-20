// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Imaginative Field: Creative and generative cognitive distributions.
//! Encodes novelty, fictionality, abstraction distance, and creative
//! divergence for generating alternative cognitive content.
//!
//! The imaginative field module provides tools for measuring how novel,
//! fictional, or divergent a piece of cognitive content is, supporting
//! creative exploration and alternative perspective generation.

pub mod novelty;
pub mod fictionality;
pub mod divergence;

pub use novelty::ImaginativeNovelty;
pub use fictionality::FictionalityScore;
pub use divergence::CreativeDivergence;

use std::fmt;

/// ImaginativeField: A container for creative and generative cognitive content.
#[derive(Debug, Clone, PartialEq)]
pub struct ImaginativeField {
    pub name: String,
    pub novelties: Vec<ImaginativeNovelty>,
    pub fictionality_scores: Vec<FictionalityScore>,
}

impl ImaginativeField {
    pub fn new(name: String) -> Self {
        Self { name, novelties: Vec::new(), fictionality_scores: Vec::new() }
    }

    pub fn add_novelty(&mut self, novelty: ImaginativeNovelty) {
        self.novelties.push(novelty);
    }

    pub fn add_fictionality(&mut self, score: FictionalityScore) {
        self.fictionality_scores.push(score);
    }

    pub fn novelty_count(&self) -> usize {
        self.novelties.len()
    }

    pub fn fictionality_count(&self) -> usize {
        self.fictionality_scores.len()
    }

    pub fn avg_novelty(&self) -> Option<f64> {
        if self.novelties.is_empty() {
            return None;
        }
        let sum: f64 = self.novelties.iter().map(|n| n.novelty_score).sum();
        Some(sum / self.novelties.len() as f64)
    }

    pub fn avg_fictionality(&self) -> Option<f64> {
        if self.fictionality_scores.is_empty() {
            return None;
        }
        let sum: f64 = self.fictionality_scores.iter().map(|f| f.fictionality).sum();
        Some(sum / self.fictionality_scores.len() as f64)
    }

    pub fn is_creative(&self) -> bool {
        self.avg_novelty().unwrap_or(0.0) > 0.6 && self.avg_fictionality().unwrap_or(0.0) < 0.5
    }
}

impl Default for ImaginativeField {
    fn default() -> Self {
        Self::new(String::from("unnamed"))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ImaginativeError {
    InvalidContent { content_id: u64 },
    InvalidScore { score: f64 },
    InvalidDistance { distance: f64 },
    InsufficientData,
    ComputationError(String),
}

impl fmt::Display for ImaginativeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImaginativeError::InvalidContent { content_id } => write!(f, "invalid content id: {}", content_id),
            ImaginativeError::InvalidScore { score } => write!(f, "invalid score: {}", score),
            ImaginativeError::InvalidDistance { distance } => write!(f, "invalid distance: {}", distance),
            ImaginativeError::InsufficientData => write!(f, "insufficient data for operation"),
            ImaginativeError::ComputationError(msg) => write!(f, "computation error: {}", msg),
        }
    }
}

impl std::error::Error for ImaginativeError {}

pub type ImaginativeResult<T> = Result<T, ImaginativeError>;
