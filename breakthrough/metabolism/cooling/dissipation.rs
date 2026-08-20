// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::MetabolismError;
use crate::cooling::Cooler;
use crate::energy_budget::EnergyBudget;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeatDissipation {
    pub cooler: Cooler,
    pub ambient_temp: u32,
    pub current_temp: u32,
    pub critical_temp: u32,
}

impl HeatDissipation {
    pub fn new(cooler: Cooler, ambient_temp: u32, critical_temp: u32) -> Self {
        Self {
            cooler,
            ambient_temp,
            current_temp: ambient_temp,
            critical_temp,
        }
    }

    pub fn dissipate(&mut self, heat_units: u64, budget: &mut EnergyBudget) -> Result<u64, MetabolismError> {
        let removed = self.cooler.thermal_headroom().min(heat_units);
        self.current_temp = self.current_temp.saturating_sub(removed as u32 / 10);
        self.cooler.update_load(removed);
        budget.replenish(0);
        Ok(removed)
    }

    pub fn is_critical(&self) -> bool {
        self.current_temp >= self.critical_temp
    }
}

