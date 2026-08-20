// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::MorphogenesisError;

use super::*;

pub struct StabilizationEquilibrium {
    pub state: EquilibriumState,
    pub energy: f64,
    pub entropy: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EquilibriumState {
    Unstable,
    Metastable,
    Stable,
}

impl StabilizationEquilibrium {
    pub fn new() -> Self {
        Self { state: EquilibriumState::Unstable, energy: 1.0, entropy: 1.0 }
    }

    pub fn relax(&mut self, rate: f64) -> Result<(), MorphogenesisError> {
        if !(0.0..=1.0).contains(&rate) { return Err(MorphogenesisError::OutOfRange { field: "rate".into(), value: rate, min: 0.0, max: 1.0 }); }
        self.energy = (self.energy * (1.0 - rate)).max(0.0);
        self.entropy = (self.entropy * (1.0 - rate * 0.5)).max(0.0);
        self.state = match (self.energy, self.entropy) {
            (e, s) if e < 0.1 && s < 0.1 => EquilibriumState::Stable,
            (e, s) if e < 0.5 && s < 0.5 => EquilibriumState::Metastable,
            _ => EquilibriumState::Unstable,
        };
        Ok(())
    }

    pub fn state(&self) -> EquilibriumState { self.state }
    pub fn is_stable(&self) -> bool { matches!(self.state, EquilibriumState::Stable) }
    pub fn perturb(&mut self, amount: f64) -> Result<(), MorphogenesisError> {
        self.energy = (self.energy + amount).min(1.0);
        self.entropy = (self.entropy + amount * 0.5).min(1.0);
        self.state = EquilibriumState::Unstable;
        Ok(())
    }
    pub fn validate(&self) -> Result<(), MorphogenesisError> {
        if !(0.0..=1.0).contains(&self.energy) { return Err(MorphogenesisError::OutOfRange { field: "energy".into(), value: self.energy, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Display for EquilibriumState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self { Self::Unstable => write!(f, "Unstable"), Self::Metastable => write!(f, "Metastable"), Self::Stable => write!(f, "Stable") }
    }
}
