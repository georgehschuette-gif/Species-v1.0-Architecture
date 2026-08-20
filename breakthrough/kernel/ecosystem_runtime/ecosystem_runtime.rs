// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::{CoherenceEngine, EventField, KernelError, KernelResult, TopologyEngine};

#[derive(Debug, Clone)]
pub struct EcosystemRuntime {
    event_field: EventField,
    coherence: CoherenceEngine,
    topology: TopologyEngine,
    tick_count: u64,
}

impl EcosystemRuntime {
    pub fn new(
        event_field: EventField,
        coherence: CoherenceEngine,
        topology: TopologyEngine,
    ) -> Self {
        Self {
            event_field,
            coherence,
            topology,
            tick_count: 0,
        }
    }

    pub fn run(&mut self, energy: f64) -> KernelResult<f64> {
        if energy <= 0.0 {
            return Err(KernelError::ValidationFailure(
                "energy input must be positive".to_string(),
            ));
        }
        let output = self.event_field.propagate(energy)?;
        self.coherence.align(output * 0.01)?;
        self.tick_count += 1;
        Ok(output)
    }

    pub fn tick_count(&self) -> u64 {
        self.tick_count
    }
}

impl std::fmt::Display for EcosystemRuntime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EcosystemRuntime(tick={}, coherence={})",
            self.tick_count,
            self.coherence.measure()
        )
    }
}
