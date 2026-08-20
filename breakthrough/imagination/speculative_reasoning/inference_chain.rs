// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

use crate::ImaginationError;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChainValidation {
    Valid,
    Invalid,
    Uncertain,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChainStep {
    pub step_index: usize,
    pub premise: String,
    pub rule: DeductionRule,
    pub conclusion: String,
    pub confidence: f64,
    pub validated: bool,
}

impl ChainStep {
    pub fn new(
        step_index: usize,
        premise: impl Into<String>,
        rule: DeductionRule,
        conclusion: impl Into<String>,
        confidence: f64,
    ) -> Result<Self, ImaginationError> {
        if !(0.0..=1.0).contains(&confidence) {
            return Err(ImaginationError::OutOfRange {
                field: "confidence".into(),
                value: confidence,
                min: 0.0,
                max: 1.0,
            });
        }
        Ok(Self {
            step_index,
            premise: premise.into(),
            rule,
            conclusion: conclusion.into(),
            confidence,
            validated: false,
        })
    }

    pub fn validate(&mut self) -> Result<(), ImaginationError> {
        if self.premise.is_empty() || self.conclusion.is_empty() {
            return Err(ImaginationError::InvalidInput(
                "premise and conclusion must not be empty".into(),
            ));
        }
        self.validated = true;
        Ok(())
    }

    pub fn is_valid(&self) -> bool {
        self.validated && !self.premise.is_empty() && !self.conclusion.is_empty()
    }
}

impl fmt::Display for ChainStep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Step[{}]: {} -> {} (via {:?}, conf={:.2})",
            self.step_index, self.premise, self.conclusion, self.rule, self.confidence
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InferenceChain {
    pub steps: Vec<ChainStep>,
    pub overall_confidence: f64,
    pub validation_status: ChainValidation,
}

impl InferenceChain {
    pub fn new() -> Self {
        Self {
            steps: Vec::new(),
            overall_confidence: 0.0,
            validation_status: ChainValidation::Uncertain,
        }
    }

    pub fn add_step(&mut self, step: ChainStep) -> Result<(), ImaginationError> {
        if step.step_index != self.steps.len() {
            return Err(ImaginationError::InvalidInput(
                "step index must be sequential".into(),
            ));
        }
        self.steps.push(step);
        self.recompute_confidence();
        Ok(())
    }

    pub fn validate_chain(&mut self) -> Result<(), ImaginationError> {
        for step in &mut self.steps {
            step.validate()?;
        }
        self.validation_status = if self.steps.len() <= MAX_CHAIN_LENGTH {
            ChainValidation::Valid
        } else {
            ChainValidation::Uncertain
        };
        Ok(())
    }

    pub fn is_valid(&self) -> bool {
        matches!(self.validation_status, ChainValidation::Valid)
            && self.steps.len() <= MAX_CHAIN_LENGTH
    }

    pub fn conclusion(&self) -> Option<&str> {
        self.steps.last().map(|s| s.conclusion.as_str())
    }

    pub fn step_count(&self) -> usize {
        self.steps.len()
    }

    fn recompute_confidence(&mut self) {
        if self.steps.is_empty() {
            self.overall_confidence = 0.0;
            return;
        }
        let product: f64 = self.steps.iter().map(|s| s.confidence).product();
        self.overall_confidence = product.powf(1.0 / self.steps.len() as f64);
    }
}

impl Default for InferenceChain {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for InferenceChain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "InferenceChain(steps={}, confidence={:.2}, valid={})",
            self.steps.len(),
            self.overall_confidence,
            self.is_valid()
        )
    }
}