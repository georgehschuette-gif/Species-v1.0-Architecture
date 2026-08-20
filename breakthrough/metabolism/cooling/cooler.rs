// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::MetabolismError;
use crate::energy_budget::EnergyBudget;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoolingMode {
    Passive,
    Active { power_draw: u64 },
    Emergency,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThermalState {
    Nominal,
    Warm,
    Hot,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cooler {
    pub mode: CoolingMode,
    pub max_dissipation: u64,
    pub current_load: u64,
}

impl Cooler {
    pub fn new(max_dissipation: u64) -> Self {
        Self {
            mode: CoolingMode::Passive,
            max_dissipation,
            current_load: 0,
        }
    }

    pub fn engage(&mut self, mode: CoolingMode, budget: &mut EnergyBudget) -> Result<(), MetabolismError> {
        if let CoolingMode::Active { power_draw } = &mode {
            budget.allocate(*power_draw)?;
        }
        self.mode = mode;
        Ok(())
    }

    pub fn thermal_headroom(&self) -> u64 {
        self.max_dissipation.saturating_sub(self.current_load)
    }

    pub fn update_load(&mut self, load: u64) {
        self.current_load = load.min(self.max_dissipation);
    }
}

impl From<u64> for ThermalState {
    fn from(load: u64) -> Self {
        match load {
            0..=25 => ThermalState::Nominal,
            26..=50 => ThermalState::Warm,
            51..=75 => ThermalState::Hot,
            _ => ThermalState::Critical,
        }
    }
}

