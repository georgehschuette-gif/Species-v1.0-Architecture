// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::{KernelError, KernelResult, TopologyEngine};

#[derive(Debug, Clone)]
pub struct CoherenceEngine {
    topology: TopologyEngine,
    target: f64,
    current: f64,
}

impl CoherenceEngine {
    pub fn new(topology: TopologyEngine, target: f64) -> Self {
        Self {
            topology,
            target,
            current: target,
        }
    }

    pub fn measure(&self) -> f64 {
        self.current
    }

    pub fn align(&mut self, delta: f64) -> KernelResult<f64> {
        if delta < -1.0 || delta > 1.0 {
            return Err(KernelError::OutOfRange {
                field: "delta".to_string(),
                value: delta,
                min: -1.0,
                max: 1.0,
            });
        }
        self.current = (self.current + delta).clamp(0.0, 1.0);
        Ok(self.current)
    }

    pub fn target(&self) -> f64 {
        self.target
    }
}

impl std::fmt::Display for CoherenceEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CoherenceEngine(current={}, target={})",
            self.current, self.target
        )
    }
}
