// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::ImaginationError;

pub mod state;
pub mod narrative;

pub use state::{DreamState, DreamPhase};
pub use narrative::DreamCycle;

pub const DEFAULT_DREAM_DURATION: usize = 90;
pub const MAX_DREAM_DEPTH: usize = 12;

pub fn create_dream_cycle(name: impl Into<String>) -> DreamCycle {
    DreamCycle::new(name)
}

pub fn create_dream_state(phase: DreamPhase) -> DreamState {
    DreamState::new(phase)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dream_cycle_creation() {
        let d = create_dream_cycle("Night Journey");
        assert_eq!(d.name(), "Night Journey");
    }
}
