// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::{CoherenceEngine, KernelError, KernelResult, TopologyEngine};

#[derive(Debug, Clone)]
pub struct Scheduler {
    topology: TopologyEngine,
    coherence: CoherenceEngine,
    max_ticks: u64,
}

impl Scheduler {
    pub fn new(topology: TopologyEngine, coherence: CoherenceEngine, max_ticks: u64) -> Self {
        Self {
            topology,
            coherence,
            max_ticks,
        }
    }

    pub fn tick(&mut self) -> KernelResult<u64> {
        let current = self.topology.current_tick();
        if current >= self.max_ticks {
            return Err(KernelError::ThresholdNotMet {
                threshold: self.max_ticks,
                actual: current,
            });
        }
        self.topology.advance_tick();
        let coherence = self.coherence.measure();
        if coherence < 0.5 {
            return Err(KernelError::InvariantViolation(format!(
                "coherence dropped to {}",
                coherence
            )));
        }
        Ok(current + 1)
    }

    pub fn topology(&self) -> &TopologyEngine {
        &self.topology
    }
}

impl std::fmt::Display for Scheduler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Scheduler(tick={}, max={})",
            self.topology.current_tick(),
            self.max_ticks
        )
    }
}
