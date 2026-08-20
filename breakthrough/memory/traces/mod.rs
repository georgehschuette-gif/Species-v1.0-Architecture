// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Traces: Short-to-medium term memory traces and engram formation.
//!
//! Traces model the synaptic and systems-level changes that encode memories
//! after initial perception. Unlike echoes, traces persist long enough to
//! undergo consolidation, reactivation, and structural modification.
//!
//! # Subsystems
//!
//! - **Engram Traces** — Coherent memory representations formed by linking
//!   feature patterns across a distributed neural substrate.
//! - **Synaptic Traces** — Localized weight changes at individual connection
//!   points, governed by Hebbian plasticity and homeostatic scaling.
//!
//! # Consolidation
//!
//! Traces transition from fragile to stable states via a multi-phase
//! consolidation process. Reconsolidation can occur when a trace is
//! reactivated within a bounded stability window.

use std::fmt;

use crate::MemoryError;

pub mod engram;
pub mod synaptic;

pub use engram::EngramTrace;
pub use synaptic::SynapticTrace;

/// Default synaptic potentiation rate per activation event.
pub const DEFAULT_HEBBIAN_RATE: f64 = 0.1;
/// Default homeostatic target synaptic strength.
pub const DEFAULT_HOMEOSTATIC_TARGET: f64 = 0.5;
/// Maximum number of features in an engram trace.
pub const MAX_ENGRAM_FEATURES: usize = 1024;
/// Minimum consolidation score for a trace to be considered stable.
pub const CONSOLIDATION_THRESHOLD: f64 = 0.7;

/// Creates a new engram trace with the given feature vector and decay rate.
///
/// # Errors
///
/// Returns [`MemoryError::OutOfRange`] if decay rate is outside [0.0, 1.0].
/// Returns [`MemoryError::DimensionMismatch`] if the feature count exceeds
/// [`MAX_ENGRAM_FEATURES`].
pub fn create_engram(
    features: &[f64],
    decay_rate: f64,
) -> Result<EngramTrace, MemoryError> {
    EngramTrace::new(features, decay_rate)
}

/// Creates a new synaptic trace with the given initial weight.
///
/// # Errors
///
/// Returns [`MemoryError::OutOfRange`] if weight is outside [0.0, 1.0].
pub fn create_synaptic(weight: f64) -> Result<SynapticTrace, MemoryError> {
    SynapticTrace::new(weight, DEFAULT_HEBBIAN_RATE, DEFAULT_HOMEOSTATIC_TARGET)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn engram_creation_succeeds() {
        let features = vec![0.1, 0.2, 0.3, 0.4, 0.5];
        let e = create_engram(&features, 0.05).unwrap();
        assert_eq!(e.features.len(), 5);
        assert!(e.strength > 0.0);
    }

    #[test]
    fn engram_consolidation_increases_strength() {
        let features = vec![0.1, 0.2, 0.3];
        let mut e = create_engram(&features, 0.05).unwrap();
        e.strength = 0.8;
        let before = e.strength;
        e.consolidate(1.0).unwrap();
        assert!(e.strength > before);
    }

    #[test]
    fn synaptic_potentiation_increases_weight() {
        let mut s = create_synaptic(0.5).unwrap();
        let before = s.weight;
        s.potentiate(0.1).unwrap();
        assert!(s.weight >= before);
    }

    #[test]
    fn synaptic_depression_decreases_weight() {
        let mut s = create_synaptic(0.5).unwrap();
        s.depress(0.3).unwrap();
        assert!(s.weight < 0.5);
    }

    #[test]
    fn default_constants_valid() {
        assert!(DEFAULT_HEBBIAN_RATE > 0.0 && DEFAULT_HEBBIAN_RATE <= 1.0);
        assert!(MAX_ENGRAM_FEATURES > 0);
        assert!(CONSOLIDATION_THRESHOLD > 0.0 && CONSOLIDATION_THRESHOLD <= 1.0);
    }
}
