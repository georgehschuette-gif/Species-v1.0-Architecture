// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! # Phase Transition Attractor
//!
//! A [`PhaseTransition`] models a boundary between distinct macroscopic
//! phases of matter or abstract systems. Phase transitions are characterized
//! by non-analytic behavior in thermodynamic potentials, diverging
//! correlation lengths, and universal critical exponents.
//!
//! ## Transition Orders
//!
//! - **First-order**: Discontinuous jump in order parameter; latent heat
//!   present.
//! - **Second-order**: Continuous order parameter with diverging
//!   susceptibility.
//! - **Continuous**: Generalization of second-order to higher-order
//!   derivatives.
//!
//! ## Universality
//!
//! Systems sharing symmetry and dimensionality belong to the same
//! universality class and exhibit identical critical exponents.

use std::cmp::PartialEq;
use std::fmt::{self, Display, Formatter};

/// Order of a phase transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransitionOrder {
    /// Discontinuous first derivative of free energy.
    FirstOrder,
    /// Continuous first derivative, discontinuous second derivative.
    SecondOrder,
    /// Higher-order continuous transition.
    HigherOrder { discontinuity_rank: usize },
    /// Tricritical point where transition order changes.
    Tricritical,
    /// Infinite-order (Kosterlitz-Thouless).
    InfiniteOrder,
    /// No true transition (crossover).
    Crossover,
}

impl Display for TransitionOrder {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::FirstOrder => f.write_str("first_order"),
            Self::SecondOrder => f.write_str("second_order"),
            Self::HigherOrder { discontinuity_rank } => write!(f, "higher_order(rank={})", discontinuity_rank),
            Self::Tricritical => f.write_str("tricritical"),
            Self::InfiniteOrder => f.write_str("infinite_order"),
            Self::Crossover => f.write_str("crossover"),
        }
    }
}

/// A phase transition attractor between distinct macroscopic phases.
///
/// # Fields
///
/// * `order_parameter` - Current value of the order parameter.
/// * `critical_temperature` - Critical point temperature T_c.
/// * `current_temperature` - Actual system temperature.
/// * `transition_order` - Classification of the transition.
/// * `susceptibility` - Response function χ.
/// * `correlation_length` - Spatial extent of critical fluctuations.
/// * `specific_heat` - Specific heat capacity at constant pressure.
/// * `phases` - Labels of the coexisting phases.
#[derive(Debug, Clone, PartialEq)]
pub struct PhaseTransition {
    pub order_parameter: f64,
    pub critical_temperature: f64,
    pub current_temperature: f64,
    pub transition_order: TransitionOrder,
    pub susceptibility: f64,
    pub correlation_length: f64,
    pub specific_heat: f64,
    pub phases: (String, String),
}

impl PhaseTransition {
    /// Creates a new phase transition model.
    ///
    /// # Arguments
    ///
    /// * `order_parameter` - Initial order parameter value.
    /// * `critical_temperature` - Critical temperature T_c.
    /// * `current_temperature` - Actual temperature.
    /// * `transition_order` - Transition classification.
    /// * `phases` - Names of the two phases.
    ///
    /// # Panics
    ///
    /// Panics if `critical_temperature` is non-positive.
    pub fn new(
        order_parameter: f64,
        critical_temperature: f64,
        current_temperature: f64,
        transition_order: TransitionOrder,
        phases: (String, String),
    ) -> Self {
        assert!(critical_temperature > 0.0, "critical_temperature must be positive");
        let reduced = (current_temperature - critical_temperature) / critical_temperature;
        let susceptibility = if reduced.abs() > 1e-9 { 1.0 / reduced.abs() } else { 1e9 };
        let correlation_length = if reduced.abs() > 1e-9 { 1.0 / reduced.abs().sqrt() } else { 1e4 };
        let specific_heat = if reduced.abs() > 1e-9 { reduced.powi(-2) } else { 1e4 };
        Self {
            order_parameter,
            critical_temperature,
            current_temperature,
            transition_order,
            susceptibility,
            correlation_length,
            specific_heat,
            phases,
        }
    }

    /// Returns the reduced temperature `t = (T - T_c) / T_c`.
    pub fn reduced_temperature(&self) -> f64 {
        (self.current_temperature - self.critical_temperature) / self.critical_temperature
    }

    /// Checks whether the system is at the critical point.
    pub fn is_critical(&self) -> bool {
        (self.current_temperature - self.critical_temperature).abs() < 1e-9
    }

    /// Computes the Binder cumulant for finite-size scaling analysis.
    pub fn binder_cumulant(&self, system_size: usize) -> f64 {
        let u = self.order_parameter.powi(4);
        let m2 = self.order_parameter.powi(2);
        if m2 > 1e-12 {
            1.0 - u / (3.0 * m2.powi(2))
        } else {
            0.0
        }
    }

    /// Computes the critical exponent β from mean-field theory.
    ///
    /// For a second-order transition, β = 0.5 in mean-field.
    pub fn critical_exponent_beta(&self) -> f64 {
        match self.transition_order {
            TransitionOrder::SecondOrder => 0.5,
            TransitionOrder::Tricritical => 0.25,
            TransitionOrder::HigherOrder { discontinuity_rank } if discontinuity_rank >= 3 => 0.0,
            _ => 0.5,
        }
    }

    /// Computes the correlation length exponent ν.
    pub fn correlation_length_exponent(&self) -> f64 {
        match self.transition_order {
            TransitionOrder::SecondOrder => 0.5,
            TransitionOrder::Tricritical => 0.5,
            _ => 0.5,
        }
    }

    /// Estimates the coexistence curve near the critical point.
    ///
    /// Returns the order parameter on the coexistence branch.
    pub fn coexistence_order_parameter(&self, reduced_t: f64) -> f64 {
        let beta = self.critical_exponent_beta();
        if reduced_t < 0.0 { (-reduced_t).powf(beta) } else { 0.0 }
    }

    /// Returns a summary string.
    pub fn summary(&self) -> String {
        format!(
            "PhaseTransition(order={:.4}, T_c={:.4}, T={:.4}, order={}, critical={})",
            self.order_parameter,
            self.critical_temperature,
            self.current_temperature,
            self.transition_order,
            self.is_critical()
        )
    }
}

impl Display for PhaseTransition {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PhaseTransition(phases=({}, {}), T_c={:.6}, order={})",
            self.phases.0, self.phases.1, self.critical_temperature, self.transition_order
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reduced_temperature() {
        let pt = PhaseTransition::new(0.0, 1.0, 1.5, TransitionOrder::SecondOrder, ("A".into(), "B".into()));
        assert!((pt.reduced_temperature() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_is_critical() {
        let pt = PhaseTransition::new(0.0, 1.0, 1.0, TransitionOrder::SecondOrder, ("A".into(), "B".into()));
        assert!(pt.is_critical());
    }

    #[test]
    fn test_coexistence_curve() {
        let pt = PhaseTransition::new(0.0, 1.0, 0.5, TransitionOrder::SecondOrder, ("A".into(), "B".into()));
        let eta = pt.coexistence_order_parameter(-0.5);
        assert!((eta - 0.5_f64.sqrt()).abs() < 1e-10);
    }
}
