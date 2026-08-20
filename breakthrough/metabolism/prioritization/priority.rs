// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::MetabolismError;
use crate::cooling::Cooler;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RequestPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrioritizedRequest {
    pub id: u64,
    pub priority: RequestPriority,
    pub energy_cost: u64,
    pub cooling_load: u64,
}

impl PrioritizedRequest {
    pub fn new(
        id: u64,
        priority: RequestPriority,
        energy_cost: u64,
        cooling_load: u64,
    ) -> Self {
        Self {
            id,
            priority,
            energy_cost,
            cooling_load,
        }
    }

    pub fn validate(&self) -> Result<(), MetabolismError> {
        if self.energy_cost == 0 && self.priority != RequestPriority::Low {
            return Err(MetabolismError::Configuration(
                "Non-low priority requires non-zero energy cost".into(),
            ));
        }
        Ok(())
    }
}

