// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Cognition: The dynamic processes of concept formation, transformation,
//! and self-awareness within the ecosystem.
//!
//! This module covers the lifecycle of concepts—birth, growth, decay,
//! fusion, split, and mutation—alongside identity, perspective,
//! metacognition, and self-modeling.
//!
//! # Cognitive Lifecycle
//!
//! Concepts progress through a lifecycle managed by the submodules
//! within this module:
//! - **Birth**: Nucleation, crystallization, and activation of new concepts.
//! - **Growth**: Reinforcement, elaboration, and integration of existing concepts.
//! - **Decay**: Forgetting, pruning, and reconsolidation of weakening concepts.
//! - **Fusion**: Merging, blending, and synthesis of related concepts.
//! - **Split**: Differentiation, partitioning, and specialization of overloaded concepts.
//! - **Mutation**: Adaptation, evolution, and reconfiguration of concept structure.
//!
//! Each phase is governed by configurable parameters and validation logic
//! to ensure structural integrity of the cognitive ecosystem.

use std::fmt;

/// Errors that can occur during cognitive processing operations.
#[derive(Debug, Clone, PartialEq)]
pub enum CognitionError {
    /// An input value is outside the valid range [`min`, `max`].
    OutOfRange {
        field: String,
        value: f64,
        min: f64,
        max: f64,
    },
    /// Required input data is missing or empty.
    MissingInput(String),
    /// An operation cannot be performed due to an invalid internal state.
    InvalidState(String),
    /// A threshold was not met for the requested operation.
    ThresholdNotMet {
        threshold: f64,
        actual: f64,
    },
    /// A capacity limit has been exceeded.
    CapacityExceeded {
        max: usize,
        attempted: usize,
    },
    /// A decay or rate value is invalid.
    InvalidDecay {
        rate: f64,
    },
    /// Two or more concepts are incompatible for the requested operation.
    IncompatibleConcepts {
        reason: String,
    },
    /// An internal consistency check failed.
    ConsistencyError(String),
}

impl fmt::Display for CognitionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CognitionError::OutOfRange { field, value, min, max } => write!(
                f,
                "field '{}' value {} is out of range [{}, {}]",
                field, value, min, max
            ),
            CognitionError::MissingInput(detail) => {
                write!(f, "missing required input: {}", detail)
            }
            CognitionError::InvalidState(detail) => {
                write!(f, "invalid state: {}", detail)
            }
            CognitionError::ThresholdNotMet { threshold, actual } => write!(
                f,
                "threshold {} not met (actual: {})",
                threshold, actual
            ),
            CognitionError::CapacityExceeded { max, attempted } => write!(
                f,
                "capacity exceeded: max {} but attempted {}",
                max, attempted
            ),
            CognitionError::InvalidDecay { rate } => {
                write!(f, "invalid decay rate: {}", rate)
            }
            CognitionError::IncompatibleConcepts { reason } => {
                write!(f, "incompatible concepts: {}", reason)
            }
            CognitionError::ConsistencyError(detail) => {
                write!(f, "consistency error: {}", detail)
            }
        }
    }
}

impl std::error::Error for CognitionError {}

pub mod concept_birth;
pub mod concept_growth;
pub mod concept_decay;
pub mod concept_fusion;
pub mod concept_split;
pub mod concept_mutation;
pub mod identity;
pub mod perspective;
pub mod metacognition;
pub mod self_model;

pub use concept_birth::{ConceptNucleation, ConceptCrystallization, ConceptActivation};
pub use concept_growth::{ConceptReinforcement, ConceptElaboration, ConceptIntegration};
pub use concept_decay::{ConceptForgetting, ConceptPruning, ConceptReconsolidation};
pub use concept_fusion::{ConceptMerging, ConceptBlending, ConceptSynthesis};
pub use concept_split::{ConceptDifferentiation, ConceptPartitioning, ConceptSpecialization};
pub use concept_mutation::{ConceptAdaptation, ConceptEvolution, ConceptReconfiguration};
pub use identity::{IdentityContinuity, SelfReference, IdentityConstancy};
pub use perspective::{CognitiveViewpoint, CognitiveFraming, PerspectiveBias};
pub use metacognition::{SelfMonitor, SelfRegulate, MetaReason};
pub use self_model::{StateRepresentation, CapabilityModel, BoundaryAwareness};