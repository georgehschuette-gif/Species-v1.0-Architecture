// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::ImaginationError;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NarrativeStructure {
    Linear,
    Fragmented,
    Recursive,
    Metamorphic,
    Dissipative,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NarrativeScene {
    pub sequence: usize,
    pub description: String,
    pub emotional_charge: f64,
    pub symbolic_density: f64,
    pub transitions: Vec<usize>,
}

pub struct DreamCycle {
    pub name: String,
    pub scenes: Vec<NarrativeScene>,
    pub structure: NarrativeStructure,
    pub total_duration: usize,
    pub recall_clarity: f64,
    pub narrative_coherence: f64,
    pub active_scene: usize,
}

impl DreamCycle {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            scenes: Vec::new(),
            structure: NarrativeStructure::Fragmented,
            total_duration: 0,
            recall_clarity: 0.3,
            narrative_coherence: 0.5,
            active_scene: 0,
        }
    }

    pub fn name(&self) -> &str { &self.name }
    pub fn scenes(&self) -> &[NarrativeScene] { &self.scenes }
    pub fn structure(&self) -> NarrativeStructure { self.structure }
    pub fn total_duration(&self) -> usize { self.total_duration }
    pub fn recall_clarity(&self) -> f64 { self.recall_clarity }
    pub fn narrative_coherence(&self) -> f64 { self.narrative_coherence }
    pub fn active_scene(&self) -> usize { self.active_scene }

    pub fn add_scene(&mut self, description: impl Into<String>, emotional_charge: f64, symbolic_density: f64) -> Result<(), ImaginationError> {
        if !(0.0..=1.0).contains(&emotional_charge) {
            return Err(ImaginationError::OutOfRange { field: "emotional_charge".into(), value: emotional_charge, min: 0.0, max: 1.0 });
        }
        if !(0.0..=1.0).contains(&symbolic_density) {
            return Err(ImaginationError::OutOfRange { field: "symbolic_density".into(), value: symbolic_density, min: 0.0, max: 1.0 });
        }
        let sequence = self.scenes.len();
        let transitions = if !self.scenes.is_empty() { vec![sequence - 1] } else { Vec::new() };
        self.scenes.push(NarrativeScene { sequence, description: description.into(), emotional_charge, symbolic_density, transitions });
        if !self.scenes.is_empty() {
            self.scenes[sequence - 1].transitions.push(sequence);
        }
        self.total_duration += 1;
        self.recalculate_coherence();
        Ok(())
    }

    pub fn set_structure(&mut self, structure: NarrativeStructure) {
        self.structure = structure;
    }

    pub fn transition_to(&mut self, scene_index: usize) -> Result<(), ImaginationError> {
        if scene_index >= self.scenes.len() {
            return Err(ImaginationError::DimensionMismatch { expected: self.scenes.len(), actual: scene_index });
        }
        self.active_scene = scene_index;
        Ok(())
    }

    pub fn enhance_recall(&mut self, clarity: f64) -> Result<(), ImaginationError> {
        if !(0.0..=1.0).contains(&clarity) {
            return Err(ImaginationError::OutOfRange { field: "clarity".into(), value: clarity, min: 0.0, max: 1.0 });
        }
        self.recall_clarity = (self.recall_clarity + clarity * 0.5).clamp(0.0, 1.0);
        Ok(())
    }

    pub fn current_scene(&self) -> Option<&NarrativeScene> {
        self.scenes.get(self.active_scene)
    }

    pub fn scene_sequence(&self) -> Vec<usize> {
        self.scenes.iter().map(|s| s.sequence).collect()
    }

    pub fn validate(&self) -> Result<(), ImaginationError> {
        if self.scenes.is_empty() {
            return Err(ImaginationError::InvalidInput("dream cycle has no scenes".into()));
        }
        Ok(())
    }

    fn recalculate_coherence(&mut self) {
        if self.scenes.is_empty() {
            self.narrative_coherence = 0.0;
            return;
        }
        let avg_symbolic: f64 = self.scenes.iter().map(|s| s.symbolic_density).sum::<f64>() / self.scenes.len() as f64;
        let avg_emotional: f64 = self.scenes.iter().map(|s| s.emotional_charge.abs()).sum::<f64>() / self.scenes.len() as f64;
        self.narrative_coherence = ((avg_symbolic + avg_emotional) / 2.0).clamp(0.0, 1.0);
    }
}

impl fmt::Display for DreamCycle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DreamCycle(name='{}', scenes={}, structure={:?}, coherence={:.2})", self.name, self.scenes.len(), self.structure, self.narrative_coherence)
    }
}

impl fmt::Display for NarrativeStructure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Linear => write!(f, "Linear"),
            Self::Fragmented => write!(f, "Fragmented"),
            Self::Recursive => write!(f, "Recursive"),
            Self::Metamorphic => write!(f, "Metamorphic"),
            Self::Dissipative => write!(f, "Dissipative"),
        }
    }
}
