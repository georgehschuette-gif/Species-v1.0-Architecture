// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use crate::genesis::{GenesisError, GenesisResult};

/// Identity Axiom: Every entity maintains a persistent, unique identity
/// across all transformations.
///
/// This axiom ensures that cognitive entities retain their identity through
/// state changes, transformations, and interactions. The identity value
/// serves as a definitive, immutable key that distinguishes one entity
/// from all others.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IdentityAxiom {
    /// The unique identifier for this identity.
    pub id: u64,
}

impl IdentityAxiom {
    /// The zero identity, used as a sentinel value for "no entity".
    pub const NULL_ID: u64 = 0;

    /// Creates a new Identity Axiom with the given identifier.
    ///
    /// # Errors
    ///
    /// Returns [`GenesisError::OutOfRange`] if `id` is zero (the null sentinel).
    pub fn new(id: u64) -> GenesisResult<Self> {
        if id == Self::NULL_ID {
            return Err(GenesisError::OutOfRange {
                field: "id".to_string(),
                value: 0.0,
                min: 1.0,
                max: u64::MAX as f64,
            });
        }
        Ok(Self { id })
    }

    /// Creates an identity without validation, intended for internal use
    /// where the caller guarantees the id is non-zero.
    pub fn new_unchecked(id: u64) -> Self {
        Self { id }
    }

    /// Returns whether this identity is the null (zero) identity.
    pub fn is_null(&self) -> bool {
        self.id == Self::NULL_ID
    }

    /// Returns whether this identity is valid (non-zero).
    pub fn is_valid(&self) -> bool {
        self.id != Self::NULL_ID
    }

    /// Compares this identity with another, returning an ordering.
    pub fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }

    /// Computes a hash-based distance between this identity and another.
    pub fn distance(&self, other: &Self) -> u64 {
        self.id.abs_diff(other.id)
    }

    /// Returns the next identity in sequence.
    ///
    /// Returns `None` if the id is already at `u64::MAX`.
    pub fn next(&self) -> Option<Self> {
        if self.id == u64::MAX {
            None
        } else {
            Some(Self { id: self.id + 1 })
        }
    }
}

impl Default for IdentityAxiom {
    fn default() -> Self {
        Self { id: Self::NULL_ID }
    }
}