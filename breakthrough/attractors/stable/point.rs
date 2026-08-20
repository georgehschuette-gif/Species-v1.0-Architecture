// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! # Point Attractor
//!
//! An [`AttractorPoint`] represents a stable point in phase space toward
//! which trajectories converge. It is the simplest form of attractor,
//! corresponding to a fixed point in continuous time or a fixed state in
//! discrete mappings.
//!
//! ## Stability Classification
//!
//! Stability is determined by examining the eigenvalues of the Jacobian
//! evaluated at the point. In continuous systems, all eigenvalues must have
//! negative real parts for asymptotic stability. In discrete systems, all
//! eigenvalues must satisfy |λ| < 1.
//!
//! ## Coordinate Representation
//!
//! Coordinates are stored as a dense `Vec<f64>` and support Euclidean
//! distance computation, perturbation analysis, and state evolution under
//! linear approximations.

use std::cmp::PartialEq;
use std::fmt::{self, Display, Formatter};

/// Stability classification for a point attractor.
#[derive(Debug, Clone, Copy, Hash)]
pub enum Stability {
    /// All perturbations decay exponentially.
    AsymptoticallyStable,
    /// Perturbations neither grow nor decay (marginal).
    LyapunovStable,
    /// Some perturbations grow unbounded.
    Unstable,
    /// Stability depends on higher-order terms or context.
    Conditional,
}

impl Display for Stability {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::AsymptoticallyStable => "asymptotically_stable",
            Self::LyapunovStable => "lyapunov_stable",
            Self::Unstable => "unstable",
            Self::Conditional => "conditional",
        })
    }
}

/// A stable point attractor in phase space.
///
/// # Fields
///
/// * `coordinates` - Position vector in the phase space.
/// * `stability` - Stability classification of the fixed point.
/// * `basin_radius` - Estimated radius of the basin of attraction.
/// * `eigenvalues` - Spectral data from local linearization.
#[derive(Debug, Clone, PartialEq)]
pub struct AttractorPoint {
    pub coordinates: Vec<f64>,
    pub stability: Stability,
    pub basin_radius: f64,
    pub eigenvalues: Vec<f64>,
}

impl AttractorPoint {
    /// Creates a new point attractor with the given coordinates and stability.
    ///
    /// # Arguments
    ///
    /// * `coordinates` - Initial position vector.
    /// * `stability` - Stability classification.
    /// * `basin_radius` - Estimated radius of attraction basin.
    /// * `eigenvalues` - Jacobian eigenvalues at the fixed point.
    ///
    /// # Panics
    ///
    /// Panics if `coordinates` is empty or `basin_radius` is negative.
    pub fn new(
        coordinates: Vec<f64>,
        stability: Stability,
        basin_radius: f64,
        eigenvalues: Vec<f64>,
    ) -> Self {
        assert!(!coordinates.is_empty(), "coordinates must be non-empty");
        assert!(basin_radius >= 0.0, "basin_radius must be non-negative");
        Self { coordinates, stability, basin_radius, eigenvalues }
    }

    /// Returns the spatial dimension of the attractor.
    pub fn dimension(&self) -> usize {
        self.coordinates.len()
    }

    /// Computes the Euclidean distance from this point to another point.
    ///
    /// # Arguments
    ///
    /// * `other` - The target point attractor.
    ///
    /// # Panics
    ///
    /// Panics if the two points have different dimensions.
    pub fn distance_to(&self, other: &Self) -> f64 {
        assert_eq!(self.coordinates.len(), other.coordinates.len(), "dimension mismatch");
        self.coordinates
            .iter()
            .zip(&other.coordinates)
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt()
    }

    /// Determines whether this point is stable in the asymptotic sense.
    ///
    /// Returns `true` only for `AsymptoticallyStable` classification.
    pub fn is_stable(&self) -> bool {
        matches!(self.stability, Stability::AsymptoticallyStable)
    }

    /// Evolves the point by a small perturbation under a linear approximation.
    ///
    /// The perturbation is multiplied by the largest (least negative) real
    /// part of the eigenvalue spectrum to approximate one integration step.
    ///
    /// # Arguments
    ///
    /// * `perturbation` - A small displacement vector.
    /// * `dt` - Time step for the integration.
    ///
    /// # Returns
    ///
    /// The perturbed coordinates after one linear step.
    ///
    /// # Panics
    ///
    /// Panics if `perturbation` length does not match point dimension.
    pub fn evolve(&self, perturbation: &[f64], dt: f64) -> Vec<f64> {
        assert_eq!(perturbation.len(), self.coordinates.len(), "dimension mismatch");
        let max_eigenvalue = self.eigenvalues.iter().cloned().fold(f64::NAN, f64::max);
        let decay = (max_eigenvalue * dt).exp();
        perturbation.iter().zip(&self.coordinates).map(|(p, c)| c + p * decay).collect()
    }

    /// Returns the dominant eigenvalue magnitude.
    ///
    /// This is the eigenvalue with the largest real part (continuous) or
    /// largest magnitude (discrete), depending on the context.
    pub fn dominant_eigenvalue(&self) -> f64 {
        self.eigenvalues.iter().cloned().fold(f64::NAN, |a, b| if a > b { a } else { b })
    }

    /// Checks whether a query point lies within the basin of attraction.
    ///
    /// # Arguments
    ///
    /// * `query` - The candidate initial condition.
    ///
    /// # Returns
    ///
    /// `true` if the Euclidean distance is less than `basin_radius`.
    pub fn in_basin(&self, query: &[f64]) -> bool {
        assert_eq!(query.len(), self.coordinates.len(), "dimension mismatch");
        let dist_sq: f64 = query.iter().zip(&self.coordinates).map(|(q, c)| (q - c).powi(2)).sum();
        dist_sq.sqrt() <= self.basin_radius
    }

    /// Returns a human-readable summary string.
    pub fn summary(&self) -> String {
        format!(
            "AttractorPoint(dim={}, stability={}, basin_r={:.4})",
            self.dimension(),
            self.stability,
            self.basin_radius
        )
    }
}

impl Display for AttractorPoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AttractorPoint(coords=[{}], stability={})",
            self.coordinates.iter().map(|c| format!("{:.6}", c)).collect::<Vec<_>>().join(", "),
            self.stability
        )
    }
}

impl PartialEq for Stability {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::AsymptoticallyStable, Self::AsymptoticallyStable)
                | (Self::LyapunovStable, Self::LyapunovStable)
                | (Self::Unstable, Self::Unstable)
                | (Self::Conditional, Self::Conditional)
        )
    }
}

impl Eq for Stability {}
