// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::ImaginationError;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutcomeStatus {
    Possible,
    Probable,
    Certain,
    Contradictory,
    Absurd,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OutcomeTrace {
    pub step: usize,
    pub value: f64,
    pub delta: f64,
}

pub struct CounterfactualOutcome {
    pub plausibility: f64,
    pub status: OutcomeStatus,
    pub trace: Vec<OutcomeTrace>,
    pub counterfactual_strength: f64,
}

impl CounterfactualOutcome {
    pub fn from_plausibility(plausibility: f64) -> Self {
        let status = Self::classify_status(plausibility);
        Self {
            plausibility,
            status,
            trace: Vec::new(),
            counterfactual_strength: 1.0 - plausibility,
        }
    }

    pub fn plausibility(&self) -> f64 { self.plausibility }
    pub fn status(&self) -> OutcomeStatus { self.status }
    pub fn trace(&self) -> &[OutcomeTrace] { &self.trace }
    pub fn counterfactual_strength(&self) -> f64 { self.counterfactual_strength }

    pub fn record_step(&mut self, step: usize, value: f64, delta: f64) -> Result<(), ImaginationError> {
        if !(0.0..=1.0).contains(&value) {
            return Err(ImaginationError::OutOfRange { field: "value".into(), value, min: 0.0, max: 1.0 });
        }
        self.trace.push(OutcomeTrace { step, value, delta });
        self.plausibility = (self.plausibility + delta * 0.1).clamp(0.0, 1.0);
        self.status = Self::classify_status(self.plausibility);
        self.counterfactual_strength = 1.0 - self.plausibility;
        Ok(())
    }

    pub fn is_coherent(&self) -> bool {
        !matches!(self.status, OutcomeStatus::Contradictory | OutcomeStatus::Absurd)
    }

    pub fn final_value(&self) -> Option<f64> {
        self.trace.last().map(|t| t.value)
    }

    pub fn validate(&self) -> Result<(), ImaginationError> {
        if !(0.0..=1.0).contains(&self.plausibility) {
            return Err(ImaginationError::OutOfRange { field: "plausibility".into(), value: self.plausibility, min: 0.0, max: 1.0 });
        }
        Ok(())
    }

    fn classify_status(plausibility: f64) -> OutcomeStatus {
        if plausibility > 0.9 { OutcomeStatus::Certain }
        else if plausibility > 0.7 { OutcomeStatus::Probable }
        else if plausibility > 0.4 { OutcomeStatus::Possible }
        else if plausibility > 0.2 { OutcomeStatus::Contradictory }
        else { OutcomeStatus::Absurd }
    }
}

impl fmt::Display for CounterfactualOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CounterfactualOutcome(plausibility={:.2}, status={:?}, strength={:.2})", self.plausibility, self.status, self.counterfactual_strength)
    }
}

impl fmt::Display for OutcomeStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Possible => write!(f, "Possible"),
            Self::Probable => write!(f, "Probable"),
            Self::Certain => write!(f, "Certain"),
            Self::Contradictory => write!(f, "Contradictory"),
            Self::Absurd => write!(f, "Absurd"),
        }
    }
}
