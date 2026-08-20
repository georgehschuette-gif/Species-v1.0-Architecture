// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Concept Fusion: The merging of multiple concepts into unified wholes.
//!
//! Fusion creates higher-order abstractions by combining related concepts.
//! The module provides three fusion strategies:
//!
//! 1. **Merging** — Combines concept strengths using weighted averages,
//!    maximum, or union strategies.
//! 2. **Blending** — Generates novel concepts by cross-pollinating features
//!    from existing sources using intersection, union, or symmetric difference.
//! 3. **Synthesis** — Iteratively constructs abstract concepts from concrete
//!    ones across multiple depth levels.
//!
//! # Merge Strategies
//!
//! - [`MergeStrategy::WeightedAverage`] — arithmetic mean of strengths.
//! - [`MergeStrategy::Maximum`] — strongest input dominates.
//! - [`MergeStrategy::Union`] — sum of strengths, capped at 1.0.
//!
//! # Feature Selection
//!
//! - [`FeatureSelection::Intersection`] — only common features survive.
//! - [`FeatureSelection::Union`] — all features survive.
//! - [`FeatureSelection::SymmetricDifference`] — unique features survive.
//!
//! # Examples
//!
//! ```
//! use breakthrough::cognition::concept_fusion::{ConceptMerging, ConceptBlending, ConceptSynthesis, MergeStrategy, FeatureSelection};
//!
//! let merger = ConceptMerging::new(0.6, MergeStrategy::WeightedAverage).expect("valid");
//! let combined = merger.merge(&[0.7, 0.8]).expect("merge succeeded");
//!
//! let blender = ConceptBlending::new(0.5, FeatureSelection::Intersection).expect("valid");
//! let novel = blender.blend(&[0.8, 0.6]).expect("blend succeeded");
//!
//! let mut synth = ConceptSynthesis::new(3, 0.0).expect("valid");
//! synth.abstract_up().expect("synthesize up");
//! ```
//!
//! # Semantic Constraints
//!
//! Fusion operations require that source concepts share sufficient
//! similarity. The merge threshold enforces this constraint.
//!
//! [`MergeStrategy::WeightedAverage`]: MergeStrategy::WeightedAverage
//! [`MergeStrategy::Maximum`]: MergeStrategy::Maximum
//! [`MergeStrategy::Union`]: MergeStrategy::Union
//! [`FeatureSelection::Intersection`]: FeatureSelection::Intersection
//! [`FeatureSelection::Union`]: FeatureSelection::Union
//! [`FeatureSelection::SymmetricDifference`]: FeatureSelection::SymmetricDifference

pub mod merging;
pub mod blending;
pub mod synthesis;

pub use super::CognitionError;
pub use merging::{ConceptMerging, MergeStrategy};
pub use blending::{ConceptBlending, FeatureSelection};
pub use synthesis::ConceptSynthesis;

/// Default merge threshold for similarity gating.
pub const DEFAULT_MERGE_THRESHOLD: f64 = 0.6;
/// Default blend factor for feature recombination.
pub const DEFAULT_BLEND_FACTOR: f64 = 0.5;
/// Maximum synthesis depth to prevent runaway abstraction.
pub const MAX_SYNTHESIS_DEPTH: usize = 100;

/// Fuses two concepts into a merged strength using the weighted average strategy.
///
/// Convenience wrapper around [`ConceptMerging::merge_pair`] with sensible
/// defaults.
///
/// # Errors
///
/// Returns [`CognitionError::OutOfRange`] if either strength is outside [0.0, 1.0].
/// Returns [`CognitionError::ThresholdNotMet`] if similarity is too low.
pub fn fuse_two(
    strength_a: f64,
    strength_b: f64,
) -> Result<(f64, f64), CognitionError> {
    let merger = ConceptMerging::new(DEFAULT_MERGE_THRESHOLD, MergeStrategy::WeightedAverage)?;
    merger.merge_pair(strength_a, strength_b)
}

/// Blends a slice of concept strengths into a novel concept.
///
/// # Errors
///
/// Returns [`CognitionError::MissingInput`] if `strengths` is empty.
pub fn blend_many(strengths: &[f64]) -> Result<f64, CognitionError> {
    let blender = ConceptBlending::new(DEFAULT_BLEND_FACTOR, FeatureSelection::Intersection)?;
    blender.blend(strengths)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_constants_valid() {
        assert!(DEFAULT_MERGE_THRESHOLD >= 0.0 && DEFAULT_MERGE_THRESHOLD <= 1.0);
        assert!(DEFAULT_BLEND_FACTOR >= 0.0 && DEFAULT_BLEND_FACTOR <= 1.0);
    }

    #[test]
    fn fuse_two_returns_similarity() {
        let (_, sim) = fuse_two(0.7, 0.8).unwrap();
        assert!(sim >= 0.0 && sim <= 1.0);
    }
}