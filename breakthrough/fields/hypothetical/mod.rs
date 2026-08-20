// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Hypothetical Field: Counterfactual and alternative-reality distributions.
//! Encodes alternative possibilities, conditional constructs, and suppositional
//! strength for reasoning about what could be.
//!
//! The hypothetical field module provides tools for evaluating counterfactual
//! scenarios, modeling conditional dependencies, and assessing the plausibility
//! of alternative states of affairs.

pub mod possibility;
pub mod conditionality;
pub mod supposition;

pub use possibility::HypotheticalPossibility;
pub use conditionality::ConditionalConstruct;
pub use supposition::SuppositionalStrength;

use std::fmt;

/// HypotheticalField: A container for counterfactual and alternative scenarios.
#[derive(Debug, Clone, PartialEq)]
pub struct HypotheticalField {
    pub name: String,
    pub possibilities: Vec<HypotheticalPossibility>,
    pub conditionals: Vec<ConditionalConstruct>,
}

impl HypotheticalField {
    pub fn new(name: String) -> Self {
        Self { name, possibilities: Vec::new(), conditionals: Vec::new() }
    }

    pub fn add_possibility(&mut self, possibility: HypotheticalPossibility) {
        self.possibilities.push(possibility);
    }

    pub fn add_conditional(&mut self, conditional: ConditionalConstruct) {
        self.conditionals.push(conditional);
    }

    pub fn possibility_count(&self) -> usize {
        self.possibilities.len()
    }

    pub fn conditional_count(&self) -> usize {
        self.conditionals.len()
    }

    pub fn avg_viability(&self) -> Option<f64> {
        if self.possibilities.is_empty() {
            return None;
        }
        let sum: f64 = self.possibilities.iter().map(|p| p.overall_viability()).sum();
        Some(sum / self.possibilities.len() as f64)
    }

    pub fn strongest_conditional(&self) -> Option<&ConditionalConstruct> {
        self.conditionals.iter().max_by(|a, b| a.conditional_probability.partial_cmp(&b.conditional_probability).unwrap_or(std::cmp::Ordering::Equal))
    }
}

impl Default for HypotheticalField {
    fn default() -> Self {
        Self::new(String::from("unnamed"))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum HypotheticalError {
    InvalidAlternative { alternative_id: u64 },
    InvalidScore { score: f64 },
    InvalidStrength { strength: f64 },
    InvalidProbability { probability: f64 },
    InvalidCondition { condition: String },
    InsufficientData,
    ComputationError(String),
}

impl fmt::Display for HypotheticalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HypotheticalError::InvalidAlternative { alternative_id } => write!(f, "invalid alternative id: {}", alternative_id),
            HypotheticalError::InvalidScore { score } => write!(f, "invalid score: {}", score),
            HypotheticalError::InvalidStrength { strength } => write!(f, "invalid strength: {}", strength),
            HypotheticalError::InvalidProbability { probability } => write!(f, "invalid probability: {}", probability),
            HypotheticalError::InvalidCondition { condition } => write!(f, "invalid condition: {}", condition),
            HypotheticalError::InsufficientData => write!(f, "insufficient data for operation"),
            HypotheticalError::ComputationError(msg) => write!(f, "computation error: {}", msg),
        }
    }
}

impl std::error::Error for HypotheticalError {}

pub type HypotheticalResult<T> = Result<T, HypotheticalError>;
