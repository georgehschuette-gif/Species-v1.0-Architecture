// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Ethical Field: Normative and moral value distributions.
//! Encodes permissibility, value alignment, fairness, and moral weight
//! for decision-making within the cognitive ecosystem.
//!
//! The ethical field module provides tools for evaluating the moral permissibility
//! of actions, measuring alignment between values and behaviors, and assessing
//! distributive fairness across affected parties.

pub mod permissibility;
pub mod alignment;
pub mod fairness;

pub use permissibility::{EthicalPermissibility, EthicalNorm};
pub use alignment::{ValueAlignment, EthicalValue};
pub use fairness::EthicalFairness;

use std::fmt;

/// EthicalField: A normative container for moral evaluation frameworks.
#[derive(Debug, Clone, PartialEq)]
pub struct EthicalField {
    pub name: String,
    pub norms: Vec<EthicalNorm>,
    pub values: Vec<EthicalValue>,
}

impl EthicalField {
    pub fn new(name: String) -> Self {
        Self { name, norms: Vec::new(), values: Vec::new() }
    }

    pub fn add_norm(&mut self, norm: EthicalNorm) {
        if !self.norms.contains(&norm) {
            self.norms.push(norm);
        }
    }

    pub fn add_value(&mut self, value: EthicalValue) {
        if !self.values.contains(&value) {
            self.values.push(value);
        }
    }

    pub fn norm_count(&self) -> usize {
        self.norms.len()
    }

    pub fn value_count(&self) -> usize {
        self.values.len()
    }

    pub fn has_norm(&self, norm: &EthicalNorm) -> bool {
        self.norms.contains(norm)
    }

    pub fn has_value(&self, value: &EthicalValue) -> bool {
        self.values.contains(value)
    }

    pub fn is_comprehensive(&self) -> bool {
        self.norms.len() >= 3 && self.values.len() >= 3
    }
}

impl Default for EthicalField {
    fn default() -> Self {
        Self::new(String::from("unnamed"))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EthicalError {
    InvalidAction { action_id: u64 },
    InvalidScore { score: f64 },
    DimensionMismatch { values: usize, actions: usize },
    InsufficientData,
    ComputationError(String),
}

impl fmt::Display for EthicalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EthicalError::InvalidAction { action_id } => write!(f, "invalid action id: {}", action_id),
            EthicalError::InvalidScore { score } => write!(f, "invalid score: {}", score),
            EthicalError::DimensionMismatch { values, actions } => write!(f, "dimension mismatch: {} values vs {} actions", values, actions),
            EthicalError::InsufficientData => write!(f, "insufficient data for operation"),
            EthicalError::ComputationError(msg) => write!(f, "computation error: {}", msg),
        }
    }
}

impl std::error::Error for EthicalError {}

pub type EthicalResult<T> = Result<T, EthicalError>;
