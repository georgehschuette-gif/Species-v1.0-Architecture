// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! # Damped Wave Attractor
//!
//! A [`DampedWave`] models an oscillatory transient whose amplitude decays
//! exponentially due to dissipation. Damped waves appear in mechanical
//! vibrations, RLC circuits, seismic waves, and acoustic transients.
//!
//! ## Damping Regimes
//!
//! - **Underdamped**: Oscillatory decay with ringing.
//! - **Critically damped**: Fastest non-oscillatory return to equilibrium.
//! - **Overdamped**: Slow exponential decay without oscillation.
//!
//! ## Quality Factor
//!
//! The quality factor `Q` measures the sharpness of the resonance peak
//! and is inversely proportional to the damping ratio.

use std::cmp::PartialEq;
use std::fmt::{self, Display, Formatter};

/// Damping regime classification for a damped wave.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DampingRegime {
    /// Oscillatory decay (ζ < 1).
    Underdamped,
    /// Non-oscillatory critical decay (ζ = 1).
    CriticallyDamped,
    /// Slow non-oscillatory decay (ζ > 1).
    Overdamped,
    /// Logarithmic decay regime.
    Logarithmic,
}

impl Display for DampingRegime {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Underdamped => "underdamped",
            Self::CriticallyDamped => "critically_damped",
            Self::Overdamped => "overdamped",
            Self::Logarithmic => "logarithmic",
        })
    }
}

/// A damped wave transient attractor with oscillatory decay.
///
/// # Fields
///
/// * `frequency` - Natural undamped angular frequency ω₀.
/// * `damping_ratio` - Dimensionless damping ζ.
/// * `phase` - Current phase angle in radians.
/// * `amplitude` - Current peak amplitude.
/// * `quality_factor` - Sharpness of resonance Q = 1/(2ζ).
/// * `decay_time` - Time for amplitude to decay by factor e.
/// * `position` - Spatial coordinates of the wave source.
#[derive(Debug, Clone, PartialEq)]
pub struct DampedWave {
    pub frequency: f64,
    pub damping_ratio: f64,
    pub phase: f64,
    pub amplitude: f64,
    pub quality_factor: f64,
    pub decay_time: f64,
    pub position: Vec<f64>,
}

impl DampedWave {
    /// Creates a new damped wave with spectral and damping parameters.
    ///
    /// # Arguments
    ///
    /// * `frequency` - Natural frequency ω₀ > 0.
    /// * `damping_ratio` - Dimensionless damping ζ >= 0.
    /// * `phase` - Initial phase.
    /// * `amplitude` - Initial amplitude.
    /// * `position` - Source coordinates.
    ///
    /// # Panics
    ///
    /// Panics if frequency is non-positive, damping_ratio is negative,
    /// or position is empty.
    pub fn new(
        frequency: f64,
        damping_ratio: f64,
        phase: f64,
        amplitude: f64,
        position: Vec<f64>,
    ) -> Self {
        assert!(frequency > 0.0, "frequency must be positive");
        assert!(damping_ratio >= 0.0, "damping_ratio must be non-negative");
        assert!(!position.is_empty(), "position must be non-empty");
        let quality_factor = if damping_ratio > 1e-12 { 1.0 / (2.0 * damping_ratio) } else { f64::INFINITY };
        let decay_time = if damping_ratio > 1e-12 { 1.0 / (damping_ratio * frequency) } else { f64::INFINITY };
        Self {
            frequency,
            damping_ratio,
            phase,
            amplitude,
            quality_factor,
            decay_time,
            position,
        }
    }

    /// Returns the spatial dimension.
    pub fn dimension(&self) -> usize {
        self.position.len()
    }

    /// Returns the damping regime classification.
    pub fn regime(&self) -> DampingRegime {
        if self.damping_ratio < 1.0 - 1e-9 {
            DampingRegime::Underdamped
        } else if (self.damping_ratio - 1.0).abs() < 1e-9 {
            DampingRegime::CriticallyDamped
        } else {
            DampingRegime::Overdamped
        }
    }

    /// Checks whether the wave is critically damped.
    pub fn is_critically_damped(&self) -> bool {
        matches!(self.regime(), DampingRegime::CriticallyDamped)
    }

    /// Computes the damped angular frequency in the underdamped regime.
    ///
    /// For overdamped systems, returns NaN.
    pub fn damped_frequency(&self) -> f64 {
        if self.damping_ratio < 1.0 {
            self.frequency * (1.0 - self.damping_ratio.powi(2)).sqrt()
        } else {
            f64::NAN
        }
    }

    /// Computes the exponential envelope at time `t`.
    ///
    /// The envelope decays as `exp(-ζ ω₀ t)`.
    ///
    /// # Arguments
    ///
    /// * `t` - Time.
    pub fn envelope(&self, t: f64) -> f64 {
        self.amplitude * (-self.damping_ratio * self.frequency * t).exp()
    }

    /// Computes the full waveform at time `t`.
    ///
    /// For underdamped systems, this is an exponentially decaying cosine.
    /// For critically damped, it is `(A + B t) exp(-ω₀ t)`.
    /// For overdamped, it is a sum of two decaying exponentials.
    ///
    /// # Arguments
    ///
    /// * `t` - Time.
    pub fn waveform(&self, t: f64) -> f64 {
        match self.regime() {
            DampingRegime::Underdamped => {
                let omega_d = self.damped_frequency();
                self.envelope(t) * (omega_d * t + self.phase).cos()
            }
            DampingRegime::CriticallyDamped => {
                let decay = (-self.frequency * t).exp();
                (self.amplitude * decay) * (1.0 + self.frequency * t)
            }
            DampingRegime::Overdamped => {
                let zeta = self.damping_ratio;
                let omega0 = self.frequency;
                let s1 = -omega0 * (zeta - (zeta.powi(2) - 1.0).sqrt());
                let s2 = -omega0 * (zeta + (zeta.powi(2) - 1.0).sqrt());
                let c1 = self.amplitude * s2 / (s2 - s1);
                let c2 = self.amplitude * s1 / (s1 - s2);
                c1 * (s1 * t).exp() + c2 * (s2 * t).exp()
            }
            DampingRegime::Logarithmic => self.amplitude * (-self.frequency * t.ln()).exp(),
        }
    }

    /// Estimates the time of the next peak in the underdamped regime.
    ///
    /// For other regimes, returns NaN.
    pub fn next_peak(&self) -> f64 {
        if self.damping_ratio < 1.0 {
            let omega_d = self.damped_frequency();
            let mut t = (2.0 * std::f64::consts::PI - self.phase) / omega_d;
            while t < 0.0 {
                t += 2.0 * std::f64::consts::PI / omega_d;
            }
            t
        } else {
            f64::NAN
        }
    }

    /// Computes the energy remaining at time `t` relative to initial.
    pub fn residual_energy(&self, t: f64) -> f64 {
        let env = self.envelope(t);
        env.powi(2) / self.amplitude.powi(2)
    }

    /// Returns a summary string.
    pub fn summary(&self) -> String {
        format!(
            "DampedWave(f={:.4}, ζ={:.4}, Q={:.4}, regime={})",
            self.frequency,
            self.damping_ratio,
            self.quality_factor,
            self.regime()
        )
    }
}

impl Display for DampedWave {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DampedWave(frequency={:.6}, damping_ratio={:.6}, regime={})",
            self.frequency, self.damping_ratio, self.regime()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regime_underdamped() {
        let wave = DampedWave::new(1.0, 0.1, 0.0, 1.0, vec![0.0]);
        assert!(matches!(wave.regime(), DampingRegime::Underdamped));
    }

    #[test]
    fn test_regime_critically_damped() {
        let wave = DampedWave::new(1.0, 1.0, 0.0, 1.0, vec![0.0]);
        assert!(wave.is_critically_damped());
    }

    #[test]
    fn test_quality_factor() {
        let wave = DampedWave::new(2.0, 0.05, 0.0, 1.0, vec![0.0]);
        assert!((wave.quality_factor - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_envelope_decay() {
        let wave = DampedWave::new(1.0, 0.2, 0.0, 1.0, vec![0.0]);
        let e1 = wave.envelope(0.0);
        let e2 = wave.envelope(1.0);
        assert!(e2 < e1);
    }
}
