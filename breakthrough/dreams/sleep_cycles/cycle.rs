// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::stage::SleepStage;
use crate::DreamsError;
use crate::DreamsResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SleepCycleType {
    NREM,
    REM,
    Intermediate,
}

impl std::fmt::Display for SleepCycleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SleepCycleType::NREM => write!(f, "NREM"),
            SleepCycleType::REM => write!(f, "REM"),
            SleepCycleType::Intermediate => write!(f, "Intermediate"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SleepCycle {
    pub cycle_id: u64,
    pub cycle_type: SleepCycleType,
    pub duration_secs: u64,
    pub stages: Vec<SleepStage>,
}

impl SleepCycle {
    pub fn new(cycle_id: u64, cycle_type: SleepCycleType, duration_secs: u64) -> Self {
        Self {
            cycle_id,
            cycle_type,
            duration_secs,
            stages: Vec::new(),
        }
    }

    pub fn add_stage(&mut self, stage: SleepStage) {
        self.stages.push(stage);
    }

    pub fn validate(&self) -> DreamsResult<()> {
        if self.duration_secs == 0 {
            return Err(DreamsError::InvalidCycle(
                "Cycle duration must be greater than zero".to_string(),
            ));
        }
        if self.stages.is_empty() {
            return Err(DreamsError::InvalidCycle(
                "Cycle must contain at least one stage".to_string(),
            ));
        }
        Ok(())
    }
}

impl std::fmt::Display for SleepCycle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SleepCycle(id={}, type={}, duration={}s, stages={})",
            self.cycle_id,
            self.cycle_type,
            self.duration_secs,
            self.stages.len()
        )
    }
}