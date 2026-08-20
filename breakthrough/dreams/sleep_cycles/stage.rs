// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::DreamsError;
use crate::DreamsResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SleepStage {
    Awake,
    NREM1,
    NREM2,
    NREM3,
    REM,
}

impl std::fmt::Display for SleepStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SleepStage::Awake => write!(f, "Awake"),
            SleepStage::NREM1 => write!(f, "NREM1"),
            SleepStage::NREM2 => write!(f, "NREM2"),
            SleepStage::NREM3 => write!(f, "NREM3"),
            SleepStage::REM => write!(f, "REM"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StageMetrics {
    pub stage: SleepStage,
    pub duration_secs: u64,
    pub spindle_count: u64,
    pub eye_movement_density: f64,
}

impl StageMetrics {
    pub fn new(stage: SleepStage, duration_secs: u64) -> Self {
        Self {
            stage,
            duration_secs,
            spindle_count: 0,
            eye_movement_density: 0.0,
        }
    }

    pub fn validate(&self) -> DreamsResult<()> {
        if self.duration_secs == 0 {
            return Err(DreamsError::InvalidStage(
                "Stage duration must be greater than zero".to_string(),
            ));
        }
        if self.eye_movement_density < 0.0 || self.eye_movement_density > 1.0 {
            return Err(DreamsError::InvalidStage(
                "Eye movement density must be between 0.0 and 1.0".to_string(),
            ));
        }
        Ok(())
    }
}

impl std::fmt::Display for StageMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "StageMetrics(stage={}, duration={}s, spindles={}, density={:.2})",
            self.stage, self.duration_secs, self.spindle_count, self.eye_movement_density
        )
    }
}