// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Concept Split: The division of a concept into distinct sub-concepts.
//!
//! Split mechanisms differentiate overloaded or ambiguous concepts,
//! distributing their features and meaning across clearer boundaries.
//!
//! 1. **Differentiation** — Identifies distinguishing features between
//!    overlapping concepts using weighted Euclidean distance.
//! 2. **Partitioning** — Divides a concept into disjoint sub-concepts,
//!    balancing coverage across partitions.
//! 3. **Specialization** — Narrows a general concept to specific contexts,
//!    controlled by a specialization rate and context dependency.
//!
//! # Differentiation Thresholds
//!
//! Two concepts are split only when their weighted feature distance
//! exceeds the differentiation threshold. This prevents spurious splits
//! for concepts that are merely similar.
//!
//! # Partition Balance
//!
//! The balance factor in partitioning enforces even distribution of
//! sub-concept coverage. Imbalanced partitions trigger rebalancing.
//!
//! # Examples
//!
//! ```
//! use breakthrough::cognition::concept_split::{ConceptDifferentiation, ConceptPartitioning, ConceptSpecialization};
//!
//! let diff = ConceptDifferentiation::new(0.3, 0.8).expect("valid");
//! let dist = diff.compute_distance(&[0.9, 0.1], &[0.2, 0.8]);
//!
//! let partitioner = ConceptPartitioning::new(4, 0.8).expect("valid");
//! assert_eq!(partitioner.partitions(), 4);
//!
//! let spec = ConceptSpecialization::new(0.1, 0.7).expect("valid");
//! spec.specialize();
//! ```
//!
//! # Context Dependency
//!
//! Specialization rate is scaled by `(1.0 - context_dependency)` so that
//! concepts tightly bound to a context generalize more slowly.
//!
//! [`CognitionError`]: super::CognitionError

pub mod differentiation;
pub mod partitioning;
pub mod specialization;

pub use super::CognitionError;
pub use differentiation::ConceptDifferentiation;
pub use partitioning::ConceptPartitioning;
pub use specialization::ConceptSpecialization;

/// Default differentiation threshold for splitting.
pub const DEFAULT_DIFFERENTIATION_THRESHOLD: f64 = 0.3;
/// Default partition count for concept division.
pub const DEFAULT_PARTITION_COUNT: usize = 4;
/// Default balance factor for partitioning.
pub const DEFAULT_BALANCE_FACTOR: f64 = 0.8;
/// Default specialization rate.
pub const DEFAULT_SPECIALIZATION_RATE: f64 = 0.1;
/// Maximum allowed partition count.
pub const MAX_PARTITION_COUNT: usize = 1000;

/// Splits a concept into multiple sub-concepts using the configured strategy.
///
/// Convenience wrapper around partitioning and differentiation.
///
/// # Errors
///
/// Returns [`CognitionError::OutOfRange`] if configuration is invalid.
pub fn split_concept(item_count: usize) -> Result<Vec<usize>, CognitionError> {
    let partitioner = ConceptPartitioning::new(DEFAULT_PARTITION_COUNT, DEFAULT_BALANCE_FACTOR)?;
    let mut sizes = Vec::new();
    for i in 0..DEFAULT_PARTITION_COUNT {
        sizes.push(partitioner.partition_size(i, item_count)?);
    }
    Ok(sizes)
}

/// Validates a split configuration without performing a split.
pub fn validate_split_config(
    differentiation_threshold: f64,
    partition_count: usize,
    specialization_rate: f64,
) -> Result<(), CognitionError> {
    ConceptDifferentiation::new(differentiation_threshold, 0.5)?.validate()?;
    ConceptPartitioning::new(partition_count, DEFAULT_BALANCE_FACTOR)?.validate()?;
    ConceptSpecialization::new(specialization_rate, 0.5)?.validate()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_concept_distributes_items() {
        let sizes = split_concept(16).unwrap();
        let total: usize = sizes.iter().sum();
        assert_eq!(total, 16);
        assert_eq!(sizes.len(), DEFAULT_PARTITION_COUNT);
    }

    #[test]
    fn constants_within_bounds() {
        assert!(DEFAULT_DIFFERENTIATION_THRESHOLD <= 1.0);
        assert!(DEFAULT_PARTITION_COUNT <= MAX_PARTITION_COUNT);
    }
}