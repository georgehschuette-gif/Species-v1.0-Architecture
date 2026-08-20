// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::DreamsError;
use crate::DreamsResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConsolidationType {
    HippocampalToCortex,
    CorticalToHippocampal,
    Synaptic,
    Systems,
}

impl std::fmt::Display for ConsolidationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConsolidationType::HippocampalToCortex => write!(f, "HippocampalToCortex"),
            ConsolidationType::CorticalToHippocampal => write!(f, "CorticalToHippocampal"),
            ConsolidationType::Synaptic => write!(f, "Synaptic"),
            ConsolidationType::Systems => write!(f, "Systems"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MemoryConsolidation {
    pub consolidation_id: u64,
    pub consolidation_type: ConsolidationType,
    pub source_memories: Vec<u64>,
    pub target_region: String,
    pub strength: f64,
}

impl MemoryConsolidation {
    pub fn new(
        consolidation_id: u64,
        consolidation_type: ConsolidationType,
        target_region: impl Into<String>,
    ) -> Self {
        Self {
            consolidation_id,
            consolidation_type,
            source_memories: Vec::new(),
            target_region: target_region.into(),
            strength: 0.0,
        }
    }

    pub fn add_source(&mut self, memory_id: u64) {
        self.source_memories.push(memory_id);
    }

    pub fn validate(&self) -> DreamsResult<()> {
        if self.source_memories.is_empty() {
            return Err(DreamsError::InvalidConsolidation(
                "Consolidation must have at least one source memory".to_string(),
            ));
        }
        if self.strength < 0.0 || self.strength > 1.0 {
            return Err(DreamsError::InvalidConsolidation(
                "Consolidation strength must be between 0.0 and 1.0".to_string(),
            ));
        }
        Ok(())
    }
}

impl std::fmt::Display for MemoryConsolidation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MemoryConsolidation(id={}, type={}, target={}, strength={:.2})",
            self.consolidation_id,
            self.consolidation_type,
            self.target_region,
            self.strength
        )
    }
}