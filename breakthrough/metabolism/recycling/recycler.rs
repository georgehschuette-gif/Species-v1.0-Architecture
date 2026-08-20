// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::MetabolismError;
use crate::decay::DegradationModel;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceRecycler {
    pub input_capacity: u64,
    pub recovered: u64,
    pub loss_ratio: u8,
}

impl ResourceRecycler {
    pub fn new(input_capacity: u64, loss_ratio: u8) -> Self {
        Self {
            input_capacity,
            recovered: 0,
            loss_ratio,
        }
    }

    pub fn process(&mut self, amount: u64, degradation: &DegradationModel) -> Result<u64, MetabolismError> {
        if amount > self.input_capacity {
            return Err(MetabolismError::Configuration(
                "Exceeds input capacity".into(),
            ));
        }
        let loss = (amount * self.loss_ratio as u64) / 100;
        let recovered = amount - loss;
        let efficiency_factor = degradation.current_factor() as u64;
        let final_recovered = (recovered * efficiency_factor) / 100;
        self.recovered += final_recovered;
        Ok(final_recovered)
    }

    pub fn throughput(&self) -> f64 {
        if self.input_capacity == 0 {
            0.0
        } else {
            (self.recovered as f64) / (self.input_capacity as f64)
        }
    }
}

