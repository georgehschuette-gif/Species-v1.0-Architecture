// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! Identity: The persistent self-referential structure of the ecosystem.
//!
//! Identity operations maintain a coherent sense of self across time,
//! state changes, and conceptual drift.
//!
//! 1. **Continuity** — Anchors core identity features against drift,
//!    ensuring the concept remains "itself" despite superficial changes.
//! 2. **Self-Reference** — Tracks the depth of self-examination and
//!    coherence of self-referential data structures.
//! 3. **Constancy** — Protects designated core features from external
//!    modification above a protective threshold.
//!
//! # Identity Anchors
//!
//! An identity anchor is a stable reference value around which the system
//! calibrates its self-model. Continuity strength resists drift away from
//! this anchor.
//!
//! # Core Feature Protection
//!
//! Core features are immutable or semi-immutable by default. The constancy
//! threshold determines whether modifications are permitted.
//!
//! # Examples
//!
//! ```
//! use breakthrough::cognition::identity::{IdentityContinuity, SelfReference, IdentityConstancy};
//!
//! let mut continuity = IdentityContinuity::new(0.8, 10.0).expect("valid");
//! let preserved = continuity.maintain(0.6).expect("preserved");
//!
//! let mut sref = SelfReference::new(3, 0.9).expect("valid");
//! sref.deepen().expect("deepened");
//!
//! let constancy = IdentityConstancy::new(0.7, vec![1, 2, 3]).expect("valid");
//! assert!(constancy.is_core(1));
//! ```
//!
//! # Consistency Guarantees
//!
//! Identity operations are fail-closed: if a validation check cannot
//! complete, the operation is rejected rather than accepting a potentially
//! corrupted self-state.
//!
//! [`CognitionError`]: super::CognitionError

pub mod continuity;
pub mod self_reference;
pub mod constancy;

pub use super::CognitionError;
pub use continuity::IdentityContinuity;
pub use self_reference::SelfReference;
pub use constancy::IdentityConstancy;

/// Default continuity strength used in self-model initialization.
pub const DEFAULT_CONTINUITY_STRENGTH: f64 = 0.8;
/// Default memory window (in abstract time units).
pub const DEFAULT_MEMORY_WINDOW: f64 = 10.0;
/// Maximum reference depth for self-reference operations.
pub const MAX_REFERENCE_DEPTH: usize = 100;
/// Maximum core features protected by identity constancy.
pub const MAX_CORE_FEATURES: usize = 1000;

/// Creates a complete identity bundle: continuity, self-reference, and constancy.
///
/// Convenience constructor for initializing a new identity module with
/// sensible defaults.
///
/// # Errors
///
/// Returns [`CognitionError::OutOfRange`] if parameters are invalid.
pub fn create_identity(
    core_features: Vec<u64>,
) -> Result<
    (IdentityContinuity, SelfReference, IdentityConstancy),
    CognitionError,
> {
    let continuity = IdentityContinuity::new(DEFAULT_CONTINUITY_STRENGTH, DEFAULT_MEMORY_WINDOW)?;
    let sref = SelfReference::new(1, 1.0)?;
    let constancy = IdentityConstancy::new(0.7, core_features)?;
    Ok((continuity, sref, constancy))
}

/// Validates the integrity of an identity configuration.
pub fn validate_identity(
    continuity: &IdentityContinuity,
    sref: &SelfReference,
    constancy: &IdentityConstancy,
) -> Result<(), CognitionError> {
    continuity.validate()?;
    sref.validate()?;
    constancy.validate()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_identity_succeeds() {
        let bundle = create_identity(vec![1, 2, 3]).unwrap();
        assert_eq!(bundle.0.continuity_strength, DEFAULT_CONTINUITY_STRENGTH);
        assert_eq!(bundle.2.core_count(), 3);
    }

    #[test]
    fn validate_identity_rejects_bad() {
        let bad_continuity = IdentityContinuity::new(-0.1, 10.0).unwrap();
        let sref = SelfReference::new(0, 1.0).unwrap();
        let constancy = IdentityConstancy::new(0.7, vec![1]).unwrap();
        assert!(validate_identity(&bad_continuity, &sref, &constancy).is_err());
    }
}