// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::DreamsError;
use crate::DreamsResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GrowthPhase {
    Seed,
    Germination,
    Branching,
    Flourishing,
    Dormant,
}

impl std::fmt::Display for GrowthPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GrowthPhase::Seed => write!(f, "Seed"),
            GrowthPhase::Germination => write!(f, "Germination"),
            GrowthPhase::Branching => write!(f, "Branching"),
            GrowthPhase::Flourishing => write!(f, "Flourishing"),
            GrowthPhase::Dormant => write!(f, "Dormant"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ImaginationGrowth {
    pub growth_id: u64,
    pub current_phase: GrowthPhase,
    pub novelty_score: f64,
    pub connectivity: u64,
}

impl ImaginationGrowth {
    pub fn new(growth_id: u64, current_phase: GrowthPhase, novelty_score: f64) -> Self {
        Self {
            growth_id,
            current_phase,
            novelty_score,
            connectivity: 0,
        }
    }

    pub fn advance_phase(&mut self) -> DreamsResult<()> {
        self.current_phase = match self.current_phase {
            GrowthPhase::Seed => GrowthPhase::Germination,
            GrowthPhase::Germination => GrowthPhase::Branching,
            GrowthPhase::Branching => GrowthPhase::Flourishing,
            GrowthPhase::Flourishing => GrowthPhase::Dormant,
            GrowthPhase::Dormant => GrowthPhase::Seed,
        };
        Ok(())
    }

    pub fn validate(&self) -> DreamsResult<()> {
        if self.novelty_score < 0.0 || self.novelty_score > 1.0 {
            return Err(DreamsError::InvalidGrowth(
                "Novelty score must be between 0.0 and 1.0".to_string(),
            ));
        }
        Ok(())
    }
}

impl std::fmt::Display for ImaginationGrowth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ImaginationGrowth(id={}, phase={}, novelty={:.2}, connectivity={})",
            self.growth_id, self.current_phase, self.novelty_score, self.connectivity
        )
    }
}