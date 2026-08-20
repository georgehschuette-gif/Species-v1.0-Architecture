// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Social Field: Relational and group-level cognitive distributions.
//! Encodes trust, influence, hierarchy, and cultural norms
//! across social agents and networks.
//!
//! The social field module models structured relationships between cognitive
//! agents, including trust dynamics, influence propagation, and hierarchical
//! authority distributions.

pub mod trust;
pub mod influence;
pub mod hierarchy;

pub use trust::SocialTrust;
pub use influence::SocialInfluence;
pub use hierarchy::{SocialHierarchy, HierarchyLevel};

use std::fmt;

/// SocialField: A relational container for social interactions among agents.
#[derive(Debug, Clone, PartialEq)]
pub struct SocialField {
    pub name: String,
    pub agents: Vec<u64>,
    pub trust_matrix: Vec<Vec<f64>>,
}

impl SocialField {
    pub fn new(name: String) -> Self {
        Self { name, agents: Vec::new(), trust_matrix: Vec::new() }
    }

    pub fn add_agent(&mut self, agent_id: u64) {
        if !self.agents.contains(&agent_id) {
            self.agents.push(agent_id);
        }
    }

    pub fn agent_count(&self) -> usize {
        self.agents.len()
    }

    pub fn set_trust(&mut self, source: usize, target: usize, value: f64) -> Result<(), SocialError> {
        if !(0.0..=1.0).contains(&value) {
            return Err(SocialError::InvalidTrust { trust: value });
        }
        if source >= self.agents.len() || target >= self.agents.len() {
            return Err(SocialError::InvalidAgent { agent_id: u64::max(source as u64, target as u64) });
        }
        if self.trust_matrix.len() <= source {
            self.trust_matrix.resize(source + 1, vec![0.5; self.agents.len()]);
        }
        for row in &mut self.trust_matrix {
            while row.len() < self.agents.len() {
                row.push(0.5);
            }
        }
        self.trust_matrix[source][target] = value;
        Ok(())
    }

    pub fn get_trust(&self, source: usize, target: usize) -> Option<f64> {
        self.trust_matrix.get(source).and_then(|row| row.get(target).copied())
    }
}

impl Default for SocialField {
    fn default() -> Self {
        Self::new(String::from("unnamed"))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SocialError {
    InvalidAgent { agent_id: u64 },
    InvalidTrust { trust: f64 },
    InvalidInfluence { influence: f64 },
    InvalidScore { score: f64 },
    InsufficientData,
    ComputationError(String),
}

impl fmt::Display for SocialError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SocialError::InvalidAgent { agent_id } => write!(f, "invalid agent id: {}", agent_id),
            SocialError::InvalidTrust { trust } => write!(f, "invalid trust score: {}", trust),
            SocialError::InvalidInfluence { influence } => write!(f, "invalid influence: {}", influence),
            SocialError::InvalidScore { score } => write!(f, "invalid score: {}", score),
            SocialError::InsufficientData => write!(f, "insufficient data for operation"),
            SocialError::ComputationError(msg) => write!(f, "computation error: {}", msg),
        }
    }
}

impl std::error::Error for SocialError {}

pub type SocialResult<T> = Result<T, SocialError>;
