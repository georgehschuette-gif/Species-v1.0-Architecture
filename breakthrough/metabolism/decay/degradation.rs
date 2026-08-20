// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::MetabolismError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DegradationModel {
    pub cycles: u32,
    pub loss_per_cycle: u8,
    pub current_degradation: u8,
}

impl DegradationModel {
    pub fn new(cycles: u32, loss_per_cycle: u8) -> Self {
        Self {
            cycles,
            loss_per_cycle,
            current_degradation: 0,
        }
    }

    pub fn apply_cycle(&mut self) -> Result<(), MetabolismError> {
        if self.current_degradation >= 100 {
            return Err(MetabolismError::DegradationFatal);
        }
        self.current_degradation = (self.current_degradation + self.loss_per_cycle).min(100);
        Ok(())
    }

    pub fn current_factor(&self) -> u64 {
        (100 - self.current_degradation).into()
    }

    pub fn remaining_life(&self) -> u32 {
        if self.loss_per_cycle == 0 {
            self.cycles
        } else {
            (self.cycles as u8 / self.loss_per_cycle).into()
        }
    }
}

