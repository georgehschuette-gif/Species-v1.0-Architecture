// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::MetabolismError;
use crate::energy_budget::EnergyBudget;
use crate::recycling::ResourceRecycler;
use crate::decay::DegradationModel;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReusePolicy {
    pub max_reuse_cycles: u32,
    pub degredation_per_cycle: u8,
}

impl ReusePolicy {
    pub fn new(max_reuse_cycles: u32, degredation_per_cycle: u8) -> Self {
        Self {
            max_reuse_cycles,
            degredation_per_cycle,
        }
    }

    pub fn recycle(&self, recycler: &mut ResourceRecycler, amount: u64) -> Result<EnergyBudget, MetabolismError> {
        let recovered = recycler.process(
            amount,
            &DegradationModel::new(self.max_reuse_cycles, self.degredation_per_cycle),
        )?;
        Ok(EnergyBudget::new(recovered, recovered))
    }
}

