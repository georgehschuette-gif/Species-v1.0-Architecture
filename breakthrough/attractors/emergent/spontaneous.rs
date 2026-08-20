// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! # Spontaneous Attractor
//!
//! A [`SpontaneousAttractor`] is an order parameter that emerges
//! spontaneously when a symmetry-breaking threshold is crossed. It is the
//! cornerstone of Landau theory, pattern formation, and self-organization.
//! The attractor is not prescribed by individual components but arises
//! from collective interactions.
//!
//! ## Order Parameter
//!
//! The order parameter `η` measures the degree of macroscopic order.
//! Below the critical threshold, `η = 0` (symmetric phase). Above the
//! threshold, `η ≠ 0` and the system selects one of many equivalent
//! broken-symmetry states.
//!
//! ## Symmetry Breaking
//!
//! Continuous symmetries are broken spontaneously, while discrete
//! symmetries may or may not be broken depending on the order parameter
//! domain.

use std::cmp::PartialEq;
use std::fmt::{self, Display, Formatter};

/// Types of symmetry that may be spontaneously broken.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SymmetryType {
    /// Continuous rotational symmetry (e.g., XY model).
    ContinuousRotational,
    /// Discrete translational symmetry (e.g., crystals).
    DiscreteTranslational,
    /// Time-reversal symmetry.
    TimeReversal,
    /// Gauge symmetry (e.g., superconductivity).
    Gauge,
    /// Combined internal and spatial symmetry.
    Mixed,
    /// No symmetry breaking.
    Unbroken,
}

impl Display for SymmetryType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::ContinuousRotational => "continuous_rotational",
            Self::DiscreteTranslational => "discrete_translational",
            Self::TimeReversal => "time_reversal",
            Self::Gauge => "gauge",
            Self::Mixed => "mixed",
            Self::Unbroken => "unbroken",
        })
    }
}

/// A spontaneously emerging attractor with order parameter dynamics.
///
/// # Fields
///
/// * `order_parameter` - Magnitude of the emergent order η.
/// * `threshold` - Critical value of the control parameter at which
///   symmetry breaks.
/// * `control_parameter` - Current value of the tuning parameter.
/// * `symmetry` - Type of symmetry being broken.
/// * `susceptibility` - Response function χ = ∂η/∂h.
/// * `correlation_length` - Spatial extent of order fluctuations.
/// * `metastable` - Whether the order parameter can persist in a
///   local minimum.
#[derive(Debug, Clone, PartialEq)]
pub struct SpontaneousAttractor {
    pub order_parameter: f64,
    pub threshold: f64,
    pub control_parameter: f64,
    pub symmetry: SymmetryType,
    pub susceptibility: f64,
    pub correlation_length: f64,
    pub metastable: bool,
}

impl SpontaneousAttractor {
    /// Creates a new spontaneous attractor with symmetry-breaking
    /// parameters.
    ///
    /// # Arguments
    ///
    /// * `order_parameter` - Initial order magnitude.
    /// * `threshold` - Critical control-parameter value.
    /// * `control_parameter` - Current tuning parameter.
    /// * `symmetry` - Type of symmetry.
    ///
    /// # Panics
    ///
    /// Panics if `threshold` is zero (would cause division by zero).
    pub fn new(
        order_parameter: f64,
        threshold: f64,
        control_parameter: f64,
        symmetry: SymmetryType,
    ) -> Self {
        assert!(threshold != 0.0, "threshold must be non-zero");
        let excess = control_parameter - threshold;
        let susceptibility = if excess.abs() > 1e-12 { 1.0 / excess.abs() } else { 1e12 };
        let correlation_length = if excess.abs() > 1e-12 { 1.0 / excess.abs().sqrt() } else { 1e6 };
        Self {
            order_parameter,
            threshold,
            control_parameter,
            symmetry,
            susceptibility,
            correlation_length,
            metastable: false,
        }
    }

    /// Returns the distance from the critical threshold.
    pub fn excess(&self) -> f64 {
        self.control_parameter - self.threshold
    }

    /// Checks whether the system is in the symmetry-broken phase.
    ///
    /// Returns `true` when `control_parameter > threshold`.
    pub fn is_broken(&self) -> bool {
        self.control_parameter > self.threshold
    }

    /// Computes the equilibrium order parameter from Landau mean-field
    /// theory.
    ///
    /// For a quartic potential `V(η) = a (T - T_c) η² + b η⁴`, the
    /// equilibrium order is `η_eq = sqrt(-a (T - T_c) / (2b))` for
    /// `T < T_c`.
    pub fn order(&self, a: f64, b: f64) -> f64 {
        let excess = self.excess();
        if excess >= 0.0 {
            0.0
        } else {
            ((a * excess) / b).sqrt()
        }
    }

    /// Computes the critical distance (reduced temperature or field).
    pub fn critical_distance(&self) -> f64 {
        (self.control_parameter / self.threshold) - 1.0
    }

    /// Checks whether the attractor is in a metastable local minimum.
    pub fn is_metastable(&self) -> bool {
        self.metastable && self.order_parameter > 1e-9
    }

    /// Sets metastability flag.
    pub fn set_metastable(&mut self, metastable: bool) {
        self.metastable = metastable;
    }

    /// Computes the free energy landscape curvature at the current order
    /// parameter.
    pub fn curvature(&self, a: f64, b: f64) -> f64 {
        let eta = self.order_parameter;
        2.0 * a * (self.control_parameter - self.threshold) + 12.0 * b * eta.powi(2)
    }

    /// Returns a summary string.
    pub fn summary(&self) -> String {
        format!(
            "SpontaneousAttractor(η={:.4}, threshold={:.4}, broken={}, symmetry={})",
            self.order_parameter,
            self.threshold,
            self.is_broken(),
            self.symmetry
        )
    }
}

impl Display for SpontaneousAttractor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SpontaneousAttractor(order_parameter={:.6}, control_parameter={:.6}, symmetry={})",
            self.order_parameter, self.control_parameter, self.symmetry
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symmetry_broken() {
        let sa = SpontaneousAttractor::new(0.5, 1.0, 1.5, SymmetryType::ContinuousRotational);
        assert!(sa.is_broken());
        assert_eq!(sa.excess(), 0.5);
    }

    #[test]
    fn test_landau_order() {
        let sa = SpontaneousAttractor::new(0.0, 1.0, 0.5, SymmetryType::Gauge);
        let eta = sa.order(-1.0, 1.0);
        assert!((eta - 0.5_f64.sqrt()).abs() < 1e-10);
    }

    #[test]
    fn test_metastability() {
        let mut sa = SpontaneousAttractor::new(0.8, 1.0, 0.5, SymmetryType::DiscreteTranslational);
        sa.set_metastable(true);
        assert!(sa.is_metastable());
    }
}
