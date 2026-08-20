// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Causal Field: Cause-effect structure across the cognitive space.
//! Encodes intervention strength, counterfactual gradients, responsibility
//! attribution, and causal mechanism classification.
//!
//! The causal field module provides tools for modeling interventions on
//! cognitive nodes, computing counterfactual sensitivities, and attributing
//! causal responsibility across a network of cause-effect relationships.

pub mod intervention;
pub mod counterfactual;
pub mod responsibility;

pub use intervention::CausalIntervention;
pub use intervention::InterventionMechanism;
pub use counterfactual::CounterfactualGradient;
pub use responsibility::CausalResponsibility;

use std::fmt;

/// CausalField: A graph-based container for causal relationships between nodes.
#[derive(Debug, Clone, PartialEq)]
pub struct CausalField {
    pub name: String,
    pub interventions: Vec<CausalIntervention>,
    pub node_ids: Vec<u64>,
}

impl CausalField {
    pub fn new(name: String) -> Self {
        Self { name, interventions: Vec::new(), node_ids: Vec::new() }
    }

    pub fn add_intervention(&mut self, intervention: CausalIntervention) {
        self.interventions.push(intervention);
    }

    pub fn add_node(&mut self, node_id: u64) {
        if !self.node_ids.contains(&node_id) {
            self.node_ids.push(node_id);
        }
    }

    pub fn intervention_count(&self) -> usize {
        self.interventions.len()
    }

    pub fn node_count(&self) -> usize {
        self.node_ids.len()
    }

    pub fn strongest_intervention(&self) -> Option<&CausalIntervention> {
        self.interventions.iter().max_by(|a, b| a.strength.partial_cmp(&b.strength).unwrap_or(std::cmp::Ordering::Equal))
    }

    pub fn has_node(&self, node_id: u64) -> bool {
        self.node_ids.contains(&node_id)
    }
}

impl Default for CausalField {
    fn default() -> Self {
        Self::new(String::from("unnamed"))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CausalError {
    InvalidNode { node_id: u64 },
    InvalidStrength { strength: f64 },
    InvalidProbability { probability: f64 },
    InsufficientData,
    ComputationError(String),
}

impl fmt::Display for CausalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CausalError::InvalidNode { node_id } => write!(f, "invalid node id: {}", node_id),
            CausalError::InvalidStrength { strength } => write!(f, "invalid strength: {}", strength),
            CausalError::InvalidProbability { probability } => write!(f, "invalid probability: {}", probability),
            CausalError::InsufficientData => write!(f, "insufficient data for operation"),
            CausalError::ComputationError(msg) => write!(f, "computation error: {}", msg),
        }
    }
}

impl std::error::Error for CausalError {}

pub type CausalResult<T> = Result<T, CausalError>;
