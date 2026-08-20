// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::MorphogenesisError;

use super::*;

pub struct PointMutation {
    pub position: usize,
    pub magnitude: f64,
    pub effect: MutationEffect,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MutationEffect {
    Neutral,
    Beneficial,
    Deleterious,
    Lethal,
}

impl PointMutation {
    pub fn new(position: usize, magnitude: f64) -> Result<Self, MorphogenesisError> {
        if !(0.0..=1.0).contains(&magnitude) { return Err(MorphogenesisError::OutOfRange { field: "magnitude".into(), value: magnitude, min: 0.0, max: 1.0 }); }
        let effect = match magnitude {
            m if m < 0.1 => MutationEffect::Neutral,
            m if m < 0.5 => MutationEffect::Beneficial,
            m if m < 0.9 => MutationEffect::Deleterious,
            _ => MutationEffect::Lethal,
        };
        Ok(Self { position, magnitude, effect })
    }

    pub fn position(&self) -> usize { self.position }
    pub fn magnitude(&self) -> f64 { self.magnitude }
    pub fn effect(&self) -> MutationEffect { self.effect }
    pub fn is_silent(&self) -> bool { matches!(self.effect, MutationEffect::Neutral) }
    pub fn validate(&self) -> Result<(), MorphogenesisError> {
        if !(0.0..=1.0).contains(&self.magnitude) { return Err(MorphogenesisError::OutOfRange { field: "magnitude".into(), value: self.magnitude, min: 0.0, max: 1.0 }); }
        Ok(())
    }
}

