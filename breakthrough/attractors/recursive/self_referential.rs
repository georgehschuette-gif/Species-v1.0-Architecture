// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! # Self-Referential Attractor
//!
//! A [`SelfReferential`] is a dynamical map that transforms its own
//! representation. It appears in formal systems, reflexively adaptive
//! control, and self-modifying software architectures. Self-referential
//! maps require a fixed-point condition for consistency, leading to
//! rich mathematical structure including Kleene's recursion theorem and
//! Gödelian incompleteness phenomena.
//!
//! ## Fixed-Point Conditions
//!
//! A self-referential map `F` satisfies `F(F) = F`. This is a
//! higher-order fixed point where the map acts on its own definition.
//!
//! ## Recursion Depth
//!
//! Practical implementations bound recursion depth to ensure termination
//! and support staged or lazy evaluation strategies.

use std::cmp::PartialEq;
use std::fmt::{self, Display, Formatter};

/// A self-referential attractor that encodes its own transformation.
///
/// # Fields
///
/// * `mapping` - The self-transform function identifier or handle.
/// * `fixed_point` - Optional fixed-point state satisfying `F(F) = F`.
/// * `recursion_depth` - Current depth of self-application.
/// * `max_depth` - Safety bound to prevent infinite descent.
/// * `is_consistent` - Whether the self-reference is logically consistent.
/// * `representation` - Serialized or symbolic representation of `F`.
#[derive(Debug, Clone)]
pub struct SelfReferential {
    pub mapping: String,
    pub fixed_point: Option<Vec<f64>>,
    pub recursion_depth: usize,
    pub max_depth: usize,
    pub is_consistent: bool,
    pub representation: String,
}

impl SelfReferential {
    /// Creates a new self-referential attractor.
    ///
    /// # Arguments
    ///
    /// * `mapping` - Identifier for the self-transform.
    /// * `max_depth` - Maximum recursion depth allowed.
    /// * `representation` - Symbolic or serialized form.
    ///
    /// # Panics
    ///
    /// Panics if `max_depth` is zero.
    pub fn new(mapping: String, max_depth: usize, representation: String) -> Self {
        assert!(max_depth > 0, "max_depth must be positive");
        Self {
            mapping,
            fixed_point: None,
            recursion_depth: 0,
            max_depth,
            is_consistent: true,
            representation,
        }
    }

    /// Returns the current recursion depth.
    pub fn depth(&self) -> usize {
        self.recursion_depth
    }

    /// Applies one level of self-reference, incrementing depth.
    ///
    /// If `max_depth` is reached, the attractor collapses to `None`.
    ///
    /// # Returns
    ///
    /// `Some(Self)` if depth limit not exceeded, `None` otherwise.
    pub fn apply(&mut self) -> Option<Self> {
        if self.recursion_depth >= self.max_depth {
            self.is_consistent = false;
            return None;
        }
        self.recursion_depth += 1;
        Some(self.clone())
    }

    /// Checks whether the attractor has reached a stable self-consistency
    /// condition.
    ///
    /// A mapping is self-consistent when applying it yields an identical
    /// representation.
    pub fn is_self_consistent(&self) -> bool {
        self.is_consistent && self.recursion_depth < self.max_depth
    }

    /// Attempts to converge to a fixed point by repeated self-application.
    ///
    /// # Arguments
    ///
    /// * `state` - Initial state vector.
    /// * `tolerance` - Convergence tolerance.
    ///
    /// # Returns
    ///
    /// Fixed-point state if convergence is achieved, `None` otherwise.
    pub fn converge(&mut self, state: Vec<f64>, tolerance: f64) -> Option<Vec<f64>> {
        if state.is_empty() {
            return None;
        }
        let mut current = state;
        for _ in 0..self.max_depth {
            let next = self.step(&current);
            if (next.iter().zip(&current).map(|(a, b)| (a - b).abs()).sum::<f64>() / current.len() as f64) < tolerance {
                self.fixed_point = Some(next.clone());
                return Some(next);
            }
            current = next;
        }
        self.is_consistent = false;
        None
    }

    /// Performs one self-referential step on a state vector.
    ///
    /// This is a placeholder for the actual mapping logic; in practice,
    /// it would invoke the encoded transformation rule.
    fn step(&self, state: &[f64]) -> Vec<f64> {
        state.to_vec()
    }

    /// Resets recursion depth to zero.
    pub fn reset(&mut self) {
        self.recursion_depth = 0;
        self.is_consistent = true;
    }

    /// Returns the effective recursion depth consumed toward the fixed
    /// point.
    pub fn remaining_depth(&self) -> usize {
        self.max_depth.saturating_sub(self.recursion_depth)
    }

    /// Returns a summary string.
    pub fn summary(&self) -> String {
        format!(
            "SelfReferential(mapping={}, depth={}/{}, consistent={})",
            self.mapping,
            self.recursion_depth,
            self.max_depth,
            self.is_consistent
        )
    }
}

impl Display for SelfReferential {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SelfReferential(mapping={}, depth={}, max_depth={})",
            self.mapping, self.recursion_depth, self.max_depth
        )
    }
}

impl PartialEq for SelfReferential {
    fn eq(&self, other: &Self) -> bool {
        self.mapping == other.mapping
            && self.recursion_depth == other.recursion_depth
            && self.max_depth == other.max_depth
            && self.is_consistent == other.is_consistent
    }
}

impl Eq for SelfReferential {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_depth_limit() {
        let mut sr = SelfReferential::new("f".into(), 2, "repr".into());
        let _ = sr.apply();
        let _ = sr.apply();
        assert!(sr.apply().is_none());
        assert!(!sr.is_self_consistent());
    }

    #[test]
    fn test_convergence() {
        let mut sr = SelfReferential::new("g".into(), 10, "repr".into());
        let fp = sr.converge(vec![1.0, 2.0], 1e-6);
        assert!(fp.is_some());
        assert!(sr.is_self_consistent());
    }

    #[test]
    fn test_reset() {
        let mut sr = SelfReferential::new("h".into(), 5, "repr".into());
        let _ = sr.apply();
        sr.reset();
        assert_eq!(sr.depth(), 0);
        assert!(sr.is_self_consistent());
    }
}
