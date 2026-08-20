// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::ImaginationError;

pub mod engine;
pub mod run;

pub use engine::SimulationEngine;
pub use run::SimulationRun;

pub const DEFAULT_SIMULATION_STEPS: usize = 1000;
pub const MAX_SIMULATION_DEPTH: usize = 100;

pub fn create_simulation_engine(seed: u64) -> SimulationEngine {
    SimulationEngine::new(seed)
}

pub fn create_simulation_run(steps: usize) -> Result<SimulationRun, ImaginationError> {
    SimulationRun::new(steps)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simulation_engine_creation() {
        let engine = create_simulation_engine(42);
        assert_eq!(engine.seed(), 42);
    }
}
