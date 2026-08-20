// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! # Latent Attractor
//!
//! A [`LatentAttractor`] is a dynamical structure that exists mathematically
//! but is not yet the dominant attractor due to an activation barrier. It
//! becomes observable only when the system state crosses a threshold or
//! receives sufficient perturbation energy.
//!
//! ## Activation Barrier
//!
//! The barrier height determines the metastable lifetime. Higher barriers
//! correspond to longer dormancy and lower spontaneous activation rates.
//!
//! ## Tunneling
//!
/// In quantum or stochastic regimes, latent attractors can be activated
/// via tunneling or thermal activation, with rates governed by Arrhenius
/// or Kramers laws.

use std::cmp::PartialEq;
use std::fmt::{self, Display, Formatter};

/// Activation mechanism for a latent attractor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActivationMechanism {
    /// Over-the-barrier thermal activation.
    Thermal,
    /// Quantum tunneling through the barrier.
    QuantumTunneling,
    /// Noise-induced escape from metastable well.
    NoiseInduced,
    /// Deterministic threshold crossing.
    Deterministic,
    /// External perturbation or kick.
    Perturbative,
    /// Unknown or mixed mechanism.
    Mixed,
}

impl Display for ActivationMechanism {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Thermal => "thermal",
            Self::QuantumTunneling => "quantum_tunneling",
            Self::NoiseInduced => "noise_induced",
            Self::Deterministic => "deterministic",
            Self::Perturbative => "perturbative",
            Self::Mixed => "mixed",
        })
    }
}

/// A latent attractor with activation barrier and tunneling properties.
///
/// # Fields
///
/// * `potential` - Potential energy value at the attractor state.
/// * `activation_energy` - Barrier height separating current state from
///   the attractor.
/// * `barrier_width` - Spatial width of the activation barrier.
/// * `mechanism` - Predominant activation mechanism.
/// * `attempt_frequency` - Attempt frequency for Kramers escape.
/// * `temperature` - Thermal energy scale k_B T.
/// * `mass` - Effective inertia or mass (for quantum tunneling).
/// * `is_activatable` - Whether conditions permit activation.
/// * `tunneling_prob` - Quantum tunneling probability.
#[derive(Debug, Clone, PartialEq)]
pub struct LatentAttractor {
    pub potential: f64,
    pub activation_energy: f64,
    pub barrier_width: f64,
    pub mechanism: ActivationMechanism,
    pub attempt_frequency: f64,
    pub temperature: f64,
    pub mass: f64,
    pub is_activatable: bool,
    pub tunneling_prob: f64,
}

impl LatentAttractor {
    /// Creates a new latent attractor with barrier properties.
    ///
    /// # Arguments
    ///
    /// * `potential` - Potential at the attractor.
    /// * `activation_energy` - Barrier height (must be non-negative).
    /// * `barrier_width` - Width of the barrier region.
    /// * `mechanism` - Activation mechanism.
    /// * `attempt_frequency` - Attempt frequency.
    /// * `temperature` - Thermal energy scale.
    /// * `mass` - Effective mass for quantum estimates.
    ///
    /// # Panics
    ///
    /// Panics if `activation_energy` is negative, or `attempt_frequency`
    /// or `temperature` are non-positive.
    pub fn new(
        potential: f64,
        activation_energy: f64,
        barrier_width: f64,
        mechanism: ActivationMechanism,
        attempt_frequency: f64,
        temperature: f64,
        mass: f64,
    ) -> Self {
        assert!(activation_energy >= 0.0, "activation_energy must be non-negative");
        assert!(attempt_frequency > 0.0, "attempt_frequency must be positive");
        assert!(temperature > 0.0, "temperature must be positive");
        let is_activatable = activation_energy > 0.0;
        let mut result = Self {
            potential,
            activation_energy,
            barrier_width,
            mechanism,
            attempt_frequency,
            temperature,
            mass,
            is_activatable,
            tunneling_prob: 0.0,
        };
        result.tunneling_prob = result.wkb_tunneling_prob(activation_energy, barrier_width, mass);
        result
    }

    /// Returns the spatial dimension (always 1 for 1D barrier models).
    pub fn dimension(&self) -> usize {
        1
    }

    /// Computes the WKB tunneling probability through a rectangular
    /// barrier.
    ///
    /// # Arguments
    ///
    /// * `energy` - Particle energy.
    /// * `width` - Barrier width.
    /// * `mass` - Particle mass.
    fn wkb_tunneling_prob(&self, energy: f64, width: f64, mass: f64) -> f64 {
        if energy >= self.activation_energy { return 1.0; }
        let kappa = (2.0 * mass * (self.activation_energy - energy)).sqrt();
        (-2.0 * kappa * width).exp()
    }

    /// Computes the Arrhenius activation rate.
    ///
    /// # Arguments
    ///
    /// * `energy` - Thermal energy available for activation.
    pub fn arrhenius_rate(&self, energy: f64) -> f64 {
        if self.activation_energy <= 0.0 {
            return self.attempt_frequency;
        }
        let exponent = -self.activation_energy / energy.max(1e-12);
        self.attempt_frequency * exponent.exp()
    }

    /// Estimates the mean first-passage time (activation time) under
    /// thermal activation.
    ///
    /// # Arguments
    ///
    /// * `kbt` - Thermal energy k_B T.
    pub fn activation_time(&self, kbt: f64) -> f64 {
        if self.activation_energy <= 0.0 { return 0.0; }
        let rate = self.arrhenius_rate(kbt);
        if rate > 1e-12 { 1.0 / rate } else { f64::INFINITY }
    }

    /// Checks whether the attractor can be activated under current
    /// conditions.
    pub fn is_activatable(&self) -> bool {
        self.is_activatable && self.activation_energy > 0.0
    }

    /// Updates the tunneling probability for a given particle energy.
    ///
    /// # Arguments
    ///
    /// * `energy` - Particle energy.
    pub fn update_tunneling_prob(&mut self, energy: f64) {
        self.tunneling_prob = self.wkb_tunneling_prob(energy, self.barrier_width, self.mass);
        self.is_activatable = self.tunneling_prob > 1e-12 || self.activation_energy == 0.0;
    }

    /// Checks whether quantum effects dominate over thermal activation.
    pub fn is_quantum(&self) -> bool {
        let thermal_length = (self.temperature / self.mass).sqrt();
        let barrier_length = self.barrier_width;
        thermal_length < barrier_length * 0.1
    }

    /// Computes the barrier transparency for a particle of given energy.
    pub fn transparency(&self, energy: f64) -> f64 {
        if energy >= self.activation_energy { 1.0 } else { self.tunneling_prob }
    }

    /// Returns a summary string.
    pub fn summary(&self) -> String {
        format!(
            "LatentAttractor(V={:.4}, Ea={:.4}, mechanism={}, T={:.4})",
            self.potential,
            self.activation_energy,
            self.mechanism,
            self.temperature
        )
    }
}

impl Display for LatentAttractor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "LatentAttractor(potential={:.6}, activation_energy={:.6}, mechanism={})",
            self.potential, self.activation_energy, self.mechanism
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arrhenius_rate() {
        let la = LatentAttractor::new(1.0, 2.0, 1.0, ActivationMechanism::Thermal, 1e12, 1.0, 1.0);
        let rate = la.arrhenius_rate(1.0);
        assert!(rate.is_finite() && rate > 0.0);
    }

    #[test]
    fn test_is_activatable() {
        let la = LatentAttractor::new(0.0, 0.0, 1.0, ActivationMechanism::Deterministic, 1.0, 1.0, 1.0);
        assert!(!la.is_activatable());
    }

    #[test]
    fn test_transparency() {
        let la = LatentAttractor::new(0.0, 1.0, 1.0, ActivationMechanism::QuantumTunneling, 1.0, 1.0, 1.0);
        assert!((la.transparency(2.0) - 1.0).abs() < 1e-10);
    }
}
