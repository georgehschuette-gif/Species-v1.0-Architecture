// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::MetabolismError;
use super::allocation::Allocation;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnergyBudget {
    pub total_joules: u64,
    pub available_joules: u64,
    pub max_burst: u64,
}

impl EnergyBudget {
    pub fn new(total: u64, max_burst: u64) -> Self {
        Self {
            total_joules: total,
            available_joules: total,
            max_burst,
        }
    }

    pub fn allocate(&mut self, amount: u64) -> Result<Allocation, MetabolismError> {
        if amount > self.available_joules {
            return Err(MetabolismError::BudgetExhausted);
        }
        if amount > self.max_burst {
            return Err(MetabolismError::Configuration(format!(
                "Burst {} exceeds max_burst {}",
                amount, self.max_burst
            )));
        }
        self.available_joules -= amount;
        Ok(Allocation {
            source: self.clone(),
            amount,
        })
    }

    pub fn replenish(&mut self, amount: u64) {
        self.available_joules = (self.available_joules + amount).min(self.total_joules);
    }

    pub fn is_depleted(&self) -> bool {
        self.available_joules == 0
    }
}

