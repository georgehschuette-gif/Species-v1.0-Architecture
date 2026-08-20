// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Abstraction: Lifting concrete percepts to higher-level concepts.
//!
//! Abstraction mechanisms transform raw feature vectors into structured,
//! reusable cognitive representations. The pipeline consists of three
//! coordinated layers:
//!
//! 1. **CategoryFormation** — Groups percepts into abstract categories
//!    using centroid-based clustering and cosine similarity.
//! 2. **PrototypeFormation** — Creates central representative vectors
//!    from category members, updating prototypes via streaming averages.
//! 3. **AbstractionHierarchy** — Organizes concepts into nested levels
//!    of abstraction with parent-child relationships.
//!
//! # Category Semantics
//!
//! A category is defined by its centroid, member set, and a human-readable
//! label. Categories with fewer members than `min_members` are pruned
//! to prevent over-fragmentation of the concept space.
//!
//! # Prototype Quality
//!
//! Prototype quality is measured as the inverse average Euclidean distance
//! to member points. Higher quality indicates a tighter, more representative
//! prototype.
//!
//! # Hierarchy Invariants
//!
//! The abstraction hierarchy enforces:
//! - Parent levels are strictly less than child levels.
//! - No cycles in parent-child relationships.
//! - Branching factor limits the number of children per node.
//!
//! # Examples
//!
//! ```
//! use breakthrough::perception::abstraction::{CategoryFormation, PrototypeFormation, AbstractionHierarchy};
//!
//! let mut cf = CategoryFormation::new(2, 0.7).expect("valid");
//! let cat_id = cf.add_percept(vec![0.1, 0.2, 0.3]).expect("added");
//!
//! let mut pf = PrototypeFormation::new(0.1).expect("valid");
//! let proto_id = pf.form_prototype(&[vec![0.1, 0.2], vec![0.15, 0.25]]).expect("formed");
//!
//! let mut hier = AbstractionHierarchy::new(3, 2).expect("valid");
//! let node_id = hier.add_node(0, vec![0.5, 0.5], "root".into()).expect("added");
//! ```
//!
//! # Trait Layer
//!
//! Each component implements a corresponding trait in this module:
//! - [`CategoryFormation`]
//! - [`PrototypeFormation`]
//! - [`AbstractionHierarchy`]
//!
//! These traits enable polymorphic use across different abstraction backends.
//!
//! [`CategoryFormation`]: CategoryFormation
//! [`PrototypeFormation`]: PrototypeFormation
//! [`AbstractionHierarchy`]: AbstractionHierarchy

pub mod category_formation;
pub mod prototype;
pub mod hierarchy;

pub use category_formation::{CategoryFormation, Category};
pub use prototype::{PrototypeFormation, Prototype};
pub use hierarchy::{AbstractionHierarchy, HierarchyNode};
pub use super::{PerceptionResult, PerceptionError};

/// Default minimum category membership.
pub const DEFAULT_MIN_MEMBERS: usize = 2;
/// Default similarity threshold for category assignment.
pub const DEFAULT_SIMILARITY_THRESHOLD: f64 = 0.7;
/// Default prototype update rate.
pub const DEFAULT_UPDATE_RATE: f64 = 0.1;
/// Default hierarchy levels.
pub const DEFAULT_HIERARCHY_LEVELS: usize = 3;
/// Default branching factor for hierarchy nodes.
pub const DEFAULT_BRANCHING_FACTOR: usize = 2;

/// Validates that an abstraction pipeline configuration is consistent.
///
/// # Errors
///
/// Returns [`PerceptionError`] if any component configuration is invalid.
pub fn validate_abstraction_pipeline(
    min_members: usize,
    similarity_threshold: f64,
    update_rate: f64,
    levels: usize,
    branching_factor: usize,
) -> PerceptionResult<()> {
    CategoryFormation::new(min_members, similarity_threshold)?.validate()?;
    PrototypeFormation::new(update_rate)?.validate()?;
    AbstractionHierarchy::new(levels, branching_factor)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pipeline_validation_succeeds() {
        assert!(validate_abstraction_pipeline(2, 0.7, 0.1, 3, 2).is_ok());
    }

    #[test]
    fn pipeline_validation_rejects_bad() {
        assert!(validate_abstraction_pipeline(0, 0.7, 0.1, 3, 2).is_err());
    }
}
