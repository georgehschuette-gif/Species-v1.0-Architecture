// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! # Explosive Attractor
//!
//! An [`ExplosiveAttractor`] models a singularity or blow-up structure in
//! which trajectories diverge to infinity in finite time. This behavior
//! appears in nonlinear PDEs, population dynamics with Allee effects, and
//! cosmological models with phantom energy.
//!
//! ## Blow-Up Dynamics
//!
//! Explosive attractors are characterized by a finite blow-up time `t*`
//! such that the state norm satisfies `||x(t)|| → ∞` as `t → t*⁻`.
//! The growth is typically super-exponential, following power-law or
//! reciprocal-time singularities.
//!
//! ## Integration
//!
//! Standard numerical integrators fail near blow-up due to stiffness and
//! overflow. Adaptive step-size methods with regularization are required
//! to approach the singularity safely.

use std::cmp::PartialEq;
use std::fmt::{self, Display, Formatter};

/// An explosive attractor with finite-time divergence.
///
/// # Fields
///
/// * `position` - Current coordinates of the explosive center.
/// * `growth_rate` - Initial exponential growth rate parameter α.
/// * `blowup_time` - Estimated finite blow-up time `t*`.
/// * `scaling_exponent` - Power-law scaling exponent `p` such that
///   `||x(t)|| ~ (t* - t)^{-p}`.
/// * `regularization` - Small cutoff to prevent numerical overflow.
/// * `energy` - Current integrated energy or norm of the state.
#[derive(Debug, Clone, PartialEq)]
pub struct ExplosiveAttractor {
    pub position: Vec<f64>,
    pub growth_rate: f64,
    pub blowup_time: f64,
    pub scaling_exponent: f64,
    pub regularization: f64,
    pub energy: f64,
}

impl ExplosiveAttractor {
    /// Creates a new explosive attractor with blow-up parameters.
    ///
    /// # Arguments
    ///
    /// * `position` - Explosive center coordinates.
    /// * `growth_rate` - Initial growth rate α > 0.
    /// * `blowup_time` - Finite blow-up time (must be positive).
    /// * `scaling_exponent` - Power-law exponent p > 0.
    /// * `regularization` - Small positive cutoff for numerical stability.
    ///
    /// # Panics
    ///
    /// Panics if `growth_rate` is non-positive, `blowup_time` is
    /// non-positive, or `regularization` is non-positive.
    pub fn new(
        position: Vec<f64>,
        growth_rate: f64,
        blowup_time: f64,
        scaling_exponent: f64,
        regularization: f64,
    ) -> Self {
        assert!(growth_rate > 0.0, "growth_rate must be positive");
        assert!(blowup_time > 0.0, "blowup_time must be positive");
        assert!(scaling_exponent > 0.0, "scaling_exponent must be positive");
        assert!(regularization > 0.0, "regularization must be positive");
        Self {
            position,
            growth_rate,
            blowup_time,
            scaling_exponent,
            regularization,
            energy: 0.0,
        }
    }

    /// Returns the phase-space dimension.
    pub fn dimension(&self) -> usize {
        self.position.len()
    }

    /// Computes the time remaining until blow-up from the current energy.
    ///
    /// This uses the inverse power-law relation to estimate remaining
    /// lifetime.
    ///
    /// # Arguments
    ///
    /// * `current_energy` - Current state norm or energy.
    ///
    /// # Returns
    ///
    /// Time to blow-up `t* - t` or `NaN` if energy is non-positive.
    pub fn time_to_blowup(&self, current_energy: f64) -> f64 {
        if current_energy <= 0.0 {
            return f64::NAN;
        }
        let p = self.scaling_exponent;
        (self.growth_rate / current_energy).powf(1.0 / p)
    }

    /// Checks whether the attractor is in an explosive regime.
    ///
    /// Returns `true` when the current time is before the blow-up time
    /// and the energy exceeds the regularization threshold.
    pub fn is_explosive(&self, current_time: f64, current_energy: f64) -> bool {
        current_time < self.blowup_time && current_energy > self.regularization
    }

    /// Integrates the state forward by a small time step using an explicit
    /// Euler scheme with regularization.
    ///
    /// # Arguments
    ///
    /// * `dt` - Time step (should be small relative to blow-up time).
    /// * `current_time` - Current simulation time.
    ///
    /// # Returns
    ///
    /// Updated position after the step.
    ///
    /// # Panics
    ///
    /// Panics if `dt` is non-positive.
    pub fn integrate(&mut self, dt: f64, current_time: f64) -> Vec<f64> {
        assert!(dt > 0.0, "dt must be positive");
        let tau_remain = (self.blowup_time - current_time).max(self.regularization);
        let growth = self.growth_rate / (tau_remain.powi(2) + self.regularization);
        let scale = (1.0 + growth * dt).max(self.regularization);
        self.energy = (self.energy + growth * dt).max(self.regularization);
        self.position.iter().map(|p| p * scale).collect()
    }

    /// Computes the instantaneous energy release rate.
    ///
    /// This is the time derivative of the energy norm under the explosive
    /// scaling.
    pub fn energy_release_rate(&self, current_time: f64) -> f64 {
        let tau_remain = (self.blowup_time - current_time).max(self.regularization);
        self.growth_rate * tau_remain.powi(-2)
    }

    /// Returns the characteristic length scale at a given time.
    ///
    /// The scale diverges as `t → blowup_time`.
    ///
    /// # Arguments
    ///
    /// * `current_time` - Time at which to evaluate scale.
    pub fn characteristic_length(&self, current_time: f64) -> f64 {
        let tau_remain = (self.blowup_time - current_time).max(self.regularization);
        (self.growth_rate * tau_remain).max(self.regularization).powf(-1.0 / self.scaling_exponent)
    }

    /// Returns a summary string.
    pub fn summary(&self) -> String {
        format!(
            "ExplosiveAttractor(growth={:.4}, t*={:.4}, p={:.4}, energy={:.4})",
            self.growth_rate, self.blowup_time, self.scaling_exponent, self.energy
        )
    }
}

impl Display for ExplosiveAttractor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ExplosiveAttractor(pos=[{}], growth_rate={:.6}, blowup_time={:.6})",
            self.position.iter().map(|c| format!("{:.6}", c)).collect::<Vec<_>>().join(", "),
            self.growth_rate,
            self.blowup_time
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_to_blowup() {
        let exp = ExplosiveAttractor::new(vec![0.0], 1.0, 2.0, 2.0, 1e-9);
        let t_remain = exp.time_to_blowup(1.0);
        assert!(t_remain > 0.0 && t_remain < 2.0);
    }

    #[test]
    fn test_is_explosive() {
        let exp = ExplosiveAttractor::new(vec![0.0], 1.0, 2.0, 2.0, 1e-9);
        assert!(exp.is_explosive(0.5, 1.5));
        assert!(!exp.is_explosive(3.0, 1.5));
        assert!(!exp.is_explosive(0.5, 1e-12));
    }

    #[test]
    fn test_energy_release_rate() {
        let exp = ExplosiveAttractor::new(vec![0.0], 2.0, 4.0, 2.0, 1e-9);
        let rate = exp.energy_release_rate(2.0);
        assert!(rate.is_finite() && rate > 0.0);
    }
}
