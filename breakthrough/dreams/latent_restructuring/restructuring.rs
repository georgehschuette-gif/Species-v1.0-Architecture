// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::DreamsError;
use crate::DreamsResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestructuringMode {
    Hierarchical,
    Distributed,
    Modular,
    Integrated,
}

impl std::fmt::Display for RestructuringMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RestructuringMode::Hierarchical => write!(f, "Hierarchical"),
            RestructuringMode::Distributed => write!(f, "Distributed"),
            RestructuringMode::Modular => write!(f, "Modular"),
            RestructuringMode::Integrated => write!(f, "Integrated"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LatentRestructuring {
    pub restructuring_id: u64,
    pub mode: RestructuringMode,
    pub connections_modified: u64,
    pub stability_index: f64,
}

impl LatentRestructuring {
    pub fn new(restructuring_id: u64, mode: RestructuringMode) -> Self {
        Self {
            restructuring_id,
            mode,
            connections_modified: 0,
            stability_index: 0.0,
        }
    }

    pub fn validate(&self) -> DreamsResult<()> {
        if self.stability_index < 0.0 || self.stability_index > 1.0 {
            return Err(DreamsError::InvalidRestructuring(
                "Stability index must be between 0.0 and 1.0".to_string(),
            ));
        }
        Ok(())
    }
}

impl std::fmt::Display for LatentRestructuring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LatentRestructuring(id={}, mode={}, modified={}, stability={:.2})",
            self.restructuring_id,
            self.mode,
            self.connections_modified,
            self.stability_index
        )
    }
}