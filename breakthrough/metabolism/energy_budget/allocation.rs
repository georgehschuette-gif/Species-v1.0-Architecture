// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::EnergyBudget;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Allocation {
    pub source: EnergyBudget,
    pub amount: u64,
}

impl Allocation {
    pub fn from_source(source: &EnergyBudget, amount: u64) -> Self {
        Self {
            source: source.clone(),
            amount,
        }
    }

    pub fn refund(mut self) -> EnergyBudget {
        self.source.available_joules += self.amount;
        self.source
    }
}

