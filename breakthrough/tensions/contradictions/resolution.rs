// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use super::super::TensionsError;
use super::tension_field::ContradictionState;

use super::*;

pub struct ContradictionResolution {
    pub strategy: ResolutionStrategy,
    pub completeness: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResolutionStrategy {
    Synthesis,
    Supression,
    Integration,
    Transcendence,
}

impl ContradictionResolution {
    pub fn new(strategy: ResolutionStrategy) -> Self {
        Self { strategy, completeness: 0.0 }
    }

    pub fn resolve(state: ContradictionState) -> Result<ResolutionOutcome, TensionsError> {
        match state {
            ContradictionState::Latent => Ok(ResolutionOutcome::NoOp),
            ContradictionState::Active => Ok(ResolutionOutcome::Partial),
            ContradictionState::Escalating => Ok(ResolutionOutcome::Full),
            ContradictionState::Resolved => Ok(ResolutionOutcome::NoOp),
        }
    }

    pub fn apply(&mut self, field: &mut ContradictionField) -> Result<f64, TensionsError> {
        self.completeness = match self.strategy {
            ResolutionStrategy::Synthesis => 0.9,
            ResolutionStrategy::Supression => 0.6,
            ResolutionStrategy::Integration => 0.8,
            ResolutionStrategy::Transcendence => 1.0,
        };
        field.resolve();
        Ok(self.completeness)
    }

    pub fn completeness(&self) -> f64 { self.completeness }
    pub fn set_strategy(&mut self, strategy: ResolutionStrategy) { self.strategy = strategy; }
    pub fn validate(&self) -> Result<(), TensionsError> {
        if !(0.0..=1.0).contains(&self.completeness) { return Err(TensionsError::OutOfRange { field: "completeness".into(), value: self.completeness, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResolutionOutcome {
    NoOp,
    Partial,
    Full,
}

impl fmt::Display for ResolutionOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self { Self::NoOp => write!(f, "NoOp"), Self::Partial => write!(f, "Partial"), Self::Full => write!(f, "Full") }
    }
}
