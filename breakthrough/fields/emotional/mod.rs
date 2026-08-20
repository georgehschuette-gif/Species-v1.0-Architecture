// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Emotional Field: Affective valence and arousal distributions.
//! Encodes mood, sentiment, emotional intensity, and affective tone
//! across cognitive agents and interactions.
//!
//! The emotional field module models the continuous dimensions of affect,
//! including valence (positive-negative) and arousal (activation level),
//! as well as mood states that persist over time with decay dynamics.

pub mod valence;
pub mod arousal;
pub mod mood;

pub use valence::EmotionalValence;
pub use arousal::EmotionalArousal;
pub use mood::EmotionalMood;

use std::fmt;

/// EmotionalField: An affective distribution container over a population.
#[derive(Debug, Clone, PartialEq)]
pub struct EmotionalField {
    pub name: String,
    pub valences: Vec<EmotionalValence>,
    pub arousals: Vec<EmotionalArousal>,
}

impl EmotionalField {
    pub fn new(name: String) -> Self {
        Self { name, valences: Vec::new(), arousals: Vec::new() }
    }

    pub fn add_valence(&mut self, valence: EmotionalValence) {
        self.valences.push(valence);
    }

    pub fn add_arousal(&mut self, arousal: EmotionalArousal) {
        self.arousals.push(arousal);
    }

    pub fn mean_valence(&self) -> Option<f64> {
        if self.valences.is_empty() {
            return None;
        }
        let sum: f64 = self.valences.iter().map(|v| v.0).sum();
        Some(sum / self.valences.len() as f64)
    }

    pub fn mean_arousal(&self) -> Option<f64> {
        if self.arousals.is_empty() {
            return None;
        }
        let sum: f64 = self.arousals.iter().map(|a| a.0).sum();
        Some(sum / self.arousals.len() as f64)
    }

    pub fn is_aroused(&self) -> bool {
        self.mean_arousal().unwrap_or(0.0) > 0.5
    }

    pub fn valence_distribution(&self) -> usize {
        self.valences.len()
    }
}

impl Default for EmotionalField {
    fn default() -> Self {
        Self::new(String::from("unnamed"))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EmotionalError {
    InvalidValue { value: f64 },
    InvalidStability { stability: f64 },
    InvalidDecay { decay: f64 },
    InsufficientData,
    ComputationError(String),
}

impl fmt::Display for EmotionalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EmotionalError::InvalidValue { value } => write!(f, "invalid emotional value: {}", value),
            EmotionalError::InvalidStability { stability } => write!(f, "invalid stability: {}", stability),
            EmotionalError::InvalidDecay { decay } => write!(f, "invalid decay rate: {}", decay),
            EmotionalError::InsufficientData => write!(f, "insufficient data for operation"),
            EmotionalError::ComputationError(msg) => write!(f, "computation error: {}", msg),
        }
    }
}

impl std::error::Error for EmotionalError {}

pub type EmotionalResult<T> = Result<T, EmotionalError>;
