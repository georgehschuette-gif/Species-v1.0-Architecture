// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::DreamsError;
use crate::DreamsResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DreamReplayType {
    Fragmented,
    Narrative,
    Abstract,
    Emotional,
}

impl std::fmt::Display for DreamReplayType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DreamReplayType::Fragmented => write!(f, "Fragmented"),
            DreamReplayType::Narrative => write!(f, "Narrative"),
            DreamReplayType::Abstract => write!(f, "Abstract"),
            DreamReplayType::Emotional => write!(f, "Emotional"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpontaneousReplay {
    pub replay_id: u64,
    pub replay_type: DreamReplayType,
    pub intensity: f64,
    pub duration_secs: u64,
}

impl SpontaneousReplay {
    pub fn new(replay_id: u64, replay_type: DreamReplayType, intensity: f64) -> Self {
        Self {
            replay_id,
            replay_type,
            intensity,
            duration_secs: 0,
        }
    }

    pub fn validate(&self) -> DreamsResult<()> {
        if self.intensity < 0.0 || self.intensity > 1.0 {
            return Err(DreamsError::InvalidReplay(
                "Replay intensity must be between 0.0 and 1.0".to_string(),
            ));
        }
        Ok(())
    }
}

impl std::fmt::Display for SpontaneousReplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SpontaneousReplay(id={}, type={}, intensity={:.2})",
            self.replay_id, self.replay_type, self.intensity
        )
    }
}