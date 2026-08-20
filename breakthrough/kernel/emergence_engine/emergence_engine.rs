// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::{EventField, KernelError, KernelResult, Scheduler, TopologyEngine};

#[derive(Debug, Clone)]
pub struct EmergenceEngine {
    scheduler: Scheduler,
    event_field: EventField,
    topology: TopologyEngine,
    active: bool,
}

impl EmergenceEngine {
    pub fn new(
        scheduler: Scheduler,
        event_field: EventField,
        topology: TopologyEngine,
    ) -> Self {
        Self {
            scheduler,
            event_field,
            topology,
            active: false,
        }
    }

    pub fn activate(&mut self) -> KernelResult<()> {
        if self.active {
            return Err(KernelError::InvalidState(
                "engine is already active".to_string(),
            ));
        }
        self.active = true;
        Ok(())
    }

    pub fn step(&mut self, energy: f64) -> KernelResult<f64> {
        if !self.active {
            return Err(KernelError::InvalidState(
                "engine is not active".to_string(),
            ));
        }
        let _tick = self.scheduler.tick()?;
        self.event_field.propagate(energy)
    }
}

impl std::fmt::Display for EmergenceEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EmergenceEngine(active={}, topology_tick={})",
            self.active,
            self.topology.current_tick()
        )
    }
}
