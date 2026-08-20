// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

//! # Transient Attractors
//!
//! Transient attractors are non-persistent structures that exist for finite
//! durations without settling into a steady state. They include localized
//! pulses, propagating waves, and damped oscillations commonly observed in
//! excitable media, fluid dynamics, and signal processing.
//!
//! ## Types
//!
//! - [`TransientPulse`]: A localized, time-limited excitation with envelope
//!   and decay characteristics. Supports Gaussian, Lorentzian, sech², and
//!   super-Gaussian shapes.
//! - [`DampedWave`]: An oscillatory transient characterized by frequency,
//!   damping ratio, and quality factor. Models underdamped, critically
//!   damped, and overdamped regimes.
//! - [`TravellingWave`]: A propagating wave structure with phase and group
//!   velocities. Supports dispersive and non-dispersive media.
//!
//! ## Modeling Assumptions
//!
//! Transient models typically assume a dissipative background medium and
//! neglect long-term recurrence. Lifetimes are bounded by energy leakage
//! or external forcing cessation.
//!
//! ## Applications
//!
//! Transient attractors appear in nerve impulse propagation, soliton
//! transmission, seismic wave attenuation, cavity ring-down spectroscopy,
//! and transient signal detection in radar and communication systems.
//!
//! ## Shape Functions
//!
//! Envelope shapes determine energy distribution, spectral content, and
//! overlap properties. Choice of shape affects temporal coherence and
//! interference patterns in multi-pulse scenarios.

pub mod pulse;
pub mod damped_wave;
pub mod travelling;

pub use pulse::TransientPulse;
pub use damped_wave::DampedWave;
pub use travelling::TravellingWave;

use crate::Attractor;

/// Returns the number of transient attractor submodules.
pub fn transient_submodule_count() -> usize {
    3
}

/// Describes the transient attractors package scope.
pub fn describe_transient_package() -> &'static str {
    "Transient attractors: pulses, damped waves, and travelling waves."
}
