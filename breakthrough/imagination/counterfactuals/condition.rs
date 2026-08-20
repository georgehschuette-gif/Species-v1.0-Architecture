// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::ImaginationError;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AntecedentType {
    Factual,
    CounterToFact,
    PlausibleAlternate,
    Impossible,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConditionalOperator {
    If,
    Unless,
    Had,
    WouldHave,
}

pub struct CounterfactualCondition {
    pub antecedent: f64,
    pub consequent: f64,
    pub operator: ConditionalOperator,
    pub antecedent_type: AntecedentType,
    pub confidence: f64,
}

impl CounterfactualCondition {
    pub fn new(antecedent: f64) -> Result<Self, ImaginationError> {
        if !(0.0..=1.0).contains(&antecedent) {
            return Err(ImaginationError::OutOfRange { field: "antecedent".into(), value: antecedent, min: 0.0, max: 1.0 });
        }
        Ok(Self {
            antecedent,
            consequent: antecedent * 0.5,
            operator: ConditionalOperator::If,
            antecedent_type: Self::classify(antecedent),
            confidence: 1.0 - antecedent.abs(),
        })
    }

    pub fn antecedent(&self) -> f64 { self.antecedent }
    pub fn consequent(&self) -> f64 { self.consequent }
    pub fn operator(&self) -> ConditionalOperator { self.operator }
    pub fn antecedent_type(&self) -> AntecedentType { self.antecedent_type }
    pub fn confidence(&self) -> f64 { self.confidence }

    pub fn set_operator(&mut self, operator: ConditionalOperator) {
        self.operator = operator;
    }

    pub fn update_consequent(&mut self, value: f64) -> Result<(), ImaginationError> {
        if !(0.0..=1.0).contains(&value) {
            return Err(ImaginationError::OutOfRange { field: "consequent".into(), value, min: 0.0, max: 1.0 });
        }
        self.consequent = value;
        Ok(())
    }

    pub fn is_feasible(&self) -> bool {
        self.antecedent_type != AntecedentType::Impossible && self.confidence > 0.3
    }

    pub fn distance_from_fact(&self) -> f64 {
        self.antecedent.abs()
    }

    pub fn validate(&self) -> Result<(), ImaginationError> {
        if !(0.0..=1.0).contains(&self.antecedent) {
            return Err(ImaginationError::OutOfRange { field: "antecedent".into(), value: self.antecedent, min: 0.0, max: 1.0 });
        }
        if !(0.0..=1.0).contains(&self.consequent) {
            return Err(ImaginationError::OutOfRange { field: "consequent".into(), value: self.consequent, min: 0.0, max: 1.0 });
        }
        Ok(())
    }

    fn classify(antecedent: f64) -> AntecedentType {
        if antecedent > 0.95 { AntecedentType::Factual }
        else if antecedent > 0.7 { AntecedentType::PlausibleAlternate }
        else if antecedent > 0.3 { AntecedentType::CounterToFact }
        else { AntecedentType::Impossible }
    }
}

impl fmt::Display for CounterfactualCondition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Counterfactual({:?} antecedent={:.2}, consequent={:.2})", self.operator, self.antecedent, self.consequent)
    }
}

impl fmt::Display for ConditionalOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::If => write!(f, "If"),
            Self::Unless => write!(f, "Unless"),
            Self::Had => write!(f, "Had"),
            Self::WouldHave => write!(f, "WouldHave"),
        }
    }
}
