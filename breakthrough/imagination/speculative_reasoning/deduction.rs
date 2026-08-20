// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;
use crate::ImaginationError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeductionRule {
    ModusPonens,
    ModusTollens,
    HypotheticalSyllogism,
    DisjunctiveSyllogism,
    UniversalInstantiation,
    ExistentialInstantiation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicalOperator {
    And,
    Or,
    Not,
    Implies,
    IfAndOnlyIf,
}

#[derive(Debug, Clone)]
pub struct DeductionStep {
    pub rule: DeductionRule,
    pub premises: Vec<usize>,
    pub conclusion: String,
    pub valid: bool,
}

impl DeductionStep {
    pub fn new(rule: DeductionRule, premises: Vec<usize>, conclusion: impl Into<String>) -> Self {
        Self {
            rule,
            premises,
            conclusion: conclusion.into(),
            valid: true,
        }
    }

    pub fn invalidate(&mut self) {
        self.valid = false;
    }

    pub fn is_valid(&self) -> bool {
        self.valid && self.premises.len() >= 1
    }

    pub fn premise_count(&self) -> usize {
        self.premises.len()
    }
}

impl fmt::Display for DeductionRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ModusPonens => write!(f, "Modus Ponens"),
            Self::ModusTollens => write!(f, "Modus Tollens"),
            Self::HypotheticalSyllogism => write!(f, "Hypothetical Syllogism"),
            Self::DisjunctiveSyllogism => write!(f, "Disjunctive Syllogism"),
            Self::UniversalInstantiation => write!(f, "Universal Instantiation"),
            Self::ExistentialInstantiation => write!(f, "Existential Instantiation"),
        }
    }
}

impl fmt::Display for LogicalOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::And => write!(f, "∧"),
            Self::Or => write!(f, "∨"),
            Self::Not => write!(f, "¬"),
            Self::Implies => write!(f, "→"),
            Self::IfAndOnlyIf => write!(f, "↔"),
        }
    }
}