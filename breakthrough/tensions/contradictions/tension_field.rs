// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::super::TensionsError;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ContradictionState {
    Latent,
    Active,
    Escalating,
    Resolved,
}

pub struct ContradictionField {
    pub strength: f64,
    pub polarity: f64,
    pub state: ContradictionState,
}

impl ContradictionField {
    pub fn new(strength: f64) -> Result<Self, TensionsError> {
        if !(0.0..=1.0).contains(&strength) {
            return Err(TensionsError::OutOfRange { field: "strength".into(), value: strength, min: 0.0, max: 1.0 });
        }
        Ok(Self { strength, polarity: 0.0, state: ContradictionState::Latent })
    }

    pub fn strength(&self) -> f64 { self.strength }
    pub fn activate(&mut self) { self.state = ContradictionState::Active; }
    pub fn escalate(&mut self, amount: f64) -> Result<(), TensionsError> {
        self.strength = (self.strength + amount).min(1.0);
        self.state = if self.strength > 0.8 { ContradictionState::Escalating } else { ContradictionState::Active };
        Ok(())
    }
    pub fn resolve(&mut self) { self.state = ContradictionState::Resolved; self.strength = 0.0; }
    pub fn is_active(&self) -> bool { matches!(self.state, ContradictionState::Active | ContradictionState::Escalating) }
    pub fn tension(&self) -> f64 { self.strength * self.polarity.abs() }
    pub fn validate(&self) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&self.strength) { return Err(TensionsError::OutOfRange { field: "strength".into(), value: self.strength, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

impl fmt::Display for ContradictionField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Contradiction(strength={:.2}, state={:?})", self.strength, self.state)
    }
}
