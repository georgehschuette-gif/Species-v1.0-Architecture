// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::DreamsError;
use crate::DreamsResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplayType {
    Forward,
    Reverse,
    Random,
    Prioritized,
}

impl std::fmt::Display for ReplayType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReplayType::Forward => write!(f, "Forward"),
            ReplayType::Reverse => write!(f, "Reverse"),
            ReplayType::Random => write!(f, "Random"),
            ReplayType::Prioritized => write!(f, "Prioritized"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MemoryReplay {
    pub replay_id: u64,
    pub replay_type: ReplayType,
    pub memories: Vec<u64>,
    pub speed_factor: f64,
}

impl MemoryReplay {
    pub fn new(replay_id: u64, replay_type: ReplayType) -> Self {
        Self {
            replay_id,
            replay_type,
            memories: Vec::new(),
            speed_factor: 1.0,
        }
    }

    pub fn add_memory(&mut self, memory_id: u64) {
        self.memories.push(memory_id);
    }

    pub fn validate(&self) -> DreamsResult<()> {
        if self.memories.is_empty() {
            return Err(DreamsError::InvalidReplay(
                "Replay must contain at least one memory".to_string(),
            ));
        }
        if self.speed_factor <= 0.0 {
            return Err(DreamsError::InvalidReplay(
                "Speed factor must be positive".to_string(),
            ));
        }
        Ok(())
    }
}

impl std::fmt::Display for MemoryReplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MemoryReplay(id={}, type={}, memories={}, speed={:.2}x)",
            self.replay_id,
            self.replay_type,
            self.memories.len(),
            self.speed_factor
        )
    }
}