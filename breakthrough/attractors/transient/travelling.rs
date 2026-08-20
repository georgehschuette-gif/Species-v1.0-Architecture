// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! # Travelling Wave Attractor
//!
//! A [`TravellingWave`] is a propagating wave structure whose shape
//! remains invariant under translation. Travelling waves mediate transport
//! and communication in reaction-diffusion systems, nerve axons, and
//! optical fibers.
//!
//! ## Velocities
//!
//! - **Phase velocity**: Speed of a constant-phase point.
//! - **Group velocity**: Speed of the wave envelope or modulation.
//!
//! For non-dispersive media, phase and group velocities coincide. In
//! dispersive media, they differ, causing waveform distortion.
//!
//! ## Dispersion Relation
//!
//! The relationship ω(k) between angular frequency and wavenumber
//! determines dispersion, stability, and allowed propagation modes.

use std::cmp::PartialEq;
use std::fmt::{self, Display, Formatter};

/// Dispersion classification for travelling waves.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DispersionRelation {
    /// ω ∝ k (linear, non-dispersive).
    Linear,
    /// ω = a k² (quadratic, diffusive spreading).
    Quadratic,
    /// ω ∝ k^α with arbitrary α.
    PowerLaw { exponent: f64 },
    /// General arbitrary dispersion.
    Arbitrary,
    /// No dispersion (wave speed independent of frequency).
    NonDispersive,
}

impl Display for DispersionRelation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Linear => f.write_str("linear"),
            Self::Quadratic => f.write_str("quadratic"),
            Self::PowerLaw { exponent } => write!(f, "power_law(α={:.4})", exponent),
            Self::Arbitrary => f.write_str("arbitrary"),
            Self::NonDispersive => f.write_str("non_dispersive"),
        }
    }
}

/// A travelling wave attractor with propagation and dispersion properties.
///
/// # Fields
///
/// * `speed` - Phase velocity of the wave front.
/// * `wavelength` - Spatial period of the waveform.
/// * `profile` - Sampled waveform amplitudes at fixed phase.
/// * `amplitude` - Peak magnitude of the wave.
/// * `wavenumber` - Spatial frequency k = 2π / λ.
/// * `frequency` - Temporal frequency ω.
/// * `dispersion` - Dispersion relation class.
/// * `origin` - Reference origin point in space.
#[derive(Debug, Clone, PartialEq)]
pub struct TravellingWave {
    pub speed: f64,
    pub wavelength: f64,
    pub profile: Vec<f64>,
    pub amplitude: f64,
    pub wavenumber: f64,
    pub frequency: f64,
    pub dispersion: DispersionRelation,
    pub origin: Vec<f64>,
}

impl TravellingWave {
    /// Creates a new travelling wave with propagation parameters.
    ///
    /// # Arguments
    ///
    /// * `speed` - Phase velocity (must be non-zero).
    /// * `wavelength` - Spatial period (must be positive).
    /// * `profile` - Sampled waveform amplitudes.
    /// * `origin` - Reference origin coordinates.
    ///
    /// # Panics
    ///
    /// Panics if `wavelength` is non-positive, `profile` is empty, or
    /// `origin` is empty.
    pub fn new(
        speed: f64,
        wavelength: f64,
        profile: Vec<f64>,
        origin: Vec<f64>,
    ) -> Self {
        assert!(wavelength > 0.0, "wavelength must be positive");
        assert!(!profile.is_empty(), "profile must be non-empty");
        assert!(!origin.is_empty(), "origin must be non-empty");
        let amplitude = profile.iter().cloned().fold(0.0_f64, |a, b| if a > b { a } else { b }).abs();
        let wavenumber = 2.0 * std::f64::consts::PI / wavelength;
        let frequency = speed * wavenumber;
        Self {
            speed,
            wavelength,
            profile,
            amplitude,
            wavenumber,
            frequency,
            dispersion: DispersionRelation::NonDispersive,
            origin,
        }
    }

    /// Returns the spatial dimension.
    pub fn dimension(&self) -> usize {
        self.origin.len()
    }

    /// Returns the phase velocity.
    pub fn phase_velocity(&self) -> f64 {
        self.speed
    }

    /// Computes the group velocity from the dispersion relation.
    ///
    /// For non-dispersive media, this equals the phase velocity.
    pub fn group_velocity(&self) -> f64 {
        match self.dispersion {
            DispersionRelation::Linear => self.speed,
            DispersionRelation::Quadratic => 2.0 * self.speed,
            DispersionRelation::PowerLaw { exponent } => exponent * self.speed,
            DispersionRelation::Arbitrary => self.speed,
            DispersionRelation::NonDispersive => self.speed,
        }
    }

    /// Checks whether the wave is dispersive (phase velocity differs from
    /// group velocity).
    pub fn is_dispersive(&self) -> bool {
        (self.group_velocity() - self.speed).abs() > 1e-12
    }

    /// Evaluates the waveform at a spatial coordinate `x` and time `t`.
    ///
    /// The travelling wave solution is `u(x, t) = f(x - c t)`.
    ///
    /// # Arguments
    ///
    /// * `x` - Spatial coordinate (scalar for 1D, projected for higher D).
    /// * `t` - Time.
    pub fn evaluate(&self, x: f64, t: f64) -> f64 {
        let xi = x - self.speed * t;
        let idx = ((xi / self.wavelength).fract() + 1.0).fract() * self.profile.len() as f64;
        let i0 = idx.floor() as usize;
        let i1 = (i0 + 1) % self.profile.len();
        let frac = idx - i0 as f64;
        self.profile[i0] * (1.0 - frac) + self.profile[i1] * frac
    }

    /// Returns the waveform as a sampled vector over one wavelength.
    pub fn waveform(&self) -> &[f64] {
        &self.profile
    }

    /// Computes the spatial gradient of the wave at `x` and `t`.
    ///
    /// This is the negative of the time derivative at the same point
    /// for a pure travelling wave.
    ///
    /// # Arguments
    ///
    /// * `x` - Spatial coordinate.
    /// * `t` - Time.
    /// * `dx` - Spatial step for finite difference.
    pub fn spatial_gradient(&self, x: f64, t: f64, dx: f64) -> f64 {
        (self.evaluate(x + dx, t) - self.evaluate(x - dx, t)) / (2.0 * dx)
    }

    /// Computes the temporal derivative at `x` and `t`.
    ///
    /// # Arguments
    ///
    /// * `x` - Spatial coordinate.
    /// * `t` - Time.
    /// * `dt` - Temporal step for finite difference.
    pub fn temporal_derivative(&self, x: f64, t: f64, dt: f64) -> f64 {
        (self.evaluate(x, t + dt) - self.evaluate(x, t - dt)) / (2.0 * dt)
    }

    /// Computes the energy flux carried by the wave.
    ///
    /// Energy flux is proportional to amplitude squared times group
    /// velocity.
    pub fn energy_flux(&self) -> f64 {
        self.amplitude.powi(2) * self.group_velocity()
    }

    /// Returns a summary string.
    pub fn summary(&self) -> String {
        format!(
            "TravellingWave(c={:.4}, λ={:.4}, f={:.4}, dispersive={})",
            self.speed,
            self.wavelength,
            self.frequency,
            self.is_dispersive()
        )
    }
}

impl Display for TravellingWave {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TravellingWave(speed={:.6}, wavelength={:.6}, frequency={:.6}, dispersion={})",
            self.speed, self.wavelength, self.frequency, self.dispersion
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_dispersive_velocity_equality() {
        let wave = TravellingWave::new(1.0, 2.0, vec![0.0, 1.0, 0.0, -1.0], vec![0.0]);
        assert!(!wave.is_dispersive());
        assert_eq!(wave.phase_velocity(), wave.group_velocity());
    }

    #[test]
    fn test_dispersion_power_law() {
        let mut wave = TravellingWave::new(2.0, 1.0, vec![0.0, 1.0, 0.0], vec![0.0]);
        wave.dispersion = DispersionRelation::PowerLaw { exponent: 1.5 };
        assert!((wave.group_velocity() - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_evaluate_periodicity() {
        let wave = TravellingWave::new(1.0, 1.0, vec![0.0, 1.0], vec![0.0]);
        let v1 = wave.evaluate(0.0, 0.0);
        let v2 = wave.evaluate(1.0, 0.0);
        assert!((v1 - v2).abs() < 1e-10);
    }
}
