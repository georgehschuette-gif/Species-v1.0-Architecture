// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! # Critical Point Attractor
//!
//! A [`CriticalPoint`] is a singular point in parameter space where
//! physical properties diverge or become non-analytic. Critical points
//! govern phase transitions, percolation thresholds, and avalanche
//! dynamics. They are characterized by universal scaling laws and
//! diverging correlation lengths.
//!
//! ## Critical Exponents
//!
//! Power-law behavior near criticality is described by exponents α, β,
//! γ, δ, ν, and η. These exponents depend only on dimensionality and
//! symmetry, forming universality classes.
//!
//! ## Scaling Relations
//!
//! Exponents are not independent; Rushbrooke, Widom, Fisher, and
//! Josephson relations connect them, reducing the independent count to
//! two for standard systems.

use std::cmp::PartialEq;
use std::fmt::{self, Display, Formatter};

/// Critical exponents describing power-law singularities.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CriticalExponents {
    /// Specific heat exponent: C ~ |t|^{-α}.
    pub alpha: f64,
    /// Order parameter exponent: η ~ (-t)^{β}.
    pub beta: f64,
    /// Susceptibility exponent: χ ~ |t|^{-γ}.
    pub gamma: f64,
    /// Critical isotherm exponent: M ~ H^{1/δ}.
    pub delta: f64,
    /// Correlation length exponent: ξ ~ |t|^{-ν}.
    pub nu: f64,
    /// Correlation function exponent: G(r) ~ r^{-(d-2+η)}.
    pub eta: f64,
}

impl CriticalExponents {
    /// Creates a new set of critical exponents.
    ///
    /// # Arguments
    ///
    /// * `alpha` - Specific heat exponent.
    /// * `beta` - Order parameter exponent.
    /// * `gamma` - Susceptibility exponent.
    /// * `delta` - Critical isotherm exponent.
    /// * `nu` - Correlation length exponent.
    /// * `eta` - Correlation function exponent.
    pub fn new(alpha: f64, beta: f64, gamma: f64, delta: f64, nu: f64, eta: f64) -> Self {
        Self { alpha, beta, gamma, delta, nu, eta }
    }

    /// Checks the Rushbrooke inequality: α + 2β + γ ≥ 0.
    pub fn rushbrooke(&self) -> f64 {
        self.alpha + 2.0 * self.beta + self.gamma
    }

    /// Checks the Widom scaling relation: γ = β(δ - 1).
    pub fn widom(&self) -> f64 {
        self.gamma - self.beta * (self.delta - 1.0)
    }

    /// Checks the Fisher relation: γ = (2 - η)ν.
    pub fn fisher(&self) -> f64 {
        self.gamma - (2.0 - self.eta) * self.nu
    }

    /// Checks the Josephson hyperscaling relation: dν = 2 - α.
    ///
    /// # Arguments
    ///
    /// * `dimension` - Spatial dimension d.
    pub fn josephson(&self, dimension: usize) -> f64 {
        dimension as f64 * self.nu - (2.0 - self.alpha)
    }
}

/// A critical point attractor with universal scaling properties.
///
/// # Fields
///
/// * `exponents` - Critical exponents for this universality class.
/// * `universality_class` - Identifier string (e.g., "Ising2D").
/// * `dimension` - Spatial or effective dimension.
/// * `correlation_length` - Diverging correlation length ξ.
/// * `fractal_dimension` - Fractal dimension of critical clusters.
/// * `is_divergent` - Whether any property diverges at this point.
/// * `anomalous_dimension` - Scaling dimension of the order parameter.
#[derive(Debug, Clone, PartialEq)]
pub struct CriticalPoint {
    pub exponents: CriticalExponents,
    pub universality_class: String,
    pub dimension: usize,
    pub correlation_length: f64,
    pub fractal_dimension: f64,
    pub is_divergent: bool,
    pub anomalous_dimension: f64,
}

impl CriticalPoint {
    /// Creates a new critical point with exponents and metadata.
    ///
    /// # Arguments
    ///
    /// * `exponents` - Critical exponents.
    /// * `universality_class` - Class name.
    /// * `dimension` - Spatial dimension.
    /// * `fractal_dimension` - Fractal dimension of order fluctuations.
    ///
    /// # Panics
    ///
    /// Panics if `dimension` is zero.
    pub fn new(
        exponents: CriticalExponents,
        universality_class: String,
        dimension: usize,
        fractal_dimension: f64,
    ) -> Self {
        assert!(dimension > 0, "dimension must be positive");
        let correlation_length = 1e6;
        Self {
            exponents,
            universality_class,
            dimension,
            correlation_length,
            fractal_dimension,
            is_divergent: true,
            anomalous_dimension: exponents.eta / 2.0,
        }
    }

    /// Returns the dimension.
    pub fn dimension(&self) -> usize {
        self.dimension
    }

    /// Checks whether the point is critical (non-analytic).
    pub fn is_critical(&self) -> bool {
        self.is_divergent
    }

    /// Computes the correlation decay exponent.
    ///
    /// At criticality, correlations decay as a power law `G(r) ~ r^{-(d-2+η)}`.
    pub fn correlation_decay(&self, r: f64) -> f64 {
        if r <= 0.0 { return 0.0; }
        r.powi(-(self.dimension as i32 - 2 + self.exponents.eta as i32))
    }

    /// Computes the fractal (Hausdorff) dimension of critical clusters.
    pub fn fractal_dim(&self) -> f64 {
        self.fractal_dimension
    }

    /// Evaluates the order parameter scaling near criticality.
    ///
    /// # Arguments
    ///
    /// * `reduced_t` - Reduced temperature `t = (T - T_c)/T_c`.
    /// * `field` - External field conjugate to the order parameter.
    pub fn order_parameter_scaling(&self, reduced_t: f64, field: f64) -> f64 {
        let beta = self.exponents.beta;
        let delta = self.exponents.delta;
        if reduced_t > 0.0 && field == 0.0 {
            0.0
        } else if field != 0.0 {
            field.powf(1.0 / delta)
        } else {
            (-reduced_t * 2.0).powf(beta)
        }
    }

    /// Computes the scaling function for the equation of state.
    ///
    /// This returns `x / sqrt(1 + x²)` where `x = H / |t|^{βδ}`.
    pub fn scaling_function(&self, reduced_t: f64, field: f64) -> f64 {
        let beta = self.exponents.beta;
        let delta = self.exponents.delta;
        let t_pow = (-reduced_t).max(0.0).powf(beta * delta);
        let x = if t_pow > 1e-12 { field / t_pow } else { f64::INFINITY };
        x / (1.0 + x.powi(2)).sqrt()
    }

    /// Updates the correlation length based on reduced temperature.
    ///
    /// # Arguments
    ///
    /// * `reduced_t` - Reduced temperature.
    pub fn update_correlation_length(&mut self, reduced_t: f64) {
        if reduced_t.abs() > 1e-12 {
            self.correlation_length = reduced_t.abs().powf(-self.exponents.nu);
        } else {
            self.correlation_length = 1e6;
        }
    }

    /// Returns a summary string.
    pub fn summary(&self) -> String {
        format!(
            "CriticalPoint(class={}, dim={}, β={:.4}, ν={:.4})",
            self.universality_class,
            self.dimension,
            self.exponents.beta,
            self.exponents.nu
        )
    }
}

impl Display for CriticalPoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CriticalPoint(universality_class={}, dimension={}, exponents=({:.4}, {:.4}, {:.4}))",
            self.universality_class,
            self.dimension,
            self.exponents.beta,
            self.exponents.gamma,
            self.exponents.nu
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_widom_relation() {
        let ex = CriticalExponents::new(0.0, 0.325, 1.24, 4.82, 0.63, 0.026);
        assert!(ex.widom().abs() < 0.01);
    }

    #[test]
    fn test_fisher_relation() {
        let ex = CriticalExponents::new(0.0, 0.325, 1.24, 4.82, 0.63, 0.026);
        assert!(ex.fisher().abs() < 0.1);
    }

    #[test]
    fn test_critical_order_parameter() {
        let cp = CriticalPoint::new(
            CriticalExponents::new(0.0, 0.5, 1.0, 3.0, 1.0, 0.0),
            "mean_field".into(),
            3,
            1.0,
        );
        let eta = cp.order_parameter_scaling(-0.25, 0.0);
        assert!((eta - 0.5_f64.sqrt()).abs() < 1e-10);
    }
}
