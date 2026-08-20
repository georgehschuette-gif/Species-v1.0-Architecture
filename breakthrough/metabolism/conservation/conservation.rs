// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::MetabolismError;
use crate::energy_budget::Allocation;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyLevel {
    Strict,
    Moderate,
    Lenient,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConservationPolicy {
    pub level: PolicyLevel,
    pub min_efficiency: u8,
    pub max_waste_percent: u8,
}

impl ConservationPolicy {
    pub fn new(level: PolicyLevel, min_efficiency: u8, max_waste_percent: u8) -> Self {
        Self {
            level,
            min_efficiency,
            max_waste_percent,
        }
    }

    pub fn evaluate(&self, allocation: &Allocation) -> Result<bool, MetabolismError> {
        let waste = allocation.amount.saturating_sub(allocation.source.available_joules);
        if waste as u8 > self.max_waste_percent {
            return Err(MetabolismError::ConservationViolation);
        }
        Ok(true)
    }
}

