// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! # Potential Well Attractor
//!
//! A [`PotentialWell`] is a localized energy minimum that traps
//! trajectories or particles. It models bound states in quantum mechanics,
//! metastable chemical intermediates, and gravitational wells. Potential
//! wells support discrete bound states and allow quantum or stochastic
//! tunneling to adjacent minima.
//!
//! ## Bound States
//!
//! In a 1D potential well, bound states correspond to discrete energy
//! eigenvalues below the continuum threshold. The ground state energy
//! sets the zero-point energy, while excited states approach the barrier
//! asymptotically.
//!
//! ## Tunneling
//!
//! Particles can tunnel through finite barriers with probabilities
//! determined by the WKB approximation, leading to decay rates and
//! resonance phenomena.

use std::cmp::PartialEq;
use std::fmt::{self, Display, Formatter};

/// Potential well shape functions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WellShape {
    /// Harmonic oscillator: V(x) = ½ k x².
    Harmonic,
    /// Quartic double well: V(x) = a x⁴ - b x².
    DoubleWell { a: f64, b: f64 },
    /// Finite square well with hard walls.
    SquareWell { depth: f64, width: f64 },
    /// Lennard-Jones type well.
    LennardJones { epsilon: f64, sigma: f64 },
    /// Morse potential: V(x) = D (1 - exp(-a x))².
    Morse { dissociation: f64, width: f64 },
    /// Custom tabulated potential.
    Custom,
}

impl Display for WellShape {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Harmonic => f.write_str("harmonic"),
            Self::DoubleWell { a, b } => write!(f, "double_well(a={:.4}, b={:.4})", a, b),
            Self::SquareWell { depth, width } => write!(f, "square_well(depth={:.4}, width={:.4})", depth, width),
            Self::LennardJones { epsilon, sigma } => write!(f, "lennard_jones(ε={:.4}, σ={:.4})", epsilon, sigma),
            Self::Morse { dissociation, width } => write!(f, "morse(D={:.4}, a={:.4})", dissociation, width),
            Self::Custom => f.write_str("custom"),
        }
    }
}

/// A potential well attractor with bound states and tunneling properties.
///
/// # Fields
///
/// * `depth` - Depth of the potential well (energy units).
/// * `width` - Characteristic spatial width of the well.
/// * `curvature` - Second derivative at the minimum (spring constant).
/// * `barriers` - Heights of barriers separating this well from neighbors.
/// * `shape` - Functional form of the potential.
/// * `zero_point` - Ground-state zero-point energy.
/// * `bound_states` - Estimated number of discrete bound states.
/// * `tunneling_rate` - Rate of escape via tunneling.
#[derive(Debug, Clone, PartialEq)]
pub struct PotentialWell {
    pub depth: f64,
    pub width: f64,
    pub curvature: f64,
    pub barriers: Vec<f64>,
    pub shape: WellShape,
    pub zero_point: f64,
    pub bound_states: usize,
    pub tunneling_rate: f64,
}

impl PotentialWell {
    /// Creates a new potential well with geometric parameters.
    ///
    /// # Arguments
    ///
    /// * `depth` - Well depth (must be positive).
    /// * `width` - Characteristic width.
    /// * `curvature` - Second derivative at minimum.
    /// * `shape` - Potential shape.
    ///
    /// # Panics
    ///
    /// Panics if `depth` is non-positive.
    pub fn new(depth: f64, width: f64, curvature: f64, shape: WellShape) -> Self {
        assert!(depth > 0.0, "depth must be positive");
        let zero_point = Self::zero_point_energy(curvature);
        let bound_states = Self::estimate_bound_states(depth, width);
        let tunneling_rate = Self::estimate_tunneling_rate(depth, width);
        Self {
            depth,
            width,
            curvature,
            barriers: Vec::new(),
            shape,
            zero_point,
            bound_states,
            tunneling_rate,
        }
    }

    /// Returns the spatial dimension (1D wells).
    pub fn dimension(&self) -> usize {
        1
    }

    /// Estimates the zero-point energy ℏω/2 for a harmonic approximation.
    fn zero_point_energy(curvature: f64) -> f64 {
        curvature.sqrt() / 2.0
    }

    /// Estimates the number of bound states using the Bohr-Sommerfeld
    /// quantization condition.
    fn estimate_bound_states(depth: f64, width: f64) -> usize {
        let n = (width * depth.sqrt() / std::f64::consts::PI).floor();
        if n < 1.0 { 1 } else { n as usize }
    }

    /// Estimates the tunneling rate using a simplified WKB expression.
    fn estimate_tunneling_rate(depth: f64, width: f64) -> f64 {
        let exponent = -2.0 * width * depth.sqrt();
        exponent.exp()
    }

    /// Adds a barrier height separating this well from a neighboring well.
    ///
    /// # Arguments
    ///
    /// * `height` - Barrier height (must be positive).
    pub fn add_barrier(&mut self, height: f64) {
        assert!(height > 0.0, "barrier height must be positive");
        self.barriers.push(height);
    }

    /// Evaluates the potential at position `x`.
    ///
    /// # Arguments
    ///
    /// * `x` - Position coordinate.
    pub fn evaluate(&self, x: f64) -> f64 {
        match self.shape {
            WellShape::Harmonic => 0.5 * self.curvature * x.powi(2),
            WellShape::DoubleWell { a, b } => a * x.powi(4) - b * x.powi(2),
            WellShape::SquareWell { depth, width } => {
                if x.abs() < width / 2.0 { 0.0 } else { depth }
            }
            WellShape::LennardJones { epsilon, sigma } => {
                let r6 = (sigma / x.max(1e-9)).powi(6);
                4.0 * epsilon * (r6.powi(2) - r6)
            }
            WellShape::Morse { dissociation, width: a } => {
                let exp_term = (-a * x).exp();
                dissociation * (1.0 - exp_term).powi(2)
            }
            WellShape::Custom => 0.5 * self.curvature * x.powi(2),
        }
    }

    /// Computes the force `-dV/dx` at position `x`.
    ///
    /// # Arguments
    ///
    /// * `x` - Position.
    /// * `dx` - Finite difference step.
    pub fn force(&self, x: f64, dx: f64) -> f64 {
        -(self.evaluate(x + dx) - self.evaluate(x - dx)) / (2.0 * dx)
    }

    /// Checks whether a given energy is below the continuum (bound).
    ///
    /// # Arguments
    ///
    /// * `energy` - Energy to test.
    pub fn is_bound(&self, energy: f64) -> bool {
        energy < self.depth - self.zero_point
    }

    /// Computes the classical oscillation period for small oscillations
    /// about the minimum.
    pub fn oscillation_period(&self) -> f64 {
        2.0 * std::f64::consts::PI / self.curvature.sqrt()
    }

    /// Returns the highest barrier adjacent to this well.
    pub fn max_barrier(&self) -> f64 {
        self.barriers.iter().cloned().fold(0.0, f64::max)
    }

    /// Returns a summary string.
    pub fn summary(&self) -> String {
        format!(
            "PotentialWell(depth={:.4}, width={:.4}, bound_states={}, tunneling_rate={:.4e})",
            self.depth,
            self.width,
            self.bound_states,
            self.tunneling_rate
        )
    }
}

impl Display for PotentialWell {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PotentialWell(depth={:.6}, width={:.6}, shape={})",
            self.depth, self.width, self.shape
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_harmonic_evaluate() {
        let well = PotentialWell::new(1.0, 1.0, 4.0, WellShape::Harmonic);
        assert!((well.evaluate(1.0) - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_is_bound() {
        let well = PotentialWell::new(1.0, 1.0, 1.0, WellShape::Harmonic);
        assert!(well.is_bound(0.4));
        assert!(!well.is_bound(1.0));
    }

    #[test]
    fn test_zero_point() {
        let well = PotentialWell::new(1.0, 1.0, 4.0, WellShape::Harmonic);
        assert!((well.zero_point - 1.0).abs() < 1e-10);
    }
}
