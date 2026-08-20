// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! # Limit Cycle Attractor
//!
//! A [`LimitCycle`] is a periodic orbit in phase space that attracts nearby
//! trajectories. Unlike fixed points, limit cycles represent sustained
//! oscillations and are fundamental to biological rhythms, electronic
//! oscillators, and chemical reaction dynamics.
//!
//! ## Stability
//!
//! A limit cycle is stable if trajectories starting within a tubular
//! neighborhood converge to it under forward time. The amplitude of
//! perturbations decays exponentially along the transverse direction.
//!
//! ## Properties
//!
//! - **Period**: Time required to complete one orbit.
//! - **Winding number**: Number of rotations per period (usually 1 for
//!   simple cycles).
//! - **Enclosed area**: Geometric area bounded by the cycle in 2D
//!   projections.

use std::cmp::PartialEq;
use std::fmt::{self, Display, Formatter};

/// A stable limit cycle attractor in phase space.
///
/// # Fields
///
/// * `period` - Duration of one complete orbit.
/// * `amplitude` - Peak deviation from the cycle's centroid.
/// * `phase` - Current angular position along the cycle [0, 2π).
/// * `winding_number` - Number of times the trajectory winds around
///   the cycle per period.
/// * `phase_centroid` - Mean coordinates over one period.
/// * `transverse_eigenvalue` - Real part governing radial decay rate.
#[derive(Debug, Clone, PartialEq)]
pub struct LimitCycle {
    pub period: f64,
    pub amplitude: f64,
    pub phase: f64,
    pub winding_number: usize,
    pub phase_centroid: Vec<f64>,
    pub transverse_eigenvalue: f64,
}

impl LimitCycle {
    /// Creates a new limit cycle with the specified geometric and spectral
    /// properties.
    ///
    /// # Arguments
    ///
    /// * `period` - Orbit period in arbitrary time units.
    /// * `amplitude` - Peak deviation from centroid.
    /// * `phase` - Initial phase angle in radians.
    /// * `winding_number` - Integer winding number per period.
    /// * `phase_centroid` - Centroid coordinates in phase space.
    /// * `transverse_eigenvalue` - Negative real part for stable cycles.
    ///
    /// # Panics
    ///
    /// Panics if `period` is non-positive, `amplitude` is negative, or
    /// `phase_centroid` is empty.
    pub fn new(
        period: f64,
        amplitude: f64,
        phase: f64,
        winding_number: usize,
        phase_centroid: Vec<f64>,
        transverse_eigenvalue: f64,
    ) -> Self {
        assert!(period > 0.0, "period must be positive");
        assert!(amplitude >= 0.0, "amplitude must be non-negative");
        assert!(!phase_centroid.is_empty(), "phase_centroid must be non-empty");
        assert!(transverse_eigenvalue < 0.0, "stable cycle requires negative transverse eigenvalue");
        Self {
            period,
            amplitude,
            phase,
            winding_number,
            phase_centroid,
            transverse_eigenvalue,
        }
    }

    /// Returns the frequency of the limit cycle in hertz (cycles per unit
    /// time).
    pub fn frequency(&self) -> f64 {
        1.0 / self.period
    }

    /// Returns the angular frequency ω = 2π / T.
    pub fn angular_frequency(&self) -> f64 {
        2.0 * std::f64::consts::PI / self.period
    }

    /// Determines stability based on the transverse eigenvalue.
    ///
    /// A negative real part indicates asymptotic stability.
    pub fn is_stable(&self) -> bool {
        self.transverse_eigenvalue < 0.0
    }

    /// Computes the area enclosed by the limit cycle in a 2D projection.
    ///
    /// This is a simplified model assuming an elliptical shape with the
    /// given amplitude.
    pub fn enclosed_area(&self) -> f64 {
        std::f64::consts::PI * self.amplitude.powi(2)
    }

    /// Applies a small perturbation to the phase and amplitude.
    ///
    /// The transverse eigenvalue governs amplitude decay, while phase
    /// is shifted linearly with time.
    ///
    /// # Arguments
    ///
    /// * `phase_shift` - Change in angular position.
    /// * `amplitude_perturbation` - Change in peak deviation.
    /// * `dt` - Time elapsed since last observation.
    ///
    /// # Returns
    ///
    /// Updated phase and amplitude after perturbation.
    pub fn perturb(&mut self, phase_shift: f64, amplitude_perturbation: f64, dt: f64) {
        let decay = (self.transverse_eigenvalue * dt).exp();
        self.amplitude = (self.amplitude + amplitude_perturbation).max(0.0) * decay;
        self.phase = (self.phase + phase_shift + self.angular_frequency() * dt)
            % (2.0 * std::f64::consts::PI);
        if self.phase < 0.0 {
            self.phase += 2.0 * std::f64::consts::PI;
        }
    }

    /// Returns the phase velocity in radians per unit time.
    pub fn phase_velocity(&self) -> f64 {
        self.angular_frequency()
    }

    /// Estimates the next crossing time of a reference phase.
    ///
    /// # Arguments
    ///
    /// * `reference_phase` - Target phase angle.
    ///
    /// # Returns
    ///
    /// Time remaining until the cycle reaches `reference_phase`.
    pub fn next_crossing(&self, reference_phase: f64) -> f64 {
        let mut diff = reference_phase - self.phase;
        if diff < 0.0 {
            diff += 2.0 * std::f64::consts::PI;
        }
        diff / self.angular_frequency()
    }

    /// Computes the Floquet multiplier (transverse Lyapunov exponent)
    /// over one full period.
    pub fn floquet_multiplier(&self) -> f64 {
        self.transverse_eigenvalue * self.period
    }

    /// Returns a summary string.
    pub fn summary(&self) -> String {
        format!(
            "LimitCycle(T={:.4}, f={:.4}, amp={:.4}, stable={})",
            self.period,
            self.frequency(),
            self.amplitude,
            self.is_stable()
        )
    }
}

impl Display for LimitCycle {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "LimitCycle(period={:.6}, amplitude={:.6}, phase={:.6}, winding={})",
            self.period, self.amplitude, self.phase, self.winding_number
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frequency_calculation() {
        let cycle = LimitCycle::new(
            2.0,
            1.0,
            0.0,
            1,
            vec![0.0, 0.0],
            -0.1,
        );
        assert!((cycle.frequency() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_perturb_amplitude_decay() {
        let mut cycle = LimitCycle::new(
            1.0,
            2.0,
            0.0,
            1,
            vec![0.0],
            -1.0,
        );
        cycle.perturb(0.0, 1.0, 1.0);
        assert!(cycle.amplitude < 2.0 + 1.0);
        assert!(cycle.amplitude > 0.0);
    }

    #[test]
    fn test_enclosed_area() {
        let cycle = LimitCycle::new(
            1.0,
            3.0,
            0.0,
            1,
            vec![0.0, 0.0],
            -0.5,
        );
        let expected = std::f64::consts::PI * 9.0;
        assert!((cycle.enclosed_area() - expected).abs() < 1e-10);
    }
}
