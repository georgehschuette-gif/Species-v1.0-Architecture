// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::ImaginationError;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MemoryVividness {
    Hyperreal,
    Vivid,
    Clear,
    Fuzzy,
    Faint,
    Dissolved,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SyntheticMemory {
    pub content: String,
    pub vividness: MemoryVividness,
    pub age: usize,
    pub emotional_valence: f64,
    pub source_memories: Vec<usize>,
    pub fabrication_depth: usize,
}

impl SyntheticMemory {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            vividness: MemoryVividness::Clear,
            age: 0,
            emotional_valence: 0.0,
            source_memories: Vec::new(),
            fabrication_depth: 1,
        }
    }

    pub fn with_vividness(mut self, vividness: MemoryVividness) -> Self {
        self.vividness = vividness;
        self
    }

    pub fn with_emotional_valence(mut self, valence: f64) -> Self {
        self.emotional_valence = valence.clamp(-1.0, 1.0);
        self
    }

    pub fn link_source(&mut self, source_id: usize) {
        if !self.source_memories.contains(&source_id) {
            self.source_memories.push(source_id);
        }
    }

    pub fn age_by(&mut self, cycles: usize) {
        self.age += cycles;
        if self.age > MAX_MEMORY_AGE / 2 {
            self.vividness = match self.vividness {
                MemoryVividness::Hyperreal => MemoryVividness::Vivid,
                MemoryVividness::Vivid => MemoryVividness::Clear,
                MemoryVividness::Clear => MemoryVividness::Fuzzy,
                MemoryVividness::Fuzzy => MemoryVividness::Faint,
                MemoryVividness::Faint => MemoryVividness::Dissolved,
                MemoryVividness::Dissolved => MemoryVividness::Dissolved,
            };
        }
    }

    pub fn is_dissolved(&self) -> bool {
        matches!(self.vividness, MemoryVividness::Dissolved) || self.age > MAX_MEMORY_AGE
    }

    pub fn recall_strength(&self) -> f64 {
        let vivid_factor = match self.vividness {
            MemoryVividness::Hyperreal => 1.0,
            MemoryVividness::Vivid => 0.8,
            MemoryVividness::Clear => 0.6,
            MemoryVividness::Fuzzy => 0.4,
            MemoryVividness::Faint => 0.2,
            MemoryVividness::Dissolved => 0.0,
        };
        let age_factor = (1.0 - (self.age as f64 / MAX_MEMORY_AGE as f64)).max(0.0);
        (vivid_factor * age_factor).clamp(0.0, 1.0)
    }

    pub fn validate(&self) -> Result<(), ImaginationError> {
        if self.content.is_empty() {
            return Err(ImaginationError::InvalidInput(
                "memory content must not be empty".into(),
            ));
        }
        if !(-1.0..=1.0).contains(&self.emotional_valence) {
            return Err(ImaginationError::OutOfRange {
                field: "emotional_valence".into(),
                value: self.emotional_valence,
                min: -1.0,
                max: 1.0,
            });
        }
        Ok(())
    }
}

impl fmt::Display for SyntheticMemory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SyntheticMemory(vividness={:?}, age={}, strength={:.2})",
            self.vividness,
            self.age,
            self.recall_strength()
        )
    }
}

impl fmt::Display for MemoryVividness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hyperreal => write!(f, "Hyperreal"),
            Self::Vivid => write!(f, "Vivid"),
            Self::Clear => write!(f, "Clear"),
            Self::Fuzzy => write!(f, "Fuzzy"),
            Self::Faint => write!(f, "Faint"),
            Self::Dissolved => write!(f, "Dissolved"),
        }
    }
}