// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Genesis: Immutable foundations of the cognitive ecosystem.
//!
//! This module contains the axioms, ontology, constants, invariants,
//! emergence rules, topology, and genesis seed that form the base layer
//! of the entire system. Nothing here may be modified at runtime.
//!
//! # Module Structure
//!
//! The `genesis` module is organized into seven submodules:
//! - **Axioms** - Foundational truths from which all cognition derives
//! - **Ontology** - Fundamental types and category structures
//! - **Constants** - System-wide immutable governing parameters
//! - **Invariants** - Guardrail properties that must never be violated
//! - **Emergence Rules** - Transformation rules for complex behavior
//! - **Topology** - Graph structures and spatial relationships
//! - **Genesis Seed** - Initial conditions and bootstrap sequence

use std::fmt;

/// Errors that can occur during genesis operations.
#[derive(Debug, Clone, PartialEq)]
pub enum GenesisError {
    /// A value is outside the valid range [`min`, `max`].
    OutOfRange {
        field: String,
        value: f64,
        min: f64,
        max: f64,
    },
    /// Required input data is missing or empty.
    MissingInput(String),
    /// The operation cannot proceed due to an invalid internal state.
    InvalidState(String),
    /// A threshold was not met for the requested operation.
    ThresholdNotMet { threshold: f64, actual: f64 },
    /// A capacity limit has been exceeded.
    CapacityExceeded { max: usize, attempted: usize },
    /// A validation check failed with the given reason.
    ValidationFailure(String),
    /// An invariant has been violated.
    InvariantViolation(String),
    /// A cycle was detected in a structure that must remain acyclic.
    CycleDetected(String),
    /// A computation produced an invalid result, such as NaN or infinity.
    ComputationError(String),
    /// Two or more entities are incompatible for the requested operation.
    IncompatibleEntities { reason: String },
}

impl fmt::Display for GenesisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GenesisError::OutOfRange { field, value, min, max } => write!(
                f,
                "field '{}' value {} is out of range [{}, {}]",
                field, value, min, max
            ),
            GenesisError::MissingInput(detail) => {
                write!(f, "missing required input: {}", detail)
            }
            GenesisError::InvalidState(detail) => {
                write!(f, "invalid state: {}", detail)
            }
            GenesisError::ThresholdNotMet { threshold, actual } => write!(
                f,
                "threshold {} not met (actual: {})",
                threshold, actual
            ),
            GenesisError::CapacityExceeded { max, attempted } => write!(
                f,
                "capacity exceeded: max {} but attempted {}",
                max, attempted
            ),
            GenesisError::ValidationFailure(reason) => {
                write!(f, "validation failed: {}", reason)
            }
            GenesisError::InvariantViolation(reason) => {
                write!(f, "invariant violation: {}", reason)
            }
            GenesisError::CycleDetected(reason) => {
                write!(f, "cycle detected: {}", reason)
            }
            GenesisError::ComputationError(reason) => {
                write!(f, "computation error: {}", reason)
            }
            GenesisError::IncompatibleEntities { reason } => {
                write!(f, "incompatible entities: {}", reason)
            }
        }
    }
}

impl std::error::Error for GenesisError {}

/// Result type alias for genesis operations.
pub type GenesisResult<T> = Result<T, GenesisError>;

pub mod axioms;
pub mod ontology;
pub mod constants;
pub mod invariants;
pub mod emergence_rules;
pub mod topology;
pub mod genesis_seed;

pub use axioms::{ConservationAxiom, IdentityAxiom, CausalityAxiom};
pub use ontology::{Entity, Relation, Category, CognitiveField};
pub use constants::{FundamentalConstants, ScalingConstants, ThresholdConstants};
pub use invariants::{ConservationInvariant, CoherenceInvariant, CausalityInvariant};
pub use emergence_rules::{AggregationRule, DifferentiationRule, StabilizationRule};
pub use topology::{CognitiveGraph, CognitiveManifold, TopologicalMetric};
pub use genesis_seed::{
    InitialState, BootstrapSequence, BootstrapStep, PrimordialField,
};