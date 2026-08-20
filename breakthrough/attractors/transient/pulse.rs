// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! # Transient Pulse Attractor
//!
//! A [`TransientPulse`] models a localized, time-limited excitation that
//! propagates or decays without settling into a steady state. Pulses are
//! ubiquitous in excitable media, nerve impulses, soliton theory, and
//! signal processing.
//!
//! ## Envelope and Shape
//!
//! The pulse is characterized by its amplitude envelope, duration, and
//! shape function. Common shapes include Gaussian, Lorentzian, sech², and
//! super-Gaussian envelopes.
//!
//! ## Decay Mechanisms
//!
//! Transient pulses decay due to dissipation, diffusion, or geometric
/// spreading. The decay rate determines the pulse lifetime and energy
/// retention.

use std::cmp::PartialEq;
use std::fmt::{self, Display, Formatter};

/// Pulse shape functions for transient excitations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PulseShape {
    /// Gaussian envelope: `exp(-(t/τ)²)`.
    Gaussian,
    /// Lorentzian envelope: `1 / (1 + (t/τ)²)`.
    Lorentzian,
    /// Sech-squared envelope: `sech²(t/τ)`.
    SechSquared,
    /// Super-Gaussian: `exp(-(t/τ)^{2n})`.
    SuperGaussian { order: usize },
    /// Rectangular with raised cosine roll-off.
    RaisedCosine,
    /// Custom or user-defined shape.
    Custom,
}

impl Display for PulseShape {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Gaussian => f.write_str("gaussian"),
            Self::Lorentzian => f.write_str("lorentzian"),
            Self::SechSquared => f.write_str("sech_squared"),
            Self::SuperGaussian { order } => write!(f, "super_gaussian(n={})", order),
            Self::RaisedCosine => f.write_str("raised_cosine"),
            Self::Custom => f.write_str("custom"),
        }
    }
}

/// A transient pulse attractor with envelope and decay characteristics.
///
/// # Fields
///
/// * `amplitude` - Peak magnitude of the pulse.
/// * `duration` - Full width at half maximum (FWHM) or equivalent.
/// * `shape` - Envelope shape function.
/// * `decay_constant` - Exponential decay rate τ.
/// * `carrier_frequency` - Oscillation frequency inside the envelope.
/// * `phase` - Carrier phase at the pulse center.
/// * `position` - Spatial coordinates of the pulse center.
#[derive(Debug, Clone, PartialEq)]
pub struct TransientPulse {
    pub amplitude: f64,
    pub duration: f64,
    pub shape: PulseShape,
    pub decay_constant: f64,
    pub carrier_frequency: f64,
    pub phase: f64,
    pub position: Vec<f64>,
}

impl TransientPulse {
    /// Creates a new transient pulse with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `amplitude` - Peak magnitude (must be non-negative).
    /// * `duration` - Pulse duration (must be positive).
    /// * `shape` - Envelope shape.
    /// * `decay_constant` - Exponential decay time constant.
    /// * `carrier_frequency` - Internal oscillation frequency.
    /// * `phase` - Carrier phase at center.
    /// * `position` - Spatial center coordinates.
    ///
    /// # Panics
    ///
    /// Panics if amplitude is negative, duration is non-positive, or
    /// position is empty.
    pub fn new(
        amplitude: f64,
        duration: f64,
        shape: PulseShape,
        decay_constant: f64,
        carrier_frequency: f64,
        phase: f64,
        position: Vec<f64>,
    ) -> Self {
        assert!(amplitude >= 0.0, "amplitude must be non-negative");
        assert!(duration > 0.0, "duration must be positive");
        assert!(decay_constant > 0.0, "decay_constant must be positive");
        assert!(!position.is_empty(), "position must be non-empty");
        Self {
            amplitude,
            duration,
            shape,
            decay_constant,
            carrier_frequency,
            phase,
            position,
        }
    }

    /// Returns the spatial dimension.
    pub fn dimension(&self) -> usize {
        self.position.len()
    }

    /// Computes the pulse envelope value at normalized time `t_norm = t / τ`.
    ///
    /// The envelope is bounded by [0, amplitude].
    ///
    /// # Arguments
    ///
    /// * `t_norm` - Normalized time (0 at pulse center).
    pub fn envelope(&self, t_norm: f64) -> f64 {
        match self.shape {
            PulseShape::Gaussian => self.amplitude * (-t_norm.powi(2)).exp(),
            PulseShape::Lorentzian => self.amplitude / (1.0 + t_norm.powi(2)),
            PulseShape::SechSquared => self.amplitude * (t_norm.cosh()).powi(-2),
            PulseShape::SuperGaussian { order } => {
                self.amplitude * (-t_norm.powi((2 * order) as i32)).exp()
            }
            PulseShape::RaisedCosine => {
                let t = t_norm.abs();
                if t > 1.0 { 0.0 } else { self.amplitude * 0.5 * (1.0 + (std::f64::consts::PI * t).cos()) }
            }
            PulseShape::Custom => self.amplitude * (-t_norm.abs()).exp(),
        }
    }

    /// Computes the full pulse waveform at time `t`, including carrier
    /// oscillation.
    ///
    /// # Arguments
    ///
    /// * `t` - Absolute time.
    pub fn shape(&self, t: f64) -> f64 {
        let t_norm = (t - self.phase) / self.decay_constant;
        self.envelope(t_norm) * (self.carrier_frequency * t + self.phase).cos()
    }

    /// Computes the total energy of the pulse (integral of amplitude
    /// squared over time).
    pub fn energy(&self) -> f64 {
        match self.shape {
            PulseShape::Gaussian => self.amplitude.powi(2) * self.decay_constant * (1.0 / (2.0f64.sqrt() * std::f64::consts::PI.sqrt())),
            PulseShape::Lorentzian => self.amplitude.powi(2) * std::f64::consts::PI * self.decay_constant,
            PulseShape::SechSquared => {
                2.0 * self.amplitude.powi(2) * self.decay_constant
            }
            PulseShape::SuperGaussian { order } => {
                let denom = (2..(2 * order)).map(|x| x as u64).product::<u64>() as f64;
                self.amplitude.powi(2) * self.decay_constant / denom.max(1.0)
            }
            PulseShape::RaisedCosine => {
                self.amplitude.powi(2) * self.decay_constant * 2.0 / 3.0
            }
            PulseShape::Custom => self.amplitude.powi(2) * self.decay_constant,
        }
    }

    /// Checks whether the pulse is still active at time `t`.
    ///
    /// Activity is defined as the envelope exceeding a small threshold.
    ///
    /// # Arguments
    ///
    /// * `t` - Query time.
    pub fn is_active(&self, t: f64) -> bool {
        let t_norm = (t - self.phase).abs() / self.decay_constant;
        self.envelope(t_norm) > 1e-9
    }

    /// Estimates the peak time (time of maximum amplitude).
    pub fn peak_time(&self) -> f64 {
        self.phase
    }

    /// Computes the overlap integral between this pulse and another.
    ///
    /// # Arguments
    ///
    /// * `other` - Another transient pulse.
    /// * `t_range` - Time range `(t_min, t_max)` for integration.
    ///
    /// # Returns
    ///
    /// Approximate overlap integral using trapezoidal rule.
    pub fn overlap(&self, other: &Self, t_range: (f64, f64)) -> f64 {
        let steps = 512;
        let dt = (t_range.1 - t_range.0) / steps as f64;
        (0..=steps)
            .map(|i| {
                let t = t_range.0 + i as f64 * dt;
                self.shape(t) * other.shape(t)
            })
            .sum::<f64>()
            * dt
    }

    /// Returns a summary string.
    pub fn summary(&self) -> String {
        format!(
            "TransientPulse(amp={:.4}, dur={:.4}, shape={}, decay={:.4})",
            self.amplitude,
            self.duration,
            self.shape,
            self.decay_constant
        )
    }
}

impl Display for TransientPulse {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TransientPulse(amplitude={:.6}, duration={:.6}, shape={})",
            self.amplitude, self.duration, self.shape
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_envelope_gaussian() {
        let pulse = TransientPulse::new(1.0, 1.0, PulseShape::Gaussian, 1.0, 0.0, 0.0, vec![0.0]);
        let val = pulse.envelope(0.0);
        assert!((val - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_energy_positive() {
        let pulse = TransientPulse::new(2.0, 1.0, PulseShape::SechSquared, 0.5, 0.0, 0.0, vec![0.0]);
        assert!(pulse.energy() > 0.0);
    }

    #[test]
    fn test_is_active_decay() {
        let pulse = TransientPulse::new(1.0, 1.0, PulseShape::Gaussian, 0.1, 0.0, 0.0, vec![0.0]);
        assert!(pulse.is_active(0.0));
        assert!(!pulse.is_active(10.0));
    }
}
