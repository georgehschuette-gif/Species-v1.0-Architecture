// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::MetabolismError;
use crate::prioritization::{PrioritizedRequest, RequestPriority};
use crate::cooling::ThermalState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PriorityScheduler {
    pub queue: Vec<PrioritizedRequest>,
    pub current_load: u64,
    pub max_load: u64,
}

impl PriorityScheduler {
    pub fn new(max_load: u64) -> Self {
        Self {
            queue: Vec::new(),
            current_load: 0,
            max_load,
        }
    }

    pub fn enqueue(&mut self, request: PrioritizedRequest) -> Result<(), MetabolismError> {
        request.validate()?;
        let thermal = ThermalState::from(self.current_load + request.cooling_load);
        if thermal == ThermalState::Critical {
            return Err(MetabolismError::ThermalLimitExceeded);
        }
        self.queue.push(request);
        self.queue.sort_by(|a, b| b.priority.cmp(&a.priority));
        Ok(())
    }

    pub fn next(&mut self) -> Option<PrioritizedRequest> {
        if self.queue.is_empty() {
            None
        } else {
            let req = self.queue.remove(0);
            self.current_load += req.cooling_load;
            Some(req)
        }
    }

    pub fn available_capacity(&self) -> u64 {
        self.max_load.saturating_sub(self.current_load)
    }
}

