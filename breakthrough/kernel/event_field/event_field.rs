// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::{EvolutionEngine, KernelError, KernelResult, ResonanceEngine};

#[derive(Debug, Clone)]
pub struct EventField {
    resonance: ResonanceEngine,
    evolution: EvolutionEngine,
    events: u64,
}

impl EventField {
    pub fn new(resonance: ResonanceEngine, evolution: EvolutionEngine) -> Self {
        Self {
            resonance,
            evolution,
            events: 0,
        }
    }

    pub fn propagate(&mut self, energy: f64) -> KernelResult<f64> {
        if energy <= 0.0 {
            return Err(KernelError::ValidationFailure(
                "energy must be positive".to_string(),
            ));
        }
        let resonated = self.resonance.resonate(energy);
        let evolved = self.evolution.mutate(resonated);
        self.events += 1;
        Ok(evolved)
    }
}

impl std::fmt::Display for EventField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EventField(events={})", self.events)
    }
}
