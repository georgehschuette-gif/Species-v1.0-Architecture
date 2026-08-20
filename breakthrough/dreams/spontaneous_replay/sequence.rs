// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::DreamsError;
use crate::DreamsResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SequenceStep {
    MemoryRetrieval,
    Association,
    Abstraction,
    Synthesis,
}

impl std::fmt::Display for SequenceStep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SequenceStep::MemoryRetrieval => write!(f, "MemoryRetrieval"),
            SequenceStep::Association => write!(f, "Association"),
            SequenceStep::Abstraction => write!(f, "Abstraction"),
            SequenceStep::Synthesis => write!(f, "Synthesis"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReplaySequence {
    pub sequence_id: u64,
    pub steps: Vec<SequenceStep>,
    pub coherence_score: f64,
}

impl ReplaySequence {
    pub fn new(sequence_id: u64) -> Self {
        Self {
            sequence_id,
            steps: Vec::new(),
            coherence_score: 0.0,
        }
    }

    pub fn add_step(&mut self, step: SequenceStep) {
        self.steps.push(step);
    }

    pub fn validate(&self) -> DreamsResult<()> {
        if self.steps.is_empty() {
            return Err(DreamsError::InvalidReplay(
                "Replay sequence must contain at least one step".to_string(),
            ));
        }
        if self.coherence_score < 0.0 || self.coherence_score > 1.0 {
            return Err(DreamsError::InvalidReplay(
                "Coherence score must be between 0.0 and 1.0".to_string(),
            ));
        }
        Ok(())
    }
}

impl std::fmt::Display for ReplaySequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ReplaySequence(id={}, steps={}, coherence={:.2})",
            self.sequence_id,
            self.steps.len(),
            self.coherence_score
        )
    }
}