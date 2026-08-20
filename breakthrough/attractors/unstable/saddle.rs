// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! # Saddle Point Attractor
//!
//! A [`SaddlePoint`] is a hybrid unstable structure possessing both stable
//! and unstable manifolds. Trajectories approach along the stable
//! directions while diverging along the unstable directions. Saddle points
//! are organizing centers in phase space and govern the topology of
//! separatrix boundaries.
//!
//! ## Manifold Structure
//!
//! - **Stable manifold**: Attracts trajectories; dimension equals the
//!   number of eigenvalues with negative real part.
//! - **Unstable manifold**: Repels trajectories; dimension equals the
//!   number of eigenvalues with positive real part.
//!
//! ## Index Theory
//!
//! The Morse index of a saddle point is the dimension of its unstable
/// manifold. Index-1 saddles are particularly important in optimization
/// and transition state theory.

use std::cmp::PartialEq;
use std::fmt::{self, Display, Formatter};

/// A saddle point attractor with stable and unstable manifolds.
///
/// # Fields
///
/// * `position` - Coordinates of the saddle point.
/// * `stable_manifold` - Basis vectors spanning the stable subspace.
/// * `unstable_manifold` - Basis vectors spanning the unstable subspace.
/// * `index` - Morse index (dimension of the unstable manifold).
/// * `eigenvalues` - Jacobian eigenvalues (real parts).
/// * `energy` - Potential energy value at the saddle (if applicable).
#[derive(Debug, Clone, PartialEq)]
pub struct SaddlePoint {
    pub position: Vec<f64>,
    pub stable_manifold: Vec<Vec<f64>>,
    pub unstable_manifold: Vec<Vec<f64>>,
    pub index: usize,
    pub eigenvalues: Vec<f64>,
    pub energy: f64,
}

impl SaddlePoint {
    /// Creates a new saddle point with manifold bases and spectral data.
    ///
    /// # Arguments
    ///
    /// * `position` - Saddle coordinates.
    /// * `stable_manifold` - Basis for stable subspace.
    /// * `unstable_manifold` - Basis for unstable subspace.
    /// * `eigenvalues` - Jacobian real eigenvalues.
    /// * `energy` - Potential energy at the saddle.
    ///
    /// # Panics
    ///
    /// Panics if dimensions are inconsistent or if stable + unstable
    /// manifold dimensions do not sum to the phase-space dimension.
    pub fn new(
        position: Vec<f64>,
        stable_manifold: Vec<Vec<f64>>,
        unstable_manifold: Vec<Vec<f64>>,
        eigenvalues: Vec<f64>,
        energy: f64,
    ) -> Self {
        let dim = position.len();
        assert!(!position.is_empty(), "position must be non-empty");
        assert_eq!(
            stable_manifold.len() + unstable_manifold.len(),
            dim,
            "manifold dimensions must sum to phase-space dimension"
        );
        for basis in &stable_manifold {
            assert_eq!(basis.len(), dim, "stable manifold basis dimension mismatch");
        }
        for basis in &unstable_manifold {
            assert_eq!(basis.len(), dim, "unstable manifold basis dimension mismatch");
        }
        assert_eq!(eigenvalues.len(), dim, "eigenvalue count must match dimension");
        let index = unstable_manifold.len();
        Self {
            position,
            stable_manifold,
            unstable_manifold,
            index,
            eigenvalues,
            energy,
        }
    }

    /// Returns the phase-space dimension.
    pub fn dimension(&self) -> usize {
        self.position.len()
    }

    /// Returns the Morse index (dimension of unstable manifold).
    pub fn index(&self) -> usize {
        self.index
    }

    /// Returns a basis vector for the stable manifold at the given index.
    ///
    /// # Panics
    ///
    /// Panics if `i` is out of bounds.
    pub fn stable_direction(&self, i: usize) -> &[f64] {
        &self.stable_manifold[i]
    }

    /// Returns a basis vector for the unstable manifold at the given index.
    ///
    /// # Panics
    ///
    /// Panics if `i` is out of bounds.
    pub fn unstable_direction(&self, i: usize) -> &[f64] {
        &self.unstable_manifold[i]
    }

    /// Computes the trajectory distance from a query point to the saddle
    /// along a specified manifold direction.
    ///
    /// # Arguments
    ///
    /// * `query` - Point in phase space.
    /// * `direction` - Manifold direction vector (need not be unit).
    ///
    /// # Returns
    ///
    /// Projected distance scalar along the direction.
    ///
    /// # Panics
    ///
    /// Panics on dimension mismatch.
    pub fn trajectory_distance(&self, query: &[f64], direction: &[f64]) -> f64 {
        assert_eq!(query.len(), self.position.len(), "dimension mismatch");
        assert_eq!(direction.len(), self.position.len(), "dimension mismatch");
        let diff: Vec<f64> = query.iter().zip(&self.position).map(|(q, p)| q - p).collect();
        let dot: f64 = diff.iter().zip(direction).map(|(d, v)| d * v).sum();
        let norm_sq: f64 = direction.iter().map(|v| v.powi(2)).sum();
        if norm_sq < 1e-12 { 0.0 } else { dot / norm_sq.sqrt() }
    }

    /// Determines whether the saddle point is a transition state (index-1
    /// saddle).
    pub fn is_transition_state(&self) -> bool {
        self.index == 1
    }

    /// Computes the ratio of unstable to stable manifold dimensions.
    pub fn instability_ratio(&self) -> f64 {
        let stable_dim = self.stable_manifold.len();
        if stable_dim == 0 { f64::INFINITY } else { self.index as f64 / stable_dim as f64 }
    }

    /// Projects a query point onto the stable manifold subspace.
    ///
    /// # Arguments
    ///
    /// * `query` - Candidate point in phase space.
    ///
    /// # Returns
    ///
    /// The closest point on the stable manifold in a least-squares sense.
    pub fn project_onto_stable(&self, query: &[f64]) -> Vec<f64> {
        assert_eq!(query.len(), self.position.len(), "dimension mismatch");
        let mut projection = self.position.clone();
        for basis in &self.stable_manifold {
            let dot: f64 = query.iter().zip(basis).map(|(q, b)| q * b).sum();
            let norm_sq: f64 = basis.iter().map(|v| v.powi(2)).sum();
            let coeff = dot / norm_sq;
            for (p, b) in projection.iter_mut().zip(basis) {
                *p += coeff * b;
            }
        }
        projection
    }

    /// Returns a summary string.
    pub fn summary(&self) -> String {
        format!(
            "SaddlePoint(index={}, stable_dim={}, unstable_dim={}, energy={:.4})",
            self.index,
            self.stable_manifold.len(),
            self.unstable_manifold.len(),
            self.energy
        )
    }
}

impl Display for SaddlePoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SaddlePoint(pos=[{}], index={}, eigenvalues=[{}])",
            self.position.iter().map(|c| format!("{:.6}", c)).collect::<Vec<_>>().join(", "),
            self.index,
            self.eigenvalues.iter().map(|e| format!("{:.6}", e)).collect::<Vec<_>>().join(", ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transition_state() {
        let saddle = SaddlePoint::new(
            vec![0.0, 0.0],
            vec![vec![0.0, 1.0]],
            vec![vec![1.0, 0.0]],
            vec![-1.0, 1.0],
            2.5,
        );
        assert!(saddle.is_transition_state());
        assert_eq!(saddle.index(), 1);
    }

    #[test]
    fn test_trajectory_distance() {
        let saddle = SaddlePoint::new(
            vec![0.0, 0.0],
            vec![vec![0.0, 1.0]],
            vec![vec![1.0, 0.0]],
            vec![-1.0, 1.0],
            0.0,
        );
        let dist = saddle.trajectory_distance(&[3.0, 4.0], &[1.0, 0.0]);
        assert!((dist - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_instability_ratio() {
        let saddle = SaddlePoint::new(
            vec![0.0, 0.0, 0.0],
            vec![vec![0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0]],
            vec![vec![1.0, 0.0, 0.0]],
            vec![-1.0, -1.0, 1.0],
            0.0,
        );
        assert!((saddle.instability_ratio() - 0.5).abs() < 1e-10);
    }
}
