// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Semantic Memory: Factual and conceptual knowledge structured as relations.
//!
//! Semantic memory stores decontextualized knowledge: facts, definitions,
//! categories, and conceptual relationships. Unlike episodic memory, semantic
//! memory is not tied to personal experience but to the abstract structure
//! of knowledge itself.
//!
//! # Components
//!
//! - **SemanticFact** — A grounded assertion linking a subject, predicate,
//!   and object, with confidence and source reliability metadata.
//! - **SemanticConcept** — A named cognitive concept with feature vectors,
//!   associative strength, and activation dynamics.
//!
//! # Knowledge Graphs
//!
//! Facts can be composed into directed graphs, where nodes are concepts and
//! edges are semantic relations. This structure supports reasoning, inference,
//! and knowledge completion.

use std::fmt;

use crate::MemoryError;

pub mod fact;
pub mod concept;

pub use fact::SemanticFact;
pub use concept::SemanticConcept;

/// Default confidence floor for a fact to be considered valid.
pub const DEFAULT_CONFIDENCE_FLOOR: f64 = 0.3;
/// Default source reliability for unverified facts.
pub const DEFAULT_SOURCE_RELIABILITY: f64 = 0.5;
/// Maximum number of features per semantic concept.
pub const MAX_CONCEPT_FEATURES: usize = 2048;
/// Minimum associative strength for a link to be considered active.
pub const MIN_ASSOCIATIVE_STRENGTH: f64 = 0.01;

/// Creates a new semantic fact about the given triple.
///
/// # Errors
///
/// Returns [`MemoryError::MissingInput`] if any part of the triple is empty.
/// Returns [`MemoryError::OutOfRange`] if confidence or reliability is outside [0.0, 1.0].
pub fn create_fact(
    subject: impl Into<String>,
    predicate: impl Into<String>,
    object: impl Into<String>,
    confidence: f64,
    source_reliability: f64,
) -> Result<SemanticFact, MemoryError> {
    SemanticFact::new(subject, predicate, object, confidence, source_reliability)
}

/// Creates a new semantic concept with the given name.
///
/// # Errors
///
/// Returns [`MemoryError::MissingInput`] if the name is empty.
pub fn create_concept(name: impl Into<String>) -> Result<SemanticConcept, MemoryError> {
    SemanticConcept::new(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fact_creation_succeeds() {
        let f = create_fact("cat", "is_a", "mammal", 0.9, 0.8).unwrap();
        assert!(f.confidence_score() >= 0.3);
    }

    #[test]
    fn fact_discount_reduces_confidence() {
        let mut f = create_fact("x", "y", "z", 1.0, 0.9).unwrap();
        f.discount(0.3);
        assert!(f.confidence < 1.0);
    }

    #[test]
    fn concept_activation_increases_strength() {
        let mut c = create_concept("tree").unwrap();
        c.activate(0.5).unwrap();
        assert!(c.activation >= 0.5);
    }

    #[test]
    fn concept_linking_increases_associative_density() {
        let mut c = create_concept("river").unwrap();
        c.link_feature(0.5).unwrap();
        c.link_feature(0.3).unwrap();
        let other = create_concept("stream").unwrap();
        let before = c.associative_density();
        c.link_concept(&other).unwrap();
        assert!(c.associative_density() > before);
    }

    #[test]
    fn default_constants_valid() {
        assert!(DEFAULT_CONFIDENCE_FLOOR >= 0.0 && DEFAULT_CONFIDENCE_FLOOR <= 1.0);
        assert!(MAX_CONCEPT_FEATURES > 0);
    }
}
