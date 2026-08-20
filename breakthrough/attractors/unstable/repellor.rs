// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! # Repellor Attractor
//!
//! A [`Repellor`] is an unstable attractor that drives trajectories away
//! from its neighborhood under forward time evolution. Repellors are
//! characterized by positive Lyapunov exponents and divergence rates that
//! grow exponentially with distance from the repellor state.
//!
//! ## Dynamical Role
//!
//! Repellors serve as sources in the phase space. They define the
//! boundaries of basins of attraction and are essential in bifurcation
//! analysis where stability is lost.
//!
//! ## Metrics
//!
//! - **Lyapunov exponent**: Average exponential divergence rate.
//! - **Divergence rate**: Local exponential growth factor.
//! - **Manifold dimension**: Dimensionality of the unstable manifold.

use std::cmp::PartialEq;
use std::fmt::{self, Display, Formatter};

/// A repellor attractor that pushes trajectories outward.
///
/// # Fields
///
/// * `position` - Coordinates of the repellor in phase space.
/// * `divergence_rate` - Local exponential growth rate λ > 0.
/// * `lyapunov_exponent` - Time-averaged divergence rate.
/// * `manifold_dimension` - Dimension of the unstable manifold.
/// * `activation_threshold` - Minimum perturbation magnitude for
///   divergence to dominate over noise or damping.
/// * `gradient` - Cached gradient of the vector field at the repellor.
#[derive(Debug, Clone)]
pub struct Repellor {
    pub position: Vec<f64>,
    pub divergence_rate: f64,
    pub lyapunov_exponent: f64,
    pub manifold_dimension: usize,
    pub activation_threshold: f64,
    pub gradient: Vec<f64>,
}

impl Repellor {
    /// Creates a new repellor with divergence and manifold metadata.
    ///
    /// # Arguments
    ///
    /// * `position` - Repellor coordinates.
    /// * `divergence_rate` - Local exponential growth rate (must be positive).
    /// * `lyapunov_exponent` - Time-averaged exponent.
    /// * `manifold_dimension` - Unstable manifold dimension.
    /// * `activation_threshold` - Minimum perturbation to observe divergence.
    /// * `gradient` - Vector field gradient at the repellor.
    ///
    /// # Panics
    ///
    /// Panics if `divergence_rate` is non-positive or if dimension checks
    /// fail.
    pub fn new(
        position: Vec<f64>,
        divergence_rate: f64,
        lyapunov_exponent: f64,
        manifold_dimension: usize,
        activation_threshold: f64,
        gradient: Vec<f64>,
    ) -> Self {
        assert!(divergence_rate > 0.0, "repellor requires positive divergence_rate");
        assert!(!position.is_empty(), "position must be non-empty");
        assert_eq!(gradient.len(), position.len(), "gradient dimension must match position");
        assert!(manifold_dimension <= position.len(), "manifold dimension cannot exceed phase space");
        Self {
            position,
            divergence_rate,
            lyapunov_exponent,
            manifold_dimension,
            activation_threshold,
            gradient,
        }
    }

    /// Returns the spatial dimension.
    pub fn dimension(&self) -> usize {
        self.position.len()
    }

    /// Checks whether the repellor is actively diverging from a given
    /// perturbation magnitude.
    ///
    /// # Arguments
    ///
    /// * `perturbation_magnitude` - Euclidean norm of the perturbation.
    pub fn is_active(&self, perturbation_magnitude: f64) -> bool {
        perturbation_magnitude >= self.activation_threshold
    }

    /// Computes the expected separation distance after time `t` from an
    /// initial separation `d0`.
    ///
    /// # Arguments
    ///
    /// * `d0` - Initial separation distance.
    /// * `t` - Time elapsed.
    pub fn divergence(&self, d0: f64, t: f64) -> f64 {
        d0 * (self.divergence_rate * t).exp()
    }

    /// Computes the Lyapunov exponent for a given trajectory sample.
    ///
    /// This approximates the time-averaged exponent from a finite sample
    /// of separation distances.
    ///
    /// # Arguments
    ///
    /// * `separations` - Log-scaled separation samples over time.
    pub fn lyapunov_exponent_from_samples(&self, separations: &[f64]) -> f64 {
        if separations.is_empty() {
            return f64::NAN;
        }
        let sum_log = separations.iter().map(|s| s.ln()).sum::<f64>();
        sum_log / separations.len() as f64
    }

    /// Generates a synthetic trajectory emanating from the repellor.
    ///
    /// This models ideal exponential divergence along the unstable
    /// manifold direction.
    ///
    /// # Arguments
    ///
    /// * `initial_perturbation` - Starting displacement vector.
    /// * `t` - Time values to evaluate.
    ///
    /// # Returns
    ///
    /// Vector of trajectory points sampled at each time value.
    ///
    /// # Panics
    ///
    /// Panics if `initial_perturbation` dimension mismatches or `t` is
    /// empty.
    pub fn trajectory(&self, initial_perturbation: &[f64], t: &[f64]) -> Vec<Vec<f64>> {
        assert_eq!(initial_perturbation.len(), self.position.len(), "dimension mismatch");
        assert!(!t.is_empty(), "time vector must be non-empty");
        t.iter()
            .map(|&ti| {
                let scale = (self.divergence_rate * ti).exp();
                self.position
                    .iter()
                    .zip(initial_perturbation)
                    .map(|(p, q)| p + q * scale)
                    .collect()
            })
            .collect()
    }

    /// Computes the Jacobian trace at the repellor, related to phase-space
    /// volume contraction or expansion.
    pub fn trace(&self) -> f64 {
        self.gradient.iter().sum()
    }

    /// Returns the unstable manifold dimension.
    pub fn manifold_dim(&self) -> usize {
        self.manifold_dimension
    }

    /// Returns a human-readable summary.
    pub fn summary(&self) -> String {
        format!(
            "Repellor(dim={}, λ={:.4}, LE={:.4}, manifold_dim={})",
            self.dimension(),
            self.divergence_rate,
            self.lyapunov_exponent,
            self.manifold_dimension
        )
    }
}

impl Display for Repellor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Repellor(pos=[{}], divergence_rate={:.6}, lyapunov={:.6})",
            self.position.iter().map(|c| format!("{:.6}", c)).collect::<Vec<_>>().join(", "),
            self.divergence_rate,
            self.lyapunov_exponent
        )
    }
}

impl PartialEq for Repellor {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
            && (self.divergence_rate - other.divergence_rate).abs() < 1e-12
            && (self.lyapunov_exponent - other.lyapunov_exponent).abs() < 1e-12
            && self.manifold_dimension == other.manifold_dimension
    }
}

impl Eq for Repellor {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divergence_growth() {
        let rep = Repellor::new(vec![0.0, 0.0], 0.5, 0.5, 2, 0.1, vec![1.0, 0.0]);
        let d1 = rep.divergence(1.0, 2.0);
        assert!(d1 > 1.0);
        assert!((d1 - (0.5 * 2.0f64).exp()).abs() < 1e-10);
    }

    #[test]
    fn test_is_active() {
        let rep = Repellor::new(vec![0.0], 1.0, 1.0, 1, 0.5, vec![1.0]);
        assert!(rep.is_active(0.6));
        assert!(!rep.is_active(0.3));
    }

    #[test]
    fn test_trajectory_length() {
        let rep = Repellor::new(vec![0.0, 0.0], 1.0, 1.0, 2, 0.1, vec![1.0, 0.0]);
        let times = vec![0.0, 0.5, 1.0];
        let traj = rep.trajectory(&[1.0, 1.0], &times);
        assert_eq!(traj.len(), 3);
        assert_eq!(traj[0].len(), 2);
    }
}
