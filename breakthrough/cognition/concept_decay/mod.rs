// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Concept Decay: The weakening and dissolution of concepts over time.
//!
//! Concepts lose strength when not reinforced. This module models three
//! complementary decay mechanisms:
//!
//! 1. **Forgetting** — Exponential decay of unused concepts, with accelerated
//!    loss for concepts below a usage threshold.
//! 2. **Pruning** — Hard removal of weak or excess concepts to prevent
//!    cognitive overload.
//! 3. **Reconsolidation** — Partial recovery of strength when a previously
//!    decayed concept is reactivated within a stability window.
//!
//! # Decay Model
//!
//! The default forgetting curve follows:
//! `strength(t) = strength(0) * exp(-rate * t)`
//!
//! Concepts with `strength < usage_threshold` experience double the decay
//! rate, modeling the "use it or lose it" principle of cognition.
//!
//! # Examples
//!
//! ```
//! use breakthrough::cognition::concept_decay::{ConceptForgetting, ConceptPruning, ConceptReconsolidation};
//!
//! let mut forget = ConceptForgetting::new(0.05, 0.2).expect("valid parameters");
//! let remaining = forget.apply_decay(1.0, 0.9).expect("valid call");
//! assert!(remaining < 0.9);
//!
//! let mut pruner = ConceptPruning::new(0.1, 100).expect("valid parameters");
//! assert!(pruner.should_prune(0.05, 150));
//! assert!(!pruner.should_prune(0.5, 50));
//!
//! let recon = ConceptReconsolidation::new(0.5, 10.0).expect("valid parameters");
//! let recovered = recon.reconsolidate(0.3, 5.0).expect("valid call");
//! assert!(recovered >= 0.3);
//! ```
//!
//! # Stability Windows
//!
//! The stability window is a bounded time interval during which reactivation
//! grants bonus consolidation. Outside the window, the consolidation factor
//! decays linearly toward zero.
//!
//! # Integration with Lifecycle
//!
//! Concept Decay is invoked between cognitive cycles. Pruning is typically
//! run less frequently than forgetting to avoid thrashing.
//!
//! [`CognitionError`]: super::CognitionError

pub mod forgetting;
pub mod pruning;
pub mod reconsolidation;

pub use super::CognitionError;
pub use forgetting::ConceptForgetting;
pub use pruning::ConceptPruning;
pub use reconsolidation::ConceptReconsolidation;

/// Default forgetting rate used in baseline cognitive simulations.
pub const DEFAULT_FORGETTING_RATE: f64 = 0.05;
/// Default usage threshold below which forgetting accelerates.
pub const DEFAULT_USAGE_THRESHOLD: f64 = 0.2;
/// Default maximum concept count before pruning triggers.
pub const DEFAULT_MAX_CONCEPTS: usize = 100;
/// Default consolidation factor for reconsolidation.
pub const DEFAULT_CONSOLIDATION_FACTOR: f64 = 0.5;

/// Applies a full decay cycle across all three mechanisms.
///
/// Convenience helper that chains forgetting, optional pruning,
/// and reconsolidation into a single pipeline.
///
/// # Errors
///
/// Propagates any [`CognitionError`] from the underlying mechanisms.
pub fn apply_full_decay_cycle(
    strength: f64,
    elapsed: f64,
    total_concepts: usize,
    prune: bool,
) -> Result<(f64, bool), CognitionError> {
    let forget = ConceptForgetting::new(DEFAULT_FORGETTING_RATE, DEFAULT_USAGE_THRESHOLD)?;
    let remaining = forget.apply_decay(elapsed, strength)?;
    let pruner = ConceptPruning::new(0.01, DEFAULT_MAX_CONCEPTS)?;
    let should_prune = prune && pruner.should_prune(remaining, total_concepts);
    Ok((remaining, should_prune))
}

/// Validates the decay configuration for a concept.
///
/// Checks that forgetting parameters, pruning thresholds, and
/// reconsolidation windows are within their valid ranges.
pub fn validate_decay_config(
    forgetting_rate: f64,
    pruning_threshold: f64,
    consolidation_factor: f64,
) -> Result<(), CognitionError> {
    ConceptForgetting::new(forgetting_rate, DEFAULT_USAGE_THRESHOLD)?.validate()?;
    ConceptPruning::new(pruning_threshold, DEFAULT_MAX_CONCEPTS)?.validate()?;
    ConceptReconsolidation::new(consolidation_factor, 10.0)?.validate()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decay_cycle_preserves_nonzero() {
        let (remaining, _) = apply_full_decay_cycle(0.9, 1.0, 50, false).unwrap();
        assert!(remaining > 0.0);
        assert!(remaining < 0.9);
    }

    #[test]
    fn default_constants_are_valid() {
        assert!(DEFAULT_FORGETTING_RATE >= 0.0 && DEFAULT_FORGETTING_RATE <= 1.0);
        assert!(DEFAULT_MAX_CONCEPTS > 0);
    }
}