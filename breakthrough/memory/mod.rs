// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Memory: The multi-layered storage and retrieval system of the cognitive ecosystem.
//!
//! This module models cognitive memory as a spectrum of interacting subsystems,
//! from fleeting perceptual echoes to durable semantic facts. Each submodule
//! implements a distinct memory type with its own formation, consolidation,
//! retrieval, and decay dynamics.
//!
//! # Memory Hierarchy
//!
//! - **echoes** — Ultra-short-term perceptual reverberations that decay within
//!   seconds and may superimpose on one another.
//! - **traces** — Short-to-medium term engram traces that undergo synaptic
//!   potentiation and homeostatic scaling.
//! - **attractor_memory** — Stable attractor basins that encode memories as
//!   fixed points in state space, robust to perturbation.
//! - **episodic** — Event memories with spatiotemporal and emotional context,
//!   subject to reconsolidation and embellishment.
//! - **semantic** — Factual and conceptual knowledge structured as labeled
//!   relations and feature vectors.
//! - **procedural** — Skill and sequence memories encoded as action chains
//!   with transition probabilities and automaticity curves.
//! - **latent_patterns** — Hidden statistical structure discovered from streams
//!   of sensory or conceptual data.
//! - **forgotten** — Decay, suppression, and dissolution models that govern
//!   how memories fade or are actively inhibited.
//! - **reconstruction** — Retrieval, partial recall, and hallucination models
//!   that capture the constructive nature of memory access.
//!
//! # Cross-Module Integration
//!
//! Memories flow upward through the hierarchy: echoes feed into traces,
//! traces seed attractors, episodic and semantic memories interconvert,
//! and procedural skills are refined from repeated episodic episodes.
//! Forgetting and reconstruction act on all levels.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum MemoryError {
    OutOfRange {
        field: String,
        value: f64,
        min: f64,
        max: f64,
    },
    MissingInput(String),
    InvalidInput(String),
    DimensionMismatch {
        expected: usize,
        actual: usize,
    },
    CapacityExceeded {
        max: usize,
        attempted: usize,
    },
    IncompatibleMemory {
        reason: String,
    },
    ThresholdNotMet {
        threshold: f64,
        actual: f64,
    },
    DecayError {
        rate: f64,
    },
    TraceError(String),
}

impl fmt::Display for MemoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfRange { field, value, min, max } => {
                write!(f, "{} out of range: {} not in [{}, {}]", field, value, min, max)
            }
            Self::MissingInput(msg) => write!(f, "missing input: {}", msg),
            Self::InvalidInput(msg) => write!(f, "invalid input: {}", msg),
            Self::DimensionMismatch { expected, actual } => {
                write!(f, "dimension mismatch: expected {}, got {}", expected, actual)
            }
            Self::CapacityExceeded { max, attempted } => {
                write!(f, "capacity exceeded: max {}, attempted {}", max, attempted)
            }
            Self::IncompatibleMemory { reason } => {
                write!(f, "incompatible memory: {}", reason)
            }
            Self::ThresholdNotMet { threshold, actual } => {
                write!(f, "threshold {} not met (actual: {})", threshold, actual)
            }
            Self::DecayError { rate } => {
                write!(f, "invalid decay rate: {}", rate)
            }
            Self::TraceError(msg) => {
                write!(f, "trace error: {}", msg)
            }
        }
    }
}

impl std::error::Error for MemoryError {}

pub mod echoes;
pub mod traces;
pub mod attractor_memory;
pub mod episodic;
pub mod semantic;
pub mod procedural;
pub mod latent_patterns;
pub mod forgotten;
pub mod reconstruction;

pub use echoes::{EchoEvent, EchoReverberation};
pub use traces::{EngramTrace, SynapticTrace};
pub use attractor_memory::{AttractorBasin, AttractorMemoryState};
pub use episodic::{EpisodicEvent, EpisodicContext};
pub use semantic::{SemanticFact, SemanticConcept};
pub use procedural::{ProceduralSkill, ActionSequence};
pub use latent_patterns::{LatentPatternDiscovery, PatternExtraction};
pub use forgotten::{MemoryDecay, MemorySuppression, DecayType};
pub use reconstruction::{MemoryRecall, MemoryHallucination};

/// Default fade rate for perceptual echoes per time unit.
pub const DEFAULT_ECHO_FADE_RATE: f64 = 0.08;
/// Default engram consolidation time in abstract units.
pub const DEFAULT_CONSOLIDATION_DELAY: f64 = 10.0;
/// Default attactor basin radius threshold.
pub const DEFAULT_BASIN_RADIUS: f64 = 0.3;
/// Default memory decay rate for forgotten items.
pub const DEFAULT_FORGETTING_RATE: f64 = 0.02;
/// Default retrieval similarity threshold.
pub const DEFAULT_RETRIEVAL_THRESHOLD: f64 = 0.65;

/// Validates that a cross-module memory strength value is within bounds.
///
/// # Errors
///
/// Returns [`MemoryError::OutOfRange`] if `strength` is outside [0.0, 1.0].
pub fn validate_strength(strength: f64) -> Result<(), MemoryError> {
    if !(0.0..=1.0).contains(&strength) {
        return Err(MemoryError::OutOfRange {
            field: "strength".into(),
            value: strength,
            min: 0.0,
            max: 1.0,
        });
    }
    Ok(())
}

/// Validates that a time duration is non-negative.
///
/// # Errors
///
/// Returns [`MemoryError::OutOfRange`] if `duration` is negative.
pub fn validate_duration(duration: f64) -> Result<(), MemoryError> {
    if duration < 0.0 {
        return Err(MemoryError::OutOfRange {
            field: "duration".into(),
            value: duration,
            min: 0.0,
            max: f64::INFINITY,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strength_validation_succeeds_for_valid_values() {
        assert!(validate_strength(0.0).is_ok());
        assert!(validate_strength(0.5).is_ok());
        assert!(validate_strength(1.0).is_ok());
    }

    #[test]
    fn strength_validation_rejects_out_of_range() {
        assert!(validate_strength(-0.1).is_err());
        assert!(validate_strength(1.1).is_err());
    }

    #[test]
    fn duration_validation_rejects_negative() {
        assert!(validate_duration(-1.0).is_err());
        assert!(validate_duration(0.0).is_ok());
        assert!(validate_duration(5.0).is_ok());
    }

    #[test]
    fn default_constants_are_sensible() {
        assert!(DEFAULT_ECHO_FADE_RATE > 0.0);
        assert!(DEFAULT_CONSOLIDATION_DELAY > 0.0);
        assert!(DEFAULT_BASIN_RADIUS > 0.0);
        assert!(DEFAULT_FORGETTING_RATE > 0.0);
        assert!(DEFAULT_RETRIEVAL_THRESHOLD > 0.0);
    }
}
