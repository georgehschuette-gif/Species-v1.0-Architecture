// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Speculative Reasoning: Extended inference beyond observed evidence.
//!
//! This module implements reasoning processes that operate in the space of
//! possibilities, generating hypotheses, evaluating counterfactual scenarios,
//! and constructing logical chains from incomplete information.
//!
//! # Core Components
//!
//! - **hypothesis** — Formulates and evaluates tentative explanations
//! - **deduction** — Applies logical rules to derive conclusions from premises
//! - **evidence** — Weights and combines evidence for and against claims
//! - **inference_chain** — Links reasoning steps into verifiable chains

use std::fmt;

use crate::ImaginationError;

pub mod hypothesis;
pub mod deduction;
pub mod evidence;
pub mod inference_chain;

pub use hypothesis::{Hypothesis, HypothesisType, HypothesisState, ConfidenceLevel};
pub use deduction::{DeductionRule, DeductionStep, LogicalOperator};
pub use evidence::{Evidence, EvidenceType, EvidenceWeight, EvidenceSource};
pub use inference_chain::{InferenceChain, ChainStep, ChainValidation};

/// Default minimum confidence threshold for accepting a hypothesis.
pub const DEFAULT_CONFIDENCE_THRESHOLD: f64 = 0.6;
/// Maximum length of an inference chain before requiring validation.
pub const MAX_CHAIN_LENGTH: usize = 50;
/// Default evidence weight for direct observations.
pub const DEFAULT_OBSERVATION_WEIGHT: f64 = 0.9;

/// Validates that a confidence value is within [0.0, 1.0].
///
/// # Errors
///
/// Returns [`ImaginationError::OutOfRange`] if `confidence` is outside [0.0, 1.0].
pub fn validate_confidence(confidence: f64) -> Result<(), ImaginationError> {
    if !(0.0..=1.0).contains(&confidence) {
        return Err(ImaginationError::OutOfRange {
            field: "confidence".into(),
            value: confidence,
            min: 0.0,
            max: 1.0,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn confidence_validation_accepts_valid() {
        assert!(validate_confidence(0.0).is_ok());
        assert!(validate_confidence(0.5).is_ok());
        assert!(validate_confidence(1.0).is_ok());
    }

    #[test]
    fn confidence_validation_rejects_invalid() {
        assert!(validate_confidence(-0.1).is_err());
        assert!(validate_confidence(1.1).is_err());
    }

    #[test]
    fn constants_are_sensible() {
        assert!(DEFAULT_CONFIDENCE_THRESHOLD > 0.0 && DEFAULT_CONFIDENCE_THRESHOLD <= 1.0);
        assert!(MAX_CHAIN_LENGTH > 0);
        assert!(DEFAULT_OBSERVATION_WEIGHT > 0.0 && DEFAULT_OBSERVATION_WEIGHT <= 1.0);
    }
}