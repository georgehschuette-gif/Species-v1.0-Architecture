// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::DreamsError;
use crate::DreamsResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PruningTarget {
    WeakSynapses,
    RedundantPathways,
    NoisyConnections,
    OutdatedMemories,
}

impl std::fmt::Display for PruningTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PruningTarget::WeakSynapses => write!(f, "WeakSynapses"),
            PruningTarget::RedundantPathways => write!(f, "RedundantPathways"),
            PruningTarget::NoisyConnections => write!(f, "NoisyConnections"),
            PruningTarget::OutdatedMemories => write!(f, "OutdatedMemories"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SynapticPruning {
    pub pruning_id: u64,
    pub target: PruningTarget,
    pub connections_removed: u64,
    pub efficiency_gain: f64,
}

impl SynapticPruning {
    pub fn new(pruning_id: u64, target: PruningTarget) -> Self {
        Self {
            pruning_id,
            target,
            connections_removed: 0,
            efficiency_gain: 0.0,
        }
    }

    pub fn validate(&self) -> DreamsResult<()> {
        if self.efficiency_gain < 0.0 {
            return Err(DreamsError::InvalidPruning(
                "Efficiency gain cannot be negative".to_string(),
            ));
        }
        Ok(())
    }
}

impl std::fmt::Display for SynapticPruning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SynapticPruning(id={}, target={}, removed={}, gain={:.2})",
            self.pruning_id,
            self.target,
            self.connections_removed,
            self.efficiency_gain
        )
    }
}