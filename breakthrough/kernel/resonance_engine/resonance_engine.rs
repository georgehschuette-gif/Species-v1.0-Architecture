// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::{CoherenceEngine, KernelError, TopologyEngine};

#[derive(Debug, Clone)]
pub struct ResonanceEngine {
    topology: TopologyEngine,
    coherence: CoherenceEngine,
    frequency: f64,
}

impl ResonanceEngine {
    pub fn new(topology: TopologyEngine, coherence: CoherenceEngine, frequency: f64) -> Self {
        Self {
            topology,
            coherence,
            frequency,
        }
    }

    pub fn resonate(&self, energy: f64) -> f64 {
        let amplitude = energy * self.frequency;
        let decay = self.coherence.measure();
        amplitude * (1.0 - decay).max(0.1)
    }

    pub fn frequency(&self) -> f64 {
        self.frequency
    }
}

impl std::fmt::Display for ResonanceEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ResonanceEngine(freq={}, coherence={})",
            self.frequency,
            self.coherence.measure()
        )
    }
}
