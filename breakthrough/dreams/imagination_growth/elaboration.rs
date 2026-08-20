// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::DreamsError;
use crate::DreamsResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElaborationStep {
    ConceptualBlending,
    MetaphorGeneration,
    ScenarioConstruction,
    EmotionalAmplification,
    DetailInjection,
}

impl std::fmt::Display for ElaborationStep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElaborationStep::ConceptualBlending => write!(f, "ConceptualBlending"),
            ElaborationStep::MetaphorGeneration => write!(f, "MetaphorGeneration"),
            ElaborationStep::ScenarioConstruction => write!(f, "ScenarioConstruction"),
            ElaborationStep::EmotionalAmplification => write!(f, "EmotionalAmplification"),
            ElaborationStep::DetailInjection => write!(f, "DetailInjection"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ElaborationProcess {
    pub process_id: u64,
    pub steps: Vec<ElaborationStep>,
    pub richness: f64,
}

impl ElaborationProcess {
    pub fn new(process_id: u64) -> Self {
        Self {
            process_id,
            steps: Vec::new(),
            richness: 0.0,
        }
    }

    pub fn add_step(&mut self, step: ElaborationStep) {
        self.steps.push(step);
    }

    pub fn validate(&self) -> DreamsResult<()> {
        if self.steps.is_empty() {
            return Err(DreamsError::InvalidGrowth(
                "Elaboration process must contain at least one step".to_string(),
            ));
        }
        if self.richness < 0.0 || self.richness > 1.0 {
            return Err(DreamsError::InvalidGrowth(
                "Richness must be between 0.0 and 1.0".to_string(),
            ));
        }
        Ok(())
    }
}

impl std::fmt::Display for ElaborationProcess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ElaborationProcess(id={}, steps={}, richness={:.2})",
            self.process_id,
            self.steps.len(),
            self.richness
        )
    }
}