// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;
use crate::ImaginationError;
use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum HypothesisType {
    Causal,
    Predictive,
    Explanatory,
    Counterfactual,
    Emergent,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HypothesisState {
    Proposed,
    Tested,
    Validated,
    Refuted,
    Suspended,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConfidenceLevel(pub f64);

impl ConfidenceLevel {
    pub const MIN: f64 = 0.0;
    pub const MAX: f64 = 1.0;

    pub fn new(value: f64) -> Result<Self, ImaginationError> {
        if !(Self::MIN..=Self::MAX).contains(&value) {
            return Err(ImaginationError::OutOfRange {
                field: "confidence".into(),
                value,
                min: Self::MIN,
                max: Self::MAX,
            });
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> f64 {
        self.0
    }

    pub fn is_significant(&self) -> bool {
        self.0 >= DEFAULT_CONFIDENCE_THRESHOLD
    }
}

impl fmt::Display for ConfidenceLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}%", self.0 * 100.0)
    }
}

#[derive(Debug, Clone)]
pub struct Hypothesis {
    pub id: String,
    pub statement: String,
    pub hypothesis_type: HypothesisType,
    pub state: HypothesisState,
    pub confidence: ConfidenceLevel,
    pub evidence_count: usize,
    pub supporting_evidence: usize,
    pub contradicting_evidence: usize,
}

impl Hypothesis {
    pub fn new(id: impl Into<String>, statement: impl Into<String>, htype: HypothesisType) -> Self {
        Self {
            id: id.into(),
            statement: statement.into(),
            hypothesis_type: htype,
            state: HypothesisState::Proposed,
            confidence: ConfidenceLevel::new(0.5).unwrap_or_else(|_| ConfidenceLevel(0.5)),
            evidence_count: 0,
            supporting_evidence: 0,
            contradicting_evidence: 0,
        }
    }

    pub fn with_confidence(mut self, confidence: f64) -> Result<Self, ImaginationError> {
        self.confidence = ConfidenceLevel::new(confidence)?;
        Ok(self)
    }

    pub fn update_confidence(&mut self) -> Result<(), ImaginationError> {
        if self.evidence_count == 0 {
            return Ok(());
        }
        let ratio = self.supporting_evidence as f64 / self.evidence_count as f64;
        self.confidence = ConfidenceLevel::new(ratio)?;
        Ok(())
    }

    pub fn add_supporting_evidence(&mut self, weight: f64) {
        self.evidence_count += 1;
        self.supporting_evidence += 1;
        let _ = self.update_confidence();
        let _ = weight;
    }

    pub fn add_contradicting_evidence(&mut self, weight: f64) {
        self.evidence_count += 1;
        self.contradicting_evidence += 1;
        let _ = self.update_confidence();
        let _ = weight;
    }

    pub fn set_state(&mut self, state: HypothesisState) {
        self.state = state;
    }

    pub fn is_validated(&self) -> bool {
        matches!(self.state, HypothesisState::Validated)
    }

    pub fn conclusion_strength(&self) -> f64 {
        self.confidence.value() * if self.is_validated() { 1.0 } else { 0.5 }
    }
}

impl fmt::Display for Hypothesis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hypothesis[{}]: {} (confidence: {})", self.id, self.statement, self.confidence)
    }
}